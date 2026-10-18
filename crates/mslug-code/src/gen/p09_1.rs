// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $097A6E: bra.w $97a80
pub fn i_097a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A72;
    cpu.ir = 0x6000;
    cpu.pc = 0x97A80;
    10
}
/// $097A80: jsr $97aaa(pc)
pub fn i_097a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97A84);
    cpu.pc = 0x97AAA;
    16
}
/// $097A84: bcc.w $97aa4
pub fn i_097a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x97AA4; return 10; }
    8
}
/// $097A88: movem.l d0-d1/a1, -(a7)
pub fn i_097a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A8C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $097A8C: lea.l $97d24(pc), a1
pub fn i_097a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97D24;
    16
}
/// $097A90: jsr $4ae.l
pub fn i_097a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A96);
    cpu.pc = 0x4AE;
    16
}
/// $097A96: movem.l (a7)+, d0-d1/a1
pub fn i_097a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A9A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $097A9A: move.l a1, $80(a0)
pub fn i_097a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A9E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $097A9E: move.w d1, $88(a0)
pub fn i_097a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AA2;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097AA2: rts
pub fn i_097aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $097AA4: clr.b $21(a6)
pub fn i_097aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AA8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $097AA8: rts
pub fn i_097aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_InsertScore_097aaa  ($097AAA, 186 B, frontend/hiscore_memcard_options_09773c.s)
/// $097AAA: movem.l d0-d1/a1, -(a7)
pub fn i_097aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AAE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $097AAE: lea.l $100002.l, a2
pub fn i_097aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AB4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x100002;
    20
}
/// $097AB4: moveq #$9, d2
pub fn i_097ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AB6;
    cpu.ir = 0x7409;
    cpu.d[2] = 0x9;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097AB6: cmp.b (a2), d1
pub fn i_097ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AB8;
    cpu.ir = 0xB212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    8
}
/// $097AB8: bne.w $97abe
pub fn i_097ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ABC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97ABE; return 10; }
    8
}
/// $097ABC: clr.b (a6)
pub fn i_097abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ABE;
    cpu.ir = 0x4216;
    let ad1 = cpu.a[6];
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    8
}
/// $097ABE: adda.l #$c, a2
pub fn i_097abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AC4;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0xC;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $097AC4: dbra d2, $97ab6
pub fn i_097ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AC8;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x97AC8; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x97AC8; return 14; }
    cpu.pc = 0x97AB6;
    10
}
/// $097AC8: moveq #$0, d3
pub fn i_097ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ACA;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097ACA: lea.l $100002.l, a2
pub fn i_097aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AD0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x100002;
    20
}
/// $097AD0: move.w #$6c, d2
pub fn i_097ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AD4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097AD4: cmp.l $2(a2, d2.w), d0
pub fn i_097ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AD8;
    cpu.ir = 0xB0B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $097AD8: blt.w $97ae8
pub fn i_097ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ADC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x97AE8; return 10; }
    8
}
/// $097ADC: addq.w #$1, d3
pub fn i_097adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ADE;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097ADE: subi.w #$c, d2
pub fn i_097ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AE2;
    cpu.ir = 0x442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097AE2: bmi.w $97ae8
pub fn i_097ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AE6;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x97AE8; return 10; }
    8
}
/// $097AE6: bra.b $97ad4
pub fn i_097ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AE8;
    cpu.ir = 0x60EC;
    cpu.pc = 0x97AD4;
    10
}
/// $097AE8: tst.w d3
pub fn i_097ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AEA;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $097AEA: beq.w $97b6a
pub fn i_097aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97B6A; return 10; }
    8
}
/// $097AEE: subq.w #$1, d3
pub fn i_097aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AF0;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097AF0: beq.w $97b38
pub fn i_097af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97B38; return 10; }
    8
}
/// $097AF4: move.w #$6c, d4
pub fn i_097af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AF8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097AF8: move.w #$60, d5
pub fn i_097af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AFC;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097AFC: move.b (a2, d5.w), (a2, d4.w)
pub fn i_097afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B02;
    cpu.ir = 0x15B2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    24
}
/// $097B02: move.b $1(a2, d5.w), d6
pub fn i_097b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B06;
    cpu.ir = 0x1C32;
    let ad2 = cpu.a[2].wrapping_add(0x1).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $097B06: cmp.l $2(a2, d5.w), d0
pub fn i_097b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B0A;
    cpu.ir = 0xB0B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $097B0A: beq.w $97b10
pub fn i_097b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97B10; return 10; }
    8
}
/// $097B0E: addq.b #$1, d6
pub fn i_097b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B10;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $097B10: move.b d6, $1(a2, d4.w)
pub fn i_097b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B14;
    cpu.ir = 0x1586;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x1).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $097B14: move.l $2(a2, d5.w), $2(a2, d4.w)
pub fn i_097b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B1A;
    cpu.ir = 0x25B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $097B1A: move.w $6(a2, d5.w), $6(a2, d4.w)
pub fn i_097b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B20;
    cpu.ir = 0x35B2;
    let ad2 = cpu.a[2].wrapping_add(0x6).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x6).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $097B20: move.w $8(a2, d5.w), $8(a2, d4.w)
pub fn i_097b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B26;
    cpu.ir = 0x35B2;
    let ad2 = cpu.a[2].wrapping_add(0x8).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x8).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $097B26: move.w $a(a2, d5.w), $a(a2, d4.w)
pub fn i_097b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B2C;
    cpu.ir = 0x35B2;
    let ad2 = cpu.a[2].wrapping_add(0xA).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0xA).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $097B2C: subi.w #$c, d4
pub fn i_097b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B30;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097B30: subi.w #$c, d5
pub fn i_097b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B34;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097B34: cmp.w d5, d2
pub fn i_097b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B36;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $097B36: bne.b $97afc
pub fn i_097b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B38;
    cpu.ir = 0x66C4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97AFC; return 10; }
    8
}
/// $097B38: addi.w #$c, d2
pub fn i_097b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B3C;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097B3C: move.b d1, (a2, d2.w)
pub fn i_097b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B40;
    cpu.ir = 0x1581;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $097B40: move.l d0, $2(a2, d2.w)
pub fn i_097b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B44;
    cpu.ir = 0x2580;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $097B44: move.w d1, d0
pub fn i_097b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B46;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097B46: jsr $98176(pc)
pub fn i_097b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97B4A);
    cpu.pc = 0x98176;
    16
}
/// $097B4A: move.b d1, $1(a2, d2.w)
pub fn i_097b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B4E;
    cpu.ir = 0x1581;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x1).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $097B4E: move.w #$b40, $6(a2, d2.w)
pub fn i_097b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B54;
    cpu.ir = 0x35BC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097B54: move.w #$b40, $8(a2, d2.w)
pub fn i_097b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B5A;
    cpu.ir = 0x35BC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x8).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097B5A: move.w #$b40, $a(a2, d2.w)
pub fn i_097b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B60;
    cpu.ir = 0x35BC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0xA).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097B60: movem.l (a7)+, d0-d1/a1
pub fn i_097b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B64;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_097b64  ($097B64, 6 B, thunks/ccr_helpers.c)
/// $097B64: ori.b #$11, ccr
pub fn i_097b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B68;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $097B68: rts
pub fn i_097b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_InsertScore_NoRank_097b6a  ($097B6A, 4 B, frontend/hiscore_memcard_options_09773c.s)
/// $097B6A: movem.l (a7)+, d0-d1/a1
pub fn i_097b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B6E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_097b6e  ($097B6E, 6 B, thunks/ccr_helpers.c)
/// $097B6E: andi.b #$ee, ccr
pub fn i_097b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B72;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $097B72: rts
pub fn i_097b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_RankOfPlayer_098176  ($098176, 56 B, frontend/hiscore_memcard_options_09773c.s)
/// $098176: lea.l $100002.l, a1
pub fn i_098176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9817C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $09817C: moveq #$1, d1
pub fn i_09817c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9817E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09817E: cmp.b (a1), d0
pub fn i_09817e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98180;
    cpu.ir = 0xB011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    8
}
/// $098180: beq.w $981a4
pub fn i_098180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98184;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x981A4; return 10; }
    8
}
/// $098184: adda.l #$c, a1
pub fn i_098184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9818A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $09818A: addq.w #$1, d1
pub fn i_09818a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9818C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09818C: cmpi.w #$b, d1
pub fn i_09818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98190;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB, Size::Word, false);
    8
}
/// $098190: bne.b $9817e
pub fn i_098190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98192;
    cpu.ir = 0x66EC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9817E; return 10; }
    8
}
/// $098192: suba.l #$c, a1
pub fn i_098192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98198;
    cpu.ir = 0x93FC;
    let v1: u32 = 0xC;
    cpu.a[1] = cpu.a[1].wrapping_sub(v1);
    8
}
/// $098198: move.b $1(a1), d1
pub fn i_098198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9819C;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09819C: addq.b #$1, d1
pub fn i_09819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9819E;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09819E: andi.w #$ff, d1
pub fn i_09819e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981A2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0981A2: rts
pub fn i_0981a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0981A4: move.b $1(a1), d1
pub fn i_0981a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981A8;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0981A8: andi.w #$ff, d1
pub fn i_0981a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981AC;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0981AC: rts
pub fn i_0981ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Copy2Bytes_10FDB6to10E3A0_0981FC  ($0981FC, 18 B, kernel/copy_start_masks_0981fc.s)
/// $0981FC: lea.l $10fdb6.l, a0
pub fn i_0981fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98202;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $098202: lea.l $10e3a0.l, a1
pub fn i_098202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98208;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E3A0;
    20
}
/// $098208: move.b (a0)+, (a1)+
pub fn i_098208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9820A;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09820A: move.b (a0), (a1)
pub fn i_09820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9820C;
    cpu.ir = 0x1290;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v2 as u8);
    12
}
/// $09820C: rts
pub fn i_09820c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9820E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawGlyphList_09820e  ($09820E, 36 B, frontend/hiscore_memcard_options_09773c.s)
/// $09820E: move.w (a0)+, d0
pub fn i_09820e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98210;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $098210: cmpi.w #$ffff, d0
pub fn i_098210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98214;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $098214: beq.w $98230
pub fn i_098214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98218;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98230; return 10; }
    8
}
/// $098218: move.w #$2, d1
pub fn i_098218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9821C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09821C: move.w #$2, d2
pub fn i_09821c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98220;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098220: move.l a1, -(a7)
pub fn i_098220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98222;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $098222: jsr $5da56.l
pub fn i_098222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98228;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98228);
    cpu.pc = 0x5DA56;
    16
}
/// $098228: movea.l (a7)+, a1
pub fn i_098228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9822A;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $09822A: adda.w #$40, a1
pub fn i_09822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9822E;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $09822E: bra.b $9820e
pub fn i_09822e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98230;
    cpu.ir = 0x60DE;
    cpu.pc = 0x9820E;
    10
}
/// $098230: rts
pub fn i_098230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98232;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_EdgePressedPlayers_098232  ($098232, 60 B, frontend/hiscore_memcard_options_09773c.s)
/// $098232: moveq #$0, d0
pub fn i_098232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98234;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098234: move.b $10e3a0.l, d7
pub fn i_098234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9823A;
    cpu.ir = 0x1E39;
    let v2 = bus.read8(0x10E3A0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v2 & 0xFF);
    16
}
/// $09823A: subq.b #$1, d7
pub fn i_09823a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9823C;
    cpu.ir = 0x5307;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09823C: cmpi.b #$2, d7
pub fn i_09823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98240;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $098240: bcc.w $9824a
pub fn i_098240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98244;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9824A; return 10; }
    8
}
/// $098244: or.b $10fd96.l, d0
pub fn i_098244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9824A;
    cpu.ir = 0x8039;
    let v2 = bus.read8(0x10FD96) as u32;
    let r = (v2 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $09824A: move.b $10e3a1.l, d7
pub fn i_09824a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98250;
    cpu.ir = 0x1E39;
    let v2 = bus.read8(0x10E3A1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v2 & 0xFF);
    16
}
/// $098250: subq.b #$1, d7
pub fn i_098250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98252;
    cpu.ir = 0x5307;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $098252: cmpi.b #$2, d7
