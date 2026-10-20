//! Cola de órdenes al driver de sonido (Z80).
//!
//! El juego no habla con el Z80 directamente: encola órdenes y en cada
//! vblank se envía una. Original: `InputQueue_InitAndPushOp4_00212E` (en
//! realidad es la cola de sonido), `Sound_PumpQueue_00226a`,
//! `Sound_PollReply_002242`.

use crate::layout::{bios_ram, hw, sound as L};
use crate::mem::Memory;

/// Vacía la cola y encola la orden de inicio.
/// Original: `InputQueue_InitAndPushOp4_00212E`.
pub fn init_queue(m: &mut impl Memory) {
    m.write_u32(L::QUEUE_READ, 0); // índices de lectura y escritura
    m.write_u8(L::REPLY, 0);
    m.write_u8(L::REPLY_SEEN, 0);
    m.write_u8(L::REPLY_AUX, 0);
    m.write_u8(L::LAST_SYSTEM_CMD, 0xFF);
    push(m, L::CMD_INIT);
}

/// Encola una orden. Las de sistema (`< FIRST_SOUND_CODE`) no se repiten
/// seguidas. Si la cola está llena, la orden se pierde.
/// Original: `$002152` (cola de `InputQueue_InitAndPushOp4`).
pub fn push(m: &mut impl Memory, cmd: u8) {
    if cmd < L::FIRST_SOUND_CODE {
        if cmd == m.read_u8(L::LAST_SYSTEM_CMD) {
            return;
        }
        m.write_u8(L::LAST_SYSTEM_CMD, cmd);
    }
    let w = m.read_u16(L::QUEUE_WRITE);
    m.write_u8(L::QUEUE + u32::from(w), cmd);
    let next = (w + 1) % L::QUEUE_LEN;
    if next == m.read_u16(L::QUEUE_READ) {
        // Llena: también se olvida la última orden de sistema.
        m.write_u8(L::LAST_SYSTEM_CMD, 0xFF);
    } else {
        m.write_u16(L::QUEUE_WRITE, next);
    }
}

/// Anula el sonido de moneda pendiente. Original: `$002126`.
pub fn cancel_coin_sound(m: &mut impl Memory) {
    m.write_u8(L::COIN_SOUND_REQUEST, 0);
}

/// La BIOS pide el sonido de moneda (vector `COIN_SOUND` de la cabecera);
/// se envía en el próximo vblank. Original: `$0022BE`.
pub fn request_coin_sound(m: &mut impl Memory) {
    m.write_u8(L::COIN_SOUND_REQUEST, 1);
}

/// Vblank: envía el sonido de moneda si se ha pedido o, si no, la siguiente
/// orden de la cola, y recoge la respuesta del Z80. Original: `Sound_PumpQueue_00226a`.
pub fn pump(m: &mut impl Memory) {
    if m.read_u8(L::COIN_SOUND_REQUEST) != 0 {
        m.write_u8(L::COIN_SOUND_REQUEST, 0);
        m.write_u8(hw::SOUND, hw::SOUND_CMD_COIN);
        return;
    }
    let reply = m.read_u8(hw::SOUND);
    m.write_u8(L::REPLY, reply);
    let r = m.read_u16(L::QUEUE_READ);
    if r != m.read_u16(L::QUEUE_WRITE) {
        let cmd = m.read_u8(L::QUEUE + u32::from(r));
        m.write_u8(hw::SOUND, cmd);
        m.write_u16(L::QUEUE_READ, (r + 1) % L::QUEUE_LEN);
    }
}

/// Respuesta nueva del driver (`0xFF` = ninguna). Las respuestas válidas
/// llevan el bit 7, y el valor va en los bits 1..6.
/// Original: `Sound_PollReply_002242`.
pub fn poll_reply(m: &mut impl Memory) -> u8 {
    let r = m.read_u8(L::REPLY);
    if r == m.read_u8(L::REPLY_SEEN) {
        return 0xFF;
    }
    m.write_u8(L::REPLY_SEEN, r);
    if r & 0x80 == 0 {
        return 0xFF;
    }
    (r - 0x80) >> 1
}

