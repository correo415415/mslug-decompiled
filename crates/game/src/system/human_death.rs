//! Muerte de los humanos (soldado rebelde, POW, otros): según el tipo de
//! humano y el tipo de daño elige cómo cae (voltereta, desplome, sale
//! despedido, vuela hacia la cámara, arde…), con sangre, trozos y llamas,
//! y al acabar parpadea y se da de baja.
//!
//! Original: `asm/pow/human_death_049ff2.s` (`HumanDeath_EntryKind0_04a014`
//! … `HumanDeath_PickGibPtr_04aba8`) y `$08F308`.

use super::{anim, collision, motion, platforms, player_ground, rng, slope, slug, soldier, sound, tasks, video};
use crate::layout::{ground_fx as G, human_death as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn add16(m: &mut impl Memory, a: u32, d: u16) {
    let v = m.read_u16(a);
    m.write_u16(a, v.wrapping_add(d));
}

fn flag(on: bool) -> u8 {
    if on { 0xFF } else { 0 }
}

fn rand(m: &mut impl Memory) -> u16 {
    rng::next(m).0
}

/// Gráfico según la sangre (`pair.0` con sangre, `pair.1` sin).
fn blood_gfx(m: &mut impl Memory, pair: (u16, u16)) -> u16 {
    if m.read_u8(L::BLOOD) != 0 { pair.0 } else { pair.1 }
}

/// Suelta los sprites y toma los gráficos `gfx`; como el original, deja
/// en `d4` la entrada nueva (la fila que luego mira el sondeo del suelo).
/// Original: las entradas de `HumanDeath_PlayCryByKind_04a0d4`
/// (`$013600` + `$00236E`).
pub fn set_gfx(m: &mut impl Machine, t: u32, gfx: u16) {
    video::release_task_entries(m, t);
    let before = m.read_u16(t + T::PAL_SLOT_COUNT);
    let read = m.read_u16(L::FREE_READ);
    anim::alloc_entry(m, t, gfx);
    let mut r = m.regs();
    // `$00236E` deja en `d3` la posición de lectura del anillo de libres
    // si tomó una entrada nueva.
    let fresh = m.read_u16(L::FREE_READ) != read;
    r.d[3] = (r.d[3] & 0xFFFF_0000) | u32::from(if fresh { read } else { 0 });
    if m.read_u16(t + T::PAL_SLOT_COUNT) != before {
        let entry = m.read_u16(t + L::ENTRY);
        let base = (u32::from(gfx) << 6).wrapping_add(L::GFX_BASE);
        r.d[4] = (base & 0xFFFF_0000) | u32::from(entry);
    }
    m.set_regs(r);
}

/// Pide el grito guardado en la tarea. Original: `HumanDeath_LoadTimer_04a1ba`.
pub fn play_cry(m: &mut impl Machine, t: u32) {
    let code = m.read_u16(t + L::CRY);
    sound::play(m, code);
}

/// Gráfico del cuerpo (con o sin sangre). Original: `HumanDeath_PlayCryByKind_04a0d4`.
pub fn body_gfx(m: &mut impl Machine, t: u32) {
    let g = blood_gfx(m, L::GFX_BODY);
    set_gfx(m, t, g);
}

/// `+$70` = ¿pisa suelo? Original: `$027F60` + `scc +$70`.
fn mark_grounded(m: &mut impl Machine, t: u32) {
    let g = soldier::on_ground(m, t);
    let mut r = m.regs();
    r.d[3] = (r.d[3] & 0xFFFF_FF00) | u32::from(flag(g));
    m.set_regs(r);
    // El original hace `scc` tras `$027F60`, que siempre sale con C = 0
    // (los `move.w` finales la borran): la marca queda siempre puesta.
    m.write_u8(t + L::GROUNDED, 0xFF);
}

/// Velocidad X `v` hacia atrás (negada si mira a la izquierda).
/// Original: `HumanDeath_SetVelXByFacing_04a1f0`.
fn vel_back(m: &mut impl Memory, t: u32, v: u16) {
    let v = if m.read_u8(t + L::FACING) & 1 == 0 { v.wrapping_neg() } else { v };
    m.write_u16(t + L::VEL_X, v);
}

/// Velocidad X `v` hacia delante. Original: `$05DCA4`.
fn vel_fwd(m: &mut impl Memory, t: u32, v: u16) {
    let v = if m.read_u8(t + L::FACING) & 1 != 0 { v.wrapping_neg() } else { v };
    m.write_u16(t + L::VEL_X, v);
}

/// Frena 1/32 las dos velocidades (al menos 1). Original:
/// `HumanDeath_DampVelocity_04a1c6`.
pub fn damp(m: &mut impl Memory, t: u32) {
    for f in [L::VEL_X, L::VEL_Y] {
        let v = m.read_u16(t + f);
        if v != 0 {
            let d = (v as i16) >> 5;
            let d = if d == 0 { 1 } else { d };
            m.write_u16(t + f, v.wrapping_sub(d as u16));
        }
    }
}

/// ¿Sale un número al azar por debajo del byte bajo de la Y?
/// Original: `HumanDeath_RandBelowY_04a202`.
fn rand_below_y(m: &mut impl Memory, t: u32) -> bool {
    let r = rand(m) as u8;
    r < m.read_u16(t + L::Y) as u8
}

/// ¿Celda sólida bajo los pies (y el tipo de tile lo permite)? Deja en
/// `d3`/`d4` la celda mirada como el original. Original: `$027FD8` (con la
/// escena) y `Entity_TestSolidAtFeet_027fac`.
fn feet(m: &mut impl Machine, t: u32, scene: bool, mark_d3: bool) -> bool {
    if scene {
        platforms::move_with_scene(m, t);
    }
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y).wrapping_sub(1));
    let tile = collision::lookup_tile(m, t, x, y);
    let mut r = m.regs();
    let (col, row) = tile.frac.unwrap_or((r.d[3] as u16, r.d[4] as u16));
    let on = slope::solid_bit(m, tile.record, col, row) && m.read_u8(tile.record + G::T_NO_STAND) == 0;
    r.d[3] = (r.d[3] & 0xFFFF_0000) | u32::from(col);
    if mark_d3 {
        r.d[3] = (r.d[3] & 0xFFFF_FF00) | u32::from(flag(on));
    }
    r.d[4] = (r.d[4] & 0xFFFF_0000) | u32::from(row);
    m.set_regs(r);
    on
}

