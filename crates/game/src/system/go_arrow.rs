//! Indicador «GO» de la capa fija: si el scroll lleva un rato quieto, no
//! hay enemigos, hay algún jugador en juego y la cámara aún no ha llegado
//! al tope de la escena, parpadea dos veces para decir que hay que
//! avanzar. Tiene tres dibujos según la variante (`VARIANT`).
//!
//! Original: `Cut_Watcher_Init_08c864`, `Cut_Watcher_Run_08c880`,
//! `Cut_Watcher_Play_08c8fa`, `Cut_InstallListByVariant_08c820`
//! (`asm/mission/cutscene_anim_08ba0c.s`) y `$05E1AA`.

use super::{hud, scroll, sound, tasks};
use crate::layout::{go_arrow as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Jugadores en juego como máscara (bit 0 = 1P, bit 1 = 2P).
/// Original: `$05E1AA` (C = ninguno).
pub fn players_mask(m: &mut impl Memory) -> u16 {
    (0..2).filter(|&p| tasks::player_task_if_alive(m, p).is_some()).map(|p| 1 << p).sum()
}

/// Original: `Cut_Watcher_Init_08c864`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::CELL, L::ARROW_CELL);
    m.write_u8(L::BLINKS, 0);
    m.write_u16(t + L::LAST_PROGRESS, 0xFFFF);
    m.write_u16(t + L::QUIET, 0);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Original: `Cut_Watcher_Run_08c880`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let progress = m.read_u16(L::PROGRESS);
    if progress != m.read_u16(t + L::LAST_PROGRESS) {
        m.write_u16(t + L::LAST_PROGRESS, progress);
        m.write_u16(t + L::QUIET, L::QUIET_FRAMES);
    } else if m.read_u16(t + L::QUIET) != 0 {
        let q = m.read_u16(t + L::QUIET) - 1;
        m.write_u16(t + L::QUIET, q);
    } else if (m.read_u16(L::ENEMIES[0]) | m.read_u16(L::ENEMIES[1])) == 0
        && players_mask(m) != 0
        && !scroll::edge_reached(m)
        && m.read_u8(L::SCENE_LOADED) != 0
    {
        m.write_u16(t + L::QUIET, L::QUIET_FRAMES);
        m.write_u8(L::BLINKS, L::BLINK_COUNT);
    }
    if m.read_u8(L::BLINKS) == 0 {
        return Flow::Return;
    }
    m.write_u32(t + T::HANDLER, L::PLAY);
    play(m)
}

/// Dibujo según la variante. Original: `Cut_InstallListByVariant_08c820`.
pub fn start_anim(m: &mut impl Memory, t: u32) {
    let list = match m.read_u8(L::VARIANT) {
        2 => L::ANIMS[2],
        1 => L::ANIMS[1],
        _ => L::ANIMS[0],
    };
    hud::fix_anim_start(m, t, list);
}

/// Original: `Cut_Watcher_Play_08c8fa`.
pub fn play(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    start_anim(m, t);
    sound::play(m, L::SOUND);
    m.write_u32(t + T::HANDLER, L::PLAY_RUN);
    play_run(m)
}

/// Al acabar cada parpadeo, otro; tras el último, a vigilar.
/// Original: `$08C90E`.
pub fn play_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !hud::fix_anim_step(m, t) {
        return Flow::Return;
    }
    let n = m.read_u8(L::BLINKS).wrapping_sub(1);
    m.write_u8(L::BLINKS, n);
    if n != 0 {
        m.write_u32(t + T::HANDLER, L::PLAY);
        return Flow::Return;
    }
    hud::fix_anim_erase(m, t);
    m.write_u32(t + T::HANDLER, L::RUN);
    Flow::Return
}
