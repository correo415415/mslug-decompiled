//! Proyectil del cañón del Slug: sale de la boca con el ángulo del cañón,
//! acelera hasta su alcance, cae planeando con un rastro de humo y, al
//! tocar el suelo o algo sólido, estalla (tres cajas de ataque seguidas y
//! la explosión).
//!
//! Según la configuración de controles, el disparo en llano puede salir
//! más rápido y lejos (con varios botones a la vez o empujando).
//!
//! Original: `VehicleLaunch_Init_0311c0` (entradas `$0311C0`, `$031240` y
//! `$0312B2`), `VehicleLaunch_Fall_031594`, `VehicleLaunch_Glide_03168a`,
//! `VehicleLaunch_Crash_0317d2` (`_CrashA/B/C`), `_Despawn_0318b8`,
//! `_ReleaseParent_0318d4` (`asm/player/tables_fx_030696.s`), el humo del
//! rastro `$031B78` y la explosión `$031BD0`.

use super::{anim, banner, collision, hits, motion, platforms, rng, slug, slug_drive, sound, tasks, trig, turret};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{slug_drive as D, slug_shell as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn parent(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

fn add16(m: &mut impl Memory, a: u32, v: u16) {
    let x = m.read_u16(a).wrapping_add(v);
    m.write_u16(a, x);
}

fn set_bits(m: &mut impl Memory, a: u32, bits: u8) {
    let v = m.read_u8(a);
    m.write_u8(a, v | bits);
}

/// Velocidad según el ángulo (`+$34`) y la rapidez `speed`.
fn aim(m: &mut impl Memory, t: u32, speed: u16) {
    let a = m.read_u16(t + L::ANGLE);
    let (x, y) = trig::polar(m, a, speed as i16);
    m.write_u16(t + L::VEL_X, x as u16);
    m.write_u16(t + L::VEL_Y, y as u16);
}

/// Ángulo del cañón del Slug (`+$80`) a ángulo del proyectil.
fn barrel_angle(m: &mut impl Memory, slug: u32) -> u16 {
    let a = m.read_u16(slug + L::SLUG_ANGLE);
    let off = if (a as i16) > 0 { L::ANGLE_OFFSET.0 } else { L::ANGLE_OFFSET.1 };
    a.wrapping_add(off) & 0xFF
}

/// Se pone en la boca del cañón (punto del cuadro actual del Slug).
fn at_muzzle(m: &mut impl Memory, t: u32, slug: u32, dy: u16) {
    let pp = m.read_u32(slug + L::SLUG_POSE);
    let pose = m.read_u32(pp);
    let (px, py) = (m.read_u16(pose + 8), m.read_u16(pose + 0xA));
    add16(m, t + L::X, px);
    add16(m, t + L::Y, py.wrapping_add(dy));
}

/// Comienzo común de las tres entradas.
fn begin(m: &mut impl Memory, t: u32) {
    m.write_u32(t + L::ATTACKS, L::SHELL_ATTACKS);
    slug::clear_flags13(m, t);
    anim::start(m, t, L::SCRIPT);
}

/// Disparo parado: posición, ángulo y alcance de la tabla por pendiente.
/// Original: `VehicleLaunch_Init_0311c0`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let p = parent(m, t);
    let k = u32::from(m.read_u16(p + L::SLUG_SLOPE_IDX)) * L::OFFSET_LEN;
    begin(m, t);
    let r = L::OFFSETS + (k & 0xFFFF);
    let dx = m.read_u16(r + 4);
    add16(m, t + L::X, dx);
    let dy = m.read_u16(r + 6);
    add16(m, t + L::Y, dy);
    let a = m.read_u16(r + 8);
    m.write_u16(t + L::ANGLE, a);
    m.write_u16(t + L::FRAME, a >> 3);
    m.write_u16(t + L::RANGE, L::RANGE_IDLE);
    m.write_u16(t + L::SPEED, L::SPEED_BASE);
    aim(m, t, L::LAUNCH_SPEED);
    launch(m, t)
}

/// Disparo hacia la izquierda. Original: `VehicleLaunch_Init_0311c0+$80`.
pub fn left(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    begin(m, t);
    let p = parent(m, t);
    let a = barrel_angle(m, p);
    m.write_u16(t + L::ANGLE, a);
    at_muzzle(m, t, p, 0);
    m.write_u16(t + L::SPEED, L::SPEED_BASE);
    m.write_u16(t + L::RANGE, L::RANGE_LEFT);
    m.write_u8(t + L::TRAIL, 0);
    m.write_u32(t + L::TRAIL_FX, L::SMOKE);
    finish_aim(m, t)
}

/// Disparo en llano (con las variantes de la configuración de controles).
/// Original: `VehicleLaunch_Init_0311c0+$F2`.
pub fn flat(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    begin(m, t);
    m.write_u8(t + L::TRAIL, L::NO_TRAIL);
    m.write_u32(t + L::TRAIL_FX, L::SMOKE);
    m.write_u16(t + L::SPEED, L::SPEED_BASE);
    m.write_u16(t + L::RANGE, L::RANGE_FLAT);
    let p = parent(m, t);
    let a = barrel_angle(m, p);
    m.write_u16(t + L::ANGLE, a);
    let layout = m.read_u8(D::CONTROL_LAYOUT);
    match layout {
        L::LAYOUT_MULTI => {
            // Con varios botones a la vez sale más fuerte y más bajo.
            if let Some(b) = input::pressed_bits(m, Entity::from_raw(p), Buttons(0xF0), Edge::Held) {
                let tilt = match (b & 0xF0).count_ones() {
                    4 => Some(0xFFF8u16),
                    3 => Some(0),
                    2 => Some(8),
                    _ => None,
                };
                if let Some(d) = tilt {
                    m.write_u16(t + L::SPEED, L::SPEED_FAST);
                    m.write_u16(t + L::RANGE, L::RANGE_FAST);
                    let a = m.read_u16(t + L::ANGLE).wrapping_sub(d) & 0xF8;
                    m.write_u16(t + L::ANGLE, a);
                }
            }
        }
        L::LAYOUT_PUSH => {
            // Disparando mientras empuja a la derecha, más fuerte.
            if input::pressed_bits(m, Entity::from_raw(p), Buttons(0xF0), Edge::Held).is_some()
                && slug_drive::push_right(m, p)
            {
                m.write_u16(t + L::SPEED, L::SPEED_FAST);
                m.write_u16(t + L::RANGE, L::RANGE_PUSH);
                let a = m.read_u16(t + L::ANGLE).wrapping_sub(8) & 0xF8;
                m.write_u16(t + L::ANGLE, a);
                m.write_u8(t + L::TRAIL, 0);
            }
        }
        _ => {}
    }
    at_muzzle(m, t, p, L::MUZZLE_DY);
    finish_aim(m, t)
}

/// Cuadro y velocidad inicial según el ángulo y al lanzamiento.
fn finish_aim(m: &mut impl Machine, t: u32) -> Flow {
    let a = m.read_u16(t + L::ANGLE);
    m.write_u16(t + L::FRAME, a >> 3);
    aim(m, t, L::LAUNCH_SPEED);
    launch(m, t)
}

/// Sonido, capa, el destello de la boca, el gráfico, cuenta el disparo en
/// el Slug y al vuelo. Original: `VehicleLaunch_Init_0311c0+$330`.
fn launch(m: &mut impl Machine, t: u32) -> Flow {
    sound::play(m, L::SOUND);
    m.write_u16(t + L::ATTR, L::ATTR_FLY);
    let c = tasks::spawn(m, L::FLASH, t);
    anim::copy_sprite_pos(m, t, c);
    anim::alloc_entry(m, t, L::GFX);
    m.write_u16(t + L::TRAVEL, 0);
    let p = parent(m, t);
    let n = m.read_u8(p + D::SHOTS).wrapping_add(1);
    m.write_u8(p + D::SHOTS, n);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a & L::ATTR_KEEP);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::FLY);
    fly(m)
}

