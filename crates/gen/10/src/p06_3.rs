// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0651D8: lea.l $58fe2.l, a1
pub fn i_0651d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FE2;
    20
}
/// $0651DE: move.l a1, (a6)
pub fn i_0651de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0651E0: jmp $58fe2.l
pub fn i_0651e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651E6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x58FE2;
    8
}
// ===== Patrol_Grenade_0651e6  ($0651E6, 142 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0651E6: move.w #$a1, d1
pub fn i_0651e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0651EA: jsr $236e.l
pub fn i_0651ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x651F0);
    cpu.pc = 0x236E;
    16
}
/// $0651F0: jsr $5e9b6.l
pub fn i_0651f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x651F6);
    cpu.pc = 0x5E9B6;
    16
}
/// $0651F6: andi.w #$1, d0
pub fn i_0651f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651FA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0651FA: beq.w $6520e
pub fn i_0651fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6520E; return 10; }
    8
}
/// $0651FE: lea.l $29c8d6.l, a0
pub fn i_0651fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65204;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C8D6;
    20
}
/// $065204: jsr $28cd4.l
pub fn i_065204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6520A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6520A);
    cpu.pc = 0x28CD4;
    16
}
/// $06520A: bra.w $6521a
pub fn i_06520a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6520E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6521A;
    10
}
/// $06520E: lea.l $29c920.l, a0
pub fn i_06520e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65214;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C920;
    20
}
/// $065214: jsr $28cd4.l
pub fn i_065214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6521A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6521A);
    cpu.pc = 0x28CD4;
    16
}
/// $06521A: move.w #$d000, $38(a6)
pub fn i_06521a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65220;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065220: lea.l $65226(pc), a1
pub fn i_065220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65224;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65226;
    16
}
/// $065224: move.l a1, (a6)
pub fn i_065224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65226;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065226: jsr $27d50.l
pub fn i_065226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6522C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6522C);
    cpu.pc = 0x27D50;
    16
}
/// $06522C: bcc.w $65236
pub fn i_06522c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65230;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65236; return 10; }
    8
}
/// $065230: lea.l $6527c(pc), a1
pub fn i_065230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65234;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6527C;
    16
}
/// $065234: move.l a1, (a6)
pub fn i_065234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65236;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065236: jsr $28d70.l
pub fn i_065236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6523C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6523C);
    cpu.pc = 0x28D70;
    16
}
/// $06523C: cmpi.w #$120, $24(a6)
pub fn i_06523c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65242;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $065242: bgt.w $6524c
pub fn i_065242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65246;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6524C; return 10; }
    8
}
/// $065246: lea.l $6527c(pc), a1
pub fn i_065246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6524A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6527C;
    16
}
/// $06524A: move.l a1, (a6)
pub fn i_06524a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6524C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06524C: jsr $2870a.l
pub fn i_06524c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65252);
    cpu.pc = 0x2870A;
    16
}
/// $065252: jsr $49fd0.l
pub fn i_065252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65258;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65258);
    cpu.pc = 0x49FD0;
    16
}
/// $065258: lea.l $5e766.l, a0
pub fn i_065258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6525E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06525E: jsr $5e770.l
pub fn i_06525e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65264);
    cpu.pc = 0x5E770;
    16
}
/// $065264: movea.l #$ffffffff, a0
pub fn i_065264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6526A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06526A: jsr $5dd56.l
pub fn i_06526a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65270);
    cpu.pc = 0x5DD56;
    16
}
/// $065270: bcc.w $6527a
pub fn i_065270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65274;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6527A; return 10; }
    8
}
// ===== SetTaskHandler_065274  ($065274, 8 B, kernel/task_handlers.c)
/// $065274: lea.l $64d8a(pc), a1
pub fn i_065274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065278: move.l a1, (a6)
pub fn i_065278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6527A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06527A: rts
pub fn i_06527a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6527C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Smoke_06527c  ($06527C, 112 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06527C: move.w #$1e, $70(a6)
pub fn i_06527c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65282;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065282: jsr $267e2.l
pub fn i_065282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65288;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65288);
    cpu.pc = 0x267E2;
    16
}
/// $065288: bset.b #$6, $12(a6)
pub fn i_065288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6528E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06528E: move.w #$f000, d0
pub fn i_06528e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65292;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065292: jsr $28134.l
pub fn i_065292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65298);
    cpu.pc = 0x28134;
    16
}
/// $065298: andi.w #$ffe3, $38(a6)
pub fn i_065298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6529E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06529E: ori.w #$1c, $38(a6)
pub fn i_06529e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652A4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0652A4: lea.l $78908.l, a1
pub fn i_0652a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652AA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78908;
    20
}
/// $0652AA: jsr $4ae.l
pub fn i_0652aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x652B0);
    cpu.pc = 0x4AE;
    16
}
/// $0652B0: jsr $5dd02.l
pub fn i_0652b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x652B6);
    cpu.pc = 0x5DD02;
    16
}
/// $0652B6: lea.l $652bc(pc), a1
pub fn i_0652b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x652BC;
    16
}
/// $0652BA: move.l a1, (a6)
pub fn i_0652ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0652BC: move.b #$1, $45(a6)
pub fn i_0652bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0652C2: jsr $2783a.l
pub fn i_0652c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x652C8);
    cpu.pc = 0x2783A;
    16
}
/// $0652C8: subq.w #$1, $70(a6)
pub fn i_0652c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652CC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0652CC: cmpi.w #$0, $70(a6)
pub fn i_0652cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0652D2: bgt.w $652dc
pub fn i_0652d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652D6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x652DC; return 10; }
    8
}
/// $0652D6: lea.l $652f4(pc), a1
pub fn i_0652d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x652F4;
    16
}
/// $0652DA: move.l a1, (a6)
pub fn i_0652da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0652DC: movea.l #$ffffffff, a0
pub fn i_0652dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652E2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0652E2: jsr $5dd56.l
pub fn i_0652e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x652E8);
    cpu.pc = 0x5DD56;
    16
}
/// $0652E8: bcc.w $652f2
pub fn i_0652e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x652F2; return 10; }
    8
}
// ===== SetTaskHandler_0652ec  ($0652EC, 8 B, kernel/task_handlers.c)
/// $0652EC: lea.l $64d8a(pc), a1
pub fn i_0652ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0652F0: move.l a1, (a6)
pub fn i_0652f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0652F2: rts
pub fn i_0652f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Shell_0652f4  ($0652F4, 126 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0652F4: subi.w #$18, $24(a6)
pub fn i_0652f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652FA;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0652FA: move.w #$100, d0
pub fn i_0652fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x652FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0652FE: btst.b #$0, $3a(a6)
pub fn i_0652fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65304;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065304: bne.w $6530a
pub fn i_065304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65308;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6530A; return 10; }
    8
}
/// $065308: neg.w d0
pub fn i_065308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6530A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06530A: move.w d0, $28(a6)
pub fn i_06530a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6530E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06530E: move.w #$160, d1
pub fn i_06530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65312;
    cpu.ir = 0x323C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065312: jsr $236e.l
pub fn i_065312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65318);
    cpu.pc = 0x236E;
    16
}
/// $065318: lea.l $29c962.l, a0
pub fn i_065318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6531E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C962;
    20
}
/// $06531E: jsr $28cd4.l
pub fn i_06531e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65324);
    cpu.pc = 0x28CD4;
    16
}
/// $065324: lea.l $6532a(pc), a1
pub fn i_065324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65328;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6532A;
    16
}
/// $065328: move.l a1, (a6)
pub fn i_065328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6532A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06532A: jsr $27cee.l
pub fn i_06532a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65330;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65330);
    cpu.pc = 0x27CEE;
    16
}
/// $065330: jsr $28d70.l
pub fn i_065330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65336);
    cpu.pc = 0x28D70;
    16
}
/// $065336: bcc.w $65346
pub fn i_065336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6533A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65346; return 10; }
    8
}
/// $06533A: lea.l $29c9c2.l, a0
pub fn i_06533a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65340;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C9C2;
    20
}
/// $065340: jsr $28cd4.l
pub fn i_065340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65346);
    cpu.pc = 0x28CD4;
    16
}
/// $065346: jsr $2870a.l
pub fn i_065346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6534C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6534C);
    cpu.pc = 0x2870A;
    16
}
/// $06534C: bcc.w $65356
pub fn i_06534c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65350;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65356; return 10; }
    8
}
/// $065350: lea.l $6537a(pc), a1
pub fn i_065350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65354;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6537A;
    16
}
/// $065354: move.l a1, (a6)
pub fn i_065354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65356;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065356: lea.l $5e766.l, a0
pub fn i_065356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6535C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06535C: jsr $5e770.l
pub fn i_06535c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65362);
    cpu.pc = 0x5E770;
    16
}
/// $065362: movea.l #$ffffffff, a0
pub fn i_065362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65368;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $065368: jsr $5dd56.l
pub fn i_065368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6536E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6536E);
    cpu.pc = 0x5DD56;
    16
}
/// $06536E: bcc.w $65378
pub fn i_06536e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65372;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65378; return 10; }
    8
}
// ===== SetTaskHandler_065372  ($065372, 8 B, kernel/task_handlers.c)
/// $065372: lea.l $64d8a(pc), a1
pub fn i_065372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65376;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065376: move.l a1, (a6)
pub fn i_065376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65378;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065378: rts
pub fn i_065378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6537A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_ShellB_06537a  ($06537A, 60 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06537A: asr.w $28(a6)
pub fn i_06537a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6537E;
    cpu.ir = 0xE0EE;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = do_shift_rotate(cpu, v2, 1, Size::Word, 0, false);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06537E: lea.l $29c992.l, a0
pub fn i_06537e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65384;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C992;
    20
}
/// $065384: jsr $28cd4.l
pub fn i_065384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6538A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6538A);
    cpu.pc = 0x28CD4;
    16
}
/// $06538A: lea.l $65390(pc), a1
pub fn i_06538a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6538E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65390;
    16
}
/// $06538E: move.l a1, (a6)
pub fn i_06538e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65390;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065390: jsr $27cee.l
pub fn i_065390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65396;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65396);
    cpu.pc = 0x27CEE;
    16
}
/// $065396: jsr $28d70.l
pub fn i_065396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6539C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6539C);
    cpu.pc = 0x28D70;
    16
}
/// $06539C: bcc.w $653a6
pub fn i_06539c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x653A6; return 10; }
    8
}
/// $0653A0: lea.l $653be(pc), a1
pub fn i_0653a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x653BE;
    16
}
/// $0653A4: move.l a1, (a6)
pub fn i_0653a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0653A6: movea.l #$ffffffff, a0
pub fn i_0653a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653AC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0653AC: jsr $5dd56.l
pub fn i_0653ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x653B2);
    cpu.pc = 0x5DD56;
    16
}
/// $0653B2: bcc.w $653bc
pub fn i_0653b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x653BC; return 10; }
    8
}
// ===== SetTaskHandler_0653b6  ($0653B6, 8 B, kernel/task_handlers.c)
/// $0653B6: lea.l $64d8a(pc), a1
pub fn i_0653b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0653BA: move.l a1, (a6)
pub fn i_0653ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0653BC: rts
pub fn i_0653bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Burst_0653be  ($0653BE, 78 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0653BE: addi.w #$18, $24(a6)
pub fn i_0653be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653C4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0653C4: jsr $13600.l
pub fn i_0653c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x653CA);
    cpu.pc = 0x13600;
    16
}
/// $0653CA: move.w #$c, d1
pub fn i_0653ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0653CE: jsr $236e.l
pub fn i_0653ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x653D4);
    cpu.pc = 0x236E;
    16
}
/// $0653D4: lea.l $2de6e0.l, a0
pub fn i_0653d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE6E0;
    20
}
/// $0653DA: jsr $28cd4.l
pub fn i_0653da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x653E0);
    cpu.pc = 0x28CD4;
    16
}
/// $0653E0: lea.l $653e6(pc), a1
pub fn i_0653e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x653E6;
    16
}
/// $0653E4: move.l a1, (a6)
pub fn i_0653e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0653E6: jsr $2783a.l
pub fn i_0653e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x653EC);
    cpu.pc = 0x2783A;
    16
}
/// $0653EC: jsr $28d70.l
pub fn i_0653ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x653F2);
    cpu.pc = 0x28D70;
    16
}
/// $0653F2: bcc.w $653fc
pub fn i_0653f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x653FC; return 10; }
    8
}
/// $0653F6: lea.l $64d8a(pc), a1
pub fn i_0653f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0653FA: move.l a1, (a6)
pub fn i_0653fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x653FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0653FC: movea.l #$ffffffff, a0
pub fn i_0653fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65402;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $065402: jsr $5dd56.l
pub fn i_065402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65408);
    cpu.pc = 0x5DD56;
    16
}
/// $065408: bcc.w $65412
pub fn i_065408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6540C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65412; return 10; }
    8
}
// ===== SetTaskHandler_06540c  ($06540C, 8 B, kernel/task_handlers.c)
/// $06540C: lea.l $64d8a(pc), a1
pub fn i_06540c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65410;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065410: move.l a1, (a6)
pub fn i_065410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65412;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065412: rts
pub fn i_065412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65414;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Flash_065414  ($065414, 78 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065414: move.w #$c000, d0
pub fn i_065414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65418;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065418: jsr $28134.l
pub fn i_065418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6541E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6541E);
    cpu.pc = 0x28134;
    16
}
/// $06541E: andi.w #$ffe3, $38(a6)
pub fn i_06541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65424;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $065424: ori.w #$1c, $38(a6)
pub fn i_065424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6542A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06542A: lea.l $4acfe.l, a0
pub fn i_06542a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65430;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x4ACFE;
    20
}
/// $065430: jsr $28cd4.l
pub fn i_065430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65436);
    cpu.pc = 0x28CD4;
    16
}
/// $065436: lea.l $6543c(pc), a1
pub fn i_065436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6543A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6543C;
    16
}
/// $06543A: move.l a1, (a6)
pub fn i_06543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6543C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06543C: jsr $2783a.l
pub fn i_06543c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65442);
    cpu.pc = 0x2783A;
    16
}
/// $065442: jsr $28d70.l
pub fn i_065442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65448);
    cpu.pc = 0x28D70;
    16
}
/// $065448: bcc.w $65452
pub fn i_065448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6544C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65452; return 10; }
    8
}
/// $06544C: lea.l $64d8a(pc), a1
pub fn i_06544c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65450;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065450: move.l a1, (a6)
pub fn i_065450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65452;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065452: movea.l #$ffffffff, a0
pub fn i_065452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65458;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $065458: jsr $5dd56.l
pub fn i_065458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6545E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6545E);
    cpu.pc = 0x5DD56;
    16
}
/// $06545E: bcc.w $65468
pub fn i_06545e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65462;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65468; return 10; }
    8
}
// ===== SetTaskHandler_065462  ($065462, 8 B, kernel/task_handlers.c)
/// $065462: lea.l $64d8a(pc), a1
pub fn i_065462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65466;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065466: move.l a1, (a6)
pub fn i_065466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65468;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065468: rts
pub fn i_065468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6546A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Debris_06546a  ($06546A, 130 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06546A: move.w #$4, $80(a6)
pub fn i_06546a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65470;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065470: move.w #$4000, d0
pub fn i_065470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65474;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065474: jsr $28134.l
pub fn i_065474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6547A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6547A);
    cpu.pc = 0x28134;
    16
}
/// $06547A: andi.w #$ffe3, $38(a6)
pub fn i_06547a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65480;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $065480: ori.w #$1c, $38(a6)
pub fn i_065480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65486;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $065486: move.w #$45, d1
pub fn i_065486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6548A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x45;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06548A: jsr $236e.l
pub fn i_06548a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65490;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65490);
    cpu.pc = 0x236E;
    16
}
/// $065490: jsr $5e9b6.l
pub fn i_065490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65496);
    cpu.pc = 0x5E9B6;
    16
}
/// $065496: andi.w #$7, d0
pub fn i_065496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6549A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06549A: beq.w $65554
pub fn i_06549a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6549E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65554; return 10; }
    8
}
/// $06549E: jsr $5e9b6.l
pub fn i_06549e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x654A4);
    cpu.pc = 0x5E9B6;
    16
}
/// $0654A4: andi.w #$3, d0
pub fn i_0654a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654A8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0654A8: move.w d0, d1
pub fn i_0654a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654AA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0654AA: add.w d1, d1
pub fn i_0654aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654AC;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0654AC: add.w d0, d1
pub fn i_0654ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654AE;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0654AE: add.w d1, d1
pub fn i_0654ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654B0;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0654B0: lea.l $2c65c4.l, a0
pub fn i_0654b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C65C4;
    20
}
/// $0654B6: move.w $4(a0, d1.w), $2a(a6)
pub fn i_0654b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654BC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0654BC: move.w $2(a0, d1.w), $2e(a6)
pub fn i_0654bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654C2;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0654C2: move.w (a0, d1.w), d0
pub fn i_0654c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654C6;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0654C6: btst.b #$0, $3a(a6)
pub fn i_0654c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654CC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0654CC: bne.w $654d2
pub fn i_0654cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x654D2; return 10; }
    8
}
/// $0654D0: neg.w d0
pub fn i_0654d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654D2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0654D2: move.w d0, $28(a6)
pub fn i_0654d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654D6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0654D6: jsr $5e9b6.l
pub fn i_0654d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x654DC);
    cpu.pc = 0x5E9B6;
    16
}
/// $0654DC: andi.w #$1f, d0
pub fn i_0654dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0654E0: addi.w #$18, d0
pub fn i_0654e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654E4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0654E4: move.w d0, $70(a6)
pub fn i_0654e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654E8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0654E8: bra.w $654f8
pub fn i_0654e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x654F8;
    10
}
// ===== Patrol_DebrisFall_0654ec  ($0654EC, 54 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0654EC: move.w $2a(a6), d0
pub fn i_0654ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654F0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0654F0: asr.w #$1, d0
pub fn i_0654f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654F2;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0654F2: neg.w d0
pub fn i_0654f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654F4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0654F4: move.w d0, $2a(a6)
pub fn i_0654f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654F8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0654F8: lea.l $2c6dd0.l, a0
pub fn i_0654f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6DD0;
    20
}
/// $0654FE: jsr $28cd4.l
pub fn i_0654fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65504);
    cpu.pc = 0x28CD4;
    16
}
/// $065504: lea.l $6550a(pc), a1
pub fn i_065504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65508;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6550A;
    16
}
/// $065508: move.l a1, (a6)
pub fn i_065508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6550A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06550A: jsr $27d50.l
pub fn i_06550a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65510);
    cpu.pc = 0x27D50;
    16
}
/// $065510: bcc.w $6551a
pub fn i_065510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65514;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6551A; return 10; }
    8
}
/// $065514: lea.l $65522(pc), a1
pub fn i_065514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65518;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65522;
    16
}
/// $065518: move.l a1, (a6)
pub fn i_065518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6551A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06551A: jsr $65db4(pc)
pub fn i_06551a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6551E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6551E);
    cpu.pc = 0x65DB4;
    16
}
/// $06551E: bra.w $655bc
pub fn i_06551e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65522;
    cpu.ir = 0x6000;
    cpu.pc = 0x655BC;
    10
}
// ===== Patrol_DebrisBounce_065522  ($065522, 202 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065522: subq.w #$1, $80(a6)
pub fn i_065522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65526;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065526: blt.w $65554
pub fn i_065526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6552A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x65554; return 10; }
    8
}
/// $06552A: lea.l $2c6dea.l, a0
pub fn i_06552a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65530;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6DEA;
    20
}
/// $065530: jsr $28cd4.l
pub fn i_065530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65536);
    cpu.pc = 0x28CD4;
    16
}
/// $065536: lea.l $6553c(pc), a1
pub fn i_065536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6553A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6553C;
    16
}
/// $06553A: move.l a1, (a6)
pub fn i_06553a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6553C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06553C: jsr $2783a.l
pub fn i_06553c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65542);
    cpu.pc = 0x2783A;
    16
}
/// $065542: jsr $65db4(pc)
pub fn i_065542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65546;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65546);
    cpu.pc = 0x65DB4;
    16
}
/// $065546: bcc.w $65550
pub fn i_065546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6554A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65550; return 10; }
    8
}
/// $06554A: lea.l $654ec(pc), a1
pub fn i_06554a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6554E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x654EC;
    16
}
/// $06554E: move.l a1, (a6)
pub fn i_06554e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65550;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065550: bra.w $655bc
pub fn i_065550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65554;
    cpu.ir = 0x6000;
    cpu.pc = 0x655BC;
    10
}
/// $065554: move.w $28(a6), d0
pub fn i_065554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65558;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065558: movem.w d0, -(a7)
pub fn i_065558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6555C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06555C: jsr $267e2.l
pub fn i_06555c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65562;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65562);
    cpu.pc = 0x267E2;
    16
}
/// $065562: movem.w (a7)+, d0
pub fn i_065562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65566;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $065566: cmpi.w #$200, d0
pub fn i_065566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6556A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Word, false);
    8
}
/// $06556A: bge.w $65588
pub fn i_06556a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6556E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x65588; return 10; }
    8
}
/// $06556E: cmpi.w #$fe00, d0
pub fn i_06556e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65572;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFE00, Size::Word, false);
    8
}
/// $065572: ble.w $65588
pub fn i_065572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65576;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x65588; return 10; }
    8
}
/// $065576: move.w #$200, d1
pub fn i_065576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6557A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06557A: cmpi.w #$0, d0
pub fn i_06557a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6557E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06557E: bge.w $65586
pub fn i_06557e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65582;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x65586; return 10; }
    8
}
/// $065582: move.w #$fe00, d1
pub fn i_065582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65586;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065586: move.w d1, d0
pub fn i_065586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65588;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065588: move.w d0, $28(a6)
pub fn i_065588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6558C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06558C: lea.l $2c6dd0.l, a0
pub fn i_06558c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65592;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6DD0;
    20
}
/// $065592: jsr $28cd4.l
pub fn i_065592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65598;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65598);
    cpu.pc = 0x28CD4;
    16
}
/// $065598: lea.l $6559e(pc), a1
pub fn i_065598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6559C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6559E;
    16
}
/// $06559C: move.l a1, (a6)
pub fn i_06559c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6559E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06559E: jsr $27a92.l
pub fn i_06559e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x655A4);
    cpu.pc = 0x27A92;
    16
}
/// $0655A4: jsr $5e9b6.l
pub fn i_0655a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x655AA);
    cpu.pc = 0x5E9B6;
    16
}
/// $0655AA: andi.w #$f, d0
pub fn i_0655aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655AE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0655AE: bne.w $655b8
pub fn i_0655ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x655B8; return 10; }
    8
}
/// $0655B2: lea.l $655f4(pc), a1
pub fn i_0655b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x655F4;
    16
}
/// $0655B6: move.l a1, (a6)
pub fn i_0655b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0655B8: jsr $65db4(pc)
pub fn i_0655b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x655BC);
    cpu.pc = 0x65DB4;
    16
}
/// $0655BC: cmpi.w #$0, $70(a6)
pub fn i_0655bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655C2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0655C2: bgt.w $655cc
pub fn i_0655c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655C6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x655CC; return 10; }
    8
}
/// $0655C6: lea.l $64d8a(pc), a1
pub fn i_0655c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0655CA: move.l a1, (a6)
pub fn i_0655ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0655CC: cmpi.w #$ffd0, $22(a6)
pub fn i_0655cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFD0, Size::Word, false);
    16
}
/// $0655D2: bgt.w $655dc
pub fn i_0655d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655D6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x655DC; return 10; }
    8
}
/// $0655D6: lea.l $64d8a(pc), a1
pub fn i_0655d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0655DA: move.l a1, (a6)
pub fn i_0655da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0655DC: movea.l #$ffffffff, a0
pub fn i_0655dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655E2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0655E2: jsr $5dd56.l
pub fn i_0655e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x655E8);
    cpu.pc = 0x5DD56;
    16
}
/// $0655E8: bcc.w $655f2
pub fn i_0655e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x655F2; return 10; }
    8
}
// ===== SetTaskHandler_0655ec  ($0655EC, 8 B, kernel/task_handlers.c)
/// $0655EC: lea.l $64d8a(pc), a1
pub fn i_0655ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0655F0: move.l a1, (a6)
pub fn i_0655f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0655F2: rts
pub fn i_0655f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Stub_0655f4  ($0655F4, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0655F4: move.w #$2a9, $2a(a6)
pub fn i_0655f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x655FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0655FA: move.w #$ff1d, $2e(a6)
pub fn i_0655fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65600;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065600: bra.w $654f8
pub fn i_065600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65604;
    cpu.ir = 0x6000;
    cpu.pc = 0x654F8;
    10
}
// ===== Patrol_Register_065604  ($065604, 92 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065604: lea.l $2c7366.l, a0
pub fn i_065604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6560A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7366;
    20
}
/// $06560A: jsr $28cd4.l
pub fn i_06560a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65610;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65610);
    cpu.pc = 0x28CD4;
    16
}
/// $065610: bra.w $65640
pub fn i_065610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65614;
    cpu.ir = 0x6000;
    cpu.pc = 0x65640;
    10
}
/// $065640: move.w #$4, d1
pub fn i_065640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65644;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065644: jsr $236e.l
pub fn i_065644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6564A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6564A);
    cpu.pc = 0x236E;
    16
}
/// $06564A: lea.l $65650(pc), a1
pub fn i_06564a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6564E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65650;
    16
}
/// $06564E: move.l a1, (a6)
pub fn i_06564e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65650;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065650: jsr $2783a.l
pub fn i_065650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65656);
    cpu.pc = 0x2783A;
    16
}
/// $065656: jsr $28d70.l
pub fn i_065656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6565C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6565C);
    cpu.pc = 0x28D70;
    16
}
/// $06565C: bcc.w $65666
pub fn i_06565c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65660;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65666; return 10; }
    8
}
// ===== SetTaskHandler_065660  ($065660, 8 B, kernel/task_handlers.c)
/// $065660: lea.l $64d8a(pc), a1
pub fn i_065660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65664;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065664: move.l a1, (a6)
pub fn i_065664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65666;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065666: rts
pub fn i_065666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65668;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Prop_Init_065668  ($065668, 252 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065668: clr.w $84(a6)
pub fn i_065668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6566C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06566C: move.w #$7fff, $8a(a0)
pub fn i_06566c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65672;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065672: bra.w $6568c
pub fn i_065672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65676;
    cpu.ir = 0x6000;
    cpu.pc = 0x6568C;
    10
}
/// $065676: move.w #$46, d1
pub fn i_065676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6567A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06567A: jsr $236e.l
pub fn i_06567a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65680);
    cpu.pc = 0x236E;
    16
}
/// $065680: lea.l $2c6e14.l, a0
pub fn i_065680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65686;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6E14;
    20
}
/// $065686: jsr $28cd4.l
pub fn i_065686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6568C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6568C);
    cpu.pc = 0x28CD4;
    16
}
/// $06568C: bset.b #$4, $6b(a6)
pub fn i_06568c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65692;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $065692: move.w #$10a4, d0
pub fn i_065692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65696;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065696: jsr $2352.l
pub fn i_065696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6569C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6569C);
    cpu.pc = 0x2352;
    16
}
/// $06569C: clr.w $70(a6)
pub fn i_06569c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656A0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0656A0: clr.w $86(a6)
pub fn i_0656a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656A4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0656A4: clr.b $3a(a6)
pub fn i_0656a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656A8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0656A8: move.w #$5, $66(a6)
pub fn i_0656a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0656AE: move.w #$200, $36(a6)
pub fn i_0656ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656B4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0656B4: andi.w #$1f, $88(a6)
pub fn i_0656b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656BA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0656BA: move.w #$d000, d0
pub fn i_0656ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656BE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0656BE: jsr $28134.l
pub fn i_0656be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x656C4);
    cpu.pc = 0x28134;
    16
}
/// $0656C4: andi.w #$ffe3, $38(a6)
pub fn i_0656c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656CA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0656CA: ori.w #$1c, $38(a6)
pub fn i_0656ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656D0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0656D0: lea.l $656d6(pc), a1
pub fn i_0656d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x656D6;
    16
}
/// $0656D4: move.l a1, (a6)
pub fn i_0656d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0656D6: subq.w #$1, $8a(a6)
pub fn i_0656d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656DA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0656DA: cmpi.w #$0, $8a(a6)
pub fn i_0656da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656E0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0656E0: ble.w $656f4
pub fn i_0656e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656E4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x656F4; return 10; }
    8
}
/// $0656E4: jsr $6576c(pc)
pub fn i_0656e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x656E8);
    cpu.pc = 0x6576C;
    16
}
/// $0656E8: jsr $6580e(pc)
pub fn i_0656e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x656EC);
    cpu.pc = 0x6580E;
    16
}
/// $0656EC: jsr $65888(pc)
pub fn i_0656ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x656F0);
    cpu.pc = 0x65888;
    16
}
/// $0656F0: jsr $65c64(pc)
pub fn i_0656f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x656F4);
    cpu.pc = 0x65C64;
    16
}
/// $0656F4: jsr $27cee.l
pub fn i_0656f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x656FA);
    cpu.pc = 0x27CEE;
    16
}
/// $0656FA: bcc.w $65704
pub fn i_0656fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65704; return 10; }
    8
}
/// $0656FE: lea.l $65166(pc), a1
pub fn i_0656fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65702;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65166;
    16
}
/// $065702: move.l a1, (a6)
pub fn i_065702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65704;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065704: cmpi.w #$20, $88(a6)
pub fn i_065704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6570A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $06570A: bcs.w $6571c
pub fn i_06570a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6570E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6571C; return 10; }
    8
}
/// $06570E: nop
pub fn i_06570e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65710;
    cpu.ir = 0x4E71;
    4
}
/// $065710: nop
pub fn i_065710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65712;
    cpu.ir = 0x4E71;
    4
}
/// $065712: cmpi.w #$20, $88(a6)
pub fn i_065712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65718;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $065718: nop
pub fn i_065718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6571A;
    cpu.ir = 0x4E71;
    4
}
/// $06571A: trap #$f
pub fn i_06571a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6571C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06571C: move.w $88(a6), $34(a6)
pub fn i_06571c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65722;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $065722: jsr $28d70.l
pub fn i_065722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65728);
    cpu.pc = 0x28D70;
    16
}
/// $065728: jsr $283d8.l
pub fn i_065728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6572E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6572E);
    cpu.pc = 0x283D8;
    16
}
/// $06572E: btst.b #$1, $13(a6)
pub fn i_06572e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65734;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065734: beq.w $6573e
pub fn i_065734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65738;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6573E; return 10; }
    8
}
/// $065738: lea.l $65194(pc), a1
pub fn i_065738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6573C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65194;
    16
}
/// $06573C: move.l a1, (a6)
pub fn i_06573c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6573E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06573E: jsr $28758.l
pub fn i_06573e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65744);
    cpu.pc = 0x28758;
    16
}
/// $065744: bcc.w $6574e
pub fn i_065744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65748;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6574E; return 10; }
    8
}
/// $065748: lea.l $65166(pc), a1
pub fn i_065748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6574C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65166;
    16
}
/// $06574C: move.l a1, (a6)
pub fn i_06574c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6574E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06574E: bclr.b #$3, $13(a6)
pub fn i_06574e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65754;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $065754: movea.l #$ffffffff, a0
pub fn i_065754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6575A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06575A: jsr $5dd56.l
pub fn i_06575a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65760);
    cpu.pc = 0x5DD56;
    16
}
/// $065760: bcc.w $6576a
pub fn i_065760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65764;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6576A; return 10; }
    8
}
// ===== SetTaskHandler_065764  ($065764, 8 B, kernel/task_handlers.c)
/// $065764: lea.l $64d8a(pc), a1
pub fn i_065764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065768: move.l a1, (a6)
pub fn i_065768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6576A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06576A: rts
pub fn i_06576a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6576C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Prop_Move_06576c  ($06576C, 162 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06576C: subq.w #$1, $70(a6)
pub fn i_06576c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65770;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065770: cmpi.w #$0, $70(a6)
pub fn i_065770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65776;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $065776: bgt.w $6580c
pub fn i_065776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6577A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6580C; return 10; }
    8
}
/// $06577A: jsr $5e9b6.l
pub fn i_06577a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65780);
    cpu.pc = 0x5E9B6;
    16
}
/// $065780: andi.w #$1f, d0
pub fn i_065780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65784;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065784: addi.w #$30, d0
pub fn i_065784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65788;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065788: move.w d0, $70(a6)
pub fn i_065788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6578C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06578C: addq.w #$1, $86(a6)
pub fn i_06578c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65790;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065790: cmpi.w #$1, $86(a6)
pub fn i_065790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65796;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $065796: bgt.w $657ce
pub fn i_065796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6579A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x657CE; return 10; }
    8
}
/// $06579A: jsr $5e9b6.l
pub fn i_06579a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x657A0);
    cpu.pc = 0x5E9B6;
    16
}
/// $0657A0: andi.w #$f, d0
pub fn i_0657a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657A4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0657A4: lsl.w #$2, d0
pub fn i_0657a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657A6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0657A6: lea.l $2c65dc.l, a0
pub fn i_0657a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C65DC;
    20
}
/// $0657AC: move.w (a0, d0.w), $80(a6)
pub fn i_0657ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657B2;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0657B2: move.w $2(a0, d0.w), $82(a6)
pub fn i_0657b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657B8;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0657B8: jsr $5e9b6.l
pub fn i_0657b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x657BE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0657BE: andi.w #$1f0, d0
pub fn i_0657be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657C2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0657C2: addi.w #$100, d0
pub fn i_0657c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657C6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0657C6: move.w d0, $36(a6)
pub fn i_0657c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657CA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0657CA: bra.w $657f0
pub fn i_0657ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x657F0;
    10
}
/// $0657CE: jsr $5e0d4.l
pub fn i_0657ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x657D4);
    cpu.pc = 0x5E0D4;
    16
}
/// $0657D4: move.w $22(a0), $80(a6)
pub fn i_0657d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657DA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0657DA: move.w $24(a0), $82(a6)
pub fn i_0657da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657E0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0657E0: move.w #$300, $36(a6)
pub fn i_0657e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0657E6: cmpi.w #$3, $86(a6)
pub fn i_0657e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $0657EC: bgt.w $6580c
pub fn i_0657ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657F0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6580C; return 10; }
    8
}
/// $0657F0: jsr $5e9b6.l
pub fn i_0657f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x657F6);
    cpu.pc = 0x5E9B6;
    16
}
/// $0657F6: andi.w #$1f, d0
pub fn i_0657f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657FA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0657FA: add.w d0, $80(a6)
pub fn i_0657fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x657FE;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0657FE: jsr $5e9b6.l
pub fn i_0657fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65804);
    cpu.pc = 0x5E9B6;
    16
}
/// $065804: andi.w #$1f, d0
pub fn i_065804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65808;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065808: add.w d0, $82(a6)
pub fn i_065808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6580C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06580C: rts
pub fn i_06580c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6580E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Prop_Aim_06580e  ($06580E, 82 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06580E: move.b $106f28.l, d0
pub fn i_06580e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65814;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $065814: andi.b #$3, d0
pub fn i_065814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65818;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $065818: bne.w $65870
pub fn i_065818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6581C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65870; return 10; }
    8
}
/// $06581C: move.w $80(a6), d0
pub fn i_06581c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65820;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065820: sub.w $22(a6), d0
pub fn i_065820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65824;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $065824: move.w $82(a6), d1
pub fn i_065824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65828;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065828: sub.w $24(a6), d1
pub fn i_065828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6582C;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06582C: jsr $5e018.l
pub fn i_06582c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65832);
    cpu.pc = 0x5E018;
    16
}
/// $065832: lsr.w #$3, d0
pub fn i_065832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65834;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $065834: move.w #$1, d1
pub fn i_065834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65838;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065838: cmp.w $88(a6), d0
pub fn i_065838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6583C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06583C: beq.w $65866
pub fn i_06583c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65840;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65866; return 10; }
    8
}
/// $065840: bgt.w $65848
pub fn i_065840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65844;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x65848; return 10; }
    8
}
/// $065844: move.w #$ffff, d1
pub fn i_065844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65848;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065848: cmpi.w #$0, $84(a6)
pub fn i_065848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6584E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06584E: beq.w $65856
pub fn i_06584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65852;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65856; return 10; }
    8
}
/// $065852: move.w $84(a6), d1
pub fn i_065852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65856;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065856: add.w d1, $88(a6)
pub fn i_065856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6585A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06585A: andi.w #$1f, $88(a6)
pub fn i_06585a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65860;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_065860  ($065860, 6 B, thunks/ccr_helpers.c)
/// $065860: ori.b #$11, ccr
pub fn i_065860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65864;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $065864: rts
pub fn i_065864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65866;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Prop_Rng_065866  ($065866, 28 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065866: move.w $70(a6), d0
pub fn i_065866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6586A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06586A: asr.w #$1, d0
pub fn i_06586a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6586C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06586C: move.w d0, $70(a6)
pub fn i_06586c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65870;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065870: jsr $5e9b6.l
pub fn i_065870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65876);
    cpu.pc = 0x5E9B6;
    16
}
/// $065876: andi.w #$1f, d0
pub fn i_065876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6587A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06587A: bne.w $65882
pub fn i_06587a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6587E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65882; return 10; }
    8
}
/// $06587E: neg.w $84(a6)
pub fn i_06587e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65882;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_065882  ($065882, 6 B, thunks/ccr_helpers.c)
/// $065882: andi.b #$ee, ccr
pub fn i_065882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65886;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $065886: rts
pub fn i_065886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65888;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Prop_Clamp_065888  ($065888, 54 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065888: move.w $22(a6), d0
pub fn i_065888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6588C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06588C: sub.w $80(a6), d0
pub fn i_06588c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65890;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $065890: move.w $24(a6), d1
pub fn i_065890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65894;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065894: sub.w $82(a6), d1
pub fn i_065894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65898;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $065898: cmpi.w #$fff0, d0
pub fn i_065898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6589C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF0, Size::Word, false);
    8
}
/// $06589C: blt.w $658bc
pub fn i_06589c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658A0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x658BC; return 10; }
    8
}
/// $0658A0: cmpi.w #$10, d0
pub fn i_0658a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658A4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10, Size::Word, false);
    8
}
/// $0658A4: bgt.w $658bc
pub fn i_0658a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658A8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x658BC; return 10; }
    8
}
/// $0658A8: cmpi.w #$fff0, d1
pub fn i_0658a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658AC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF0, Size::Word, false);
    8
}
/// $0658AC: blt.w $658bc
pub fn i_0658ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658B0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x658BC; return 10; }
    8
}
/// $0658B0: cmpi.w #$10, d1
pub fn i_0658b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658B4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10, Size::Word, false);
    8
}
/// $0658B4: bgt.w $658bc
pub fn i_0658b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658B8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x658BC; return 10; }
    8
}
/// $0658B8: clr.w $70(a6)
pub fn i_0658b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658BC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0658BC: rts
pub fn i_0658bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Prop_Sound_0658be  ($0658BE, 118 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0658BE: move.w #$12, d0
pub fn i_0658be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0658C2: move.w #$1e, d1
pub fn i_0658c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0658C6: bra.w $658ea
pub fn i_0658c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x658EA;
    10
}
/// $0658EA: btst.b #$0, $3a(a6)
pub fn i_0658ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658F0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0658F0: beq.w $658f6
pub fn i_0658f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x658F6; return 10; }
    8
}
/// $0658F4: move.w d1, d0
pub fn i_0658f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658F6;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0658F6: move.w d0, $88(a6)
pub fn i_0658f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658FA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0658FA: movea.l $c(a6), a0
pub fn i_0658fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x658FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0658FE: move.w $84(a0), d0
pub fn i_0658fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65902;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065902: andi.w #$3, d0
pub fn i_065902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65906;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065906: add.w d0, d0
pub fn i_065906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65908;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065908: lea.l $2c65a4.l, a0
pub fn i_065908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6590E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C65A4;
    20
}
/// $06590E: move.w (a0, d0.w), $84(a6)
pub fn i_06590e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65914;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad4, v3 as u16);
    22
}
/// $065914: movea.l $c(a6), a0
pub fn i_065914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65918;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $065918: movea.l $c(a0), a0
pub fn i_065918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6591C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06591C: move.b $9b(a0), d0
pub fn i_06591c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65920;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $065920: andi.w #$ff, d0
pub fn i_065920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65924;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065924: bne.w $6592c
pub fn i_065924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65928;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6592C; return 10; }
    8
}
/// $065928: move.w #$7fff, d0
pub fn i_065928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6592C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06592C: move.w d0, $8a(a6)
pub fn i_06592c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65930;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065930: bra.w $65676
pub fn i_065930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65934;
    cpu.ir = 0x6000;
    cpu.pc = 0x65676;
    10
}
// ===== Scene3Prop_Spark_065934  ($065934, 48 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065934: move.w #$4, d1
pub fn i_065934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65938;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065938: jsr $236e.l
pub fn i_065938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6593E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6593E);
    cpu.pc = 0x236E;
    16
}
/// $06593E: lea.l $2c74ae.l, a0
pub fn i_06593e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65944;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C74AE;
    20
}
/// $065944: jsr $28cd4.l
pub fn i_065944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6594A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6594A);
    cpu.pc = 0x28CD4;
    16
}
/// $06594A: move.w #$4000, d0
pub fn i_06594a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6594E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06594E: jsr $28134.l
pub fn i_06594e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65954);
    cpu.pc = 0x28134;
    16
}
/// $065954: andi.w #$ffe3, $38(a6)
pub fn i_065954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6595A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06595A: ori.w #$1c, $38(a6)
pub fn i_06595a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65960;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $065960: bra.w $6564a
pub fn i_065960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65964;
    cpu.ir = 0x6000;
    cpu.pc = 0x6564A;
    10
}
// ===== Scene3Prop_Shell_065964  ($065964, 312 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065964: bset.b #$4, $6b(a6)
pub fn i_065964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6596A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06596A: move.w #$15d, d1
pub fn i_06596a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6596E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06596E: jsr $236e.l
pub fn i_06596e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65974);
    cpu.pc = 0x236E;
    16
}
/// $065974: move.w #$164, d1
pub fn i_065974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65978;
    cpu.ir = 0x323C;
    let v1: u32 = 0x164;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065978: jsr $236e.l
