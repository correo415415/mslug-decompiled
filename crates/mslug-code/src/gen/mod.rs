// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.

pub mod bios;
pub mod p00;
pub mod p01;
pub mod p02;
pub mod p03;
pub mod p04;
pub mod p05;
pub mod p06;
pub mod p07;
pub mod p08;
pub mod p09;
pub mod p28;

pub const TABLES: &[&[(u32, crate::InsFn)]] = &[bios::TABLE, p00::TABLE, p01::TABLE, p02::TABLE, p03::TABLE, p04::TABLE, p05::TABLE, p06::TABLE, p07::TABLE, p08::TABLE, p09::TABLE, p28::TABLE];
pub const COUNT: usize = 28593;
