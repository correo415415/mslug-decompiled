// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p05_7;
pub mod p05_8;
pub mod p06_0;
pub mod p06_1;

pub const TABLES: &[&[(u32, InsFn)]] = &[p05_7::TABLE, p05_8::TABLE, p06_0::TABLE, p06_1::TABLE];
pub const COUNT: usize = 5583;
