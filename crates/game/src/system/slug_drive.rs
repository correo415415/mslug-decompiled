//! Slug conducido: el bucle principal mientras el jugador va montado.
//! Empuja con las flechas, salta, dispara el cañón, se agacha (abajo) o
//! queda sin piloto si el jugador se ha bajado.
//!
//! Original: `Slug_DriveAlt_02e9c8` (con la entrada `$02EA1C` y el bucle
//! `$02EA26`, `asm/slug/vehicle_02dd20.s`), `Slug_CallGroundProbeA_02a328`,
//! `Slug_CallGroundProbeB_02a34e`, `Slug_CheckFreeThenC_02a59a`,
//! `Slug_InputDirByLayoutA_02aaf0`, `Slug_CanFire_02aac0`, `$02AB86`,
//! `Slug_ConsumeField89_02ac80`, `ClearXN_02abc0`, `$02ABC6`,
//! `Slug_UpdateAngleIsSlope_02a4f0` y el Slug parado en llano
//! (`Slug_IdleFlat_02b38c`, bucle `$02B3E0`, `asm/slug/states_02ae3e.s`),
//! parado en cuesta (`Slug_IdleSlope_02b4d2`, bucle `$02B544`), el crucero
//! (`Slug_CruiseRightB_02bf64`, `Slug_CruiseLeftB_02c07a`) y la aceleración
//! (`Slug_AccelRightB_02b8d8`, `Slug_AccelLeftB_02ba3e`) y la frenada
//! (`Slug_BrakeRight_02bd00`, `Slug_BrakeLeft_02be32`)
//! (`asm/slug/helpers_0295a6.s`).

use super::{anim, collision, debris, hits, items, player_ground, slug, slug_attack, slug_hunker, slug_idle, slug_jump, sound, tasks};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{slug_drive as L, slug_idle as I, task as T};
use crate::mem::{Machine, Memory};
use crate::player::controls;
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn held(m: &mut impl Memory, t: u32, b: Buttons) -> bool {
    input::pressed(m, Entity::from_raw(t), b, Edge::Held)
}

/// Entrada: guion según la pendiente, ataque, `+$8B`, `+$20` y el bucle.
/// Original: `Slug_DriveAlt_02e9c8`.
pub fn enter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + I::FLAGS_6B, I::BIT_6B_0, true);
    set_bits(m, t + I::FLAGS_8C, I::BIT_8C_5 | I::BIT_8C_1, false);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(L::SCRIPTS + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    let attack = m.read_u32(L::ATTACKS + u32::from(idx) * 4);
    m.write_u32(t + L::HITBOX_CB, attack);
    m.write_u8(t + L::BYTE_8B, L::BYTE_8B_DRIVE);
    m.write_u8(t + L::BYTE_20, L::BYTE_20_BASE.wrapping_add(idx as u8));
    resume(m)
}

/// Sin aceleración y al bucle. Original: `Slug_DriveAlt_02e9c8__L02ea1c`.
pub fn resume(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + I::ACC_X, 0);
    m.write_u32(t + T::HANDLER, L::LOOP);
    drive(m)
}

/// Bucle conducido. Original: `Slug_DriveAlt_02e9c8__L02ea26`.
pub fn drive(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    m.write_u16(t + I::VEL_X, 0);
    if push_right(m, t) {
        m.write_u16(t + I::VEL_X, L::PUSH_RIGHT);
    }
    if push_left(m, t) {
        m.write_u16(t + I::VEL_X, L::PUSH_LEFT);
    }
    if !slug_hunker::physics(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_SLOPE);
    }
    m.write_u16(t + I::VEL_X, 0);
    slug::update_steering(m, t);
    if anim::step(m, t) {
        m.write_u32(t + L::HITBOX_CB, L::DONE_HITBOX);
        m.write_u32(t + L::HANDLER_60, L::DONE_HANDLER_60);
        m.write_u32(t + T::HANDLER, L::IDLE_FLAT);
        if push_right(m, t) {
            m.write_u32(t + T::HANDLER, L::CRUISE_RIGHT);
        }
        if push_left(m, t) {
            m.write_u32(t + T::HANDLER, L::CRUISE_LEFT);
        }
    }
    if down_held(m, t) {
        m.write_u32(t + T::HANDLER, L::TURN);
    }
    if slug::ground_contact(m, t).in_air {
        m.write_u32(t + T::HANDLER, L::FALL);
    }
    if jump_dir(m, t).is_some() {
        m.write_u32(t + T::HANDLER, L::JUMP);
    }
    if can_fire(m, t) {
        m.write_u32(t + T::HANDLER, L::FIRE);
    }
    if left_alone(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_ENTER_B);
    }
    if slug_hunker::eject_pressed(m, t) {
        m.write_u32(t + T::HANDLER, L::EJECT);
    }
    let side = slug_hunker::damage_check(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::HIT_HANDLERS, side, None);
    Flow::Return
}