pub fn i_065978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6597E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6597E);
    cpu.pc = 0x236E;
    16
}
/// $06597E: move.w #$165, d1
pub fn i_06597e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65982;
    cpu.ir = 0x323C;
    let v1: u32 = 0x165;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065982: jsr $236e.l
pub fn i_065982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65988;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65988);
    cpu.pc = 0x236E;
    16
}
/// $065988: clr.b $7a(a6)
pub fn i_065988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6598C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06598C: move.w #$8, $66(a6)
pub fn i_06598c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65992;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065992: cmpi.w #$178, $24(a6)
pub fn i_065992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65998;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x178, Size::Word, false);
    16
}
/// $065998: blt.w $659c0
pub fn i_065998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6599C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x659C0; return 10; }
    8
}
/// $06599C: move.w #$111, d0
pub fn i_06599c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x111;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0659A0: jsr $5dca4.l
pub fn i_0659a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x659A6);
    cpu.pc = 0x5DCA4;
    16
}
/// $0659A6: move.w d0, $28(a6)
pub fn i_0659a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659AA: move.w #$21c, $2a(a6)
pub fn i_0659aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x21C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659B0: move.w #$ffdc, $2e(a6)
pub fn i_0659b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFDC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659B6: move.w #$0, $2c(a6)
pub fn i_0659b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659BC: bra.w $659e0
pub fn i_0659bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x659E0;
    10
}
/// $0659C0: move.w #$5b, d0
pub fn i_0659c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659C4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0659C4: jsr $5dca4.l
pub fn i_0659c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x659CA);
    cpu.pc = 0x5DCA4;
    16
}
/// $0659CA: move.w d0, $28(a6)
pub fn i_0659ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659CE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659CE: move.w #$5a0, $2a(a6)
pub fn i_0659ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659D4: move.w #$ffe0, $2e(a6)
pub fn i_0659d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659DA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659DA: move.w #$0, $2c(a6)
pub fn i_0659da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0659E0: move.w #$d000, d0
pub fn i_0659e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0659E4: jsr $28134.l
pub fn i_0659e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x659EA);
    cpu.pc = 0x28134;
    16
}
/// $0659EA: andi.w #$ffe3, $38(a6)
pub fn i_0659ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659F0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0659F0: ori.w #$0, $38(a6)
pub fn i_0659f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659F6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0659F6: lea.l $2c72b4.l, a0
pub fn i_0659f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x659FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C72B4;
    20
}
/// $0659FC: jsr $28cd4.l
pub fn i_0659fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A02);
    cpu.pc = 0x28CD4;
    16
}
/// $065A02: lea.l $65a08(pc), a1
pub fn i_065a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65A08;
    16
}
/// $065A06: move.l a1, (a6)
pub fn i_065a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065A08: btst.b #$0, $106f28.l
pub fn i_065a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A10;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $065A10: bne.w $65a38
pub fn i_065a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65A38; return 10; }
    8
}
/// $065A14: move.b $7a(a6), d0
pub fn i_065a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A18;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $065A18: addq.b #$1, $7a(a6)
pub fn i_065a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A1C;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $065A1C: andi.w #$3, d0
pub fn i_065a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A20;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065A20: add.w d0, d0
pub fn i_065a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A22;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065A22: lea.l $2c65bc.l, a0
pub fn i_065a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C65BC;
    20
}
/// $065A28: move.w (a0, d0.w), d0
pub fn i_065a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A2C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $065A2C: andi.w #$3, d0
pub fn i_065a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A30;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065A30: add.w d0, d0
pub fn i_065a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A32;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065A32: move.w $16(a6, d0.w), $14(a6)
pub fn i_065a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A38;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $065A38: jsr $27d50.l
pub fn i_065a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A3E);
    cpu.pc = 0x27D50;
    16
}
/// $065A3E: bcc.w $65a48
pub fn i_065a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65A48; return 10; }
    8
}
/// $065A42: lea.l $65174(pc), a1
pub fn i_065a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65174;
    16
}
/// $065A46: move.l a1, (a6)
pub fn i_065a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065A48: jsr $28d70.l
pub fn i_065a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A4E);
    cpu.pc = 0x28D70;
    16
}
/// $065A4E: jsr $283d8.l
pub fn i_065a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A54);
    cpu.pc = 0x283D8;
    16
}
/// $065A54: btst.b #$1, $13(a6)
pub fn i_065a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A5A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065A5A: beq.w $65a64
pub fn i_065a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65A64; return 10; }
    8
}
/// $065A5E: lea.l $65194(pc), a1
pub fn i_065a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65194;
    16
}
/// $065A62: move.l a1, (a6)
pub fn i_065a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065A64: jsr $2870a.l
pub fn i_065a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A6A);
    cpu.pc = 0x2870A;
    16
}
/// $065A6A: bclr.b #$3, $13(a6)
pub fn i_065a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $065A70: jsr $28758.l
pub fn i_065a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A76);
    cpu.pc = 0x28758;
    16
}
/// $065A76: bcc.w $65a80
pub fn i_065a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65A80; return 10; }
    8
}
/// $065A7A: lea.l $6517e(pc), a1
pub fn i_065a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6517E;
    16
}
/// $065A7E: move.l a1, (a6)
pub fn i_065a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065A80: lea.l $5e766.l, a0
pub fn i_065a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $065A86: jsr $5e770.l
pub fn i_065a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A8C);
    cpu.pc = 0x5E770;
    16
}
/// $065A8C: movea.l #$ffffffff, a0
pub fn i_065a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A92;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $065A92: jsr $5dd56.l
pub fn i_065a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65A98);
    cpu.pc = 0x5DD56;
    16
}
/// $065A98: bcc.w $65aa2
pub fn i_065a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65A9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65AA2; return 10; }
    8
}
// ===== SetTaskHandler_065a9c  ($065A9C, 8 B, kernel/task_handlers.c)
/// $065A9C: lea.l $64d8a(pc), a1
pub fn i_065a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065AA0: move.l a1, (a6)
pub fn i_065aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065AA2: rts
pub fn i_065aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Prop_ShellB_065aa4  ($065AA4, 72 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065AA4: move.w #$4, d1
pub fn i_065aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AA8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065AA8: jsr $236e.l
pub fn i_065aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65AAE);
    cpu.pc = 0x236E;
    16
}
/// $065AAE: move.w #$20, $2a(a6)
pub fn i_065aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065AB4: lea.l $2dd80c.l, a0
pub fn i_065ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65ABA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD80C;
    20
}
/// $065ABA: jsr $28cd4.l
pub fn i_065aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65AC0);
    cpu.pc = 0x28CD4;
    16
}
/// $065AC0: lea.l $65ac6(pc), a1
pub fn i_065ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65AC6;
    16
}
/// $065AC4: move.l a1, (a6)
pub fn i_065ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065AC6: jsr $27cee.l
pub fn i_065ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65ACC);
    cpu.pc = 0x27CEE;
    16
}
/// $065ACC: jsr $28d70.l
pub fn i_065acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65AD2);
    cpu.pc = 0x28D70;
    16
}
/// $065AD2: bcc.w $65adc
pub fn i_065ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65ADC; return 10; }
    8
}
/// $065AD6: lea.l $64d8a(pc), a1
pub fn i_065ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65ADA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065ADA: move.l a1, (a6)
pub fn i_065ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65ADC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065ADC: movea.l #$ffffffff, a0
pub fn i_065adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AE2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $065AE2: jsr $5dd56.l
pub fn i_065ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65AE8);
    cpu.pc = 0x5DD56;
    16
}
/// $065AE8: bcc.w $65af2
pub fn i_065ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65AF2; return 10; }
    8
}
// ===== SetTaskHandler_065aec  ($065AEC, 8 B, kernel/task_handlers.c)
/// $065AEC: lea.l $64d8a(pc), a1
pub fn i_065aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065AF0: move.l a1, (a6)
pub fn i_065af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065AF2: rts
pub fn i_065af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_PlayerNear_065af4  ($065AF4, 54 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065AF4: cmpi.w #$20, $22(a6)
pub fn i_065af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AFA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $065AFA: blt.w $65b30
pub fn i_065afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65AFE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x65B30; return 10; }
    8
}
/// $065AFE: cmpi.w #$120, $22(a6)
pub fn i_065afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B04;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $065B04: bgt.w $65b30
pub fn i_065b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B08;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x65B30; return 10; }
    8
}
/// $065B08: subq.w #$1, $72(a6)
pub fn i_065b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B0C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065B0C: bcc.w $65b16
pub fn i_065b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65B16; return 10; }
    8
}
/// $065B10: move.w #$0, $72(a6)
pub fn i_065b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B16;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065B16: cmpi.w #$0, $8e(a6)
pub fn i_065b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B1C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $065B1C: beq.w $65b30
pub fn i_065b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B20;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65B30; return 10; }
    8
}
/// $065B20: cmpi.w #$0, $72(a6)
pub fn i_065b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B26;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $065B26: bgt.w $65b30
pub fn i_065b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B2A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x65B30; return 10; }
    8
}
// ===== SetXN_065b2a  ($065B2A, 6 B, thunks/ccr_helpers.c)
/// $065B2A: ori.b #$11, ccr
pub fn i_065b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B2E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $065B2E: rts
pub fn i_065b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_065b30  ($065B30, 6 B, thunks/ccr_helpers.c)
/// $065B30: andi.b #$ee, ccr
pub fn i_065b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B34;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $065B34: rts
pub fn i_065b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_AimAtPlayer_065b36  ($065B36, 104 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065B36: jsr $5e136.l
pub fn i_065b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65B3C);
    cpu.pc = 0x5E136;
    16
}
/// $065B3C: bcs.w $65b98
pub fn i_065b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B40;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x65B98; return 10; }
    8
}
/// $065B40: cmpi.w #$80, d0
pub fn i_065b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B44;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $065B44: bge.w $65b50
pub fn i_065b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B48;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x65B50; return 10; }
    8
}
/// $065B48: move.w #$100, d1
pub fn i_065b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065B4C: sub.w d0, d1
pub fn i_065b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B4E;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065B4E: move.w d1, d0
pub fn i_065b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B50;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065B50: subi.w #$80, d0
pub fn i_065b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B54;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065B54: lsr.w #$3, d0
pub fn i_065b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B56;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $065B56: lea.l $2c6560.l, a0
pub fn i_065b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B5C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6560;
    20
}
/// $065B5C: btst.b #$0, $3a(a6)
pub fn i_065b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B62;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065B62: beq.w $65b6c
pub fn i_065b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65B6C; return 10; }
    8
}
/// $065B66: lea.l $2c6582.l, a0
pub fn i_065b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6582;
    20
}
/// $065B6C: add.w d0, d0
pub fn i_065b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B6E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065B6E: move.w (a0, d0.w), d1
pub fn i_065b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B72;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $065B72: move.w #$ffff, d0
pub fn i_065b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B76;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065B76: cmp.w $88(a6), d1
pub fn i_065b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B7A;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $065B7A: beq.w $65ba4
pub fn i_065b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65BA4; return 10; }
    8
}
/// $065B7E: blt.w $65b86
pub fn i_065b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B82;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x65B86; return 10; }
    8
}
/// $065B82: move.w #$1, d0
pub fn i_065b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B86;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065B86: move.b $106f28.l, d1
pub fn i_065b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B8C;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $065B8C: andi.w #$1, d1
pub fn i_065b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B90;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065B90: bne.w $65b98
pub fn i_065b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65B98; return 10; }
    8
}
/// $065B94: add.w d0, $88(a6)
pub fn i_065b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B98;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065B98: andi.w #$7, $88(a6)
pub fn i_065b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65B9E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== ClearXN_065b9e  ($065B9E, 6 B, thunks/ccr_helpers.c)
/// $065B9E: andi.b #$ee, ccr
pub fn i_065b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BA2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $065BA2: rts
pub fn i_065ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rts_065ba4  ($065BA4, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065BA4: andi.w #$7, $88(a6)
pub fn i_065ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BAA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_065baa  ($065BAA, 6 B, thunks/ccr_helpers.c)
/// $065BAA: ori.b #$11, ccr
pub fn i_065baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BAE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $065BAE: rts
pub fn i_065bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_FollowParent_065bb0  ($065BB0, 82 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065BB0: jsr $5e506.l
pub fn i_065bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65BB6);
    cpu.pc = 0x5E506;
    16
}
/// $065BB6: move.b $21(a6), $21(a0)
pub fn i_065bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BBC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
/// $065BBC: move.w $8e(a0), $8e(a6)
pub fn i_065bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BC2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $065BC2: move.b $20(a0), d0
pub fn i_065bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BC6;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $065BC6: andi.w #$f, d0
pub fn i_065bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BCA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065BCA: cmpi.w #$f, d0
pub fn i_065bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BCE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF, Size::Word, false);
    8
}
/// $065BCE: beq.w $65c00
pub fn i_065bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65C00; return 10; }
    8
}
/// $065BD2: move.b #$f, $20(a0)
pub fn i_065bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BD8;
    cpu.ir = 0x117C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $065BD8: movem.l d0, -(a7)
