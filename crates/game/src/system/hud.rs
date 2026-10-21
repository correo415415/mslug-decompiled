//! Textos fijos de la capa fija: créditos y nivel de dificultad, y el
//! marco que se borra al empezar cada escena.
//!
//! Original: `Hud_DrawCreditsAndOverlay_001db8`,
//! `FixOverlay_DrawCreditsOrFree_052050`, `FixOverlay_PutCreditsLine_0520e2`,
//! `FixOverlay_ClearCreditsArea_0521b2`, `$05223A` (nivel),
//! `FixLayer_QuadBatch_046AC6`, `Fix_BlitRect_05DA9C`.

use crate::layout::{bios_ram, hud as H, hw};
use crate::layout::fix_anim as FA;
use crate::mem::Memory;

/// Escribe un carácter (con sus atributos) en la capa fija.
fn put(m: &mut impl Memory, at: u16, ch: u16) {
    m.write_u16(hw::VRAM_ADDR, at);
    m.write_u16(hw::VRAM_RW, ch);
}

/// `n` caracteres en blanco seguidos desde `at`.
fn blank(m: &mut impl Memory, at: u16, n: u16) {
    for k in 0..n {
        put(m, at + k * H::COLUMN, H::ATTR | u16::from(H::BLANK));
    }
}

fn arcade(m: &mut impl Memory) -> bool {
    m.read_u8(bios_ram::MVS_FLAG) != 0
}

/// Rellena un rectángulo de la capa fija con `ch` (sólo dentro de la zona
/// visible). Original: `Fix_BlitRect_05DA9C` (a1, d0, d1, d2).
pub fn fill_rect(m: &mut impl Memory, at: u16, ch: u16, cols: u16, rows: u16) {
    for r in 0..rows {
        for c in 0..cols {
            let a = at.wrapping_add(r).wrapping_add(c.wrapping_mul(H::COLUMN));
            if (H::FIX_FIRST..=H::FIX_LAST).contains(&a) {
                put(m, a, ch);
            }
        }
    }
}

/// Bloque de caracteres consecutivos: `ch` en la primera columna, +1 en
/// cada columna y +`TILE_ROW_STEP` en cada fila (sólo dentro de la zona
/// visible). Original: `ThunkTarget_05da56` (a1, d0, d1, d2).
pub fn fill_tiles(m: &mut impl Memory, at: u16, ch: u16, cols: u16, rows: u16) {
    for r in 0..rows {
        let row_ch = ch.wrapping_add(r.wrapping_mul(crate::layout::fix::TILE_ROW_STEP));
        for c in 0..cols {
            let a = at.wrapping_add(r).wrapping_add(c.wrapping_mul(H::COLUMN));
            if (H::FIX_FIRST..=H::FIX_LAST).contains(&a) {
                put(m, a, row_ch.wrapping_add(c));
            }
        }
    }
}

/// Texto de la ROM en `text` desde `at` con los atributos `attr`
/// (`TEXT_NEWLINE` = fila siguiente, `TEXT_END` = fin).
/// Original: `Fix_BlitStream_05DAD8` (a1, a2, d0). Devuelve dónde acabó el
/// texto en la ROM.
pub fn draw_text(m: &mut impl Memory, at: u16, text: u32, attr: u16) -> u32 {
    use crate::layout::fix::{TEXT_END, TEXT_NEWLINE};
    let (mut row, mut pos, mut p) = (at, at, text);
    loop {
        let c = m.read_u8(p);
        p += 1;
        match c {
            TEXT_END => return p,
            TEXT_NEWLINE => {
                row = row.wrapping_add(1);
                pos = row;
            }
            _ => {
                if (H::FIX_FIRST..=H::FIX_LAST).contains(&pos) {
                    put(m, pos, attr | u16::from(c));
                }
                pos = pos.wrapping_add(H::COLUMN);
            }
        }
    }
}

/// Borra el marco de la pantalla. Original: `FixLayer_QuadBatch_046AC6`.
pub fn clear_border(m: &mut impl Memory) {
    for (at, cols, rows) in H::BORDER {
        fill_rect(m, at, u16::from(H::BLANK), cols, rows);
    }
}

/// Una línea "CREDITS nn" en `at` con los créditos BCD de `credits`.
/// Original: `FixOverlay_PutCreditsLine_0520e2`.
fn credits_line(m: &mut impl Memory, at: u16, credits: u32) {
    let n = m.read_u8(credits);
    let mut text = *H::CREDITS_TEXT;
    if n < 2 {
        text[6] = H::BLANK;
    }
    let digits = [b'0' + (n >> 4), b'0' + (n & 0xF)];
    for (k, &ch) in text.iter().chain(digits.iter()).enumerate() {
        put(m, at + k as u16 * H::COLUMN, H::ATTR | u16::from(ch));
    }
}

