//! Cola de órdenes al driver de sonido (Z80).
//!
//! El juego no habla con el Z80 directamente: encola órdenes y en cada
//! vblank se envía una. Original: `InputQueue_InitAndPushOp4_00212E` (en
//! realidad es la cola de sonido), `Sound_PumpQueue_00226a`,
//! `Sound_PollReply_002242`.

use crate::layout::{hw, sound as L};
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

/// Anula la petición de reiniciar el driver. Original: `$002126`.
pub fn cancel_reset(m: &mut impl Memory) {
    m.write_u8(L::RESET_REQUEST, 0);
}

/// Pide reiniciar el driver en el próximo vblank. Original: `$0022BE`.
pub fn request_reset(m: &mut impl Memory) {
    m.write_u8(L::RESET_REQUEST, 1);
}

/// Vblank: envía la siguiente orden de la cola (o el reinicio, si se ha
/// pedido) y recoge la respuesta del Z80. Original: `Sound_PumpQueue_00226a`.
pub fn pump(m: &mut impl Memory) {
    if m.read_u8(L::RESET_REQUEST) != 0 {
        m.write_u8(L::RESET_REQUEST, 0);
        m.write_u8(hw::SOUND, hw::SOUND_CMD_RESET);
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