/// Al vuelo: acelera hasta su alcance y luego cae; rastro de humo, choque
/// y golpes. Original: `VehicleLaunch_Init_0311c0+$382` (`$031542`).
pub fn fly(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u16(t + L::TRAVEL) < m.read_u16(t + L::RANGE) {
        let s = m.read_u16(t + L::SPEED);
        aim(m, t, s);
        set_handler(m, t, L::CRUISE);
    }
    cruise(m)
}

/// Original: `$03156C`.
pub fn cruise(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = m.read_u16(t + L::VEL_X);
    add16(m, t + L::TRAVEL, v);
    if m.read_u16(t + L::TRAVEL) >= m.read_u16(t + L::RANGE) {
        set_handler(m, t, L::FALL);
        let g = (m.read_u16(t + L::SPEED) as i16 >> 4).wrapping_neg();
        m.write_u16(t + L::ACC_Y, g as u16);
        return fall(m);
    }
    fly_tail(m, t)
}

/// Cayendo: cuando deja de subir, planea. Original: `VehicleLaunch_Fall_031594`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u16(t + L::VEL_Y) as i16 <= 0 {
        set_handler(m, t, L::GLIDE);
    }
    fly_tail(m, t)
}

/// Cuadro según la dirección del movimiento.
fn face_motion(m: &mut impl Memory, t: u32) {
    let vx = (m.read_u16(t + L::VEL_X) as i16 >> 4) as u16;
    let vy = (m.read_u16(t + L::VEL_Y) as i16 >> 4) as u16;
    let a = banner::atan2(m, vx, vy);
    m.write_u16(t + L::FRAME, u16::from(a) >> 3);
}

