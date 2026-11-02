//! Mandos del frame y pausa.
//!
//! En cada frame se copian los mandos que ha leído la BIOS a los mandos de
//! trabajo, calculando los botones recién pulsados y la repetición
//! automática (para los menús). Con una demo en marcha los botones salen
//! de la demo (o se graban en ella).
//!
//! La demo es una lista de bytes: cada uno es una racha (frames sin
//! cambios); el byte de un cambio lleva en los 4 bits altos el botón que
//! cambia (0–7 mando 1, 8–15 mando 2) y en los bajos su racha (hasta 15).
//!
//! Original: `Input_ReadFrame_05CC1A`, `Input_Poll_LiveMode_025066`,
//! `Input_ReplayFlag_025118` (`$02511A`), `InputRec_*` (`$025282`…`$025432`),
//! `Pause_Poll_013c3c`, `Pause_Active_013cae`.

use crate::layout::{bios_ram, pads as P, pause as Z};
use crate::mem::{Machine, Memory};

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

/// Mandos del frame con demo: graba o reproduce un frame y actualiza los
/// mandos de trabajo con sus botones; con la demo acabada, a cero.
/// Original: `$02511A`.
pub fn read_demo(m: &mut impl Memory) {
    let mut held = [0u8; 2];
    if m.read_u8(P::DEMO_MODE) == P::DEMO_PLAYING {
        if m.read_u8(P::DEMO_CLEAR_FLAG) != 0 {
            record_frame(m);
        } else {
            playback_frame(m);
        }
        for k in 0..2 {
            held[k] = m.read_u8(P::DEMO_HELD[k]);
        }
    }
    for k in 0..2 {
        update_pad(m, P::WORK[k], held[k]);
    }
}

/// Cierra la racha actual en el búfer `buf`. Original: `InputRec_FlushRun_025366`.
pub fn flush_run(m: &mut impl Memory, buf: u32) {
    let pos = m.read_u16(P::DEMO_POS);
    if u32::from(pos) >= P::DEMO_BUFFER_LEN {
        return;
    }
    let run = m.read_u8(P::DEMO_RUN);
    m.write_u8(buf + u32::from(pos), run);
    m.write_u8(P::DEMO_RUN, 0);
    m.write_u16(P::DEMO_POS, pos + 1);
}

/// Apunta que ha cambiado el botón `bit` (0–15): cierra la racha y guarda
/// el botón en los 4 bits altos del byte del último cambio.
/// Original: `InputRec_StoreBitToggle_025390`.
pub fn store_toggle(m: &mut impl Memory, buf: u32, bit: u8) {
    flush_run(m, buf);
    let at = m.read_u16(P::DEMO_TOGGLE_POS);
    if u32::from(at) >= P::DEMO_BUFFER_LEN {
        return;
    }
    let a = buf + u32::from(at);
    let v = (bit << 4) | (m.read_u8(a) & 0x0F);
    m.write_u8(a, v);
    let pos = m.read_u16(P::DEMO_POS);
    m.write_u16(P::DEMO_TOGGLE_POS, pos);
}

/// Graba un frame: los cambios de botones respecto al anterior.
/// Original: `InputRec_RecordFrame_025282`.
pub fn record_frame(m: &mut impl Memory) {
    let buf = m.read_u32(P::DEMO_PTR);
    let limit = if m.read_u16(P::DEMO_TOGGLE_POS) == m.read_u16(P::DEMO_POS) {
        P::DEMO_RUN_SHARED
    } else {
        P::DEMO_RUN_FULL
    };
    if limit <= m.read_u8(P::DEMO_RUN) {
        flush_run(m, buf);
    }
    let mut changed = 0u16;
    for (k, bios) in [P::BIOS_P1, P::BIOS_P2].into_iter().enumerate() {
        let now = m.read_u8(bios);
        let before = m.read_u8(P::DEMO_HELD[k]);
        m.write_u8(P::DEMO_HELD[k], now);
        changed |= u16::from(now ^ before) << (8 * k);
    }
    for bit in (0..16u8).rev() {
        if changed & (1 << bit) != 0 {
            store_toggle(m, buf, bit);
        }
    }
    let run = m.read_u8(P::DEMO_RUN).wrapping_add(1);
    m.write_u8(P::DEMO_RUN, run);
    if u32::from(m.read_u16(P::DEMO_POS)) >= P::DEMO_BUFFER_LEN {
        for a in P::DEMO_HELD {
            m.write_u8(a, 0);
        }
        m.write_u8(P::DEMO_MODE, P::DEMO_DONE);
    }
}

