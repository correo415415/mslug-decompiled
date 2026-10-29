//! Barril flotante del río de la misión 1 y su mina acuática. El barril
//! espera mirando al jugador; cuando lo tiene a tiro da unas vueltas, se
//! hunde (soltando una salpicadura) y vuelve a salir, deriva hacia el
//! jugador y, si toca agua, salpica y desaparece. Si le dan revienta:
//! suelta una astilla, sale despedido con la velocidad del que le ha dado y
//! se queda cabeceando en el agua. Su guion suelta minas que avanzan con
//! la ola, se arman al estabilizarse y estallan al tocar algo.
//!
//! Original: `Barrel_Tmpl8D_065f40` / `Barrel_Idle_065fba`
//! (`asm/enemies/soldiers/sniper_camper_mortar_062008.s`) y
//! `FloatBarrel_*` / `FloatMine_*` (`$06600E`–`$0666AE`,
//! `asm/enemies/soldiers/barrel_paratrooper_shield_tank_066000.s`).

use super::{anim, collision, fort, ground_fx, hits, items, manual_icons, motion, pickup, platforms};
use super::{slug, soldier, sound, tasks};
use crate::layout::{barrel as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Columna y fila que deja el original en `d3`/`d4`.
fn at(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

/// Valor absoluto de una palabra (`$8000` se queda igual).
/// Original: `Math_AbsW_08b58e`.
pub fn abs16(v: u16) -> u16 {
    if v & 0x8000 != 0 {
        v.wrapping_neg()
    } else {
        v
    }
}

/// Jugador más cercano en X (el 2 si no hay ninguno, como deja `a0`).
fn nearest(m: &mut impl Memory, t: u32) -> u32 {
    pickup::nearest_player_x(m, t).unwrap_or(crate::layout::PLAYER_ENTITY[1])
}

/// Mira hacia el jugador más cercano. Original: `FloatBarrel_FaceTarget_066622`.
pub fn face_target(m: &mut impl Memory, t: u32) {
    let p = nearest(m, t);
    let d = m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X));
    m.write_u8(t + L::FACING, u8::from((d as i16) < 0));
}

/// ¿Está el jugador más cercano a menos de `RANGE` × 16 px en X? Devuelve
/// el jugador si sí. Original: `FloatBarrel_TargetInRange_066644` (C).
pub fn target_in_range(m: &mut impl Memory, t: u32) -> Option<u32> {
    let p = nearest(m, t);
    let d = abs16(m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X)));
    let range = u16::from(m.read_u8(t + L::RANGE)).wrapping_mul(L::RANGE_STEP);
    ((range as i16) >= (d as i16)).then_some(p)
}

/// Hija en la posición del sprite de `t`.
fn child(m: &mut impl Memory, t: u32, handler: u32) -> u32 {
    let c = tasks::spawn(m, handler, t);
    anim::copy_sprite_pos(m, t, c);
    c
}

/// Salpicadura detrás, un poco más abajo.
fn splash_fx(m: &mut impl Memory, t: u32) {
    let c = child(m, t, L::SMOKE);
    let y = m.read_u16(c + L::Y).wrapping_add(L::SMOKE_DY);
    m.write_u16(c + L::Y, y);
    let f = m.read_u8(c + L::FLAGS_12);
    m.write_u8(c + L::FLAGS_12, f | L::BEHIND);
}

/// Se pierde si sale de la pantalla.
fn cull(m: &mut impl Machine, t: u32) -> Flow {
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Si le han dado revienta (con astilla si `shard`).
fn check_hit(m: &mut impl Memory, t: u32, shard: bool) {
    if fort::hit_side(m, t).is_some() {
        if shard {
            child(m, t, L::SHARD);
        }
        set_handler(m, t, L::POP);
    }
}

/// ¿Pisa agua? (con [`ground_fx::solid_at_feet`]).
fn on_water(m: &mut impl Machine, t: u32) -> bool {
    let at = at(m);
    let (on, raw) = ground_fx::solid_at_feet(m, t, at);
    on && raw == L::WATER
}

/// ¿Flota sobre agua? (con [`collision::probe_floor`]).
fn floats(m: &mut impl Machine, t: u32) -> bool {
    let at = at(m);
    let f = collision::probe_floor(m, t, at);
    !f.in_air && f.tile.raw == L::WATER
}

fn move_probe(m: &mut impl Machine, t: u32) {
    let at = at(m);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
}

fn move_e(m: &mut impl Machine, t: u32) {
    let at = at(m);
    motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at));
}