pub fn i_065bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BDC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $065BDC: move.b d0, $20(a6)
pub fn i_065bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BE0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $065BE0: movea.l #$2c6510, a0
pub fn i_065be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BE6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C6510;
    cpu.a[0] = v1;
    4
}
/// $065BE6: lsl.w #$2, d0
pub fn i_065be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BE8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $065BE8: movea.l (a0, d0.w), a0
pub fn i_065be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BEC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $065BEC: cmpa.l #$ffffffff, a0
pub fn i_065bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BF2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $065BF2: beq.w $65bfc
pub fn i_065bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65BFC; return 10; }
    8
}
/// $065BF6: jsr $28cd4.l
pub fn i_065bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65BFC);
    cpu.pc = 0x28CD4;
    16
}
/// $065BFC: movem.l (a7)+, d0
pub fn i_065bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C00;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $065C00: rts
pub fn i_065c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_StepAnim_065c02  ($065C02, 34 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065C02: move.w $80(a6), d0
pub fn i_065c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C06;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065C06: btst.b #$0, $3a(a6)
pub fn i_065c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C0C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065C0C: beq.w $65c1c
pub fn i_065c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C10;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65C1C; return 10; }
    8
}
/// $065C10: cmp.w $22(a6), d0
pub fn i_065c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C14;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $065C14: bgt.w $65c2a
pub fn i_065c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C18;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x65C2A; return 10; }
    8
}
/// $065C18: bra.w $65c24
pub fn i_065c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x65C24;
    10
}
/// $065C1C: cmp.w $22(a6), d0
pub fn i_065c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C20;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $065C20: blt.w $65c2a
pub fn i_065c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C24;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x65C2A; return 10; }
    8
}
// ===== SetXN_065c24  ($065C24, 6 B, thunks/ccr_helpers.c)
/// $065C24: ori.b #$11, ccr
pub fn i_065c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C28;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $065C28: rts
pub fn i_065c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_065c2a  ($065C2A, 6 B, thunks/ccr_helpers.c)
/// $065C2A: andi.b #$ee, ccr
pub fn i_065c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C2E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $065C2E: rts
pub fn i_065c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_StepAnimB_065c30  ($065C30, 52 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065C30: move.w $82(a6), d0
pub fn i_065c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C34;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065C34: btst.b #$0, $3a(a6)
pub fn i_065c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C3A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065C3A: bne.w $65c52
pub fn i_065c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C3E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65C52; return 10; }
    8
}
/// $065C3E: cmp.w $28(a6), d0
pub fn i_065c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C42;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $065C42: blt.w $65c62
pub fn i_065c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C46;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x65C62; return 10; }
    8
}
/// $065C46: move.w d0, $28(a6)
pub fn i_065c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C4A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065C4A: clr.w $2c(a6)
pub fn i_065c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C4E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $065C4E: bra.w $65c62
pub fn i_065c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C52;
    cpu.ir = 0x6000;
    cpu.pc = 0x65C62;
    10
}
/// $065C52: cmp.w $28(a6), d0
pub fn i_065c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C56;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $065C56: bgt.w $65c62
pub fn i_065c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C5A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x65C62; return 10; }
    8
}
/// $065C5A: move.w d0, $28(a6)
pub fn i_065c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065C5E: clr.w $2c(a6)
pub fn i_065c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C62;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $065C62: rts
pub fn i_065c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_SinCos_065c64  ($065C64, 48 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065C64: move.w $88(a6), d0
pub fn i_065c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C68;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065C68: lsl.w #$4, d0
pub fn i_065c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C6A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $065C6A: lea.l $2c072c.l, a1
pub fn i_065c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C70;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $065C70: lea.l $2c07ac.l, a2
pub fn i_065c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C76;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C07AC;
    20
}
/// $065C76: move.w (a1, d0.w), d1
pub fn i_065c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C7A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $065C7A: move.w (a2, d0.w), d2
pub fn i_065c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C7E;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $065C7E: move.w $36(a6), d0
pub fn i_065c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C82;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065C82: muls.w d0, d1
pub fn i_065c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C84;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $065C84: muls.w d0, d2
pub fn i_065c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C86;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $065C86: asr.l #$8, d1
pub fn i_065c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C88;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $065C88: asr.l #$8, d2
pub fn i_065c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C8A;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $065C8A: move.w d1, $2a(a6)
pub fn i_065c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C8E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065C8E: move.w d2, $28(a6)
pub fn i_065c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C92;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065C92: rts
pub fn i_065c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityGroup_SpawnLinkedFromTemplateList_065C94  ($065C94, 84 B, entity/group_spawn_linked_from_templates_065c94.s)
/// $065C94: move.l (a1), d0
pub fn i_065c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C96;
    cpu.ir = 0x2011;
    let v2 = bus.read32(cpu.a[1]);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065C96: cmpi.l #$ffffffff, d0
pub fn i_065c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C9C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $065C9C: beq.w $65ce6
pub fn i_065c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CA0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65CE6; return 10; }
    8
}
/// $065CA0: movem.l d0/a1, -(a7)
pub fn i_065ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CA4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $065CA4: lea.l $64d8a(pc), a1
pub fn i_065ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065CA8: jsr $4ae.l
pub fn i_065ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65CAE);
    cpu.pc = 0x4AE;
    16
}
/// $065CAE: jsr $5dd02.l
pub fn i_065cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65CB4);
    cpu.pc = 0x5DD02;
    16
}
/// $065CB4: movem.l (a7)+, d0/a1
pub fn i_065cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CB8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $065CB8: move.l d0, (a0)
pub fn i_065cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CBA;
    cpu.ir = 0x2080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065CBA: move.w $4(a1), d0
pub fn i_065cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CBE;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065CBE: jsr $65d32(pc)
pub fn i_065cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65CC2);
    cpu.pc = 0x65D32;
    16
}
/// $065CC2: add.w d0, $22(a0)
pub fn i_065cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CC6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065CC6: move.w $6(a1), d0
pub fn i_065cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CCA;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065CCA: add.w d0, $24(a0)
pub fn i_065cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CCE;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065CCE: move.w $8(a1), d0
pub fn i_065cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CD2;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065CD2: eor.b d0, $3a(a0)
pub fn i_065cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CD6;
    cpu.ir = 0xB128;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $065CD6: move.w $a(a1), d0
pub fn i_065cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CDA;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065CDA: add.w d0, $38(a0)
pub fn i_065cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CDE;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065CDE: adda.l #$c, a1
pub fn i_065cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CE4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $065CE4: bra.b $65c94
pub fn i_065ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CE6;
    cpu.ir = 0x60AE;
    cpu.pc = 0x65C94;
    10
}
/// $065CE6: rts
pub fn i_065ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_SpawnAtAngle_065ce8  ($065CE8, 74 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065CE8: lea.l $65934(pc), a1
pub fn i_065ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65934;
    16
}
/// $065CEC: jsr $4ae.l
pub fn i_065cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65CF2);
    cpu.pc = 0x4AE;
    16
}
/// $065CF2: jsr $5dd02.l
pub fn i_065cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65CF8);
    cpu.pc = 0x5DD02;
    16
}
/// $065CF8: move.w $88(a6), d0
pub fn i_065cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CFC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065CFC: lsl.w #$3, d0
pub fn i_065cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65CFE;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $065CFE: andi.w #$ff, d0
pub fn i_065cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D02;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $065D02: add.w d0, d0
pub fn i_065d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D04;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065D04: lea.l $2c072c.l, a1
pub fn i_065d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D0A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $065D0A: lea.l $2c07ac.l, a2
pub fn i_065d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C07AC;
    20
}
/// $065D10: move.w (a1, d0.w), d1
pub fn i_065d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D14;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $065D14: move.w (a2, d0.w), d2
pub fn i_065d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D18;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $065D18: asr.w #$4, d1
pub fn i_065d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D1A;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $065D1A: asr.w #$4, d2
pub fn i_065d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D1C;
    cpu.ir = 0xE842;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $065D1C: move.w d1, d3
pub fn i_065d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D1E;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065D1E: move.w d2, d4
pub fn i_065d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D20;
    cpu.ir = 0x3802;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065D20: asr.w #$1, d1
pub fn i_065d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D22;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $065D22: asr.w #$1, d2
pub fn i_065d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D24;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $065D24: add.w d3, d1
pub fn i_065d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D26;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065D26: add.w d4, d2
pub fn i_065d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D28;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $065D28: sub.w d1, $24(a0)
pub fn i_065d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D2C;
    cpu.ir = 0x9368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065D2C: sub.w d2, $22(a0)
pub fn i_065d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D30;
    cpu.ir = 0x9568;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065D30: rts
pub fn i_065d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MirrorDeltaByFacing_065D32  ($065D32, 12 B, entity/mirror_delta_by_facing_065d32.s)
/// $065D32: btst.b #$0, $3a(a6)
pub fn i_065d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D38;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065D38: beq.w $65d44
pub fn i_065d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65D44; return 10; }
    8
}
/// $065D3C: neg.w d0
pub fn i_065d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D3E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_065d3e  ($065D3E, 6 B, thunks/ccr_helpers.c)
/// $065D3E: ori.b #$11, ccr
pub fn i_065d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D42;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $065D42: rts
pub fn i_065d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_065d44  ($065D44, 6 B, thunks/ccr_helpers.c)
/// $065D44: andi.b #$ee, ccr
pub fn i_065d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D48;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $065D48: rts
pub fn i_065d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_CountRead_065d4a  ($065D4A, 48 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065D4A: cmpi.b #$0, $78(a6)
pub fn i_065d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D50;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $065D50: bne.w $65d64
pub fn i_065d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65D64; return 10; }
    8
}
/// $065D54: move.b $10e276.l, d0
pub fn i_065d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D5A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E276) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $065D5A: move.b $10e277.l, d1
pub fn i_065d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D60;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10E277) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $065D60: bra.w $65d70
pub fn i_065d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D64;
    cpu.ir = 0x6000;
    cpu.pc = 0x65D70;
    10
}
/// $065D64: move.b $10e278.l, d0
pub fn i_065d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D6A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E278) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $065D6A: move.b $10e279.l, d1
pub fn i_065d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D70;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10E279) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $065D70: cmp.b d0, d1
pub fn i_065d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D72;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $065D72: ble.w $65d78
pub fn i_065d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D76;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x65D78; return 10; }
    8
}
/// $065D76: move.b d1, d0
pub fn i_065d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D78;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $065D78: rts
pub fn i_065d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_CountInc_065d7a  ($065D7A, 26 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065D7A: cmpi.b #$0, $78(a6)
pub fn i_065d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D80;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $065D80: bne.w $65d8c
pub fn i_065d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65D8C; return 10; }
    8
}
/// $065D84: addq.b #$1, $10e277.l
pub fn i_065d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D8A;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10E277) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E277, r2 as u8);
    16
}
/// $065D8A: rts
pub fn i_065d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $065D8C: addq.b #$1, $10e279.l
pub fn i_065d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D92;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10E279) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E279, r2 as u8);
    16
}
/// $065D92: rts
pub fn i_065d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_Assert_065d94  ($065D94, 26 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065D94: move.w $82(a6), d0
pub fn i_065d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D98;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065D98: asr.w #$5, d0
pub fn i_065d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D9A;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $065D9A: cmpi.w #$0, d0
pub fn i_065d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65D9E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $065D9E: bne.w $65dae
pub fn i_065d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DA2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65DAE; return 10; }
    8
}
/// $065DA2: nop
pub fn i_065da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DA4;
    cpu.ir = 0x4E71;
    4
}
/// $065DA4: nop
pub fn i_065da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DA6;
    cpu.ir = 0x4E71;
    4
}
/// $065DA6: cmpi.w #$0, d0
pub fn i_065da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DAA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $065DAA: nop
pub fn i_065daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DAC;
    cpu.ir = 0x4E71;
    4
}
/// $065DAC: trap #$f
pub fn i_065dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DAE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
// ===== SetTaskW_065dae  ($065DAE, 6 B, kernel/task_setters_w.c)
/// $065DAE: move.w d0, $2c(a6)
pub fn i_065dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DB2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065DB2: rts
pub fn i_065db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Timer_065db4  ($065DB4, 26 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065DB4: subq.w #$1, $70(a6)
pub fn i_065db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DB8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065DB8: cmpi.w #$f, $70(a6)
pub fn i_065db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DBE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Word, false);
    16
}
/// $065DBE: bgt.w $65dce
pub fn i_065dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DC2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x65DCE; return 10; }
    8
}
/// $065DC2: move.w $70(a6), d0
pub fn i_065dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DC6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065DC6: btst.b #$0, d0
pub fn i_065dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DCA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $065DCA: beq.w $65dd6
pub fn i_065dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65DD6; return 10; }
    8
}
// ===== JsrAbsThunk_065dce  ($065DCE, 8 B, thunks/jsr_abs_thunks.c)
/// $065DCE: jsr $28d70.l
pub fn i_065dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65DD4);
    cpu.pc = 0x28D70;
    16
}
/// $065DD4: rts
pub fn i_065dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_065dd6  ($065DD6, 6 B, thunks/ccr_helpers.c)
/// $065DD6: andi.b #$ee, ccr
pub fn i_065dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DDA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $065DDA: rts
pub fn i_065dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_SpawnTriple_065ddc  ($065DDC, 84 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065DDC: move.w #$4000, d0
pub fn i_065ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065DE0: jsr $28134.l
pub fn i_065de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65DE6);
    cpu.pc = 0x28134;
    16
}
/// $065DE6: andi.w #$ffe3, $38(a6)
pub fn i_065de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DEC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $065DEC: ori.w #$0, $38(a6)
pub fn i_065dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DF2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $065DF2: lea.l $77fd6.l, a1
pub fn i_065df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DF8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $065DF8: jsr $4ae.l
pub fn i_065df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65DFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65DFE);
    cpu.pc = 0x4AE;
    16
}
/// $065DFE: jsr $5dd02.l
pub fn i_065dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E04);
    cpu.pc = 0x5DD02;
    16
}
/// $065E04: subq.w #$1, $38(a0)
pub fn i_065e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E08;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065E08: lea.l $6546a(pc), a1
pub fn i_065e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6546A;
    16
}
/// $065E0C: jsr $4ae.l
pub fn i_065e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E12);
    cpu.pc = 0x4AE;
    16
}
/// $065E12: jsr $5dd02.l
pub fn i_065e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E18);
    cpu.pc = 0x5DD02;
    16
}
/// $065E18: lea.l $6546a(pc), a1
pub fn i_065e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6546A;
    16
}
/// $065E1C: jsr $4ae.l
pub fn i_065e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E22);
    cpu.pc = 0x4AE;
    16
}
/// $065E22: jsr $5dd02.l
pub fn i_065e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E28);
    cpu.pc = 0x5DD02;
    16
}
/// $065E28: eori.b #$1, $3a(a0)
pub fn i_065e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E2E;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $065E2E: rts
pub fn i_065e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_SpawnPair_065e30  ($065E30, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065E30: lea.l $65414(pc), a1
pub fn i_065e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65414;
    16
}
/// $065E34: jsr $4ae.l
pub fn i_065e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E3A);
    cpu.pc = 0x4AE;
    16
}
/// $065E3A: jsr $5dd02.l
pub fn i_065e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E40);
    cpu.pc = 0x5DD02;
    16
}
/// $065E40: subq.w #$1, $38(a0)
pub fn i_065e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E44;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065E44: addi.w #$10, $24(a0)
pub fn i_065e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E4A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $065E4A: cmpi.b #$1, $79(a6)
pub fn i_065e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E50;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $065E50: bne.w $65e6c
pub fn i_065e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65E6C; return 10; }
    8
}
/// $065E54: lea.l $65414(pc), a1
pub fn i_065e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65414;
    16
}
/// $065E58: jsr $4ae.l
pub fn i_065e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E5E);
    cpu.pc = 0x4AE;
    16
}
/// $065E5E: jsr $5dd02.l
pub fn i_065e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E64);
    cpu.pc = 0x5DD02;
    16
}
/// $065E64: subq.w #$1, $38(a0)
pub fn i_065e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E68;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065E68: addq.w #$8, $24(a0)
pub fn i_065e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E6C;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065E6C: rts
pub fn i_065e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_SpawnPairPrio_065e6e  ($065E6E, 122 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065E6E: cmpi.b #$1, $78(a6)
pub fn i_065e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E74;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $065E74: beq.w $65ea4
pub fn i_065e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E78;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65EA4; return 10; }
    8
}
/// $065E78: jsr $4a16e.l
pub fn i_065e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E7E);
    cpu.pc = 0x4A16E;
    16
}
/// $065E7E: move.w $38(a6), d0
pub fn i_065e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E82;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065E82: subq.w #$1, d0
pub fn i_065e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E84;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $065E84: move.w d0, $38(a0)
pub fn i_065e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E88;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065E88: addi.w #$10, $24(a0)
pub fn i_065e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E8E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $065E8E: cmpi.b #$1, $79(a6)
pub fn i_065e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E94;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $065E94: bne.w $65ea2
pub fn i_065e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65EA2; return 10; }
    8
}
/// $065E98: jsr $4a16e.l
pub fn i_065e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65E9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65E9E);
    cpu.pc = 0x4A16E;
    16
}
/// $065E9E: addq.w #$8, $24(a0)
pub fn i_065e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EA2;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065EA2: rts
pub fn i_065ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $065EA4: lea.l $4a7a6.l, a1
pub fn i_065ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EAA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4A7A6;
    20
}
/// $065EAA: jsr $4ae.l
pub fn i_065eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65EB0);
    cpu.pc = 0x4AE;
    16
}
/// $065EB0: jsr $5dd02.l
pub fn i_065eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65EB6);
    cpu.pc = 0x5DD02;
    16
}
/// $065EB6: move.w $38(a6), d0
pub fn i_065eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EBA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065EBA: subq.w #$1, d0
pub fn i_065eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EBC;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $065EBC: move.w d0, $38(a0)
pub fn i_065ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EC0;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065EC0: addi.w #$10, $24(a0)
pub fn i_065ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EC6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $065EC6: cmpi.b #$1, $79(a6)
pub fn i_065ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65ECC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $065ECC: bne.w $65ee6
pub fn i_065ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65ED0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65EE6; return 10; }
    8
}
/// $065ED0: lea.l $4a7a6.l, a1
pub fn i_065ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65ED6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4A7A6;
    20
}
/// $065ED6: jsr $4ae.l
pub fn i_065ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65EDC);
    cpu.pc = 0x4AE;
    16
}
/// $065EDC: jsr $5dd02.l
pub fn i_065edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65EE2);
    cpu.pc = 0x5DD02;
    16
}
/// $065EE2: addq.w #$8, $24(a0)
pub fn i_065ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EE6;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065EE6: rts
pub fn i_065ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Register0_065ee8  ($065EE8, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065EE8: lea.l $2c6500.l, a1
pub fn i_065ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EEE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C6500;
    20
}
// ===== JsrPcThunk_065eee  ($065EEE, 6 B, thunks/jsr_pc_thunks.c)
/// $065EEE: jsr $65c94(pc)
pub fn i_065eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65EF2);
    cpu.pc = 0x65C94;
    16
}
/// $065EF2: rts
pub fn i_065ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Register1_065ef4  ($065EF4, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065EF4: lea.l $2c6460.l, a1
pub fn i_065ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EFA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C6460;
    20
}
// ===== JsrPcThunk_065efa  ($065EFA, 6 B, thunks/jsr_pc_thunks.c)
/// $065EFA: jsr $65c94(pc)
pub fn i_065efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65EFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65EFE);
    cpu.pc = 0x65C94;
    16
}
/// $065EFE: rts
pub fn i_065efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Register2_065f00  ($065F00, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065F00: lea.l $2c6488.l, a1
pub fn i_065f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F06;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C6488;
    20
}
// ===== JsrPcThunk_065f06  ($065F06, 6 B, thunks/jsr_pc_thunks.c)
/// $065F06: jsr $65c94(pc)
pub fn i_065f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65F0A);
    cpu.pc = 0x65C94;
    16
}
/// $065F0A: rts
pub fn i_065f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Register3_065f0c  ($065F0C, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065F0C: lea.l $2c64b0.l, a1
pub fn i_065f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C64B0;
    20
}
// ===== JsrPcThunk_065f12  ($065F12, 6 B, thunks/jsr_pc_thunks.c)
/// $065F12: jsr $65c94(pc)
pub fn i_065f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65F16);
    cpu.pc = 0x65C94;
    16
}
/// $065F16: rts
pub fn i_065f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Register4_065f18  ($065F18, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065F18: lea.l $2c64d8.l, a1
pub fn i_065f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F1E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C64D8;
    20
}
// ===== JsrPcThunk_065f1e  ($065F1E, 6 B, thunks/jsr_pc_thunks.c)
/// $065F1E: jsr $65c94(pc)
pub fn i_065f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65F22);
    cpu.pc = 0x65C94;
    16
}
/// $065F22: rts
pub fn i_065f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_StubB_065f24  ($065F24, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065F24: movea.l $8(a6), a1
pub fn i_065f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F28;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $065F28: move.b $10(a6), d0
pub fn i_065f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F2C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $065F2C: cmp.b $10(a1), d0
pub fn i_065f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F30;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $065F30: bcs.w $65f3a
pub fn i_065f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F34;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x65F3A; return 10; }
    8
}
// ===== ClearXN_065f34  ($065F34, 6 B, thunks/ccr_helpers.c)
/// $065F34: andi.b #$ee, ccr
pub fn i_065f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F38;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $065F38: rts
pub fn i_065f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_065f3a  ($065F3A, 6 B, thunks/ccr_helpers.c)
/// $065F3A: ori.b #$11, ccr
pub fn i_065f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F3E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $065F3E: rts
pub fn i_065f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrel_Tmpl8D_065f40  ($065F40, 122 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065F40: move.w #$179, d1
pub fn i_065f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x179;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065F44: jsr $236e.l
pub fn i_065f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65F4A);
    cpu.pc = 0x236E;
    16
}
/// $065F4A: bset.b #$6, $12(a6)
pub fn i_065f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F50;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $065F50: move.w #$8000, $38(a6)
pub fn i_065f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065F56: jsr $267e2.l
pub fn i_065f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65F5C);
    cpu.pc = 0x267E2;
    16
}
/// $065F5C: jsr $27cee.l
pub fn i_065f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65F62);
    cpu.pc = 0x27CEE;
    16
}
/// $065F62: move.w #$1, $66(a6)
pub fn i_065f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065F68: lea.l $2c756c.l, a0
pub fn i_065f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C756C;
    20
}
/// $065F6E: jsr $28cd4.l
pub fn i_065f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65F74);
    cpu.pc = 0x28CD4;
    16
}
/// $065F74: lea.l $65f7a(pc), a1
pub fn i_065f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65F7A;
    16
}
/// $065F78: move.l a1, (a6)
pub fn i_065f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065F7A: jsr $2783a.l
pub fn i_065f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65F80);
    cpu.pc = 0x2783A;
    16
}
/// $065F80: jsr $28d70.l
pub fn i_065f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65F86);
    cpu.pc = 0x28D70;
    16
}
/// $065F86: jsr $66622(pc)
pub fn i_065f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65F8A);
    cpu.pc = 0x66622;
    16
}
/// $065F8A: jsr $66644(pc)
pub fn i_065f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65F8E);
    cpu.pc = 0x66644;
    16
}
/// $065F8E: bcc.w $65f9c
pub fn i_065f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65F9C; return 10; }
    8
}
/// $065F92: move.l a0, $70(a6)
pub fn i_065f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F96;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $065F96: lea.l $65fba(pc), a1
pub fn i_065f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65FBA;
    16
}
/// $065F9A: move.l a1, (a6)
pub fn i_065f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65F9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065F9C: movea.l #$ffffffff, a0
pub fn i_065f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FA2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $065FA2: lea.l $2c7998.l, a0
pub fn i_065fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $065FA8: jsr $5dd5c.l
pub fn i_065fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65FAE);
    cpu.pc = 0x5DD5C;
    16
}
/// $065FAE: bcc.w $65fb8
pub fn i_065fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65FB8; return 10; }
    8
}
/// $065FB2: jmp $518.l
pub fn i_065fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $065FB8: rts
pub fn i_065fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barrel_Idle_065fba  ($065FBA, 70 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065FBA: lea.l $2c75e6.l, a0
pub fn i_065fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C75E6;
    20
}
/// $065FC0: jsr $28cd4.l
pub fn i_065fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65FC6);
    cpu.pc = 0x28CD4;
    16
}
/// $065FC6: move.b $98(a6), $74(a6)
pub fn i_065fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FCC;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad4, v3 as u8);
    20
}
/// $065FCC: lea.l $65fd2(pc), a1
pub fn i_065fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65FD2;
    16
}
/// $065FD0: move.l a1, (a6)
pub fn i_065fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065FD2: jsr $2783a.l
pub fn i_065fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65FD8);
    cpu.pc = 0x2783A;
    16
}
/// $065FD8: jsr $28d70.l
pub fn i_065fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65FDE);
    cpu.pc = 0x28D70;
    16
}
/// $065FDE: bcc.w $65ff0
pub fn i_065fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x65FF0; return 10; }
    8
}
/// $065FE2: subq.b #$1, $74(a6)
pub fn i_065fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FE6;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $065FE6: bne.w $65ff0
pub fn i_065fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65FF0; return 10; }
    8
}
/// $065FEA: lea.l $6600e(pc), a1
pub fn i_065fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6600E;
    16
}
/// $065FEE: move.l a1, (a6)
pub fn i_065fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065FF0: movea.l #$ffffffff, a0
pub fn i_065ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FF6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $065FF6: lea.l $2c7998.l, a0
pub fn i_065ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65FFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $065FFC: jsr $5dd5c.l
pub fn i_065ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66002;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66002);
    cpu.pc = 0x5DD5C;
    16
}
// ===== FloatBarrel_OffworldFree_066000  ($066000, 14 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066000: add.w d6, (a4)+
pub fn i_066000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66002;
    cpu.ir = 0xDD5C;
    let ad1 = cpu.a[4]; cpu.a[4] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $066002: bcc.w $6600c
