// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $061AA6: bne.w $61bf2
pub fn i_061aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AAA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x61BF2; return 10; }
    8
}
/// $061AAA: move.w #$109d, d0
pub fn i_061aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061AAE: jsr $2352.l
pub fn i_061aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61AB4);
    cpu.pc = 0x2352;
    16
}
/// $061AB4: jsr $267e2.l
pub fn i_061ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61ABA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61ABA);
    cpu.pc = 0x267E2;
    16
}
/// $061ABA: jsr $626d8(pc)
pub fn i_061aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61ABE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61ABE);
    cpu.pc = 0x626D8;
    16
}
/// $061ABE: bcs.w $61adc
pub fn i_061abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AC2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x61ADC; return 10; }
    8
}
/// $061AC2: lea.l $2c3812.l, a0
pub fn i_061ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3812;
    20
}
/// $061AC8: jsr $28cd4.l
pub fn i_061ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61ACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61ACE);
    cpu.pc = 0x28CD4;
    16
}
/// $061ACE: move.w #$ff00, d0
pub fn i_061ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AD2;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061AD2: move.l #$2c37ea, d1
pub fn i_061ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AD8;
    cpu.ir = 0x223C;
    let v1: u32 = 0x2C37EA;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $061AD8: bra.w $61af2
pub fn i_061ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61ADC;
    cpu.ir = 0x6000;
    cpu.pc = 0x61AF2;
    10
}
/// $061ADC: lea.l $2c3880.l, a0
pub fn i_061adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3880;
    20
}
/// $061AE2: jsr $28cd4.l
pub fn i_061ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61AE8);
    cpu.pc = 0x28CD4;
    16
}
/// $061AE8: move.w #$100, d0
pub fn i_061ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061AEC: move.l #$2c3858, d1
pub fn i_061aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AF2;
    cpu.ir = 0x223C;
    let v1: u32 = 0x2C3858;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $061AF2: btst.b #$0, $3a(a6)
pub fn i_061af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AF8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $061AF8: beq.w $61afe
pub fn i_061af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61AFE; return 10; }
    8
}
/// $061AFC: neg.w d0
pub fn i_061afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AFE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $061AFE: move.l d1, $7c(a6)
pub fn i_061afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B02;
    cpu.ir = 0x2D41;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $061B02: move.w d0, $28(a6)