/// Alta: quieto en la superficie, con 1 de vida y el guion de espera.
/// Original: `Barrel_Tmpl8D_065f40`.
pub fn barrel(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f | L::BEHIND);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    manual_icons::stop_motion(m, t);
    move_probe(m, t);
    m.write_u16(t + L::HP, 1);
    anim::start(m, t, L::SCRIPT_WAIT);
    set_handler(m, t, L::LOOP);
    barrel_loop(m)
}

/// Espera con la escena hasta tener al jugador a tiro.
/// Original: `Barrel_Tmpl8D_065f40+$3A`.
pub fn barrel_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    face_target(m, t);
    if let Some(p) = target_in_range(m, t) {
        m.write_u32(t + L::TARGET, p);
        set_handler(m, t, L::IDLE);
    }
    cull(m, t)
}

/// Da `TURNS` vueltas de su guion antes de hundirse.
/// Original: `Barrel_Idle_065fba`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::SCRIPT_IDLE);
    let n = m.read_u8(t + L::TURNS);
    m.write_u8(t + L::COUNT, n);
    set_handler(m, t, L::IDLE_LOOP);
    idle_loop(m)
}

/// Original: `Barrel_Idle_065fba+$18`.
pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        let n = m.read_u8(t + L::COUNT).wrapping_sub(1);
        m.write_u8(t + L::COUNT, n);
        if n == 0 {
            set_handler(m, t, L::SUBMERGE);
        }
    }
    cull(m, t)
}

/// Se hunde (más despacio cuanto mayor `SINK & 7`) dejando una
/// salpicadura. Original: `FloatBarrel_Submerge_06600e`.
pub fn submerge(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, L::SUBMERGE_VEL_Y);
    m.write_u16(t + L::ACC_X, 0);
    let n = u16::from(m.read_u8(t + L::SINK) & 7);
    m.write_u16(t + L::ACC_Y, L::SUBMERGE_ACC_Y.wrapping_add(n * L::SINK_STEP));
    anim::start(m, t, L::SCRIPT_SUBMERGE);
    splash_fx(m, t);
    set_handler(m, t, L::SUBMERGE_LOOP);
    submerge_loop(m)
}

/// Hasta que empieza a subir. Original: `FloatBarrel_Submerge_06600e+$60`.
pub fn submerge_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    move_probe(m, t);
    anim::step(m, t);
    if m.read_u16(t + L::VEL_Y) & 0x8000 != 0 {
        set_handler(m, t, L::SURFACE);
    }
    check_hit(m, t, true);
    cull(m, t)
}

/// Sale a flote frenando la subida, mirando al jugador.
/// Original: `FloatBarrel_Surface_0660cc`.
pub fn surface(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let a = m.read_u16(t + L::ACC_Y) as i16 >> 1;
    m.write_u16(t + L::ACC_Y, a as u16);
    face_target(m, t);
    anim::start(m, t, L::SCRIPT_SURFACE);
    set_handler(m, t, L::SURFACE_LOOP);
    surface_loop(m)
}

/// Con el jugador cerca (pero no encima) se pone a derivar; en el agua
/// salpica. Original: `FloatBarrel_Surface_0660cc+$20`.
pub fn surface_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    move_probe(m, t);
    anim::step(m, t);
    if pickup::player_in_box(m, t, L::NEAR_BOX).is_some() && pickup::player_in_box(m, t, L::CLOSE_BOX).is_none() {
        set_handler(m, t, L::DRIFT);
    }
    if on_water(m, t) {
        set_handler(m, t, L::SPLASH);
    }
    check_hit(m, t, true);
    cull(m, t)
}

/// Deriva. Original: `FloatBarrel_Drift_066174`.
pub fn drift(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::SCRIPT_DRIFT);
    set_handler(m, t, L::DRIFT_LOOP);
    drift_loop(m)
}

/// Original: `FloatBarrel_Drift_066174+$12`.
pub fn drift_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    move_probe(m, t);
    anim::step(m, t);
    if floats(m, t) {
        set_handler(m, t, L::SPLASH);
    }
    check_hit(m, t, false);
    cull(m, t)
}

/// Se mete en el agua: salpicadura (si se ve) y baja.
/// Original: `FloatBarrel_Splash_0661d8`.
pub fn splash(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        sound::play(m, L::SND_SPLASH);
        splash_fx(m, t);
    }
    set_handler(m, t, L::SPLASH_LOOP);
    splash_loop(m)
}

