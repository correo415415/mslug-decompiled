// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $04F002: move.w #$bfff, $38(a6)
pub fn i_04f002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F008;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F008: lea.l $4f00e(pc), a1
pub fn i_04f008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F00C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F00E;
    16
}
/// $04F00C: move.l a1, (a6)
pub fn i_04f00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F00E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F00E: jsr $2783a.l
pub fn i_04f00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F014);
    cpu.pc = 0x2783A;
    16
}
/// $04F014: jsr $28d70.l
pub fn i_04f014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F01A);
    cpu.pc = 0x28D70;
    16
}
/// $04F01A: movea.l $c(a6), a0
pub fn i_04f01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F01E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04F01E: cmpi.b #$88, $20(a0)
pub fn i_04f01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F024;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04F024: bne.w $4f02e
pub fn i_04f024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F028;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F02E; return 10; }
    8
}
/// $04F028: jmp $518.l
pub fn i_04f028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F02E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F02E: rts
pub fn i_04f02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F030;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FortressTurretMount_04f030  ($04F030, 128 B, props/fortress_04e580.s)
/// $04F030: move.w #$40, d1
pub fn i_04f030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F034;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F034: jsr $236e.l
pub fn i_04f034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F03A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F03A);
    cpu.pc = 0x236E;
    16
}
/// $04F03A: move.w #$60, $70(a6)
pub fn i_04f03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F040;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F040: lea.l $29466c.l, a0
pub fn i_04f040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F046;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29466C;
    20
}
/// $04F046: jsr $28cd4.l
pub fn i_04f046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F04C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F04C);
    cpu.pc = 0x28CD4;
    16
}
/// $04F04C: move.w #$bfff, $38(a6)
pub fn i_04f04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F052;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F052: lea.l $4f058(pc), a1
pub fn i_04f052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F056;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F058;
    16
}
/// $04F056: move.l a1, (a6)
pub fn i_04f056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F058;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F058: jsr $2783a.l
pub fn i_04f058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F05E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F05E);
    cpu.pc = 0x2783A;
    16
}
/// $04F05E: move.l $106f50.l, d0
pub fn i_04f05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F064;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04F064: swap d0
pub fn i_04f064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F066;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04F066: cmpi.w #$a70, d0
pub fn i_04f066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F06A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA70, Size::Word, false);
    8
}
/// $04F06A: blt.w $4f074
pub fn i_04f06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F06E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4F074; return 10; }
    8
}
/// $04F06E: jsr $28d70.l
pub fn i_04f06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F074);
    cpu.pc = 0x28D70;
    16
}
/// $04F074: movea.l $c(a6), a0
pub fn i_04f074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F078;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04F078: cmpi.b #$ff, $20(a0)
pub fn i_04f078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F07E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04F07E: bne.w $4f0a0
pub fn i_04f07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F082;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F0A0; return 10; }
    8
}
/// $04F082: lea.l $295c32.l, a1
pub fn i_04f082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F088;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C32;
    20
}
/// $04F088: jsr $77c7e.l
pub fn i_04f088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F08E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F08E);
    cpu.pc = 0x77C7E;
    16
}
/// $04F08E: lea.l $295cd4.l, a1
pub fn i_04f08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F094;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CD4;
    20
}
/// $04F094: jsr $77c7e.l
pub fn i_04f094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F09A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F09A);
    cpu.pc = 0x77C7E;
    16
}
/// $04F09A: lea.l $4f0b0(pc), a1
pub fn i_04f09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F09E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F0B0;
    16
}
/// $04F09E: move.l a1, (a6)
pub fn i_04f09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F0A0: jsr $4fa70(pc)
pub fn i_04f0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F0A4);
    cpu.pc = 0x4FA70;
    16
}
/// $04F0A4: bcc.w $4f0ae
pub fn i_04f0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F0AE; return 10; }
    8
}
/// $04F0A8: jmp $518.l
pub fn i_04f0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0AE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F0AE: rts
pub fn i_04f0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FortressTurretMountFall_04f0b0  ($04F0B0, 136 B, props/fortress_04e580.s)
/// $04F0B0: lea.l $29467c.l, a0
pub fn i_04f0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29467C;
    20
}
/// $04F0B6: jsr $28cd4.l
pub fn i_04f0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F0BC);
    cpu.pc = 0x28CD4;
    16
}
/// $04F0BC: jsr $267e2.l
pub fn i_04f0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F0C2);
    cpu.pc = 0x267E2;
    16
}
/// $04F0C2: move.w #$fff0, $2e(a6)
pub fn i_04f0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F0C8: lea.l $4f0ce(pc), a1
pub fn i_04f0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F0CE;
    16
}
/// $04F0CC: move.l a1, (a6)
pub fn i_04f0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F0CE: jsr $27cee.l
pub fn i_04f0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F0D4);
    cpu.pc = 0x27CEE;
    16
}
/// $04F0D4: jsr $28d70.l
pub fn i_04f0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F0DA);
    cpu.pc = 0x28D70;
    16
}
/// $04F0DA: cmpi.w #$160, $24(a6)
pub fn i_04f0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0E0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $04F0E0: bgt.w $4f128
pub fn i_04f0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0E4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4F128; return 10; }
    8
}
/// $04F0E4: move.w #$1030, d0
pub fn i_04f0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0E8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F0E8: jsr $2352.l
pub fn i_04f0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F0EE);
    cpu.pc = 0x2352;
    16
}
/// $04F0EE: subi.w #$20, $24(a6)
pub fn i_04f0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0F4;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F0F4: lea.l $295c32.l, a1
pub fn i_04f0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F0FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C32;
    20
}
/// $04F0FA: jsr $77c7e.l
pub fn i_04f0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F100);
    cpu.pc = 0x77C7E;
    16
}
/// $04F100: addi.w #$20, $24(a0)
pub fn i_04f100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F106;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F106: lea.l $295c44.l, a1
pub fn i_04f106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F10C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C44;
    20
}
/// $04F10C: jsr $77c7e.l
pub fn i_04f10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F112;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F112);
    cpu.pc = 0x77C7E;
    16
}
/// $04F112: addi.w #$20, $24(a0)
pub fn i_04f112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F118;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F118: lea.l $295cd4.l, a1
pub fn i_04f118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F11E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CD4;
    20
}
/// $04F11E: jsr $77c7e.l
pub fn i_04f11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F124);
    cpu.pc = 0x77C7E;
    16
}
/// $04F124: bra.w $4f130
pub fn i_04f124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F128;
    cpu.ir = 0x6000;
    cpu.pc = 0x4F130;
    10
}
/// $04F128: jsr $4fa70(pc)
pub fn i_04f128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F12C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F12C);
    cpu.pc = 0x4FA70;
    16
}
/// $04F12C: bcc.w $4f136
pub fn i_04f12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F130;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F136; return 10; }
    8
}
/// $04F130: jmp $518.l
pub fn i_04f130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F136;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F136: rts
pub fn i_04f136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F138;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FortressSide_04f138  ($04F138, 188 B, props/fortress_04e580.s)
/// $04F138: move.w #$24, d1
pub fn i_04f138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F13C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F13C: jsr $236e.l
pub fn i_04f13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F142;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F142);
    cpu.pc = 0x236E;
    16
}
/// $04F142: move.w #$60, $70(a6)
pub fn i_04f142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F148;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F148: addi.w #$a8, $22(a6)
pub fn i_04f148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F14E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F14E: addi.w #$0, $24(a6)
pub fn i_04f14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F154;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F154: move.w #$0, $38(a6)
pub fn i_04f154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F15A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F15A: lea.l $4f160(pc), a1
pub fn i_04f15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F15E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F160;
    16
}
/// $04F15E: move.l a1, (a6)
pub fn i_04f15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F160;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F160: jsr $2783a.l
pub fn i_04f160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F166);
    cpu.pc = 0x2783A;
    16
}
/// $04F166: movea.l $c(a6), a0
pub fn i_04f166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F16A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04F16A: cmpi.b #$ff, $20(a0)
pub fn i_04f16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F170;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04F170: bne.w $4f1ae
pub fn i_04f170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F174;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F1AE; return 10; }
    8
}
/// $04F174: lea.l $295f4a.l, a1
pub fn i_04f174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F17A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295F4A;
    20
}
/// $04F17A: jsr $43fac.l
pub fn i_04f17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F180;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F180);
    cpu.pc = 0x43FAC;
    16
}
/// $04F180: subi.w #$50, $22(a6)
pub fn i_04f180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F186;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F186: lea.l $295b5a.l, a1
pub fn i_04f186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F18C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B5A;
    20
}
/// $04F18C: jsr $77c7e.l
pub fn i_04f18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F192);
    cpu.pc = 0x77C7E;
    16
}
/// $04F192: lea.l $295b6c.l, a1
pub fn i_04f192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F198;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B6C;
    20
}
/// $04F198: jsr $77c7e.l
pub fn i_04f198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F19E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F19E);
    cpu.pc = 0x77C7E;
    16
}
/// $04F19E: lea.l $295cb0.l, a1
pub fn i_04f19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1A4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04F1A4: jsr $77c7e.l
pub fn i_04f1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F1AA);
    cpu.pc = 0x77C7E;
    16
}
/// $04F1AA: bra.w $4f1ec
pub fn i_04f1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4F1EC;
    10
}
/// $04F1AE: move.w $72(a0), d0
pub fn i_04f1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1B2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04F1B2: andi.l #$3, d0
pub fn i_04f1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1B8;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $04F1B8: movea.l #$294610, a0
pub fn i_04f1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1BE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x294610;
    cpu.a[0] = v1;
    4
}
/// $04F1BE: lsl.w #$2, d0
pub fn i_04f1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1C0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04F1C0: movea.l (a0, d0.w), a0
pub fn i_04f1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1C4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04F1C4: cmpa.l #$ffffffff, a0
pub fn i_04f1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1CA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04F1CA: beq.w $4f1d4
pub fn i_04f1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4F1D4; return 10; }
    8
}
/// $04F1CE: jsr $28cd4.l
pub fn i_04f1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F1D4);
    cpu.pc = 0x28CD4;
    16
}
/// $04F1D4: cmpi.w #$190, $22(a6)
pub fn i_04f1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1DA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x190, Size::Word, false);
    16
}
/// $04F1DA: bgt.w $4f1e4
pub fn i_04f1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4F1E4; return 10; }
    8
}
/// $04F1DE: jsr $28d70.l
pub fn i_04f1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F1E4);
    cpu.pc = 0x28D70;
    16
}
/// $04F1E4: jsr $4fa70(pc)
pub fn i_04f1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F1E8);
    cpu.pc = 0x4FA70;
    16
}
/// $04F1E8: bcc.w $4f1f2
pub fn i_04f1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F1F2; return 10; }
    8
}
/// $04F1EC: jmp $518.l
pub fn i_04f1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F1F2: rts
pub fn i_04f1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BlitSequence_04f1f4  ($04F1F4, 176 B, props/fortress_04e580.s)
/// $04F1F4: move.b #$0, $20(a6)
pub fn i_04f1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F1FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F1FA: move.w #$1e, $66(a6)
pub fn i_04f1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F200;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F200: lea.l $4f206(pc), a1
pub fn i_04f200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F204;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F206;
    16
}
/// $04F204: move.l a1, (a6)
pub fn i_04f204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F206;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F206: movea.l $c(a6), a0
pub fn i_04f206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F20A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04F20A: cmpi.b #$ff, $20(a0)
pub fn i_04f20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F210;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04F210: beq.w $4f284
pub fn i_04f210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F214;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4F284; return 10; }
    8
}
/// $04F214: jsr $2783a.l
pub fn i_04f214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F21A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F21A);
    cpu.pc = 0x2783A;
    16
}
/// $04F21A: jsr $28d70.l
pub fn i_04f21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F220);
    cpu.pc = 0x28D70;
    16
}
/// $04F220: jsr $2870a.l
pub fn i_04f220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F226);
    cpu.pc = 0x2870A;
    16
}
/// $04F226: bcc.w $4f23c
pub fn i_04f226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F22A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F23C; return 10; }
    8
}
/// $04F22A: lea.l $5e766.l, a0
pub fn i_04f22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F230;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04F230: jsr $5e770.l
pub fn i_04f230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F236);
    cpu.pc = 0x5E770;
    16
}
/// $04F236: bclr.b #$3, $13(a6)
pub fn i_04f236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F23C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04F23C: jsr $28758.l
pub fn i_04f23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F242);
    cpu.pc = 0x28758;
    16
}
/// $04F242: bcc.w $4f294
pub fn i_04f242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F246;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F294; return 10; }
    8
}
/// $04F246: move.w #$1e, $66(a6)
pub fn i_04f246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F24C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F24C: bclr.b #$0, $13(a6)
pub fn i_04f24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F252;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04F252: move.b $20(a6), d0
pub fn i_04f252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F256;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04F256: andi.l #$ff, d0
pub fn i_04f256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F25C;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $04F25C: asl.l #$2, d0
pub fn i_04f25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F25E;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $04F25E: movea.l $70(a6, d0.w), a2
pub fn i_04f25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F262;
    cpu.ir = 0x2476;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $04F262: jsr $5022a(pc)
pub fn i_04f262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F266;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F266);
    cpu.pc = 0x5022A;
    16
}
/// $04F266: addq.b #$1, $20(a6)
pub fn i_04f266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F26A;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04F26A: move.b $20(a6), d0
pub fn i_04f26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F26E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04F26E: andi.l #$ff, d0
pub fn i_04f26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F274;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $04F274: asl.l #$2, d0
pub fn i_04f274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F276;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $04F276: move.l $70(a6, d0.w), d0
pub fn i_04f276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F27A;
    cpu.ir = 0x2036;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $04F27A: cmpi.l #$ffffffff, d0
