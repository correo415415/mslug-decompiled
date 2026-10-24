//! Slug conducido: el bucle principal mientras el jugador va montado.
//! Empuja con las flechas, salta, dispara el cañón, se agacha (abajo) o
//! queda sin piloto si el jugador se ha bajado.
//!
//! Original: `Slug_DriveAlt_02e9c8` (con la entrada `$02EA1C` y el bucle
//! `$02EA26`, `asm/slug/vehicle_02dd20.s`), `Slug_CallGroundProbeA_02a328`,
//! `Slug_CallGroundProbeB_02a34e`, `Slug_CheckFreeThenC_02a59a`,
//! `Slug_InputDirByLayoutA_02aaf0`, `Slug_CanFire_02aac0`, `$02AB86`,
//! `Slug_ConsumeField89_02ac80`, `ClearXN_02abc0` y `$02ABC6`
//! (`asm/slug/helpers_0295a6.s`).

use super::{anim, slug, slug_hunker, slug_idle, sound, tasks};
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
