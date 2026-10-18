// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p06_5;
pub mod p06_6;
pub mod p06_7;
pub mod p06_8;

pub const TABLES: &[&[(u32, InsFn)]] = &[p06_5::TABLE, p06_6::TABLE, p06_7::TABLE, p06_8::TABLE];
pub const COUNT: usize = 6000;
