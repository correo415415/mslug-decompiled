// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05ADF2: cmpa.l a2, a3
pub fn i_05adf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF4;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ADF4: beq.w $5adfe
pub fn i_05adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ADFE; return 10; }
    8
}
/// $05ADF8: ori.b #$80, -$6(a3)
pub fn i_05adf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADFE;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ADFE: rts
pub fn i_05adfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AE00: cmpa.l a2, a3
pub fn i_05ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE02;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AE02: bcc.w $5af2e
pub fn i_05ae02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF2E; return 10; }
    8
}
/// $05AE06: move.b (a0)+, -(a7)
pub fn i_05ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE08;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AE08: move.b (a0)+, d2
pub fn i_05ae08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE0A;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AE0A: eor.b d5, d2
pub fn i_05ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE0C;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AE0C: andi.w #$ff, d4
pub fn i_05ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE10;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE10: move.w d4, d7
pub fn i_05ae10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE12;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE12: addq.w #$1, d7
pub fn i_05ae12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE14;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AE14: swap d7
pub fn i_05ae14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE16;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE16: move.w d3, d7
pub fn i_05ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE18;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE18: andi.w #$ff, d7
pub fn i_05ae18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE1C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE1C: addq.w #$1, d7
pub fn i_05ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE1E;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AE1E: lsl.w #$4, d4
pub fn i_05ae1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE20;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AE20: eori.b #$f0, d4
pub fn i_05ae20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE24;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AE24: move.b d4, $616e(a5)
pub fn i_05ae24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE28;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AE28: move.b d3, d4
pub fn i_05ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE2A;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AE2A: andi.b #$f0, d4
pub fn i_05ae2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE2E;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AE2E: move.w d4, d6
pub fn i_05ae2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE30;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE30: subi.w #$100, d4
pub fn i_05ae30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE34;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE34: swap d4
pub fn i_05ae34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE36;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE36: move.w d6, d4
pub fn i_05ae36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE38;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE38: andi.w #$f, d3
pub fn i_05ae38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE3C;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE3C: swap d2
pub fn i_05ae3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE3E;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE3E: move.w d3, d2
pub fn i_05ae3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE40;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE40: swap d2
pub fn i_05ae40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE42;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE42: moveq #$0, d5
pub fn i_05ae42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE44;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE44: move.l (a0)+, d1
pub fn i_05ae44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE46;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AE46: moveq #$0, d5
pub fn i_05ae46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE48;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE48: move.b (a0)+, d5
pub fn i_05ae48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE4A;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AE4A: swap d5
pub fn i_05ae4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE4C;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE4C: move.b (a0)+, d5
pub fn i_05ae4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE4E;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AE4E: swap d1
pub fn i_05ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE50;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE50: not.w d1
pub fn i_05ae50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE52;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AE52: swap d1
pub fn i_05ae52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE54;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE54: moveq #$0, d3
pub fn i_05ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE56;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE56: move.l d1, d6
pub fn i_05ae56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE58;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE58: muls.w d7, d6
pub fn i_05ae58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE5A;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AE5A: spl.b d3
pub fn i_05ae5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE5C;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AE5C: add.l d3, d6
pub fn i_05ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE5E;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05AE5E: asr.l #$8, d6
pub fn i_05ae5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE60;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05AE60: swap d7
pub fn i_05ae60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE62;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE62: swap d1
pub fn i_05ae62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE64;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE64: muls.w d7, d1
pub fn i_05ae64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE66;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AE66: move.b d1, $616d(a5)
pub fn i_05ae66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE6A;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AE6A: lsl.l #$8, d1
pub fn i_05ae6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE6C;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05AE6C: move.w d6, d1
pub fn i_05ae6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE6E;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE6E: swap d7
pub fn i_05ae6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE70;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE70: move.w d5, d6
pub fn i_05ae70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE72;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE72: cmpi.b #$11, d6
pub fn i_05ae72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE76;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05AE76: bcc.w $5ae90
pub fn i_05ae76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AE90; return 10; }
    8
}
/// $05AE7A: subq.b #$1, d6
pub fn i_05ae7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE7C;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AE7C: lsl.b #$4, d6
pub fn i_05ae7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE7E;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05AE7E: swap d2
pub fn i_05ae7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE80;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE80: or.b d2, d6
pub fn i_05ae80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE82;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AE82: lea.l $5a8ba(pc), a4
pub fn i_05ae82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE86;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05AE86: move.b (a4, d6.w), d6
pub fn i_05ae86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE8A;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05AE8A: swap d2
pub fn i_05ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE8C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE8C: bra.w $5ae96
pub fn i_05ae8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE90;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AE96;
    10
}
/// $05AE90: swap d2
pub fn i_05ae90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE92;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE92: move.w d2, d6
pub fn i_05ae92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE94;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE94: swap d2
pub fn i_05ae94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE96;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE96: andi.b #$f0, d4
pub fn i_05ae96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE9A;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AE9A: or.b d4, d6
pub fn i_05ae9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE9C;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AE9C: move.b d6, d4
pub fn i_05ae9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE9E;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AE9E: swap d4
pub fn i_05ae9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA0;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEA0: move.b d6, d4
pub fn i_05aea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA2;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AEA2: swap d4
pub fn i_05aea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA4;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEA4: move.w d7, d6
pub fn i_05aea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA6;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AEA6: mulu.w d5, d6
pub fn i_05aea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA8;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AEA8: addi.w #$ff, d6
pub fn i_05aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEAC;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEAC: lsr.w #$8, d6
pub fn i_05aeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEAE;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AEAE: swap d6
pub fn i_05aeae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB0;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEB0: move.w d5, d6
pub fn i_05aeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB2;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AEB2: add.w d0, d1
pub fn i_05aeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB4;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AEB4: swap d1
pub fn i_05aeb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB6;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEB6: swap d0
pub fn i_05aeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB8;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEB8: add.w d0, d1
pub fn i_05aeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEBA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AEBA: swap d0
pub fn i_05aeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEBC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEBC: andi.w #$1ff, d1
pub fn i_05aebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC0;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEC0: lsl.l #$7, d1
pub fn i_05aec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC2;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AEC2: or.l d6, d1
pub fn i_05aec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC4;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEC4: clr.w d5
pub fn i_05aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC6;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AEC6: swap d5
pub fn i_05aec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC8;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEC8: subq.b #$1, d5
pub fn i_05aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AECA;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AECA: add.w d6, d6
pub fn i_05aeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AECC;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AECC: movea.l d4, a4
pub fn i_05aecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AECE;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05AECE: move.b $616e(a5), d3
pub fn i_05aece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AED2;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05AED2: add.b d3, $616d(a5)
pub fn i_05aed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AED6;
    cpu.ir = 0xD72D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05AED6: bcc.w $5aee2
pub fn i_05aed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AEE2; return 10; }
    8
}
/// $05AEDA: swap d4
pub fn i_05aeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEDC;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEDC: tst.w d4
pub fn i_05aedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEDE;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05AEDE: bmi.w $5af0e
pub fn i_05aede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEE2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5AF0E; return 10; }
    8
}
/// $05AEE2: move.w d4, d3
pub fn i_05aee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEE4;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AEE4: andi.w #$f00, d3
pub fn i_05aee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEE8;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEE8: addi.w #$100, d3
pub fn i_05aee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEEC;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEEC: lsr.w #$1, d3
pub fn i_05aeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEEE;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05AEEE: sub.w d3, d1
pub fn i_05aeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEF0;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AEF0: cmpi.w #$f800, d1
pub fn i_05aef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEF4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AEF4: bcc.w $5af00
pub fn i_05aef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF00; return 10; }
    8
}
/// $05AEF8: cmpi.w #$a000, d1
pub fn i_05aef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEFC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AEFC: bcc.w $5af0e
pub fn i_05aefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF0E; return 10; }
    8
}
/// $05AF00: move.l a0, (a3)+
pub fn i_05af00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF02;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AF02: move.w d2, (a3)+
pub fn i_05af02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF04;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AF04: move.w d4, (a3)+
pub fn i_05af04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF06;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AF06: move.l d1, (a3)+
pub fn i_05af06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF08;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AF08: cmpa.l a2, a3
pub fn i_05af08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF0A;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AF0A: bcc.w $5af1c
pub fn i_05af0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF1C; return 10; }
    8
}
/// $05AF0E: move.l a4, d4
pub fn i_05af0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF10;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF10: adda.w d6, a0
pub fn i_05af10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF12;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AF12: dbra d5, $5aece
pub fn i_05af12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF16;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AF16; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AF16; return 14; }
    cpu.pc = 0x5AECE;
    10
}
/// $05AF16: subq.b #$1, (a7)
pub fn i_05af16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF18;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AF18: bne.w $5ae42
pub fn i_05af18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF1C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AE42; return 10; }
    8
}
/// $05AF1C: addq.w #$2, a7
pub fn i_05af1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF1E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AF1E: lea.l $4258(a5), a2
pub fn i_05af1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF22;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AF22: cmpa.l a2, a3
pub fn i_05af22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF24;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AF24: beq.w $5af2e
pub fn i_05af24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AF2E; return 10; }
    8
}
/// $05AF28: ori.b #$80, -$6(a3)
pub fn i_05af28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF2E;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AF2E: rts
pub fn i_05af2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AF30: cmpa.l a2, a3
pub fn i_05af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF32;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AF32: bcc.w $5b06c
pub fn i_05af32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B06C; return 10; }
    8
}
/// $05AF36: move.b (a0)+, -(a7)
pub fn i_05af36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF38;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AF38: move.b (a0)+, d2
pub fn i_05af38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF3A;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AF3A: eor.b d5, d2
pub fn i_05af3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF3C;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AF3C: andi.w #$ff, d4
pub fn i_05af3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF40;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF40: move.w d4, d7
pub fn i_05af40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF42;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF42: addq.w #$1, d7
pub fn i_05af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF44;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF44: swap d7
pub fn i_05af44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF46;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF46: move.w d3, d7
pub fn i_05af46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF48;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF48: andi.w #$ff, d7
pub fn i_05af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF4C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF4C: addq.w #$1, d7
pub fn i_05af4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF4E;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF4E: lsl.w #$4, d4
pub fn i_05af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF50;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AF50: eori.b #$f0, d4
pub fn i_05af50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF54;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AF54: move.b d4, $616e(a5)
pub fn i_05af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF58;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AF58: move.b d3, d4
pub fn i_05af58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF5A;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AF5A: andi.b #$f0, d4
pub fn i_05af5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF5E;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AF5E: move.w d4, d6
pub fn i_05af5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF60;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF60: subi.w #$100, d4
pub fn i_05af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF64;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF64: swap d4
pub fn i_05af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF66;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF66: move.w d6, d4
pub fn i_05af66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF68;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF68: ori.l #$10001000, d4
pub fn i_05af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF6E;
    cpu.ir = 0x84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 | 0x10001000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $05AF6E: andi.w #$f, d3
pub fn i_05af6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF72;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF72: swap d2
pub fn i_05af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF74;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF74: move.w d3, d2
pub fn i_05af74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF76;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF76: swap d2
pub fn i_05af76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF78;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF78: moveq #$0, d5
pub fn i_05af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF7A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF7A: move.l (a0)+, d1
pub fn i_05af7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF7C;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AF7C: moveq #$0, d5
pub fn i_05af7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF7E;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF7E: move.b (a0)+, d5
pub fn i_05af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF80;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AF80: swap d5
pub fn i_05af80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF82;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF82: move.b (a0)+, d5
pub fn i_05af82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF84;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AF84: neg.w d1
pub fn i_05af84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF86;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF86: move.w d5, d6
pub fn i_05af86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF88;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF88: lsl.w #$4, d6
pub fn i_05af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF8A;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AF8A: subq.w #$1, d6
pub fn i_05af8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF8C;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF8C: add.w d6, d1
pub fn i_05af8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF8E;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AF8E: moveq #$0, d3
pub fn i_05af8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF90;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF90: move.l d1, d6
pub fn i_05af90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF92;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF92: muls.w d7, d6
pub fn i_05af92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF94;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AF94: spl.b d3
pub fn i_05af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF96;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AF96: add.l d3, d6
pub fn i_05af96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF98;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05AF98: asr.l #$8, d6
pub fn i_05af98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF9A;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05AF9A: swap d7
pub fn i_05af9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF9C;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF9C: swap d1
pub fn i_05af9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF9E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF9E: muls.w d7, d1
pub fn i_05af9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA0;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AFA0: move.b d1, $616d(a5)
pub fn i_05afa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA4;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AFA4: lsl.l #$8, d1
pub fn i_05afa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA6;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05AFA6: move.w d6, d1
pub fn i_05afa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA8;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFA8: swap d7
pub fn i_05afa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFAA;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFAA: move.w d5, d6
pub fn i_05afaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFAC;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFAC: cmpi.b #$11, d6
pub fn i_05afac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB0;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05AFB0: bcc.w $5afca
pub fn i_05afb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AFCA; return 10; }
    8
}
/// $05AFB4: subq.b #$1, d6
pub fn i_05afb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB6;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AFB6: lsl.b #$4, d6
pub fn i_05afb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB8;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05AFB8: swap d2
pub fn i_05afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFBA;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFBA: or.b d2, d6
pub fn i_05afba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFBC;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AFBC: lea.l $5a8ba(pc), a4
pub fn i_05afbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFC0;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05AFC0: move.b (a4, d6.w), d6
pub fn i_05afc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFC4;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05AFC4: swap d2
pub fn i_05afc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFC6;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFC6: bra.w $5afd0
pub fn i_05afc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AFD0;
    10
}
/// $05AFCA: swap d2
pub fn i_05afca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFCC;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFCC: move.w d2, d6
pub fn i_05afcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFCE;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFCE: swap d2
pub fn i_05afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD0;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFD0: andi.b #$f0, d4
pub fn i_05afd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD4;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AFD4: or.b d4, d6
pub fn i_05afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD6;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AFD6: move.b d6, d4
pub fn i_05afd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD8;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AFD8: swap d4
pub fn i_05afd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFDA;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFDA: move.b d6, d4
pub fn i_05afda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFDC;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AFDC: swap d4
pub fn i_05afdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFDE;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFDE: move.w d7, d6
pub fn i_05afde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE0;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFE0: mulu.w d5, d6
pub fn i_05afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE2;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AFE2: addi.w #$ff, d6
pub fn i_05afe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE6;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AFE6: lsr.w #$8, d6
pub fn i_05afe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE8;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AFE8: swap d6
pub fn i_05afe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFEA;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFEA: move.w d5, d6
pub fn i_05afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFEC;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFEC: add.w d0, d1
pub fn i_05afec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFEE;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AFEE: swap d1
pub fn i_05afee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF0;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFF0: swap d0
pub fn i_05aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFF2: add.w d0, d1
pub fn i_05aff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF4;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AFF4: swap d0
pub fn i_05aff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFF6: andi.w #$1ff, d1
pub fn i_05aff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFFA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AFFA: lsl.l #$7, d1
pub fn i_05affa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFFC;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AFFC: or.l d6, d1
pub fn i_05affc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFFE;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFFE: clr.w d5
pub fn i_05affe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B000;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B000: swap d5
pub fn i_05b000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B002;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B002: subq.b #$1, d5
pub fn i_05b002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B004;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B004: add.w d6, d6
pub fn i_05b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B006;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B006: adda.w d6, a0
pub fn i_05b006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B008;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B008: movea.l d4, a4
pub fn i_05b008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B00A;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05B00A: move.b $616e(a5), d3
pub fn i_05b00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B00E;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05B00E: sub.b d3, $616d(a5)
pub fn i_05b00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B012;
    cpu.ir = 0x972D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B012: bcc.w $5b01e
pub fn i_05b012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B016;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B01E; return 10; }
    8
}
/// $05B016: swap d4
pub fn i_05b016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B018;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B018: tst.w d4
pub fn i_05b018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B01A;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05B01A: bmi.w $5b04a
pub fn i_05b01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B01E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5B04A; return 10; }
    8
}
/// $05B01E: move.w d4, d3
pub fn i_05b01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B020;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B020: andi.w #$f00, d3
pub fn i_05b020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B024;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B024: addi.w #$100, d3
pub fn i_05b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B028;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B028: lsr.w #$1, d3
pub fn i_05b028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B02A;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B02A: cmpi.w #$f800, d1
pub fn i_05b02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B02E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05B02E: bcc.w $5b03a
pub fn i_05b02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B032;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B03A; return 10; }
    8
}
/// $05B032: cmpi.w #$a000, d1
pub fn i_05b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B036;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05B036: bcc.w $5b048
pub fn i_05b036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B03A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B048; return 10; }
    8
}
/// $05B03A: move.l a0, (a3)+
pub fn i_05b03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B03C;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B03C: move.w d2, (a3)+
pub fn i_05b03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B03E;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B03E: move.w d4, (a3)+
pub fn i_05b03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B040;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B040: move.l d1, (a3)+
pub fn i_05b040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B042;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B042: cmpa.l a2, a3
pub fn i_05b042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B044;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B044: bcc.w $5b05a
pub fn i_05b044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B048;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B05A; return 10; }
    8
}
/// $05B048: add.w d3, d1
pub fn i_05b048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B04A;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B04A: move.l a4, d4
pub fn i_05b04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B04C;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B04C: adda.w d6, a0
pub fn i_05b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B04E;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B04E: dbra d5, $5b00a
pub fn i_05b04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B052;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B052; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B052; return 14; }
    cpu.pc = 0x5B00A;
    10
}
/// $05B052: suba.w d6, a0
pub fn i_05b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B054;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B054: subq.b #$1, (a7)
pub fn i_05b054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B056;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05B056: bne.w $5af78
pub fn i_05b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B05A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AF78; return 10; }
    8
}
/// $05B05A: addq.w #$2, a7
pub fn i_05b05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B05C;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05B05C: lea.l $4258(a5), a2
pub fn i_05b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B060;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05B060: cmpa.l a2, a3
pub fn i_05b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B062;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B062: beq.w $5b06c
pub fn i_05b062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B066;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B06C; return 10; }
    8
}
/// $05B066: ori.b #$80, -$6(a3)
pub fn i_05b066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B06C;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05B06C: rts
pub fn i_05b06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B06E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B06E: cmpa.l a2, a3
pub fn i_05b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B070;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B070: bcc.w $5b1b0
pub fn i_05b070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B074;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B1B0; return 10; }
    8
}
/// $05B074: move.b (a0)+, -(a7)
pub fn i_05b074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B076;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05B076: move.b (a0)+, d2
pub fn i_05b076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B078;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05B078: eor.b d5, d2
pub fn i_05b078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B07A;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B07A: andi.w #$ff, d4
pub fn i_05b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B07E;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B07E: move.w d4, d7
pub fn i_05b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B080;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B080: addq.w #$1, d7
pub fn i_05b080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B082;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B082: swap d7
pub fn i_05b082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B084;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B084: move.w d3, d7
pub fn i_05b084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B086;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B086: andi.w #$ff, d7
pub fn i_05b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B08A;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B08A: addq.w #$1, d7
pub fn i_05b08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B08C;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B08C: lsl.w #$4, d4
pub fn i_05b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B08E;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05B08E: eori.b #$f0, d4
pub fn i_05b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B092;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B092: move.b d4, $616e(a5)
pub fn i_05b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B096;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05B096: move.b d3, d4
pub fn i_05b096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B098;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B098: andi.b #$f0, d4
pub fn i_05b098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B09C;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B09C: move.w d4, d6
pub fn i_05b09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B09E;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B09E: subi.w #$100, d4
pub fn i_05b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0A2;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B0A2: swap d4
pub fn i_05b0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0A4;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0A4: move.w d6, d4
pub fn i_05b0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0A6;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0A6: ori.l #$10001000, d4
pub fn i_05b0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0AC;
    cpu.ir = 0x84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 | 0x10001000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $05B0AC: andi.w #$f, d3