pub fn i_098252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98256;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $098256: bcc.w $98260
pub fn i_098256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9825A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98260; return 10; }
    8
}
/// $09825A: or.b $10fd9c.l, d0
pub fn i_09825a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98260;
    cpu.ir = 0x8039;
    let v2 = bus.read8(0x10FD9C) as u32;
    let r = (v2 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $098260: move.b $74(a6), d1
pub fn i_098260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98264;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $098264: move.b d0, $74(a6)
pub fn i_098264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98268;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $098268: eor.b d0, d1
pub fn i_098268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9826A;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09826A: and.b d1, d0
pub fn i_09826a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9826C;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09826C: rts
pub fn i_09826c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9826E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_Detect_09826e  ($09826E, 14 B, frontend/hiscore_memcard_options_09773c.s)
/// $09826E: move.b $380000.l, d0
pub fn i_09826e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98274;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $098274: andi.b #$30, d0
pub fn i_098274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98278;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $098278: beq.w $98282
pub fn i_098278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9827C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98282; return 10; }
    8
}
// ===== ClearC_09827c  ($09827C, 6 B, thunks/ccr_helpers.c)
/// $09827C: andi.b #$fe, ccr
pub fn i_09827c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98280;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $098280: rts
pub fn i_098280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_098282  ($098282, 6 B, thunks/ccr_helpers.c)
/// $098282: ori.b #$1, ccr
pub fn i_098282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98286;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $098286: rts
pub fn i_098286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_LoadDialog_Init_098288  ($098288, 120 B, frontend/hiscore_memcard_options_09773c.s)
/// $098288: jsr $98578(pc)
pub fn i_098288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9828C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9828C);
    cpu.pc = 0x98578;
    16
}
/// $09828C: bcs.w $983b0
pub fn i_09828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98290;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x983B0; return 10; }
    8
}
/// $098290: movea.l $98(a6), a0
pub fn i_098290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98294;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $098294: cmpa.l #$0, a0
pub fn i_098294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9829A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09829A: beq.w $982a0
pub fn i_09829a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9829E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x982A0; return 10; }
    8
}
/// $09829E: jsr (a0)
pub fn i_09829e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982A0;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x982A0);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $0982A0: movea.l #$7000, a1
pub fn i_0982a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982A6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1;
    4
}
/// $0982A6: move.w #$ff, d0
pub fn i_0982a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0982AA: move.w #$26, d1
pub fn i_0982aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0982AE: move.w #$1b, d2
pub fn i_0982ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982B2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0982B2: jsr $5da9c.l
pub fn i_0982b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x982B8);
    cpu.pc = 0x5DA9C;
    16
}
/// $0982B8: movea.l #$7204, a1
pub fn i_0982b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982BE;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7204;
    cpu.a[1] = v1;
    4
}
/// $0982BE: lea.l $98692(pc), a0
pub fn i_0982be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98692;
    16
}
/// $0982C2: bsr.w $9820e
pub fn i_0982c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982C6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982C6);
    cpu.pc = 0x9820E;
    18
}
/// $0982C6: movea.l #$724a, a1
pub fn i_0982c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982CC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724A;
    cpu.a[1] = v1;
    4
}
/// $0982CC: lea.l $9869e(pc), a0
pub fn i_0982cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982D0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9869E;
    16
}
/// $0982D0: bsr.w $9820e
pub fn i_0982d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982D4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982D4);
    cpu.pc = 0x9820E;
    18
}
/// $0982D4: movea.l #$724e, a1
pub fn i_0982d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982DA;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724E;
    cpu.a[1] = v1;
    4
}
/// $0982DA: lea.l $986a6(pc), a0
pub fn i_0982da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982DE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986A6;
    16
}
/// $0982DE: bsr.w $9820e
pub fn i_0982de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982E2);
    cpu.pc = 0x9820E;
    18
}
/// $0982E2: movea.l #$7116, a1
pub fn i_0982e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982E8;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7116;
    cpu.a[1] = v1;
    4
}
/// $0982E8: lea.l $986ac(pc), a0
pub fn i_0982e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986AC;
    16
}
/// $0982EC: bsr.w $9820e
pub fn i_0982ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982F0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982F0);
    cpu.pc = 0x9820E;
    18
}
/// $0982F0: move.w #$0, $72(a6)
pub fn i_0982f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0982F6: jsr $98232(pc)
pub fn i_0982f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x982FA);
    cpu.pc = 0x98232;
    16
}
/// $0982FA: move.w #$12c, $70(a6)
pub fn i_0982fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98300;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_098300  ($098300, 8 B, kernel/task_handlers.c)
/// $098300: lea.l $98308(pc), a1
pub fn i_098300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98304;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98308;
    16
}
/// $098304: move.l a1, (a6)
pub fn i_098304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98306;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098306: rts
pub fn i_098306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98308;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0983B0: lea.l $10e3a2.l, a0
pub fn i_0983b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3A2;
    20
}
/// $0983B6: clr.b $14(a0)
pub fn i_0983b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983BA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x14);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983BA: clr.b $15(a0)
pub fn i_0983ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983BE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x15);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983BE: clr.b $16(a0)
pub fn i_0983be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983C2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x16);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983C2: clr.b $17(a0)
pub fn i_0983c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983C6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x17);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983C6: clr.b $18(a0)
pub fn i_0983c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983CA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x18);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983CA: clr.b $19(a0)
pub fn i_0983ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983CE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x19);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983CE: clr.b $1a(a0)
pub fn i_0983ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983D2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x1A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983D2: clr.b $1b(a0)
pub fn i_0983d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983D6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x1B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983D6: bra.w $983fe
pub fn i_0983d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x983FE;
    10
}
/// $0983FE: clr.b $106ed2.l
pub fn i_0983fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98404;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $098404: jmp $518.l
pub fn i_098404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9840A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== MemCard_SaveDialog_Init_09840c  ($09840C, 110 B, frontend/hiscore_memcard_options_09773c.s)
/// $09840C: move.b $380000.l, d0
pub fn i_09840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98412;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $098412: andi.b #$30, d0
pub fn i_098412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98416;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $098416: beq.w $9841e
pub fn i_098416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9841A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9841E; return 10; }
    8
}
/// $09841A: bra.w $9856a
pub fn i_09841a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9841E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9856A;
    10
}
/// $09841E: movea.l #$7000, a1
pub fn i_09841e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98424;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1;
    4
}
/// $098424: move.w #$ff, d0
pub fn i_098424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98428;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098428: move.w #$26, d1
pub fn i_098428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9842C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09842C: move.w #$1b, d2
pub fn i_09842c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98430;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098430: jsr $5da9c.l
pub fn i_098430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98436);
    cpu.pc = 0x5DA9C;
    16
}
/// $098436: movea.l #$7204, a1
pub fn i_098436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9843C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7204;
    cpu.a[1] = v1;
    4
}
/// $09843C: lea.l $986de(pc), a0
pub fn i_09843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98440;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986DE;
    16
}
/// $098440: bsr.w $9820e
pub fn i_098440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98444;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98444);
    cpu.pc = 0x9820E;
    18
}
/// $098444: movea.l #$724a, a1
pub fn i_098444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9844A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724A;
    cpu.a[1] = v1;
    4
}
/// $09844A: lea.l $9869e(pc), a0
pub fn i_09844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9844E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9869E;
    16
}
/// $09844E: bsr.w $9820e
pub fn i_09844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98452;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98452);
    cpu.pc = 0x9820E;
    18
}
/// $098452: movea.l #$724e, a1
pub fn i_098452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98458;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724E;
    cpu.a[1] = v1;
    4
}
/// $098458: lea.l $986a6(pc), a0
pub fn i_098458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9845C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986A6;
    16
}
/// $09845C: bsr.w $9820e
pub fn i_09845c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98460;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98460);
    cpu.pc = 0x9820E;
    18
}
/// $098460: movea.l #$7116, a1
pub fn i_098460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98466;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7116;
    cpu.a[1] = v1;
    4
}
/// $098466: lea.l $986ac(pc), a0
pub fn i_098466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9846A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986AC;
    16
}
/// $09846A: bsr.w $9820e
pub fn i_09846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9846E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9846E);
    cpu.pc = 0x9820E;
    18
}
/// $09846E: move.w #$1, $72(a6)
pub fn i_09846e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98474;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098474: move.w #$12c, $70(a6)
pub fn i_098474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9847A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_09847a  ($09847A, 8 B, kernel/task_handlers.c)
/// $09847A: lea.l $98482(pc), a1
pub fn i_09847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9847E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98482;
    16
}
/// $09847E: move.l a1, (a6)
pub fn i_09847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098480: rts
pub fn i_098480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_SaveDialog_Run_098482  ($098482, 246 B, frontend/hiscore_memcard_options_09773c.s)
/// $098482: lea.l $98700(pc), a0
pub fn i_098482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98486;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98700;
    16
}
/// $098486: move.w $72(a6), d0
pub fn i_098486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9848A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09848A: add.w d0, d0
pub fn i_09848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9848C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09848C: move.w (a0, d0.w), d0
pub fn i_09848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98490;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $098490: movea.l #$720a, a1
pub fn i_098490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98496;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720A;
    cpu.a[1] = v1;
    4
}
/// $098496: move.w #$2, d1
pub fn i_098496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9849A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09849A: move.w #$2, d2
pub fn i_09849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9849E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09849E: jsr $5da56.l
pub fn i_09849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x984A4);
    cpu.pc = 0x5DA56;
    16
}
/// $0984A4: lea.l $98700(pc), a0
pub fn i_0984a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984A8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98700;
    16
}
/// $0984A8: move.w $72(a6), d0
pub fn i_0984a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0984AC: eori.w #$1, d0
pub fn i_0984ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984B0;
    cpu.ir = 0xA40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0984B0: add.w d0, d0
pub fn i_0984b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984B2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0984B2: move.w (a0, d0.w), d0
pub fn i_0984b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984B6;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0984B6: movea.l #$720e, a1
pub fn i_0984b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984BC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720E;
    cpu.a[1] = v1;
    4
}
/// $0984BC: move.w #$2, d1
pub fn i_0984bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0984C0: move.w #$2, d2
pub fn i_0984c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984C4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0984C4: jsr $5da56.l
pub fn i_0984c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x984CA);
    cpu.pc = 0x5DA56;
    16
}
/// $0984CA: subq.w #$1, $70(a6)
pub fn i_0984ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984CE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0984CE: beq.w $984fc
pub fn i_0984ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x984FC; return 10; }
    8
}
/// $0984D2: bsr.w $98232
pub fn i_0984d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x984D6);
    cpu.pc = 0x98232;
    18
}
/// $0984D6: btst.b #$4, d0
pub fn i_0984d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984DA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0984DA: bne.w $984fc
pub fn i_0984da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x984FC; return 10; }
    8
}
/// $0984DE: btst.b #$0, d0
pub fn i_0984de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984E2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0984E2: beq.w $984ec
pub fn i_0984e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x984EC; return 10; }
    8
}
/// $0984E6: move.w #$0, $72(a6)
pub fn i_0984e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0984EC: btst.b #$1, d0
pub fn i_0984ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984F0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0984F0: beq.w $984fa
pub fn i_0984f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x984FA; return 10; }
    8
}
/// $0984F4: move.w #$1, $72(a6)
pub fn i_0984f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0984FA: rts
pub fn i_0984fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0984FC: move.w #$ff, d0
pub fn i_0984fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98500;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098500: movea.l #$7116, a1
pub fn i_098500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98506;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7116;
    cpu.a[1] = v1;
    4
}
/// $098506: move.w #$1a, d1
pub fn i_098506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9850A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09850A: move.w #$2, d2
pub fn i_09850a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9850E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09850E: jsr $5da9c.l
pub fn i_09850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98514);
    cpu.pc = 0x5DA9C;
    16
}
/// $098514: tst.w $72(a6)
pub fn i_098514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98518;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $098518: bne.w $9856a
pub fn i_098518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9851C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9856A; return 10; }
    8
}
/// $09851C: move.b #$0, $3a0005.l
pub fn i_09851c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98524;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $098524: move.b #$0, $3a0017.l
pub fn i_098524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9852C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $09852C: move.b #$0, $3a0019.l
pub fn i_09852c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98534;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $098534: move.b #$0, $380011.l
pub fn i_098534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9853C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $09853C: jsr $9866a(pc)
pub fn i_09853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98540;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x98540);
    cpu.pc = 0x9866A;
    16
}
/// $098540: jsr $985ca(pc)
pub fn i_098540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98544;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x98544);
    cpu.pc = 0x985CA;
    16
}
/// $098544: bcs.w $9856a
pub fn i_098544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98548;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9856A; return 10; }
    8
}
/// $098548: movea.l #$7152, a1
pub fn i_098548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9854E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7152;
    cpu.a[1] = v1;
    4
}
/// $09854E: lea.l $986ea(pc), a0
pub fn i_09854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98552;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986EA;
    16
}
/// $098552: bsr.w $9820e
pub fn i_098552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98556;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98556);
    cpu.pc = 0x9820E;
    18
}
/// $098556: move.w #$1e, $70(a6)
pub fn i_098556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9855C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09855C: lea.l $98562(pc), a1
pub fn i_09855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98560;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98562;
    16
}
/// $098560: move.l a1, (a6)
pub fn i_098560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98562;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098562: subq.w #$1, $70(a6)
pub fn i_098562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98566;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098566: bne.w $98576
pub fn i_098566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9856A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98576; return 10; }
    8
}
/// $09856A: clr.b $106ed2.l
pub fn i_09856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98570;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $098570: jmp $518.l
pub fn i_098570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98576;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $098576: rts
pub fn i_098576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98578;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosRead_098578  ($098578, 66 B, frontend/hiscore_memcard_options_09773c.s)
/// $098578: movem.l d0-d7/a0-a6, -(a7)
pub fn i_098578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9857C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $09857C: move.b #$2, $10fdc4.l
pub fn i_09857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98584;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC4, v1 as u8);
    16
}
/// $098584: move.w #$201, $10fdce.l
pub fn i_098584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9858C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCE, v1 as u16);
    16
}
/// $09858C: move.b #$0, $10fdd0.l
pub fn i_09858c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98594;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDD0, v1 as u8);
    16
}
/// $098594: move.l #$10e3a2, $10fdc8.l
pub fn i_098594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9859E;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10E3A2;
    bus.write32(0x10FDC8, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09859E: move.w #$1c, $10fdcc.l
pub fn i_09859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985A6;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCC, v1 as u16);
    16
}
/// $0985A6: jsr $c00468.l
pub fn i_0985a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x985AC);
    cpu.pc = 0xC00468;
    16
}
/// $0985AC: tst.b $10fdc6.l
pub fn i_0985ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985B2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0985B2: beq.w $985c0
pub fn i_0985b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x985C0; return 10; }
    8
}
/// $0985B6: movem.l (a7)+, d0-d7/a0-a6
pub fn i_0985b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985BA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_0985ba  ($0985BA, 6 B, thunks/ccr_helpers.c)
/// $0985BA: ori.b #$11, ccr
pub fn i_0985ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0985BE: rts
pub fn i_0985be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosRead_Ok_0985c0  ($0985C0, 4 B, frontend/hiscore_memcard_options_09773c.s)
/// $0985C0: movem.l (a7)+, d0-d7/a0-a6
pub fn i_0985c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985C4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_0985c4  ($0985C4, 6 B, thunks/ccr_helpers.c)
/// $0985C4: andi.b #$ee, ccr
pub fn i_0985c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985C8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0985C8: rts
pub fn i_0985c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosWrite_0985ca  ($0985CA, 144 B, frontend/hiscore_memcard_options_09773c.s)
/// $0985CA: movem.l d0-d7/a0-a6, -(a7)
pub fn i_0985ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985CE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0985CE: move.b #$3, $10fdc4.l
pub fn i_0985ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985D6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC4, v1 as u8);
    16
}
/// $0985D6: move.w #$201, $10fdce.l
pub fn i_0985d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985DE;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCE, v1 as u16);
    16
}
/// $0985DE: move.b #$0, $10fdd0.l
pub fn i_0985de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985E6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDD0, v1 as u8);
    16
}
/// $0985E6: lea.l $10e3a2.l, a0
pub fn i_0985e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3A2;
    20
}
/// $0985EC: lea.l $981e8(pc), a1
pub fn i_0985ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x981E8;
    16
}
/// $0985F0: move.l (a1)+, (a0)+
pub fn i_0985f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F2;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F2: move.l (a1)+, (a0)+
pub fn i_0985f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F4;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F4: move.l (a1)+, (a0)+
pub fn i_0985f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F6;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F6: move.l (a1)+, (a0)+
pub fn i_0985f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F8;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F8: move.l (a1)+, (a0)+
pub fn i_0985f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985FA;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985FA: move.l #$10e3a2, $10fdc8.l
pub fn i_0985fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98604;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10E3A2;
    bus.write32(0x10FDC8, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $098604: move.w #$1c, $10fdcc.l
pub fn i_098604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9860C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCC, v1 as u16);
    16
}
/// $09860C: jsr $c00468.l
pub fn i_09860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98612);
    cpu.pc = 0xC00468;
    16
}
/// $098612: tst.b $10fdc6.l
pub fn i_098612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98618;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $098618: beq.w $98660
pub fn i_098618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9861C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98660; return 10; }
    8
}
/// $09861C: movea.l #$7000, a1
pub fn i_09861c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98622;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1;
    4
}
/// $098622: move.w #$ff, d0
pub fn i_098622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98626;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098626: move.w #$26, d1
pub fn i_098626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9862A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09862A: move.w #$1b, d2
pub fn i_09862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9862E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09862E: jsr $5da9c.l
pub fn i_09862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98634);
    cpu.pc = 0x5DA9C;
    16
}
/// $098634: move.b $10fdc2.l, -(a7)
pub fn i_098634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9863A;
    cpu.ir = 0x1F39;
    let v2 = bus.read8(0x10FDC2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad3;
    bus.write8(ad3, v2 as u8);
    22
}
/// $09863A: clr.b $10fdc2.l
pub fn i_09863a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98640;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FDC2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $098640: jsr $c0046e.l
pub fn i_098640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98646);
    cpu.pc = 0xC0046E;
    16
}
/// $098646: move.b (a7)+, $10fdc2.l
pub fn i_098646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9864C;
    cpu.ir = 0x13DF;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC2, v3 as u8);
    20
}
/// $09864C: tst.b $10fdc6.l
pub fn i_09864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98652;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $098652: beq.w $98660
pub fn i_098652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98656;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98660; return 10; }
    8
}
/// $098656: movem.l (a7)+, d0-d7/a0-a6
pub fn i_098656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9865A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_09865a  ($09865A, 6 B, thunks/ccr_helpers.c)
/// $09865A: ori.b #$11, ccr
pub fn i_09865a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9865E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09865E: rts
pub fn i_09865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98660;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosWrite_Ok_098660  ($098660, 4 B, frontend/hiscore_memcard_options_09773c.s)
/// $098660: movem.l (a7)+, d0-d7/a0-a6
pub fn i_098660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98664;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_098664  ($098664, 6 B, thunks/ccr_helpers.c)
/// $098664: andi.b #$ee, ccr
pub fn i_098664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98668;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $098668: rts
pub fn i_098668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9866A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosCreate_09866a  ($09866A, 40 B, frontend/hiscore_memcard_options_09773c.s)
/// $09866A: movem.l d0-d7/a0-a6, -(a7)
pub fn i_09866a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9866E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $09866E: move.b #$4, $10fdc4.l
pub fn i_09866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98676;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC4, v1 as u8);
    16
}
/// $098676: move.w #$201, $10fdce.l
pub fn i_098676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9867E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCE, v1 as u16);
    16
}
/// $09867E: move.b #$0, $10fdd0.l
pub fn i_09867e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98686;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDD0, v1 as u8);
    16
}
/// $098686: jsr $c00468.l
pub fn i_098686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9868C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9868C);
    cpu.pc = 0xC00468;
    16
}
/// $09868C: movem.l (a7)+, d0-d7/a0-a6
pub fn i_09868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98690;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $098690: rts
pub fn i_098690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98692;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_RingReset_0997b8  ($0997B8, 20 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997B8: clr.w $10e47e.l
pub fn i_0997b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997BE;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E47E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E47E, r2 as u16);
    16
}
/// $0997BE: clr.w $10e480.l
pub fn i_0997be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997C4;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E480) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E480, r2 as u16);
    16
}
/// $0997C4: clr.w $10e482.l
pub fn i_0997c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997CA;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E482) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E482, r2 as u16);
    16
}
/// $0997CA: rts
pub fn i_0997ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_RingAdvance_0997cc  ($0997CC, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997CC: move.w $10e480.l, $10e47e.l
pub fn i_0997cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997D6;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x10E480) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E47E, v2 as u16);
    28
}
/// $0997D6: move.w $10e482.l, $10e480.l
pub fn i_0997d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E0;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x10E482) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E480, v2 as u16);
    28
}
/// $0997E0: rts
pub fn i_0997e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TrailRecord_099812  ($099812, 184 B, entity/trail_record_099812.s)
/// $099812: lea.l $10e3be.l, a0
pub fn i_099812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99818;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099818: move.b $6c(a6), d7
pub fn i_099818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9981C;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09981C: lsr.b #$4, d7
pub fn i_09981c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9981E;
    cpu.ir = 0xE80F;
    let h1 = cpu.d[7] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    14
}
/// $09981E: cmpi.b #$f, d7
pub fn i_09981e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99822;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $099822: bne.w $9985a
pub fn i_099822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99826;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9985A; return 10; }
    8
}
/// $099826: move.b $10e484.l, d7
pub fn i_099826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9982C;
    cpu.ir = 0x1E39;
    let v2 = bus.read8(0x10E484) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v2 & 0xFF);
    16
}
/// $09982C: addq.b #$1, d7
pub fn i_09982c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9982E;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09982E: cmpi.b #$e, d7
pub fn i_09982e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99832;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $099832: bcs.w $9983a
pub fn i_099832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99836;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9983A; return 10; }
    8
}
/// $099836: move.b #$0, d7
pub fn i_099836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9983A;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09983A: move.b d7, $10e484.l
pub fn i_09983a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99840;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E484, v1 as u8);
    16
}
/// $099840: move.b d7, d6
pub fn i_099840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99842;
    cpu.ir = 0x1C07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $099842: lsl.b #$4, d6