pub fn i_061b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B06;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061B06: clr.w $78(a6)
pub fn i_061b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B0A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $061B0A: jsr $62732(pc)
pub fn i_061b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61B0E);
    cpu.pc = 0x62732;
    16
}
/// $061B0E: lea.l $61b14(pc), a1
pub fn i_061b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61B14;
    16
}
/// $061B12: move.l a1, (a6)
pub fn i_061b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061B14: jsr $28998.l
pub fn i_061b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B1A);
    cpu.pc = 0x28998;
    16
}
/// $061B1A: jsr $27afc.l
pub fn i_061b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B20);
    cpu.pc = 0x27AFC;
    16
}
/// $061B20: bcc.w $61b2a
pub fn i_061b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61B2A; return 10; }
    8
}
/// $061B24: lea.l $61c54(pc), a1
pub fn i_061b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61C54;
    16
}
/// $061B28: move.l a1, (a6)
pub fn i_061b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061B2A: jsr $28d70.l
pub fn i_061b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B30);
    cpu.pc = 0x28D70;
    16
}
/// $061B30: jsr $626f0(pc)
pub fn i_061b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61B34);
    cpu.pc = 0x626F0;
    16
}
/// $061B34: bcc.w $61b3e
pub fn i_061b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61B3E; return 10; }
    8
}
/// $061B38: lea.l $61c4e(pc), a1
pub fn i_061b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61C4E;
    16
}
/// $061B3C: move.l a1, (a6)
pub fn i_061b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061B3E: jsr $62710(pc)
pub fn i_061b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61B42);
    cpu.pc = 0x62710;
    16
}
/// $061B42: bcc.w $61b4c
pub fn i_061b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B46;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61B4C; return 10; }
    8
}
/// $061B46: lea.l $61c54(pc), a1
pub fn i_061b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61C54;
    16
}
/// $061B4A: move.l a1, (a6)
pub fn i_061b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061B4C: jsr $62758(pc)
pub fn i_061b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61B50);
    cpu.pc = 0x62758;
    16
}
/// $061B50: bcc.w $61b5a
pub fn i_061b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61B5A; return 10; }
    8
}
/// $061B54: lea.l $61c54(pc), a1
pub fn i_061b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61C54;
    16
}
/// $061B58: move.l a1, (a6)
pub fn i_061b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061B5A: jsr $283ca.l
pub fn i_061b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B60);
    cpu.pc = 0x283CA;
    16
}
/// $061B60: jsr $283d8.l
pub fn i_061b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B66);
    cpu.pc = 0x283D8;
    16
}
/// $061B66: bra.w $62014
pub fn i_061b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x62014;
    10
}
// ===== LateProp_Idle_061b6a  ($061B6A, 136 B, props/late_props_turrets_05e000.s)
/// $061B6A: move.w #$109d, d0
pub fn i_061b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061B6E: jsr $2352.l
pub fn i_061b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B74);
    cpu.pc = 0x2352;
    16
}
/// $061B74: lea.l $2b7cac.l, a0
pub fn i_061b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7CAC;
    20
}
/// $061B7A: jsr $799de.l
pub fn i_061b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B80);
    cpu.pc = 0x799DE;
    16
}
/// $061B80: move.w d0, $80(a6)
pub fn i_061b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B84;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061B84: lea.l $2b7d2e.l, a0
pub fn i_061b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7D2E;
    20
}
/// $061B8A: jsr $799de.l
pub fn i_061b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61B90);
    cpu.pc = 0x799DE;
    16
}
/// $061B90: andi.w #$3, d0
pub fn i_061b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B94;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $061B94: lsl.w #$2, d0
pub fn i_061b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B96;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $061B96: movea.l #$2c38c6, a0
pub fn i_061b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61B9C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C38C6;
    cpu.a[0] = v1;
    4
}
/// $061B9C: move.l (a0, d0.w), $5c(a6)
pub fn i_061b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BA2;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $061BA2: lea.l $2c390a.l, a0
pub fn i_061ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C390A;
    20
}
/// $061BA8: jsr $28cd4.l
pub fn i_061ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BAE);
    cpu.pc = 0x28CD4;
    16
}
/// $061BAE: lea.l $61bb4(pc), a1
pub fn i_061bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61BB4;
    16
}
/// $061BB2: move.l a1, (a6)
pub fn i_061bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061BB4: jsr $28998.l
pub fn i_061bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BBA);
    cpu.pc = 0x28998;
    16
}
/// $061BBA: jsr $2783a.l
pub fn i_061bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BC0);
    cpu.pc = 0x2783A;
    16
}
/// $061BC0: jsr $28d70.l
pub fn i_061bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BC6);
    cpu.pc = 0x28D70;
    16
}
/// $061BC6: bcc.w $61bd0
pub fn i_061bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61BD0; return 10; }
    8
}
/// $061BCA: lea.l $61bf2(pc), a1
pub fn i_061bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61BF2;
    16
}
/// $061BCE: move.l a1, (a6)
pub fn i_061bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061BD0: bclr.b #$3, $13(a6)
pub fn i_061bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BD6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $061BD6: lea.l $5e766.l, a0
pub fn i_061bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $061BDC: jsr $5e770.l
pub fn i_061bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BE2);
    cpu.pc = 0x5E770;
    16
}
/// $061BE2: jsr $283ca.l
pub fn i_061be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BE8);
    cpu.pc = 0x283CA;
    16
}
/// $061BE8: jsr $283d8.l
pub fn i_061be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BEE);
    cpu.pc = 0x283D8;
    16
}
/// $061BEE: bra.w $62046
pub fn i_061bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x62046;
    10
}
// ===== LateProp_Hit_061bf2  ($061BF2, 92 B, props/late_props_turrets_05e000.s)
/// $061BF2: lea.l $2c399c.l, a0
pub fn i_061bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C399C;
    20
}
/// $061BF8: jsr $28cd4.l
pub fn i_061bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61BFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61BFE);
    cpu.pc = 0x28CD4;
    16
}
/// $061BFE: lea.l $61c04(pc), a1
pub fn i_061bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61C04;
    16
}
/// $061C02: move.l a1, (a6)
pub fn i_061c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061C04: jsr $28998.l
pub fn i_061c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61C0A);
    cpu.pc = 0x28998;
    16
}
/// $061C0A: jsr $2783a.l
pub fn i_061c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61C10);
    cpu.pc = 0x2783A;
    16
}
/// $061C10: jsr $28d70.l
pub fn i_061c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61C16);
    cpu.pc = 0x28D70;
    16
}
/// $061C16: bcc.w $61c2e
pub fn i_061c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C1A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61C2E; return 10; }
    8
}
/// $061C1A: lea.l $61d04(pc), a1
pub fn i_061c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61D04;
    16
}
/// $061C1E: move.l a1, (a6)
pub fn i_061c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061C20: tst.b $98(a6)
pub fn i_061c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C24;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $061C24: beq.w $61c2e
pub fn i_061c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61C2E; return 10; }
    8
}
/// $061C28: lea.l $61bf2(pc), a1
pub fn i_061c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61BF2;
    16
}
/// $061C2C: move.l a1, (a6)
pub fn i_061c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061C2E: jsr $62710(pc)
pub fn i_061c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61C32);
    cpu.pc = 0x62710;
    16
}
/// $061C32: bcc.w $61c3c
pub fn i_061c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61C3C; return 10; }
    8
}
/// $061C36: lea.l $61d84(pc), a1
pub fn i_061c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61D84;
    16
}
/// $061C3A: move.l a1, (a6)
pub fn i_061c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C3C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061C3C: jsr $62758(pc)
pub fn i_061c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61C40);
    cpu.pc = 0x62758;
    16
}
/// $061C40: bcc.w $61c4a
pub fn i_061c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61C4A; return 10; }
    8
}
/// $061C44: lea.l $61b6a(pc), a1
pub fn i_061c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61B6A;
    16
}
/// $061C48: move.l a1, (a6)
pub fn i_061c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061C4A: bra.w $62014
pub fn i_061c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x62014;
    10
}
// ===== LateProp_Rts_061c4e  ($061C4E, 6 B, props/late_props_turrets_05e000.s)
/// $061C4E: eori.b #$1, $77(a6)
pub fn i_061c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C54;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== LateProp_Die_061c54  ($061C54, 176 B, props/late_props_turrets_05e000.s)
/// $061C54: move.w $28(a6), d0
pub fn i_061c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C58;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061C58: btst.b #$0, $3a(a6)
pub fn i_061c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C5E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $061C5E: beq.w $61c64
pub fn i_061c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61C64; return 10; }
    8
}
/// $061C62: neg.w d0
pub fn i_061c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C64;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $061C64: cmpi.w #$0, d0
pub fn i_061c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C68;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $061C68: bgt.w $61c82
pub fn i_061c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C6C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x61C82; return 10; }
    8
}
/// $061C6C: lea.l $2c39da.l, a0
pub fn i_061c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C39DA;
    20
}
/// $061C72: jsr $28cd4.l
pub fn i_061c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61C78);
    cpu.pc = 0x28CD4;
    16
}
/// $061C78: move.l #$2c37ea, d0
pub fn i_061c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C7E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2C37EA;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $061C7E: bra.w $61c94
pub fn i_061c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C82;
    cpu.ir = 0x6000;
    cpu.pc = 0x61C94;
    10
}
/// $061C82: lea.l $2c3a22.l, a0
pub fn i_061c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3A22;
    20
}
/// $061C88: jsr $28cd4.l
pub fn i_061c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61C8E);
    cpu.pc = 0x28CD4;
    16
}
/// $061C8E: move.l #$2c3858, d0
pub fn i_061c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C94;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2C3858;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $061C94: move.l d0, $7c(a6)
pub fn i_061c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C98;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $061C98: move.w $28(a6), d0
pub fn i_061c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C9C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061C9C: asr.w #$5, d0
pub fn i_061c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61C9E;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $061C9E: clr.w $78(a6)
pub fn i_061c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CA2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $061CA2: neg.w d0
pub fn i_061ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CA4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $061CA4: move.w d0, $2c(a6)
pub fn i_061ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CA8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061CA8: move.w #$109d, d0
pub fn i_061ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CAC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061CAC: jsr $2222.l
pub fn i_061cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61CB2);
    cpu.pc = 0x2222;
    16
}
/// $061CB2: lea.l $61cb8(pc), a1
pub fn i_061cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61CB8;
    16
}
/// $061CB6: move.l a1, (a6)
pub fn i_061cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061CB8: jsr $28998.l
pub fn i_061cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61CBE);
    cpu.pc = 0x28998;
    16
}
/// $061CBE: jsr $27afc.l
pub fn i_061cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61CC4);
    cpu.pc = 0x27AFC;
    16
}
/// $061CC4: tst.w $2c(a6)
pub fn i_061cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CC8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $061CC8: beq.w $61ce8
pub fn i_061cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CCC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61CE8; return 10; }
    8
}
/// $061CCC: tst.w $28(a6)
pub fn i_061ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CD0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $061CD0: bne.w $61ce8
pub fn i_061cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x61CE8; return 10; }
    8
}
/// $061CD4: move.w $2c(a6), d0
pub fn i_061cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CD8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061CD8: clr.w $2c(a6)
pub fn i_061cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CDC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $061CDC: cmpi.w #$0, d0
pub fn i_061cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CE0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $061CE0: blt.w $61ce8
pub fn i_061ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CE4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x61CE8; return 10; }
    8
}
/// $061CE4: bra.w $61ce8
pub fn i_061ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x61CE8;
    10
}
/// $061CE8: jsr $62732(pc)
pub fn i_061ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61CEC);
    cpu.pc = 0x62732;
    16
}
/// $061CEC: jsr $28d70.l
pub fn i_061cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61CF2);
    cpu.pc = 0x28D70;
    16
}
/// $061CF2: bcc.w $61cfc
pub fn i_061cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61CFC; return 10; }
    8
}
/// $061CF6: lea.l $61bf2(pc), a1
pub fn i_061cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61BF2;
    16
}
/// $061CFA: move.l a1, (a6)
pub fn i_061cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61CFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061CFC: jsr $62710(pc)
pub fn i_061cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61D00);
    cpu.pc = 0x62710;
    16
}
/// $061D00: bra.w $62014
pub fn i_061d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D04;
    cpu.ir = 0x6000;
    cpu.pc = 0x62014;
    10
}
// ===== LateProp_Piece_061d04  ($061D04, 128 B, props/late_props_turrets_05e000.s)
/// $061D04: jsr $62710(pc)
pub fn i_061d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61D08);
    cpu.pc = 0x62710;
    16
}
/// $061D08: bcs.w $61d84
pub fn i_061d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D0C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x61D84; return 10; }
    8
}
/// $061D0C: tst.b $98(a6)
pub fn i_061d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D10;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $061D10: bne.w $61bf2
pub fn i_061d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x61BF2; return 10; }
    8
}
/// $061D14: jsr $626d8(pc)
pub fn i_061d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61D18);
    cpu.pc = 0x626D8;
    16
}
/// $061D18: bcs.w $61d30
pub fn i_061d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D1C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x61D30; return 10; }
    8
}
/// $061D1C: lea.l $2c3a6a.l, a0
pub fn i_061d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3A6A;
    20
}
/// $061D22: jsr $28cd4.l
pub fn i_061d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D28);
    cpu.pc = 0x28CD4;
    16
}
/// $061D28: move.w #$ff00, d0
pub fn i_061d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061D2C: bra.w $61d40
pub fn i_061d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D30;
    cpu.ir = 0x6000;
    cpu.pc = 0x61D40;
    10
}
/// $061D30: lea.l $2c3aec.l, a0
pub fn i_061d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3AEC;
    20
}
/// $061D36: jsr $28cd4.l
pub fn i_061d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D3C);
    cpu.pc = 0x28CD4;
    16
}
/// $061D3C: move.w #$100, d0
pub fn i_061d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D40;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061D40: btst.b #$0, $3a(a6)
pub fn i_061d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D46;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $061D46: beq.w $61d4c
pub fn i_061d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61D4C; return 10; }
    8
}
/// $061D4A: neg.w d0
pub fn i_061d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D4C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $061D4C: asr.w #$5, d0
pub fn i_061d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D4E;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $061D4E: move.w d0, $2c(a6)
pub fn i_061d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D52;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061D52: lea.l $61d58(pc), a1
pub fn i_061d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61D58;
    16
}
/// $061D56: move.l a1, (a6)
pub fn i_061d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061D58: jsr $28998.l
pub fn i_061d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D5E);
    cpu.pc = 0x28998;
    16
}
/// $061D5E: jsr $27afc.l
pub fn i_061d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D64);
    cpu.pc = 0x27AFC;
    16
}
/// $061D64: jsr $28d70.l
pub fn i_061d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D6A);
    cpu.pc = 0x28D70;
    16
}
/// $061D6A: bcc.w $61d74
pub fn i_061d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61D74; return 10; }
    8
}
/// $061D6E: lea.l $61aa2(pc), a1
pub fn i_061d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61AA2;
    16
}
/// $061D72: move.l a1, (a6)
pub fn i_061d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061D74: jsr $283ca.l
pub fn i_061d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D7A);
    cpu.pc = 0x283CA;
    16
}
/// $061D7A: jsr $283d8.l
pub fn i_061d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D80);
    cpu.pc = 0x283D8;
    16
}
/// $061D80: bra.w $62014
pub fn i_061d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D84;
    cpu.ir = 0x6000;
    cpu.pc = 0x62014;
    10
}
// ===== LateProp_PieceIdle_061d84  ($061D84, 50 B, props/late_props_turrets_05e000.s)
/// $061D84: lea.l $2c3b6e.l, a0
pub fn i_061d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3B6E;
    20
}
/// $061D8A: jsr $28cd4.l
pub fn i_061d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D90);
    cpu.pc = 0x28CD4;
    16
}
/// $061D90: lea.l $61d96(pc), a1
pub fn i_061d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61D96;
    16
}
/// $061D94: move.l a1, (a6)
pub fn i_061d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061D96: jsr $28998.l
pub fn i_061d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61D9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61D9C);
    cpu.pc = 0x28998;
    16
}
/// $061D9C: jsr $2783a.l
pub fn i_061d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61DA2);
    cpu.pc = 0x2783A;
    16
}
/// $061DA2: jsr $28d70.l
pub fn i_061da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61DA8);
    cpu.pc = 0x28D70;
    16
}
/// $061DA8: bcc.w $61db2
pub fn i_061da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61DB2; return 10; }
    8
}
/// $061DAC: lea.l $61db6(pc), a1
pub fn i_061dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61DB6;
    16
}
/// $061DB0: move.l a1, (a6)
pub fn i_061db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061DB2: bra.w $62084
pub fn i_061db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x62084;
    10
}
// ===== LateProp_RngHP_061db6  ($061DB6, 16 B, props/late_props_turrets_05e000.s)
/// $061DB6: lea.l $2b7ba8.l, a0
pub fn i_061db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DBC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7BA8;
    20
}
/// $061DBC: jsr $799de.l
pub fn i_061dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61DC2);
    cpu.pc = 0x799DE;
    16
}
/// $061DC2: move.b d0, $76(a6)
pub fn i_061dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DC6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== LateProp_Debris_061dc6  ($061DC6, 172 B, props/late_props_turrets_05e000.s)
/// $061DC6: lea.l $2b7c2a.l, a0
pub fn i_061dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7C2A;
    20
}
/// $061DCC: jsr $799de.l
pub fn i_061dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61DD2);
    cpu.pc = 0x799DE;
    16
}
/// $061DD2: move.w d0, $74(a6)
pub fn i_061dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DD6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061DD6: cmpi.b #$1, $76(a6)
pub fn i_061dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DDC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $061DDC: ble.w $61dfa
pub fn i_061ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DE0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x61DFA; return 10; }
    8
}
/// $061DE0: cmpi.w #$3c, $74(a6)
pub fn i_061de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DE6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Word, false);
    16
}
/// $061DE6: bgt.w $61dfa
pub fn i_061de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DEA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x61DFA; return 10; }
    8
}
/// $061DEA: lea.l $2c3c74.l, a0
pub fn i_061dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3C74;
    20
}
/// $061DF0: jsr $28cd4.l
pub fn i_061df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61DF6);
    cpu.pc = 0x28CD4;
    16
}
/// $061DF6: bra.w $61e06
pub fn i_061df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61DFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x61E06;
    10
}
/// $061DFA: lea.l $2c3be0.l, a0
pub fn i_061dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3BE0;
    20
}
/// $061E00: jsr $28cd4.l
pub fn i_061e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E06);
    cpu.pc = 0x28CD4;
    16
}
/// $061E06: lea.l $61e0c(pc), a1
pub fn i_061e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61E0C;
    16
}
/// $061E0A: move.l a1, (a6)
pub fn i_061e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061E0C: jsr $28998.l
pub fn i_061e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E12);
    cpu.pc = 0x28998;
    16
}
/// $061E12: jsr $2783a.l
pub fn i_061e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E18);
    cpu.pc = 0x2783A;
    16
}
/// $061E18: jsr $28d70.l
pub fn i_061e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E1E);
    cpu.pc = 0x28D70;
    16
}
/// $061E1E: bcc.w $61e42
pub fn i_061e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61E42; return 10; }
    8
}
/// $061E22: cmpi.b #$0, $76(a6)
pub fn i_061e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E28;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $061E28: bgt.w $61e42
pub fn i_061e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E2C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x61E42; return 10; }
    8
}
/// $061E2C: lea.l $61bf2(pc), a1
pub fn i_061e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61BF2;
    16
}
/// $061E30: move.l a1, (a6)
pub fn i_061e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061E32: lea.l $2b7b26.l, a0
pub fn i_061e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7B26;
    20
}
/// $061E38: jsr $799de.l
pub fn i_061e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E3E);
    cpu.pc = 0x799DE;
    16
}
/// $061E3E: move.w d0, $74(a6)
pub fn i_061e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061E42: cmpi.b #$0, $76(a6)
pub fn i_061e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E48;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $061E48: ble.w $61e6e
pub fn i_061e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E4C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x61E6E; return 10; }
    8
}
/// $061E4C: subq.w #$1, $74(a6)
pub fn i_061e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E50;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $061E50: cmpi.w #$0, $74(a6)
pub fn i_061e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E56;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $061E56: bgt.w $61e6e
pub fn i_061e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E5A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x61E6E; return 10; }
    8
}
/// $061E5A: subq.b #$1, $76(a6)
pub fn i_061e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E5E;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $061E5E: cmpi.b #$0, $76(a6)
pub fn i_061e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E64;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $061E64: ble.w $61e6e
pub fn i_061e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E68;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x61E6E; return 10; }
    8
}
/// $061E68: lea.l $61dc6(pc), a1
pub fn i_061e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61DC6;
    16
}
/// $061E6C: move.l a1, (a6)
pub fn i_061e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061E6E: bra.w $62084
pub fn i_061e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E72;
    cpu.ir = 0x6000;
    cpu.pc = 0x62084;
    10
}
// ===== LateProp_Spark_061e72  ($061E72, 76 B, props/late_props_turrets_05e000.s)
/// $061E72: move.w #$109d, d0
pub fn i_061e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061E76: jsr $2222.l
pub fn i_061e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E7C);
    cpu.pc = 0x2222;
    16
}
/// $061E7C: bclr.b #$3, $13(a6)
pub fn i_061e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E82;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $061E82: jsr $267e2.l
pub fn i_061e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E88);
    cpu.pc = 0x267E2;
    16
}
/// $061E88: lea.l $2c3d7a.l, a0
pub fn i_061e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3D7A;
    20
}
/// $061E8E: jsr $28cd4.l
pub fn i_061e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61E94);
    cpu.pc = 0x28CD4;
    16
}
/// $061E94: lea.l $61e9a(pc), a1
pub fn i_061e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61E9A;
    16
}
/// $061E98: move.l a1, (a6)
pub fn i_061e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061E9A: jsr $28998.l
pub fn i_061e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61EA0);
    cpu.pc = 0x28998;
    16
}
/// $061EA0: jsr $2783a.l
pub fn i_061ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61EA6);
    cpu.pc = 0x2783A;
    16
}
/// $061EA6: jsr $28d70.l
pub fn i_061ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61EAC);
    cpu.pc = 0x28D70;
    16
}
/// $061EAC: bcc.w $61eb6
pub fn i_061eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EB0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61EB6; return 10; }
    8
}
/// $061EB0: lea.l $61d04(pc), a1
pub fn i_061eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61D04;
    16
}
/// $061EB4: move.l a1, (a6)
pub fn i_061eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061EB6: jsr $62710(pc)
pub fn i_061eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EBA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61EBA);
    cpu.pc = 0x62710;
    16
}
/// $061EBA: bra.w $62014
pub fn i_061eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x62014;
    10
}
// ===== LateProp_SparkHit_061ebe  ($061EBE, 104 B, props/late_props_turrets_05e000.s)
/// $061EBE: bclr.b #$3, $13(a6)
pub fn i_061ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EC4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $061EC4: jsr $267e2.l
pub fn i_061ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61ECA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61ECA);
    cpu.pc = 0x267E2;
    16
}
/// $061ECA: move.w #$109d, d0
pub fn i_061eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61ECE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061ECE: jsr $2222.l
pub fn i_061ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61ED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61ED4);
    cpu.pc = 0x2222;
    16
}
/// $061ED4: lea.l $2c3cec.l, a0
pub fn i_061ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3CEC;
    20
}
/// $061EDA: jsr $28cd4.l
pub fn i_061eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61EE0);
    cpu.pc = 0x28CD4;
    16
}
/// $061EE0: lea.l $61ee6(pc), a1
pub fn i_061ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61EE6;
    16
}
/// $061EE4: move.l a1, (a6)
pub fn i_061ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061EE6: jsr $28998.l
pub fn i_061ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61EEC);
    cpu.pc = 0x28998;
    16
}
/// $061EEC: jsr $2783a.l
pub fn i_061eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61EF2);
    cpu.pc = 0x2783A;
    16
}
/// $061EF2: jsr $28d70.l
pub fn i_061ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61EF8);
    cpu.pc = 0x28D70;
    16
}
/// $061EF8: bcc.w $61f02
pub fn i_061ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61EFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61F02; return 10; }
    8
}
/// $061EFC: lea.l $61d04(pc), a1
pub fn i_061efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61D04;
    16
}
/// $061F00: move.l a1, (a6)
pub fn i_061f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061F02: jsr $62710(pc)
pub fn i_061f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61F06);
    cpu.pc = 0x62710;
    16
}
/// $061F06: jsr $2870a.l
pub fn i_061f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F0C);
    cpu.pc = 0x2870A;
    16
}
/// $061F0C: bcc.w $61f22
pub fn i_061f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61F22; return 10; }
    8
}
/// $061F10: lea.l $5e766.l, a0
pub fn i_061f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $061F16: jsr $5e770.l
pub fn i_061f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F1C);
    cpu.pc = 0x5E770;
    16
}
/// $061F1C: bclr.b #$3, $13(a6)
pub fn i_061f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F22;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $061F22: bra.w $62046
pub fn i_061f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F26;
    cpu.ir = 0x6000;
    cpu.pc = 0x62046;
    10
}
// ===== LateProp_Shot_061f26  ($061F26, 112 B, props/late_props_turrets_05e000.s)
/// $061F26: jsr $267e2.l
pub fn i_061f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F2C);
    cpu.pc = 0x267E2;
    16
}
/// $061F2C: lea.l $77fd6.l, a1
pub fn i_061f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F32;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $061F32: jsr $4ae.l
pub fn i_061f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F38);
    cpu.pc = 0x4AE;
    16
}
/// $061F38: jsr $5dd02.l
pub fn i_061f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F3E);
    cpu.pc = 0x5DD02;
    16
}
/// $061F3E: move.w #$18, d0
pub fn i_061f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061F42: jsr $626b8(pc)
pub fn i_061f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61F46);
    cpu.pc = 0x626B8;
    16
}
/// $061F46: add.w d0, $22(a0)
pub fn i_061f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F4A;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $061F4A: move.b #$ff, $20(a6)
pub fn i_061f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F50;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061F50: lea.l $2c3da4.l, a0
pub fn i_061f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F56;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3DA4;
    20
}
/// $061F56: jsr $28cd4.l
pub fn i_061f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F5C);
    cpu.pc = 0x28CD4;
    16
}
/// $061F5C: lea.l $61f62(pc), a1
pub fn i_061f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61F62;
    16
}
/// $061F60: move.l a1, (a6)
pub fn i_061f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061F62: jsr $2783a.l
pub fn i_061f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F68);
    cpu.pc = 0x2783A;
    16
}
/// $061F68: jsr $28d70.l
pub fn i_061f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F6E);
    cpu.pc = 0x28D70;
    16
}
/// $061F6E: bcc.w $61f80
pub fn i_061f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61F80; return 10; }
    8
}
/// $061F72: tst.b $20(a6)
pub fn i_061f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F76;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $061F76: bne.w $61f80
pub fn i_061f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x61F80; return 10; }
    8
}
/// $061F7A: lea.l $61f9e(pc), a1
pub fn i_061f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61F9E;
    16
}
/// $061F7E: move.l a1, (a6)
pub fn i_061f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061F80: movea.l #$ffffffff, a0
pub fn i_061f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F86;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $061F86: lea.l $2c36f4.l, a0
pub fn i_061f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C36F4;
    20
}
/// $061F8C: jsr $5dd5c.l
pub fn i_061f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61F92);
    cpu.pc = 0x5DD5C;
    16
}
/// $061F92: bcc.w $61f9c
pub fn i_061f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61F9C; return 10; }
    8
}
// ===== SetTaskHandler_061f96  ($061F96, 8 B, kernel/task_handlers.c)
/// $061F96: lea.l $620a0(pc), a1
pub fn i_061f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A0;
    16
}
/// $061F9A: move.l a1, (a6)
pub fn i_061f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061F9C: rts
pub fn i_061f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61F9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_ShotB_061f9e  ($061F9E, 98 B, props/late_props_turrets_05e000.s)
/// $061F9E: lea.l $2c3e48.l, a0
pub fn i_061f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3E48;
    20
}
/// $061FA4: jsr $28cd4.l
pub fn i_061fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61FAA);
    cpu.pc = 0x28CD4;
    16
}
/// $061FAA: bra.w $61fba
pub fn i_061faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x61FBA;
    10
}
/// $061FAE: lea.l $2c3e9c.l, a0
pub fn i_061fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FB4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3E9C;
    20
}
/// $061FB4: jsr $28cd4.l
pub fn i_061fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61FBA);
    cpu.pc = 0x28CD4;
    16
}
/// $061FBA: bclr.b #$1, $12(a6)
pub fn i_061fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FC0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $061FC0: jsr $267e2.l
pub fn i_061fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61FC6);
    cpu.pc = 0x267E2;
    16
}
/// $061FC6: lea.l $61fcc(pc), a1
pub fn i_061fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61FCC;
    16
}
/// $061FCA: move.l a1, (a6)
pub fn i_061fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061FCC: jsr $2783a.l
pub fn i_061fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61FD2);
    cpu.pc = 0x2783A;
    16
}
/// $061FD2: jsr $28d70.l
pub fn i_061fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61FD8);
    cpu.pc = 0x28D70;
    16
}
/// $061FD8: bcc.w $61fea
pub fn i_061fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61FEA; return 10; }
    8
}
/// $061FDC: tst.b $83(a6)
pub fn i_061fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FE0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $061FE0: beq.w $61fea
pub fn i_061fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FE4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61FEA; return 10; }
    8
}
/// $061FE4: lea.l $62008(pc), a1
pub fn i_061fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62008;
    16
}
/// $061FE8: move.l a1, (a6)
pub fn i_061fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061FEA: movea.l #$ffffffff, a0
pub fn i_061fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FF0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $061FF0: lea.l $2c36f4.l, a0
pub fn i_061ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C36F4;
    20
}
/// $061FF6: jsr $5dd5c.l
pub fn i_061ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61FFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61FFC);
    cpu.pc = 0x5DD5C;
    16
}
/// $061FFC: bcc.w $62006
pub fn i_061ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62000;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62006; return 10; }
    8
}
// ===== SetTaskHandler_062000  ($062000, 8 B, kernel/task_handlers.c)
/// $062000: lea.l $620a0(pc), a1
pub fn i_062000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62004;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A0;
    16
}
/// $062004: move.l a1, (a6)
pub fn i_062004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62006;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062006: rts
pub fn i_062006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62008;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_HitThenDie_062008  ($062008, 4 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062008: jsr $6282e(pc)
pub fn i_062008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6200C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6200C);
    cpu.pc = 0x6282E;
    16
}
// ===== SetTaskHandler_06200c  ($06200C, 8 B, kernel/task_handlers.c)
/// $06200C: lea.l $620a0(pc), a1
pub fn i_06200c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A0;
    16
}
/// $062010: move.l a1, (a6)
pub fn i_062010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062012: rts
pub fn i_062012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62014;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_TakeHit_062014  ($062014, 50 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062014: jsr $2870a.l
pub fn i_062014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6201A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6201A);
    cpu.pc = 0x2870A;
    16
}
/// $06201A: bcc.w $62066
pub fn i_06201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6201E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62066; return 10; }
    8
}
/// $06201E: bclr.b #$3, $13(a6)
pub fn i_06201e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62024;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062024: lea.l $5e766.l, a0
pub fn i_062024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6202A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06202A: jsr $5e770.l
pub fn i_06202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62030;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62030);
    cpu.pc = 0x5E770;
    16
}
/// $062030: lea.l $61e72(pc), a1
pub fn i_062030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62034;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61E72;
    16
}
/// $062034: move.l a1, (a6)
pub fn i_062034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62036;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062036: jsr $5e844.l
pub fn i_062036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6203C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6203C);
    cpu.pc = 0x5E844;
    16
}
/// $06203C: bcc.w $62046
pub fn i_06203c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62040;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62046; return 10; }
    8
}
/// $062040: lea.l $61ebe(pc), a1
pub fn i_062040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62044;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61EBE;
    16
}
/// $062044: move.l a1, (a6)
pub fn i_062044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62046;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== LateProp_HPCheck_062046  ($062046, 54 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062046: jsr $28758.l
pub fn i_062046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6204C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6204C);
    cpu.pc = 0x28758;
    16
}
/// $06204C: bcc.w $62066
pub fn i_06204c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62050;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62066; return 10; }
    8
}
/// $062050: lea.l $61f26(pc), a1
pub fn i_062050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62054;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61F26;
    16
}
/// $062054: move.l a1, (a6)
pub fn i_062054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62056;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062056: jsr $5e844.l
pub fn i_062056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6205C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6205C);
    cpu.pc = 0x5E844;
    16
}
/// $06205C: bcc.w $62066
pub fn i_06205c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62060;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62066; return 10; }
    8
}
/// $062060: lea.l $61fae(pc), a1
pub fn i_062060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62064;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61FAE;
    16
}
/// $062064: move.l a1, (a6)
pub fn i_062064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62066;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062066: movea.l #$ffffffff, a0
pub fn i_062066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6206C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06206C: lea.l $2c36f4.l, a0
pub fn i_06206c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62072;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C36F4;
    20
}
/// $062072: jsr $5dd5c.l
pub fn i_062072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62078);
    cpu.pc = 0x5DD5C;
    16
}
/// $062078: bcc.w $62082
pub fn i_062078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6207C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62082; return 10; }
    8
}
// ===== SetTaskHandler_06207c  ($06207C, 8 B, kernel/task_handlers.c)
/// $06207C: lea.l $620a0(pc), a1
pub fn i_06207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62080;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A0;
    16
}
/// $062080: move.l a1, (a6)
pub fn i_062080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62082;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062082: rts
pub fn i_062082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62084;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_TakeHitB_062084  ($062084, 28 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062084: jsr $2870a.l
pub fn i_062084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6208A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6208A);
    cpu.pc = 0x2870A;
    16
}
/// $06208A: bcc.b $62046
pub fn i_06208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6208C;
    cpu.ir = 0x64BA;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62046; return 10; }
    8
}
/// $06208C: lea.l $5e766.l, a0
pub fn i_06208c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62092;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $062092: jsr $5e770.l
pub fn i_062092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62098);
    cpu.pc = 0x5E770;
    16
}
/// $062098: bclr.b #$3, $13(a6)
pub fn i_062098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6209E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06209E: bra.b $62046
pub fn i_06209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620A0;
    cpu.ir = 0x60A6;
    cpu.pc = 0x62046;
    10
}
// ===== JmpScheduler_0620a0  ($0620A0, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0620A0: jmp $518.l
pub fn i_0620a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620A6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0620a6  ($0620A6, 2 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0620A6: rts
pub fn i_0620a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_0620a8  ($0620A8, 14 B, kernel/jsr5b6_jmp518.c)
/// $0620A8: jsr $5b6.l
pub fn i_0620a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x620AE);
    cpu.pc = 0x5B6;
    16
}
/// $0620AE: jmp $518.l
pub fn i_0620ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620B4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0620B4: rts
pub fn i_0620b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SetHP_0620b6  ($0620B6, 36 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0620B6: lea.l $2b7aa4.l, a0
pub fn i_0620b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7AA4;
    20
}
/// $0620BC: jsr $799de.l
pub fn i_0620bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x620C2);
    cpu.pc = 0x799DE;
    16
}
/// $0620C2: move.w d0, $36(a6)
pub fn i_0620c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0620C6: tst.b $99(a6)
pub fn i_0620c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620CA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0620CA: bne.w $620d4
pub fn i_0620ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x620D4; return 10; }
    8
}
/// $0620CE: jmp $4c862.l
pub fn i_0620ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x4C862;
    8
}
/// $0620D4: jmp $4c846.l
pub fn i_0620d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x4C846;
    8
}
// ===== TurretCar_Gunner_0620da  ($0620DA, 72 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0620DA: move.w #$8, d1
pub fn i_0620da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0620DE: jsr $236e.l
pub fn i_0620de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x620E4);
    cpu.pc = 0x236E;
    16
}
/// $0620E4: eori.b #$1, $3a(a6)
pub fn i_0620e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620EA;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0620EA: move.w #$4000, d0
pub fn i_0620ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0620EE: jsr $28134.l
pub fn i_0620ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x620F4);
    cpu.pc = 0x28134;
    16
}
/// $0620F4: andi.w #$ffe3, $38(a6)
pub fn i_0620f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x620FA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0620FA: ori.w #$1c, $38(a6)
pub fn i_0620fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62100;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062100: lea.l $2c4022.l, a0
pub fn i_062100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62106;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4022;
    20
}
/// $062106: jsr $28cd4.l
pub fn i_062106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6210C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6210C);
    cpu.pc = 0x28CD4;
    16
}
/// $06210C: lea.l $62112(pc), a1
pub fn i_06210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62110;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62112;
    16
}
/// $062110: move.l a1, (a6)
pub fn i_062110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62112;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062112: jsr $2783a.l
pub fn i_062112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62118;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62118);
    cpu.pc = 0x2783A;
    16
}
/// $062118: jsr $28d70.l
pub fn i_062118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6211E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6211E);
    cpu.pc = 0x28D70;
    16
}
/// $06211E: bcc.w $62128
pub fn i_06211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62122;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62128; return 10; }
    8
}
// ===== SetTaskHandler_062122  ($062122, 8 B, kernel/task_handlers.c)
/// $062122: lea.l $620a8(pc), a1
pub fn i_062122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62126;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $062126: move.l a1, (a6)
pub fn i_062126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62128;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062128: rts
pub fn i_062128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6212A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_Init_06212a  ($06212A, 126 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06212A: move.w #$1a, d1
pub fn i_06212a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6212E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06212E: jsr $236e.l
pub fn i_06212e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62134;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62134);
    cpu.pc = 0x236E;
    16
}
/// $062134: jsr $5e0d4.l
pub fn i_062134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6213A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6213A);
    cpu.pc = 0x5E0D4;
    16
}
/// $06213A: bcs.w $62156
pub fn i_06213a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6213E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x62156; return 10; }
    8
}
/// $06213E: andi.b #$fe, $3a(a6)
pub fn i_06213e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62144;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062144: move.w $22(a0), d0
pub fn i_062144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62148;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $062148: cmp.w $22(a6), d0
pub fn i_062148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6214C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06214C: bcs.w $62156
pub fn i_06214c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62150;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x62156; return 10; }
    8
}
/// $062150: ori.b #$1, $3a(a6)
pub fn i_062150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62156;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062156: move.b $9a(a6), d0
pub fn i_062156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6215A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06215A: cmpi.b #$2, d0
pub fn i_06215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6215E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $06215E: bne.w $6216c
pub fn i_06215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62162;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6216C; return 10; }
    8
}
/// $062162: jsr $5e9b6.l
pub fn i_062162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62168);
    cpu.pc = 0x5E9B6;
    16
}
/// $062168: andi.b #$1, d0
pub fn i_062168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6216C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06216C: tst.b d0
pub fn i_06216c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6216E;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $06216E: beq.w $621fe
pub fn i_06216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62172;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x621FE; return 10; }
    8
}
/// $062172: move.b #$0, $9a(a6)
pub fn i_062172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62178;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $062178: move.b #$1, $82(a6)
pub fn i_062178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6217E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06217E: lea.l $2c413a.l, a0
pub fn i_06217e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62184;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C413A;
    20
}
/// $062184: jsr $28cd4.l
pub fn i_062184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6218A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6218A);
    cpu.pc = 0x28CD4;
    16
}
/// $06218A: lea.l $62190(pc), a1
pub fn i_06218a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6218E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62190;
    16
}
/// $06218E: move.l a1, (a6)
pub fn i_06218e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62190;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062190: jsr $62684(pc)
pub fn i_062190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62194;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62194);
    cpu.pc = 0x62684;
    16
}
/// $062194: jsr $28d70.l
pub fn i_062194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6219A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6219A);
    cpu.pc = 0x28D70;
    16
}
/// $06219A: bcc.w $621a4
pub fn i_06219a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6219E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x621A4; return 10; }
    8
}
/// $06219E: lea.l $621a8(pc), a1
pub fn i_06219e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x621A8;
    16
}
/// $0621A2: move.l a1, (a6)
pub fn i_0621a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0621A4: bra.w $62326
pub fn i_0621a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x62326;
    10
}
// ===== Sniper_Aim_0621a8  ($0621A8, 48 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0621A8: move.b #$2, $82(a6)
pub fn i_0621a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0621AE: lea.l $2c4188.l, a0
pub fn i_0621ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4188;
    20
}
/// $0621B4: jsr $28cd4.l
pub fn i_0621b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x621BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0621BA: lea.l $621c0(pc), a1
pub fn i_0621ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x621C0;
    16
}
/// $0621BE: move.l a1, (a6)
pub fn i_0621be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0621C0: jsr $62684(pc)
pub fn i_0621c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x621C4);
    cpu.pc = 0x62684;
    16
}
/// $0621C4: jsr $28d70.l
pub fn i_0621c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x621CA);
    cpu.pc = 0x28D70;
    16
}
/// $0621CA: bcc.w $621d4
pub fn i_0621ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x621D4; return 10; }
    8
}
/// $0621CE: lea.l $621d8(pc), a1
pub fn i_0621ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x621D8;
    16
}
/// $0621D2: move.l a1, (a6)
pub fn i_0621d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0621D4: bra.w $62326
pub fn i_0621d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x62326;
    10
}
// ===== Sniper_Fire_0621d8  ($0621D8, 92 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0621D8: lea.l $62456(pc), a1
pub fn i_0621d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62456;
    16
}
/// $0621DC: jsr $4ae.l
pub fn i_0621dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x621E2);
    cpu.pc = 0x4AE;
    16
}
/// $0621E2: jsr $5dd02.l
pub fn i_0621e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x621E8);
    cpu.pc = 0x5DD02;
    16
}
/// $0621E8: move.b #$0, $82(a6)
pub fn i_0621e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0621EE: lea.l $2c4106.l, a0
pub fn i_0621ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4106;
    20
}
/// $0621F4: jsr $28cd4.l
pub fn i_0621f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x621FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0621FA: bra.w $62216
pub fn i_0621fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x621FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x62216;
    10
}
/// $0621FE: move.b #$1, $9a(a6)
pub fn i_0621fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62204;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $062204: move.b #$0, $82(a6)
pub fn i_062204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6220A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06220A: lea.l $2c40c4.l, a0
pub fn i_06220a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62210;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C40C4;
    20
}
/// $062210: jsr $28cd4.l
pub fn i_062210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62216;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62216);
    cpu.pc = 0x28CD4;
    16
}
/// $062216: lea.l $6221c(pc), a1
pub fn i_062216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6221A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6221C;
    16
}
/// $06221A: move.l a1, (a6)
pub fn i_06221a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6221C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06221C: jsr $62684(pc)
pub fn i_06221c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62220;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62220);
    cpu.pc = 0x62684;
    16
}
/// $062220: jsr $28d70.l
pub fn i_062220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62226);
    cpu.pc = 0x28D70;
    16
}
/// $062226: bcc.w $62230
pub fn i_062226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6222A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62230; return 10; }
    8
}
/// $06222A: lea.l $62234(pc), a1
pub fn i_06222a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6222E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62234;
    16
}
/// $06222E: move.l a1, (a6)
pub fn i_06222e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62230;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062230: bra.w $62326
pub fn i_062230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62234;
    cpu.ir = 0x6000;
    cpu.pc = 0x62326;
    10
}
// ===== Sniper_Flee_062234  ($062234, 90 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062234: eori.b #$1, $3a(a6)
pub fn i_062234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6223A;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06223A: move.w #$ff78, d0
pub fn i_06223a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6223E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06223E: jsr $5dca4.l
pub fn i_06223e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62244;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62244);
    cpu.pc = 0x5DCA4;
    16
}
/// $062244: move.w d0, $28(a6)
pub fn i_062244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62248;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062248: move.w #$43f, $2a(a6)
pub fn i_062248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6224E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x43F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06224E: move.w #$ff6f, $2e(a6)
pub fn i_06224e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62254;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF6F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062254: move.w #$0, $2c(a6)
pub fn i_062254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6225A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06225A: move.w #$8000, d0
pub fn i_06225a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6225E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06225E: jsr $28134.l
pub fn i_06225e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62264);
    cpu.pc = 0x28134;
    16
}
/// $062264: andi.w #$ffe3, $38(a6)
pub fn i_062264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6226A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06226A: ori.w #$18, $38(a6)
pub fn i_06226a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62270;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062270: movea.l $c(a6), a0
pub fn i_062270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62274;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $062274: clr.b $20(a0)
pub fn i_062274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62278;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $062278: cmpi.b #$1, $9a(a6)
pub fn i_062278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6227E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06227E: beq.w $62288
pub fn i_06227e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62282;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62288; return 10; }
    8
}
/// $062282: jmp $58f82.l
pub fn i_062282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62288;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x58F82;
    8
}
/// $062288: jmp $58fe2.l
pub fn i_062288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6228E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x58FE2;
    8
}
// ===== Sniper_Die_06228e  ($06228E, 144 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06228E: clr.b $44(a6)
pub fn i_06228e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62292;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $062292: jsr $626b8(pc)
pub fn i_062292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62296;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62296);
    cpu.pc = 0x626B8;
    16
}
/// $062296: bcs.w $622aa
pub fn i_062296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6229A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x622AA; return 10; }
    8
}
/// $06229A: lea.l $2c41ea.l, a0
pub fn i_06229a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C41EA;
    20
}
/// $0622A0: jsr $28cd4.l
pub fn i_0622a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x622A6);
    cpu.pc = 0x28CD4;
    16
}
/// $0622A6: bra.w $622b6
pub fn i_0622a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x622B6;
    10
}
/// $0622AA: lea.l $2c4274.l, a0
pub fn i_0622aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4274;
    20
}
/// $0622B0: jsr $28cd4.l
pub fn i_0622b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x622B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0622B6: cmpi.b #$0, $82(a6)
pub fn i_0622b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622BC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0622BC: beq.w $622ee
pub fn i_0622bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x622EE; return 10; }
    8
}
/// $0622C0: cmpi.b #$2, $82(a6)
pub fn i_0622c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622C6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0622C6: beq.w $622de
pub fn i_0622c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x622DE; return 10; }
    8
}
/// $0622CA: lea.l $62446(pc), a1
pub fn i_0622ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62446;
    16
}
/// $0622CE: jsr $4ae.l
pub fn i_0622ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x622D4);
    cpu.pc = 0x4AE;
    16
}
/// $0622D4: jsr $5dd02.l
pub fn i_0622d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x622DA);
    cpu.pc = 0x5DD02;
    16
}
/// $0622DA: bra.w $622ee
pub fn i_0622da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x622EE;
    10
}
/// $0622DE: lea.l $62456(pc), a1
pub fn i_0622de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62456;
    16
}
/// $0622E2: jsr $4ae.l
pub fn i_0622e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x622E8);
    cpu.pc = 0x4AE;
    16
}
/// $0622E8: jsr $5dd02.l
pub fn i_0622e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x622EE);
    cpu.pc = 0x5DD02;
    16
}
/// $0622EE: lea.l $622f4(pc), a1
pub fn i_0622ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x622F4;
    16
}
/// $0622F2: move.l a1, (a6)
pub fn i_0622f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0622F4: jsr $62684(pc)
pub fn i_0622f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x622F8);
    cpu.pc = 0x62684;
    16
}
/// $0622F8: addq.w #$1, $38(a6)
pub fn i_0622f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x622FC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0622FC: jsr $28d70.l
pub fn i_0622fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62302);
    cpu.pc = 0x28D70;
    16
}
/// $062302: bcc.w $62314
pub fn i_062302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62306;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62314; return 10; }
    8
}
/// $062306: movea.l $c(a6), a0
pub fn i_062306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6230A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06230A: clr.b $20(a0)
pub fn i_06230a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6230E;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06230E: lea.l $620a0(pc), a1
pub fn i_06230e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62312;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A0;
    16
}
/// $062312: move.l a1, (a6)
pub fn i_062312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62314;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062314: jsr $5e45a.l
pub fn i_062314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6231A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6231A);
    cpu.pc = 0x5E45A;
    16
}
/// $06231A: bcc.w $62324
pub fn i_06231a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6231E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62324; return 10; }
    8
}
// ===== SetTaskHandler_06231e  ($06231E, 8 B, kernel/task_handlers.c)
/// $06231E: lea.l $620a0(pc), a1
pub fn i_06231e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62322;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A0;
    16
}
/// $062322: move.l a1, (a6)
pub fn i_062322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62324;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062324: rts
pub fn i_062324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62326;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_HitCheck_062326  ($062326, 44 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062326: jsr $2870a.l
pub fn i_062326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6232C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6232C);
    cpu.pc = 0x2870A;
    16
}
/// $06232C: bcc.w $62348
pub fn i_06232c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62330;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62348; return 10; }
    8
}
/// $062330: lea.l $5e766.l, a0
pub fn i_062330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62336;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $062336: jsr $5e770.l
pub fn i_062336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6233C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6233C);
    cpu.pc = 0x5E770;
    16
}
/// $06233C: lea.l $6228e(pc), a1
pub fn i_06233c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62340;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6228E;
    16
}
/// $062340: move.l a1, (a6)
pub fn i_062340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62342;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062342: jsr $519be.l
pub fn i_062342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62348);
    cpu.pc = 0x519BE;
    16
}
/// $062348: jsr $5e45a.l
pub fn i_062348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6234E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6234E);
    cpu.pc = 0x5E45A;
    16
}
/// $06234E: bcc.w $62358
pub fn i_06234e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62352;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62358; return 10; }
    8
}
// ===== SetTaskHandler_062352  ($062352, 8 B, kernel/task_handlers.c)
/// $062352: lea.l $620a0(pc), a1
pub fn i_062352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62356;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A0;
    16
}
/// $062356: move.l a1, (a6)
pub fn i_062356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62358;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062358: rts
pub fn i_062358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6235A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_Grenade_06235a  ($06235A, 186 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06235A: bset.b #$4, $6b(a6)
pub fn i_06235a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62360;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062360: move.w #$d000, $38(a6)
pub fn i_062360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62366;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062366: move.w #$16, d1
pub fn i_062366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6236A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06236A: jsr $236e.l
pub fn i_06236a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62370);
    cpu.pc = 0x236E;
    16
}
/// $062370: move.w #$1d2, d1
pub fn i_062370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62374;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062374: jsr $236e.l
pub fn i_062374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6237A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6237A);
    cpu.pc = 0x236E;
    16
}
/// $06237A: move.w #$167, d1
pub fn i_06237a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6237E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x167;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06237E: jsr $236e.l
pub fn i_06237e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62384);
    cpu.pc = 0x236E;
    16
}
/// $062384: move.w #$200, d0
pub fn i_062384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62388;
    cpu.ir = 0x303C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062388: btst.b #$0, $3a(a6)