pub fn i_04f27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F280;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $04F280: bne.w $4f294
pub fn i_04f280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F284;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F294; return 10; }
    8
}
/// $04F284: move.b $21(a6), d0
pub fn i_04f284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F288;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04F288: movea.l $c(a6), a0
pub fn i_04f288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F28C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04F28C: or.b d0, $21(a0)
pub fn i_04f28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F290;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04F290: bra.w $4f29c
pub fn i_04f290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F294;
    cpu.ir = 0x6000;
    cpu.pc = 0x4F29C;
    10
}
/// $04F294: jsr $4fa70(pc)
pub fn i_04f294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F298;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F298);
    cpu.pc = 0x4FA70;
    16
}
/// $04F298: bcc.w $4f2a2
pub fn i_04f298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F29C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F2A2; return 10; }
    8
}
/// $04F29C: jmp $518.l
pub fn i_04f29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F2A2: rts
pub fn i_04f2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SlotPrioCheckRts_04f2a4  ($04F2A4, 30 B, props/fortress_04e580.s)
/// $04F2A4: jsr $2783a.l
pub fn i_04f2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F2AA);
    cpu.pc = 0x2783A;
    16
}
/// $04F2AA: movea.l $8(a6), a1
pub fn i_04f2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2AE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04F2AE: move.b $10(a6), d0
pub fn i_04f2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2B2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04F2B2: cmp.b $10(a1), d0
pub fn i_04f2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2B6;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04F2B6: bcs.w $4f2c0
pub fn i_04f2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2BA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4F2C0; return 10; }
    8
}
/// $04F2BA: jmp $518.l
pub fn i_04f2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2C0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F2C0: rts
pub fn i_04f2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Roof_04f2c2  ($04F2C2, 130 B, props/fortress_04e580.s)
/// $04F2C2: move.w #$3d, d1
pub fn i_04f2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F2C6: jsr $236e.l
pub fn i_04f2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F2CC);
    cpu.pc = 0x236E;
    16
}
/// $04F2CC: move.b #$ff, $32(a6)
pub fn i_04f2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F2D2: move.b #$ff, $33(a6)
pub fn i_04f2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F2D8: move.w #$28, $70(a6)
pub fn i_04f2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F2DE: move.b #$0, $3a(a6)
pub fn i_04f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F2E4: move.w #$14, $66(a6)
pub fn i_04f2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F2EA: lea.l $294254.l, a0
pub fn i_04f2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294254;
    20
}
/// $04F2F0: jsr $28cd4.l
pub fn i_04f2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F2F6);
    cpu.pc = 0x28CD4;
    16
}
/// $04F2F6: lea.l $4f2fc(pc), a1
pub fn i_04f2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2FC;
    16
}
/// $04F2FA: move.l a1, (a6)
pub fn i_04f2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F2FC: jsr $2783a.l
pub fn i_04f2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F302);
    cpu.pc = 0x2783A;
    16
}
/// $04F302: jsr $28d70.l
pub fn i_04f302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F308);
    cpu.pc = 0x28D70;
    16
}
/// $04F308: jsr $2870a.l
pub fn i_04f308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F30E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F30E);
    cpu.pc = 0x2870A;
    16
}
/// $04F30E: bcc.w $4f324
pub fn i_04f30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F312;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F324; return 10; }
    8
}
/// $04F312: lea.l $5e766.l, a0
pub fn i_04f312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F318;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04F318: jsr $5e770.l
pub fn i_04f318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F31E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F31E);
    cpu.pc = 0x5E770;
    16
}
/// $04F31E: bclr.b #$3, $13(a6)
pub fn i_04f31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F324;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04F324: jsr $28758.l
pub fn i_04f324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F32A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F32A);
    cpu.pc = 0x28758;
    16
}
/// $04F32A: bcc.w $4f334
pub fn i_04f32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F32E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F334; return 10; }
    8
}
/// $04F32E: lea.l $4f344(pc), a1
pub fn i_04f32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F332;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F344;
    16
}
/// $04F332: move.l a1, (a6)
pub fn i_04f332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F334;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F334: jsr $4fa70(pc)
pub fn i_04f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F338;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F338);
    cpu.pc = 0x4FA70;
    16
}
/// $04F338: bcc.w $4f342
pub fn i_04f338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F33C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F342; return 10; }
    8
}
/// $04F33C: jmp $518.l
pub fn i_04f33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F342;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F342: rts
pub fn i_04f342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F344;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_RoofFall_04f344  ($04F344, 98 B, props/fortress_04e580.s)
/// $04F344: lea.l $295fba.l, a0
pub fn i_04f344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F34A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x295FBA;
    20
}
/// $04F34A: move.l a0, $4c(a6)
pub fn i_04f34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F34E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F34E: jsr $283ca.l
pub fn i_04f34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F354;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F354);
    cpu.pc = 0x283CA;
    16
}
/// $04F354: jsr $283ca.l
pub fn i_04f354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F35A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F35A);
    cpu.pc = 0x283CA;
    16
}
/// $04F35A: jsr $283d8.l
pub fn i_04f35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F360);
    cpu.pc = 0x283D8;
    16
}
/// $04F360: lea.l $29426a.l, a0
pub fn i_04f360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F366;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29426A;
    20
}
/// $04F366: jsr $28cd4.l
pub fn i_04f366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F36C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F36C);
    cpu.pc = 0x28CD4;
    16
}
/// $04F36C: lea.l $4f372(pc), a1
pub fn i_04f36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F370;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F372;
    16
}
/// $04F370: move.l a1, (a6)
pub fn i_04f370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F372;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F372: jsr $2783a.l
pub fn i_04f372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F378);
    cpu.pc = 0x2783A;
    16
}
/// $04F378: jsr $28d70.l
pub fn i_04f378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F37E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F37E);
    cpu.pc = 0x28D70;
    16
}
/// $04F37E: bcc.w $4f3ac
pub fn i_04f37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F382;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F3AC; return 10; }
    8
}
/// $04F382: lea.l $ffff.w, a0
pub fn i_04f382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F386;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04F386: move.l a0, $4c(a6)
pub fn i_04f386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F38A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F38A: jsr $283ca.l
pub fn i_04f38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F390);
    cpu.pc = 0x283CA;
    16
}
/// $04F390: move.w #$1028, d0
pub fn i_04f390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F394;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F394: jsr $2352.l
pub fn i_04f394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F39A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F39A);
    cpu.pc = 0x2352;
    16
}
/// $04F39A: lea.l $295bd8.l, a1
pub fn i_04f39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BD8;
    20
}
/// $04F3A0: jsr $77c7e.l
pub fn i_04f3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F3A6);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetTaskHandler_04f3a6  ($04F3A6, 8 B, kernel/task_handlers.c)
/// $04F3A6: lea.l $4f2a4(pc), a1
pub fn i_04f3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04F3AA: move.l a1, (a6)
pub fn i_04f3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F3AC: rts
pub fn i_04f3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink3_PhaseA_04f3ae  ($04F3AE, 94 B, props/fortress_04e580.s)
/// $04F3AE: move.w #$1e, d1
pub fn i_04f3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3B2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3B2: move.w #$111, d2
pub fn i_04f3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3B6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x111;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3B6: move.w #$ffff, d3
pub fn i_04f3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3BA;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3BA: move.w #$1, d4
pub fn i_04f3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3BE;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3BE: jsr $2c26.l
pub fn i_04f3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F3C4);
    cpu.pc = 0x2C26;
    16
}
/// $04F3C4: move.w #$1, $72(a6)
pub fn i_04f3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F3CA: lea.l $4f3d0(pc), a1
pub fn i_04f3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F3D0;
    16
}
/// $04F3CE: move.l a1, (a6)
pub fn i_04f3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F3D0: subq.w #$1, $72(a6)
pub fn i_04f3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3D4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04F3D4: bne.w $4f3de
pub fn i_04f3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F3DE; return 10; }
    8
}
/// $04F3D8: lea.l $4f40c(pc), a1
pub fn i_04f3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F40C;
    16
}
/// $04F3DC: move.l a1, (a6)
pub fn i_04f3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F3DE: move.l $106f5c.l, d0
pub fn i_04f3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3E4;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04F3E4: swap d0
pub fn i_04f3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3E6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04F3E6: cmpi.w #$280, d0
pub fn i_04f3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3EA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x280, Size::Word, false);
    8
}
/// $04F3EA: blt.w $4f40a
pub fn i_04f3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3EE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4F40A; return 10; }
    8
}
/// $04F3EE: move.w #$1e, d1
pub fn i_04f3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3F2: move.w #$ce, d2
pub fn i_04f3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3F6;
    cpu.ir = 0x343C;
    let v1: u32 = 0xCE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3F6: move.w #$ffff, d3
pub fn i_04f3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3FA;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3FA: move.w #$1, d4
pub fn i_04f3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F3FE;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F3FE: jsr $2c26.l
pub fn i_04f3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F404;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F404);
    cpu.pc = 0x2C26;
    16
}
/// $04F404: jmp $518.l
pub fn i_04f404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F40A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F40A: rts
pub fn i_04f40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F40C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink3_PhaseB_04f40c  ($04F40C, 94 B, props/fortress_04e580.s)
/// $04F40C: move.w #$1e, d1
pub fn i_04f40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F410;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F410: move.w #$ce, d2
pub fn i_04f410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F414;
    cpu.ir = 0x343C;
    let v1: u32 = 0xCE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F414: move.w #$ffff, d3
pub fn i_04f414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F418;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F418: move.w #$1, d4
pub fn i_04f418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F41C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F41C: jsr $2c26.l
pub fn i_04f41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F422);
    cpu.pc = 0x2C26;
    16
}
/// $04F422: move.w #$1, $72(a6)
pub fn i_04f422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F428;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F428: lea.l $4f42e(pc), a1
pub fn i_04f428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F42C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F42E;
    16
}
/// $04F42C: move.l a1, (a6)
pub fn i_04f42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F42E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F42E: subq.w #$1, $72(a6)
pub fn i_04f42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F432;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04F432: bne.w $4f43c
pub fn i_04f432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F436;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F43C; return 10; }
    8
}
/// $04F436: lea.l $4f3ae(pc), a1
pub fn i_04f436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F43A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F3AE;
    16
}
/// $04F43A: move.l a1, (a6)
pub fn i_04f43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F43C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F43C: move.l $106f5c.l, d0
pub fn i_04f43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F442;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04F442: swap d0
pub fn i_04f442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F444;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04F444: cmpi.w #$280, d0
pub fn i_04f444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F448;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x280, Size::Word, false);
    8
}
/// $04F448: blt.w $4f468
pub fn i_04f448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F44C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4F468; return 10; }
    8
}
/// $04F44C: move.w #$1e, d1
pub fn i_04f44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F450;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F450: move.w #$ce, d2
pub fn i_04f450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F454;
    cpu.ir = 0x343C;
    let v1: u32 = 0xCE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F454: move.w #$ffff, d3
