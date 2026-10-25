//! Selector de guion del modo de prueba de la escena de mensaje: elige una
//! de las 16 listas de escenas (`TABLE`) según el índice guardado en la
//! tarjeta, la ejecuta y, al volver, enseña su rótulo en la capa fija; con
//! B pasa al siguiente rótulo y con A vuelve a lanzar la lista elegida.
//!
//! Original: `BootScr_MsgSelect_Dispatch_001452` (y sus sub-entradas
//! `$0014AE` / `$0014C4`), `BootMsg_AdvanceIndex_0014dc`,
//! `BootMsg_DrawRow_0014f4` (`asm/boot/boot_script_sound_palslots_000400.s`),
//! `MemCard_CopySlotFlag_051696` y `Fix_DrawMessageRow_05eae4`.

use super::{hud, inc_u16, rng, script, video};
use crate::entity::PlayerId;
use crate::input::{self, Buttons, Edge};
use crate::layout::{bios, msg_select as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Entrada del selector. Original: `BootScr_MsgSelect_Dispatch_001452`.
pub fn select(m: &mut impl Machine) -> Flow {
    inc_u16(m, L::SEED_COUNTER);
    let seed = m.read_u16(L::SEED_COUNTER);
    rng::seed(m, seed);
    copy_saved_index(m);
    if m.read_u8(L::INDEX) >= L::FIRST_LIMIT {
        m.write_u8(L::INDEX, L::FIRST_LIMIT - 1);
    }
    run_selected(m)
}

/// Lanza la lista del índice actual (fuera de rango: la 0).
/// Original: `$001482` (dentro de `BootScr_MsgSelect_Dispatch_001452`).
pub fn run_selected(m: &mut impl Machine) -> Flow {
    if m.read_u8(L::INDEX) >= L::COUNT {
        m.write_u8(L::INDEX, 0);
    }
    let e = entry(m);
    let list = m.read_u32(e);
    let t = m.task_register();
    m.write_u32(t + T::SCRIPT_CURSOR, list);
    script::dispatch(m)
}

fn entry(m: &mut impl Memory) -> u32 {
    L::TABLE + u32::from(m.read_u8(L::INDEX)) * L::ENTRY_LEN
}

/// Copia el índice guardado. Original: `MemCard_CopySlotFlag_051696`.
pub fn copy_saved_index(m: &mut impl Memory) {
    let v = m.read_u8(L::SAVED_INDEX);
    m.write_u8(L::INDEX, v);
}

/// Vuelta de la lista: borra la capa fija y enseña el primer rótulo.
/// Original: sub-entrada `$0014AE`.
pub fn show(m: &mut impl Machine) -> Flow {
    m.write_u8(L::INDEX, 0);
    m.call(bios::FIX_CLEAR);
    draw_row(m);
    script::next(m)
}

/// B: siguiente rótulo; A: lanza la lista elegida. Original: sub-entrada `$0014C4`.
pub fn browse(m: &mut impl Machine) -> Flow {
    if input::player_pad_pressed(m, PlayerId::P1, Buttons::B, Edge::Pressed) {
        advance(m);
    }
    if input::player_pad_pressed(m, PlayerId::P1, Buttons::A, Edge::Pressed) {
        return run_selected(m);
    }
    Flow::Return
}

/// Siguiente índice (módulo 16) y su rótulo. Original: `BootMsg_AdvanceIndex_0014dc`.
pub fn advance(m: &mut impl Memory) {
    let i = m.read_u8(L::INDEX).wrapping_add(1);
    m.write_u8(L::INDEX, if i >= L::COUNT { 0 } else { i });
    draw_row(m);
}

/// Rótulo del índice actual; las entradas sin rótulo se saltan.
/// Original: `BootMsg_DrawRow_0014f4`.
pub fn draw_row(m: &mut impl Memory) {
    let e = entry(m);
    let text = m.read_u32(e + L::TEXT);
    if text & 0x8000_0000 != 0 {
        return advance(m);
    }
    let row = m.read_u8(text);
    draw_message_row(m, row);
}

/// Dibuja la fila `row` de rótulos (paleta incluida).
/// Original: `Fix_DrawMessageRow_05eae4`.
pub fn draw_message_row(m: &mut impl Memory, row: u8) {
    video::load_high_entries(m, L::ROW_PALETTE);
    let mut at = L::ROW_ORIGIN;
    let mut p = m.read_u32(L::ROWS + u32::from(row) * 4);
    loop {
        let g = m.read_u16(p);
        if g == L::ROW_END {
            break;
        }
        hud::fill_tiles(m, at, (g | L::TILE_BANK) & L::TILE_MASK, 2, 2);
        p += 2;
        at = at.wrapping_add(L::ROW_STEP);
    }
    let (i, first, len, mode) = L::ROW_PALETTE_ANIM;
    video::start_high_entry_anim(m, i, first, len, mode);
}