pub fn i_062388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6238E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06238E: bne.w $62394
pub fn i_06238e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62392;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x62394; return 10; }
    8
}
/// $062392: neg.w d0
pub fn i_062392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62394;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $062394: move.w d0, $28(a6)
pub fn i_062394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62398;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062398: asr.w #$5, d0
pub fn i_062398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6239A;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $06239A: move.w d0, $2c(a6)
pub fn i_06239a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6239E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06239E: move.w #$1, $66(a6)
pub fn i_06239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0623A4: lea.l $2c4504.l, a0
pub fn i_0623a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4504;
    20
}
/// $0623AA: jsr $28cd4.l
pub fn i_0623aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x623B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0623B0: lea.l $623b6(pc), a1
pub fn i_0623b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x623B6;
    16
}
/// $0623B4: move.l a1, (a6)
pub fn i_0623b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0623B6: lea.l $2c3750.l, a0
pub fn i_0623b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3750;
    20
}
/// $0623BC: move.b $106f28.l, d0
pub fn i_0623bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623C2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0623C2: andi.w #$3, d0
pub fn i_0623c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0623C6: add.w d0, d0
pub fn i_0623c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623C8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0623C8: move.w (a0, d0.w), d0
pub fn i_0623c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623CC;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0623CC: move.w (a6, d0.w), $14(a6)
pub fn i_0623cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623D2;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0623D2: jsr $27cee.l
pub fn i_0623d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x623D8);
    cpu.pc = 0x27CEE;
    16
}
/// $0623D8: jsr $28d70.l
pub fn i_0623d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x623DE);
    cpu.pc = 0x28D70;
    16
}
/// $0623DE: jsr $283d8.l
pub fn i_0623de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x623E4);
    cpu.pc = 0x283D8;
    16
}
/// $0623E4: btst.b #$1, $13(a6)
pub fn i_0623e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0623EA: beq.w $623f4
pub fn i_0623ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x623F4; return 10; }
    8
}
/// $0623EE: lea.l $6241c(pc), a1
pub fn i_0623ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6241C;
    16
}
/// $0623F2: move.l a1, (a6)
pub fn i_0623f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0623F4: jsr $2870a.l
pub fn i_0623f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x623FA);
    cpu.pc = 0x2870A;
    16
}
/// $0623FA: bcc.w $62404
pub fn i_0623fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x623FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62404; return 10; }
    8
}
/// $0623FE: lea.l $6241c(pc), a1
pub fn i_0623fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62402;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6241C;
    16
}
/// $062402: move.l a1, (a6)
pub fn i_062402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62404;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062404: movea.l #$ffffffff, a0
pub fn i_062404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6240A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06240A: jsr $5dd56.l
pub fn i_06240a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62410);
    cpu.pc = 0x5DD56;
    16
}
/// $062410: bcc.w $6241a
pub fn i_062410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62414;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6241A; return 10; }
    8
}
// ===== SetTaskHandler_062414  ($062414, 8 B, kernel/task_handlers.c)
/// $062414: lea.l $620a8(pc), a1
pub fn i_062414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62418;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $062418: move.l a1, (a6)
pub fn i_062418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6241A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06241A: rts
pub fn i_06241a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6241C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_GrenadeBurst_06241c  ($06241C, 42 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06241C: jsr $13600.l
pub fn i_06241c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62422);
    cpu.pc = 0x13600;
    16
}
/// $062422: move.w #$4000, d0
pub fn i_062422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62426;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062426: jsr $28134.l
pub fn i_062426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6242C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6242C);
    cpu.pc = 0x28134;
    16
}
/// $06242C: andi.w #$ffe3, $38(a6)
pub fn i_06242c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62432;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062432: ori.w #$0, $38(a6)
pub fn i_062432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62438;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062438: move.l #$ffffffff, $48(a6)
pub fn i_062438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62440;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $062440: jmp $77f6a.l
pub fn i_062440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62446;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Sniper_Shell_062446  ($062446, 136 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062446: lea.l $2c4398.l, a0
pub fn i_062446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6244C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4398;
    20
}
/// $06244C: jsr $28cd4.l
pub fn i_06244c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62452);
    cpu.pc = 0x28CD4;
    16
}
/// $062452: bra.w $62462
pub fn i_062452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62456;
    cpu.ir = 0x6000;
    cpu.pc = 0x62462;
    10
}
/// $062456: lea.l $2c444e.l, a0
pub fn i_062456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6245C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C444E;
    20
}
/// $06245C: jsr $28cd4.l
pub fn i_06245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62462);
    cpu.pc = 0x28CD4;
    16
}
/// $062462: move.w #$1a, d1
pub fn i_062462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62466;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062466: jsr $236e.l
pub fn i_062466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6246C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6246C);
    cpu.pc = 0x236E;
    16
}
/// $06246C: move.w #$2d, d0
pub fn i_06246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62470;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062470: jsr $5dca4.l
pub fn i_062470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62476);
    cpu.pc = 0x5DCA4;
    16
}
/// $062476: move.w d0, $28(a6)
pub fn i_062476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6247A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06247A: move.w #$b56, $2a(a6)
pub fn i_06247a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62480;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062480: move.w #$ff7f, $2e(a6)
pub fn i_062480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62486;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062486: move.w #$0, $2c(a6)
pub fn i_062486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6248C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06248C: move.w #$8000, d0
pub fn i_06248c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62490;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062490: jsr $28134.l
pub fn i_062490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62496);
    cpu.pc = 0x28134;
    16
}
/// $062496: andi.w #$ffe3, $38(a6)
pub fn i_062496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6249C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06249C: ori.w #$0, $38(a6)
pub fn i_06249c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624A2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0624A2: lea.l $624a8(pc), a1
pub fn i_0624a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x624A8;
    16
}
/// $0624A6: move.l a1, (a6)
pub fn i_0624a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0624A8: jsr $27c8c.l
pub fn i_0624a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x624AE);
    cpu.pc = 0x27C8C;
    16
}
/// $0624AE: bcc.w $624b8
pub fn i_0624ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x624B8; return 10; }
    8
}
/// $0624B2: lea.l $620a8(pc), a1
pub fn i_0624b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $0624B6: move.l a1, (a6)
pub fn i_0624b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0624B8: jsr $28d70.l
pub fn i_0624b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x624BE);
    cpu.pc = 0x28D70;
    16
}
/// $0624BE: movea.l #$ffffffff, a0
pub fn i_0624be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624C4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0624C4: jsr $5dd56.l
pub fn i_0624c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x624CA);
    cpu.pc = 0x5DD56;
    16
}
/// $0624CA: bcc.w $624d4
pub fn i_0624ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x624D4; return 10; }
    8
}
// ===== SetTaskHandler_0624ce  ($0624CE, 8 B, kernel/task_handlers.c)
/// $0624CE: lea.l $620a8(pc), a1
pub fn i_0624ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $0624D2: move.l a1, (a6)
pub fn i_0624d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0624D4: rts
pub fn i_0624d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_Muzzle_0624d6  ($0624D6, 88 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0624D6: move.w #$4000, d0
pub fn i_0624d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0624DA: jsr $28134.l
pub fn i_0624da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x624E0);
    cpu.pc = 0x28134;
    16
}
/// $0624E0: andi.w #$ffe3, $38(a6)
pub fn i_0624e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624E6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0624E6: ori.w #$1c, $38(a6)
pub fn i_0624e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624EC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0624EC: move.w #$18, d1
pub fn i_0624ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0624F0: jsr $236e.l
pub fn i_0624f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x624F6);
    cpu.pc = 0x236E;
    16
}
/// $0624F6: lea.l $2c4306.l, a0
pub fn i_0624f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4306;
    20
}
/// $0624FC: jsr $28cd4.l
pub fn i_0624fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62502);
    cpu.pc = 0x28CD4;
    16
}
/// $062502: lea.l $62508(pc), a1
pub fn i_062502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62506;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62508;
    16
}
/// $062506: move.l a1, (a6)
pub fn i_062506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62508;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062508: jsr $2783a.l
pub fn i_062508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6250E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6250E);
    cpu.pc = 0x2783A;
    16
}
/// $06250E: jsr $28d70.l
pub fn i_06250e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62514);
    cpu.pc = 0x28D70;
    16
}
/// $062514: bcc.w $6251e
pub fn i_062514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62518;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6251E; return 10; }
    8
}
/// $062518: lea.l $620a8(pc), a1
pub fn i_062518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6251C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $06251C: move.l a1, (a6)
pub fn i_06251c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6251E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06251E: movea.l #$ffffffff, a0
pub fn i_06251e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62524;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062524: jsr $5dd56.l
pub fn i_062524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6252A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6252A);
    cpu.pc = 0x5DD56;
    16
}
/// $06252A: bcc.w $62534
pub fn i_06252a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6252E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62534; return 10; }
    8
}
// ===== SetTaskHandler_06252e  ($06252E, 8 B, kernel/task_handlers.c)
/// $06252E: lea.l $620a8(pc), a1
pub fn i_06252e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62532;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $062532: move.l a1, (a6)
pub fn i_062532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62534;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062534: rts
pub fn i_062534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62536;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_SmokeA_062536  ($062536, 82 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062536: move.w #$d000, $38(a6)
pub fn i_062536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6253C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06253C: jsr $5e9b6.l
pub fn i_06253c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62542);
    cpu.pc = 0x5E9B6;
    16
}
/// $062542: move.w d0, d1
pub fn i_062542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62544;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062544: andi.w #$700, d1
pub fn i_062544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62548;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $062548: addi.w #$200, d1
pub fn i_062548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6254C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06254C: move.w d1, $36(a6)
pub fn i_06254c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62550;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062550: btst.b #$0, d0
pub fn i_062550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62554;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $062554: beq.w $6256c
pub fn i_062554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62558;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6256C; return 10; }
    8
}
/// $062558: lea.l $2de43a.l, a0
pub fn i_062558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6255E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $06255E: jsr $28cd4.l
pub fn i_06255e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62564);
    cpu.pc = 0x28CD4;
    16
}
/// $062564: move.w #$77, d1
pub fn i_062564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62568;
    cpu.ir = 0x323C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062568: bra.w $6257c
pub fn i_062568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6256C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6257C;
    10
}
/// $06256C: lea.l $2de5f4.l, a0
pub fn i_06256c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62572;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE5F4;
    20
}
/// $062572: jsr $28cd4.l
pub fn i_062572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62578);
    cpu.pc = 0x28CD4;
    16
}
/// $062578: move.w #$78, d1
pub fn i_062578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6257C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06257C: jsr $236e.l
pub fn i_06257c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62582);
    cpu.pc = 0x236E;
    16
}
/// $062582: jmp $6dce0.l
pub fn i_062582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62588;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
// ===== Sniper_SmokeB_062588  ($062588, 82 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062588: move.w #$d000, $38(a6)
pub fn i_062588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6258E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06258E: jsr $5e9b6.l
pub fn i_06258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62594;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62594);
    cpu.pc = 0x5E9B6;
    16
}
/// $062594: move.w d0, d1
pub fn i_062594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62596;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062596: andi.w #$700, d1
pub fn i_062596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6259A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06259A: addi.w #$100, d1
pub fn i_06259a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6259E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06259E: move.w d1, $36(a6)
pub fn i_06259e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625A2;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0625A2: btst.b #$0, d0
pub fn i_0625a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625A6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0625A6: beq.w $625be
pub fn i_0625a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x625BE; return 10; }
    8
}
/// $0625AA: lea.l $2de43a.l, a0
pub fn i_0625aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0625B0: jsr $28cd4.l
pub fn i_0625b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x625B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0625B6: move.w #$77, d1
pub fn i_0625b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0625BA: bra.w $625ce
pub fn i_0625ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x625CE;
    10
}
/// $0625BE: lea.l $2de5f4.l, a0
pub fn i_0625be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE5F4;
    20
}
/// $0625C4: jsr $28cd4.l
pub fn i_0625c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x625CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0625CA: move.w #$78, d1
pub fn i_0625ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0625CE: jsr $236e.l
pub fn i_0625ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x625D4);
    cpu.pc = 0x236E;
    16
}
/// $0625D4: jmp $6dd16.l
pub fn i_0625d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DD16;
    8
}
// ===== Sniper_Flash_0625da  ($0625DA, 88 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0625DA: move.w #$d000, d0
pub fn i_0625da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0625DE: jsr $28134.l
pub fn i_0625de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x625E4);
    cpu.pc = 0x28134;
    16
}
/// $0625E4: andi.w #$ffe3, $38(a6)
pub fn i_0625e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625EA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0625EA: ori.w #$14, $38(a6)
pub fn i_0625ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625F0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0625F0: move.w #$148, d1
pub fn i_0625f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x148;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0625F4: jsr $236e.l
pub fn i_0625f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x625FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x625FA);
    cpu.pc = 0x236E;
    16
}
/// $0625FA: lea.l $2c3f0a.l, a0
pub fn i_0625fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62600;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3F0A;
    20
}
/// $062600: jsr $28cd4.l
pub fn i_062600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62606;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62606);
    cpu.pc = 0x28CD4;
    16
}
/// $062606: lea.l $6260c(pc), a1
pub fn i_062606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6260A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6260C;
    16
}
/// $06260A: move.l a1, (a6)
pub fn i_06260a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6260C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06260C: jsr $2783a.l
pub fn i_06260c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62612);
    cpu.pc = 0x2783A;
    16
}
/// $062612: jsr $28d70.l
pub fn i_062612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62618;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62618);
    cpu.pc = 0x28D70;
    16
}
/// $062618: bcc.w $62622
pub fn i_062618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6261C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62622; return 10; }
    8
}
/// $06261C: lea.l $620a8(pc), a1
pub fn i_06261c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62620;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $062620: move.l a1, (a6)
pub fn i_062620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62622;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062622: movea.l #$ffffffff, a0
pub fn i_062622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62628;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062628: jsr $5dd56.l
pub fn i_062628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6262E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6262E);
    cpu.pc = 0x5DD56;
    16
}
/// $06262E: bcc.w $62638
pub fn i_06262e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62632;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62638; return 10; }
    8
}
// ===== SetTaskHandler_062632  ($062632, 8 B, kernel/task_handlers.c)
/// $062632: lea.l $620a8(pc), a1
pub fn i_062632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62636;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $062636: move.l a1, (a6)
pub fn i_062636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62638;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062638: rts
pub fn i_062638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6263A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_Spark_06263a  ($06263A, 66 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06263A: move.w #$4, d1
pub fn i_06263a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6263E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06263E: jsr $236e.l
pub fn i_06263e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62644);
    cpu.pc = 0x236E;
    16
}
/// $062644: lea.l $2dd9ee.l, a0
pub fn i_062644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6264A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD9EE;
    20
}
/// $06264A: jsr $28cd4.l
pub fn i_06264a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62650);
    cpu.pc = 0x28CD4;
    16
}
/// $062650: lea.l $62656(pc), a1
pub fn i_062650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62654;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62656;
    16
}
/// $062654: move.l a1, (a6)
pub fn i_062654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62656;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062656: jsr $2783a.l
pub fn i_062656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6265C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6265C);
    cpu.pc = 0x2783A;
    16
}
/// $06265C: jsr $28d70.l
pub fn i_06265c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62662);
    cpu.pc = 0x28D70;
    16
}
/// $062662: bcc.w $6266c
pub fn i_062662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62666;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6266C; return 10; }
    8
}
/// $062666: lea.l $620a8(pc), a1
pub fn i_062666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6266A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $06266A: move.l a1, (a6)
pub fn i_06266a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6266C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06266C: movea.l #$ffffffff, a0
pub fn i_06266c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62672;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062672: jsr $5dd56.l
pub fn i_062672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62678;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62678);
    cpu.pc = 0x5DD56;
    16
}
/// $062678: bcc.w $62682
pub fn i_062678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6267C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62682; return 10; }
    8
}
// ===== SetTaskHandler_06267c  ($06267C, 8 B, kernel/task_handlers.c)
/// $06267C: lea.l $620a8(pc), a1
pub fn i_06267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62680;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620A8;
    16
}
/// $062680: move.l a1, (a6)
pub fn i_062680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62682;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062682: rts
pub fn i_062682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62684;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sniper_ChildPos_062684  ($062684, 46 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062684: movea.l $c(a6), a0
pub fn i_062684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62688;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $062688: move.w #$fffc, d0
pub fn i_062688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6268C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06268C: btst.b #$0, $3a(a0)
pub fn i_06268c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62692;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $062692: beq.w $62698
pub fn i_062692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62696;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62698; return 10; }
    8
}
/// $062696: neg.w d0
pub fn i_062696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62698;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $062698: add.w $22(a0), d0
pub fn i_062698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6269C;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06269C: move.w d0, $22(a6)
pub fn i_06269c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626A0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0626A0: move.w $24(a0), d0
pub fn i_0626a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626A4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0626A4: addi.w #$28, d0
pub fn i_0626a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626A8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x28, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0626A8: move.w d0, $24(a6)
pub fn i_0626a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626AC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0626AC: move.w $38(a0), d0
pub fn i_0626ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626B0;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0626B0: subq.w #$1, d0
pub fn i_0626b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626B2;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_0626b2  ($0626B2, 6 B, kernel/task_setters_w.c)
/// $0626B2: move.w d0, $38(a6)
pub fn i_0626b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626B6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0626B6: rts
pub fn i_0626b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Facing_SignDelta_0626b8  ($0626B8, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0626B8: btst.b #$0, $3a(a6)
pub fn i_0626b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626BE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0626BE: bne.w $626ca
pub fn i_0626be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x626CA; return 10; }
    8
}
/// $0626C2: neg.w d0
pub fn i_0626c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626C4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_0626c4  ($0626C4, 6 B, thunks/ccr_helpers.c)
/// $0626C4: ori.b #$11, ccr
pub fn i_0626c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626C8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0626C8: rts
pub fn i_0626c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0626ca  ($0626CA, 6 B, thunks/ccr_helpers.c)
/// $0626CA: andi.b #$ee, ccr
pub fn i_0626ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626CE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0626CE: rts
pub fn i_0626ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Facing_SignDelta77_0626d0  ($0626D0, 8 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0626D0: btst.b #$0, $77(a6)
pub fn i_0626d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626D6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0626D6: bra.b $626be
pub fn i_0626d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626D8;
    cpu.ir = 0x60E6;
    cpu.pc = 0x626BE;
    10
}
// ===== Facing_Matches77_0626d8  ($0626D8, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0626D8: move.b $3a(a6), d0
pub fn i_0626d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626DC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0626DC: cmp.b $77(a6), d0
pub fn i_0626dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626E0;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0626E0: bne.w $626ea
pub fn i_0626e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x626EA; return 10; }
    8
}
// ===== ClearXN_0626e4  ($0626E4, 6 B, thunks/ccr_helpers.c)
/// $0626E4: andi.b #$ee, ccr
pub fn i_0626e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626E8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0626E8: rts
pub fn i_0626e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0626ea  ($0626EA, 6 B, thunks/ccr_helpers.c)
/// $0626EA: ori.b #$11, ccr
pub fn i_0626ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626EE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0626EE: rts
pub fn i_0626ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_XPastThreshold_0626f0  ($0626F0, 32 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0626F0: move.w $22(a6), d1
pub fn i_0626f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626F4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0626F4: move.w #$88, d0
pub fn i_0626f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0626F8: jsr $626d0(pc)
pub fn i_0626f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x626FC);
    cpu.pc = 0x626D0;
    16
}
/// $0626FC: bcs.w $62708
pub fn i_0626fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62700;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x62708; return 10; }
    8
}
/// $062700: add.w $72(a6), d0
pub fn i_062700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62704;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $062704: cmp.w d1, d0
pub fn i_062704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62706;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $062706: rts
pub fn i_062706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62708;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $062708: add.w $72(a6), d0
pub fn i_062708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6270C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06270C: cmp.w d0, d1
pub fn i_06270c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6270E;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06270E: rts
pub fn i_06270e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62710;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_StepX_062710  ($062710, 22 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062710: subq.w #$1, $74(a6)
pub fn i_062710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62714;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062714: bgt.w $6272c
pub fn i_062714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62718;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6272C; return 10; }
    8
}
/// $062718: cmpi.w #$120, $22(a6)
pub fn i_062718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6271E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $06271E: bgt.w $6272c
pub fn i_06271e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62722;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6272C; return 10; }
    8
}
/// $062722: clr.w $74(a6)
pub fn i_062722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62726;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetXN_062726  ($062726, 6 B, thunks/ccr_helpers.c)
/// $062726: ori.b #$11, ccr
pub fn i_062726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6272A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06272A: rts
pub fn i_06272a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6272C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06272c  ($06272C, 6 B, thunks/ccr_helpers.c)
/// $06272C: andi.b #$ee, ccr
pub fn i_06272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62730;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $062730: rts
pub fn i_062730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62732;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_ClampX_062732  ($062732, 38 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062732: clr.l d0
pub fn i_062732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62734;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $062734: move.w $78(a6), d0
pub fn i_062734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62738;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $062738: addq.w #$1, $78(a6)
pub fn i_062738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6273C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06273C: lsr.w #$4, d0
pub fn i_06273c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6273E;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06273E: cmpi.w #$3, d0
pub fn i_06273e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62742;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $062742: ble.w $6274a
pub fn i_062742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62746;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6274A; return 10; }
    8
}
/// $062746: move.w #$3, d0
pub fn i_062746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6274A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06274A: lsl.w #$2, d0
pub fn i_06274a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6274C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06274C: movea.l $7c(a6), a0
pub fn i_06274c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62750;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $062750: move.l (a0, d0.w), $5c(a6)
pub fn i_062750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62756;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $062756: rts
pub fn i_062756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62758;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_StepAnim_062758  ($062758, 14 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062758: cmpi.w #$0, $80(a6)
pub fn i_062758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6275E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06275E: ble.w $6276c
pub fn i_06275e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62762;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6276C; return 10; }
    8
}
/// $062762: subq.w #$1, $80(a6)
pub fn i_062762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62766;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_062766  ($062766, 6 B, thunks/ccr_helpers.c)
/// $062766: andi.b #$ee, ccr
pub fn i_062766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6276A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06276A: rts
pub fn i_06276a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6276C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_06276c  ($06276C, 8 B, thunks/jsr_abs_thunks.c)
/// $06276C: jsr $5e5a8.l
pub fn i_06276c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62772);
    cpu.pc = 0x5E5A8;
    16
}
/// $062772: rts
pub fn i_062772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62774;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_Register_062774  ($062774, 78 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062774: move.w #$4000, d0
pub fn i_062774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62778;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062778: jsr $28134.l
pub fn i_062778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6277E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6277E);
    cpu.pc = 0x28134;
    16
}
/// $06277E: andi.w #$ffe3, $38(a6)
pub fn i_06277e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62784;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062784: ori.w #$14, $38(a6)
pub fn i_062784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6278A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06278A: tst.b $83(a6)
pub fn i_06278a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6278E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06278E: bne.w $627c8
pub fn i_06278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62792;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x627C8; return 10; }
    8
}
/// $062792: move.l #$ffffffff, $4c(a6)
pub fn i_062792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6279A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06279A: lea.l $2c379a.l, a1
pub fn i_06279a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C379A;
    20
}
/// $0627A0: tst.b $84(a6)
pub fn i_0627a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627A4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0627A4: beq.w $627ae
pub fn i_0627a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x627AE; return 10; }
    8
}
/// $0627A8: lea.l $2c37c2.l, a1
pub fn i_0627a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627AE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C37C2;
    20
}
/// $0627AE: jsr $43fac.l
pub fn i_0627ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x627B4);
    cpu.pc = 0x43FAC;
    16
}
/// $0627B4: move.l #$2c3650, $4c(a6)
pub fn i_0627b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627BC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C3650;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0627BC: jsr $283ca.l
pub fn i_0627bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x627C2);
    cpu.pc = 0x283CA;
    16
}
// ===== JsrAbsThunk_0627c2  ($0627C2, 8 B, thunks/jsr_abs_thunks.c)
/// $0627C2: jsr $283d8.l
pub fn i_0627c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x627C8);
    cpu.pc = 0x283D8;
    16
}
/// $0627C8: rts
pub fn i_0627c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_SpawnDebris2_0627ca  ($0627CA, 76 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0627CA: move.w #$1067, d0
pub fn i_0627ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1067;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0627CE: jsr $2352.l
pub fn i_0627ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x627D4);
    cpu.pc = 0x2352;
    16
}
/// $0627D4: lea.l $620b6(pc), a1
pub fn i_0627d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620B6;
    16
}
/// $0627D8: jsr $6fe.l
pub fn i_0627d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x627DE);
    cpu.pc = 0x6FE;
    16
}
/// $0627DE: jsr $5dd02.l
pub fn i_0627de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x627E4);
    cpu.pc = 0x5DD02;
    16
}
/// $0627E4: addi.w #$2c, $24(a0)
pub fn i_0627e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627EA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0627EA: move.w #$ffe2, d0
pub fn i_0627ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0627EE: btst.b #$0, $3a(a6)
pub fn i_0627ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627F4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0627F4: beq.w $627fa
pub fn i_0627f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x627FA; return 10; }
    8
}
/// $0627F8: neg.w d0
pub fn i_0627f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627FA;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0627FA: add.w d0, $22(a0)
pub fn i_0627fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x627FE;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0627FE: lea.l $620da(pc), a1
pub fn i_0627fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62802;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x620DA;
    16
}
/// $062802: jsr $6fe.l
pub fn i_062802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62808);
    cpu.pc = 0x6FE;
    16
}
/// $062808: jsr $5dd02.l
pub fn i_062808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6280E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6280E);
    cpu.pc = 0x5DD02;
    16
}
/// $06280E: addi.w #$a, $24(a0)
pub fn i_06280e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62814;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $062814: rts
pub fn i_062814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62816;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_RegisterB_062816  ($062816, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062816: move.w #$1033, d0
pub fn i_062816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6281A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06281A: jsr $2352.l
pub fn i_06281a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62820);
    cpu.pc = 0x2352;
    16
}
/// $062820: lea.l $2c3788.l, a1
pub fn i_062820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62826;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C3788;
    20
}
// ===== JsrAbsThunk_062826  ($062826, 8 B, thunks/jsr_abs_thunks.c)
/// $062826: jsr $77c7e.l
pub fn i_062826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6282C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6282C);
    cpu.pc = 0x77C7E;
    16
}
/// $06282C: rts
pub fn i_06282c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6282E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_SpawnPair_06282e  ($06282E, 68 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06282E: move.w #$1033, d0
pub fn i_06282e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62832;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062832: jsr $2352.l
pub fn i_062832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62838);
    cpu.pc = 0x2352;
    16
}
/// $062838: lea.l $77fd6.l, a1
pub fn i_062838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6283E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06283E: jsr $4ae.l
pub fn i_06283e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62844);
    cpu.pc = 0x4AE;
    16
}
/// $062844: jsr $5dd02.l
pub fn i_062844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6284A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6284A);
    cpu.pc = 0x5DD02;
    16
}
/// $06284A: move.w #$10, d0
pub fn i_06284a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6284E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06284E: movem.w d0, -(a7)
pub fn i_06284e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62852;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $062852: lea.l $62536(pc), a1
pub fn i_062852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62856;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62536;
    16
}
/// $062856: jsr $4ae.l
pub fn i_062856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6285C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6285C);
    cpu.pc = 0x4AE;
    16
}
/// $06285C: jsr $5dd02.l
pub fn i_06285c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62862);
    cpu.pc = 0x5DD02;
    16
}
/// $062862: addi.w #$18, $24(a0)
pub fn i_062862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62868;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $062868: movem.w (a7)+, d0
pub fn i_062868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6286C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06286C: subq.w #$1, d0
pub fn i_06286c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6286E;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06286E: bcc.b $6284e
pub fn i_06286e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62870;
    cpu.ir = 0x64DE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6284E; return 10; }
    8
}
/// $062870: rts
pub fn i_062870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62872;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_SpawnChild_062872  ($062872, 24 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062872: lea.l $6212a(pc), a1
pub fn i_062872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62876;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6212A;
    16
}
/// $062876: jsr $4ae.l
pub fn i_062876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6287C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6287C);
    cpu.pc = 0x4AE;
    16
}
/// $06287C: jsr $5dd02.l
pub fn i_06287c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62882);
    cpu.pc = 0x5DD02;
    16
}
/// $062882: move.b $9a(a6), $9a(a0)
pub fn i_062882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62888;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $062888: rts
pub fn i_062888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6288A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_SpawnChildB_06288a  ($06288A, 10 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06288A: lea.l $625da(pc), a1
pub fn i_06288a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6288E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x625DA;
    16
}
/// $06288E: jsr $4ae.l
pub fn i_06288e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62894);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_062894  ($062894, 8 B, thunks/jsr_abs_thunks.c)
/// $062894: jsr $5dd02.l
pub fn i_062894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6289A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6289A);
    cpu.pc = 0x5DD02;
    16
}
/// $06289A: rts
pub fn i_06289a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6289C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_Sound9C_06289c  ($06289C, 8 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06289C: move.b $9b(a6), d0
pub fn i_06289c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628A0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0628A0: move.w #$9c, d1
pub fn i_0628a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628A4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_0628a4  ($0628A4, 8 B, thunks/jsr_abs_thunks.c)
/// $0628A4: jsr $9a7aa.l
pub fn i_0628a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x628AA);
    cpu.pc = 0x9A7AA;
    16
}
/// $0628AA: rts
pub fn i_0628aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_SpawnPairB_0628ac  ($0628AC, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0628AC: move.w #$10aa, d0
pub fn i_0628ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0628B0: jsr $2352.l
pub fn i_0628b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x628B6);
    cpu.pc = 0x2352;
    16
}
/// $0628B6: lea.l $624d6(pc), a1
pub fn i_0628b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x624D6;
    16
}
/// $0628BA: jsr $4ae.l
pub fn i_0628ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x628C0);
    cpu.pc = 0x4AE;
    16
}
/// $0628C0: jsr $5dd02.l
pub fn i_0628c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x628C6);
    cpu.pc = 0x5DD02;
    16
}
/// $0628C6: lea.l $6235a(pc), a1
pub fn i_0628c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6235A;
    16
}
/// $0628CA: jsr $4ae.l
pub fn i_0628ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x628D0);
    cpu.pc = 0x4AE;
    16
}
/// $0628D0: jsr $5dd02.l
pub fn i_0628d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x628D6);
    cpu.pc = 0x5DD02;
    16
}
/// $0628D6: addi.w #$10, $24(a0)
pub fn i_0628d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628DC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0628DC: move.w #$10, d0
pub fn i_0628dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0628E0: jsr $626b8(pc)
pub fn i_0628e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x628E4);
    cpu.pc = 0x626B8;
    16
}
/// $0628E4: add.w d0, $22(a0)
pub fn i_0628e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628E8;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0628E8: rts
pub fn i_0628e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_SpawnChildC_0628ea  ($0628EA, 38 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0628EA: move.w #$10, d0
pub fn i_0628ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0628EE: move.w d0, -(a7)
pub fn i_0628ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628F0;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0628F0: lea.l $62588(pc), a1
pub fn i_0628f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62588;
    16
}
/// $0628F4: jsr $4ae.l
pub fn i_0628f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x628FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x628FA);
    cpu.pc = 0x4AE;
    16
}
/// $0628FA: jsr $5dd02.l
pub fn i_0628fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62900);
    cpu.pc = 0x5DD02;
    16
}
/// $062900: addq.w #$1, $38(a0)
pub fn i_062900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62904;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062904: move.w (a7)+, d0
pub fn i_062904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62906;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $062906: subq.w #$1, d0
pub fn i_062906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62908;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $062908: cmpi.w #$0, d0
pub fn i_062908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6290C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06290C: bgt.b $628ee
pub fn i_06290c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6290E;
    cpu.ir = 0x6EE0;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x628EE; return 10; }
    8
}
/// $06290E: rts
pub fn i_06290e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62910;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_SpawnChildD_062910  ($062910, 10 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062910: lea.l $6263a(pc), a1
pub fn i_062910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62914;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6263A;
    16
}
/// $062914: jsr $4ae.l
pub fn i_062914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6291A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6291A);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_06291a  ($06291A, 8 B, thunks/jsr_abs_thunks.c)
/// $06291A: jsr $5dd02.l
pub fn i_06291a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62920;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62920);
    cpu.pc = 0x5DD02;
    16
}
/// $062920: rts
pub fn i_062920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62922;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LateProp_Stub_062922  ($062922, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062922: movea.l $8(a6), a1
pub fn i_062922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62926;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $062926: move.b $10(a6), d0
pub fn i_062926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6292A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06292A: cmp.b $10(a1), d0
pub fn i_06292a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6292E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06292E: bcs.w $62938
pub fn i_06292e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62932;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x62938; return 10; }
    8
}
// ===== ClearXN_062932  ($062932, 6 B, thunks/ccr_helpers.c)
/// $062932: andi.b #$ee, ccr
pub fn i_062932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62936;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $062936: rts
pub fn i_062936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62938;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_062938  ($062938, 6 B, thunks/ccr_helpers.c)
/// $062938: ori.b #$11, ccr
pub fn i_062938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6293C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06293C: rts
pub fn i_06293c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6293E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MultiStage_Decal_06293e  ($06293E, 194 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06293E: lea.l $62a10(pc), a1
pub fn i_06293e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62A10;
    16
}
/// $062942: jsr $4ae.l
pub fn i_062942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62948;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62948);
    cpu.pc = 0x4AE;
    16
}
/// $062948: jsr $5dd02.l
pub fn i_062948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6294E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6294E);
    cpu.pc = 0x5DD02;
    16
}
/// $06294E: move.b #$1, $83(a0)
pub fn i_06294e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62954;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $062954: move.b #$1, $21(a6)
pub fn i_062954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6295A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06295A: move.b $98(a6), d0
pub fn i_06295a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6295E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06295E: andi.w #$3, d0
pub fn i_06295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62962;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $062962: lsl.w #$2, d0
pub fn i_062962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62964;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $062964: lea.l $2c4a34.l, a1
pub fn i_062964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6296A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C4A34;
    20
}
/// $06296A: movea.l (a1, d0.w), a0
pub fn i_06296a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6296E;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06296E: jsr $799de.l
pub fn i_06296e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62974);
    cpu.pc = 0x799DE;
    16
}
/// $062974: move.w d0, $86(a6)
pub fn i_062974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62978;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062978: move.w #$2d, $70(a6)
pub fn i_062978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6297E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06297E: lea.l $62984(pc), a1
pub fn i_06297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62982;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62984;
    16
}
/// $062982: move.l a1, (a6)
pub fn i_062982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62984;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062984: jsr $2783a.l
pub fn i_062984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6298A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6298A);
    cpu.pc = 0x2783A;
    16
}
/// $06298A: movea.l $c(a6), a0
pub fn i_06298a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6298E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06298E: move.b $20(a0), $20(a6)
pub fn i_06298e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62994;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $062994: tst.b $21(a6)
pub fn i_062994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62998;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $062998: bne.w $629e6
pub fn i_062998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6299C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x629E6; return 10; }
    8
}
/// $06299C: subq.w #$1, $70(a6)
pub fn i_06299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629A0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0629A0: cmpi.w #$0, $70(a6)
pub fn i_0629a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629A6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0629A6: bgt.w $629e6
pub fn i_0629a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629AA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x629E6; return 10; }
    8
}
/// $0629AA: subq.w #$1, $86(a6)
pub fn i_0629aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629AE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0629AE: cmpi.w #$0, $86(a6)
pub fn i_0629ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0629B4: blt.w $62a08
pub fn i_0629b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629B8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x62A08; return 10; }
    8
}
/// $0629B8: move.w #$2d, $70(a6)
pub fn i_0629b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0629BE: lea.l $62a10(pc), a1
pub fn i_0629be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62A10;
    16
}
/// $0629C2: jsr $4ae.l
pub fn i_0629c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x629C8);
    cpu.pc = 0x4AE;
    16
}
/// $0629C8: jsr $5dd02.l
pub fn i_0629c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x629CE);
    cpu.pc = 0x5DD02;
    16
}
/// $0629CE: clr.b $83(a0)
pub fn i_0629ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629D2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x83);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0629D2: movea.l $c(a6), a1
pub fn i_0629d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629D6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0629D6: cmpi.b #$ff, $20(a1)
pub fn i_0629d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629DC;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0629DC: beq.w $629e6
pub fn i_0629dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x629E6; return 10; }
    8
}
/// $0629E0: move.b #$1, $83(a0)
pub fn i_0629e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629E6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0629E6: clr.b $21(a6)
pub fn i_0629e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629EA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0629EA: movea.l #$ffffffff, a0
pub fn i_0629ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629F0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0629F0: lea.l $2c49c4.l, a0
pub fn i_0629f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49C4;
    20
}
/// $0629F6: jsr $5dd5c.l
pub fn i_0629f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x629FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x629FC);
    cpu.pc = 0x5DD5C;
    16
}
/// $0629FC: bcc.w $62a06
pub fn i_0629fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62A06; return 10; }
    8
}
// ===== SetTaskHandler_062a00  ($062A00, 8 B, kernel/task_handlers.c)
/// $062A00: lea.l $62f8c(pc), a1
pub fn i_062a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F8C;
    16
}
/// $062A04: move.l a1, (a6)
pub fn i_062a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062A06: rts
pub fn i_062a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_062a08  ($062A08, 8 B, kernel/jmp_to_scheduler.c)
/// $062A08: jmp $518.l
pub fn i_062a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A0E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $062A0E: rts
pub fn i_062a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MultiStage_DecalIdle_062a10  ($062A10, 54 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062A10: move.b #$0, $20(a6)
pub fn i_062a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $062A16: lea.l $62a1c(pc), a1
pub fn i_062a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62A1C;
    16
}
/// $062A1A: move.l a1, (a6)
pub fn i_062a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062A1C: movea.l $c(a6), a0
pub fn i_062a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A20;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $062A20: cmpi.b #$ff, $20(a0)
pub fn i_062a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A26;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $062A26: bne.w $62a2e
pub fn i_062a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x62A2E; return 10; }
    8
}
/// $062A2A: clr.b $83(a6)
pub fn i_062a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x83);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $062A2E: jsr $63336(pc)
pub fn i_062a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62A32);
    cpu.pc = 0x63336;
    16
}
/// $062A32: cmpi.w #$20, $22(a6)
pub fn i_062a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A38;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $062A38: ble.w $62f8c
pub fn i_062a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A3C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x62F8C; return 10; }
    8
}
/// $062A3C: cmpi.w #$110, $22(a6)
pub fn i_062a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A42;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $062A42: ble.w $62a4e
pub fn i_062a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A46;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x62A4E; return 10; }
    8
}
// ===== JsrAbsThunk_062a46  ($062A46, 8 B, thunks/jsr_abs_thunks.c)
/// $062A46: jsr $2783a.l
pub fn i_062a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62A4C);
    cpu.pc = 0x2783A;
    16
}
/// $062A4C: rts
pub fn i_062a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Init_062a4e  ($062A4E, 304 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062A4E: bset.b #$1, $12(a6)
pub fn i_062a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A54;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062A54: move.w #$6d, d1
pub fn i_062a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A58;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062A58: jsr $236e.l
pub fn i_062a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62A5E);
    cpu.pc = 0x236E;
    16
}
/// $062A5E: move.w #$5, $1c(a6)
pub fn i_062a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A64;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062A64: jsr $138fe.l
pub fn i_062a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62A6A);
    cpu.pc = 0x138FE;
    16
}
/// $062A6A: lea.l $2b80fc.l, a0
pub fn i_062a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B80FC;
    20
}
/// $062A70: jsr $799de.l
pub fn i_062a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62A76);
    cpu.pc = 0x799DE;
    16
}
/// $062A76: move.w d0, $66(a6)
pub fn i_062a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A7A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062A7A: lea.l $2b8200.l, a0
pub fn i_062a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8200;
    20
}
/// $062A80: jsr $799de.l
pub fn i_062a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62A86);
    cpu.pc = 0x799DE;
    16
}
/// $062A86: move.w d0, $72(a6)
pub fn i_062a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A8A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062A8A: jsr $5e9b6.l
pub fn i_062a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62A90);
    cpu.pc = 0x5E9B6;
    16
}
/// $062A90: andi.w #$1f, d0
pub fn i_062a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A94;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $062A94: add.w d0, $72(a6)
pub fn i_062a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A98;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062A98: jsr $5e9b6.l
pub fn i_062a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62A9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62A9E);
    cpu.pc = 0x5E9B6;
    16
}
/// $062A9E: andi.b #$1, d0
pub fn i_062a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AA2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $062AA2: addq.b #$2, d0
pub fn i_062aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AA4;
    cpu.ir = 0x5400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $062AA4: move.b d0, $82(a6)
