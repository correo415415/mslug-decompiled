//! Listas de la misión que se renuevan cada frame, antes de que las
//! rellenen las entidades: plataformas (dos copias que se alternan según
//! la paridad del frame), anillos de tres posiciones, una estela y un byte
//! por paridad. El frame del guion de scroll las llama al empezar.
//!
//! Original: `Platform_ClearListByScroll_077092`, `Rings_CompactAll_08f172`
//! (con `Ring_Compact_08f148`), `Trail_RingAdvance_0997cc`,
//! `PlayerSlot_ClearP3Cur_08f8e2`.

use crate::layout::lists as L;
use crate::mem::Memory;

fn odd_frame(m: &mut impl Memory) -> bool {
    m.read_u8(L::FRAME) & 1 != 0
}

/// Vacía la lista de plataformas de este frame. Original: `Platform_ClearListByScroll_077092`.
pub fn clear_platforms(m: &mut impl Memory) {
    let list = L::PLATFORMS[usize::from(odd_frame(m))];
    m.write_u16(list + L::PLATFORM_COUNT, 0);
}

/// Cada anillo pasa lo actual a anterior y lo nuevo a actual.
/// Original: `Rings_CompactAll_08f172`.
pub fn advance_rings(m: &mut impl Memory) {
    for r in L::RINGS {
        advance_ring(m, r);
    }
}

/// Original: `Ring_Compact_08f148`.
pub fn advance_ring(m: &mut impl Memory, r: u32) {
    let current = m.read_u16(r + 4);
    m.write_u16(r, current);
    let new = m.read_u16(r + 2);
    m.write_u16(r + 4, new);
    m.write_u16(r + 6, 0);
}

/// La estela avanza una posición. Original: `Trail_RingAdvance_0997cc`.
pub fn advance_trail(m: &mut impl Memory) {
    for i in 0..2 {
        let v = m.read_u16(L::TRAIL[i + 1]);
        m.write_u16(L::TRAIL[i], v);
    }
}

/// Borra el byte de la otra paridad. Original: `PlayerSlot_ClearP3Cur_08f8e2`.
pub fn clear_parity_byte(m: &mut impl Memory) {
    let i = u32::from(!odd_frame(m));
    m.write_u8(L::PARITY_TASK + L::PARITY_BYTE + i, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn rings_and_trail_shift() {
        let mut m = TestRam::default();
        let r = L::RINGS[0];
        for (i, v) in [1u16, 2, 3, 4].iter().enumerate() {
            m.write_u16(r + 2 * i as u32, *v);
        }
        advance_rings(&mut m);
        assert_eq!([m.read_u16(r), m.read_u16(r + 2), m.read_u16(r + 4), m.read_u16(r + 6)], [3, 2, 2, 0]);
        for (i, a) in L::TRAIL.iter().enumerate() {
            m.write_u16(*a, i as u16 + 10);
        }
        advance_trail(&mut m);
        assert_eq!(L::TRAIL.map(|a| m.read_u16(a)), [11, 12, 12]);
    }

    #[test]
    fn parity_picks_the_list() {
        let mut m = TestRam::default();
        m.write_u8(L::FRAME, 1);
        m.write_u16(L::PLATFORMS[1] + L::PLATFORM_COUNT, 5);
        m.write_u16(L::PLATFORMS[0] + L::PLATFORM_COUNT, 5);
        clear_platforms(&mut m);
        assert_eq!(m.read_u16(L::PLATFORMS[1] + L::PLATFORM_COUNT), 0);
        assert_eq!(m.read_u16(L::PLATFORMS[0] + L::PLATFORM_COUNT), 5);
        m.write_u8(L::PARITY_TASK + L::PARITY_BYTE, 9);
        clear_parity_byte(&mut m);
        assert_eq!(m.read_u8(L::PARITY_TASK + L::PARITY_BYTE), 0);
    }
}
