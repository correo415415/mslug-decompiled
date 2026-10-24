//! Slug al montarse un jugador: se agacha (guion según la pendiente), crea
//! el brazo del jugador montado y espera a que acabe el guion para pasar a
//! la conducción. Mientras, puede caerse, el jugador puede bajarse (A+B) o
//! recibir golpes.
//!
//! Original: `Slug_Hunker_02e3b0` y su bucle `$02E42A`
//! (`asm/slug/vehicle_02dd20.s`), `Slug_PhysicsE_02a7d8`,
//! `Slug_TryStartDestroyedB_02a690`, `Slug_DamageTick_02a6be`,
//! `Slug_ResetHP_02a712`, `JsrAbsThunk_02a5cc` (→ `$05D00E`) y
//! `ClearXN_028992` (`asm/slug/helpers_0295a6.s`).

use super::{anim, fort, player, slug, slug_idle, sound, tasks};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{slug_idle as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

/// Entrada: guion de agacharse según la pendiente, `+$20`, el brazo del
/// jugador montado (pegado al Slug), caja de golpe de la espera, y sigue
/// en el bucle. Original: `Slug_Hunker_02e3b0`.
///
/// El original escribe antes en `+$48` el ataque de la tabla
/// `HUNKER_ATTACKS`, pero lo pisa enseguida con la caja de golpe.
pub fn enter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_6B, L::BIT_6B_0, true);
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5 | L::BIT_8C_1, false);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(L::HUNKER_SCRIPTS + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, L::HUNKER_BYTE_20.wrapping_add(idx as u8));
    let arm = tasks::spawn(m, L::ARM_OVERLAY, t);
    anim::copy_sprite_pos(m, t, arm);
    m.write_u32(t + L::HANDLER_60, L::NO_HANDLER);
    m.write_u32(t + L::HITBOX_CB, L::IDLE_HITBOX);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + T::HANDLER, L::HUNKER_LOOP);
    hunker(m)
}

/// Bucle agachado. Original: `Slug_Hunker_02e3b0__L02e42a`.
pub fn hunker(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    m.write_u16(t + L::VEL_X, 0);
    physics(m, t);
    m.write_u16(t + L::VEL_X, 0);
    slug::update_steering(m, t);
    if anim::step(m, t) {
        m.write_u32(t + L::HITBOX_CB, L::DRIVE_HITBOX);
        m.write_u32(t + L::HANDLER_60, L::DRIVE_HANDLER_60);
        m.write_u32(t + T::HANDLER, L::DRIVE);
    }
    if slug::ground_contact(m, t).in_air {
        m.write_u32(t + T::HANDLER, L::FALL);
    }
    if eject_pressed(m, t) {
        m.write_u32(t + T::HANDLER, L::EJECT);
    }
    let side = damage_check(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::HIT_HANDLERS, side, None);
    Flow::Return
}

/// Física con la Y de apoyo: tope contra paredes, bits de nivel, paso,
/// flechas y, con el bit 5 de `+$5A`, la Y de apoyo baja un poco.
/// Devuelve si las ruedas siguen a nivel (el original deja C = no).
/// Original: `Slug_PhysicsE_02a7d8`.
pub fn physics(m: &mut impl Machine, t: u32) -> bool {
    if let Some((_, dir)) = slug::wheel_against_wall(m, t) {
        slug::latch_x(m, dir as u16);
    }
    slug_idle::update_level_bits(m, t);
    slug_idle::step_on_base(m, t);
    player::dir_flags(m, t);
    if m.read_u8(t + L::FLAGS_5A) & L::BIT_5A_5 != 0 {
        let y = m.read_u16(t + L::BASE_Y);
        m.write_u16(t + L::BASE_Y, y.wrapping_add(L::BASE_Y_DROP));
    }
    slug_idle::wheels_level(m, t)
}

/// ¿Ha pulsado el piloto A y B a la vez? Original: `JsrAbsThunk_02a5cc`
/// (→ `$05D00E`).
pub fn eject_pressed(m: &mut impl Memory, t: u32) -> bool {
    input::all_pressed(m, Entity::from_raw(t), Buttons(L::EJECT_BUTTONS), Edge::Pressed)
}

/// Sin piloto y sin el temporizador de rango empieza la destrucción
/// (golpeado y destruido) y no hay golpe; si no, el tick de daño.
/// Devuelve el lado del golpe (1 o 2) si le han dado este frame.
/// Original: `Slug_TryStartDestroyedB_02a690` (C = golpe, lado en `d0`).
pub fn damage_check(m: &mut impl Memory, t: u32) -> Option<u16> {
    if m.read_u8(t + L::FLAGS_8D) & L::MANNED == 0 && m.read_u16(L::RANK_TIMER) == 0 {
        set_bits(m, t + L::FLAGS_13, L::HIT | L::DESTROYED, true);
        m.write_u32(t + T::HANDLER, L::DEATH_START);
        return None;
    }
    damage_tick(m, t)
}

/// Tick de daño: mientras es invulnerable bloquea a los jugadores este
/// frame; con la depuración la vida queda fija; si le han dado suena el
/// golpe. Original: `Slug_DamageTick_02a6be` / `Slug_ResetHP_02a712`.
pub fn damage_tick(m: &mut impl Memory, t: u32) -> Option<u16> {
    if m.read_u8(t + L::INVULNERABLE) != 0 {
        block_players(m, t);
    }
    let debug = m.read_u8(L::DEBUG);
    if debug & L::DEBUG_HP_RESET != 0 {
        m.write_u16(t + L::HP, L::HP_RESET);
        return None;
    }
    if debug & L::DEBUG_HP_BIG != 0 {
        m.write_u16(t + L::HP, L::HP_DEBUG);
    }
    let side = fort::hit_side(m, t)?;
    sound::play(m, L::HIT_SOUND);
    Some(side)
}

/// Bloquea a los jugadores en el byte de máscara del frame (`+$96` en
/// los impares, `+$97` en los pares). Original: `$08F6DA`.
pub fn block_players(m: &mut impl Memory, t: u32) {
    let k = u32::from(!m.read_u8(L::FRAME_COUNTER) & 1);
    m.write_u8(t + L::PLAYER_MASK + k, L::MASK_BLOCKED);
}