/// Siguiente cambio de botón de la demo si toca en este frame (la racha
/// guardada coincide con los frames contados). Al pasar del búfer, la demo
/// se acaba. Original: `InputRec_ReadNextRun_0253be` (C = hay cambio, en `d0`).
pub fn read_next_run(m: &mut impl Memory, buf: u32) -> Option<u8> {
    let mut pos = m.read_u16(P::DEMO_POS);
    if u32::from(pos) >= P::DEMO_BUFFER_LEN {
        m.write_u8(P::DEMO_MODE, P::DEMO_DONE);
        return None;
    }
    let mut run = m.read_u8(buf + u32::from(pos));
    let mut full = P::DEMO_RUN_FULL;
    if pos == m.read_u16(P::DEMO_TOGGLE_POS) {
        full = P::DEMO_RUN_SHARED;
        run &= full;
    }
    if run != m.read_u8(P::DEMO_RUN) {
        m.write_u16(P::DEMO_POS, pos);
        return None;
    }
    m.write_u8(P::DEMO_RUN, 0);
    pos += 1;
    if run == full {
        // Racha llena: sigue otra salvo que venga un 0.
        if m.read_u8(buf + u32::from(pos)) != 0 {
            m.write_u16(P::DEMO_POS, pos);
            return None;
        }
        pos += 1;
    }
    m.write_u16(P::DEMO_POS, pos);
    let at = m.read_u16(P::DEMO_TOGGLE_POS);
    m.write_u16(P::DEMO_TOGGLE_POS, pos);
    Some(m.read_u8(buf + u32::from(at)) >> 4)
}

/// Reproduce un frame: aplica todos los cambios que tocan y cuenta el
/// frame. Original: `InputRec_PlaybackFrame_02531e`.
pub fn playback_frame(m: &mut impl Memory) {
    loop {
        let buf = m.read_u32(P::DEMO_PTR);
        let Some(b) = read_next_run(m, buf) else { break };
        let b = b & 0x0F;
        let k = usize::from(b & P::DEMO_P2_BIT != 0);
        let v = m.read_u8(P::DEMO_HELD[k]) ^ (1 << (b & 7));
        m.write_u8(P::DEMO_HELD[k], v);
    }
    let run = m.read_u8(P::DEMO_RUN).wrapping_add(1);
    m.write_u8(P::DEMO_RUN, run);
}

/// Mandos del frame: lectura, copia a los mandos de cada jugador, botones
/// de inicio y mandos "en juego". Original: `$05CC1A`.
pub fn read_frame(m: &mut impl Machine) {
    if m.read_u8(P::DEMO_MODE) == 0 {
        read_live(m);
    } else {
        read_demo(m);
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


/// Mandos de trabajo a cero y, si `demo` es 0 o 1, empieza a reproducir
/// (o grabar) esa demo. Original: `PlayerCtx_ResetTwoBlocks_024FEC`.
pub fn start_demo(m: &mut impl Memory, demo: u16) {
    for p in P::WORK {
        super::fill(m, p, 6, 0);
    }
    if demo >= P::DEMO_COUNT {
        m.write_u8(P::DEMO_MODE, 0);
        return;
    }
    m.write_u8(P::DEMO_MODE, 1);
    let ptr = m.read_u32(P::DEMO_TABLE + u32::from(demo) * 4);
    m.write_u32(P::DEMO_PTR, ptr);
    for a in P::DEMO_WORDS {
        m.write_u16(a, 0);
    }
    m.write_u8(P::DEMO_BYTE, 0);
    if m.read_u8(P::DEMO_CLEAR_FLAG) != 0 {
        super::fill(m, ptr, P::DEMO_BUFFER_LEN, 0xFF);
    }
}

/// Permite o no la pausa en la escena actual.
/// Original: `SetRamByteFF_013d20` / `$013D2A`.
pub fn set_pause_enabled(m: &mut impl Memory, on: bool) {
    m.write_u8(Z::ENABLED, if on { 0xFF } else { 0 });
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
        super::sound::play(m, Z::SOUND_PAUSE);
        return true;
    }
    if pressed {
        m.write_u8(Z::ACTIVE, 0);
        pause_text(m, false);
        super::sound::stop(m, Z::SOUND_PAUSE);
        super::sound::play(m, Z::SOUND_UNPAUSE);
        return false;
    }
    let t = m.read_u8(Z::BLINK);
    m.write_u8(Z::BLINK, t.wrapping_add(1));
    match t & Z::BLINK_PERIOD_MASK {
        0 => {
            pause_text(m, true);
        }
        Z::BLINK_HIDE_AT => {
            pause_text(m, false);
        }
        _ => {}
    }
    true
}

/// Escribe "PAUSE" en la capa fija o lo borra.
/// Original: `$013D46` / `$013D3E`.
pub fn pause_text(m: &mut impl Memory, show: bool) {
    let mut p = if show { Z::TEXT_SHOW } else { Z::TEXT_HIDE };
    let mut at = Z::TEXT_AT;
    loop {
        let c = m.read_u8(p);
        if c == 0 {
            return;
        }
        m.write_u32(crate::layout::hw::VRAM_ADDR, (u32::from(at) << 16) | u32::from(Z::TEXT_ATTR | u16::from(c)));
        at = at.wrapping_add(Z::TEXT_COLUMN);
        p += 1;
    }
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
