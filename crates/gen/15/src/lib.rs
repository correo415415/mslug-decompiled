// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p08_3;
pub mod p08_4;
pub mod p08_5;
pub mod p08_6;

pub const TABLES: &[&[(u32, InsFn)]] = &[p08_3::TABLE, p08_4::TABLE, p08_5::TABLE, p08_6::TABLE];
pub const COUNT: usize = 6000;