pub fn i_099842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99844;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $099844: move.b $6c(a6), d5
pub fn i_099844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99848;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099848: andi.b #$f, d5
pub fn i_099848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9984C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09984C: or.b d6, d5
pub fn i_09984c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9984E;
    cpu.ir = 0x8A06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09984E: move.b d5, $6c(a6)
pub fn i_09984e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99852;
    cpu.ir = 0x1D45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099852: clr.w d5
pub fn i_099852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99854;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099854: clr.w d6
pub fn i_099854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99856;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099856: bra.w $99896
pub fn i_099856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9985A;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $09985A: move.w $10e47e.l, d3
pub fn i_09985a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99860;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $099860: cmp.w $10e480.l, d3
pub fn i_099860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99866;
    cpu.ir = 0xB679;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099866: bne.w $99872
pub fn i_099866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99872; return 10; }
    8
}
/// $09986A: clr.w d5
pub fn i_09986a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986C;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09986C: clr.w d6
pub fn i_09986c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986E;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09986E: bra.w $99896
pub fn i_09986e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99872;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $099872: cmp.b (a0, d3.w), d7
pub fn i_099872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99876;
    cpu.ir = 0xBE30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $099876: beq.w $9988a
pub fn i_099876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9987A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9988A; return 10; }
    8
}
/// $09987A: addi.w #$c, d3
pub fn i_09987a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9987E;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09987E: cmpi.w #$c0, d3
pub fn i_09987e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99882;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $099882: bcs.w $99888
pub fn i_099882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99886;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99888; return 10; }
    8
}
/// $099886: clr.w d3
pub fn i_099886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99888;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099888: bra.b $99860
pub fn i_099888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9988A;
    cpu.ir = 0x60D6;
    cpu.pc = 0x99860;
    10
}
/// $09988A: move.w d0, d5
pub fn i_09988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9988C;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09988C: sub.w $2(a0, d3.w), d5
pub fn i_09988c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99890;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099890: move.w d1, d6
pub fn i_099890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99892;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099892: sub.w $4(a0, d3.w), d6
pub fn i_099892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99896;
    cpu.ir = 0x9C70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099896: move.w $10e482.l, d3
pub fn i_099896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9989C;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10E482) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $09989C: move.b d7, (a0, d3.w)
pub fn i_09989c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A0;
    cpu.ir = 0x1187;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $0998A0: move.w d0, $2(a0, d3.w)
pub fn i_0998a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A4;
    cpu.ir = 0x3180;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998A4: move.w d1, $4(a0, d3.w)
pub fn i_0998a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A8;
    cpu.ir = 0x3181;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998A8: move.w d2, $6(a0, d3.w)
pub fn i_0998a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998AC;
    cpu.ir = 0x3182;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998AC: move.w d5, $8(a0, d3.w)
pub fn i_0998ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B0;
    cpu.ir = 0x3185;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998B0: move.w d6, $a(a0, d3.w)
pub fn i_0998b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B4;
    cpu.ir = 0x3186;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998B4: addi.w #$c, d3
pub fn i_0998b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B8;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998B8: cmpi.w #$c0, d3
pub fn i_0998b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998BC;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $0998BC: bcs.w $998c2
pub fn i_0998bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x998C2; return 10; }
    8
}
/// $0998C0: clr.w d3
pub fn i_0998c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C2;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0998C2: move.w d3, $10e482.l
pub fn i_0998c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C8;
    cpu.ir = 0x33C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E482, v1 as u16);
    16
}
/// $0998C8: rts
pub fn i_0998c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_0998ca  ($0998CA, 30 B, frontend/hiscore_memcard_options_09773c.s)
/// $0998CA: movem.l d7/a0, -(a7)
pub fn i_0998ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998CE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0998CE: lea.l $10e3be.l, a0
pub fn i_0998ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $0998D4: move.w $10e480.l, d7
pub fn i_0998d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998DA;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E480) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0998DA: cmp.w $10e47e.l, d7
pub fn i_0998da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E0;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E47E) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $0998E0: bne.w $998ee
pub fn i_0998e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x998EE; return 10; }
    8
}
/// $0998E4: movem.l (a7)+, d7/a0
pub fn i_0998e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearC_0998e8  ($0998E8, 6 B, thunks/ccr_helpers.c)
/// $0998E8: andi.b #$fe, ccr
pub fn i_0998e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0998EC: rts
pub fn i_0998ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_Step_0998ee  ($0998EE, 70 B, frontend/hiscore_memcard_options_09773c.s)
/// $0998EE: subi.w #$c, d7
pub fn i_0998ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998F2;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998F2: bcc.w $998fa
pub fn i_0998f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x998FA; return 10; }
    8
}
/// $0998F6: addi.w #$c0, d7
pub fn i_0998f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998FA;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC0, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998FA: cmp.w $4(a0, d7.w), d2
pub fn i_0998fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998FE;
    cpu.ir = 0xB470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $0998FE: bne.w $9993a
pub fn i_0998fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99902;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $099902: move.w d1, d5
pub fn i_099902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99904;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099904: sub.w $2(a0, d7.w), d5
pub fn i_099904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99908;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099908: cmp.w $6(a0, d7.w), d5
pub fn i_099908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9990C;
    cpu.ir = 0xBA70;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $09990C: bcc.w $9993a
pub fn i_09990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99910;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $099910: move.b $6c(a6), d5
pub fn i_099910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99914;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099914: lsr.b #$4, d5
pub fn i_099914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99916;
    cpu.ir = 0xE80D;
    let h1 = cpu.d[5] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    14
}
/// $099916: cmp.b (a0, d7.w), d5
pub fn i_099916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9991A;
    cpu.ir = 0xBA30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $09991A: beq.w $9993a
pub fn i_09991a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9991E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $09991E: andi.b #$f0, $6c(a6)
pub fn i_09991e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99924;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $099924: move.b (a0, d7.w), d5
pub fn i_099924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99928;
    cpu.ir = 0x1A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $099928: andi.b #$f, d5
pub fn i_099928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9992C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09992C: or.b d5, $6c(a6)
pub fn i_09992c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99930;
    cpu.ir = 0x8B2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $099930: movem.l (a7)+, d7/a0
pub fn i_099930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99934;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetC_099934  ($099934, 6 B, thunks/ccr_helpers.c)
/// $099934: ori.b #$1, ccr
pub fn i_099934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99938;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $099938: rts
pub fn i_099938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9993A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_Next_09993a  ($09993A, 2 B, frontend/hiscore_memcard_options_09773c.s)
/// $09993A: bra.b $998da
pub fn i_09993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9993C;
    cpu.ir = 0x609E;
    cpu.pc = 0x998DA;
    10
}
// ===== Trail_FindNearest_09993c  ($09993C, 120 B, frontend/hiscore_memcard_options_09773c.s)
/// $09993C: movem.l d7/a0, -(a7)
pub fn i_09993c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99940;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $099940: lea.l $10e3be.l, a0
pub fn i_099940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99946;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099946: move.w $10e47e.l, d7
pub fn i_099946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9994C;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $09994C: cmp.w $10e480.l, d7
pub fn i_09994c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99952;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099952: bne.w $99976
pub fn i_099952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99956;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99976; return 10; }
    8
}
/// $099956: move.b $6c(a6), d7
pub fn i_099956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9995A;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09995A: andi.b #$f, d7
pub fn i_09995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9995E;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09995E: cmpi.b #$e, d7
pub fn i_09995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99962;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $099962: bne.w $9996c
pub fn i_099962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99966;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9996C; return 10; }
    8
}
/// $099966: ori.b #$f, $6c(a6)
pub fn i_099966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9996C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09996C: move.b #$f, d6
pub fn i_09996c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99970;
    cpu.ir = 0x1C3C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $099970: movem.l (a7)+, d7/a0
pub fn i_099970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99974;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $099974: rts
pub fn i_099974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99976;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $099976: move.w $4(a0, d7.w), d5
pub fn i_099976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9997A;
    cpu.ir = 0x3A30;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09997A: cmp.w d5, d2
pub fn i_09997a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9997C;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $09997C: bgt.w $999ba
pub fn i_09997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99980;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $099980: subi.w #$14, d5
pub fn i_099980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99984;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x14, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099984: cmp.w d5, d2
pub fn i_099984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99986;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $099986: ble.w $999ba
pub fn i_099986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9998A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $09998A: move.w d1, d5
pub fn i_09998a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9998C;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09998C: sub.w $2(a0, d7.w), d5
pub fn i_09998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99990;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099990: cmp.w $6(a0, d7.w), d5
pub fn i_099990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99994;
    cpu.ir = 0xBA70;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $099994: bcc.w $999ba
pub fn i_099994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99998;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $099998: move.b $6c(a6), d6
pub fn i_099998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9999C;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09999C: lsr.b #$4, d6
pub fn i_09999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9999E;
    cpu.ir = 0xE80E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $09999E: cmp.b (a0, d7.w), d6
pub fn i_09999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999A2;
    cpu.ir = 0xBC30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $0999A2: beq.w $999ba
pub fn i_0999a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $0999A6: move.w $4(a0, d7.w), d5
pub fn i_0999a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999AA;
    cpu.ir = 0x3A30;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0999AA: sub.w d2, d5
pub fn i_0999aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999AC;
    cpu.ir = 0x9A42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0999AC: move.b (a0, d7.w), d6
pub fn i_0999ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B0;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0999B0: movem.l (a7)+, d7/a0
pub fn i_0999b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetC_0999b4  ($0999B4, 6 B, thunks/ccr_helpers.c)
/// $0999B4: ori.b #$1, ccr
pub fn i_0999b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0999B8: rts
pub fn i_0999b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindNearest_Next_0999ba  ($0999BA, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999BA: addi.w #$c, d7
pub fn i_0999ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999BE;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999BE: cmpi.w #$c0, d7
pub fn i_0999be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C2;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $0999C2: bcs.w $999c8
pub fn i_0999c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x999C8; return 10; }
    8
}
/// $0999C6: clr.w d7
pub fn i_0999c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C8;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0999C8: bra.b $9994c
pub fn i_0999c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999CA;
    cpu.ir = 0x6082;
    cpu.pc = 0x9994C;
    10
}
// ===== Trail_MergeIdNibble_0999ca  ($0999CA, 14 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999CA: move.b $6c(a6), d7
pub fn i_0999ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999CE;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0999CE: andi.w #$f0, d7
pub fn i_0999ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D2;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999D2: andi.w #$f, d0
pub fn i_0999d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999D6: or.w d7, d0
pub fn i_0999d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D8;
    cpu.ir = 0x8047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
// ===== SetTaskB_0999d8  ($0999D8, 6 B, kernel/task_setters_b.c)
/// $0999D8: move.b d0, $6c(a6)
pub fn i_0999d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999DC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0999DC: rts
pub fn i_0999dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Clipping_Test_0999DE  ($0999DE, 24 B, video/clipping_test_0999de.s)
/// $0999DE: move.b $6c(a6), d5
pub fn i_0999de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E2;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0999E2: move.b d5, d6
pub fn i_0999e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E4;
    cpu.ir = 0x1C05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0999E4: andi.b #$f, d5
pub fn i_0999e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E8;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0999E8: cmpi.b #$f, d5
pub fn i_0999e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999EC;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $0999EC: bne.w $999f6
pub fn i_0999ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x999F6; return 10; }
    8
}
/// $0999F0: andi.b #$fe, ccr
pub fn i_0999f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0999F4: rts
pub fn i_0999f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_LookupById_0999f6  ($0999F6, 38 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999F6: lsr.b #$4, d6
pub fn i_0999f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F8;
    cpu.ir = 0xE80E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $0999F8: cmp.b d6, d5
pub fn i_0999f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999FA;
    cpu.ir = 0xBA06;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0999FA: bne.w $99a00
pub fn i_0999fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99A00; return 10; }
    8
}
/// $0999FE: trap #$f
pub fn i_0999fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A00;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $099A00: lea.l $10e3be.l, a0
pub fn i_099a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099A06: move.w $10e47e.l, d7
pub fn i_099a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A0C;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $099A0C: cmp.w $10e480.l, d7
pub fn i_099a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A12;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099A12: bne.w $99a22
pub fn i_099a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99A22; return 10; }
    8
}
/// $099A16: ori.b #$f, $6c(a6)
pub fn i_099a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A1C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_099a1c  ($099A1C, 6 B, thunks/ccr_helpers.c)
/// $099A1C: andi.b #$fe, ccr
pub fn i_099a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A20;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $099A20: rts
pub fn i_099a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_LookupById_Step_099a22  ($099A22, 34 B, frontend/hiscore_memcard_options_09773c.s)
/// $099A22: cmp.b (a0, d7.w), d5
pub fn i_099a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A26;
    cpu.ir = 0xBA30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $099A26: beq.w $99a3a
pub fn i_099a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A3A; return 10; }
    8
}
/// $099A2A: addi.w #$c, d7
pub fn i_099a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A2E;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099A2E: cmpi.w #$c0, d7
pub fn i_099a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A32;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $099A32: bcs.w $99a38
pub fn i_099a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A36;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99A38; return 10; }
    8
}
/// $099A36: clr.w d7
pub fn i_099a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A38;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099A38: bra.b $99a0c
pub fn i_099a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A3A;
    cpu.ir = 0x60D2;
    cpu.pc = 0x99A0C;
    10
}
/// $099A3A: adda.w d7, a0
pub fn i_099a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A3C;
    cpu.ir = 0xD0C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $099A3C: move.w $8(a0), d2