fn row(m: &mut impl Machine) -> u16 {
    m.regs().d[4] as u16
}

fn at(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

/// Física en el aire: si no pisa, cae con gravedad; si pisa, anda sobre el
/// suelo. Original: `HumanDeath_PhysicsAir_04a218`.
pub fn physics_air(m: &mut impl Machine, t: u32) {
    let f = soldier::probe(m, t);
    m.write_u8(t + L::GROUNDED, flag(!f.in_air));
    damp(m, t);
    if f.in_air {
        m.write_u16(t + L::ACC_Y, L::FALL_ACC);
        let row = row(m);
        let c = motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row));
        m.write_u8(t + L::GROUNDED, flag(c));
        return;
    }
    damp(m, t);
    player_ground::move_ground(m, t);
}

/// Física pegada al suelo. Original: `HumanDeath_PhysicsGround_04a268`.
pub fn physics_ground(m: &mut impl Machine, t: u32) {
    let on = feet(m, t, true, true);
    m.write_u8(t + L::GROUNDED, flag(on));
    damp(m, t);
    if !on {
        let at = at(m);
        let c = motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at));
        m.write_u8(t + L::GROUNDED, flag(c));
        return;
    }
    damp(m, t);
    player_ground::move_ground(m, t);
    let on = feet(m, t, false, false);
    m.write_u8(t + L::GROUNDED, flag(on));
}

/// Segundo paso de caída (sin frenar) y vuelve a mirar el suelo.
/// Original: `HumanDeath_PhysicsFall_04a2be`.
pub fn physics_fall(m: &mut impl Machine, t: u32) {
    let f = soldier::probe(m, t);
    m.write_u8(t + L::GROUNDED, flag(!f.in_air));
    if f.in_air {
        m.write_u16(t + L::ACC_Y, L::FALL_ACC);
        let row = row(m);
        let c = motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row));
        m.write_u8(t + L::GROUNDED, flag(c));
    }
    player_ground::move_ground(m, t);
    let at = at(m);
    let f = collision::probe_floor(m, t, at);
    m.write_u8(t + L::GROUNDED, flag(!f.in_air));
}

