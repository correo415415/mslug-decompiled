//! Brazo del jugador montado en el Slug: una tarea hija que dibuja al
//! jugador asomando por la escotilla mientras el Slug se agacha. Sigue al
//! Slug con un desplazamiento según su inclinación y se acaba con su guion.
//!
//! Original: `SlugCannon_ArmOverlay_03d944`, `$03DA2A`, `$03DA4C`
//! (`asm/player/fire_shells_03c62a.s`), `Slug_IsRiddenByPlayer_02ac0e`,
//! `Slug_TestField100609_02ac4c` y `$02FF8E`
//! (`asm/slug/helpers_0295a6.s`, `asm/slug/states_02ae3e.s`).

use super::{anim, platforms, slug, sound, tasks};
use crate::layout::{slug_arm as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// ¿Lleva el Slug al jugador `p` (0 o 1) y se puede montar? Si sí, la
/// tarea `t` copia su piloto. Original: `Slug_IsRiddenByPlayer_02ac0e`.
pub fn ridden_by(m: &mut impl Memory, t: u32, p: usize) -> bool {
    if m.read_u8(L::SLUG + L::CONTROLLER) != p as u8 + 1 {
        return false;
    }
    take_pilot(m, t, L::PLAYERS[p])
}

/// Si se puede montar, `t` copia el piloto de `player`.
/// Original: `Slug_TestField100609_02ac4c` (C = sí).
pub fn take_pilot(m: &mut impl Memory, t: u32, player: u32) -> bool {
    if m.read_u8(L::RIDE_ENABLED) == 0 {
        return false;
    }
    let pilot = m.read_u8(player + L::PILOT);
    m.write_u8(t + L::PILOT, pilot);
    true
}

/// Inclinación del Slug convertida en índice 0..=`$40`. Original: `$02FF8E`.
pub fn slope_index(m: &mut impl Memory) -> u16 {
    let s = m.read_u16(L::SLUG + L::SLOPE) as i16;
    let d = 0x20i16.wrapping_sub(s >> 1) as u16;
    d.min(0x40)
}

/// Entrada: según qué jugador lleve el Slug, sus gráficos, el guion de su
/// personaje y el desplazamiento; si no lo lleva nadie, se acaba.
/// Original: `SlugCannon_ArmOverlay_03d944`.
pub fn enter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let Some(p) = (0..2).find(|&p| ridden_by(m, t, p)) else {
        return tasks::finish(m);
    };
    for g in L::GFX[p] {
        anim::alloc_entry(m, t, g);
    }
    m.write_u16(t + L::TIMER_1C, L::TIMER_1C_INIT[p]);
    slug::add_timer(m, t);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, if p == 1 { f | L::PLAYER2 } else { f & !L::PLAYER2 });
    let k = u32::from(m.read_u8(L::PLAYERS[p] + L::CHARACTER)) + p as u32 * L::SCRIPTS_PER_PLAYER;
    let script = m.read_u32(L::SCRIPTS + k * 4);
    anim::start(m, t, script);
    let i = u32::from(slope_index(m)) * 4;
    let (dx, dy) = (m.read_u16(L::OFFSETS + i), m.read_u16(L::OFFSETS + i + 2));
    m.write_u16(t + L::OFFSET_X, dx);
    m.write_u16(t + L::OFFSET_Y, dy);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Bucle: se mueve con la escena, se pega al Slug y anima; al acabar el
/// guion suena y se acaba el frame siguiente. Original: `$03DA4C`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let x = m.read_u16(L::SLUG + L::X).wrapping_add(m.read_u16(t + L::OFFSET_X));
    let y = m.read_u16(L::SLUG + L::Y).wrapping_add(m.read_u16(t + L::OFFSET_Y));
    let attr = m.read_u16(L::SLUG + L::ATTR);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    m.write_u16(t + L::ATTR, (attr & L::ATTR_KEEP) | L::ATTR_SET);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::DONE);
        sound::play(m, L::DONE_SOUND);
    }
    Flow::Return
}
