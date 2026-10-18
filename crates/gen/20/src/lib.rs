// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p09_4;
pub mod p28_0;
pub mod p29_0;
pub mod p2b_0;

pub const TABLES: &[&[(u32, InsFn)]] = &[p09_4::TABLE, p28_0::TABLE, p29_0::TABLE, p2b_0::TABLE];
pub const COUNT: usize = 1062;