/// ¿Empuja a la derecha? (la tabla del original por `+$94` tiene en todas
/// las entradas el mismo test). Original: `Slug_CallGroundProbeA_02a328`.
pub fn push_right(m: &mut impl Memory, t: u32) -> bool {
    held(m, t, Buttons::RIGHT)
}

/// ¿Empuja a la izquierda? Original: `Slug_CallGroundProbeB_02a34e`.
pub fn push_left(m: &mut impl Memory, t: u32) -> bool {
    held(m, t, Buttons::LEFT)
}

/// ¿Abajo mantenido? Original: `Slug_CheckFreeThenC_02a59a` / `$02ABC6`.
pub fn down_held(m: &mut impl Memory, t: u32) -> bool {
    held(m, t, Buttons::DOWN)
}

/// Salto según la configuración de controles: la dirección horizontal +1
/// (1 vertical, 2 izquierda, 3 derecha). Original:
/// `Slug_InputDirByLayoutA_02aaf0` (C = salta, valor en `d0`).
pub fn jump_dir(m: &mut impl Memory, t: u32) -> Option<u8> {
    controls::jump_held(m, Entity::from_raw(t)).map(|d| d + 1)
}

/// ¿Puede disparar el cañón (munición, menos de dos disparos y sin
/// espera) y se ha pulsado su botón? Original: `Slug_CanFire_02aac0`.
pub fn can_fire(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(t + L::AMMO) == 0
        || m.read_u8(t + L::SHOTS) >= L::MAX_SHOTS
        || m.read_u8(t + L::COOLDOWN) != 0
    {
        return false;
    }
    fire_pressed(m, t)
}

/// Botón del cañón según la configuración: A (4), C (0) o cualquiera de
/// A–D. Original: `$02AB86`.
pub fn fire_pressed(m: &mut impl Memory, t: u32) -> bool {
    let b = match m.read_u8(L::CONTROL_LAYOUT) {
        4 => Buttons::A,
        0 => Buttons::C,
        _ => Buttons(0xF0),
    };
    input::pressed(m, Entity::from_raw(t), b, Edge::Pressed)
}

/// Sin aviso de montado este frame, se queda sin controlador (y devuelve
/// sí); con aviso, lo consume. Original: `Slug_ConsumeField89_02ac80`.
pub fn left_alone(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(t + L::BOARDED) == 0 {
        m.write_u8(t + L::CONTROLLER, L::NO_CONTROLLER);
        return true;
    }
    m.write_u8(t + L::BOARDED, 0);
    false
}

/// Llamada del guion: si queda pendiente (`+$8B`), pide el sonido `code`
/// una vez. Original: `Slug_StopMusic108F_02a2f8` / `Slug_StopMusic1092_02a310`.
pub fn engine_sound(m: &mut impl Memory, t: u32, code: u16) {
    if m.read_u8(t + L::BYTE_8B) != 0 {
        sound::play(m, code);
        m.write_u8(t + L::BYTE_8B, 0);
    }
}

/// ¿Ha cambiado la pendiente y la nueva es cuesta? Guarda la nueva en
/// `+$80`. Original: `Slug_UpdateAngleIsSlope_02a4f0` (C = sí).
pub fn slope_changed_to_hill(m: &mut impl Memory, t: u32) -> bool {
    let (_, _, dy) = slug::terrain_slope(m, t);
    if dy == m.read_u16(t + L::SLOPE) {
        return false;
    }
    m.write_u16(t + L::SLOPE, dy);
    m.read_u8(L::FLAT_BY_SLOPE.wrapping_add(dy as i16 as u32)) == 0
}

/// Entrada parado en llano: quieto, guion y ataque según la pendiente,
/// `+$20`, `+$60` y el bucle. Original: `Slug_IdleFlat_02b38c`.
pub fn flat_enter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player_ground::clear_motion(m, t);
    m.write_u16(t + I::VEL_X, 0);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(L::FLAT_SCRIPTS + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    let attack = m.read_u32(L::FLAT_ATTACKS + u32::from(idx) * 4);
    m.write_u32(t + L::HITBOX_CB, attack);
    m.write_u8(t + L::BYTE_20, L::FLAT_BYTE_20.wrapping_add(idx as u8));
    m.write_u32(t + L::HANDLER_60, L::FLAT_HANDLER_60);
    m.write_u32(t + T::HANDLER, L::FLAT_LOOP);
    flat(m)
}

