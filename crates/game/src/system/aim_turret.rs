//! Torreta que apunta (plantilla `$8E`, misión 1): un cuerpo que vuela
//! hacia una altura de vigilancia sobre el jugador más cercano en X y,
//! cuando llega, orbita arriba y abajo; dispara ráfagas de proyectiles
//! (con su casquillo) según la dificultad. Lleva un cañón como tarea hija.
//! Si la dan por muerta cae echando humo y se estrella (o, con
//! `PARAM_GATE` = $FF, sale volando hacia la derecha); si el scroll avanza
//! demasiado o pasa su tiempo, se retira hacia abajo.
//!
//! Original: `AimTurret_*` (`$060E62`–`$0619BE`,
//! `asm/props/late_props_turrets_05e000.s`).

use super::{anim, banner, fort, hits, items, manual_icons, motion, parachute, pickup, platforms, slug, soldier, sound, spawner, sprite, tasks, trig};
use crate::layout::{aim_turret as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn add16(m: &mut impl Memory, a: u32, v: u16) {
    let x = m.read_u16(a);
    m.write_u16(a, x.wrapping_add(v));
}

fn sub16(m: &mut impl Memory, a: u32, v: u16) {
    let x = m.read_u16(a);
    m.write_u16(a, x.wrapping_sub(v));
}

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

fn facing_left(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FACING) & L::FACING_LEFT != 0
}

/// Paso de movimiento con colisión, sin mirar el resultado (`$027CEE`).
fn probe_move(m: &mut impl Memory, t: u32, at: (u16, u16)) {
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
}

/// `d3`/`d4` con los que entra el original (para [`probe_move`]).
fn entry_at(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

/// El `d4` que deja `Atan2_Angle256_05e018`: $FF si el primer cateto (en
/// bytes) menos el segundo da negativo, si no 0.
fn atan2_d4(dx: u16, dy: u16) -> u16 {
    let x = (dx as i16) >> 1;
    let y = (dy as i16) >> 1;
    let a = (x as u8) ^ if x < 0 { 0xFF } else { 0 };
    let b = (y as u8) ^ if y < 0 { 0xFF } else { 0 };
    if a.wrapping_sub(b) & 0x80 != 0 { 0xFF } else { 0 }
}

fn target(m: &mut impl Memory, t: u32) -> ((u16, u16), (u16, u16)) {
    let pos = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    let tgt = (m.read_u16(t + L::TARGET_X), m.read_u16(t + L::TARGET_Y));
    (pos, tgt)
}

/// Altura de vigilancia (`$100` + `$10` × `PARAM_HEIGHT`) y X del jugador
/// en juego más cercano. Original: `AimTurret_Acquire_06179e`.
pub fn acquire(m: &mut impl Memory, t: u32) {
    let h = u16::from(m.read_u8(t + L::PARAM_HEIGHT) & 0xF);
    m.write_u16(t + L::TARGET_Y, L::HEIGHT_BASE + L::HEIGHT_STEP * h);
    if let Some(p) = pickup::nearest_player_x(m, t) {
        let x = m.read_u16(p + L::X);
        m.write_u16(t + L::TARGET_X, x);
    }
}

/// Acelera hacia el objetivo (ángulo con su desviación `SPREAD`, fuerza
/// según la distancia) y se mueve. `low`: la variante de caída (más
/// fuerza en X y siempre hacia abajo). Original:
/// `AimTurret_AimAtPlayer_06164c` / `AimTurret_AimAtPlayerB_0616ca`.
pub fn aim(m: &mut impl Memory, t: u32, low: bool) {
    let ((x, y), (tx, ty)) = target(m, t);
    let dx = ((tx.wrapping_sub(x) as i16) >> 1) as u16;
    let dy = ((ty.wrapping_sub(y) as i16) >> 1) as u16;
    let a = banner::atan2(m, dx, dy);
    let d4 = atan2_d4(dx, dy);
    let mut spread = u16::from(m.read_u8(t + L::SPREAD)) & L::SPREAD_MASK;
    if facing_left(m, t) {
        spread = spread.wrapping_neg();
    }
    let angle = u16::from(a).wrapping_add(spread) & 0xFF;
    m.write_u16(t + L::ANGLE, angle);
    let shift = if low { 4 } else { 3 };
    let r = banner::distance((x, y), (tx, ty)).wrapping_shl(shift);
    let (px, py) = trig::polar(m, angle, r as i16);
    let vx = m.read_u16(t + L::VEL_X);
    let vy = m.read_u16(t + L::VEL_Y);
    let ax = ((px as u16).wrapping_sub(vx) as i16) >> shift;
    let ay = ((py as u16).wrapping_sub(vy) as i16) >> 4;
    m.write_u16(t + L::ACC_X, ax as u16);
    m.write_u16(t + L::ACC_Y, if low { L::FALL_ACC_Y } else { ay as u16 });
    probe_move(m, t, (ty, d4));
}

/// Órbita: el ángulo gira y la velocidad vertical sigue su seno (radio
/// `$40`). Original: `AimTurret_SinCos40_06161e`.
pub fn orbit_step(m: &mut impl Memory, t: u32, at: (u16, u16)) {
    let a = m.read_u16(t + L::ANGLE).wrapping_sub(L::ORBIT_STEP) & 0xFF;
    m.write_u16(t + L::ANGLE, a);
    let (_, vy) = trig::polar(m, a, L::ORBIT_RADIUS);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, vy as u16);
    probe_move(m, t, at);
}