pub fn i_05b0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B0;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B0B0: swap d2
pub fn i_05b0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B2;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0B2: move.w d3, d2
pub fn i_05b0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B4;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0B4: swap d2
pub fn i_05b0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B6;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0B6: moveq #$0, d5
pub fn i_05b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B8;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0B8: move.l (a0)+, d1
pub fn i_05b0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0BA;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B0BA: moveq #$0, d5
pub fn i_05b0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0BC;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0BC: move.b (a0)+, d5
pub fn i_05b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0BE;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05B0BE: swap d5
pub fn i_05b0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C0;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0C0: move.b (a0)+, d5
pub fn i_05b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C2;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05B0C2: swap d1
pub fn i_05b0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C4;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0C4: not.w d1
pub fn i_05b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C6;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B0C6: swap d1
pub fn i_05b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C8;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0C8: neg.w d1
pub fn i_05b0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0CA;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B0CA: move.w d5, d6
pub fn i_05b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0CC;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0CC: lsl.w #$4, d6
pub fn i_05b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0CE;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05B0CE: subq.w #$1, d6
pub fn i_05b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D0;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B0D0: add.w d6, d1
pub fn i_05b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D2;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B0D2: moveq #$0, d3
pub fn i_05b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D4;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0D4: move.l d1, d6
pub fn i_05b0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D6;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0D6: muls.w d7, d6
pub fn i_05b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D8;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05B0D8: spl.b d3
pub fn i_05b0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0DA;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05B0DA: add.l d3, d6
pub fn i_05b0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0DC;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B0DC: asr.l #$8, d6
pub fn i_05b0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0DE;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05B0DE: swap d7
pub fn i_05b0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E0;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0E0: swap d1
pub fn i_05b0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E2;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0E2: muls.w d7, d1
pub fn i_05b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E4;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05B0E4: move.b d1, $616d(a5)
pub fn i_05b0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E8;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05B0E8: lsl.l #$8, d1
pub fn i_05b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0EA;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05B0EA: move.w d6, d1
pub fn i_05b0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0EC;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0EC: swap d7
pub fn i_05b0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0EE;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0EE: move.w d5, d6
pub fn i_05b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0F0;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0F0: cmpi.b #$11, d6
pub fn i_05b0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0F4;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05B0F4: bcc.w $5b10e
pub fn i_05b0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B10E; return 10; }
    8
}
/// $05B0F8: subq.b #$1, d6
pub fn i_05b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0FA;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B0FA: lsl.b #$4, d6
pub fn i_05b0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0FC;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05B0FC: swap d2
pub fn i_05b0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0FE;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0FE: or.b d2, d6
pub fn i_05b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B100;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05B100: lea.l $5a8ba(pc), a4
pub fn i_05b100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B104;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05B104: move.b (a4, d6.w), d6
pub fn i_05b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B108;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05B108: swap d2
pub fn i_05b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B10A;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B10A: bra.w $5b114
pub fn i_05b10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B10E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B114;
    10
}
/// $05B10E: swap d2
pub fn i_05b10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B110;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B110: move.w d2, d6
pub fn i_05b110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B112;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B112: swap d2
pub fn i_05b112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B114;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B114: andi.b #$f0, d4
pub fn i_05b114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B118;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B118: or.b d4, d6
pub fn i_05b118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B11A;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05B11A: move.b d6, d4
pub fn i_05b11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B11C;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B11C: swap d4
pub fn i_05b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B11E;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B11E: move.b d6, d4
pub fn i_05b11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B120;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B120: swap d4
pub fn i_05b120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B122;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B122: move.w d7, d6
pub fn i_05b122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B124;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B124: mulu.w d5, d6
pub fn i_05b124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B126;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05B126: addi.w #$ff, d6
pub fn i_05b126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B12A;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B12A: lsr.w #$8, d6
pub fn i_05b12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B12C;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05B12C: swap d6
pub fn i_05b12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B12E;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B12E: move.w d5, d6
pub fn i_05b12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B130;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B130: add.w d0, d1
pub fn i_05b130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B132;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B132: swap d1
pub fn i_05b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B134;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B134: swap d0
pub fn i_05b134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B136;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B136: add.w d0, d1
pub fn i_05b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B138;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B138: swap d0
pub fn i_05b138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B13A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B13A: andi.w #$1ff, d1
pub fn i_05b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B13E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B13E: lsl.l #$7, d1
pub fn i_05b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B140;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05B140: or.l d6, d1
pub fn i_05b140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B142;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B142: clr.w d5
pub fn i_05b142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B144;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B144: swap d5
pub fn i_05b144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B146;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B146: subq.b #$1, d5
pub fn i_05b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B148;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B148: add.w d6, d6
pub fn i_05b148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B14A: adda.w d6, a0
pub fn i_05b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14C;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B14C: movea.l d4, a4
pub fn i_05b14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14E;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05B14E: move.b $616e(a5), d3
pub fn i_05b14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B152;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05B152: add.b d3, $616d(a5)
pub fn i_05b152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B156;
    cpu.ir = 0xD72D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B156: bcc.w $5b162
pub fn i_05b156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B162; return 10; }
    8
}
/// $05B15A: swap d4
pub fn i_05b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B15C: tst.w d4
pub fn i_05b15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15E;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05B15E: bmi.w $5b18e
pub fn i_05b15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B162;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5B18E; return 10; }
    8
}
/// $05B162: move.w d4, d3
pub fn i_05b162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B164;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B164: andi.w #$f00, d3
pub fn i_05b164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B168;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B168: addi.w #$100, d3
pub fn i_05b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B16C;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B16C: lsr.w #$1, d3
pub fn i_05b16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B16E;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B16E: sub.w d3, d1
pub fn i_05b16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B170;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B170: cmpi.w #$f800, d1
pub fn i_05b170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B174;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05B174: bcc.w $5b180
pub fn i_05b174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B178;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B180; return 10; }
    8
}
/// $05B178: cmpi.w #$a000, d1
pub fn i_05b178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B17C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05B17C: bcc.w $5b18e
pub fn i_05b17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B180;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B18E; return 10; }
    8
}
/// $05B180: move.l a0, (a3)+
pub fn i_05b180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B182;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B182: move.w d2, (a3)+
pub fn i_05b182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B184;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B184: move.w d4, (a3)+
pub fn i_05b184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B186;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B186: move.l d1, (a3)+
pub fn i_05b186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B188;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B188: cmpa.l a2, a3
pub fn i_05b188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B18A;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B18A: bcc.w $5b19e
pub fn i_05b18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B18E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B19E; return 10; }
    8
}
/// $05B18E: move.l a4, d4
pub fn i_05b18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B190;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B190: adda.w d6, a0
pub fn i_05b190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B192;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B192: dbra d5, $5b14e
pub fn i_05b192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B196;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B196; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B196; return 14; }
    cpu.pc = 0x5B14E;
    10
}
/// $05B196: suba.w d6, a0
pub fn i_05b196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B198;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B198: subq.b #$1, (a7)
pub fn i_05b198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B19A;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05B19A: bne.w $5b0b6
pub fn i_05b19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B19E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B0B6; return 10; }
    8
}
/// $05B19E: addq.w #$2, a7
pub fn i_05b19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A0;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05B1A0: lea.l $4258(a5), a2
pub fn i_05b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A4;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05B1A4: cmpa.l a2, a3
pub fn i_05b1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A6;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B1A6: beq.w $5b1b0
pub fn i_05b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B1B0; return 10; }
    8
}
/// $05B1AA: ori.b #$80, -$6(a3)
pub fn i_05b1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B0;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05B1B0: rts
pub fn i_05b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_DispatchSplashHook_05b1b2  ($05B1B2, 96 B, video/sprite_queue_render_05aa96.s)
/// $05B1B2: movem.l d0-d5/a0-a1, -(a7)
pub fn i_05b1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B1B6: cmpi.b #$0, $106ece.l
pub fn i_05b1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1BE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $05B1BE: bne.w $5b20c
pub fn i_05b1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1C2: move.l $106f50.l, d6
pub fn i_05b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1C8;
    cpu.ir = 0x2C39;
    let v2 = bus.read32(0x106F50);
    cpu.d[6] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05B1C8: swap d6
pub fn i_05b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1CA;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B1CA: cmpi.w #$780, d6
pub fn i_05b1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1CE;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x780, Size::Word, false);
    8
}
/// $05B1CE: bgt.w $5b20c
pub fn i_05b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1D2: move.w $22(a6), d0
pub fn i_05b1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1D6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B1D6: move.w $24(a6), d1
pub fn i_05b1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1DA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B1DA: addi.w #$8, d1
pub fn i_05b1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1DE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B1DE: jsr $43f02.l
pub fn i_05b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5B1E4);
    cpu.pc = 0x43F02;
    16
}
/// $05B1E4: move.b d0, d6
pub fn i_05b1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1E6;
    cpu.ir = 0x1C00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B1E6: cmpi.b #$3a, d6
pub fn i_05b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1EA;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x3A, Size::Byte, false);
    8
}
/// $05B1EA: bne.w $5b20c
pub fn i_05b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1EE: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1F2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B1F2: movem.l d0-d5/a0-a1, -(a7)
pub fn i_05b1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1F6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B1F6: eori.b #$2, d5
pub fn i_05b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1FA;
    cpu.ir = 0xA05;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B1FA: move.b #$7f, d3
pub fn i_05b1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1FE;
    cpu.ir = 0x163C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B1FE: jsr $5b212(pc)
pub fn i_05b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B202;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5B202);
    cpu.pc = 0x5B212;
    16
}
/// $05B202: jsr $5a9ea(pc)
pub fn i_05b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B206;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5B206);
    cpu.pc = 0x5A9EA;
    16
}
/// $05B206: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B20A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B20A: rts
pub fn i_05b20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B20C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B20C: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B210;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B210: rts
pub fn i_05b210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B212;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_SplashScreenY_05b212  ($05B212, 32 B, video/sprite_queue_render_05aa96.s)
/// $05B212: movem.l d0-d4, -(a7)
pub fn i_05b212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B216;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B216: move.w d0, d6
pub fn i_05b216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B218;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B218: addq.w #$2, d6
pub fn i_05b218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B21A;
    cpu.ir = 0x5446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B21A: move.w #$e8, d1
pub fn i_05b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B21E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B21E: jsr $96a5a.l
pub fn i_05b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5B224);
    cpu.pc = 0x96A5A;
    16
}
/// $05B224: sub.w d1, d6
pub fn i_05b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B226;
    cpu.ir = 0x9C41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B226: sub.w d6, d1
pub fn i_05b226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B228;
    cpu.ir = 0x9246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B228: move.w d1, d6
pub fn i_05b228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B22A;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B22A: movem.l (a7)+, d0-d4
pub fn i_05b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B22E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B22E: move.w d6, d0
pub fn i_05b22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B230;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B230: rts
pub fn i_05b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B232;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_SortAndRenderSCB1_05b232  ($05B232, 318 B, video/sprite_queue_render_05aa96.s)
/// $05B232: lea.l $108080.l, a5
pub fn i_05b232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B238;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05B238: lea.l $5424(a5), a0
pub fn i_05b238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B23C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x5424);
    cpu.a[0] = ad1;
    16
}
/// $05B23C: move.w $6148(a5), d7
pub fn i_05b23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B240;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B240: beq.w $5b2de
pub fn i_05b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B244;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DE; return 10; }
    8
}
/// $05B244: move.w d7, d6
pub fn i_05b244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B246;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B246: lsr.w #$1, d6
pub fn i_05b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B248;
    cpu.ir = 0xE24E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B248: andi.w #$fffc, d6
pub fn i_05b248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B24C;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFFFC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B24C: beq.w $5b2de
pub fn i_05b24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B250;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DE; return 10; }
    8
}
/// $05B250: move.l (a0, d6.w), d0
pub fn i_05b250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B254;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B254: move.w d6, d5
pub fn i_05b254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B256;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B256: move.w d5, d4
pub fn i_05b256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B258;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B258: add.w d4, d4
pub fn i_05b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B25A;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B25A: move.l (a0, d4.w), d1
pub fn i_05b25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B25E;
    cpu.ir = 0x2230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B25E: cmp.w d7, d4
pub fn i_05b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B260;
    cpu.ir = 0xB847;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B260: bcc.w $5b272
pub fn i_05b260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B264;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B272; return 10; }
    8
}
/// $05B264: move.l $4(a0, d4.w), d2
pub fn i_05b264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B268;
    cpu.ir = 0x2430;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B268: cmp.l d1, d2
pub fn i_05b268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B26A;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B26A: bcs.w $5b272
pub fn i_05b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B26E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B272; return 10; }
    8
}
/// $05B26E: move.l d2, d1
pub fn i_05b26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B270;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B270: addq.w #$4, d4
pub fn i_05b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B272;
    cpu.ir = 0x5844;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B272: cmp.l d0, d1
pub fn i_05b272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B274;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B274: bcs.w $5b284
pub fn i_05b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B278;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B284; return 10; }
    8
}
/// $05B278: move.l d1, (a0, d5.w)
pub fn i_05b278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B27C;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B27C: move.w d4, d5
pub fn i_05b27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B27E;
    cpu.ir = 0x3A04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B27E: add.w d4, d4
pub fn i_05b27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B280;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B280: cmp.w d4, d7
pub fn i_05b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B282;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B282: bcc.b $5b25a
pub fn i_05b282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B284;
    cpu.ir = 0x64D6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B25A; return 10; }
    8
}
/// $05B284: move.l d0, (a0, d5.w)
pub fn i_05b284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B288;
    cpu.ir = 0x2180;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B288: subq.w #$4, d6
pub fn i_05b288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B28A;
    cpu.ir = 0x5946;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B28A: bne.b $5b250
pub fn i_05b28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B28C;
    cpu.ir = 0x66C4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B250; return 10; }
    8
}
/// $05B28C: move.w #$4, d6
pub fn i_05b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B290;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B290: move.l $4(a0), d1
pub fn i_05b290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B294;
    cpu.ir = 0x2228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B294: move.l (a0, d7.w), d0
pub fn i_05b294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B298;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B298: move.l d1, (a0, d7.w)
pub fn i_05b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B29C;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B29C: subq.w #$4, d7
pub fn i_05b29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B29E;
    cpu.ir = 0x5947;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B29E: cmp.w d6, d7
pub fn i_05b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A0;
    cpu.ir = 0xBE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2A0: beq.w $5b2da
pub fn i_05b2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DA; return 10; }
    8
}
/// $05B2A4: move.w d6, d5
pub fn i_05b2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A6;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2A6: move.w d5, d4
pub fn i_05b2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A8;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2A8: add.w d4, d4
pub fn i_05b2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2AA;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B2AA: move.l (a0, d4.w), d1
pub fn i_05b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2AE;
    cpu.ir = 0x2230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2AE: cmp.w d7, d4
pub fn i_05b2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B0;
    cpu.ir = 0xB847;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2B0: bcc.w $5b2c2
pub fn i_05b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B2C2; return 10; }
    8
}
/// $05B2B4: move.l $4(a0, d4.w), d2
pub fn i_05b2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B8;
    cpu.ir = 0x2430;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2B8: cmp.l d1, d2
pub fn i_05b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2BA;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B2BA: bcs.w $5b2c2
pub fn i_05b2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2C2; return 10; }
    8
}
/// $05B2BE: move.l d2, d1
pub fn i_05b2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C0;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B2C0: addq.w #$4, d4
pub fn i_05b2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C2;
    cpu.ir = 0x5844;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B2C2: cmp.l d0, d1
pub fn i_05b2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C4;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B2C4: bcs.w $5b2d4
pub fn i_05b2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2D4; return 10; }
    8
}
/// $05B2C8: move.l d1, (a0, d5.w)
pub fn i_05b2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2CC;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2CC: move.w d4, d5
pub fn i_05b2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2CE;
    cpu.ir = 0x3A04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2CE: add.w d4, d4
pub fn i_05b2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D0;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B2D0: cmp.w d4, d7
pub fn i_05b2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D2;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2D2: bcc.b $5b2aa
pub fn i_05b2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D4;
    cpu.ir = 0x64D6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B2AA; return 10; }
    8
}
/// $05B2D4: move.l d0, (a0, d5.w)
pub fn i_05b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D8;
    cpu.ir = 0x2180;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2D8: bra.b $5b290
pub fn i_05b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2DA;
    cpu.ir = 0x60B6;
    cpu.pc = 0x5B290;
    10
}
/// $05B2DA: move.l d0, $4(a0)
pub fn i_05b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2DE;
    cpu.ir = 0x2140;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B2DE: lea.l $5428(a5), a0
pub fn i_05b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2E2;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[0] = ad1;
    16
}
/// $05B2E2: lea.l $348(a0), a1
pub fn i_05b2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2E6;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x348);
    cpu.a[1] = ad1;
    16
}
/// $05B2E6: move.w $614a(a5), d7
pub fn i_05b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2EA;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B2EA: cmpi.w #$344, d7
pub fn i_05b2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2EE;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x344, Size::Word, false);
    8
}
/// $05B2EE: bcc.w $5b31a
pub fn i_05b2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B31A; return 10; }
    8
}
/// $05B2F2: adda.w d7, a0
pub fn i_05b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F4;
    cpu.ir = 0xD0C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B2F4: lea.l $4(a0), a2
pub fn i_05b2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F8;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    cpu.a[2] = ad1;
    16
}
/// $05B2F8: movea.l a2, a3
pub fn i_05b2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FA;
    cpu.ir = 0x264A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $05B2FA: move.l (a2)+, d0
pub fn i_05b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FC;
    cpu.ir = 0x201A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B2FC: move.l -(a3), d1
pub fn i_05b2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FE;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B2FE: cmp.l d0, d1
pub fn i_05b2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B300;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B300: bls.w $5b316
pub fn i_05b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B304;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B316; return 10; }
    8
}
/// $05B304: move.l d1, $4(a3)
pub fn i_05b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B308;
    cpu.ir = 0x2741;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B308: move.l -(a3), d1
pub fn i_05b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30A;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B30A: cmpa.l a3, a0
pub fn i_05b30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30C;
    cpu.ir = 0xB1CB;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B30C: bhi.b $5b312
pub fn i_05b30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30E;
    cpu.ir = 0x6204;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5B312; return 10; }
    8
}
/// $05B30E: cmp.l d0, d1
pub fn i_05b30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B310;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B310: bhi.b $5b304
pub fn i_05b310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B312;
    cpu.ir = 0x62F2;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5B304; return 10; }
    8
}
/// $05B312: move.l d0, $4(a3)
pub fn i_05b312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B316;
    cpu.ir = 0x2740;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B316: cmpa.l a1, a2
pub fn i_05b316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B318;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B318: bcs.b $5b2f8
pub fn i_05b318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B31A;
    cpu.ir = 0x65DE;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2F8; return 10; }
    8
}
/// $05B31A: move.l a6, -(a7)
pub fn i_05b31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B31C;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B31C: lea.l $3c0000.l, a4
pub fn i_05b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B322;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x3C0000;
    20
}
/// $05B322: lea.l $614c(a5), a0
pub fn i_05b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B326;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05B326: move.w $10e1f6.l, (a0)
pub fn i_05b326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B32C;
    cpu.ir = 0x30B9;
    let v2 = bus.read16(0x10E1F6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v2 as u16);
    20
}
/// $05B32C: move.w $10e1fa.l, $2(a0)
pub fn i_05b32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B334;
    cpu.ir = 0x3179;
    let v2 = bus.read16(0x10E1FA) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad3, v2 as u16);
    24
}
/// $05B334: lea.l $542a(a5), a1
pub fn i_05b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B338;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05B338: move.w $6148(a5), d7
pub fn i_05b338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B33C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B33C: lea.l (a1, d7.w), a2
pub fn i_05b33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B340;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $05B340: bsr.w $5b370
pub fn i_05b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B344;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B344);
    cpu.pc = 0x5B370;
    18
}
/// $05B344: lea.l $6158(a5), a0
pub fn i_05b344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B348;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05B348: move.w $10e1f8.l, (a0)
pub fn i_05b348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B34E;
    cpu.ir = 0x30B9;
    let v2 = bus.read16(0x10E1F8) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v2 as u16);
    20
}
/// $05B34E: move.w #$17c, $2(a0)
pub fn i_05b34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B354;
    cpu.ir = 0x317C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B354: lea.l $542a(a5), a1
pub fn i_05b354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B358;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05B358: lea.l $348(a1), a2
pub fn i_05b358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B35C;
    cpu.ir = 0x45E9;
    let ad1 = cpu.a[1].wrapping_add(0x348);
    cpu.a[2] = ad1;
    16
}
/// $05B35C: move.w $614a(a5), d7
pub fn i_05b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B360;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B360: adda.w d7, a1
pub fn i_05b360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B362;
    cpu.ir = 0xD2C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B362: bsr.w $5b370
pub fn i_05b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B366;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B366);
    cpu.pc = 0x5B370;
    18
}
/// $05B366: movea.l (a7)+, a6
pub fn i_05b366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B368;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05B368: move.b #$ff, $616c(a5)
pub fn i_05b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B36E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05B36E: rts
pub fn i_05b36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B370;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_RenderRange_05b370  ($05B370, 144 B, video/sprite_queue_render_05aa96.s)
/// $05B370: move.w $4(a0), d2
pub fn i_05b370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B374;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B374: not.b $8(a0)
pub fn i_05b374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B378;
    cpu.ir = 0x4628;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B378: bpl.w $5b3be
pub fn i_05b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B37C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B3BE; return 10; }
    8
}
/// $05B37C: move.w (a0), d0
pub fn i_05b37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B37E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B37E: move.w d0, d1
pub fn i_05b37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B380;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B380: lsl.w #$6, d1
pub fn i_05b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B382;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B382: move.w $2(a0), d3
pub fn i_05b382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B386;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B386: cmpa.l a1, a2
pub fn i_05b386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B388;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B388: bls.w $5b3ba
pub fn i_05b388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B38C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B3BA; return 10; }
    8
}
/// $05B38C: move.w (a1), d5
pub fn i_05b38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B38E;
    cpu.ir = 0x3A11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B38E: lea.l $4258(a5), a3
pub fn i_05b38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B392;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B392: adda.w d5, a3
pub fn i_05b392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B394;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B394: cmp.w d3, d0
pub fn i_05b394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B396;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B396: bcc.w $5b3fa
pub fn i_05b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B39A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B39A: cmp.w d2, d0
pub fn i_05b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B39C;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B39C: bcc.w $5b3fa
pub fn i_05b39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3A0: bsr.w $5b52e
pub fn i_05b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B3A4);
    cpu.pc = 0x5B52E;
    18
}
/// $05B3A4: addq.w #$1, d0
pub fn i_05b3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B3A6: addi.w #$40, d1
pub fn i_05b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3AA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B3AA: tst.b $6(a3)
pub fn i_05b3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3AE;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B3AE: adda.w #$c, a3
pub fn i_05b3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B2;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3B2: bpl.b $5b394
pub fn i_05b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B4;
    cpu.ir = 0x6AE0;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B394; return 10; }
    8
}
/// $05B3B4: adda.w #$4, a1
pub fn i_05b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B8;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3B8: bra.b $5b386
pub fn i_05b3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3BA;
    cpu.ir = 0x60CC;
    cpu.pc = 0x5B386;
    10
}
/// $05B3BA: bra.w $5b3fa
pub fn i_05b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B3FA;
    10
}
/// $05B3BE: move.w $2(a0), d0
pub fn i_05b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B3C2: move.w d0, d1
pub fn i_05b3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C4;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B3C4: lsl.w #$6, d1
pub fn i_05b3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C6;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B3C6: move.w (a0), d3
pub fn i_05b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C8;
    cpu.ir = 0x3610;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B3C8: cmpa.l a1, a2
