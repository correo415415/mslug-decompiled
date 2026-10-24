//! Escena de mensaje del final del tutorial y el diálogo de cargar la
//! tarjeta de memoria: si hay tarjeta pregunta si cargar (sí / no, con un
//! límite de tiempo); sin tarjeta, borra los récords guardados y se acaba.
//!
//! Original: `BootScr_Message_Prepare_001416`,
//! `BootScr_Message_WaitRun_001438` (`asm/boot/`), `$0981FC`, `$05162C`,
//! `$05164A`, `$044236`, `MemCard_LoadDialog_Init_098288`,
//! `MemCard_LoadDialog_Run_098308`, `MemCard_Dialog_ClearAndExit_0983b0`,
//! `MemCard_Dialog_ExitDelay_0983f6`, `MemCard_BiosRead_098578`,
//! `Fix_DrawGlyphList_09820e` e `Input_EdgePressedPlayers_098232`
//! (`asm/frontend/hiscore_memcard_options_09773c.s`).

use super::{hud, scene, script, scroll, tasks};
use crate::layout::{bios, memcard as L};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Preparación de la escena de mensaje. Original: `BootScr_Message_Prepare_001416`.
pub fn prepare(m: &mut impl Machine) -> Flow {
    m.call(bios::FIX_CLEAR);
    hud::clear_border(m);
    m.write_u8(L::MESSAGE_INDEX, 0);
    copy_start_keys(m);
    spawn_dialog(m);
    script::next(m)
}

/// Espera a que acabe el diálogo moviendo la escena de fondo.
/// Original: `BootScr_Message_WaitRun_001438`.
pub fn wait(m: &mut impl Machine) -> Flow {
    hud::draw(m);
    if m.read_u8(L::BUSY) != 0 {
        return Flow::Return;
    }
    idle_backdrop(m);
    script::next(m)
}

/// Copia las teclas de inicio de la BIOS. Original: `$0981FC`.
pub fn copy_start_keys(m: &mut impl Memory) {
    for k in 0..2 {
        let v = m.read_u8(L::START_KEYS_SRC + k);
        m.write_u8(L::START_KEYS + k, v);
    }
}

/// Crea el diálogo (marca la escena ocupada). Original: `$05162C`.
pub fn spawn_dialog(m: &mut impl Machine) {
    m.write_u8(L::BUSY, 1);
    let parent = m.task_register();
    let d = tasks::spawn(m, L::DIALOG, parent);
    m.write_u32(d + L::EXTRA, L::DIALOG_EXTRA);
}

/// Rutina extra del diálogo: carga la escena de fondo. Original: `$05164A`.
pub fn backdrop(m: &mut impl Machine) {
    scene::load_descriptor(m, L::BACKDROP);
    scroll::frame(m, (0, 0));
}

/// Escena quieta tras el diálogo. Original: `$044236`.
pub fn idle_backdrop(m: &mut impl Machine) {
    scene::load_descriptor(m, L::BACKDROP_IDLE);
    scroll::frame(m, (0, 0));
}

/// Lee la tarjeta de memoria en `SAVE`. Devuelve si ha fallado (no hay
/// tarjeta o no tiene datos). Original: `MemCard_BiosRead_098578` (C = error).
pub fn read(m: &mut impl Machine) -> bool {
    m.write_u8(L::CARD_COMMAND, L::CARD_READ);
    m.write_u16(L::CARD_MODE, L::CARD_MODE_GAME);
    m.write_u8(L::CARD_OPTION, 0);
    m.write_u32(L::CARD_BUFFER, L::SAVE);
    m.write_u16(L::CARD_SIZE, L::SAVE_LEN);
    let r = m.regs();
    m.call(bios::CARD);
    m.set_regs(r);
    m.read_u8(L::CARD_ERROR) != 0
}