pub fn i_062aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AA8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $062AA8: move.w #$100, $38(a6)
pub fn i_062aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AAE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062AAE: move.w #$2, $7c(a6)
pub fn i_062aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062AB4: move.w #$18, $28(a6)
pub fn i_062ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62ABA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062ABA: move.w #$120, $2a(a6)
pub fn i_062aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062AC0: move.w #$58, $70(a6)
pub fn i_062ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062AC6: movea.l $c(a6), a0
pub fn i_062ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62ACA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $062ACA: cmpi.b #$ff, $20(a0)
pub fn i_062aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AD0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $062AD0: bne.w $62ae6
pub fn i_062ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x62AE6; return 10; }
    8
}
/// $062AD4: move.w #$38, $28(a6)
pub fn i_062ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62ADA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062ADA: move.w #$180, $2a(a6)
pub fn i_062ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AE0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062AE0: move.w #$30, $70(a6)
pub fn i_062ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062AE6: clr.b $76(a6)
pub fn i_062ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AEA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $062AEA: clr.b $77(a6)
pub fn i_062aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AEE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $062AEE: lea.l $2c4a74.l, a0
pub fn i_062aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4A74;
    20
}
/// $062AF4: jsr $28cd4.l
pub fn i_062af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62AFA);
    cpu.pc = 0x28CD4;
    16
}
/// $062AFA: lea.l $62b00(pc), a1
pub fn i_062afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62AFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62B00;
    16
}
/// $062AFE: move.l a1, (a6)
pub fn i_062afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062B00: cmpi.w #$30, $70(a6)
pub fn i_062b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B06;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $062B06: bcc.w $62b1a
pub fn i_062b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62B1A; return 10; }
    8
}
/// $062B0A: move.w #$1071, d0
pub fn i_062b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1071;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062B0E: jsr $2352.l
pub fn i_062b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B14);
    cpu.pc = 0x2352;
    16
}
/// $062B14: lea.l $62b1a(pc), a1
pub fn i_062b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62B1A;
    16
}
/// $062B18: move.l a1, (a6)
pub fn i_062b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062B1A: move.b $106f28.l, d0
pub fn i_062b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B20;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $062B20: andi.b #$1, d0
pub fn i_062b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B24;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $062B24: bne.w $62b3e
pub fn i_062b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x62B3E; return 10; }
    8
}
/// $062B28: lea.l $6331a(pc), a1
pub fn i_062b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6331A;
    16
}
/// $062B2C: jsr $4ae.l
pub fn i_062b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B32);
    cpu.pc = 0x4AE;
    16
}
/// $062B32: jsr $5dd02.l
pub fn i_062b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B38);
    cpu.pc = 0x5DD02;
    16
}
/// $062B38: addi.w #$18, $24(a0)
pub fn i_062b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B3E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $062B3E: move.b $106f28.l, d0
pub fn i_062b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B44;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $062B44: andi.b #$3, d0
pub fn i_062b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B48;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $062B48: bne.w $62b5e
pub fn i_062b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x62B5E; return 10; }
    8
}
/// $062B4C: neg.w $2a(a6)
pub fn i_062b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B50;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062B50: jsr $27cee.l
pub fn i_062b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B56);
    cpu.pc = 0x27CEE;
    16
}
/// $062B56: neg.w $2a(a6)
pub fn i_062b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B5A;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062B5A: bra.w $62b64
pub fn i_062b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x62B64;
    10
}
/// $062B5E: jsr $27cee.l
pub fn i_062b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B64);
    cpu.pc = 0x27CEE;
    16
}
/// $062B64: jsr $28d70.l
pub fn i_062b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B6A);
    cpu.pc = 0x28D70;
    16
}
/// $062B6A: subq.w #$1, $70(a6)
pub fn i_062b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B6E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062B6E: cmpi.w #$0, $70(a6)
pub fn i_062b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B74;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $062B74: bgt.w $62b7e
pub fn i_062b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B78;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x62B7E; return 10; }
    8
}
/// $062B78: lea.l $62b84(pc), a1
pub fn i_062b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62B84;
    16
}
/// $062B7C: move.l a1, (a6)
pub fn i_062b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_062b7e  ($062B7E, 6 B, thunks/jsr_pc_thunks.c)
/// $062B7E: jsr $63336(pc)
pub fn i_062b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62B82);
    cpu.pc = 0x63336;
    16
}
/// $062B82: rts
pub fn i_062b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Idle_062b84  ($062B84, 68 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062B84: move.w #$1071, d0
pub fn i_062b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B88;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1071;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062B88: jsr $2222.l
pub fn i_062b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B8E);
    cpu.pc = 0x2222;
    16
}
/// $062B8E: move.l #$2c459c, $48(a6)
pub fn i_062b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B96;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C459C;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $062B96: jsr $267e2.l
pub fn i_062b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62B9C);
    cpu.pc = 0x267E2;
    16
}
/// $062B9C: lea.l $2c4aa2.l, a0
pub fn i_062b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BA2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4AA2;
    20
}
/// $062BA2: jsr $28cd4.l
pub fn i_062ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62BA8);
    cpu.pc = 0x28CD4;
    16
}
/// $062BA8: lea.l $62bae(pc), a1
pub fn i_062ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62BAE;
    16
}
/// $062BAC: move.l a1, (a6)
pub fn i_062bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062BAE: jsr $2783a.l
pub fn i_062bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62BB4);
    cpu.pc = 0x2783A;
    16
}
/// $062BB4: jsr $28d70.l
pub fn i_062bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62BBA);
    cpu.pc = 0x28D70;
    16
}
/// $062BBA: bcc.w $62bc4
pub fn i_062bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62BC4; return 10; }
    8
}
/// $062BBE: lea.l $62bc8(pc), a1
pub fn i_062bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62BC8;
    16
}
/// $062BC2: move.l a1, (a6)
pub fn i_062bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062BC4: bra.w $62ee0
pub fn i_062bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x62EE0;
    10
}
// ===== Camper_Watch_062bc8  ($062BC8, 110 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062BC8: andi.w #$3, $7c(a6)
pub fn i_062bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BCE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062BCE: lea.l $2c4b38.l, a0
pub fn i_062bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4B38;
    20
}
/// $062BD4: jsr $28cd4.l
pub fn i_062bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62BDA);
    cpu.pc = 0x28CD4;
    16
}
/// $062BDA: lea.l $62be0(pc), a1
pub fn i_062bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62BE0;
    16
}
/// $062BDE: move.l a1, (a6)
pub fn i_062bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062BE0: jsr $2783a.l
pub fn i_062be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62BE6);
    cpu.pc = 0x2783A;
    16
}
/// $062BE6: jsr $28d70.l
pub fn i_062be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62BEC);
    cpu.pc = 0x28D70;
    16
}
/// $062BEC: subq.w #$1, $72(a6)
pub fn i_062bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BF0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062BF0: cmpi.w #$0, $72(a6)
pub fn i_062bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BF6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $062BF6: bgt.w $62c32
pub fn i_062bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BFA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x62C32; return 10; }
    8
}
/// $062BFA: jsr $63396(pc)
pub fn i_062bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62BFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62BFE);
    cpu.pc = 0x63396;
    16
}
/// $062BFE: bcs.w $62c32
pub fn i_062bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C02;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x62C32; return 10; }
    8
}
/// $062C02: move.w #$1, d1
pub fn i_062c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C06;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062C06: cmp.w $7c(a6), d0
pub fn i_062c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C0A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $062C0A: beq.w $62c24
pub fn i_062c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62C24; return 10; }
    8
}
/// $062C0E: bgt.w $62c16
pub fn i_062c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C12;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x62C16; return 10; }
    8
}
/// $062C12: move.w #$ffff, d1
pub fn i_062c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C16;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062C16: move.w d1, $80(a6)
pub fn i_062c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C1A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062C1A: lea.l $62c36(pc), a1
pub fn i_062c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62C36;
    16
}
/// $062C1E: move.l a1, (a6)
pub fn i_062c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062C20: bra.w $62c32
pub fn i_062c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C24;
    cpu.ir = 0x6000;
    cpu.pc = 0x62C32;
    10
}
/// $062C24: jsr $6340e(pc)
pub fn i_062c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62C28);
    cpu.pc = 0x6340E;
    16
}
/// $062C28: bcs.w $62c32
pub fn i_062c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C2C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x62C32; return 10; }
    8
}
/// $062C2C: lea.l $62cb4(pc), a1
pub fn i_062c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62CB4;
    16
}
/// $062C30: move.l a1, (a6)
pub fn i_062c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062C32: bra.w $62ee0
pub fn i_062c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C36;
    cpu.ir = 0x6000;
    cpu.pc = 0x62EE0;
    10
}
// ===== Camper_Alert_062c36  ($062C36, 126 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062C36: move.w $7c(a6), d0
pub fn i_062c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C3A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $062C3A: lsl.w #$1, d0
pub fn i_062c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C3C;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $062C3C: move.w d0, $34(a6)
pub fn i_062c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C40;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062C40: clr.w $70(a6)
pub fn i_062c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C44;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $062C44: clr.b $78(a6)
pub fn i_062c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C48;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $062C48: andi.w #$7, $34(a6)
pub fn i_062c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C4E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062C4E: lea.l $2c4bd6.l, a0
pub fn i_062c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4BD6;
    20
}
/// $062C54: jsr $28cd4.l
pub fn i_062c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62C5A);
    cpu.pc = 0x28CD4;
    16
}
/// $062C5A: lea.l $62c60(pc), a1
pub fn i_062c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62C60;
    16
}
/// $062C5E: move.l a1, (a6)
pub fn i_062c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062C60: jsr $2783a.l
pub fn i_062c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62C66);
    cpu.pc = 0x2783A;
    16
}
/// $062C66: subq.w #$1, $70(a6)
pub fn i_062c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C6A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062C6A: cmpi.w #$0, $70(a6)
pub fn i_062c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C70;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $062C70: bgt.w $62caa
pub fn i_062c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C74;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x62CAA; return 10; }
    8
}
/// $062C74: addq.b #$1, $78(a6)
pub fn i_062c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C78;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $062C78: cmpi.b #$3, $78(a6)
pub fn i_062c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C7E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $062C7E: beq.w $62bc8
pub fn i_062c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62BC8; return 10; }
    8
}
/// $062C82: lea.l $2b8386.l, a0
pub fn i_062c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8386;
    20
}
/// $062C88: jsr $799de.l
pub fn i_062c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62C8E);
    cpu.pc = 0x799DE;
    16
}
/// $062C8E: move.w d0, $70(a6)
pub fn i_062c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C92;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062C92: move.w $80(a6), d0
pub fn i_062c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $062C96: move.w $34(a6), d1
pub fn i_062c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C9A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $062C9A: add.w d1, d0
pub fn i_062c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C9C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $062C9C: andi.w #$7, d0
pub fn i_062c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CA0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $062CA0: move.w d0, $34(a6)
pub fn i_062ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CA4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062CA4: lsr.w #$1, d0
pub fn i_062ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CA6;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $062CA6: move.w d0, $7c(a6)
pub fn i_062ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CAA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062CAA: jsr $28d70.l
pub fn i_062caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62CB0);
    cpu.pc = 0x28D70;
    16
}
/// $062CB0: bra.w $62ee0
pub fn i_062cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x62EE0;
    10
}
// ===== Camper_Rearm_062cb4  ($062CB4, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062CB4: lea.l $2b817e.l, a0
pub fn i_062cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B817E;
    20
}
/// $062CBA: jsr $799de.l
pub fn i_062cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62CC0);
    cpu.pc = 0x799DE;
    16
}
/// $062CC0: move.w d0, $74(a6)
pub fn i_062cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CC4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Camper_Fire_062cc4  ($062CC4, 174 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062CC4: jsr $6340e(pc)
pub fn i_062cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62CC8);
    cpu.pc = 0x6340E;
    16
}
/// $062CC8: bcs.w $62bc8
pub fn i_062cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CCC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x62BC8; return 10; }
    8
}
/// $062CCC: lea.l $2b8282.l, a0
pub fn i_062ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8282;
    20
}
/// $062CD2: jsr $799de.l
pub fn i_062cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62CD8);
    cpu.pc = 0x799DE;
    16
}
/// $062CD8: move.w d0, $72(a6)
pub fn i_062cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CDC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062CDC: andi.w #$3, $7c(a6)
pub fn i_062cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CE2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062CE2: move.w $7c(a6), d0
pub fn i_062ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CE6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $062CE6: lea.l $2c49ec.l, a0
pub fn i_062ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49EC;
    20
}
/// $062CEC: cmpi.w #$1, $74(a6)
pub fn i_062cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CF2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $062CF2: blt.w $62d06
pub fn i_062cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CF6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x62D06; return 10; }
    8
}
/// $062CF6: cmpi.w #$2d, $72(a6)
pub fn i_062cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62CFC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2D, Size::Word, false);
    16
}
/// $062CFC: bge.w $62d06
pub fn i_062cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D00;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x62D06; return 10; }
    8
}
/// $062D00: lea.l $2c49fc.l, a0
pub fn i_062d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49FC;
    20
}
/// $062D06: lsl.w #$2, d0
pub fn i_062d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D08;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $062D08: movea.l (a0, d0.w), a0
pub fn i_062d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D0C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $062D0C: cmpa.l #$ffffffff, a0
pub fn i_062d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D12;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $062D12: beq.w $62d1c
pub fn i_062d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62D1C; return 10; }
    8
}
/// $062D16: jsr $28cd4.l
pub fn i_062d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D1C);
    cpu.pc = 0x28CD4;
    16
}
/// $062D1C: lea.l $62d22(pc), a1
pub fn i_062d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62D22;
    16
}
/// $062D20: move.l a1, (a6)
pub fn i_062d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062D22: jsr $2783a.l
pub fn i_062d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D28);
    cpu.pc = 0x2783A;
    16
}
/// $062D28: jsr $633e0(pc)
pub fn i_062d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62D2C);
    cpu.pc = 0x633E0;
    16
}
/// $062D2C: jsr $28d70.l
pub fn i_062d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D32);
    cpu.pc = 0x28D70;
    16
}
/// $062D32: bcc.w $62d6e
pub fn i_062d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62D6E; return 10; }
    8
}
/// $062D36: lea.l $62cc4(pc), a1
pub fn i_062d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62CC4;
    16
}
/// $062D3A: move.l a1, (a6)
pub fn i_062d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D3C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062D3C: subq.w #$1, $74(a6)
pub fn i_062d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D40;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062D40: cmpi.w #$0, $74(a6)
pub fn i_062d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D46;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $062D46: bgt.w $62d6e
pub fn i_062d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D4A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x62D6E; return 10; }
    8
}
/// $062D4A: lea.l $2b8200.l, a0
pub fn i_062d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8200;
    20
}
/// $062D50: jsr $799de.l
pub fn i_062d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D56);
    cpu.pc = 0x799DE;
    16
}
/// $062D56: move.w d0, $72(a6)
pub fn i_062d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D5A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062D5A: jsr $5e9b6.l
pub fn i_062d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D60);
    cpu.pc = 0x5E9B6;
    16
}
/// $062D60: andi.w #$1f, d0
pub fn i_062d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D64;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $062D64: add.w d0, $72(a6)
pub fn i_062d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D68;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062D68: lea.l $62bc8(pc), a1
pub fn i_062d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62BC8;
    16
}
/// $062D6C: move.l a1, (a6)
pub fn i_062d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062D6E: bra.w $62ee0
pub fn i_062d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D72;
    cpu.ir = 0x6000;
    cpu.pc = 0x62EE0;
    10
}
// ===== Camper_Die_062d72  ($062D72, 82 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062D72: move.w #$1020, d0
pub fn i_062d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062D76: jsr $2352.l
pub fn i_062d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D7C);
    cpu.pc = 0x2352;
    16
}
/// $062D7C: jsr $267e2.l
pub fn i_062d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D82);
    cpu.pc = 0x267E2;
    16
}
/// $062D82: lea.l $2c50f0.l, a0
pub fn i_062d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C50F0;
    20
}
/// $062D88: jsr $28cd4.l
pub fn i_062d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D8E);
    cpu.pc = 0x28CD4;
    16
}
/// $062D8E: lea.l $62d94(pc), a1
pub fn i_062d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62D94;
    16
}
/// $062D92: move.l a1, (a6)
pub fn i_062d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062D94: jsr $63336(pc)
pub fn i_062d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62D98);
    cpu.pc = 0x63336;
    16
}
/// $062D98: jsr $2783a.l
pub fn i_062d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62D9E);
    cpu.pc = 0x2783A;
    16
}
/// $062D9E: jsr $28d70.l
pub fn i_062d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62DA4);
    cpu.pc = 0x28D70;
    16
}
/// $062DA4: bcc.w $62dae
pub fn i_062da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62DAE; return 10; }
    8
}
/// $062DA8: lea.l $62dcc(pc), a1
pub fn i_062da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62DCC;
    16
}
/// $062DAC: move.l a1, (a6)
pub fn i_062dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062DAE: movea.l #$ffffffff, a0
pub fn i_062dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DB4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062DB4: lea.l $2c49c4.l, a0
pub fn i_062db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49C4;
    20
}
/// $062DBA: jsr $5dd56.l
pub fn i_062dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62DC0);
    cpu.pc = 0x5DD56;
    16
}
/// $062DC0: bcc.w $62dca
pub fn i_062dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62DCA; return 10; }
    8
}
// ===== SetTaskHandler_062dc4  ($062DC4, 8 B, kernel/task_handlers.c)
/// $062DC4: lea.l $62f8c(pc), a1
pub fn i_062dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F8C;
    16
}
/// $062DC8: move.l a1, (a6)
pub fn i_062dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062DCA: rts
pub fn i_062dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DCC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Bullet_062dcc  ($062DCC, 112 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062DCC: move.w #$8000, $38(a6)
pub fn i_062dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062DD2: jsr $267e2.l
pub fn i_062dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62DD8);
    cpu.pc = 0x267E2;
    16
}
/// $062DD8: subi.w #$20, $24(a6)
pub fn i_062dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DDE;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $062DDE: lea.l $2c5162.l, a0
pub fn i_062dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5162;
    20
}
/// $062DE4: jsr $28cd4.l
pub fn i_062de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62DEA);
    cpu.pc = 0x28CD4;
    16
}
/// $062DEA: lea.l $62df0(pc), a1
pub fn i_062dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62DF0;
    16
}
/// $062DEE: move.l a1, (a6)
pub fn i_062dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062DF0: jsr $63336(pc)
pub fn i_062df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62DF4);
    cpu.pc = 0x63336;
    16
}
/// $062DF4: jsr $27bc8.l
pub fn i_062df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62DFA);
    cpu.pc = 0x27BC8;
    16
}
/// $062DFA: bcc.w $62e04
pub fn i_062dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62DFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62E04; return 10; }
    8
}
/// $062DFE: lea.l $62e44(pc), a1
pub fn i_062dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62E44;
    16
}
/// $062E02: move.l a1, (a6)
pub fn i_062e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062E04: jsr $28d70.l
pub fn i_062e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E0A);
    cpu.pc = 0x28D70;
    16
}
/// $062E0A: jsr $283ca.l
pub fn i_062e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E10);
    cpu.pc = 0x283CA;
    16
}
/// $062E10: jsr $283d8.l
pub fn i_062e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E16);
    cpu.pc = 0x283D8;
    16
}
/// $062E16: btst.b #$1, $13(a6)
pub fn i_062e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E1C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $062E1C: beq.w $62e26
pub fn i_062e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E20;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62E26; return 10; }
    8
}
/// $062E20: lea.l $62e8a(pc), a1
pub fn i_062e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62E8A;
    16
}
/// $062E24: move.l a1, (a6)
pub fn i_062e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062E26: movea.l #$ffffffff, a0
pub fn i_062e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E2C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062E2C: lea.l $2c49c4.l, a0
pub fn i_062e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49C4;
    20
}
/// $062E32: jsr $5dd56.l
pub fn i_062e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E38);
    cpu.pc = 0x5DD56;
    16
}
/// $062E38: bcc.w $62e42
pub fn i_062e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62E42; return 10; }
    8
}
// ===== SetTaskHandler_062e3c  ($062E3C, 8 B, kernel/task_handlers.c)
/// $062E3C: lea.l $62f8c(pc), a1
pub fn i_062e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F8C;
    16
}
/// $062E40: move.l a1, (a6)
pub fn i_062e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062E42: rts
pub fn i_062e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Flash_062e44  ($062E44, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062E44: lea.l $2c51ca.l, a0
pub fn i_062e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E4A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C51CA;
    20
}
/// $062E4A: jsr $28cd4.l
pub fn i_062e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E50);
    cpu.pc = 0x28CD4;
    16
}
/// $062E50: lea.l $62e56(pc), a1
pub fn i_062e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62E56;
    16
}
/// $062E54: move.l a1, (a6)
pub fn i_062e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062E56: jsr $2783a.l
pub fn i_062e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E5C);
    cpu.pc = 0x2783A;
    16
}
/// $062E5C: jsr $28d70.l
pub fn i_062e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E62);
    cpu.pc = 0x28D70;
    16
}
/// $062E62: bcc.w $62e6c
pub fn i_062e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62E6C; return 10; }
    8
}
/// $062E66: bclr.b #$1, $12(a6)
pub fn i_062e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E6C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062E6C: movea.l #$ffffffff, a0
pub fn i_062e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E72;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062E72: lea.l $2c49c4.l, a0
pub fn i_062e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49C4;
    20
}
/// $062E78: jsr $5dd56.l
pub fn i_062e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E7E);
    cpu.pc = 0x5DD56;
    16
}
/// $062E7E: bcc.w $62e88
pub fn i_062e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62E88; return 10; }
    8
}
// ===== SetTaskHandler_062e82  ($062E82, 8 B, kernel/task_handlers.c)
/// $062E82: lea.l $62f8c(pc), a1
pub fn i_062e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F8C;
    16
}
/// $062E86: move.l a1, (a6)
pub fn i_062e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062E88: rts
pub fn i_062e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Smoke_062e8a  ($062E8A, 78 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062E8A: bclr.b #$1, $12(a6)
pub fn i_062e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062E90: jsr $267e2.l
pub fn i_062e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62E96);
    cpu.pc = 0x267E2;
    16
}
/// $062E96: lea.l $2c5236.l, a0
pub fn i_062e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62E9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5236;
    20
}
/// $062E9C: jsr $28cd4.l
pub fn i_062e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62EA2);
    cpu.pc = 0x28CD4;
    16
}
/// $062EA2: lea.l $62ea8(pc), a1
pub fn i_062ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62EA8;
    16
}
/// $062EA6: move.l a1, (a6)
pub fn i_062ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062EA8: jsr $63336(pc)
pub fn i_062ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62EAC);
    cpu.pc = 0x63336;
    16
}
/// $062EAC: jsr $2783a.l
pub fn i_062eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62EB2);
    cpu.pc = 0x2783A;
    16
}
/// $062EB2: jsr $28d70.l
pub fn i_062eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62EB8);
    cpu.pc = 0x28D70;
    16
}
/// $062EB8: bcc.w $62ec2
pub fn i_062eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62EC2; return 10; }
    8
}
/// $062EBC: lea.l $62f8c(pc), a1
pub fn i_062ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F8C;
    16
}
/// $062EC0: move.l a1, (a6)
pub fn i_062ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062EC2: movea.l #$ffffffff, a0
pub fn i_062ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EC8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062EC8: lea.l $2c49c4.l, a0
pub fn i_062ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62ECE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49C4;
    20
}
/// $062ECE: jsr $5dd56.l
pub fn i_062ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62ED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62ED4);
    cpu.pc = 0x5DD56;
    16
}
/// $062ED4: bcc.w $62ede
pub fn i_062ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62ED8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62EDE; return 10; }
    8
}
// ===== SetTaskHandler_062ed8  ($062ED8, 8 B, kernel/task_handlers.c)
/// $062ED8: lea.l $62f8c(pc), a1
pub fn i_062ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F8C;
    16
}
/// $062EDC: move.l a1, (a6)
pub fn i_062edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062EDE: rts
pub fn i_062ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_HitCheck_062ee0  ($062EE0, 164 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062EE0: jsr $63336(pc)
pub fn i_062ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62EE4);
    cpu.pc = 0x63336;
    16
}
/// $062EE4: lea.l $5e766.l, a0
pub fn i_062ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $062EEA: jsr $5e770.l
pub fn i_062eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62EF0);
    cpu.pc = 0x5E770;
    16
}
/// $062EF0: jsr $2870a.l
pub fn i_062ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62EF6);
    cpu.pc = 0x2870A;
    16
}
/// $062EF6: bcc.w $62f06
pub fn i_062ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62EFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62F06; return 10; }
    8
}
/// $062EFA: bclr.b #$3, $13(a6)
pub fn i_062efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F00;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062F00: move.b #$a, $76(a6)
pub fn i_062f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $062F06: subq.b #$1, $76(a6)
pub fn i_062f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F0A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $062F0A: cmpi.b #$0, $76(a6)
pub fn i_062f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F10;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $062F10: bgt.w $62f1a
pub fn i_062f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F14;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x62F1A; return 10; }
    8
}
/// $062F14: move.b #$0, $76(a6)
pub fn i_062f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F1A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $062F1A: cmpi.b #$0, $76(a6)
pub fn i_062f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F20;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $062F20: beq.w $62f40
pub fn i_062f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62F40; return 10; }
    8
}
/// $062F24: addq.b #$1, $77(a6)
pub fn i_062f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F28;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $062F28: move.b $77(a6), d0
pub fn i_062f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F2C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $062F2C: andi.w #$3, d0
pub fn i_062f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F30;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $062F30: add.w d0, d0
pub fn i_062f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F32;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $062F32: lea.l $2c49d4.l, a0
pub fn i_062f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49D4;
    20
}
/// $062F38: move.w (a0, d0.w), d0
pub fn i_062f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F3C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $062F3C: add.w d0, $24(a6)
pub fn i_062f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F40;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $062F40: cmpi.b #$0, $83(a6)
pub fn i_062f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F46;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $062F46: beq.w $62f5e
pub fn i_062f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x62F5E; return 10; }
    8
}
/// $062F4A: movea.l $c(a6), a0
pub fn i_062f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F4E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $062F4E: cmpi.b #$ff, $20(a0)
pub fn i_062f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F54;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $062F54: bne.w $62f5e
pub fn i_062f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F58;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x62F5E; return 10; }
    8
}
/// $062F58: lea.l $62d72(pc), a1
pub fn i_062f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62D72;
    16
}
/// $062F5C: move.l a1, (a6)
pub fn i_062f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062F5E: jsr $28758.l
pub fn i_062f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62F64);
    cpu.pc = 0x28758;
    16
}
/// $062F64: bcc.w $62f6e
pub fn i_062f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62F6E; return 10; }
    8
}
/// $062F68: lea.l $62e8a(pc), a1
pub fn i_062f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62E8A;
    16
}
/// $062F6C: move.l a1, (a6)
pub fn i_062f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062F6E: movea.l #$ffffffff, a0
pub fn i_062f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F74;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $062F74: lea.l $2c49c4.l, a0
pub fn i_062f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49C4;
    20
}
/// $062F7A: jsr $5dd56.l
pub fn i_062f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62F80);
    cpu.pc = 0x5DD56;
    16
}
/// $062F80: bcc.w $62f8a
pub fn i_062f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x62F8A; return 10; }
    8
}
// ===== SetTaskHandler_062f84  ($062F84, 8 B, kernel/task_handlers.c)
/// $062F84: lea.l $62f8c(pc), a1
pub fn i_062f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F8C;
    16
}
/// $062F88: move.l a1, (a6)
pub fn i_062f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062F8A: rts
pub fn i_062f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_062f8c  ($062F8C, 14 B, kernel/jsr5b6_jmp518.c)
/// $062F8C: jsr $5b6.l
pub fn i_062f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62F92);
    cpu.pc = 0x5B6;
    16
}
/// $062F92: jmp $518.l
pub fn i_062f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F98;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $062F98: rts
pub fn i_062f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62F9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_062f9a  ($062F9A, 14 B, kernel/jsr5b6_jmp518.c)
/// $062F9A: jsr $5b6.l
pub fn i_062f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62FA0);
    cpu.pc = 0x5B6;
    16
}
/// $062FA0: jmp $518.l
pub fn i_062fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FA6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $062FA6: rts
pub fn i_062fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Explode_062fa8  ($062FA8, 36 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062FA8: move.w #$4000, d0
pub fn i_062fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FAC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062FAC: jsr $28134.l
pub fn i_062fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62FB2);
    cpu.pc = 0x28134;
    16
}
/// $062FB2: andi.w #$ffe3, $38(a6)
pub fn i_062fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FB8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062FB8: ori.w #$1c, $38(a6)
pub fn i_062fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FBE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $062FBE: move.l #$ffffffff, $48(a6)
pub fn i_062fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FC6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $062FC6: jmp $77f6a.l
pub fn i_062fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Camper_Grenade_062fcc  ($062FCC, 116 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $062FCC: bset.b #$4, $6b(a6)
pub fn i_062fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FD2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $062FD2: move.w #$58, d1
pub fn i_062fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FD6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $062FD6: jsr $236e.l
pub fn i_062fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62FDC);
    cpu.pc = 0x236E;
    16
}
/// $062FDC: move.w #$d000, $38(a6)
pub fn i_062fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $062FE2: jsr $63342(pc)
pub fn i_062fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x62FE6);
    cpu.pc = 0x63342;
    16
}
/// $062FE6: lea.l $2c5248.l, a0
pub fn i_062fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5248;
    20
}
/// $062FEC: jsr $28cd4.l
pub fn i_062fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62FF2);
    cpu.pc = 0x28CD4;
    16
}
/// $062FF2: lea.l $62ff8(pc), a1
pub fn i_062ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62FF8;
    16
}
/// $062FF6: move.l a1, (a6)
pub fn i_062ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $062FF8: jsr $27d50.l
pub fn i_062ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x62FFE);
    cpu.pc = 0x27D50;
    16
}
/// $062FFE: bcc.w $63008
pub fn i_062ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63002;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63008; return 10; }
    8
}
/// $063002: lea.l $63048(pc), a1
pub fn i_063002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63048;
    16
}
/// $063006: move.l a1, (a6)
pub fn i_063006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63008;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063008: jsr $28d70.l
pub fn i_063008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6300E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6300E);
    cpu.pc = 0x28D70;
    16
}
/// $06300E: jsr $283ca.l
pub fn i_06300e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63014);
    cpu.pc = 0x283CA;
    16
}
/// $063014: jsr $283d8.l
pub fn i_063014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6301A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6301A);
    cpu.pc = 0x283D8;
    16
}
/// $06301A: btst.b #$1, $13(a6)
pub fn i_06301a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63020;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063020: beq.w $6302a
pub fn i_063020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63024;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6302A; return 10; }
    8
}
/// $063024: lea.l $62fa8(pc), a1
pub fn i_063024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63028;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62FA8;
    16
}
/// $063028: move.l a1, (a6)
pub fn i_063028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6302A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06302A: movea.l #$ffffffff, a0
pub fn i_06302a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63030;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $063030: lea.l $2c49cc.l, a0
pub fn i_063030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63036;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C49CC;
    20
}
/// $063036: jsr $5dd56.l
pub fn i_063036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6303C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6303C);
    cpu.pc = 0x5DD56;
    16
}
/// $06303C: bcc.w $63046
pub fn i_06303c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63040;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63046; return 10; }
    8
}
// ===== SetTaskHandler_063040  ($063040, 8 B, kernel/task_handlers.c)
/// $063040: lea.l $62f9a(pc), a1
pub fn i_063040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63044;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $063044: move.l a1, (a6)
pub fn i_063044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63046;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063046: rts
pub fn i_063046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63048;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Music_063048  ($063048, 14 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063048: move.w #$1022, d0
pub fn i_063048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6304C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06304C: jsr $2352.l
pub fn i_06304c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63052);
    cpu.pc = 0x2352;
    16
}
/// $063052: bra.w $62fa8
pub fn i_063052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63056;
    cpu.ir = 0x6000;
    cpu.pc = 0x62FA8;
    10
}
// ===== Camper_Burst_063056  ($063056, 94 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063056: move.w #$4000, d0
pub fn i_063056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6305A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06305A: jsr $28134.l
pub fn i_06305a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63060;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63060);
    cpu.pc = 0x28134;
    16
}
/// $063060: andi.w #$ffe3, $38(a6)
pub fn i_063060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63066;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063066: ori.w #$0, $38(a6)
pub fn i_063066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6306C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06306C: eori.b #$1, $3a(a6)
pub fn i_06306c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63072;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $063072: move.w #$8, d1
pub fn i_063072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63076;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063076: jsr $236e.l
pub fn i_063076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6307C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6307C);
    cpu.pc = 0x236E;
    16
}
/// $06307C: lea.l $2c9c60.l, a0
pub fn i_06307c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63082;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9C60;
    20
}
/// $063082: jsr $28cd4.l
pub fn i_063082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63088);
    cpu.pc = 0x28CD4;
    16
}
/// $063088: lea.l $6308e(pc), a1
pub fn i_063088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6308C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6308E;
    16
}
/// $06308C: move.l a1, (a6)
pub fn i_06308c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6308E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06308E: jsr $2783a.l
pub fn i_06308e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63094);
    cpu.pc = 0x2783A;
    16
}
/// $063094: jsr $28d70.l
pub fn i_063094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6309A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6309A);
    cpu.pc = 0x28D70;
    16
}
/// $06309A: bcc.w $630a4
pub fn i_06309a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6309E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x630A4; return 10; }
    8
}
/// $06309E: lea.l $62f9a(pc), a1
pub fn i_06309e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $0630A2: move.l a1, (a6)
pub fn i_0630a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0630A4: movea.l #$ffffffff, a0
pub fn i_0630a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630AA;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0630AA: jsr $5dd56.l
pub fn i_0630aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x630B0);
    cpu.pc = 0x5DD56;
    16
}
/// $0630B0: bcc.w $630ba
pub fn i_0630b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x630BA; return 10; }
    8
}
// ===== SetTaskHandler_0630b4  ($0630B4, 8 B, kernel/task_handlers.c)
/// $0630B4: lea.l $62f9a(pc), a1
pub fn i_0630b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $0630B8: move.l a1, (a6)
pub fn i_0630b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0630BA: rts
pub fn i_0630ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Spark_0630bc  ($0630BC, 66 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0630BC: move.w #$1d, d1
pub fn i_0630bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0630C0: jsr $236e.l
pub fn i_0630c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x630C6);
    cpu.pc = 0x236E;
    16
}
/// $0630C6: lea.l $2e2286.l, a0
pub fn i_0630c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2286;
    20
}
/// $0630CC: jsr $28cd4.l
pub fn i_0630cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x630D2);
    cpu.pc = 0x28CD4;
    16
}
/// $0630D2: lea.l $630d8(pc), a1
pub fn i_0630d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x630D8;
    16
}
/// $0630D6: move.l a1, (a6)
pub fn i_0630d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0630D8: jsr $2783a.l
pub fn i_0630d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x630DE);
    cpu.pc = 0x2783A;
    16
}
/// $0630DE: jsr $28d70.l
pub fn i_0630de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x630E4);
    cpu.pc = 0x28D70;
    16
}
/// $0630E4: bcc.w $630ee
pub fn i_0630e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x630EE; return 10; }
    8
}
/// $0630E8: lea.l $63106(pc), a1
pub fn i_0630e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63106;
    16
}
/// $0630EC: move.l a1, (a6)
pub fn i_0630ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0630EE: movea.l #$ffffffff, a0
pub fn i_0630ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630F4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0630F4: jsr $5dd56.l
pub fn i_0630f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x630FA);
    cpu.pc = 0x5DD56;
    16
}
/// $0630FA: bcc.w $63104
pub fn i_0630fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63104; return 10; }
    8
}
// ===== SetTaskHandler_0630fe  ($0630FE, 8 B, kernel/task_handlers.c)
/// $0630FE: lea.l $62f9a(pc), a1
pub fn i_0630fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63102;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $063102: move.l a1, (a6)
pub fn i_063102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63104;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063104: rts
pub fn i_063104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63106;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_DropItem_063106  ($063106, 46 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063106: jsr $5e9b6.l
pub fn i_063106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6310C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6310C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06310C: lea.l $2c4a44.l, a1
pub fn i_06310c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63112;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C4A44;
    20
}
/// $063112: andi.w #$3, d1
pub fn i_063112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63116;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063116: lsl.w #$2, d1
pub fn i_063116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63118;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $063118: movem.l d1/a1, -(a7)
pub fn i_063118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6311C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $06311C: lea.l $62f9a(pc), a1
pub fn i_06311c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63120;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $063120: jsr $4498e.l
pub fn i_063120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63126);
    cpu.pc = 0x4498E;
    16
}
/// $063126: jsr $5dd02.l
pub fn i_063126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6312C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6312C);
    cpu.pc = 0x5DD02;
    16
}
/// $06312C: movem.l (a7)+, d1/a1
pub fn i_06312c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63130;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $063130: move.l (a1, d1.w), (a0)
pub fn i_063130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63134;
    cpu.ir = 0x20B1;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[0], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