pub fn i_05b3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3CA;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B3CA: bls.w $5b3fa
pub fn i_05b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3CE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3CE: subq.l #$4, a2
pub fn i_05b3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D0;
    cpu.ir = 0x598A;
    cpu.a[2] = cpu.a[2].wrapping_sub(4);
    8
}
/// $05B3D0: move.w (a2), d5
pub fn i_05b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D2;
    cpu.ir = 0x3A12;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B3D2: lea.l $4258(a5), a3
pub fn i_05b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D6;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B3D6: adda.w d5, a3
pub fn i_05b3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D8;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3D8: cmp.w d0, d3
pub fn i_05b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3DA;
    cpu.ir = 0xB640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B3DA: bcc.w $5b3fa
pub fn i_05b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3DE: cmp.w d0, d2
pub fn i_05b3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E0;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B3E0: bcc.w $5b3fa
pub fn i_05b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3E4: subq.w #$1, d0
pub fn i_05b3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E6;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B3E6: subi.w #$40, d1
pub fn i_05b3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3EA;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B3EA: bsr.w $5b52e
pub fn i_05b3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3EE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B3EE);
    cpu.pc = 0x5B52E;
    18
}
/// $05B3EE: tst.b $6(a3)
pub fn i_05b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F2;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B3F2: adda.w #$c, a3
pub fn i_05b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F6;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3F6: bpl.b $5b3d8
pub fn i_05b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F8;
    cpu.ir = 0x6AE0;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B3D8; return 10; }
    8
}
/// $05B3F8: bra.b $5b3c8
pub fn i_05b3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3FA;
    cpu.ir = 0x60CE;
    cpu.pc = 0x5B3C8;
    10
}
/// $05B3FA: move.w d0, $6(a0)
pub fn i_05b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3FE;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B3FE: rts
pub fn i_05b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_RenderSCB234_05b400  ($05B400, 302 B, video/sprite_queue_render_05aa96.s)
/// $05B400: tst.b $8(a0)
pub fn i_05b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B404;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B404: bpl.w $5b496
pub fn i_05b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B408;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B496; return 10; }
    8
}
/// $05B408: move.w (a0), d0
pub fn i_05b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40A;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B40A: move.w d0, d1
pub fn i_05b40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B40C: lsl.w #$6, d1
pub fn i_05b40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40E;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B40E: move.w $2(a0), d2
pub fn i_05b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B412;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B412: move.w #$40, d3
pub fn i_05b412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B416;
    cpu.ir = 0x363C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B416: cmpa.l a1, a2
pub fn i_05b416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B418;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B418: bls.w $5b45c
pub fn i_05b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B41C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B45C; return 10; }
    8
}
/// $05B41C: move.w (a1), d5
pub fn i_05b41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B41E;
    cpu.ir = 0x3A11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B41E: lea.l $4258(a5), a3
pub fn i_05b41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B422;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B422: adda.w d5, a3
pub fn i_05b422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B424;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B424: cmp.w d2, d0
pub fn i_05b424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B426;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B426: bcc.w $5b524
pub fn i_05b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B42A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B524; return 10; }
    8
}
/// $05B42A: cmp.w $6(a0), d0
pub fn i_05b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B42E;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05B42E: bcc.w $5b444
pub fn i_05b42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B432;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B444; return 10; }
    8
}
/// $05B432: move.w #$8201, d7
pub fn i_05b432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B436;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B436: add.w d0, d7
pub fn i_05b436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B438;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B438: swap d7
pub fn i_05b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B43A;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B43A: move.w $8(a3), d7
pub fn i_05b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B43E;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B43E: move.l d7, (a4)
pub fn i_05b43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B440;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B440: bra.w $5b448
pub fn i_05b440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B444;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B448;
    10
}
/// $05B444: bsr.w $5bf76
pub fn i_05b444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B448;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B448);
    cpu.pc = 0x5BF76;
    18
}
/// $05B448: addq.w #$1, d0
pub fn i_05b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B44A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B44A: add.w d3, d1
pub fn i_05b44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B44C;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B44C: tst.b $6(a3)
pub fn i_05b44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B450;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B450: adda.w #$c, a3
pub fn i_05b450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B454;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B454: bpl.b $5b424
pub fn i_05b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B456;
    cpu.ir = 0x6ACE;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B424; return 10; }
    8
}
/// $05B456: adda.w #$4, a1
pub fn i_05b456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B45A: bra.b $5b416
pub fn i_05b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45C;
    cpu.ir = 0x60BA;
    cpu.pc = 0x5B416;
    10
}
/// $05B45C: move.w d2, d1
pub fn i_05b45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45E;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B45E: sub.w d0, d1
pub fn i_05b45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B460;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B460: move.w d1, $a(a0)
pub fn i_05b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B464;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B464: move.w $4(a0), d1
pub fn i_05b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B468;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B468: bpl.w $5b46e
pub fn i_05b468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B46C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B46E; return 10; }
    8
}
/// $05B46C: move.w d2, d1
pub fn i_05b46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B46E;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B46E: cmp.w d0, d1
pub fn i_05b46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B470;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B470: bcc.w $5b476
pub fn i_05b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B474;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B476; return 10; }
    8
}
/// $05B474: move.w d0, d1
pub fn i_05b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B476;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B476: sub.w d1, d2
pub fn i_05b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B478;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B478: move.l #$8201, d7
pub fn i_05b478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B47E;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x8201;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B47E: add.w d1, d7
pub fn i_05b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B480;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B480: swap d7
pub fn i_05b480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B482;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B482: moveq #$1, d6
pub fn i_05b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B484;
    cpu.ir = 0x7C01;
    cpu.d[6] = 0x1;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B484: swap d6
pub fn i_05b484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B486;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B486: bra.w $5b48e
pub fn i_05b486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B48E;
    10
}
/// $05B48A: move.l d7, (a4)
pub fn i_05b48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B48C: add.l d6, d7
pub fn i_05b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48E;
    cpu.ir = 0xDE86;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B48E: dbra d2, $5b48a
pub fn i_05b48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B492;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B492; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B492; return 14; }
    cpu.pc = 0x5B48A;
    10
}
/// $05B492: bra.w $5b51e
pub fn i_05b492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B496;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B51E;
    10
}
/// $05B496: move.w $2(a0), d0
pub fn i_05b496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B49A: move.w d0, d1
pub fn i_05b49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B49C: lsl.w #$6, d1
pub fn i_05b49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49E;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B49E: move.w (a0), d2
pub fn i_05b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A0;
    cpu.ir = 0x3410;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B4A0: move.w #$40, d3
pub fn i_05b4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A4;
    cpu.ir = 0x363C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4A4: cmpa.l a1, a2
pub fn i_05b4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A6;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B4A6: bls.w $5b4e8
pub fn i_05b4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B4E8; return 10; }
    8
}
/// $05B4AA: subq.l #$4, a2
pub fn i_05b4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AC;
    cpu.ir = 0x598A;
    cpu.a[2] = cpu.a[2].wrapping_sub(4);
    8
}
/// $05B4AC: move.w (a2), d5
pub fn i_05b4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AE;
    cpu.ir = 0x3A12;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B4AE: lea.l $4258(a5), a3
pub fn i_05b4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B2;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B4B2: adda.w d5, a3
pub fn i_05b4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B4;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B4B4: cmp.w d0, d2
pub fn i_05b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B6;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B4B6: bcc.w $5b524
pub fn i_05b4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B524; return 10; }
    8
}
/// $05B4BA: subq.w #$1, d0
pub fn i_05b4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BC;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B4BC: sub.w d3, d1
pub fn i_05b4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BE;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4BE: cmp.w $6(a0), d0
pub fn i_05b4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4C2;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05B4C2: bcs.w $5b4d8
pub fn i_05b4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B4D8; return 10; }
    8
}
/// $05B4C6: move.w #$8201, d7
pub fn i_05b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4CA: add.w d0, d7
pub fn i_05b4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CC;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4CC: swap d7
pub fn i_05b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CE;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B4CE: move.w $8(a3), d7
pub fn i_05b4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D2;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B4D2: move.l d7, (a4)
pub fn i_05b4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B4D4: bra.w $5b4dc
pub fn i_05b4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B4DC;
    10
}
/// $05B4D8: bsr.w $5bf76
pub fn i_05b4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B4DC);
    cpu.pc = 0x5BF76;
    18
}
/// $05B4DC: tst.b $6(a3)
pub fn i_05b4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E0;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B4E0: adda.w #$c, a3
pub fn i_05b4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E4;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B4E4: bpl.b $5b4b4
pub fn i_05b4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E6;
    cpu.ir = 0x6ACE;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B4B4; return 10; }
    8
}
/// $05B4E6: bra.b $5b4a4
pub fn i_05b4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E8;
    cpu.ir = 0x60BC;
    cpu.pc = 0x5B4A4;
    10
}
/// $05B4E8: move.w d0, d1
pub fn i_05b4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4EA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4EA: sub.w d2, d1
pub fn i_05b4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4EC;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4EC: move.w d1, $a(a0)
pub fn i_05b4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F0;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B4F0: move.w $4(a0), d1
pub fn i_05b4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F4;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B4F4: bpl.w $5b4fa
pub fn i_05b4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F8;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B4FA; return 10; }
    8
}
/// $05B4F8: move.w d2, d1
pub fn i_05b4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4FA;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4FA: cmp.w d0, d1
pub fn i_05b4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4FC;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B4FC: bcs.w $5b502
pub fn i_05b4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B500;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B502; return 10; }
    8
}
/// $05B500: move.w d0, d1
pub fn i_05b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B502;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B502: sub.w d2, d1
pub fn i_05b502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B504;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B504: move.l #$8201, d7
pub fn i_05b504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50A;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x8201;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B50A: add.w d2, d7
pub fn i_05b50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50C;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B50C: swap d7
pub fn i_05b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B50E: moveq #$1, d6
pub fn i_05b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B510;
    cpu.ir = 0x7C01;
    cpu.d[6] = 0x1;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B510: swap d6
pub fn i_05b510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B512;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B512: bra.w $5b51a
pub fn i_05b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B516;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B51A;
    10
}
/// $05B516: move.l d7, (a4)
pub fn i_05b516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B518;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B518: add.l d6, d7
pub fn i_05b518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B51A;
    cpu.ir = 0xDE86;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B51A: dbra d1, $5b516
pub fn i_05b51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B51E;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B51E; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B51E; return 14; }
    cpu.pc = 0x5B516;
    10
}
/// $05B51E: move.w d0, $4(a0)
pub fn i_05b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B522;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B522: rts
pub fn i_05b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B524: move.w d0, $4(a0)
pub fn i_05b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B528;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B528: clr.w $a(a0)
pub fn i_05b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B52C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0xA);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05B52C: rts
pub fn i_05b52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B52E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SCB1_WriteTileColumn_05b52e  ($05B52E, 2632 B, video/sprite_queue_render_05aa96.s)
/// $05B52E: move.w $6(a3), d7
pub fn i_05b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B532;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B532: addq.b #$1, d7
pub fn i_05b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B534;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B534: sne.b d6
pub fn i_05b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B536;
    cpu.ir = 0x56C6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05B536: andi.b #$4, d6
pub fn i_05b536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B53A;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B53A: rol.w #$7, d7
pub fn i_05b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B53C;
    cpu.ir = 0xEF5F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 3, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $05B53C: andi.w #$8, d7
pub fn i_05b53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B540;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B540: or.b d6, d7
pub fn i_05b540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B542;
    cpu.ir = 0x8E06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05B542: jmp $5b546(pc, d7.w)
