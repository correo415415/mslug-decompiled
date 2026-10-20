//! Pantalla de título: el logo (cuatro sprites animados), el copyright, el
//! mensaje parpadeante según los créditos y la cuenta atrás de la
//! recreativa.
//!
//! Cada pieza es una tarea hija de la tarea del logo (una corrutina: la
//! rutina de la tarea cambia al pasar de una fase a otra).
//!
//! Original: `$091330` y siguientes (en la decompilación se llaman
//! `Continue_*`, pero es la tarea del título que crea `AttractHandler_00109C`)
//! y `$052328` (estado de créditos).

use super::{anim, hud, tasks};
use crate::layout::{bios, bios_ram, credits, title as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Primera vuelta de la tarea del logo: empieza en la siguiente.
/// Original: `TaskTpl_091330`.
pub fn begin(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::START);
    Flow::Return
}

/// Coloca la tarea del logo y crea los cuatro sprites del logo (en su
/// sitio).
fn spawn_sprites(m: &mut impl Machine, t: u32) {
    for (handler, ..) in L::SPRITES {
        let child = tasks::spawn(m, handler, t);
        anim::copy_sprite_pos(m, t, child);
    }
}

fn place(m: &mut impl Memory, t: u32) {
    m.write_u16(t + anim_x(), L::POS.0);
    m.write_u16(t + anim_y(), L::POS.1);
    for off in L::STATE {
        m.write_u8(t + off, 0);
    }
}

fn anim_x() -> u32 {
    crate::layout::anim::X
}

fn anim_y() -> u32 {
    crate::layout::anim::Y
}

/// Título completo: cuenta atrás, mensaje, logo y copyright.
/// Original: `Continue_Spawn_091338`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t);
    tasks::spawn(m, L::COUNTDOWN, t);
    tasks::spawn(m, L::TEXT, t);
    spawn_sprites(m, t);
    draw_copyright(m);
    m.write_u32(t + T::HANDLER, L::IDLE);
    Flow::Return
}

/// Sólo el logo (pantalla de continuar). Original: `Continue_Tpl_0913ac`.
pub fn start_logo_only(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t);
    spawn_sprites(m, t);
    m.write_u32(t + T::HANDLER, L::IDLE);
    Flow::Return
}

/// La tarea del logo ya no hace nada (sus hijas siguen). Original: `$0913AA`.
pub fn idle(_: &mut impl Machine) -> Flow {
    Flow::Return
}

/// Sprite `k` del logo: gráficos, posición, atributos y guion de
/// animación; después se anima cada frame.
/// Original: `Continue_Sprite_Left_09148c`, `_Right_09144a`, `_Mid_091408`,
/// `_MidB_0914ce`.
pub fn sprite(m: &mut impl Machine, k: usize) -> bool {
    let t = m.task_register();
    let (_, gfx, x, y, attr, script) = L::SPRITES[k];
    anim::alloc_entry(m, t, gfx);
    m.write_u16(t + anim_x(), x);
    m.write_u16(t + anim_y(), y);
    anim::set_attr(m, t, attr);
    let a = m.read_u16(t + T::SPRITE_ATTR);
    m.write_u16(t + T::SPRITE_ATTR, a & T::SPRITE_ATTR_KEEP);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, L::SPRITE_ANIM);
    animate(m)
}

/// Un frame de la animación del sprite. Devuelve el flag C.
/// Original: `JsrAbsThunk_09150c`.
pub fn animate(m: &mut impl Machine) -> bool {
    let t = m.task_register();
    anim::step(m, t)
}

/// "©1996 NAZCA" (o lo que diga la ROM) en la capa fija.
/// Original: `Continue_Text_DrawTitle_091618`.
pub fn draw_copyright(m: &mut impl Memory) {
    hud::draw_text(m, L::COPYRIGHT_POS, L::COPYRIGHT_TEXT, L::COPYRIGHT_ATTR);
}

/// Tarea del mensaje: empieza oculto. Original: `Continue_Text_Init_091514`.
pub fn text(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::BLINK_TIMER, 1);
    m.write_u16(t + L::BLINK_PERIOD_FIELD, L::BLINK_PERIOD);
    m.write_u16(t + L::BLINK_AUX, 0);
    m.write_u32(t + T::HANDLER, L::TEXT_HIDDEN);
    text_hidden(m)
}

/// Cuenta un frame del parpadeo; al acabar el periodo pasa a `next`.
fn blink_step(m: &mut impl Machine, next: u32) {
    let t = m.task_register();
    let n = m.read_u16(t + L::BLINK_TIMER).wrapping_sub(1);
    m.write_u16(t + L::BLINK_TIMER, n);
    if n == 0 {
        let p = m.read_u16(t + L::BLINK_PERIOD_FIELD);
        m.write_u16(t + L::BLINK_TIMER, p);
        m.write_u32(t + T::HANDLER, next);
    }
}

/// Mensaje oculto: borra su zona. Original: `Continue_Text_Clear_09152c`.
pub fn text_hidden(m: &mut impl Machine) -> Flow {
    hud::fill_rect(m, L::MESSAGE_POS, L::MESSAGE_BLANK, L::MESSAGE_COLS, 2);
    blink_step(m, L::TEXT_SHOWN);
    Flow::Return
}