/// Fuera de la zona (por los lados o por abajo) se da de baja.
/// Original: `HumanDeath_PhysicsFall_04a2be__L04a35e`.
fn cull(m: &mut impl Machine, t: u32) -> Flow {
    let x = m.read_u16(t + L::X).wrapping_add(L::CULL_X.0);
    if x >= L::CULL_X.1 || m.read_u16(t + L::Y) < L::CULL_Y {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Apunta la posición en el anillo de los caídos. Original: `$08F308`.
pub fn ring_push(m: &mut impl Memory, t: u32) {
    let head = m.read_u16(L::RING + 2);
    let e = L::RING + 8 + u32::from(head);
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    m.write_u16(e, x);
    m.write_u16(e + 2, y);
    let n = m.read_u16(L::RING + 6);
    if n < L::RING_MAX {
        let next = head + 4;
        m.write_u16(L::RING + 2, if next < L::RING_SIZE { next } else { 0 });
        m.write_u16(L::RING + 6, n + 1);
    }
}

/// Cuerpo nuevo: sigue a la escena, gráficos del cuerpo, sin caja ni
/// rutina extra, sin aceleración y delante. Original: `HumanDeath_ResetBody_04a09c`.
pub fn reset_body(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    body_gfx(m, t);
    set_bits(m, t + L::FLAGS_12, L::BIT_12_1, false);
    m.write_u16(t + L::FIELD_34, 0xFFFF);
    m.write_u32(t + L::HANDLER_60, L::NIL);
    mark_grounded(m, t);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u16(t + L::ACC_Y, 0);
    m.write_u16(t + L::ATTR, L::BODY_ATTR);
}

/// Muerte del humano de tipo `kind` (0–3): por dónde le dieron, el estado
/// de su tabla según el tipo de daño, cuerpo nuevo y anillo.
/// Original: `HumanDeath_EntryKind0_04a014` / `1_04a024` / `2_04a034` /
/// `__L04a044` (y `__L04a050`).
pub fn entry(m: &mut impl Machine, kind: usize) -> Flow {
    let t = m.task_register();
    mark_grounded(m, t);
    let state = pick_state(m, t, kind);
    run(m, state)
}

fn pick_state(m: &mut impl Machine, t: u32, kind: usize) -> u32 {
    let ahead = m.read_u16(t + L::X) >= m.read_u16(t + L::HIT_X);
    let facing = m.read_u8(t + L::FACING) & 1 != 0;
    m.write_u8(t + L::BEHIND, flag(ahead) ^ flag(facing));
    let mut k = m.read_u8(t + L::HIT_KIND);
    if k >= L::HIT_KINDS {
        k = L::HIT_KIND_DEFAULT;
    }
    let state = m.read_u32(L::STATE_TABLES[kind] + u32::from(k) * 4);
    reset_body(m, t);
    ring_push(m, t);
    state
}

/// Ejecuta el estado de muerte `state` (una entrada de las tablas).
pub fn run(m: &mut impl Machine, state: u32) -> Flow {
    match state {
        0x04_A3F8 => tumble(m),
        0x04_A568 => fall(m),
        0x04_A5A6 => slump(m),
        0x04_A5B4 => crumple(m),
        0x04_A604 => shot(m),
        0x04_A680 => sit_or_flip(m),
        0x04_A68C => sit(m),
        0x04_A702 => burst_pick(m),
        0x04_A72E => burst_high(m),
        0x04_A7B0 => burst_ground(m),
        0x04_A88C => burst_ground_probe(m),
        0x04_A89A => blown_back(m),
        0x04_A8FA => blown_up(m),
        0x04_A948 => on_fire(m),
        other => Flow::Goto(other),
    }
}

/// Anima hasta el final y se apaga. Original: `HumanDeath_PhysicsFall_04a2be__L04a382`.
pub fn anim_loop_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::ANIM_LOOP);
    anim_loop(m)
}

/// Original: `HumanDeath_PhysicsFall_04a2be+$CA`.
pub fn anim_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_air(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::FADE);
    }
    cull(m, t)
}

fn start_and_loop(m: &mut impl Machine, script: u32) -> Flow {
    let t = m.task_register();
    anim::start(m, t, script);
    anim_loop_start(m)
}

/// Unos frames quieto y luego parpadea hasta desaparecer.
/// Original: `HumanDeath_FadeOut_04a39e`.
pub fn fade(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::COUNT, L::FADE_FRAMES);
    set_handler(m, t, L::FADE_LOOP);
    fade_loop(m)
}