pub fn i_05b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B546;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B546_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B546: bra.w $5b556
pub fn i_05b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B54A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B556;
    10
}
/// $05B54A: bra.w $5b962
pub fn i_05b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B54E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B962;
    10
}
/// $05B54E: bra.w $5b75c
pub fn i_05b54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B552;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B75C;
    10
}
/// $05B552: bra.w $5bc6c
pub fn i_05b552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B556;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BC6C;
    10
}
/// $05B556: move.w #$40, d7
pub fn i_05b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B55A: add.w d1, d7
pub fn i_05b55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55C;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B55C: move.w d7, d5
pub fn i_05b55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55E;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B55E: addq.w #$1, d5
pub fn i_05b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B560;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B560: swap d5
pub fn i_05b560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B562;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B562: move.w $4(a3), d5
pub fn i_05b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B566;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B566: swap d7
pub fn i_05b566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B568;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B568: moveq #$2, d4
pub fn i_05b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56A;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B56A: swap d4
pub fn i_05b56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B56C: movea.l (a3), a6
pub fn i_05b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56E;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B56E: move.w $a(a3), d6
pub fn i_05b56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B572;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B572: subq.b #$1, d6
pub fn i_05b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B574;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B574: andi.w #$1f, d6
pub fn i_05b574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B578;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B578: add.w d6, d6
pub fn i_05b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B57A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B57A: add.w d6, d6
pub fn i_05b57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B57C;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B57C: jmp $5b580(pc, d6.w)
pub fn i_05b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B580;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B580_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B580: bra.w $5b732
pub fn i_05b580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B584;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B732;
    10
}
/// $05B584: bra.w $5b728
pub fn i_05b584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B588;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B728;
    10
}
/// $05B588: bra.w $5b71e
pub fn i_05b588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B58C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B71E;
    10
}
/// $05B58C: bra.w $5b714
pub fn i_05b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B590;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B714;
    10
}
/// $05B590: bra.w $5b70a
pub fn i_05b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B594;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B70A;
    10
}
/// $05B594: bra.w $5b700
pub fn i_05b594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B598;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B700;
    10
}
/// $05B598: bra.w $5b6f6
pub fn i_05b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B59C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6F6;
    10
}
/// $05B59C: bra.w $5b6ec
pub fn i_05b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6EC;
    10
}
/// $05B5A0: bra.w $5b6e2
pub fn i_05b5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6E2;
    10
}
/// $05B5A4: bra.w $5b6d8
pub fn i_05b5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6D8;
    10
}
/// $05B5A8: bra.w $5b6ce
pub fn i_05b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6CE;
    10
}
/// $05B5AC: bra.w $5b6c4
pub fn i_05b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6C4;
    10
}
/// $05B5B0: bra.w $5b6ba
pub fn i_05b5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6BA;
    10
}
/// $05B5B4: bra.w $5b6b0
pub fn i_05b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6B0;
    10
}
/// $05B5B8: bra.w $5b6a6
pub fn i_05b5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6A6;
    10
}
/// $05B5BC: bra.w $5b69c
pub fn i_05b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B69C;
    10
}
/// $05B5DC: bra.w $5b64c
pub fn i_05b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B64C;
    10
}
/// $05B64C: move.w (a6)+, d7
pub fn i_05b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B64E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B64E: move.l d7, (a4)
pub fn i_05b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B650;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B650: add.l d4, d7
pub fn i_05b650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B652;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B652: move.l d5, (a4)
pub fn i_05b652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B654;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B654: add.l d4, d5
pub fn i_05b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B656;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B656: move.w (a6)+, d7
pub fn i_05b656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B658;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B658: move.l d7, (a4)
pub fn i_05b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B65A: add.l d4, d7
pub fn i_05b65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B65C: move.l d5, (a4)
pub fn i_05b65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B65E: add.l d4, d5
pub fn i_05b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B660;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B660: move.w (a6)+, d7
pub fn i_05b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B662;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B662: move.l d7, (a4)
pub fn i_05b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B664;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B664: add.l d4, d7
pub fn i_05b664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B666;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B666: move.l d5, (a4)
pub fn i_05b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B668;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B668: add.l d4, d5
pub fn i_05b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B66A: move.w (a6)+, d7
pub fn i_05b66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B66C: move.l d7, (a4)
pub fn i_05b66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B66E: add.l d4, d7
pub fn i_05b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B670;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B670: move.l d5, (a4)
pub fn i_05b670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B672;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B672: add.l d4, d5
pub fn i_05b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B674;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B674: move.w (a6)+, d7
pub fn i_05b674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B676;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B676: move.l d7, (a4)
pub fn i_05b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B678;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B678: add.l d4, d7
pub fn i_05b678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B67A: move.l d5, (a4)
pub fn i_05b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B67C: add.l d4, d5
pub fn i_05b67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B67E: move.w (a6)+, d7
pub fn i_05b67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B680;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B680: move.l d7, (a4)
pub fn i_05b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B682;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B682: add.l d4, d7
pub fn i_05b682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B684;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B684: move.l d5, (a4)
pub fn i_05b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B686;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B686: add.l d4, d5
pub fn i_05b686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B688;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B688: move.w (a6)+, d7
pub fn i_05b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B68A: move.l d7, (a4)
pub fn i_05b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B68C: add.l d4, d7
pub fn i_05b68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B68E: move.l d5, (a4)
pub fn i_05b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B690;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B690: add.l d4, d5
pub fn i_05b690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B692;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B692: move.w (a6)+, d7
pub fn i_05b692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B694;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B694: move.l d7, (a4)
pub fn i_05b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B696;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B696: add.l d4, d7
pub fn i_05b696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B698;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B698: move.l d5, (a4)
pub fn i_05b698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B69A: add.l d4, d5
pub fn i_05b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B69C: move.w (a6)+, d7
pub fn i_05b69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B69E: move.l d7, (a4)
pub fn i_05b69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6A0: add.l d4, d7
pub fn i_05b6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6A2: move.l d5, (a4)
pub fn i_05b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6A4: add.l d4, d5
pub fn i_05b6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6A6: move.w (a6)+, d7
pub fn i_05b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6A8: move.l d7, (a4)
pub fn i_05b6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6AA: add.l d4, d7
pub fn i_05b6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6AC: move.l d5, (a4)
pub fn i_05b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6AE: add.l d4, d5
pub fn i_05b6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6B0: move.w (a6)+, d7
pub fn i_05b6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B2;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6B2: move.l d7, (a4)
pub fn i_05b6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6B4: add.l d4, d7
pub fn i_05b6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6B6: move.l d5, (a4)
pub fn i_05b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6B8: add.l d4, d5
pub fn i_05b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6BA: move.w (a6)+, d7
pub fn i_05b6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6BC: move.l d7, (a4)
pub fn i_05b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6BE: add.l d4, d7
pub fn i_05b6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6C0: move.l d5, (a4)
pub fn i_05b6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6C2: add.l d4, d5
pub fn i_05b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6C4: move.w (a6)+, d7
pub fn i_05b6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6C6: move.l d7, (a4)
pub fn i_05b6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6C8: add.l d4, d7
pub fn i_05b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6CA: move.l d5, (a4)
pub fn i_05b6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6CC: add.l d4, d5
pub fn i_05b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6CE: move.w (a6)+, d7
pub fn i_05b6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6D0: move.l d7, (a4)
pub fn i_05b6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6D2: add.l d4, d7
pub fn i_05b6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6D4: move.l d5, (a4)
pub fn i_05b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6D6: add.l d4, d5
pub fn i_05b6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6D8: move.w (a6)+, d7
pub fn i_05b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6DA: move.l d7, (a4)
pub fn i_05b6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6DC: add.l d4, d7
pub fn i_05b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6DE: move.l d5, (a4)
pub fn i_05b6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6E0: add.l d4, d5
pub fn i_05b6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6E2: move.w (a6)+, d7
pub fn i_05b6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E4;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6E4: move.l d7, (a4)
pub fn i_05b6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6E6: add.l d4, d7
pub fn i_05b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6E8: move.l d5, (a4)
pub fn i_05b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6EA: add.l d4, d5
pub fn i_05b6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6EC: move.w (a6)+, d7
pub fn i_05b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6EE: move.l d7, (a4)
pub fn i_05b6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6F0: add.l d4, d7
pub fn i_05b6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6F2: move.l d5, (a4)
pub fn i_05b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6F4: add.l d4, d5
pub fn i_05b6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6F6: move.w (a6)+, d7
pub fn i_05b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6F8: move.l d7, (a4)
pub fn i_05b6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6FA: add.l d4, d7
pub fn i_05b6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6FC: move.l d5, (a4)
pub fn i_05b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6FE: add.l d4, d5
pub fn i_05b6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B700;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B700: move.w (a6)+, d7
pub fn i_05b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B702;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B702: move.l d7, (a4)
pub fn i_05b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B704;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B704: add.l d4, d7
pub fn i_05b704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B706;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B706: move.l d5, (a4)
pub fn i_05b706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B708;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B708: add.l d4, d5
pub fn i_05b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B70A: move.w (a6)+, d7
pub fn i_05b70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B70C: move.l d7, (a4)
pub fn i_05b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B70E: add.l d4, d7
pub fn i_05b70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B710;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B710: move.l d5, (a4)
pub fn i_05b710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B712;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B712: add.l d4, d5
pub fn i_05b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B714;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B714: move.w (a6)+, d7
pub fn i_05b714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B716;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B716: move.l d7, (a4)
pub fn i_05b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B718;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B718: add.l d4, d7
pub fn i_05b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B71A: move.l d5, (a4)
pub fn i_05b71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B71C: add.l d4, d5
pub fn i_05b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B71E: move.w (a6)+, d7
pub fn i_05b71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B720;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B720: move.l d7, (a4)
pub fn i_05b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B722;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B722: add.l d4, d7
pub fn i_05b722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B724;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B724: move.l d5, (a4)
pub fn i_05b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B726;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B726: add.l d4, d5
pub fn i_05b726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B728;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B728: move.w (a6)+, d7
pub fn i_05b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B72A: move.l d7, (a4)
pub fn i_05b72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B72C: add.l d4, d7
pub fn i_05b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B72E: move.l d5, (a4)
pub fn i_05b72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B730;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B730: add.l d4, d5
pub fn i_05b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B732;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B732: move.w (a6)+, d7
pub fn i_05b732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B734;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B734: move.l d7, (a4)
pub fn i_05b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B736;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B736: add.l d4, d7
pub fn i_05b736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B738;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B738: move.l d5, (a4)
pub fn i_05b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B73A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B73A: add.l d4, d5
pub fn i_05b73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B73C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B73C: move.w #$8401, d6
pub fn i_05b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B740;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B740: add.w d0, d6
pub fn i_05b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B742;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B742: swap d6
pub fn i_05b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B744;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B744: move.w $a(a3), d6
pub fn i_05b744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B748;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B748: move.l #$2000000, d5
pub fn i_05b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B74E;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B74E: move.l d6, (a4)
pub fn i_05b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B750;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B750: sub.l d5, d6
pub fn i_05b750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B752;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B752: sub.l d5, d6
pub fn i_05b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B754;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B754: move.w $6(a3), d6
pub fn i_05b754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B758;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B758: move.l d6, (a4)
pub fn i_05b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B75A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B75A: rts
pub fn i_05b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B75C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B75C: move.w #$40, d7
pub fn i_05b75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B760;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B760: add.w d1, d7
pub fn i_05b760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B762;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B762: move.w d7, d5
pub fn i_05b762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B764;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B764: addq.w #$1, d5
pub fn i_05b764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B766;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B766: swap d5
pub fn i_05b766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B768;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B768: move.w $4(a3), d5
pub fn i_05b768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B76C;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B76C: swap d7
pub fn i_05b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B76E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B76E: moveq #$2, d4
pub fn i_05b76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B770;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B770: swap d4
pub fn i_05b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B772;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B772: movea.l (a3), a6
pub fn i_05b772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B774;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B774: move.w $a(a3), d6
pub fn i_05b774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B778;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B778: subq.b #$1, d6
pub fn i_05b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B77A;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B77A: andi.w #$1f, d6
pub fn i_05b77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B77E;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B77E: add.w d6, d6
pub fn i_05b77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B780;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B780: add.w d6, d6
pub fn i_05b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B782;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B782: jmp $5b786(pc, d6.w)
pub fn i_05b782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B786;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B786_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B786: bra.w $5b938
pub fn i_05b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B78A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B938;
    10
}
/// $05B78A: bra.w $5b92e
pub fn i_05b78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B78E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B92E;
    10
}
/// $05B78E: bra.w $5b924
pub fn i_05b78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B792;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B924;
    10
}
/// $05B792: bra.w $5b91a
pub fn i_05b792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B796;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B91A;
    10
}
/// $05B796: bra.w $5b910
pub fn i_05b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B79A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B910;
    10
}
/// $05B79A: bra.w $5b906
pub fn i_05b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B79E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B906;
    10
}
/// $05B79E: bra.w $5b8fc
pub fn i_05b79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8FC;
    10
}
/// $05B7A2: bra.w $5b8f2
pub fn i_05b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8F2;
    10
}
/// $05B7A6: bra.w $5b8e8
pub fn i_05b7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8E8;
    10
}
/// $05B7AA: bra.w $5b8de
pub fn i_05b7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8DE;
    10
}
/// $05B8DE: move.w -(a6), d7
pub fn i_05b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8E0: move.l d7, (a4)
pub fn i_05b8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8E2: add.l d4, d7
pub fn i_05b8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8E4: move.l d5, (a4)
pub fn i_05b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8E6: add.l d4, d5
pub fn i_05b8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8E8: move.w -(a6), d7
pub fn i_05b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8EA: move.l d7, (a4)
pub fn i_05b8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8EC: add.l d4, d7
pub fn i_05b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8EE: move.l d5, (a4)
pub fn i_05b8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8F0: add.l d4, d5
pub fn i_05b8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8F2: move.w -(a6), d7
pub fn i_05b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8F4: move.l d7, (a4)
pub fn i_05b8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8F6: add.l d4, d7
pub fn i_05b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8F8: move.l d5, (a4)
pub fn i_05b8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8FA: add.l d4, d5
pub fn i_05b8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8FC: move.w -(a6), d7
pub fn i_05b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8FE: move.l d7, (a4)
pub fn i_05b8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B900;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B900: add.l d4, d7
pub fn i_05b900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B902;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B902: move.l d5, (a4)
pub fn i_05b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B904;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B904: add.l d4, d5
pub fn i_05b904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B906;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B906: move.w -(a6), d7
pub fn i_05b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B908;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B908: move.l d7, (a4)
pub fn i_05b908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B90A: add.l d4, d7
pub fn i_05b90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B90C: move.l d5, (a4)
pub fn i_05b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B90E: add.l d4, d5
pub fn i_05b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B910;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B910: move.w -(a6), d7
pub fn i_05b910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B912;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B912: move.l d7, (a4)
pub fn i_05b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B914;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B914: add.l d4, d7
pub fn i_05b914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B916;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B916: move.l d5, (a4)
pub fn i_05b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B918;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B918: add.l d4, d5
pub fn i_05b918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B91A: move.w -(a6), d7
pub fn i_05b91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B91C: move.l d7, (a4)
pub fn i_05b91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B91E: add.l d4, d7
pub fn i_05b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B920;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B920: move.l d5, (a4)
pub fn i_05b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B922;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B922: add.l d4, d5
pub fn i_05b922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B924;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B924: move.w -(a6), d7
pub fn i_05b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B926;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B926: move.l d7, (a4)
pub fn i_05b926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B928;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B928: add.l d4, d7
pub fn i_05b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B92A: move.l d5, (a4)
pub fn i_05b92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B92C: add.l d4, d5
pub fn i_05b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B92E: move.w -(a6), d7
pub fn i_05b92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B930;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B930: move.l d7, (a4)
pub fn i_05b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B932;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B932: add.l d4, d7
pub fn i_05b932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B934;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B934: move.l d5, (a4)
pub fn i_05b934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B936;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B936: add.l d4, d5
pub fn i_05b936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B938;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B938: move.w -(a6), d7
pub fn i_05b938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B93A: move.l d7, (a4)
pub fn i_05b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B93C: add.l d4, d7
pub fn i_05b93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B93E: move.l d5, (a4)
pub fn i_05b93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B940;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B940: add.l d4, d5
pub fn i_05b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B942;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B942: move.w #$8401, d6
pub fn i_05b942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B946;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B946: add.w d0, d6
pub fn i_05b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B948;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B948: swap d6
pub fn i_05b948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B94A;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B94A: move.w $a(a3), d6
pub fn i_05b94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B94E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B94E: move.l #$2000000, d5
pub fn i_05b94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B954;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B954: move.l d6, (a4)
pub fn i_05b954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B956;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B956: sub.l d5, d6
pub fn i_05b956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B958;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B958: sub.l d5, d6
pub fn i_05b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B95A;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B95A: move.w $6(a3), d6
pub fn i_05b95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B95E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B95E: move.l d6, (a4)
pub fn i_05b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B960;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B960: rts
pub fn i_05b960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B962;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B962: move.w #$40, d7
pub fn i_05b962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B966;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B966: add.w d1, d7
pub fn i_05b966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B968;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B968: move.w d7, d5
pub fn i_05b968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96A;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B96A: addq.w #$1, d5
pub fn i_05b96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96C;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B96C: swap d5
pub fn i_05b96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B96E: move.w $4(a3), d5
pub fn i_05b96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B972;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B972: swap d7
pub fn i_05b972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B974;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B974: moveq #$2, d4
pub fn i_05b974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B976;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B976: swap d4
pub fn i_05b976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B978;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B978: movea.l (a3), a6
pub fn i_05b978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B97A;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B97A: move.w $a(a3), d6
pub fn i_05b97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B97E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B97E: subq.b #$1, d6
pub fn i_05b97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B980;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B980: andi.w #$1f, d6
pub fn i_05b980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B984;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B984: add.w d6, d6
pub fn i_05b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B986;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B986: add.w d6, d6
pub fn i_05b986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B988;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B988: jmp $5b98c(pc, d6.w)
pub fn i_05b988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B98C;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B98C_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B98C: bra.w $5bb3e
pub fn i_05b98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B990;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB3E;
    10
}
/// $05B990: bra.w $5bb34
pub fn i_05b990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B994;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB34;
    10
}
/// $05B994: bra.w $5bb2a
pub fn i_05b994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B998;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB2A;
    10
}
/// $05B998: bra.w $5bb20
pub fn i_05b998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B99C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB20;
    10
}
/// $05B99C: bra.w $5bb16
pub fn i_05b99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB16;
    10
}
/// $05B9A0: bra.w $5bb0c
pub fn i_05b9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB0C;
    10
}
/// $05B9A4: bra.w $5bb02
pub fn i_05b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB02;
    10
}
/// $05B9A8: bra.w $5baf8
pub fn i_05b9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAF8;
    10
}
/// $05B9AC: bra.w $5baee
pub fn i_05b9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAEE;
    10
}
/// $05B9B0: bra.w $5bae4
pub fn i_05b9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAE4;
    10
}
/// $05B9B4: bra.w $5bada
pub fn i_05b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BADA;
    10
}
/// $05BADA: move.w (a6)+, d7
pub fn i_05bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BADC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BADC: move.l d7, (a4)
pub fn i_05badc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BADE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BADE: add.l d4, d7
pub fn i_05bade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAE0: move.l d5, (a4)
pub fn i_05bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAE2: add.l d4, d5
pub fn i_05bae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAE4: move.w (a6)+, d7
pub fn i_05bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAE6: move.l d7, (a4)
pub fn i_05bae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAE8: add.l d4, d7
pub fn i_05bae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAEA: move.l d5, (a4)
pub fn i_05baea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAEC: add.l d4, d5
pub fn i_05baec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAEE: move.w (a6)+, d7
pub fn i_05baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAF0: move.l d7, (a4)
pub fn i_05baf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAF2: add.l d4, d7
pub fn i_05baf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAF4: move.l d5, (a4)
pub fn i_05baf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAF6: add.l d4, d5
pub fn i_05baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAF8: move.w (a6)+, d7
pub fn i_05baf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAFA: move.l d7, (a4)
pub fn i_05bafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAFC: add.l d4, d7
pub fn i_05bafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAFE: move.l d5, (a4)
pub fn i_05bafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB00;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB00: add.l d4, d5
pub fn i_05bb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB02;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB02: move.w (a6)+, d7
pub fn i_05bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB04;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB04: move.l d7, (a4)
pub fn i_05bb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB06: add.l d4, d7
pub fn i_05bb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB08: move.l d5, (a4)
pub fn i_05bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB0A: add.l d4, d5
pub fn i_05bb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB0C: move.w (a6)+, d7
pub fn i_05bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB0E: move.l d7, (a4)
pub fn i_05bb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB10: add.l d4, d7
pub fn i_05bb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB12: move.l d5, (a4)
pub fn i_05bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB14;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB14: add.l d4, d5
pub fn i_05bb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB16;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB16: move.w (a6)+, d7
pub fn i_05bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB18;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB18: move.l d7, (a4)
pub fn i_05bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB1A: add.l d4, d7
pub fn i_05bb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB1C: move.l d5, (a4)
pub fn i_05bb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB1E: add.l d4, d5
pub fn i_05bb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB20;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB20: move.w (a6)+, d7
pub fn i_05bb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB22;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB22: move.l d7, (a4)
pub fn i_05bb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB24: add.l d4, d7
pub fn i_05bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB26: move.l d5, (a4)
pub fn i_05bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB28;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB28: add.l d4, d5
pub fn i_05bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB2A: move.w (a6)+, d7
pub fn i_05bb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB2C: move.l d7, (a4)
pub fn i_05bb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB2E: add.l d4, d7
pub fn i_05bb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB30: move.l d5, (a4)
pub fn i_05bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB32;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB32: add.l d4, d5
pub fn i_05bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB34;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB34: move.w (a6)+, d7
pub fn i_05bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB36;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB36: move.l d7, (a4)
pub fn i_05bb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB38: add.l d4, d7
pub fn i_05bb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB3A: move.l d5, (a4)
pub fn i_05bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB3C: add.l d4, d5
pub fn i_05bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB3E: move.w (a6)+, d7
pub fn i_05bb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB40;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB40: move.l d7, (a4)
pub fn i_05bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB42: add.l d4, d7
pub fn i_05bb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB44: move.l d5, (a4)
pub fn i_05bb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB46;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB46: add.l d4, d5
pub fn i_05bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB48;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB48: clr.w d7
pub fn i_05bb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BB4A: moveq #$1, d4
pub fn i_05bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4C;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BB4C: swap d4
pub fn i_05bb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4E;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BB4E: add.w d6, d6
pub fn i_05bb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB50;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BB50: jmp $5bb54(pc, d6.w)
pub fn i_05bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB54;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BB54_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BB54: move.l d7, (a4)
pub fn i_05bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB56;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB56: add.l d4, d7
pub fn i_05bb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB58;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB58: move.l d7, (a4)
pub fn i_05bb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB5A: add.l d4, d7
pub fn i_05bb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB5C: move.l d7, (a4)
pub fn i_05bb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB5E: add.l d4, d7
pub fn i_05bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB60;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB60: move.l d7, (a4)
pub fn i_05bb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB62;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB62: add.l d4, d7
pub fn i_05bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB64;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB64: move.l d7, (a4)
pub fn i_05bb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB66;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB66: add.l d4, d7
pub fn i_05bb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB68;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB68: move.l d7, (a4)
pub fn i_05bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB6A: add.l d4, d7
pub fn i_05bb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB6C: move.l d7, (a4)
pub fn i_05bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB6E: add.l d4, d7
pub fn i_05bb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB70;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB70: move.l d7, (a4)
pub fn i_05bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB72;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB72: add.l d4, d7
pub fn i_05bb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB74;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB74: move.l d7, (a4)
pub fn i_05bb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB76;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB76: add.l d4, d7
pub fn i_05bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB78;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB78: move.l d7, (a4)
pub fn i_05bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB7A: add.l d4, d7
pub fn i_05bb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB7C: move.l d7, (a4)
pub fn i_05bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB7E: add.l d4, d7
pub fn i_05bb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB80;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB80: move.l d7, (a4)
pub fn i_05bb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB82;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB82: add.l d4, d7
pub fn i_05bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB84;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB84: move.l d7, (a4)
pub fn i_05bb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB86;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB86: add.l d4, d7
pub fn i_05bb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB88;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB88: move.l d7, (a4)
pub fn i_05bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB8A: add.l d4, d7
pub fn i_05bb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB8C: move.l d7, (a4)
pub fn i_05bb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB8E: add.l d4, d7
pub fn i_05bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB90;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB90: move.l d7, (a4)
pub fn i_05bb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB92;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB92: add.l d4, d7
pub fn i_05bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB94;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB94: move.l d7, (a4)
pub fn i_05bb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB96;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB96: add.l d4, d7
pub fn i_05bb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB98;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB98: move.l d7, (a4)
pub fn i_05bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB9A: add.l d4, d7
pub fn i_05bb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB9C: move.l d7, (a4)
pub fn i_05bb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB9E: add.l d4, d7
pub fn i_05bb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA0: move.l d7, (a4)
pub fn i_05bba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBA2: add.l d4, d7
pub fn i_05bba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA4: move.l d7, (a4)
pub fn i_05bba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBA6: add.l d4, d7
pub fn i_05bba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA8: move.l d7, (a4)
pub fn i_05bba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBAA: add.l d4, d7
pub fn i_05bbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBAC: move.l d7, (a4)
pub fn i_05bbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBAE: add.l d4, d7
pub fn i_05bbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB0: move.l d7, (a4)
pub fn i_05bbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBB2: add.l d4, d7
pub fn i_05bbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB4: move.l d7, (a4)
pub fn i_05bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBB6: add.l d4, d7
pub fn i_05bbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB8: move.l d7, (a4)
pub fn i_05bbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBBA: add.l d4, d7
pub fn i_05bbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBBC: move.l d7, (a4)
pub fn i_05bbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBBE: add.l d4, d7
pub fn i_05bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC0: move.l d7, (a4)
pub fn i_05bbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBC2: add.l d4, d7
pub fn i_05bbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC4: move.l d7, (a4)
pub fn i_05bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBC6: add.l d4, d7
pub fn i_05bbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC8: move.l d7, (a4)
pub fn i_05bbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBCA: add.l d4, d7
pub fn i_05bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBCC: move.l d7, (a4)
pub fn i_05bbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBCE: add.l d4, d7
pub fn i_05bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD0: move.l d7, (a4)
pub fn i_05bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBD2: add.l d4, d7
pub fn i_05bbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD4: move.l d7, (a4)
pub fn i_05bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBD6: add.l d4, d7
pub fn i_05bbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD8: move.l d7, (a4)
pub fn i_05bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBDA: add.l d4, d7
pub fn i_05bbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBDC: move.l d7, (a4)
pub fn i_05bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBDE: add.l d4, d7
pub fn i_05bbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE0: move.l d7, (a4)
pub fn i_05bbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBE2: add.l d4, d7
pub fn i_05bbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE4: move.l d7, (a4)
pub fn i_05bbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBE6: add.l d4, d7
pub fn i_05bbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE8: move.l d7, (a4)
pub fn i_05bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBEA: add.l d4, d7
pub fn i_05bbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBEC: move.l d7, (a4)
pub fn i_05bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBEE: add.l d4, d7
pub fn i_05bbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF0: move.l d7, (a4)
pub fn i_05bbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBF2: add.l d4, d7
pub fn i_05bbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF4: move.l d7, (a4)
pub fn i_05bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBF6: add.l d4, d7
pub fn i_05bbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF8: move.l d7, (a4)
pub fn i_05bbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBFA: add.l d4, d7
pub fn i_05bbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBFC: move.l d7, (a4)
pub fn i_05bbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBFE: add.l d4, d7
pub fn i_05bbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC00;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC00: move.l d7, (a4)
pub fn i_05bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC02;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC02: add.l d4, d7
pub fn i_05bc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC04;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC04: move.l d7, (a4)
pub fn i_05bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC06: add.l d4, d7
pub fn i_05bc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC08: move.l d7, (a4)
pub fn i_05bc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC0A: add.l d4, d7
pub fn i_05bc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC0C: move.l d7, (a4)
pub fn i_05bc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC0E: add.l d4, d7
pub fn i_05bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC10;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC10: move.l d7, (a4)
pub fn i_05bc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC12;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC12: add.l d4, d7
pub fn i_05bc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC14;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC14: move.l d7, (a4)
pub fn i_05bc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC16;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC16: add.l d4, d7
pub fn i_05bc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC18;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC18: move.l d7, (a4)
pub fn i_05bc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC1A: add.l d4, d7
pub fn i_05bc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC1C: move.l d7, (a4)
pub fn i_05bc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC1E: add.l d4, d7
pub fn i_05bc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC20;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC20: move.l d7, (a4)
pub fn i_05bc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC22;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC22: add.l d4, d7
pub fn i_05bc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC24;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC24: move.l d7, (a4)
pub fn i_05bc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC26;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC26: add.l d4, d7
pub fn i_05bc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC28;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC28: move.l d7, (a4)
pub fn i_05bc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC2A: add.l d4, d7
pub fn i_05bc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC2C: move.l d7, (a4)
pub fn i_05bc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC2E: add.l d4, d7
pub fn i_05bc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC30: move.l d7, (a4)
pub fn i_05bc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC32;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC32: add.l d4, d7
pub fn i_05bc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC34;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC34: move.l d7, (a4)
pub fn i_05bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC36;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC36: add.l d4, d7
pub fn i_05bc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC38;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC38: move.l d7, (a4)
pub fn i_05bc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC3A: add.l d4, d7
pub fn i_05bc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC3C: move.l d7, (a4)
pub fn i_05bc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC3E: add.l d4, d7
pub fn i_05bc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC40;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC40: move.l d7, (a4)
pub fn i_05bc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC42: add.l d4, d7
pub fn i_05bc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC44: move.l d7, (a4)
pub fn i_05bc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC46;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC46: add.l d4, d7
pub fn i_05bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC48;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC48: move.l d7, (a4)
pub fn i_05bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC4A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC4A: add.l d4, d7
pub fn i_05bc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC4C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC4C: move.w #$8401, d6
pub fn i_05bc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC50;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC50: add.w d0, d6
pub fn i_05bc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC52;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC52: swap d6
pub fn i_05bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC54;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC54: move.w $a(a3), d6
pub fn i_05bc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC58;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC58: move.l #$2000000, d5
pub fn i_05bc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC5E;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC5E: move.l d6, (a4)
pub fn i_05bc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC60;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC60: sub.l d5, d6
pub fn i_05bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC62;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BC62: sub.l d5, d6
pub fn i_05bc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC64;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BC64: move.w $6(a3), d6
pub fn i_05bc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC68;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC68: move.l d6, (a4)
pub fn i_05bc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC6A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC6A: rts
pub fn i_05bc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05BC6C: move.w #$40, d7
pub fn i_05bc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC70;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC70: add.w d1, d7
pub fn i_05bc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC72;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC72: move.w d7, d5
pub fn i_05bc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC74;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC74: addq.w #$1, d5
pub fn i_05bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC76;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BC76: swap d5
pub fn i_05bc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC78;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC78: move.w $4(a3), d5
pub fn i_05bc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC7C;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC7C: swap d7
pub fn i_05bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC7E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC7E: moveq #$2, d4
pub fn i_05bc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC80;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC80: swap d4
pub fn i_05bc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC82;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC82: movea.l (a3), a6
pub fn i_05bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC84;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05BC84: move.w $a(a3), d6
pub fn i_05bc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC88;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC88: subq.b #$1, d6
pub fn i_05bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC8A;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BC8A: andi.w #$1f, d6
pub fn i_05bc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC8E;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BC8E: add.w d6, d6
pub fn i_05bc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC90;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC90: add.w d6, d6
pub fn i_05bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC92;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC92: jmp $5bc96(pc, d6.w)
pub fn i_05bc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC96;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BC96_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BC96: bra.w $5be48
pub fn i_05bc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE48;
    10
}
/// $05BC9A: bra.w $5be3e
pub fn i_05bc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE3E;
    10
}
/// $05BC9E: bra.w $5be34
pub fn i_05bc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE34;
    10
}
/// $05BCA2: bra.w $5be2a
pub fn i_05bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE2A;
    10
}
/// $05BCA6: bra.w $5be20
pub fn i_05bca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE20;
    10
}
/// $05BCAA: bra.w $5be16
pub fn i_05bcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE16;
    10
}
/// $05BCAE: bra.w $5be0c
pub fn i_05bcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE0C;
    10
}
/// $05BCB2: bra.w $5be02
pub fn i_05bcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE02;
    10
}
/// $05BCB6: bra.w $5bdf8
pub fn i_05bcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCBA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDF8;
    10
}
/// $05BCBA: bra.w $5bdee
pub fn i_05bcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDEE;
    10
}
/// $05BCBE: bra.w $5bde4
pub fn i_05bcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDE4;
    10
}
/// $05BCCA: bra.w $5bdc6
pub fn i_05bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCCE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDC6;
    10
}
/// $05BCCE: bra.w $5bdbc
pub fn i_05bcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDBC;
    10
}
/// $05BDBC: move.w -(a6), d7
pub fn i_05bdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDBE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDBE: move.l d7, (a4)
pub fn i_05bdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDC0: add.l d4, d7
pub fn i_05bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDC2: move.l d5, (a4)
pub fn i_05bdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDC4: add.l d4, d5
pub fn i_05bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDC6: move.w -(a6), d7
pub fn i_05bdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC8;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDC8: move.l d7, (a4)
pub fn i_05bdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDCA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDCA: add.l d4, d7
pub fn i_05bdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDCC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDCC: move.l d5, (a4)
pub fn i_05bdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDCE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDCE: add.l d4, d5
pub fn i_05bdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDD0: move.w -(a6), d7
pub fn i_05bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD2;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDD2: move.l d7, (a4)
pub fn i_05bdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDD4: add.l d4, d7
pub fn i_05bdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDD6: move.l d5, (a4)
pub fn i_05bdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDD8: add.l d4, d5
pub fn i_05bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDDA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDDA: move.w -(a6), d7
pub fn i_05bdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDDC;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDDC: move.l d7, (a4)
pub fn i_05bddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDDE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDDE: add.l d4, d7
pub fn i_05bdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDE0: move.l d5, (a4)
pub fn i_05bde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDE2: add.l d4, d5
pub fn i_05bde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDE4: move.w -(a6), d7
pub fn i_05bde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE6;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDE6: move.l d7, (a4)
pub fn i_05bde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDE8: add.l d4, d7
pub fn i_05bde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDEA: move.l d5, (a4)
pub fn i_05bdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDEC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDEC: add.l d4, d5
pub fn i_05bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDEE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDEE: move.w -(a6), d7
pub fn i_05bdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDF0: move.l d7, (a4)
pub fn i_05bdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDF2: add.l d4, d7
pub fn i_05bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDF4: move.l d5, (a4)
pub fn i_05bdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDF6: add.l d4, d5
pub fn i_05bdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDF8: move.w -(a6), d7
pub fn i_05bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDFA: move.l d7, (a4)
pub fn i_05bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDFC: add.l d4, d7
pub fn i_05bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDFE: move.l d5, (a4)
pub fn i_05bdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE00;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE00: add.l d4, d5
pub fn i_05be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE02;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE02: move.w -(a6), d7
pub fn i_05be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE04;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE04: move.l d7, (a4)
pub fn i_05be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE06: add.l d4, d7
pub fn i_05be06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE08: move.l d5, (a4)
pub fn i_05be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE0A: add.l d4, d5
pub fn i_05be0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE0C: move.w -(a6), d7
pub fn i_05be0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE0E: move.l d7, (a4)
pub fn i_05be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE10: add.l d4, d7
pub fn i_05be10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE12: move.l d5, (a4)
pub fn i_05be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE14;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE14: add.l d4, d5
pub fn i_05be14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE16;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE16: move.w -(a6), d7
pub fn i_05be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE18;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE18: move.l d7, (a4)
pub fn i_05be18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE1A: add.l d4, d7
pub fn i_05be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE1C: move.l d5, (a4)
pub fn i_05be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE1E: add.l d4, d5
pub fn i_05be1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE20;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE20: move.w -(a6), d7
pub fn i_05be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE22;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE22: move.l d7, (a4)
pub fn i_05be22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE24: add.l d4, d7
pub fn i_05be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE26: move.l d5, (a4)
pub fn i_05be26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE28;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE28: add.l d4, d5
pub fn i_05be28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE2A: move.w -(a6), d7
pub fn i_05be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE2C: move.l d7, (a4)
pub fn i_05be2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE2E: add.l d4, d7
pub fn i_05be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE30: move.l d5, (a4)
pub fn i_05be30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE32;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE32: add.l d4, d5
pub fn i_05be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE34;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE34: move.w -(a6), d7
pub fn i_05be34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE36;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE36: move.l d7, (a4)
pub fn i_05be36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE38: add.l d4, d7
pub fn i_05be38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE3A: move.l d5, (a4)
pub fn i_05be3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE3C: add.l d4, d5
pub fn i_05be3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE3E: move.w -(a6), d7
pub fn i_05be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE40;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE40: move.l d7, (a4)
pub fn i_05be40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE42: add.l d4, d7
pub fn i_05be42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE44: move.l d5, (a4)
pub fn i_05be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE46;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE46: add.l d4, d5
pub fn i_05be46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE48;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE48: move.w -(a6), d7
pub fn i_05be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE4A: move.l d7, (a4)
pub fn i_05be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE4C: add.l d4, d7
pub fn i_05be4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE4E: move.l d5, (a4)
pub fn i_05be4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE50;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE50: add.l d4, d5
pub fn i_05be50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE52;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE52: clr.w d7
pub fn i_05be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE54;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BE54: moveq #$1, d4
pub fn i_05be54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE56;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BE56: swap d4
pub fn i_05be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE58;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BE58: add.w d6, d6
pub fn i_05be58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE5A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BE5A: jmp $5be5e(pc, d6.w)
pub fn i_05be5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE5E;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BE5E_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BE5E: move.l d7, (a4)
pub fn i_05be5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE60;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE60: add.l d4, d7
pub fn i_05be60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE62;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE62: move.l d7, (a4)
pub fn i_05be62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE64;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE64: add.l d4, d7
pub fn i_05be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE66;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE66: move.l d7, (a4)
pub fn i_05be66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE68;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE68: add.l d4, d7
pub fn i_05be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE6A: move.l d7, (a4)
pub fn i_05be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE6C: add.l d4, d7
pub fn i_05be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE6E: move.l d7, (a4)
pub fn i_05be6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE70;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE70: add.l d4, d7
pub fn i_05be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE72;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE72: move.l d7, (a4)
pub fn i_05be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE74;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE74: add.l d4, d7
pub fn i_05be74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE76;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE76: move.l d7, (a4)
pub fn i_05be76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE78;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE78: add.l d4, d7
pub fn i_05be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE7A: move.l d7, (a4)
pub fn i_05be7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE7C: add.l d4, d7
pub fn i_05be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE7E: move.l d7, (a4)
pub fn i_05be7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE80;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE80: add.l d4, d7
pub fn i_05be80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE82;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE82: move.l d7, (a4)
pub fn i_05be82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE84;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE84: add.l d4, d7
pub fn i_05be84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE86;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE86: move.l d7, (a4)
pub fn i_05be86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE88;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE88: add.l d4, d7
pub fn i_05be88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE8A: move.l d7, (a4)
pub fn i_05be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE8C: add.l d4, d7
pub fn i_05be8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE8E: move.l d7, (a4)
pub fn i_05be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE90;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE90: add.l d4, d7
pub fn i_05be90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE92;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE92: move.l d7, (a4)
pub fn i_05be92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE94;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE94: add.l d4, d7
pub fn i_05be94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE96;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE96: move.l d7, (a4)
pub fn i_05be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE98;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE98: add.l d4, d7
pub fn i_05be98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE9A: move.l d7, (a4)
pub fn i_05be9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE9C: add.l d4, d7
pub fn i_05be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE9E: move.l d7, (a4)
pub fn i_05be9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA0: add.l d4, d7
pub fn i_05bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEA2: move.l d7, (a4)
pub fn i_05bea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA4: add.l d4, d7
pub fn i_05bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEA6: move.l d7, (a4)
pub fn i_05bea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA8: add.l d4, d7
pub fn i_05bea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEAA: move.l d7, (a4)
pub fn i_05beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEAC: add.l d4, d7
pub fn i_05beac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEAE: move.l d7, (a4)
pub fn i_05beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB0: add.l d4, d7
pub fn i_05beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEB2: move.l d7, (a4)
pub fn i_05beb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB4: add.l d4, d7
pub fn i_05beb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEB6: move.l d7, (a4)
pub fn i_05beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB8: add.l d4, d7
pub fn i_05beb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEBA: move.l d7, (a4)
pub fn i_05beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEBC: add.l d4, d7
pub fn i_05bebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEBE: move.l d7, (a4)
pub fn i_05bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC0: add.l d4, d7
pub fn i_05bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEC2: move.l d7, (a4)
pub fn i_05bec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC4: add.l d4, d7
pub fn i_05bec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEC6: move.l d7, (a4)
pub fn i_05bec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC8: add.l d4, d7
pub fn i_05bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BECA: move.l d7, (a4)
pub fn i_05beca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BECC: add.l d4, d7
pub fn i_05becc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BECE: move.l d7, (a4)
pub fn i_05bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED0: add.l d4, d7
pub fn i_05bed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BED2: move.l d7, (a4)
pub fn i_05bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED4: add.l d4, d7
pub fn i_05bed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BED6: move.l d7, (a4)
pub fn i_05bed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED8: add.l d4, d7
pub fn i_05bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEDA: move.l d7, (a4)
pub fn i_05beda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEDC: add.l d4, d7
pub fn i_05bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEDE: move.l d7, (a4)
pub fn i_05bede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE0: add.l d4, d7
pub fn i_05bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEE2: move.l d7, (a4)
pub fn i_05bee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE4: add.l d4, d7
pub fn i_05bee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEE6: move.l d7, (a4)
pub fn i_05bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE8: add.l d4, d7
pub fn i_05bee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEEA: move.l d7, (a4)
pub fn i_05beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEEC: add.l d4, d7
pub fn i_05beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEEE: move.l d7, (a4)
pub fn i_05beee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF0: add.l d4, d7
pub fn i_05bef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEF2: move.l d7, (a4)
pub fn i_05bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF4: add.l d4, d7
pub fn i_05bef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEF6: move.l d7, (a4)
pub fn i_05bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF8: add.l d4, d7
pub fn i_05bef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEFA: move.l d7, (a4)
pub fn i_05befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEFC: add.l d4, d7
pub fn i_05befc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEFE: move.l d7, (a4)
pub fn i_05befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF00;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF00: add.l d4, d7
pub fn i_05bf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF02;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF02: move.l d7, (a4)
pub fn i_05bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF04;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF04: add.l d4, d7
pub fn i_05bf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF06;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF06: move.l d7, (a4)
pub fn i_05bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF08;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF08: add.l d4, d7
pub fn i_05bf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF0A: move.l d7, (a4)
pub fn i_05bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF0C: add.l d4, d7
pub fn i_05bf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF0E: move.l d7, (a4)
pub fn i_05bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF10: add.l d4, d7
pub fn i_05bf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF12: move.l d7, (a4)
pub fn i_05bf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF14;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF14: add.l d4, d7
pub fn i_05bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF16;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF16: move.l d7, (a4)
pub fn i_05bf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF18;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF18: add.l d4, d7
pub fn i_05bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF1A: move.l d7, (a4)
pub fn i_05bf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF1C: add.l d4, d7
pub fn i_05bf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF1E: move.l d7, (a4)
pub fn i_05bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF20;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF20: add.l d4, d7
pub fn i_05bf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF22;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF22: move.l d7, (a4)
pub fn i_05bf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF24: add.l d4, d7
pub fn i_05bf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF26: move.l d7, (a4)
pub fn i_05bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF28;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF28: add.l d4, d7
pub fn i_05bf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF2A: move.l d7, (a4)
pub fn i_05bf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF2C: add.l d4, d7
pub fn i_05bf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF2E: move.l d7, (a4)
pub fn i_05bf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF30;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF30: add.l d4, d7
pub fn i_05bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF32;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF32: move.l d7, (a4)
pub fn i_05bf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF34;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF34: add.l d4, d7
pub fn i_05bf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF36;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF36: move.l d7, (a4)
pub fn i_05bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF38: add.l d4, d7
pub fn i_05bf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF3A: move.l d7, (a4)
pub fn i_05bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF3C: add.l d4, d7
pub fn i_05bf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF3E: move.l d7, (a4)
pub fn i_05bf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF40;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF40: add.l d4, d7
pub fn i_05bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF42;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF42: move.l d7, (a4)
pub fn i_05bf42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF44;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF44: add.l d4, d7
pub fn i_05bf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF46;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF46: move.l d7, (a4)
pub fn i_05bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF48;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF48: add.l d4, d7
pub fn i_05bf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF4A: move.l d7, (a4)
pub fn i_05bf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF4C: add.l d4, d7
pub fn i_05bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF4E: move.l d7, (a4)
pub fn i_05bf4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF50;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF50: add.l d4, d7
pub fn i_05bf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF52;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF52: move.l d7, (a4)
pub fn i_05bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF54;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF54: add.l d4, d7
pub fn i_05bf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF56;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF56: move.w #$8401, d6
pub fn i_05bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5A;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BF5A: add.w d0, d6
pub fn i_05bf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5C;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BF5C: swap d6
pub fn i_05bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5E;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BF5E: move.w $a(a3), d6
pub fn i_05bf5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF62;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF62: move.l #$2000000, d5
pub fn i_05bf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF68;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BF68: move.l d6, (a4)
pub fn i_05bf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF6A: sub.l d5, d6
pub fn i_05bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6C;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BF6C: sub.l d5, d6
pub fn i_05bf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6E;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BF6E: move.w $6(a3), d6
pub fn i_05bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF72;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF72: move.l d6, (a4)
pub fn i_05bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF74;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF74: rts
pub fn i_05bf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SCB1_WriteTileColumnTerm_05bf76  ($05BF76, 2656 B, video/sprite_queue_render_05aa96.s)
/// $05BF76: move.w $6(a3), d7
pub fn i_05bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7A;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF7A: addq.b #$1, d7
pub fn i_05bf7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7C;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BF7C: sne.b d6
pub fn i_05bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7E;
    cpu.ir = 0x56C6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05BF7E: andi.b #$4, d6