/// Bucle parado en llano: pasa a cuesta, a acelerar, a agacharse, a caer,
/// a saltar, a disparar, sin piloto, a bajarse o a los golpes; fuera de la
/// pantalla se da de baja. Original: `Slug_IdleFlat_02b38c__L02b3e0`.
pub fn flat(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::DAMAGE_BOX, L::FLAT_DAMAGE_BOX);
    slug::clear_flags13(m, t);
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    slug_hunker::physics(m, t);
    if slope_changed_to_hill(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_SLOPE);
    }
    if push_right(m, t) {
        m.write_u32(t + T::HANDLER, L::ACCEL_RIGHT);
    }
    if push_left(m, t) {
        m.write_u32(t + T::HANDLER, L::ACCEL_LEFT);
    }
    if down_held(m, t) {
        let idx = slug::slope_anim_idx(m, t) & 0xFF;
        tasks::goto_by_table(m, t, L::CROUCH_BY_SLOPE, idx, None);
    }
    anim::step(m, t);
    if slug::ground_contact(m, t).in_air {
        m.write_u32(t + T::HANDLER, L::FALL);
    }
    let dir = jump_dir(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::JUMP_BY_DIR, u16::from(dir), None);
    if can_fire(m, t) {
        m.write_u32(t + T::HANDLER, L::FIRE);
    }
    if left_alone(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_ENTER_B);
    }
    if slug_hunker::eject_pressed(m, t) {
        m.write_u32(t + T::HANDLER, L::EJECT);
    }
    let side = slug_hunker::damage_check(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::HIT_HANDLERS, side, None);
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return Flow::Goto(L::DEATH_FINISH);
    }
    Flow::Return
}

/// Llamada del guion: sonido según el terreno bajo el Slug. El original
/// mira los bits `$C0` del tipo de suelo que deja el sondeo, que nunca los
/// lleva (es el tipo ya enmascarado a `$3F`, o `$11` sobre plataforma), así
/// que siempre suena el primero. Devuelve el código.
/// Original: `Slug_MusicByTerrainA_02a5d4` / `Slug_MusicByTerrainB_02a606`.
pub fn land_sound(m: &mut impl Memory, t: u32, sounds: [u16; 2], frac: (u16, u16)) -> u16 {
    let f = collision::probe_floor(m, t, frac);
    let kind = if f.platform.is_some() { 0x11 } else { f.tile.kind };
    let code = if matches!(kind & 0xC0, 0x40 | 0x80) { sounds[1] } else { sounds[0] };
    sound::play(m, code);
    code
}

/// Inclina el sprite según la pendiente `angle` (guarda ésta en `+$80`):
/// guion por el índice de inclinación y `+$20` = índice / 4.
fn slope_pose(m: &mut impl Memory, t: u32, angle: u16) {
    m.write_u16(t + L::SLOPE, angle);
    let k = L::TILT_CENTER.wrapping_sub(((angle as i16) >> 1) as u16).min(L::TILT_MAX);
    let s = slug::angle_sprite(m, k);
    let script = m.read_u32(L::TILT_SCRIPTS.wrapping_add(s.wrapping_mul(4) as i16 as u32));
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, (k >> 2) as u8);
}

/// Entrada parado en cuesta: sin velocidad pedida ni aceleración, el
/// sprite inclinado y `+$60`. Original: `Slug_IdleSlope_02b4d2` (y las dos
/// entradas de `Slug_IdleSlopeJmp_02b4ca`).
pub fn slope_enter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::SPEED, 0);
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    m.write_u16(t + L::ACC_X, 0);
    let angle = slug::terrain_slope(m, t).2;
    slope_pose(m, t, angle);
    m.write_u32(t + L::HANDLER_60, L::FLAT_HANDLER_60);
    m.write_u32(t + T::HANDLER, L::SLOPE_LOOP);
    slope(m)
}

