//! Jugador montado en el Slug: va pegado a él (un píxel más abajo), le pasa
//! su piloto y, según el mando, se baja saltando (B, A+B o abajo), pasa a
//! la otra pose (abajo + C) o muere si el Slug cae al vacío.
//!
//! Original: `Player_RideSlug_0364a2`, su bucle `$036508` y
//! `Player_RideSlug_Frame_03652e` (`asm/player/states_0342c4.s`),
//! `Slug_IsInAir_02abf0`, `Slug_IsReady_02abd2` y `Scene_NotLoaded_02a25c`
//! (`asm/slug/helpers_0295a6.s`).

use super::{anim, platforms, player, player_air, player_ground, slug_idle};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{player_ride as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn button(m: &mut impl Memory, t: u32, b: Buttons, edge: Edge) -> bool {
    input::pressed(m, Entity::from_raw(t), b, edge)
}

/// ¿Está el Slug en el aire? Original: `Slug_IsInAir_02abf0`.
pub fn slug_in_air(m: &mut impl Memory) -> bool {
    m.read_u8(L::SLUG + L::SLUG_8D) & L::SLUG_IN_AIR != 0
}

/// ¿Está el Slug listo (bit 4 de `+$8D`)? Original: `Slug_IsReady_02abd2`.
pub fn slug_ready(m: &mut impl Memory) -> bool {
    m.read_u8(L::SLUG + L::SLUG_8D) & L::SLUG_READY != 0
}

fn slug_destroyed(m: &mut impl Memory) -> bool {
    m.read_u8(L::SLUG + L::SLUG_13) & L::SLUG_DESTROYED != 0
}

/// ¿No hay escena cargada? Original: `$02A25C` (C = sí).
pub fn scene_not_loaded(m: &mut impl Memory) -> bool {
    m.read_u8(L::SCENE_LOADED) == 0
}

/// Entrada: sin brazo ni giro, caja de golpe, desplazamiento, pose y guion
/// de montado, quieto, y sigue en el bucle. Original: `Player_RideSlug_0364a2`.
pub fn enter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8C, L::CLEAR_8C, false);
    set_bits(m, t + L::FACING, L::FACING_LEFT, false);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX);
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, L::RIDE_OFFSET_Y);
    m.write_u8(t + L::STATE, L::STATE_RIDE);
    let pose = m.read_u32(L::SCRIPT - 4);
    m.write_u32(t + L::POSE, pose);
    m.write_u8(t + L::BYTE_21, 0xFF);
    anim::start(m, t, L::SCRIPT);
    for a in L::SPEEDS {
        m.write_u16(t + a, 0);
    }
    set_handler(m, t, L::LOOP);
    ride(m)
}

/// Bucle: paso común, con la escena, pegado al Slug y la animación.
/// Original: `Player_RideSlug_0364a2__L036508`.
pub fn ride(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common_no_prio(m, t);
    platforms::move_with_scene(m, t);
    let x = m.read_u16(L::SLUG + L::X);
    let y = m.read_u16(L::SLUG + L::Y).wrapping_add(1);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    player::anim_done(m, t);
    ride_frame(m, t);
    Flow::Return
}

fn jump_off(m: &mut impl Memory, t: u32) {
    set_handler(m, t, L::JUMP_OFF);
    let y = m.read_u16(t + L::Y).wrapping_add(L::JUMP_OFF_DY);
    m.write_u16(t + L::Y, y);
}

/// El resto del frame montado: avisa al Slug, le pasa el piloto y mira el
/// mando y la caída al vacío. Original: `Player_RideSlug_Frame_03652e`.
pub fn ride_frame(m: &mut impl Machine, t: u32) {
    slug_idle::mark_boarded(m);
    let pilot = m.read_u8(t + L::PILOT);
    m.write_u8(L::SLUG + L::PILOT, pilot);
    let mut eject = true;
    if !slug_in_air(m) && (button(m, t, Buttons::DOWN, Edge::Held) || slug_destroyed(m)) {
        if m.read_u8(t + L::FLAGS_8C) & L::BUSY == 0
            && button(m, t, Buttons::C, Edge::Pressed)
            && slug_ready(m)
            && !slug_destroyed(m)
        {
            set_handler(m, t, L::POSE2);
        }
        if button(m, t, Buttons::B, Edge::Pressed) && slug_ready(m) {
            jump_off(m, t);
            eject = false;
        }
    }
    if eject {
        if !slug_in_air(m)
            && input::all_pressed(m, Entity::from_raw(t), Buttons::A.or(Buttons::B), Edge::Pressed)
        {
            jump_off(m, t);
            m.write_u8(t + L::INVULN, L::EJECT_INVULN);
            m.write_u8(t + L::BYTE_59, L::EJECT_INVULN);
        }
        if scene_not_loaded(m) {
            set_handler(m, t, L::JUMP_OFF);
        }
    }
    player_ground::check_pit(m, t);
    player_air::publish_state(m, t);
}