/// Rastro de humo cada 12 frames (o en cada uno, según los controles).
/// Devuelve el humo creado.
fn trail(m: &mut impl Memory, t: u32) -> Option<u32> {
    let n = m.read_u8(t + L::TRAIL);
    if n == L::NO_TRAIL {
        return None;
    }
    let n = n.wrapping_add(1);
    m.write_u8(t + L::TRAIL, n);
    let mask = match m.read_u8(D::CONTROL_LAYOUT) {
        L::LAYOUT_EVERY_A | L::LAYOUT_EVERY_B => 0,
        _ => L::TRAIL_MASK,
    };
    if n & mask != 0 {
        return None;
    }
    let fx = m.read_u32(t + L::TRAIL_FX);
    let c = tasks::spawn(m, fx, t);
    anim::copy_sprite_pos(m, t, c);
    Some(c)
}

/// Final común al vuelo: humo, avance, animación y golpes.
fn fly_tail(m: &mut impl Machine, t: u32) -> Flow {
    face_motion(m, t);
    if let Some(c) = trail(m, t) {
        for f in [L::ANGLE, L::FRAME, L::ATTR] {
            let v = m.read_u16(t + f);
            m.write_u16(c + f, v);
        }
    }
    set_bits(m, t + T::FLAGS2, T::FLAGS2_FRESH);
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    if motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at)) {
        set_handler(m, t, L::CRASH_JMP);
    }
    hit_tail(m, t)
}

/// Animación, cajas de ataque, choque contra algo sólido o contra un
/// blindado, y fuera de la pantalla.
fn hit_tail(m: &mut impl Machine, t: u32) -> Flow {
    anim::step(m, t);
    let d1 = m.regs().d[1] as u8;
    let (_, _, d1) = hits::run_attack_list_d1(m, t, d1);
    if m.read_u8(t + T::FLAGS3) & L::WALL != 0 {
        set_handler(m, t, L::CRASH);
    }
    if m.read_u8(t + L::FLAGS_13) & L::BLOCKED != 0 {
        set_handler(m, t, L::CRASH_JMP);
        if d1 == L::ARMOURED {
            set_handler(m, t, L::DESPAWN);
        }
    }
    if platforms_out(m, t) {
        set_handler(m, t, L::RELEASE);
    }
    Flow::Return
}

fn platforms_out(m: &mut impl Memory, t: u32) -> bool {
    super::items::out_of_view_seen(m, t, Some(L::MARGINS))
}

/// Planeo: con el disparo fuerte vuelve a la rapidez normal; frena en X,
/// cae con gravedad y estalla al tocar suelo.
/// Original: `VehicleLaunch_Glide_03168a`.
pub fn glide(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u16(t + L::RANGE) == L::RANGE_FAST {
        m.write_u16(t + L::SPEED, L::SPEED_BASE);
        let a = m.read_u16(t + L::ANGLE);
        let (x, _) = trig::polar(m, a, L::SPEED_BASE as i16);
        m.write_u16(t + L::VEL_X, x as u16);
    }
    m.write_u16(t + L::ACC_X, L::GLIDE_ACC.0);
    m.write_u16(t + L::ACC_Y, L::GLIDE_ACC.1);
    set_handler(m, t, L::GLIDE_LOOP);
    glide_loop(m)
}

/// Original: `VehicleLaunch_Glide_03168a__L0316be`.
pub fn glide_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if let Some(c) = trail(m, t) {
        for f in [L::FRAME, L::ATTR] {
            let v = m.read_u16(t + f);
            m.write_u16(c + f, v);
        }
    }
    let (vy, _) = collision::clamp_velocity(m.read_u16(t + L::VEL_Y), L::MAX_FALL);
    m.write_u16(t + L::VEL_Y, vy);
    if m.read_u16(t + L::VEL_X) as i16 <= 0 {
        m.write_u16(t + L::VEL_X, 0);
        m.write_u16(t + L::ACC_X, 0);
    }
    set_bits(m, t + T::FLAGS2, T::FLAGS2_FRESH);
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    if motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at)) {
        set_handler(m, t, L::CRASH_JMP);
        let r = m.regs();
        if !collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
            set_handler(m, t, L::CRASH);
        }
    }
    face_motion(m, t);
    hit_tail(m, t)
}