/// Bucle parado en cuesta: empuja si se pulsa (con la configuración
/// `LAYOUT_BOOST`, más cuesta abajo), sigue la pendiente y pasa a llano,
/// a rodar, a agacharse, a caer, a saltar, a disparar, sin piloto, a
/// destruido o a los golpes. Original: `Slug_IdleSlope_02b4d2+$72`.
pub fn slope(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::DAMAGE_BOX, L::FLAT_DAMAGE_BOX);
    slug::clear_flags13(m, t);
    set_bits(m, t + L::FLAGS_8D, L::BIT_8D_2, true);
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    let mut v = m.read_u16(t + L::SPEED);
    if m.read_u8(t + L::PUSH_LOCK) == 0 {
        if push_right(m, t) {
            if m.read_u16(t + L::SPEED) == 0 {
                v = L::PUSH_RIGHT;
            }
        } else if push_left(m, t) && m.read_u16(t + L::SPEED) == 0 {
            v = L::PUSH_LEFT;
        }
    }
    if m.read_u8(L::CONTROL_LAYOUT) == L::LAYOUT_BOOST {
        let d = slug::terrain_slope(m, t).2.wrapping_sub(m.read_u16(t + L::SLOPE));
        if (d ^ v) & 0x8000 == 0 {
            v = v.wrapping_add(((v as i16) >> 1) as u16);
        }
    }
    m.write_u16(t + I::VEL_X, v);
    slug::update_steering(m, t);
    slug_attack::physics_c(m, t);
    let angle = slug::terrain_slope(m, t).2;
    if angle != m.read_u16(t + L::SLOPE) {
        slope_pose(m, t, angle);
    }
    anim::step(m, t);
    if !slug_jump::stored_slope_is_hill(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_FLAT);
        if push_right(m, t) {
            m.write_u32(t + T::HANDLER, L::CRUISE_RIGHT);
        }
        if push_left(m, t) {
            m.write_u32(t + T::HANDLER, L::CRUISE_LEFT);
        }
    }
    if m.read_u8(t + L::NO_ATTACKS) == 0 {
        hits::run_attack_list(m, t);
    }
    if down_held(m, t) {
        m.write_u32(t + T::HANDLER, L::SLOPE_CROUCH);
    }
    if slug::ground_contact(m, t).in_air {
        m.write_u32(t + T::HANDLER, L::FALL_START);
    }
    let dir = slug_jump::jump_pressed_dir(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::JUMP_BY_DIR, u16::from(dir), None);
    if can_fire(m, t) {
        m.write_u32(t + T::HANDLER, L::FIRE_SLOPE);
    }
    if left_alone(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_ENTER_B);
    }
    if slug_hunker::eject_pressed(m, t) {
        m.write_u32(t + T::HANDLER, L::DESTROYED);
    }
    let side = slug_hunker::damage_check(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::HIT_HANDLERS, side, None);
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return Flow::Goto(L::DEATH_FINISH);
    }
    Flow::Return
}

