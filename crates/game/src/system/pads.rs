//! Mandos del frame y pausa.
//!
//! En cada frame se copian los mandos que ha leído la BIOS a los mandos de
//! trabajo, calculando los botones recién pulsados y la repetición
//! automática (para los menús). Con una demo grabada en marcha, la lectura
//! la hace la rutina de demo original (aún sin portar).
//!
//! Original: `Input_ReadFrame_05CC1A`, `Input_Poll_LiveMode_025066`,
//! `Pause_Poll_013c3c`, `Pause_Active_013cae`.

use crate::layout::{bios_ram, pads as P, pause as Z};
use crate::mem::{Machine, Memory, Regs};

/// Actualiza un mando de trabajo con los botones mantenidos de este frame.
fn update_pad(m: &mut impl Memory, pad: u32, held: u8) {
    let prev = m.read_u8(pad + P::PAD_HELD);
    m.write_u8(pad + P::PAD_PREV, prev);
    m.write_u8(pad + P::PAD_HELD, held);
    let pressed = (held ^ prev) & held;
    m.write_u8(pad + P::PAD_PRESSED, pressed);
    if pressed != 0 {
        // Pulsación nueva: cuenta como repetición y arranca el retardo.
        m.write_u8(pad + P::PAD_REPEAT, pressed);
        let first = m.read_u8(P::REPEAT_FIRST);
        m.write_u8(pad + P::PAD_REPEAT_TIMER, first);
        return;
    }
    let t = m.read_u8(pad + P::PAD_REPEAT_TIMER).wrapping_sub(1);
    m.write_u8(pad + P::PAD_REPEAT_TIMER, t);
    if t == 0 {
        // Se mantiene: repite todos los botones mantenidos.
        m.write_u8(pad + P::PAD_REPEAT, held);
        let next = m.read_u8(P::REPEAT_NEXT);
        m.write_u8(pad + P::PAD_REPEAT_TIMER, next);
    } else {
        m.write_u8(pad + P::PAD_REPEAT, 0);
    }
}

/// Lee los mandos del frame (sin demo). Original: `Input_Poll_LiveMode_025066`.
pub fn read_live(m: &mut impl Memory) {
    let p1 = m.read_u8(P::BIOS_P1);
    update_pad(m, P::WORK[0], p1);
    let p2 = m.read_u8(P::BIOS_P2);
    update_pad(m, P::WORK[1], p2);
}

/// Mandos del frame: lectura, copia a los mandos de cada jugador, botones
/// de inicio y mandos "en juego". Original: `$05CC1A`.
pub fn read_frame(m: &mut impl Machine) {
    if m.read_u8(P::DEMO_MODE) == 0 {
        read_live(m);
    } else {
        let mut r = m.regs();
        m.call_original(P::DEMO_READ, &mut r);
        m.set_regs(r);
    }
    for (work, live) in P::WORK.into_iter().zip(crate::layout::input::PAD_LIVE) {
        super::copy(m, work, live, 6);
    }
    let prev = m.read_u8(P::START_HELD);
    let now = m.read_u8(P::BIOS_START);
    m.write_u8(P::START_HELD, now);
    m.write_u8(P::START_PRESSED, (now ^ prev) & now);
    for k in 0..2 {
        match m.read_u8(P::BIOS_PLAYER_STATE[k]) {
            P::BIOS_PLAYER_OFF => {}
            P::BIOS_PLAYER_GAME_OVER => super::fill(m, P::PAD_INGAME[k], 6, 0),
            _ => super::copy(m, crate::layout::input::PAD_LIVE[k], P::PAD_INGAME[k], 6),
        }
    }
    // El original deja a0 apuntando al estado de los jugadores de la BIOS.
    let mut r = m.regs();
    r.a[0] = P::BIOS_PLAYER_STATE[0];
    m.set_regs(r);
}

/// Pide un sonido a la rutina original (aún sin portar).
fn sound(m: &mut impl Machine, routine: u32, code: u16) {
    let mut r: Regs = m.regs();
    r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(code);
    m.call_original(routine, &mut r);
    m.set_regs(r);
}

/// Pausa con el botón de inicio de un jugador que esté jugando (sólo en
/// consola). Devuelve `true` mientras el juego está en pausa.
/// Original: `Pause_Poll_013c3c` / `Pause_Active_013cae`.
pub fn poll_pause(m: &mut impl Machine) -> bool {
    if m.read_u8(bios_ram::MVS_FLAG) != 0 || m.read_u8(Z::ENABLED) == 0 {
        return false;
    }
    let mut buttons = 0u8;
    for k in 0..2 {
        if m.read_u8(P::BIOS_PLAYER_STATE[k]) == P::BIOS_PLAYER_PLAYING {
            buttons |= Z::START_BIT[k];
        }
    }
    let pressed = m.read_u8(P::START_PRESSED) & buttons != 0;
    if m.read_u8(Z::ACTIVE) == 0 {
        if !pressed {
            return false;
        }
        m.write_u8(Z::ACTIVE, 0xFF);
        m.write_u8(Z::BLINK, 0);
        sound(m, Z::SOUND_REQUEST, Z::SOUND_PAUSE);
        return true;
    }
    if pressed {
        m.write_u8(Z::ACTIVE, 0);
        m.call(Z::HIDE_TEXT);
        sound(m, Z::SOUND_STOP, Z::SOUND_PAUSE);
        sound(m, Z::SOUND_REQUEST, Z::SOUND_UNPAUSE);
        return false;
    }
    let t = m.read_u8(Z::BLINK);
    m.write_u8(Z::BLINK, t.wrapping_add(1));
    match t & Z::BLINK_PERIOD_MASK {
        0 => {
            m.call(Z::SHOW_TEXT);
        }
        Z::BLINK_HIDE_AT => {
            m.call(Z::HIDE_TEXT);
        }
        _ => {}
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn pressed_and_autorepeat() {
        let mut m = TestMachine::default();
        m.write_u8(P::REPEAT_FIRST, 2);
        m.write_u8(P::REPEAT_NEXT, 1);
        m.write_u8(P::BIOS_P1, 0x10);
        read_live(&mut m);
        let pad = P::WORK[0];
        assert_eq!(m.read_u8(pad + P::PAD_PRESSED), 0x10);
        assert_eq!(m.read_u8(pad + P::PAD_REPEAT), 0x10);
        read_live(&mut m);
        assert_eq!(m.read_u8(pad + P::PAD_PRESSED), 0);
        assert_eq!(m.read_u8(pad + P::PAD_REPEAT), 0);
        read_live(&mut m);
        assert_eq!(m.read_u8(pad + P::PAD_REPEAT), 0x10);
    }

    #[test]
    fn no_pause_on_arcade() {
        let mut m = TestMachine::default();
        m.write_u8(bios_ram::MVS_FLAG, 1);
        m.write_u8(Z::ENABLED, 0xFF);
        m.write_u8(P::START_PRESSED, 0xFF);
        assert!(!poll_pause(&mut m));
    }
}
