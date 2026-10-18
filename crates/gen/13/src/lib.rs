// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p07_4;
pub mod p07_5;
pub mod p07_6;
pub mod p07_7;

pub const TABLES: &[&[(u32, InsFn)]] = &[p07_4::TABLE, p07_5::TABLE, p07_6::TABLE, p07_7::TABLE];
pub const COUNT: usize = 5292;
