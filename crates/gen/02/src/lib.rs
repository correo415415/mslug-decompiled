// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod bios_8;
pub mod bios_9;
pub mod p00_0;
pub mod p00_1;
pub mod p01_0;

pub const TABLES: &[&[(u32, InsFn)]] = &[bios_8::TABLE, bios_9::TABLE, p00_0::TABLE, p00_1::TABLE, p01_0::TABLE];
pub const COUNT: usize = 5066;