/// Crucero hacia la derecha (`right`) o la izquierda: velocidad fija,
/// guion y ataque por pendiente y `+$20`. Originales:
/// `Slug_CruiseRightB_02bf64`, `Slug_CruiseLeftB_02c07a`.
pub fn cruise_enter(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let (vel, scripts, byte_20, run) = if right {
        (L::PUSH_RIGHT, L::CRUISE_RIGHT_SCRIPTS, L::CRUISE_RIGHT_BYTE_20, L::CRUISE_RIGHT_LOOP)
    } else {
        (L::PUSH_LEFT, L::CRUISE_LEFT_SCRIPTS, L::CRUISE_LEFT_BYTE_20, L::CRUISE_LEFT_LOOP)
    };
    m.write_u16(t + I::VEL_X, vel);
    m.write_u16(t + L::ACC_X, 0);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(scripts + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    let attack = m.read_u32(L::FLAT_ATTACKS + u32::from(idx) * 4);
    m.write_u32(t + L::HITBOX_CB, attack);
    m.write_u8(t + L::BYTE_20, byte_20.wrapping_add(idx as u8));
    m.write_u32(t + T::HANDLER, run);
    cruise(m, right)
}

/// Bucle del crucero: a cuesta, a frenar si se suelta, al crucero
/// contrario, a agacharse, a caer, a saltar, a disparar, sin piloto, a
/// bajarse o a los golpes. Originales: `Slug_CruiseRightB_02bf64+$4C`,
/// `Slug_CruiseLeftB_02c07a+$4C`.
pub fn cruise(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let (slope, brake, turn) = if right {
        (L::SLOPE_FROM_RIGHT, L::BRAKE_RIGHT, L::CRUISE_LEFT)
    } else {
        (L::SLOPE_FROM_LEFT, L::BRAKE_LEFT, L::CRUISE_RIGHT)
    };
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    if !slug_hunker::physics(m, t) {
        m.write_u32(t + T::HANDLER, slope);
    }
    if slope_changed_to_hill(m, t) {
        m.write_u32(t + T::HANDLER, slope);
    }
    let (keep, other) = if right {
        (push_right(m, t), push_left(m, t))
    } else {
        (push_left(m, t), push_right(m, t))
    };
    if !keep {
        m.write_u32(t + T::HANDLER, brake);
    }
    if other {
        m.write_u32(t + T::HANDLER, turn);
    }
    anim::step(m, t);
    hits::run_attack_list(m, t);
    drive_tail(m, t)
}

/// Aceleración hacia la derecha (`right`) o la izquierda, levantando polvo.
/// Original: `Slug_AccelRightB_02b8d8`, `Slug_AccelLeftB_02ba3e`.
pub fn accel_dust(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    debris::dust_pair(m, t, if right { 0 } else { 1 });
    accel_enter(m, right)
}

/// Aceleración (sin polvo): la que falta hasta la velocidad de crucero en
/// `ACCEL_FRAMES` frames, guion y sonido por pendiente y `+$20`.
/// Original: `Slug_AccelRightB_02b8d8__L02b8de`, `Slug_AccelLeftB_02ba3e__L02ba44`.
pub fn accel_enter(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let (vel, scripts, sounds, byte_20, run) = if right {
        (L::PUSH_RIGHT, L::ACCEL_RIGHT_SCRIPTS, L::ACCEL_RIGHT_SOUNDS, L::ACCEL_RIGHT_BYTE_20, L::ACCEL_RIGHT_LOOP)
    } else {
        (L::PUSH_LEFT, L::ACCEL_LEFT_SCRIPTS, L::ACCEL_LEFT_SOUNDS, L::ACCEL_LEFT_BYTE_20, L::ACCEL_LEFT_LOOP)
    };
    let acc = (vel.wrapping_sub(m.read_u16(t + I::VEL_X)) as i16) >> 3;
    m.write_u16(t + L::ACC_X, acc as u16);
    m.write_u8(t + L::ACCEL_TIMER, L::ACCEL_FRAMES);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(scripts.wrapping_add(u32::from(idx.wrapping_mul(4)) & 0xFFFF));
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    let code = m.read_u16(sounds + u32::from(idx.wrapping_mul(2)) & 0xFF_FFFF);
    if code != L::NO_SOUND {
        sound::play(m, code);
    }
    m.write_u8(t + L::BYTE_20, byte_20.wrapping_add(idx as u8));
    m.write_u32(t + T::HANDLER, run);
    accel(m, right)
}

/// Cuenta los frames de aceleración; al acabar fija la velocidad de
/// crucero y sigue en [`accel_run`]. Original: `Slug_AccelRightB_02b8d8+$70`,
/// `Slug_AccelLeftB_02ba3e+$70`.
pub fn accel(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let n = m.read_u8(t + L::ACCEL_TIMER).wrapping_sub(1);
    m.write_u8(t + L::ACCEL_TIMER, n);
    if n == 0 {
        let (vel, run) = if right { (L::PUSH_RIGHT, L::ACCEL_RIGHT_RUN) } else { (L::PUSH_LEFT, L::ACCEL_LEFT_RUN) };
        m.write_u16(t + I::VEL_X, vel);
        m.write_u16(t + L::ACC_X, 0);
        m.write_u32(t + T::HANDLER, run);
    }
    accel_run(m, right)
}

/// Acelerando: como el crucero, pero al acabar el guion pasa a crucero.
/// Original: `Slug_AccelRightB_02b8d8+$88`, `Slug_AccelLeftB_02ba3e+$88`.
pub fn accel_run(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let (slope, brake, same, turn) = if right {
        (L::SLOPE_FROM_RIGHT, L::BRAKE_RIGHT, L::CRUISE_RIGHT, L::CRUISE_LEFT)
    } else {
        (L::SLOPE_FROM_LEFT, L::BRAKE_LEFT, L::CRUISE_LEFT, L::CRUISE_RIGHT)
    };
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    if !slug_hunker::physics(m, t) {
        m.write_u32(t + T::HANDLER, slope);
    }
    if slope_changed_to_hill(m, t) {
        m.write_u32(t + T::HANDLER, slope);
    }
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, same);
    }
    hits::run_attack_list(m, t);
    let (keep, other) = if right {
        (push_right(m, t), push_left(m, t))
    } else {
        (push_left(m, t), push_right(m, t))
    };
    if !keep {
        m.write_u32(t + T::HANDLER, brake);
    }
    if other {
        m.write_u32(t + T::HANDLER, turn);
    }
    drive_tail(m, t)
}