/// Original: `HumanDeath_FadeOut_04a39e+$0C`.
pub fn fade_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_air(m, t);
    anim::step(m, t);
    let n = m.read_u8(t + L::COUNT).wrapping_sub(1);
    m.write_u8(t + L::COUNT, n);
    if n == 0 {
        set_handler(m, t, L::BLINK);
    }
    cull(m, t)
}

/// Original: `HumanDeath_FadeOut_04a39e+$26`.
pub fn blink(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::COUNT, L::FADE_FRAMES);
    set_handler(m, t, L::BLINK_LOOP);
    blink_loop(m)
}

/// Original: `HumanDeath_FadeOut_04a39e+$32`.
pub fn blink_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_air(m, t);
    let n = m.read_u8(t + L::COUNT).wrapping_sub(1);
    m.write_u8(t + L::COUNT, n);
    if n == 0 {
        return tasks::finish(m);
    }
    if n & 1 == 0 {
        anim::step(m, t);
    }
    cull(m, t)
}

/// Sangre y voltereta hacia donde le empujó el golpe.
/// Original: `HumanDeath_FadeOut_04a39e__L04a3f8`.
pub fn tumble(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    spawn_blood(m, t, 1);
    let back = m.read_u8(t + L::BEHIND) != 0;
    tumble_start(m, back)
}

/// Original: `HumanDeath_TumbleBackStart_04a420` / `TumbleFwdStart_04a4c8`.
pub fn tumble_start(m: &mut impl Machine, back: bool) -> Flow {
    let t = m.task_register();
    if rand_below_y(m, t) {
        return flip(m);
    }
    if (rand(m) as i16) < 0 {
        return fall(m);
    }
    m.write_u8(t + L::FRAME, 0);
    tumble_step(m, back)
}

/// Un rebote de la voltereta. Original: `HumanDeath_TumbleBack_04a438` /
/// `HumanDeath_TumbleFwd_04a4e0`.
pub fn tumble_step(m: &mut impl Machine, back: bool) -> Flow {
    let t = m.task_register();
    let table = if back { L::TUMBLE_BACK } else { L::TUMBLE_FWD };
    let k = u32::from(m.read_u8(t + L::FRAME));
    let s = m.read_u32(table + k * 4);
    if s == 0 {
        return if back { tumble_land(m) } else { collapse(m) };
    }
    anim::start(m, t, s);
    m.write_u8(t + L::FRAME, (k as u8).wrapping_add(1));
    set_bits(m, t + L::FLAGS_13, L::BIT_13_3, false);
    vel_back(m, t, if back { L::TUMBLE_VEL.wrapping_neg() } else { L::TUMBLE_VEL });
    add16(m, t + L::VEL_Y, L::TUMBLE_VEL);
    add16(m, t + L::Y, 2);
    set_handler(m, t, if back { L::TUMBLE_BACK_LOOP } else { L::TUMBLE_FWD_LOOP });
    tumble_loop(m, back)
}

/// Original: `HumanDeath_TumbleBack_04a438+$3E` / `TumbleFwd_04a4e0+$3E`.
pub fn tumble_loop(m: &mut impl Machine, back: bool) -> Flow {
    let t = m.task_register();
    physics_air(m, t);
    if anim::step(m, t) {
        let again = super::fort::hit_side(m, t).is_some();
        let h = match (again, back) {
            (true, true) => L::TUMBLE_BACK_STATE,
            (true, false) => L::TUMBLE_FWD_STATE,
            (false, true) => L::TUMBLE_BACK_LAND,
            (false, false) => L::COLLAPSE,
        };
        set_handler(m, t, h);
    }
    cull(m, t)
}

/// Original: `HumanDeath_TumbleBackLand_04a4a2`.
pub fn tumble_land(m: &mut impl Machine) -> Flow {
    start_and_loop(m, L::S_TUMBLE_LAND)
}

/// Para los humanos montados (que caen de su vehículo): al suelo y
/// voltereta hacia atrás. Original: `HumanDeath_PlayCryByKind_04a0d4+$72`.
pub fn fall_land(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    mark_grounded(m, t);
    tumble_land(m)
}

