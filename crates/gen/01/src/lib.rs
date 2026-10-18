// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p01_0;
pub mod p02_0;
pub mod p02_1;
pub mod p02_2;

pub const TABLES: &[&[(u32, InsFn)]] = &[p01_0::TABLE, p02_0::TABLE, p02_1::TABLE, p02_2::TABLE];
pub const COUNT: usize = 5034;