/// Original: `FloatBarrel_Splash_0661d8+$42`.
pub fn splash_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::step(m, t);
    tasks::finish(m)
}

/// Revienta: sale despedido con 1/4 de la velocidad del que le ha dado y
/// pasa a cabecear. Original: `FloatBarrel_Pop_066228`.
pub fn pop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_POP);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f & !L::BEHIND);
    manual_icons::stop_motion(m, t);
    let a = m.read_u32(t + L::ATTACKER);
    let v = m.read_u16(a.wrapping_add(L::VEL_X)) as i16 >> 2;
    m.write_u16(t + L::VEL_X, v as u16);
    anim::start(m, t, L::SCRIPT_POP);
    bob(m)
}

/// Sube flotando hasta pisar. Original: `FloatBarrel_Bob_066258`.
pub fn bob(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_Y, 0);
    m.write_u16(t + L::ACC_Y, L::BOB_ACC_Y);
    set_handler(m, t, L::BOB_LOOP);
    bob_loop(m)
}

/// Original: `FloatBarrel_Bob_066258+$12`.
pub fn bob_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    move_e(m, t);
    if !collision::probe_floor_here(m) {
        set_handler(m, t, L::SETTLE);
    }
    anim::step(m, t);
    cull(m, t)
}

/// Se asienta frenando en X; en agua salpica y se va.
/// Original: `FloatBarrel_BobSettle_0662a4`.
pub fn settle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if floats(m, t) {
        sound::play(m, L::SND_SPLASH);
        splash_fx(m, t);
        return tasks::finish(m);
    }
    let a = m.read_u16(t + L::VEL_X).wrapping_neg() as i16 >> 5;
    m.write_u16(t + L::ACC_X, a as u16);
    m.write_u16(t + L::VEL_Y, 0);
    m.write_u16(t + L::ACC_Y, 0);
    anim::start(m, t, L::SCRIPT_SETTLE);
    set_handler(m, t, L::SETTLE_LOOP);
    settle_loop(m)
}

/// Frena hasta parar (sin cambiar de sentido); si deja de pisar vuelve a
/// cabecear. Original: `FloatBarrel_BobSettle_0662a4+$68`.
pub fn settle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let before = m.read_u16(t + L::VEL_X);
    move_e(m, t);
    if (before ^ m.read_u16(t + L::VEL_X)) & 0x8000 != 0 {
        m.write_u16(t + L::VEL_X, 0);
        m.write_u16(t + L::ACC_X, 0);
    }
    if collision::probe_floor_here(m) {
        set_handler(m, t, L::BOB);
    }
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    cull(m, t)
}

/// Llamada del guion: suelta una mina. Original: `FloatBarrel_SpawnMine_066680`.
pub fn spawn_mine(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    child(m, t, L::MINE);
    Flow::Return
}

/// ¿Le quedan hijas? (la tarea siguiente tiene más prioridad).
/// Original: `FloatBarrel_HitCheck_066692` (C).
pub fn has_children(m: &mut impl Memory, t: u32) -> bool {
    let next = m.read_u32(t + T::NEXT);
    m.read_u8(t + T::PRIORITY) < m.read_u8(next + T::PRIORITY)
}

/// En los frames pares intercambia las dos palabras del sprite (parpadeo).
/// Original: `FloatBarrel_FlipByFrame_0665c6`.
pub fn flip_by_frame(m: &mut impl Memory, t: u32) {
    if m.read_u8(L::FRAME_COUNTER) & 1 != 0 {
        return;
    }
    let a = m.read_u16(t + L::SPRITE_A);
    let b = m.read_u16(t + L::SPRITE_B);
    m.write_u16(t + L::SPRITE_A, b);
    m.write_u16(t + L::SPRITE_B, a);
    m.write_u16(t + L::SPRITE_C, a);
}

/// Sigue la ola: la aceleración tiende a la altura de reposo. Devuelve si
/// ya está casi estable (y se puede armar). Original: `FloatBarrel_WaveBob_0665e8` (C).
pub fn wave_bob(m: &mut impl Machine, t: u32) -> bool {
    let w = m.read_u16(t + L::WAVE);
    let a = w.wrapping_sub(m.read_u16(t + L::VEL_Y)) as i16 >> 3;
    m.write_u16(t + L::ACC_Y, a as u16);
    move_probe(m, t);
    let w = m.read_u16(t + L::WAVE).wrapping_sub(m.read_u16(t + L::VEL_Y));
    m.write_u16(t + L::WAVE, w);
    abs16(w) as i16 <= L::WAVE_ARMED
}

