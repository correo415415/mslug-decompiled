// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p29_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $29C956: move.w #$28, d1
pub fn i_29c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C95A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $29C95A: jsr $236e.l
pub fn i_29c95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29C960);
    cpu.pc = 0x236E;
    16
}
/// $29C960: rts
pub fn i_29c960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C962;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $29CB32: move.w #$ff00, d0
pub fn i_29cb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB36;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $29CB36: jsr $5dca4.l
pub fn i_29cb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29CB3C);
    cpu.pc = 0x5DCA4;
    16
}
/// $29CB3C: move.w d0, $28(a6)
pub fn i_29cb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB40;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $29CB40: move.w #$1fe, $2a(a6)
pub fn i_29cb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB46;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $29CB46: move.w #$ffde, $2e(a6)
pub fn i_29cb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB4C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $29CB4C: move.w #$0, $2c(a6)
pub fn i_29cb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $29CB52: jsr $5e9b6.l
pub fn i_29cb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29CB58);
    cpu.pc = 0x5E9B6;
    16
}
/// $29CB58: move.w #$7f, d0
pub fn i_29cb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB5C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $29CB5C: subi.w #$3f, d0
pub fn i_29cb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB60;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x3F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $29CB60: add.w $28(a6), d0
pub fn i_29cb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB64;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $29CB64: move.w d0, $28(a6)
pub fn i_29cb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB68;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $29CB68: rts
pub fn i_29cb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CB6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x29C956, i_29c956),
    (0x29C95A, i_29c95a),
    (0x29C960, i_29c960),
    (0x29CB32, i_29cb32),
    (0x29CB36, i_29cb36),
    (0x29CB3C, i_29cb3c),
    (0x29CB40, i_29cb40),
    (0x29CB46, i_29cb46),
    (0x29CB4C, i_29cb4c),
    (0x29CB52, i_29cb52),
    (0x29CB58, i_29cb58),
    (0x29CB5C, i_29cb5c),
    (0x29CB60, i_29cb60),
    (0x29CB64, i_29cb64),
    (0x29CB68, i_29cb68),
];