/// Estalla: marca el impacto, sonido, sin cajas propias, descuenta el
/// disparo, pasa a la explosión y crea la onda (tres cajas seguidas).
/// Original: `VehicleLaunch_Crash_0317d2` (y `_JmpCrash_0318b4`).
pub fn crash(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(L::IMPACT_FLAG, 1);
    sound::play(m, L::CRASH_SOUND);
    m.write_u16(t + L::ATTR, L::ATTR_FLY);
    m.write_u32(t + L::ATTACKS, T::NIL);
    slug::clear_flags13(m, t);
    let p = parent(m, t);
    let n = m.read_u8(p + D::SHOTS).wrapping_sub(1);
    m.write_u8(p + D::SHOTS, n);
    set_handler(m, t, L::EXPLOSION);
    let c = tasks::spawn(m, L::BLAST_A, t);
    anim::copy_sprite_pos(m, t, c);
    turret::copy_pilot(m, t, c);
    Flow::Return
}

/// Una caja de la onda: la prueba y sigue al proyectil.
fn blast(m: &mut impl Machine, t: u32, attacks: u32) {
    slug::clear_flags13(m, t);
    m.write_u32(t + L::ATTACKS, attacks);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
}

/// Original: `VehicleLaunch_CrashA_031826`.
pub fn blast_a(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::BLAST_B);
    blast(m, t, L::BLAST_ATTACKS[0]);
    slug::follow_owner(m, t);
    Flow::Return
}

/// Original: `VehicleLaunch_CrashB_03184e`.
pub fn blast_b(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    blast(m, t, L::BLAST_ATTACKS[1]);
    set_handler(m, t, L::BLAST_C);
    slug::follow_owner(m, t);
    Flow::Return
}

/// Original: `VehicleLaunch_CrashC_031876`.
pub fn blast_c(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    blast(m, t, L::BLAST_ATTACKS[2]);
    slug::follow_owner(m, t);
    m.write_u32(t + L::ATTACKS, T::NIL);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::BLAST_END);
    Flow::Return
}

/// Original: `$0318AC`.
pub fn blast_end(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}

/// Contra un blindado: desaparece sin estallar.
/// Original: `VehicleLaunch_Despawn_0318b8`.
pub fn despawn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::kill_children(m, t);
    m.write_u32(t + L::ATTACKS, T::NIL);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::RELEASE);
    Flow::Return
}

/// Descuenta el disparo del Slug y se da de baja.
/// Original: `VehicleLaunch_ReleaseParent_0318d4`.
pub fn release(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let p = parent(m, t);
    let n = m.read_u8(p + D::SHOTS).wrapping_sub(1);
    m.write_u8(p + D::SHOTS, n);
    tasks::finish(m)
}

/// Explosión del proyectil. Original: `$031BD0`.
pub fn explosion(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::EXPLOSION_GFX);
    anim::start(m, t, L::EXPLOSION_SCRIPT);
    m.write_u16(t + L::ATTR, L::EXPLOSION_ATTR);
    let (r, _) = rng::next(m);
    add16(m, t + L::X, (r & 0xF).wrapping_sub(8));
    set_handler(m, t, L::EXPLOSION_LOOP);
    explosion_loop(m)
}

/// Original: `$031C02`.
pub fn explosion_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Humo del rastro: dos frames, cada uno un poco más al fondo.
/// Original: `$031B78`.
pub fn smoke(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::SMOKE_GFX);
    anim::start(m, t, L::SMOKE_SCRIPT);
    m.write_u16(t + L::TRAVEL, L::SMOKE_FRAMES);
    set_handler(m, t, L::SMOKE_LOOP);
    smoke_loop(m)
}

/// Original: `$031B98`.
pub fn smoke_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let a = m.read_u16(t + L::ATTR).wrapping_sub(1);
    m.write_u16(t + L::ATTR, a);
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::ANIM_TIMER) == 0 {
        m.write_u8(t + L::ANIM_TIMER, 0xFF);
    }
    let n = m.read_u16(t + L::TRAVEL).wrapping_sub(1);
    m.write_u16(t + L::TRAVEL, n);
    if (n as i16) > 0 {
        return Flow::Return;
    }
    tasks::kill_children(m, t);
    tasks::finish(m)
}
