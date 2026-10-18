// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod bios_4;
pub mod bios_5;
pub mod bios_6;
pub mod bios_7;

pub const TABLES: &[&[(u32, InsFn)]] = &[bios_4::TABLE, bios_5::TABLE, bios_6::TABLE, bios_7::TABLE];
pub const COUNT: usize = 6000;
