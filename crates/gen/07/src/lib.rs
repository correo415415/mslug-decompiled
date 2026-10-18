// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p04_1;
pub mod p04_2;
pub mod p04_3;
pub mod p04_4;

pub const TABLES: &[&[(u32, InsFn)]] = &[p04_1::TABLE, p04_2::TABLE, p04_3::TABLE, p04_4::TABLE];
pub const COUNT: usize = 6000;