pub fn i_04f454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F458;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F458: move.w #$1, d4
pub fn i_04f458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F45C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F45C: jsr $2c26.l
pub fn i_04f45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F462);
    cpu.pc = 0x2C26;
    16
}
/// $04F462: jmp $518.l
pub fn i_04f462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F468;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F468: rts
pub fn i_04f468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F46A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Sensor_04f46a  ($04F46A, 16 B, props/fortress_04e580.s)
/// $04F46A: movea.l $3c(a6), a1
pub fn i_04f46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F46E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04F46E: jsr $2942a.l
pub fn i_04f46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F474;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F474);
    cpu.pc = 0x2942A;
    16
}
/// $04F474: move.w #$90, $70(a6)
pub fn i_04f474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F47A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Prop_SensorWait_04f47a  ($04F47A, 60 B, props/fortress_04e580.s)
/// $04F47A: lea.l $4f480(pc), a1
pub fn i_04f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F47E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F480;
    16
}
/// $04F47E: move.l a1, (a6)
pub fn i_04f47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F480: jsr $2783a.l
pub fn i_04f480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F486);
    cpu.pc = 0x2783A;
    16
}
/// $04F486: lea.l $295fb0.l, a0
pub fn i_04f486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F48C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x295FB0;
    20
}
/// $04F48C: jsr $5e086.l
pub fn i_04f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F492);
    cpu.pc = 0x5E086;
    16
}
/// $04F492: bcs.w $4f4a6
pub fn i_04f492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F496;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4F4A6; return 10; }
    8
}
/// $04F496: jsr $31fc2.l
pub fn i_04f496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F49C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F49C);
    cpu.pc = 0x31FC2;
    16
}
/// $04F49C: beq.w $4f4a6
pub fn i_04f49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4F4A6; return 10; }
    8
}
/// $04F4A0: lea.l $4f4b6(pc), a1
pub fn i_04f4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F4B6;
    16
}
/// $04F4A4: move.l a1, (a6)
pub fn i_04f4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F4A6: jsr $4fa70(pc)
pub fn i_04f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F4AA);
    cpu.pc = 0x4FA70;
    16
}
/// $04F4AA: bcc.w $4f4b4
pub fn i_04f4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F4B4; return 10; }
    8
}
/// $04F4AE: jmp $518.l
pub fn i_04f4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4B4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F4B4: rts
pub fn i_04f4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SensorTriggered_04f4b6  ($04F4B6, 56 B, props/fortress_04e580.s)
/// $04F4B6: lea.l $2948a6.l, a0
pub fn i_04f4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2948A6;
    20
}
/// $04F4BC: jsr $28cd4.l
pub fn i_04f4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F4C2);
    cpu.pc = 0x28CD4;
    16
}
/// $04F4C2: lea.l $4f4c8(pc), a1
pub fn i_04f4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F4C8;
    16
}
/// $04F4C6: move.l a1, (a6)
pub fn i_04f4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F4C8: jsr $2783a.l
pub fn i_04f4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F4CE);
    cpu.pc = 0x2783A;
    16
}
/// $04F4CE: jsr $28d70.l
pub fn i_04f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F4D4);
    cpu.pc = 0x28D70;
    16
}
/// $04F4D4: bcc.w $4f4de
pub fn i_04f4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F4DE; return 10; }
    8
}
/// $04F4D8: lea.l $4f47a(pc), a1
pub fn i_04f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F47A;
    16
}
/// $04F4DC: move.l a1, (a6)
pub fn i_04f4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F4DE: jsr $4fa70(pc)
pub fn i_04f4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F4E2);
    cpu.pc = 0x4FA70;
    16
}
/// $04F4E2: bcc.w $4f4ec
pub fn i_04f4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F4EC; return 10; }
    8
}
/// $04F4E6: jmp $518.l
pub fn i_04f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F4EC: rts
pub fn i_04f4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Crate_04f4ee  ($04F4EE, 200 B, props/fortress_04e580.s)
/// $04F4EE: movea.l $3c(a6), a1
pub fn i_04f4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4F2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04F4F2: jsr $2942a.l
pub fn i_04f4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F4F8);
    cpu.pc = 0x2942A;
    16
}
/// $04F4F8: move.w #$20, $70(a6)
pub fn i_04f4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4FE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F4FE: move.b #$ff, $32(a6)
pub fn i_04f4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F504;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F504: move.b #$ff, $33(a6)
pub fn i_04f504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F50A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F50A: move.b #$0, $3a(a6)
pub fn i_04f50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F510;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F510: move.w #$14, $66(a6)
pub fn i_04f510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F516;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F516: lea.l $295160.l, a0
pub fn i_04f516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F51C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x295160;
    20
}
/// $04F51C: move.l a0, $48(a6)
pub fn i_04f51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F520;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F520: lea.l $4f526(pc), a1
pub fn i_04f520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F524;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F526;
    16
}
/// $04F524: move.l a1, (a6)
pub fn i_04f524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F526;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F526: jsr $2783a.l
pub fn i_04f526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F52C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F52C);
    cpu.pc = 0x2783A;
    16
}
/// $04F52C: jsr $2870a.l
pub fn i_04f52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F532);
    cpu.pc = 0x2870A;
    16
}
/// $04F532: bcc.w $4f548
pub fn i_04f532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F536;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F548; return 10; }
    8
}
/// $04F536: lea.l $5e766.l, a0
pub fn i_04f536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F53C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04F53C: jsr $5e770.l
pub fn i_04f53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F542);
    cpu.pc = 0x5E770;
    16
}
/// $04F542: bclr.b #$3, $13(a6)
pub fn i_04f542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F548;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04F548: jsr $28758.l
pub fn i_04f548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F54E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F54E);
    cpu.pc = 0x28758;
    16
}
/// $04F54E: bcc.w $4f5a6
pub fn i_04f54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F552;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F5A6; return 10; }
    8
}
/// $04F552: lea.l $78066.l, a1
pub fn i_04f552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F558;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78066;
    20
}
/// $04F558: jsr $4ae.l
pub fn i_04f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F55E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F55E);
    cpu.pc = 0x4AE;
    16
}
/// $04F55E: jsr $5dd22.l
pub fn i_04f55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F564);
    cpu.pc = 0x5DD22;
    16
}
/// $04F564: addi.w #$28, $22(a0)
pub fn i_04f564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F56A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F56A: lea.l $2957ae.l, a2
pub fn i_04f56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F570;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2957AE;
    20
}
/// $04F570: jsr $5022a(pc)
pub fn i_04f570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F574;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F574);
    cpu.pc = 0x5022A;
    16
}
/// $04F574: lea.l $295f5e.l, a1
pub fn i_04f574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F57A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295F5E;
    20
}
/// $04F57A: jsr $43fac.l
pub fn i_04f57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F580;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F580);
    cpu.pc = 0x43FAC;
    16
}
/// $04F580: lea.l $29605e.l, a0
pub fn i_04f580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F586;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29605E;
    20
}
/// $04F586: move.l a0, $4c(a6)
pub fn i_04f586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F58A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F58A: jsr $283ca.l
pub fn i_04f58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F590);
    cpu.pc = 0x283CA;
    16
}
/// $04F590: jsr $283ca.l
pub fn i_04f590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F596);
    cpu.pc = 0x283CA;
    16
}
/// $04F596: jsr $283d8.l
pub fn i_04f596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F59C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F59C);
    cpu.pc = 0x283D8;
    16
}
/// $04F59C: lea.l $4f5b6(pc), a1
pub fn i_04f59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F5B6;
    16
}
/// $04F5A0: move.l a1, (a6)
pub fn i_04f5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F5A2: bra.w $4f5b4
pub fn i_04f5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4F5B4;
    10
}
/// $04F5A6: jsr $4fa70(pc)
pub fn i_04f5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F5AA);
    cpu.pc = 0x4FA70;
    16
}
/// $04F5AA: bcc.w $4f5b4
pub fn i_04f5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F5B4; return 10; }
    8
}
/// $04F5AE: jmp $518.l
pub fn i_04f5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5B4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F5B4: rts
pub fn i_04f5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_04f5b6  ($04F5B6, 8 B, kernel/jmp_to_scheduler.c)
/// $04F5B6: jmp $518.l
pub fn i_04f5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5BC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Debris_Bouncer_04f5be  ($04F5BE, 142 B, props/fortress_04e580.s)
/// $04F5BE: jsr $5e7c0.l
pub fn i_04f5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F5C4);
    cpu.pc = 0x5E7C0;
    16
}
/// $04F5C4: move.w #$126, d1
pub fn i_04f5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F5C8: jsr $236e.l
pub fn i_04f5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F5CE);
    cpu.pc = 0x236E;
    16
}
/// $04F5CE: move.b #$ff, $32(a6)
pub fn i_04f5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F5D4: move.b #$ff, $33(a6)
pub fn i_04f5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F5DA: move.b #$0, $3a(a6)
pub fn i_04f5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F5E0: jsr $267e2.l
pub fn i_04f5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F5E6);
    cpu.pc = 0x267E2;
    16
}
/// $04F5E6: move.w #$0, $38(a6)
pub fn i_04f5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F5EC: lea.l $2948f8.l, a0
pub fn i_04f5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2948F8;
    20
}
/// $04F5F2: jsr $28cd4.l
pub fn i_04f5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F5F8);
    cpu.pc = 0x28CD4;
    16
}
/// $04F5F8: lea.l $4f5fe(pc), a1
pub fn i_04f5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F5FE;
    16
}
/// $04F5FC: move.l a1, (a6)
pub fn i_04f5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F5FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F5FE: jsr $2783a.l
pub fn i_04f5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F604);
    cpu.pc = 0x2783A;
    16
}
/// $04F604: jsr $28d70.l
pub fn i_04f604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F60A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F60A);
    cpu.pc = 0x28D70;
    16
}
/// $04F60A: jsr $27eba.l
pub fn i_04f60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F610;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F610);
    cpu.pc = 0x27EBA;
    16
}
/// $04F610: bcc.w $4f61a
pub fn i_04f610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F614;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F61A; return 10; }
    8
}
/// $04F614: lea.l $4f64c(pc), a1
pub fn i_04f614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F618;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F64C;
    16
}
/// $04F618: move.l a1, (a6)
pub fn i_04f618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F61A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F61A: jsr $5e9b6.l
pub fn i_04f61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F620);
    cpu.pc = 0x5E9B6;
    16
}
/// $04F620: andi.w #$7, d0
pub fn i_04f620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F624;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F624: bne.w $4f634
pub fn i_04f624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F628;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F634; return 10; }
    8
}
/// $04F628: lea.l $294936.l, a0
pub fn i_04f628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F62E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294936;
    20
}
/// $04F62E: jsr $28cd4.l
pub fn i_04f62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F634);
    cpu.pc = 0x28CD4;
    16
}
/// $04F634: lea.l $29624a.l, a0
pub fn i_04f634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F63A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29624A;
    20
}
/// $04F63A: jsr $5dd5c.l
pub fn i_04f63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F640);
    cpu.pc = 0x5DD5C;
    16
}
/// $04F640: bcc.w $4f64a
pub fn i_04f640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F644;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F64A; return 10; }
    8
}
/// $04F644: jmp $518.l
pub fn i_04f644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F64A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F64A: rts
pub fn i_04f64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F64C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debris_BouncerHop_04f64c  ($04F64C, 70 B, props/fortress_04e580.s)
/// $04F64C: move.w #$400, $2a(a6)
pub fn i_04f64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F652;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F652: lea.l $29494e.l, a0
pub fn i_04f652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F658;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29494E;
    20
}
/// $04F658: jsr $28cd4.l
pub fn i_04f658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F65E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F65E);
    cpu.pc = 0x28CD4;
    16
}
/// $04F65E: lea.l $4f664(pc), a1
pub fn i_04f65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F662;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F664;
    16
}
/// $04F662: move.l a1, (a6)
pub fn i_04f662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F664;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F664: jsr $27c8c.l
pub fn i_04f664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F66A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F66A);
    cpu.pc = 0x27C8C;
    16
}
/// $04F66A: bcc.w $4f674
pub fn i_04f66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F66E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F674; return 10; }
    8
}
/// $04F66E: lea.l $4f692(pc), a1
pub fn i_04f66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F672;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F692;
    16
}
/// $04F672: move.l a1, (a6)
pub fn i_04f672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F674;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F674: jsr $28d70.l
pub fn i_04f674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F67A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F67A);
    cpu.pc = 0x28D70;
    16
}
/// $04F67A: lea.l $29624a.l, a0
pub fn i_04f67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F680;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29624A;
    20
}
/// $04F680: jsr $5dd5c.l
pub fn i_04f680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F686;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F686);
    cpu.pc = 0x5DD5C;
    16
}
/// $04F686: bcc.w $4f690
pub fn i_04f686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F68A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F690; return 10; }
    8
}
/// $04F68A: jmp $518.l
pub fn i_04f68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F690;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F690: rts
pub fn i_04f690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F692;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debris_BouncerRoll_04f692  ($04F692, 124 B, props/fortress_04e580.s)
/// $04F692: jsr $267e2.l
pub fn i_04f692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F698);
    cpu.pc = 0x267E2;
    16
}
/// $04F698: move.w #$80, $28(a6)
pub fn i_04f698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F69E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F69E: lea.l $2948f8.l, a0
pub fn i_04f69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2948F8;
    20
}
/// $04F6A4: jsr $28cd4.l
pub fn i_04f6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F6AA);
    cpu.pc = 0x28CD4;
    16
}
/// $04F6AA: btst.b #$0, $3a(a6)
pub fn i_04f6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6B0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04F6B0: bne.w $4f6c0
pub fn i_04f6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F6C0; return 10; }
    8
}
/// $04F6B4: lea.l $294936.l, a0
pub fn i_04f6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294936;
    20
}
/// $04F6BA: jsr $28cd4.l
pub fn i_04f6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F6C0);
    cpu.pc = 0x28CD4;
    16
}
/// $04F6C0: lea.l $4f6c6(pc), a1
pub fn i_04f6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F6C6;
    16
}
/// $04F6C4: move.l a1, (a6)
pub fn i_04f6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F6C6: jsr $27a92.l
pub fn i_04f6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F6CC);
    cpu.pc = 0x27A92;
    16
}
/// $04F6CC: jsr $28d70.l
pub fn i_04f6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F6D2);
    cpu.pc = 0x28D70;
    16
}
/// $04F6D2: jsr $27eba.l
pub fn i_04f6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F6D8);
    cpu.pc = 0x27EBA;
    16
}
/// $04F6D8: bcc.w $4f6e2
pub fn i_04f6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F6E2; return 10; }
    8
}
/// $04F6DC: lea.l $4f64c(pc), a1
pub fn i_04f6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F64C;
    16
}
/// $04F6E0: move.l a1, (a6)
pub fn i_04f6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F6E2: jsr $5e9b6.l
pub fn i_04f6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F6E8);
    cpu.pc = 0x5E9B6;
    16
}
/// $04F6E8: andi.w #$f, d0
pub fn i_04f6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6EC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F6EC: bne.w $4f6f6
pub fn i_04f6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F6F6; return 10; }
    8
}
/// $04F6F0: lea.l $4f64c(pc), a1
pub fn i_04f6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F64C;
    16
}
/// $04F6F4: move.l a1, (a6)
pub fn i_04f6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F6F6: lea.l $29624a.l, a0
pub fn i_04f6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29624A;
    20
}
/// $04F6FC: jsr $5dd5c.l
pub fn i_04f6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F702);
    cpu.pc = 0x5DD5C;
    16
}
/// $04F702: bcc.w $4f70c
pub fn i_04f702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F706;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F70C; return 10; }
    8
}
/// $04F706: jmp $518.l
pub fn i_04f706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F70C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F70C: rts
pub fn i_04f70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F70E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SignA_04f70e  ($04F70E, 94 B, props/fortress_04e580.s)
/// $04F70E: move.w #$186, d1
pub fn i_04f70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F712;
    cpu.ir = 0x323C;
    let v1: u32 = 0x186;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F712: jsr $236e.l
pub fn i_04f712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F718);
    cpu.pc = 0x236E;
    16
}
/// $04F718: lea.l $2949ae.l, a0
pub fn i_04f718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F71E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2949AE;
    20
}
/// $04F71E: jsr $28cd4.l
pub fn i_04f71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F724);
    cpu.pc = 0x28CD4;
    16
}
/// $04F724: move.w #$0, $38(a6)
pub fn i_04f724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F72A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F72A: jsr $2783a.l
pub fn i_04f72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F730;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F730);
    cpu.pc = 0x2783A;
    16
}
/// $04F730: lea.l $295f6e.l, a1
pub fn i_04f730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F736;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295F6E;
    20
}
/// $04F736: jsr $43fac.l
pub fn i_04f736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F73C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F73C);
    cpu.pc = 0x43FAC;
    16
}
/// $04F73C: lea.l $4f742(pc), a1
pub fn i_04f73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F740;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F742;
    16
}
/// $04F740: move.l a1, (a6)
pub fn i_04f740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F742;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F742: jsr $2783a.l
pub fn i_04f742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F748);
    cpu.pc = 0x2783A;
    16
}
/// $04F748: jsr $28d70.l
pub fn i_04f748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F74E);
    cpu.pc = 0x28D70;
    16
}
/// $04F74E: movea.l #$ffffffff, a0
pub fn i_04f74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F754;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04F754: lea.l $29625e.l, a0
pub fn i_04f754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F75A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29625E;
    20
}
/// $04F75A: jsr $5dd5c.l
pub fn i_04f75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F760);
    cpu.pc = 0x5DD5C;
    16
}
/// $04F760: bcc.w $4f76a
pub fn i_04f760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F764;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F76A; return 10; }
    8
}
/// $04F764: jmp $518.l
pub fn i_04f764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F76A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F76A: rts
pub fn i_04f76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F76C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SignB_04f76c  ($04F76C, 94 B, props/fortress_04e580.s)
/// $04F76C: move.w #$1b6, d1
pub fn i_04f76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F770;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F770: jsr $236e.l
pub fn i_04f770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F776);
    cpu.pc = 0x236E;
    16
}
/// $04F776: lea.l $2949be.l, a0
pub fn i_04f776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F77C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2949BE;
    20
}
/// $04F77C: jsr $28cd4.l
pub fn i_04f77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F782);
    cpu.pc = 0x28CD4;
    16
}
/// $04F782: move.w #$0, $38(a6)
pub fn i_04f782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F788;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F788: jsr $2783a.l
pub fn i_04f788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F78E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F78E);
    cpu.pc = 0x2783A;
    16
}
/// $04F78E: lea.l $295f94.l, a1
pub fn i_04f78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F794;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295F94;
    20
}
/// $04F794: jsr $43fac.l
pub fn i_04f794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F79A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F79A);
    cpu.pc = 0x43FAC;
    16
}
/// $04F79A: lea.l $4f7a0(pc), a1
pub fn i_04f79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F79E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F7A0;
    16
}
/// $04F79E: move.l a1, (a6)
pub fn i_04f79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F7A0: jsr $2783a.l
pub fn i_04f7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F7A6);
    cpu.pc = 0x2783A;
    16
}
/// $04F7A6: jsr $28d70.l
pub fn i_04f7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F7AC);
    cpu.pc = 0x28D70;
    16
}
/// $04F7AC: movea.l #$ffffffff, a0
pub fn i_04f7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7B2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04F7B2: lea.l $29625e.l, a0
pub fn i_04f7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29625E;
    20
}
/// $04F7B8: jsr $5dd5c.l
pub fn i_04f7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F7BE);
    cpu.pc = 0x5DD5C;
    16
}
/// $04F7BE: bcc.w $4f7c8
pub fn i_04f7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F7C8; return 10; }
    8
}
/// $04F7C2: jmp $518.l
pub fn i_04f7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7C8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F7C8: rts
pub fn i_04f7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Boat_04f7ca  ($04F7CA, 394 B, props/fortress_04e580.s)
/// $04F7CA: move.w #$1ca, d1
pub fn i_04f7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F7CE: jsr $236e.l
pub fn i_04f7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F7D4);
    cpu.pc = 0x236E;
    16
}
/// $04F7D4: move.w #$1d8, d1
pub fn i_04f7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F7D8: jsr $236e.l
pub fn i_04f7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F7DE);
    cpu.pc = 0x236E;
    16
}
/// $04F7DE: move.w #$1cb, d1
pub fn i_04f7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F7E2: jsr $236e.l
pub fn i_04f7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F7E8);
    cpu.pc = 0x236E;
    16
}
/// $04F7E8: lea.l $2c0092.l, a0
pub fn i_04f7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0092;
    20
}
/// $04F7EE: jsr $799de.l
pub fn i_04f7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F7F4);
    cpu.pc = 0x799DE;
    16
}
/// $04F7F4: move.w d0, $66(a6)
pub fn i_04f7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7F8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F7F8: lea.l $2949ce.l, a0
pub fn i_04f7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F7FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2949CE;
    20
}
/// $04F7FE: jsr $28cd4.l
pub fn i_04f7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F804);
    cpu.pc = 0x28CD4;
    16
}
/// $04F804: cmpi.b #$1, $98(a6)
pub fn i_04f804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F80A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04F80A: bne.w $4f81c
pub fn i_04f80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F80E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F81C; return 10; }
    8
}
/// $04F80E: lea.l $2954a8.l, a0
pub fn i_04f80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F814;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2954A8;
    20
}
/// $04F814: move.l a0, $48(a6)
pub fn i_04f814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F818;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F818: bra.w $4f826
pub fn i_04f818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F81C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4F826;
    10
}
/// $04F81C: lea.l $295454.l, a0
pub fn i_04f81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F822;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x295454;
    20
}
/// $04F822: move.l a0, $48(a6)
pub fn i_04f822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F826;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F826: jsr $267e2.l
pub fn i_04f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F82C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F82C);
    cpu.pc = 0x267E2;
    16
}
/// $04F82C: jsr $5e7c0.l
pub fn i_04f82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F832);
    cpu.pc = 0x5E7C0;
    16
}
/// $04F832: move.w #$2000, $38(a6)
pub fn i_04f832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F838;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F838: move.l #$296356, $60(a6)
pub fn i_04f838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F840;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x296356;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F840: move.w $16(a6), $14(a6)
pub fn i_04f840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F846;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04F846: lea.l $4f84c(pc), a1
pub fn i_04f846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F84A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F84C;
    16
}
/// $04F84A: move.l a1, (a6)
pub fn i_04f84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F84C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F84C: jsr $2783a.l
pub fn i_04f84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F852);
    cpu.pc = 0x2783A;
    16
}
/// $04F852: cmpi.w #$148, $22(a6)
pub fn i_04f852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F858;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x148, Size::Word, false);
    16
}
/// $04F858: bmi.w $4f862
pub fn i_04f858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F85C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4F862; return 10; }
    8
}
/// $04F85C: move.b #$2, $45(a6)
pub fn i_04f85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F862;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F862: move.b $98(a6), d0
pub fn i_04f862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F866;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04F866: cmpi.b #$1, d0
pub fn i_04f866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F86A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $04F86A: beq.w $4f88e
pub fn i_04f86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F86E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4F88E; return 10; }
    8
}
/// $04F86E: jsr $28998.l
pub fn i_04f86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F874);
    cpu.pc = 0x28998;
    16
}
/// $04F874: move.w $22(a6), d0
pub fn i_04f874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F878;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04F878: subi.w #$12, d0
pub fn i_04f878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F87C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x12, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F87C: move.w $24(a6), d1
pub fn i_04f87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F880;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04F880: addi.w #$24, d1
pub fn i_04f880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F884;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x24, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F884: move.w #$24, d2
pub fn i_04f884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F888;
    cpu.ir = 0x343C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F888: jsr $99812.l