/// Alta de la mina: velocidad por dificultad hacia donde mira el barril,
/// ataques, caja de golpe y vida. Original: `FloatMine_Init_06637a`.
pub fn mine(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    anim::alloc_entry(m, t, L::MINE_GFX);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    manual_icons::stop_motion(m, t);
    let mut v = soldier::by_difficulty(m, t, L::MINE_SPEED);
    let mut dx = L::MINE_DX;
    if m.read_u8(t + L::FACING) & 1 == 0 {
        v = v.wrapping_neg();
        dx = dx.wrapping_neg();
    }
    m.write_u16(t + L::VEL_X, v);
    let x = m.read_u16(t + L::X).wrapping_add(dx);
    m.write_u16(t + L::X, x);
    let y = m.read_u16(t + L::Y).wrapping_add(L::MINE_DY);
    m.write_u16(t + L::Y, y);
    anim::start(m, t, L::SCRIPT_MINE);
    m.write_u32(t + L::ATTACKS, L::MINE_ATTACKS);
    slug::clear_flags13(m, t);
    m.write_u32(t + L::HITBOX, L::MINE_HITBOX);
    m.write_u16(t + L::HP, L::MINE_HP);
    m.write_u16(t + L::WAVE, L::WAVE_INIT);
    set_handler(m, t, L::MINE_LOOP);
    mine_loop(m)
}

/// Avanza con la ola hasta estabilizarse. Original: `FloatMine_Init_06637a+$86`.
pub fn mine_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if wave_bob(m, t) {
        set_handler(m, t, L::ARMED);
    }
    mine_tail(m, t)
}

/// Armada. Original: `FloatMine_Armed_066490`.
pub fn armed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::SCRIPT_ARMED);
    set_handler(m, t, L::ARMED_LOOP);
    armed_loop(m)
}

/// Original: `FloatMine_Armed_066490+$12`.
pub fn armed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    wave_bob(m, t);
    mine_tail(m, t)
}

/// Parpadeo, guion, golpes y estallido: al morir, al tocar suelo firme, al
/// acertar o al salir de la pantalla.
fn mine_tail(m: &mut impl Machine, t: u32) -> Flow {
    flip_by_frame(m, t);
    anim::step(m, t);
    if fort::hit_side(m, t).is_some() {
        let f = m.read_u8(t + L::FLAGS_13);
        m.write_u8(t + L::FLAGS_13, f & !L::HIT_FLASH);
        sound::play(m, L::SND_HIT);
    }
    let blast = fort::dead(m, t) || {
        let at = at(m);
        ground_fx::solid_at_feet(m, t, at).0
    } || {
        hits::run_attack_list(m, t);
        m.read_u8(t + L::FLAGS_13) & L::ATTACK_HIT != 0
    } || items::out_of_view_seen(m, t, Some(L::MARGINS_SMALL));
    if !blast {
        return Flow::Return;
    }
    sound::play(m, L::SND_BLAST);
    child(m, t, L::EXPLOSION);
    tasks::finish(m)
}

/// Astilla del barril: sale hacia arriba con la mitad de la velocidad del
/// que le dio al barril (o `$400`). Original: `FloatBarrel_Shard_066528`.
pub fn shard(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    anim::start(m, t, L::SCRIPT_SHARD);
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::VEL_Y, L::SHARD_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::SHARD_ACC_Y);
    let o = m.read_u32(t + L::OWNER);
    let a = m.read_u32(o + L::ATTACKER);
    let mut v = m.read_u16(a.wrapping_add(L::VEL_X));
    if v == 0 {
        v = L::SHARD_VEL_X;
    }
    m.write_u16(t + L::VEL_X, (v as i16 >> 1) as u16);
    set_handler(m, t, L::SHARD_LOOP);
    shard_loop(m)
}

/// Vuela hasta tocar suelo (en el agua salpica) o salir de la pantalla.
/// Original: `FloatBarrel_Shard_066528+$4C`.
pub fn shard_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    move_probe(m, t);
    let at = at(m);
    let (on, raw) = ground_fx::solid_at_feet(m, t, at);
    if on {
        if raw == L::WATER {
            child(m, t, L::SMOKE);
        }
        return tasks::finish(m);
    }
    anim::step(m, t);
    if items::out_of_view_seen(m, t, Some(L::MARGINS_SMALL)) {
        return tasks::finish(m);
    }
    Flow::Return
}
