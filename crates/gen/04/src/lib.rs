// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p03_4;
pub mod p03_5;
pub mod p03_6;
pub mod p03_7;

pub const TABLES: &[&[(u32, InsFn)]] = &[p03_4::TABLE, p03_5::TABLE, p03_6::TABLE, p03_7::TABLE];
pub const COUNT: usize = 5008;