/// ¿Está a `limit` o menos del objetivo? Original:
/// `AimTurret_StepAngle_06174a` (8) / `AimTurret_StepAngleB_061774` ($10).
pub fn near(m: &mut impl Memory, t: u32, limit: i16) -> bool {
    let (pos, tgt) = target(m, t);
    banner::distance(pos, tgt) as i16 <= limit
}

/// Pose (0–6) según la velocidad horizontal y el espejo.
/// Original: `AimTurret_Anim_0617d2`.
pub fn update_pose(m: &mut impl Memory, t: u32) {
    let v = m.read_u16(t + L::VEL_X) as i16;
    let k = L::POSE_THRESHOLDS.iter().filter(|&&th| v >= th).count() as u8;
    let pose = if facing_left(m, t) { L::LAST_POSE - k } else { k };
    m.write_u8(t + L::AIM, pose);
}

fn pose(m: &mut impl Memory, t: u32) -> u32 {
    let p = m.read_u8(t + L::AIM);
    u32::from(if p <= L::LAST_POSE { p } else { 0 })
}

/// Guion del cuerpo para su pose (si tiene). Original:
/// `AimTurret_Cooldown_0614b6`.
pub fn body_script(m: &mut impl Memory, t: u32) {
    let p = pose(m, t);
    let s = m.read_u32(L::POSE_SCRIPTS + p * 4);
    if s != 0xFFFF_FFFF {
        anim::start(m, t, s);
    }
}

/// Alterna las dos paletas cada dos frames y, recién golpeada (este frame
/// y no el anterior), pone la del destello. Original:
/// `AimTurret_HitCheck_061580`.
pub fn flash(m: &mut impl Memory, t: u32) {
    let k = m.read_u8(t + L::FLASH_TICK).wrapping_add(1);
    m.write_u8(t + L::FLASH_TICK, k);
    if k == 2 {
        let b = m.read_u16(t + L::PAL_B);
        let a = m.read_u16(t + L::PAL_A);
        m.write_u16(t + L::PAL_B, a);
        m.write_u16(t + L::PAL_A, b);
        m.write_u16(t + L::PAL, b);
        m.write_u8(t + L::FLASH_TICK, 0);
    }
    let h = m.read_u8(t + L::HIT_HISTORY) << 1;
    m.write_u8(t + L::HIT_HISTORY, h);
    if fort::hit_side(m, t).is_some() {
        m.write_u8(t + L::HIT_HISTORY, h | 1);
        let f = m.read_u8(t + L::FLAGS2);
        m.write_u8(t + L::FLAGS2, f & !L::HIT);
        sound::play(m, L::SND_HIT);
    }
    let h = m.read_u8(t + L::HIT_HISTORY);
    let src = if h & 1 != 0 && h & 2 == 0 { L::PAL_FLASH } else { L::PAL_A };
    let p = m.read_u16(t + src);
    m.write_u16(t + L::PAL, p);
}