/// Como [`fall_land`] pero con el cuerpo nuevo y desplomándose.
/// Original: `HumanDeath_PlayCryByKind_04a0d4+$80`.
pub fn fall_slump(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    mark_grounded(m, t);
    body_gfx(m, t);
    slump(m)
}

/// Se desploma (una de dos). Original: `HumanDeath_Collapse_04a54a`.
pub fn collapse(m: &mut impl Machine) -> Flow {
    let k = usize::from((rand(m) as i16) < 0);
    start_and_loop(m, L::S_COLLAPSE[k])
}

/// Cae de espaldas o de frente. Original: `HumanDeath_Collapse_04a54a__L04a568`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::BEHIND) != 0 {
        anim::start(m, t, L::S_FALL_BACK);
        vel_back(m, t, L::FALL_VEL_X.wrapping_neg());
    } else {
        anim::start(m, t, L::S_FALL_FWD);
        vel_back(m, t, L::FALL_VEL_X);
    }
    m.write_u16(t + L::VEL_Y, L::FALL_VEL_Y);
    add16(m, t + L::Y, 2);
    anim_loop_start(m)
}

/// Original: `HumanDeath_Collapse_04a54a__L04a5a6`.
pub fn slump(m: &mut impl Machine) -> Flow {
    start_and_loop(m, L::S_SLUMP)
}

/// Original: `HumanDeath_Collapse_04a54a+$6A`.
pub fn crumple(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let k = usize::from(m.read_u8(t + L::BEHIND) != 0);
    start_and_loop(m, L::S_CRUMPLE[k])
}

/// Reventado por una explosión. Original: `HumanDeath_Collapse_04a54a+$84`.
pub fn blasted(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    mark_grounded(m, t);
    let g = blood_gfx(m, L::GFX_BLAST);
    set_gfx(m, t, g);
    start_and_loop(m, L::S_BLAST)
}

/// Original: `HumanDeath_Collapse_04a54a+$BA`.
pub fn shot(m: &mut impl Machine) -> Flow {
    start_and_loop(m, L::S_SHOT)
}

/// Voltereta por el suelo (la otra animación). Original:
/// `HumanDeath_Collapse_04a54a__L04a612`.
pub fn flip_b(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_FLIP_B);
    flip_go(m, t)
}

/// Original: `HumanDeath_Collapse_04a54a__L04a620`.
pub fn flip(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_FLIP);
    flip_go(m, t)
}

fn flip_go(m: &mut impl Machine, t: u32) -> Flow {
    if m.read_u8(t + L::BEHIND) == 0 {
        let f = m.read_u8(t + L::FACING);
        m.write_u8(t + L::FACING, f ^ 1);
    }
    vel_back(m, t, L::FLIP_VEL_X);
    m.write_u16(t + L::ACC_Y, L::FALL_ACC);
    m.write_u16(t + L::VEL_Y, 0);
    set_handler(m, t, L::FLIP_LOOP);
    flip_loop(m)
}

/// Original: `HumanDeath_Collapse_04a54a+$108`.
pub fn flip_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_ground(m, t);
    if anim::step(m, t) && m.read_u8(t + L::GROUNDED) != 0 {
        set_handler(m, t, L::KNOCKDOWN);
    }
    cull(m, t)
}

/// Original: `HumanDeath_Knockdown_04a672`.
pub fn knockdown(m: &mut impl Machine) -> Flow {
    start_and_loop(m, L::S_KNOCKDOWN)
}

/// Original: `HumanDeath_Knockdown_04a672+$0E`.
pub fn sit_or_flip(m: &mut impl Machine) -> Flow {
    if (rand(m) as i16) < 0 {
        return flip_b(m);
    }
    sit(m)
}

/// Se queda sentado y desaparece al acabar. Original: `HumanDeath_Knockdown_04a672+$1A`.
pub fn sit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_SIT);
    set_handler(m, t, L::SIT_LOOP);
    sit_loop(m)
}

/// Original: `HumanDeath_Knockdown_04a672+$2A`.
pub fn sit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_ground(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    cull(m, t)
}