pub fn i_05bf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF82;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05BF82: rol.w #$7, d7
pub fn i_05bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF84;
    cpu.ir = 0xEF5F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 3, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $05BF84: andi.w #$8, d7
pub fn i_05bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF88;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BF88: or.b d6, d7
pub fn i_05bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF8A;
    cpu.ir = 0x8E06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05BF8A: jmp $5bf8e(pc, d7.w)
pub fn i_05bf8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF8E;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BF8E_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BF8E: bra.w $5bf9e
pub fn i_05bf8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF92;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BF9E;
    10
}
/// $05BF96: bra.w $5c1aa
pub fn i_05bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C1AA;
    10
}
/// $05BF9E: move.w #$40, d7
pub fn i_05bf9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA2;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BFA2: add.w d1, d7
pub fn i_05bfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA4;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BFA4: move.w d7, d5
pub fn i_05bfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA6;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BFA6: addq.w #$1, d5
pub fn i_05bfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA8;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BFA8: swap d5
pub fn i_05bfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFAA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFAA: move.w $4(a3), d5
pub fn i_05bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFAE;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BFAE: swap d7
pub fn i_05bfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB0;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFB0: moveq #$2, d4
pub fn i_05bfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB2;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFB2: swap d4
pub fn i_05bfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB4;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFB4: movea.l (a3), a6
pub fn i_05bfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB6;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05BFB6: move.w $a(a3), d6
pub fn i_05bfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFBA;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BFBA: subq.b #$1, d6
pub fn i_05bfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFBC;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BFBC: andi.w #$1f, d6
pub fn i_05bfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC0;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BFC0: add.w d6, d6
pub fn i_05bfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC2;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BFC2: add.w d6, d6
pub fn i_05bfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC4;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BFC4: jmp $5bfc8(pc, d6.w)
pub fn i_05bfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC8;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BFC8_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BFC8: bra.w $5c17a
pub fn i_05bfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C17A;
    10
}
/// $05BFCC: bra.w $5c170
pub fn i_05bfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C170;
    10
}
/// $05BFD0: bra.w $5c166
pub fn i_05bfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFD4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C166;
    10
}
/// $05BFD4: bra.w $5c15c
pub fn i_05bfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C15C;
    10
}
/// $05BFD8: bra.w $5c152
pub fn i_05bfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C152;
    10
}
/// $05BFDC: bra.w $5c148
pub fn i_05bfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C148;
    10
}
/// $05BFE0: bra.w $5c13e
pub fn i_05bfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C13E;
    10
}
/// $05BFE4: bra.w $5c134
pub fn i_05bfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C134;
    10
}
/// $05BFE8: bra.w $5c12a
pub fn i_05bfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C12A;
    10
}
/// $05BFEC: bra.w $5c120
pub fn i_05bfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C120;
    10
}
/// $05C120: move.w (a6)+, d7
pub fn i_05c120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C122;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C122: move.l d7, (a4)
pub fn i_05c122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C124;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C124: add.l d4, d7
pub fn i_05c124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C126;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C126: move.l d5, (a4)
pub fn i_05c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C128;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C128: add.l d4, d5
pub fn i_05c128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C12A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C12A: move.w (a6)+, d7
pub fn i_05c12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C12C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C12C: move.l d7, (a4)
pub fn i_05c12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C12E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C12E: add.l d4, d7
pub fn i_05c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C130;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C130: move.l d5, (a4)
pub fn i_05c130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C132;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C132: add.l d4, d5
pub fn i_05c132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C134;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C134: move.w (a6)+, d7
pub fn i_05c134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C136;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C136: move.l d7, (a4)
pub fn i_05c136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C138;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C138: add.l d4, d7
pub fn i_05c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C13A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C13A: move.l d5, (a4)
pub fn i_05c13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C13C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C13C: add.l d4, d5
pub fn i_05c13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C13E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C13E: move.w (a6)+, d7
pub fn i_05c13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C140;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C140: move.l d7, (a4)
pub fn i_05c140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C142;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C142: add.l d4, d7
pub fn i_05c142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C144;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C144: move.l d5, (a4)
pub fn i_05c144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C146;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C146: add.l d4, d5
pub fn i_05c146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C148;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C148: move.w (a6)+, d7
pub fn i_05c148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C14A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C14A: move.l d7, (a4)
pub fn i_05c14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C14C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C14C: add.l d4, d7
pub fn i_05c14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C14E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C14E: move.l d5, (a4)
pub fn i_05c14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C150;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C150: add.l d4, d5
pub fn i_05c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C152;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C152: move.w (a6)+, d7
pub fn i_05c152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C154;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C154: move.l d7, (a4)
pub fn i_05c154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C156;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C156: add.l d4, d7
pub fn i_05c156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C158;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C158: move.l d5, (a4)
pub fn i_05c158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C15A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C15A: add.l d4, d5
pub fn i_05c15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C15C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C15C: move.w (a6)+, d7
pub fn i_05c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C15E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C15E: move.l d7, (a4)
pub fn i_05c15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C160;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C160: add.l d4, d7
pub fn i_05c160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C162;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C162: move.l d5, (a4)
pub fn i_05c162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C164;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C164: add.l d4, d5
pub fn i_05c164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C166;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C166: move.w (a6)+, d7
pub fn i_05c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C168;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C168: move.l d7, (a4)
pub fn i_05c168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C16A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C16A: add.l d4, d7
pub fn i_05c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C16C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C16C: move.l d5, (a4)
pub fn i_05c16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C16E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C16E: add.l d4, d5
pub fn i_05c16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C170;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C170: move.w (a6)+, d7
pub fn i_05c170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C172;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C172: move.l d7, (a4)
pub fn i_05c172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C174;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C174: add.l d4, d7
pub fn i_05c174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C176;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C176: move.l d5, (a4)
pub fn i_05c176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C178;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C178: add.l d4, d5
pub fn i_05c178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C17A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C17A: move.w (a6)+, d7
pub fn i_05c17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C17C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C17C: move.l d7, (a4)
pub fn i_05c17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C17E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C17E: add.l d4, d7
pub fn i_05c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C180;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C180: move.l d5, (a4)
pub fn i_05c180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C182;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C182: add.l d4, d5
pub fn i_05c182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C184;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C184: move.w #$8401, d6
pub fn i_05c184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C188;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C188: add.w d0, d6
pub fn i_05c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C18A;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C18A: swap d6
pub fn i_05c18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C18C;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C18C: move.w $a(a3), d6
pub fn i_05c18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C190;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C190: move.l #$2000000, d5
pub fn i_05c190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C196;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C196: move.l d6, (a4)
pub fn i_05c196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C198;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C198: sub.l d5, d6
pub fn i_05c198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C19A;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C19A: move.w $8(a3), d6
pub fn i_05c19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C19E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C19E: move.l d6, (a4)
pub fn i_05c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A0;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C1A0: sub.l d5, d6
pub fn i_05c1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A2;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C1A2: move.w $6(a3), d6
pub fn i_05c1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A6;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C1A6: move.l d6, (a4)
pub fn i_05c1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A8;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C1A8: rts
pub fn i_05c1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05C1AA: move.w #$40, d7
pub fn i_05c1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1AE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C1AE: add.w d1, d7
pub fn i_05c1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B0;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C1B0: move.w d7, d5
pub fn i_05c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B2;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C1B2: addq.w #$1, d5
pub fn i_05c1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B4;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05C1B4: swap d5
pub fn i_05c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B6;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1B6: move.w $4(a3), d5
pub fn i_05c1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1BA;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C1BA: swap d7
pub fn i_05c1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1BC;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1BC: moveq #$2, d4
pub fn i_05c1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1BE;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1BE: swap d4
pub fn i_05c1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C0;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1C0: movea.l (a3), a6
pub fn i_05c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C2;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05C1C2: move.w $a(a3), d6
pub fn i_05c1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C6;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C1C6: subq.b #$1, d6
pub fn i_05c1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C8;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05C1C8: andi.w #$1f, d6
pub fn i_05c1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1CC;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05C1CC: add.w d6, d6
pub fn i_05c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1CE;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C1CE: add.w d6, d6
pub fn i_05c1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1D0;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C1D0: jmp $5c1d4(pc, d6.w)
pub fn i_05c1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1D4;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5C1D4_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05C1D4: bra.w $5c386
pub fn i_05c1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C386;
    10
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05ADF2, i_05adf2),
    (0x05ADF4, i_05adf4),
    (0x05ADF8, i_05adf8),
    (0x05ADFE, i_05adfe),
    (0x05AE00, i_05ae00),
    (0x05AE02, i_05ae02),
    (0x05AE06, i_05ae06),
    (0x05AE08, i_05ae08),
    (0x05AE0A, i_05ae0a),
    (0x05AE0C, i_05ae0c),
    (0x05AE10, i_05ae10),
    (0x05AE12, i_05ae12),
    (0x05AE14, i_05ae14),
    (0x05AE16, i_05ae16),
    (0x05AE18, i_05ae18),
    (0x05AE1C, i_05ae1c),
    (0x05AE1E, i_05ae1e),
    (0x05AE20, i_05ae20),
    (0x05AE24, i_05ae24),
    (0x05AE28, i_05ae28),
    (0x05AE2A, i_05ae2a),
    (0x05AE2E, i_05ae2e),
    (0x05AE30, i_05ae30),
    (0x05AE34, i_05ae34),
    (0x05AE36, i_05ae36),
    (0x05AE38, i_05ae38),
    (0x05AE3C, i_05ae3c),
    (0x05AE3E, i_05ae3e),
    (0x05AE40, i_05ae40),
    (0x05AE42, i_05ae42),
    (0x05AE44, i_05ae44),
    (0x05AE46, i_05ae46),
    (0x05AE48, i_05ae48),
    (0x05AE4A, i_05ae4a),
    (0x05AE4C, i_05ae4c),
    (0x05AE4E, i_05ae4e),
    (0x05AE50, i_05ae50),
    (0x05AE52, i_05ae52),
    (0x05AE54, i_05ae54),
    (0x05AE56, i_05ae56),
    (0x05AE58, i_05ae58),
    (0x05AE5A, i_05ae5a),
    (0x05AE5C, i_05ae5c),
    (0x05AE5E, i_05ae5e),
    (0x05AE60, i_05ae60),
    (0x05AE62, i_05ae62),
    (0x05AE64, i_05ae64),
    (0x05AE66, i_05ae66),
    (0x05AE6A, i_05ae6a),
    (0x05AE6C, i_05ae6c),
    (0x05AE6E, i_05ae6e),
    (0x05AE70, i_05ae70),
    (0x05AE72, i_05ae72),
    (0x05AE76, i_05ae76),
    (0x05AE7A, i_05ae7a),
    (0x05AE7C, i_05ae7c),
    (0x05AE7E, i_05ae7e),
    (0x05AE80, i_05ae80),
    (0x05AE82, i_05ae82),
    (0x05AE86, i_05ae86),
    (0x05AE8A, i_05ae8a),
    (0x05AE8C, i_05ae8c),
    (0x05AE90, i_05ae90),
    (0x05AE92, i_05ae92),
    (0x05AE94, i_05ae94),
    (0x05AE96, i_05ae96),
    (0x05AE9A, i_05ae9a),
    (0x05AE9C, i_05ae9c),
    (0x05AE9E, i_05ae9e),
    (0x05AEA0, i_05aea0),
    (0x05AEA2, i_05aea2),
    (0x05AEA4, i_05aea4),
    (0x05AEA6, i_05aea6),
    (0x05AEA8, i_05aea8),
    (0x05AEAC, i_05aeac),
    (0x05AEAE, i_05aeae),
    (0x05AEB0, i_05aeb0),
    (0x05AEB2, i_05aeb2),
    (0x05AEB4, i_05aeb4),
    (0x05AEB6, i_05aeb6),
    (0x05AEB8, i_05aeb8),
    (0x05AEBA, i_05aeba),
    (0x05AEBC, i_05aebc),
    (0x05AEC0, i_05aec0),
    (0x05AEC2, i_05aec2),
    (0x05AEC4, i_05aec4),
    (0x05AEC6, i_05aec6),
    (0x05AEC8, i_05aec8),
    (0x05AECA, i_05aeca),
    (0x05AECC, i_05aecc),
    (0x05AECE, i_05aece),
    (0x05AED2, i_05aed2),
    (0x05AED6, i_05aed6),
    (0x05AEDA, i_05aeda),
    (0x05AEDC, i_05aedc),
    (0x05AEDE, i_05aede),
    (0x05AEE2, i_05aee2),
    (0x05AEE4, i_05aee4),
    (0x05AEE8, i_05aee8),
    (0x05AEEC, i_05aeec),
    (0x05AEEE, i_05aeee),
    (0x05AEF0, i_05aef0),
    (0x05AEF4, i_05aef4),
    (0x05AEF8, i_05aef8),
    (0x05AEFC, i_05aefc),
    (0x05AF00, i_05af00),
    (0x05AF02, i_05af02),
    (0x05AF04, i_05af04),
    (0x05AF06, i_05af06),
    (0x05AF08, i_05af08),
    (0x05AF0A, i_05af0a),
    (0x05AF0E, i_05af0e),
    (0x05AF10, i_05af10),
    (0x05AF12, i_05af12),
    (0x05AF16, i_05af16),
    (0x05AF18, i_05af18),
    (0x05AF1C, i_05af1c),
    (0x05AF1E, i_05af1e),
    (0x05AF22, i_05af22),
    (0x05AF24, i_05af24),
    (0x05AF28, i_05af28),
    (0x05AF2E, i_05af2e),
    (0x05AF30, i_05af30),
    (0x05AF32, i_05af32),
    (0x05AF36, i_05af36),
    (0x05AF38, i_05af38),
    (0x05AF3A, i_05af3a),
    (0x05AF3C, i_05af3c),
    (0x05AF40, i_05af40),
    (0x05AF42, i_05af42),
    (0x05AF44, i_05af44),
    (0x05AF46, i_05af46),
    (0x05AF48, i_05af48),
    (0x05AF4C, i_05af4c),
    (0x05AF4E, i_05af4e),
    (0x05AF50, i_05af50),
    (0x05AF54, i_05af54),
    (0x05AF58, i_05af58),
    (0x05AF5A, i_05af5a),
    (0x05AF5E, i_05af5e),
    (0x05AF60, i_05af60),
    (0x05AF64, i_05af64),
    (0x05AF66, i_05af66),
    (0x05AF68, i_05af68),
    (0x05AF6E, i_05af6e),
    (0x05AF72, i_05af72),
    (0x05AF74, i_05af74),
    (0x05AF76, i_05af76),
    (0x05AF78, i_05af78),
    (0x05AF7A, i_05af7a),
    (0x05AF7C, i_05af7c),
    (0x05AF7E, i_05af7e),
    (0x05AF80, i_05af80),
    (0x05AF82, i_05af82),
    (0x05AF84, i_05af84),
    (0x05AF86, i_05af86),
    (0x05AF88, i_05af88),
    (0x05AF8A, i_05af8a),
    (0x05AF8C, i_05af8c),
    (0x05AF8E, i_05af8e),
    (0x05AF90, i_05af90),
    (0x05AF92, i_05af92),
    (0x05AF94, i_05af94),
    (0x05AF96, i_05af96),
    (0x05AF98, i_05af98),
    (0x05AF9A, i_05af9a),
    (0x05AF9C, i_05af9c),
    (0x05AF9E, i_05af9e),
    (0x05AFA0, i_05afa0),
    (0x05AFA4, i_05afa4),
    (0x05AFA6, i_05afa6),
    (0x05AFA8, i_05afa8),
    (0x05AFAA, i_05afaa),
    (0x05AFAC, i_05afac),
    (0x05AFB0, i_05afb0),
    (0x05AFB4, i_05afb4),
    (0x05AFB6, i_05afb6),
    (0x05AFB8, i_05afb8),
    (0x05AFBA, i_05afba),
    (0x05AFBC, i_05afbc),
    (0x05AFC0, i_05afc0),
    (0x05AFC4, i_05afc4),
    (0x05AFC6, i_05afc6),
    (0x05AFCA, i_05afca),
    (0x05AFCC, i_05afcc),
    (0x05AFCE, i_05afce),
    (0x05AFD0, i_05afd0),
    (0x05AFD4, i_05afd4),
    (0x05AFD6, i_05afd6),
    (0x05AFD8, i_05afd8),
    (0x05AFDA, i_05afda),
    (0x05AFDC, i_05afdc),
    (0x05AFDE, i_05afde),
    (0x05AFE0, i_05afe0),
    (0x05AFE2, i_05afe2),
    (0x05AFE6, i_05afe6),
    (0x05AFE8, i_05afe8),
    (0x05AFEA, i_05afea),
    (0x05AFEC, i_05afec),
    (0x05AFEE, i_05afee),
    (0x05AFF0, i_05aff0),
    (0x05AFF2, i_05aff2),
    (0x05AFF4, i_05aff4),
    (0x05AFF6, i_05aff6),
    (0x05AFFA, i_05affa),
    (0x05AFFC, i_05affc),
    (0x05AFFE, i_05affe),
    (0x05B000, i_05b000),
    (0x05B002, i_05b002),
    (0x05B004, i_05b004),
    (0x05B006, i_05b006),
    (0x05B008, i_05b008),
    (0x05B00A, i_05b00a),
    (0x05B00E, i_05b00e),
    (0x05B012, i_05b012),
    (0x05B016, i_05b016),
    (0x05B018, i_05b018),
    (0x05B01A, i_05b01a),
    (0x05B01E, i_05b01e),
    (0x05B020, i_05b020),
    (0x05B024, i_05b024),
    (0x05B028, i_05b028),
    (0x05B02A, i_05b02a),
    (0x05B02E, i_05b02e),
    (0x05B032, i_05b032),
    (0x05B036, i_05b036),
    (0x05B03A, i_05b03a),
    (0x05B03C, i_05b03c),
    (0x05B03E, i_05b03e),
    (0x05B040, i_05b040),
    (0x05B042, i_05b042),
    (0x05B044, i_05b044),
    (0x05B048, i_05b048),
    (0x05B04A, i_05b04a),
    (0x05B04C, i_05b04c),
    (0x05B04E, i_05b04e),
    (0x05B052, i_05b052),
    (0x05B054, i_05b054),
    (0x05B056, i_05b056),
    (0x05B05A, i_05b05a),
    (0x05B05C, i_05b05c),
    (0x05B060, i_05b060),
    (0x05B062, i_05b062),
    (0x05B066, i_05b066),
    (0x05B06C, i_05b06c),
    (0x05B06E, i_05b06e),
    (0x05B070, i_05b070),
    (0x05B074, i_05b074),
    (0x05B076, i_05b076),
    (0x05B078, i_05b078),
    (0x05B07A, i_05b07a),
    (0x05B07E, i_05b07e),
    (0x05B080, i_05b080),
    (0x05B082, i_05b082),
    (0x05B084, i_05b084),
    (0x05B086, i_05b086),
    (0x05B08A, i_05b08a),
    (0x05B08C, i_05b08c),
    (0x05B08E, i_05b08e),
    (0x05B092, i_05b092),
    (0x05B096, i_05b096),
    (0x05B098, i_05b098),
    (0x05B09C, i_05b09c),
    (0x05B09E, i_05b09e),
    (0x05B0A2, i_05b0a2),
    (0x05B0A4, i_05b0a4),
    (0x05B0A6, i_05b0a6),
    (0x05B0AC, i_05b0ac),
    (0x05B0B0, i_05b0b0),
    (0x05B0B2, i_05b0b2),
    (0x05B0B4, i_05b0b4),
    (0x05B0B6, i_05b0b6),
    (0x05B0B8, i_05b0b8),
    (0x05B0BA, i_05b0ba),
    (0x05B0BC, i_05b0bc),
    (0x05B0BE, i_05b0be),
    (0x05B0C0, i_05b0c0),
    (0x05B0C2, i_05b0c2),
    (0x05B0C4, i_05b0c4),
    (0x05B0C6, i_05b0c6),
    (0x05B0C8, i_05b0c8),
    (0x05B0CA, i_05b0ca),
    (0x05B0CC, i_05b0cc),
    (0x05B0CE, i_05b0ce),
    (0x05B0D0, i_05b0d0),
    (0x05B0D2, i_05b0d2),
    (0x05B0D4, i_05b0d4),
    (0x05B0D6, i_05b0d6),
    (0x05B0D8, i_05b0d8),
    (0x05B0DA, i_05b0da),
    (0x05B0DC, i_05b0dc),
    (0x05B0DE, i_05b0de),
    (0x05B0E0, i_05b0e0),
    (0x05B0E2, i_05b0e2),
    (0x05B0E4, i_05b0e4),
    (0x05B0E8, i_05b0e8),
    (0x05B0EA, i_05b0ea),
    (0x05B0EC, i_05b0ec),
    (0x05B0EE, i_05b0ee),
    (0x05B0F0, i_05b0f0),
    (0x05B0F4, i_05b0f4),
    (0x05B0F8, i_05b0f8),
    (0x05B0FA, i_05b0fa),
    (0x05B0FC, i_05b0fc),
    (0x05B0FE, i_05b0fe),
    (0x05B100, i_05b100),
    (0x05B104, i_05b104),
    (0x05B108, i_05b108),
    (0x05B10A, i_05b10a),
    (0x05B10E, i_05b10e),
    (0x05B110, i_05b110),
    (0x05B112, i_05b112),
    (0x05B114, i_05b114),
    (0x05B118, i_05b118),
    (0x05B11A, i_05b11a),
    (0x05B11C, i_05b11c),
    (0x05B11E, i_05b11e),
    (0x05B120, i_05b120),
    (0x05B122, i_05b122),
    (0x05B124, i_05b124),
    (0x05B126, i_05b126),
    (0x05B12A, i_05b12a),
    (0x05B12C, i_05b12c),
    (0x05B12E, i_05b12e),
    (0x05B130, i_05b130),
    (0x05B132, i_05b132),
    (0x05B134, i_05b134),
    (0x05B136, i_05b136),
    (0x05B138, i_05b138),
    (0x05B13A, i_05b13a),
    (0x05B13E, i_05b13e),
    (0x05B140, i_05b140),
    (0x05B142, i_05b142),
    (0x05B144, i_05b144),
    (0x05B146, i_05b146),
    (0x05B148, i_05b148),
    (0x05B14A, i_05b14a),
    (0x05B14C, i_05b14c),
    (0x05B14E, i_05b14e),
    (0x05B152, i_05b152),
    (0x05B156, i_05b156),
    (0x05B15A, i_05b15a),
    (0x05B15C, i_05b15c),
    (0x05B15E, i_05b15e),
    (0x05B162, i_05b162),
    (0x05B164, i_05b164),
    (0x05B168, i_05b168),
    (0x05B16C, i_05b16c),
    (0x05B16E, i_05b16e),
    (0x05B170, i_05b170),
    (0x05B174, i_05b174),
    (0x05B178, i_05b178),
    (0x05B17C, i_05b17c),
    (0x05B180, i_05b180),
    (0x05B182, i_05b182),
    (0x05B184, i_05b184),
    (0x05B186, i_05b186),
    (0x05B188, i_05b188),
    (0x05B18A, i_05b18a),
    (0x05B18E, i_05b18e),
    (0x05B190, i_05b190),
    (0x05B192, i_05b192),
    (0x05B196, i_05b196),
    (0x05B198, i_05b198),
    (0x05B19A, i_05b19a),
    (0x05B19E, i_05b19e),
    (0x05B1A0, i_05b1a0),
    (0x05B1A4, i_05b1a4),
    (0x05B1A6, i_05b1a6),
    (0x05B1AA, i_05b1aa),
    (0x05B1B0, i_05b1b0),
    (0x05B1B2, i_05b1b2),
    (0x05B1B6, i_05b1b6),
    (0x05B1BE, i_05b1be),
    (0x05B1C2, i_05b1c2),
    (0x05B1C8, i_05b1c8),
    (0x05B1CA, i_05b1ca),
    (0x05B1CE, i_05b1ce),
    (0x05B1D2, i_05b1d2),
    (0x05B1D6, i_05b1d6),
    (0x05B1DA, i_05b1da),
    (0x05B1DE, i_05b1de),
    (0x05B1E4, i_05b1e4),
    (0x05B1E6, i_05b1e6),
    (0x05B1EA, i_05b1ea),
    (0x05B1EE, i_05b1ee),
    (0x05B1F2, i_05b1f2),
    (0x05B1F6, i_05b1f6),
    (0x05B1FA, i_05b1fa),
    (0x05B1FE, i_05b1fe),
    (0x05B202, i_05b202),
    (0x05B206, i_05b206),
    (0x05B20A, i_05b20a),
    (0x05B20C, i_05b20c),
    (0x05B210, i_05b210),
    (0x05B212, i_05b212),
    (0x05B216, i_05b216),
    (0x05B218, i_05b218),
    (0x05B21A, i_05b21a),
    (0x05B21E, i_05b21e),
    (0x05B224, i_05b224),
    (0x05B226, i_05b226),
    (0x05B228, i_05b228),
    (0x05B22A, i_05b22a),
    (0x05B22E, i_05b22e),
    (0x05B230, i_05b230),
    (0x05B232, i_05b232),
    (0x05B238, i_05b238),
    (0x05B23C, i_05b23c),
    (0x05B240, i_05b240),
    (0x05B244, i_05b244),
    (0x05B246, i_05b246),
    (0x05B248, i_05b248),
    (0x05B24C, i_05b24c),
    (0x05B250, i_05b250),
    (0x05B254, i_05b254),
    (0x05B256, i_05b256),
    (0x05B258, i_05b258),
    (0x05B25A, i_05b25a),
    (0x05B25E, i_05b25e),
    (0x05B260, i_05b260),
    (0x05B264, i_05b264),
    (0x05B268, i_05b268),
    (0x05B26A, i_05b26a),
    (0x05B26E, i_05b26e),
    (0x05B270, i_05b270),
    (0x05B272, i_05b272),
    (0x05B274, i_05b274),
    (0x05B278, i_05b278),
    (0x05B27C, i_05b27c),
    (0x05B27E, i_05b27e),
    (0x05B280, i_05b280),
    (0x05B282, i_05b282),
    (0x05B284, i_05b284),
    (0x05B288, i_05b288),
    (0x05B28A, i_05b28a),
    (0x05B28C, i_05b28c),
    (0x05B290, i_05b290),
    (0x05B294, i_05b294),
    (0x05B298, i_05b298),
    (0x05B29C, i_05b29c),
    (0x05B29E, i_05b29e),
    (0x05B2A0, i_05b2a0),
    (0x05B2A4, i_05b2a4),
    (0x05B2A6, i_05b2a6),
    (0x05B2A8, i_05b2a8),
    (0x05B2AA, i_05b2aa),
    (0x05B2AE, i_05b2ae),
    (0x05B2B0, i_05b2b0),
    (0x05B2B4, i_05b2b4),
    (0x05B2B8, i_05b2b8),
    (0x05B2BA, i_05b2ba),
    (0x05B2BE, i_05b2be),
    (0x05B2C0, i_05b2c0),
    (0x05B2C2, i_05b2c2),
    (0x05B2C4, i_05b2c4),
    (0x05B2C8, i_05b2c8),
    (0x05B2CC, i_05b2cc),
    (0x05B2CE, i_05b2ce),
    (0x05B2D0, i_05b2d0),
    (0x05B2D2, i_05b2d2),
    (0x05B2D4, i_05b2d4),
    (0x05B2D8, i_05b2d8),
    (0x05B2DA, i_05b2da),
    (0x05B2DE, i_05b2de),
    (0x05B2E2, i_05b2e2),
    (0x05B2E6, i_05b2e6),
    (0x05B2EA, i_05b2ea),
    (0x05B2EE, i_05b2ee),
    (0x05B2F2, i_05b2f2),
    (0x05B2F4, i_05b2f4),
    (0x05B2F8, i_05b2f8),
    (0x05B2FA, i_05b2fa),
    (0x05B2FC, i_05b2fc),
    (0x05B2FE, i_05b2fe),
    (0x05B300, i_05b300),
    (0x05B304, i_05b304),
    (0x05B308, i_05b308),
    (0x05B30A, i_05b30a),
    (0x05B30C, i_05b30c),
    (0x05B30E, i_05b30e),
    (0x05B310, i_05b310),
    (0x05B312, i_05b312),
    (0x05B316, i_05b316),
    (0x05B318, i_05b318),
    (0x05B31A, i_05b31a),
    (0x05B31C, i_05b31c),
    (0x05B322, i_05b322),
    (0x05B326, i_05b326),
    (0x05B32C, i_05b32c),
    (0x05B334, i_05b334),
    (0x05B338, i_05b338),
    (0x05B33C, i_05b33c),
    (0x05B340, i_05b340),
    (0x05B344, i_05b344),
    (0x05B348, i_05b348),
    (0x05B34E, i_05b34e),
    (0x05B354, i_05b354),
    (0x05B358, i_05b358),
    (0x05B35C, i_05b35c),
    (0x05B360, i_05b360),
    (0x05B362, i_05b362),
    (0x05B366, i_05b366),
    (0x05B368, i_05b368),
    (0x05B36E, i_05b36e),
    (0x05B370, i_05b370),
    (0x05B374, i_05b374),
    (0x05B378, i_05b378),
    (0x05B37C, i_05b37c),
    (0x05B37E, i_05b37e),
    (0x05B380, i_05b380),
    (0x05B382, i_05b382),
    (0x05B386, i_05b386),
    (0x05B388, i_05b388),
    (0x05B38C, i_05b38c),
    (0x05B38E, i_05b38e),
    (0x05B392, i_05b392),
    (0x05B394, i_05b394),
    (0x05B396, i_05b396),
    (0x05B39A, i_05b39a),
    (0x05B39C, i_05b39c),
    (0x05B3A0, i_05b3a0),
    (0x05B3A4, i_05b3a4),
    (0x05B3A6, i_05b3a6),
    (0x05B3AA, i_05b3aa),
    (0x05B3AE, i_05b3ae),
    (0x05B3B2, i_05b3b2),
    (0x05B3B4, i_05b3b4),
    (0x05B3B8, i_05b3b8),
    (0x05B3BA, i_05b3ba),
    (0x05B3BE, i_05b3be),
    (0x05B3C2, i_05b3c2),
    (0x05B3C4, i_05b3c4),
    (0x05B3C6, i_05b3c6),
    (0x05B3C8, i_05b3c8),
    (0x05B3CA, i_05b3ca),
    (0x05B3CE, i_05b3ce),
    (0x05B3D0, i_05b3d0),
    (0x05B3D2, i_05b3d2),
    (0x05B3D6, i_05b3d6),
    (0x05B3D8, i_05b3d8),
    (0x05B3DA, i_05b3da),
    (0x05B3DE, i_05b3de),
    (0x05B3E0, i_05b3e0),
    (0x05B3E4, i_05b3e4),
    (0x05B3E6, i_05b3e6),
    (0x05B3EA, i_05b3ea),
    (0x05B3EE, i_05b3ee),
    (0x05B3F2, i_05b3f2),
    (0x05B3F6, i_05b3f6),
    (0x05B3F8, i_05b3f8),
    (0x05B3FA, i_05b3fa),
    (0x05B3FE, i_05b3fe),
    (0x05B400, i_05b400),
    (0x05B404, i_05b404),
    (0x05B408, i_05b408),
    (0x05B40A, i_05b40a),
    (0x05B40C, i_05b40c),
    (0x05B40E, i_05b40e),
    (0x05B412, i_05b412),
    (0x05B416, i_05b416),
    (0x05B418, i_05b418),
    (0x05B41C, i_05b41c),
    (0x05B41E, i_05b41e),
    (0x05B422, i_05b422),
    (0x05B424, i_05b424),
    (0x05B426, i_05b426),
    (0x05B42A, i_05b42a),
    (0x05B42E, i_05b42e),
    (0x05B432, i_05b432),
    (0x05B436, i_05b436),
    (0x05B438, i_05b438),
    (0x05B43A, i_05b43a),
    (0x05B43E, i_05b43e),
    (0x05B440, i_05b440),
    (0x05B444, i_05b444),
    (0x05B448, i_05b448),
    (0x05B44A, i_05b44a),
    (0x05B44C, i_05b44c),
    (0x05B450, i_05b450),
    (0x05B454, i_05b454),
    (0x05B456, i_05b456),
    (0x05B45A, i_05b45a),
    (0x05B45C, i_05b45c),
    (0x05B45E, i_05b45e),
    (0x05B460, i_05b460),
    (0x05B464, i_05b464),
    (0x05B468, i_05b468),
    (0x05B46C, i_05b46c),
    (0x05B46E, i_05b46e),
    (0x05B470, i_05b470),
    (0x05B474, i_05b474),
    (0x05B476, i_05b476),
    (0x05B478, i_05b478),
    (0x05B47E, i_05b47e),
    (0x05B480, i_05b480),
    (0x05B482, i_05b482),
    (0x05B484, i_05b484),
    (0x05B486, i_05b486),
    (0x05B48A, i_05b48a),
    (0x05B48C, i_05b48c),
    (0x05B48E, i_05b48e),
    (0x05B492, i_05b492),
    (0x05B496, i_05b496),
    (0x05B49A, i_05b49a),
    (0x05B49C, i_05b49c),
    (0x05B49E, i_05b49e),
    (0x05B4A0, i_05b4a0),
    (0x05B4A4, i_05b4a4),
    (0x05B4A6, i_05b4a6),
    (0x05B4AA, i_05b4aa),
    (0x05B4AC, i_05b4ac),
    (0x05B4AE, i_05b4ae),
    (0x05B4B2, i_05b4b2),
    (0x05B4B4, i_05b4b4),
    (0x05B4B6, i_05b4b6),
    (0x05B4BA, i_05b4ba),
    (0x05B4BC, i_05b4bc),
    (0x05B4BE, i_05b4be),
    (0x05B4C2, i_05b4c2),
    (0x05B4C6, i_05b4c6),
    (0x05B4CA, i_05b4ca),
    (0x05B4CC, i_05b4cc),
    (0x05B4CE, i_05b4ce),
    (0x05B4D2, i_05b4d2),
    (0x05B4D4, i_05b4d4),
    (0x05B4D8, i_05b4d8),
    (0x05B4DC, i_05b4dc),
    (0x05B4E0, i_05b4e0),
    (0x05B4E4, i_05b4e4),
    (0x05B4E6, i_05b4e6),
    (0x05B4E8, i_05b4e8),
    (0x05B4EA, i_05b4ea),
    (0x05B4EC, i_05b4ec),
    (0x05B4F0, i_05b4f0),
    (0x05B4F4, i_05b4f4),
    (0x05B4F8, i_05b4f8),
    (0x05B4FA, i_05b4fa),
    (0x05B4FC, i_05b4fc),
    (0x05B500, i_05b500),
    (0x05B502, i_05b502),
    (0x05B504, i_05b504),
    (0x05B50A, i_05b50a),
    (0x05B50C, i_05b50c),
    (0x05B50E, i_05b50e),
    (0x05B510, i_05b510),
    (0x05B512, i_05b512),
    (0x05B516, i_05b516),
    (0x05B518, i_05b518),
    (0x05B51A, i_05b51a),
    (0x05B51E, i_05b51e),
    (0x05B522, i_05b522),
    (0x05B524, i_05b524),
    (0x05B528, i_05b528),
    (0x05B52C, i_05b52c),
    (0x05B52E, i_05b52e),
    (0x05B532, i_05b532),
    (0x05B534, i_05b534),
    (0x05B536, i_05b536),
    (0x05B53A, i_05b53a),
    (0x05B53C, i_05b53c),
    (0x05B540, i_05b540),
    (0x05B542, i_05b542),
    (0x05B546, i_05b546),
    (0x05B54A, i_05b54a),
    (0x05B54E, i_05b54e),
    (0x05B552, i_05b552),
    (0x05B556, i_05b556),
    (0x05B55A, i_05b55a),
    (0x05B55C, i_05b55c),
    (0x05B55E, i_05b55e),
    (0x05B560, i_05b560),
    (0x05B562, i_05b562),
    (0x05B566, i_05b566),
    (0x05B568, i_05b568),
    (0x05B56A, i_05b56a),
    (0x05B56C, i_05b56c),
    (0x05B56E, i_05b56e),
    (0x05B572, i_05b572),
    (0x05B574, i_05b574),
    (0x05B578, i_05b578),
    (0x05B57A, i_05b57a),
    (0x05B57C, i_05b57c),
    (0x05B580, i_05b580),
    (0x05B584, i_05b584),
    (0x05B588, i_05b588),
    (0x05B58C, i_05b58c),
    (0x05B590, i_05b590),
    (0x05B594, i_05b594),
    (0x05B598, i_05b598),
    (0x05B59C, i_05b59c),
    (0x05B5A0, i_05b5a0),
    (0x05B5A4, i_05b5a4),
    (0x05B5A8, i_05b5a8),
    (0x05B5AC, i_05b5ac),
    (0x05B5B0, i_05b5b0),
    (0x05B5B4, i_05b5b4),
    (0x05B5B8, i_05b5b8),
    (0x05B5BC, i_05b5bc),
    (0x05B5DC, i_05b5dc),
    (0x05B64C, i_05b64c),
    (0x05B64E, i_05b64e),
    (0x05B650, i_05b650),
    (0x05B652, i_05b652),
    (0x05B654, i_05b654),
    (0x05B656, i_05b656),
    (0x05B658, i_05b658),
    (0x05B65A, i_05b65a),
    (0x05B65C, i_05b65c),
    (0x05B65E, i_05b65e),
    (0x05B660, i_05b660),
    (0x05B662, i_05b662),
    (0x05B664, i_05b664),
    (0x05B666, i_05b666),
    (0x05B668, i_05b668),
    (0x05B66A, i_05b66a),
    (0x05B66C, i_05b66c),
    (0x05B66E, i_05b66e),
    (0x05B670, i_05b670),
    (0x05B672, i_05b672),
    (0x05B674, i_05b674),
    (0x05B676, i_05b676),
    (0x05B678, i_05b678),
    (0x05B67A, i_05b67a),
    (0x05B67C, i_05b67c),
    (0x05B67E, i_05b67e),
    (0x05B680, i_05b680),
    (0x05B682, i_05b682),
    (0x05B684, i_05b684),
    (0x05B686, i_05b686),
    (0x05B688, i_05b688),
    (0x05B68A, i_05b68a),
    (0x05B68C, i_05b68c),
    (0x05B68E, i_05b68e),
    (0x05B690, i_05b690),
    (0x05B692, i_05b692),
    (0x05B694, i_05b694),
    (0x05B696, i_05b696),
    (0x05B698, i_05b698),
    (0x05B69A, i_05b69a),
    (0x05B69C, i_05b69c),
    (0x05B69E, i_05b69e),
    (0x05B6A0, i_05b6a0),
    (0x05B6A2, i_05b6a2),
    (0x05B6A4, i_05b6a4),
    (0x05B6A6, i_05b6a6),
    (0x05B6A8, i_05b6a8),
    (0x05B6AA, i_05b6aa),
    (0x05B6AC, i_05b6ac),
    (0x05B6AE, i_05b6ae),
    (0x05B6B0, i_05b6b0),
    (0x05B6B2, i_05b6b2),
    (0x05B6B4, i_05b6b4),
    (0x05B6B6, i_05b6b6),
    (0x05B6B8, i_05b6b8),
    (0x05B6BA, i_05b6ba),
    (0x05B6BC, i_05b6bc),
    (0x05B6BE, i_05b6be),
    (0x05B6C0, i_05b6c0),
    (0x05B6C2, i_05b6c2),
    (0x05B6C4, i_05b6c4),
    (0x05B6C6, i_05b6c6),
    (0x05B6C8, i_05b6c8),
    (0x05B6CA, i_05b6ca),
    (0x05B6CC, i_05b6cc),
    (0x05B6CE, i_05b6ce),
    (0x05B6D0, i_05b6d0),
    (0x05B6D2, i_05b6d2),
    (0x05B6D4, i_05b6d4),
    (0x05B6D6, i_05b6d6),
    (0x05B6D8, i_05b6d8),
    (0x05B6DA, i_05b6da),
    (0x05B6DC, i_05b6dc),
    (0x05B6DE, i_05b6de),
    (0x05B6E0, i_05b6e0),
    (0x05B6E2, i_05b6e2),
    (0x05B6E4, i_05b6e4),
    (0x05B6E6, i_05b6e6),
    (0x05B6E8, i_05b6e8),
    (0x05B6EA, i_05b6ea),
    (0x05B6EC, i_05b6ec),
    (0x05B6EE, i_05b6ee),
    (0x05B6F0, i_05b6f0),
    (0x05B6F2, i_05b6f2),
    (0x05B6F4, i_05b6f4),
    (0x05B6F6, i_05b6f6),
    (0x05B6F8, i_05b6f8),
    (0x05B6FA, i_05b6fa),
    (0x05B6FC, i_05b6fc),
    (0x05B6FE, i_05b6fe),
    (0x05B700, i_05b700),
    (0x05B702, i_05b702),
    (0x05B704, i_05b704),
    (0x05B706, i_05b706),
    (0x05B708, i_05b708),
    (0x05B70A, i_05b70a),
    (0x05B70C, i_05b70c),
    (0x05B70E, i_05b70e),
    (0x05B710, i_05b710),
    (0x05B712, i_05b712),
    (0x05B714, i_05b714),
    (0x05B716, i_05b716),
    (0x05B718, i_05b718),
    (0x05B71A, i_05b71a),
    (0x05B71C, i_05b71c),
    (0x05B71E, i_05b71e),
    (0x05B720, i_05b720),
    (0x05B722, i_05b722),
    (0x05B724, i_05b724),
    (0x05B726, i_05b726),
    (0x05B728, i_05b728),
    (0x05B72A, i_05b72a),
    (0x05B72C, i_05b72c),
    (0x05B72E, i_05b72e),
    (0x05B730, i_05b730),
    (0x05B732, i_05b732),
    (0x05B734, i_05b734),
    (0x05B736, i_05b736),
    (0x05B738, i_05b738),
    (0x05B73A, i_05b73a),
    (0x05B73C, i_05b73c),
    (0x05B740, i_05b740),
    (0x05B742, i_05b742),
    (0x05B744, i_05b744),
    (0x05B748, i_05b748),
    (0x05B74E, i_05b74e),
    (0x05B750, i_05b750),
    (0x05B752, i_05b752),
    (0x05B754, i_05b754),
    (0x05B758, i_05b758),
    (0x05B75A, i_05b75a),
    (0x05B75C, i_05b75c),
    (0x05B760, i_05b760),
    (0x05B762, i_05b762),
    (0x05B764, i_05b764),
    (0x05B766, i_05b766),
    (0x05B768, i_05b768),
    (0x05B76C, i_05b76c),
    (0x05B76E, i_05b76e),
    (0x05B770, i_05b770),
    (0x05B772, i_05b772),
    (0x05B774, i_05b774),
    (0x05B778, i_05b778),
    (0x05B77A, i_05b77a),
    (0x05B77E, i_05b77e),
    (0x05B780, i_05b780),
    (0x05B782, i_05b782),
    (0x05B786, i_05b786),
    (0x05B78A, i_05b78a),
    (0x05B78E, i_05b78e),
    (0x05B792, i_05b792),
    (0x05B796, i_05b796),
    (0x05B79A, i_05b79a),
    (0x05B79E, i_05b79e),
    (0x05B7A2, i_05b7a2),
    (0x05B7A6, i_05b7a6),
    (0x05B7AA, i_05b7aa),
    (0x05B8DE, i_05b8de),
    (0x05B8E0, i_05b8e0),
    (0x05B8E2, i_05b8e2),
    (0x05B8E4, i_05b8e4),
    (0x05B8E6, i_05b8e6),
    (0x05B8E8, i_05b8e8),
    (0x05B8EA, i_05b8ea),
    (0x05B8EC, i_05b8ec),
    (0x05B8EE, i_05b8ee),
    (0x05B8F0, i_05b8f0),
    (0x05B8F2, i_05b8f2),
    (0x05B8F4, i_05b8f4),
    (0x05B8F6, i_05b8f6),
    (0x05B8F8, i_05b8f8),
    (0x05B8FA, i_05b8fa),
    (0x05B8FC, i_05b8fc),
    (0x05B8FE, i_05b8fe),
    (0x05B900, i_05b900),
    (0x05B902, i_05b902),
    (0x05B904, i_05b904),
    (0x05B906, i_05b906),
    (0x05B908, i_05b908),
    (0x05B90A, i_05b90a),
    (0x05B90C, i_05b90c),
    (0x05B90E, i_05b90e),
    (0x05B910, i_05b910),
    (0x05B912, i_05b912),
    (0x05B914, i_05b914),
    (0x05B916, i_05b916),
    (0x05B918, i_05b918),
    (0x05B91A, i_05b91a),
    (0x05B91C, i_05b91c),
    (0x05B91E, i_05b91e),
    (0x05B920, i_05b920),
    (0x05B922, i_05b922),
    (0x05B924, i_05b924),
    (0x05B926, i_05b926),
    (0x05B928, i_05b928),
    (0x05B92A, i_05b92a),
    (0x05B92C, i_05b92c),
    (0x05B92E, i_05b92e),
    (0x05B930, i_05b930),
    (0x05B932, i_05b932),
    (0x05B934, i_05b934),
    (0x05B936, i_05b936),
    (0x05B938, i_05b938),
    (0x05B93A, i_05b93a),
    (0x05B93C, i_05b93c),
    (0x05B93E, i_05b93e),
    (0x05B940, i_05b940),
    (0x05B942, i_05b942),
    (0x05B946, i_05b946),
    (0x05B948, i_05b948),
    (0x05B94A, i_05b94a),
    (0x05B94E, i_05b94e),
    (0x05B954, i_05b954),
    (0x05B956, i_05b956),
    (0x05B958, i_05b958),
    (0x05B95A, i_05b95a),
    (0x05B95E, i_05b95e),
    (0x05B960, i_05b960),
    (0x05B962, i_05b962),
    (0x05B966, i_05b966),
    (0x05B968, i_05b968),
    (0x05B96A, i_05b96a),
    (0x05B96C, i_05b96c),
    (0x05B96E, i_05b96e),
    (0x05B972, i_05b972),
    (0x05B974, i_05b974),
    (0x05B976, i_05b976),
    (0x05B978, i_05b978),
    (0x05B97A, i_05b97a),
    (0x05B97E, i_05b97e),
    (0x05B980, i_05b980),
    (0x05B984, i_05b984),
    (0x05B986, i_05b986),
    (0x05B988, i_05b988),
    (0x05B98C, i_05b98c),
    (0x05B990, i_05b990),
    (0x05B994, i_05b994),
    (0x05B998, i_05b998),
    (0x05B99C, i_05b99c),
    (0x05B9A0, i_05b9a0),
    (0x05B9A4, i_05b9a4),
    (0x05B9A8, i_05b9a8),
    (0x05B9AC, i_05b9ac),
    (0x05B9B0, i_05b9b0),
    (0x05B9B4, i_05b9b4),
    (0x05BADA, i_05bada),
    (0x05BADC, i_05badc),
    (0x05BADE, i_05bade),
    (0x05BAE0, i_05bae0),
    (0x05BAE2, i_05bae2),
    (0x05BAE4, i_05bae4),
    (0x05BAE6, i_05bae6),
    (0x05BAE8, i_05bae8),
    (0x05BAEA, i_05baea),
    (0x05BAEC, i_05baec),
    (0x05BAEE, i_05baee),
    (0x05BAF0, i_05baf0),
    (0x05BAF2, i_05baf2),
    (0x05BAF4, i_05baf4),
    (0x05BAF6, i_05baf6),
    (0x05BAF8, i_05baf8),
    (0x05BAFA, i_05bafa),
    (0x05BAFC, i_05bafc),
    (0x05BAFE, i_05bafe),
    (0x05BB00, i_05bb00),
    (0x05BB02, i_05bb02),
    (0x05BB04, i_05bb04),
    (0x05BB06, i_05bb06),
    (0x05BB08, i_05bb08),
    (0x05BB0A, i_05bb0a),
    (0x05BB0C, i_05bb0c),
    (0x05BB0E, i_05bb0e),
    (0x05BB10, i_05bb10),
    (0x05BB12, i_05bb12),
    (0x05BB14, i_05bb14),
    (0x05BB16, i_05bb16),
    (0x05BB18, i_05bb18),
    (0x05BB1A, i_05bb1a),
    (0x05BB1C, i_05bb1c),
    (0x05BB1E, i_05bb1e),
    (0x05BB20, i_05bb20),
    (0x05BB22, i_05bb22),
    (0x05BB24, i_05bb24),
    (0x05BB26, i_05bb26),
    (0x05BB28, i_05bb28),
    (0x05BB2A, i_05bb2a),
    (0x05BB2C, i_05bb2c),
    (0x05BB2E, i_05bb2e),
    (0x05BB30, i_05bb30),
    (0x05BB32, i_05bb32),
    (0x05BB34, i_05bb34),
    (0x05BB36, i_05bb36),
    (0x05BB38, i_05bb38),
    (0x05BB3A, i_05bb3a),
    (0x05BB3C, i_05bb3c),
    (0x05BB3E, i_05bb3e),
    (0x05BB40, i_05bb40),
    (0x05BB42, i_05bb42),
    (0x05BB44, i_05bb44),
    (0x05BB46, i_05bb46),
    (0x05BB48, i_05bb48),
    (0x05BB4A, i_05bb4a),
    (0x05BB4C, i_05bb4c),
    (0x05BB4E, i_05bb4e),
    (0x05BB50, i_05bb50),
    (0x05BB54, i_05bb54),
    (0x05BB56, i_05bb56),
    (0x05BB58, i_05bb58),
    (0x05BB5A, i_05bb5a),
    (0x05BB5C, i_05bb5c),
    (0x05BB5E, i_05bb5e),
    (0x05BB60, i_05bb60),
    (0x05BB62, i_05bb62),
    (0x05BB64, i_05bb64),
    (0x05BB66, i_05bb66),
    (0x05BB68, i_05bb68),
    (0x05BB6A, i_05bb6a),
    (0x05BB6C, i_05bb6c),
    (0x05BB6E, i_05bb6e),
    (0x05BB70, i_05bb70),
    (0x05BB72, i_05bb72),
    (0x05BB74, i_05bb74),
    (0x05BB76, i_05bb76),
    (0x05BB78, i_05bb78),
    (0x05BB7A, i_05bb7a),
    (0x05BB7C, i_05bb7c),
    (0x05BB7E, i_05bb7e),
    (0x05BB80, i_05bb80),
    (0x05BB82, i_05bb82),
    (0x05BB84, i_05bb84),
    (0x05BB86, i_05bb86),
    (0x05BB88, i_05bb88),
    (0x05BB8A, i_05bb8a),
    (0x05BB8C, i_05bb8c),
    (0x05BB8E, i_05bb8e),
    (0x05BB90, i_05bb90),
    (0x05BB92, i_05bb92),
    (0x05BB94, i_05bb94),
    (0x05BB96, i_05bb96),
    (0x05BB98, i_05bb98),
    (0x05BB9A, i_05bb9a),
    (0x05BB9C, i_05bb9c),
    (0x05BB9E, i_05bb9e),
    (0x05BBA0, i_05bba0),
    (0x05BBA2, i_05bba2),
    (0x05BBA4, i_05bba4),
    (0x05BBA6, i_05bba6),
    (0x05BBA8, i_05bba8),
    (0x05BBAA, i_05bbaa),
    (0x05BBAC, i_05bbac),
    (0x05BBAE, i_05bbae),
    (0x05BBB0, i_05bbb0),
    (0x05BBB2, i_05bbb2),
    (0x05BBB4, i_05bbb4),
    (0x05BBB6, i_05bbb6),
    (0x05BBB8, i_05bbb8),
    (0x05BBBA, i_05bbba),
    (0x05BBBC, i_05bbbc),
    (0x05BBBE, i_05bbbe),
    (0x05BBC0, i_05bbc0),
    (0x05BBC2, i_05bbc2),
    (0x05BBC4, i_05bbc4),
    (0x05BBC6, i_05bbc6),
    (0x05BBC8, i_05bbc8),
    (0x05BBCA, i_05bbca),
    (0x05BBCC, i_05bbcc),
    (0x05BBCE, i_05bbce),
    (0x05BBD0, i_05bbd0),
    (0x05BBD2, i_05bbd2),
    (0x05BBD4, i_05bbd4),
    (0x05BBD6, i_05bbd6),
    (0x05BBD8, i_05bbd8),
    (0x05BBDA, i_05bbda),
    (0x05BBDC, i_05bbdc),
    (0x05BBDE, i_05bbde),
    (0x05BBE0, i_05bbe0),
    (0x05BBE2, i_05bbe2),
    (0x05BBE4, i_05bbe4),
    (0x05BBE6, i_05bbe6),
    (0x05BBE8, i_05bbe8),
    (0x05BBEA, i_05bbea),
    (0x05BBEC, i_05bbec),
    (0x05BBEE, i_05bbee),
    (0x05BBF0, i_05bbf0),
    (0x05BBF2, i_05bbf2),
    (0x05BBF4, i_05bbf4),
    (0x05BBF6, i_05bbf6),
    (0x05BBF8, i_05bbf8),
    (0x05BBFA, i_05bbfa),
    (0x05BBFC, i_05bbfc),
    (0x05BBFE, i_05bbfe),
    (0x05BC00, i_05bc00),
    (0x05BC02, i_05bc02),
    (0x05BC04, i_05bc04),
    (0x05BC06, i_05bc06),
    (0x05BC08, i_05bc08),
    (0x05BC0A, i_05bc0a),
    (0x05BC0C, i_05bc0c),
    (0x05BC0E, i_05bc0e),
    (0x05BC10, i_05bc10),
    (0x05BC12, i_05bc12),
    (0x05BC14, i_05bc14),
    (0x05BC16, i_05bc16),
    (0x05BC18, i_05bc18),
    (0x05BC1A, i_05bc1a),
    (0x05BC1C, i_05bc1c),
    (0x05BC1E, i_05bc1e),
    (0x05BC20, i_05bc20),
    (0x05BC22, i_05bc22),
    (0x05BC24, i_05bc24),
    (0x05BC26, i_05bc26),
    (0x05BC28, i_05bc28),
    (0x05BC2A, i_05bc2a),
    (0x05BC2C, i_05bc2c),
    (0x05BC2E, i_05bc2e),
    (0x05BC30, i_05bc30),
    (0x05BC32, i_05bc32),
    (0x05BC34, i_05bc34),
    (0x05BC36, i_05bc36),
    (0x05BC38, i_05bc38),
    (0x05BC3A, i_05bc3a),
    (0x05BC3C, i_05bc3c),
    (0x05BC3E, i_05bc3e),
    (0x05BC40, i_05bc40),
    (0x05BC42, i_05bc42),
    (0x05BC44, i_05bc44),
    (0x05BC46, i_05bc46),
    (0x05BC48, i_05bc48),
    (0x05BC4A, i_05bc4a),
    (0x05BC4C, i_05bc4c),
    (0x05BC50, i_05bc50),
    (0x05BC52, i_05bc52),
    (0x05BC54, i_05bc54),
    (0x05BC58, i_05bc58),
    (0x05BC5E, i_05bc5e),
    (0x05BC60, i_05bc60),
    (0x05BC62, i_05bc62),
    (0x05BC64, i_05bc64),
    (0x05BC68, i_05bc68),
    (0x05BC6A, i_05bc6a),
    (0x05BC6C, i_05bc6c),
    (0x05BC70, i_05bc70),
    (0x05BC72, i_05bc72),
    (0x05BC74, i_05bc74),
    (0x05BC76, i_05bc76),
    (0x05BC78, i_05bc78),
    (0x05BC7C, i_05bc7c),
    (0x05BC7E, i_05bc7e),
    (0x05BC80, i_05bc80),
    (0x05BC82, i_05bc82),
    (0x05BC84, i_05bc84),
    (0x05BC88, i_05bc88),
    (0x05BC8A, i_05bc8a),
    (0x05BC8E, i_05bc8e),
    (0x05BC90, i_05bc90),
    (0x05BC92, i_05bc92),
    (0x05BC96, i_05bc96),
    (0x05BC9A, i_05bc9a),
    (0x05BC9E, i_05bc9e),
    (0x05BCA2, i_05bca2),
    (0x05BCA6, i_05bca6),
    (0x05BCAA, i_05bcaa),
    (0x05BCAE, i_05bcae),
    (0x05BCB2, i_05bcb2),
    (0x05BCB6, i_05bcb6),
    (0x05BCBA, i_05bcba),
    (0x05BCBE, i_05bcbe),
    (0x05BCCA, i_05bcca),
    (0x05BCCE, i_05bcce),
    (0x05BDBC, i_05bdbc),
    (0x05BDBE, i_05bdbe),
    (0x05BDC0, i_05bdc0),
    (0x05BDC2, i_05bdc2),
    (0x05BDC4, i_05bdc4),
    (0x05BDC6, i_05bdc6),
    (0x05BDC8, i_05bdc8),
    (0x05BDCA, i_05bdca),
    (0x05BDCC, i_05bdcc),
    (0x05BDCE, i_05bdce),
    (0x05BDD0, i_05bdd0),
    (0x05BDD2, i_05bdd2),
    (0x05BDD4, i_05bdd4),
    (0x05BDD6, i_05bdd6),
    (0x05BDD8, i_05bdd8),
    (0x05BDDA, i_05bdda),
    (0x05BDDC, i_05bddc),
    (0x05BDDE, i_05bdde),
    (0x05BDE0, i_05bde0),
    (0x05BDE2, i_05bde2),
    (0x05BDE4, i_05bde4),
    (0x05BDE6, i_05bde6),
    (0x05BDE8, i_05bde8),
    (0x05BDEA, i_05bdea),
    (0x05BDEC, i_05bdec),
    (0x05BDEE, i_05bdee),
    (0x05BDF0, i_05bdf0),
    (0x05BDF2, i_05bdf2),
    (0x05BDF4, i_05bdf4),
    (0x05BDF6, i_05bdf6),
    (0x05BDF8, i_05bdf8),
    (0x05BDFA, i_05bdfa),
    (0x05BDFC, i_05bdfc),
    (0x05BDFE, i_05bdfe),
    (0x05BE00, i_05be00),
    (0x05BE02, i_05be02),
    (0x05BE04, i_05be04),
    (0x05BE06, i_05be06),
    (0x05BE08, i_05be08),
    (0x05BE0A, i_05be0a),
    (0x05BE0C, i_05be0c),
    (0x05BE0E, i_05be0e),
    (0x05BE10, i_05be10),
    (0x05BE12, i_05be12),
    (0x05BE14, i_05be14),
    (0x05BE16, i_05be16),
    (0x05BE18, i_05be18),
    (0x05BE1A, i_05be1a),
    (0x05BE1C, i_05be1c),
    (0x05BE1E, i_05be1e),
    (0x05BE20, i_05be20),
    (0x05BE22, i_05be22),
    (0x05BE24, i_05be24),
    (0x05BE26, i_05be26),
    (0x05BE28, i_05be28),
    (0x05BE2A, i_05be2a),
    (0x05BE2C, i_05be2c),
    (0x05BE2E, i_05be2e),
    (0x05BE30, i_05be30),
    (0x05BE32, i_05be32),
    (0x05BE34, i_05be34),
    (0x05BE36, i_05be36),
    (0x05BE38, i_05be38),
    (0x05BE3A, i_05be3a),
    (0x05BE3C, i_05be3c),
    (0x05BE3E, i_05be3e),
    (0x05BE40, i_05be40),
    (0x05BE42, i_05be42),
    (0x05BE44, i_05be44),
    (0x05BE46, i_05be46),
    (0x05BE48, i_05be48),
    (0x05BE4A, i_05be4a),
    (0x05BE4C, i_05be4c),
    (0x05BE4E, i_05be4e),
    (0x05BE50, i_05be50),
    (0x05BE52, i_05be52),
    (0x05BE54, i_05be54),
    (0x05BE56, i_05be56),
    (0x05BE58, i_05be58),
    (0x05BE5A, i_05be5a),
    (0x05BE5E, i_05be5e),
    (0x05BE60, i_05be60),
    (0x05BE62, i_05be62),
    (0x05BE64, i_05be64),
    (0x05BE66, i_05be66),
    (0x05BE68, i_05be68),
    (0x05BE6A, i_05be6a),
    (0x05BE6C, i_05be6c),
    (0x05BE6E, i_05be6e),
    (0x05BE70, i_05be70),
    (0x05BE72, i_05be72),
    (0x05BE74, i_05be74),
    (0x05BE76, i_05be76),
    (0x05BE78, i_05be78),
    (0x05BE7A, i_05be7a),
    (0x05BE7C, i_05be7c),
    (0x05BE7E, i_05be7e),
    (0x05BE80, i_05be80),
    (0x05BE82, i_05be82),
    (0x05BE84, i_05be84),
    (0x05BE86, i_05be86),
    (0x05BE88, i_05be88),
    (0x05BE8A, i_05be8a),
    (0x05BE8C, i_05be8c),
    (0x05BE8E, i_05be8e),
    (0x05BE90, i_05be90),
    (0x05BE92, i_05be92),
    (0x05BE94, i_05be94),
    (0x05BE96, i_05be96),
    (0x05BE98, i_05be98),
    (0x05BE9A, i_05be9a),
    (0x05BE9C, i_05be9c),
    (0x05BE9E, i_05be9e),
    (0x05BEA0, i_05bea0),
    (0x05BEA2, i_05bea2),
    (0x05BEA4, i_05bea4),
    (0x05BEA6, i_05bea6),
    (0x05BEA8, i_05bea8),
    (0x05BEAA, i_05beaa),
    (0x05BEAC, i_05beac),
    (0x05BEAE, i_05beae),
    (0x05BEB0, i_05beb0),
    (0x05BEB2, i_05beb2),
    (0x05BEB4, i_05beb4),
    (0x05BEB6, i_05beb6),
    (0x05BEB8, i_05beb8),
    (0x05BEBA, i_05beba),
    (0x05BEBC, i_05bebc),
    (0x05BEBE, i_05bebe),
    (0x05BEC0, i_05bec0),
    (0x05BEC2, i_05bec2),
    (0x05BEC4, i_05bec4),
    (0x05BEC6, i_05bec6),
    (0x05BEC8, i_05bec8),
    (0x05BECA, i_05beca),
    (0x05BECC, i_05becc),
    (0x05BECE, i_05bece),
    (0x05BED0, i_05bed0),
    (0x05BED2, i_05bed2),
    (0x05BED4, i_05bed4),
    (0x05BED6, i_05bed6),
    (0x05BED8, i_05bed8),
    (0x05BEDA, i_05beda),
    (0x05BEDC, i_05bedc),
    (0x05BEDE, i_05bede),
    (0x05BEE0, i_05bee0),
    (0x05BEE2, i_05bee2),
    (0x05BEE4, i_05bee4),
    (0x05BEE6, i_05bee6),
    (0x05BEE8, i_05bee8),
    (0x05BEEA, i_05beea),
    (0x05BEEC, i_05beec),
    (0x05BEEE, i_05beee),
    (0x05BEF0, i_05bef0),
    (0x05BEF2, i_05bef2),
    (0x05BEF4, i_05bef4),
    (0x05BEF6, i_05bef6),
    (0x05BEF8, i_05bef8),
    (0x05BEFA, i_05befa),
    (0x05BEFC, i_05befc),
    (0x05BEFE, i_05befe),
    (0x05BF00, i_05bf00),
    (0x05BF02, i_05bf02),
    (0x05BF04, i_05bf04),
    (0x05BF06, i_05bf06),
    (0x05BF08, i_05bf08),
    (0x05BF0A, i_05bf0a),
    (0x05BF0C, i_05bf0c),
    (0x05BF0E, i_05bf0e),
    (0x05BF10, i_05bf10),
    (0x05BF12, i_05bf12),
    (0x05BF14, i_05bf14),
    (0x05BF16, i_05bf16),
    (0x05BF18, i_05bf18),
    (0x05BF1A, i_05bf1a),
    (0x05BF1C, i_05bf1c),
    (0x05BF1E, i_05bf1e),
    (0x05BF20, i_05bf20),
    (0x05BF22, i_05bf22),
    (0x05BF24, i_05bf24),
    (0x05BF26, i_05bf26),
    (0x05BF28, i_05bf28),
    (0x05BF2A, i_05bf2a),
    (0x05BF2C, i_05bf2c),
    (0x05BF2E, i_05bf2e),
    (0x05BF30, i_05bf30),
    (0x05BF32, i_05bf32),
    (0x05BF34, i_05bf34),
    (0x05BF36, i_05bf36),
    (0x05BF38, i_05bf38),
    (0x05BF3A, i_05bf3a),
    (0x05BF3C, i_05bf3c),
    (0x05BF3E, i_05bf3e),
    (0x05BF40, i_05bf40),
    (0x05BF42, i_05bf42),
    (0x05BF44, i_05bf44),
    (0x05BF46, i_05bf46),
    (0x05BF48, i_05bf48),
    (0x05BF4A, i_05bf4a),
    (0x05BF4C, i_05bf4c),
    (0x05BF4E, i_05bf4e),
    (0x05BF50, i_05bf50),
    (0x05BF52, i_05bf52),
    (0x05BF54, i_05bf54),
    (0x05BF56, i_05bf56),
    (0x05BF5A, i_05bf5a),
    (0x05BF5C, i_05bf5c),
    (0x05BF5E, i_05bf5e),
    (0x05BF62, i_05bf62),
    (0x05BF68, i_05bf68),
    (0x05BF6A, i_05bf6a),
    (0x05BF6C, i_05bf6c),
    (0x05BF6E, i_05bf6e),
    (0x05BF72, i_05bf72),
    (0x05BF74, i_05bf74),
    (0x05BF76, i_05bf76),
    (0x05BF7A, i_05bf7a),
    (0x05BF7C, i_05bf7c),
    (0x05BF7E, i_05bf7e),
    (0x05BF82, i_05bf82),
    (0x05BF84, i_05bf84),
    (0x05BF88, i_05bf88),
    (0x05BF8A, i_05bf8a),
    (0x05BF8E, i_05bf8e),
    (0x05BF96, i_05bf96),
    (0x05BF9E, i_05bf9e),
    (0x05BFA2, i_05bfa2),
    (0x05BFA4, i_05bfa4),
    (0x05BFA6, i_05bfa6),
    (0x05BFA8, i_05bfa8),
    (0x05BFAA, i_05bfaa),
    (0x05BFAE, i_05bfae),
    (0x05BFB0, i_05bfb0),
    (0x05BFB2, i_05bfb2),
    (0x05BFB4, i_05bfb4),
    (0x05BFB6, i_05bfb6),
    (0x05BFBA, i_05bfba),
    (0x05BFBC, i_05bfbc),
    (0x05BFC0, i_05bfc0),
    (0x05BFC2, i_05bfc2),
    (0x05BFC4, i_05bfc4),
    (0x05BFC8, i_05bfc8),
    (0x05BFCC, i_05bfcc),
    (0x05BFD0, i_05bfd0),
    (0x05BFD4, i_05bfd4),
    (0x05BFD8, i_05bfd8),
    (0x05BFDC, i_05bfdc),
    (0x05BFE0, i_05bfe0),
    (0x05BFE4, i_05bfe4),
    (0x05BFE8, i_05bfe8),
    (0x05BFEC, i_05bfec),
    (0x05C120, i_05c120),
    (0x05C122, i_05c122),
    (0x05C124, i_05c124),
    (0x05C126, i_05c126),
    (0x05C128, i_05c128),
    (0x05C12A, i_05c12a),
    (0x05C12C, i_05c12c),
    (0x05C12E, i_05c12e),
    (0x05C130, i_05c130),
    (0x05C132, i_05c132),
    (0x05C134, i_05c134),
    (0x05C136, i_05c136),
    (0x05C138, i_05c138),
    (0x05C13A, i_05c13a),
    (0x05C13C, i_05c13c),
    (0x05C13E, i_05c13e),
    (0x05C140, i_05c140),
    (0x05C142, i_05c142),
    (0x05C144, i_05c144),
    (0x05C146, i_05c146),
    (0x05C148, i_05c148),
    (0x05C14A, i_05c14a),
    (0x05C14C, i_05c14c),
    (0x05C14E, i_05c14e),
    (0x05C150, i_05c150),
    (0x05C152, i_05c152),
    (0x05C154, i_05c154),
    (0x05C156, i_05c156),
    (0x05C158, i_05c158),
    (0x05C15A, i_05c15a),
    (0x05C15C, i_05c15c),
    (0x05C15E, i_05c15e),
    (0x05C160, i_05c160),
    (0x05C162, i_05c162),
    (0x05C164, i_05c164),
    (0x05C166, i_05c166),
    (0x05C168, i_05c168),
    (0x05C16A, i_05c16a),
    (0x05C16C, i_05c16c),
    (0x05C16E, i_05c16e),
    (0x05C170, i_05c170),
    (0x05C172, i_05c172),
    (0x05C174, i_05c174),
    (0x05C176, i_05c176),
    (0x05C178, i_05c178),
    (0x05C17A, i_05c17a),
    (0x05C17C, i_05c17c),
    (0x05C17E, i_05c17e),
    (0x05C180, i_05c180),
    (0x05C182, i_05c182),
    (0x05C184, i_05c184),
    (0x05C188, i_05c188),
    (0x05C18A, i_05c18a),
    (0x05C18C, i_05c18c),
    (0x05C190, i_05c190),
    (0x05C196, i_05c196),
    (0x05C198, i_05c198),
    (0x05C19A, i_05c19a),
    (0x05C19E, i_05c19e),
    (0x05C1A0, i_05c1a0),
    (0x05C1A2, i_05c1a2),
    (0x05C1A6, i_05c1a6),
    (0x05C1A8, i_05c1a8),
    (0x05C1AA, i_05c1aa),
    (0x05C1AE, i_05c1ae),
    (0x05C1B0, i_05c1b0),
    (0x05C1B2, i_05c1b2),
    (0x05C1B4, i_05c1b4),
    (0x05C1B6, i_05c1b6),
    (0x05C1BA, i_05c1ba),
    (0x05C1BC, i_05c1bc),
    (0x05C1BE, i_05c1be),
    (0x05C1C0, i_05c1c0),
    (0x05C1C2, i_05c1c2),
    (0x05C1C6, i_05c1c6),
    (0x05C1C8, i_05c1c8),
    (0x05C1CC, i_05c1cc),
    (0x05C1CE, i_05c1ce),
    (0x05C1D0, i_05c1d0),
    (0x05C1D4, i_05c1d4),
];