/// Paleta del cañón: la B si su cuerpo acaba de recibir un golpe.
/// Original: `AimTurret_GunOffset_0615f6`.
pub fn gun_palette(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let h = m.read_u8(o + L::HIT_HISTORY);
    let src = if h & 1 != 0 && h & 2 == 0 { L::PAL_B } else { L::PAL_A };
    let p = m.read_u16(t + src);
    m.write_u16(t + L::PAL, p);
}

/// Guion propio del cañón (tabla por pose del cuerpo: gráfico y espera
/// por fotograma, −1 = vuelta al principio) y su dibujo.
/// Original: `AimTurret_AnimScript_0614e6`.
pub fn gun_draw(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let p = pose(m, o);
    let script = m.read_u32(L::GUN_SCRIPTS + p * 4);
    m.write_u32(t + L::SCRIPT, script);
    let w = m.read_u8(t + L::WAIT).wrapping_sub(1);
    m.write_u8(t + L::WAIT, w);
    if w as i8 <= 0 {
        let f = m.read_u8(t + L::FRAME).wrapping_add(1);
        m.write_u8(t + L::FRAME, f);
        if m.read_u32(script + u32::from(f) * L::FRAME_LEN) == L::FRAME_END {
            m.write_u8(t + L::FRAME, 0);
        }
        let f = u32::from(m.read_u8(t + L::FRAME));
        let wait = m.read_u8(script + f * L::FRAME_LEN + L::FRAME_WAIT);
        m.write_u8(t + L::WAIT, wait);
    }
    let f = u32::from(m.read_u8(t + L::FRAME));
    let gfx = m.read_u32(script + f * L::FRAME_LEN);
    let req = sprite::Request {
        pos: m.read_u32(t + L::X),
        key: m.read_u16(t + L::ATTR),
        palette: m.read_u16(t + L::PAL),
        shrink_v: m.read_u8(t + L::SHRINK_V),
        shrink_h: m.read_u8(t + L::SHRINK_H),
        flip: m.read_u8(t + L::FACING) & 3,
        gfx,
    };
    sprite::dispatch(m, t, req, false);
}

/// Control de tiro: espera entre ráfagas; al empezar una, toma la espera
/// entre disparos y los disparos de la dificultad; dispara un par
/// (proyectil y casquillo) cada vez que vence la espera.
/// Original: `AimTurret_RngDifficulty_0618b0` (con `AimTurret_Rts_061946`
/// y `AimTurret_RtsB_06194c`).
pub fn fire_control(m: &mut impl Memory, t: u32) {
    let bw = m.read_u16(t + L::BURST_WAIT);
    if bw as i16 > 0 {
        m.write_u16(t + L::BURST_WAIT, bw - 1);
        return;
    }
    if m.read_u8(t + L::SHOTS) == L::SHOTS_NEW {
        let w = soldier::by_difficulty(m, t, L::T_SHOT_WAIT);
        m.write_u8(t + L::SHOT_WAIT, w as u8);
        let n = soldier::by_difficulty(m, t, L::T_SHOTS);
        m.write_u8(t + L::SHOTS, n as u8);
    }
    let sw = m.read_u8(t + L::SHOT_WAIT);
    if sw as i8 > 0 {
        m.write_u8(t + L::SHOT_WAIT, sw.wrapping_sub(1));
        return;
    }
    if m.read_u8(t + L::SHOTS) != 0 {
        spawn_pair(m, t);
        return;
    }
    m.write_u8(t + L::SHOTS, L::SHOTS_NEW);
    let w = soldier::by_difficulty(m, t, L::T_BURST_WAIT);
    m.write_u16(t + L::BURST_WAIT, w);
}