/// Velocidad de salir despedido: hacia arriba con algo de azar y en X
/// alejándose del golpe. Original: `HumanDeath_InitBurst_04a6b2`.
pub fn init_burst(m: &mut impl Memory, t: u32) {
    vel_fwd(m, t, 0);
    m.write_u16(t + L::VEL_Y, L::BURST_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::BURST_ACC_Y);
    m.write_u16(t + L::ACC_X, 0);
    let r = rand(m) & L::BURST_RAND;
    let away = if (m.read_u16(t + L::X).wrapping_sub(m.read_u16(t + L::HIT_X)) as i16) < 0 { 0xFFFF } else { 0 };
    m.write_u16(t + L::VEL_X, r ^ away);
    let r = rand(m) & L::BURST_RAND;
    add16(m, t + L::VEL_Y, r);
    m.write_u8(t + L::GROUNDED, 0);
}

/// Al azar: vuela hacia la cámara, voltereta, al suelo o despedido.
/// Original: `HumanDeath_InitBurst_04a6b2+$50`.
pub fn burst_pick(m: &mut impl Machine) -> Flow {
    if rand(m) & 0x7F == 0 {
        return launch(m);
    }
    if (rand(m) as i16) < 0 {
        return tumble(m);
    }
    if rand(m) & 3 == 0 {
        return burst_ground(m);
    }
    burst(m)
}

/// Original: `HumanDeath_InitBurst_04a6b2__L04a72e`.
pub fn burst_high(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    add16(m, t + L::Y, 4);
    burst(m)
}

/// Sale despedido por el aire. Original: `HumanDeath_InitBurst_04a6b2+$80`.
pub fn burst(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_BURST);
    init_burst(m, t);
    m.write_u32(t + L::HITBOX, L::NIL);
    set_bits(m, t + L::FLAGS_13, L::BIT_13_3, false);
    set_handler(m, t, L::BURST_RISE);
    burst_rise(m)
}

/// Subiendo; al empezar a bajar recupera la caja. Original: `HumanDeath_InitBurst_04a6b2+$A2`.
pub fn burst_rise(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if (m.read_u16(t + L::VEL_Y) as i16) < 0 {
        m.write_u32(t + L::HITBOX, L::S_BURST_HITBOX);
        set_handler(m, t, L::BURST_FALL);
    }
    burst_fall(m)
}

/// Bajando: trozo al azar, si le vuelven a dar vuelve a morir, y al tocar
/// suelo cae. Original: `HumanDeath_InitBurst_04a6b2+$B8`.
pub fn burst_fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_ground(m, t);
    let r = rand(m);
    pick_gib(m, t, r);
    if super::fort::hit_side(m, t).is_some() {
        mark_grounded(m, t);
        let state = pick_state(m, t, 3);
        let f = run(m, state);
        if !matches!(f, Flow::Return) {
            return f;
        }
    }
    if m.read_u8(t + L::GROUNDED) != 0 {
        set_handler(m, t, L::BURST_LAND);
    }
    cull(m, t)
}

/// Trozo al azar y avanza la animación (devuelve si acabó, como el
/// acarreo del original). Original: `HumanDeath_PickGibPtr_04aba8`.
pub fn pick_gib(m: &mut impl Machine, t: u32, r: u16) -> bool {
    let g = m.read_u32(L::GIBS + u32::from(r & 7) * 4);
    m.write_u32(t + L::GIB, g);
    anim::step(m, t)
}

/// Original: `HumanDeath_BurstLand_04a798`.
pub fn burst_land(m: &mut impl Machine) -> Flow {
    start_and_loop(m, L::S_BURST_LAND)
}

/// Original: `HumanDeath_BurstLand_04a798+$0E` (y `HumanDeath_Launched_04a7f4+$98`).
pub fn burst_ground_probe(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    mark_grounded(m, t);
    burst_ground(m)
}

/// Despedido rasante por el suelo. Original: `HumanDeath_BurstLand_04a798__L04a7b0`.
pub fn burst_ground(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    add16(m, t + L::Y, 4);
    anim::start(m, t, L::S_BURST_GROUND);
    init_burst(m, t);
    set_gfx(m, t, L::GFX_BURST);
    set_handler(m, t, L::BURST_GROUND_LOOP);
    burst_ground_loop(m)
}

/// Original: `HumanDeath_BurstLand_04a798+$40`.
pub fn burst_ground_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_ground(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::GROUNDED) != 0 {
        set_handler(m, t, L::LAUNCHED);
    }
    cull(m, t)
}

/// Original: `HumanDeath_Launched_04a7f4`.
pub fn launched(m: &mut impl Machine) -> Flow {
    start_and_loop(m, L::S_LAUNCHED)
}

