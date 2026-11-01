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
//!
//! También la escena de guardar al acabar la partida: si hay tarjeta
//! pregunta si grabar (por defecto «no») y la graba con la BIOS.
//! Original: `BootScr_MemCardSave_001526`, `BootScr_WaitHudDirty_001544`
//! (`asm/boot/`), `MemCard_DetectAndSave_051660`, `MemCard_Detect_09826e`,
//! `MemCard_SaveDialog_Init_09840c`, `MemCard_SaveDialog_Run_098482`,
//! `MemCard_BiosWrite_0985ca` y `MemCard_BiosCreate_09866a`.

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

/// Llama a la rutina `routine` de la BIOS con el mismo marco de pila que
/// el original (dirección de vuelta, 15 registros guardados y `extra`
/// bytes más): la BIOS guarda el puntero de pila en su RAM.
fn call_bios(m: &mut impl Machine, routine: u32, extra: u32) {
    let r = m.regs();
    let sp = m.stack_pointer();
    m.set_stack_pointer(sp - L::CARD_FRAME - extra);
    m.call(routine);
    m.set_stack_pointer(sp);
    m.set_regs(r);
}

/// Prepara una orden de la tarjeta (modo de juego, sin opción).
fn card_command(m: &mut impl Memory, command: u8) {
    m.write_u8(L::CARD_COMMAND, command);
    m.write_u16(L::CARD_MODE, L::CARD_MODE_GAME);
    m.write_u8(L::CARD_OPTION, 0);
}

/// Lee la tarjeta de memoria en `SAVE`. Devuelve si ha fallado (no hay
/// tarjeta o no tiene datos). Original: `MemCard_BiosRead_098578` (C = error).
pub fn read(m: &mut impl Machine) -> bool {
    card_command(m, L::CARD_READ);
    m.write_u32(L::CARD_BUFFER, L::SAVE);
    m.write_u16(L::CARD_SIZE, L::SAVE_LEN);
    call_bios(m, bios::CARD, 0);
    m.read_u8(L::CARD_ERROR) != 0
}

/// Crea el fichero del juego en la tarjeta. Original:
/// `MemCard_BiosCreate_09866a`.
pub fn create(m: &mut impl Machine) {
    card_command(m, L::CARD_CREATE);
    call_bios(m, bios::CARD, 0);
}

/// Graba `SAVE` (con el nombre del fichero delante) en la tarjeta; si
/// falla, borra la pantalla y deja que la BIOS muestre el error. Devuelve
/// si sigue fallando. Original: `MemCard_BiosWrite_0985ca` (C = error).
pub fn write(m: &mut impl Machine) -> bool {
    card_command(m, L::CARD_WRITE);
    for k in 0..L::FILE_NAME_LEN {
        let b = m.read_u8(L::FILE_NAME + k);
        m.write_u8(L::SAVE + k, b);
    }
    m.write_u32(L::CARD_BUFFER, L::SAVE);
    m.write_u16(L::CARD_SIZE, L::SAVE_LEN);
    call_bios(m, bios::CARD, 0);
    if m.read_u8(L::CARD_ERROR) == 0 {
        return false;
    }
    hud::fill_rect(m, L::FIX_ORIGIN, L::BLANK, L::BG_SIZE.0, L::BG_SIZE.1);
    let mode = m.read_u8(L::CARD_ERROR_MODE);
    m.write_u8(L::CARD_ERROR_MODE, 0);
    call_bios(m, bios::CARD_ERROR, L::CARD_ERROR_FRAME);
    m.write_u8(L::CARD_ERROR_MODE, mode);
    m.read_u8(L::CARD_ERROR) != 0
}

/// ¿Hay una tarjeta en la ranura? Original: `MemCard_Detect_09826e`
/// (C = sí).
pub fn present(m: &mut impl Memory) -> bool {
    m.read_u8(L::CARD_STATUS) & L::CARD_ABSENT == 0
}