/// Frenada tras ir a la derecha (`right`) o a la izquierda: la
/// deceleración hasta pararse en `ACCEL_FRAMES` frames, guion por
/// pendiente y `+$20`. Original: `Slug_BrakeRight_02bd00`, `Slug_BrakeLeft_02be32`.
pub fn brake_enter(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let (scripts, byte_20, run) = if right {
        (L::BRAKE_RIGHT_SCRIPTS, L::BRAKE_RIGHT_BYTE_20, L::BRAKE_RIGHT_LOOP)
    } else {
        (L::BRAKE_LEFT_SCRIPTS, L::BRAKE_LEFT_BYTE_20, L::BRAKE_LEFT_LOOP)
    };
    let acc = (0u16.wrapping_sub(m.read_u16(t + I::VEL_X)) as i16) >> 3;
    m.write_u16(t + L::ACC_X, acc as u16);
    m.write_u8(t + L::ACCEL_TIMER, L::ACCEL_FRAMES);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(scripts.wrapping_add(u32::from(idx.wrapping_mul(4)) & 0xFFFF));
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, byte_20.wrapping_add(idx as u8));
    m.write_u32(t + T::HANDLER, run);
    brake(m, right)
}

/// Cuenta los frames de frenada; al acabar se queda parado y sigue en
/// [`brake_run`]. Original: `Slug_BrakeRight_02bd00+$48`, `Slug_BrakeLeft_02be32+$48`.
pub fn brake(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let n = m.read_u8(t + L::ACCEL_TIMER).wrapping_sub(1);
    m.write_u8(t + L::ACCEL_TIMER, n);
    if n == 0 {
        m.write_u16(t + I::VEL_X, 0);
        m.write_u16(t + L::ACC_X, 0);
        m.write_u32(t + T::HANDLER, if right { L::BRAKE_RIGHT_RUN } else { L::BRAKE_LEFT_RUN });
    }
    brake_run(m, right)
}

/// Frenando: a cuesta, parado al acabar el guion, de nuevo en crucero si
/// se empuja, y lo común. Original: `Slug_BrakeRight_02bd00+$5E`,
/// `Slug_BrakeLeft_02be32+$5E`.
pub fn brake_run(m: &mut impl Machine, right: bool) -> Flow {
    let t = m.task_register();
    let slope = if right { L::SLOPE_FROM_RIGHT } else { L::SLOPE_FROM_LEFT };
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    if !slug_hunker::physics(m, t) {
        m.write_u32(t + T::HANDLER, slope);
    }
    if slope_changed_to_hill(m, t) {
        m.write_u32(t + T::HANDLER, slope);
    }
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_FLAT);
    }
    hits::run_attack_list(m, t);
    if push_right(m, t) {
        m.write_u32(t + T::HANDLER, L::CRUISE_RIGHT);
    }
    if push_left(m, t) {
        m.write_u32(t + T::HANDLER, L::CRUISE_LEFT);
    }
    drive_tail(m, t)
}

/// Lo común al final de crucero, aceleración y frenada: agacharse, caer, saltar,
/// disparar, sin piloto, bajarse y los golpes.
fn drive_tail(m: &mut impl Machine, t: u32) -> Flow {
    if down_held(m, t) {
        let idx = slug::slope_anim_idx(m, t) & 0xFF;
        tasks::goto_by_table(m, t, L::CROUCH_BY_SLOPE, idx, None);
    }
    if slug::ground_contact(m, t).in_air {
        m.write_u32(t + T::HANDLER, L::FALL);
    }
    let dir = jump_dir(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::JUMP_BY_DIR, u16::from(dir), None);
    if can_fire(m, t) {
        m.write_u32(t + T::HANDLER, L::FIRE);
    }
    if left_alone(m, t) {
        m.write_u32(t + T::HANDLER, L::IDLE_ENTER_B);
    }
    if slug_hunker::eject_pressed(m, t) {
        m.write_u32(t + T::HANDLER, L::EJECT);
    }
    let side = slug_hunker::damage_check(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::HIT_HANDLERS, side, None);
    Flow::Return
}