/// Un disparo: proyectil y casquillo, y la espera hasta el siguiente.
/// Original: `AimTurret_SpawnPair_061910`.
pub fn spawn_pair(m: &mut impl Memory, t: u32) {
    for h in [L::SHELL, L::BARREL] {
        let c = tasks::spawn(m, h, t);
        anim::copy_sprite_pos(m, t, c);
    }
    let w = soldier::by_difficulty(m, t, L::T_SHOT_WAIT);
    m.write_u8(t + L::SHOT_WAIT, w as u8);
    let n = m.read_u8(t + L::SHOTS).wrapping_sub(1);
    m.write_u8(t + L::SHOTS, n);
}

/// Puerta de retirada: tiempo (`PARAM_GATE` × 64), distancia de scroll
/// permitida (`PARAM_DIST` × 16) y cámara de partida.
/// Original: `AimTurret_ScrollGate_061952`.
pub fn gate_start(m: &mut impl Memory, t: u32) {
    let g = (u16::from(m.read_u8(t + L::PARAM_GATE)) << 6) & 0x7FFF;
    m.write_u16(t + L::GATE_TIMER, g);
    let d = u16::from(m.read_u8(t + L::PARAM_DIST)) << 4;
    m.write_u16(t + L::GATE_DIST, d);
    let cam = m.read_u16(L::CAMERA_X);
    m.write_u16(t + L::GATE_CAM, cam);
}

/// ¿Toca retirarse (tiempo acabado o la cámara ha avanzado demasiado)?
/// Original: `AimTurret_ScrollGateB_06197c`.
pub fn gate(m: &mut impl Memory, t: u32) -> bool {
    let g = m.read_u16(t + L::GATE_TIMER).wrapping_sub(L::GATE_STEP);
    m.write_u16(t + L::GATE_TIMER, g);
    if g as i16 <= 0 {
        return true;
    }
    let d = m.read_u16(L::CAMERA_X).wrapping_sub(m.read_u16(t + L::GATE_CAM));
    d as i16 > m.read_u16(t + L::GATE_DIST) as i16
}

/// ¿Tiene menos prioridad que la tarea siguiente?
/// Original: `AimTurret_Stub_0619ae`.
pub fn below_next(m: &mut impl Memory, t: u32) -> bool {
    let next = m.read_u32(t + T::NEXT);
    m.read_u8(t + T::PRIORITY) < m.read_u8(next + T::PRIORITY)
}

/// Suelta su objeto un poco por delante. Original:
/// `AimTurret_DropItem_061482` (sigue en `$0614B0`).
pub fn drop_item(m: &mut impl Memory, t: u32) -> Option<u32> {
    let x = m.read_u16(t + L::X);
    let dx = if facing_left(m, t) { L::ITEM_DX.wrapping_neg() } else { L::ITEM_DX };
    m.write_u16(t + L::X, x.wrapping_add(dx));
    let kind = m.read_u8(t + L::ITEM);
    let o = items::spawn_from(m, t, kind, L::ITEM_PARAMS);
    m.write_u16(t + L::X, x);
    o
}