pub fn i_04f888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F88E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F88E);
    cpu.pc = 0x99812;
    16
}
/// $04F88E: jsr $28d70.l
pub fn i_04f88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F894);
    cpu.pc = 0x28D70;
    16
}
/// $04F894: move.w $16(a6), d0
pub fn i_04f894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F898;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04F898: btst.b #$3, $106f28.l
pub fn i_04f898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8A0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((3) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $04F8A0: beq.w $4f8a8
pub fn i_04f8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4F8A8; return 10; }
    8
}
/// $04F8A4: move.w $18(a6), d0
pub fn i_04f8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8A8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04F8A8: move.w d0, $14(a6)
pub fn i_04f8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8AC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F8AC: jsr $2870a.l
pub fn i_04f8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F8B2);
    cpu.pc = 0x2870A;
    16
}
/// $04F8B2: bcc.w $4f8cc
pub fn i_04f8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F8CC; return 10; }
    8
}
/// $04F8B6: move.w $1a(a6), $14(a6)
pub fn i_04f8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8BC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04F8BC: bclr.b #$3, $13(a6)
pub fn i_04f8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8C2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04F8C2: move.w #$108d, d0
pub fn i_04f8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F8C6: jsr $2352.l
pub fn i_04f8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F8CC);
    cpu.pc = 0x2352;
    16
}
/// $04F8CC: jsr $28758.l
pub fn i_04f8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F8D2);
    cpu.pc = 0x28758;
    16
}
/// $04F8D2: bcc.w $4f936
pub fn i_04f8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F936; return 10; }
    8
}
/// $04F8D6: move.w #$1030, d0
pub fn i_04f8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F8DA: jsr $2352.l
pub fn i_04f8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F8E0);
    cpu.pc = 0x2352;
    16
}
/// $04F8E0: lea.l $4f954(pc), a1
pub fn i_04f8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F954;
    16
}
/// $04F8E4: jsr $4ae.l
pub fn i_04f8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F8EA);
    cpu.pc = 0x4AE;
    16
}
/// $04F8EA: jsr $5dd02.l
pub fn i_04f8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F8F0);
    cpu.pc = 0x5DD02;
    16
}
/// $04F8F0: lea.l $295c7a.l, a1
pub fn i_04f8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C7A;
    20
}
/// $04F8F6: jsr $77c7e.l
pub fn i_04f8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F8FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F8FC);
    cpu.pc = 0x77C7E;
    16
}
/// $04F8FC: lea.l $295c8c.l, a1
pub fn i_04f8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F902;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C8C;
    20
}
/// $04F902: jsr $77c7e.l
pub fn i_04f902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F908;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F908);
    cpu.pc = 0x77C7E;
    16
}
/// $04F908: lea.l $295c9e.l, a1
pub fn i_04f908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F90E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C9E;
    20
}
/// $04F90E: jsr $77c7e.l
pub fn i_04f90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F914);
    cpu.pc = 0x77C7E;
    16
}
/// $04F914: jsr $5e9b6.l
pub fn i_04f914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F91A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F91A);
    cpu.pc = 0x5E9B6;
    16
}
/// $04F91A: andi.b #$3f, d0
pub fn i_04f91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F91E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04F91E: bne.w $4f94c
pub fn i_04f91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F922;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F94C; return 10; }
    8
}
/// $04F922: lea.l $4f992(pc), a1
pub fn i_04f922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F926;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F992;
    16
}
/// $04F926: jsr $4ae.l
pub fn i_04f926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F92C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F92C);
    cpu.pc = 0x4AE;
    16
}
/// $04F92C: jsr $5dd02.l
pub fn i_04f92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F932);
    cpu.pc = 0x5DD02;
    16
}
/// $04F932: bra.w $4f94c
pub fn i_04f932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F936;
    cpu.ir = 0x6000;
    cpu.pc = 0x4F94C;
    10
}
/// $04F936: movea.l #$ffffffff, a0
pub fn i_04f936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F93C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04F93C: lea.l $296268.l, a0
pub fn i_04f93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F942;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296268;
    20
}
/// $04F942: jsr $5dd5c.l
pub fn i_04f942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F948;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F948);
    cpu.pc = 0x5DD5C;
    16
}
/// $04F948: bcc.w $4f952
pub fn i_04f948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F94C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F952; return 10; }
    8
}
/// $04F94C: jmp $518.l
pub fn i_04f94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F952;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F952: rts
pub fn i_04f952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F954;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BoatBlast_04f954  ($04F954, 62 B, props/fortress_04e580.s)
/// $04F954: move.w #$8, $72(a6)
pub fn i_04f954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F95A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F95A: lea.l $4f960(pc), a1
pub fn i_04f95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F95E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F960;
    16
}
/// $04F95E: move.l a1, (a6)
pub fn i_04f95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F960;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F960: jsr $2783a.l
pub fn i_04f960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F966;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F966);
    cpu.pc = 0x2783A;
    16
}
/// $04F966: lea.l $2961a6.l, a0
pub fn i_04f966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F96C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2961A6;
    20
}
/// $04F96C: move.l a0, $4c(a6)
pub fn i_04f96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F970;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04F970: jsr $283ca.l
pub fn i_04f970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F976);
    cpu.pc = 0x283CA;
    16
}
/// $04F976: jsr $283ca.l
pub fn i_04f976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F97C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F97C);
    cpu.pc = 0x283CA;
    16
}
/// $04F97C: jsr $283d8.l
pub fn i_04f97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F982;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F982);
    cpu.pc = 0x283D8;
    16
}
/// $04F982: subq.w #$1, $72(a6)
pub fn i_04f982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F986;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04F986: bpl.w $4f990
pub fn i_04f986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F98A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4F990; return 10; }
    8
}
/// $04F98A: jmp $518.l
pub fn i_04f98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F990;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F990: rts
pub fn i_04f990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F992;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BoatDebris_04f992  ($04F992, 174 B, props/fortress_04e580.s)
/// $04F992: move.w #$1c6, d1
pub fn i_04f992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F996;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F996: jsr $236e.l
pub fn i_04f996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F99C);
    cpu.pc = 0x236E;
    16
}
/// $04F99C: lea.l $2949de.l, a0
pub fn i_04f99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2949DE;
    20
}
/// $04F9A2: jsr $28cd4.l
pub fn i_04f9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F9A8);
    cpu.pc = 0x28CD4;
    16
}
/// $04F9A8: move.b #$0, $3a(a6)
pub fn i_04f9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04F9AE: jsr $267e2.l
pub fn i_04f9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F9B4);
    cpu.pc = 0x267E2;
    16
}
/// $04F9B4: jsr $5e9b6.l
pub fn i_04f9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F9BA);
    cpu.pc = 0x5E9B6;
    16
}
/// $04F9BA: move.w d0, d1
pub fn i_04f9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9BC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F9BC: andi.w #$7ff, d0
pub fn i_04f9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9C0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F9C0: addi.w #$1000, d0
pub fn i_04f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9C4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F9C4: move.w d1, d0
pub fn i_04f9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9C6;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F9C6: andi.w #$ff, d0
pub fn i_04f9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9CA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F9CA: btst.b #$0, d0
pub fn i_04f9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9CE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04F9CE: bne.w $4f9d4
pub fn i_04f9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4F9D4; return 10; }
    8
}
/// $04F9D2: neg.w d0
pub fn i_04f9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9D4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04F9D4: move.w d0, $28(a6)
pub fn i_04f9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9D8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F9D8: move.w d1, d0
pub fn i_04f9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9DA;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F9DA: andi.w #$7f, d0
pub fn i_04f9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9DE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F9DE: addi.w #$7f, d0
pub fn i_04f9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9E2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04F9E2: move.w d0, $2e(a6)
pub fn i_04f9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F9E6: subi.w #$10, $38(a6)
pub fn i_04f9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9EC;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04F9EC: lea.l $4f9f2(pc), a1
pub fn i_04f9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F9F2;
    16
}
/// $04F9F0: move.l a1, (a6)
pub fn i_04f9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F9F2: jsr $27cee.l
pub fn i_04f9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F9F8);
    cpu.pc = 0x27CEE;
    16
}
/// $04F9F8: jsr $28d70.l
pub fn i_04f9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F9FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F9FE);
    cpu.pc = 0x28D70;
    16
}
/// $04F9FE: move.b $106f28.l, d0
pub fn i_04f9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA04;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $04FA04: andi.b #$7, d0
pub fn i_04fa04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA08;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04FA08: bne.w $4fa22
pub fn i_04fa08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FA22; return 10; }
    8
}
/// $04FA0C: lea.l $77efe.l, a1
pub fn i_04fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $04FA12: jsr $4ae.l
pub fn i_04fa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FA18);
    cpu.pc = 0x4AE;
    16
}
/// $04FA18: jsr $5dd02.l
pub fn i_04fa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FA1E);
    cpu.pc = 0x5DD02;
    16
}
/// $04FA1E: subq.w #$1, $38(a6)
pub fn i_04fa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA22;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04FA22: movea.l #$ffffffff, a0
pub fn i_04fa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA28;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04FA28: lea.l $296268.l, a0
pub fn i_04fa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296268;
    20
}
/// $04FA2E: jsr $5dd5c.l
pub fn i_04fa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FA34);
    cpu.pc = 0x5DD5C;
    16
}
/// $04FA34: bcc.w $4fa3e
pub fn i_04fa34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4FA3E; return 10; }
    8
}
/// $04FA38: jmp $518.l
pub fn i_04fa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA3E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04FA3E: rts
pub fn i_04fa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixTile_Set11C2_04fa40  ($04FA40, 16 B, props/fortress_04e580.s)
/// $04FA40: move.w #$11, d1
pub fn i_04fa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA44: move.w #$c2, d2
pub fn i_04fa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA48;
    cpu.ir = 0x343C;
    let v1: u32 = 0xC2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA48: move.w #$ffff, d3