pub fn i_099a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A40;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099A40: move.w $a(a0), d3
pub fn i_099a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A44;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== SetC_099a44  ($099A44, 6 B, thunks/ccr_helpers.c)
/// $099A44: ori.b #$1, ccr
pub fn i_099a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A48;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $099A48: rts
pub fn i_099a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AttractInit_Single_099AE2  ($099AE2, 26 B, frontend/attract_init_single_099ae2.s)
/// $099AE2: move.b #$3, $10fd88.l
pub fn i_099ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AEA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD88, v1 as u8);
    16
}
/// $099AEA: move.b #$2, $10fd8b.l
pub fn i_099aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AF2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD8B, v1 as u8);
    16
}
/// $099AF2: move.b #$0, $10e486.l
pub fn i_099af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AFA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E486, v1 as u8);
    16
}
/// $099AFA: rts
pub fn i_099afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Global_Clear10E486_099AFC  ($099AFC, 10 B, kernel/global_clear_10e486_099afc.s)
/// $099AFC: move.b #$0, $10e486.l
pub fn i_099afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B04;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E486, v1 as u8);
    16
}
/// $099B04: rts
pub fn i_099b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_GroundTail_09a75a  ($09A75A, 80 B, props/items_score_crates_09a0bc.s)
/// $09A75A: cmpi.b #$1, $90(a6)
pub fn i_09a75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A760;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $09A760: bne.w $9a76a
pub fn i_09a760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A764;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A76A; return 10; }
    8
}
/// $09A764: addq.b #$1, $10e48b.l
pub fn i_09a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A76A;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10E48B) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E48B, r2 as u8);
    16
}
/// $09A76A: tst.b $9f(a6)
pub fn i_09a76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A76E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A76E: beq.w $9a78e
pub fn i_09a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A772;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A78E; return 10; }
    8
}
/// $09A772: movea.l #$ffffffff, a0
pub fn i_09a772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A778;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A778: lea.l $9a5f4(pc), a0
pub fn i_09a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A77C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A5F4;
    16
}
/// $09A77C: jsr $5dd56.l
pub fn i_09a77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A782);
    cpu.pc = 0x5DD56;
    16
}
/// $09A782: bcc.w $9a78c
pub fn i_09a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A786;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A78C; return 10; }
    8
}
/// $09A786: jmp $518.l
pub fn i_09a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A78C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A78C: rts
pub fn i_09a78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A78E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A78E: movea.l #$ffffffff, a0
pub fn i_09a78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A794;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A794: lea.l $9a5f4(pc), a0
pub fn i_09a794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A798;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A5F4;
    16
}
/// $09A798: jsr $5dd5c.l
pub fn i_09a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A79E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A79E);
    cpu.pc = 0x5DD5C;
    16
}
/// $09A79E: bcc.w $9a7a8
pub fn i_09a79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A7A8; return 10; }
    8
}
/// $09A7A2: jmp $518.l
pub fn i_09a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A7A8: rts
pub fn i_09a7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeMoveX_09A7AA  ($09A7AA, 34 B, entity/probe_move_x_09a7aa.s)
/// $09A7AA: jsr $9a7cc(pc)
pub fn i_09a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A7AE);
    cpu.pc = 0x9A7CC;
    16
}
/// $09A7AE: bcs.w $9a7ca
pub fn i_09a7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7B2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A7CA; return 10; }
    8
}
/// $09A7B2: move.w #$10, d0
pub fn i_09a7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A7B6: btst.b #$0, $3a(a6)
pub fn i_09a7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7BC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A7BC: beq.w $9a7c2
pub fn i_09a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A7C2; return 10; }
    8
}
/// $09A7C0: neg.w d0
pub fn i_09a7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A7C2: add.w d0, $22(a0)
pub fn i_09a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A7C6: andi.b #$ee, ccr
pub fn i_09a7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09A7CA: rts
pub fn i_09a7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SpawnFromParent_09a7cc  ($09A7CC, 106 B, props/items_score_crates_09a0bc.s)
/// $09A7CC: tst.b d0
pub fn i_09a7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CE;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09A7CE: beq.w $9a830
pub fn i_09a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A830; return 10; }
    8
}
/// $09A7D2: move.w d1, -(a7)
pub fn i_09a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D4;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $09A7D4: andi.w #$3f, d0
pub fn i_09a7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A7D8: cmpi.w #$1e, d0
pub fn i_09a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7DC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1E, Size::Word, false);
    8
}
/// $09A7DC: bcs.w $9a7e4
pub fn i_09a7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A7E4; return 10; }
    8
}
/// $09A7E0: move.w #$a, d0
pub fn i_09a7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A7E4: add.w d0, d0
pub fn i_09a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A7E6: add.w d0, d0
pub fn i_09a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A7E8: lea.l $9a6de(pc), a0
pub fn i_09a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A6DE;
    16
}
/// $09A7EC: movea.l (a0, d0.w), a1
pub fn i_09a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F0;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $09A7F0: move.l a6, -(a7)
pub fn i_09a7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F2;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A7F2: lea.l $100800.l, a6
pub fn i_09a7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F8;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $09A7F8: jsr $4ae.l
pub fn i_09a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A7FE);
    cpu.pc = 0x4AE;
    16
}
/// $09A7FE: movea.l (a7)+, a6
pub fn i_09a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A800;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $09A800: move.w $24(a6), $24(a0)
pub fn i_09a800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A806;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A806: move.w #$10, d0
pub fn i_09a806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A80A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A80A: btst.b #$0, $3a(a6)
pub fn i_09a80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A810;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A810: bne.w $9a816
pub fn i_09a810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A814;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A816; return 10; }
    8
}
/// $09A814: neg.w d0
pub fn i_09a814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A816;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A816: add.w $22(a6), d0
pub fn i_09a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A81A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A81A: move.w d0, $22(a0)
pub fn i_09a81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A81E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A81E: move.w (a7)+, d1
pub fn i_09a81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A820;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $09A820: move.b (a6, d1.w), $98(a0)
pub fn i_09a820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A826;
    cpu.ir = 0x1176;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    22
}
/// $09A826: move.b $1(a6, d1.w), $99(a0)
pub fn i_09a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A82C;
    cpu.ir = 0x1176;
    let ad2 = cpu.a[6].wrapping_add(0x1).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    22
}
/// $09A82C: ori.b #$11, ccr
pub fn i_09a82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A830;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09A830: eori.b #$11, ccr
pub fn i_09a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A834;
    cpu.ir = 0xA3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) ^ 0x11);
    20
}
/// $09A834: rts
pub fn i_09a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_PickupTestA_09a836  ($09A836, 12 B, props/items_score_crates_09a0bc.s)
/// $09A836: lea.l $9a6ce(pc), a1
pub fn i_09a836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A83A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A6CE;
    16
}
/// $09A83A: tst.b $45(a6)
pub fn i_09a83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A83E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A83E: beq.w $9a848
pub fn i_09a83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A842;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A848; return 10; }
    8
}
// ===== ClearC_09a842  ($09A842, 6 B, thunks/ccr_helpers.c)
/// $09A842: andi.b #$fe, ccr
pub fn i_09a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A846;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $09A846: rts
pub fn i_09a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A848;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_09a848  ($09A848, 6 B, thunks/jmp_abs_thunks.c)
/// $09A848: jmp $3ee48.l
pub fn i_09a848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A84E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x3EE48;
    8
}
// ===== Item_PickupTestB_09a84e  ($09A84E, 6 B, props/items_score_crates_09a0bc.s)
/// $09A84E: lea.l $9a6d6(pc), a1
pub fn i_09a84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A852;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A6D6;
    16
}
/// $09A852: bra.b $9a83a
pub fn i_09a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A854;
    cpu.ir = 0x60E6;
    cpu.pc = 0x9A83A;
    10
}
// ===== Item_GiveAmmoKind_09a854  ($09A854, 34 B, props/items_score_crates_09a0bc.s)
/// $09A854: move.b $98(a6), d2
pub fn i_09a854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A858;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A858: jsr $32b58.l
pub fn i_09a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A85E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A85E);
    cpu.pc = 0x32B58;
    16
}
/// $09A85E: cmpi.b #$2, $70(a1)
pub fn i_09a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A864;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09A864: bne.w $9a876
pub fn i_09a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A868;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A876; return 10; }
    8
}
/// $09A868: move.l a1, -(a7)
pub fn i_09a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A86A;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A86A: move.w #$10fd, d0
pub fn i_09a86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A86E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A86E: jsr $2352.l
pub fn i_09a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A874);
    cpu.pc = 0x2352;
    16
}
/// $09A874: movea.l (a7)+, a1
pub fn i_09a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A876;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
// ===== JsrAbsThunk_09a876  ($09A876, 8 B, thunks/jsr_abs_thunks.c)
/// $09A876: jsr $32cba.l
pub fn i_09a876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A87C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A87C);
    cpu.pc = 0x32CBA;
    16
}
/// $09A87C: rts
pub fn i_09a87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A87E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Tmpl_PowCounted_09a8be  ($09A8BE, 18 B, props/items_score_crates_09a0bc.s)
/// $09A8BE: move.b #$1, $90(a6)
pub fn i_09a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A8C4: cmpi.b #$2, $10e48a.l
pub fn i_09a8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8CC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E48A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $09A8CC: blt.w $9a8de
pub fn i_09a8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8D0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x9A8DE; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_09a8d0  ($09A8D0, 14 B, kernel/jsr5b6_jmp518.c)
/// $09A8D0: jsr $5b6.l
pub fn i_09a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8D6);
    cpu.pc = 0x5B6;
    16
}
/// $09A8D6: jmp $518.l
pub fn i_09a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8DC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Item_FlyAway_09a8de  ($09A8DE, 58 B, props/items_score_crates_09a0bc.s)
/// $09A8DE: move.w #$180, d1
pub fn i_09a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A8E2: jsr $236e.l
pub fn i_09a8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8E8);
    cpu.pc = 0x236E;
    16
}
/// $09A8E8: lea.l $2f678a.l, a0
pub fn i_09a8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F678A;
    20
}
/// $09A8EE: jsr $28cd4.l
pub fn i_09a8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8F4);
    cpu.pc = 0x28CD4;
    16
}
/// $09A8F4: lea.l $9a8fa(pc), a1
pub fn i_09a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A8FA;
    16
}
/// $09A8F8: move.l a1, (a6)
pub fn i_09a8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A8FA: jsr $2783a.l
pub fn i_09a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A900);
    cpu.pc = 0x2783A;
    16
}
/// $09A900: jsr $28d70.l
pub fn i_09a900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A906);
    cpu.pc = 0x28D70;
    16
}
/// $09A906: bcc.w $9a914
pub fn i_09a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A90A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A914; return 10; }
    8
}
/// $09A90A: jsr $13600.l
pub fn i_09a90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A910;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A910);
    cpu.pc = 0x13600;
    16
}
/// $09A910: move.l $5c(a6), (a6)
pub fn i_09a910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A914;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $09A914: bra.w $9a75a
pub fn i_09a914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A918;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl284_Ammo4_09a918  ($09A918, 94 B, props/items_score_crates_09a0bc.s)
/// $09A918: move.w #$17d, d1
pub fn i_09a918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A91C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A91C: jsr $236e.l
pub fn i_09a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A922);
    cpu.pc = 0x236E;
    16
}
/// $09A922: move.b #$8, $45(a6)
pub fn i_09a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A928;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A928: move.w #$8000, d0
pub fn i_09a928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A92C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A92C: jsr $28134.l
pub fn i_09a92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A932);
    cpu.pc = 0x28134;
    16
}
/// $09A932: andi.w #$ffe3, $38(a6)
pub fn i_09a932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A938;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A938: ori.w #$0, $38(a6)
pub fn i_09a938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A93E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A93E: lea.l $2f67dc.l, a0
pub fn i_09a93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A944;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F67DC;
    20
}
/// $09A944: jsr $28cd4.l
pub fn i_09a944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A94A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A94A);
    cpu.pc = 0x28CD4;
    16
}
/// $09A94A: lea.l $9a950(pc), a1
pub fn i_09a94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A94E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A950;
    16
}
/// $09A94E: move.l a1, (a6)
pub fn i_09a94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A950: jsr $27c8c.l
pub fn i_09a950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A956);
    cpu.pc = 0x27C8C;
    16
}
/// $09A956: jsr $28d70.l
pub fn i_09a956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A95C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A95C);
    cpu.pc = 0x28D70;
    16
}
/// $09A95C: jsr $9a836(pc)
pub fn i_09a95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A960;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A960);
    cpu.pc = 0x9A836;
    16
}
/// $09A960: bcc.w $9a972
pub fn i_09a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A964;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A972; return 10; }
    8
}
/// $09A964: move.b #$4, d1
pub fn i_09a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A968;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09A968: jsr $9a854(pc)
pub fn i_09a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A96C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A96C);
    cpu.pc = 0x9A854;
    16
}
/// $09A96C: lea.l $9ae26(pc), a1
pub fn i_09a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09A970: move.l a1, (a6)
pub fn i_09a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A972: bra.w $9a75a
pub fn i_09a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A976;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
/// $09AF2A: clr.b $9c(a6)
pub fn i_09af2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF2E: lea.l $2f6cca.l, a0
pub fn i_09af2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6CCA;
    20
}
/// $09AF34: jsr $28cd4.l
pub fn i_09af34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF3A);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF3A: bra.w $9b064
pub fn i_09af3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AF68: clr.b $9c(a6)
pub fn i_09af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF6C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF6C: lea.l $2f706c.l, a0
pub fn i_09af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F706C;
    20
}
/// $09AF72: jsr $28cd4.l
pub fn i_09af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF78);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF78: bra.w $9b05c
pub fn i_09af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AFE0: clr.b $9c(a6)
pub fn i_09afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFE4: lea.l $2f745e.l, a0
pub fn i_09afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F745E;
    20
}
/// $09AFEA: jsr $28cd4.l
pub fn i_09afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AFF0);
    cpu.pc = 0x28CD4;
    16
}
/// $09AFF0: bra.w $9b064
pub fn i_09aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09B05C: move.w #$1ac, d1
pub fn i_09b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B060;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B060: bra.w $9b068
pub fn i_09b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B064;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B068;
    10
}
/// $09B064: move.w #$1c6, d1
pub fn i_09b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B068;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B068: cmpi.b #$ff, $9a(a6)
pub fn i_09b068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B06E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B06E: beq.w $9b078
pub fn i_09b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B072;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B078; return 10; }
    8
}
/// $09B072: eori.b #$1, $3a(a6)
pub fn i_09b072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B078;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B078: jsr $236e.l
pub fn i_09b078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B07E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B07E);
    cpu.pc = 0x236E;
    16
}
/// $09B07E: move.b #$8, $45(a6)
pub fn i_09b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B084;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B084: move.w #$8000, d0
pub fn i_09b084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B088;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B088: jsr $28134.l
pub fn i_09b088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B08E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B08E);
    cpu.pc = 0x28134;
    16
}
/// $09B08E: andi.w #$ffe3, $38(a6)
pub fn i_09b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B094;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B094: ori.w #$0, $38(a6)
pub fn i_09b094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B09A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B09A: jsr $9bb3c(pc)
pub fn i_09b09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B09E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B09E);
    cpu.pc = 0x9BB3C;
    16
}
/// $09B09E: move.w d0, $70(a6)
pub fn i_09b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B0A2: lea.l $9b0a8(pc), a1
pub fn i_09b0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0A8;
    16
}
/// $09B0A6: move.l a1, (a6)
pub fn i_09b0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Item_Food_Wait_09b0a8  ($09B0A8, 48 B, props/items_score_crates_09a0bc.s)
/// $09B0A8: jsr $9b8c0(pc)
pub fn i_09b0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0AC);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B0AC: jsr $28d70.l
pub fn i_09b0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B0B2);
    cpu.pc = 0x28D70;
    16
}
/// $09B0B2: subq.w #$1, $70(a6)
pub fn i_09b0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0B6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B0B6: cmpi.w #$0, $70(a6)
pub fn i_09b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0BC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B0BC: bgt.w $9b0c6
pub fn i_09b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B0C6; return 10; }
    8
}
/// $09B0C0: lea.l $9b0d8(pc), a1
pub fn i_09b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0D8;
    16
}
/// $09B0C4: move.l a1, (a6)
pub fn i_09b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B0C6: jsr $9b920(pc)
pub fn i_09b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0CA);
    cpu.pc = 0x9B920;
    16
}
/// $09B0CA: bcc.w $9b0d4
pub fn i_09b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B0D4; return 10; }
    8
}
/// $09B0CE: lea.l $9b474(pc), a1
pub fn i_09b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B0D2: move.l a1, (a6)
pub fn i_09b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B0D4: bra.w $9a75a
pub fn i_09b0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl293_Pow_09b3c6  ($09B3C6, 120 B, props/items_score_crates_09a0bc.s)
/// $09B3C6: move.w #$1ab, d1
pub fn i_09b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B3CA: jsr $236e.l
pub fn i_09b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3D0);
    cpu.pc = 0x236E;
    16
}
/// $09B3D0: move.l #$9a672, $60(a6)
pub fn i_09b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3D8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x9A672;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B3D8: move.w #$8000, d0
pub fn i_09b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3DC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B3DC: jsr $28134.l
pub fn i_09b3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3E2);
    cpu.pc = 0x28134;
    16
}
/// $09B3E2: andi.w #$ffe3, $38(a6)
pub fn i_09b3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3E8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B3E8: ori.w #$0, $38(a6)
pub fn i_09b3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3EE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B3EE: lea.l $2f772a.l, a0
pub fn i_09b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F772A;
    20
}
/// $09B3F4: jsr $28cd4.l
pub fn i_09b3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3FA);
    cpu.pc = 0x28CD4;
    16
}
/// $09B3FA: lea.l $9b400(pc), a1
pub fn i_09b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B400;
    16
}
/// $09B3FE: move.l a1, (a6)
pub fn i_09b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B400;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B400: jsr $9b8c0(pc)
pub fn i_09b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B404;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B404);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B404: jsr $9b8ee(pc)
pub fn i_09b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B408;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B408);
    cpu.pc = 0x9B8EE;
    16
}
/// $09B408: jsr $28998.l
pub fn i_09b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B40E);
    cpu.pc = 0x28998;
    16
}
/// $09B40E: move.w #$8000, $38(a6)
pub fn i_09b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B414;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B414: jsr $28d70.l
pub fn i_09b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B41A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B41A);
    cpu.pc = 0x28D70;
    16
}
/// $09B41A: jsr $2870a.l
pub fn i_09b41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B420);
    cpu.pc = 0x2870A;
    16
}
/// $09B420: bcc.w $9b42a
pub fn i_09b420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B42A; return 10; }
    8
}
/// $09B424: bclr.b #$3, $13(a6)
pub fn i_09b424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B42A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B42A: jsr $28758.l
pub fn i_09b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B430);
    cpu.pc = 0x28758;
    16
}
/// $09B430: bcc.w $9b43a
pub fn i_09b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B434;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B43A; return 10; }
    8
}
/// $09B434: lea.l $9b43e(pc), a1
pub fn i_09b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B438;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B43E;
    16
}
/// $09B438: move.l a1, (a6)
pub fn i_09b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B43A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B43A: bra.w $9a76a
pub fn i_09b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B43E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Pow_Die_09b43e  ($09B43E, 54 B, props/items_score_crates_09a0bc.s)
/// $09B43E: move.w #$102e, d0
pub fn i_09b43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B442;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B442: jsr $2352.l
pub fn i_09b442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B448);
    cpu.pc = 0x2352;
    16
}
/// $09B448: lea.l $2f773c.l, a0
pub fn i_09b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B44E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F773C;
    20
}
/// $09B44E: jsr $28cd4.l
pub fn i_09b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B454);
    cpu.pc = 0x28CD4;
    16
}
/// $09B454: lea.l $9b45a(pc), a1
pub fn i_09b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B458;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B45A;
    16
}
/// $09B458: move.l a1, (a6)
pub fn i_09b458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B45A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B45A: jsr $2783a.l
pub fn i_09b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B460);
    cpu.pc = 0x2783A;
    16
}
/// $09B460: jsr $28d70.l
pub fn i_09b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B466);
    cpu.pc = 0x28D70;
    16
}
/// $09B466: bcc.w $9b470
pub fn i_09b466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B46A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B470; return 10; }
    8
}
/// $09B46A: lea.l $9b474(pc), a1
pub fn i_09b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B46E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B46E: move.l a1, (a6)
pub fn i_09b46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B470;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B470: bra.w $9a76a
pub fn i_09b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B474;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Free_09b474  ($09B474, 6 B, props/items_score_crates_09a0bc.s)
/// $09B474: jmp $518.l
pub fn i_09b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B47A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_09b47c  ($09B47C, 14 B, kernel/jsr5b6_jmp518.c)
/// $09B47C: jsr $5b6.l
pub fn i_09b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B482);
    cpu.pc = 0x5B6;
    16
}
/// $09B482: jmp $518.l
pub fn i_09b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B488;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Score_Popup_Value_09b51e  ($09B51E, 324 B, props/items_score_crates_09a0bc.s)
/// $09B51E: move.b $68(a6), $6e(a6)
pub fn i_09b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B524;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B524: move.b $98(a6), d1
pub fn i_09b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B528;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B528: lea.l $2f792c.l, a1
pub fn i_09b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B52E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F792C;
    20
}
/// $09B52E: tst.b $9c(a6)
pub fn i_09b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B532;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B532: beq.w $9b53c
pub fn i_09b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B536;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B53C; return 10; }
    8
}
/// $09B536: lea.l $2f794c.l, a1
pub fn i_09b536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B53C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F794C;
    20
}
/// $09B53C: andi.w #$f, d1
pub fn i_09b53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B540;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B540: lsl.w #$2, d1
pub fn i_09b540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B542;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09B542: move.l (a1, d1.w), d0
pub fn i_09b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B546;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $09B546: move.l d0, $80(a6)
pub fn i_09b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B54A;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B54A: jsr $51a44.l
pub fn i_09b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B550);
    cpu.pc = 0x51A44;
    16
}
/// $09B550: move.b $98(a6), d1
pub fn i_09b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B554;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B554: lea.l $2f798c.l, a1
pub fn i_09b554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B55A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F798C;
    20
}
/// $09B55A: tst.b $9c(a6)
pub fn i_09b55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B55E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B55E: beq.w $9b568
pub fn i_09b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B562;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B568; return 10; }
    8
}
/// $09B562: lea.l $2f7994.l, a1
pub fn i_09b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B568;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F7994;
    20
}
/// $09B568: andi.w #$f, d1
pub fn i_09b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B56C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B56C: move.b (a1, d1.w), d1
pub fn i_09b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B570;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09B570: move.b d1, $72(a6)
pub fn i_09b570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B574;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B574: move.b d1, $21(a6)
pub fn i_09b574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B578;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B578: andi.w #$f, d1
pub fn i_09b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B57C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B57C: move.w #$8, d0
pub fn i_09b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B580;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B580: muls.w d1, d0
pub fn i_09b580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B582;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $09B582: add.w d0, $22(a6)
pub fn i_09b582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B586;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B586: jsr $5e9b6.l
pub fn i_09b586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B58C);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B58C: andi.w #$e, d0
pub fn i_09b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B590;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B590: addi.w #$9, d0
pub fn i_09b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B594;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B594: add.w d0, $24(a6)
pub fn i_09b594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B598;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B598: clr.b $3a(a6)
pub fn i_09b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B59C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B59C: lea.l $9b66a(pc), a1
pub fn i_09b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B66A;
    16
}
/// $09B5A0: jsr $4ae.l
pub fn i_09b5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5A6);
    cpu.pc = 0x4AE;
    16
}
/// $09B5A6: jsr $5dd02.l
pub fn i_09b5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5AC);
    cpu.pc = 0x5DD02;
    16
}
/// $09B5AC: move.b $68(a6), $68(a0)
pub fn i_09b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5B2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B5B2: subq.w #$8, $22(a6)
pub fn i_09b5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5B6;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B5B6: move.l $80(a6), d0
pub fn i_09b5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BA;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B5BA: move.l d0, d1
pub fn i_09b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BC;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09B5BC: lsr.l #$4, d1
pub fn i_09b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BE;
    cpu.ir = 0xE889;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 1, false);
    cpu.d[1] = r;
    14
}
/// $09B5BE: move.l d1, $80(a6)
pub fn i_09b5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5C2;
    cpu.ir = 0x2D41;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B5C2: andi.w #$f, d0