/// Mensaje visible: el que toca según los créditos.
/// Original: `Continue_Text_Blink_091558`.
pub fn text_shown(m: &mut impl Machine) -> Flow {
    let s = credit_state(m);
    draw_message(m, s);
    blink_step(m, L::TEXT_HIDDEN);
    Flow::Return
}

/// Estado de créditos: en consola `credits::CONSOLE`; en recreativa
/// pregunta a la BIOS si cada jugador tiene crédito (`credits::P1`/`P2`).
/// Original: `$052328`.
pub fn credit_state(m: &mut impl Machine) -> u16 {
    if m.read_u8(bios_ram::MVS_FLAG) == 0 {
        return credits::CONSOLE;
    }
    for (addr, v) in bios_ram::CREDIT_REQUEST.into_iter().zip([1u8, 1, 0, 0]) {
        m.write_u8(addr, v);
    }
    m.call(bios::CREDIT_CHECK);
    let mut s = 0;
    if m.read_u8(bios_ram::CREDIT_REQUEST[0]) != 0 {
        s |= credits::P1;
    }
    if m.read_u8(bios_ram::CREDIT_REQUEST[1]) != 0 {
        s |= credits::P2;
    }
    s
}

/// Dibuja el mensaje del estado `state`: una lista de la ROM con el primer
/// carácter de cada columna (bloques de 1×2). Original:
/// `Continue_Text_DrawRows_0915dc`.
pub fn draw_message(m: &mut impl Memory, state: u16) {
    use crate::layout::fix::LIST_END;
    let list = m.read_u32(L::MESSAGES + u32::from(state.wrapping_mul(4)));
    let mut at = L::MESSAGE_POS;
    let mut p = list;
    loop {
        let ch = m.read_u16(p);
        if ch == LIST_END {
            return;
        }
        hud::fill_tiles(m, at, ch, 1, 2);
        at = at.wrapping_add(crate::layout::hud::COLUMN);
        p += 2;
    }
}

/// Tarea de la cuenta atrás: rótulo y después las cifras cada frame.
/// Original: `Continue_Text_Countdown_091578`.
pub fn countdown(m: &mut impl Machine) -> Flow {
    hud::draw_text(m, L::TIMER_LABEL_POS, L::TIMER_LABEL, L::TIMER_ATTR);
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::COUNTDOWN_TICK);
    countdown_tick(m)
}

/// Las dos cifras (BCD) del temporizador de la BIOS. Original: `$091594`.
pub fn countdown_tick(m: &mut impl Machine) -> Flow {
    let v = m.read_u8(bios_ram::COMPULSION_TIMER);
    for (at, digit) in L::TIMER_DIGITS.into_iter().zip([v >> 4, v & 0xF]) {
        hud::fill_tiles(m, at, L::TIMER_ATTR | u16::from(b'0' + digit), 1, 1);
    }
    Flow::Return
}

/// Tarea del rótulo grande del título en el modo alterno: dos bloques de
/// caracteres; deja de contar como marcada y sigue como el mensaje.
/// Original: `Continue_Text_DrawCredits_091630`.
pub fn banner(m: &mut impl Machine) -> Flow {
    for (at, ch) in L::BANNER {
        hud::fill_tiles(m, at, ch, L::BANNER_SIZE.0, L::BANNER_SIZE.1);
    }
    let t = m.task_register();
    let f = m.read_u8(t + T::FLAGS);
    m.write_u8(t + T::FLAGS, f & !T::FLAGS_MARKED);
    m.write_u32(t + T::HANDLER, L::TEXT);
    Flow::Return
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn message_blinks_every_period() {
        let mut m = TestMachine::default();
        let t = 0x10_1000;
        m.write_u8(bios_ram::MVS_FLAG, 0);
        // Mensaje de consola: una columna.
        m.write_u32(L::MESSAGES + 4 * u32::from(credits::CONSOLE), 0x20_0000);
        m.write_u16(0x20_0000, 0x1234);
        m.write_u16(0x20_0002, 0xFFFF);
        m.set_task_register(t);
        text(&mut m);
        // Timer 1: al primer frame pasa a visible.
        assert_eq!(m.read_u32(t + T::HANDLER), L::TEXT_SHOWN);
        for _ in 0..L::BLINK_PERIOD - 1 {
            text_shown(&mut m);
            assert_eq!(m.read_u32(t + T::HANDLER), L::TEXT_SHOWN);
        }
        text_shown(&mut m);
        assert_eq!(m.read_u32(t + T::HANDLER), L::TEXT_HIDDEN);
    }

    #[test]
    fn arcade_asks_the_bios_for_credits() {
        let mut m = TestMachine::default();
        m.write_u8(bios_ram::MVS_FLAG, 1);
        let s = credit_state(&mut m);
        assert_eq!(m.calls, vec![bios::CREDIT_CHECK]);
        // La máquina de prueba no cambia la petición: los dos tienen crédito.
        assert_eq!(s, credits::P1 | credits::P2);
    }
}