/// Sale volando hacia la cámara (cada vez más grande).
/// Original: `HumanDeath_Launched_04a7f4__L04a802`.
pub fn launch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    mark_grounded(m, t);
    set_gfx(m, t, L::GFX_LAUNCH);
    vel_fwd(m, t, L::LAUNCH_VEL.0);
    m.write_u16(t + L::VEL_Y, L::LAUNCH_VEL.1);
    m.write_u16(t + L::ACC_Y, L::LAUNCH_VEL.2);
    m.write_u16(t + L::ACC_X, 0);
    anim::start(m, t, L::S_LAUNCH);
    m.write_u16(t + L::COUNT, L::LAUNCH_ZOOM);
    m.write_u16(t + L::ATTR, 0xFFFF);
    set_bits(m, t + L::FLAGS_12, L::BIT_12_6, true);
    set_handler(m, t, L::LAUNCH_LOOP);
    launch_loop(m)
}

/// Original: `HumanDeath_Launched_04a7f4+$6A`.
pub fn launch_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let z = m.read_u16(t + L::COUNT);
    let z = z.checked_add(z >> 5).unwrap_or(0xFFFF);
    m.write_u16(t + L::COUNT, z);
    for f in L::ZOOM {
        m.write_u8(t + f, (z >> 8) as u8);
    }
    let at = at(m);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    anim::step(m, t);
    cull(m, t)
}

/// Despedido hacia atrás con sangre. Original: `HumanDeath_Launched_04a7f4+$A6`.
pub fn blown_back(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_FALL_BACK);
    let a = m.read_u32(t + L::ATTACKER);
    let right = m.read_u16(a + L::COUNT) != 0;
    set_bits(m, t + L::FACING, 1, right);
    vel_back(m, t, L::BACK_VEL.0);
    m.write_u16(t + L::VEL_Y, L::BACK_VEL.1);
    add16(m, t + L::Y, 2);
    spawn_blood(m, t, 0);
    set_handler(m, t, L::BACK_LOOP);
    blown_back_loop(m)
}

/// Original: `HumanDeath_Launched_04a7f4+$EA`.
pub fn blown_back_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_air(m, t);
    physics_fall(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::FADE);
    }
    cull(m, t)
}

/// Saltado hacia arriba con sangre. Original: `HumanDeath_Launched_04a7f4+$106`.
pub fn blown_up(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_X, 0);
    anim::start(m, t, L::S_BURST);
    m.write_u16(t + L::VEL_Y, L::UP_VEL_Y);
    add16(m, t + L::Y, 2);
    m.write_u16(t + L::VEL_X, 0);
    spawn_blood(m, t, 0);
    set_handler(m, t, L::UP_LOOP);
    blown_up_loop(m)
}

/// Original: `HumanDeath_Launched_04a7f4+$12C`.
pub fn blown_up_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_air(m, t);
    m.write_u16(t + L::ACC_Y, 0);
    physics_fall(m, t);
    let r = rand(m);
    pick_gib(m, t, r);
    if m.read_u8(t + L::GROUNDED) != 0 {
        set_handler(m, t, L::BURST_LAND);
    }
    cull(m, t)
}

/// Quemado: a veces echa a correr ardiendo. Original: `HumanDeath_Launched_04a7f4+$154`.
pub fn on_fire(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::set_attr(m, t, L::FIRE_ATTR);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a & L::LAYER_KEEP);
    if rand(m) & 3 == 0 {
        return burn_run(m);
    }
    anim::start(m, t, L::S_ON_FIRE);
    m.write_u16(t + L::TIMER, L::FIRE_FRAMES);
    set_handler(m, t, L::FIRE_LOOP);
    on_fire_loop(m)
}

/// Original: `HumanDeath_Launched_04a7f4+$18E`.
pub fn on_fire_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_air(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::TIMER);
    if n != 0 {
        m.write_u16(t + L::TIMER, n - 1);
    } else if m.read_u8(t + L::GROUNDED) != 0 {
        set_handler(m, t, L::BURNING);
    }
    cull(m, t)
}

/// Original: `HumanDeath_Burning_04a9b0`.
pub fn burning(m: &mut impl Machine) -> Flow {
    start_and_loop(m, L::S_BURNING)
}