pub fn i_09b5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B5C6: move.w d0, $74(a0)
pub fn i_09b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5CA;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5CA: move.b $72(a6), d0
pub fn i_09b5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5CE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B5CE: andi.w #$7, d0
pub fn i_09b5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B5D2: lsl.w #$1, d0
pub fn i_09b5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D4;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09B5D4: move.w d0, $70(a0)
pub fn i_09b5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D8;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5D8: subq.b #$1, $72(a6)
pub fn i_09b5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5DC;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B5DC: cmpi.b #$0, $72(a6)
pub fn i_09b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $09B5E2: bgt.b $9b59c
pub fn i_09b5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5E4;
    cpu.ir = 0x6EB8;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B59C; return 10; }
    8
}
/// $09B5E4: move.w #$1e, $70(a6)
pub fn i_09b5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5EA: move.w #$96, $84(a6)
pub fn i_09b5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5F0: lea.l $9b5f6(pc), a1
pub fn i_09b5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B5F6;
    16
}
/// $09B5F4: move.l a1, (a6)
pub fn i_09b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B5F6: jsr $2783a.l
pub fn i_09b5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5FC);
    cpu.pc = 0x2783A;
    16
}
/// $09B5FC: cmpi.b #$ff, $21(a6)
pub fn i_09b5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B602;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B602: beq.w $9b60e
pub fn i_09b602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B606;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B60E; return 10; }
    8
}
/// $09B606: tst.b $21(a6)
pub fn i_09b606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B60A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B60A: bne.w $9b63a
pub fn i_09b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B60E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B63A; return 10; }
    8
}
/// $09B60E: subq.w #$1, $70(a6)
pub fn i_09b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B612;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B612: cmpi.w #$0, $70(a6)
pub fn i_09b612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B618;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B618: bgt.w $9b63a
pub fn i_09b618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B61C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B63A; return 10; }
    8
}
/// $09B61C: tst.b $21(a6)
pub fn i_09b61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B620;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B620: bne.w $9b634
pub fn i_09b620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B624;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B634; return 10; }
    8
}
/// $09B624: move.b #$ff, $21(a6)
pub fn i_09b624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B62A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B62A: move.w #$a, $70(a6)
pub fn i_09b62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B630;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B630: bra.w $9b63a
pub fn i_09b630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B634;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B63A;
    10
}
/// $09B634: lea.l $9b47c(pc), a1
pub fn i_09b634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B638;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B638: move.l a1, (a6)
pub fn i_09b638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B63A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B63A: subq.w #$1, $84(a6)
pub fn i_09b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B63E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B63E: cmpi.w #$0, $84(a6)
pub fn i_09b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B644;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B644: bgt.w $9b64e
pub fn i_09b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B648;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B64E; return 10; }
    8
}
/// $09B648: lea.l $9b47c(pc), a1
pub fn i_09b648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B64C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B64C: move.l a1, (a6)
pub fn i_09b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B64E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B64E: movea.l #$ffffffff, a0
pub fn i_09b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B654;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09B654: lea.l $9a6c6(pc), a0
pub fn i_09b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B658;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A6C6;
    16
}
/// $09B658: jsr $5dd56.l
pub fn i_09b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B65E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B65E);
    cpu.pc = 0x5DD56;
    16
}
/// $09B65E: bcc.w $9b668
pub fn i_09b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B662;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B668; return 10; }
    8
}
// ===== SetTaskHandler_09b662  ($09B662, 8 B, kernel/task_handlers.c)
/// $09B662: lea.l $9b47c(pc), a1
pub fn i_09b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B666;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B666: move.l a1, (a6)
pub fn i_09b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B668;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B668: rts
pub fn i_09b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B66A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_09b66a  ($09B66A, 232 B, props/items_score_crates_09a0bc.s)
/// $09B66A: move.b $68(a6), d0
pub fn i_09b66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B66E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B66E: cmpi.b #$0, d0
pub fn i_09b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B672;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $09B672: bne.w $9b68e
pub fn i_09b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B676;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B68E; return 10; }
    8
}
/// $09B676: move.w #$1b8, d1
pub fn i_09b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B67A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B67A: jsr $236e.l
pub fn i_09b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B680);
    cpu.pc = 0x236E;
    16
}
/// $09B680: move.w #$1b9, d1
pub fn i_09b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B684;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B684: jsr $236e.l
pub fn i_09b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B68A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B68A);
    cpu.pc = 0x236E;
    16
}
/// $09B68A: bra.w $9b6a2
pub fn i_09b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B68E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B6A2;
    10
}
/// $09B68E: move.w #$1cd, d1
pub fn i_09b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B692;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B692: jsr $236e.l
pub fn i_09b692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B698);
    cpu.pc = 0x236E;
    16
}
/// $09B698: move.w #$1ce, d1
pub fn i_09b698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B69C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B69C: jsr $236e.l
pub fn i_09b69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6A2);
    cpu.pc = 0x236E;
    16
}
/// $09B6A2: move.w #$c000, d0
pub fn i_09b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B6A6: jsr $28134.l
pub fn i_09b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6AC);
    cpu.pc = 0x28134;
    16
}
/// $09B6AC: andi.w #$ffe3, $38(a6)
pub fn i_09b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6B2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B6B2: ori.w #$1c, $38(a6)
pub fn i_09b6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6B8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B6B8: move.w #$0, d0
pub fn i_09b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B6BC: jsr $5dca4.l
pub fn i_09b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6C2);
    cpu.pc = 0x5DCA4;
    16
}
/// $09B6C2: move.w d0, $28(a6)
pub fn i_09b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6C6: move.w #$886, $2a(a6)
pub fn i_09b6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x886;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6CC: move.w #$fedd, $2e(a6)
pub fn i_09b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEDD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6D2: move.w #$0, $2c(a6)
pub fn i_09b6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6D8: move.w $74(a6), d0
pub fn i_09b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6DC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B6DC: andi.w #$1f, d0
pub fn i_09b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B6E0: movea.l #$2f77a4, a0
pub fn i_09b6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F77A4;
    cpu.a[0] = v1;
    4
}
/// $09B6E6: lsl.w #$2, d0
pub fn i_09b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09B6E8: movea.l (a0, d0.w), a0
pub fn i_09b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $09B6EC: cmpa.l #$ffffffff, a0
pub fn i_09b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6F2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09B6F2: beq.w $9b6fc
pub fn i_09b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B6FC; return 10; }
    8
}
/// $09B6F6: jsr $28cd4.l
pub fn i_09b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6FC);
    cpu.pc = 0x28CD4;
    16
}
/// $09B6FC: lea.l $9b702(pc), a1
pub fn i_09b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B700;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B702;
    16
}
/// $09B700: move.l a1, (a6)
pub fn i_09b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B702;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B702: jsr $2783a.l
pub fn i_09b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B708);
    cpu.pc = 0x2783A;
    16
}
/// $09B708: subq.w #$1, $70(a6)
pub fn i_09b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B70C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B70C: cmpi.w #$0, $70(a6)
pub fn i_09b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B712;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B712: bgt.w $9b748
pub fn i_09b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B716;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B748; return 10; }
    8
}
/// $09B716: jsr $27cee.l
pub fn i_09b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B71C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B71C);
    cpu.pc = 0x27CEE;
    16
}
/// $09B71C: movea.l $c(a6), a0
pub fn i_09b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B720;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B720: move.w $24(a0), d0
pub fn i_09b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B724;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B724: cmp.w $24(a6), d0
pub fn i_09b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B728;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $09B728: ble.w $9b73e
pub fn i_09b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B72C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B73E; return 10; }
    8
}
/// $09B72C: addi.w #$10, d0
pub fn i_09b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B730;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B730: move.w d0, $24(a6)
pub fn i_09b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B734;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B734: subq.b #$1, $21(a0)
pub fn i_09b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B738;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B738: lea.l $9b75a(pc), a1
pub fn i_09b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B73C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B75A;
    16
}
/// $09B73C: move.l a1, (a6)
pub fn i_09b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B73E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B73E: jsr $9bb22(pc)
pub fn i_09b73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B742);
    cpu.pc = 0x9BB22;
    16
}
/// $09B742: jsr $28d70.l
pub fn i_09b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B748);
    cpu.pc = 0x28D70;
    16
}
/// $09B748: jsr $5e45a.l
pub fn i_09b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B74E);
    cpu.pc = 0x5E45A;
    16
}
/// $09B74E: bcc.w $9b758
pub fn i_09b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B752;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B758; return 10; }
    8
}
// ===== SetTaskHandler_09b752  ($09B752, 8 B, kernel/task_handlers.c)
/// $09B752: lea.l $9b47c(pc), a1
pub fn i_09b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B756;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B756: move.l a1, (a6)
pub fn i_09b756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B758;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B758: rts
pub fn i_09b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B75A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Lift_09b75a  ($09B75A, 90 B, props/items_score_crates_09a0bc.s)
/// $09B75A: jsr $267e2.l
pub fn i_09b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B760);
    cpu.pc = 0x267E2;
    16
}
/// $09B760: move.w #$400, $2a(a6)
pub fn i_09b760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B766;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B766: move.w #$2, $70(a6)
pub fn i_09b766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B76C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B76C: lea.l $9b772(pc), a1
pub fn i_09b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B770;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B772;
    16
}
/// $09B770: move.l a1, (a6)
pub fn i_09b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B772;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B772: jsr $27cee.l
pub fn i_09b772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B778);
    cpu.pc = 0x27CEE;
    16
}
/// $09B778: jsr $9bb22(pc)
pub fn i_09b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B77C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B77C);
    cpu.pc = 0x9BB22;
    16
}
/// $09B77C: jsr $28d70.l
pub fn i_09b77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B782);
    cpu.pc = 0x28D70;
    16
}
/// $09B782: subq.w #$1, $70(a6)
pub fn i_09b782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B786;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B786: cmpi.w #$0, $70(a6)
pub fn i_09b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B78C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B78C: bgt.w $9b796
pub fn i_09b78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B790;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B796; return 10; }
    8
}
/// $09B790: lea.l $9b7bc(pc), a1
pub fn i_09b790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B794;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7BC;
    16
}
/// $09B794: move.l a1, (a6)
pub fn i_09b794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B796;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B796: movea.l $c(a6), a0
pub fn i_09b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B79A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B79A: cmpi.b #$ff, $21(a0)
pub fn i_09b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B7A0: bne.w $9b7aa
pub fn i_09b7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B7AA; return 10; }
    8
}
/// $09B7A4: lea.l $9b7f8(pc), a1
pub fn i_09b7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7F8;
    16
}
/// $09B7A8: move.l a1, (a6)
pub fn i_09b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7AA: jsr $5e45a.l
pub fn i_09b7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7B0);
    cpu.pc = 0x5E45A;
    16
}
/// $09B7B0: bcc.w $9b7ba
pub fn i_09b7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B7BA; return 10; }
    8
}
// ===== SetTaskHandler_09b7b4  ($09B7B4, 8 B, kernel/task_handlers.c)
/// $09B7B4: lea.l $9b47c(pc), a1
pub fn i_09b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B7B8: move.l a1, (a6)
pub fn i_09b7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7BA: rts
pub fn i_09b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Hold_09b7bc  ($09B7BC, 52 B, props/items_score_crates_09a0bc.s)
/// $09B7BC: lea.l $9b7c2(pc), a1
pub fn i_09b7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7C2;
    16
}
/// $09B7C0: move.l a1, (a6)
pub fn i_09b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7C2: jsr $2783a.l
pub fn i_09b7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7C8);
    cpu.pc = 0x2783A;
    16
}
/// $09B7C8: jsr $9bb22(pc)
pub fn i_09b7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B7CC);
    cpu.pc = 0x9BB22;
    16
}
/// $09B7CC: jsr $28d70.l
pub fn i_09b7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7D2);
    cpu.pc = 0x28D70;
    16
}
/// $09B7D2: movea.l $c(a6), a0
pub fn i_09b7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7D6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B7D6: cmpi.b #$ff, $21(a0)
pub fn i_09b7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7DC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B7DC: bne.w $9b7e6
pub fn i_09b7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B7E6; return 10; }
    8
}
/// $09B7E0: lea.l $9b7f8(pc), a1
pub fn i_09b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7F8;
    16
}
/// $09B7E4: move.l a1, (a6)
pub fn i_09b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7E6: jsr $5e45a.l
pub fn i_09b7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7EC);
    cpu.pc = 0x5E45A;
    16
}
/// $09B7EC: bcc.w $9b7f6
pub fn i_09b7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B7F6; return 10; }
    8
}
// ===== SetTaskHandler_09b7f0  ($09B7F0, 8 B, kernel/task_handlers.c)
/// $09B7F0: lea.l $9b47c(pc), a1
pub fn i_09b7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B7F4: move.l a1, (a6)
pub fn i_09b7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7F6: rts
pub fn i_09b7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Blink_09b7f8  ($09B7F8, 44 B, props/items_score_crates_09a0bc.s)
/// $09B7F8: lea.l $9b7fe(pc), a1
pub fn i_09b7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7FE;
    16
}
/// $09B7FC: move.l a1, (a6)
pub fn i_09b7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7FE: jsr $2783a.l
pub fn i_09b7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B804);
    cpu.pc = 0x2783A;
    16
}
/// $09B804: btst.b #$0, $106f28.l
pub fn i_09b804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B80C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $09B80C: beq.w $9b81a
pub fn i_09b80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B810;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B81A; return 10; }
    8
}
/// $09B810: jsr $9bb22(pc)
pub fn i_09b810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B814);
    cpu.pc = 0x9BB22;
    16
}
/// $09B814: jsr $28d70.l
pub fn i_09b814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B81A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B81A);
    cpu.pc = 0x28D70;
    16
}
/// $09B81A: jsr $5e45a.l
pub fn i_09b81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B820);
    cpu.pc = 0x5E45A;
    16
}
/// $09B820: bcc.w $9b82a
pub fn i_09b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B824;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B82A; return 10; }
    8
}
// ===== SetTaskHandler_09b824  ($09B824, 8 B, kernel/task_handlers.c)
/// $09B824: lea.l $9b47c(pc), a1
pub fn i_09b824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B828;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B828: move.l a1, (a6)
pub fn i_09b828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B82A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B82A: rts
pub fn i_09b82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B82C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Tmpl316_ComboTimer_09b82c  ($09B82C, 70 B, props/items_score_crates_09a0bc.s)
/// $09B82C: clr.b $10e488.l
pub fn i_09b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B832;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E488) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E488, r2 as u8);
    16
}
/// $09B832: clr.b $10e489.l
pub fn i_09b832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B838;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E489) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E489, r2 as u8);
    16
}
/// $09B838: lea.l $9b83e(pc), a1
pub fn i_09b838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B83C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B83E;
    16
}
/// $09B83C: move.l a1, (a6)
pub fn i_09b83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B83E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B83E: subq.b #$1, $10e488.l
pub fn i_09b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B844;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10E488) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E488, r2 as u8);
    16
}
/// $09B844: cmpi.b #$0, $10e488.l
pub fn i_09b844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B84C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E488) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $09B84C: bgt.w $9b85c
pub fn i_09b84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B850;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B85C; return 10; }
    8
}
/// $09B850: clr.b $10e488.l
pub fn i_09b850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B856;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E488) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E488, r2 as u8);
    16
}
/// $09B856: clr.b $10e489.l
pub fn i_09b856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B85C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E489) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E489, r2 as u8);
    16
}
/// $09B85C: cmpi.b #$e, $10e489.l
pub fn i_09b85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B864;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E489) as u32;
    sub_with_flags(cpu, v2, 0xE, Size::Byte, false);
    20
}
/// $09B864: ble.w $9b870
pub fn i_09b864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B868;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B870; return 10; }
    8
}
/// $09B868: move.b #$e, $10e489.l
pub fn i_09b868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B870;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E489, v1 as u8);
    16
}
/// $09B870: rts
pub fn i_09b870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B872;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SpawnAtOffset_09b872  ($09B872, 76 B, props/items_score_crates_09a0bc.s)
/// $09B872: jsr $4cac4.l
pub fn i_09b872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B878);
    cpu.pc = 0x4CAC4;
    16
}
/// $09B878: tst.w $98(a6)
pub fn i_09b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B87C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $09B87C: bne.w $9b886
pub fn i_09b87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B880;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B886; return 10; }
    8
}
/// $09B880: add.w d3, d0
pub fn i_09b880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B882;
    cpu.ir = 0xD043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B882: bra.w $9b888