/// Alta: gráficos, sonido, quieta, parámetros, objetivo, tiempos de la
/// dificultad, caja, puerta y su cañón; sigue activa.
/// Original: `AimTurret_Tmpl08E_060e62`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = entry_at(m);
    for g in L::GFX_BODY {
        anim::alloc_entry(m, t, g);
    }
    sound::play(m, L::SND_LOOP);
    m.write_u8(t + L::HIT_HISTORY, 0);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    manual_icons::stop_motion(m, t);
    probe_move(m, t, at);
    let f = m.read_u8(t + L::PARAM_FACING) & 3;
    m.write_u8(t + L::FACING, f);
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    m.write_u16(t + L::TARGET_X, x);
    m.write_u16(t + L::TARGET_Y, y);
    acquire(m, t);
    let v = soldier::by_difficulty(m, t, L::T_TIMER);
    m.write_u16(t + L::TIMER, v);
    let v = soldier::by_difficulty(m, t, L::T_BURST_WAIT_START);
    m.write_u16(t + L::BURST_WAIT, v);
    m.write_u8(t + L::SHOTS, L::SHOTS_NEW);
    m.write_u32(t + L::HITBOX, L::HITBOX_BODY);
    m.write_u8(t + L::GONE, 0);
    gate_start(m, t);
    tasks::spawn(m, L::GUN, t);
    active(m)
}

fn check_dead(m: &mut impl Memory, t: u32) {
    if fort::dead(m, t) {
        let h = if m.read_u8(t + L::PARAM_GATE) == 0xFF { L::DIE_B } else { L::DIE };
        set_handler(m, t, h);
    }
}

fn leave_if_out(m: &mut impl Machine, t: u32) -> Flow {
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        m.write_u8(t + L::GONE, 0xFF);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Original: `AimTurret_Active_060f04`.
pub fn active(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::ACTIVE_LOOP);
    active_loop(m)
}

/// Vuela hacia su objetivo; al llegar, orbita. Original:
/// `AimTurret_Active_060f04+$6`.
pub fn active_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    acquire(m, t);
    aim(m, t, false);
    flash(m, t);
    update_pose(m, t);
    body_script(m, t);
    anim::step(m, t);
    check_dead(m, t);
    fire_control(m, t);
    if near(m, t, L::CLOSE) {
        set_handler(m, t, L::ORBIT);
    }
    if gate(m, t) {
        set_handler(m, t, L::DORMANT);
    }
    leave_if_out(m, t)
}

/// Original: `AimTurret_ActiveB_060f88`.
pub fn orbit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::ORBIT_LOOP);
    orbit_loop(m)
}

/// Orbita sobre el objetivo; si se aleja, vuelve a perseguirlo.
/// Original: `AimTurret_ActiveB_060f88+$6`.
pub fn orbit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = entry_at(m);
    acquire(m, t);
    orbit_step(m, t, at);
    flash(m, t);
    update_pose(m, t);
    body_script(m, t);
    anim::step(m, t);
    check_dead(m, t);
    fire_control(m, t);
    if !near(m, t, L::CLOSE_ORBIT) {
        set_handler(m, t, L::ACTIVE);
    }
    if gate(m, t) {
        set_handler(m, t, L::DORMANT);
    }
    leave_if_out(m, t)
}

/// Se estrella: explosión, restos, sonidos y se va.
fn crash(m: &mut impl Memory, t: u32) {
    let c = tasks::spawn(m, L::EXPLOSION, t);
    anim::copy_sprite_pos(m, t, c);
    sub16(m, c + L::Y, L::EXPLOSION_DY);
    spawner::install(m, t, L::DEBRIS);
    sound::play(m, L::SND_CRASH);
    sound::stop(m, L::SND_LOOP);
    m.write_u8(t + L::GONE, 0xFF);
}

/// Cayendo: humo cada 4 frames y fuera si sale de la vista.
fn falling(m: &mut impl Machine, t: u32) -> Flow {
    if m.read_u8(L::FRAME_COUNTER) & 3 == 0 {
        let s = spawner::install(m, t, L::SMOKE);
        let dx = if facing_left(m, t) { L::SMOKE_DX.wrapping_neg() } else { L::SMOKE_DX };
        add16(m, s + L::X, dx);
        sub16(m, s + L::Y, L::SMOKE_DY);
    }
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        sound::play(m, L::SND_CRASH);
        sound::stop(m, L::SND_LOOP);
        m.write_u8(t + L::GONE, 0xFF);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Muere: cae hacia el suelo. Original: `AimTurret_Die_06100c`.
pub fn die(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TARGET_Y, L::LOW_TARGET_Y);
    anim::start(m, t, L::S_DIE);
    set_handler(m, t, L::DIE_LOOP);
    die_loop(m)
}

