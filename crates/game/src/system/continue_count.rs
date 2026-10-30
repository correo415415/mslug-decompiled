//! Pantalla de CONTINUE: la cuenta atrás grande del centro (9 → 0, con el
//! letrero «CONTINUE» y un tic por segundo; la cifra entra encogiéndose) y
//! las cifras pequeñas junto al «CONTINUE» de cada jugador. Si el jugador
//! pulsa START mientras puede continuar, la cifra pasa a la siguiente; al
//! acabar la cuenta, avisan a su madre.
//!
//! Original: `ContinueDigits_P1P2_Task_0466f6`, `ContinueBig_Init_0467f6`,
//! `ContinueBig_Countdown_046878`, `ContinueBig_ClearAndExit_04692e`,
//! `Continue_IsStart*_04694a…0469e0`
//! (`asm/enemies/vehicles/enemy46_drops_banners_046260.s`), los caracteres
//! grandes `$047822`/`$04784C` y `PalSlot_SetBit3_002c66`.

use super::{anim, hud, player_hud, sound, tasks};
use crate::layout::{continue_count as L, player_hud as H, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

/// Carácter grande (2×2) del byte `code` con la paleta `pal`.
/// Original: `$047822`.
pub fn big_glyph(m: &mut impl Memory, at: u16, code: u16, pal: u16) {
    let ch = ((code & L::GLYPH_HIGH) << 5) + L::GLYPH_BASE + ((code & L::GLYPH_LOW) << 1);
    hud::fill_tiles(m, at, ch | (pal << 12), 2, 2);
}

/// Texto de caracteres grandes (bytes hasta `END`). Original: `$04784C`.
pub fn big_text(m: &mut impl Memory, at: u16, text: u32, pal: u16) {
    let mut at = at;
    let mut p = text;
    loop {
        let c = m.read_u8(p);
        p += 1;
        if c == L::END {
            return;
        }
        big_glyph(m, at, u16::from(c), pal);
        at = at.wrapping_add(L::GLYPH_STEP);
    }
}

/// La entrada `k` de la tabla de sprites no se recoloca.
/// Original: `PalSlot_SetBit3_002c66`.
pub fn pin_sprite(m: &mut impl Memory, k: u16) {
    let a = sprite_entry(k);
    let v = m.read_u8(a);
    m.write_u8(a, v | L::SPRITE_FIXED);
}

/// Deshace [`pin_sprite`]. Original: `$002C76`.
pub fn unpin_sprite(m: &mut impl Memory, k: u16) {
    let a = sprite_entry(k);
    let v = m.read_u8(a);
    m.write_u8(a, v & !L::SPRITE_FIXED);
}

/// Entrada `k` de la tabla de sprites (el índice va con signo, como en el
/// original).
fn sprite_entry(k: u16) -> u32 {
    let off = k.wrapping_mul(L::SPRITE_SIZE) as i16;
    L::SPRITE_TABLE.wrapping_add(off as i32 as u32)
}

/// ¿Pulsa START el jugador `p` cuando aún puede continuar?
/// Original: `Continue_IsStartP1_04694a` / `Continue_IsStartP2_04698c` (C).
pub fn start_pressed(m: &mut impl Memory, t: u32, p: usize) -> bool {
    m.read_u8(L::BIOS_STATE[p]) == L::WAITING
        && m.read_u16(t + L::DIGIT) <= L::LAST_DIGIT
        && m.read_u8(L::PAD_EDGE[p]) & L::START_KEYS != 0
}

/// Original: `Continue_IsStartAny_0469ce` (C).
pub fn start_any(m: &mut impl Memory, t: u32) -> bool {
    start_pressed(m, t, 0) || start_pressed(m, t, 1)
}

/// Pasa la cuenta (al pulsar START, enseguida). Devuelve `None` si sigue
/// la misma cifra, `Some(true)` si cambia y `Some(false)` si se ha agotado
/// (y se lo dice a la madre).
fn count(m: &mut impl Memory, t: u32, pressed: bool) -> Option<bool> {
    if pressed {
        m.write_u16(t + L::TIMER, 0);
    }
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n as i16 > 0 {
        return None;
    }
    let d = m.read_u16(t + L::DIGIT).wrapping_sub(1);
    m.write_u16(t + L::DIGIT, d);
    if (d as i16) < 0 {
        let o = owner(m, t);
        m.write_u8(o + L::OWNER_EXPIRED, 0);
        return Some(false);
    }
    Some(true)
}

// --- cuenta grande

/// Original: `ContinueBig_Init_0467f6` (`player` = 0) y `+$A` (1).
pub fn big(m: &mut impl Machine, player: u16) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::PLAYER, player);
    m.write_u16(t + L::ATTR, L::BIG_ATTR);
    let f = m.read_u8(t + L::FLAGS);
    m.write_u8(t + L::FLAGS, f | L::FIXED_POS);
    anim::alloc_entry(m, t, L::BIG_GFX);
    let k = m.read_u16(t + L::SPRITE);
    pin_sprite(m, k);
    anim::start(m, t, L::BIG_SCRIPT);
    m.write_u16(t + L::X, L::BIG_POS.0);
    m.write_u16(t + L::Y, L::BIG_POS.1);
    m.write_u16(t + L::SUB, 0);
    m.write_u16(t + L::DIGIT, L::START_DIGIT);
    m.write_u16(t + L::TIMER, L::DIGIT_FRAMES);
    m.write_u8(t + L::PRESSED, 0xFF);
    big_text(m, L::SIGN_AT, L::SIGN_TEXT, L::SIGN_PALETTE);
    set_handler(m, t, L::BIG_RUN);
    sound::play(m, L::TICK_SOUND);
    big_run(m)
}