// ===== SetTaskHandler_063134  ($063134, 8 B, kernel/task_handlers.c)
/// $063134: lea.l $62f9a(pc), a1
pub fn i_063134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63138;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $063138: move.l a1, (a6)
pub fn i_063138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6313A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06313A: rts
pub fn i_06313a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6313C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_06313c  ($06313C, 6 B, thunks/jmp_abs_thunks.c)
/// $06313C: jmp $57226.l
pub fn i_06313c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63142;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x57226;
    8
}
// ===== Camper_ToSoldier_063142  ($063142, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063142: jsr $4a0d4.l
pub fn i_063142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63148);
    cpu.pc = 0x4A0D4;
    16
}
/// $063148: jmp $5724e.l
pub fn i_063148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6314E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5724E;
    8
}
// ===== Camper_ToSoldierFlee_06314e  ($06314E, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06314E: jsr $4a0d4.l
pub fn i_06314e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63154);
    cpu.pc = 0x4A0D4;
    16
}
/// $063154: jmp $58fc2.l
pub fn i_063154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6315A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x58FC2;
    8
}
// ===== Camper_Fire2_06315a  ($06315A, 110 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06315A: move.w #$4000, d0
pub fn i_06315a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6315E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06315E: jsr $28134.l
pub fn i_06315e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63164;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63164);
    cpu.pc = 0x28134;
    16
}
/// $063164: andi.w #$ffe3, $38(a6)
pub fn i_063164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6316A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06316A: ori.w #$1c, $38(a6)
pub fn i_06316a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63170;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063170: move.w #$8000, d0
pub fn i_063170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63174;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063174: jsr $28134.l
pub fn i_063174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6317A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6317A);
    cpu.pc = 0x28134;
    16
}
/// $06317A: andi.w #$ffe3, $38(a6)
pub fn i_06317a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63180;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063180: ori.w #$1c, $38(a6)
pub fn i_063180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63186;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063186: lea.l $2dd37e.l, a0
pub fn i_063186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6318C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $06318C: jsr $28cd4.l
pub fn i_06318c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63192);
    cpu.pc = 0x28CD4;
    16
}
/// $063192: move.w #$6c, d1
pub fn i_063192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63196;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063196: jsr $236e.l
pub fn i_063196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6319C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6319C);
    cpu.pc = 0x236E;
    16
}
/// $06319C: lea.l $631a2(pc), a1
pub fn i_06319c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x631A2;
    16
}
/// $0631A0: move.l a1, (a6)
pub fn i_0631a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0631A2: jsr $2783a.l
pub fn i_0631a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x631A8);
    cpu.pc = 0x2783A;
    16
}
/// $0631A8: jsr $28d70.l
pub fn i_0631a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x631AE);
    cpu.pc = 0x28D70;
    16
}
/// $0631AE: bcc.w $631b8
pub fn i_0631ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x631B8; return 10; }
    8
}
/// $0631B2: lea.l $62f9a(pc), a1
pub fn i_0631b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $0631B6: move.l a1, (a6)
pub fn i_0631b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0631B8: movea.l #$ffffffff, a0
pub fn i_0631b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631BE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0631BE: jsr $5dd56.l
pub fn i_0631be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x631C4);
    cpu.pc = 0x5DD56;
    16
}
/// $0631C4: bcc.w $631ce
pub fn i_0631c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x631CE; return 10; }
    8
}
// ===== SetTaskHandler_0631c8  ($0631C8, 8 B, kernel/task_handlers.c)
/// $0631C8: lea.l $62f9a(pc), a1
pub fn i_0631c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $0631CC: move.l a1, (a6)
pub fn i_0631cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0631CE: rts
pub fn i_0631ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tent_Debris_0631d0  ($0631D0, 78 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0631D0: move.w #$2000, $38(a6)
pub fn i_0631d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0631D6: move.w #$6d, d1
pub fn i_0631d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631DA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0631DA: jsr $236e.l
pub fn i_0631da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x631E0);
    cpu.pc = 0x236E;
    16
}
/// $0631E0: jsr $267e2.l
pub fn i_0631e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x631E6);
    cpu.pc = 0x267E2;
    16
}
/// $0631E6: lea.l $2c52ba.l, a0
pub fn i_0631e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C52BA;
    20
}
/// $0631EC: jsr $28cd4.l
pub fn i_0631ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x631F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0631F2: lea.l $631f8(pc), a1
pub fn i_0631f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x631F8;
    16
}
/// $0631F6: move.l a1, (a6)
pub fn i_0631f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0631F8: jsr $2783a.l
pub fn i_0631f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x631FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x631FE);
    cpu.pc = 0x2783A;
    16
}
/// $0631FE: jsr $28d70.l
pub fn i_0631fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63204);
    cpu.pc = 0x28D70;
    16
}
/// $063204: bcc.w $6320e
pub fn i_063204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63208;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6320E; return 10; }
    8
}
/// $063208: lea.l $62f9a(pc), a1
pub fn i_063208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6320C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $06320C: move.l a1, (a6)
pub fn i_06320c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6320E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06320E: movea.l #$ffffffff, a0
pub fn i_06320e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63214;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $063214: jsr $5dd56.l
pub fn i_063214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6321A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6321A);
    cpu.pc = 0x5DD56;
    16
}
/// $06321A: bcc.w $63224
pub fn i_06321a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6321E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63224; return 10; }
    8
}
// ===== SetTaskHandler_06321e  ($06321E, 8 B, kernel/task_handlers.c)
/// $06321E: lea.l $62f9a(pc), a1
pub fn i_06321e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63222;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $063222: move.l a1, (a6)
pub fn i_063222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63224;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063224: rts
pub fn i_063224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63226;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tent_SmokeA_063226  ($063226, 86 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063226: move.w #$8000, d0
pub fn i_063226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6322A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06322A: jsr $28134.l
pub fn i_06322a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63230);
    cpu.pc = 0x28134;
    16
}
/// $063230: andi.w #$ffe3, $38(a6)
pub fn i_063230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63236;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063236: ori.w #$0, $38(a6)
pub fn i_063236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6323C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06323C: move.w #$6e, d1
pub fn i_06323c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63240;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063240: jsr $236e.l
pub fn i_063240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63246);
    cpu.pc = 0x236E;
    16
}
/// $063246: jsr $5e9b6.l
pub fn i_063246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6324C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6324C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06324C: btst.b #$0, d0
pub fn i_06324c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63250;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $063250: beq.w $63264
pub fn i_063250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63254;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63264; return 10; }
    8
}
/// $063254: lea.l $2ddf9e.l, a0
pub fn i_063254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6325A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDF9E;
    20
}
/// $06325A: jsr $28cd4.l
pub fn i_06325a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63260;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63260);
    cpu.pc = 0x28CD4;
    16
}
/// $063260: bra.w $63270
pub fn i_063260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63264;
    cpu.ir = 0x6000;
    cpu.pc = 0x63270;
    10
}
/// $063264: lea.l $2de014.l, a0
pub fn i_063264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6326A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE014;
    20
}
/// $06326A: jsr $28cd4.l
pub fn i_06326a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63270);
    cpu.pc = 0x28CD4;
    16
}
/// $063270: move.w #$200, $36(a6)
pub fn i_063270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63276;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063276: jmp $6dce0.l
pub fn i_063276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6327C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
// ===== Tent_SmokeB_06327c  ($06327C, 84 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06327C: move.w #$8000, d0
pub fn i_06327c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63280;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063280: jsr $28134.l
pub fn i_063280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63286);
    cpu.pc = 0x28134;
    16
}
/// $063286: andi.w #$ffe3, $38(a6)
pub fn i_063286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6328C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06328C: ori.w #$0, $38(a6)
pub fn i_06328c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63292;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063292: jsr $5e9b6.l
pub fn i_063292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63298);
    cpu.pc = 0x5E9B6;
    16
}
/// $063298: btst.b #$0, d0
pub fn i_063298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6329C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $06329C: beq.w $632b4
pub fn i_06329c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x632B4; return 10; }
    8
}
/// $0632A0: lea.l $2de43a.l, a0
pub fn i_0632a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0632A6: jsr $28cd4.l
pub fn i_0632a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x632AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0632AC: move.w #$77, d1
pub fn i_0632ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0632B0: bra.w $632c4
pub fn i_0632b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x632C4;
    10
}
/// $0632B4: lea.l $2de5f4.l, a0
pub fn i_0632b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE5F4;
    20
}
/// $0632BA: jsr $28cd4.l
pub fn i_0632ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x632C0);
    cpu.pc = 0x28CD4;
    16
}
/// $0632C0: move.w #$78, d1
pub fn i_0632c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0632C4: jsr $236e.l
pub fn i_0632c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x632CA);
    cpu.pc = 0x236E;
    16
}
/// $0632CA: jmp $6dce0.l
pub fn i_0632ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632D0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
// ===== Tent_Spark_0632d0  ($0632D0, 66 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0632D0: move.w #$4, d1
pub fn i_0632d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0632D4: jsr $236e.l
pub fn i_0632d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x632DA);
    cpu.pc = 0x236E;
    16
}
/// $0632DA: lea.l $2dd72a.l, a0
pub fn i_0632da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD72A;
    20
}
/// $0632E0: jsr $28cd4.l
pub fn i_0632e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x632E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0632E6: lea.l $632ec(pc), a1
pub fn i_0632e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x632EC;
    16
}
/// $0632EA: move.l a1, (a6)
pub fn i_0632ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0632EC: jsr $2783a.l
pub fn i_0632ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x632F2);
    cpu.pc = 0x2783A;
    16
}
/// $0632F2: jsr $28d70.l
pub fn i_0632f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x632F8);
    cpu.pc = 0x28D70;
    16
}
/// $0632F8: bcc.w $63302
pub fn i_0632f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x632FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63302; return 10; }
    8
}
/// $0632FC: lea.l $62f9a(pc), a1
pub fn i_0632fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63300;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $063300: move.l a1, (a6)
pub fn i_063300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63302;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063302: movea.l #$ffffffff, a0
pub fn i_063302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63308;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $063308: jsr $5dd56.l
pub fn i_063308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6330E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6330E);
    cpu.pc = 0x5DD56;
    16
}
/// $06330E: bcc.w $63318
pub fn i_06330e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63312;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63318; return 10; }
    8
}
// ===== SetTaskHandler_063312  ($063312, 8 B, kernel/task_handlers.c)
/// $063312: lea.l $62f9a(pc), a1
pub fn i_063312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63316;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62F9A;
    16
}
/// $063316: move.l a1, (a6)
pub fn i_063316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63318;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063318: rts
pub fn i_063318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6331A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tent_SparkB_06331a  ($06331A, 28 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06331A: move.w #$4, d1
pub fn i_06331a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6331E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06331E: jsr $236e.l
pub fn i_06331e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63324);
    cpu.pc = 0x236E;
    16
}
/// $063324: lea.l $2dd80c.l, a0
pub fn i_063324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6332A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD80C;
    20
}
/// $06332A: jsr $28cd4.l
pub fn i_06332a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63330;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63330);
    cpu.pc = 0x28CD4;
    16
}
/// $063330: jmp $725ca.l
pub fn i_063330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63336;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x725CA;
    8
}
// ===== Camper_ScrollGate_063336  ($063336, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063336: movea.l $c(a6), a0
pub fn i_063336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6333A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06333A: move.b #$1, $21(a0)
pub fn i_06333a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63340;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $063340: rts
pub fn i_063340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63342;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_RngHP_063342  ($063342, 84 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063342: lea.l $2b8304.l, a0
pub fn i_063342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63348;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8304;
    20
}
/// $063348: jsr $799de.l
pub fn i_063348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6334E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6334E);
    cpu.pc = 0x799DE;
    16
}
/// $06334E: andi.w #$3, d0
pub fn i_06334e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63352;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063352: lsl.w #$2, d0
pub fn i_063352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63354;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $063354: lea.l $2c4a0c.l, a0
pub fn i_063354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6335A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4A0C;
    20
}
/// $06335A: move.l (a0, d0.w), $5c(a6)
pub fn i_06335a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63360;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $063360: move.w $7c(a6), d0
pub fn i_063360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63364;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063364: andi.w #$3, d0
pub fn i_063364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63368;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063368: move.w d0, d1
pub fn i_063368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6336A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06336A: add.w d0, d0
pub fn i_06336a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6336C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06336C: add.w d1, d0
pub fn i_06336c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6336E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06336E: add.w d0, d0
pub fn i_06336e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63370;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $063370: movea.l $5c(a6), a0
pub fn i_063370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63374;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $063374: move.w (a0, d0.w), $28(a6)
pub fn i_063374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6337A;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06337A: move.w $2(a0, d0.w), $2e(a6)
pub fn i_06337a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63380;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $063380: move.w $4(a0, d0.w), $2a(a6)
pub fn i_063380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63386;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $063386: btst.b #$0, $3a(a6)
pub fn i_063386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6338C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06338C: beq.w $63394
pub fn i_06338c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63390;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63394; return 10; }
    8
}
/// $063390: neg.w $28(a6)
pub fn i_063390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63394;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063394: rts
pub fn i_063394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63396;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_PlayerSide_063396  ($063396, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063396: jsr $5e0d4.l
pub fn i_063396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6339C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6339C);
    cpu.pc = 0x5E0D4;
    16
}
/// $06339C: bcs.w $633da
pub fn i_06339c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633A0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x633DA; return 10; }
    8
}
/// $0633A0: move.w $22(a6), d1
pub fn i_0633a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633A4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0633A4: sub.w $22(a0), d1
pub fn i_0633a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633A8;
    cpu.ir = 0x9268;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0633A8: btst.b #$0, $3a(a6)