/// Escribe una lista de glifos (2 × 2 tiles cada uno, terminada en
/// `GLYPH_END`) desde `at`. Original: `Fix_DrawGlyphList_09820e`.
pub fn draw_glyphs(m: &mut impl Memory, mut at: u16, list: u32) {
    let mut p = list;
    loop {
        let g = m.read_u16(p);
        if g == L::GLYPH_END {
            return;
        }
        hud::fill_tiles(m, at, g, 2, 2);
        at = at.wrapping_add(L::GLYPH_STEP);
        p += 2;
    }
}

/// Teclas de inicio recién pulsadas por los jugadores en juego (1 o 2).
/// Original: `Input_EdgePressedPlayers_098232`.
pub fn pressed_keys(m: &mut impl Memory, t: u32) -> u8 {
    let mut now = 0u8;
    for p in 0..2 {
        if m.read_u8(L::START_KEYS + p).wrapping_sub(1) < 2 {
            now |= m.read_u8(L::PLAYER_KEYS[p as usize]);
        }
    }
    let before = m.read_u8(t + L::KEYS);
    m.write_u8(t + L::KEYS, now);
    now & (now ^ before)
}

/// Sin tarjeta: borra los récords guardados y se acaba.
fn clear_and_exit(m: &mut impl Machine) -> Flow {
    for k in L::SAVE_SCORES {
        m.write_u8(L::SAVE + k, 0);
    }
    exit(m)
}

fn exit(m: &mut impl Machine) -> Flow {
    m.write_u8(L::BUSY, 0);
    tasks::finish(m)
}

/// Entrada del diálogo de carga. Original: `MemCard_LoadDialog_Init_098288`.
pub fn dialog(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if read(m) {
        return clear_and_exit(m);
    }
    if m.read_u32(t + L::EXTRA) == L::DIALOG_EXTRA {
        backdrop(m);
    } else if m.read_u32(t + L::EXTRA) != 0 {
        let mut r = m.regs();
        let extra = m.read_u32(t + L::EXTRA);
        m.call_original(extra, &mut r);
    }
    hud::fill_rect(m, L::FIX_ORIGIN, L::BLANK, L::BG_SIZE.0, L::BG_SIZE.1);
    for (at, text) in L::TEXTS {
        draw_glyphs(m, at, text);
    }
    m.write_u16(t + L::CHOICE, 0);
    pressed_keys(m, t);
    m.write_u16(t + L::TIMER, L::TIMEOUT);
    m.write_u32(t + crate::layout::task::HANDLER, L::RUN);
    dialog_run(m)
}

/// Pregunta: cursor parpadeante, arriba/abajo elige, A o el tiempo
/// confirman; «sí» lee la tarjeta y avisa. Original: `MemCard_LoadDialog_Run_098308`.
pub fn dialog_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let choice = m.read_u16(t + L::CHOICE);
    for (k, at) in L::CURSORS.into_iter().enumerate() {
        let i = u32::from((choice.wrapping_add(k as u16)) & 1) * 2;
        let g = m.read_u16(L::CURSOR_GLYPHS + i);
        hud::fill_tiles(m, at, g, 2, 2);
    }
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n != 0 {
        let keys = pressed_keys(m, t);
        if keys & L::KEY_A == 0 {
            if keys & L::KEY_UP != 0 {
                m.write_u16(t + L::CHOICE, 0);
            }
            if keys & L::KEY_DOWN != 0 {
                m.write_u16(t + L::CHOICE, 1);
            }
            return Flow::Return;
        }
    }
    hud::fill_rect(m, L::PROMPT, L::BLANK, L::PROMPT_SIZE.0, L::PROMPT_SIZE.1);
    if m.read_u16(t + L::CHOICE) != 0 || read(m) {
        return clear_and_exit(m);
    }
    draw_glyphs(m, L::DONE_TEXT.0, L::DONE_TEXT.1);
    m.write_u16(t + L::TIMER, L::DONE_DELAY);
    m.write_u32(t + crate::layout::task::HANDLER, L::EXIT_DELAY);
    exit_delay(m)
}

/// Espera tras cargar y se acaba. Original: `MemCard_Dialog_ExitDelay_0983f6`.
pub fn exit_delay(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        return exit(m);
    }
    Flow::Return
}
