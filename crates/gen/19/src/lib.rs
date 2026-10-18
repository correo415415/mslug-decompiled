// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p09_0;
pub mod p09_1;
pub mod p09_2;
pub mod p09_3;

pub const TABLES: &[&[(u32, InsFn)]] = &[p09_0::TABLE, p09_1::TABLE, p09_2::TABLE, p09_3::TABLE];
pub const COUNT: usize = 6000;