pub fn i_09b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B886;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B888;
    10
}
/// $09B886: sub.w d3, d0
pub fn i_09b886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B888;
    cpu.ir = 0x9043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B888: addq.w #$8, d1
pub fn i_09b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B88A;
    cpu.ir = 0x5041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09B88A: move.w d0, $22(a6)
pub fn i_09b88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B88E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B88E: move.w d1, $24(a6)
pub fn i_09b88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B892;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B892: move.b d4, $98(a6)
pub fn i_09b892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B896;
    cpu.ir = 0x1D44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B896: move.b d5, $99(a6)
pub fn i_09b896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B89A;
    cpu.ir = 0x1D45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B89A: move.b d6, $9a(a6)
pub fn i_09b89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B89E;
    cpu.ir = 0x1D46;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B89E: move.b $98(a6), d0
pub fn i_09b89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8A2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B8A2: move.w #$99, d1
pub fn i_09b8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B8A6: jsr $9a7aa(pc)
pub fn i_09b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B8AA);
    cpu.pc = 0x9A7AA;
    16
}
/// $09B8AA: bcs.w $9b8b8
pub fn i_09b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8AE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9B8B8; return 10; }
    8
}
/// $09B8AE: move.l (a0), $5c(a0)
pub fn i_09b8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8B2;
    cpu.ir = 0x2150;
    let v2 = bus.read32(cpu.a[0]);
    let ad3 = cpu.a[0].wrapping_add(0x5C);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $09B8B2: move.l #$9a8be, (a0)
pub fn i_09b8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8B8;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x9A8BE;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B8B8: jmp $518.l
pub fn i_09b8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8BE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Item_PhysAndProbe_09b8c0  ($09B8C0, 40 B, props/items_score_crates_09a0bc.s)
/// $09B8C0: jsr $2783a.l
pub fn i_09b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B8C6);
    cpu.pc = 0x2783A;
    16
}
/// $09B8C6: tst.b $9b(a6)
pub fn i_09b8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8CA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B8CA: beq.w $9b8d8
pub fn i_09b8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B8D8; return 10; }
    8
}
/// $09B8CE: cmpi.b #$ff, $9b(a6)
pub fn i_09b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B8D4: bne.w $9b8e8
pub fn i_09b8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B8E8; return 10; }
    8
}
/// $09B8D8: jsr $27eba.l
pub fn i_09b8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B8DE);
    cpu.pc = 0x27EBA;
    16
}
/// $09B8DE: bcc.w $9b8e8
pub fn i_09b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B8E8; return 10; }
    8
}
/// $09B8E2: jsr $27c8c.l
pub fn i_09b8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B8E8);
    cpu.pc = 0x27C8C;
    16
}
// ===== ClearXN_09b8e8  ($09B8E8, 6 B, thunks/ccr_helpers.c)
/// $09B8E8: andi.b #$ee, ccr
pub fn i_09b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09B8EC: rts
pub fn i_09b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_TrailRecord_09b8ee  ($09B8EE, 20 B, props/items_score_crates_09a0bc.s)
/// $09B8EE: move.w $22(a6), d0
pub fn i_09b8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8F2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B8F2: move.w $24(a6), d1
pub fn i_09b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8F6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B8F6: subi.w #$10, d0
pub fn i_09b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8FA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B8FA: addi.w #$10, d1
pub fn i_09b8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8FE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B8FE: move.w #$20, d2
pub fn i_09b8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B902;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_09b902  ($09B902, 8 B, thunks/jsr_abs_thunks.c)
/// $09B902: jsr $99812.l
pub fn i_09b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B908;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B908);
    cpu.pc = 0x99812;
    16
}
/// $09B908: rts
pub fn i_09b908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B90A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_RespawnBelow_09b90a  ($09B90A, 22 B, props/items_score_crates_09a0bc.s)
/// $09B90A: move.b $98(a6), d0
pub fn i_09b90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B90E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B90E: move.w #$99, d1
pub fn i_09b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B912;
    cpu.ir = 0x323C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B912: jsr $9a7aa(pc)
pub fn i_09b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B916);
    cpu.pc = 0x9A7AA;
    16
}
/// $09B916: bcs.w $9b91e
pub fn i_09b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B91A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9B91E; return 10; }
    8
}
/// $09B91A: addq.w #$4, $24(a0)
pub fn i_09b91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B91E;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B91E: rts
pub fn i_09b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B920;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Food_Pickup_09b920  ($09B920, 142 B, props/items_score_crates_09a0bc.s)
/// $09B920: jsr $9a84e(pc)
pub fn i_09b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B924;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B924);
    cpu.pc = 0x9A84E;
    16
}
/// $09B924: bcc.w $9b9b4
pub fn i_09b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B928;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B9B4; return 10; }
    8
}
/// $09B928: move.b d0, $68(a6)
pub fn i_09b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B92C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B92C: move.w #$1121, d0
pub fn i_09b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B930;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1121;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B930: jsr $2352.l
pub fn i_09b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B936);
    cpu.pc = 0x2352;
    16
}
/// $09B936: lea.l $9b51e(pc), a1
pub fn i_09b936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B93A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B51E;
    16
}
/// $09B93A: jsr $4ae.l
pub fn i_09b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B940);
    cpu.pc = 0x4AE;
    16
}
/// $09B940: jsr $5dd02.l
pub fn i_09b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B946);
    cpu.pc = 0x5DD02;
    16
}
/// $09B946: move.b $68(a6), $68(a0)
pub fn i_09b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B94C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B94C: move.b $98(a6), d1
pub fn i_09b94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B950;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B950: move.b $9c(a6), d0
pub fn i_09b950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B954;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B954: move.b d0, $9c(a0)
pub fn i_09b954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B958;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B958: tst.b d0
pub fn i_09b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B95A;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09B95A: beq.w $9b99e
pub fn i_09b95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B95E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B99E; return 10; }
    8
}
/// $09B95E: cmpi.b #$2, d0
pub fn i_09b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B962;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $09B962: bne.w $9b97e
pub fn i_09b962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B966;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B97E; return 10; }
    8
}
/// $09B966: move.b #$f, d1
pub fn i_09b966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B96A;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09B96A: move.b #$e, $10e489.l
pub fn i_09b96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B972;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E489, v1 as u8);
    16
}
/// $09B972: move.b #$2d, $10e488.l
pub fn i_09b972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B97A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E488, v1 as u8);
    16
}
/// $09B97A: bra.w $9b9aa
pub fn i_09b97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B97E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B9AA;
    10
}
/// $09B97E: move.b $10e489.l, d2
pub fn i_09b97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B984;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x10E489) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $09B984: cmp.b d1, d2
pub fn i_09b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B986;
    cpu.ir = 0xB401;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $09B986: ble.w $9b98c
pub fn i_09b986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B98A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B98C; return 10; }
    8
}
/// $09B98A: move.b d2, d1
pub fn i_09b98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B98C;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09B98C: move.w d1, d2
pub fn i_09b98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B98E;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B98E: addq.b #$1, d2
pub fn i_09b98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B990;
    cpu.ir = 0x5202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09B990: move.b d2, $10e489.l
pub fn i_09b990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B996;
    cpu.ir = 0x13C2;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E489, v1 as u8);
    16
}
/// $09B996: move.b #$2d, $10e488.l
pub fn i_09b996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B99E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E488, v1 as u8);
    16
}
/// $09B99E: cmpi.b #$e, d1
pub fn i_09b99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9A2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $09B9A2: ble.w $9b9aa
pub fn i_09b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9A6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B9AA; return 10; }
    8
}
/// $09B9A6: move.b #$e, d1
pub fn i_09b9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9AA;
    cpu.ir = 0x123C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09B9AA: move.b d1, $98(a0)
