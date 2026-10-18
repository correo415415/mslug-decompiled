// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod bios_0;
pub mod bios_1;
pub mod bios_2;
pub mod p00_0;
pub mod p00_1;

pub const TABLES: &[&[(u32, InsFn)]] = &[bios_0::TABLE, bios_1::TABLE, bios_2::TABLE, p00_0::TABLE, p00_1::TABLE];
pub const COUNT: usize = 5946;