/// Escribe una lista de glifos (2 × 2 tiles cada uno, terminada en
/// `GLYPH_END`) desde `at`. Devuelve dónde acaba (posición y lista tras
/// la marca). Original: `Fix_DrawGlyphList_09820e`.
pub fn draw_glyphs(m: &mut impl Memory, mut at: u16, list: u32) -> (u16, u32) {
    let mut p = list;
    loop {
        let g = m.read_u16(p);
        if g == L::GLYPH_END {
            return (at, p + 2);
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

/// Escena de guardar al acabar la partida: borra la pantalla, marca la
/// escena ocupada y, si hay tarjeta, abre el diálogo. Original:
/// `BootScr_MemCardSave_001526`.
pub fn save_scene(m: &mut impl Machine) -> Flow {
    m.call(bios::FIX_CLEAR);
    hud::clear_border(m);
    m.write_u8(L::BUSY, 1);
    detect_and_save(m);
    script::next(m)
}

/// Espera a que acabe el diálogo de guardar. Original:
/// `BootScr_WaitHudDirty_001544`.
pub fn save_scene_wait(m: &mut impl Machine) -> Flow {
    hud::draw(m);
    if m.read_u8(L::BUSY) == 0 {
        return script::next(m);
    }
    Flow::Return
}

/// Con tarjeta: carga la escena de fondo, marca los datos pendientes y
/// lanza el diálogo de guardar (que devuelve); sin ella, la escena ya ha
/// acabado. Original: `MemCard_DetectAndSave_051660`.
pub fn detect_and_save(m: &mut impl Machine) -> Option<u32> {
    if !present(m) {
        m.write_u8(L::BUSY, 0);
        return None;
    }
    m.write_u8(L::BUSY, 1);
    backdrop(m);
    m.write_u8(L::SAVE_PENDING, 0xFF);
    let parent = m.task_register();
    Some(tasks::spawn(m, L::SAVE_DIALOG, parent))
}

/// Entrada del diálogo de guardar: si no hay tarjeta se acaba; si no,
/// dibuja la pregunta con «no» elegido y un límite de tiempo; la pregunta
/// empieza a correr en el frame siguiente.
/// Original: `MemCard_SaveDialog_Init_09840c`.
pub fn save_dialog(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !present(m) {
        return exit(m);
    }
    hud::fill_rect(m, L::FIX_ORIGIN, L::BLANK, L::BG_SIZE.0, L::BG_SIZE.1);
    for (at, text) in L::SAVE_TEXTS {
        draw_glyphs(m, at, text);
    }
    m.write_u16(t + L::CHOICE, L::SAVE_CHOICE);
    m.write_u16(t + L::TIMER, L::TIMEOUT);
    // A diferencia del de cargar, el bucle empieza en el frame siguiente.
    m.write_u32(t + crate::layout::task::HANDLER, L::SAVE_RUN);
    Flow::Return
}

/// Pregunta de guardar: como la de cargar; «sí» abre la ranura, crea el
/// fichero, lo graba y avisa. Original: `MemCard_SaveDialog_Run_098482`.
pub fn save_dialog_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let choice = m.read_u16(t + L::CHOICE);
    for (at, c) in L::CURSORS.into_iter().zip([choice, choice ^ 1]) {
        let g = m.read_u16(L::CURSOR_GLYPHS + u32::from(c) * 2);
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
    if m.read_u16(t + L::CHOICE) != 0 {
        return exit(m);
    }
    for r in L::CARD_UNLOCK {
        m.write_u8(r, 0);
    }
    create(m);
    if write(m) {
        return exit(m);
    }
    draw_glyphs(m, L::SAVE_DONE_TEXT.0, L::SAVE_DONE_TEXT.1);
    m.write_u16(t + L::TIMER, L::DONE_DELAY);
    m.write_u32(t + crate::layout::task::HANDLER, L::SAVE_DELAY);
    exit_delay(m)
}