/// Cada frame de la cuenta grande. Original: `ContinueBig_Countdown_046878`.
pub fn big_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = owner(m, t);
    if m.read_u8(o + L::OWNER_ACTIVE) == 0 {
        return big_exit(m);
    }
    let pressed = start_any(m, t);
    if pressed {
        m.write_u8(t + L::PRESSED, 0xFF);
    }
    match count(m, t, pressed) {
        Some(false) => return big_exit(m),
        Some(true) => {
            sound::play(m, L::TICK_SOUND);
            m.write_u16(t + L::TIMER, L::DIGIT_FRAMES);
            let d = u32::from(m.read_u16(t + L::DIGIT));
            let script = m.read_u32(L::BIG_DIGITS + d * 4);
            anim::start(m, t, script);
        }
        None => {}
    }
    let timer = m.read_u16(t + L::TIMER);
    let scale = m.read_u8(L::BIG_SCALE + u32::from(timer));
    if (timer as i16) > L::BIG_SHRINK_FROM {
        let s = if m.read_u8(t + L::PRESSED) != 0 { 0xFF } else { scale };
        m.write_u8(t + L::SCALE, s);
    } else {
        m.write_u8(t + L::PRESSED, 0);
        m.write_u8(t + L::SCALE, scale);
    }
    anim::step(m, t);
    Flow::Return
}

/// Borra el letrero y acaba. Original: `ContinueBig_ClearAndExit_04692e`.
pub fn big_exit(m: &mut impl Machine) -> Flow {
    big_text(m, L::SIGN_AT, L::SIGN_CLEAR, L::SIGN_PALETTE);
    tasks::finish(m)
}

// --- cifras pequeñas

fn small_digit(m: &mut impl Memory, t: u32) {
    let at = m.read_u16(t + L::X);
    let ch = m.read_u16(t + L::DIGIT).wrapping_add(L::SMALL_CHAR);
    hud::put(m, at, ch);
    hud::put(m, at.wrapping_add(1), ch.wrapping_add(L::SMALL_ROW));
}

/// Original: `ContinueDigits_P1P2_Task_0466f6` (1P) y `+$1E` (2P).
pub fn small(m: &mut impl Machine, p: usize) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::PLAYER, p as u16);
    m.write_u16(t + L::X, L::SMALL_AT[p]);
    player_hud::clear_message(m, p);
    player_hud::draw_message(m, p, H::TEXT_CONTINUE);
    m.write_u16(t + L::DIGIT, L::START_DIGIT);
    m.write_u16(t + L::TIMER, L::DIGIT_FRAMES);
    small_digit(m, t);
    set_handler(m, t, L::SMALL_RUN);
    small_run(m)
}

/// Original: `ContinueDigits_P1P2_Task_0466f6+$78`.
pub fn small_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = owner(m, t);
    if m.read_u8(o + L::OWNER_ACTIVE) == 0 {
        return tasks::finish(m);
    }
    let p = usize::from(m.read_u16(t + L::PLAYER) != 0);
    let pressed = start_pressed(m, t, p);
    match count(m, t, pressed) {
        Some(false) => return tasks::finish(m),
        Some(true) => {
            small_digit(m, t);
            m.write_u16(t + L::TIMER, L::DIGIT_FRAMES);
        }
        None => {}
    }
    Flow::Return
}
