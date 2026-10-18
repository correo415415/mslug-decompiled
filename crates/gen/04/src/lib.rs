// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p02_4;
pub mod p02_5;
pub mod p02_6;
pub mod p03_0;

pub const TABLES: &[&[(u32, InsFn)]] = &[p02_4::TABLE, p02_5::TABLE, p02_6::TABLE, p03_0::TABLE];
pub const COUNT: usize = 5245;
