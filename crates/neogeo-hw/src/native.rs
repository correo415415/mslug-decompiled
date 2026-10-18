//! Port: tabla de funciones nativas.
//!
//! Cada función portada a Rust se registra con la dirección de entrada de la
//! rutina 68000 original. Cuando el PC llega a esa dirección, `Cpu::step`
//! llama a la función Rust en vez de ejecutar las instrucciones de la ROM.
//!
//! Contrato 1:1 (verificado por el modo diferencial de `mslug`): al volver,
//! registros D0-D7/A0-A7, SR (incluidos X N Z V C), PC, memoria y ciclos
//! consumidos son exactamente los que habría producido el código 68000.

use crate::cpu::m68k::{Bus, Cpu};
use std::collections::HashMap;

/// Firma: recibe CPU y bus "dinámico", devuelve los ciclos 68000 consumidos.
pub type NativeFn = fn(&mut Cpu, &mut DynBus) -> u32;
/// Bus visto por las funciones nativas.
pub type DynBus<'a> = &'a mut dyn Bus;

#[derive(Default)]
pub struct NativeTable {
    map: HashMap<u32, NativeFn>,
}

impl NativeTable {
    pub fn new() -> Self { Self::default() }
    pub fn insert(&mut self, addr: u32, f: NativeFn) { self.map.insert(addr, f); }
    pub fn remove(&mut self, addr: u32) { self.map.remove(&addr); }
    #[inline]
    pub fn get(&self, addr: u32) -> Option<NativeFn> { self.map.get(&addr).copied() }
    pub fn len(&self) -> usize { self.map.len() }
    pub fn is_empty(&self) -> bool { self.map.is_empty() }
    pub fn addrs_unsorted(&self) -> impl Iterator<Item = &u32> { self.map.keys() }
    pub fn addrs(&self) -> Vec<u32> { let mut v: Vec<u32> = self.map.keys().copied().collect(); v.sort(); v }
}

/// `dyn Bus` también es un `Bus` (para que las funciones nativas reutilicen
/// los helpers genéricos de la CPU).
impl Bus for &mut dyn Bus {
    fn read8(&mut self, addr: u32) -> u8 { (**self).read8(addr) }
    fn read16(&mut self, addr: u32) -> u16 { (**self).read16(addr) }
    fn read32(&mut self, addr: u32) -> u32 { (**self).read32(addr) }
    fn write8(&mut self, addr: u32, v: u8) { (**self).write8(addr, v) }
    fn write16(&mut self, addr: u32, v: u16) { (**self).write16(addr, v) }
    fn write32(&mut self, addr: u32, v: u32) { (**self).write32(addr, v) }
}

impl std::fmt::Debug for NativeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NativeTable({} funciones)", self.map.len())
    }
}