/// Original: `AimTurret_Die_06100c+$18`.
pub fn die_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    aim(m, t, true);
    flash(m, t);
    update_pose(m, t);
    anim::step(m, t);
    if m.read_u16(t + L::Y) as i16 >= L::CRASH_Y {
        crash(m, t);
        drop_item(m, t);
        return tasks::finish(m);
    }
    falling(m, t)
}

/// Muere saliendo volando hacia la derecha. Original:
/// `AimTurret_DieB_0610f4`.
pub fn die_b(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TARGET_X, L::FLY_OFF_TARGET.0);
    m.write_u16(t + L::TARGET_Y, L::FLY_OFF_TARGET.1);
    let s = m.read_u8(t + L::SPREAD) << 1;
    m.write_u8(t + L::SPREAD, s);
    anim::start(m, t, L::S_DIE);
    set_handler(m, t, L::DIE_B_LOOP);
    die_b_loop(m)
}

/// Original: `AimTurret_DieB_0610f4+$28`.
pub fn die_b_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    aim(m, t, false);
    flash(m, t);
    update_pose(m, t);
    anim::step(m, t);
    // `$04400E`: X de pantalla.
    let sx = m.read_u16(t + L::X).wrapping_add(m.read_u16(L::SCREEN_X));
    if sx as i16 >= L::FLY_OFF_SCREEN_X {
        crash(m, t);
        let c = tasks::spawn(m, L::GUN_ATTACK, t);
        anim::copy_sprite_pos(m, t, c);
        return tasks::finish(m);
    }
    falling(m, t)
}

/// Se retira hacia abajo. Original: `AimTurret_Dormant_061204`.
pub fn dormant(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TARGET_Y, L::LOW_TARGET_Y);
    set_handler(m, t, L::DORMANT_LOOP);
    dormant_loop(m)
}

/// Original: `AimTurret_Dormant_061204+$C`.
pub fn dormant_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    aim(m, t, true);
    flash(m, t);
    update_pose(m, t);
    body_script(m, t);
    anim::step(m, t);
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        m.write_u8(t + L::GONE, 0xFF);
        sound::stop(m, L::SND_LOOP);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Cañón (tarea hija del cuerpo). Original: `AimTurret_Gun_061254`.
pub fn gun(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    for g in L::GFX_GUN {
        anim::alloc_entry(m, t, g);
    }
    m.write_u8(t + L::WAIT, 0);
    m.write_u8(t + L::FRAME, 0);
    set_handler(m, t, L::GUN_LOOP);
    gun_loop(m)
}

/// Sigue al cuerpo, se dibuja y se va con él. Original:
/// `AimTurret_Gun_061254+$26`.
pub fn gun_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    parachute::follow_owner(m, t);
    let o = owner(m, t);
    let f = m.read_u8(o + L::FACING);
    m.write_u8(t + L::FACING, f);
    gun_palette(m, t);
    gun_draw(m, t);
    if m.read_u8(o + L::GONE) == 0xFF {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Golpe del cuerpo al salir volando: su lista de ataque unos frames.
/// Original: `AimTurret_GunAttack_0612a8`.
pub fn gun_attack(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::COUNT, L::GUN_ATTACK_FRAMES);
    set_handler(m, t, L::GUN_ATTACK_LOOP);
    gun_attack_loop(m)
}

