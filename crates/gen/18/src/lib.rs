// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p08_6;
pub mod p08_7;
pub mod p08_8;
pub mod p08_9;

pub const TABLES: &[&[(u32, InsFn)]] = &[p08_6::TABLE, p08_7::TABLE, p08_8::TABLE, p08_9::TABLE];
pub const COUNT: usize = 5002;