/// Corre ardiendo, cambiando de sentido. Original: `HumanDeath_Burning_04a9b0__L04a9be`.
pub fn burn_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_BURN_RUN);
    sound::play(m, L::BURN_MUSIC);
    m.write_u8(t + L::DEAD, 0);
    tasks::spawn(m, L::FLAME, t);
    vel_back(m, t, L::BURN_VEL);
    m.write_u16(t + L::TIMER, L::BURN_FRAMES);
    let turns = if (rand(m) as i16) < 0 { L::BURN_TURNS.0 } else { L::BURN_TURNS.1 };
    m.write_u8(t + L::FRAME, turns);
    m.write_u8(t + L::FRAME_LEFT, turns);
    m.write_u16(t + L::ACC_Y, L::BURN_ACC_Y);
    set_handler(m, t, L::BURN_LOOP);
    burn_loop(m)
}

/// Original: `HumanDeath_Burning_04a9b0+$6E`.
pub fn burn_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_ground(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::FLAGS_5A) & L::BIT_5A_5 != 0 {
        set_handler(m, t, L::BURNED_DOWN);
    }
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        set_handler(m, t, L::BURNED_DOWN);
    }
    let k = m.read_u8(t + L::FRAME_LEFT).wrapping_sub(1);
    m.write_u8(t + L::FRAME_LEFT, k);
    if k == 0 {
        let f = m.read_u8(t + L::FACING);
        m.write_u8(t + L::FACING, f ^ 1);
        let turns = m.read_u8(t + L::FRAME);
        m.write_u8(t + L::FRAME_LEFT, turns);
        let v = m.read_u16(t + L::VEL_X);
        m.write_u16(t + L::VEL_X, v.wrapping_neg());
    }
    cull(m, t)
}

/// Original: `HumanDeath_BurnedDown_04aa62`.
pub fn burned_down(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_LAUNCHED);
    m.write_u8(t + L::DEAD, 0xFF);
    anim_loop_start(m)
}

/// La llama que sigue al que arde. Original: `HumanDeath_FlameChild_04aa76`.
pub fn flame(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX_FLAME);
    anim::start(m, t, L::S_FLAME);
    set_handler(m, t, L::FLAME_LOOP);
    flame_loop(m)
}

/// Original: `HumanDeath_FlameChild_04aa76+$1A`.
pub fn flame_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = m.read_u32(t + T::OWNER);
    if m.read_u32(o + T::HANDLER) == L::TASK_FINISHING || m.read_u8(o + L::DEAD) != 0 {
        return tasks::finish(m);
    }
    let x = m.read_u16(o + L::X);
    m.write_u16(t + L::X, x);
    let y = m.read_u16(o + L::Y).wrapping_add(L::FLAME_DY);
    m.write_u16(t + L::Y, y);
    let a = m.read_u16(o + L::ATTR);
    m.write_u16(t + L::ATTR, a);
    anim::step(m, t);
    Flow::Return
}

/// Salpicadura de sangre `k` (0 = A, 1 = B) a un lado.
/// Original: `HumanDeath_SpawnBloodSplash_04aad2` (y `__L04aae0`).
pub fn spawn_blood(m: &mut impl Memory, t: u32, k: usize) -> u32 {
    let c = tasks::spawn(m, L::BLOOD_SPLASH[k], t);
    let y = m.read_u16(t + L::Y);
    m.write_u16(c + L::Y, y);
    let mut d = L::BLOOD_DX;
    if m.read_u8(t + L::FACING) & 1 == 0 {
        let f = m.read_u8(c + L::FACING);
        m.write_u8(c + L::FACING, f | 1);
        d = d.wrapping_neg();
    }
    let x = m.read_u16(t + L::X).wrapping_add(d);
    m.write_u16(c + L::X, x);
    c
}

/// Original: `HumanDeath_BloodSplashA_04ab10` / `B_04ab52`.
pub fn blood(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let g = blood_gfx(m, L::GFX_BLOOD);
    anim::alloc_entry(m, t, g);
    anim::set_attr(m, t, L::BODY_ATTR);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::LAYER_KEEP) | L::BLOOD_LAYER);
    anim::start(m, t, L::S_BLOOD[k]);
    set_handler(m, t, L::BLOOD_LOOP);
    blood_loop(m)
}

/// Original: `HumanDeath_BloodSplash_Loop_04ab90`.
pub fn blood_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