/// Créditos. Original: `FixOverlay_DrawCreditsOrFree_052050`.
pub fn draw_credits(m: &mut impl Memory) {
    if arcade(m) && m.read_u8(H::BIOS_HIDE) & H::HIDE_CREDITS != 0 {
        return;
    }
    let flags = m.read_u8(H::FLAGS);
    if flags & H::CLEAR_CREDITS != 0 {
        for (at, n) in H::CREDITS_CLEAR {
            blank(m, at, n);
        }
        m.write_u8(H::FLAGS, flags & !(H::DRAW_CREDITS | H::CLEAR_CREDITS));
        return;
    }
    if flags & H::DRAW_CREDITS == 0 {
        return;
    }
    for (at, n) in H::CREDITS_AREA {
        blank(m, at, n);
    }
    let mut credits = if !arcade(m) {
        H::CREDITS_AES
    } else if m.read_u16(H::CREDITS_MVS_VALID) != 0 {
        H::CREDITS_MVS
    } else {
        H::CREDITS_BACKUP
    };
    // Con una sola ranura (o en consola) hay un contador por jugador.
    if !arcade(m) || m.read_u8(H::BIOS_SLOTS) == 1 {
        credits_line(m, H::CREDITS_P1, credits);
        credits += 1;
    }
    credits_line(m, H::CREDITS_P2, credits);
}

/// Nivel de dificultad (sólo recreativa). Original: `$05223A`.
pub fn draw_level(m: &mut impl Memory) {
    if !arcade(m) || m.read_u8(H::BIOS_HIDE) & H::HIDE_LEVEL != 0 {
        return;
    }
    let flags = m.read_u8(H::FLAGS);
    if flags & H::CLEAR_LEVEL != 0 {
        blank(m, H::LEVEL_POS, H::LEVEL_CLEAR);
        m.write_u8(H::FLAGS, flags & !(H::DRAW_LEVEL | H::CLEAR_LEVEL));
        return;
    }
    if flags & H::DRAW_LEVEL == 0 {
        return;
    }
    for (k, &t) in H::LEVEL_TILES.iter().enumerate() {
        put(m, H::LEVEL_POS + k as u16 * H::COLUMN, H::ATTR | u16::from(t));
    }
    let level = m.read_u8(bios_ram::SOFT_DIP_DIFFICULTY).wrapping_add(b'1');
    put(m, H::LEVEL_POS + 6 * H::COLUMN, H::ATTR | u16::from(level));
}

/// Lo de cada frame en las escenas: créditos y nivel.
/// Original: `Hud_DrawCreditsAndOverlay_001db8`.
pub fn draw(m: &mut impl Memory) {
    m.write_u8(H::FLAGS, H::EVERY_FRAME);
    draw_credits(m);
    draw_level(m);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn border_stays_inside_fix_layer() {
        let mut m = TestMachine::default();
        fill_rect(&mut m, 0x74E0, 0x20, 2, 1);
        // La segunda columna (0x7500) queda fuera: la última escrita es 0x74E0.
        assert_eq!(m.read_u16(hw::VRAM_ADDR), 0x74E0);
    }
}

/// Animación de la capa fija de la tarea `t`: lista de cuadros de 8 bytes
/// {primer tile (`$FFFF` = fin; 0 = salta a la lista de `+2`), columnas,
/// filas, frames} dibujada en la celda `+$22`. Arranca la lista `list`.
/// Original: `$05DB58`.
pub fn fix_anim_start(m: &mut impl Memory, t: u32, list: u32) {
    m.write_u32(t + FA::CURSOR, list);
    let n = m.read_u16(list + FA::FRAMES);
    m.write_u16(t + FA::LEFT, n);
    fix_anim_draw(m, t);
}

/// Dibuja el cuadro actual. Original: `$05DBC2`.
pub fn fix_anim_draw(m: &mut impl Memory, t: u32) {
    let f = m.read_u32(t + FA::CURSOR);
    let at = m.read_u16(t + FA::CELL);
    let (ch, cols, rows) = (m.read_u16(f), m.read_u16(f + 2), m.read_u16(f + 4));
    fill_tiles(m, at, ch, cols, rows);
}

/// Borra el cuadro actual. Original: `$05DBDC`.
pub fn fix_anim_erase(m: &mut impl Memory, t: u32) {
    let f = m.read_u32(t + FA::CURSOR);
    if m.read_u16(f) == FA::END {
        return;
    }
    let at = m.read_u16(t + FA::CELL);
    let (cols, rows) = (m.read_u16(f + 2), m.read_u16(f + 4));
    fill_rect(m, at, FA::BLANK, cols, rows);
}

/// Un frame de la animación; devuelve si ha acabado. Original: `$05DB6A`.
pub fn fix_anim_step(m: &mut impl Memory, t: u32) -> bool {
    let cur = m.read_u32(t + FA::CURSOR);
    if m.read_u16(cur) == FA::END {
        return true;
    }
    let n = m.read_u16(t + FA::LEFT).wrapping_sub(1);
    m.write_u16(t + FA::LEFT, n);
    if n as i16 > 0 {
        return false;
    }
    fix_anim_erase(m, t);
    let f = m.read_u32(t + FA::CURSOR) + FA::FRAME_LEN;
    m.write_u32(t + FA::CURSOR, f);
    match m.read_u16(f) {
        0 => {
            let next = m.read_u32(f + 2);
            fix_anim_start(m, t, next);
            false
        }
        FA::END => true,
        _ => {
            let n = m.read_u16(f + FA::FRAMES);
            m.write_u16(t + FA::LEFT, n);
            fix_anim_draw(m, t);
            false
        }
    }
}