pub fn i_0633a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633AE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0633AE: beq.w $633b4
pub fn i_0633ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x633B4; return 10; }
    8
}
/// $0633B2: neg.w d1
pub fn i_0633b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633B4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0633B4: clr.w d0
pub fn i_0633b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633B6;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0633B6: cmpi.w #$60, d1
pub fn i_0633b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633BA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $0633BA: bgt.w $633d4
pub fn i_0633ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633BE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x633D4; return 10; }
    8
}
/// $0633BE: addq.w #$1, d0
pub fn i_0633be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633C0;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0633C0: cmpi.w #$0, d1
pub fn i_0633c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633C4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0633C4: bgt.w $633d4
pub fn i_0633c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633C8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x633D4; return 10; }
    8
}
/// $0633C8: addq.w #$1, d0
pub fn i_0633c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633CA;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0633CA: cmpi.w #$ffa0, d1
pub fn i_0633ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633CE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFA0, Size::Word, false);
    8
}
/// $0633CE: bgt.w $633d4
pub fn i_0633ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x633D4; return 10; }
    8
}
/// $0633D2: addq.w #$1, d0
pub fn i_0633d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633D4;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== ClearXN_0633d4  ($0633D4, 6 B, thunks/ccr_helpers.c)
/// $0633D4: andi.b #$ee, ccr
pub fn i_0633d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633D8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0633D8: rts
pub fn i_0633d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0633da  ($0633DA, 6 B, thunks/ccr_helpers.c)
/// $0633DA: ori.b #$11, ccr
pub fn i_0633da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633DE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0633DE: rts
pub fn i_0633de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_SetFacing_0633e0  ($0633E0, 22 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0633E0: subq.w #$1, $72(a6)
pub fn i_0633e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633E4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0633E4: cmpi.w #$0, $72(a6)
pub fn i_0633e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633EA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0633EA: ble.w $633f4
pub fn i_0633ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633EE;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x633F4; return 10; }
    8
}
/// $0633EE: move.b #$1, $44(a6)
pub fn i_0633ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0633F4: rts
pub fn i_0633f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Offset_0633f6  ($0633F6, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0633F6: btst.b #$0, $3a(a6)
pub fn i_0633f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x633FC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0633FC: beq.w $63408
pub fn i_0633fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63400;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63408; return 10; }
    8
}
/// $063400: neg.w d0
pub fn i_063400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63402;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_063402  ($063402, 6 B, thunks/ccr_helpers.c)
/// $063402: ori.b #$11, ccr
pub fn i_063402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63406;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $063406: rts
pub fn i_063406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63408;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_063408  ($063408, 6 B, thunks/ccr_helpers.c)
/// $063408: andi.b #$ee, ccr
pub fn i_063408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6340C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06340C: rts
pub fn i_06340c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6340E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_Timer_06340e  ($06340E, 20 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06340E: cmpi.w #$10, $22(a6)
pub fn i_06340e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63414;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $063414: blt.w $63428
pub fn i_063414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63418;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x63428; return 10; }
    8
}
/// $063418: cmpi.w #$110, $22(a6)
pub fn i_063418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6341E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $06341E: bgt.w $63428
pub fn i_06341e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63422;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x63428; return 10; }
    8
}
// ===== ClearXN_063422  ($063422, 6 B, thunks/ccr_helpers.c)
/// $063422: andi.b #$ee, ccr
pub fn i_063422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63426;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $063426: rts
pub fn i_063426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63428;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_063428  ($063428, 6 B, thunks/ccr_helpers.c)
/// $063428: ori.b #$11, ccr
pub fn i_063428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6342C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06342C: rts
pub fn i_06342c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6342E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_SpawnPair_06342e  ($06342E, 80 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06342E: lea.l $632d0(pc), a1
pub fn i_06342e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63432;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x632D0;
    16
}
/// $063432: jsr $4ae.l
pub fn i_063432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63438;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63438);
    cpu.pc = 0x4AE;
    16
}
/// $063438: jsr $5dd02.l
pub fn i_063438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6343E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6343E);
    cpu.pc = 0x5DD02;
    16
}
/// $06343E: move.w #$ffd8, d0
pub fn i_06343e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63442;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063442: jsr $633f6(pc)
pub fn i_063442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63446;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63446);
    cpu.pc = 0x633F6;
    16
}
/// $063446: add.w d0, $22(a0)
pub fn i_063446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6344A;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06344A: addq.w #$3, $24(a0)
pub fn i_06344a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6344E;
    cpu.ir = 0x5668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06344E: addq.w #$1, $38(a0)
pub fn i_06344e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63452;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063452: lea.l $632d0(pc), a1
pub fn i_063452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63456;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x632D0;
    16
}
/// $063456: jsr $4ae.l
pub fn i_063456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6345C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6345C);
    cpu.pc = 0x4AE;
    16
}
/// $06345C: jsr $5dd02.l
pub fn i_06345c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63462);
    cpu.pc = 0x5DD02;
    16
}
/// $063462: move.w #$24, d0
pub fn i_063462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63466;
    cpu.ir = 0x303C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063466: jsr $633f6(pc)
pub fn i_063466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6346A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6346A);
    cpu.pc = 0x633F6;
    16
}
/// $06346A: add.w d0, $22(a0)
pub fn i_06346a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6346E;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06346E: addq.w #$3, $24(a0)
pub fn i_06346e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63472;
    cpu.ir = 0x5668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063472: eori.b #$1, $3a(a0)
pub fn i_063472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63478;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $063478: subq.w #$1, $38(a0)
pub fn i_063478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6347C;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06347C: rts
pub fn i_06347c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6347E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_SpawnGroup_06347e  ($06347E, 114 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06347E: move.w #$1067, d0
pub fn i_06347e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63482;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1067;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063482: jsr $2352.l
pub fn i_063482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63488);
    cpu.pc = 0x2352;
    16
}
/// $063488: move.w $7c(a6), d3
pub fn i_063488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6348C;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06348C: andi.w #$3, d3
pub fn i_06348c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63490;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063490: lsl.w #$2, d3
pub fn i_063490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63492;
    cpu.ir = 0xE54B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $063492: lea.l $2c49dc.l, a1