pub fn i_04fa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA4C;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA4C: move.w #$1, d4
pub fn i_04fa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA50;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04fa50  ($04FA50, 8 B, thunks/jsr_abs_thunks.c)
/// $04FA50: jsr $2c26.l
pub fn i_04fa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FA56);
    cpu.pc = 0x2C26;
    16
}
/// $04FA56: rts
pub fn i_04fa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixTile_Set11C1_04fa58  ($04FA58, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FA58: move.w #$11, d1
pub fn i_04fa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA5C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA5C: move.w #$c1, d2
pub fn i_04fa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA60;
    cpu.ir = 0x343C;
    let v1: u32 = 0xC1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA60: move.w #$ffff, d3
pub fn i_04fa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA64;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA64: move.w #$1, d4
pub fn i_04fa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA68;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04fa68  ($04FA68, 8 B, thunks/jsr_abs_thunks.c)
/// $04FA68: jsr $2c26.l
pub fn i_04fa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FA6E);
    cpu.pc = 0x2C26;
    16
}
/// $04FA6E: rts
pub fn i_04fa6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_OffscreenLeftCheck_04fa70  ($04FA70, 14 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FA70: move.w $70(a6), d0
pub fn i_04fa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA74;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04FA74: neg.w d0
pub fn i_04fa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA76;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04FA76: cmp.w $22(a6), d0
pub fn i_04fa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA7A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04FA7A: ble.w $4fa84
pub fn i_04fa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA7E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4FA84; return 10; }
    8
}
// ===== SetC_04fa7e  ($04FA7E, 6 B, thunks/ccr_helpers.c)
/// $04FA7E: ori.b #$1, ccr
pub fn i_04fa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA82;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04FA82: rts
pub fn i_04fa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_04fa84  ($04FA84, 6 B, thunks/ccr_helpers.c)
/// $04FA84: andi.b #$fe, ccr
pub fn i_04fa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA88;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04FA88: rts
pub fn i_04fa88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBlitState10_04fa8a  ($04FA8A, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FA8A: cmpi.b #$10, $21(a6)
pub fn i_04fa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA90;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Byte, false);
    16
}
/// $04FA90: bne.w $4faa0
pub fn i_04fa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FAA0; return 10; }
    8
}
/// $04FA94: lea.l $29557e.l, a2
pub fn i_04fa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA9A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29557E;
    20
}
// ===== JsrPcThunk_04fa9a  ($04FA9A, 6 B, thunks/jsr_pc_thunks.c)
/// $04FA9A: jsr $5022a(pc)
pub fn i_04fa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FA9E);
    cpu.pc = 0x5022A;
    16
}
/// $04FA9E: rts
pub fn i_04fa9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBlitState11_04faa0  ($04FAA0, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FAA0: cmpi.b #$11, $21(a6)
pub fn i_04faa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAA6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Byte, false);
    16
}
/// $04FAA6: bne.w $4fab6
pub fn i_04faa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAAA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FAB6; return 10; }
    8
}
/// $04FAAA: lea.l $295592.l, a2
pub fn i_04faaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAB0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295592;
    20
}
// ===== JsrPcThunk_04fab0  ($04FAB0, 6 B, thunks/jsr_pc_thunks.c)
/// $04FAB0: jsr $5022a(pc)
pub fn i_04fab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FAB4);
    cpu.pc = 0x5022A;
    16
}
/// $04FAB4: rts
pub fn i_04fab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBlitState12_04fab6  ($04FAB6, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FAB6: cmpi.b #$12, $21(a6)
pub fn i_04fab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FABC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x12, Size::Byte, false);
    16
}
/// $04FABC: bne.w $4facc
pub fn i_04fabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FACC; return 10; }
    8
}
/// $04FAC0: lea.l $2955a6.l, a2
pub fn i_04fac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAC6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2955A6;
    20
}
// ===== JsrPcThunk_04fac6  ($04FAC6, 6 B, thunks/jsr_pc_thunks.c)
/// $04FAC6: jsr $5022a(pc)
pub fn i_04fac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FACA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FACA);
    cpu.pc = 0x5022A;
    16
}
/// $04FACA: rts
pub fn i_04faca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FACC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBlitState01_04facc  ($04FACC, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FACC: cmpi.b #$1, $21(a6)
pub fn i_04facc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAD2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04FAD2: bne.w $4fae2
pub fn i_04fad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FAE2; return 10; }
    8
}
/// $04FAD6: lea.l $2955ba.l, a2
pub fn i_04fad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FADC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2955BA;
    20
}
// ===== JsrPcThunk_04fadc  ($04FADC, 6 B, thunks/jsr_pc_thunks.c)
/// $04FADC: jsr $5022a(pc)
pub fn i_04fadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FAE0);
    cpu.pc = 0x5022A;
    16
}
/// $04FAE0: rts
pub fn i_04fae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBlitState02_04fae2  ($04FAE2, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FAE2: cmpi.b #$2, $21(a6)
pub fn i_04fae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAE8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04FAE8: bne.w $4faf6
pub fn i_04fae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FAF6; return 10; }
    8
}
/// $04FAEC: lea.l $2955ce.l, a2
pub fn i_04faec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAF2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2955CE;
    20
}
// ===== JsrPcThunk_04faf2  ($04FAF2, 6 B, thunks/jsr_pc_thunks.c)
/// $04FAF2: jsr $5022a(pc)
pub fn i_04faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FAF6);
    cpu.pc = 0x5022A;
    16
}
/// $04FAF6: rts
pub fn i_04faf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BunkerBlitState10_04faf8  ($04FAF8, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FAF8: cmpi.b #$10, $21(a6)
pub fn i_04faf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FAFE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Byte, false);
    16
}
/// $04FAFE: bne.w $4fb0e
pub fn i_04fafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB02;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FB0E; return 10; }
    8
}
/// $04FB02: lea.l $295646.l, a2
pub fn i_04fb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB08;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295646;
    20
}
// ===== JsrPcThunk_04fb08  ($04FB08, 6 B, thunks/jsr_pc_thunks.c)
/// $04FB08: jsr $5022a(pc)
pub fn i_04fb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB0C);
    cpu.pc = 0x5022A;
    16
}
/// $04FB0C: rts
pub fn i_04fb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BunkerBlitState01_04fb0e  ($04FB0E, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FB0E: cmpi.b #$1, $21(a6)
pub fn i_04fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB14;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04FB14: bne.w $4fb24
pub fn i_04fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FB24; return 10; }
    8
}
/// $04FB18: lea.l $29565a.l, a2
pub fn i_04fb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB1E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29565A;
    20
}
// ===== JsrPcThunk_04fb1e  ($04FB1E, 6 B, thunks/jsr_pc_thunks.c)
/// $04FB1E: jsr $5022a(pc)
pub fn i_04fb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB22);
    cpu.pc = 0x5022A;
    16
}
/// $04FB22: rts
pub fn i_04fb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BunkerBlitState11_04fb24  ($04FB24, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FB24: cmpi.b #$11, $21(a6)
pub fn i_04fb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB2A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Byte, false);
    16
}
/// $04FB2A: bne.w $4fb3a
pub fn i_04fb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4FB3A; return 10; }
    8
}
/// $04FB2E: lea.l $29566e.l, a2
pub fn i_04fb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29566E;
    20
}
// ===== JsrPcThunk_04fb34  ($04FB34, 6 B, thunks/jsr_pc_thunks.c)
/// $04FB34: jsr $5022a(pc)
pub fn i_04fb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB38);
    cpu.pc = 0x5022A;
    16
}
/// $04FB38: rts
pub fn i_04fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0004FB3A  ($04FB3A, 2 B, thunks/trivial_rts.c)
/// $04FB3A: rts
pub fn i_04fb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NestBlitByFacing_04fb3c  ($04FB3C, 50 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FB3C: btst.b #$0, $3a(a6)
pub fn i_04fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB42;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04FB42: beq.w $4fb5e
pub fn i_04fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4FB5E; return 10; }
    8
}
/// $04FB46: lea.l $2956d2.l, a2
pub fn i_04fb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB4C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956D2;
    20
}
/// $04FB4C: jsr $5022a(pc)
pub fn i_04fb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB50);
    cpu.pc = 0x5022A;
    16
}
/// $04FB50: lea.l $295736.l, a2
pub fn i_04fb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB56;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295736;
    20
}
/// $04FB56: jsr $5022a(pc)
pub fn i_04fb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB5A);
    cpu.pc = 0x5022A;
    16
}
/// $04FB5A: bra.w $4fb72
pub fn i_04fb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4FB72;
    10
}
/// $04FB5E: lea.l $295696.l, a2
pub fn i_04fb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB64;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295696;
    20
}
/// $04FB64: jsr $5022a(pc)
pub fn i_04fb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB68);
    cpu.pc = 0x5022A;
    16
}
/// $04FB68: lea.l $29570e.l, a2
pub fn i_04fb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB6E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29570E;
    20
}
// ===== JsrPcThunk_04fb6e  ($04FB6E, 6 B, thunks/jsr_pc_thunks.c)
/// $04FB6E: jsr $5022a(pc)
pub fn i_04fb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB72);
    cpu.pc = 0x5022A;
    16
}
/// $04FB72: rts
pub fn i_04fb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NestBlitPair_04fb74  ($04FB74, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FB74: lea.l $29575e.l, a2
pub fn i_04fb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB7A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29575E;
    20
}
/// $04FB7A: jsr $5022a(pc)
pub fn i_04fb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB7E);
    cpu.pc = 0x5022A;
    16
}
/// $04FB7E: lea.l $295772.l, a2
pub fn i_04fb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB84;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295772;
    20
}
// ===== JsrPcThunk_04fb84  ($04FB84, 6 B, thunks/jsr_pc_thunks.c)
/// $04FB84: jsr $5022a(pc)
pub fn i_04fb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4FB88);
    cpu.pc = 0x5022A;
    16
}
/// $04FB88: rts
pub fn i_04fb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrier_SpawnPiece01_04fb8a  ($04FB8A, 86 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FB8A: lea.l $4f1f4(pc), a1
pub fn i_04fb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FB8E: jsr $4ae.l
pub fn i_04fb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FB94);
    cpu.pc = 0x4AE;
    16
}
/// $04FB94: jsr $5dd22.l
pub fn i_04fb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FB9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FB9A);
    cpu.pc = 0x5DD22;
    16
}
/// $04FB9A: move.b #$1, $21(a0)
pub fn i_04fb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBA0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FBA0: lea.l $295826.l, a2
pub fn i_04fba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBA6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295826;
    20
}
/// $04FBA6: move.l a2, $70(a0)
pub fn i_04fba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBAA;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FBAA: lea.l $29583a.l, a2
pub fn i_04fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBB0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29583A;
    20
}
/// $04FBB0: move.l a2, $74(a0)
pub fn i_04fbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBB4;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FBB4: move.l #$ffffffff, $78(a0)
pub fn i_04fbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBBC;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FBBC: addi.w #$40, $22(a0)
pub fn i_04fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBC2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FBC2: addi.w #$70, $24(a0)
pub fn i_04fbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBC8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FBC8: movem.l a6, -(a7)
pub fn i_04fbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBCC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FBCC: movea.l a0, a6
pub fn i_04fbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBCE;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FBCE: lea.l $2946a2.l, a0
pub fn i_04fbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FBD4: jsr $28cd4.l
pub fn i_04fbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FBDA);
    cpu.pc = 0x28CD4;
    16
}
/// $04FBDA: movem.l (a7)+, a6
pub fn i_04fbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBDE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FBDE: rts
pub fn i_04fbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrier_SpawnPiece02_04fbe0  ($04FBE0, 86 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FBE0: lea.l $4f1f4(pc), a1
pub fn i_04fbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FBE4: jsr $4ae.l
pub fn i_04fbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FBEA);
    cpu.pc = 0x4AE;
    16
}
/// $04FBEA: jsr $5dd22.l
pub fn i_04fbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FBF0);
    cpu.pc = 0x5DD22;
    16
}
/// $04FBF0: move.b #$2, $21(a0)
pub fn i_04fbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBF6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FBF6: lea.l $295862.l, a2
pub fn i_04fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FBFC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295862;
    20
}
/// $04FBFC: move.l a2, $70(a0)
pub fn i_04fbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC00;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FC00: lea.l $295876.l, a2
pub fn i_04fc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC06;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295876;
    20
}
/// $04FC06: move.l a2, $74(a0)
pub fn i_04fc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC0A;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FC0A: move.l #$ffffffff, $78(a0)
pub fn i_04fc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC12;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FC12: addi.w #$20, $22(a0)
pub fn i_04fc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC18;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FC18: addi.w #$50, $24(a0)
pub fn i_04fc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC1E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FC1E: movem.l a6, -(a7)
pub fn i_04fc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC22;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FC22: movea.l a0, a6
pub fn i_04fc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC24;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FC24: lea.l $2946a2.l, a0
pub fn i_04fc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FC2A: jsr $28cd4.l
pub fn i_04fc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FC30);
    cpu.pc = 0x28CD4;
    16
}
/// $04FC30: movem.l (a7)+, a6
pub fn i_04fc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC34;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FC34: rts
pub fn i_04fc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrier_SpawnPiece04_04fc36  ($04FC36, 86 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FC36: lea.l $4f1f4(pc), a1
pub fn i_04fc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FC3A: jsr $4ae.l
pub fn i_04fc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FC40);
    cpu.pc = 0x4AE;
    16
}
/// $04FC40: jsr $5dd22.l
pub fn i_04fc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FC46);
    cpu.pc = 0x5DD22;
    16
}
/// $04FC46: move.b #$4, $21(a0)
pub fn i_04fc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC4C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FC4C: lea.l $29588a.l, a2
pub fn i_04fc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC52;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29588A;
    20
}
/// $04FC52: move.l a2, $70(a0)
pub fn i_04fc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC56;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FC56: lea.l $29589e.l, a2
pub fn i_04fc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC5C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29589E;
    20
}
/// $04FC5C: move.l a2, $74(a0)
pub fn i_04fc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC60;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FC60: move.l #$ffffffff, $78(a0)
pub fn i_04fc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC68;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FC68: addi.w #$40, $22(a0)
pub fn i_04fc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC6E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FC6E: addi.w #$50, $24(a0)
pub fn i_04fc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC74;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FC74: movem.l a6, -(a7)
pub fn i_04fc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC78;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FC78: movea.l a0, a6
pub fn i_04fc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC7A;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FC7A: lea.l $2946a2.l, a0
pub fn i_04fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FC80: jsr $28cd4.l
pub fn i_04fc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FC86);
    cpu.pc = 0x28CD4;
    16
}
/// $04FC86: movem.l (a7)+, a6
pub fn i_04fc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC8A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FC8A: rts
pub fn i_04fc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrier_SpawnPiece08_04fc8c  ($04FC8C, 76 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FC8C: lea.l $4f1f4(pc), a1
pub fn i_04fc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FC90: jsr $4ae.l
pub fn i_04fc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FC96);
    cpu.pc = 0x4AE;
    16
}
/// $04FC96: jsr $5dd22.l
pub fn i_04fc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FC9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FC9C);
    cpu.pc = 0x5DD22;
    16
}
/// $04FC9C: move.b #$8, $21(a0)
pub fn i_04fc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCA2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FCA2: lea.l $2958b2.l, a2
pub fn i_04fca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCA8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2958B2;
    20
}
/// $04FCA8: move.l a2, $70(a0)
pub fn i_04fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCAC;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FCAC: move.l #$ffffffff, $74(a0)
pub fn i_04fcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCB4;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FCB4: addi.w #$60, $22(a0)
pub fn i_04fcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCBA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FCBA: addi.w #$50, $24(a0)
pub fn i_04fcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCC0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FCC0: movem.l a6, -(a7)
pub fn i_04fcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCC4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FCC4: movea.l a0, a6
pub fn i_04fcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCC6;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FCC6: lea.l $2946a2.l, a0
pub fn i_04fcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FCCC: jsr $28cd4.l
pub fn i_04fccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FCD2);
    cpu.pc = 0x28CD4;
    16
}
/// $04FCD2: movem.l (a7)+, a6
pub fn i_04fcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCD6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FCD6: rts
pub fn i_04fcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrier_SpawnPieceLeft_04fcd8  ($04FCD8, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FCD8: lea.l $4f1f4(pc), a1
pub fn i_04fcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FCDC: jsr $4ae.l
pub fn i_04fcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FCE2);
    cpu.pc = 0x4AE;
    16
}
/// $04FCE2: jsr $5dd22.l
pub fn i_04fce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FCE8);
    cpu.pc = 0x5DD22;
    16
}
/// $04FCE8: move.b #$0, $21(a0)
pub fn i_04fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCEE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FCEE: lea.l $295af6.l, a2
pub fn i_04fcee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCF4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295AF6;
    20
}
/// $04FCF4: move.l a2, $70(a0)
pub fn i_04fcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FCF8;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FCF8: move.l #$ffffffff, $74(a0)
pub fn i_04fcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD00;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FD00: addi.w #$20, $22(a0)
pub fn i_04fd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD06;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FD06: addi.w #$50, $24(a0)
pub fn i_04fd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD0C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FD0C: movem.l a6, -(a7)
pub fn i_04fd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD10;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FD10: movem.l a0, -(a7)
pub fn i_04fd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD14;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FD14: movea.l a0, a6
pub fn i_04fd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD16;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FD16: lea.l $2946ce.l, a0
pub fn i_04fd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946CE;
    20
}
/// $04FD1C: jsr $28cd4.l
pub fn i_04fd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FD22);
    cpu.pc = 0x28CD4;
    16
}
/// $04FD22: movem.l (a7)+, a0
pub fn i_04fd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD26;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FD26: movem.l (a7)+, a6
pub fn i_04fd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD2A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FD2A: rts
pub fn i_04fd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrier_SpawnPieceRight_04fd2c  ($04FD2C, 94 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FD2C: lea.l $4f1f4(pc), a1
pub fn i_04fd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FD30: jsr $4ae.l
pub fn i_04fd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FD36);
    cpu.pc = 0x4AE;
    16
}
/// $04FD36: jsr $5dd22.l
pub fn i_04fd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FD3C);
    cpu.pc = 0x5DD22;
    16
}
/// $04FD3C: move.b #$0, $21(a0)
pub fn i_04fd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD42;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FD42: lea.l $295b1e.l, a2
pub fn i_04fd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD48;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295B1E;
    20
}
/// $04FD48: move.l a2, $70(a0)
pub fn i_04fd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD4C;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FD4C: lea.l $295b32.l, a2
pub fn i_04fd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD52;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295B32;
    20
}
/// $04FD52: move.l a2, $74(a0)
pub fn i_04fd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD56;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FD56: move.l #$ffffffff, $78(a0)
pub fn i_04fd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD5E;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FD5E: addi.w #$40, $22(a0)
pub fn i_04fd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD64;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FD64: addi.w #$30, $24(a0)
pub fn i_04fd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD6A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FD6A: movem.l a6, -(a7)
pub fn i_04fd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD6E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FD6E: movem.l a0, -(a7)
pub fn i_04fd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD72;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FD72: movea.l a0, a6
pub fn i_04fd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD74;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FD74: lea.l $2946ce.l, a0
pub fn i_04fd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946CE;
    20
}
/// $04FD7A: jsr $28cd4.l
pub fn i_04fd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FD80);
    cpu.pc = 0x28CD4;
    16
}
/// $04FD80: movem.l (a7)+, a0
pub fn i_04fd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD84;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FD84: movem.l (a7)+, a6
pub fn i_04fd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD88;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FD88: rts
pub fn i_04fd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece01_04fd8a  ($04FD8A, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FD8A: lea.l $4f1f4(pc), a1
pub fn i_04fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FD8E: jsr $4ae.l
pub fn i_04fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FD94);
    cpu.pc = 0x4AE;
    16
}
/// $04FD94: jsr $5dd22.l
pub fn i_04fd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FD9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FD9A);
    cpu.pc = 0x5DD22;
    16
}
/// $04FD9A: move.b #$1, $21(a0)
pub fn i_04fd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDA0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FDA0: lea.l $2958ee.l, a2
pub fn i_04fda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDA6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2958EE;
    20
}
/// $04FDA6: move.l a2, $70(a0)
pub fn i_04fda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDAA;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FDAA: move.l #$ffffffff, $74(a0)
pub fn i_04fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDB2;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FDB2: addi.w #$10, $22(a0)
pub fn i_04fdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDB8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FDB8: addi.w #$90, $24(a0)
pub fn i_04fdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDBE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FDBE: movem.l a6, -(a7)
pub fn i_04fdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDC2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FDC2: movem.l a0, -(a7)
pub fn i_04fdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDC6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FDC6: movea.l a0, a6
pub fn i_04fdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDC8;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FDC8: lea.l $2946a2.l, a0
pub fn i_04fdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDCE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FDCE: jsr $28cd4.l
pub fn i_04fdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FDD4);
    cpu.pc = 0x28CD4;
    16
}
/// $04FDD4: movem.l (a7)+, a0
pub fn i_04fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDD8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FDD8: movem.l (a7)+, a6
pub fn i_04fdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDDC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FDDC: rts
pub fn i_04fddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDDE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece02_04fdde  ($04FDDE, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FDDE: lea.l $4f1f4(pc), a1
pub fn i_04fdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FDE2: jsr $4ae.l
pub fn i_04fde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FDE8);
    cpu.pc = 0x4AE;
    16
}
/// $04FDE8: jsr $5dd22.l
pub fn i_04fde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FDEE);
    cpu.pc = 0x5DD22;
    16
}
/// $04FDEE: move.b #$2, $21(a0)
pub fn i_04fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDF4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FDF4: lea.l $295916.l, a2
pub fn i_04fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDFA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295916;
    20
}
/// $04FDFA: move.l a2, $70(a0)
pub fn i_04fdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FDFE;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FDFE: move.l #$ffffffff, $74(a0)
pub fn i_04fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE06;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FE06: addi.w #$30, $22(a0)
pub fn i_04fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE0C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FE0C: addi.w #$90, $24(a0)
pub fn i_04fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE12;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FE12: movem.l a6, -(a7)
pub fn i_04fe12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE16;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FE16: movem.l a0, -(a7)
pub fn i_04fe16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE1A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FE1A: movea.l a0, a6
pub fn i_04fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE1C;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FE1C: lea.l $2946a2.l, a0
pub fn i_04fe1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FE22: jsr $28cd4.l
pub fn i_04fe22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FE28);
    cpu.pc = 0x28CD4;
    16
}
/// $04FE28: movem.l (a7)+, a0
pub fn i_04fe28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE2C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FE2C: movem.l (a7)+, a6
pub fn i_04fe2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE30;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FE30: rts
pub fn i_04fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece04_04fe32  ($04FE32, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FE32: lea.l $4f1f4(pc), a1
pub fn i_04fe32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FE36: jsr $4ae.l
pub fn i_04fe36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FE3C);
    cpu.pc = 0x4AE;
    16
}
/// $04FE3C: jsr $5dd22.l
pub fn i_04fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FE42);
    cpu.pc = 0x5DD22;
    16
}
/// $04FE42: move.b #$4, $21(a0)
pub fn i_04fe42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE48;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FE48: lea.l $29593e.l, a2
pub fn i_04fe48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE4E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29593E;
    20
}
/// $04FE4E: move.l a2, $70(a0)
pub fn i_04fe4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE52;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FE52: move.l #$ffffffff, $74(a0)
pub fn i_04fe52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE5A;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FE5A: addi.w #$50, $22(a0)
pub fn i_04fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE60;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FE60: addi.w #$90, $24(a0)
pub fn i_04fe60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE66;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FE66: movem.l a6, -(a7)
pub fn i_04fe66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE6A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FE6A: movem.l a0, -(a7)
pub fn i_04fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE6E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FE6E: movea.l a0, a6
pub fn i_04fe6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE70;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FE70: lea.l $2946a2.l, a0
pub fn i_04fe70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FE76: jsr $28cd4.l
pub fn i_04fe76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FE7C);
    cpu.pc = 0x28CD4;
    16
}
/// $04FE7C: movem.l (a7)+, a0
pub fn i_04fe7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE80;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FE80: movem.l (a7)+, a6
pub fn i_04fe80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE84;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FE84: rts
pub fn i_04fe84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece08_04fe86  ($04FE86, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FE86: lea.l $4f1f4(pc), a1
pub fn i_04fe86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FE8A: jsr $4ae.l
pub fn i_04fe8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FE90);
    cpu.pc = 0x4AE;
    16
}
/// $04FE90: jsr $5dd22.l
pub fn i_04fe90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FE96);
    cpu.pc = 0x5DD22;
    16
}
/// $04FE96: move.b #$8, $21(a0)
pub fn i_04fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE9C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FE9C: lea.l $295966.l, a2
pub fn i_04fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEA2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295966;
    20
}
/// $04FEA2: move.l a2, $70(a0)
pub fn i_04fea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEA6;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FEA6: move.l #$ffffffff, $74(a0)
pub fn i_04fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEAE;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FEAE: addi.w #$70, $22(a0)
pub fn i_04feae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEB4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FEB4: addi.w #$90, $24(a0)
pub fn i_04feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEBA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FEBA: movem.l a6, -(a7)
pub fn i_04feba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEBE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FEBE: movem.l a0, -(a7)
pub fn i_04febe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEC2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FEC2: movea.l a0, a6
pub fn i_04fec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEC4;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FEC4: lea.l $2946a2.l, a0
pub fn i_04fec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FECA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FECA: jsr $28cd4.l
pub fn i_04feca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FED0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FED0);
    cpu.pc = 0x28CD4;
    16
}
/// $04FED0: movem.l (a7)+, a0
pub fn i_04fed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FED4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FED4: movem.l (a7)+, a6
pub fn i_04fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FED8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FED8: rts
pub fn i_04fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece10_04feda  ($04FEDA, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FEDA: lea.l $4f1f4(pc), a1
pub fn i_04feda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FEDE: jsr $4ae.l
pub fn i_04fede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FEE4);
    cpu.pc = 0x4AE;
    16
}
/// $04FEE4: jsr $5dd22.l
pub fn i_04fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FEEA);
    cpu.pc = 0x5DD22;
    16
}
/// $04FEEA: move.b #$10, $21(a0)
pub fn i_04feea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEF0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FEF0: lea.l $29598e.l, a2
pub fn i_04fef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEF6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29598E;
    20
}
/// $04FEF6: move.l a2, $70(a0)
pub fn i_04fef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FEFA;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FEFA: move.l #$ffffffff, $74(a0)
pub fn i_04fefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF02;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FF02: addi.w #$10, $22(a0)
pub fn i_04ff02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF08;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FF08: addi.w #$70, $24(a0)
pub fn i_04ff08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF0E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FF0E: movem.l a6, -(a7)
pub fn i_04ff0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF12;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FF12: movem.l a0, -(a7)
pub fn i_04ff12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF16;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FF16: movea.l a0, a6
pub fn i_04ff16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF18;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FF18: lea.l $2946a2.l, a0
pub fn i_04ff18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FF1E: jsr $28cd4.l
pub fn i_04ff1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FF24);
    cpu.pc = 0x28CD4;
    16
}
/// $04FF24: movem.l (a7)+, a0
pub fn i_04ff24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF28;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FF28: movem.l (a7)+, a6
pub fn i_04ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF2C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FF2C: rts
pub fn i_04ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece20_04ff2e  ($04FF2E, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FF2E: lea.l $4f1f4(pc), a1
pub fn i_04ff2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FF32: jsr $4ae.l
pub fn i_04ff32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FF38);
    cpu.pc = 0x4AE;
    16
}
/// $04FF38: jsr $5dd22.l
pub fn i_04ff38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FF3E);
    cpu.pc = 0x5DD22;
    16
}
/// $04FF3E: move.b #$20, $21(a0)
pub fn i_04ff3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF44;
    cpu.ir = 0x117C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FF44: lea.l $2959b6.l, a2
