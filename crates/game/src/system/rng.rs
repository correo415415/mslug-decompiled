//! Números aleatorios: una tabla de valores generada desde una semilla con
//! un generador congruencial. Original: `Rng_Seed_05e998`.

use crate::layout::rng as R;
use crate::mem::Memory;

/// Rellena la tabla desde `seed` y vuelve al principio.
pub fn seed(m: &mut impl Memory, seed: u16) {
    let mut v = seed;
    for i in 0..R::LEN {
        m.write_u16(R::TABLE + i * 2, v);
        v = v.wrapping_mul(R::MUL).wrapping_add(R::ADD);
    }
    m.write_u16(R::INDEX, 0);
}

/// Avanza el índice (de dos en dos, en 64 bytes) y mezcla.
fn step(m: &mut impl Memory, rotate: bool) -> (u32, u16) {
    let i = (m.read_u16(R::INDEX).wrapping_add(2)) & R::INDEX_MASK;
    m.write_u16(R::INDEX, i);
    let mut v = m.read_u16(R::TABLE + u32::from(i));
    if rotate {
        v = v.rotate_left(1);
    }
    let j = i.wrapping_sub(R::LAG) & R::INDEX_MASK;
    let a = R::TABLE + u32::from(j);
    let w = m.read_u16(a) ^ v;
    m.write_u16(a, w);
    (u32::from(j), w)
}

/// Siguiente número aleatorio (16 bits). Original: `$05E9B6`.
/// Devuelve también la posición mezclada (el original la deja en a4/d7).
pub fn next(m: &mut impl Memory) -> (u16, u16) {
    let (j, w) = step(m, false);
    (w, j as u16)
}

/// Número aleatorio en `0..n`. Original: `$05E9E4`.
pub fn below(m: &mut impl Memory, n: u16) -> (u16, u16) {
    let (j, w) = step(m, true);
    (((u32::from(w) * u32::from(n)) >> 16) as u16, j as u16)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn table_follows_lcg() {
        let mut m = TestMachine::default();
        seed(&mut m, 1);
        assert_eq!(m.read_u16(R::TABLE), 1);
        assert_eq!(m.read_u16(R::TABLE + 2), 0xB7C7u16.wrapping_add(0x81F5));
    }
}
