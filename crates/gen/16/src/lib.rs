// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p07_7;
pub mod p07_8;
pub mod p08_0;
pub mod p08_1;

pub const TABLES: &[&[(u32, InsFn)]] = &[p07_7::TABLE, p07_8::TABLE, p08_0::TABLE, p08_1::TABLE];
pub const COUNT: usize = 5387;