pub fn i_04ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF4A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2959B6;
    20
}
/// $04FF4A: move.l a2, $70(a0)
pub fn i_04ff4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF4E;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FF4E: move.l #$ffffffff, $74(a0)
pub fn i_04ff4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF56;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FF56: addi.w #$30, $22(a0)
pub fn i_04ff56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF5C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FF5C: addi.w #$70, $24(a0)
pub fn i_04ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF62;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FF62: movem.l a6, -(a7)
pub fn i_04ff62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF66;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FF66: movem.l a0, -(a7)
pub fn i_04ff66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF6A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FF6A: movea.l a0, a6
pub fn i_04ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF6C;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FF6C: lea.l $2946a2.l, a0
pub fn i_04ff6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FF72: jsr $28cd4.l
pub fn i_04ff72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FF78);
    cpu.pc = 0x28CD4;
    16
}
/// $04FF78: movem.l (a7)+, a0
pub fn i_04ff78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF7C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FF7C: movem.l (a7)+, a6
pub fn i_04ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF80;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FF80: rts
pub fn i_04ff80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece40_04ff82  ($04FF82, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FF82: lea.l $4f1f4(pc), a1
pub fn i_04ff82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FF86: jsr $4ae.l
pub fn i_04ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FF8C);
    cpu.pc = 0x4AE;
    16
}
/// $04FF8C: jsr $5dd22.l
pub fn i_04ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FF92);
    cpu.pc = 0x5DD22;
    16
}
/// $04FF92: move.b #$40, $21(a0)
pub fn i_04ff92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF98;
    cpu.ir = 0x117C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FF98: lea.l $2959de.l, a2