pub fn i_066002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66006;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6600C; return 10; }
    8
}
/// $066006: jmp $518.l
pub fn i_066006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6600C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06600C: rts
pub fn i_06600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6600E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_Submerge_06600e  ($06600E, 190 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06600E: move.w #$0, $28(a6)
pub fn i_06600e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66014;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066014: move.w #$a00, $2a(a6)
pub fn i_066014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6601A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06601A: move.w #$0, $2c(a6)
pub fn i_06601a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66020;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066020: move.w #$ff80, $2e(a6)
pub fn i_066020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66026;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066026: move.b $99(a6), d0
pub fn i_066026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6602A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06602A: andi.b #$7, d0
pub fn i_06602a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6602E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06602E: cmpi.b #$0, d0
pub fn i_06602e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66032;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $066032: beq.w $66040
pub fn i_066032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66036;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x66040; return 10; }
    8
}
/// $066036: subq.b #$1, d0
pub fn i_066036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66038;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $066038: addi.w #$8, $2e(a6)
pub fn i_066038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6603E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06603E: bra.b $6602e
pub fn i_06603e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66040;
    cpu.ir = 0x60EE;
    cpu.pc = 0x6602E;
    10
}
/// $066040: lea.l $2c7626.l, a0
pub fn i_066040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66046;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7626;
    20
}
/// $066046: jsr $28cd4.l
pub fn i_066046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6604C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6604C);
    cpu.pc = 0x28CD4;
    16
}
/// $06604C: lea.l $788c0.l, a1
pub fn i_06604c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66052;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x788C0;
    20
}
/// $066052: jsr $4ae.l
pub fn i_066052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66058);
    cpu.pc = 0x4AE;
    16
}
/// $066058: jsr $5dd02.l
pub fn i_066058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6605E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6605E);
    cpu.pc = 0x5DD02;
    16
}
/// $06605E: addq.w #$8, $24(a0)
pub fn i_06605e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66062;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $066062: bset.b #$6, $12(a0)
pub fn i_066062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66068;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $066068: lea.l $6606e(pc), a1
pub fn i_066068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6606C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6606E;
    16
}
/// $06606C: move.l a1, (a6)
pub fn i_06606c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6606E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06606E: jsr $27cee.l
pub fn i_06606e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66074);
    cpu.pc = 0x27CEE;
    16
}
/// $066074: jsr $28d70.l
pub fn i_066074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6607A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6607A);
    cpu.pc = 0x28D70;
    16
}
/// $06607A: clr.l d0
pub fn i_06607a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6607C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $06607C: move.w $2a(a6), d0
pub fn i_06607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66080;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066080: btst.b #$f, d0
pub fn i_066080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66084;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $066084: beq.w $6608e
pub fn i_066084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66088;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6608E; return 10; }
    8
}
/// $066088: lea.l $660cc(pc), a1
pub fn i_066088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6608C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x660CC;
    16
}
/// $06608C: move.l a1, (a6)
pub fn i_06608c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6608E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06608E: jsr $2870a.l
pub fn i_06608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66094);
    cpu.pc = 0x2870A;
    16
}
/// $066094: bcc.w $660ae
pub fn i_066094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66098;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x660AE; return 10; }
    8
}
/// $066098: lea.l $66528(pc), a1
pub fn i_066098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6609C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66528;
    16
}
/// $06609C: jsr $4ae.l
pub fn i_06609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x660A2);
    cpu.pc = 0x4AE;
    16
}
/// $0660A2: jsr $5dd02.l
pub fn i_0660a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x660A8);
    cpu.pc = 0x5DD02;
    16
}
/// $0660A8: lea.l $66228(pc), a1
pub fn i_0660a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66228;
    16
}
/// $0660AC: move.l a1, (a6)
pub fn i_0660ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0660AE: movea.l #$ffffffff, a0
pub fn i_0660ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0660B4: lea.l $2c7998.l, a0
pub fn i_0660b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $0660BA: jsr $5dd5c.l
pub fn i_0660ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x660C0);
    cpu.pc = 0x5DD5C;
    16
}
/// $0660C0: bcc.w $660ca
pub fn i_0660c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x660CA; return 10; }
    8
}
/// $0660C4: jmp $518.l
pub fn i_0660c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0660CA: rts
pub fn i_0660ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_Surface_0660cc  ($0660CC, 168 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0660CC: move.w $2e(a6), d0
pub fn i_0660cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0660D0: asr.w #$1, d0
pub fn i_0660d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660D2;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0660D2: move.w d0, $2e(a6)
pub fn i_0660d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660D6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0660D6: jsr $66622(pc)
pub fn i_0660d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x660DA);
    cpu.pc = 0x66622;
    16
}
/// $0660DA: lea.l $2c765c.l, a0
pub fn i_0660da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C765C;
    20
}
/// $0660E0: jsr $28cd4.l
pub fn i_0660e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x660E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0660E6: lea.l $660ec(pc), a1
pub fn i_0660e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x660EC;
    16
}
/// $0660EA: move.l a1, (a6)
pub fn i_0660ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0660EC: jsr $27cee.l
pub fn i_0660ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x660F2);
    cpu.pc = 0x27CEE;
    16
}
/// $0660F2: jsr $28d70.l
pub fn i_0660f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x660F8);
    cpu.pc = 0x28D70;
    16
}
/// $0660F8: lea.l $2c79ac.l, a0
pub fn i_0660f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x660FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C79AC;
    20
}
/// $0660FE: jsr $5e086.l
pub fn i_0660fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66104;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66104);
    cpu.pc = 0x5E086;
    16
}
/// $066104: bcs.w $6611e
pub fn i_066104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66108;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6611E; return 10; }
    8
}
/// $066108: lea.l $2c79b6.l, a0
pub fn i_066108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6610E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C79B6;
    20
}
/// $06610E: jsr $5e086.l
pub fn i_06610e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66114;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66114);
    cpu.pc = 0x5E086;
    16
}
/// $066114: bcc.w $6611e
pub fn i_066114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66118;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6611E; return 10; }
    8
}
/// $066118: lea.l $66174(pc), a1
pub fn i_066118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6611C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66174;
    16
}
/// $06611C: move.l a1, (a6)
pub fn i_06611c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6611E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06611E: jsr $27fac.l
pub fn i_06611e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66124);
    cpu.pc = 0x27FAC;
    16
}
/// $066124: bcs.w $66136
pub fn i_066124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66128;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x66136; return 10; }
    8
}
/// $066128: cmpi.b #$41, d7
pub fn i_066128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6612C;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x41, Size::Byte, false);
    8
}
/// $06612C: bne.w $66136
pub fn i_06612c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66130;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66136; return 10; }
    8
}
/// $066130: lea.l $661d8(pc), a1
pub fn i_066130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66134;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x661D8;
    16
}
/// $066134: move.l a1, (a6)
pub fn i_066134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66136;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066136: jsr $2870a.l
pub fn i_066136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6613C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6613C);
    cpu.pc = 0x2870A;
    16
}
/// $06613C: bcc.w $66156
pub fn i_06613c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66140;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66156; return 10; }
    8
}
/// $066140: lea.l $66528(pc), a1
pub fn i_066140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66144;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66528;
    16
}
/// $066144: jsr $4ae.l
pub fn i_066144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6614A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6614A);
    cpu.pc = 0x4AE;
    16
}
/// $06614A: jsr $5dd02.l
pub fn i_06614a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66150);
    cpu.pc = 0x5DD02;
    16
}
/// $066150: lea.l $66228(pc), a1
pub fn i_066150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66154;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66228;
    16
}
/// $066154: move.l a1, (a6)
pub fn i_066154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66156;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066156: movea.l #$ffffffff, a0
pub fn i_066156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6615C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06615C: lea.l $2c7998.l, a0
pub fn i_06615c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66162;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $066162: jsr $5dd5c.l
pub fn i_066162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66168);
    cpu.pc = 0x5DD5C;
    16
}
/// $066168: bcc.w $66172
pub fn i_066168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6616C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66172; return 10; }
    8
}
/// $06616C: jmp $518.l
pub fn i_06616c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66172;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $066172: rts
pub fn i_066172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66174;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_Drift_066174  ($066174, 100 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066174: lea.l $2c7688.l, a0
pub fn i_066174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6617A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7688;
    20
}
/// $06617A: jsr $28cd4.l
pub fn i_06617a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66180;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66180);
    cpu.pc = 0x28CD4;
    16
}
/// $066180: lea.l $66186(pc), a1
pub fn i_066180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66184;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66186;
    16
}
/// $066184: move.l a1, (a6)
pub fn i_066184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66186;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066186: jsr $27cee.l
pub fn i_066186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6618C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6618C);
    cpu.pc = 0x27CEE;
    16
}
/// $06618C: jsr $28d70.l
pub fn i_06618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66192);
    cpu.pc = 0x28D70;
    16
}
/// $066192: jsr $27eba.l
pub fn i_066192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66198;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66198);
    cpu.pc = 0x27EBA;
    16
}
/// $066198: bcs.w $661aa
pub fn i_066198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6619C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x661AA; return 10; }
    8
}
/// $06619C: cmpi.b #$41, d7
pub fn i_06619c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661A0;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x41, Size::Byte, false);
    8
}
/// $0661A0: bne.w $661aa
pub fn i_0661a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x661AA; return 10; }
    8
}
/// $0661A4: lea.l $661d8(pc), a1
pub fn i_0661a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x661D8;
    16
}
/// $0661A8: move.l a1, (a6)
pub fn i_0661a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0661AA: jsr $2870a.l
pub fn i_0661aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x661B0);
    cpu.pc = 0x2870A;
    16
}
/// $0661B0: bcc.w $661ba
pub fn i_0661b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x661BA; return 10; }
    8
}
/// $0661B4: lea.l $66228(pc), a1
pub fn i_0661b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66228;
    16
}
/// $0661B8: move.l a1, (a6)
pub fn i_0661b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0661BA: movea.l #$ffffffff, a0
pub fn i_0661ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661C0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0661C0: lea.l $2c7998.l, a0
pub fn i_0661c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $0661C6: jsr $5dd5c.l
pub fn i_0661c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x661CC);
    cpu.pc = 0x5DD5C;
    16
}
/// $0661CC: bcc.w $661d6
pub fn i_0661cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x661D6; return 10; }
    8
}
/// $0661D0: jmp $518.l
pub fn i_0661d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0661D6: rts
pub fn i_0661d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_Splash_0661d8  ($0661D8, 78 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0661D8: movea.l #$ffffffff, a0
pub fn i_0661d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661DE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0661DE: lea.l $2c7998.l, a0
pub fn i_0661de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $0661E4: jsr $5dd56.l
pub fn i_0661e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x661EA);
    cpu.pc = 0x5DD56;
    16
}
/// $0661EA: bcs.w $66214
pub fn i_0661ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x66214; return 10; }
    8
}
/// $0661EE: move.w #$1052, d0
pub fn i_0661ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661F2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1052;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0661F2: jsr $2352.l
pub fn i_0661f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x661F8);
    cpu.pc = 0x2352;
    16
}
/// $0661F8: lea.l $788c0.l, a1
pub fn i_0661f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x661FE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x788C0;
    20
}
/// $0661FE: jsr $4ae.l
pub fn i_0661fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66204);
    cpu.pc = 0x4AE;
    16
}
/// $066204: jsr $5dd02.l
pub fn i_066204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6620A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6620A);
    cpu.pc = 0x5DD02;
    16
}
/// $06620A: addq.w #$8, $24(a0)
pub fn i_06620a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6620E;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06620E: bset.b #$6, $12(a0)
pub fn i_06620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66214;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $066214: lea.l $6621a(pc), a1
pub fn i_066214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66218;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6621A;
    16
}
/// $066218: move.l a1, (a6)
pub fn i_066218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6621A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06621A: jsr $28d70.l
pub fn i_06621a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66220);
    cpu.pc = 0x28D70;
    16
}
/// $066220: jmp $518.l
pub fn i_066220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66226;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_066226  ($066226, 2 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066226: rts
pub fn i_066226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66228;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_Pop_066228  ($066228, 48 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066228: move.w #$103e, d0
pub fn i_066228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6622C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x103E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06622C: jsr $2352.l
pub fn i_06622c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66232);
    cpu.pc = 0x2352;
    16
}
/// $066232: bclr.b #$6, $12(a6)
pub fn i_066232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66238;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $066238: jsr $267e2.l
pub fn i_066238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6623E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6623E);
    cpu.pc = 0x267E2;
    16
}
/// $06623E: movea.l $50(a6), a0
pub fn i_06623e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66242;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $066242: move.w $28(a0), d0
pub fn i_066242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66246;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066246: asr.w #$2, d0
pub fn i_066246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66248;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $066248: move.w d0, $28(a6)
pub fn i_066248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6624C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06624C: lea.l $2c76e8.l, a0
pub fn i_06624c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66252;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C76E8;
    20
}
/// $066252: jsr $28cd4.l
pub fn i_066252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66258;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66258);
    cpu.pc = 0x28CD4;
    16
}
// ===== FloatBarrel_Bob_066258  ($066258, 76 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066258: move.w #$0, $2a(a6)
pub fn i_066258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6625E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06625E: move.w #$ffd0, $2e(a6)
pub fn i_06625e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66264;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066264: lea.l $6626a(pc), a1
pub fn i_066264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66268;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6626A;
    16
}
/// $066268: move.l a1, (a6)
pub fn i_066268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6626A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06626A: jsr $27d50.l
pub fn i_06626a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66270);
    cpu.pc = 0x27D50;
    16
}
/// $066270: jsr $27eba.l
pub fn i_066270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66276;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66276);
    cpu.pc = 0x27EBA;
    16
}
/// $066276: bcs.w $66280
pub fn i_066276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6627A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x66280; return 10; }
    8
}
/// $06627A: lea.l $662a4(pc), a1
pub fn i_06627a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6627E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x662A4;
    16
}
/// $06627E: move.l a1, (a6)
pub fn i_06627e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66280;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066280: jsr $28d70.l
pub fn i_066280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66286);
    cpu.pc = 0x28D70;
    16
}
/// $066286: movea.l #$ffffffff, a0
pub fn i_066286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6628C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06628C: lea.l $2c7998.l, a0
pub fn i_06628c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66292;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $066292: jsr $5dd5c.l
pub fn i_066292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66298);
    cpu.pc = 0x5DD5C;
    16
}
/// $066298: bcc.w $662a2
pub fn i_066298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6629C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x662A2; return 10; }
    8
}
/// $06629C: jmp $518.l
pub fn i_06629c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0662A2: rts
pub fn i_0662a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_BobSettle_0662a4  ($0662A4, 214 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0662A4: jsr $27eba.l
pub fn i_0662a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x662AA);
    cpu.pc = 0x27EBA;
    16
}
/// $0662AA: bcs.w $662e2
pub fn i_0662aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662AE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x662E2; return 10; }
    8
}
/// $0662AE: cmpi.b #$41, d7
pub fn i_0662ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662B2;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x41, Size::Byte, false);
    8
}
/// $0662B2: bne.w $662e2
pub fn i_0662b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x662E2; return 10; }
    8
}
/// $0662B6: move.w #$1052, d0
pub fn i_0662b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1052;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0662BA: jsr $2352.l
pub fn i_0662ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x662C0);
    cpu.pc = 0x2352;
    16
}
/// $0662C0: lea.l $788c0.l, a1
pub fn i_0662c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x788C0;
    20
}
/// $0662C6: jsr $4ae.l
pub fn i_0662c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x662CC);
    cpu.pc = 0x4AE;
    16
}
/// $0662CC: jsr $5dd02.l
pub fn i_0662cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x662D2);
    cpu.pc = 0x5DD02;
    16
}
/// $0662D2: addq.w #$8, $24(a0)
pub fn i_0662d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662D6;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0662D6: bset.b #$6, $12(a0)
pub fn i_0662d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662DC;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0662DC: jmp $518.l
pub fn i_0662dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662E2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0662E2: move.w $28(a6), d0
pub fn i_0662e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662E6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0662E6: neg.w d0
pub fn i_0662e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662E8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0662E8: asr.w #$5, d0
pub fn i_0662e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662EA;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0662EA: move.w d0, $2c(a6)
pub fn i_0662ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0662EE: move.w #$0, $2a(a6)
pub fn i_0662ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0662F4: move.w #$0, $2e(a6)
pub fn i_0662f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0662FA: lea.l $2c7744.l, a0
pub fn i_0662fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66300;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7744;
    20
}
/// $066300: jsr $28cd4.l
pub fn i_066300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66306;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66306);
    cpu.pc = 0x28CD4;
    16
}
/// $066306: lea.l $6630c(pc), a1
pub fn i_066306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6630A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6630C;
    16
}
/// $06630A: move.l a1, (a6)
pub fn i_06630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6630C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06630C: move.w $28(a6), d0
pub fn i_06630c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66310;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066310: movem.w d0, -(a7)
pub fn i_066310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66314;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $066314: jsr $27d50.l
pub fn i_066314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6631A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6631A);
    cpu.pc = 0x27D50;
    16
}
/// $06631A: movem.w (a7)+, d0
pub fn i_06631a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6631E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06631E: move.w $28(a6), d1
pub fn i_06631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66322;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066322: andi.w #$8000, d0
pub fn i_066322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66326;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $066326: andi.w #$8000, d1
pub fn i_066326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6632A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06632A: cmp.w d0, d1
pub fn i_06632a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6632C;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06632C: beq.w $6633c
pub fn i_06632c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66330;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6633C; return 10; }
    8
}
/// $066330: move.w #$0, $28(a6)
pub fn i_066330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66336;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066336: move.w #$0, $2c(a6)
pub fn i_066336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6633C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06633C: jsr $27eba.l
pub fn i_06633c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66342);
    cpu.pc = 0x27EBA;
    16
}
/// $066342: bcc.w $6634c
pub fn i_066342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66346;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6634C; return 10; }
    8
}
/// $066346: lea.l $66258(pc), a1
pub fn i_066346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6634A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66258;
    16
}
/// $06634A: move.l a1, (a6)
pub fn i_06634a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6634C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06634C: jsr $28d70.l
pub fn i_06634c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66352);
    cpu.pc = 0x28D70;
    16
}
/// $066352: bcc.w $6635c
pub fn i_066352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66356;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6635C; return 10; }
    8
}
/// $066356: jmp $518.l
pub fn i_066356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6635C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06635C: movea.l #$ffffffff, a0
pub fn i_06635c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66362;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $066362: lea.l $2c7998.l, a0
pub fn i_066362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66368;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7998;
    20
}
/// $066368: jsr $5dd5c.l
pub fn i_066368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6636E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6636E);
    cpu.pc = 0x5DD5C;
    16
}
/// $06636E: bcc.w $66378
pub fn i_06636e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66372;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66378; return 10; }
    8
}
/// $066372: jmp $518.l
pub fn i_066372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66378;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $066378: rts
pub fn i_066378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6637A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatMine_Init_06637a  ($06637A, 278 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06637A: move.w #$179, d1
pub fn i_06637a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6637E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x179;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06637E: jsr $236e.l
pub fn i_06637e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66384);
    cpu.pc = 0x236E;
    16
}
/// $066384: move.w #$1da, d1
pub fn i_066384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66388;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066388: jsr $236e.l
pub fn i_066388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6638E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6638E);
    cpu.pc = 0x236E;
    16
}
/// $06638E: move.w #$8000, $38(a6)
pub fn i_06638e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66394;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066394: jsr $267e2.l
pub fn i_066394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6639A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6639A);
    cpu.pc = 0x267E2;
    16
}
/// $06639A: lea.l $2bdb84.l, a0
pub fn i_06639a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDB84;
    20
}
/// $0663A0: jsr $799de.l
pub fn i_0663a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x663A6);
    cpu.pc = 0x799DE;
    16
}
/// $0663A6: move.w d0, d0
pub fn i_0663a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663A8;
    cpu.ir = 0x3000;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0663A8: move.w #$18, d1
