// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p04_8;
pub mod p05_0;
pub mod p05_1;
pub mod p05_2;

pub const TABLES: &[&[(u32, InsFn)]] = &[p04_8::TABLE, p05_0::TABLE, p05_1::TABLE, p05_2::TABLE];
pub const COUNT: usize = 5349;
