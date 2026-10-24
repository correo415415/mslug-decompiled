//! Seno y coseno con las tablas de la ROM.
//!
//! Original: `$013C0E`.

use crate::layout::trig as L;
use crate::mem::Memory;

/// Vector de longitud `r` con el ángulo `angle` (1/256 de vuelta):
/// (`r`·cos, `r`·sin) en 1/256, como `d1` y `d2` del original.
pub fn polar(m: &mut impl Memory, angle: u16, r: i16) -> (i32, i32) {
    let i = u32::from(angle.wrapping_mul(2));
    let c = m.read_u16(L::COS.wrapping_add(i as i16 as u32)) as i16;
    let s = m.read_u16(L::SIN.wrapping_add(i as i16 as u32)) as i16;
    ((i32::from(r) * i32::from(c)) >> 8, (i32::from(r) * i32::from(s)) >> 8)
}