pub fn i_0663a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663AC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0663AC: btst.b #$0, $3a(a6)
pub fn i_0663ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0663B2: bne.w $663ba
pub fn i_0663b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x663BA; return 10; }
    8
}
/// $0663B6: neg.w d0
pub fn i_0663b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663B8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0663B8: neg.w d1
pub fn i_0663b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663BA;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0663BA: move.w d0, $28(a6)
pub fn i_0663ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663BE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0663BE: add.w d1, $22(a6)
pub fn i_0663be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663C2;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0663C2: addi.w #$24, $24(a6)
pub fn i_0663c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663C8;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x24, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0663C8: lea.l $2c7798.l, a0
pub fn i_0663c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7798;
    20
}
/// $0663CE: jsr $28cd4.l
pub fn i_0663ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x663D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0663D4: lea.l $2c784c.l, a0
pub fn i_0663d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C784C;
    20
}
/// $0663DA: move.l a0, $4c(a6)
pub fn i_0663da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663DE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0663DE: jsr $283ca.l
pub fn i_0663de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x663E4);
    cpu.pc = 0x283CA;
    16
}
/// $0663E4: lea.l $2c7944.l, a0
pub fn i_0663e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7944;
    20
}
/// $0663EA: move.l a0, $48(a6)
pub fn i_0663ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663EE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0663EE: move.w #$64, $66(a6)
pub fn i_0663ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0663F4: move.w #$f400, $76(a6)
pub fn i_0663f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0663FA: lea.l $66400(pc), a1
pub fn i_0663fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x663FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66400;
    16
}
/// $0663FE: move.l a1, (a6)
pub fn i_0663fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66400;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066400: jsr $665e8(pc)
pub fn i_066400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66404;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x66404);
    cpu.pc = 0x665E8;
    16
}
/// $066404: bcc.w $6640e
pub fn i_066404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66408;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6640E; return 10; }
    8
}
/// $066408: lea.l $66490(pc), a1
pub fn i_066408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6640C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66490;
    16
}
/// $06640C: move.l a1, (a6)
pub fn i_06640c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6640E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06640E: jsr $665c6(pc)
pub fn i_06640e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66412;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x66412);
    cpu.pc = 0x665C6;
    16
}
/// $066412: jsr $28d70.l
pub fn i_066412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66418);
    cpu.pc = 0x28D70;
    16
}
/// $066418: jsr $2870a.l
pub fn i_066418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6641E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6641E);
    cpu.pc = 0x2870A;
    16
}
/// $06641E: bcc.w $66432
pub fn i_06641e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66422;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66432; return 10; }
    8
}
/// $066422: bclr.b #$3, $13(a6)
pub fn i_066422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66428;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $066428: move.w #$108d, d0
pub fn i_066428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6642C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06642C: jsr $2352.l
pub fn i_06642c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66432);
    cpu.pc = 0x2352;
    16
}
/// $066432: jsr $28758.l
pub fn i_066432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66438;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66438);
    cpu.pc = 0x28758;
    16
}
/// $066438: bcs.w $6646c
pub fn i_066438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6643C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6646C; return 10; }
    8
}
/// $06643C: jsr $27fac.l
pub fn i_06643c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66442);
    cpu.pc = 0x27FAC;
    16
}
/// $066442: bcc.w $6646c
pub fn i_066442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66446;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6646C; return 10; }
    8
}
/// $066446: jsr $283d8.l
pub fn i_066446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6644C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6644C);
    cpu.pc = 0x283D8;
    16
}
/// $06644C: btst.b #$1, $13(a6)
pub fn i_06644c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66452;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $066452: bne.w $6646c
pub fn i_066452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66456;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6646C; return 10; }
    8
}
/// $066456: movea.l #$ffffffff, a0
pub fn i_066456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6645C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06645C: lea.l $2c79a2.l, a0
pub fn i_06645c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66462;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C79A2;
    20
}
/// $066462: jsr $5dd56.l
pub fn i_066462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66468);
    cpu.pc = 0x5DD56;
    16
}
/// $066468: bcc.w $6648e
pub fn i_066468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6646C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6648E; return 10; }
    8
}
/// $06646C: move.w #$1022, d0
pub fn i_06646c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66470;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066470: jsr $2352.l
pub fn i_066470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66476);
    cpu.pc = 0x2352;
    16
}
/// $066476: lea.l $77f6a.l, a1
pub fn i_066476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6647C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $06647C: jsr $4ae.l
pub fn i_06647c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66482);
    cpu.pc = 0x4AE;
    16
}
/// $066482: jsr $5dd02.l
pub fn i_066482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66488);
    cpu.pc = 0x5DD02;
    16
}
/// $066488: jmp $518.l
pub fn i_066488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6648E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06648E: rts
pub fn i_06648e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66490;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatMine_Armed_066490  ($066490, 152 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066490: lea.l $2c77a8.l, a0
pub fn i_066490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66496;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C77A8;
    20
}
/// $066496: jsr $28cd4.l
pub fn i_066496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6649C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6649C);
    cpu.pc = 0x28CD4;
    16
}
/// $06649C: lea.l $664a2(pc), a1
pub fn i_06649c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x664A2;
    16
}
/// $0664A0: move.l a1, (a6)
pub fn i_0664a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0664A2: jsr $665e8(pc)
pub fn i_0664a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x664A6);
    cpu.pc = 0x665E8;
    16
}
/// $0664A6: jsr $665c6(pc)
pub fn i_0664a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x664AA);
    cpu.pc = 0x665C6;
    16
}
/// $0664AA: jsr $28d70.l
pub fn i_0664aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x664B0);
    cpu.pc = 0x28D70;
    16
}
/// $0664B0: jsr $2870a.l
pub fn i_0664b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x664B6);
    cpu.pc = 0x2870A;
    16
}
/// $0664B6: bcc.w $664ca
pub fn i_0664b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x664CA; return 10; }
    8
}
/// $0664BA: bclr.b #$3, $13(a6)
pub fn i_0664ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664C0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0664C0: move.w #$108d, d0
pub fn i_0664c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664C4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0664C4: jsr $2352.l
pub fn i_0664c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x664CA);
    cpu.pc = 0x2352;
    16
}
/// $0664CA: jsr $28758.l
pub fn i_0664ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x664D0);
    cpu.pc = 0x28758;
    16
}
/// $0664D0: bcs.w $66504
pub fn i_0664d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664D4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x66504; return 10; }
    8
}
/// $0664D4: jsr $27fac.l
pub fn i_0664d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x664DA);
    cpu.pc = 0x27FAC;
    16
}
/// $0664DA: bcc.w $66504
pub fn i_0664da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66504; return 10; }
    8
}
/// $0664DE: jsr $283d8.l
pub fn i_0664de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x664E4);
    cpu.pc = 0x283D8;
    16
}
/// $0664E4: btst.b #$1, $13(a6)
pub fn i_0664e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0664EA: bne.w $66504
pub fn i_0664ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66504; return 10; }
    8
}
/// $0664EE: movea.l #$ffffffff, a0
pub fn i_0664ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664F4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0664F4: lea.l $2c79a2.l, a0
pub fn i_0664f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x664FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C79A2;
    20
}
/// $0664FA: jsr $5dd56.l
pub fn i_0664fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66500;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66500);
    cpu.pc = 0x5DD56;
    16
}
/// $066500: bcc.w $66526
pub fn i_066500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66504;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66526; return 10; }
    8
}
/// $066504: move.w #$1022, d0
pub fn i_066504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66508;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066508: jsr $2352.l
pub fn i_066508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6650E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6650E);
    cpu.pc = 0x2352;
    16
}
/// $06650E: lea.l $77f6a.l, a1
pub fn i_06650e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66514;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $066514: jsr $4ae.l
pub fn i_066514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6651A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6651A);
    cpu.pc = 0x4AE;
    16
}
/// $06651A: jsr $5dd02.l
pub fn i_06651a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66520;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66520);
    cpu.pc = 0x5DD02;
    16
}
/// $066520: jmp $518.l
pub fn i_066520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66526;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $066526: rts
pub fn i_066526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66528;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_Shard_066528  ($066528, 158 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066528: move.w #$179, d1
pub fn i_066528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6652C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x179;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06652C: jsr $236e.l
pub fn i_06652c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66532);
    cpu.pc = 0x236E;
    16
}
/// $066532: lea.l $2c77e6.l, a0
pub fn i_066532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66538;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C77E6;
    20
}
/// $066538: jsr $28cd4.l
pub fn i_066538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6653E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6653E);
    cpu.pc = 0x28CD4;
    16
}
/// $06653E: jsr $267e2.l
pub fn i_06653e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66544;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66544);
    cpu.pc = 0x267E2;
    16
}
/// $066544: move.w #$700, $2a(a6)
pub fn i_066544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6654A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x700;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06654A: move.w #$ff80, $2e(a6)
pub fn i_06654a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66550;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066550: movea.l $c(a6), a0
pub fn i_066550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66554;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $066554: movea.l $50(a0), a0
pub fn i_066554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66558;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $066558: move.w $28(a0), d0
pub fn i_066558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6655C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06655C: cmpi.w #$0, d0
pub fn i_06655c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66560;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $066560: bne.w $66568
pub fn i_066560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66564;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66568; return 10; }
    8
}
/// $066564: move.w #$800, d0
pub fn i_066564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66568;
    cpu.ir = 0x303C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066568: asr.w #$1, d0