/// Original: `AimTurret_GunAttack_0612a8+$C`.
pub fn gun_attack_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    m.write_u32(t + L::ATTACKS, L::ATTACKS_GUN);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    let c = m.read_u16(t + L::COUNT).wrapping_sub(1);
    m.write_u16(t + L::COUNT, c);
    if (c as i16) < 0 {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Casquillo que sale despedido al disparar. Original:
/// `AimTurret_Barrel_0612e6`.
pub fn barrel(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX_BARREL);
    parachute::follow_owner(m, t);
    sub16(m, t + L::ATTR, 1);
    let o = owner(m, t);
    let f = m.read_u8(o + L::FACING);
    m.write_u8(t + L::FACING, f);
    anim::start(m, t, L::S_BARREL);
    set_handler(m, t, L::BARREL_LOOP);
    barrel_loop(m)
}

/// Original: `AimTurret_Barrel_0612e6+$30`.
pub fn barrel_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) || items::out_of_view_seen(m, t, Some(L::MARGINS_SHELL)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Proyectil: sale de la torreta con su velocidad X y sube; frena en X.
/// Original: `AimTurret_Shell_061344`.
pub fn shell(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_SHOT);
    anim::alloc_entry(m, t, L::GFX_SHELL);
    anim::start(m, t, L::S_SHELL);
    manual_icons::stop_motion(m, t);
    let v = soldier::by_difficulty(m, t, L::T_SHELL_SPEED);
    m.write_u16(t + L::ACC_Y, v.wrapping_neg());
    let o = owner(m, t);
    for f in [L::VEL_X, L::X, L::Y] {
        let v = m.read_u16(o + f);
        m.write_u16(t + f, v);
    }
    sub16(m, t + L::Y, L::SHELL_DY);
    sub16(m, t + L::ATTR, 1);
    m.write_u16(t + L::TIMER, L::SHELL_TIMER);
    m.write_u32(t + L::HITBOX, L::HITBOX_SHELL);
    m.write_u32(t + L::ATTACKS, L::ATTACKS_SHELL);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::SHELL_LOOP);
    shell_loop(m)
}

/// Vuela; revienta al tocar suelo, si su torreta se va, si la destruyen
/// o si su ataque acierta. Original: `AimTurret_Shell_061344+$80`.
pub fn shell_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = entry_at(m);
    if motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at)) {
        set_handler(m, t, L::SHELL_BURST);
    }
    anim::step(m, t);
    let o = owner(m, t);
    if m.read_u8(o + L::GONE) == 0xFF {
        set_handler(m, t, L::SHELL_BURST);
    }
    if fort::hit_side(m, t).is_some() {
        let f = m.read_u8(t + L::FLAGS2);
        m.write_u8(t + L::FLAGS2, f & !L::HIT);
        sound::play(m, L::SND_HIT);
    }
    if fort::dead(m, t) {
        set_handler(m, t, L::SHELL_BURST);
    } else {
        hits::run_attack_list(m, t);
        if m.read_u8(t + L::FLAGS2) & L::ATTACK_HIT != 0 {
            set_handler(m, t, L::SHELL_BURST);
        }
    }
    let vx = m.read_u16(t + L::VEL_X);
    m.write_u16(t + L::ACC_X, ((0u16.wrapping_sub(vx) as i16) >> 4) as u16);
    if items::out_of_view_seen(m, t, Some(L::MARGINS_SHELL)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Revienta: sonido, fuego y fuera. Original: `AimTurret_ShellBurst_061452`.
pub fn shell_burst(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_BURST);
    let c = tasks::spawn(m, L::FIRE, t);
    anim::copy_sprite_pos(m, t, c);
    set_handler(m, t, L::SHELL_BURST_LOOP);
    shell_burst_loop(m)
}

/// Original: `AimTurret_ShellBurst_061452+$22`.
pub fn shell_burst_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::step(m, t);
    tasks::finish(m)
}

#[cfg(test)]
mod tests {
    use super::atan2_d4;

    #[test]
    fn d4_sign() {
        assert_eq!(atan2_d4(0, 0), 0);
        assert_eq!(atan2_d4(4, 40), 0xFF);
        assert_eq!(atan2_d4(40, 4), 0);
    }
}