/// Pide el sonido `code` (byte alto = canal / tipo, bajo = sonido): se
/// encolan sus dos bytes. En la demo de la recreativa, sólo si el ajuste
/// `DEMO SOUND` está activo. Original: `$002352` (→ `$00219C`).
pub fn play(m: &mut impl Memory, code: u16) {
    let in_demo = m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_ATTRACT;
    if in_demo && m.read_u8(L::DEMO_SOUND) == 0 {
        return;
    }
    play_always(m, code);
}

/// Como [`play`] sin mirar la demo. Original: `$00219C`.
pub fn play_always(m: &mut impl Memory, code: u16) {
    if code >= L::CODE_LIMIT || (code as u8) < L::FIRST_SOUND_CODE {
        return;
    }
    let hi = code & 0xFF00;
    if hi == 0x100 || hi == 0x200 || hi == 0x300 {
        return;
    }
    if hi == 0 || hi >= L::MUSIC_FIRST {
        let mvs = m.read_u8(bios_ram::MVS_FLAG) != 0;
        if mvs
            && m.read_u8(L::MVS_DEMO_SOUND) != 0
            && m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_ATTRACT
        {
            return;
        }
    }
    if m.read_u8(L::SOUND_LOCK) != 0 && code >= L::MUSIC_FIRST {
        return;
    }
    push(m, (code >> 8) as u8);
    push(m, code as u8);
}

/// Para el sonido `code`. Original: `$002222`.
pub fn stop(m: &mut impl Memory, code: u16) {
    push(m, L::CMD_STOP);
    play_always(m, code);
}

/// Bloquea / desbloquea músicas y voces. Original: `$002230` / `$00223A`.
pub fn lock(m: &mut impl Memory, on: bool) {
    m.write_u8(L::SOUND_LOCK, if on { 0xFF } else { 0 });
}

/// Reinicia el driver / silencia todo. Original: `$0022C8` / `$0022CE`.
pub fn reset(m: &mut impl Memory) {
    push(m, L::CMD_RESET);
}
pub fn silence(m: &mut impl Memory) {
    push(m, L::CMD_SILENCE);
}

/// Ajuste de efectos (`$002308`) o de música (`$002312`): la orden y el
/// valor (que se codifica como sonido: `v + 0x20`, con `0xFF` si se pasa).
pub fn setting(m: &mut impl Memory, music: bool, v: u8) {
    push(m, if music { L::CMD_MUSIC_SETTING } else { L::CMD_EFFECT_SETTING });
    let (x, carry) = v.overflowing_add(L::FIRST_SOUND_CODE);
    push(m, if carry { 0xFF } else { x });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn queue_dedupes_system_cmds_and_drops_when_full() {
        let mut m = TestMachine::default();
        init_queue(&mut m);
        assert_eq!(m.read_u16(L::QUEUE_WRITE), 1);
        push(&mut m, L::CMD_INIT); // orden de sistema repetida: no se encola
        assert_eq!(m.read_u16(L::QUEUE_WRITE), 1);
        for _ in 0..40 {
            push(&mut m, 0x40);
        }
        assert_eq!(m.read_u16(L::QUEUE_WRITE), 31);
        pump(&mut m);
        assert_eq!(m.read_u16(L::QUEUE_READ), 1);
        assert_eq!(m.read_u8(hw::SOUND), L::CMD_INIT);
    }

    #[test]
    fn reply_is_reported_once() {
        let mut m = TestMachine::default();
        m.write_u8(L::REPLY, 0x86);
        assert_eq!(poll_reply(&mut m), 3);
        assert_eq!(poll_reply(&mut m), 0xFF);
    }
}