pub fn i_066568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6656A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06656A: move.w d0, $28(a6)
pub fn i_06656a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6656E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06656E: lea.l $66574(pc), a1
pub fn i_06656e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66572;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66574;
    16
}
/// $066572: move.l a1, (a6)
pub fn i_066572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66574;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066574: jsr $27cee.l
pub fn i_066574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6657A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6657A);
    cpu.pc = 0x27CEE;
    16
}
/// $06657A: jsr $27fac.l
pub fn i_06657a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66580;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66580);
    cpu.pc = 0x27FAC;
    16
}
/// $066580: bcs.w $665a2
pub fn i_066580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66584;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x665A2; return 10; }
    8
}
/// $066584: cmpi.b #$41, d7
pub fn i_066584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66588;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x41, Size::Byte, false);
    8
}
/// $066588: bne.w $665be
pub fn i_066588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6658C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x665BE; return 10; }
    8
}
/// $06658C: lea.l $788c0.l, a1
pub fn i_06658c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66592;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x788C0;
    20
}
/// $066592: jsr $4ae.l
pub fn i_066592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66598;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66598);
    cpu.pc = 0x4AE;
    16
}
/// $066598: jsr $5dd02.l
pub fn i_066598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6659E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6659E);
    cpu.pc = 0x5DD02;
    16
}
/// $06659E: bra.w $665be
pub fn i_06659e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x665BE;
    10
}
/// $0665A2: jsr $28d70.l
pub fn i_0665a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x665A8);
    cpu.pc = 0x28D70;
    16
}
/// $0665A8: movea.l #$ffffffff, a0
pub fn i_0665a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665AE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0665AE: lea.l $2c79a2.l, a0
pub fn i_0665ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C79A2;
    20
}
/// $0665B4: jsr $5dd56.l
pub fn i_0665b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x665BA);
    cpu.pc = 0x5DD56;
    16
}
/// $0665BA: bcc.w $665c4
pub fn i_0665ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x665C4; return 10; }
    8
}
/// $0665BE: jmp $518.l
pub fn i_0665be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0665C4: rts
pub fn i_0665c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_FlipByFrame_0665c6  ($0665C6, 28 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0665C6: move.b $106f28.l, d0
pub fn i_0665c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665CC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0665CC: btst.b #$0, d0
pub fn i_0665cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665D0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0665D0: bne.w $665e6
pub fn i_0665d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x665E6; return 10; }
    8
}
/// $0665D4: move.w $16(a6), d0
pub fn i_0665d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665D8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0665D8: move.w $18(a6), $16(a6)
pub fn i_0665d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665DE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0665DE: move.w d0, $18(a6)
pub fn i_0665de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665E2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskW_0665e2  ($0665E2, 6 B, kernel/task_setters_w.c)
/// $0665E2: move.w d0, $14(a6)
pub fn i_0665e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0665E6: rts
pub fn i_0665e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_WaveBob_0665e8  ($0665E8, 46 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0665E8: move.w $76(a6), d0
pub fn i_0665e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665EC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0665EC: sub.w $2a(a6), d0
pub fn i_0665ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665F0;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0665F0: asr.w #$3, d0
pub fn i_0665f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665F2;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0665F2: move.w d0, $2e(a6)
pub fn i_0665f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665F6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0665F6: jsr $27cee.l
pub fn i_0665f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x665FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x665FC);
    cpu.pc = 0x27CEE;
    16
}
/// $0665FC: move.w $2a(a6), d0
pub fn i_0665fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66600;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066600: sub.w d0, $76(a6)
pub fn i_066600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66604;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $066604: move.w $76(a6), d0
pub fn i_066604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66608;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066608: jsr $8b58e.l
pub fn i_066608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6660E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6660E);
    cpu.pc = 0x8B58E;
    16
}
/// $06660E: cmpi.w #$600, d0
pub fn i_06660e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66612;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x600, Size::Word, false);
    8
}
/// $066612: bgt.w $6661c
pub fn i_066612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66616;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6661C; return 10; }
    8
}
// ===== SetXN_066616  ($066616, 6 B, thunks/ccr_helpers.c)
/// $066616: ori.b #$11, ccr
pub fn i_066616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6661A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06661A: rts
pub fn i_06661a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6661C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06661c  ($06661C, 6 B, thunks/ccr_helpers.c)
/// $06661C: andi.b #$ee, ccr
pub fn i_06661c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66620;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $066620: rts
pub fn i_066620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66622;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_FaceTarget_066622  ($066622, 34 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066622: jsr $5e0d4.l
pub fn i_066622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66628;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66628);
    cpu.pc = 0x5E0D4;
    16
}
/// $066628: move.w $22(a6), d0
pub fn i_066628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6662C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06662C: sub.w $22(a0), d0
pub fn i_06662c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66630;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $066630: bmi.w $6663c
pub fn i_066630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66634;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x6663C; return 10; }
    8
}
/// $066634: move.b #$0, $3a(a6)
pub fn i_066634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6663A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06663A: rts
pub fn i_06663a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6663C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $06663C: move.b #$1, $3a(a6)
pub fn i_06663c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66642;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $066642: rts
pub fn i_066642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66644;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_TargetInRange_066644  ($066644, 48 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066644: jsr $5e0d4.l
pub fn i_066644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6664A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6664A);
    cpu.pc = 0x5E0D4;
    16
}
/// $06664A: move.w $22(a6), d0
pub fn i_06664a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6664E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06664E: sub.w $22(a0), d0
pub fn i_06664e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66652;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $066652: jsr $8b58e.l
pub fn i_066652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66658);
    cpu.pc = 0x8B58E;
    16
}
/// $066658: clr.w d2
pub fn i_066658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6665A;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06665A: move.b $9b(a6), d1
pub fn i_06665a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6665E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06665E: cmpi.b #$0, d1
pub fn i_06665e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66662;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $066662: beq.w $6666e
pub fn i_066662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66666;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6666E; return 10; }
    8
}
/// $066666: subq.b #$1, d1
pub fn i_066666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66668;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $066668: addi.w #$10, d2
pub fn i_066668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6666C;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06666C: bra.b $6665e
pub fn i_06666c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6666E;
    cpu.ir = 0x60F0;
    cpu.pc = 0x6665E;
    10
}
/// $06666E: cmp.w d0, d2
pub fn i_06666e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66670;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $066670: blt.w $6667a
pub fn i_066670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66674;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6667A; return 10; }
    8
}
// ===== SetXN_066674  ($066674, 6 B, thunks/ccr_helpers.c)
/// $066674: ori.b #$11, ccr
pub fn i_066674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66678;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $066678: rts
pub fn i_066678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6667A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06667a  ($06667A, 6 B, thunks/ccr_helpers.c)
/// $06667A: andi.b #$ee, ccr
pub fn i_06667a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6667E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06667E: rts
pub fn i_06667e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66680;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_SpawnMine_066680  ($066680, 10 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066680: lea.l $6637a(pc), a1
pub fn i_066680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66684;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6637A;
    16
}
/// $066684: jsr $4ae.l
pub fn i_066684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6668A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6668A);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_06668a  ($06668A, 8 B, thunks/jsr_abs_thunks.c)
/// $06668A: jsr $5dd02.l
pub fn i_06668a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66690;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66690);
    cpu.pc = 0x5DD02;
    16
}
/// $066690: rts
pub fn i_066690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66692;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FloatBarrel_HitCheck_066692  ($066692, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066692: movea.l $8(a6), a1
pub fn i_066692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66696;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $066696: move.b $10(a6), d0
pub fn i_066696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6669A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06669A: cmp.b $10(a1), d0
pub fn i_06669a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6669E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06669E: bcs.w $666a8
pub fn i_06669e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666A2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x666A8; return 10; }
    8
}
// ===== ClearXN_0666a2  ($0666A2, 6 B, thunks/ccr_helpers.c)
/// $0666A2: andi.b #$ee, ccr
pub fn i_0666a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666A6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0666A6: rts
pub fn i_0666a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0666a8  ($0666A8, 6 B, thunks/ccr_helpers.c)
/// $0666A8: ori.b #$11, ccr
pub fn i_0666a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666AC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0666AC: rts
pub fn i_0666ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_Spawner_Tmpl5E_0666ae  ($0666AE, 230 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0666AE: clr.b $80(a6)
pub fn i_0666ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666B2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0666B2: bra.w $666bc
pub fn i_0666b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x666BC;
    10
}
/// $0666BC: tst.b $98(a6)
pub fn i_0666bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666C0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0666C0: bne.w $666ca
pub fn i_0666c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x666CA; return 10; }
    8
}
/// $0666C4: move.b #$ff, $98(a6)
pub fn i_0666c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0666CA: tst.b $9b(a6)
pub fn i_0666ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666CE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0666CE: bne.w $666d8
pub fn i_0666ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x666D8; return 10; }
    8
}
/// $0666D2: move.b #$ff, $9b(a6)
pub fn i_0666d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0666D8: move.b $99(a6), d0
pub fn i_0666d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666DC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0666DC: andi.w #$ff, d0
pub fn i_0666dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0666E0: lsl.w #$5, d0
pub fn i_0666e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666E2;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0666E2: move.w d0, $7e(a6)
pub fn i_0666e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0666E6: clr.b $21(a6)
pub fn i_0666e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666EA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0666EA: clr.b $20(a6)
pub fn i_0666ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666EE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0666EE: lea.l $666f4(pc), a1
pub fn i_0666ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x666F4;
    16
}
/// $0666F2: move.l a1, (a6)
pub fn i_0666f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0666F4: jsr $2783a.l
pub fn i_0666f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x666FA);
    cpu.pc = 0x2783A;
    16
}
/// $0666FA: move.w $7e(a6), d0
pub fn i_0666fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x666FE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0666FE: cmp.w $106f50.l, d0
pub fn i_0666fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66704;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $066704: blt.w $6679c
pub fn i_066704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66708;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6679C; return 10; }
    8
}
/// $066708: tst.b $21(a6)
pub fn i_066708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6670C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06670C: bne.w $6676c
pub fn i_06670c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66710;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6676C; return 10; }
    8
}
/// $066710: tst.b $98(a6)
pub fn i_066710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66714;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $066714: beq.w $6679c
pub fn i_066714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66718;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6679C; return 10; }
    8
}
/// $066718: subq.w #$1, $70(a6)
pub fn i_066718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6671C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06671C: cmpi.w #$0, $70(a6)
pub fn i_06671c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66722;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $066722: bgt.w $6676c
pub fn i_066722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66726;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6676C; return 10; }
    8
}
/// $066726: lea.l $2b889a.l, a0
pub fn i_066726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6672C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B889A;
    20
}
/// $06672C: jsr $799de.l
pub fn i_06672c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66732;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66732);
    cpu.pc = 0x799DE;
    16
}
/// $066732: move.w d0, $70(a6)
pub fn i_066732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66736;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066736: lea.l $667c4(pc), a1
pub fn i_066736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6673A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x667C4;
    16
}
/// $06673A: jsr $4ae.l
pub fn i_06673a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66740);
    cpu.pc = 0x4AE;
    16
}
/// $066740: jsr $5dd02.l
pub fn i_066740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66746);
    cpu.pc = 0x5DD02;
    16
}
/// $066746: move.b $80(a6), $80(a0)
pub fn i_066746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6674C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x80);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06674C: move.b $9a(a6), $9a(a0)
pub fn i_06674c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66752;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $066752: move.b $9b(a6), $9b(a0)
pub fn i_066752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66758;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $066758: move.b $9c(a6), $9c(a0)
pub fn i_066758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6675E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06675E: cmpi.b #$ff, $98(a6)
pub fn i_06675e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66764;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $066764: beq.w $6676c
pub fn i_066764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66768;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6676C; return 10; }
    8
}
/// $066768: subq.b #$1, $98(a6)
pub fn i_066768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6676C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06676C: clr.b $21(a6)
pub fn i_06676c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66770;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $066770: cmpi.w #$10, $22(a6)
pub fn i_066770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66776;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $066776: bgt.w $66780
pub fn i_066776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6677A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66780; return 10; }
    8
}
/// $06677A: lea.l $667a4(pc), a1
pub fn i_06677a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6677E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x667A4;
    16
}
/// $06677E: move.l a1, (a6)
pub fn i_06677e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66780;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066780: movea.l #$ffffffff, a0
pub fn i_066780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66786;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $066786: lea.l $66dd0(pc), a0
pub fn i_066786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6678A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66DD0;
    16
}
/// $06678A: jsr $5dd5c.l
pub fn i_06678a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66790);
    cpu.pc = 0x5DD5C;
    16
}
/// $066790: bcc.w $6679a
pub fn i_066790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66794;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6679A; return 10; }
    8
}
// ===== SetTaskHandler_066794  ($066794, 8 B, kernel/task_handlers.c)
/// $066794: lea.l $667a4(pc), a1
pub fn i_066794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66798;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x667A4;
    16
}
/// $066798: move.l a1, (a6)
pub fn i_066798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6679A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06679A: rts
pub fn i_06679a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6679C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_06679c  ($06679C, 8 B, kernel/task_handlers.c)
/// $06679C: lea.l $667a4(pc), a1
pub fn i_06679c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x667A4;
    16
}
/// $0667A0: move.l a1, (a6)
pub fn i_0667a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0667A2: rts
pub fn i_0667a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_Spawner_Done_0667a4  ($0667A4, 32 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0667A4: move.b #$1, $20(a6)
pub fn i_0667a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0667AA: lea.l $667b0(pc), a1
pub fn i_0667aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x667B0;
    16
}
/// $0667AE: move.l a1, (a6)
pub fn i_0667ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0667B0: tst.b $21(a6)
pub fn i_0667b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667B4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0667B4: bne.w $667be
pub fn i_0667b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x667BE; return 10; }
    8
}
/// $0667B8: lea.l $66a86(pc), a1
pub fn i_0667b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A86;
    16
}
/// $0667BC: move.l a1, (a6)
pub fn i_0667bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0667BE: clr.b $21(a6)
pub fn i_0667be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667C2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0667C2: rts
pub fn i_0667c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_Init_0667c4  ($0667C4, 156 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0667C4: move.w #$183, d1
pub fn i_0667c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x183;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0667C8: jsr $236e.l
pub fn i_0667c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x667CE);
    cpu.pc = 0x236E;
    16
}
/// $0667CE: jsr $267e2.l
pub fn i_0667ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x667D4);
    cpu.pc = 0x267E2;
    16
}
/// $0667D4: move.w #$4000, d0
pub fn i_0667d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0667D8: jsr $28134.l
pub fn i_0667d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x667DE);
    cpu.pc = 0x28134;
    16
}
/// $0667DE: andi.w #$ffe3, $38(a6)
pub fn i_0667de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667E4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0667E4: ori.w #$14, $38(a6)
pub fn i_0667e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667EA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0667EA: move.b $9c(a6), d0
pub fn i_0667ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667EE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0667EE: andi.w #$ff, d0
pub fn i_0667ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667F2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0667F2: bne.w $66802
pub fn i_0667f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66802; return 10; }
    8
}
/// $0667F6: lea.l $2b891c.l, a0
pub fn i_0667f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x667FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B891C;
    20
}
/// $0667FC: jsr $799de.l
pub fn i_0667fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66802);
    cpu.pc = 0x799DE;
    16
}
/// $066802: move.w d0, $70(a6)
pub fn i_066802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66806;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066806: tst.b $80(a6)
pub fn i_066806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6680A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06680A: beq.w $66836
pub fn i_06680a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6680E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x66836; return 10; }
    8
}
/// $06680E: move.w #$a0, $22(a6)
pub fn i_06680e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66814;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066814: jsr $5e1ea.l
pub fn i_066814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6681A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6681A);
    cpu.pc = 0x5E1EA;
    16
}
/// $06681A: bcs.w $66836
pub fn i_06681a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6681E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x66836; return 10; }
    8
}
/// $06681E: move.w $22(a0), d0
pub fn i_06681e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66822;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066822: cmpi.w #$20, d0
pub fn i_066822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66826;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $066826: blt.w $66836
pub fn i_066826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6682A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x66836; return 10; }
    8
}
/// $06682A: cmpi.w #$120, d0
pub fn i_06682a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6682E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $06682E: bgt.w $66836
pub fn i_06682e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66832;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66836; return 10; }
    8
}
/// $066832: move.w d0, $22(a6)
pub fn i_066832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66836;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066836: lea.l $66dd8(pc), a0
pub fn i_066836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6683A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66DD8;
    16
}
/// $06683A: jsr $28cd4.l
pub fn i_06683a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66840);
    cpu.pc = 0x28CD4;
    16
}
/// $066840: lea.l $66846(pc), a1
pub fn i_066840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66844;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66846;
    16
}
/// $066844: move.l a1, (a6)
pub fn i_066844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66846;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066846: jsr $2783a.l
pub fn i_066846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6684C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6684C);
    cpu.pc = 0x2783A;
    16
}
/// $06684C: jsr $28d70.l
pub fn i_06684c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66852);
    cpu.pc = 0x28D70;
    16
}
/// $066852: bcc.w $6685c
pub fn i_066852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66856;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6685C; return 10; }
    8
}
/// $066856: lea.l $66860(pc), a1
pub fn i_066856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6685A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66860;
    16
}
/// $06685A: move.l a1, (a6)
pub fn i_06685a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6685C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06685C: bra.w $66a6c
pub fn i_06685c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66860;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A6C;
    10
}
// ===== Paratrooper_Land_066860  ($066860, 54 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066860: move.l #$66d7c, $48(a6)
pub fn i_066860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66868;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x66D7C;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $066868: lea.l $66e48(pc), a0
pub fn i_066868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6686C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66E48;
    16
}
/// $06686C: jsr $28cd4.l
pub fn i_06686c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66872;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66872);
    cpu.pc = 0x28CD4;
    16
}
/// $066872: lea.l $66878(pc), a1
pub fn i_066872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66876;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66878;
    16
}
/// $066876: move.l a1, (a6)
pub fn i_066876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66878;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066878: jsr $2783a.l
pub fn i_066878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6687E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6687E);
    cpu.pc = 0x2783A;
    16
}
/// $06687E: jsr $28d70.l
pub fn i_06687e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66884);
    cpu.pc = 0x28D70;
    16
}
/// $066884: jsr $66c12(pc)
pub fn i_066884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66888;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x66888);
    cpu.pc = 0x66C12;
    16
}
/// $066888: bcc.w $66892
pub fn i_066888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6688C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66892; return 10; }
    8
}
/// $06688C: lea.l $66896(pc), a1
pub fn i_06688c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66890;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66896;
    16
}
/// $066890: move.l a1, (a6)
pub fn i_066890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66892;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066892: bra.w $66a3c
pub fn i_066892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66896;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A3C;
    10
}
// ===== Paratrooper_PickWalk_066896  ($066896, 62 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066896: cmpi.b #$ff, $9b(a6)
pub fn i_066896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6689C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $06689C: beq.w $668a4
pub fn i_06689c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x668A4; return 10; }
    8
}
/// $0668A0: subq.b #$1, $9b(a6)
pub fn i_0668a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668A4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0668A4: tst.b $9a(a6)
pub fn i_0668a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668A8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0668A8: beq.w $668b4
pub fn i_0668a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x668B4; return 10; }
    8
}
/// $0668AC: jsr $66bca(pc)
pub fn i_0668ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x668B0);
    cpu.pc = 0x66BCA;
    16
}
/// $0668B0: bcc.w $6693a
pub fn i_0668b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6693A; return 10; }
    8
}
/// $0668B4: lea.l $2b8a60.l, a0
pub fn i_0668b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8A60;
    20
}
/// $0668BA: jsr $799de.l
pub fn i_0668ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x668C0);
    cpu.pc = 0x799DE;
    16
}
/// $0668C0: move.w d0, $74(a6)
pub fn i_0668c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668C4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0668C4: lea.l $2b89de.l, a0
pub fn i_0668c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B89DE;
    20
}
/// $0668CA: jsr $799de.l
pub fn i_0668ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x668D0);
    cpu.pc = 0x799DE;
    16
}
/// $0668D0: move.w d0, $72(a6)
pub fn i_0668d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668D4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Paratrooper_WalkA_0668d4  ($0668D4, 134 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0668D4: move.w $72(a6), $70(a6)
pub fn i_0668d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668DA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0668DA: lea.l $66e8a(pc), a0
pub fn i_0668da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668DE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66E8A;
    16
}
/// $0668DE: jsr $28cd4.l
pub fn i_0668de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x668E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0668E4: lea.l $668ea(pc), a1
pub fn i_0668e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x668EA;
    16
}
/// $0668E8: move.l a1, (a6)
pub fn i_0668e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0668EA: jsr $2783a.l
pub fn i_0668ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x668F0);
    cpu.pc = 0x2783A;
    16
}
/// $0668F0: jsr $28d70.l
pub fn i_0668f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x668F6);
    cpu.pc = 0x28D70;
    16
}
/// $0668F6: bcc.w $66904
pub fn i_0668f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66904; return 10; }
    8
}
/// $0668FA: lea.l $66e48(pc), a0
pub fn i_0668fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66E48;
    16
}
/// $0668FE: jsr $28cd4.l
pub fn i_0668fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66904);
    cpu.pc = 0x28CD4;
    16
}
/// $066904: jsr $66c12(pc)
pub fn i_066904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66908;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x66908);
    cpu.pc = 0x66C12;
    16
}
/// $066908: bcc.w $66936
pub fn i_066908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6690C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66936; return 10; }
    8
}
/// $06690C: lea.l $668d4(pc), a1
pub fn i_06690c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66910;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x668D4;
    16
}
/// $066910: move.l a1, (a6)
pub fn i_066910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66912;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066912: subq.w #$1, $74(a6)
pub fn i_066912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66916;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $066916: cmpi.w #$0, $74(a6)
pub fn i_066916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6691C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06691C: bgt.w $66936
pub fn i_06691c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66920;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66936; return 10; }
    8
}
/// $066920: lea.l $66860(pc), a1
pub fn i_066920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66924;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66860;
    16
}
/// $066924: move.l a1, (a6)
pub fn i_066924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66926;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066926: lea.l $2b891c.l, a0
pub fn i_066926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6692C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B891C;
    20
}
/// $06692C: jsr $799de.l
pub fn i_06692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66932);
    cpu.pc = 0x799DE;
    16
}
/// $066932: move.w d0, $70(a6)
pub fn i_066932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66936;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066936: bra.w $66a3c
pub fn i_066936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6693A;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A3C;
    10
}
/// $06693A: lea.l $2b8a60.l, a0
pub fn i_06693a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66940;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8A60;
    20
}
/// $066940: jsr $799de.l
pub fn i_066940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66946);
    cpu.pc = 0x799DE;
    16
}
/// $066946: move.w d0, $74(a6)
pub fn i_066946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6694A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06694A: lea.l $2b89de.l, a0
pub fn i_06694a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66950;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B89DE;
    20
}
/// $066950: jsr $799de.l
pub fn i_066950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66956);
    cpu.pc = 0x799DE;
    16
}
/// $066956: move.w d0, $72(a6)
pub fn i_066956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6695A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Paratrooper_WalkB_06695a  ($06695A, 102 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06695A: move.w $72(a6), $70(a6)
pub fn i_06695a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66960;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $066960: lea.l $66ec4(pc), a0
pub fn i_066960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66964;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66EC4;
    16
}
/// $066964: jsr $28cd4.l
pub fn i_066964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6696A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6696A);
    cpu.pc = 0x28CD4;
    16
}
/// $06696A: lea.l $66970(pc), a1
pub fn i_06696a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6696E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66970;
    16
}
/// $06696E: move.l a1, (a6)
pub fn i_06696e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66970;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066970: jsr $2783a.l
pub fn i_066970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66976);
    cpu.pc = 0x2783A;
    16
}
/// $066976: jsr $28d70.l
pub fn i_066976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6697C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6697C);
    cpu.pc = 0x28D70;
    16
}
/// $06697C: bcc.w $6698a
pub fn i_06697c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66980;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6698A; return 10; }
    8
}
/// $066980: lea.l $66e48(pc), a0
pub fn i_066980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66984;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66E48;
    16
}
/// $066984: jsr $28cd4.l
pub fn i_066984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6698A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6698A);
    cpu.pc = 0x28CD4;
    16
}
/// $06698A: jsr $66c12(pc)
pub fn i_06698a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6698E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6698E);
    cpu.pc = 0x66C12;
    16
}
/// $06698E: bcc.w $669bc
pub fn i_06698e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66992;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x669BC; return 10; }
    8
}
/// $066992: lea.l $6695a(pc), a1
pub fn i_066992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66996;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6695A;
    16
}
/// $066996: move.l a1, (a6)
pub fn i_066996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66998;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066998: subq.w #$1, $74(a6)
pub fn i_066998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6699C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06699C: cmpi.w #$0, $74(a6)
pub fn i_06699c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669A2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0669A2: bgt.w $669bc
pub fn i_0669a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669A6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x669BC; return 10; }
    8
}
/// $0669A6: lea.l $66860(pc), a1
pub fn i_0669a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66860;
    16
}
/// $0669AA: move.l a1, (a6)
pub fn i_0669aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0669AC: lea.l $2b891c.l, a0
pub fn i_0669ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B891C;
    20
}
/// $0669B2: jsr $799de.l
pub fn i_0669b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669B8);
    cpu.pc = 0x799DE;
    16
}
/// $0669B8: move.w d0, $70(a6)
pub fn i_0669b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0669BC: bra.w $66a3c
pub fn i_0669bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A3C;
    10
}
// ===== Paratrooper_Leave_0669c0  ($0669C0, 42 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0669C0: lea.l $66efe(pc), a0
pub fn i_0669c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669C4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66EFE;
    16
}
/// $0669C4: jsr $28cd4.l
pub fn i_0669c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0669CA: lea.l $669d0(pc), a1
pub fn i_0669ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x669D0;
    16
}
/// $0669CE: move.l a1, (a6)
pub fn i_0669ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0669D0: jsr $2783a.l
pub fn i_0669d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669D6);
    cpu.pc = 0x2783A;
    16
}
/// $0669D6: jsr $28d70.l
pub fn i_0669d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669DC);
    cpu.pc = 0x28D70;
    16
}
/// $0669DC: bcc.w $669e6
pub fn i_0669dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x669E6; return 10; }
    8
}
/// $0669E0: lea.l $66a86(pc), a1
pub fn i_0669e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A86;
    16
}
/// $0669E4: move.l a1, (a6)
pub fn i_0669e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0669E6: bra.w $66a5c
pub fn i_0669e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A5C;
    10
}
// ===== Paratrooper_Die_0669ea  ($0669EA, 148 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0669EA: jsr $13600.l
pub fn i_0669ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669F0);
    cpu.pc = 0x13600;
    16
}
/// $0669F0: move.w #$183, d1
pub fn i_0669f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x183;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0669F4: tst.b $10fd8f.l
pub fn i_0669f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669FA;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0669FA: bne.w $66a02
pub fn i_0669fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66A02; return 10; }
    8
}
/// $0669FE: move.w #$1d7, d1
pub fn i_0669fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066A02: jsr $236e.l
pub fn i_066a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A08);
    cpu.pc = 0x236E;
    16
}
/// $066A08: move.w #$1043, d0
pub fn i_066a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1043;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066A0C: jsr $2352.l
pub fn i_066a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A12);
    cpu.pc = 0x2352;
    16
}
/// $066A12: lea.l $66f6e(pc), a0
pub fn i_066a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A16;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66F6E;
    16
}
/// $066A16: jsr $28cd4.l
pub fn i_066a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A1C);
    cpu.pc = 0x28CD4;
    16
}
/// $066A1C: lea.l $66a22(pc), a1
pub fn i_066a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A22;
    16
}
/// $066A20: move.l a1, (a6)
pub fn i_066a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A22: jsr $2783a.l
pub fn i_066a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A28);
    cpu.pc = 0x2783A;
    16
}
/// $066A28: jsr $28d70.l
pub fn i_066a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A2E);
    cpu.pc = 0x28D70;
    16
}
/// $066A2E: bcc.w $66a38
pub fn i_066a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66A38; return 10; }
    8
}
/// $066A32: lea.l $66a86(pc), a1
pub fn i_066a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A86;
    16
}
/// $066A36: move.l a1, (a6)
pub fn i_066a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A38: bra.w $66a6c
pub fn i_066a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A6C;
    10
}
/// $066A3C: movea.l $c(a6), a0
pub fn i_066a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A40;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $066A40: tst.b $20(a0)
pub fn i_066a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A44;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $066A44: beq.w $66a4e
pub fn i_066a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x66A4E; return 10; }
    8
}
/// $066A48: lea.l $669c0(pc), a1
pub fn i_066a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x669C0;
    16
}
/// $066A4C: move.l a1, (a6)
pub fn i_066a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A4E: tst.b $9b(a6)
pub fn i_066a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A52;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $066A52: bne.w $66a5c
pub fn i_066a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66A5C; return 10; }
    8
}
/// $066A56: lea.l $669c0(pc), a1
pub fn i_066a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x669C0;
    16
}
/// $066A5A: move.l a1, (a6)
pub fn i_066a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A5C: jsr $2870a.l
pub fn i_066a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A62);
    cpu.pc = 0x2870A;
    16
}
/// $066A62: bcc.w $66a6c
pub fn i_066a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66A6C; return 10; }
    8
}
/// $066A66: lea.l $669ea(pc), a1
pub fn i_066a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x669EA;
    16
}
/// $066A6A: move.l a1, (a6)
pub fn i_066a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A6C: movea.l $c(a6), a0
pub fn i_066a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A70;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $066A70: addq.b #$1, $21(a0)
pub fn i_066a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A74;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $066A74: cmpi.w #$0, $22(a6)
pub fn i_066a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A7A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $066A7A: bgt.w $66a84
pub fn i_066a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A7E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66A84; return 10; }
    8
}
// ===== SetTaskHandler_066a7e  ($066A7E, 8 B, kernel/task_handlers.c)
/// $066A7E: lea.l $66a86(pc), a1
pub fn i_066a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A86;
    16
}
/// $066A82: move.l a1, (a6)
pub fn i_066a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A84: rts
pub fn i_066a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_066a86  ($066A86, 8 B, kernel/jmp_to_scheduler.c)
/// $066A86: jmp $518.l
pub fn i_066a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A8C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_066a8e  ($066A8E, 14 B, kernel/jsr5b6_jmp518.c)
/// $066A8E: jsr $5b6.l
pub fn i_066a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A94);
    cpu.pc = 0x5B6;
    16
}
/// $066A94: jmp $518.l
pub fn i_066a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A9A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Paratrooper_FragExplode_066a9c  ($066A9C, 28 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066A9C: move.w #$c000, d0
pub fn i_066a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AA0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066AA0: jsr $28134.l
pub fn i_066aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66AA6);
    cpu.pc = 0x28134;
    16
}
/// $066AA6: andi.w #$ffe3, $38(a6)
pub fn i_066aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AAC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066AAC: ori.w #$14, $38(a6)
pub fn i_066aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AB2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066AB2: jmp $77f6a.l
pub fn i_066ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Paratrooper_Frag_066ab8  ($066AB8, 172 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066AB8: lea.l $2b8ae2.l, a0
pub fn i_066ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ABE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8AE2;
    20
}
/// $066ABE: jsr $799de.l
pub fn i_066abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66AC4);
    cpu.pc = 0x799DE;
    16
}
/// $066AC4: move.b $76(a6), d3
pub fn i_066ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AC8;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $066AC8: andi.w #$ff, d3
pub fn i_066ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ACC;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $066ACC: add.w d3, d3
pub fn i_066acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ACE;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $066ACE: lea.l $2c07ac.l, a1
pub fn i_066ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AD4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $066AD4: lea.l $2c072c.l, a2
pub fn i_066ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ADA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $066ADA: move.w (a1, d3.w), d1
pub fn i_066ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ADE;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $066ADE: move.w (a2, d3.w), d2
pub fn i_066ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE2;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $066AE2: muls.w d0, d1
pub fn i_066ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE4;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $066AE4: muls.w d0, d2
pub fn i_066ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE6;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $066AE6: asr.l #$8, d1
pub fn i_066ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE8;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $066AE8: asr.l #$8, d2
pub fn i_066ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AEA;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $066AEA: move.w d1, $28(a6)
pub fn i_066aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AEE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066AEE: move.w d2, $2a(a6)
pub fn i_066aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AF2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066AF2: move.w #$d000, d0
pub fn i_066af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AF6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066AF6: jsr $28134.l
pub fn i_066af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66AFC);
    cpu.pc = 0x28134;
    16
}
/// $066AFC: andi.w #$ffe3, $38(a6)
pub fn i_066afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B02;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B02: ori.w #$14, $38(a6)
pub fn i_066b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B08;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B08: move.w #$17b, d1
pub fn i_066b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B0C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066B0C: jsr $236e.l
pub fn i_066b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B12);
    cpu.pc = 0x236E;
    16
}
/// $066B12: lea.l $66fee(pc), a0
pub fn i_066b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B16;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66FEE;
    16
}
/// $066B16: jsr $28cd4.l
pub fn i_066b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B1C);
    cpu.pc = 0x28CD4;
    16
}
/// $066B1C: lea.l $66b22(pc), a1
pub fn i_066b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66B22;
    16
}
/// $066B20: move.l a1, (a6)
pub fn i_066b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B22: jsr $27cee.l
pub fn i_066b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B28);
    cpu.pc = 0x27CEE;
    16
}
/// $066B28: jsr $28d70.l
pub fn i_066b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B2E);
    cpu.pc = 0x28D70;
    16
}
/// $066B2E: jsr $283d8.l
pub fn i_066b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B34);
    cpu.pc = 0x283D8;
    16
}
/// $066B34: btst.b #$1, $13(a6)
pub fn i_066b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B3A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $066B3A: beq.w $66b44
pub fn i_066b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x66B44; return 10; }
    8
}
/// $066B3E: lea.l $66a9c(pc), a1
pub fn i_066b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A9C;
    16
}
/// $066B42: move.l a1, (a6)
pub fn i_066b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B44: cmpi.w #$1c8, $24(a6)
pub fn i_066b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B4A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1C8, Size::Word, false);
    16
}
/// $066B4A: blt.w $66b54
pub fn i_066b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B4E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x66B54; return 10; }
    8
}
/// $066B4E: lea.l $66a9c(pc), a1
pub fn i_066b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A9C;
    16
}
/// $066B52: move.l a1, (a6)
pub fn i_066b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B54: movea.l #$ffffffff, a0
pub fn i_066b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B5A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $066B5A: jsr $5dd56.l
pub fn i_066b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B60);
    cpu.pc = 0x5DD56;
    16
}
/// $066B60: bcc.w $66b6a
pub fn i_066b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B64;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66B6A; return 10; }
    8
}
// ===== SetTaskHandler_066b64  ($066B64, 8 B, kernel/task_handlers.c)
/// $066B64: lea.l $66a8e(pc), a1
pub fn i_066b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A8E;
    16
}
/// $066B68: move.l a1, (a6)
pub fn i_066b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B6A: rts
pub fn i_066b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_Smoke_066b6c  ($066B6C, 86 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066B6C: move.w #$c000, d0
pub fn i_066b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B70;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066B70: jsr $28134.l
pub fn i_066b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B76);
    cpu.pc = 0x28134;
    16
}
/// $066B76: andi.w #$ffe3, $38(a6)
pub fn i_066b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B7C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B7C: ori.w #$14, $38(a6)
pub fn i_066b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B82;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B82: move.w #$4, d1
pub fn i_066b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B86;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066B86: jsr $236e.l
pub fn i_066b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B8C);
    cpu.pc = 0x236E;
    16
}
/// $066B8C: lea.l $67014(pc), a0
pub fn i_066b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x67014;
    16
}
/// $066B90: jsr $28cd4.l
pub fn i_066b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B96);
    cpu.pc = 0x28CD4;
    16
}
/// $066B96: lea.l $66b9c(pc), a1
pub fn i_066b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66B9C;
    16
}
/// $066B9A: move.l a1, (a6)
pub fn i_066b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B9C: jsr $2783a.l
pub fn i_066b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BA2);
    cpu.pc = 0x2783A;
    16
}
/// $066BA2: jsr $28d70.l
pub fn i_066ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BA8);
    cpu.pc = 0x28D70;
    16
}
/// $066BA8: bcc.w $66bb2
pub fn i_066ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66BB2; return 10; }
    8
}
/// $066BAC: lea.l $66a8e(pc), a1
pub fn i_066bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A8E;
    16
}
/// $066BB0: move.l a1, (a6)
pub fn i_066bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066BB2: movea.l #$ffffffff, a0
pub fn i_066bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BB8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $066BB8: jsr $5dd56.l
pub fn i_066bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BBE);
    cpu.pc = 0x5DD56;
    16
}
/// $066BBE: bcc.w $66bc8
pub fn i_066bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66BC8; return 10; }
    8
}
// ===== SetTaskHandler_066bc2  ($066BC2, 8 B, kernel/task_handlers.c)
/// $066BC2: lea.l $66a8e(pc), a1
pub fn i_066bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A8E;
    16
}
/// $066BC6: move.l a1, (a6)
pub fn i_066bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066BC8: rts
pub fn i_066bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_Approach_066bca  ($066BCA, 54 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066BCA: jsr $5e0d4.l
pub fn i_066bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BD0);
    cpu.pc = 0x5E0D4;
    16
}
/// $066BD0: move.w $22(a0), d0
pub fn i_066bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BD4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066BD4: sub.w $22(a6), d0
pub fn i_066bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BD8;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $066BD8: cmpi.w #$0, d0
pub fn i_066bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BDC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $066BDC: bgt.w $66be2
pub fn i_066bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BE0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66BE2; return 10; }
    8
}
/// $066BE0: neg.w d0
pub fn i_066be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BE2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $066BE2: cmpi.w #$20, d0
pub fn i_066be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BE6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $066BE6: blt.w $66c00
pub fn i_066be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BEA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x66C00; return 10; }
    8
}
/// $066BEA: subi.w #$18, $22(a6)
pub fn i_066bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BF0;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066BF0: jsr $5e5a8.l
pub fn i_066bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BF6);
    cpu.pc = 0x5E5A8;
    16
}
/// $066BF6: bcc.w $66c06
pub fn i_066bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66C06; return 10; }
    8
}
/// $066BFA: addi.w #$18, $22(a6)
pub fn i_066bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C00;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_066c00  ($066C00, 6 B, thunks/ccr_helpers.c)
/// $066C00: ori.b #$11, ccr
pub fn i_066c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C04;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $066C04: rts
pub fn i_066c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_ApproachFail_066c06  ($066C06, 6 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066C06: addi.w #$18, $22(a6)
pub fn i_066c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C0C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== ClearXN_066c0c  ($066C0C, 6 B, thunks/ccr_helpers.c)
/// $066C0C: andi.b #$ee, ccr
pub fn i_066c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C10;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $066C10: rts
pub fn i_066c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_TimerInBounds_066c12  ($066C12, 34 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066C12: subq.w #$1, $70(a6)
pub fn i_066c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C16;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $066C16: cmpi.w #$20, $22(a6)
pub fn i_066c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C1C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $066C1C: blt.w $66c3a
pub fn i_066c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C20;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x66C3A; return 10; }
    8
}
/// $066C20: cmpi.w #$120, $22(a6)
pub fn i_066c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C26;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $066C26: bgt.w $66c3a
pub fn i_066c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C2A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66C3A; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0651D8, i_0651d8),
    (0x0651DE, i_0651de),
    (0x0651E0, i_0651e0),
    (0x0651E6, i_0651e6),
    (0x0651EA, i_0651ea),
    (0x0651F0, i_0651f0),
    (0x0651F6, i_0651f6),
    (0x0651FA, i_0651fa),
    (0x0651FE, i_0651fe),
    (0x065204, i_065204),
    (0x06520A, i_06520a),
    (0x06520E, i_06520e),
    (0x065214, i_065214),
    (0x06521A, i_06521a),
    (0x065220, i_065220),
    (0x065224, i_065224),
    (0x065226, i_065226),
    (0x06522C, i_06522c),
    (0x065230, i_065230),
    (0x065234, i_065234),
    (0x065236, i_065236),
    (0x06523C, i_06523c),
    (0x065242, i_065242),
    (0x065246, i_065246),
    (0x06524A, i_06524a),
    (0x06524C, i_06524c),
    (0x065252, i_065252),
    (0x065258, i_065258),
    (0x06525E, i_06525e),
    (0x065264, i_065264),
    (0x06526A, i_06526a),
    (0x065270, i_065270),
    (0x065274, i_065274),
    (0x065278, i_065278),
    (0x06527A, i_06527a),
    (0x06527C, i_06527c),
    (0x065282, i_065282),
    (0x065288, i_065288),
    (0x06528E, i_06528e),
    (0x065292, i_065292),
    (0x065298, i_065298),
    (0x06529E, i_06529e),
    (0x0652A4, i_0652a4),
    (0x0652AA, i_0652aa),
    (0x0652B0, i_0652b0),
    (0x0652B6, i_0652b6),
    (0x0652BA, i_0652ba),
    (0x0652BC, i_0652bc),
    (0x0652C2, i_0652c2),
    (0x0652C8, i_0652c8),
    (0x0652CC, i_0652cc),
    (0x0652D2, i_0652d2),
    (0x0652D6, i_0652d6),
    (0x0652DA, i_0652da),
    (0x0652DC, i_0652dc),
    (0x0652E2, i_0652e2),
    (0x0652E8, i_0652e8),
    (0x0652EC, i_0652ec),
    (0x0652F0, i_0652f0),
    (0x0652F2, i_0652f2),
    (0x0652F4, i_0652f4),
    (0x0652FA, i_0652fa),
    (0x0652FE, i_0652fe),
    (0x065304, i_065304),
    (0x065308, i_065308),
    (0x06530A, i_06530a),
    (0x06530E, i_06530e),
    (0x065312, i_065312),
    (0x065318, i_065318),
    (0x06531E, i_06531e),
    (0x065324, i_065324),
    (0x065328, i_065328),
    (0x06532A, i_06532a),
    (0x065330, i_065330),
    (0x065336, i_065336),
    (0x06533A, i_06533a),
    (0x065340, i_065340),
    (0x065346, i_065346),
    (0x06534C, i_06534c),
    (0x065350, i_065350),
    (0x065354, i_065354),
    (0x065356, i_065356),
    (0x06535C, i_06535c),
    (0x065362, i_065362),
    (0x065368, i_065368),
    (0x06536E, i_06536e),
    (0x065372, i_065372),
    (0x065376, i_065376),
    (0x065378, i_065378),
    (0x06537A, i_06537a),
    (0x06537E, i_06537e),
    (0x065384, i_065384),
    (0x06538A, i_06538a),
    (0x06538E, i_06538e),
    (0x065390, i_065390),
    (0x065396, i_065396),
    (0x06539C, i_06539c),
    (0x0653A0, i_0653a0),
    (0x0653A4, i_0653a4),
    (0x0653A6, i_0653a6),
    (0x0653AC, i_0653ac),
    (0x0653B2, i_0653b2),
    (0x0653B6, i_0653b6),
    (0x0653BA, i_0653ba),
    (0x0653BC, i_0653bc),
    (0x0653BE, i_0653be),
    (0x0653C4, i_0653c4),
    (0x0653CA, i_0653ca),
    (0x0653CE, i_0653ce),
    (0x0653D4, i_0653d4),
    (0x0653DA, i_0653da),
    (0x0653E0, i_0653e0),
    (0x0653E4, i_0653e4),
    (0x0653E6, i_0653e6),
    (0x0653EC, i_0653ec),
    (0x0653F2, i_0653f2),
    (0x0653F6, i_0653f6),
    (0x0653FA, i_0653fa),
    (0x0653FC, i_0653fc),
    (0x065402, i_065402),
    (0x065408, i_065408),
    (0x06540C, i_06540c),
    (0x065410, i_065410),
    (0x065412, i_065412),
    (0x065414, i_065414),
    (0x065418, i_065418),
    (0x06541E, i_06541e),
    (0x065424, i_065424),
    (0x06542A, i_06542a),
    (0x065430, i_065430),
    (0x065436, i_065436),
    (0x06543A, i_06543a),
    (0x06543C, i_06543c),
    (0x065442, i_065442),
    (0x065448, i_065448),
    (0x06544C, i_06544c),
    (0x065450, i_065450),
    (0x065452, i_065452),
    (0x065458, i_065458),
    (0x06545E, i_06545e),
    (0x065462, i_065462),
    (0x065466, i_065466),
    (0x065468, i_065468),
    (0x06546A, i_06546a),
    (0x065470, i_065470),
    (0x065474, i_065474),
    (0x06547A, i_06547a),
    (0x065480, i_065480),
    (0x065486, i_065486),
    (0x06548A, i_06548a),
    (0x065490, i_065490),
    (0x065496, i_065496),
    (0x06549A, i_06549a),
    (0x06549E, i_06549e),
    (0x0654A4, i_0654a4),
    (0x0654A8, i_0654a8),
    (0x0654AA, i_0654aa),
    (0x0654AC, i_0654ac),
    (0x0654AE, i_0654ae),
    (0x0654B0, i_0654b0),
    (0x0654B6, i_0654b6),
    (0x0654BC, i_0654bc),
    (0x0654C2, i_0654c2),
    (0x0654C6, i_0654c6),
    (0x0654CC, i_0654cc),
    (0x0654D0, i_0654d0),
    (0x0654D2, i_0654d2),
    (0x0654D6, i_0654d6),
    (0x0654DC, i_0654dc),
    (0x0654E0, i_0654e0),
    (0x0654E4, i_0654e4),
    (0x0654E8, i_0654e8),
    (0x0654EC, i_0654ec),
    (0x0654F0, i_0654f0),
    (0x0654F2, i_0654f2),
    (0x0654F4, i_0654f4),
    (0x0654F8, i_0654f8),
    (0x0654FE, i_0654fe),
    (0x065504, i_065504),
    (0x065508, i_065508),
    (0x06550A, i_06550a),
    (0x065510, i_065510),
    (0x065514, i_065514),
    (0x065518, i_065518),
    (0x06551A, i_06551a),
    (0x06551E, i_06551e),
    (0x065522, i_065522),
    (0x065526, i_065526),
    (0x06552A, i_06552a),
    (0x065530, i_065530),
    (0x065536, i_065536),
    (0x06553A, i_06553a),
    (0x06553C, i_06553c),
    (0x065542, i_065542),
    (0x065546, i_065546),
    (0x06554A, i_06554a),
    (0x06554E, i_06554e),
    (0x065550, i_065550),
    (0x065554, i_065554),
    (0x065558, i_065558),
    (0x06555C, i_06555c),
    (0x065562, i_065562),
    (0x065566, i_065566),
    (0x06556A, i_06556a),
    (0x06556E, i_06556e),
    (0x065572, i_065572),
    (0x065576, i_065576),
    (0x06557A, i_06557a),
    (0x06557E, i_06557e),
    (0x065582, i_065582),
    (0x065586, i_065586),
    (0x065588, i_065588),
    (0x06558C, i_06558c),
    (0x065592, i_065592),
    (0x065598, i_065598),
    (0x06559C, i_06559c),
    (0x06559E, i_06559e),
    (0x0655A4, i_0655a4),
    (0x0655AA, i_0655aa),
    (0x0655AE, i_0655ae),
    (0x0655B2, i_0655b2),
    (0x0655B6, i_0655b6),
    (0x0655B8, i_0655b8),
    (0x0655BC, i_0655bc),
    (0x0655C2, i_0655c2),
    (0x0655C6, i_0655c6),
    (0x0655CA, i_0655ca),
    (0x0655CC, i_0655cc),
    (0x0655D2, i_0655d2),
    (0x0655D6, i_0655d6),
    (0x0655DA, i_0655da),
    (0x0655DC, i_0655dc),
    (0x0655E2, i_0655e2),
    (0x0655E8, i_0655e8),
    (0x0655EC, i_0655ec),
    (0x0655F0, i_0655f0),
    (0x0655F2, i_0655f2),
    (0x0655F4, i_0655f4),
    (0x0655FA, i_0655fa),
    (0x065600, i_065600),
    (0x065604, i_065604),
    (0x06560A, i_06560a),
    (0x065610, i_065610),
    (0x065640, i_065640),
    (0x065644, i_065644),
    (0x06564A, i_06564a),
    (0x06564E, i_06564e),
    (0x065650, i_065650),
    (0x065656, i_065656),
    (0x06565C, i_06565c),
    (0x065660, i_065660),
    (0x065664, i_065664),
    (0x065666, i_065666),
    (0x065668, i_065668),
    (0x06566C, i_06566c),
    (0x065672, i_065672),
    (0x065676, i_065676),
    (0x06567A, i_06567a),
    (0x065680, i_065680),
    (0x065686, i_065686),
    (0x06568C, i_06568c),
    (0x065692, i_065692),
    (0x065696, i_065696),
    (0x06569C, i_06569c),
    (0x0656A0, i_0656a0),
    (0x0656A4, i_0656a4),
    (0x0656A8, i_0656a8),
    (0x0656AE, i_0656ae),
    (0x0656B4, i_0656b4),
    (0x0656BA, i_0656ba),
    (0x0656BE, i_0656be),
    (0x0656C4, i_0656c4),
    (0x0656CA, i_0656ca),
    (0x0656D0, i_0656d0),
    (0x0656D4, i_0656d4),
    (0x0656D6, i_0656d6),
    (0x0656DA, i_0656da),
    (0x0656E0, i_0656e0),
    (0x0656E4, i_0656e4),
    (0x0656E8, i_0656e8),
    (0x0656EC, i_0656ec),
    (0x0656F0, i_0656f0),
    (0x0656F4, i_0656f4),
    (0x0656FA, i_0656fa),
    (0x0656FE, i_0656fe),
    (0x065702, i_065702),
    (0x065704, i_065704),
    (0x06570A, i_06570a),
    (0x06570E, i_06570e),
    (0x065710, i_065710),
    (0x065712, i_065712),
    (0x065718, i_065718),
    (0x06571A, i_06571a),
    (0x06571C, i_06571c),
    (0x065722, i_065722),
    (0x065728, i_065728),
    (0x06572E, i_06572e),
    (0x065734, i_065734),
    (0x065738, i_065738),
    (0x06573C, i_06573c),
    (0x06573E, i_06573e),
    (0x065744, i_065744),
    (0x065748, i_065748),
    (0x06574C, i_06574c),
    (0x06574E, i_06574e),
    (0x065754, i_065754),
    (0x06575A, i_06575a),
    (0x065760, i_065760),
    (0x065764, i_065764),
    (0x065768, i_065768),
    (0x06576A, i_06576a),
    (0x06576C, i_06576c),
    (0x065770, i_065770),
    (0x065776, i_065776),
    (0x06577A, i_06577a),
    (0x065780, i_065780),
    (0x065784, i_065784),
    (0x065788, i_065788),
    (0x06578C, i_06578c),
    (0x065790, i_065790),
    (0x065796, i_065796),
    (0x06579A, i_06579a),
    (0x0657A0, i_0657a0),
    (0x0657A4, i_0657a4),
    (0x0657A6, i_0657a6),
    (0x0657AC, i_0657ac),
    (0x0657B2, i_0657b2),
    (0x0657B8, i_0657b8),
    (0x0657BE, i_0657be),
    (0x0657C2, i_0657c2),
    (0x0657C6, i_0657c6),
    (0x0657CA, i_0657ca),
    (0x0657CE, i_0657ce),
    (0x0657D4, i_0657d4),
    (0x0657DA, i_0657da),
    (0x0657E0, i_0657e0),
    (0x0657E6, i_0657e6),
    (0x0657EC, i_0657ec),
    (0x0657F0, i_0657f0),
    (0x0657F6, i_0657f6),
    (0x0657FA, i_0657fa),
    (0x0657FE, i_0657fe),
    (0x065804, i_065804),
    (0x065808, i_065808),
    (0x06580C, i_06580c),
    (0x06580E, i_06580e),
    (0x065814, i_065814),
    (0x065818, i_065818),
    (0x06581C, i_06581c),
    (0x065820, i_065820),
    (0x065824, i_065824),
    (0x065828, i_065828),
    (0x06582C, i_06582c),
    (0x065832, i_065832),
    (0x065834, i_065834),
    (0x065838, i_065838),
    (0x06583C, i_06583c),
    (0x065840, i_065840),
    (0x065844, i_065844),
    (0x065848, i_065848),
    (0x06584E, i_06584e),
    (0x065852, i_065852),
    (0x065856, i_065856),
    (0x06585A, i_06585a),
    (0x065860, i_065860),
    (0x065864, i_065864),
    (0x065866, i_065866),
    (0x06586A, i_06586a),
    (0x06586C, i_06586c),
    (0x065870, i_065870),
    (0x065876, i_065876),
    (0x06587A, i_06587a),
    (0x06587E, i_06587e),
    (0x065882, i_065882),
    (0x065886, i_065886),
    (0x065888, i_065888),
    (0x06588C, i_06588c),
    (0x065890, i_065890),
    (0x065894, i_065894),
    (0x065898, i_065898),
    (0x06589C, i_06589c),
    (0x0658A0, i_0658a0),
    (0x0658A4, i_0658a4),
    (0x0658A8, i_0658a8),
    (0x0658AC, i_0658ac),
    (0x0658B0, i_0658b0),
    (0x0658B4, i_0658b4),
    (0x0658B8, i_0658b8),
    (0x0658BC, i_0658bc),
    (0x0658BE, i_0658be),
    (0x0658C2, i_0658c2),
    (0x0658C6, i_0658c6),
    (0x0658EA, i_0658ea),
    (0x0658F0, i_0658f0),
    (0x0658F4, i_0658f4),
    (0x0658F6, i_0658f6),
    (0x0658FA, i_0658fa),
    (0x0658FE, i_0658fe),
    (0x065902, i_065902),
    (0x065906, i_065906),
    (0x065908, i_065908),
    (0x06590E, i_06590e),
    (0x065914, i_065914),
    (0x065918, i_065918),
    (0x06591C, i_06591c),
    (0x065920, i_065920),
    (0x065924, i_065924),
    (0x065928, i_065928),
    (0x06592C, i_06592c),
    (0x065930, i_065930),
    (0x065934, i_065934),
    (0x065938, i_065938),
    (0x06593E, i_06593e),
    (0x065944, i_065944),
    (0x06594A, i_06594a),
    (0x06594E, i_06594e),
    (0x065954, i_065954),
    (0x06595A, i_06595a),
    (0x065960, i_065960),
    (0x065964, i_065964),
    (0x06596A, i_06596a),
    (0x06596E, i_06596e),
    (0x065974, i_065974),
    (0x065978, i_065978),
    (0x06597E, i_06597e),
    (0x065982, i_065982),
    (0x065988, i_065988),
    (0x06598C, i_06598c),
    (0x065992, i_065992),
    (0x065998, i_065998),
    (0x06599C, i_06599c),
    (0x0659A0, i_0659a0),
    (0x0659A6, i_0659a6),
    (0x0659AA, i_0659aa),
    (0x0659B0, i_0659b0),
    (0x0659B6, i_0659b6),
    (0x0659BC, i_0659bc),
    (0x0659C0, i_0659c0),
    (0x0659C4, i_0659c4),
    (0x0659CA, i_0659ca),
    (0x0659CE, i_0659ce),
    (0x0659D4, i_0659d4),
    (0x0659DA, i_0659da),
    (0x0659E0, i_0659e0),
    (0x0659E4, i_0659e4),
    (0x0659EA, i_0659ea),
    (0x0659F0, i_0659f0),
    (0x0659F6, i_0659f6),
    (0x0659FC, i_0659fc),
    (0x065A02, i_065a02),
    (0x065A06, i_065a06),
    (0x065A08, i_065a08),
    (0x065A10, i_065a10),
    (0x065A14, i_065a14),
    (0x065A18, i_065a18),
    (0x065A1C, i_065a1c),
    (0x065A20, i_065a20),
    (0x065A22, i_065a22),
    (0x065A28, i_065a28),
    (0x065A2C, i_065a2c),
    (0x065A30, i_065a30),
    (0x065A32, i_065a32),
    (0x065A38, i_065a38),
    (0x065A3E, i_065a3e),
    (0x065A42, i_065a42),
    (0x065A46, i_065a46),
    (0x065A48, i_065a48),
    (0x065A4E, i_065a4e),
    (0x065A54, i_065a54),
    (0x065A5A, i_065a5a),
    (0x065A5E, i_065a5e),
    (0x065A62, i_065a62),
    (0x065A64, i_065a64),
    (0x065A6A, i_065a6a),
    (0x065A70, i_065a70),
    (0x065A76, i_065a76),
    (0x065A7A, i_065a7a),
    (0x065A7E, i_065a7e),
    (0x065A80, i_065a80),
    (0x065A86, i_065a86),
    (0x065A8C, i_065a8c),
    (0x065A92, i_065a92),
    (0x065A98, i_065a98),
    (0x065A9C, i_065a9c),
    (0x065AA0, i_065aa0),
    (0x065AA2, i_065aa2),
    (0x065AA4, i_065aa4),
    (0x065AA8, i_065aa8),
    (0x065AAE, i_065aae),
    (0x065AB4, i_065ab4),
    (0x065ABA, i_065aba),
    (0x065AC0, i_065ac0),
    (0x065AC4, i_065ac4),
    (0x065AC6, i_065ac6),
    (0x065ACC, i_065acc),
    (0x065AD2, i_065ad2),
    (0x065AD6, i_065ad6),
    (0x065ADA, i_065ada),
    (0x065ADC, i_065adc),
    (0x065AE2, i_065ae2),
    (0x065AE8, i_065ae8),
    (0x065AEC, i_065aec),
    (0x065AF0, i_065af0),
    (0x065AF2, i_065af2),
    (0x065AF4, i_065af4),
    (0x065AFA, i_065afa),
    (0x065AFE, i_065afe),
    (0x065B04, i_065b04),
    (0x065B08, i_065b08),
    (0x065B0C, i_065b0c),
    (0x065B10, i_065b10),
    (0x065B16, i_065b16),
    (0x065B1C, i_065b1c),
    (0x065B20, i_065b20),
    (0x065B26, i_065b26),
    (0x065B2A, i_065b2a),
    (0x065B2E, i_065b2e),
    (0x065B30, i_065b30),
    (0x065B34, i_065b34),
    (0x065B36, i_065b36),
    (0x065B3C, i_065b3c),
    (0x065B40, i_065b40),
    (0x065B44, i_065b44),
    (0x065B48, i_065b48),
    (0x065B4C, i_065b4c),
    (0x065B4E, i_065b4e),
    (0x065B50, i_065b50),
    (0x065B54, i_065b54),
    (0x065B56, i_065b56),
    (0x065B5C, i_065b5c),
    (0x065B62, i_065b62),
    (0x065B66, i_065b66),
    (0x065B6C, i_065b6c),
    (0x065B6E, i_065b6e),
    (0x065B72, i_065b72),
    (0x065B76, i_065b76),
    (0x065B7A, i_065b7a),
    (0x065B7E, i_065b7e),
    (0x065B82, i_065b82),
    (0x065B86, i_065b86),
    (0x065B8C, i_065b8c),
    (0x065B90, i_065b90),
    (0x065B94, i_065b94),
    (0x065B98, i_065b98),
    (0x065B9E, i_065b9e),
    (0x065BA2, i_065ba2),
    (0x065BA4, i_065ba4),
    (0x065BAA, i_065baa),
    (0x065BAE, i_065bae),
    (0x065BB0, i_065bb0),
    (0x065BB6, i_065bb6),
    (0x065BBC, i_065bbc),
    (0x065BC2, i_065bc2),
    (0x065BC6, i_065bc6),
    (0x065BCA, i_065bca),
    (0x065BCE, i_065bce),
    (0x065BD2, i_065bd2),
    (0x065BD8, i_065bd8),
    (0x065BDC, i_065bdc),
    (0x065BE0, i_065be0),
    (0x065BE6, i_065be6),
    (0x065BE8, i_065be8),
    (0x065BEC, i_065bec),
    (0x065BF2, i_065bf2),
    (0x065BF6, i_065bf6),
    (0x065BFC, i_065bfc),
    (0x065C00, i_065c00),
    (0x065C02, i_065c02),
    (0x065C06, i_065c06),
    (0x065C0C, i_065c0c),
    (0x065C10, i_065c10),
    (0x065C14, i_065c14),
    (0x065C18, i_065c18),
    (0x065C1C, i_065c1c),
    (0x065C20, i_065c20),
    (0x065C24, i_065c24),
    (0x065C28, i_065c28),
    (0x065C2A, i_065c2a),
    (0x065C2E, i_065c2e),
    (0x065C30, i_065c30),
    (0x065C34, i_065c34),
    (0x065C3A, i_065c3a),
    (0x065C3E, i_065c3e),
    (0x065C42, i_065c42),
    (0x065C46, i_065c46),
    (0x065C4A, i_065c4a),
    (0x065C4E, i_065c4e),
    (0x065C52, i_065c52),
    (0x065C56, i_065c56),
    (0x065C5A, i_065c5a),
    (0x065C5E, i_065c5e),
    (0x065C62, i_065c62),
    (0x065C64, i_065c64),
    (0x065C68, i_065c68),
    (0x065C6A, i_065c6a),
    (0x065C70, i_065c70),
    (0x065C76, i_065c76),
    (0x065C7A, i_065c7a),
    (0x065C7E, i_065c7e),
    (0x065C82, i_065c82),
    (0x065C84, i_065c84),
    (0x065C86, i_065c86),
    (0x065C88, i_065c88),
    (0x065C8A, i_065c8a),
    (0x065C8E, i_065c8e),
    (0x065C92, i_065c92),
    (0x065C94, i_065c94),
    (0x065C96, i_065c96),
    (0x065C9C, i_065c9c),
    (0x065CA0, i_065ca0),
    (0x065CA4, i_065ca4),
    (0x065CA8, i_065ca8),
    (0x065CAE, i_065cae),
    (0x065CB4, i_065cb4),
    (0x065CB8, i_065cb8),
    (0x065CBA, i_065cba),
    (0x065CBE, i_065cbe),
    (0x065CC2, i_065cc2),
    (0x065CC6, i_065cc6),
    (0x065CCA, i_065cca),
    (0x065CCE, i_065cce),
    (0x065CD2, i_065cd2),
    (0x065CD6, i_065cd6),
    (0x065CDA, i_065cda),
    (0x065CDE, i_065cde),
    (0x065CE4, i_065ce4),
    (0x065CE6, i_065ce6),
    (0x065CE8, i_065ce8),
    (0x065CEC, i_065cec),
    (0x065CF2, i_065cf2),
    (0x065CF8, i_065cf8),
    (0x065CFC, i_065cfc),
    (0x065CFE, i_065cfe),
    (0x065D02, i_065d02),
    (0x065D04, i_065d04),
    (0x065D0A, i_065d0a),
    (0x065D10, i_065d10),
    (0x065D14, i_065d14),
    (0x065D18, i_065d18),
    (0x065D1A, i_065d1a),
    (0x065D1C, i_065d1c),
    (0x065D1E, i_065d1e),
    (0x065D20, i_065d20),
    (0x065D22, i_065d22),
    (0x065D24, i_065d24),
    (0x065D26, i_065d26),
    (0x065D28, i_065d28),
    (0x065D2C, i_065d2c),
    (0x065D30, i_065d30),
    (0x065D32, i_065d32),
    (0x065D38, i_065d38),
    (0x065D3C, i_065d3c),
    (0x065D3E, i_065d3e),
    (0x065D42, i_065d42),
    (0x065D44, i_065d44),
    (0x065D48, i_065d48),
    (0x065D4A, i_065d4a),
    (0x065D50, i_065d50),
    (0x065D54, i_065d54),
    (0x065D5A, i_065d5a),
    (0x065D60, i_065d60),
    (0x065D64, i_065d64),
    (0x065D6A, i_065d6a),
    (0x065D70, i_065d70),
    (0x065D72, i_065d72),
    (0x065D76, i_065d76),
    (0x065D78, i_065d78),
    (0x065D7A, i_065d7a),
    (0x065D80, i_065d80),
    (0x065D84, i_065d84),
    (0x065D8A, i_065d8a),
    (0x065D8C, i_065d8c),
    (0x065D92, i_065d92),
    (0x065D94, i_065d94),
    (0x065D98, i_065d98),
    (0x065D9A, i_065d9a),
    (0x065D9E, i_065d9e),
    (0x065DA2, i_065da2),
    (0x065DA4, i_065da4),
    (0x065DA6, i_065da6),
    (0x065DAA, i_065daa),
    (0x065DAC, i_065dac),
    (0x065DAE, i_065dae),
    (0x065DB2, i_065db2),
    (0x065DB4, i_065db4),
    (0x065DB8, i_065db8),
    (0x065DBE, i_065dbe),
    (0x065DC2, i_065dc2),
    (0x065DC6, i_065dc6),
    (0x065DCA, i_065dca),
    (0x065DCE, i_065dce),
    (0x065DD4, i_065dd4),
    (0x065DD6, i_065dd6),
    (0x065DDA, i_065dda),
    (0x065DDC, i_065ddc),
    (0x065DE0, i_065de0),
    (0x065DE6, i_065de6),
    (0x065DEC, i_065dec),
    (0x065DF2, i_065df2),
    (0x065DF8, i_065df8),
    (0x065DFE, i_065dfe),
    (0x065E04, i_065e04),
    (0x065E08, i_065e08),
    (0x065E0C, i_065e0c),
    (0x065E12, i_065e12),
    (0x065E18, i_065e18),
    (0x065E1C, i_065e1c),
    (0x065E22, i_065e22),
    (0x065E28, i_065e28),
    (0x065E2E, i_065e2e),
    (0x065E30, i_065e30),
    (0x065E34, i_065e34),
    (0x065E3A, i_065e3a),
    (0x065E40, i_065e40),
    (0x065E44, i_065e44),
    (0x065E4A, i_065e4a),
    (0x065E50, i_065e50),
    (0x065E54, i_065e54),
    (0x065E58, i_065e58),
    (0x065E5E, i_065e5e),
    (0x065E64, i_065e64),
    (0x065E68, i_065e68),
    (0x065E6C, i_065e6c),
    (0x065E6E, i_065e6e),
    (0x065E74, i_065e74),
    (0x065E78, i_065e78),
    (0x065E7E, i_065e7e),
    (0x065E82, i_065e82),
    (0x065E84, i_065e84),
    (0x065E88, i_065e88),
    (0x065E8E, i_065e8e),
    (0x065E94, i_065e94),
    (0x065E98, i_065e98),
    (0x065E9E, i_065e9e),
    (0x065EA2, i_065ea2),
    (0x065EA4, i_065ea4),
    (0x065EAA, i_065eaa),
    (0x065EB0, i_065eb0),
    (0x065EB6, i_065eb6),
    (0x065EBA, i_065eba),
    (0x065EBC, i_065ebc),
    (0x065EC0, i_065ec0),
    (0x065EC6, i_065ec6),
    (0x065ECC, i_065ecc),
    (0x065ED0, i_065ed0),
    (0x065ED6, i_065ed6),
    (0x065EDC, i_065edc),
    (0x065EE2, i_065ee2),
    (0x065EE6, i_065ee6),
    (0x065EE8, i_065ee8),
    (0x065EEE, i_065eee),
    (0x065EF2, i_065ef2),
    (0x065EF4, i_065ef4),
    (0x065EFA, i_065efa),
    (0x065EFE, i_065efe),
    (0x065F00, i_065f00),
    (0x065F06, i_065f06),
    (0x065F0A, i_065f0a),
    (0x065F0C, i_065f0c),
    (0x065F12, i_065f12),
    (0x065F16, i_065f16),
    (0x065F18, i_065f18),
    (0x065F1E, i_065f1e),
    (0x065F22, i_065f22),
    (0x065F24, i_065f24),
    (0x065F28, i_065f28),
    (0x065F2C, i_065f2c),
    (0x065F30, i_065f30),
    (0x065F34, i_065f34),
    (0x065F38, i_065f38),
    (0x065F3A, i_065f3a),
    (0x065F3E, i_065f3e),
    (0x065F40, i_065f40),
    (0x065F44, i_065f44),
    (0x065F4A, i_065f4a),
    (0x065F50, i_065f50),
    (0x065F56, i_065f56),
    (0x065F5C, i_065f5c),
    (0x065F62, i_065f62),
    (0x065F68, i_065f68),
    (0x065F6E, i_065f6e),
    (0x065F74, i_065f74),
    (0x065F78, i_065f78),
    (0x065F7A, i_065f7a),
    (0x065F80, i_065f80),
    (0x065F86, i_065f86),
    (0x065F8A, i_065f8a),
    (0x065F8E, i_065f8e),
    (0x065F92, i_065f92),
    (0x065F96, i_065f96),
    (0x065F9A, i_065f9a),
    (0x065F9C, i_065f9c),
    (0x065FA2, i_065fa2),
    (0x065FA8, i_065fa8),
    (0x065FAE, i_065fae),
    (0x065FB2, i_065fb2),
    (0x065FB8, i_065fb8),
    (0x065FBA, i_065fba),
    (0x065FC0, i_065fc0),
    (0x065FC6, i_065fc6),
    (0x065FCC, i_065fcc),
    (0x065FD0, i_065fd0),
    (0x065FD2, i_065fd2),
    (0x065FD8, i_065fd8),
    (0x065FDE, i_065fde),
    (0x065FE2, i_065fe2),
    (0x065FE6, i_065fe6),
    (0x065FEA, i_065fea),
    (0x065FEE, i_065fee),
    (0x065FF0, i_065ff0),
    (0x065FF6, i_065ff6),
    (0x065FFC, i_065ffc),
    (0x066000, i_066000),
    (0x066002, i_066002),
    (0x066006, i_066006),
    (0x06600C, i_06600c),
    (0x06600E, i_06600e),
    (0x066014, i_066014),
    (0x06601A, i_06601a),
    (0x066020, i_066020),
    (0x066026, i_066026),
    (0x06602A, i_06602a),
    (0x06602E, i_06602e),
    (0x066032, i_066032),
    (0x066036, i_066036),
    (0x066038, i_066038),
    (0x06603E, i_06603e),
    (0x066040, i_066040),
    (0x066046, i_066046),
    (0x06604C, i_06604c),
    (0x066052, i_066052),
    (0x066058, i_066058),
    (0x06605E, i_06605e),
    (0x066062, i_066062),
    (0x066068, i_066068),
    (0x06606C, i_06606c),
    (0x06606E, i_06606e),
    (0x066074, i_066074),
    (0x06607A, i_06607a),
    (0x06607C, i_06607c),
    (0x066080, i_066080),
    (0x066084, i_066084),
    (0x066088, i_066088),
    (0x06608C, i_06608c),
    (0x06608E, i_06608e),
    (0x066094, i_066094),
    (0x066098, i_066098),
    (0x06609C, i_06609c),
    (0x0660A2, i_0660a2),
    (0x0660A8, i_0660a8),
    (0x0660AC, i_0660ac),
    (0x0660AE, i_0660ae),
    (0x0660B4, i_0660b4),
    (0x0660BA, i_0660ba),
    (0x0660C0, i_0660c0),
    (0x0660C4, i_0660c4),
    (0x0660CA, i_0660ca),
    (0x0660CC, i_0660cc),
    (0x0660D0, i_0660d0),
    (0x0660D2, i_0660d2),
    (0x0660D6, i_0660d6),
    (0x0660DA, i_0660da),
    (0x0660E0, i_0660e0),
    (0x0660E6, i_0660e6),
    (0x0660EA, i_0660ea),
    (0x0660EC, i_0660ec),
    (0x0660F2, i_0660f2),
    (0x0660F8, i_0660f8),
    (0x0660FE, i_0660fe),
    (0x066104, i_066104),
    (0x066108, i_066108),
    (0x06610E, i_06610e),
    (0x066114, i_066114),
    (0x066118, i_066118),
    (0x06611C, i_06611c),
    (0x06611E, i_06611e),
    (0x066124, i_066124),
    (0x066128, i_066128),
    (0x06612C, i_06612c),
    (0x066130, i_066130),
    (0x066134, i_066134),
    (0x066136, i_066136),
    (0x06613C, i_06613c),
    (0x066140, i_066140),
    (0x066144, i_066144),
    (0x06614A, i_06614a),
    (0x066150, i_066150),
    (0x066154, i_066154),
    (0x066156, i_066156),
    (0x06615C, i_06615c),
    (0x066162, i_066162),
    (0x066168, i_066168),
    (0x06616C, i_06616c),
    (0x066172, i_066172),
    (0x066174, i_066174),
    (0x06617A, i_06617a),
    (0x066180, i_066180),
    (0x066184, i_066184),
    (0x066186, i_066186),
    (0x06618C, i_06618c),
    (0x066192, i_066192),
    (0x066198, i_066198),
    (0x06619C, i_06619c),
    (0x0661A0, i_0661a0),
    (0x0661A4, i_0661a4),
    (0x0661A8, i_0661a8),
    (0x0661AA, i_0661aa),
    (0x0661B0, i_0661b0),
    (0x0661B4, i_0661b4),
    (0x0661B8, i_0661b8),
    (0x0661BA, i_0661ba),
    (0x0661C0, i_0661c0),
    (0x0661C6, i_0661c6),
    (0x0661CC, i_0661cc),
    (0x0661D0, i_0661d0),
    (0x0661D6, i_0661d6),
    (0x0661D8, i_0661d8),
    (0x0661DE, i_0661de),
    (0x0661E4, i_0661e4),
    (0x0661EA, i_0661ea),
    (0x0661EE, i_0661ee),
    (0x0661F2, i_0661f2),
    (0x0661F8, i_0661f8),
    (0x0661FE, i_0661fe),
    (0x066204, i_066204),
    (0x06620A, i_06620a),
    (0x06620E, i_06620e),
    (0x066214, i_066214),
    (0x066218, i_066218),
    (0x06621A, i_06621a),
    (0x066220, i_066220),
    (0x066226, i_066226),
    (0x066228, i_066228),
    (0x06622C, i_06622c),
    (0x066232, i_066232),
    (0x066238, i_066238),
    (0x06623E, i_06623e),
    (0x066242, i_066242),
    (0x066246, i_066246),
    (0x066248, i_066248),
    (0x06624C, i_06624c),
    (0x066252, i_066252),
    (0x066258, i_066258),
    (0x06625E, i_06625e),
    (0x066264, i_066264),
    (0x066268, i_066268),
    (0x06626A, i_06626a),
    (0x066270, i_066270),
    (0x066276, i_066276),
    (0x06627A, i_06627a),
    (0x06627E, i_06627e),
    (0x066280, i_066280),
    (0x066286, i_066286),
    (0x06628C, i_06628c),
    (0x066292, i_066292),
    (0x066298, i_066298),
    (0x06629C, i_06629c),
    (0x0662A2, i_0662a2),
    (0x0662A4, i_0662a4),
    (0x0662AA, i_0662aa),
    (0x0662AE, i_0662ae),
    (0x0662B2, i_0662b2),
    (0x0662B6, i_0662b6),
    (0x0662BA, i_0662ba),
    (0x0662C0, i_0662c0),
    (0x0662C6, i_0662c6),
    (0x0662CC, i_0662cc),
    (0x0662D2, i_0662d2),
    (0x0662D6, i_0662d6),
    (0x0662DC, i_0662dc),
    (0x0662E2, i_0662e2),
    (0x0662E6, i_0662e6),
    (0x0662E8, i_0662e8),
    (0x0662EA, i_0662ea),
    (0x0662EE, i_0662ee),
    (0x0662F4, i_0662f4),
    (0x0662FA, i_0662fa),
    (0x066300, i_066300),
    (0x066306, i_066306),
    (0x06630A, i_06630a),
    (0x06630C, i_06630c),
    (0x066310, i_066310),
    (0x066314, i_066314),
    (0x06631A, i_06631a),
    (0x06631E, i_06631e),
    (0x066322, i_066322),
    (0x066326, i_066326),
    (0x06632A, i_06632a),
    (0x06632C, i_06632c),
    (0x066330, i_066330),
    (0x066336, i_066336),
    (0x06633C, i_06633c),
    (0x066342, i_066342),
    (0x066346, i_066346),
    (0x06634A, i_06634a),
    (0x06634C, i_06634c),
    (0x066352, i_066352),
    (0x066356, i_066356),
    (0x06635C, i_06635c),
    (0x066362, i_066362),
    (0x066368, i_066368),
    (0x06636E, i_06636e),
    (0x066372, i_066372),
    (0x066378, i_066378),
    (0x06637A, i_06637a),
    (0x06637E, i_06637e),
    (0x066384, i_066384),
    (0x066388, i_066388),
    (0x06638E, i_06638e),
    (0x066394, i_066394),
    (0x06639A, i_06639a),
    (0x0663A0, i_0663a0),
    (0x0663A6, i_0663a6),
    (0x0663A8, i_0663a8),
    (0x0663AC, i_0663ac),
    (0x0663B2, i_0663b2),
    (0x0663B6, i_0663b6),
    (0x0663B8, i_0663b8),
    (0x0663BA, i_0663ba),
    (0x0663BE, i_0663be),
    (0x0663C2, i_0663c2),
    (0x0663C8, i_0663c8),
    (0x0663CE, i_0663ce),
    (0x0663D4, i_0663d4),
    (0x0663DA, i_0663da),
    (0x0663DE, i_0663de),
    (0x0663E4, i_0663e4),
    (0x0663EA, i_0663ea),
    (0x0663EE, i_0663ee),
    (0x0663F4, i_0663f4),
    (0x0663FA, i_0663fa),
    (0x0663FE, i_0663fe),
    (0x066400, i_066400),
    (0x066404, i_066404),
    (0x066408, i_066408),
    (0x06640C, i_06640c),
    (0x06640E, i_06640e),
    (0x066412, i_066412),
    (0x066418, i_066418),
    (0x06641E, i_06641e),
    (0x066422, i_066422),
    (0x066428, i_066428),
    (0x06642C, i_06642c),
    (0x066432, i_066432),
    (0x066438, i_066438),
    (0x06643C, i_06643c),
    (0x066442, i_066442),
    (0x066446, i_066446),
    (0x06644C, i_06644c),
    (0x066452, i_066452),
    (0x066456, i_066456),
    (0x06645C, i_06645c),
    (0x066462, i_066462),
    (0x066468, i_066468),
    (0x06646C, i_06646c),
    (0x066470, i_066470),
    (0x066476, i_066476),
    (0x06647C, i_06647c),
    (0x066482, i_066482),
    (0x066488, i_066488),
    (0x06648E, i_06648e),
    (0x066490, i_066490),
    (0x066496, i_066496),
    (0x06649C, i_06649c),
    (0x0664A0, i_0664a0),
    (0x0664A2, i_0664a2),
    (0x0664A6, i_0664a6),
    (0x0664AA, i_0664aa),
    (0x0664B0, i_0664b0),
    (0x0664B6, i_0664b6),
    (0x0664BA, i_0664ba),
    (0x0664C0, i_0664c0),
    (0x0664C4, i_0664c4),
    (0x0664CA, i_0664ca),
    (0x0664D0, i_0664d0),
    (0x0664D4, i_0664d4),
    (0x0664DA, i_0664da),
    (0x0664DE, i_0664de),
    (0x0664E4, i_0664e4),
    (0x0664EA, i_0664ea),
    (0x0664EE, i_0664ee),
    (0x0664F4, i_0664f4),
    (0x0664FA, i_0664fa),
    (0x066500, i_066500),
    (0x066504, i_066504),
    (0x066508, i_066508),
    (0x06650E, i_06650e),
    (0x066514, i_066514),
    (0x06651A, i_06651a),
    (0x066520, i_066520),
    (0x066526, i_066526),
    (0x066528, i_066528),
    (0x06652C, i_06652c),
    (0x066532, i_066532),
    (0x066538, i_066538),
    (0x06653E, i_06653e),
    (0x066544, i_066544),
    (0x06654A, i_06654a),
    (0x066550, i_066550),
    (0x066554, i_066554),
    (0x066558, i_066558),
    (0x06655C, i_06655c),
    (0x066560, i_066560),
    (0x066564, i_066564),
    (0x066568, i_066568),
    (0x06656A, i_06656a),
    (0x06656E, i_06656e),
    (0x066572, i_066572),
    (0x066574, i_066574),
    (0x06657A, i_06657a),
    (0x066580, i_066580),
    (0x066584, i_066584),
    (0x066588, i_066588),
    (0x06658C, i_06658c),
    (0x066592, i_066592),
    (0x066598, i_066598),
    (0x06659E, i_06659e),
    (0x0665A2, i_0665a2),
    (0x0665A8, i_0665a8),
    (0x0665AE, i_0665ae),
    (0x0665B4, i_0665b4),
    (0x0665BA, i_0665ba),
    (0x0665BE, i_0665be),
    (0x0665C4, i_0665c4),
    (0x0665C6, i_0665c6),
    (0x0665CC, i_0665cc),
    (0x0665D0, i_0665d0),
    (0x0665D4, i_0665d4),
    (0x0665D8, i_0665d8),
    (0x0665DE, i_0665de),
    (0x0665E2, i_0665e2),
    (0x0665E6, i_0665e6),
    (0x0665E8, i_0665e8),
    (0x0665EC, i_0665ec),
    (0x0665F0, i_0665f0),
    (0x0665F2, i_0665f2),
    (0x0665F6, i_0665f6),
    (0x0665FC, i_0665fc),
    (0x066600, i_066600),
    (0x066604, i_066604),
    (0x066608, i_066608),
    (0x06660E, i_06660e),
    (0x066612, i_066612),
    (0x066616, i_066616),
    (0x06661A, i_06661a),
    (0x06661C, i_06661c),
    (0x066620, i_066620),
    (0x066622, i_066622),
    (0x066628, i_066628),
    (0x06662C, i_06662c),
    (0x066630, i_066630),
    (0x066634, i_066634),
    (0x06663A, i_06663a),
    (0x06663C, i_06663c),
    (0x066642, i_066642),
    (0x066644, i_066644),
    (0x06664A, i_06664a),
    (0x06664E, i_06664e),
    (0x066652, i_066652),
    (0x066658, i_066658),
    (0x06665A, i_06665a),
    (0x06665E, i_06665e),
    (0x066662, i_066662),
    (0x066666, i_066666),
    (0x066668, i_066668),
    (0x06666C, i_06666c),
    (0x06666E, i_06666e),
    (0x066670, i_066670),
    (0x066674, i_066674),
    (0x066678, i_066678),
    (0x06667A, i_06667a),
    (0x06667E, i_06667e),
    (0x066680, i_066680),
    (0x066684, i_066684),
    (0x06668A, i_06668a),
    (0x066690, i_066690),
    (0x066692, i_066692),
    (0x066696, i_066696),
    (0x06669A, i_06669a),
    (0x06669E, i_06669e),
    (0x0666A2, i_0666a2),
    (0x0666A6, i_0666a6),
    (0x0666A8, i_0666a8),
    (0x0666AC, i_0666ac),
    (0x0666AE, i_0666ae),
    (0x0666B2, i_0666b2),
    (0x0666BC, i_0666bc),
    (0x0666C0, i_0666c0),
    (0x0666C4, i_0666c4),
    (0x0666CA, i_0666ca),
    (0x0666CE, i_0666ce),
    (0x0666D2, i_0666d2),
    (0x0666D8, i_0666d8),
    (0x0666DC, i_0666dc),
    (0x0666E0, i_0666e0),
    (0x0666E2, i_0666e2),
    (0x0666E6, i_0666e6),
    (0x0666EA, i_0666ea),
    (0x0666EE, i_0666ee),
    (0x0666F2, i_0666f2),
    (0x0666F4, i_0666f4),
    (0x0666FA, i_0666fa),
    (0x0666FE, i_0666fe),
    (0x066704, i_066704),
    (0x066708, i_066708),
    (0x06670C, i_06670c),
    (0x066710, i_066710),
    (0x066714, i_066714),
    (0x066718, i_066718),
    (0x06671C, i_06671c),
    (0x066722, i_066722),
    (0x066726, i_066726),
    (0x06672C, i_06672c),
    (0x066732, i_066732),
    (0x066736, i_066736),
    (0x06673A, i_06673a),
    (0x066740, i_066740),
    (0x066746, i_066746),
    (0x06674C, i_06674c),
    (0x066752, i_066752),
    (0x066758, i_066758),
    (0x06675E, i_06675e),
    (0x066764, i_066764),
    (0x066768, i_066768),
    (0x06676C, i_06676c),
    (0x066770, i_066770),
    (0x066776, i_066776),
    (0x06677A, i_06677a),
    (0x06677E, i_06677e),
    (0x066780, i_066780),
    (0x066786, i_066786),
    (0x06678A, i_06678a),
    (0x066790, i_066790),
    (0x066794, i_066794),
    (0x066798, i_066798),
    (0x06679A, i_06679a),
    (0x06679C, i_06679c),
    (0x0667A0, i_0667a0),
    (0x0667A2, i_0667a2),
    (0x0667A4, i_0667a4),
    (0x0667AA, i_0667aa),
    (0x0667AE, i_0667ae),
    (0x0667B0, i_0667b0),
    (0x0667B4, i_0667b4),
    (0x0667B8, i_0667b8),
    (0x0667BC, i_0667bc),
    (0x0667BE, i_0667be),
    (0x0667C2, i_0667c2),
    (0x0667C4, i_0667c4),
    (0x0667C8, i_0667c8),
    (0x0667CE, i_0667ce),
    (0x0667D4, i_0667d4),
    (0x0667D8, i_0667d8),
    (0x0667DE, i_0667de),
    (0x0667E4, i_0667e4),
    (0x0667EA, i_0667ea),
    (0x0667EE, i_0667ee),
    (0x0667F2, i_0667f2),
    (0x0667F6, i_0667f6),
    (0x0667FC, i_0667fc),
    (0x066802, i_066802),
    (0x066806, i_066806),
    (0x06680A, i_06680a),
    (0x06680E, i_06680e),
    (0x066814, i_066814),
    (0x06681A, i_06681a),
    (0x06681E, i_06681e),
    (0x066822, i_066822),
    (0x066826, i_066826),
    (0x06682A, i_06682a),
    (0x06682E, i_06682e),
    (0x066832, i_066832),
    (0x066836, i_066836),
    (0x06683A, i_06683a),
    (0x066840, i_066840),
    (0x066844, i_066844),
    (0x066846, i_066846),
    (0x06684C, i_06684c),
    (0x066852, i_066852),
    (0x066856, i_066856),
    (0x06685A, i_06685a),
    (0x06685C, i_06685c),
    (0x066860, i_066860),
    (0x066868, i_066868),
    (0x06686C, i_06686c),
    (0x066872, i_066872),
    (0x066876, i_066876),
    (0x066878, i_066878),
    (0x06687E, i_06687e),
    (0x066884, i_066884),
    (0x066888, i_066888),
    (0x06688C, i_06688c),
    (0x066890, i_066890),
    (0x066892, i_066892),
    (0x066896, i_066896),
    (0x06689C, i_06689c),
    (0x0668A0, i_0668a0),
    (0x0668A4, i_0668a4),
    (0x0668A8, i_0668a8),
    (0x0668AC, i_0668ac),
    (0x0668B0, i_0668b0),
    (0x0668B4, i_0668b4),
    (0x0668BA, i_0668ba),
    (0x0668C0, i_0668c0),
    (0x0668C4, i_0668c4),
    (0x0668CA, i_0668ca),
    (0x0668D0, i_0668d0),
    (0x0668D4, i_0668d4),
    (0x0668DA, i_0668da),
    (0x0668DE, i_0668de),
    (0x0668E4, i_0668e4),
    (0x0668E8, i_0668e8),
    (0x0668EA, i_0668ea),
    (0x0668F0, i_0668f0),
    (0x0668F6, i_0668f6),
    (0x0668FA, i_0668fa),
    (0x0668FE, i_0668fe),
    (0x066904, i_066904),
    (0x066908, i_066908),
    (0x06690C, i_06690c),
    (0x066910, i_066910),
    (0x066912, i_066912),
    (0x066916, i_066916),
    (0x06691C, i_06691c),
    (0x066920, i_066920),
    (0x066924, i_066924),
    (0x066926, i_066926),
    (0x06692C, i_06692c),
    (0x066932, i_066932),
    (0x066936, i_066936),
    (0x06693A, i_06693a),
    (0x066940, i_066940),
    (0x066946, i_066946),
    (0x06694A, i_06694a),
    (0x066950, i_066950),
    (0x066956, i_066956),
    (0x06695A, i_06695a),
    (0x066960, i_066960),
    (0x066964, i_066964),
    (0x06696A, i_06696a),
    (0x06696E, i_06696e),
    (0x066970, i_066970),
    (0x066976, i_066976),
    (0x06697C, i_06697c),
    (0x066980, i_066980),
    (0x066984, i_066984),
    (0x06698A, i_06698a),
    (0x06698E, i_06698e),
    (0x066992, i_066992),
    (0x066996, i_066996),
    (0x066998, i_066998),
    (0x06699C, i_06699c),
    (0x0669A2, i_0669a2),
    (0x0669A6, i_0669a6),
    (0x0669AA, i_0669aa),
    (0x0669AC, i_0669ac),
    (0x0669B2, i_0669b2),
    (0x0669B8, i_0669b8),
    (0x0669BC, i_0669bc),
    (0x0669C0, i_0669c0),
    (0x0669C4, i_0669c4),
    (0x0669CA, i_0669ca),
    (0x0669CE, i_0669ce),
    (0x0669D0, i_0669d0),
    (0x0669D6, i_0669d6),
    (0x0669DC, i_0669dc),
    (0x0669E0, i_0669e0),
    (0x0669E4, i_0669e4),
    (0x0669E6, i_0669e6),
    (0x0669EA, i_0669ea),
    (0x0669F0, i_0669f0),
    (0x0669F4, i_0669f4),
    (0x0669FA, i_0669fa),
    (0x0669FE, i_0669fe),
    (0x066A02, i_066a02),
    (0x066A08, i_066a08),
    (0x066A0C, i_066a0c),
    (0x066A12, i_066a12),
    (0x066A16, i_066a16),
    (0x066A1C, i_066a1c),
    (0x066A20, i_066a20),
    (0x066A22, i_066a22),
    (0x066A28, i_066a28),
    (0x066A2E, i_066a2e),
    (0x066A32, i_066a32),
    (0x066A36, i_066a36),
    (0x066A38, i_066a38),
    (0x066A3C, i_066a3c),
    (0x066A40, i_066a40),
    (0x066A44, i_066a44),
    (0x066A48, i_066a48),
    (0x066A4C, i_066a4c),
    (0x066A4E, i_066a4e),
    (0x066A52, i_066a52),
    (0x066A56, i_066a56),
    (0x066A5A, i_066a5a),
    (0x066A5C, i_066a5c),
    (0x066A62, i_066a62),
    (0x066A66, i_066a66),
    (0x066A6A, i_066a6a),
    (0x066A6C, i_066a6c),
    (0x066A70, i_066a70),
    (0x066A74, i_066a74),
    (0x066A7A, i_066a7a),
    (0x066A7E, i_066a7e),
    (0x066A82, i_066a82),
    (0x066A84, i_066a84),
    (0x066A86, i_066a86),
    (0x066A8E, i_066a8e),
    (0x066A94, i_066a94),
    (0x066A9C, i_066a9c),
    (0x066AA0, i_066aa0),
    (0x066AA6, i_066aa6),
    (0x066AAC, i_066aac),
    (0x066AB2, i_066ab2),
    (0x066AB8, i_066ab8),
    (0x066ABE, i_066abe),
    (0x066AC4, i_066ac4),
    (0x066AC8, i_066ac8),
    (0x066ACC, i_066acc),
    (0x066ACE, i_066ace),
    (0x066AD4, i_066ad4),
    (0x066ADA, i_066ada),
    (0x066ADE, i_066ade),
    (0x066AE2, i_066ae2),
    (0x066AE4, i_066ae4),
    (0x066AE6, i_066ae6),
    (0x066AE8, i_066ae8),
    (0x066AEA, i_066aea),
    (0x066AEE, i_066aee),
    (0x066AF2, i_066af2),
    (0x066AF6, i_066af6),
    (0x066AFC, i_066afc),
    (0x066B02, i_066b02),
    (0x066B08, i_066b08),
    (0x066B0C, i_066b0c),
    (0x066B12, i_066b12),
    (0x066B16, i_066b16),
    (0x066B1C, i_066b1c),
    (0x066B20, i_066b20),
    (0x066B22, i_066b22),
    (0x066B28, i_066b28),
    (0x066B2E, i_066b2e),
    (0x066B34, i_066b34),
    (0x066B3A, i_066b3a),
    (0x066B3E, i_066b3e),
    (0x066B42, i_066b42),
    (0x066B44, i_066b44),
    (0x066B4A, i_066b4a),
    (0x066B4E, i_066b4e),
    (0x066B52, i_066b52),
    (0x066B54, i_066b54),
    (0x066B5A, i_066b5a),
    (0x066B60, i_066b60),
    (0x066B64, i_066b64),
    (0x066B68, i_066b68),
    (0x066B6A, i_066b6a),
    (0x066B6C, i_066b6c),
    (0x066B70, i_066b70),
    (0x066B76, i_066b76),
    (0x066B7C, i_066b7c),
    (0x066B82, i_066b82),
    (0x066B86, i_066b86),
    (0x066B8C, i_066b8c),
    (0x066B90, i_066b90),
    (0x066B96, i_066b96),
    (0x066B9A, i_066b9a),
    (0x066B9C, i_066b9c),
    (0x066BA2, i_066ba2),
    (0x066BA8, i_066ba8),
    (0x066BAC, i_066bac),
    (0x066BB0, i_066bb0),
    (0x066BB2, i_066bb2),
    (0x066BB8, i_066bb8),
    (0x066BBE, i_066bbe),
    (0x066BC2, i_066bc2),
    (0x066BC6, i_066bc6),
    (0x066BC8, i_066bc8),
    (0x066BCA, i_066bca),
    (0x066BD0, i_066bd0),
    (0x066BD4, i_066bd4),
    (0x066BD8, i_066bd8),
    (0x066BDC, i_066bdc),
    (0x066BE0, i_066be0),
    (0x066BE2, i_066be2),
    (0x066BE6, i_066be6),
    (0x066BEA, i_066bea),
    (0x066BF0, i_066bf0),
    (0x066BF6, i_066bf6),
    (0x066BFA, i_066bfa),
    (0x066C00, i_066c00),
    (0x066C04, i_066c04),
    (0x066C06, i_066c06),
    (0x066C0C, i_066c0c),
    (0x066C10, i_066c10),
    (0x066C12, i_066c12),
    (0x066C16, i_066c16),
    (0x066C1C, i_066c1c),
    (0x066C20, i_066c20),
    (0x066C26, i_066c26),
];
