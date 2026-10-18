// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p04_5;
pub mod p04_6;
pub mod p04_7;
pub mod p04_8;

pub const TABLES: &[&[(u32, InsFn)]] = &[p04_5::TABLE, p04_6::TABLE, p04_7::TABLE, p04_8::TABLE];
pub const COUNT: usize = 5979;
