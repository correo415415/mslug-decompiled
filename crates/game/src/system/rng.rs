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