pub fn i_04ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FF9E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2959DE;
    20
}
/// $04FF9E: move.l a2, $70(a0)
pub fn i_04ff9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFA2;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FFA2: move.l #$ffffffff, $74(a0)
pub fn i_04ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFAA;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FFAA: addi.w #$50, $22(a0)
pub fn i_04ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFB0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FFB0: addi.w #$70, $24(a0)
pub fn i_04ffb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFB6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04FFB6: movem.l a6, -(a7)
pub fn i_04ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFBA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FFBA: movem.l a0, -(a7)
pub fn i_04ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFBE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04FFBE: movea.l a0, a6
pub fn i_04ffbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFC0;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $04FFC0: lea.l $2946a2.l, a0
pub fn i_04ffc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $04FFC6: jsr $28cd4.l
pub fn i_04ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FFCC);
    cpu.pc = 0x28CD4;
    16
}
/// $04FFCC: movem.l (a7)+, a0
pub fn i_04ffcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFD0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FFD0: movem.l (a7)+, a6
pub fn i_04ffd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFD4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04FFD4: rts
pub fn i_04ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_SpawnPiece80_04ffd6  ($04FFD6, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FFD6: lea.l $4f1f4(pc), a1
pub fn i_04ffd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $04FFDA: jsr $4ae.l
pub fn i_04ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FFE0);
    cpu.pc = 0x4AE;
    16
}
/// $04FFE0: jsr $5dd22.l
pub fn i_04ffe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FFE6);
    cpu.pc = 0x5DD22;
    16
}
/// $04FFE6: move.b #$80, $21(a0)
pub fn i_04ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFEC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04FFEC: lea.l $295a06.l, a2
pub fn i_04ffec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFF2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A06;
    20
}
/// $04FFF2: move.l a2, $70(a0)
pub fn i_04fff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFF6;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FFF6: move.l #$ffffffff, $74(a0)
pub fn i_04fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FFFE;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04FFFE: addi.w #$70, $22(a0)
pub fn i_04fffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50004;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x04F002, i_04f002),
    (0x04F008, i_04f008),
    (0x04F00C, i_04f00c),
    (0x04F00E, i_04f00e),
    (0x04F014, i_04f014),
    (0x04F01A, i_04f01a),
    (0x04F01E, i_04f01e),
    (0x04F024, i_04f024),
    (0x04F028, i_04f028),
    (0x04F02E, i_04f02e),
    (0x04F030, i_04f030),
    (0x04F034, i_04f034),
    (0x04F03A, i_04f03a),
    (0x04F040, i_04f040),
    (0x04F046, i_04f046),
    (0x04F04C, i_04f04c),
    (0x04F052, i_04f052),
    (0x04F056, i_04f056),
    (0x04F058, i_04f058),
    (0x04F05E, i_04f05e),
    (0x04F064, i_04f064),
    (0x04F066, i_04f066),
    (0x04F06A, i_04f06a),
    (0x04F06E, i_04f06e),
    (0x04F074, i_04f074),
    (0x04F078, i_04f078),
    (0x04F07E, i_04f07e),
    (0x04F082, i_04f082),
    (0x04F088, i_04f088),
    (0x04F08E, i_04f08e),
    (0x04F094, i_04f094),
    (0x04F09A, i_04f09a),
    (0x04F09E, i_04f09e),
    (0x04F0A0, i_04f0a0),
    (0x04F0A4, i_04f0a4),
    (0x04F0A8, i_04f0a8),
    (0x04F0AE, i_04f0ae),
    (0x04F0B0, i_04f0b0),
    (0x04F0B6, i_04f0b6),
    (0x04F0BC, i_04f0bc),
    (0x04F0C2, i_04f0c2),
    (0x04F0C8, i_04f0c8),
    (0x04F0CC, i_04f0cc),
    (0x04F0CE, i_04f0ce),
    (0x04F0D4, i_04f0d4),
    (0x04F0DA, i_04f0da),
    (0x04F0E0, i_04f0e0),
    (0x04F0E4, i_04f0e4),
    (0x04F0E8, i_04f0e8),
    (0x04F0EE, i_04f0ee),
    (0x04F0F4, i_04f0f4),
    (0x04F0FA, i_04f0fa),
    (0x04F100, i_04f100),
    (0x04F106, i_04f106),
    (0x04F10C, i_04f10c),
    (0x04F112, i_04f112),
    (0x04F118, i_04f118),
    (0x04F11E, i_04f11e),
    (0x04F124, i_04f124),
    (0x04F128, i_04f128),
    (0x04F12C, i_04f12c),
    (0x04F130, i_04f130),
    (0x04F136, i_04f136),
    (0x04F138, i_04f138),
    (0x04F13C, i_04f13c),
    (0x04F142, i_04f142),
    (0x04F148, i_04f148),
    (0x04F14E, i_04f14e),
    (0x04F154, i_04f154),
    (0x04F15A, i_04f15a),
    (0x04F15E, i_04f15e),
    (0x04F160, i_04f160),
    (0x04F166, i_04f166),
    (0x04F16A, i_04f16a),
    (0x04F170, i_04f170),
    (0x04F174, i_04f174),
    (0x04F17A, i_04f17a),
    (0x04F180, i_04f180),
    (0x04F186, i_04f186),
    (0x04F18C, i_04f18c),
    (0x04F192, i_04f192),
    (0x04F198, i_04f198),
    (0x04F19E, i_04f19e),
    (0x04F1A4, i_04f1a4),
    (0x04F1AA, i_04f1aa),
    (0x04F1AE, i_04f1ae),
    (0x04F1B2, i_04f1b2),
    (0x04F1B8, i_04f1b8),
    (0x04F1BE, i_04f1be),
    (0x04F1C0, i_04f1c0),
    (0x04F1C4, i_04f1c4),
    (0x04F1CA, i_04f1ca),
    (0x04F1CE, i_04f1ce),
    (0x04F1D4, i_04f1d4),
    (0x04F1DA, i_04f1da),
    (0x04F1DE, i_04f1de),
    (0x04F1E4, i_04f1e4),
    (0x04F1E8, i_04f1e8),
    (0x04F1EC, i_04f1ec),
    (0x04F1F2, i_04f1f2),
    (0x04F1F4, i_04f1f4),
    (0x04F1FA, i_04f1fa),
    (0x04F200, i_04f200),
    (0x04F204, i_04f204),
    (0x04F206, i_04f206),
    (0x04F20A, i_04f20a),
    (0x04F210, i_04f210),
    (0x04F214, i_04f214),
    (0x04F21A, i_04f21a),
    (0x04F220, i_04f220),
    (0x04F226, i_04f226),
    (0x04F22A, i_04f22a),
    (0x04F230, i_04f230),
    (0x04F236, i_04f236),
    (0x04F23C, i_04f23c),
    (0x04F242, i_04f242),
    (0x04F246, i_04f246),
    (0x04F24C, i_04f24c),
    (0x04F252, i_04f252),
    (0x04F256, i_04f256),
    (0x04F25C, i_04f25c),
    (0x04F25E, i_04f25e),
    (0x04F262, i_04f262),
    (0x04F266, i_04f266),
    (0x04F26A, i_04f26a),
    (0x04F26E, i_04f26e),
    (0x04F274, i_04f274),
    (0x04F276, i_04f276),
    (0x04F27A, i_04f27a),
    (0x04F280, i_04f280),
    (0x04F284, i_04f284),
    (0x04F288, i_04f288),
    (0x04F28C, i_04f28c),
    (0x04F290, i_04f290),
    (0x04F294, i_04f294),
    (0x04F298, i_04f298),
    (0x04F29C, i_04f29c),
    (0x04F2A2, i_04f2a2),
    (0x04F2A4, i_04f2a4),
    (0x04F2AA, i_04f2aa),
    (0x04F2AE, i_04f2ae),
    (0x04F2B2, i_04f2b2),
    (0x04F2B6, i_04f2b6),
    (0x04F2BA, i_04f2ba),
    (0x04F2C0, i_04f2c0),
    (0x04F2C2, i_04f2c2),
    (0x04F2C6, i_04f2c6),
    (0x04F2CC, i_04f2cc),
    (0x04F2D2, i_04f2d2),
    (0x04F2D8, i_04f2d8),
    (0x04F2DE, i_04f2de),
    (0x04F2E4, i_04f2e4),
    (0x04F2EA, i_04f2ea),
    (0x04F2F0, i_04f2f0),
    (0x04F2F6, i_04f2f6),
    (0x04F2FA, i_04f2fa),
    (0x04F2FC, i_04f2fc),
    (0x04F302, i_04f302),
    (0x04F308, i_04f308),
    (0x04F30E, i_04f30e),
    (0x04F312, i_04f312),
    (0x04F318, i_04f318),
    (0x04F31E, i_04f31e),
    (0x04F324, i_04f324),
    (0x04F32A, i_04f32a),
    (0x04F32E, i_04f32e),
    (0x04F332, i_04f332),
    (0x04F334, i_04f334),
    (0x04F338, i_04f338),
    (0x04F33C, i_04f33c),
    (0x04F342, i_04f342),
    (0x04F344, i_04f344),
    (0x04F34A, i_04f34a),
    (0x04F34E, i_04f34e),
    (0x04F354, i_04f354),
    (0x04F35A, i_04f35a),
    (0x04F360, i_04f360),
    (0x04F366, i_04f366),
    (0x04F36C, i_04f36c),
    (0x04F370, i_04f370),
    (0x04F372, i_04f372),
    (0x04F378, i_04f378),
    (0x04F37E, i_04f37e),
    (0x04F382, i_04f382),
    (0x04F386, i_04f386),
    (0x04F38A, i_04f38a),
    (0x04F390, i_04f390),
    (0x04F394, i_04f394),
    (0x04F39A, i_04f39a),
    (0x04F3A0, i_04f3a0),
    (0x04F3A6, i_04f3a6),
    (0x04F3AA, i_04f3aa),
    (0x04F3AC, i_04f3ac),
    (0x04F3AE, i_04f3ae),
    (0x04F3B2, i_04f3b2),
    (0x04F3B6, i_04f3b6),
    (0x04F3BA, i_04f3ba),
    (0x04F3BE, i_04f3be),
    (0x04F3C4, i_04f3c4),
    (0x04F3CA, i_04f3ca),
    (0x04F3CE, i_04f3ce),
    (0x04F3D0, i_04f3d0),
    (0x04F3D4, i_04f3d4),
    (0x04F3D8, i_04f3d8),
    (0x04F3DC, i_04f3dc),
    (0x04F3DE, i_04f3de),
    (0x04F3E4, i_04f3e4),
    (0x04F3E6, i_04f3e6),
    (0x04F3EA, i_04f3ea),
    (0x04F3EE, i_04f3ee),
    (0x04F3F2, i_04f3f2),
    (0x04F3F6, i_04f3f6),
    (0x04F3FA, i_04f3fa),
    (0x04F3FE, i_04f3fe),
    (0x04F404, i_04f404),
    (0x04F40A, i_04f40a),
    (0x04F40C, i_04f40c),
    (0x04F410, i_04f410),
    (0x04F414, i_04f414),
    (0x04F418, i_04f418),
    (0x04F41C, i_04f41c),
    (0x04F422, i_04f422),
    (0x04F428, i_04f428),
    (0x04F42C, i_04f42c),
    (0x04F42E, i_04f42e),
    (0x04F432, i_04f432),
    (0x04F436, i_04f436),
    (0x04F43A, i_04f43a),
    (0x04F43C, i_04f43c),
    (0x04F442, i_04f442),
    (0x04F444, i_04f444),
    (0x04F448, i_04f448),
    (0x04F44C, i_04f44c),
    (0x04F450, i_04f450),
    (0x04F454, i_04f454),
    (0x04F458, i_04f458),
    (0x04F45C, i_04f45c),
    (0x04F462, i_04f462),
    (0x04F468, i_04f468),
    (0x04F46A, i_04f46a),
    (0x04F46E, i_04f46e),
    (0x04F474, i_04f474),
    (0x04F47A, i_04f47a),
    (0x04F47E, i_04f47e),
    (0x04F480, i_04f480),
    (0x04F486, i_04f486),
    (0x04F48C, i_04f48c),
    (0x04F492, i_04f492),
    (0x04F496, i_04f496),
    (0x04F49C, i_04f49c),
    (0x04F4A0, i_04f4a0),
    (0x04F4A4, i_04f4a4),
    (0x04F4A6, i_04f4a6),
    (0x04F4AA, i_04f4aa),
    (0x04F4AE, i_04f4ae),
    (0x04F4B4, i_04f4b4),
    (0x04F4B6, i_04f4b6),
    (0x04F4BC, i_04f4bc),
    (0x04F4C2, i_04f4c2),
    (0x04F4C6, i_04f4c6),
    (0x04F4C8, i_04f4c8),
    (0x04F4CE, i_04f4ce),
    (0x04F4D4, i_04f4d4),
    (0x04F4D8, i_04f4d8),
    (0x04F4DC, i_04f4dc),
    (0x04F4DE, i_04f4de),
    (0x04F4E2, i_04f4e2),
    (0x04F4E6, i_04f4e6),
    (0x04F4EC, i_04f4ec),
    (0x04F4EE, i_04f4ee),
    (0x04F4F2, i_04f4f2),
    (0x04F4F8, i_04f4f8),
    (0x04F4FE, i_04f4fe),
    (0x04F504, i_04f504),
    (0x04F50A, i_04f50a),
    (0x04F510, i_04f510),
    (0x04F516, i_04f516),
    (0x04F51C, i_04f51c),
    (0x04F520, i_04f520),
    (0x04F524, i_04f524),
    (0x04F526, i_04f526),
    (0x04F52C, i_04f52c),
    (0x04F532, i_04f532),
    (0x04F536, i_04f536),
    (0x04F53C, i_04f53c),
    (0x04F542, i_04f542),
    (0x04F548, i_04f548),
    (0x04F54E, i_04f54e),
    (0x04F552, i_04f552),
    (0x04F558, i_04f558),
    (0x04F55E, i_04f55e),
    (0x04F564, i_04f564),
    (0x04F56A, i_04f56a),
    (0x04F570, i_04f570),
    (0x04F574, i_04f574),
    (0x04F57A, i_04f57a),
    (0x04F580, i_04f580),
    (0x04F586, i_04f586),
    (0x04F58A, i_04f58a),
    (0x04F590, i_04f590),
    (0x04F596, i_04f596),
    (0x04F59C, i_04f59c),
    (0x04F5A0, i_04f5a0),
    (0x04F5A2, i_04f5a2),
    (0x04F5A6, i_04f5a6),
    (0x04F5AA, i_04f5aa),
    (0x04F5AE, i_04f5ae),
    (0x04F5B4, i_04f5b4),
    (0x04F5B6, i_04f5b6),
    (0x04F5BE, i_04f5be),
    (0x04F5C4, i_04f5c4),
    (0x04F5C8, i_04f5c8),
    (0x04F5CE, i_04f5ce),
    (0x04F5D4, i_04f5d4),
    (0x04F5DA, i_04f5da),
    (0x04F5E0, i_04f5e0),
    (0x04F5E6, i_04f5e6),
    (0x04F5EC, i_04f5ec),
    (0x04F5F2, i_04f5f2),
    (0x04F5F8, i_04f5f8),
    (0x04F5FC, i_04f5fc),
    (0x04F5FE, i_04f5fe),
    (0x04F604, i_04f604),
    (0x04F60A, i_04f60a),
    (0x04F610, i_04f610),
    (0x04F614, i_04f614),
    (0x04F618, i_04f618),
    (0x04F61A, i_04f61a),
    (0x04F620, i_04f620),
    (0x04F624, i_04f624),
    (0x04F628, i_04f628),
    (0x04F62E, i_04f62e),
    (0x04F634, i_04f634),
    (0x04F63A, i_04f63a),
    (0x04F640, i_04f640),
    (0x04F644, i_04f644),
    (0x04F64A, i_04f64a),
    (0x04F64C, i_04f64c),
    (0x04F652, i_04f652),
    (0x04F658, i_04f658),
    (0x04F65E, i_04f65e),
    (0x04F662, i_04f662),
    (0x04F664, i_04f664),
    (0x04F66A, i_04f66a),
    (0x04F66E, i_04f66e),
    (0x04F672, i_04f672),
    (0x04F674, i_04f674),
    (0x04F67A, i_04f67a),
    (0x04F680, i_04f680),
    (0x04F686, i_04f686),
    (0x04F68A, i_04f68a),
    (0x04F690, i_04f690),
    (0x04F692, i_04f692),
    (0x04F698, i_04f698),
    (0x04F69E, i_04f69e),
    (0x04F6A4, i_04f6a4),
    (0x04F6AA, i_04f6aa),
    (0x04F6B0, i_04f6b0),
    (0x04F6B4, i_04f6b4),
    (0x04F6BA, i_04f6ba),
    (0x04F6C0, i_04f6c0),
    (0x04F6C4, i_04f6c4),
    (0x04F6C6, i_04f6c6),
    (0x04F6CC, i_04f6cc),
    (0x04F6D2, i_04f6d2),
    (0x04F6D8, i_04f6d8),
    (0x04F6DC, i_04f6dc),
    (0x04F6E0, i_04f6e0),
    (0x04F6E2, i_04f6e2),
    (0x04F6E8, i_04f6e8),
    (0x04F6EC, i_04f6ec),
    (0x04F6F0, i_04f6f0),
    (0x04F6F4, i_04f6f4),
    (0x04F6F6, i_04f6f6),
    (0x04F6FC, i_04f6fc),
    (0x04F702, i_04f702),
    (0x04F706, i_04f706),
    (0x04F70C, i_04f70c),
    (0x04F70E, i_04f70e),
    (0x04F712, i_04f712),
    (0x04F718, i_04f718),
    (0x04F71E, i_04f71e),
    (0x04F724, i_04f724),
    (0x04F72A, i_04f72a),
    (0x04F730, i_04f730),
    (0x04F736, i_04f736),
    (0x04F73C, i_04f73c),
    (0x04F740, i_04f740),
    (0x04F742, i_04f742),
    (0x04F748, i_04f748),
    (0x04F74E, i_04f74e),
    (0x04F754, i_04f754),
    (0x04F75A, i_04f75a),
    (0x04F760, i_04f760),
    (0x04F764, i_04f764),
    (0x04F76A, i_04f76a),
    (0x04F76C, i_04f76c),
    (0x04F770, i_04f770),
    (0x04F776, i_04f776),
    (0x04F77C, i_04f77c),
    (0x04F782, i_04f782),
    (0x04F788, i_04f788),
    (0x04F78E, i_04f78e),
    (0x04F794, i_04f794),
    (0x04F79A, i_04f79a),
    (0x04F79E, i_04f79e),
    (0x04F7A0, i_04f7a0),
    (0x04F7A6, i_04f7a6),
    (0x04F7AC, i_04f7ac),
    (0x04F7B2, i_04f7b2),
    (0x04F7B8, i_04f7b8),
    (0x04F7BE, i_04f7be),
    (0x04F7C2, i_04f7c2),
    (0x04F7C8, i_04f7c8),
    (0x04F7CA, i_04f7ca),
    (0x04F7CE, i_04f7ce),
    (0x04F7D4, i_04f7d4),
    (0x04F7D8, i_04f7d8),
    (0x04F7DE, i_04f7de),
    (0x04F7E2, i_04f7e2),
    (0x04F7E8, i_04f7e8),
    (0x04F7EE, i_04f7ee),
    (0x04F7F4, i_04f7f4),
    (0x04F7F8, i_04f7f8),
    (0x04F7FE, i_04f7fe),
    (0x04F804, i_04f804),
    (0x04F80A, i_04f80a),
    (0x04F80E, i_04f80e),
    (0x04F814, i_04f814),
    (0x04F818, i_04f818),
    (0x04F81C, i_04f81c),
    (0x04F822, i_04f822),
    (0x04F826, i_04f826),
    (0x04F82C, i_04f82c),
    (0x04F832, i_04f832),
    (0x04F838, i_04f838),
    (0x04F840, i_04f840),
    (0x04F846, i_04f846),
    (0x04F84A, i_04f84a),
    (0x04F84C, i_04f84c),
    (0x04F852, i_04f852),
    (0x04F858, i_04f858),
    (0x04F85C, i_04f85c),
    (0x04F862, i_04f862),
    (0x04F866, i_04f866),
    (0x04F86A, i_04f86a),
    (0x04F86E, i_04f86e),
    (0x04F874, i_04f874),
    (0x04F878, i_04f878),
    (0x04F87C, i_04f87c),
    (0x04F880, i_04f880),
    (0x04F884, i_04f884),
    (0x04F888, i_04f888),
    (0x04F88E, i_04f88e),
    (0x04F894, i_04f894),
    (0x04F898, i_04f898),
    (0x04F8A0, i_04f8a0),
    (0x04F8A4, i_04f8a4),
    (0x04F8A8, i_04f8a8),
    (0x04F8AC, i_04f8ac),
    (0x04F8B2, i_04f8b2),
    (0x04F8B6, i_04f8b6),
    (0x04F8BC, i_04f8bc),
    (0x04F8C2, i_04f8c2),
    (0x04F8C6, i_04f8c6),
    (0x04F8CC, i_04f8cc),
    (0x04F8D2, i_04f8d2),
    (0x04F8D6, i_04f8d6),
    (0x04F8DA, i_04f8da),
    (0x04F8E0, i_04f8e0),
    (0x04F8E4, i_04f8e4),
    (0x04F8EA, i_04f8ea),
    (0x04F8F0, i_04f8f0),
    (0x04F8F6, i_04f8f6),
    (0x04F8FC, i_04f8fc),
    (0x04F902, i_04f902),
    (0x04F908, i_04f908),
    (0x04F90E, i_04f90e),
    (0x04F914, i_04f914),
    (0x04F91A, i_04f91a),
    (0x04F91E, i_04f91e),
    (0x04F922, i_04f922),
    (0x04F926, i_04f926),
    (0x04F92C, i_04f92c),
    (0x04F932, i_04f932),
    (0x04F936, i_04f936),
    (0x04F93C, i_04f93c),
    (0x04F942, i_04f942),
    (0x04F948, i_04f948),
    (0x04F94C, i_04f94c),
    (0x04F952, i_04f952),
    (0x04F954, i_04f954),
    (0x04F95A, i_04f95a),
    (0x04F95E, i_04f95e),
    (0x04F960, i_04f960),
    (0x04F966, i_04f966),
    (0x04F96C, i_04f96c),
    (0x04F970, i_04f970),
    (0x04F976, i_04f976),
    (0x04F97C, i_04f97c),
    (0x04F982, i_04f982),
    (0x04F986, i_04f986),
    (0x04F98A, i_04f98a),
    (0x04F990, i_04f990),
    (0x04F992, i_04f992),
    (0x04F996, i_04f996),
    (0x04F99C, i_04f99c),
    (0x04F9A2, i_04f9a2),
    (0x04F9A8, i_04f9a8),
    (0x04F9AE, i_04f9ae),
    (0x04F9B4, i_04f9b4),
    (0x04F9BA, i_04f9ba),
    (0x04F9BC, i_04f9bc),
    (0x04F9C0, i_04f9c0),
    (0x04F9C4, i_04f9c4),
    (0x04F9C6, i_04f9c6),
    (0x04F9CA, i_04f9ca),
    (0x04F9CE, i_04f9ce),
    (0x04F9D2, i_04f9d2),
    (0x04F9D4, i_04f9d4),
    (0x04F9D8, i_04f9d8),
    (0x04F9DA, i_04f9da),
    (0x04F9DE, i_04f9de),
    (0x04F9E2, i_04f9e2),
    (0x04F9E6, i_04f9e6),
    (0x04F9EC, i_04f9ec),
    (0x04F9F0, i_04f9f0),
    (0x04F9F2, i_04f9f2),
    (0x04F9F8, i_04f9f8),
    (0x04F9FE, i_04f9fe),
    (0x04FA04, i_04fa04),
    (0x04FA08, i_04fa08),
    (0x04FA0C, i_04fa0c),
    (0x04FA12, i_04fa12),
    (0x04FA18, i_04fa18),
    (0x04FA1E, i_04fa1e),
    (0x04FA22, i_04fa22),
    (0x04FA28, i_04fa28),
    (0x04FA2E, i_04fa2e),
    (0x04FA34, i_04fa34),
    (0x04FA38, i_04fa38),
    (0x04FA3E, i_04fa3e),
    (0x04FA40, i_04fa40),
    (0x04FA44, i_04fa44),
    (0x04FA48, i_04fa48),
    (0x04FA4C, i_04fa4c),
    (0x04FA50, i_04fa50),
    (0x04FA56, i_04fa56),
    (0x04FA58, i_04fa58),
    (0x04FA5C, i_04fa5c),
    (0x04FA60, i_04fa60),
    (0x04FA64, i_04fa64),
    (0x04FA68, i_04fa68),
    (0x04FA6E, i_04fa6e),
    (0x04FA70, i_04fa70),
    (0x04FA74, i_04fa74),
    (0x04FA76, i_04fa76),
    (0x04FA7A, i_04fa7a),
    (0x04FA7E, i_04fa7e),
    (0x04FA82, i_04fa82),
    (0x04FA84, i_04fa84),
    (0x04FA88, i_04fa88),
    (0x04FA8A, i_04fa8a),
    (0x04FA90, i_04fa90),
    (0x04FA94, i_04fa94),
    (0x04FA9A, i_04fa9a),
    (0x04FA9E, i_04fa9e),
    (0x04FAA0, i_04faa0),
    (0x04FAA6, i_04faa6),
    (0x04FAAA, i_04faaa),
    (0x04FAB0, i_04fab0),
    (0x04FAB4, i_04fab4),
    (0x04FAB6, i_04fab6),
    (0x04FABC, i_04fabc),
    (0x04FAC0, i_04fac0),
    (0x04FAC6, i_04fac6),
    (0x04FACA, i_04faca),
    (0x04FACC, i_04facc),
    (0x04FAD2, i_04fad2),
    (0x04FAD6, i_04fad6),
    (0x04FADC, i_04fadc),
    (0x04FAE0, i_04fae0),
    (0x04FAE2, i_04fae2),
    (0x04FAE8, i_04fae8),
    (0x04FAEC, i_04faec),
    (0x04FAF2, i_04faf2),
    (0x04FAF6, i_04faf6),
    (0x04FAF8, i_04faf8),
    (0x04FAFE, i_04fafe),
    (0x04FB02, i_04fb02),
    (0x04FB08, i_04fb08),
    (0x04FB0C, i_04fb0c),
    (0x04FB0E, i_04fb0e),
    (0x04FB14, i_04fb14),
    (0x04FB18, i_04fb18),
    (0x04FB1E, i_04fb1e),
    (0x04FB22, i_04fb22),
    (0x04FB24, i_04fb24),
    (0x04FB2A, i_04fb2a),
    (0x04FB2E, i_04fb2e),
    (0x04FB34, i_04fb34),
    (0x04FB38, i_04fb38),
    (0x04FB3A, i_04fb3a),
    (0x04FB3C, i_04fb3c),
    (0x04FB42, i_04fb42),
    (0x04FB46, i_04fb46),
    (0x04FB4C, i_04fb4c),
    (0x04FB50, i_04fb50),
    (0x04FB56, i_04fb56),
    (0x04FB5A, i_04fb5a),
    (0x04FB5E, i_04fb5e),
    (0x04FB64, i_04fb64),
    (0x04FB68, i_04fb68),
    (0x04FB6E, i_04fb6e),
    (0x04FB72, i_04fb72),
    (0x04FB74, i_04fb74),
    (0x04FB7A, i_04fb7a),
    (0x04FB7E, i_04fb7e),
    (0x04FB84, i_04fb84),
    (0x04FB88, i_04fb88),
    (0x04FB8A, i_04fb8a),
    (0x04FB8E, i_04fb8e),
    (0x04FB94, i_04fb94),
    (0x04FB9A, i_04fb9a),
    (0x04FBA0, i_04fba0),
    (0x04FBA6, i_04fba6),
    (0x04FBAA, i_04fbaa),
    (0x04FBB0, i_04fbb0),
    (0x04FBB4, i_04fbb4),
    (0x04FBBC, i_04fbbc),
    (0x04FBC2, i_04fbc2),
    (0x04FBC8, i_04fbc8),
    (0x04FBCC, i_04fbcc),
    (0x04FBCE, i_04fbce),
    (0x04FBD4, i_04fbd4),
    (0x04FBDA, i_04fbda),
    (0x04FBDE, i_04fbde),
    (0x04FBE0, i_04fbe0),
    (0x04FBE4, i_04fbe4),
    (0x04FBEA, i_04fbea),
    (0x04FBF0, i_04fbf0),
    (0x04FBF6, i_04fbf6),
    (0x04FBFC, i_04fbfc),
    (0x04FC00, i_04fc00),
    (0x04FC06, i_04fc06),
    (0x04FC0A, i_04fc0a),
    (0x04FC12, i_04fc12),
    (0x04FC18, i_04fc18),
    (0x04FC1E, i_04fc1e),
    (0x04FC22, i_04fc22),
    (0x04FC24, i_04fc24),
    (0x04FC2A, i_04fc2a),
    (0x04FC30, i_04fc30),
    (0x04FC34, i_04fc34),
    (0x04FC36, i_04fc36),
    (0x04FC3A, i_04fc3a),
    (0x04FC40, i_04fc40),
    (0x04FC46, i_04fc46),
    (0x04FC4C, i_04fc4c),
    (0x04FC52, i_04fc52),
    (0x04FC56, i_04fc56),
    (0x04FC5C, i_04fc5c),
    (0x04FC60, i_04fc60),
    (0x04FC68, i_04fc68),
    (0x04FC6E, i_04fc6e),
    (0x04FC74, i_04fc74),
    (0x04FC78, i_04fc78),
    (0x04FC7A, i_04fc7a),
    (0x04FC80, i_04fc80),
    (0x04FC86, i_04fc86),
    (0x04FC8A, i_04fc8a),
    (0x04FC8C, i_04fc8c),
    (0x04FC90, i_04fc90),
    (0x04FC96, i_04fc96),
    (0x04FC9C, i_04fc9c),
    (0x04FCA2, i_04fca2),
    (0x04FCA8, i_04fca8),
    (0x04FCAC, i_04fcac),
    (0x04FCB4, i_04fcb4),
    (0x04FCBA, i_04fcba),
    (0x04FCC0, i_04fcc0),
    (0x04FCC4, i_04fcc4),
    (0x04FCC6, i_04fcc6),
    (0x04FCCC, i_04fccc),
    (0x04FCD2, i_04fcd2),
    (0x04FCD6, i_04fcd6),
    (0x04FCD8, i_04fcd8),
    (0x04FCDC, i_04fcdc),
    (0x04FCE2, i_04fce2),
    (0x04FCE8, i_04fce8),
    (0x04FCEE, i_04fcee),
    (0x04FCF4, i_04fcf4),
    (0x04FCF8, i_04fcf8),
    (0x04FD00, i_04fd00),
    (0x04FD06, i_04fd06),
    (0x04FD0C, i_04fd0c),
    (0x04FD10, i_04fd10),
    (0x04FD14, i_04fd14),
    (0x04FD16, i_04fd16),
    (0x04FD1C, i_04fd1c),
    (0x04FD22, i_04fd22),
    (0x04FD26, i_04fd26),
    (0x04FD2A, i_04fd2a),
    (0x04FD2C, i_04fd2c),
    (0x04FD30, i_04fd30),
    (0x04FD36, i_04fd36),
    (0x04FD3C, i_04fd3c),
    (0x04FD42, i_04fd42),
    (0x04FD48, i_04fd48),
    (0x04FD4C, i_04fd4c),
    (0x04FD52, i_04fd52),
    (0x04FD56, i_04fd56),
    (0x04FD5E, i_04fd5e),
    (0x04FD64, i_04fd64),
    (0x04FD6A, i_04fd6a),
    (0x04FD6E, i_04fd6e),
    (0x04FD72, i_04fd72),
    (0x04FD74, i_04fd74),
    (0x04FD7A, i_04fd7a),
    (0x04FD80, i_04fd80),
    (0x04FD84, i_04fd84),
    (0x04FD88, i_04fd88),
    (0x04FD8A, i_04fd8a),
    (0x04FD8E, i_04fd8e),
    (0x04FD94, i_04fd94),
    (0x04FD9A, i_04fd9a),
    (0x04FDA0, i_04fda0),
    (0x04FDA6, i_04fda6),
    (0x04FDAA, i_04fdaa),
    (0x04FDB2, i_04fdb2),
    (0x04FDB8, i_04fdb8),
    (0x04FDBE, i_04fdbe),
    (0x04FDC2, i_04fdc2),
    (0x04FDC6, i_04fdc6),
    (0x04FDC8, i_04fdc8),
    (0x04FDCE, i_04fdce),
    (0x04FDD4, i_04fdd4),
    (0x04FDD8, i_04fdd8),
    (0x04FDDC, i_04fddc),
    (0x04FDDE, i_04fdde),
    (0x04FDE2, i_04fde2),
    (0x04FDE8, i_04fde8),
    (0x04FDEE, i_04fdee),
    (0x04FDF4, i_04fdf4),
    (0x04FDFA, i_04fdfa),
    (0x04FDFE, i_04fdfe),
    (0x04FE06, i_04fe06),
    (0x04FE0C, i_04fe0c),
    (0x04FE12, i_04fe12),
    (0x04FE16, i_04fe16),
    (0x04FE1A, i_04fe1a),
    (0x04FE1C, i_04fe1c),
    (0x04FE22, i_04fe22),
    (0x04FE28, i_04fe28),
    (0x04FE2C, i_04fe2c),
    (0x04FE30, i_04fe30),
    (0x04FE32, i_04fe32),
    (0x04FE36, i_04fe36),
    (0x04FE3C, i_04fe3c),
    (0x04FE42, i_04fe42),
    (0x04FE48, i_04fe48),
    (0x04FE4E, i_04fe4e),
    (0x04FE52, i_04fe52),
    (0x04FE5A, i_04fe5a),
    (0x04FE60, i_04fe60),
    (0x04FE66, i_04fe66),
    (0x04FE6A, i_04fe6a),
    (0x04FE6E, i_04fe6e),
    (0x04FE70, i_04fe70),
    (0x04FE76, i_04fe76),
    (0x04FE7C, i_04fe7c),
    (0x04FE80, i_04fe80),
    (0x04FE84, i_04fe84),
    (0x04FE86, i_04fe86),
    (0x04FE8A, i_04fe8a),
    (0x04FE90, i_04fe90),
    (0x04FE96, i_04fe96),
    (0x04FE9C, i_04fe9c),
    (0x04FEA2, i_04fea2),
    (0x04FEA6, i_04fea6),
    (0x04FEAE, i_04feae),
    (0x04FEB4, i_04feb4),
    (0x04FEBA, i_04feba),
    (0x04FEBE, i_04febe),
    (0x04FEC2, i_04fec2),
    (0x04FEC4, i_04fec4),
    (0x04FECA, i_04feca),
    (0x04FED0, i_04fed0),
    (0x04FED4, i_04fed4),
    (0x04FED8, i_04fed8),
    (0x04FEDA, i_04feda),
    (0x04FEDE, i_04fede),
    (0x04FEE4, i_04fee4),
    (0x04FEEA, i_04feea),
    (0x04FEF0, i_04fef0),
    (0x04FEF6, i_04fef6),
    (0x04FEFA, i_04fefa),
    (0x04FF02, i_04ff02),
    (0x04FF08, i_04ff08),
    (0x04FF0E, i_04ff0e),
    (0x04FF12, i_04ff12),
    (0x04FF16, i_04ff16),
    (0x04FF18, i_04ff18),
    (0x04FF1E, i_04ff1e),
    (0x04FF24, i_04ff24),
    (0x04FF28, i_04ff28),
    (0x04FF2C, i_04ff2c),
    (0x04FF2E, i_04ff2e),
    (0x04FF32, i_04ff32),
    (0x04FF38, i_04ff38),
    (0x04FF3E, i_04ff3e),
    (0x04FF44, i_04ff44),
    (0x04FF4A, i_04ff4a),
    (0x04FF4E, i_04ff4e),
    (0x04FF56, i_04ff56),
    (0x04FF5C, i_04ff5c),
    (0x04FF62, i_04ff62),
    (0x04FF66, i_04ff66),
    (0x04FF6A, i_04ff6a),
    (0x04FF6C, i_04ff6c),
    (0x04FF72, i_04ff72),
    (0x04FF78, i_04ff78),
    (0x04FF7C, i_04ff7c),
    (0x04FF80, i_04ff80),
    (0x04FF82, i_04ff82),
    (0x04FF86, i_04ff86),
    (0x04FF8C, i_04ff8c),
    (0x04FF92, i_04ff92),
    (0x04FF98, i_04ff98),
    (0x04FF9E, i_04ff9e),
    (0x04FFA2, i_04ffa2),
    (0x04FFAA, i_04ffaa),
    (0x04FFB0, i_04ffb0),
    (0x04FFB6, i_04ffb6),
    (0x04FFBA, i_04ffba),
    (0x04FFBE, i_04ffbe),
    (0x04FFC0, i_04ffc0),
    (0x04FFC6, i_04ffc6),
    (0x04FFCC, i_04ffcc),
    (0x04FFD0, i_04ffd0),
    (0x04FFD4, i_04ffd4),
    (0x04FFD6, i_04ffd6),
    (0x04FFDA, i_04ffda),
    (0x04FFE0, i_04ffe0),
    (0x04FFE6, i_04ffe6),
    (0x04FFEC, i_04ffec),
    (0x04FFF2, i_04fff2),
    (0x04FFF6, i_04fff6),
    (0x04FFFE, i_04fffe),
];
