// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
#![allow(clippy::all)]
pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;

pub mod p06_1;
pub mod p06_2;
pub mod p06_3;
pub mod p06_4;

pub const TABLES: &[&[(u32, InsFn)]] = &[p06_1::TABLE, p06_2::TABLE, p06_3::TABLE, p06_4::TABLE];
pub const COUNT: usize = 6000;