pub fn i_09b9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9AE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_09b9ae  ($09B9AE, 6 B, thunks/ccr_helpers.c)
/// $09B9AE: ori.b #$11, ccr
pub fn i_09b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9B2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09B9B2: rts
pub fn i_09b9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_09b9b4  ($09B9B4, 6 B, thunks/ccr_helpers.c)
/// $09B9B4: andi.b #$ee, ccr
pub fn i_09b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9B8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09B9B8: rts
pub fn i_09b9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SelectAnimParity_09bb22  ($09BB22, 20 B, props/items_score_crates_09a0bc.s)
/// $09BB22: move.w $16(a6), d0
pub fn i_09bb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB26;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09BB26: btst.b #$0, $106f28.l
pub fn i_09bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB2E;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $09BB2E: beq.w $9bb36
pub fn i_09bb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BB36; return 10; }
    8
}
/// $09BB32: move.w $18(a6), d0
pub fn i_09bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB36;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== SetTaskW_09bb36  ($09BB36, 6 B, kernel/task_setters_w.c)
/// $09BB36: move.w d0, $14(a6)
pub fn i_09bb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB3A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BB3A: rts
pub fn i_09bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_LifetimeFromParam_09bb3c  ($09BB3C, 26 B, props/items_score_crates_09a0bc.s)
/// $09BB3C: move.b $99(a6), d0
pub fn i_09bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BB40: move.b $99(a6), d0
pub fn i_09bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB44;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BB44: andi.w #$ff, d0
pub fn i_09bb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB48;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09BB48: lsl.w #$4, d0
pub fn i_09bb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB4A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $09BB4A: tst.w d0
pub fn i_09bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB4C;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $09BB4C: bne.w $9bb54
pub fn i_09bb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9BB54; return 10; }
    8
}
/// $09BB50: move.w #$7fff, d0
pub fn i_09bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB54;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BB54: rts
pub fn i_09bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x097A6E, i_097a6e),
    (0x097A80, i_097a80),
    (0x097A84, i_097a84),
    (0x097A88, i_097a88),
    (0x097A8C, i_097a8c),
    (0x097A90, i_097a90),
    (0x097A96, i_097a96),
    (0x097A9A, i_097a9a),
    (0x097A9E, i_097a9e),
    (0x097AA2, i_097aa2),
    (0x097AA4, i_097aa4),
    (0x097AA8, i_097aa8),
    (0x097AAA, i_097aaa),
    (0x097AAE, i_097aae),
    (0x097AB4, i_097ab4),
    (0x097AB6, i_097ab6),
    (0x097AB8, i_097ab8),
    (0x097ABC, i_097abc),
    (0x097ABE, i_097abe),
    (0x097AC4, i_097ac4),
    (0x097AC8, i_097ac8),
    (0x097ACA, i_097aca),
    (0x097AD0, i_097ad0),
    (0x097AD4, i_097ad4),
    (0x097AD8, i_097ad8),
    (0x097ADC, i_097adc),
    (0x097ADE, i_097ade),
    (0x097AE2, i_097ae2),
    (0x097AE6, i_097ae6),
    (0x097AE8, i_097ae8),
    (0x097AEA, i_097aea),
    (0x097AEE, i_097aee),
    (0x097AF0, i_097af0),
    (0x097AF4, i_097af4),
    (0x097AF8, i_097af8),
    (0x097AFC, i_097afc),
    (0x097B02, i_097b02),
    (0x097B06, i_097b06),
    (0x097B0A, i_097b0a),
    (0x097B0E, i_097b0e),
    (0x097B10, i_097b10),
    (0x097B14, i_097b14),
    (0x097B1A, i_097b1a),
    (0x097B20, i_097b20),
    (0x097B26, i_097b26),
    (0x097B2C, i_097b2c),
    (0x097B30, i_097b30),
    (0x097B34, i_097b34),
    (0x097B36, i_097b36),
    (0x097B38, i_097b38),
    (0x097B3C, i_097b3c),
    (0x097B40, i_097b40),
    (0x097B44, i_097b44),
    (0x097B46, i_097b46),
    (0x097B4A, i_097b4a),
    (0x097B4E, i_097b4e),
    (0x097B54, i_097b54),
    (0x097B5A, i_097b5a),
    (0x097B60, i_097b60),
    (0x097B64, i_097b64),
    (0x097B68, i_097b68),
    (0x097B6A, i_097b6a),
    (0x097B6E, i_097b6e),
    (0x097B72, i_097b72),
    (0x098176, i_098176),
    (0x09817C, i_09817c),
    (0x09817E, i_09817e),
    (0x098180, i_098180),
    (0x098184, i_098184),
    (0x09818A, i_09818a),
    (0x09818C, i_09818c),
    (0x098190, i_098190),
    (0x098192, i_098192),
    (0x098198, i_098198),
    (0x09819C, i_09819c),
    (0x09819E, i_09819e),
    (0x0981A2, i_0981a2),
    (0x0981A4, i_0981a4),
    (0x0981A8, i_0981a8),
    (0x0981AC, i_0981ac),
    (0x0981FC, i_0981fc),
    (0x098202, i_098202),
    (0x098208, i_098208),
    (0x09820A, i_09820a),
    (0x09820C, i_09820c),
    (0x09820E, i_09820e),
    (0x098210, i_098210),
    (0x098214, i_098214),
    (0x098218, i_098218),
    (0x09821C, i_09821c),
    (0x098220, i_098220),
    (0x098222, i_098222),
    (0x098228, i_098228),
    (0x09822A, i_09822a),
    (0x09822E, i_09822e),
    (0x098230, i_098230),
    (0x098232, i_098232),
    (0x098234, i_098234),
    (0x09823A, i_09823a),
    (0x09823C, i_09823c),
    (0x098240, i_098240),
    (0x098244, i_098244),
    (0x09824A, i_09824a),
    (0x098250, i_098250),
    (0x098252, i_098252),
    (0x098256, i_098256),
    (0x09825A, i_09825a),
    (0x098260, i_098260),
    (0x098264, i_098264),
    (0x098268, i_098268),
    (0x09826A, i_09826a),
    (0x09826C, i_09826c),
    (0x09826E, i_09826e),
    (0x098274, i_098274),
    (0x098278, i_098278),
    (0x09827C, i_09827c),
    (0x098280, i_098280),
    (0x098282, i_098282),
    (0x098286, i_098286),
    (0x098288, i_098288),
    (0x09828C, i_09828c),
    (0x098290, i_098290),
    (0x098294, i_098294),
    (0x09829A, i_09829a),
    (0x09829E, i_09829e),
    (0x0982A0, i_0982a0),
    (0x0982A6, i_0982a6),
    (0x0982AA, i_0982aa),
    (0x0982AE, i_0982ae),
    (0x0982B2, i_0982b2),
    (0x0982B8, i_0982b8),
    (0x0982BE, i_0982be),
    (0x0982C2, i_0982c2),
    (0x0982C6, i_0982c6),
    (0x0982CC, i_0982cc),
    (0x0982D0, i_0982d0),
    (0x0982D4, i_0982d4),
    (0x0982DA, i_0982da),
    (0x0982DE, i_0982de),
    (0x0982E2, i_0982e2),
    (0x0982E8, i_0982e8),
    (0x0982EC, i_0982ec),
    (0x0982F0, i_0982f0),
    (0x0982F6, i_0982f6),
    (0x0982FA, i_0982fa),
    (0x098300, i_098300),
    (0x098304, i_098304),
    (0x098306, i_098306),
    (0x0983B0, i_0983b0),
    (0x0983B6, i_0983b6),
    (0x0983BA, i_0983ba),
    (0x0983BE, i_0983be),
    (0x0983C2, i_0983c2),
    (0x0983C6, i_0983c6),
    (0x0983CA, i_0983ca),
    (0x0983CE, i_0983ce),
    (0x0983D2, i_0983d2),
    (0x0983D6, i_0983d6),
    (0x0983FE, i_0983fe),
    (0x098404, i_098404),
    (0x09840C, i_09840c),
    (0x098412, i_098412),
    (0x098416, i_098416),
    (0x09841A, i_09841a),
    (0x09841E, i_09841e),
    (0x098424, i_098424),
    (0x098428, i_098428),
    (0x09842C, i_09842c),
    (0x098430, i_098430),
    (0x098436, i_098436),
    (0x09843C, i_09843c),
    (0x098440, i_098440),
    (0x098444, i_098444),
    (0x09844A, i_09844a),
    (0x09844E, i_09844e),
    (0x098452, i_098452),
    (0x098458, i_098458),
    (0x09845C, i_09845c),
    (0x098460, i_098460),
    (0x098466, i_098466),
    (0x09846A, i_09846a),
    (0x09846E, i_09846e),
    (0x098474, i_098474),
    (0x09847A, i_09847a),
    (0x09847E, i_09847e),
    (0x098480, i_098480),
    (0x098482, i_098482),
    (0x098486, i_098486),
    (0x09848A, i_09848a),
    (0x09848C, i_09848c),
    (0x098490, i_098490),
    (0x098496, i_098496),
    (0x09849A, i_09849a),
    (0x09849E, i_09849e),
    (0x0984A4, i_0984a4),
    (0x0984A8, i_0984a8),
    (0x0984AC, i_0984ac),
    (0x0984B0, i_0984b0),
    (0x0984B2, i_0984b2),
    (0x0984B6, i_0984b6),
    (0x0984BC, i_0984bc),
    (0x0984C0, i_0984c0),
    (0x0984C4, i_0984c4),
    (0x0984CA, i_0984ca),
    (0x0984CE, i_0984ce),
    (0x0984D2, i_0984d2),
    (0x0984D6, i_0984d6),
    (0x0984DA, i_0984da),
    (0x0984DE, i_0984de),
    (0x0984E2, i_0984e2),
    (0x0984E6, i_0984e6),
    (0x0984EC, i_0984ec),
    (0x0984F0, i_0984f0),
    (0x0984F4, i_0984f4),
    (0x0984FA, i_0984fa),
    (0x0984FC, i_0984fc),
    (0x098500, i_098500),
    (0x098506, i_098506),
    (0x09850A, i_09850a),
    (0x09850E, i_09850e),
    (0x098514, i_098514),
    (0x098518, i_098518),
    (0x09851C, i_09851c),
    (0x098524, i_098524),
    (0x09852C, i_09852c),
    (0x098534, i_098534),
    (0x09853C, i_09853c),
    (0x098540, i_098540),
    (0x098544, i_098544),
    (0x098548, i_098548),
    (0x09854E, i_09854e),
    (0x098552, i_098552),
    (0x098556, i_098556),
    (0x09855C, i_09855c),
    (0x098560, i_098560),
    (0x098562, i_098562),
    (0x098566, i_098566),
    (0x09856A, i_09856a),
    (0x098570, i_098570),
    (0x098576, i_098576),
    (0x098578, i_098578),
    (0x09857C, i_09857c),
    (0x098584, i_098584),
    (0x09858C, i_09858c),
    (0x098594, i_098594),
    (0x09859E, i_09859e),
    (0x0985A6, i_0985a6),
    (0x0985AC, i_0985ac),
    (0x0985B2, i_0985b2),
    (0x0985B6, i_0985b6),
    (0x0985BA, i_0985ba),
    (0x0985BE, i_0985be),
    (0x0985C0, i_0985c0),
    (0x0985C4, i_0985c4),
    (0x0985C8, i_0985c8),
    (0x0985CA, i_0985ca),
    (0x0985CE, i_0985ce),
    (0x0985D6, i_0985d6),
    (0x0985DE, i_0985de),
    (0x0985E6, i_0985e6),
    (0x0985EC, i_0985ec),
    (0x0985F0, i_0985f0),
    (0x0985F2, i_0985f2),
    (0x0985F4, i_0985f4),
    (0x0985F6, i_0985f6),
    (0x0985F8, i_0985f8),
    (0x0985FA, i_0985fa),
    (0x098604, i_098604),
    (0x09860C, i_09860c),
    (0x098612, i_098612),
    (0x098618, i_098618),
    (0x09861C, i_09861c),
    (0x098622, i_098622),
    (0x098626, i_098626),
    (0x09862A, i_09862a),
    (0x09862E, i_09862e),
    (0x098634, i_098634),
    (0x09863A, i_09863a),
    (0x098640, i_098640),
    (0x098646, i_098646),
    (0x09864C, i_09864c),
    (0x098652, i_098652),
    (0x098656, i_098656),
    (0x09865A, i_09865a),
    (0x09865E, i_09865e),
    (0x098660, i_098660),
    (0x098664, i_098664),
    (0x098668, i_098668),
    (0x09866A, i_09866a),
    (0x09866E, i_09866e),
    (0x098676, i_098676),
    (0x09867E, i_09867e),
    (0x098686, i_098686),
    (0x09868C, i_09868c),
    (0x098690, i_098690),
    (0x0997B8, i_0997b8),
    (0x0997BE, i_0997be),
    (0x0997C4, i_0997c4),
    (0x0997CA, i_0997ca),
    (0x0997CC, i_0997cc),
    (0x0997D6, i_0997d6),
    (0x0997E0, i_0997e0),
    (0x099812, i_099812),
    (0x099818, i_099818),
    (0x09981C, i_09981c),
    (0x09981E, i_09981e),
    (0x099822, i_099822),
    (0x099826, i_099826),
    (0x09982C, i_09982c),
    (0x09982E, i_09982e),
    (0x099832, i_099832),
    (0x099836, i_099836),
    (0x09983A, i_09983a),
    (0x099840, i_099840),
    (0x099842, i_099842),
    (0x099844, i_099844),
    (0x099848, i_099848),
    (0x09984C, i_09984c),
    (0x09984E, i_09984e),
    (0x099852, i_099852),
    (0x099854, i_099854),
    (0x099856, i_099856),
    (0x09985A, i_09985a),
    (0x099860, i_099860),
    (0x099866, i_099866),
    (0x09986A, i_09986a),
    (0x09986C, i_09986c),
    (0x09986E, i_09986e),
    (0x099872, i_099872),
    (0x099876, i_099876),
    (0x09987A, i_09987a),
    (0x09987E, i_09987e),
    (0x099882, i_099882),
    (0x099886, i_099886),
    (0x099888, i_099888),
    (0x09988A, i_09988a),
    (0x09988C, i_09988c),
    (0x099890, i_099890),
    (0x099892, i_099892),
    (0x099896, i_099896),
    (0x09989C, i_09989c),
    (0x0998A0, i_0998a0),
    (0x0998A4, i_0998a4),
    (0x0998A8, i_0998a8),
    (0x0998AC, i_0998ac),
    (0x0998B0, i_0998b0),
    (0x0998B4, i_0998b4),
    (0x0998B8, i_0998b8),
    (0x0998BC, i_0998bc),
    (0x0998C0, i_0998c0),
    (0x0998C2, i_0998c2),
    (0x0998C8, i_0998c8),
    (0x0998CA, i_0998ca),
    (0x0998CE, i_0998ce),
    (0x0998D4, i_0998d4),
    (0x0998DA, i_0998da),
    (0x0998E0, i_0998e0),
    (0x0998E4, i_0998e4),
    (0x0998E8, i_0998e8),
    (0x0998EC, i_0998ec),
    (0x0998EE, i_0998ee),
    (0x0998F2, i_0998f2),
    (0x0998F6, i_0998f6),
    (0x0998FA, i_0998fa),
    (0x0998FE, i_0998fe),
    (0x099902, i_099902),
    (0x099904, i_099904),
    (0x099908, i_099908),
    (0x09990C, i_09990c),
    (0x099910, i_099910),
    (0x099914, i_099914),
    (0x099916, i_099916),
    (0x09991A, i_09991a),
    (0x09991E, i_09991e),
    (0x099924, i_099924),
    (0x099928, i_099928),
    (0x09992C, i_09992c),
    (0x099930, i_099930),
    (0x099934, i_099934),
    (0x099938, i_099938),
    (0x09993A, i_09993a),
    (0x09993C, i_09993c),
    (0x099940, i_099940),
    (0x099946, i_099946),
    (0x09994C, i_09994c),
    (0x099952, i_099952),
    (0x099956, i_099956),
    (0x09995A, i_09995a),
    (0x09995E, i_09995e),
    (0x099962, i_099962),
    (0x099966, i_099966),
    (0x09996C, i_09996c),
    (0x099970, i_099970),
    (0x099974, i_099974),
    (0x099976, i_099976),
    (0x09997A, i_09997a),
    (0x09997C, i_09997c),
    (0x099980, i_099980),
    (0x099984, i_099984),
    (0x099986, i_099986),
    (0x09998A, i_09998a),
    (0x09998C, i_09998c),
    (0x099990, i_099990),
    (0x099994, i_099994),
    (0x099998, i_099998),
    (0x09999C, i_09999c),
    (0x09999E, i_09999e),
    (0x0999A2, i_0999a2),
    (0x0999A6, i_0999a6),
    (0x0999AA, i_0999aa),
    (0x0999AC, i_0999ac),
    (0x0999B0, i_0999b0),
    (0x0999B4, i_0999b4),
    (0x0999B8, i_0999b8),
    (0x0999BA, i_0999ba),
    (0x0999BE, i_0999be),
    (0x0999C2, i_0999c2),
    (0x0999C6, i_0999c6),
    (0x0999C8, i_0999c8),
    (0x0999CA, i_0999ca),
    (0x0999CE, i_0999ce),
    (0x0999D2, i_0999d2),
    (0x0999D6, i_0999d6),
    (0x0999D8, i_0999d8),
    (0x0999DC, i_0999dc),
    (0x0999DE, i_0999de),
    (0x0999E2, i_0999e2),
    (0x0999E4, i_0999e4),
    (0x0999E8, i_0999e8),
    (0x0999EC, i_0999ec),
    (0x0999F0, i_0999f0),
    (0x0999F4, i_0999f4),
    (0x0999F6, i_0999f6),
    (0x0999F8, i_0999f8),
    (0x0999FA, i_0999fa),
    (0x0999FE, i_0999fe),
    (0x099A00, i_099a00),
    (0x099A06, i_099a06),
    (0x099A0C, i_099a0c),
    (0x099A12, i_099a12),
    (0x099A16, i_099a16),
    (0x099A1C, i_099a1c),
    (0x099A20, i_099a20),
    (0x099A22, i_099a22),
    (0x099A26, i_099a26),
    (0x099A2A, i_099a2a),
    (0x099A2E, i_099a2e),
    (0x099A32, i_099a32),
    (0x099A36, i_099a36),
    (0x099A38, i_099a38),
    (0x099A3A, i_099a3a),
    (0x099A3C, i_099a3c),
    (0x099A40, i_099a40),
    (0x099A44, i_099a44),
    (0x099A48, i_099a48),
    (0x099AE2, i_099ae2),
    (0x099AEA, i_099aea),
    (0x099AF2, i_099af2),
    (0x099AFA, i_099afa),
    (0x099AFC, i_099afc),
    (0x099B04, i_099b04),
    (0x09A75A, i_09a75a),
    (0x09A760, i_09a760),
    (0x09A764, i_09a764),
    (0x09A76A, i_09a76a),
    (0x09A76E, i_09a76e),
    (0x09A772, i_09a772),
    (0x09A778, i_09a778),
    (0x09A77C, i_09a77c),
    (0x09A782, i_09a782),
    (0x09A786, i_09a786),
    (0x09A78C, i_09a78c),
    (0x09A78E, i_09a78e),
    (0x09A794, i_09a794),
    (0x09A798, i_09a798),
    (0x09A79E, i_09a79e),
    (0x09A7A2, i_09a7a2),
    (0x09A7A8, i_09a7a8),
    (0x09A7AA, i_09a7aa),
    (0x09A7AE, i_09a7ae),
    (0x09A7B2, i_09a7b2),
    (0x09A7B6, i_09a7b6),
    (0x09A7BC, i_09a7bc),
    (0x09A7C0, i_09a7c0),
    (0x09A7C2, i_09a7c2),
    (0x09A7C6, i_09a7c6),
    (0x09A7CA, i_09a7ca),
    (0x09A7CC, i_09a7cc),
    (0x09A7CE, i_09a7ce),
    (0x09A7D2, i_09a7d2),
    (0x09A7D4, i_09a7d4),
    (0x09A7D8, i_09a7d8),
    (0x09A7DC, i_09a7dc),
    (0x09A7E0, i_09a7e0),
    (0x09A7E4, i_09a7e4),
    (0x09A7E6, i_09a7e6),
    (0x09A7E8, i_09a7e8),
    (0x09A7EC, i_09a7ec),
    (0x09A7F0, i_09a7f0),
    (0x09A7F2, i_09a7f2),
    (0x09A7F8, i_09a7f8),
    (0x09A7FE, i_09a7fe),
    (0x09A800, i_09a800),
    (0x09A806, i_09a806),
    (0x09A80A, i_09a80a),
    (0x09A810, i_09a810),
    (0x09A814, i_09a814),
    (0x09A816, i_09a816),
    (0x09A81A, i_09a81a),
    (0x09A81E, i_09a81e),
    (0x09A820, i_09a820),
    (0x09A826, i_09a826),
    (0x09A82C, i_09a82c),
    (0x09A830, i_09a830),
    (0x09A834, i_09a834),
    (0x09A836, i_09a836),
    (0x09A83A, i_09a83a),
    (0x09A83E, i_09a83e),
    (0x09A842, i_09a842),
    (0x09A846, i_09a846),
    (0x09A848, i_09a848),
    (0x09A84E, i_09a84e),
    (0x09A852, i_09a852),
    (0x09A854, i_09a854),
    (0x09A858, i_09a858),
    (0x09A85E, i_09a85e),
    (0x09A864, i_09a864),
    (0x09A868, i_09a868),
    (0x09A86A, i_09a86a),
    (0x09A86E, i_09a86e),
    (0x09A874, i_09a874),
    (0x09A876, i_09a876),
    (0x09A87C, i_09a87c),
    (0x09A8BE, i_09a8be),
    (0x09A8C4, i_09a8c4),
    (0x09A8CC, i_09a8cc),
    (0x09A8D0, i_09a8d0),
    (0x09A8D6, i_09a8d6),
    (0x09A8DE, i_09a8de),
    (0x09A8E2, i_09a8e2),
    (0x09A8E8, i_09a8e8),
    (0x09A8EE, i_09a8ee),
    (0x09A8F4, i_09a8f4),
    (0x09A8F8, i_09a8f8),
    (0x09A8FA, i_09a8fa),
    (0x09A900, i_09a900),
    (0x09A906, i_09a906),
    (0x09A90A, i_09a90a),
    (0x09A910, i_09a910),
    (0x09A914, i_09a914),
    (0x09A918, i_09a918),
    (0x09A91C, i_09a91c),
    (0x09A922, i_09a922),
    (0x09A928, i_09a928),
    (0x09A92C, i_09a92c),
    (0x09A932, i_09a932),
    (0x09A938, i_09a938),
    (0x09A93E, i_09a93e),
    (0x09A944, i_09a944),
    (0x09A94A, i_09a94a),
    (0x09A94E, i_09a94e),
    (0x09A950, i_09a950),
    (0x09A956, i_09a956),
    (0x09A95C, i_09a95c),
    (0x09A960, i_09a960),
    (0x09A964, i_09a964),
    (0x09A968, i_09a968),
    (0x09A96C, i_09a96c),
    (0x09A970, i_09a970),
    (0x09A972, i_09a972),
    (0x09AF2A, i_09af2a),
    (0x09AF2E, i_09af2e),
    (0x09AF34, i_09af34),
    (0x09AF3A, i_09af3a),
    (0x09AF68, i_09af68),
    (0x09AF6C, i_09af6c),
    (0x09AF72, i_09af72),
    (0x09AF78, i_09af78),
    (0x09AFE0, i_09afe0),
    (0x09AFE4, i_09afe4),
    (0x09AFEA, i_09afea),
    (0x09AFF0, i_09aff0),
    (0x09B05C, i_09b05c),
    (0x09B060, i_09b060),
    (0x09B064, i_09b064),
    (0x09B068, i_09b068),
    (0x09B06E, i_09b06e),
    (0x09B072, i_09b072),
    (0x09B078, i_09b078),
    (0x09B07E, i_09b07e),
    (0x09B084, i_09b084),
    (0x09B088, i_09b088),
    (0x09B08E, i_09b08e),
    (0x09B094, i_09b094),
    (0x09B09A, i_09b09a),
    (0x09B09E, i_09b09e),
    (0x09B0A2, i_09b0a2),
    (0x09B0A6, i_09b0a6),
    (0x09B0A8, i_09b0a8),
    (0x09B0AC, i_09b0ac),
    (0x09B0B2, i_09b0b2),
    (0x09B0B6, i_09b0b6),
    (0x09B0BC, i_09b0bc),
    (0x09B0C0, i_09b0c0),
    (0x09B0C4, i_09b0c4),
    (0x09B0C6, i_09b0c6),
    (0x09B0CA, i_09b0ca),
    (0x09B0CE, i_09b0ce),
    (0x09B0D2, i_09b0d2),
    (0x09B0D4, i_09b0d4),
    (0x09B3C6, i_09b3c6),
    (0x09B3CA, i_09b3ca),
    (0x09B3D0, i_09b3d0),
    (0x09B3D8, i_09b3d8),
    (0x09B3DC, i_09b3dc),
    (0x09B3E2, i_09b3e2),
    (0x09B3E8, i_09b3e8),
    (0x09B3EE, i_09b3ee),
    (0x09B3F4, i_09b3f4),
    (0x09B3FA, i_09b3fa),
    (0x09B3FE, i_09b3fe),
    (0x09B400, i_09b400),
    (0x09B404, i_09b404),
    (0x09B408, i_09b408),
    (0x09B40E, i_09b40e),
    (0x09B414, i_09b414),
    (0x09B41A, i_09b41a),
    (0x09B420, i_09b420),
    (0x09B424, i_09b424),
    (0x09B42A, i_09b42a),
    (0x09B430, i_09b430),
    (0x09B434, i_09b434),
    (0x09B438, i_09b438),
    (0x09B43A, i_09b43a),
    (0x09B43E, i_09b43e),
    (0x09B442, i_09b442),
    (0x09B448, i_09b448),
    (0x09B44E, i_09b44e),
    (0x09B454, i_09b454),
    (0x09B458, i_09b458),
    (0x09B45A, i_09b45a),
    (0x09B460, i_09b460),
    (0x09B466, i_09b466),
    (0x09B46A, i_09b46a),
    (0x09B46E, i_09b46e),
    (0x09B470, i_09b470),
    (0x09B474, i_09b474),
    (0x09B47C, i_09b47c),
    (0x09B482, i_09b482),
    (0x09B51E, i_09b51e),
    (0x09B524, i_09b524),
    (0x09B528, i_09b528),
    (0x09B52E, i_09b52e),
    (0x09B532, i_09b532),
    (0x09B536, i_09b536),
    (0x09B53C, i_09b53c),
    (0x09B540, i_09b540),
    (0x09B542, i_09b542),
    (0x09B546, i_09b546),
    (0x09B54A, i_09b54a),
    (0x09B550, i_09b550),
    (0x09B554, i_09b554),
    (0x09B55A, i_09b55a),
    (0x09B55E, i_09b55e),
    (0x09B562, i_09b562),
    (0x09B568, i_09b568),
    (0x09B56C, i_09b56c),
    (0x09B570, i_09b570),
    (0x09B574, i_09b574),
    (0x09B578, i_09b578),
    (0x09B57C, i_09b57c),
    (0x09B580, i_09b580),
    (0x09B582, i_09b582),
    (0x09B586, i_09b586),
    (0x09B58C, i_09b58c),
    (0x09B590, i_09b590),
    (0x09B594, i_09b594),
    (0x09B598, i_09b598),
    (0x09B59C, i_09b59c),
    (0x09B5A0, i_09b5a0),
    (0x09B5A6, i_09b5a6),
    (0x09B5AC, i_09b5ac),
    (0x09B5B2, i_09b5b2),
    (0x09B5B6, i_09b5b6),
    (0x09B5BA, i_09b5ba),
    (0x09B5BC, i_09b5bc),
    (0x09B5BE, i_09b5be),
    (0x09B5C2, i_09b5c2),
    (0x09B5C6, i_09b5c6),
    (0x09B5CA, i_09b5ca),
    (0x09B5CE, i_09b5ce),
    (0x09B5D2, i_09b5d2),
    (0x09B5D4, i_09b5d4),
    (0x09B5D8, i_09b5d8),
    (0x09B5DC, i_09b5dc),
    (0x09B5E2, i_09b5e2),
    (0x09B5E4, i_09b5e4),
    (0x09B5EA, i_09b5ea),
    (0x09B5F0, i_09b5f0),
    (0x09B5F4, i_09b5f4),
    (0x09B5F6, i_09b5f6),
    (0x09B5FC, i_09b5fc),
    (0x09B602, i_09b602),
    (0x09B606, i_09b606),
    (0x09B60A, i_09b60a),
    (0x09B60E, i_09b60e),
    (0x09B612, i_09b612),
    (0x09B618, i_09b618),
    (0x09B61C, i_09b61c),
    (0x09B620, i_09b620),
    (0x09B624, i_09b624),
    (0x09B62A, i_09b62a),
    (0x09B630, i_09b630),
    (0x09B634, i_09b634),
    (0x09B638, i_09b638),
    (0x09B63A, i_09b63a),
    (0x09B63E, i_09b63e),
    (0x09B644, i_09b644),
    (0x09B648, i_09b648),
    (0x09B64C, i_09b64c),
    (0x09B64E, i_09b64e),
    (0x09B654, i_09b654),
    (0x09B658, i_09b658),
    (0x09B65E, i_09b65e),
    (0x09B662, i_09b662),
    (0x09B666, i_09b666),
    (0x09B668, i_09b668),
    (0x09B66A, i_09b66a),
    (0x09B66E, i_09b66e),
    (0x09B672, i_09b672),
    (0x09B676, i_09b676),
    (0x09B67A, i_09b67a),
    (0x09B680, i_09b680),
    (0x09B684, i_09b684),
    (0x09B68A, i_09b68a),
    (0x09B68E, i_09b68e),
    (0x09B692, i_09b692),
    (0x09B698, i_09b698),
    (0x09B69C, i_09b69c),
    (0x09B6A2, i_09b6a2),
    (0x09B6A6, i_09b6a6),
    (0x09B6AC, i_09b6ac),
    (0x09B6B2, i_09b6b2),
    (0x09B6B8, i_09b6b8),
    (0x09B6BC, i_09b6bc),
    (0x09B6C2, i_09b6c2),
    (0x09B6C6, i_09b6c6),
    (0x09B6CC, i_09b6cc),
    (0x09B6D2, i_09b6d2),
    (0x09B6D8, i_09b6d8),
    (0x09B6DC, i_09b6dc),
    (0x09B6E0, i_09b6e0),
    (0x09B6E6, i_09b6e6),
    (0x09B6E8, i_09b6e8),
    (0x09B6EC, i_09b6ec),
    (0x09B6F2, i_09b6f2),
    (0x09B6F6, i_09b6f6),
    (0x09B6FC, i_09b6fc),
    (0x09B700, i_09b700),
    (0x09B702, i_09b702),
    (0x09B708, i_09b708),
    (0x09B70C, i_09b70c),
    (0x09B712, i_09b712),
    (0x09B716, i_09b716),
    (0x09B71C, i_09b71c),
    (0x09B720, i_09b720),
    (0x09B724, i_09b724),
    (0x09B728, i_09b728),
    (0x09B72C, i_09b72c),
    (0x09B730, i_09b730),
    (0x09B734, i_09b734),
    (0x09B738, i_09b738),
    (0x09B73C, i_09b73c),
    (0x09B73E, i_09b73e),
    (0x09B742, i_09b742),
    (0x09B748, i_09b748),
    (0x09B74E, i_09b74e),
    (0x09B752, i_09b752),
    (0x09B756, i_09b756),
    (0x09B758, i_09b758),
    (0x09B75A, i_09b75a),
    (0x09B760, i_09b760),
    (0x09B766, i_09b766),
    (0x09B76C, i_09b76c),
    (0x09B770, i_09b770),
    (0x09B772, i_09b772),
    (0x09B778, i_09b778),
    (0x09B77C, i_09b77c),
    (0x09B782, i_09b782),
    (0x09B786, i_09b786),
    (0x09B78C, i_09b78c),
    (0x09B790, i_09b790),
    (0x09B794, i_09b794),
    (0x09B796, i_09b796),
    (0x09B79A, i_09b79a),
    (0x09B7A0, i_09b7a0),
    (0x09B7A4, i_09b7a4),
    (0x09B7A8, i_09b7a8),
    (0x09B7AA, i_09b7aa),
    (0x09B7B0, i_09b7b0),
    (0x09B7B4, i_09b7b4),
    (0x09B7B8, i_09b7b8),
    (0x09B7BA, i_09b7ba),
    (0x09B7BC, i_09b7bc),
    (0x09B7C0, i_09b7c0),
    (0x09B7C2, i_09b7c2),
    (0x09B7C8, i_09b7c8),
    (0x09B7CC, i_09b7cc),
    (0x09B7D2, i_09b7d2),
    (0x09B7D6, i_09b7d6),
    (0x09B7DC, i_09b7dc),
    (0x09B7E0, i_09b7e0),
    (0x09B7E4, i_09b7e4),
    (0x09B7E6, i_09b7e6),
    (0x09B7EC, i_09b7ec),
    (0x09B7F0, i_09b7f0),
    (0x09B7F4, i_09b7f4),
    (0x09B7F6, i_09b7f6),
    (0x09B7F8, i_09b7f8),
    (0x09B7FC, i_09b7fc),
    (0x09B7FE, i_09b7fe),
    (0x09B804, i_09b804),
    (0x09B80C, i_09b80c),
    (0x09B810, i_09b810),
    (0x09B814, i_09b814),
    (0x09B81A, i_09b81a),
    (0x09B820, i_09b820),
    (0x09B824, i_09b824),
    (0x09B828, i_09b828),
    (0x09B82A, i_09b82a),
    (0x09B82C, i_09b82c),
    (0x09B832, i_09b832),
    (0x09B838, i_09b838),
    (0x09B83C, i_09b83c),
    (0x09B83E, i_09b83e),
    (0x09B844, i_09b844),
    (0x09B84C, i_09b84c),
    (0x09B850, i_09b850),
    (0x09B856, i_09b856),
    (0x09B85C, i_09b85c),
    (0x09B864, i_09b864),
    (0x09B868, i_09b868),
    (0x09B870, i_09b870),
    (0x09B872, i_09b872),
    (0x09B878, i_09b878),
    (0x09B87C, i_09b87c),
    (0x09B880, i_09b880),
    (0x09B882, i_09b882),
    (0x09B886, i_09b886),
    (0x09B888, i_09b888),
    (0x09B88A, i_09b88a),
    (0x09B88E, i_09b88e),
    (0x09B892, i_09b892),
    (0x09B896, i_09b896),
    (0x09B89A, i_09b89a),
    (0x09B89E, i_09b89e),
    (0x09B8A2, i_09b8a2),
    (0x09B8A6, i_09b8a6),
    (0x09B8AA, i_09b8aa),
    (0x09B8AE, i_09b8ae),
    (0x09B8B2, i_09b8b2),
    (0x09B8B8, i_09b8b8),
    (0x09B8C0, i_09b8c0),
    (0x09B8C6, i_09b8c6),
    (0x09B8CA, i_09b8ca),
    (0x09B8CE, i_09b8ce),
    (0x09B8D4, i_09b8d4),
    (0x09B8D8, i_09b8d8),
    (0x09B8DE, i_09b8de),
    (0x09B8E2, i_09b8e2),
    (0x09B8E8, i_09b8e8),
    (0x09B8EC, i_09b8ec),
    (0x09B8EE, i_09b8ee),
    (0x09B8F2, i_09b8f2),
    (0x09B8F6, i_09b8f6),
    (0x09B8FA, i_09b8fa),
    (0x09B8FE, i_09b8fe),
    (0x09B902, i_09b902),
    (0x09B908, i_09b908),
    (0x09B90A, i_09b90a),
    (0x09B90E, i_09b90e),
    (0x09B912, i_09b912),
    (0x09B916, i_09b916),
    (0x09B91A, i_09b91a),
    (0x09B91E, i_09b91e),
    (0x09B920, i_09b920),
    (0x09B924, i_09b924),
    (0x09B928, i_09b928),
    (0x09B92C, i_09b92c),
    (0x09B930, i_09b930),
    (0x09B936, i_09b936),
    (0x09B93A, i_09b93a),
    (0x09B940, i_09b940),
    (0x09B946, i_09b946),
    (0x09B94C, i_09b94c),
    (0x09B950, i_09b950),
    (0x09B954, i_09b954),
    (0x09B958, i_09b958),
    (0x09B95A, i_09b95a),
    (0x09B95E, i_09b95e),
    (0x09B962, i_09b962),
    (0x09B966, i_09b966),
    (0x09B96A, i_09b96a),
    (0x09B972, i_09b972),
    (0x09B97A, i_09b97a),
    (0x09B97E, i_09b97e),
    (0x09B984, i_09b984),
    (0x09B986, i_09b986),
    (0x09B98A, i_09b98a),
    (0x09B98C, i_09b98c),
    (0x09B98E, i_09b98e),
    (0x09B990, i_09b990),
    (0x09B996, i_09b996),
    (0x09B99E, i_09b99e),
    (0x09B9A2, i_09b9a2),
    (0x09B9A6, i_09b9a6),
    (0x09B9AA, i_09b9aa),
    (0x09B9AE, i_09b9ae),
    (0x09B9B2, i_09b9b2),
    (0x09B9B4, i_09b9b4),
    (0x09B9B8, i_09b9b8),
    (0x09BB22, i_09bb22),
    (0x09BB26, i_09bb26),
    (0x09BB2E, i_09bb2e),
    (0x09BB32, i_09bb32),
    (0x09BB36, i_09bb36),
    (0x09BB3A, i_09bb3a),
    (0x09BB3C, i_09bb3c),
    (0x09BB40, i_09bb40),
    (0x09BB44, i_09bb44),
    (0x09BB48, i_09bb48),
    (0x09BB4A, i_09bb4a),
    (0x09BB4C, i_09bb4c),
    (0x09BB50, i_09bb50),
    (0x09BB54, i_09bb54),
];
