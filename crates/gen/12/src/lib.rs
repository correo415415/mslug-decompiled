// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p07_0;
pub mod p07_1;
pub mod p07_2;
pub mod p07_3;

pub const TABLES: &[&[(u32, InsFn)]] = &[p07_0::TABLE, p07_1::TABLE, p07_2::TABLE, p07_3::TABLE];
pub const COUNT: usize = 6000;