pub fn i_063492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63498;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C49DC;
    20
}
/// $063498: move.w (a1, d3.w), d0
pub fn i_063498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6349C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06349C: jsr $633f6(pc)
pub fn i_06349c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x634A0);
    cpu.pc = 0x633F6;
    16
}
/// $0634A0: move.w $2(a1, d3.w), d1
pub fn i_0634a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634A4;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0634A4: movem.l d0-d1, -(a7)
pub fn i_0634a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634A8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0634A8: lea.l $63056(pc), a1
pub fn i_0634a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63056;
    16
}
/// $0634AC: jsr $4ae.l
pub fn i_0634ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x634B2);
    cpu.pc = 0x4AE;
    16
}
/// $0634B2: jsr $5dd02.l
pub fn i_0634b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x634B8);
    cpu.pc = 0x5DD02;
    16
}
/// $0634B8: movem.l (a7)+, d0-d1
pub fn i_0634b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634BC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0634BC: add.w d0, $22(a0)
pub fn i_0634bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634C0;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0634C0: add.w d1, $24(a0)
pub fn i_0634c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634C4;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0634C4: move.w $7c(a6), $7c(a0)
pub fn i_0634c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634CA;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0634CA: movem.l d0-d1, -(a7)
pub fn i_0634ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634CE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0634CE: lea.l $62fcc(pc), a1
pub fn i_0634ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x62FCC;
    16
}
/// $0634D2: jsr $4ae.l
pub fn i_0634d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x634D8);
    cpu.pc = 0x4AE;
    16
}
/// $0634D8: jsr $5dd02.l
pub fn i_0634d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x634DE);
    cpu.pc = 0x5DD02;
    16
}
/// $0634DE: movem.l (a7)+, d0-d1
pub fn i_0634de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634E2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0634E2: add.w d0, $22(a0)
pub fn i_0634e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634E6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0634E6: add.w d1, $24(a0)
pub fn i_0634e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634EA;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0634EA: move.w $7c(a6), $7c(a0)
pub fn i_0634ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634F0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7C);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== JsrPcThunk_0634f0  ($0634F0, 6 B, thunks/jsr_pc_thunks.c)
/// $0634F0: jsr $634f6(pc)
pub fn i_0634f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x634F4);
    cpu.pc = 0x634F6;
    16
}
/// $0634F4: rts
pub fn i_0634f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_SpawnPairB_0634f6  ($0634F6, 46 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0634F6: lea.l $631d0(pc), a1
pub fn i_0634f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x634FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x631D0;
    16
}
/// $0634FA: jsr $4ae.l
pub fn i_0634fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63500;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63500);
    cpu.pc = 0x4AE;
    16
}
/// $063500: jsr $5dd02.l
pub fn i_063500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63506;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63506);
    cpu.pc = 0x5DD02;
    16
}
/// $063506: addi.w #$18, $22(a0)
pub fn i_063506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6350C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06350C: lea.l $631d0(pc), a1
pub fn i_06350c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63510;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x631D0;
    16
}
/// $063510: jsr $4ae.l
pub fn i_063510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63516);
    cpu.pc = 0x4AE;
    16
}
/// $063516: jsr $5dd02.l
pub fn i_063516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6351C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6351C);
    cpu.pc = 0x5DD02;
    16
}
/// $06351C: addi.w #$fff0, $22(a0)
pub fn i_06351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63522;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $063522: rts
pub fn i_063522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camper_SpawnPairC_063524  ($063524, 56 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063524: lea.l $6315a(pc), a1
pub fn i_063524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63528;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6315A;
    16
}
/// $063528: jsr $4ae.l
pub fn i_063528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6352E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6352E);
    cpu.pc = 0x4AE;
    16
}
/// $06352E: jsr $5dd02.l
pub fn i_06352e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63534;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63534);
    cpu.pc = 0x5DD02;
    16
}
/// $063534: move.w #$10, d0
pub fn i_063534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63538;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063538: movem.w d0, -(a7)
pub fn i_063538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6353C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x061AA6, i_061aa6),
    (0x061AAA, i_061aaa),
    (0x061AAE, i_061aae),
    (0x061AB4, i_061ab4),
    (0x061ABA, i_061aba),
    (0x061ABE, i_061abe),
    (0x061AC2, i_061ac2),
    (0x061AC8, i_061ac8),
    (0x061ACE, i_061ace),
    (0x061AD2, i_061ad2),
    (0x061AD8, i_061ad8),
    (0x061ADC, i_061adc),
    (0x061AE2, i_061ae2),
    (0x061AE8, i_061ae8),
    (0x061AEC, i_061aec),
    (0x061AF2, i_061af2),
    (0x061AF8, i_061af8),
    (0x061AFC, i_061afc),
    (0x061AFE, i_061afe),
    (0x061B02, i_061b02),
    (0x061B06, i_061b06),
    (0x061B0A, i_061b0a),
    (0x061B0E, i_061b0e),
    (0x061B12, i_061b12),
    (0x061B14, i_061b14),
    (0x061B1A, i_061b1a),
    (0x061B20, i_061b20),
    (0x061B24, i_061b24),
    (0x061B28, i_061b28),
    (0x061B2A, i_061b2a),
    (0x061B30, i_061b30),
    (0x061B34, i_061b34),
    (0x061B38, i_061b38),
    (0x061B3C, i_061b3c),
    (0x061B3E, i_061b3e),
    (0x061B42, i_061b42),
    (0x061B46, i_061b46),
    (0x061B4A, i_061b4a),
    (0x061B4C, i_061b4c),
    (0x061B50, i_061b50),
    (0x061B54, i_061b54),
    (0x061B58, i_061b58),
    (0x061B5A, i_061b5a),
    (0x061B60, i_061b60),
    (0x061B66, i_061b66),
    (0x061B6A, i_061b6a),
    (0x061B6E, i_061b6e),
    (0x061B74, i_061b74),
    (0x061B7A, i_061b7a),
    (0x061B80, i_061b80),
    (0x061B84, i_061b84),
    (0x061B8A, i_061b8a),
    (0x061B90, i_061b90),
    (0x061B94, i_061b94),
    (0x061B96, i_061b96),
    (0x061B9C, i_061b9c),
    (0x061BA2, i_061ba2),
    (0x061BA8, i_061ba8),
    (0x061BAE, i_061bae),
    (0x061BB2, i_061bb2),
    (0x061BB4, i_061bb4),
    (0x061BBA, i_061bba),
    (0x061BC0, i_061bc0),
    (0x061BC6, i_061bc6),
    (0x061BCA, i_061bca),
    (0x061BCE, i_061bce),
    (0x061BD0, i_061bd0),
    (0x061BD6, i_061bd6),
    (0x061BDC, i_061bdc),
    (0x061BE2, i_061be2),
    (0x061BE8, i_061be8),
    (0x061BEE, i_061bee),
    (0x061BF2, i_061bf2),
    (0x061BF8, i_061bf8),
    (0x061BFE, i_061bfe),
    (0x061C02, i_061c02),
    (0x061C04, i_061c04),
    (0x061C0A, i_061c0a),
    (0x061C10, i_061c10),
    (0x061C16, i_061c16),
    (0x061C1A, i_061c1a),
    (0x061C1E, i_061c1e),
    (0x061C20, i_061c20),
    (0x061C24, i_061c24),
    (0x061C28, i_061c28),
    (0x061C2C, i_061c2c),
    (0x061C2E, i_061c2e),
    (0x061C32, i_061c32),
    (0x061C36, i_061c36),
    (0x061C3A, i_061c3a),
    (0x061C3C, i_061c3c),
    (0x061C40, i_061c40),
    (0x061C44, i_061c44),
    (0x061C48, i_061c48),
    (0x061C4A, i_061c4a),
    (0x061C4E, i_061c4e),
    (0x061C54, i_061c54),
    (0x061C58, i_061c58),
    (0x061C5E, i_061c5e),
    (0x061C62, i_061c62),
    (0x061C64, i_061c64),
    (0x061C68, i_061c68),
    (0x061C6C, i_061c6c),
    (0x061C72, i_061c72),
    (0x061C78, i_061c78),
    (0x061C7E, i_061c7e),
    (0x061C82, i_061c82),
    (0x061C88, i_061c88),
    (0x061C8E, i_061c8e),
    (0x061C94, i_061c94),
    (0x061C98, i_061c98),
    (0x061C9C, i_061c9c),
    (0x061C9E, i_061c9e),
    (0x061CA2, i_061ca2),
    (0x061CA4, i_061ca4),
    (0x061CA8, i_061ca8),
    (0x061CAC, i_061cac),
    (0x061CB2, i_061cb2),
    (0x061CB6, i_061cb6),
    (0x061CB8, i_061cb8),
    (0x061CBE, i_061cbe),
    (0x061CC4, i_061cc4),
    (0x061CC8, i_061cc8),
    (0x061CCC, i_061ccc),
    (0x061CD0, i_061cd0),
    (0x061CD4, i_061cd4),
    (0x061CD8, i_061cd8),
    (0x061CDC, i_061cdc),
    (0x061CE0, i_061ce0),
    (0x061CE4, i_061ce4),
    (0x061CE8, i_061ce8),
    (0x061CEC, i_061cec),
    (0x061CF2, i_061cf2),
    (0x061CF6, i_061cf6),
    (0x061CFA, i_061cfa),
    (0x061CFC, i_061cfc),
    (0x061D00, i_061d00),
    (0x061D04, i_061d04),
    (0x061D08, i_061d08),
    (0x061D0C, i_061d0c),
    (0x061D10, i_061d10),
    (0x061D14, i_061d14),
    (0x061D18, i_061d18),
    (0x061D1C, i_061d1c),
    (0x061D22, i_061d22),
    (0x061D28, i_061d28),
    (0x061D2C, i_061d2c),
    (0x061D30, i_061d30),
    (0x061D36, i_061d36),
    (0x061D3C, i_061d3c),
    (0x061D40, i_061d40),
    (0x061D46, i_061d46),
    (0x061D4A, i_061d4a),
    (0x061D4C, i_061d4c),
    (0x061D4E, i_061d4e),
    (0x061D52, i_061d52),
    (0x061D56, i_061d56),
    (0x061D58, i_061d58),
    (0x061D5E, i_061d5e),
    (0x061D64, i_061d64),
    (0x061D6A, i_061d6a),
    (0x061D6E, i_061d6e),
    (0x061D72, i_061d72),
    (0x061D74, i_061d74),
    (0x061D7A, i_061d7a),
    (0x061D80, i_061d80),
    (0x061D84, i_061d84),
    (0x061D8A, i_061d8a),
    (0x061D90, i_061d90),
    (0x061D94, i_061d94),
    (0x061D96, i_061d96),
    (0x061D9C, i_061d9c),
    (0x061DA2, i_061da2),
    (0x061DA8, i_061da8),
    (0x061DAC, i_061dac),
    (0x061DB0, i_061db0),
    (0x061DB2, i_061db2),
    (0x061DB6, i_061db6),
    (0x061DBC, i_061dbc),
    (0x061DC2, i_061dc2),
    (0x061DC6, i_061dc6),
    (0x061DCC, i_061dcc),
    (0x061DD2, i_061dd2),
    (0x061DD6, i_061dd6),
    (0x061DDC, i_061ddc),
    (0x061DE0, i_061de0),
    (0x061DE6, i_061de6),
    (0x061DEA, i_061dea),
    (0x061DF0, i_061df0),
    (0x061DF6, i_061df6),
    (0x061DFA, i_061dfa),
    (0x061E00, i_061e00),
    (0x061E06, i_061e06),
    (0x061E0A, i_061e0a),
    (0x061E0C, i_061e0c),
    (0x061E12, i_061e12),
    (0x061E18, i_061e18),
    (0x061E1E, i_061e1e),
    (0x061E22, i_061e22),
    (0x061E28, i_061e28),
    (0x061E2C, i_061e2c),
    (0x061E30, i_061e30),
    (0x061E32, i_061e32),
    (0x061E38, i_061e38),
    (0x061E3E, i_061e3e),
    (0x061E42, i_061e42),
    (0x061E48, i_061e48),
    (0x061E4C, i_061e4c),
    (0x061E50, i_061e50),
    (0x061E56, i_061e56),
    (0x061E5A, i_061e5a),
    (0x061E5E, i_061e5e),
    (0x061E64, i_061e64),
    (0x061E68, i_061e68),
    (0x061E6C, i_061e6c),
    (0x061E6E, i_061e6e),
    (0x061E72, i_061e72),
    (0x061E76, i_061e76),
    (0x061E7C, i_061e7c),
    (0x061E82, i_061e82),
    (0x061E88, i_061e88),
    (0x061E8E, i_061e8e),
    (0x061E94, i_061e94),
    (0x061E98, i_061e98),
    (0x061E9A, i_061e9a),
    (0x061EA0, i_061ea0),
    (0x061EA6, i_061ea6),
    (0x061EAC, i_061eac),
    (0x061EB0, i_061eb0),
    (0x061EB4, i_061eb4),
    (0x061EB6, i_061eb6),
    (0x061EBA, i_061eba),
    (0x061EBE, i_061ebe),
    (0x061EC4, i_061ec4),
    (0x061ECA, i_061eca),
    (0x061ECE, i_061ece),
    (0x061ED4, i_061ed4),
    (0x061EDA, i_061eda),
    (0x061EE0, i_061ee0),
    (0x061EE4, i_061ee4),
    (0x061EE6, i_061ee6),
    (0x061EEC, i_061eec),
    (0x061EF2, i_061ef2),
    (0x061EF8, i_061ef8),
    (0x061EFC, i_061efc),
    (0x061F00, i_061f00),
    (0x061F02, i_061f02),
    (0x061F06, i_061f06),
    (0x061F0C, i_061f0c),
    (0x061F10, i_061f10),
    (0x061F16, i_061f16),
    (0x061F1C, i_061f1c),
    (0x061F22, i_061f22),
    (0x061F26, i_061f26),
    (0x061F2C, i_061f2c),
    (0x061F32, i_061f32),
    (0x061F38, i_061f38),
    (0x061F3E, i_061f3e),
    (0x061F42, i_061f42),
    (0x061F46, i_061f46),
    (0x061F4A, i_061f4a),
    (0x061F50, i_061f50),
    (0x061F56, i_061f56),
    (0x061F5C, i_061f5c),
    (0x061F60, i_061f60),
    (0x061F62, i_061f62),
    (0x061F68, i_061f68),
    (0x061F6E, i_061f6e),
    (0x061F72, i_061f72),
    (0x061F76, i_061f76),
    (0x061F7A, i_061f7a),
    (0x061F7E, i_061f7e),
    (0x061F80, i_061f80),
    (0x061F86, i_061f86),
    (0x061F8C, i_061f8c),
    (0x061F92, i_061f92),
    (0x061F96, i_061f96),
    (0x061F9A, i_061f9a),
    (0x061F9C, i_061f9c),
    (0x061F9E, i_061f9e),
    (0x061FA4, i_061fa4),
    (0x061FAA, i_061faa),
    (0x061FAE, i_061fae),
    (0x061FB4, i_061fb4),
    (0x061FBA, i_061fba),
    (0x061FC0, i_061fc0),
    (0x061FC6, i_061fc6),
    (0x061FCA, i_061fca),
    (0x061FCC, i_061fcc),
    (0x061FD2, i_061fd2),
    (0x061FD8, i_061fd8),
    (0x061FDC, i_061fdc),
    (0x061FE0, i_061fe0),
    (0x061FE4, i_061fe4),
    (0x061FE8, i_061fe8),
    (0x061FEA, i_061fea),
    (0x061FF0, i_061ff0),
    (0x061FF6, i_061ff6),
    (0x061FFC, i_061ffc),
    (0x062000, i_062000),
    (0x062004, i_062004),
    (0x062006, i_062006),
    (0x062008, i_062008),
    (0x06200C, i_06200c),
    (0x062010, i_062010),
    (0x062012, i_062012),
    (0x062014, i_062014),
    (0x06201A, i_06201a),
    (0x06201E, i_06201e),
    (0x062024, i_062024),
    (0x06202A, i_06202a),
    (0x062030, i_062030),
    (0x062034, i_062034),
    (0x062036, i_062036),
    (0x06203C, i_06203c),
    (0x062040, i_062040),
    (0x062044, i_062044),
    (0x062046, i_062046),
    (0x06204C, i_06204c),
    (0x062050, i_062050),
    (0x062054, i_062054),
    (0x062056, i_062056),
    (0x06205C, i_06205c),
    (0x062060, i_062060),
    (0x062064, i_062064),
    (0x062066, i_062066),
    (0x06206C, i_06206c),
    (0x062072, i_062072),
    (0x062078, i_062078),
    (0x06207C, i_06207c),
    (0x062080, i_062080),
    (0x062082, i_062082),
    (0x062084, i_062084),
    (0x06208A, i_06208a),
    (0x06208C, i_06208c),
    (0x062092, i_062092),
    (0x062098, i_062098),
    (0x06209E, i_06209e),
    (0x0620A0, i_0620a0),
    (0x0620A6, i_0620a6),
    (0x0620A8, i_0620a8),
    (0x0620AE, i_0620ae),
    (0x0620B4, i_0620b4),
    (0x0620B6, i_0620b6),
    (0x0620BC, i_0620bc),
    (0x0620C2, i_0620c2),
    (0x0620C6, i_0620c6),
    (0x0620CA, i_0620ca),
    (0x0620CE, i_0620ce),
    (0x0620D4, i_0620d4),
    (0x0620DA, i_0620da),
    (0x0620DE, i_0620de),
    (0x0620E4, i_0620e4),
    (0x0620EA, i_0620ea),
    (0x0620EE, i_0620ee),
    (0x0620F4, i_0620f4),
    (0x0620FA, i_0620fa),
    (0x062100, i_062100),
    (0x062106, i_062106),
    (0x06210C, i_06210c),
    (0x062110, i_062110),
    (0x062112, i_062112),
    (0x062118, i_062118),
    (0x06211E, i_06211e),
    (0x062122, i_062122),
    (0x062126, i_062126),
    (0x062128, i_062128),
    (0x06212A, i_06212a),
    (0x06212E, i_06212e),
    (0x062134, i_062134),
    (0x06213A, i_06213a),
    (0x06213E, i_06213e),
    (0x062144, i_062144),
    (0x062148, i_062148),
    (0x06214C, i_06214c),
    (0x062150, i_062150),
    (0x062156, i_062156),
    (0x06215A, i_06215a),
    (0x06215E, i_06215e),
    (0x062162, i_062162),
    (0x062168, i_062168),
    (0x06216C, i_06216c),
    (0x06216E, i_06216e),
    (0x062172, i_062172),
    (0x062178, i_062178),
    (0x06217E, i_06217e),
    (0x062184, i_062184),
    (0x06218A, i_06218a),
    (0x06218E, i_06218e),
    (0x062190, i_062190),
    (0x062194, i_062194),
    (0x06219A, i_06219a),
    (0x06219E, i_06219e),
    (0x0621A2, i_0621a2),
    (0x0621A4, i_0621a4),
    (0x0621A8, i_0621a8),
    (0x0621AE, i_0621ae),
    (0x0621B4, i_0621b4),
    (0x0621BA, i_0621ba),
    (0x0621BE, i_0621be),
    (0x0621C0, i_0621c0),
    (0x0621C4, i_0621c4),
    (0x0621CA, i_0621ca),
    (0x0621CE, i_0621ce),
    (0x0621D2, i_0621d2),
    (0x0621D4, i_0621d4),
    (0x0621D8, i_0621d8),
    (0x0621DC, i_0621dc),
    (0x0621E2, i_0621e2),
    (0x0621E8, i_0621e8),
    (0x0621EE, i_0621ee),
    (0x0621F4, i_0621f4),
    (0x0621FA, i_0621fa),
    (0x0621FE, i_0621fe),
    (0x062204, i_062204),
    (0x06220A, i_06220a),
    (0x062210, i_062210),
    (0x062216, i_062216),
    (0x06221A, i_06221a),
    (0x06221C, i_06221c),
    (0x062220, i_062220),
    (0x062226, i_062226),
    (0x06222A, i_06222a),
    (0x06222E, i_06222e),
    (0x062230, i_062230),
    (0x062234, i_062234),
    (0x06223A, i_06223a),
    (0x06223E, i_06223e),
    (0x062244, i_062244),
    (0x062248, i_062248),
    (0x06224E, i_06224e),
    (0x062254, i_062254),
    (0x06225A, i_06225a),
    (0x06225E, i_06225e),
    (0x062264, i_062264),
    (0x06226A, i_06226a),
    (0x062270, i_062270),
    (0x062274, i_062274),
    (0x062278, i_062278),
    (0x06227E, i_06227e),
    (0x062282, i_062282),
    (0x062288, i_062288),
    (0x06228E, i_06228e),
    (0x062292, i_062292),
    (0x062296, i_062296),
    (0x06229A, i_06229a),
    (0x0622A0, i_0622a0),
    (0x0622A6, i_0622a6),
    (0x0622AA, i_0622aa),
    (0x0622B0, i_0622b0),
    (0x0622B6, i_0622b6),
    (0x0622BC, i_0622bc),
    (0x0622C0, i_0622c0),
    (0x0622C6, i_0622c6),
    (0x0622CA, i_0622ca),
    (0x0622CE, i_0622ce),
    (0x0622D4, i_0622d4),
    (0x0622DA, i_0622da),
    (0x0622DE, i_0622de),
    (0x0622E2, i_0622e2),
    (0x0622E8, i_0622e8),
    (0x0622EE, i_0622ee),
    (0x0622F2, i_0622f2),
    (0x0622F4, i_0622f4),
    (0x0622F8, i_0622f8),
    (0x0622FC, i_0622fc),
    (0x062302, i_062302),
    (0x062306, i_062306),
    (0x06230A, i_06230a),
    (0x06230E, i_06230e),
    (0x062312, i_062312),
    (0x062314, i_062314),
    (0x06231A, i_06231a),
    (0x06231E, i_06231e),
    (0x062322, i_062322),
    (0x062324, i_062324),
    (0x062326, i_062326),
    (0x06232C, i_06232c),
    (0x062330, i_062330),
    (0x062336, i_062336),
    (0x06233C, i_06233c),
    (0x062340, i_062340),
    (0x062342, i_062342),
    (0x062348, i_062348),
    (0x06234E, i_06234e),
    (0x062352, i_062352),
    (0x062356, i_062356),
    (0x062358, i_062358),
    (0x06235A, i_06235a),
    (0x062360, i_062360),
    (0x062366, i_062366),
    (0x06236A, i_06236a),
    (0x062370, i_062370),
    (0x062374, i_062374),
    (0x06237A, i_06237a),
    (0x06237E, i_06237e),
    (0x062384, i_062384),
    (0x062388, i_062388),
    (0x06238E, i_06238e),
    (0x062392, i_062392),
    (0x062394, i_062394),
    (0x062398, i_062398),
    (0x06239A, i_06239a),
    (0x06239E, i_06239e),
    (0x0623A4, i_0623a4),
    (0x0623AA, i_0623aa),
    (0x0623B0, i_0623b0),
    (0x0623B4, i_0623b4),
    (0x0623B6, i_0623b6),
    (0x0623BC, i_0623bc),
    (0x0623C2, i_0623c2),
    (0x0623C6, i_0623c6),
    (0x0623C8, i_0623c8),
    (0x0623CC, i_0623cc),
    (0x0623D2, i_0623d2),
    (0x0623D8, i_0623d8),
    (0x0623DE, i_0623de),
    (0x0623E4, i_0623e4),
    (0x0623EA, i_0623ea),
    (0x0623EE, i_0623ee),
    (0x0623F2, i_0623f2),
    (0x0623F4, i_0623f4),
    (0x0623FA, i_0623fa),
    (0x0623FE, i_0623fe),
    (0x062402, i_062402),
    (0x062404, i_062404),
    (0x06240A, i_06240a),
    (0x062410, i_062410),
    (0x062414, i_062414),
    (0x062418, i_062418),
    (0x06241A, i_06241a),
    (0x06241C, i_06241c),
    (0x062422, i_062422),
    (0x062426, i_062426),
    (0x06242C, i_06242c),
    (0x062432, i_062432),
    (0x062438, i_062438),
    (0x062440, i_062440),
    (0x062446, i_062446),
    (0x06244C, i_06244c),
    (0x062452, i_062452),
    (0x062456, i_062456),
    (0x06245C, i_06245c),
    (0x062462, i_062462),
    (0x062466, i_062466),
    (0x06246C, i_06246c),
    (0x062470, i_062470),
    (0x062476, i_062476),
    (0x06247A, i_06247a),
    (0x062480, i_062480),
    (0x062486, i_062486),
    (0x06248C, i_06248c),
    (0x062490, i_062490),
    (0x062496, i_062496),
    (0x06249C, i_06249c),
    (0x0624A2, i_0624a2),
    (0x0624A6, i_0624a6),
    (0x0624A8, i_0624a8),
    (0x0624AE, i_0624ae),
    (0x0624B2, i_0624b2),
    (0x0624B6, i_0624b6),
    (0x0624B8, i_0624b8),
    (0x0624BE, i_0624be),
    (0x0624C4, i_0624c4),
    (0x0624CA, i_0624ca),
    (0x0624CE, i_0624ce),
    (0x0624D2, i_0624d2),
    (0x0624D4, i_0624d4),
    (0x0624D6, i_0624d6),
    (0x0624DA, i_0624da),
    (0x0624E0, i_0624e0),
    (0x0624E6, i_0624e6),
    (0x0624EC, i_0624ec),
    (0x0624F0, i_0624f0),
    (0x0624F6, i_0624f6),
    (0x0624FC, i_0624fc),
    (0x062502, i_062502),
    (0x062506, i_062506),
    (0x062508, i_062508),
    (0x06250E, i_06250e),
    (0x062514, i_062514),
    (0x062518, i_062518),
    (0x06251C, i_06251c),
    (0x06251E, i_06251e),
    (0x062524, i_062524),
    (0x06252A, i_06252a),
    (0x06252E, i_06252e),
    (0x062532, i_062532),
    (0x062534, i_062534),
    (0x062536, i_062536),
    (0x06253C, i_06253c),
    (0x062542, i_062542),
    (0x062544, i_062544),
    (0x062548, i_062548),
    (0x06254C, i_06254c),
    (0x062550, i_062550),
    (0x062554, i_062554),
    (0x062558, i_062558),
    (0x06255E, i_06255e),
    (0x062564, i_062564),
    (0x062568, i_062568),
    (0x06256C, i_06256c),
    (0x062572, i_062572),
    (0x062578, i_062578),
    (0x06257C, i_06257c),
    (0x062582, i_062582),
    (0x062588, i_062588),
    (0x06258E, i_06258e),
    (0x062594, i_062594),
    (0x062596, i_062596),
    (0x06259A, i_06259a),
    (0x06259E, i_06259e),
    (0x0625A2, i_0625a2),
    (0x0625A6, i_0625a6),
    (0x0625AA, i_0625aa),
    (0x0625B0, i_0625b0),
    (0x0625B6, i_0625b6),
    (0x0625BA, i_0625ba),
    (0x0625BE, i_0625be),
    (0x0625C4, i_0625c4),
    (0x0625CA, i_0625ca),
    (0x0625CE, i_0625ce),
    (0x0625D4, i_0625d4),
    (0x0625DA, i_0625da),
    (0x0625DE, i_0625de),
    (0x0625E4, i_0625e4),
    (0x0625EA, i_0625ea),
    (0x0625F0, i_0625f0),
    (0x0625F4, i_0625f4),
    (0x0625FA, i_0625fa),
    (0x062600, i_062600),
    (0x062606, i_062606),
    (0x06260A, i_06260a),
    (0x06260C, i_06260c),
    (0x062612, i_062612),
    (0x062618, i_062618),
    (0x06261C, i_06261c),
    (0x062620, i_062620),
    (0x062622, i_062622),
    (0x062628, i_062628),
    (0x06262E, i_06262e),
    (0x062632, i_062632),
    (0x062636, i_062636),
    (0x062638, i_062638),
    (0x06263A, i_06263a),
    (0x06263E, i_06263e),
    (0x062644, i_062644),
    (0x06264A, i_06264a),
    (0x062650, i_062650),
    (0x062654, i_062654),
    (0x062656, i_062656),
    (0x06265C, i_06265c),
    (0x062662, i_062662),
    (0x062666, i_062666),
    (0x06266A, i_06266a),
    (0x06266C, i_06266c),
    (0x062672, i_062672),
    (0x062678, i_062678),
    (0x06267C, i_06267c),
    (0x062680, i_062680),
    (0x062682, i_062682),
    (0x062684, i_062684),
    (0x062688, i_062688),
    (0x06268C, i_06268c),
    (0x062692, i_062692),
    (0x062696, i_062696),
    (0x062698, i_062698),
    (0x06269C, i_06269c),
    (0x0626A0, i_0626a0),
    (0x0626A4, i_0626a4),
    (0x0626A8, i_0626a8),
    (0x0626AC, i_0626ac),
    (0x0626B0, i_0626b0),
    (0x0626B2, i_0626b2),
    (0x0626B6, i_0626b6),
    (0x0626B8, i_0626b8),
    (0x0626BE, i_0626be),
    (0x0626C2, i_0626c2),
    (0x0626C4, i_0626c4),
    (0x0626C8, i_0626c8),
    (0x0626CA, i_0626ca),
    (0x0626CE, i_0626ce),
    (0x0626D0, i_0626d0),
    (0x0626D6, i_0626d6),
    (0x0626D8, i_0626d8),
    (0x0626DC, i_0626dc),
    (0x0626E0, i_0626e0),
    (0x0626E4, i_0626e4),
    (0x0626E8, i_0626e8),
    (0x0626EA, i_0626ea),
    (0x0626EE, i_0626ee),
    (0x0626F0, i_0626f0),
    (0x0626F4, i_0626f4),
    (0x0626F8, i_0626f8),
    (0x0626FC, i_0626fc),
    (0x062700, i_062700),
    (0x062704, i_062704),
    (0x062706, i_062706),
    (0x062708, i_062708),
    (0x06270C, i_06270c),
    (0x06270E, i_06270e),
    (0x062710, i_062710),
    (0x062714, i_062714),
    (0x062718, i_062718),
    (0x06271E, i_06271e),
    (0x062722, i_062722),
    (0x062726, i_062726),
    (0x06272A, i_06272a),
    (0x06272C, i_06272c),
    (0x062730, i_062730),
    (0x062732, i_062732),
    (0x062734, i_062734),
    (0x062738, i_062738),
    (0x06273C, i_06273c),
    (0x06273E, i_06273e),
    (0x062742, i_062742),
    (0x062746, i_062746),
    (0x06274A, i_06274a),
    (0x06274C, i_06274c),
    (0x062750, i_062750),
    (0x062756, i_062756),
    (0x062758, i_062758),
    (0x06275E, i_06275e),
    (0x062762, i_062762),
    (0x062766, i_062766),
    (0x06276A, i_06276a),
    (0x06276C, i_06276c),
    (0x062772, i_062772),
    (0x062774, i_062774),
    (0x062778, i_062778),
    (0x06277E, i_06277e),
    (0x062784, i_062784),
    (0x06278A, i_06278a),
    (0x06278E, i_06278e),
    (0x062792, i_062792),
    (0x06279A, i_06279a),
    (0x0627A0, i_0627a0),
    (0x0627A4, i_0627a4),
    (0x0627A8, i_0627a8),
    (0x0627AE, i_0627ae),
    (0x0627B4, i_0627b4),
    (0x0627BC, i_0627bc),
    (0x0627C2, i_0627c2),
    (0x0627C8, i_0627c8),
    (0x0627CA, i_0627ca),
    (0x0627CE, i_0627ce),
    (0x0627D4, i_0627d4),
    (0x0627D8, i_0627d8),
    (0x0627DE, i_0627de),
    (0x0627E4, i_0627e4),
    (0x0627EA, i_0627ea),
    (0x0627EE, i_0627ee),
    (0x0627F4, i_0627f4),
    (0x0627F8, i_0627f8),
    (0x0627FA, i_0627fa),
    (0x0627FE, i_0627fe),
    (0x062802, i_062802),
    (0x062808, i_062808),
    (0x06280E, i_06280e),
    (0x062814, i_062814),
    (0x062816, i_062816),
    (0x06281A, i_06281a),
    (0x062820, i_062820),
    (0x062826, i_062826),
    (0x06282C, i_06282c),
    (0x06282E, i_06282e),
    (0x062832, i_062832),
    (0x062838, i_062838),
    (0x06283E, i_06283e),
    (0x062844, i_062844),
    (0x06284A, i_06284a),
    (0x06284E, i_06284e),
    (0x062852, i_062852),
    (0x062856, i_062856),
    (0x06285C, i_06285c),
    (0x062862, i_062862),
    (0x062868, i_062868),
    (0x06286C, i_06286c),
    (0x06286E, i_06286e),
    (0x062870, i_062870),
    (0x062872, i_062872),
    (0x062876, i_062876),
    (0x06287C, i_06287c),
    (0x062882, i_062882),
    (0x062888, i_062888),
    (0x06288A, i_06288a),
    (0x06288E, i_06288e),
    (0x062894, i_062894),
    (0x06289A, i_06289a),
    (0x06289C, i_06289c),
    (0x0628A0, i_0628a0),
    (0x0628A4, i_0628a4),
    (0x0628AA, i_0628aa),
    (0x0628AC, i_0628ac),
    (0x0628B0, i_0628b0),
    (0x0628B6, i_0628b6),
    (0x0628BA, i_0628ba),
    (0x0628C0, i_0628c0),
    (0x0628C6, i_0628c6),
    (0x0628CA, i_0628ca),
    (0x0628D0, i_0628d0),
    (0x0628D6, i_0628d6),
    (0x0628DC, i_0628dc),
    (0x0628E0, i_0628e0),
    (0x0628E4, i_0628e4),
    (0x0628E8, i_0628e8),
    (0x0628EA, i_0628ea),
    (0x0628EE, i_0628ee),
    (0x0628F0, i_0628f0),
    (0x0628F4, i_0628f4),
    (0x0628FA, i_0628fa),
    (0x062900, i_062900),
    (0x062904, i_062904),
    (0x062906, i_062906),
    (0x062908, i_062908),
    (0x06290C, i_06290c),
    (0x06290E, i_06290e),
    (0x062910, i_062910),
    (0x062914, i_062914),
    (0x06291A, i_06291a),
    (0x062920, i_062920),
    (0x062922, i_062922),
    (0x062926, i_062926),
    (0x06292A, i_06292a),
    (0x06292E, i_06292e),
    (0x062932, i_062932),
    (0x062936, i_062936),
    (0x062938, i_062938),
    (0x06293C, i_06293c),
    (0x06293E, i_06293e),
    (0x062942, i_062942),
    (0x062948, i_062948),
    (0x06294E, i_06294e),
    (0x062954, i_062954),
    (0x06295A, i_06295a),
    (0x06295E, i_06295e),
    (0x062962, i_062962),
    (0x062964, i_062964),
    (0x06296A, i_06296a),
    (0x06296E, i_06296e),
    (0x062974, i_062974),
    (0x062978, i_062978),
    (0x06297E, i_06297e),
    (0x062982, i_062982),
    (0x062984, i_062984),
    (0x06298A, i_06298a),
    (0x06298E, i_06298e),
    (0x062994, i_062994),
    (0x062998, i_062998),
    (0x06299C, i_06299c),
    (0x0629A0, i_0629a0),
    (0x0629A6, i_0629a6),
    (0x0629AA, i_0629aa),
    (0x0629AE, i_0629ae),
    (0x0629B4, i_0629b4),
    (0x0629B8, i_0629b8),
    (0x0629BE, i_0629be),
    (0x0629C2, i_0629c2),
    (0x0629C8, i_0629c8),
    (0x0629CE, i_0629ce),
    (0x0629D2, i_0629d2),
    (0x0629D6, i_0629d6),
    (0x0629DC, i_0629dc),
    (0x0629E0, i_0629e0),
    (0x0629E6, i_0629e6),
    (0x0629EA, i_0629ea),
    (0x0629F0, i_0629f0),
    (0x0629F6, i_0629f6),
    (0x0629FC, i_0629fc),
    (0x062A00, i_062a00),
    (0x062A04, i_062a04),
    (0x062A06, i_062a06),
    (0x062A08, i_062a08),
    (0x062A0E, i_062a0e),
    (0x062A10, i_062a10),
    (0x062A16, i_062a16),
    (0x062A1A, i_062a1a),
    (0x062A1C, i_062a1c),
    (0x062A20, i_062a20),
    (0x062A26, i_062a26),
    (0x062A2A, i_062a2a),
    (0x062A2E, i_062a2e),
    (0x062A32, i_062a32),
    (0x062A38, i_062a38),
    (0x062A3C, i_062a3c),
    (0x062A42, i_062a42),
    (0x062A46, i_062a46),
    (0x062A4C, i_062a4c),
    (0x062A4E, i_062a4e),
    (0x062A54, i_062a54),
    (0x062A58, i_062a58),
    (0x062A5E, i_062a5e),
    (0x062A64, i_062a64),
    (0x062A6A, i_062a6a),
    (0x062A70, i_062a70),
    (0x062A76, i_062a76),
    (0x062A7A, i_062a7a),
    (0x062A80, i_062a80),
    (0x062A86, i_062a86),
    (0x062A8A, i_062a8a),
    (0x062A90, i_062a90),
    (0x062A94, i_062a94),
    (0x062A98, i_062a98),
    (0x062A9E, i_062a9e),
    (0x062AA2, i_062aa2),
    (0x062AA4, i_062aa4),
    (0x062AA8, i_062aa8),
    (0x062AAE, i_062aae),
    (0x062AB4, i_062ab4),
    (0x062ABA, i_062aba),
    (0x062AC0, i_062ac0),
    (0x062AC6, i_062ac6),
    (0x062ACA, i_062aca),
    (0x062AD0, i_062ad0),
    (0x062AD4, i_062ad4),
    (0x062ADA, i_062ada),
    (0x062AE0, i_062ae0),
    (0x062AE6, i_062ae6),
    (0x062AEA, i_062aea),
    (0x062AEE, i_062aee),
    (0x062AF4, i_062af4),
    (0x062AFA, i_062afa),
    (0x062AFE, i_062afe),
    (0x062B00, i_062b00),
    (0x062B06, i_062b06),
    (0x062B0A, i_062b0a),
    (0x062B0E, i_062b0e),
    (0x062B14, i_062b14),
    (0x062B18, i_062b18),
    (0x062B1A, i_062b1a),
    (0x062B20, i_062b20),
    (0x062B24, i_062b24),
    (0x062B28, i_062b28),
    (0x062B2C, i_062b2c),
    (0x062B32, i_062b32),
    (0x062B38, i_062b38),
    (0x062B3E, i_062b3e),
    (0x062B44, i_062b44),
    (0x062B48, i_062b48),
    (0x062B4C, i_062b4c),
    (0x062B50, i_062b50),
    (0x062B56, i_062b56),
    (0x062B5A, i_062b5a),
    (0x062B5E, i_062b5e),
    (0x062B64, i_062b64),
    (0x062B6A, i_062b6a),
    (0x062B6E, i_062b6e),
    (0x062B74, i_062b74),
    (0x062B78, i_062b78),
    (0x062B7C, i_062b7c),
    (0x062B7E, i_062b7e),
    (0x062B82, i_062b82),
    (0x062B84, i_062b84),
    (0x062B88, i_062b88),
    (0x062B8E, i_062b8e),
    (0x062B96, i_062b96),
    (0x062B9C, i_062b9c),
    (0x062BA2, i_062ba2),
    (0x062BA8, i_062ba8),
    (0x062BAC, i_062bac),
    (0x062BAE, i_062bae),
    (0x062BB4, i_062bb4),
    (0x062BBA, i_062bba),
    (0x062BBE, i_062bbe),
    (0x062BC2, i_062bc2),
    (0x062BC4, i_062bc4),
    (0x062BC8, i_062bc8),
    (0x062BCE, i_062bce),
    (0x062BD4, i_062bd4),
    (0x062BDA, i_062bda),
    (0x062BDE, i_062bde),
    (0x062BE0, i_062be0),
    (0x062BE6, i_062be6),
    (0x062BEC, i_062bec),
    (0x062BF0, i_062bf0),
    (0x062BF6, i_062bf6),
    (0x062BFA, i_062bfa),
    (0x062BFE, i_062bfe),
    (0x062C02, i_062c02),
    (0x062C06, i_062c06),
    (0x062C0A, i_062c0a),
    (0x062C0E, i_062c0e),
    (0x062C12, i_062c12),
    (0x062C16, i_062c16),
    (0x062C1A, i_062c1a),
    (0x062C1E, i_062c1e),
    (0x062C20, i_062c20),
    (0x062C24, i_062c24),
    (0x062C28, i_062c28),
    (0x062C2C, i_062c2c),
    (0x062C30, i_062c30),
    (0x062C32, i_062c32),
    (0x062C36, i_062c36),
    (0x062C3A, i_062c3a),
    (0x062C3C, i_062c3c),
    (0x062C40, i_062c40),
    (0x062C44, i_062c44),
    (0x062C48, i_062c48),
    (0x062C4E, i_062c4e),
    (0x062C54, i_062c54),
    (0x062C5A, i_062c5a),
    (0x062C5E, i_062c5e),
    (0x062C60, i_062c60),
    (0x062C66, i_062c66),
    (0x062C6A, i_062c6a),
    (0x062C70, i_062c70),
    (0x062C74, i_062c74),
    (0x062C78, i_062c78),
    (0x062C7E, i_062c7e),
    (0x062C82, i_062c82),
    (0x062C88, i_062c88),
    (0x062C8E, i_062c8e),
    (0x062C92, i_062c92),
    (0x062C96, i_062c96),
    (0x062C9A, i_062c9a),
    (0x062C9C, i_062c9c),
    (0x062CA0, i_062ca0),
    (0x062CA4, i_062ca4),
    (0x062CA6, i_062ca6),
    (0x062CAA, i_062caa),
    (0x062CB0, i_062cb0),
    (0x062CB4, i_062cb4),
    (0x062CBA, i_062cba),
    (0x062CC0, i_062cc0),
    (0x062CC4, i_062cc4),
    (0x062CC8, i_062cc8),
    (0x062CCC, i_062ccc),
    (0x062CD2, i_062cd2),
    (0x062CD8, i_062cd8),
    (0x062CDC, i_062cdc),
    (0x062CE2, i_062ce2),
    (0x062CE6, i_062ce6),
    (0x062CEC, i_062cec),
    (0x062CF2, i_062cf2),
    (0x062CF6, i_062cf6),
    (0x062CFC, i_062cfc),
    (0x062D00, i_062d00),
    (0x062D06, i_062d06),
    (0x062D08, i_062d08),
    (0x062D0C, i_062d0c),
    (0x062D12, i_062d12),
    (0x062D16, i_062d16),
    (0x062D1C, i_062d1c),
    (0x062D20, i_062d20),
    (0x062D22, i_062d22),
    (0x062D28, i_062d28),
    (0x062D2C, i_062d2c),
    (0x062D32, i_062d32),
    (0x062D36, i_062d36),
    (0x062D3A, i_062d3a),
    (0x062D3C, i_062d3c),
    (0x062D40, i_062d40),
    (0x062D46, i_062d46),
    (0x062D4A, i_062d4a),
    (0x062D50, i_062d50),
    (0x062D56, i_062d56),
    (0x062D5A, i_062d5a),
    (0x062D60, i_062d60),
    (0x062D64, i_062d64),
    (0x062D68, i_062d68),
    (0x062D6C, i_062d6c),
    (0x062D6E, i_062d6e),
    (0x062D72, i_062d72),
    (0x062D76, i_062d76),
    (0x062D7C, i_062d7c),
    (0x062D82, i_062d82),
    (0x062D88, i_062d88),
    (0x062D8E, i_062d8e),
    (0x062D92, i_062d92),
    (0x062D94, i_062d94),
    (0x062D98, i_062d98),
    (0x062D9E, i_062d9e),
    (0x062DA4, i_062da4),
    (0x062DA8, i_062da8),
    (0x062DAC, i_062dac),
    (0x062DAE, i_062dae),
    (0x062DB4, i_062db4),
    (0x062DBA, i_062dba),
    (0x062DC0, i_062dc0),
    (0x062DC4, i_062dc4),
    (0x062DC8, i_062dc8),
    (0x062DCA, i_062dca),
    (0x062DCC, i_062dcc),
    (0x062DD2, i_062dd2),
    (0x062DD8, i_062dd8),
    (0x062DDE, i_062dde),
    (0x062DE4, i_062de4),
    (0x062DEA, i_062dea),
    (0x062DEE, i_062dee),
    (0x062DF0, i_062df0),
    (0x062DF4, i_062df4),
    (0x062DFA, i_062dfa),
    (0x062DFE, i_062dfe),
    (0x062E02, i_062e02),
    (0x062E04, i_062e04),
    (0x062E0A, i_062e0a),
    (0x062E10, i_062e10),
    (0x062E16, i_062e16),
    (0x062E1C, i_062e1c),
    (0x062E20, i_062e20),
    (0x062E24, i_062e24),
    (0x062E26, i_062e26),
    (0x062E2C, i_062e2c),
    (0x062E32, i_062e32),
    (0x062E38, i_062e38),
    (0x062E3C, i_062e3c),
    (0x062E40, i_062e40),
    (0x062E42, i_062e42),
    (0x062E44, i_062e44),
    (0x062E4A, i_062e4a),
    (0x062E50, i_062e50),
    (0x062E54, i_062e54),
    (0x062E56, i_062e56),
    (0x062E5C, i_062e5c),
    (0x062E62, i_062e62),
    (0x062E66, i_062e66),
    (0x062E6C, i_062e6c),
    (0x062E72, i_062e72),
    (0x062E78, i_062e78),
    (0x062E7E, i_062e7e),
    (0x062E82, i_062e82),
    (0x062E86, i_062e86),
    (0x062E88, i_062e88),
    (0x062E8A, i_062e8a),
    (0x062E90, i_062e90),
    (0x062E96, i_062e96),
    (0x062E9C, i_062e9c),
    (0x062EA2, i_062ea2),
    (0x062EA6, i_062ea6),
    (0x062EA8, i_062ea8),
    (0x062EAC, i_062eac),
    (0x062EB2, i_062eb2),
    (0x062EB8, i_062eb8),
    (0x062EBC, i_062ebc),
    (0x062EC0, i_062ec0),
    (0x062EC2, i_062ec2),
    (0x062EC8, i_062ec8),
    (0x062ECE, i_062ece),
    (0x062ED4, i_062ed4),
    (0x062ED8, i_062ed8),
    (0x062EDC, i_062edc),
    (0x062EDE, i_062ede),
    (0x062EE0, i_062ee0),
    (0x062EE4, i_062ee4),
    (0x062EEA, i_062eea),
    (0x062EF0, i_062ef0),
    (0x062EF6, i_062ef6),
    (0x062EFA, i_062efa),
    (0x062F00, i_062f00),
    (0x062F06, i_062f06),
    (0x062F0A, i_062f0a),
    (0x062F10, i_062f10),
    (0x062F14, i_062f14),
    (0x062F1A, i_062f1a),
    (0x062F20, i_062f20),
    (0x062F24, i_062f24),
    (0x062F28, i_062f28),
    (0x062F2C, i_062f2c),
    (0x062F30, i_062f30),
    (0x062F32, i_062f32),
    (0x062F38, i_062f38),
    (0x062F3C, i_062f3c),
    (0x062F40, i_062f40),
    (0x062F46, i_062f46),
    (0x062F4A, i_062f4a),
    (0x062F4E, i_062f4e),
    (0x062F54, i_062f54),
    (0x062F58, i_062f58),
    (0x062F5C, i_062f5c),
    (0x062F5E, i_062f5e),
    (0x062F64, i_062f64),
    (0x062F68, i_062f68),
    (0x062F6C, i_062f6c),
    (0x062F6E, i_062f6e),
    (0x062F74, i_062f74),
    (0x062F7A, i_062f7a),
    (0x062F80, i_062f80),
    (0x062F84, i_062f84),
    (0x062F88, i_062f88),
    (0x062F8A, i_062f8a),
    (0x062F8C, i_062f8c),
    (0x062F92, i_062f92),
    (0x062F98, i_062f98),
    (0x062F9A, i_062f9a),
    (0x062FA0, i_062fa0),
    (0x062FA6, i_062fa6),
    (0x062FA8, i_062fa8),
    (0x062FAC, i_062fac),
    (0x062FB2, i_062fb2),
    (0x062FB8, i_062fb8),
    (0x062FBE, i_062fbe),
    (0x062FC6, i_062fc6),
    (0x062FCC, i_062fcc),
    (0x062FD2, i_062fd2),
    (0x062FD6, i_062fd6),
    (0x062FDC, i_062fdc),
    (0x062FE2, i_062fe2),
    (0x062FE6, i_062fe6),
    (0x062FEC, i_062fec),
    (0x062FF2, i_062ff2),
    (0x062FF6, i_062ff6),
    (0x062FF8, i_062ff8),
    (0x062FFE, i_062ffe),
    (0x063002, i_063002),
    (0x063006, i_063006),
    (0x063008, i_063008),
    (0x06300E, i_06300e),
    (0x063014, i_063014),
    (0x06301A, i_06301a),
    (0x063020, i_063020),
    (0x063024, i_063024),
    (0x063028, i_063028),
    (0x06302A, i_06302a),
    (0x063030, i_063030),
    (0x063036, i_063036),
    (0x06303C, i_06303c),
    (0x063040, i_063040),
    (0x063044, i_063044),
    (0x063046, i_063046),
    (0x063048, i_063048),
    (0x06304C, i_06304c),
    (0x063052, i_063052),
    (0x063056, i_063056),
    (0x06305A, i_06305a),
    (0x063060, i_063060),
    (0x063066, i_063066),
    (0x06306C, i_06306c),
    (0x063072, i_063072),
    (0x063076, i_063076),
    (0x06307C, i_06307c),
    (0x063082, i_063082),
    (0x063088, i_063088),
    (0x06308C, i_06308c),
    (0x06308E, i_06308e),
    (0x063094, i_063094),
    (0x06309A, i_06309a),
    (0x06309E, i_06309e),
    (0x0630A2, i_0630a2),
    (0x0630A4, i_0630a4),
    (0x0630AA, i_0630aa),
    (0x0630B0, i_0630b0),
    (0x0630B4, i_0630b4),
    (0x0630B8, i_0630b8),
    (0x0630BA, i_0630ba),
    (0x0630BC, i_0630bc),
    (0x0630C0, i_0630c0),
    (0x0630C6, i_0630c6),
    (0x0630CC, i_0630cc),
    (0x0630D2, i_0630d2),
    (0x0630D6, i_0630d6),
    (0x0630D8, i_0630d8),
    (0x0630DE, i_0630de),
    (0x0630E4, i_0630e4),
    (0x0630E8, i_0630e8),
    (0x0630EC, i_0630ec),
    (0x0630EE, i_0630ee),
    (0x0630F4, i_0630f4),
    (0x0630FA, i_0630fa),
    (0x0630FE, i_0630fe),
    (0x063102, i_063102),
    (0x063104, i_063104),
    (0x063106, i_063106),
    (0x06310C, i_06310c),
    (0x063112, i_063112),
    (0x063116, i_063116),
    (0x063118, i_063118),
    (0x06311C, i_06311c),
    (0x063120, i_063120),
    (0x063126, i_063126),
    (0x06312C, i_06312c),
    (0x063130, i_063130),
    (0x063134, i_063134),
    (0x063138, i_063138),
    (0x06313A, i_06313a),
    (0x06313C, i_06313c),
    (0x063142, i_063142),
    (0x063148, i_063148),
    (0x06314E, i_06314e),
    (0x063154, i_063154),
    (0x06315A, i_06315a),
    (0x06315E, i_06315e),
    (0x063164, i_063164),
    (0x06316A, i_06316a),
    (0x063170, i_063170),
    (0x063174, i_063174),
    (0x06317A, i_06317a),
    (0x063180, i_063180),
    (0x063186, i_063186),
    (0x06318C, i_06318c),
    (0x063192, i_063192),
    (0x063196, i_063196),
    (0x06319C, i_06319c),
    (0x0631A0, i_0631a0),
    (0x0631A2, i_0631a2),
    (0x0631A8, i_0631a8),
    (0x0631AE, i_0631ae),
    (0x0631B2, i_0631b2),
    (0x0631B6, i_0631b6),
    (0x0631B8, i_0631b8),
    (0x0631BE, i_0631be),
    (0x0631C4, i_0631c4),
    (0x0631C8, i_0631c8),
    (0x0631CC, i_0631cc),
    (0x0631CE, i_0631ce),
    (0x0631D0, i_0631d0),
    (0x0631D6, i_0631d6),
    (0x0631DA, i_0631da),
    (0x0631E0, i_0631e0),
    (0x0631E6, i_0631e6),
    (0x0631EC, i_0631ec),
    (0x0631F2, i_0631f2),
    (0x0631F6, i_0631f6),
    (0x0631F8, i_0631f8),
    (0x0631FE, i_0631fe),
    (0x063204, i_063204),
    (0x063208, i_063208),
    (0x06320C, i_06320c),
    (0x06320E, i_06320e),
    (0x063214, i_063214),
    (0x06321A, i_06321a),
    (0x06321E, i_06321e),
    (0x063222, i_063222),
    (0x063224, i_063224),
    (0x063226, i_063226),
    (0x06322A, i_06322a),
    (0x063230, i_063230),
    (0x063236, i_063236),
    (0x06323C, i_06323c),
    (0x063240, i_063240),
    (0x063246, i_063246),
    (0x06324C, i_06324c),
    (0x063250, i_063250),
    (0x063254, i_063254),
    (0x06325A, i_06325a),
    (0x063260, i_063260),
    (0x063264, i_063264),
    (0x06326A, i_06326a),
    (0x063270, i_063270),
    (0x063276, i_063276),
    (0x06327C, i_06327c),
    (0x063280, i_063280),
    (0x063286, i_063286),
    (0x06328C, i_06328c),
    (0x063292, i_063292),
    (0x063298, i_063298),
    (0x06329C, i_06329c),
    (0x0632A0, i_0632a0),
    (0x0632A6, i_0632a6),
    (0x0632AC, i_0632ac),
    (0x0632B0, i_0632b0),
    (0x0632B4, i_0632b4),
    (0x0632BA, i_0632ba),
    (0x0632C0, i_0632c0),
    (0x0632C4, i_0632c4),
    (0x0632CA, i_0632ca),
    (0x0632D0, i_0632d0),
    (0x0632D4, i_0632d4),
    (0x0632DA, i_0632da),
    (0x0632E0, i_0632e0),
    (0x0632E6, i_0632e6),
    (0x0632EA, i_0632ea),
    (0x0632EC, i_0632ec),
    (0x0632F2, i_0632f2),
    (0x0632F8, i_0632f8),
    (0x0632FC, i_0632fc),
    (0x063300, i_063300),
    (0x063302, i_063302),
    (0x063308, i_063308),
    (0x06330E, i_06330e),
    (0x063312, i_063312),
    (0x063316, i_063316),
    (0x063318, i_063318),
    (0x06331A, i_06331a),
    (0x06331E, i_06331e),
    (0x063324, i_063324),
    (0x06332A, i_06332a),
    (0x063330, i_063330),
    (0x063336, i_063336),
    (0x06333A, i_06333a),
    (0x063340, i_063340),
    (0x063342, i_063342),
    (0x063348, i_063348),
    (0x06334E, i_06334e),
    (0x063352, i_063352),
    (0x063354, i_063354),
    (0x06335A, i_06335a),
    (0x063360, i_063360),
    (0x063364, i_063364),
    (0x063368, i_063368),
    (0x06336A, i_06336a),
    (0x06336C, i_06336c),
    (0x06336E, i_06336e),
    (0x063370, i_063370),
    (0x063374, i_063374),
    (0x06337A, i_06337a),
    (0x063380, i_063380),
    (0x063386, i_063386),
    (0x06338C, i_06338c),
    (0x063390, i_063390),
    (0x063394, i_063394),
    (0x063396, i_063396),
    (0x06339C, i_06339c),
    (0x0633A0, i_0633a0),
    (0x0633A4, i_0633a4),
    (0x0633A8, i_0633a8),
    (0x0633AE, i_0633ae),
    (0x0633B2, i_0633b2),
    (0x0633B4, i_0633b4),
    (0x0633B6, i_0633b6),
    (0x0633BA, i_0633ba),
    (0x0633BE, i_0633be),
    (0x0633C0, i_0633c0),
    (0x0633C4, i_0633c4),
    (0x0633C8, i_0633c8),
    (0x0633CA, i_0633ca),
    (0x0633CE, i_0633ce),
    (0x0633D2, i_0633d2),
    (0x0633D4, i_0633d4),
    (0x0633D8, i_0633d8),
    (0x0633DA, i_0633da),
    (0x0633DE, i_0633de),
    (0x0633E0, i_0633e0),
    (0x0633E4, i_0633e4),
    (0x0633EA, i_0633ea),
    (0x0633EE, i_0633ee),
    (0x0633F4, i_0633f4),
    (0x0633F6, i_0633f6),
    (0x0633FC, i_0633fc),
    (0x063400, i_063400),
    (0x063402, i_063402),
    (0x063406, i_063406),
    (0x063408, i_063408),
    (0x06340C, i_06340c),
    (0x06340E, i_06340e),
    (0x063414, i_063414),
    (0x063418, i_063418),
    (0x06341E, i_06341e),
    (0x063422, i_063422),
    (0x063426, i_063426),
    (0x063428, i_063428),
    (0x06342C, i_06342c),
    (0x06342E, i_06342e),
    (0x063432, i_063432),
    (0x063438, i_063438),
    (0x06343E, i_06343e),
    (0x063442, i_063442),
    (0x063446, i_063446),
    (0x06344A, i_06344a),
    (0x06344E, i_06344e),
    (0x063452, i_063452),
    (0x063456, i_063456),
    (0x06345C, i_06345c),
    (0x063462, i_063462),
    (0x063466, i_063466),
    (0x06346A, i_06346a),
    (0x06346E, i_06346e),
    (0x063472, i_063472),
    (0x063478, i_063478),
    (0x06347C, i_06347c),
    (0x06347E, i_06347e),
    (0x063482, i_063482),
    (0x063488, i_063488),
    (0x06348C, i_06348c),
    (0x063490, i_063490),
    (0x063492, i_063492),
    (0x063498, i_063498),
    (0x06349C, i_06349c),
    (0x0634A0, i_0634a0),
    (0x0634A4, i_0634a4),
    (0x0634A8, i_0634a8),
    (0x0634AC, i_0634ac),
    (0x0634B2, i_0634b2),
    (0x0634B8, i_0634b8),
    (0x0634BC, i_0634bc),
    (0x0634C0, i_0634c0),
    (0x0634C4, i_0634c4),
    (0x0634CA, i_0634ca),
    (0x0634CE, i_0634ce),
    (0x0634D2, i_0634d2),
    (0x0634D8, i_0634d8),
    (0x0634DE, i_0634de),
    (0x0634E2, i_0634e2),
    (0x0634E6, i_0634e6),
    (0x0634EA, i_0634ea),
    (0x0634F0, i_0634f0),
    (0x0634F4, i_0634f4),
    (0x0634F6, i_0634f6),
    (0x0634FA, i_0634fa),
    (0x063500, i_063500),
    (0x063506, i_063506),
    (0x06350C, i_06350c),
    (0x063510, i_063510),
    (0x063516, i_063516),
    (0x06351C, i_06351c),
    (0x063522, i_063522),
    (0x063524, i_063524),
    (0x063528, i_063528),
    (0x06352E, i_06352e),
    (0x063534, i_063534),
    (0x063538, i_063538),
];
