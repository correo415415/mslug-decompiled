// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $04566A: bra.w $4568e
pub fn i_04566a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4566E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4568E;
    10
}
/// $04566E: cmpi.b #$3, d0
pub fn i_04566e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45672;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $045672: bne.w $4567e
pub fn i_045672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45676;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4567E; return 10; }
    8
}
/// $045676: move.b #$f0, d0
pub fn i_045676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4567A;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04567A: bra.w $4568e
pub fn i_04567a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4567E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4568E;
    10
}
/// $04567E: cmpi.b #$5, d0
pub fn i_04567e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45682;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $045682: bne.w $4568a
pub fn i_045682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45686;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4568A; return 10; }
    8
}
/// $045686: move.b #$30, d0
pub fn i_045686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4568A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04568A: move.b #$10, d0
pub fn i_04568a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4568E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04568E: and.b $72(a6), d0
pub fn i_04568e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45692;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $045692: beq.w $4569c
pub fn i_045692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45696;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4569C; return 10; }
    8
}
/// $045696: move.b #$ff, $76(a6)
pub fn i_045696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4569C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04569C: move.b $75(a6), d0
pub fn i_04569c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456A0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0456A0: beq.w $456ae
pub fn i_0456a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x456AE; return 10; }
    8
}
/// $0456A4: subq.b #$1, d0
pub fn i_0456a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456A6;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0456A6: move.b d0, $75(a6)
pub fn i_0456a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456AA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0456AA: bra.w $45700
pub fn i_0456aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x45700;
    10
}
/// $0456AE: tst.b $74(a6)
pub fn i_0456ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456B2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0456B2: bne.w $456ee
pub fn i_0456b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x456EE; return 10; }
    8
}
/// $0456B6: cmpi.b #$3, $106f2a.l
pub fn i_0456b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456BE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0456BE: beq.w $456ee
pub fn i_0456be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x456EE; return 10; }
    8
}
/// $0456C2: cmpi.b #$2, $106f2a.l
pub fn i_0456c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456CA;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $0456CA: beq.w $456ee
pub fn i_0456ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x456EE; return 10; }
    8
}
/// $0456CE: cmpi.b #$1, $106f2a.l
pub fn i_0456ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456D6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0456D6: bne.w $456e4
pub fn i_0456d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x456E4; return 10; }
    8
}
/// $0456DA: move.b #$18, $75(a6)
pub fn i_0456da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0456E0: bra.w $45706
pub fn i_0456e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x45706;
    10
}
/// $0456E4: move.b #$6, $75(a6)
pub fn i_0456e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0456EA: bra.w $45706
pub fn i_0456ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x45706;
    10
}
/// $0456EE: tst.b $76(a6)
pub fn i_0456ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456F2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0456F2: beq.w $45700
pub fn i_0456f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45700; return 10; }
    8
}
/// $0456F6: move.b #$2, $75(a6)
pub fn i_0456f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x456FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0456FC: bra.w $45706
pub fn i_0456fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45700;
    cpu.ir = 0x6000;
    cpu.pc = 0x45706;
    10
}
// ===== ClearXN_045700  ($045700, 6 B, thunks/ccr_helpers.c)
/// $045700: andi.b #$ee, ccr
pub fn i_045700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45704;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $045704: rts
pub fn i_045704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45706;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ent_AmmoTick_045706  ($045706, 10 B, input/entity_aim_input_044f8a.s)
/// $045706: subi.b #$1, $79(a6)
pub fn i_045706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4570C;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04570C: clr.b $76(a6)
pub fn i_04570c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45710;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetXN_045710  ($045710, 6 B, thunks/ccr_helpers.c)
/// $045710: ori.b #$11, ccr
pub fn i_045710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45714;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $045714: rts
pub fn i_045714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45716;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ent_ClampY_045716  ($045716, 22 B, input/entity_aim_input_044f8a.s)
/// $045716: cmpi.w #$170, $24(a6)
pub fn i_045716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4571C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $04571C: bge.w $45720
pub fn i_04571c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45720;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x45720; return 10; }
    8
}
/// $045720: cmpi.w #$170, $24(a6)
pub fn i_045720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45726;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $045726: ble.w $4572a
pub fn i_045726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4572A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4572A; return 10; }
    8
}
/// $04572A: rts
pub fn i_04572a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4572C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ent_GroundProbe_04572C  ($04572C, 82 B, input/entity_aim_input_044f8a.s)
/// $04572C: move.b $35(a6), d0
pub fn i_04572c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45730;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x35);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045730: lsl.w #$2, d0
pub fn i_045730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45732;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $045732: andi.w #$ff, d0
pub fn i_045732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45736;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045736: lea.l $29d752.l, a0
pub fn i_045736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4573C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D752;
    20
}
/// $04573C: move.w $22(a6), d1
pub fn i_04573c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45740;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045740: move.w $24(a6), d2
pub fn i_045740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45744;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045744: add.w (a0, d0.w), d1
pub fn i_045744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45748;
    cpu.ir = 0xD270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $045748: add.w $2(a0, d0.w), d2
pub fn i_045748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4574C;
    cpu.ir = 0xD470;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $04574C: jsr $280c6.l
pub fn i_04574c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45752;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45752);
    cpu.pc = 0x280C6;
    16
}
/// $045752: cmpi.b #$0, d0
pub fn i_045752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45756;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $045756: beq.w $45784
pub fn i_045756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4575A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45784; return 10; }
    8
}
/// $04575A: cmpi.b #$10, d0
pub fn i_04575a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4575E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $04575E: beq.w $4577e
pub fn i_04575e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45762;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4577E; return 10; }
    8
}
/// $045762: cmpi.b #$20, d0
pub fn i_045762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45766;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $045766: beq.w $4577e
pub fn i_045766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4576A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4577E; return 10; }
    8
}
/// $04576A: cmpi.b #$21, d0
pub fn i_04576a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4576E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $04576E: beq.w $4577e
pub fn i_04576e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45772;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4577E; return 10; }
    8
}
/// $045772: andi.b #$f0, d0
pub fn i_045772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45776;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $045776: cmpi.b #$0, d0
pub fn i_045776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4577A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $04577A: bne.w $45784
pub fn i_04577a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4577E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45784; return 10; }
    8
}
// ===== ClearXN_04577e  ($04577E, 6 B, thunks/ccr_helpers.c)
/// $04577E: andi.b #$ee, ccr
pub fn i_04577e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45782;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $045782: rts
pub fn i_045782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45784;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_045784  ($045784, 6 B, thunks/ccr_helpers.c)
/// $045784: ori.b #$11, ccr
pub fn i_045784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45788;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $045788: rts
pub fn i_045788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4578A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_04578A  ($04578A, 16 B, input/entity_aim_input_044f8a.s)
/// $04578A: movea.l $8(a6), a1
pub fn i_04578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4578E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04578E: move.b $10(a6), d0
pub fn i_04578e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45792;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045792: cmp.b $10(a1), d0
pub fn i_045792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45796;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $045796: bcs.w $457a0
pub fn i_045796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4579A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x457A0; return 10; }
    8
}
// ===== ClearXN_04579a  ($04579A, 6 B, thunks/ccr_helpers.c)
/// $04579A: andi.b #$ee, ccr
pub fn i_04579a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4579E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04579E: rts
pub fn i_04579e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0457a0  ($0457A0, 6 B, thunks/ccr_helpers.c)
/// $0457A0: ori.b #$11, ccr
pub fn i_0457a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457A4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0457A4: rts
pub fn i_0457a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ent_SfxByMode_0457A6  ($0457A6, 38 B, input/entity_aim_input_044f8a.s)
/// $0457A6: move.w #$1d6, d1
pub fn i_0457a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457AA: move.b $6d(a6), d0
pub fn i_0457aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457AE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0457AE: cmpi.b #$1, d0
pub fn i_0457ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457B2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0457B2: beq.w $457c6
pub fn i_0457b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x457C6; return 10; }
    8
}
/// $0457B6: move.w #$1d6, d1
pub fn i_0457b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457BA: cmpi.b #$2, d0
pub fn i_0457ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457BE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $0457BE: beq.w $457c6
pub fn i_0457be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x457C6; return 10; }
    8
}
/// $0457C2: move.w #$2, d1
pub fn i_0457c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457C6: jmp $236e.l
pub fn i_0457c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x236E;
    8
}
// ===== Ent_AnimFrame_0457CC  ($0457CC, 58 B, input/entity_aim_input_044f8a.s)
/// $0457CC: lea.l $289204.l, a0
pub fn i_0457cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x289204;
    20
}
/// $0457D2: move.b $34(a6), d0
pub fn i_0457d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457D6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0457D6: addq.w #$4, d0
pub fn i_0457d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457D8;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0457D8: andi.w #$f8, d0
pub fn i_0457d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0457DC: move.w d0, d1
pub fn i_0457dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457DE;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457DE: add.w d0, d0
pub fn i_0457de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457E0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457E0: add.w d0, d1
pub fn i_0457e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457E2;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457E2: move.b $3b(a6), d0
pub fn i_0457e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457E6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0457E6: andi.w #$ff, d0
pub fn i_0457e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457EA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0457EA: add.w d0, d0
pub fn i_0457ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457EC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457EC: add.w d0, d0
pub fn i_0457ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457EE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457EE: add.w d1, d0
pub fn i_0457ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457F0;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457F0: move.l (a0, d0.w), $3c(a6)
pub fn i_0457f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457F6;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $0457F6: jsr $5ca2a.l
pub fn i_0457f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x457FC);
    cpu.pc = 0x5CA2A;
    16
}
/// $0457FC: move.b $3b(a6), d0
pub fn i_0457fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45800;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045800: beq.w $4580a
pub fn i_045800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45804;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4580A; return 10; }
    8
}
/// $045804: subq.b #$1, d0
pub fn i_045804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45806;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetTaskB_045806  ($045806, 6 B, kernel/task_setters_b.c)
/// $045806: move.b d0, $3b(a6)
pub fn i_045806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4580A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04580A: rts
pub fn i_04580a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4580C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_FireGroundSnap_04580C  ($04580C, 208 B, enemies/vehicles/turret_boss2_04580c.s)
/// $04580C: movea.l $c(a6), a0
pub fn i_04580c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45810;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045810: move.b $68(a0), $68(a6)
pub fn i_045810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45816;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045816: lea.l $45fde(pc), a1
pub fn i_045816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4581A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45FDE;
    16
}
/// $04581A: jsr $6fe.l
pub fn i_04581a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45820);
    cpu.pc = 0x6FE;
    16
}
/// $045820: jsr $5dd02.l
pub fn i_045820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45826);
    cpu.pc = 0x5DD02;
    16
}
/// $045826: jsr $517fe.l
pub fn i_045826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4582C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4582C);
    cpu.pc = 0x517FE;
    16
}
/// $04582C: move.b $34(a6), d0
pub fn i_04582c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45830;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045830: addq.b #$4, d0
pub fn i_045830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45832;
    cpu.ir = 0x5800;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045832: move.b d0, $98(a0)
pub fn i_045832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45836;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045836: move.b $80(a6), $9a(a0)
pub fn i_045836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4583C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04583C: move.b $81(a6), $9b(a0)
pub fn i_04583c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45842;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045842: addi.b #$1, $81(a6)
pub fn i_045842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45848;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $045848: movea.l $c(a6), a0
pub fn i_045848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4584C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04584C: bset.b #$2, $8c(a0)
pub fn i_04584c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45852;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045852: subi.b #$1, $80(a6)
pub fn i_045852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45858;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $045858: move.b $34(a6), d0
pub fn i_045858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4585C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04585C: addi.w #$4, d0
pub fn i_04585c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45860;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045860: andi.w #$f8, d0
pub fn i_045860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45864;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045864: lsr.w #$1, d0
pub fn i_045864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45866;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $045866: lea.l $29d752.l, a0
pub fn i_045866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4586C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D752;
    20
}
/// $04586C: move.w $22(a6), d1
pub fn i_04586c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45870;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045870: move.w $24(a6), d2
pub fn i_045870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45874;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045874: add.w (a0, d0.w), d1
pub fn i_045874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45878;
    cpu.ir = 0xD270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $045878: add.w $2(a0, d0.w), d2
pub fn i_045878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4587C;
    cpu.ir = 0xD470;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $04587C: jsr $280c6.l
pub fn i_04587c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45882);
    cpu.pc = 0x280C6;
    16
}
/// $045882: cmpi.b #$0, d0
pub fn i_045882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45886;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $045886: beq.w $458d4
pub fn i_045886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4588A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458D4; return 10; }
    8
}
/// $04588A: cmpi.b #$10, d0
pub fn i_04588a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4588E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $04588E: beq.w $458ae
pub fn i_04588e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45892;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458AE; return 10; }
    8
}
/// $045892: cmpi.b #$20, d0
pub fn i_045892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45896;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $045896: beq.w $458ae
pub fn i_045896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4589A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458AE; return 10; }
    8
}
/// $04589A: cmpi.b #$21, d0
pub fn i_04589a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4589E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $04589E: beq.w $458ae
pub fn i_04589e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458AE; return 10; }
    8
}
/// $0458A2: andi.b #$f0, d0
pub fn i_0458a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458A6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0458A6: cmpi.b #$0, d0
pub fn i_0458a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458AA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0458AA: bne.w $458d4
pub fn i_0458aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x458D4; return 10; }
    8
}
/// $0458AE: lea.l $30c14.l, a1
pub fn i_0458ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x30C14;
    20
}
/// $0458B4: jsr $4ae.l
pub fn i_0458b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x458BA);
    cpu.pc = 0x4AE;
    16
}
/// $0458BA: jsr $5dd02.l
pub fn i_0458ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x458C0);
    cpu.pc = 0x5DD02;
    16
}
/// $0458C0: move.b $34(a6), d0
pub fn i_0458c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458C4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0458C4: addi.w #$4, d0
pub fn i_0458c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458C8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0458C8: andi.w #$f8, d0
pub fn i_0458c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458CC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0458CC: lsr.w #$3, d0
pub fn i_0458cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458CE;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0458CE: move.b d0, $98(a0)
pub fn i_0458ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458D2;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0458D2: rts
pub fn i_0458d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0458D4: move.b #$5, $3b(a6)
pub fn i_0458d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0458DA: rts
pub fn i_0458da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_FollowParent_0458DC  ($0458DC, 94 B, enemies/vehicles/turret_boss2_04580c.s)
/// $0458DC: movea.l $c(a6), a0
pub fn i_0458dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0458E0: move.b $59(a0), $59(a6)
pub fn i_0458e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458E6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0458E6: move.w $38(a0), d0
pub fn i_0458e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458EA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0458EA: move.w $22(a0), d1
pub fn i_0458ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458EE;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0458EE: move.w $24(a0), d2
pub fn i_0458ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458F2;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0458F2: movea.l $70(a0), a3
pub fn i_0458f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458F6;
    cpu.ir = 0x2668;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $0458F6: add.w $4(a3), d1
pub fn i_0458f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458FA;
    cpu.ir = 0xD26B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0458FA: add.w $6(a3), d2
pub fn i_0458fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458FE;
    cpu.ir = 0xD46B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0458FE: movea.l (a3), a3
pub fn i_0458fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45900;
    cpu.ir = 0x2653;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[3] = v2;
    12
}
/// $045900: add.w (a3), d1
pub fn i_045900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45902;
    cpu.ir = 0xD253;
    let v2 = bus.read16(cpu.a[3]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $045902: add.w $2(a3), d2
pub fn i_045902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45906;
    cpu.ir = 0xD46B;
    let ad2 = cpu.a[3].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045906: ori.w #$1, d0
pub fn i_045906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4590A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04590A: move.w d0, $38(a6)
pub fn i_04590a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4590E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04590E: move.w d1, $22(a6)
pub fn i_04590e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45912;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045912: move.w d2, $24(a6)
pub fn i_045912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45916;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045916: move.b $6d(a0), d0
pub fn i_045916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4591A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04591A: cmp.b $6d(a6), d0
pub fn i_04591a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4591E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04591E: beq.w $45938
pub fn i_04591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45922;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45938; return 10; }
    8
}
/// $045922: move.b d0, $6d(a6)
pub fn i_045922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45926;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045926: move.b $68(a0), d0
pub fn i_045926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4592A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04592A: move.b d0, $68(a6)
pub fn i_04592a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4592E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04592E: jsr $13600.l
pub fn i_04592e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45934);
    cpu.pc = 0x13600;
    16
}
/// $045934: bsr.w $457a6
pub fn i_045934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45938;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45938);
    cpu.pc = 0x457A6;
    18
}
/// $045938: rts
pub fn i_045938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4593A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_ReadPlayerInput_04593A  ($04593A, 110 B, enemies/vehicles/turret_boss2_04580c.s)
/// $04593A: move.b $6d(a6), d0
pub fn i_04593a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4593E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04593E: bpl.w $4594e
pub fn i_04593e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45942;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4594E; return 10; }
    8
}
/// $045942: clr.b $71(a6)
pub fn i_045942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45946;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x71);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $045946: clr.b $7c(a6)
pub fn i_045946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4594A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04594A: bra.w $459a6
pub fn i_04594a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4594E;
    cpu.ir = 0x6000;
    cpu.pc = 0x459A6;
    10
}
/// $04594E: cmpi.b #$1, d0
pub fn i_04594e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45952;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $045952: bne.w $45960
pub fn i_045952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45956;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45960; return 10; }
    8
}
/// $045956: lea.l $100440.l, a1
pub fn i_045956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4595C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $04595C: bra.w $45966
pub fn i_04595c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45960;
    cpu.ir = 0x6000;
    cpu.pc = 0x45966;
    10
}
/// $045960: lea.l $1004e0.l, a1
pub fn i_045960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45966;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $045966: movea.l $c(a1), a1
pub fn i_045966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4596A;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04596A: movea.l $72(a1), a2
pub fn i_04596a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4596E;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $04596E: jsr $5d674.l
pub fn i_04596e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45974);
    cpu.pc = 0x5D674;
    16
}
/// $045974: move.b $2(a2), d0
pub fn i_045974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45978;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045978: andi.b #$f, d0
pub fn i_045978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4597C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04597C: move.b d0, $71(a6)
pub fn i_04597c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45980;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045980: move.b $3(a2), d0
pub fn i_045980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45984;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045984: andi.b #$10, d0
pub fn i_045984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45988;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $045988: beq.w $459a6
pub fn i_045988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4598C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x459A6; return 10; }
    8
}
/// $04598C: move.w #$108c, d0
pub fn i_04598c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45990;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045990: jsr $2352.l
pub fn i_045990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45996);
    cpu.pc = 0x2352;
    16
}
/// $045996: tst.b $7c(a6)
pub fn i_045996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4599A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04599A: sne.b d0
pub fn i_04599a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4599C;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04599C: move.b #$3, $7c(a6)
pub fn i_04599c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0459A2: and.b d0, $7d(a6)
pub fn i_0459a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459A6;
    cpu.ir = 0xC12E;
    let ad1 = cpu.a[6].wrapping_add(0x7D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0459A6: rts
pub fn i_0459a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_InitDir0_0459A8  ($0459A8, 24 B, enemies/vehicles/turret_boss2_04580c.s)
/// $0459A8: move.b #$0, $70(a6)
pub fn i_0459a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0459AE: move.l #$289504, $74(a6)
pub fn i_0459ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459B6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x289504;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0459B6: move.w #$4000, $34(a6)
pub fn i_0459b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0459BC: bra.w $45a0a
pub fn i_0459bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x45A0A;
    10
}
// ===== Turret_InitDir1_0459C0  ($0459C0, 24 B, enemies/vehicles/turret_boss2_04580c.s)
/// $0459C0: move.b #$1, $70(a6)
pub fn i_0459c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0459C6: move.l #$289544, $74(a6)
pub fn i_0459c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459CE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x289544;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0459CE: move.w #$8000, $34(a6)
pub fn i_0459ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0459D4: bra.w $45a0a
pub fn i_0459d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x45A0A;
    10
}
// ===== Turret_InitDir2_0459D8  ($0459D8, 24 B, enemies/vehicles/turret_boss2_04580c.s)
/// $0459D8: move.b #$2, $70(a6)
pub fn i_0459d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0459DE: move.l #$289584, $74(a6)
pub fn i_0459de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459E6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x289584;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0459E6: move.w #$c000, $34(a6)
pub fn i_0459e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0459EC: bra.w $45a0a
pub fn i_0459ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x45A0A;
    10
}
// ===== Turret_InitDir3_0459F0  ($0459F0, 26 B, enemies/vehicles/turret_boss2_04580c.s)
/// $0459F0: move.b #$3, $70(a6)
pub fn i_0459f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459F6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0459F6: move.l #$2895c4, $74(a6)
pub fn i_0459f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459FE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2895C4;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0459FE: move.w #$0, $34(a6)
pub fn i_0459fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A04;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045A04: bra.w $45a0a
pub fn i_045a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A08;
    cpu.ir = 0x6000;
    cpu.pc = 0x45A0A;
    10
}
// ===== Turret_SetupCommon_045A0A  ($045A0A, 46 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045A0A: jsr $13600.l
pub fn i_045a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45A10);
    cpu.pc = 0x13600;
    16
}
/// $045A10: move.w #$2, d1
pub fn i_045a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045A14: jsr $236e.l
pub fn i_045a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45A1A);
    cpu.pc = 0x236E;
    16
}
/// $045A1A: move.b #$ff, $68(a6)
pub fn i_045a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045A20: move.b #$ff, $6d(a6)
pub fn i_045a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045A26: clr.b $81(a6)
pub fn i_045a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A2A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $045A2A: clr.b $3b(a6)
pub fn i_045a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $045A2E: clr.w $78(a6)
pub fn i_045a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A32;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $045A32: lea.l $45a38(pc), a1
pub fn i_045a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45A38;
    16
}
/// $045A36: move.l a1, (a6)
pub fn i_045a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Turret_Track_045A38  ($045A38, 134 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045A38: bsr.w $458dc
pub fn i_045a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A3C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45A3C);
    cpu.pc = 0x458DC;
    18
}
/// $045A3C: bsr.w $4593a
pub fn i_045a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A40;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45A40);
    cpu.pc = 0x4593A;
    18
}
/// $045A40: movea.l $74(a6), a0
pub fn i_045a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A44;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045A44: move.b $71(a6), d0
pub fn i_045a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A48;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045A48: andi.w #$f, d0
pub fn i_045a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A4C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045A4C: add.w d0, d0
pub fn i_045a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A4E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045A4E: add.w d0, d0
pub fn i_045a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A50;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045A50: move.l (a0, d0.w), d0
pub fn i_045a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A54;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $045A54: bmi.w $45a88
pub fn i_045a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A58;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x45A88; return 10; }
    8
}
/// $045A58: sub.w $34(a6), d0
pub fn i_045a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A5C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045A5C: ext.l d0
pub fn i_045a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A5E;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $045A5E: bpl.w $45a64
pub fn i_045a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A62;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x45A64; return 10; }
    8
}
/// $045A62: neg.w d0
pub fn i_045a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A64;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $045A64: asr.w #$2, d0
pub fn i_045a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A66;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $045A66: move.w $78(a6), d1
pub fn i_045a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A6A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045A6A: addi.w #$100, d1
pub fn i_045a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A6E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045A6E: cmp.w d0, d1
pub fn i_045a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A70;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $045A70: bcs.w $45a76
pub fn i_045a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A74;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x45A76; return 10; }
    8
}
/// $045A74: move.w d0, d1
pub fn i_045a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A76;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045A76: move.w d1, $78(a6)
pub fn i_045a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A7A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045A7A: swap d0
pub fn i_045a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A7C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $045A7C: eor.w d0, d1
pub fn i_045a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A7E;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $045A7E: sub.w d0, d1
pub fn i_045a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A80;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045A80: add.w d1, $34(a6)
pub fn i_045a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A84;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $045A84: bra.w $45a8c
pub fn i_045a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A88;
    cpu.ir = 0x6000;
    cpu.pc = 0x45A8C;
    10
}
/// $045A88: clr.w $78(a6)
pub fn i_045a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A8C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $045A8C: move.b $7d(a6), d0
pub fn i_045a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A90;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045A90: beq.w $45a9e
pub fn i_045a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45A9E; return 10; }
    8
}
/// $045A94: subq.b #$1, d0
pub fn i_045a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A96;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045A96: move.b d0, $7d(a6)
pub fn i_045a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A9A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045A9A: bra.w $45ab6
pub fn i_045a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x45AB6;
    10
}
/// $045A9E: move.b #$1, $7d(a6)
pub fn i_045a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AA4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045AA4: move.b $7c(a6), d0
pub fn i_045aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AA8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045AA8: beq.w $45ab6
pub fn i_045aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45AB6; return 10; }
    8
}
/// $045AAC: subq.b #$1, d0
pub fn i_045aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AAE;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045AAE: move.b d0, $7c(a6)
pub fn i_045aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AB2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045AB2: bsr.w $4580c
pub fn i_045ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AB6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45AB6);
    cpu.pc = 0x4580C;
    18
}
/// $045AB6: bsr.w $457cc
pub fn i_045ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ABA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45ABA);
    cpu.pc = 0x457CC;
    18
}
/// $045ABA: bra.w $45b9c
pub fn i_045aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ABE;
    cpu.ir = 0x6000;
    cpu.pc = 0x45B9C;
    10
}
// ===== Turret_InitDir4_045ABE  ($045ABE, 68 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045ABE: move.b #$4, $70(a6)
pub fn i_045abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045AC4: move.l #$289604, $74(a6)
pub fn i_045ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ACC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x289604;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $045ACC: move.w #$0, $34(a6)
pub fn i_045acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045AD2: move.w #$0, $7e(a6)
pub fn i_045ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AD8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045AD8: jsr $13600.l
pub fn i_045ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45ADE);
    cpu.pc = 0x13600;
    16
}
/// $045ADE: move.w #$2, d1
pub fn i_045ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AE2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045AE2: jsr $236e.l
pub fn i_045ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45AE8);
    cpu.pc = 0x236E;
    16
}
/// $045AE8: move.b #$ff, $68(a6)
pub fn i_045ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AEE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045AEE: move.b #$ff, $6d(a6)
pub fn i_045aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045AF4: clr.b $3b(a6)
pub fn i_045af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AF8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $045AF8: clr.w $78(a6)
pub fn i_045af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AFC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $045AFC: lea.l $45b02(pc), a1
pub fn i_045afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45B02;
    16
}
/// $045B00: move.l a1, (a6)
pub fn i_045b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Turret_AimSmooth_045B02  ($045B02, 154 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045B02: bsr.w $458dc
pub fn i_045b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B06;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45B06);
    cpu.pc = 0x458DC;
    18
}
/// $045B06: bsr.w $4593a
pub fn i_045b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B0A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45B0A);
    cpu.pc = 0x4593A;
    18
}
/// $045B0A: movea.l $74(a6), a0
pub fn i_045b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B0E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045B0E: move.b $71(a6), d0
pub fn i_045b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B12;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045B12: andi.w #$f, d0
pub fn i_045b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B16;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045B16: add.w d0, d0
pub fn i_045b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B18;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045B18: add.w d0, d0
pub fn i_045b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B1A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045B1A: move.l (a0, d0.w), d0
pub fn i_045b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B1E;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $045B1E: bmi.w $45b2a
pub fn i_045b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B22;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x45B2A; return 10; }
    8
}
/// $045B22: move.w d0, $7e(a6)
pub fn i_045b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B26;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045B26: bra.w $45b2e
pub fn i_045b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x45B2E;
    10
}
/// $045B2A: move.w $7e(a6), d0
pub fn i_045b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045B2E: move.w d0, d1
pub fn i_045b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B30;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045B30: sub.w $34(a6), d0
pub fn i_045b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B34;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045B34: ext.l d0
pub fn i_045b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B36;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $045B36: bpl.w $45b3c
pub fn i_045b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B3A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x45B3C; return 10; }
    8
}
/// $045B3A: neg.w d0
pub fn i_045b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B3C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $045B3C: move.w #$200, d2
pub fn i_045b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B40;
    cpu.ir = 0x343C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045B40: cmpi.w #$5000, d0
pub fn i_045b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B44;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5000, Size::Word, false);
    8
}
/// $045B44: bcs.w $45b4c
pub fn i_045b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B48;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x45B4C; return 10; }
    8
}
/// $045B48: move.w #$800, d2
pub fn i_045b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B4C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045B4C: asr.w #$1, d0
pub fn i_045b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B4E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $045B4E: move.w $78(a6), d1
pub fn i_045b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B52;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045B52: add.w d2, d1
pub fn i_045b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B54;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045B54: cmp.w d0, d1
pub fn i_045b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B56;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $045B56: bcs.w $45b5c
pub fn i_045b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B5A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x45B5C; return 10; }
    8
}
/// $045B5A: move.w d0, d1
pub fn i_045b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B5C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045B5C: move.w d1, $78(a6)
pub fn i_045b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B60;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045B60: swap d0
pub fn i_045b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B62;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $045B62: eor.w d0, d1
pub fn i_045b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B64;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $045B64: sub.w d0, d1
pub fn i_045b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B66;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045B66: add.w d1, $34(a6)
pub fn i_045b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B6A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $045B6A: move.b $7d(a6), d0
pub fn i_045b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B6E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045B6E: beq.w $45b7c
pub fn i_045b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45B7C; return 10; }
    8
}
/// $045B72: subq.b #$1, d0
pub fn i_045b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B74;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045B74: move.b d0, $7d(a6)
pub fn i_045b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B78;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045B78: bra.w $45b94
pub fn i_045b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x45B94;
    10
}
/// $045B7C: move.b #$1, $7d(a6)
pub fn i_045b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B82;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045B82: move.b $7c(a6), d0
pub fn i_045b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B86;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045B86: beq.w $45b94
pub fn i_045b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45B94; return 10; }
    8
}
/// $045B8A: subq.b #$1, d0
pub fn i_045b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B8C;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045B8C: move.b d0, $7c(a6)
pub fn i_045b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B90;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045B90: bsr.w $4580c
pub fn i_045b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B94;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45B94);
    cpu.pc = 0x4580C;
    18
}
/// $045B94: bsr.w $457cc
pub fn i_045b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B98;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45B98);
    cpu.pc = 0x457CC;
    18
}
/// $045B98: bra.w $45b9c
pub fn i_045b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45B9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x45B9C;
    10
}
// ===== Turret_Tail_045B9C  ($045B9C, 118 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045B9C: movea.l $c(a6), a0
pub fn i_045b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BA0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045BA0: move.b $68(a0), $68(a6)
pub fn i_045ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BA6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045BA6: bclr.b #$3, $8c(a6)
pub fn i_045ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045BAC: movem.l a6, -(a7)
pub fn i_045bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BB0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $045BB0: movea.l $c(a6), a6
pub fn i_045bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BB4;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $045BB4: move.b #$8, d1
pub fn i_045bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BB8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $045BB8: jsr $8f714.l
pub fn i_045bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45BBE);
    cpu.pc = 0x8F714;
    16
}
/// $045BBE: movem.l (a7)+, a6
pub fn i_045bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BC2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $045BC2: bcc.w $45bcc
pub fn i_045bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45BCC; return 10; }
    8
}
/// $045BC6: lea.l $45c20(pc), a1
pub fn i_045bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45C20;
    16
}
/// $045BCA: move.l a1, (a6)
pub fn i_045bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045BCC: jsr $5e452.l
pub fn i_045bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45BD2);
    cpu.pc = 0x5E452;
    16
}
/// $045BD2: bcc.w $45be6
pub fn i_045bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45BE6; return 10; }
    8
}
/// $045BD6: movea.l $c(a6), a0
pub fn i_045bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BDA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045BDA: btst.b #$0, $13(a0)
pub fn i_045bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045BE0: bne.w $45be6
pub fn i_045be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45BE6; return 10; }
    8
}
/// $045BE4: rts
pub fn i_045be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $045BE6: move.l a6, -(a7)
pub fn i_045be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE8;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $045BE8: lea.l $100800.l, a6
pub fn i_045be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BEE;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $045BEE: lea.l $45d3a(pc), a1
pub fn i_045bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45D3A;
    16
}
/// $045BF2: jsr $4ae.l
pub fn i_045bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45BF8);
    cpu.pc = 0x4AE;
    16
}
/// $045BF8: movea.l (a7)+, a6
pub fn i_045bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BFA;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $045BFA: move.w $22(a6), $22(a0)
pub fn i_045bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C00;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $045C00: move.w $24(a6), $24(a0)
pub fn i_045c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C06;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $045C06: move.b $68(a6), $68(a0)
pub fn i_045c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C0C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045C0C: move.b $6d(a6), $6d(a0)
pub fn i_045c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C12;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6D);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== Jsr5B6ThenJmpScheduler_045c12  ($045C12, 14 B, kernel/jsr5b6_jmp518.c)
/// $045C12: jsr $5b6.l
pub fn i_045c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45C18);
    cpu.pc = 0x5B6;
    16
}
/// $045C18: jmp $518.l
pub fn i_045c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C1E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Boss2_Explode_045C20  ($045C20, 82 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045C20: move.w #$106a, d0
pub fn i_045c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C24;
    cpu.ir = 0x303C;
    let v1: u32 = 0x106A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045C24: jsr $2352.l
pub fn i_045c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45C2A);
    cpu.pc = 0x2352;
    16
}
/// $045C2A: move.l a6, -(a7)
pub fn i_045c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C2C;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $045C2C: lea.l $100800.l, a6
pub fn i_045c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C32;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $045C32: lea.l $45d3a(pc), a1
pub fn i_045c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45D3A;
    16
}
/// $045C36: jsr $4ae.l
pub fn i_045c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45C3C);
    cpu.pc = 0x4AE;
    16
}
/// $045C3C: movea.l (a7)+, a6
pub fn i_045c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C3E;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $045C3E: move.w $22(a6), $22(a0)
pub fn i_045c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C44;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $045C44: move.w $24(a6), $24(a0)
pub fn i_045c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C4A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $045C4A: move.b $68(a6), $68(a0)
pub fn i_045c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C50;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045C50: move.b $6d(a6), $6d(a0)
pub fn i_045c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C56;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045C56: lea.l $77f6a.l, a1
pub fn i_045c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C5C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $045C5C: jsr $6fe.l
pub fn i_045c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45C62);
    cpu.pc = 0x6FE;
    16
}
/// $045C62: jsr $5dd02.l
pub fn i_045c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45C68);
    cpu.pc = 0x5DD02;
    16
}
/// $045C68: clr.w $7a(a6)
pub fn i_045c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C6C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $045C6C: lea.l $45c72(pc), a1
pub fn i_045c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45C72;
    16
}
/// $045C70: move.l a1, (a6)
pub fn i_045c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Boss2_Fall_045C72  ($045C72, 38 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045C72: move.w $106f6c.l, d0
pub fn i_045c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C78;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $045C78: bpl.w $45c7e
pub fn i_045c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C7C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x45C7E; return 10; }
    8
}
/// $045C7C: neg.w d0
pub fn i_045c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C7E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $045C7E: add.w $7a(a6), d0
pub fn i_045c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C82;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045C82: move.w d0, $7a(a6)
pub fn i_045c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C86;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045C86: cmpi.w #$1e0, d0
pub fn i_045c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C8A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1E0, Size::Word, false);
    8
}
/// $045C8A: ble.w $45c94
pub fn i_045c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C8E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x45C94; return 10; }
    8
}
/// $045C8E: lea.l $45c98(pc), a1
pub fn i_045c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45C98;
    16
}
/// $045C92: move.l a1, (a6)
pub fn i_045c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045C94: bra.w $45cc2
pub fn i_045c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C98;
    cpu.ir = 0x6000;
    cpu.pc = 0x45CC2;
    10
}
// ===== Boss2_FlagSwap_045C98  ($045C98, 22 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045C98: movea.l $c(a6), a0
pub fn i_045c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C9C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045C9C: bset.b #$3, $8c(a0)
pub fn i_045c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CA2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045CA2: bclr.b #$4, $8c(a0)
pub fn i_045ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CA8;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045CA8: lea.l $45cae(pc), a1
pub fn i_045ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45CAE;
    16
}
/// $045CAC: move.l a1, (a6)
pub fn i_045cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Boss2_WaitBit4_045CAE  ($045CAE, 20 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045CAE: movea.l $c(a6), a0
pub fn i_045cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CB2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045CB2: btst.b #$4, $8c(a0)
pub fn i_045cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CB8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045CB8: beq.w $45cc2
pub fn i_045cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CBC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45CC2; return 10; }
    8
}
/// $045CBC: lea.l $45cea(pc), a1
pub fn i_045cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45CEA;
    16
}
/// $045CC0: move.l a1, (a6)
pub fn i_045cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Boss2_TickParent_045CC2  ($045CC2, 20 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045CC2: move.l a6, -(a7)
pub fn i_045cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CC4;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $045CC4: movea.l $c(a6), a6
pub fn i_045cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CC8;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $045CC8: move.b #$1, d1
pub fn i_045cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CCC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $045CCC: jsr $8f6f2.l
pub fn i_045ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45CD2);
    cpu.pc = 0x8F6F2;
    16
}
/// $045CD2: movea.l (a7)+, a6
pub fn i_045cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CD4;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $045CD4: rts
pub fn i_045cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss2_Death_045CEA  ($045CEA, 80 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045CEA: jsr $13600.l
pub fn i_045cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45CF0);
    cpu.pc = 0x13600;
    16
}
/// $045CF0: bsr.w $457a6
pub fn i_045cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CF4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45CF4);
    cpu.pc = 0x457A6;
    18
}
/// $045CF4: move.w #$106a, d0
pub fn i_045cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x106A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045CF8: jsr $2352.l
pub fn i_045cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45CFE);
    cpu.pc = 0x2352;
    16
}
/// $045CFE: lea.l $29d3a8.l, a0
pub fn i_045cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D3A8;
    20
}
/// $045D04: jsr $28cd4.l
pub fn i_045d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D0A);
    cpu.pc = 0x28CD4;
    16
}
/// $045D0A: lea.l $45d10(pc), a1
pub fn i_045d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45D10;
    16
}
/// $045D0E: move.l a1, (a6)
pub fn i_045d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045D10: bsr.w $458dc
pub fn i_045d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D14;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45D14);
    cpu.pc = 0x458DC;
    18
}
/// $045D14: jsr $28d70.l
pub fn i_045d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D1A);
    cpu.pc = 0x28D70;
    16
}
/// $045D1A: bcc.w $45d2e
pub fn i_045d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45D2E; return 10; }
    8
}
/// $045D1E: move.b $70(a6), d0
pub fn i_045d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D22;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045D22: andi.w #$ff, d0
pub fn i_045d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D26;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045D26: add.w d0, d0
pub fn i_045d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D28;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045D28: add.w d0, d0
pub fn i_045d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D2A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045D2A: lea.l $45cd6(pc), a0
pub fn i_045d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D2E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x45CD6;
    16
}
/// $045D2E: jsr $5e452.l
pub fn i_045d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D34);
    cpu.pc = 0x5E452;
    16
}
/// $045D34: bcs.w $45be6
pub fn i_045d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D38;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x45BE6; return 10; }
    8
}
/// $045D38: bra.b $45cc2
pub fn i_045d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D3A;
    cpu.ir = 0x6088;
    cpu.pc = 0x45CC2;
    10
}
// ===== Boss2Shot_Init_045D3A  ($045D3A, 82 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045D3A: bsr.w $457a6
pub fn i_045d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D3E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45D3E);
    cpu.pc = 0x457A6;
    18
}
/// $045D3E: move.w #$d000, d0
pub fn i_045d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D42;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045D42: jsr $28134.l
pub fn i_045d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D48);
    cpu.pc = 0x28134;
    16
}
/// $045D48: andi.w #$ffe3, $38(a6)
pub fn i_045d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D4E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $045D4E: ori.w #$10, $38(a6)
pub fn i_045d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D54;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $045D54: move.w #$38, d0
pub fn i_045d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D58;
    cpu.ir = 0x303C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045D58: jsr $5dca4.l
pub fn i_045d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D5E);
    cpu.pc = 0x5DCA4;
    16
}
/// $045D5E: move.w d0, $28(a6)
pub fn i_045d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D62;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045D62: move.w #$384, $2a(a6)
pub fn i_045d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x384;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045D68: move.w #$ffec, $2e(a6)
pub fn i_045d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045D6E: move.w #$0, $2c(a6)
pub fn i_045d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045D74: move.b #$3c, $45(a6)
pub fn i_045d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045D7A: lea.l $29d090.l, a0
pub fn i_045d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D090;
    20
}
/// $045D80: jsr $28cd4.l
pub fn i_045d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D86);
    cpu.pc = 0x28CD4;
    16
}
/// $045D86: lea.l $45d8c(pc), a1
pub fn i_045d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45D8C;
    16
}
/// $045D8A: move.l a1, (a6)
pub fn i_045d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Boss2Shot_Fly_045D8C  ($045D8C, 58 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045D8C: jsr $27d50.l
pub fn i_045d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D92);
    cpu.pc = 0x27D50;
    16
}
/// $045D92: jsr $28d70.l
pub fn i_045d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45D98);
    cpu.pc = 0x28D70;
    16
}
/// $045D98: tst.b $45(a6)
pub fn i_045d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45D9C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $045D9C: bne.w $45db6
pub fn i_045d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45DB6; return 10; }
    8
}
/// $045DA0: move.w #$1022, d0
pub fn i_045da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045DA4: jsr $2352.l
pub fn i_045da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45DAA);
    cpu.pc = 0x2352;
    16
}
/// $045DAA: jsr $13600.l
pub fn i_045daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45DB0);
    cpu.pc = 0x13600;
    16
}
/// $045DB0: jmp $77f6a.l
pub fn i_045db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DB6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
/// $045DB6: movea.l #$ffffffff, a0
pub fn i_045db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DBC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $045DBC: jsr $5dd56.l
pub fn i_045dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45DC2);
    cpu.pc = 0x5DD56;
    16
}
/// $045DC2: bcc.w $45dd2
pub fn i_045dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45DD2; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_045dc6  ($045DC6, 14 B, kernel/jsr5b6_jmp518.c)
/// $045DC6: jsr $5b6.l
pub fn i_045dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45DCC);
    cpu.pc = 0x5B6;
    16
}
/// $045DCC: jmp $518.l
pub fn i_045dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DD2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $045DD2: rts
pub fn i_045dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Miniboss2_Attach_045DE4  ($045DE4, 94 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045DE4: lea.l $100580.l, a0
pub fn i_045de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $045DEA: cmpi.l #$2ae3e, (a0)
pub fn i_045dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DF0;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2AE3E, Size::Long, false);
    16
}
/// $045DF0: bne.w $45dfa
pub fn i_045df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DF4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45DFA; return 10; }
    8
}
/// $045DF4: jmp $518.l
pub fn i_045df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45DFA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $045DFA: btst.b #$3, $8c(a0)
pub fn i_045dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E00;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045E00: bne.w $45e0a
pub fn i_045e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45E0A; return 10; }
    8
}
/// $045E04: jmp $518.l
pub fn i_045e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E0A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $045E0A: move.w #$2, d1
pub fn i_045e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E0E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045E0E: jsr $236e.l
pub fn i_045e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45E14);
    cpu.pc = 0x236E;
    16
}
/// $045E14: bset.b #$2, $5b(a6)
pub fn i_045e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E1A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045E1A: move.w #$c000, d0
pub fn i_045e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E1E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045E1E: jsr $28134.l
pub fn i_045e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45E24);
    cpu.pc = 0x28134;
    16
}
/// $045E24: andi.w #$ffe3, $38(a6)
pub fn i_045e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E2A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $045E2A: ori.w #$10, $38(a6)
pub fn i_045e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E30;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $045E30: lea.l $29d090.l, a0
pub fn i_045e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D090;
    20
}
/// $045E36: jsr $28cd4.l
pub fn i_045e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45E3C);
    cpu.pc = 0x28CD4;
    16
}
/// $045E3C: lea.l $45e42(pc), a1
pub fn i_045e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45E42;
    16
}
/// $045E40: move.l a1, (a6)
pub fn i_045e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Miniboss2_Ride_045E42  ($045E42, 76 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045E42: jsr $27c8c.l
pub fn i_045e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45E48);
    cpu.pc = 0x27C8C;
    16
}
/// $045E48: bcc.w $45e70
pub fn i_045e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45E70; return 10; }
    8
}
/// $045E4C: lea.l $45ea4(pc), a1
pub fn i_045e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45EA4;
    16
}
/// $045E50: move.l a1, (a6)
pub fn i_045e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045E52: cmpi.b #$0, $106ece.l
pub fn i_045e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E5A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $045E5A: bne.w $45e70
pub fn i_045e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E5E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45E70; return 10; }
    8
}
/// $045E5E: andi.b #$c0, d7
pub fn i_045e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E62;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $045E62: cmpi.b #$40, d7
pub fn i_045e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E66;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $045E66: bne.w $45e70
pub fn i_045e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E6A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45E70; return 10; }
    8
}
/// $045E6A: lea.l $45e8e(pc), a1
pub fn i_045e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45E8E;
    16
}
/// $045E6E: move.l a1, (a6)
pub fn i_045e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045E70: jsr $28d70.l
pub fn i_045e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45E76);
    cpu.pc = 0x28D70;
    16
}
/// $045E76: movea.l #$ffffffff, a0
pub fn i_045e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E7C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $045E7C: jsr $5dd5c.l
pub fn i_045e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45E82);
    cpu.pc = 0x5DD5C;
    16
}
/// $045E82: bcc.w $45e8c
pub fn i_045e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45E8C; return 10; }
    8
}
/// $045E86: jmp $518.l
pub fn i_045e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E8C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $045E8C: rts
pub fn i_045e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Miniboss2_Dead_045E8E  ($045E8E, 22 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045E8E: lea.l $29d234.l, a0
pub fn i_045e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D234;
    20
}
/// $045E94: jsr $28cd4.l
pub fn i_045e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45E9A);
    cpu.pc = 0x28CD4;
    16
}
/// $045E9A: lea.l $45eea(pc), a1
pub fn i_045e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45E9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45EEA;
    16
}
/// $045E9E: move.l a1, (a6)
pub fn i_045e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045EA0: bra.w $45eea
pub fn i_045ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x45EEA;
    10
}
// ===== Miniboss2_Hop_045EA4  ($045EA4, 128 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045EA4: lea.l $29d178.l, a0
pub fn i_045ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D178;
    20
}
/// $045EAA: jsr $28cd4.l
pub fn i_045eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45EB0);
    cpu.pc = 0x28CD4;
    16
}
/// $045EB0: move.w #$10, d0
pub fn i_045eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045EB4: jsr $5dca4.l
pub fn i_045eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45EBA);
    cpu.pc = 0x5DCA4;
    16
}
/// $045EBA: move.w d0, $28(a6)
pub fn i_045eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EBE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045EBE: move.w #$480, $2a(a6)
pub fn i_045ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x480;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045EC4: move.w #$ff70, $2e(a6)
pub fn i_045ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ECA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045ECA: move.w #$0, $2c(a6)
pub fn i_045eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ED0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045ED0: lea.l $45ed6(pc), a1
pub fn i_045ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ED4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45ED6;
    16
}
/// $045ED4: move.l a1, (a6)
pub fn i_045ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ED6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045ED6: jsr $27bc8.l
pub fn i_045ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45EDC);
    cpu.pc = 0x27BC8;
    16
}
/// $045EDC: bcc.w $45ee6
pub fn i_045edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45EE6; return 10; }
    8
}
/// $045EE0: lea.l $45eea(pc), a1
pub fn i_045ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45EEA;
    16
}
/// $045EE4: move.l a1, (a6)
pub fn i_045ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045EE6: bra.w $45ef0
pub fn i_045ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x45EF0;
    10
}
/// $045EEA: jsr $2783a.l
pub fn i_045eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45EF0);
    cpu.pc = 0x2783A;
    16
}
/// $045EF0: jsr $28d70.l
pub fn i_045ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45EF6);
    cpu.pc = 0x28D70;
    16
}
/// $045EF6: movea.l #$ffffffff, a0
pub fn i_045ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45EFC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $045EFC: jsr $5dd5c.l
pub fn i_045efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45F02);
    cpu.pc = 0x5DD5C;
    16
}
/// $045F02: bcc.w $45f0c
pub fn i_045f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45F0C; return 10; }
    8
}
/// $045F06: jmp $518.l
pub fn i_045f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F0C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $045F0C: lea.l $100580.l, a1
pub fn i_045f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $045F12: lea.l $45dd4(pc), a2
pub fn i_045f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F16;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x45DD4;
    16
}
/// $045F16: lea.l $45ddc(pc), a0
pub fn i_045f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F1A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x45DDC;
    16
}
/// $045F1A: jsr $28c20.l
pub fn i_045f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45F20);
    cpu.pc = 0x28C20;
    16
}
/// $045F20: bcc.w $45f2a
pub fn i_045f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45F2A; return 10; }
    8
}
// ===== SetTaskHandler_045f24  ($045F24, 8 B, kernel/task_handlers.c)
/// $045F24: lea.l $45f2c(pc), a1
pub fn i_045f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45F2C;
    16
}
/// $045F28: move.l a1, (a6)
pub fn i_045f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045F2A: rts
pub fn i_045f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_KillFlag10060C_045F2C  ($045F2C, 14 B, enemies/vehicles/deploy_045f2c.s)
/// $045F2C: bset.b #$4, $10060c.l
pub fn i_045f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F34;
    cpu.ir = 0x8F9;
    let mk: u32 = 1u32 << ((4) % 8);
    let v1 = bus.read8(0x10060C) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    bus.write8(0x10060C, r2 as u8);
    20
}
/// $045F34: jmp $518.l
pub fn i_045f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45F3A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Vehicle_JmpDeploy_045FDE  ($045FDE, 6 B, enemies/vehicles/deploy_045f2c.s)
/// $045FDE: jmp $308b0.l
pub fn i_045fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45FE4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x308B0;
    8
}
// ===== Vehicle_Launch_045FE4  ($045FE4, 148 B, enemies/vehicles/deploy_045f2c.s)
/// $045FE4: bset.b #$4, $6b(a6)
pub fn i_045fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45FEA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045FEA: lea.l $30804.l, a0
pub fn i_045fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45FF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x30804;
    20
}
/// $045FF0: move.l a0, $4c(a6)
pub fn i_045ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45FF4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $045FF4: jsr $283ca.l
pub fn i_045ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45FFA);
    cpu.pc = 0x283CA;
    16
}
/// $045FFA: move.w #$d000, $38(a6)
pub fn i_045ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46000;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046000: lea.l $77a96.l, a1
pub fn i_046000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46006;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77A96;
    20
}
/// $046006: jsr $4ae.l
pub fn i_046006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4600C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4600C);
    cpu.pc = 0x4AE;
    16
}
/// $04600C: jsr $5dd02.l
pub fn i_04600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46012);
    cpu.pc = 0x5DD02;
    16
}
/// $046012: move.b $98(a6), d0
pub fn i_046012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46016;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046016: move.w d0, d1
pub fn i_046016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46018;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046018: moveq #$0, d0
pub fn i_046018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4601A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04601A: andi.w #$3, d0
pub fn i_04601a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4601E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04601E: subq.w #$2, d0
pub fn i_04601e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46020;
    cpu.ir = 0x5540;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046020: add.w d1, d0
pub fn i_046020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46022;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046022: andi.w #$ff, d0
pub fn i_046022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46026;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046026: move.w #$800, d1
pub fn i_046026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4602A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04602A: jsr $13c0e.l
pub fn i_04602a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46030;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46030);
    cpu.pc = 0x13C0E;
    16
}
/// $046030: move.w d1, $28(a6)
pub fn i_046030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46034;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046034: move.w d2, $2a(a6)
pub fn i_046034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46038;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046038: move.w #$2, d1
pub fn i_046038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4603C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04603C: jsr $236e.l
pub fn i_04603c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46042);
    cpu.pc = 0x236E;
    16
}
/// $046042: move.b $98(a6), d0
pub fn i_046042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46046;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046046: addi.w #$8, d0
pub fn i_046046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4604A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04604A: andi.w #$f0, d0
pub fn i_04604a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4604E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04604E: lsr.w #$4, d0
pub fn i_04604e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46050;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $046050: movea.l #$29d452, a0
pub fn i_046050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46056;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29D452;
    cpu.a[0] = v1;
    4
}
/// $046056: lsl.w #$2, d0
pub fn i_046056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46058;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $046058: movea.l (a0, d0.w), a0
pub fn i_046058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4605C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04605C: cmpa.l #$ffffffff, a0
pub fn i_04605c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46062;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $046062: beq.w $4606c
pub fn i_046062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46066;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4606C; return 10; }
    8
}
/// $046066: jsr $28cd4.l
pub fn i_046066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4606C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4606C);
    cpu.pc = 0x28CD4;
    16
}
/// $04606C: jsr $283ca.l
pub fn i_04606c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46072);
    cpu.pc = 0x283CA;
    16
}
/// $046072: lea.l $46078(pc), a1
pub fn i_046072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46076;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46078;
    16
}
/// $046076: move.l a1, (a6)
pub fn i_046076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46078;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Vehicle_Flight_046078  ($046078, 118 B, enemies/vehicles/deploy_045f2c.s)
/// $046078: bset.b #$6, $13(a6)
pub fn i_046078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4607E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04607E: jsr $27cee.l
pub fn i_04607e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46084;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46084);
    cpu.pc = 0x27CEE;
    16
}
/// $046084: bcs.w $46092
pub fn i_046084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46088;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x46092; return 10; }
    8
}
/// $046088: jsr $27cee.l
pub fn i_046088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4608E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4608E);
    cpu.pc = 0x27CEE;
    16
}
/// $04608E: bcc.w $460ba
pub fn i_04608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46092;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x460BA; return 10; }
    8
}
/// $046092: lea.l $460ee(pc), a1
pub fn i_046092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x460EE;
    16
}
/// $046096: move.l a1, (a6)
pub fn i_046096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46098;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046098: cmpi.b #$38, d0
pub fn i_046098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4609C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x38, Size::Byte, false);
    8
}
/// $04609C: beq.w $460b4
pub fn i_04609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x460B4; return 10; }
    8
}
/// $0460A0: andi.b #$c0, d7
pub fn i_0460a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460A4;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0460A4: cmpi.b #$0, d7
pub fn i_0460a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460A8;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0460A8: beq.w $460ba
pub fn i_0460a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x460BA; return 10; }
    8
}
/// $0460AC: cmpi.b #$c0, d7
pub fn i_0460ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460B0;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Byte, false);
    8
}
/// $0460B0: beq.w $460ba
pub fn i_0460b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x460BA; return 10; }
    8
}
/// $0460B4: jmp $518.l
pub fn i_0460b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460BA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0460BA: jsr $28d70.l
pub fn i_0460ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x460C0);
    cpu.pc = 0x28D70;
    16
}
/// $0460C0: jsr $283d8.l
pub fn i_0460c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x460C6);
    cpu.pc = 0x283D8;
    16
}
/// $0460C6: btst.b #$1, $13(a6)
pub fn i_0460c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460CC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0460CC: beq.w $460d6
pub fn i_0460cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x460D6; return 10; }
    8
}
/// $0460D0: lea.l $460fa(pc), a1
pub fn i_0460d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x460FA;
    16
}
/// $0460D4: move.l a1, (a6)
pub fn i_0460d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0460D6: movea.l #$ffffffff, a0
pub fn i_0460d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460DC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0460DC: jsr $5dd56.l
pub fn i_0460dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x460E2);
    cpu.pc = 0x5DD56;
    16
}
/// $0460E2: bcc.w $460ec
pub fn i_0460e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x460EC; return 10; }
    8
}
/// $0460E6: jmp $518.l
pub fn i_0460e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0460EC: rts
pub fn i_0460ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Vehicle_CrashA_0460EE  ($0460EE, 12 B, enemies/vehicles/deploy_045f2c.s)
/// $0460EE: jsr $13600.l
pub fn i_0460ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x460F4);
    cpu.pc = 0x13600;
    16
}
/// $0460F4: jmp $31c72.l
pub fn i_0460f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460FA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x31C72;
    8
}
// ===== Vehicle_CrashB_0460FA  ($0460FA, 12 B, enemies/vehicles/deploy_045f2c.s)
/// $0460FA: jsr $13600.l
pub fn i_0460fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46100);
    cpu.pc = 0x13600;
    16
}
/// $046100: jmp $31cca.l
pub fn i_046100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46106;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x31CCA;
    8
}
// ===== Stub_00046106  ($046106, 2 B, enemies/vehicles/deploy_045f2c.s)
/// $046106: rts
pub fn i_046106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46108;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_046108  ($046108, 16 B, enemies/vehicles/deploy_045f2c.s)
/// $046108: movea.l $8(a6), a1
pub fn i_046108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4610C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04610C: move.b $10(a6), d0
pub fn i_04610c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46110;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046110: cmp.b $10(a1), d0
pub fn i_046110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46114;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $046114: bcs.w $4611e
pub fn i_046114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46118;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4611E; return 10; }
    8
}
// ===== ClearXN_046118  ($046118, 6 B, thunks/ccr_helpers.c)
/// $046118: andi.b #$ee, ccr
pub fn i_046118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4611C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04611C: rts
pub fn i_04611c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4611E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04611e  ($04611E, 6 B, thunks/ccr_helpers.c)
/// $04611E: ori.b #$11, ccr
pub fn i_04611e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46122;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $046122: rts
pub fn i_046122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46124;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_046124  ($046124, 16 B, enemies/vehicles/deploy_045f2c.s)
/// $046124: movea.l $8(a6), a1
pub fn i_046124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46128;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $046128: move.b $10(a6), d0
pub fn i_046128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4612C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04612C: cmp.b $10(a1), d0
pub fn i_04612c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46130;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $046130: bcs.w $4613a
pub fn i_046130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46134;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4613A; return 10; }
    8
}
// ===== ClearXN_046134  ($046134, 6 B, thunks/ccr_helpers.c)
/// $046134: andi.b #$ee, ccr
pub fn i_046134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46138;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $046138: rts
pub fn i_046138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4613A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04613a  ($04613A, 6 B, thunks/ccr_helpers.c)
/// $04613A: ori.b #$11, ccr
pub fn i_04613a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4613E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04613E: rts
pub fn i_04613e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46140;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Enemy46_Boot_046140  ($046140, 82 B, enemies/vehicles/deploy_045f2c.s)
/// $046140: jsr $5e7c0.l
pub fn i_046140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46146);
    cpu.pc = 0x5E7C0;
    16
}
/// $046146: move.w #$1a, d1
pub fn i_046146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4614A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04614A: tst.b $9a(a6)
pub fn i_04614a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4614E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04614E: beq.w $46156
pub fn i_04614e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46152;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x46156; return 10; }
    8
}
/// $046152: move.w #$19f, d1
pub fn i_046152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46156;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046156: jsr $236e.l
pub fn i_046156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4615C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4615C);
    cpu.pc = 0x236E;
    16
}
/// $04615C: move.b $99(a6), $3a(a6)
pub fn i_04615c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46162;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $046162: move.w #$8000, d0
pub fn i_046162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46166;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046166: jsr $28134.l
pub fn i_046166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4616C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4616C);
    cpu.pc = 0x28134;
    16
}
/// $04616C: andi.w #$ffe3, $38(a6)
pub fn i_04616c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46172;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $046172: ori.w #$18, $38(a6)
pub fn i_046172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46178;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $046178: tst.b $9d(a6)
pub fn i_046178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4617C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04617C: beq.w $461bc
pub fn i_04617c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46180;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x461BC; return 10; }
    8
}
/// $046180: lea.l $2e2286.l, a0
pub fn i_046180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46186;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2286;
    20
}
/// $046186: jsr $28cd4.l
pub fn i_046186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4618C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4618C);
    cpu.pc = 0x28CD4;
    16
}
/// $04618C: lea.l $46192(pc), a1
pub fn i_04618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46190;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46192;
    16
}
/// $046190: move.l a1, (a6)
pub fn i_046190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46192;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Enemy46_PhaseA_046192  ($046192, 42 B, enemies/vehicles/deploy_045f2c.s)
/// $046192: jsr $2783a.l
pub fn i_046192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46198;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46198);
    cpu.pc = 0x2783A;
    16
}
/// $046198: jsr $27eba.l
pub fn i_046198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4619E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4619E);
    cpu.pc = 0x27EBA;
    16
}
/// $04619E: bcc.w $461a8
pub fn i_04619e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x461A8; return 10; }
    8
}
/// $0461A2: jsr $27c8c.l
pub fn i_0461a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x461A8);
    cpu.pc = 0x27C8C;
    16
}
/// $0461A8: jsr $28d70.l
pub fn i_0461a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x461AE);
    cpu.pc = 0x28D70;
    16
}
/// $0461AE: bcc.w $461b8
pub fn i_0461ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x461B8; return 10; }
    8
}
/// $0461B2: lea.l $461bc(pc), a1
pub fn i_0461b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x461BC;
    16
}
/// $0461B6: move.l a1, (a6)
pub fn i_0461b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0461B8: bra.w $46220
pub fn i_0461b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x46220;
    10
}
// ===== Enemy46_Move_0461BC  ($0461BC, 46 B, enemies/vehicles/deploy_045f2c.s)
/// $0461BC: lea.l $2b7716.l, a0
pub fn i_0461bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7716;
    20
}
/// $0461C2: jsr $799de.l
pub fn i_0461c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x461C8);
    cpu.pc = 0x799DE;
    16
}
/// $0461C8: btst.b #$0, $3a(a6)
pub fn i_0461c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461CE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0461CE: bne.w $461d4
pub fn i_0461ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x461D4; return 10; }
    8
}
/// $0461D2: neg.w d0
pub fn i_0461d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461D4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0461D4: move.w d0, $28(a6)
pub fn i_0461d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461D8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0461D8: lea.l $29b744.l, a0
pub fn i_0461d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $0461DE: jsr $28cd4.l
pub fn i_0461de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x461E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0461E4: lea.l $461ea(pc), a1
pub fn i_0461e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x461EA;
    16
}
/// $0461E8: move.l a1, (a6)
pub fn i_0461e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Enemy46_PhaseB_0461EA  ($0461EA, 54 B, enemies/vehicles/deploy_045f2c.s)
/// $0461EA: jsr $2783a.l
pub fn i_0461ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x461F0);
    cpu.pc = 0x2783A;
    16
}
/// $0461F0: jsr $27eba.l
pub fn i_0461f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x461F6);
    cpu.pc = 0x27EBA;
    16
}
/// $0461F6: bcc.w $46204
pub fn i_0461f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x461FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46204; return 10; }
    8
}
/// $0461FA: jsr $27c8c.l
pub fn i_0461fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46200);
    cpu.pc = 0x27C8C;
    16
}
/// $046200: bra.w $4621a
pub fn i_046200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46204;
    cpu.ir = 0x6000;
    cpu.pc = 0x4621A;
    10
}
/// $046204: jsr $27a92.l
pub fn i_046204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4620A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4620A);
    cpu.pc = 0x27A92;
    16
}
/// $04620A: btst.b #$5, $5a(a6)
pub fn i_04620a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46210;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $046210: beq.w $4621a
pub fn i_046210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46214;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4621A; return 10; }
    8
}
/// $046214: lea.l $46260(pc), a1
pub fn i_046214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46218;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46260;
    16
}
/// $046218: move.l a1, (a6)
pub fn i_046218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4621A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04621A: jsr $28d70.l
pub fn i_04621a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46220);
    cpu.pc = 0x28D70;
    16
}
// ===== Enemy46_Tail_046220  ($046220, 56 B, enemies/vehicles/deploy_045f2c.s)
/// $046220: jsr $2870a.l
pub fn i_046220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46226);
    cpu.pc = 0x2870A;
    16
}
/// $046226: bcc.w $46248
pub fn i_046226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4622A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46248; return 10; }
    8
}
/// $04622A: jsr $463c2(pc)
pub fn i_04622a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4622E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4622E);
    cpu.pc = 0x463C2;
    16
}
/// $04622E: tst.b $9a(a6)
pub fn i_04622e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46232;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $046232: bne.w $46240
pub fn i_046232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46236;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46240; return 10; }
    8
}
/// $046236: jsr $49fd0.l
pub fn i_046236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4623C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4623C);
    cpu.pc = 0x49FD0;
    16
}
/// $04623C: bra.w $46248
pub fn i_04623c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46240;
    cpu.ir = 0x6000;
    cpu.pc = 0x46248;
    10
}
/// $046240: lea.l $4abc0.l, a1
pub fn i_046240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46246;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4ABC0;
    20
}
/// $046246: move.l a1, (a6)
pub fn i_046246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46248;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046248: movea.l #$ffffffff, a0
pub fn i_046248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4624E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04624E: jsr $5dd5c.l
pub fn i_04624e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46254;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46254);
    cpu.pc = 0x5DD5C;
    16
}
/// $046254: bcc.w $4625e
pub fn i_046254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46258;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4625E; return 10; }
    8
}
// ===== SetTaskHandler_046258  ($046258, 8 B, kernel/task_handlers.c)
/// $046258: lea.l $463ba(pc), a1
pub fn i_046258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4625C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x463BA;
    16
}
/// $04625C: move.l a1, (a6)
pub fn i_04625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4625E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04625E: rts
pub fn i_04625e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46260;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Enemy46_PhaseC_046260  ($046260, 42 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046260: lea.l $29b7c8.l, a0
pub fn i_046260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B7C8;
    20
}
/// $046266: jsr $28cd4.l
pub fn i_046266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4626C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4626C);
    cpu.pc = 0x28CD4;
    16
}
/// $04626C: lea.l $46272(pc), a1
pub fn i_04626c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46270;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46272;
    16
}
/// $046270: move.l a1, (a6)
pub fn i_046270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46272;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046272: jsr $2783a.l
pub fn i_046272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46278;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46278);
    cpu.pc = 0x2783A;
    16
}
/// $046278: jsr $28d70.l
pub fn i_046278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4627E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4627E);
    cpu.pc = 0x28D70;
    16
}
/// $04627E: bcc.w $46288
pub fn i_04627e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46282;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46288; return 10; }
    8
}
/// $046282: lea.l $4628a(pc), a1
pub fn i_046282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46286;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4628A;
    16
}
/// $046286: move.l a1, (a6)
pub fn i_046286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46288;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046288: bra.b $46220
pub fn i_046288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4628A;
    cpu.ir = 0x6096;
    cpu.pc = 0x46220;
    10
}
// ===== Enemy46_PhaseD_Flip_04628a  ($04628A, 54 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04628A: addq.w #$1, $80(a6)
pub fn i_04628a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4628E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04628E: lea.l $29bfb8.l, a0
pub fn i_04628e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46294;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFB8;
    20
}
/// $046294: jsr $28cd4.l
pub fn i_046294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4629A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4629A);
    cpu.pc = 0x28CD4;
    16
}
/// $04629A: lea.l $462a0(pc), a1
pub fn i_04629a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4629E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x462A0;
    16
}
/// $04629E: move.l a1, (a6)
pub fn i_04629e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0462A0: jsr $2783a.l
pub fn i_0462a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x462A6);
    cpu.pc = 0x2783A;
    16
}
/// $0462A6: jsr $28d70.l
pub fn i_0462a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x462AC);
    cpu.pc = 0x28D70;
    16
}
/// $0462AC: bcc.w $462bc
pub fn i_0462ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x462BC; return 10; }
    8
}
/// $0462B0: eori.b #$1, $3a(a6)
pub fn i_0462b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462B6;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0462B6: lea.l $462c0(pc), a1
pub fn i_0462b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x462C0;
    16
}
/// $0462BA: move.l a1, (a6)
pub fn i_0462ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0462BC: bra.w $46220
pub fn i_0462bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x46220;
    10
}
// ===== Enemy46_RandomPause_0462c0  ($0462C0, 98 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0462C0: jsr $5e9b6.l
pub fn i_0462c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x462C6);
    cpu.pc = 0x5E9B6;
    16
}
/// $0462C6: andi.w #$1f, d0
pub fn i_0462c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462CA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0462CA: addi.w #$14, d0
pub fn i_0462ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462CE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x14, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0462CE: move.w d0, $70(a6)
pub fn i_0462ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462D2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0462D2: jsr $5e9b6.l
pub fn i_0462d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x462D8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0462D8: andi.w #$3, d0
pub fn i_0462d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0462DC: movea.l #$28dc36, a0
pub fn i_0462dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462E2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28DC36;
    cpu.a[0] = v1;
    4
}
/// $0462E2: lsl.w #$2, d0
pub fn i_0462e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462E4;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0462E4: movea.l (a0, d0.w), a0
pub fn i_0462e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462E8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0462E8: cmpa.l #$ffffffff, a0
pub fn i_0462e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462EE;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0462EE: beq.w $462f8
pub fn i_0462ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x462F8; return 10; }
    8
}
/// $0462F2: jsr $28cd4.l
pub fn i_0462f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x462F8);
    cpu.pc = 0x28CD4;
    16
}
/// $0462F8: lea.l $462fe(pc), a1
pub fn i_0462f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x462FE;
    16
}
/// $0462FC: move.l a1, (a6)
pub fn i_0462fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x462FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0462FE: jsr $2783a.l
pub fn i_0462fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46304);
    cpu.pc = 0x2783A;
    16
}
/// $046304: jsr $28d70.l
pub fn i_046304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4630A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4630A);
    cpu.pc = 0x28D70;
    16
}
/// $04630A: subq.w #$1, $70(a6)
pub fn i_04630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4630E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04630E: cmpi.w #$0, $70(a6)
pub fn i_04630e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46314;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $046314: bgt.w $4631e
pub fn i_046314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46318;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4631E; return 10; }
    8
}
/// $046318: lea.l $461bc(pc), a1
pub fn i_046318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4631C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x461BC;
    16
}
/// $04631C: move.l a1, (a6)
pub fn i_04631c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4631E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04631E: bra.w $46220
pub fn i_04631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46322;
    cpu.ir = 0x6000;
    cpu.pc = 0x46220;
    10
}
// ===== Drop_SpawnRandom_046322  ($046322, 144 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046322: move.b $98(a6), d0
pub fn i_046322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46326;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046326: tst.b d0
pub fn i_046326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46328;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $046328: bne.w $46346
pub fn i_046328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4632C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46346; return 10; }
    8
}
/// $04632C: jsr $5e9b6.l
pub fn i_04632c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46332);
    cpu.pc = 0x5E9B6;
    16
}
/// $046332: andi.b #$1, d0
pub fn i_046332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46336;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $046336: move.b d0, $99(a6)
pub fn i_046336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4633A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04633A: jsr $5e9b6.l
pub fn i_04633a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46340);
    cpu.pc = 0x5E9B6;
    16
}
/// $046340: andi.w #$7, d0
pub fn i_046340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46344;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046344: addq.b #$1, d0
pub fn i_046344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46346;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $046346: subq.b #$1, d0
pub fn i_046346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46348;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $046348: andi.w #$f, d0
pub fn i_046348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4634C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04634C: move.b d0, $98(a6)
pub fn i_04634c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46350;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046350: lsl.w #$1, d0
pub fn i_046350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46352;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $046352: lea.l $28dbc6.l, a0
pub fn i_046352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46358;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DBC6;
    20
}
/// $046358: move.b $9a(a6), d1
pub fn i_046358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4635C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04635C: andi.w #$3, d1
pub fn i_04635c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46360;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046360: lsl.w #$2, d1
pub fn i_046360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46362;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $046362: movea.l (a0, d1.w), a0
pub fn i_046362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46366;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $046366: move.w (a0, d0.w), d1
pub fn i_046366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4636A;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04636A: lsl.w #$1, d0
pub fn i_04636a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4636C;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04636C: lea.l $28db86.l, a0
pub fn i_04636c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46372;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DB86;
    20
}
/// $046372: move.l (a0, d0.w), $3c(a6)
pub fn i_046372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46378;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $046378: jsr $236e.l
pub fn i_046378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4637E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4637E);
    cpu.pc = 0x236E;
    16
}
/// $04637E: move.b $99(a6), d0
pub fn i_04637e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46382;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046382: andi.b #$1, d0
pub fn i_046382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46386;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $046386: move.b d0, $3a(a6)
pub fn i_046386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4638A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04638A: lea.l $46390(pc), a1
pub fn i_04638a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4638E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46390;
    16
}
/// $04638E: move.l a1, (a6)
pub fn i_04638e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46390;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046390: jsr $2783a.l
pub fn i_046390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46396;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46396);
    cpu.pc = 0x2783A;
    16
}
/// $046396: jsr $5ca60.l
pub fn i_046396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4639C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4639C);
    cpu.pc = 0x5CA60;
    16
}
/// $04639C: movea.l #$ffffffff, a0
pub fn i_04639c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463A2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0463A2: lea.l $28db7e.l, a0
pub fn i_0463a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DB7E;
    20
}
/// $0463A8: jsr $5dd5c.l
pub fn i_0463a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x463AE);
    cpu.pc = 0x5DD5C;
    16
}
/// $0463AE: bcc.w $463b8
pub fn i_0463ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x463B8; return 10; }
    8
}
// ===== SetTaskHandler_0463b2  ($0463B2, 8 B, kernel/task_handlers.c)
/// $0463B2: lea.l $463ba(pc), a1
pub fn i_0463b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x463BA;
    16
}
/// $0463B6: move.l a1, (a6)
pub fn i_0463b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0463B8: rts
pub fn i_0463b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0463ba  ($0463BA, 8 B, kernel/jmp_to_scheduler.c)
/// $0463BA: jmp $518.l
pub fn i_0463ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463C0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Drop_ProbeAndNudgeY_0463c2  ($0463C2, 24 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0463C2: move.b $9a(a6), d0
pub fn i_0463c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0463C6: move.w #$9b, d1
pub fn i_0463c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0463CA: jsr $9a7aa.l
pub fn i_0463ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x463D0);
    cpu.pc = 0x9A7AA;
    16
}
/// $0463D0: bcs.w $463d8
pub fn i_0463d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463D4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x463D8; return 10; }
    8
}
/// $0463D4: addq.w #$4, $24(a0)
pub fn i_0463d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463D8;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0463D8: rts
pub fn i_0463d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Drop_Spawn_Tmpl4F_0463da  ($0463DA, 20 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0463DA: move.w #$4f, d1
pub fn i_0463da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0463DE: jsr $236e.l
pub fn i_0463de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x463E4);
    cpu.pc = 0x236E;
    16
}
/// $0463E4: move.l #$233c16, $3c(a6)
pub fn i_0463e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463EC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x233C16;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0463EC: bra.b $4637e
pub fn i_0463ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463EE;
    cpu.ir = 0x6090;
    cpu.pc = 0x4637E;
    10
}
// ===== Drop_Spawn_TmplF2_0463ee  ($0463EE, 22 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0463EE: move.w #$f2, d1
pub fn i_0463ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0463F2: jsr $236e.l
pub fn i_0463f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x463F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x463F8);
    cpu.pc = 0x236E;
    16
}
/// $0463F8: move.l #$24d800, $3c(a6)
pub fn i_0463f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46400;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x24D800;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $046400: bra.w $4637e
pub fn i_046400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46404;
    cpu.ir = 0x6000;
    cpu.pc = 0x4637E;
    10
}
// ===== Drop_Spawn_Tmpl1B_046404  ($046404, 22 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046404: move.w #$1b, d1
pub fn i_046404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46408;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046408: jsr $236e.l
pub fn i_046408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4640E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4640E);
    cpu.pc = 0x236E;
    16
}
/// $04640E: move.l #$23603a, $3c(a6)
pub fn i_04640e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46416;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x23603A;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $046416: bra.w $4637e
pub fn i_046416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4641A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4637E;
    10
}
// ===== Drop_Spawn_Tmpl1B_B_04641a  ($04641A, 22 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04641A: move.w #$1b, d1
pub fn i_04641a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4641E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04641E: jsr $236e.l
pub fn i_04641e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46424;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46424);
    cpu.pc = 0x236E;
    16
}
/// $046424: move.l #$23603a, $3c(a6)
pub fn i_046424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4642C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x23603A;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04642C: bra.w $4637e
pub fn i_04642c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46430;
    cpu.ir = 0x6000;
    cpu.pc = 0x4637E;
    10
}
// ===== Drop_Spawn_Tmpl6D_046430  ($046430, 22 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046430: move.w #$6d, d1
pub fn i_046430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46434;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046434: jsr $236e.l
pub fn i_046434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4643A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4643A);
    cpu.pc = 0x236E;
    16
}
/// $04643A: move.l #$23ac84, $3c(a6)
pub fn i_04643a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46442;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x23AC84;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $046442: bra.w $4637e
pub fn i_046442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46446;
    cpu.ir = 0x6000;
    cpu.pc = 0x4637E;
    10
}
// ===== Drop_Spawn_Tmpl2C_046446  ($046446, 22 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046446: move.w #$2c, d1
pub fn i_046446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4644A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04644A: jsr $236e.l
pub fn i_04644a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46450);
    cpu.pc = 0x236E;
    16
}
/// $046450: move.l #$23c778, $3c(a6)
pub fn i_046450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46458;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x23C778;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $046458: bra.w $4637e
pub fn i_046458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4645C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4637E;
    10
}
// ===== Drop_Spawn_Tmpl2B_04645c  ($04645C, 40 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04645C: move.l #$239a4e, d0
pub fn i_04645c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46462;
    cpu.ir = 0x203C;
    let v1: u32 = 0x239A4E;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $046462: move.l d0, $3c(a6)
pub fn i_046462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46466;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $046466: move.w #$2b, d1
pub fn i_046466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4646A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04646A: jsr $236e.l
pub fn i_04646a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46470;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46470);
    cpu.pc = 0x236E;
    16
}
/// $046470: bra.w $4637e
pub fn i_046470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46474;
    cpu.ir = 0x6000;
    cpu.pc = 0x4637E;
    10
}
// ===== Drop_Spawn_Tmpl3A_12F_046484  ($046484, 34 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046484: move.w #$3a, d1
pub fn i_046484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46488;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046488: tst.b $98(a6)
pub fn i_046488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4648C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04648C: beq.w $46494
pub fn i_04648c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46490;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x46494; return 10; }
    8
}
/// $046490: move.w #$12f, d1
pub fn i_046490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46494;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046494: jsr $236e.l
pub fn i_046494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4649A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4649A);
    cpu.pc = 0x236E;
    16
}
/// $04649A: move.l #$23a068, $3c(a6)
pub fn i_04649a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x464A2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x23A068;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0464A2: bra.w $4637e
pub fn i_0464a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x464A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4637E;
    10
}
// ===== Entity_CmpDepthToParent_046518  ($046518, 16 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046518: movea.l $8(a6), a1
pub fn i_046518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4651C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04651C: move.b $10(a6), d0
pub fn i_04651c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46520;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046520: cmp.b $10(a1), d0
pub fn i_046520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46524;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $046524: bcs.w $4652e
pub fn i_046524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46528;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4652E; return 10; }
    8
}
// ===== ClearXN_046528  ($046528, 6 B, thunks/ccr_helpers.c)
/// $046528: andi.b #$ee, ccr
pub fn i_046528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4652C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04652C: rts
pub fn i_04652c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4652E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04652e  ($04652E, 6 B, thunks/ccr_helpers.c)
/// $04652E: ori.b #$11, ccr
pub fn i_04652e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46532;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $046532: rts
pub fn i_046532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46534;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Drop_SpawnThrown_046534  ($046534, 162 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046534: move.b $99(a6), d0
pub fn i_046534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46538;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046538: ext.w d0
pub fn i_046538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4653A;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04653A: lsl.w #$4, d0
pub fn i_04653a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4653C;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $04653C: move.w d0, $28(a6)
pub fn i_04653c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46540;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046540: btst.b #$7, $99(a6)
pub fn i_046540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46546;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $046546: beq.w $46550
pub fn i_046546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4654A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x46550; return 10; }
    8
}
/// $04654A: move.b #$1, $3a(a6)
pub fn i_04654a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46550;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046550: move.b $98(a6), d0
pub fn i_046550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46554;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046554: andi.w #$3, d0
pub fn i_046554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46558;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046558: lsl.w #$2, d0
pub fn i_046558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4655A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04655A: move.b $9a(a6), d1
pub fn i_04655a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4655E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04655E: andi.w #$3, d1
pub fn i_04655e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46562;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046562: add.w d1, d0
pub fn i_046562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46564;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046564: lsl.w #$1, d0
pub fn i_046564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46566;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $046566: lea.l $28dc4e.l, a0
pub fn i_046566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4656C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DC4E;
    20
}
/// $04656C: move.w (a0, d0.w), d1
pub fn i_04656c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46570;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $046570: jsr $236e.l
pub fn i_046570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46576;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46576);
    cpu.pc = 0x236E;
    16
}
/// $046576: move.b $9b(a6), d0
pub fn i_046576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4657A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04657A: tst.b d0
pub fn i_04657a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4657C;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $04657C: bne.w $46584
pub fn i_04657c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46580;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46584; return 10; }
    8
}
/// $046580: move.b #$ff, d0
pub fn i_046580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46584;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $046584: move.b d0, $32(a6)
pub fn i_046584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46588;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046588: move.b d0, $33(a6)
pub fn i_046588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4658C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04658C: move.w #$8000, d0
pub fn i_04658c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46590;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046590: jsr $28134.l
pub fn i_046590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46596);
    cpu.pc = 0x28134;
    16
}
/// $046596: andi.w #$ffe3, $38(a6)
pub fn i_046596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4659C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04659C: ori.w #$10, $38(a6)
pub fn i_04659c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465A2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0465A2: lea.l $28dc6e.l, a0
pub fn i_0465a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DC6E;
    20
}
/// $0465A8: jsr $28cd4.l
pub fn i_0465a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x465AE);
    cpu.pc = 0x28CD4;
    16
}
/// $0465AE: lea.l $465b4(pc), a1
pub fn i_0465ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x465B4;
    16
}
/// $0465B2: move.l a1, (a6)
pub fn i_0465b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0465B4: jsr $27cee.l
pub fn i_0465b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x465BA);
    cpu.pc = 0x27CEE;
    16
}
/// $0465BA: jsr $28d70.l
pub fn i_0465ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x465C0);
    cpu.pc = 0x28D70;
    16
}
/// $0465C0: movea.l #$ffffffff, a0
pub fn i_0465c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0465C6: lea.l $28dc46.l, a0
pub fn i_0465c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DC46;
    20
}
/// $0465CC: jsr $5dd5c.l
pub fn i_0465cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x465D2);
    cpu.pc = 0x5DD5C;
    16
}
/// $0465D2: bcc.w $465dc
pub fn i_0465d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x465DC; return 10; }
    8
}
// ===== SetTaskHandler_0465d6  ($0465D6, 8 B, kernel/task_handlers.c)
/// $0465D6: lea.l $465de(pc), a1
pub fn i_0465d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x465DE;
    16
}
/// $0465DA: move.l a1, (a6)
pub fn i_0465da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0465DC: rts
pub fn i_0465dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_0465de  ($0465DE, 14 B, kernel/jsr5b6_jmp518.c)
/// $0465DE: jsr $5b6.l
pub fn i_0465de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x465E4);
    cpu.pc = 0x5B6;
    16
}
/// $0465E4: jmp $518.l
pub fn i_0465e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Entity_CmpDepthToParent_0465ec  ($0465EC, 16 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0465EC: movea.l $8(a6), a1
pub fn i_0465ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465F0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0465F0: move.b $10(a6), d0
pub fn i_0465f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465F4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0465F4: cmp.b $10(a1), d0
pub fn i_0465f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465F8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0465F8: bcs.w $46602
pub fn i_0465f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x465FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x46602; return 10; }
    8
}
// ===== ClearXN_0465fc  ($0465FC, 6 B, thunks/ccr_helpers.c)
/// $0465FC: andi.b #$ee, ccr
pub fn i_0465fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46600;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $046600: rts
pub fn i_046600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46602;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_046602  ($046602, 6 B, thunks/ccr_helpers.c)
/// $046602: ori.b #$11, ccr
pub fn i_046602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46606;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $046606: rts
pub fn i_046606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46608;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fade_WhiteFlash_Task_046608  ($046608, 84 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046608: move.w #$0, d0
pub fn i_046608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4660C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04660C: move.w #$1f, d1
pub fn i_04660c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46610;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046610: move.w #$1f, d2
pub fn i_046610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46614;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046614: move.w #$10, d3
pub fn i_046614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46618;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046618: jsr $52580.l
pub fn i_046618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4661E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4661E);
    cpu.pc = 0x52580;
    16
}
/// $04661E: move.b #$ff, $1081b1.l
pub fn i_04661e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46626;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081B1, v1 as u8);
    16
}
/// $046626: move.w #$20, $70(a6)
pub fn i_046626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4662C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04662C: lea.l $46632(pc), a1
pub fn i_04662c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46632;
    16
}
/// $046630: move.l a1, (a6)
pub fn i_046630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46632;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046632: subq.w #$1, $70(a6)
pub fn i_046632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46636;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $046636: cmpi.w #$0, $70(a6)
pub fn i_046636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4663C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04663C: bgt.w $46662
pub fn i_04663c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46640;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46662; return 10; }
    8
}
/// $046640: move.w #$1f, d0
pub fn i_046640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46644;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046644: move.w #$1f, d1
pub fn i_046644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46648;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046648: move.w #$1f, d2
pub fn i_046648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4664C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04664C: move.w #$10, d3
pub fn i_04664c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46650;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046650: jsr $52580.l
pub fn i_046650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46656);
    cpu.pc = 0x52580;
    16
}
/// $046656: move.w #$10, $70(a6)
pub fn i_046656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4665C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_04665c  ($04665C, 8 B, kernel/task_handlers.c)
/// $04665C: lea.l $46664(pc), a1
pub fn i_04665c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46660;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46664;
    16
}
/// $046660: move.l a1, (a6)
pub fn i_046660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46662;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046662: rts
pub fn i_046662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46664;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fade_WhiteFlash_Done_046664  ($046664, 30 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046664: subq.w #$1, $70(a6)
pub fn i_046664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46668;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $046668: cmpi.w #$0, $70(a6)
pub fn i_046668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4666E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04666E: bgt.w $46680
pub fn i_04666e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46672;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46680; return 10; }
    8
}
/// $046672: movea.l $c(a6), a1
pub fn i_046672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46676;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $046676: clr.b $21(a1)
pub fn i_046676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4667A;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04667A: jmp $518.l
pub fn i_04667a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46680;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $046680: rts
pub fn i_046680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46682;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneC_Load_Task_046682  ($046682, 42 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046682: jsr $22c8.l
pub fn i_046682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46688);
    cpu.pc = 0x22C8;
    16
}
/// $046688: move.b #$c, d0
pub fn i_046688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4668C;
    cpu.ir = 0x103C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04668C: jsr $43568.l
pub fn i_04668c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46692;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46692);
    cpu.pc = 0x43568;
    16
}
/// $046692: moveq #$0, d0
pub fn i_046692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46694;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $046694: moveq #$0, d1
pub fn i_046694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46696;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $046696: jsr $437da.l
pub fn i_046696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4669C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4669C);
    cpu.pc = 0x437DA;
    16
}
/// $04669C: move.w #$3, d0
pub fn i_04669c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0466A0: jsr $523b2.l
pub fn i_0466a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x466A6);
    cpu.pc = 0x523B2;
    16
}
/// $0466A6: move.w #$78, $70(a6)
pub fn i_0466a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_0466ac  ($0466AC, 8 B, kernel/task_handlers.c)
/// $0466AC: lea.l $466b4(pc), a1
pub fn i_0466ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x466B4;
    16
}
/// $0466B0: move.l a1, (a6)
pub fn i_0466b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0466B2: rts
pub fn i_0466b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneC_Load_Spawn2_0466b4  ($0466B4, 30 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0466B4: subq.w #$1, $70(a6)
pub fn i_0466b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0466B8: cmpi.w #$0, $70(a6)
pub fn i_0466b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466BE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0466BE: bgt.w $466d8
pub fn i_0466be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466C2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x466D8; return 10; }
    8
}
/// $0466C2: move.w #$2, d0
pub fn i_0466c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0466C6: jsr $5239e.l
pub fn i_0466c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x466CC);
    cpu.pc = 0x5239E;
    16
}
/// $0466CC: move.w #$3c, $70(a6)
pub fn i_0466cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_0466d2  ($0466D2, 8 B, kernel/task_handlers.c)
/// $0466D2: lea.l $466da(pc), a1
pub fn i_0466d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x466DA;
    16
}
/// $0466D6: move.l a1, (a6)
pub fn i_0466d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0466D8: rts
pub fn i_0466d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneC_Load_Finish_0466da  ($0466DA, 28 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0466DA: subq.w #$1, $70(a6)
pub fn i_0466da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466DE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0466DE: cmpi.w #$0, $70(a6)
pub fn i_0466de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466E4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0466E4: bgt.w $466f4
pub fn i_0466e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466E8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x466F4; return 10; }
    8
}
/// $0466E8: clr.b $106ed2.l
pub fn i_0466e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466EE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $0466EE: jmp $518.l
pub fn i_0466ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466F4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0466F4: rts
pub fn i_0466f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ContinueDigits_P1P2_Task_0466f6  ($0466F6, 256 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0466F6: move.w #$0, $74(a6)
pub fn i_0466f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0466FC: move.w #$7183, d0
pub fn i_0466fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46700;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7183;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046700: move.w d0, $22(a6)
pub fn i_046700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46704;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046704: jsr $260c4.l
pub fn i_046704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4670A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4670A);
    cpu.pc = 0x260C4;
    16
}
/// $04670A: jsr $2604c.l
pub fn i_04670a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46710);
    cpu.pc = 0x2604C;
    16
}
/// $046710: bra.w $4673a
pub fn i_046710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46714;
    cpu.ir = 0x6000;
    cpu.pc = 0x4673A;
    10
}
/// $04673A: move.w #$9, $5c(a6)
pub fn i_04673a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46740;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046740: move.w #$3b, $70(a6)
pub fn i_046740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46746;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046746: move.w $22(a6), d0
pub fn i_046746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4674A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04674A: move.w $5c(a6), d1
pub fn i_04674a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4674E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04674E: addi.w #$3460, d1
pub fn i_04674e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46752;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3460, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046752: movem.w d0-d1, $3c0000.l
pub fn i_046752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4675A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04675A: addq.w #$1, d0
pub fn i_04675a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4675C;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04675C: addi.w #$10, d1
pub fn i_04675c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46760;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046760: movem.w d0-d1, $3c0000.l
pub fn i_046760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46768;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046768: lea.l $4676e(pc), a1
pub fn i_046768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4676C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4676E;
    16
}
/// $04676C: move.l a1, (a6)
pub fn i_04676c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4676E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04676E: movea.l $c(a6), a1
pub fn i_04676e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46772;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $046772: cmpi.b #$0, $20(a1)
pub fn i_046772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46778;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $046778: bne.w $46784
pub fn i_046778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4677C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46784; return 10; }
    8
}
/// $04677C: jmp $518.l
pub fn i_04677c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46782;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $046784: cmpi.w #$0, $74(a6)
pub fn i_046784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4678A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04678A: bne.w $46796
pub fn i_04678a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4678E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46796; return 10; }
    8
}
/// $04678E: jsr $4694a(pc)
pub fn i_04678e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46792;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x46792);
    cpu.pc = 0x4694A;
    16
}
/// $046792: bra.w $4679a
pub fn i_046792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46796;
    cpu.ir = 0x6000;
    cpu.pc = 0x4679A;
    10
}
/// $046796: jsr $4698c(pc)
pub fn i_046796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4679A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4679A);
    cpu.pc = 0x4698C;
    16
}
/// $04679A: bcc.w $467a2
pub fn i_04679a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4679E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x467A2; return 10; }
    8
}
/// $04679E: clr.w $70(a6)
pub fn i_04679e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467A2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0467A2: subq.w #$1, $70(a6)
pub fn i_0467a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467A6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0467A6: cmpi.w #$0, $70(a6)
pub fn i_0467a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467AC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0467AC: bgt.w $467f4
pub fn i_0467ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467B0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x467F4; return 10; }
    8
}
/// $0467B0: subq.w #$1, $5c(a6)
pub fn i_0467b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467B4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0467B4: cmpi.w #$0, $5c(a6)
pub fn i_0467b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467BA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0467BA: bge.w $467cc
pub fn i_0467ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467BE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x467CC; return 10; }
    8
}
/// $0467BE: movea.l $c(a6), a1
pub fn i_0467be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467C2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0467C2: clr.b $21(a1)
pub fn i_0467c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467C6;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0467C6: jmp $518.l
pub fn i_0467c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0467CC: move.w $22(a6), d0
pub fn i_0467cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0467D0: move.w $5c(a6), d1
pub fn i_0467d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0467D4: addi.w #$3460, d1
pub fn i_0467d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467D8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3460, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0467D8: movem.w d0-d1, $3c0000.l
pub fn i_0467d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467E0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0467E0: addq.w #$1, d0
pub fn i_0467e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467E2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0467E2: addi.w #$10, d1
pub fn i_0467e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467E6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0467E6: movem.w d0-d1, $3c0000.l
pub fn i_0467e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467EE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0467EE: move.w #$3b, $70(a6)
pub fn i_0467ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0467F4: rts
pub fn i_0467f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ContinueBig_Init_0467f6  ($0467F6, 130 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0467F6: move.w #$0, $74(a6)
pub fn i_0467f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0467FC: bra.w $46806
pub fn i_0467fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46800;
    cpu.ir = 0x6000;
    cpu.pc = 0x46806;
    10
}
/// $046806: move.w #$ffff, $38(a6)
pub fn i_046806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4680C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04680C: bset.b #$6, $12(a6)
pub fn i_04680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46812;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $046812: move.w #$5f, d1
pub fn i_046812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46816;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046816: jsr $236e.l
pub fn i_046816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4681C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4681C);
    cpu.pc = 0x236E;
    16
}
/// $04681C: move.w $14(a6), d1
pub fn i_04681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46820;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $046820: jsr $2c66.l
pub fn i_046820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46826);
    cpu.pc = 0x2C66;
    16
}
/// $046826: lea.l $28de2a.l, a0
pub fn i_046826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4682C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DE2A;
    20
}
/// $04682C: jsr $28cd4.l
pub fn i_04682c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46832);
    cpu.pc = 0x28CD4;
    16
}
/// $046832: move.w #$a0, $22(a6)
pub fn i_046832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46838;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046838: move.w #$178, $24(a6)
pub fn i_046838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4683E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04683E: clr.w $26(a6)
pub fn i_04683e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46842;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $046842: move.w #$9, $5c(a6)
pub fn i_046842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046848: move.w #$3b, $70(a6)
pub fn i_046848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4684E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04684E: move.b #$ff, $72(a6)
pub fn i_04684e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46854;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046854: movea.w #$716a, a1
pub fn i_046854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46858;
    cpu.ir = 0x327C;
    let v1: u32 = 0x716A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046858: lea.l $28de36.l, a2
pub fn i_046858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4685E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DE36;
    20
}
/// $04685E: move.w #$4, d1
pub fn i_04685e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46862;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046862: jsr $4784c.l
pub fn i_046862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46868);
    cpu.pc = 0x4784C;
    16
}
/// $046868: lea.l $46878(pc), a1
pub fn i_046868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4686C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46878;
    16
}
/// $04686C: move.l a1, (a6)
pub fn i_04686c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4686E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04686E: move.w #$10d4, d0
pub fn i_04686e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46872;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046872: jsr $2352.l
pub fn i_046872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46878);
    cpu.pc = 0x2352;
    16
}
// ===== ContinueBig_Countdown_046878  ($046878, 174 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046878: movea.l $c(a6), a1
pub fn i_046878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4687C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04687C: cmpi.b #$0, $20(a1)
pub fn i_04687c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46882;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $046882: bne.w $4688a
pub fn i_046882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46886;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4688A; return 10; }
    8
}
/// $046886: bra.w $4692e
pub fn i_046886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4688A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4692E;
    10
}
/// $04688A: jsr $469ce(pc)
pub fn i_04688a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4688E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4688E);
    cpu.pc = 0x469CE;
    16
}
/// $04688E: bcc.w $4689c
pub fn i_04688e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46892;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4689C; return 10; }
    8
}
/// $046892: move.b #$ff, $72(a6)
pub fn i_046892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46898;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046898: clr.w $70(a6)
pub fn i_046898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4689C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04689C: subq.w #$1, $70(a6)
pub fn i_04689c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468A0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0468A0: cmpi.w #$0, $70(a6)
pub fn i_0468a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468A6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0468A6: bgt.w $468ea
pub fn i_0468a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468AA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x468EA; return 10; }
    8
}
/// $0468AA: subq.w #$1, $5c(a6)
pub fn i_0468aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468AE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0468AE: cmpi.w #$0, $5c(a6)
pub fn i_0468ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0468B4: bge.w $468c4
pub fn i_0468b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468B8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x468C4; return 10; }
    8
}
/// $0468B8: movea.l $c(a6), a1
pub fn i_0468b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468BC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0468BC: clr.b $21(a1)
pub fn i_0468bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468C0;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0468C0: bra.w $4692e
pub fn i_0468c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4692E;
    10
}
/// $0468C4: move.w #$10d4, d0
pub fn i_0468c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0468C8: jsr $2352.l
pub fn i_0468c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x468CE);
    cpu.pc = 0x2352;
    16
}
/// $0468CE: move.w #$3b, $70(a6)
pub fn i_0468ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0468D4: lea.l $28dd96.l, a1
pub fn i_0468d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DD96;
    20
}
/// $0468DA: move.w $5c(a6), d0
pub fn i_0468da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0468DE: lsl.w #$2, d0
pub fn i_0468de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468E0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0468E0: movea.l (a1, d0.w), a0
pub fn i_0468e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468E4;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0468E4: jsr $28cd4.l
pub fn i_0468e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x468EA);
    cpu.pc = 0x28CD4;
    16
}
/// $0468EA: lea.l $28de54.l, a1
pub fn i_0468ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DE54;
    20
}
/// $0468F0: move.w $70(a6), d1
pub fn i_0468f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468F4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0468F4: move.b (a1, d1.w), d0
pub fn i_0468f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468F8;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0468F8: cmpi.w #$37, $70(a6)
pub fn i_0468f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x37, Size::Word, false);
    16
}
/// $0468FE: ble.w $4691e
pub fn i_0468fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46902;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4691E; return 10; }
    8
}
/// $046902: cmpi.b #$0, $72(a6)
pub fn i_046902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46908;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $046908: bne.w $46914
pub fn i_046908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4690C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46914; return 10; }
    8
}
/// $04690C: move.b d0, $32(a6)
pub fn i_04690c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46910;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046910: bra.w $4691a
pub fn i_046910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46914;
    cpu.ir = 0x6000;
    cpu.pc = 0x4691A;
    10
}
/// $046914: move.b #$ff, $32(a6)
pub fn i_046914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4691A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04691A: bra.w $46926
pub fn i_04691a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4691E;
    cpu.ir = 0x6000;
    cpu.pc = 0x46926;
    10
}
/// $04691E: clr.b $72(a6)
pub fn i_04691e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46922;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $046922: move.b d0, $32(a6)
pub fn i_046922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46926;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrAbsThunk_046926  ($046926, 8 B, thunks/jsr_abs_thunks.c)
/// $046926: jsr $28d70.l
pub fn i_046926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4692C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4692C);
    cpu.pc = 0x28D70;
    16
}
/// $04692C: rts
pub fn i_04692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4692E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ContinueBig_ClearAndExit_04692e  ($04692E, 26 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04692E: movea.w #$716a, a1
pub fn i_04692e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46932;
    cpu.ir = 0x327C;
    let v1: u32 = 0x716A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046932: lea.l $28de40.l, a2
pub fn i_046932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46938;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DE40;
    20
}
/// $046938: move.w #$4, d1
pub fn i_046938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4693C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04693C: jsr $4784c.l
pub fn i_04693c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46942);
    cpu.pc = 0x4784C;
    16
}
/// $046942: jmp $518.l
pub fn i_046942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46948;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_046948  ($046948, 2 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046948: rts
pub fn i_046948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4694A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP1_04694a  ($04694A, 12 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04694A: cmpi.b #$2, $10fdb6.l
pub fn i_04694a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46952;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $046952: beq.w $4695c
pub fn i_046952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46956;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4695C; return 10; }
    8
}
// ===== ClearXN_046956  ($046956, 6 B, thunks/ccr_helpers.c)
/// $046956: andi.b #$ee, ccr
pub fn i_046956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4695A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04695A: rts
pub fn i_04695a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4695C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP1_CheckCount_04695c  ($04695C, 10 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04695C: cmpi.w #$8, $5c(a6)
pub fn i_04695c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46962;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $046962: bls.w $4696c
pub fn i_046962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46966;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x4696C; return 10; }
    8
}
// ===== ClearXN_046966  ($046966, 6 B, thunks/ccr_helpers.c)
/// $046966: andi.b #$ee, ccr
pub fn i_046966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4696A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04696A: rts
pub fn i_04696a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4696C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP1_CheckTask_04696c  ($04696C, 14 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04696C: move.b $10e203.l, d0
pub fn i_04696c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46972;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E203) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $046972: andi.b #$f0, d0
pub fn i_046972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46976;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $046976: beq.w $46984
pub fn i_046976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4697A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x46984; return 10; }
    8
}
// ===== SetXN_04697a  ($04697A, 6 B, thunks/ccr_helpers.c)
/// $04697A: ori.b #$11, ccr
pub fn i_04697a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4697E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04697E: rts
pub fn i_04697e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46980;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP1_Yes_046980  ($046980, 4 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046980: bra.w $4698a
pub fn i_046980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46984;
    cpu.ir = 0x6000;
    cpu.pc = 0x4698A;
    10
}
// ===== ClearXN_046984  ($046984, 6 B, thunks/ccr_helpers.c)
/// $046984: andi.b #$ee, ccr
pub fn i_046984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46988;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $046988: rts
pub fn i_046988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4698A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0004698A  ($04698A, 2 B, thunks/trivial_rts.c)
/// $04698A: rts
pub fn i_04698a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4698C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP2_04698c  ($04698C, 12 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04698C: cmpi.b #$2, $10fdb7.l
pub fn i_04698c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46994;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $046994: beq.w $4699e
pub fn i_046994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46998;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4699E; return 10; }
    8
}
// ===== ClearXN_046998  ($046998, 6 B, thunks/ccr_helpers.c)
/// $046998: andi.b #$ee, ccr
pub fn i_046998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4699C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04699C: rts
pub fn i_04699c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4699E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP2_CheckCount_04699e  ($04699E, 10 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04699E: cmpi.w #$8, $5c(a6)
pub fn i_04699e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $0469A4: bls.w $469ae
pub fn i_0469a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469A8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x469AE; return 10; }
    8
}
// ===== ClearXN_0469a8  ($0469A8, 6 B, thunks/ccr_helpers.c)
/// $0469A8: andi.b #$ee, ccr
pub fn i_0469a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469AC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0469AC: rts
pub fn i_0469ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP2_CheckTask_0469ae  ($0469AE, 14 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469AE: move.b $10e209.l, d0
pub fn i_0469ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469B4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E209) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0469B4: andi.b #$f0, d0
pub fn i_0469b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469B8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0469B8: beq.w $469c6
pub fn i_0469b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x469C6; return 10; }
    8
}
// ===== SetXN_0469bc  ($0469BC, 6 B, thunks/ccr_helpers.c)
/// $0469BC: ori.b #$11, ccr
pub fn i_0469bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0469C0: rts
pub fn i_0469c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP2_Yes_0469c2  ($0469C2, 4 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469C2: bra.w $469cc
pub fn i_0469c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x469CC;
    10
}
// ===== ClearXN_0469c6  ($0469C6, 6 B, thunks/ccr_helpers.c)
/// $0469C6: andi.b #$ee, ccr
pub fn i_0469c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0469CA: rts
pub fn i_0469ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_000469CC  ($0469CC, 2 B, thunks/trivial_rts.c)
/// $0469CC: rts
pub fn i_0469cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartAny_0469ce  ($0469CE, 10 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469CE: jsr $4694a(pc)
pub fn i_0469ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x469D2);
    cpu.pc = 0x4694A;
    16
}
/// $0469D2: bcc.w $469dc
pub fn i_0469d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x469DC; return 10; }
    8
}
/// $0469D6: rts
pub fn i_0469d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartAny_Tail_0469d8  ($0469D8, 4 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469D8: bra.w $469e0
pub fn i_0469d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x469E0;
    10
}
// ===== JsrPcThunk_0469dc  ($0469DC, 6 B, thunks/jsr_pc_thunks.c)
/// $0469DC: jsr $4698c(pc)
pub fn i_0469dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x469E0);
    cpu.pc = 0x4698C;
    16
}
/// $0469E0: rts
pub fn i_0469e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_StoreCount_Exit_0469e2  ($0469E2, 26 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469E2: move.b $98(a6), d0
pub fn i_0469e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469E6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0469E6: cmpi.b #$ff, d0
pub fn i_0469e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469EA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0469EA: bne.w $469f0
pub fn i_0469ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x469F0; return 10; }
    8
}
/// $0469EE: clr.b d0
pub fn i_0469ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469F0;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0469F0: move.b d0, $1081b0.l
pub fn i_0469f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469F6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081B0, v1 as u8);
    16
}
/// $0469F6: jmp $518.l
pub fn i_0469f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469FC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0469fc  ($0469FC, 2 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469FC: rts
pub fn i_0469fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_ClearMissionBanner_0469fe  ($0469FE, 38 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469FE: movea.l #$7014, a1
pub fn i_0469fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A04;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7014;
    cpu.a[1] = v1;
    4
}
/// $046A04: lea.l $28de92.l, a2
pub fn i_046a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A0A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DE92;
    20
}
/// $046A0A: move.b #$9, d1
pub fn i_046a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A0E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $046A0E: jsr $47888.l
pub fn i_046a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46A14);
    cpu.pc = 0x47888;
    16
}
/// $046A14: movea.l #$7017, a1
pub fn i_046a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A1A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7017;
    cpu.a[1] = v1;
    4
}
/// $046A1A: lea.l $28de92.l, a2
pub fn i_046a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A20;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DE92;
    20
}
/// $046A20: move.b #$9, d1
pub fn i_046a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A24;
    cpu.ir = 0x123C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== JsrAbsThunk_046a24  ($046A24, 8 B, thunks/jsr_abs_thunks.c)
/// $046A24: jsr $47888.l
pub fn i_046a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46A2A);
    cpu.pc = 0x47888;
    16
}
/// $046A2A: rts
pub fn i_046a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_046a2c  ($046A2C, 16 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046A2C: movea.l $8(a6), a1
pub fn i_046a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A30;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $046A30: move.b $10(a6), d0
pub fn i_046a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A34;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $046A34: cmp.b $10(a1), d0
pub fn i_046a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A38;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $046A38: bcs.w $46a42
pub fn i_046a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A3C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x46A42; return 10; }
    8
}
// ===== ClearXN_046a3c  ($046A3C, 6 B, thunks/ccr_helpers.c)
/// $046A3C: andi.b #$ee, ccr
pub fn i_046a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A40;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $046A40: rts
pub fn i_046a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_046a42  ($046A42, 6 B, thunks/ccr_helpers.c)
/// $046A42: ori.b #$11, ccr
pub fn i_046a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A46;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $046A46: rts
pub fn i_046a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TimeUp_Banner_Task_046a48  ($046A48, 78 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046A48: move.w #$5a, $70(a6)
pub fn i_046a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046A4E: movea.w #$71af, a1
pub fn i_046a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A52;
    cpu.ir = 0x327C;
    let v1: u32 = 0x71AF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046A52: move.b #$4, d1
pub fn i_046a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A56;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $046A56: lea.l $28debc.l, a2
pub fn i_046a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A5C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DEBC;
    20
}
/// $046A5C: jsr $4784c(pc)
pub fn i_046a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x46A60);
    cpu.pc = 0x4784C;
    16
}
/// $046A60: lea.l $46a66(pc), a1
pub fn i_046a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46A66;
    16
}
/// $046A64: move.l a1, (a6)
pub fn i_046a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046A66: subq.w #$1, $70(a6)
pub fn i_046a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A6A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $046A6A: cmpi.w #$0, $70(a6)
pub fn i_046a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A70;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $046A70: bgt.w $46a94
pub fn i_046a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A74;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46A94; return 10; }
    8
}
/// $046A74: movea.w #$71af, a1
pub fn i_046a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A78;
    cpu.ir = 0x327C;
    let v1: u32 = 0x71AF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046A78: move.b #$4, d1
pub fn i_046a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A7C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $046A7C: lea.l $28dec4.l, a2
pub fn i_046a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A82;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DEC4;
    20
}
/// $046A82: jsr $4784c(pc)
pub fn i_046a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x46A86);
    cpu.pc = 0x4784C;
    16
}
/// $046A86: movea.l $c(a6), a1
pub fn i_046a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A8A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $046A8A: clr.b $21(a1)
pub fn i_046a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A8E;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $046A8E: jmp $518.l
pub fn i_046a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A94;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $046A94: rts
pub fn i_046a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixLayer_ClearTopRows_046a96  ($046A96, 48 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046A96: movea.w #$7000, a1
pub fn i_046a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A9A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046A9A: move.w #$20, d0
pub fn i_046a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46A9E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046A9E: move.w #$28, d1
pub fn i_046a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AA2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AA2: move.w #$2, d2
pub fn i_046aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AA6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AA6: jsr $5da9c.l
pub fn i_046aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46AAC);
    cpu.pc = 0x5DA9C;
    16
}
/// $046AAC: movea.w #$701a, a1
pub fn i_046aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AB0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x701A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046AB0: move.w #$20, d0
pub fn i_046ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AB4: move.w #$28, d1
pub fn i_046ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AB8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AB8: move.w #$6, d2
pub fn i_046ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ABC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046ABC: jsr $5da9c.l
pub fn i_046abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46AC2);
    cpu.pc = 0x5DA9C;
    16
}
/// $046AC2: bra.w $46af2
pub fn i_046ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x46AF2;
    10
}
// ===== FixLayer_QuadBatch_046AC6  ($046AC6, 90 B, entity/misc_helpers_024fec.s)
/// $046AC6: movea.w #$7000, a1
pub fn i_046ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ACA;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046ACA: move.w #$20, d0
pub fn i_046aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ACE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046ACE: move.w #$28, d1
pub fn i_046ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AD2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AD2: move.w #$2, d2
pub fn i_046ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AD6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AD6: jsr $5da9c.l
pub fn i_046ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46ADC);
    cpu.pc = 0x5DA9C;
    16
}
/// $046ADC: movea.w #$701e, a1
pub fn i_046adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AE0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x701E;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046AE0: move.w #$20, d0
pub fn i_046ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AE4: move.w #$28, d1
pub fn i_046ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AE8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AE8: move.w #$2, d2
pub fn i_046ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AEC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AEC: jsr $5da9c.l
pub fn i_046aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46AF2);
    cpu.pc = 0x5DA9C;
    16
}
/// $046AF2: movea.w #$7000, a1
pub fn i_046af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AF6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046AF6: move.w #$20, d0
pub fn i_046af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AFA: move.w #$1, d1
pub fn i_046afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AFE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AFE: move.w #$20, d2
pub fn i_046afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B02;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B02: jsr $5da9c.l
pub fn i_046b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46B08);
    cpu.pc = 0x5DA9C;
    16
}
/// $046B08: movea.w #$74e0, a1
pub fn i_046b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B0C;
    cpu.ir = 0x327C;
    let v1: u32 = 0x74E0;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046B0C: move.w #$20, d0
pub fn i_046b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B10;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B10: move.w #$1, d1
pub fn i_046b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B14: move.w #$20, d2
pub fn i_046b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B18;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B18: jsr $5da9c.l
pub fn i_046b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46B1E);
    cpu.pc = 0x5DA9C;
    16
}
/// $046B1E: rts
pub fn i_046b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlit_BatchRow4x2_046B20  ($046B20, 186 B, video/fix_blit_batch_046b20.s)
/// $046B20: move.w #$7000, d0
pub fn i_046b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B24;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B24: asl.w #$5, d2
pub fn i_046b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B26;
    cpu.ir = 0xEB42;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $046B26: add.w d2, d0
pub fn i_046b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B28;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046B28: add.w d3, d0
pub fn i_046b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B2A;
    cpu.ir = 0xD043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046B2A: subi.w #$20, d0
pub fn i_046b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B2E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046B2E: cmpi.w #$0, d4
pub fn i_046b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B32;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $046B32: bne.w $46b3e
pub fn i_046b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B36;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46B3E; return 10; }
    8
}
/// $046B36: move.w #$0, d4
pub fn i_046b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B3A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B3A: bra.w $46b42
pub fn i_046b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x46B42;
    10
}
/// $046B3E: move.w #$1000, d4
pub fn i_046b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B42;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B42: cmpi.w #$74ff, d0
pub fn i_046b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B46;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046B46: bls.w $46b52
pub fn i_046b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B4A;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46B52; return 10; }
    8
}
/// $046B4A: addi.w #$20, d0
pub fn i_046b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B4E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046B4E: bra.w $46bba
pub fn i_046b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B52;
    cpu.ir = 0x6000;
    cpu.pc = 0x46BBA;
    10
}
/// $046B52: cmpi.w #$7000, d0
pub fn i_046b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B56;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046B56: bcc.w $46b62
pub fn i_046b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B5A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46B62; return 10; }
    8
}
/// $046B5A: addi.w #$20, d0
pub fn i_046b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B5E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046B5E: bra.w $46bba
pub fn i_046b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B62;
    cpu.ir = 0x6000;
    cpu.pc = 0x46BBA;
    10
}
/// $046B62: move.w #$2320, d1
pub fn i_046b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B66;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B66: movem.w d0-d1, $3c0000.l
pub fn i_046b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B6E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046B6E: addq.w #$1, d0
pub fn i_046b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B70;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046B70: movem.w d0-d1, $3c0000.l
pub fn i_046b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B78;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046B78: addq.w #$1, d0
pub fn i_046b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B7A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046B7A: movem.w d0-d1, $3c0000.l
pub fn i_046b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B82;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046B82: addq.w #$1, d0
pub fn i_046b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B84;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046B84: movem.w d0-d1, $3c0000.l
pub fn i_046b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B8C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046B8C: addi.w #$1d, d0
pub fn i_046b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B90;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046B90: movem.w d0-d1, $3c0000.l
pub fn i_046b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B98;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046B98: addq.w #$1, d0
pub fn i_046b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B9A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046B9A: movem.w d0-d1, $3c0000.l
pub fn i_046b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BA2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046BA2: addq.w #$1, d0
pub fn i_046ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BA4;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046BA4: movem.w d0-d1, $3c0000.l
pub fn i_046ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BAC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046BAC: addq.w #$1, d0
pub fn i_046bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BAE;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046BAE: movem.w d0-d1, $3c0000.l
pub fn i_046bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BB6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046BB6: addi.w #$1d, d0
pub fn i_046bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BBA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046BBA: move.w #$3e00, d1
pub fn i_046bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BBE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3E00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046BBE: add.w d4, d1
pub fn i_046bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BC0;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046BC0: move.w #$0, d6
pub fn i_046bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BC4;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046BC4: bra.b $46bc8
pub fn i_046bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BC6;
    cpu.ir = 0x6002;
    cpu.pc = 0x46BC8;
    10
}
/// $046BC6: addq.w #$1, d6
pub fn i_046bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BC8;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046BC8: cmpi.w #$f, d6
pub fn i_046bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BCC;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF, Size::Word, false);
    8
}
/// $046BCC: bgt.w $46c28
pub fn i_046bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BD0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46C28; return 10; }
    8
}
/// $046BD0: cmpi.w #$74ff, d0
pub fn i_046bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BD4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046BD4: bls.w $46bda
pub fn i_046bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BD8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46BDA; return 10; }
    8
}
/// $046BD8: rts
pub fn i_046bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlit_BatchRow4x1_ColorInc_046BDA  ($046BDA, 110 B, video/fix_blit_batch_046b20.s)
/// $046BDA: cmpi.w #$7000, d0
pub fn i_046bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BDE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046BDE: bcc.w $46bec
pub fn i_046bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46BEC; return 10; }
    8
}
/// $046BE2: addi.w #$20, d0
pub fn i_046be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BE6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046BE6: addq.w #$1, d1
pub fn i_046be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BE8;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046BE8: bra.w $46c26
pub fn i_046be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x46C26;
    10
}
/// $046BEC: movem.w d0-d1, $3c0000.l
pub fn i_046bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BF4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046BF4: addq.w #$1, d0
pub fn i_046bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BF6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046BF6: addi.w #$10, d1
pub fn i_046bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46BFA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046BFA: movem.w d0-d1, $3c0000.l
pub fn i_046bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C02;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046C02: addq.w #$1, d0
pub fn i_046c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C04;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046C04: addi.w #$10, d1
pub fn i_046c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C08;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C08: movem.w d0-d1, $3c0000.l
pub fn i_046c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C10;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046C10: addq.w #$1, d0
pub fn i_046c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C12;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046C12: addi.w #$10, d1
pub fn i_046c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C16;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C16: movem.w d0-d1, $3c0000.l
pub fn i_046c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C1E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046C1E: addi.w #$1d, d0
pub fn i_046c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C22;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C22: subi.w #$2f, d1
pub fn i_046c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C26;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x2F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C26: bra.b $46bc6
pub fn i_046c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C28;
    cpu.ir = 0x609E;
    cpu.pc = 0x46BC6;
    10
}
/// $046C28: move.w #$3f00, d1
pub fn i_046c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C2C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3F00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046C2C: add.w d4, d1
pub fn i_046c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C2E;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046C2E: move.w #$0, d6
pub fn i_046c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C32;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046C32: bra.b $46c36
pub fn i_046c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C34;
    cpu.ir = 0x6002;
    cpu.pc = 0x46C36;
    10
}
/// $046C34: addq.w #$1, d6
pub fn i_046c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C36;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046C36: cmpi.w #$3, d6
pub fn i_046c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C3A;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $046C3A: bgt.w $46c96
pub fn i_046c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C3E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46C96; return 10; }
    8
}
/// $046C3E: cmpi.w #$74ff, d0
pub fn i_046c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C42;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046C42: bls.w $46c48
pub fn i_046c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C46;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46C48; return 10; }
    8
}
/// $046C46: rts
pub fn i_046c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlit_Row4x1_Step_046c48  ($046C48, 78 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046C48: cmpi.w #$7000, d0
pub fn i_046c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C4C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046C4C: bcc.w $46c5a
pub fn i_046c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46C5A; return 10; }
    8
}
/// $046C50: addi.w #$20, d0
pub fn i_046c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C54;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C54: addq.w #$1, d1
pub fn i_046c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C56;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046C56: bra.w $46c94
pub fn i_046c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x46C94;
    10
}
/// $046C5A: movem.w d0-d1, $3c0000.l
pub fn i_046c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C62;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046C62: addq.w #$1, d0
pub fn i_046c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C64;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046C64: addi.w #$10, d1
pub fn i_046c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C68;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C68: movem.w d0-d1, $3c0000.l
pub fn i_046c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C70;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046C70: addq.w #$1, d0
pub fn i_046c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C72;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046C72: addi.w #$10, d1
pub fn i_046c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C76;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C76: movem.w d0-d1, $3c0000.l
pub fn i_046c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C7E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046C7E: addq.w #$1, d0
pub fn i_046c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C80;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046C80: addi.w #$10, d1
pub fn i_046c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C84;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C84: movem.w d0-d1, $3c0000.l
pub fn i_046c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C8C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046C8C: addi.w #$1d, d0
pub fn i_046c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C90;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C90: subi.w #$2f, d1
pub fn i_046c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C94;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x2F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046C94: bra.b $46c34
pub fn i_046c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C96;
    cpu.ir = 0x609E;
    cpu.pc = 0x46C34;
    10
}
// ===== FixBlit_BatchRow4x1_FromTable_046c96  ($046C96, 120 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046C96: lea.l $28def4.l, a1
pub fn i_046c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DEF4;
    20
}
/// $046C9C: lsl.w #$1, d5
pub fn i_046c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C9E;
    cpu.ir = 0xE34D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $046C9E: move.w (a1, d5.w), d1
pub fn i_046c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CA2;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $046CA2: add.w d4, d1
pub fn i_046ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CA4;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046CA4: move.w #$0, d6
pub fn i_046ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CA8;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046CA8: bra.b $46cac
pub fn i_046ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CAA;
    cpu.ir = 0x6002;
    cpu.pc = 0x46CAC;
    10
}
/// $046CAA: addq.w #$1, d6
pub fn i_046caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CAC;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046CAC: cmpi.w #$3, d6
pub fn i_046cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CB0;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $046CB0: bgt.w $46d0c
pub fn i_046cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CB4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46D0C; return 10; }
    8
}
/// $046CB4: cmpi.w #$74ff, d0
pub fn i_046cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CB8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046CB8: bls.w $46cbe
pub fn i_046cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CBC;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46CBE; return 10; }
    8
}
/// $046CBC: rts
pub fn i_046cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046CBE: cmpi.w #$7000, d0
pub fn i_046cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CC2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046CC2: bcc.w $46cd0
pub fn i_046cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46CD0; return 10; }
    8
}
/// $046CC6: addi.w #$20, d0
pub fn i_046cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CCA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046CCA: addq.w #$1, d1
pub fn i_046cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CCC;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046CCC: bra.w $46d0a
pub fn i_046ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x46D0A;
    10
}
/// $046CD0: movem.w d0-d1, $3c0000.l
pub fn i_046cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CD8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046CD8: addq.w #$1, d0
pub fn i_046cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CDA;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046CDA: addi.w #$10, d1
pub fn i_046cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CDE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046CDE: movem.w d0-d1, $3c0000.l
pub fn i_046cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CE6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046CE6: addq.w #$1, d0
pub fn i_046ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CE8;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046CE8: addi.w #$10, d1
pub fn i_046ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CEC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046CEC: movem.w d0-d1, $3c0000.l
pub fn i_046cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CF4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046CF4: addq.w #$1, d0
pub fn i_046cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CF6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046CF6: addi.w #$10, d1
pub fn i_046cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46CFA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046CFA: movem.w d0-d1, $3c0000.l
pub fn i_046cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D02;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046D02: addi.w #$1d, d0
pub fn i_046d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D06;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D06: subi.w #$2f, d1
pub fn i_046d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D0A;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x2F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D0A: bra.b $46caa
pub fn i_046d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D0C;
    cpu.ir = 0x609E;
    cpu.pc = 0x46CAA;
    10
}
/// $046D0C: rts
pub fn i_046d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBanner_MissionStart_Blit_046d0e  ($046D0E, 356 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046D0E: move.w #$7000, d0
pub fn i_046d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D12;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046D12: asl.w #$5, d2
pub fn i_046d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D14;
    cpu.ir = 0xEB42;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $046D14: add.w d2, d0
pub fn i_046d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D16;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046D16: add.w d3, d0
pub fn i_046d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D18;
    cpu.ir = 0xD043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046D18: cmpi.w #$0, d4
pub fn i_046d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D1C;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $046D1C: bne.w $46d28
pub fn i_046d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46D28; return 10; }
    8
}
/// $046D20: move.w #$0, d4
pub fn i_046d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D24;
    cpu.ir = 0x383C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046D24: bra.w $46d2c
pub fn i_046d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D28;
    cpu.ir = 0x6000;
    cpu.pc = 0x46D2C;
    10
}
/// $046D28: move.w #$1000, d4
pub fn i_046d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D2C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046D2C: move.w #$3e40, d1
pub fn i_046d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3E40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046D30: add.w d4, d1
pub fn i_046d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D32;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046D32: move.w #$0, d5
pub fn i_046d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D36;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046D36: bra.b $46d3a
pub fn i_046d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D38;
    cpu.ir = 0x6002;
    cpu.pc = 0x46D3A;
    10
}
/// $046D38: addq.w #$1, d5
pub fn i_046d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D3A;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046D3A: cmpi.w #$f, d5
pub fn i_046d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D3E;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF, Size::Word, false);
    8
}
/// $046D3E: bgt.w $46d9a
pub fn i_046d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D42;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46D9A; return 10; }
    8
}
/// $046D42: cmpi.w #$74ff, d0
pub fn i_046d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D46;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046D46: bls.w $46d4c
pub fn i_046d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D4A;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46D4C; return 10; }
    8
}
/// $046D4A: rts
pub fn i_046d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046D4C: cmpi.w #$7000, d0
pub fn i_046d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D50;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046D50: bcc.w $46d5e
pub fn i_046d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46D5E; return 10; }
    8
}
/// $046D54: addi.w #$20, d0
pub fn i_046d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D58;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D58: addq.w #$1, d1
pub fn i_046d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D5A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046D5A: bra.w $46d98
pub fn i_046d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x46D98;
    10
}
/// $046D5E: movem.w d0-d1, $3c0000.l
pub fn i_046d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D66;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046D66: addq.w #$1, d0
pub fn i_046d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D68;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046D68: addi.w #$10, d1
pub fn i_046d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D6C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D6C: movem.w d0-d1, $3c0000.l
pub fn i_046d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D74;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046D74: addq.w #$1, d0
pub fn i_046d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D76;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046D76: addi.w #$10, d1
pub fn i_046d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D7A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D7A: movem.w d0-d1, $3c0000.l
pub fn i_046d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D82;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046D82: addq.w #$1, d0
pub fn i_046d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D84;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046D84: addi.w #$10, d1
pub fn i_046d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D88;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D88: movem.w d0-d1, $3c0000.l
pub fn i_046d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D90;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046D90: addi.w #$1d, d0
pub fn i_046d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D94;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D94: subi.w #$2f, d1
pub fn i_046d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D98;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x2F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046D98: bra.b $46d38
pub fn i_046d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D9A;
    cpu.ir = 0x609E;
    cpu.pc = 0x46D38;
    10
}
/// $046D9A: move.w #$3f40, d1
pub fn i_046d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46D9E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3F40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046D9E: add.w d4, d1
pub fn i_046d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DA0;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046DA0: move.w #$0, d5
pub fn i_046da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DA4;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046DA4: bra.b $46da8
pub fn i_046da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DA6;
    cpu.ir = 0x6002;
    cpu.pc = 0x46DA8;
    10
}
/// $046DA6: addq.w #$1, d5
pub fn i_046da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DA8;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046DA8: cmpi.w #$1, d5
pub fn i_046da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DAC;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $046DAC: bgt.w $46e08
pub fn i_046dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DB0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46E08; return 10; }
    8
}
/// $046DB0: cmpi.w #$74ff, d0
pub fn i_046db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DB4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046DB4: bls.w $46dba
pub fn i_046db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DB8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46DBA; return 10; }
    8
}
/// $046DB8: rts
pub fn i_046db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046DBA: cmpi.w #$7000, d0
pub fn i_046dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DBE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046DBE: bcc.w $46dcc
pub fn i_046dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46DCC; return 10; }
    8
}
/// $046DC2: addi.w #$20, d0
pub fn i_046dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DC6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046DC6: addq.w #$1, d1
pub fn i_046dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DC8;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046DC8: bra.w $46e06
pub fn i_046dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x46E06;
    10
}
/// $046DCC: movem.w d0-d1, $3c0000.l
pub fn i_046dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DD4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046DD4: addq.w #$1, d0
pub fn i_046dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DD6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046DD6: addi.w #$10, d1
pub fn i_046dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DDA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046DDA: movem.w d0-d1, $3c0000.l
pub fn i_046dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DE2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046DE2: addq.w #$1, d0
pub fn i_046de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DE4;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046DE4: addi.w #$10, d1
pub fn i_046de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DE8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046DE8: movem.w d0-d1, $3c0000.l
pub fn i_046de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DF0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046DF0: addq.w #$1, d0
pub fn i_046df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DF2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046DF2: addi.w #$10, d1
pub fn i_046df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DF6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046DF6: movem.w d0-d1, $3c0000.l
pub fn i_046df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46DFE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046DFE: addi.w #$1d, d0
pub fn i_046dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E02;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046E02: subi.w #$2f, d1
pub fn i_046e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E06;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x2F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046E06: bra.b $46da6
pub fn i_046e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E08;
    cpu.ir = 0x609E;
    cpu.pc = 0x46DA6;
    10
}
/// $046E08: cmpi.w #$74ff, d0
pub fn i_046e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E0C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046E0C: bls.w $46e12
pub fn i_046e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E10;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46E12; return 10; }
    8
}
/// $046E10: rts
pub fn i_046e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046E12: cmpi.w #$7000, d0
pub fn i_046e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E16;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046E16: bcc.w $46e1c
pub fn i_046e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E1A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46E1C; return 10; }
    8
}
/// $046E1A: rts
pub fn i_046e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046E1C: move.w #$2320, d1
pub fn i_046e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E20;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046E20: movem.w d0-d1, $3c0000.l
pub fn i_046e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E28;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E28: addq.w #$1, d0
pub fn i_046e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E2A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046E2A: movem.w d0-d1, $3c0000.l
pub fn i_046e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E32;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E32: addq.w #$1, d0
pub fn i_046e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E34;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046E34: movem.w d0-d1, $3c0000.l
pub fn i_046e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E3C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E3C: addq.w #$1, d0
pub fn i_046e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E3E;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046E3E: movem.w d0-d1, $3c0000.l
pub fn i_046e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E46;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E46: addi.w #$1d, d0
pub fn i_046e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E4A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046E4A: movem.w d0-d1, $3c0000.l
pub fn i_046e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E52;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E52: addq.w #$1, d0
pub fn i_046e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E54;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046E54: movem.w d0-d1, $3c0000.l
pub fn i_046e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E5C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E5C: addq.w #$1, d0
pub fn i_046e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E5E;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046E5E: movem.w d0-d1, $3c0000.l
pub fn i_046e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E66;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E66: addq.w #$1, d0
pub fn i_046e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E68;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046E68: movem.w d0-d1, $3c0000.l
pub fn i_046e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E70;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046E70: rts
pub fn i_046e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBanner_MissionComplete_Blit_046e72  ($046E72, 356 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046E72: move.w #$7000, d0
pub fn i_046e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046E76: asl.w #$5, d2
pub fn i_046e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E78;
    cpu.ir = 0xEB42;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $046E78: add.w d2, d0
pub fn i_046e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E7A;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046E7A: add.w d3, d0
pub fn i_046e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E7C;
    cpu.ir = 0xD043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046E7C: cmpi.w #$0, d4
pub fn i_046e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E80;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $046E80: bne.w $46e8c
pub fn i_046e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46E8C; return 10; }
    8
}
/// $046E84: move.w #$0, d4
pub fn i_046e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E88;
    cpu.ir = 0x383C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046E88: bra.w $46e90
pub fn i_046e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x46E90;
    10
}
/// $046E8C: move.w #$1000, d4
pub fn i_046e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E90;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046E90: move.w #$3e80, d1
pub fn i_046e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E94;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3E80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046E94: add.w d4, d1
pub fn i_046e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E96;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046E96: move.w #$0, d5
pub fn i_046e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E9A;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046E9A: bra.b $46e9e
pub fn i_046e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E9C;
    cpu.ir = 0x6002;
    cpu.pc = 0x46E9E;
    10
}
/// $046E9C: addq.w #$1, d5
pub fn i_046e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46E9E;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046E9E: cmpi.w #$f, d5
pub fn i_046e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EA2;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF, Size::Word, false);
    8
}
/// $046EA2: bgt.w $46efe
pub fn i_046ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EA6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46EFE; return 10; }
    8
}
/// $046EA6: cmpi.w #$74ff, d0
pub fn i_046ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EAA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046EAA: bls.w $46eb0
pub fn i_046eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EAE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46EB0; return 10; }
    8
}
/// $046EAE: rts
pub fn i_046eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046EB0: cmpi.w #$7000, d0
pub fn i_046eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EB4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046EB4: bcc.w $46ec2
pub fn i_046eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46EC2; return 10; }
    8
}
/// $046EB8: addi.w #$20, d0
pub fn i_046eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EBC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046EBC: addq.w #$1, d1
pub fn i_046ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EBE;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046EBE: bra.w $46efc
pub fn i_046ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x46EFC;
    10
}
/// $046EC2: movem.w d0-d1, $3c0000.l
pub fn i_046ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ECA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046ECA: addq.w #$1, d0
pub fn i_046eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ECC;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046ECC: addi.w #$10, d1
pub fn i_046ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ED0;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046ED0: movem.w d0-d1, $3c0000.l
pub fn i_046ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ED8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046ED8: addq.w #$1, d0
pub fn i_046ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EDA;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046EDA: addi.w #$10, d1
pub fn i_046eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EDE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046EDE: movem.w d0-d1, $3c0000.l
pub fn i_046ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EE6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046EE6: addq.w #$1, d0
pub fn i_046ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EE8;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046EE8: addi.w #$10, d1
pub fn i_046ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EEC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046EEC: movem.w d0-d1, $3c0000.l
pub fn i_046eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EF4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046EF4: addi.w #$1d, d0
pub fn i_046ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EF8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046EF8: subi.w #$2f, d1
pub fn i_046ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EFC;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x2F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046EFC: bra.b $46e9c
pub fn i_046efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46EFE;
    cpu.ir = 0x609E;
    cpu.pc = 0x46E9C;
    10
}
/// $046EFE: move.w #$3f80, d1
pub fn i_046efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3F80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046F02: add.w d4, d1
pub fn i_046f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F04;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $046F04: move.w #$0, d5
pub fn i_046f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F08;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046F08: bra.b $46f0c
pub fn i_046f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F0A;
    cpu.ir = 0x6002;
    cpu.pc = 0x46F0C;
    10
}
/// $046F0A: addq.w #$1, d5
pub fn i_046f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F0C;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046F0C: cmpi.w #$b, d5
pub fn i_046f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F10;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB, Size::Word, false);
    8
}
/// $046F10: bgt.w $46f6c
pub fn i_046f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F14;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46F6C; return 10; }
    8
}
/// $046F14: cmpi.w #$74ff, d0
pub fn i_046f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F18;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046F18: bls.w $46f1e
pub fn i_046f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F1C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46F1E; return 10; }
    8
}
/// $046F1C: rts
pub fn i_046f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046F1E: cmpi.w #$7000, d0
pub fn i_046f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F22;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046F22: bcc.w $46f30
pub fn i_046f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46F30; return 10; }
    8
}
/// $046F26: addi.w #$20, d0
pub fn i_046f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F2A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046F2A: addq.w #$1, d1
pub fn i_046f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F2C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046F2C: bra.w $46f6a
pub fn i_046f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F30;
    cpu.ir = 0x6000;
    cpu.pc = 0x46F6A;
    10
}
/// $046F30: movem.w d0-d1, $3c0000.l
pub fn i_046f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F38;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046F38: addq.w #$1, d0
pub fn i_046f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F3A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046F3A: addi.w #$10, d1
pub fn i_046f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F3E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046F3E: movem.w d0-d1, $3c0000.l
pub fn i_046f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F46;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046F46: addq.w #$1, d0
pub fn i_046f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F48;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046F48: addi.w #$10, d1
pub fn i_046f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F4C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046F4C: movem.w d0-d1, $3c0000.l
pub fn i_046f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F54;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046F54: addq.w #$1, d0
pub fn i_046f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F56;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046F56: addi.w #$10, d1
pub fn i_046f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F5A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046F5A: movem.w d0-d1, $3c0000.l
pub fn i_046f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F62;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046F62: addi.w #$1d, d0
pub fn i_046f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F66;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046F66: subi.w #$2f, d1
pub fn i_046f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F6A;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x2F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046F6A: bra.b $46f0a
pub fn i_046f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F6C;
    cpu.ir = 0x609E;
    cpu.pc = 0x46F0A;
    10
}
/// $046F6C: cmpi.w #$74ff, d0
pub fn i_046f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F70;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $046F70: bls.w $46f76
pub fn i_046f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F74;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46F76; return 10; }
    8
}
/// $046F74: rts
pub fn i_046f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046F76: cmpi.w #$7000, d0
pub fn i_046f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F7A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $046F7A: bcc.w $46f80
pub fn i_046f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x46F80; return 10; }
    8
}
/// $046F7E: rts
pub fn i_046f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $046F80: move.w #$2320, d1
pub fn i_046f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F84;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046F84: movem.w d0-d1, $3c0000.l
pub fn i_046f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F8C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046F8C: addq.w #$1, d0
pub fn i_046f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F8E;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046F8E: movem.w d0-d1, $3c0000.l
pub fn i_046f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F96;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046F96: addq.w #$1, d0
pub fn i_046f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46F98;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046F98: movem.w d0-d1, $3c0000.l
pub fn i_046f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FA0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046FA0: addq.w #$1, d0
pub fn i_046fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FA2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046FA2: movem.w d0-d1, $3c0000.l
pub fn i_046fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FAA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046FAA: addi.w #$1d, d0
pub fn i_046faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FAE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046FAE: movem.w d0-d1, $3c0000.l
pub fn i_046fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FB6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046FB6: addq.w #$1, d0
pub fn i_046fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FB8;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046FB8: movem.w d0-d1, $3c0000.l
pub fn i_046fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FC0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046FC0: addq.w #$1, d0
pub fn i_046fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FC2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046FC2: movem.w d0-d1, $3c0000.l
pub fn i_046fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FCA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046FCA: addq.w #$1, d0
pub fn i_046fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FCC;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046FCC: movem.w d0-d1, $3c0000.l
pub fn i_046fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FD4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046FD4: rts
pub fn i_046fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionNumBanner_Task_046fd6  ($046FD6, 92 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046FD6: clr.w d0
pub fn i_046fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FD8;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $046FD8: move.b $106ecf.l, d0
pub fn i_046fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FDE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $046FDE: cmpi.w #$6, d0
pub fn i_046fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FE2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $046FE2: bls.w $46fea
pub fn i_046fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FE6;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x46FEA; return 10; }
    8
}
/// $046FE6: move.w #$5, d0
pub fn i_046fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046FEA: move.w d0, $5c(a6)
pub fn i_046fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FEE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046FEE: move.w #$ffe9, $22(a6)
pub fn i_046fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FF4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046FF4: move.w #$9, $24(a6)
pub fn i_046ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46FFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046FFA: move.w #$2, $28(a6)
pub fn i_046ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47000;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047000: lea.l $47006(pc), a1
pub fn i_047000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47004;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47006;
    16
}
/// $047004: move.l a1, (a6)
pub fn i_047004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47006;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047006: move.w $28(a6), d0
pub fn i_047006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4700A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04700A: add.w d0, $22(a6)
pub fn i_04700a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4700E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04700E: move.w $22(a6), d2
pub fn i_04700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47012;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047012: move.w $24(a6), d3
pub fn i_047012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47016;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047016: move.w $14(a6), d4
pub fn i_047016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4701A;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04701A: move.w $5c(a6), d5
pub fn i_04701a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4701E;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04701E: jsr $46b20(pc)
pub fn i_04701e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47022;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47022);
    cpu.pc = 0x46B20;
    16
}
/// $047022: cmpi.w #$7, $22(a6)
pub fn i_047022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47028;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Word, false);
    16
}
/// $047028: blt.w $47038
pub fn i_047028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4702C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x47038; return 10; }
    8
}
/// $04702C: move.w #$78, $70(a6)
pub fn i_04702c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47032;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_047032  ($047032, 8 B, kernel/task_handlers.c)
/// $047032: lea.l $4703a(pc), a1
pub fn i_047032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47036;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4703A;
    16
}
/// $047036: move.l a1, (a6)
pub fn i_047036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47038;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047038: rts
pub fn i_047038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4703A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionNumBanner_WaitDismiss_04703a  ($04703A, 14 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04703A: movea.l $c(a6), a1
pub fn i_04703a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4703E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04703E: cmpi.b #$0, $20(a1)
pub fn i_04703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47044;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $047044: beq.w $4704e
pub fn i_047044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47048;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4704E; return 10; }
    8
}
// ===== SetTaskHandler_047048  ($047048, 8 B, kernel/task_handlers.c)
/// $047048: lea.l $47050(pc), a1
pub fn i_047048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4704C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47050;
    16
}
/// $04704C: move.l a1, (a6)
pub fn i_04704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4704E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04704E: rts
pub fn i_04704e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47050;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionNumBanner_ScrollOut_047050  ($047050, 46 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047050: move.w $28(a6), d0
pub fn i_047050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47054;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047054: add.w d0, $22(a6)
pub fn i_047054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47058;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047058: move.w $22(a6), d2
pub fn i_047058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4705C;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04705C: move.w $24(a6), d3
pub fn i_04705c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47060;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047060: move.w $14(a6), d4
pub fn i_047060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47064;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047064: move.w $5c(a6), d5
pub fn i_047064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47068;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047068: jsr $46b20(pc)
pub fn i_047068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4706C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4706C);
    cpu.pc = 0x46B20;
    16
}
/// $04706C: cmpi.w #$27, $22(a6)
pub fn i_04706c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47072;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x27, Size::Word, false);
    16
}
/// $047072: ble.w $4707c
pub fn i_047072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47076;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4707C; return 10; }
    8
}
/// $047076: jmp $518.l
pub fn i_047076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4707C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04707C: rts
pub fn i_04707c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4707E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionNum_TileByMission_04707e  ($04707E, 24 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04707E: clr.l d1
pub fn i_04707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47080;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $047080: move.b $106ecf.l, d1
pub fn i_047080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47086;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ECF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $047086: andi.w #$7, d1
pub fn i_047086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4708A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04708A: lsl.w #$1, d1
pub fn i_04708a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4708C;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04708C: lea.l $28df00.l, a1
pub fn i_04708c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47092;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DF00;
    20
}
/// $047092: move.w (a1, d1.w), d0
pub fn i_047092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47096;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
// ===== JsrAbsThunk_047096  ($047096, 8 B, thunks/jsr_abs_thunks.c)
/// $047096: jsr $2352.l
pub fn i_047096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4709C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4709C);
    cpu.pc = 0x2352;
    16
}
/// $04709C: rts
pub fn i_04709c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4709E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionNum_PalByMission_04709e  ($04709E, 24 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04709E: clr.l d1
pub fn i_04709e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470A0;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $0470A0: move.b $106ecf.l, d1
pub fn i_0470a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470A6;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ECF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0470A6: andi.w #$7, d1
pub fn i_0470a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470AA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0470AA: lsl.w #$1, d1
pub fn i_0470aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470AC;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0470AC: lea.l $28df10.l, a1
pub fn i_0470ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470B2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DF10;
    20
}
/// $0470B2: move.w (a1, d1.w), d0
pub fn i_0470b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470B6;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
// ===== JsrAbsThunk_0470b6  ($0470B6, 8 B, thunks/jsr_abs_thunks.c)
/// $0470B6: jsr $2352.l
pub fn i_0470b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x470BC);
    cpu.pc = 0x2352;
    16
}
/// $0470BC: rts
pub fn i_0470bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionStart_Task_0470be  ($0470BE, 96 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0470BE: clr.w d0
pub fn i_0470be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470C0;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0470C0: move.w d0, $14(a6)
pub fn i_0470c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470C4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0470C4: lea.l $46fd6(pc), a1
pub fn i_0470c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46FD6;
    16
}
/// $0470C8: jsr $4ae.l
pub fn i_0470c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x470CE);
    cpu.pc = 0x4AE;
    16
}
/// $0470CE: move.w $14(a6), $14(a0)
pub fn i_0470ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470D4;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0470D4: clr.b $20(a6)
pub fn i_0470d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470D8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0470D8: move.w #$27, $22(a6)
pub fn i_0470d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0470DE: move.w #$f, $24(a6)
pub fn i_0470de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0470E4: move.w #$fffe, $28(a6)
pub fn i_0470e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0470EA: lea.l $470f0(pc), a1
pub fn i_0470ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x470F0;
    16
}
/// $0470EE: move.l a1, (a6)
pub fn i_0470ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0470F0: move.w $28(a6), d0
pub fn i_0470f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470F4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0470F4: add.w d0, $22(a6)
pub fn i_0470f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470F8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0470F8: move.w $22(a6), d2
pub fn i_0470f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470FC;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0470FC: move.w $24(a6), d3
pub fn i_0470fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47100;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047100: move.w $14(a6), d4
pub fn i_047100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47104;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047104: jsr $46d0e(pc)
pub fn i_047104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47108);
    cpu.pc = 0x46D0E;
    16
}
/// $047108: cmpi.w #$b, $22(a6)
pub fn i_047108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4710E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB, Size::Word, false);
    16
}
/// $04710E: bgt.w $47122
pub fn i_04710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47112;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x47122; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x04566A, i_04566a),
    (0x04566E, i_04566e),
    (0x045672, i_045672),
    (0x045676, i_045676),
    (0x04567A, i_04567a),
    (0x04567E, i_04567e),
    (0x045682, i_045682),
    (0x045686, i_045686),
    (0x04568A, i_04568a),
    (0x04568E, i_04568e),
    (0x045692, i_045692),
    (0x045696, i_045696),
    (0x04569C, i_04569c),
    (0x0456A0, i_0456a0),
    (0x0456A4, i_0456a4),
    (0x0456A6, i_0456a6),
    (0x0456AA, i_0456aa),
    (0x0456AE, i_0456ae),
    (0x0456B2, i_0456b2),
    (0x0456B6, i_0456b6),
    (0x0456BE, i_0456be),
    (0x0456C2, i_0456c2),
    (0x0456CA, i_0456ca),
    (0x0456CE, i_0456ce),
    (0x0456D6, i_0456d6),
    (0x0456DA, i_0456da),
    (0x0456E0, i_0456e0),
    (0x0456E4, i_0456e4),
    (0x0456EA, i_0456ea),
    (0x0456EE, i_0456ee),
    (0x0456F2, i_0456f2),
    (0x0456F6, i_0456f6),
    (0x0456FC, i_0456fc),
    (0x045700, i_045700),
    (0x045704, i_045704),
    (0x045706, i_045706),
    (0x04570C, i_04570c),
    (0x045710, i_045710),
    (0x045714, i_045714),
    (0x045716, i_045716),
    (0x04571C, i_04571c),
    (0x045720, i_045720),
    (0x045726, i_045726),
    (0x04572A, i_04572a),
    (0x04572C, i_04572c),
    (0x045730, i_045730),
    (0x045732, i_045732),
    (0x045736, i_045736),
    (0x04573C, i_04573c),
    (0x045740, i_045740),
    (0x045744, i_045744),
    (0x045748, i_045748),
    (0x04574C, i_04574c),
    (0x045752, i_045752),
    (0x045756, i_045756),
    (0x04575A, i_04575a),
    (0x04575E, i_04575e),
    (0x045762, i_045762),
    (0x045766, i_045766),
    (0x04576A, i_04576a),
    (0x04576E, i_04576e),
    (0x045772, i_045772),
    (0x045776, i_045776),
    (0x04577A, i_04577a),
    (0x04577E, i_04577e),
    (0x045782, i_045782),
    (0x045784, i_045784),
    (0x045788, i_045788),
    (0x04578A, i_04578a),
    (0x04578E, i_04578e),
    (0x045792, i_045792),
    (0x045796, i_045796),
    (0x04579A, i_04579a),
    (0x04579E, i_04579e),
    (0x0457A0, i_0457a0),
    (0x0457A4, i_0457a4),
    (0x0457A6, i_0457a6),
    (0x0457AA, i_0457aa),
    (0x0457AE, i_0457ae),
    (0x0457B2, i_0457b2),
    (0x0457B6, i_0457b6),
    (0x0457BA, i_0457ba),
    (0x0457BE, i_0457be),
    (0x0457C2, i_0457c2),
    (0x0457C6, i_0457c6),
    (0x0457CC, i_0457cc),
    (0x0457D2, i_0457d2),
    (0x0457D6, i_0457d6),
    (0x0457D8, i_0457d8),
    (0x0457DC, i_0457dc),
    (0x0457DE, i_0457de),
    (0x0457E0, i_0457e0),
    (0x0457E2, i_0457e2),
    (0x0457E6, i_0457e6),
    (0x0457EA, i_0457ea),
    (0x0457EC, i_0457ec),
    (0x0457EE, i_0457ee),
    (0x0457F0, i_0457f0),
    (0x0457F6, i_0457f6),
    (0x0457FC, i_0457fc),
    (0x045800, i_045800),
    (0x045804, i_045804),
    (0x045806, i_045806),
    (0x04580A, i_04580a),
    (0x04580C, i_04580c),
    (0x045810, i_045810),
    (0x045816, i_045816),
    (0x04581A, i_04581a),
    (0x045820, i_045820),
    (0x045826, i_045826),
    (0x04582C, i_04582c),
    (0x045830, i_045830),
    (0x045832, i_045832),
    (0x045836, i_045836),
    (0x04583C, i_04583c),
    (0x045842, i_045842),
    (0x045848, i_045848),
    (0x04584C, i_04584c),
    (0x045852, i_045852),
    (0x045858, i_045858),
    (0x04585C, i_04585c),
    (0x045860, i_045860),
    (0x045864, i_045864),
    (0x045866, i_045866),
    (0x04586C, i_04586c),
    (0x045870, i_045870),
    (0x045874, i_045874),
    (0x045878, i_045878),
    (0x04587C, i_04587c),
    (0x045882, i_045882),
    (0x045886, i_045886),
    (0x04588A, i_04588a),
    (0x04588E, i_04588e),
    (0x045892, i_045892),
    (0x045896, i_045896),
    (0x04589A, i_04589a),
    (0x04589E, i_04589e),
    (0x0458A2, i_0458a2),
    (0x0458A6, i_0458a6),
    (0x0458AA, i_0458aa),
    (0x0458AE, i_0458ae),
    (0x0458B4, i_0458b4),
    (0x0458BA, i_0458ba),
    (0x0458C0, i_0458c0),
    (0x0458C4, i_0458c4),
    (0x0458C8, i_0458c8),
    (0x0458CC, i_0458cc),
    (0x0458CE, i_0458ce),
    (0x0458D2, i_0458d2),
    (0x0458D4, i_0458d4),
    (0x0458DA, i_0458da),
    (0x0458DC, i_0458dc),
    (0x0458E0, i_0458e0),
    (0x0458E6, i_0458e6),
    (0x0458EA, i_0458ea),
    (0x0458EE, i_0458ee),
    (0x0458F2, i_0458f2),
    (0x0458F6, i_0458f6),
    (0x0458FA, i_0458fa),
    (0x0458FE, i_0458fe),
    (0x045900, i_045900),
    (0x045902, i_045902),
    (0x045906, i_045906),
    (0x04590A, i_04590a),
    (0x04590E, i_04590e),
    (0x045912, i_045912),
    (0x045916, i_045916),
    (0x04591A, i_04591a),
    (0x04591E, i_04591e),
    (0x045922, i_045922),
    (0x045926, i_045926),
    (0x04592A, i_04592a),
    (0x04592E, i_04592e),
    (0x045934, i_045934),
    (0x045938, i_045938),
    (0x04593A, i_04593a),
    (0x04593E, i_04593e),
    (0x045942, i_045942),
    (0x045946, i_045946),
    (0x04594A, i_04594a),
    (0x04594E, i_04594e),
    (0x045952, i_045952),
    (0x045956, i_045956),
    (0x04595C, i_04595c),
    (0x045960, i_045960),
    (0x045966, i_045966),
    (0x04596A, i_04596a),
    (0x04596E, i_04596e),
    (0x045974, i_045974),
    (0x045978, i_045978),
    (0x04597C, i_04597c),
    (0x045980, i_045980),
    (0x045984, i_045984),
    (0x045988, i_045988),
    (0x04598C, i_04598c),
    (0x045990, i_045990),
    (0x045996, i_045996),
    (0x04599A, i_04599a),
    (0x04599C, i_04599c),
    (0x0459A2, i_0459a2),
    (0x0459A6, i_0459a6),
    (0x0459A8, i_0459a8),
    (0x0459AE, i_0459ae),
    (0x0459B6, i_0459b6),
    (0x0459BC, i_0459bc),
    (0x0459C0, i_0459c0),
    (0x0459C6, i_0459c6),
    (0x0459CE, i_0459ce),
    (0x0459D4, i_0459d4),
    (0x0459D8, i_0459d8),
    (0x0459DE, i_0459de),
    (0x0459E6, i_0459e6),
    (0x0459EC, i_0459ec),
    (0x0459F0, i_0459f0),
    (0x0459F6, i_0459f6),
    (0x0459FE, i_0459fe),
    (0x045A04, i_045a04),
    (0x045A0A, i_045a0a),
    (0x045A10, i_045a10),
    (0x045A14, i_045a14),
    (0x045A1A, i_045a1a),
    (0x045A20, i_045a20),
    (0x045A26, i_045a26),
    (0x045A2A, i_045a2a),
    (0x045A2E, i_045a2e),
    (0x045A32, i_045a32),
    (0x045A36, i_045a36),
    (0x045A38, i_045a38),
    (0x045A3C, i_045a3c),
    (0x045A40, i_045a40),
    (0x045A44, i_045a44),
    (0x045A48, i_045a48),
    (0x045A4C, i_045a4c),
    (0x045A4E, i_045a4e),
    (0x045A50, i_045a50),
    (0x045A54, i_045a54),
    (0x045A58, i_045a58),
    (0x045A5C, i_045a5c),
    (0x045A5E, i_045a5e),
    (0x045A62, i_045a62),
    (0x045A64, i_045a64),
    (0x045A66, i_045a66),
    (0x045A6A, i_045a6a),
    (0x045A6E, i_045a6e),
    (0x045A70, i_045a70),
    (0x045A74, i_045a74),
    (0x045A76, i_045a76),
    (0x045A7A, i_045a7a),
    (0x045A7C, i_045a7c),
    (0x045A7E, i_045a7e),
    (0x045A80, i_045a80),
    (0x045A84, i_045a84),
    (0x045A88, i_045a88),
    (0x045A8C, i_045a8c),
    (0x045A90, i_045a90),
    (0x045A94, i_045a94),
    (0x045A96, i_045a96),
    (0x045A9A, i_045a9a),
    (0x045A9E, i_045a9e),
    (0x045AA4, i_045aa4),
    (0x045AA8, i_045aa8),
    (0x045AAC, i_045aac),
    (0x045AAE, i_045aae),
    (0x045AB2, i_045ab2),
    (0x045AB6, i_045ab6),
    (0x045ABA, i_045aba),
    (0x045ABE, i_045abe),
    (0x045AC4, i_045ac4),
    (0x045ACC, i_045acc),
    (0x045AD2, i_045ad2),
    (0x045AD8, i_045ad8),
    (0x045ADE, i_045ade),
    (0x045AE2, i_045ae2),
    (0x045AE8, i_045ae8),
    (0x045AEE, i_045aee),
    (0x045AF4, i_045af4),
    (0x045AF8, i_045af8),
    (0x045AFC, i_045afc),
    (0x045B00, i_045b00),
    (0x045B02, i_045b02),
    (0x045B06, i_045b06),
    (0x045B0A, i_045b0a),
    (0x045B0E, i_045b0e),
    (0x045B12, i_045b12),
    (0x045B16, i_045b16),
    (0x045B18, i_045b18),
    (0x045B1A, i_045b1a),
    (0x045B1E, i_045b1e),
    (0x045B22, i_045b22),
    (0x045B26, i_045b26),
    (0x045B2A, i_045b2a),
    (0x045B2E, i_045b2e),
    (0x045B30, i_045b30),
    (0x045B34, i_045b34),
    (0x045B36, i_045b36),
    (0x045B3A, i_045b3a),
    (0x045B3C, i_045b3c),
    (0x045B40, i_045b40),
    (0x045B44, i_045b44),
    (0x045B48, i_045b48),
    (0x045B4C, i_045b4c),
    (0x045B4E, i_045b4e),
    (0x045B52, i_045b52),
    (0x045B54, i_045b54),
    (0x045B56, i_045b56),
    (0x045B5A, i_045b5a),
    (0x045B5C, i_045b5c),
    (0x045B60, i_045b60),
    (0x045B62, i_045b62),
    (0x045B64, i_045b64),
    (0x045B66, i_045b66),
    (0x045B6A, i_045b6a),
    (0x045B6E, i_045b6e),
    (0x045B72, i_045b72),
    (0x045B74, i_045b74),
    (0x045B78, i_045b78),
    (0x045B7C, i_045b7c),
    (0x045B82, i_045b82),
    (0x045B86, i_045b86),
    (0x045B8A, i_045b8a),
    (0x045B8C, i_045b8c),
    (0x045B90, i_045b90),
    (0x045B94, i_045b94),
    (0x045B98, i_045b98),
    (0x045B9C, i_045b9c),
    (0x045BA0, i_045ba0),
    (0x045BA6, i_045ba6),
    (0x045BAC, i_045bac),
    (0x045BB0, i_045bb0),
    (0x045BB4, i_045bb4),
    (0x045BB8, i_045bb8),
    (0x045BBE, i_045bbe),
    (0x045BC2, i_045bc2),
    (0x045BC6, i_045bc6),
    (0x045BCA, i_045bca),
    (0x045BCC, i_045bcc),
    (0x045BD2, i_045bd2),
    (0x045BD6, i_045bd6),
    (0x045BDA, i_045bda),
    (0x045BE0, i_045be0),
    (0x045BE4, i_045be4),
    (0x045BE6, i_045be6),
    (0x045BE8, i_045be8),
    (0x045BEE, i_045bee),
    (0x045BF2, i_045bf2),
    (0x045BF8, i_045bf8),
    (0x045BFA, i_045bfa),
    (0x045C00, i_045c00),
    (0x045C06, i_045c06),
    (0x045C0C, i_045c0c),
    (0x045C12, i_045c12),
    (0x045C18, i_045c18),
    (0x045C20, i_045c20),
    (0x045C24, i_045c24),
    (0x045C2A, i_045c2a),
    (0x045C2C, i_045c2c),
    (0x045C32, i_045c32),
    (0x045C36, i_045c36),
    (0x045C3C, i_045c3c),
    (0x045C3E, i_045c3e),
    (0x045C44, i_045c44),
    (0x045C4A, i_045c4a),
    (0x045C50, i_045c50),
    (0x045C56, i_045c56),
    (0x045C5C, i_045c5c),
    (0x045C62, i_045c62),
    (0x045C68, i_045c68),
    (0x045C6C, i_045c6c),
    (0x045C70, i_045c70),
    (0x045C72, i_045c72),
    (0x045C78, i_045c78),
    (0x045C7C, i_045c7c),
    (0x045C7E, i_045c7e),
    (0x045C82, i_045c82),
    (0x045C86, i_045c86),
    (0x045C8A, i_045c8a),
    (0x045C8E, i_045c8e),
    (0x045C92, i_045c92),
    (0x045C94, i_045c94),
    (0x045C98, i_045c98),
    (0x045C9C, i_045c9c),
    (0x045CA2, i_045ca2),
    (0x045CA8, i_045ca8),
    (0x045CAC, i_045cac),
    (0x045CAE, i_045cae),
    (0x045CB2, i_045cb2),
    (0x045CB8, i_045cb8),
    (0x045CBC, i_045cbc),
    (0x045CC0, i_045cc0),
    (0x045CC2, i_045cc2),
    (0x045CC4, i_045cc4),
    (0x045CC8, i_045cc8),
    (0x045CCC, i_045ccc),
    (0x045CD2, i_045cd2),
    (0x045CD4, i_045cd4),
    (0x045CEA, i_045cea),
    (0x045CF0, i_045cf0),
    (0x045CF4, i_045cf4),
    (0x045CF8, i_045cf8),
    (0x045CFE, i_045cfe),
    (0x045D04, i_045d04),
    (0x045D0A, i_045d0a),
    (0x045D0E, i_045d0e),
    (0x045D10, i_045d10),
    (0x045D14, i_045d14),
    (0x045D1A, i_045d1a),
    (0x045D1E, i_045d1e),
    (0x045D22, i_045d22),
    (0x045D26, i_045d26),
    (0x045D28, i_045d28),
    (0x045D2A, i_045d2a),
    (0x045D2E, i_045d2e),
    (0x045D34, i_045d34),
    (0x045D38, i_045d38),
    (0x045D3A, i_045d3a),
    (0x045D3E, i_045d3e),
    (0x045D42, i_045d42),
    (0x045D48, i_045d48),
    (0x045D4E, i_045d4e),
    (0x045D54, i_045d54),
    (0x045D58, i_045d58),
    (0x045D5E, i_045d5e),
    (0x045D62, i_045d62),
    (0x045D68, i_045d68),
    (0x045D6E, i_045d6e),
    (0x045D74, i_045d74),
    (0x045D7A, i_045d7a),
    (0x045D80, i_045d80),
    (0x045D86, i_045d86),
    (0x045D8A, i_045d8a),
    (0x045D8C, i_045d8c),
    (0x045D92, i_045d92),
    (0x045D98, i_045d98),
    (0x045D9C, i_045d9c),
    (0x045DA0, i_045da0),
    (0x045DA4, i_045da4),
    (0x045DAA, i_045daa),
    (0x045DB0, i_045db0),
    (0x045DB6, i_045db6),
    (0x045DBC, i_045dbc),
    (0x045DC2, i_045dc2),
    (0x045DC6, i_045dc6),
    (0x045DCC, i_045dcc),
    (0x045DD2, i_045dd2),
    (0x045DE4, i_045de4),
    (0x045DEA, i_045dea),
    (0x045DF0, i_045df0),
    (0x045DF4, i_045df4),
    (0x045DFA, i_045dfa),
    (0x045E00, i_045e00),
    (0x045E04, i_045e04),
    (0x045E0A, i_045e0a),
    (0x045E0E, i_045e0e),
    (0x045E14, i_045e14),
    (0x045E1A, i_045e1a),
    (0x045E1E, i_045e1e),
    (0x045E24, i_045e24),
    (0x045E2A, i_045e2a),
    (0x045E30, i_045e30),
    (0x045E36, i_045e36),
    (0x045E3C, i_045e3c),
    (0x045E40, i_045e40),
    (0x045E42, i_045e42),
    (0x045E48, i_045e48),
    (0x045E4C, i_045e4c),
    (0x045E50, i_045e50),
    (0x045E52, i_045e52),
    (0x045E5A, i_045e5a),
    (0x045E5E, i_045e5e),
    (0x045E62, i_045e62),
    (0x045E66, i_045e66),
    (0x045E6A, i_045e6a),
    (0x045E6E, i_045e6e),
    (0x045E70, i_045e70),
    (0x045E76, i_045e76),
    (0x045E7C, i_045e7c),
    (0x045E82, i_045e82),
    (0x045E86, i_045e86),
    (0x045E8C, i_045e8c),
    (0x045E8E, i_045e8e),
    (0x045E94, i_045e94),
    (0x045E9A, i_045e9a),
    (0x045E9E, i_045e9e),
    (0x045EA0, i_045ea0),
    (0x045EA4, i_045ea4),
    (0x045EAA, i_045eaa),
    (0x045EB0, i_045eb0),
    (0x045EB4, i_045eb4),
    (0x045EBA, i_045eba),
    (0x045EBE, i_045ebe),
    (0x045EC4, i_045ec4),
    (0x045ECA, i_045eca),
    (0x045ED0, i_045ed0),
    (0x045ED4, i_045ed4),
    (0x045ED6, i_045ed6),
    (0x045EDC, i_045edc),
    (0x045EE0, i_045ee0),
    (0x045EE4, i_045ee4),
    (0x045EE6, i_045ee6),
    (0x045EEA, i_045eea),
    (0x045EF0, i_045ef0),
    (0x045EF6, i_045ef6),
    (0x045EFC, i_045efc),
    (0x045F02, i_045f02),
    (0x045F06, i_045f06),
    (0x045F0C, i_045f0c),
    (0x045F12, i_045f12),
    (0x045F16, i_045f16),
    (0x045F1A, i_045f1a),
    (0x045F20, i_045f20),
    (0x045F24, i_045f24),
    (0x045F28, i_045f28),
    (0x045F2A, i_045f2a),
    (0x045F2C, i_045f2c),
    (0x045F34, i_045f34),
    (0x045FDE, i_045fde),
    (0x045FE4, i_045fe4),
    (0x045FEA, i_045fea),
    (0x045FF0, i_045ff0),
    (0x045FF4, i_045ff4),
    (0x045FFA, i_045ffa),
    (0x046000, i_046000),
    (0x046006, i_046006),
    (0x04600C, i_04600c),
    (0x046012, i_046012),
    (0x046016, i_046016),
    (0x046018, i_046018),
    (0x04601A, i_04601a),
    (0x04601E, i_04601e),
    (0x046020, i_046020),
    (0x046022, i_046022),
    (0x046026, i_046026),
    (0x04602A, i_04602a),
    (0x046030, i_046030),
    (0x046034, i_046034),
    (0x046038, i_046038),
    (0x04603C, i_04603c),
    (0x046042, i_046042),
    (0x046046, i_046046),
    (0x04604A, i_04604a),
    (0x04604E, i_04604e),
    (0x046050, i_046050),
    (0x046056, i_046056),
    (0x046058, i_046058),
    (0x04605C, i_04605c),
    (0x046062, i_046062),
    (0x046066, i_046066),
    (0x04606C, i_04606c),
    (0x046072, i_046072),
    (0x046076, i_046076),
    (0x046078, i_046078),
    (0x04607E, i_04607e),
    (0x046084, i_046084),
    (0x046088, i_046088),
    (0x04608E, i_04608e),
    (0x046092, i_046092),
    (0x046096, i_046096),
    (0x046098, i_046098),
    (0x04609C, i_04609c),
    (0x0460A0, i_0460a0),
    (0x0460A4, i_0460a4),
    (0x0460A8, i_0460a8),
    (0x0460AC, i_0460ac),
    (0x0460B0, i_0460b0),
    (0x0460B4, i_0460b4),
    (0x0460BA, i_0460ba),
    (0x0460C0, i_0460c0),
    (0x0460C6, i_0460c6),
    (0x0460CC, i_0460cc),
    (0x0460D0, i_0460d0),
    (0x0460D4, i_0460d4),
    (0x0460D6, i_0460d6),
    (0x0460DC, i_0460dc),
    (0x0460E2, i_0460e2),
    (0x0460E6, i_0460e6),
    (0x0460EC, i_0460ec),
    (0x0460EE, i_0460ee),
    (0x0460F4, i_0460f4),
    (0x0460FA, i_0460fa),
    (0x046100, i_046100),
    (0x046106, i_046106),
    (0x046108, i_046108),
    (0x04610C, i_04610c),
    (0x046110, i_046110),
    (0x046114, i_046114),
    (0x046118, i_046118),
    (0x04611C, i_04611c),
    (0x04611E, i_04611e),
    (0x046122, i_046122),
    (0x046124, i_046124),
    (0x046128, i_046128),
    (0x04612C, i_04612c),
    (0x046130, i_046130),
    (0x046134, i_046134),
    (0x046138, i_046138),
    (0x04613A, i_04613a),
    (0x04613E, i_04613e),
    (0x046140, i_046140),
    (0x046146, i_046146),
    (0x04614A, i_04614a),
    (0x04614E, i_04614e),
    (0x046152, i_046152),
    (0x046156, i_046156),
    (0x04615C, i_04615c),
    (0x046162, i_046162),
    (0x046166, i_046166),
    (0x04616C, i_04616c),
    (0x046172, i_046172),
    (0x046178, i_046178),
    (0x04617C, i_04617c),
    (0x046180, i_046180),
    (0x046186, i_046186),
    (0x04618C, i_04618c),
    (0x046190, i_046190),
    (0x046192, i_046192),
    (0x046198, i_046198),
    (0x04619E, i_04619e),
    (0x0461A2, i_0461a2),
    (0x0461A8, i_0461a8),
    (0x0461AE, i_0461ae),
    (0x0461B2, i_0461b2),
    (0x0461B6, i_0461b6),
    (0x0461B8, i_0461b8),
    (0x0461BC, i_0461bc),
    (0x0461C2, i_0461c2),
    (0x0461C8, i_0461c8),
    (0x0461CE, i_0461ce),
    (0x0461D2, i_0461d2),
    (0x0461D4, i_0461d4),
    (0x0461D8, i_0461d8),
    (0x0461DE, i_0461de),
    (0x0461E4, i_0461e4),
    (0x0461E8, i_0461e8),
    (0x0461EA, i_0461ea),
    (0x0461F0, i_0461f0),
    (0x0461F6, i_0461f6),
    (0x0461FA, i_0461fa),
    (0x046200, i_046200),
    (0x046204, i_046204),
    (0x04620A, i_04620a),
    (0x046210, i_046210),
    (0x046214, i_046214),
    (0x046218, i_046218),
    (0x04621A, i_04621a),
    (0x046220, i_046220),
    (0x046226, i_046226),
    (0x04622A, i_04622a),
    (0x04622E, i_04622e),
    (0x046232, i_046232),
    (0x046236, i_046236),
    (0x04623C, i_04623c),
    (0x046240, i_046240),
    (0x046246, i_046246),
    (0x046248, i_046248),
    (0x04624E, i_04624e),
    (0x046254, i_046254),
    (0x046258, i_046258),
    (0x04625C, i_04625c),
    (0x04625E, i_04625e),
    (0x046260, i_046260),
    (0x046266, i_046266),
    (0x04626C, i_04626c),
    (0x046270, i_046270),
    (0x046272, i_046272),
    (0x046278, i_046278),
    (0x04627E, i_04627e),
    (0x046282, i_046282),
    (0x046286, i_046286),
    (0x046288, i_046288),
    (0x04628A, i_04628a),
    (0x04628E, i_04628e),
    (0x046294, i_046294),
    (0x04629A, i_04629a),
    (0x04629E, i_04629e),
    (0x0462A0, i_0462a0),
    (0x0462A6, i_0462a6),
    (0x0462AC, i_0462ac),
    (0x0462B0, i_0462b0),
    (0x0462B6, i_0462b6),
    (0x0462BA, i_0462ba),
    (0x0462BC, i_0462bc),
    (0x0462C0, i_0462c0),
    (0x0462C6, i_0462c6),
    (0x0462CA, i_0462ca),
    (0x0462CE, i_0462ce),
    (0x0462D2, i_0462d2),
    (0x0462D8, i_0462d8),
    (0x0462DC, i_0462dc),
    (0x0462E2, i_0462e2),
    (0x0462E4, i_0462e4),
    (0x0462E8, i_0462e8),
    (0x0462EE, i_0462ee),
    (0x0462F2, i_0462f2),
    (0x0462F8, i_0462f8),
    (0x0462FC, i_0462fc),
    (0x0462FE, i_0462fe),
    (0x046304, i_046304),
    (0x04630A, i_04630a),
    (0x04630E, i_04630e),
    (0x046314, i_046314),
    (0x046318, i_046318),
    (0x04631C, i_04631c),
    (0x04631E, i_04631e),
    (0x046322, i_046322),
    (0x046326, i_046326),
    (0x046328, i_046328),
    (0x04632C, i_04632c),
    (0x046332, i_046332),
    (0x046336, i_046336),
    (0x04633A, i_04633a),
    (0x046340, i_046340),
    (0x046344, i_046344),
    (0x046346, i_046346),
    (0x046348, i_046348),
    (0x04634C, i_04634c),
    (0x046350, i_046350),
    (0x046352, i_046352),
    (0x046358, i_046358),
    (0x04635C, i_04635c),
    (0x046360, i_046360),
    (0x046362, i_046362),
    (0x046366, i_046366),
    (0x04636A, i_04636a),
    (0x04636C, i_04636c),
    (0x046372, i_046372),
    (0x046378, i_046378),
    (0x04637E, i_04637e),
    (0x046382, i_046382),
    (0x046386, i_046386),
    (0x04638A, i_04638a),
    (0x04638E, i_04638e),
    (0x046390, i_046390),
    (0x046396, i_046396),
    (0x04639C, i_04639c),
    (0x0463A2, i_0463a2),
    (0x0463A8, i_0463a8),
    (0x0463AE, i_0463ae),
    (0x0463B2, i_0463b2),
    (0x0463B6, i_0463b6),
    (0x0463B8, i_0463b8),
    (0x0463BA, i_0463ba),
    (0x0463C2, i_0463c2),
    (0x0463C6, i_0463c6),
    (0x0463CA, i_0463ca),
    (0x0463D0, i_0463d0),
    (0x0463D4, i_0463d4),
    (0x0463D8, i_0463d8),
    (0x0463DA, i_0463da),
    (0x0463DE, i_0463de),
    (0x0463E4, i_0463e4),
    (0x0463EC, i_0463ec),
    (0x0463EE, i_0463ee),
    (0x0463F2, i_0463f2),
    (0x0463F8, i_0463f8),
    (0x046400, i_046400),
    (0x046404, i_046404),
    (0x046408, i_046408),
    (0x04640E, i_04640e),
    (0x046416, i_046416),
    (0x04641A, i_04641a),
    (0x04641E, i_04641e),
    (0x046424, i_046424),
    (0x04642C, i_04642c),
    (0x046430, i_046430),
    (0x046434, i_046434),
    (0x04643A, i_04643a),
    (0x046442, i_046442),
    (0x046446, i_046446),
    (0x04644A, i_04644a),
    (0x046450, i_046450),
    (0x046458, i_046458),
    (0x04645C, i_04645c),
    (0x046462, i_046462),
    (0x046466, i_046466),
    (0x04646A, i_04646a),
    (0x046470, i_046470),
    (0x046484, i_046484),
    (0x046488, i_046488),
    (0x04648C, i_04648c),
    (0x046490, i_046490),
    (0x046494, i_046494),
    (0x04649A, i_04649a),
    (0x0464A2, i_0464a2),
    (0x046518, i_046518),
    (0x04651C, i_04651c),
    (0x046520, i_046520),
    (0x046524, i_046524),
    (0x046528, i_046528),
    (0x04652C, i_04652c),
    (0x04652E, i_04652e),
    (0x046532, i_046532),
    (0x046534, i_046534),
    (0x046538, i_046538),
    (0x04653A, i_04653a),
    (0x04653C, i_04653c),
    (0x046540, i_046540),
    (0x046546, i_046546),
    (0x04654A, i_04654a),
    (0x046550, i_046550),
    (0x046554, i_046554),
    (0x046558, i_046558),
    (0x04655A, i_04655a),
    (0x04655E, i_04655e),
    (0x046562, i_046562),
    (0x046564, i_046564),
    (0x046566, i_046566),
    (0x04656C, i_04656c),
    (0x046570, i_046570),
    (0x046576, i_046576),
    (0x04657A, i_04657a),
    (0x04657C, i_04657c),
    (0x046580, i_046580),
    (0x046584, i_046584),
    (0x046588, i_046588),
    (0x04658C, i_04658c),
    (0x046590, i_046590),
    (0x046596, i_046596),
    (0x04659C, i_04659c),
    (0x0465A2, i_0465a2),
    (0x0465A8, i_0465a8),
    (0x0465AE, i_0465ae),
    (0x0465B2, i_0465b2),
    (0x0465B4, i_0465b4),
    (0x0465BA, i_0465ba),
    (0x0465C0, i_0465c0),
    (0x0465C6, i_0465c6),
    (0x0465CC, i_0465cc),
    (0x0465D2, i_0465d2),
    (0x0465D6, i_0465d6),
    (0x0465DA, i_0465da),
    (0x0465DC, i_0465dc),
    (0x0465DE, i_0465de),
    (0x0465E4, i_0465e4),
    (0x0465EC, i_0465ec),
    (0x0465F0, i_0465f0),
    (0x0465F4, i_0465f4),
    (0x0465F8, i_0465f8),
    (0x0465FC, i_0465fc),
    (0x046600, i_046600),
    (0x046602, i_046602),
    (0x046606, i_046606),
    (0x046608, i_046608),
    (0x04660C, i_04660c),
    (0x046610, i_046610),
    (0x046614, i_046614),
    (0x046618, i_046618),
    (0x04661E, i_04661e),
    (0x046626, i_046626),
    (0x04662C, i_04662c),
    (0x046630, i_046630),
    (0x046632, i_046632),
    (0x046636, i_046636),
    (0x04663C, i_04663c),
    (0x046640, i_046640),
    (0x046644, i_046644),
    (0x046648, i_046648),
    (0x04664C, i_04664c),
    (0x046650, i_046650),
    (0x046656, i_046656),
    (0x04665C, i_04665c),
    (0x046660, i_046660),
    (0x046662, i_046662),
    (0x046664, i_046664),
    (0x046668, i_046668),
    (0x04666E, i_04666e),
    (0x046672, i_046672),
    (0x046676, i_046676),
    (0x04667A, i_04667a),
    (0x046680, i_046680),
    (0x046682, i_046682),
    (0x046688, i_046688),
    (0x04668C, i_04668c),
    (0x046692, i_046692),
    (0x046694, i_046694),
    (0x046696, i_046696),
    (0x04669C, i_04669c),
    (0x0466A0, i_0466a0),
    (0x0466A6, i_0466a6),
    (0x0466AC, i_0466ac),
    (0x0466B0, i_0466b0),
    (0x0466B2, i_0466b2),
    (0x0466B4, i_0466b4),
    (0x0466B8, i_0466b8),
    (0x0466BE, i_0466be),
    (0x0466C2, i_0466c2),
    (0x0466C6, i_0466c6),
    (0x0466CC, i_0466cc),
    (0x0466D2, i_0466d2),
    (0x0466D6, i_0466d6),
    (0x0466D8, i_0466d8),
    (0x0466DA, i_0466da),
    (0x0466DE, i_0466de),
    (0x0466E4, i_0466e4),
    (0x0466E8, i_0466e8),
    (0x0466EE, i_0466ee),
    (0x0466F4, i_0466f4),
    (0x0466F6, i_0466f6),
    (0x0466FC, i_0466fc),
    (0x046700, i_046700),
    (0x046704, i_046704),
    (0x04670A, i_04670a),
    (0x046710, i_046710),
    (0x04673A, i_04673a),
    (0x046740, i_046740),
    (0x046746, i_046746),
    (0x04674A, i_04674a),
    (0x04674E, i_04674e),
    (0x046752, i_046752),
    (0x04675A, i_04675a),
    (0x04675C, i_04675c),
    (0x046760, i_046760),
    (0x046768, i_046768),
    (0x04676C, i_04676c),
    (0x04676E, i_04676e),
    (0x046772, i_046772),
    (0x046778, i_046778),
    (0x04677C, i_04677c),
    (0x046784, i_046784),
    (0x04678A, i_04678a),
    (0x04678E, i_04678e),
    (0x046792, i_046792),
    (0x046796, i_046796),
    (0x04679A, i_04679a),
    (0x04679E, i_04679e),
    (0x0467A2, i_0467a2),
    (0x0467A6, i_0467a6),
    (0x0467AC, i_0467ac),
    (0x0467B0, i_0467b0),
    (0x0467B4, i_0467b4),
    (0x0467BA, i_0467ba),
    (0x0467BE, i_0467be),
    (0x0467C2, i_0467c2),
    (0x0467C6, i_0467c6),
    (0x0467CC, i_0467cc),
    (0x0467D0, i_0467d0),
    (0x0467D4, i_0467d4),
    (0x0467D8, i_0467d8),
    (0x0467E0, i_0467e0),
    (0x0467E2, i_0467e2),
    (0x0467E6, i_0467e6),
    (0x0467EE, i_0467ee),
    (0x0467F4, i_0467f4),
    (0x0467F6, i_0467f6),
    (0x0467FC, i_0467fc),
    (0x046806, i_046806),
    (0x04680C, i_04680c),
    (0x046812, i_046812),
    (0x046816, i_046816),
    (0x04681C, i_04681c),
    (0x046820, i_046820),
    (0x046826, i_046826),
    (0x04682C, i_04682c),
    (0x046832, i_046832),
    (0x046838, i_046838),
    (0x04683E, i_04683e),
    (0x046842, i_046842),
    (0x046848, i_046848),
    (0x04684E, i_04684e),
    (0x046854, i_046854),
    (0x046858, i_046858),
    (0x04685E, i_04685e),
    (0x046862, i_046862),
    (0x046868, i_046868),
    (0x04686C, i_04686c),
    (0x04686E, i_04686e),
    (0x046872, i_046872),
    (0x046878, i_046878),
    (0x04687C, i_04687c),
    (0x046882, i_046882),
    (0x046886, i_046886),
    (0x04688A, i_04688a),
    (0x04688E, i_04688e),
    (0x046892, i_046892),
    (0x046898, i_046898),
    (0x04689C, i_04689c),
    (0x0468A0, i_0468a0),
    (0x0468A6, i_0468a6),
    (0x0468AA, i_0468aa),
    (0x0468AE, i_0468ae),
    (0x0468B4, i_0468b4),
    (0x0468B8, i_0468b8),
    (0x0468BC, i_0468bc),
    (0x0468C0, i_0468c0),
    (0x0468C4, i_0468c4),
    (0x0468C8, i_0468c8),
    (0x0468CE, i_0468ce),
    (0x0468D4, i_0468d4),
    (0x0468DA, i_0468da),
    (0x0468DE, i_0468de),
    (0x0468E0, i_0468e0),
    (0x0468E4, i_0468e4),
    (0x0468EA, i_0468ea),
    (0x0468F0, i_0468f0),
    (0x0468F4, i_0468f4),
    (0x0468F8, i_0468f8),
    (0x0468FE, i_0468fe),
    (0x046902, i_046902),
    (0x046908, i_046908),
    (0x04690C, i_04690c),
    (0x046910, i_046910),
    (0x046914, i_046914),
    (0x04691A, i_04691a),
    (0x04691E, i_04691e),
    (0x046922, i_046922),
    (0x046926, i_046926),
    (0x04692C, i_04692c),
    (0x04692E, i_04692e),
    (0x046932, i_046932),
    (0x046938, i_046938),
    (0x04693C, i_04693c),
    (0x046942, i_046942),
    (0x046948, i_046948),
    (0x04694A, i_04694a),
    (0x046952, i_046952),
    (0x046956, i_046956),
    (0x04695A, i_04695a),
    (0x04695C, i_04695c),
    (0x046962, i_046962),
    (0x046966, i_046966),
    (0x04696A, i_04696a),
    (0x04696C, i_04696c),
    (0x046972, i_046972),
    (0x046976, i_046976),
    (0x04697A, i_04697a),
    (0x04697E, i_04697e),
    (0x046980, i_046980),
    (0x046984, i_046984),
    (0x046988, i_046988),
    (0x04698A, i_04698a),
    (0x04698C, i_04698c),
    (0x046994, i_046994),
    (0x046998, i_046998),
    (0x04699C, i_04699c),
    (0x04699E, i_04699e),
    (0x0469A4, i_0469a4),
    (0x0469A8, i_0469a8),
    (0x0469AC, i_0469ac),
    (0x0469AE, i_0469ae),
    (0x0469B4, i_0469b4),
    (0x0469B8, i_0469b8),
    (0x0469BC, i_0469bc),
    (0x0469C0, i_0469c0),
    (0x0469C2, i_0469c2),
    (0x0469C6, i_0469c6),
    (0x0469CA, i_0469ca),
    (0x0469CC, i_0469cc),
    (0x0469CE, i_0469ce),
    (0x0469D2, i_0469d2),
    (0x0469D6, i_0469d6),
    (0x0469D8, i_0469d8),
    (0x0469DC, i_0469dc),
    (0x0469E0, i_0469e0),
    (0x0469E2, i_0469e2),
    (0x0469E6, i_0469e6),
    (0x0469EA, i_0469ea),
    (0x0469EE, i_0469ee),
    (0x0469F0, i_0469f0),
    (0x0469F6, i_0469f6),
    (0x0469FC, i_0469fc),
    (0x0469FE, i_0469fe),
    (0x046A04, i_046a04),
    (0x046A0A, i_046a0a),
    (0x046A0E, i_046a0e),
    (0x046A14, i_046a14),
    (0x046A1A, i_046a1a),
    (0x046A20, i_046a20),
    (0x046A24, i_046a24),
    (0x046A2A, i_046a2a),
    (0x046A2C, i_046a2c),
    (0x046A30, i_046a30),
    (0x046A34, i_046a34),
    (0x046A38, i_046a38),
    (0x046A3C, i_046a3c),
    (0x046A40, i_046a40),
    (0x046A42, i_046a42),
    (0x046A46, i_046a46),
    (0x046A48, i_046a48),
    (0x046A4E, i_046a4e),
    (0x046A52, i_046a52),
    (0x046A56, i_046a56),
    (0x046A5C, i_046a5c),
    (0x046A60, i_046a60),
    (0x046A64, i_046a64),
    (0x046A66, i_046a66),
    (0x046A6A, i_046a6a),
    (0x046A70, i_046a70),
    (0x046A74, i_046a74),
    (0x046A78, i_046a78),
    (0x046A7C, i_046a7c),
    (0x046A82, i_046a82),
    (0x046A86, i_046a86),
    (0x046A8A, i_046a8a),
    (0x046A8E, i_046a8e),
    (0x046A94, i_046a94),
    (0x046A96, i_046a96),
    (0x046A9A, i_046a9a),
    (0x046A9E, i_046a9e),
    (0x046AA2, i_046aa2),
    (0x046AA6, i_046aa6),
    (0x046AAC, i_046aac),
    (0x046AB0, i_046ab0),
    (0x046AB4, i_046ab4),
    (0x046AB8, i_046ab8),
    (0x046ABC, i_046abc),
    (0x046AC2, i_046ac2),
    (0x046AC6, i_046ac6),
    (0x046ACA, i_046aca),
    (0x046ACE, i_046ace),
    (0x046AD2, i_046ad2),
    (0x046AD6, i_046ad6),
    (0x046ADC, i_046adc),
    (0x046AE0, i_046ae0),
    (0x046AE4, i_046ae4),
    (0x046AE8, i_046ae8),
    (0x046AEC, i_046aec),
    (0x046AF2, i_046af2),
    (0x046AF6, i_046af6),
    (0x046AFA, i_046afa),
    (0x046AFE, i_046afe),
    (0x046B02, i_046b02),
    (0x046B08, i_046b08),
    (0x046B0C, i_046b0c),
    (0x046B10, i_046b10),
    (0x046B14, i_046b14),
    (0x046B18, i_046b18),
    (0x046B1E, i_046b1e),
    (0x046B20, i_046b20),
    (0x046B24, i_046b24),
    (0x046B26, i_046b26),
    (0x046B28, i_046b28),
    (0x046B2A, i_046b2a),
    (0x046B2E, i_046b2e),
    (0x046B32, i_046b32),
    (0x046B36, i_046b36),
    (0x046B3A, i_046b3a),
    (0x046B3E, i_046b3e),
    (0x046B42, i_046b42),
    (0x046B46, i_046b46),
    (0x046B4A, i_046b4a),
    (0x046B4E, i_046b4e),
    (0x046B52, i_046b52),
    (0x046B56, i_046b56),
    (0x046B5A, i_046b5a),
    (0x046B5E, i_046b5e),
    (0x046B62, i_046b62),
    (0x046B66, i_046b66),
    (0x046B6E, i_046b6e),
    (0x046B70, i_046b70),
    (0x046B78, i_046b78),
    (0x046B7A, i_046b7a),
    (0x046B82, i_046b82),
    (0x046B84, i_046b84),
    (0x046B8C, i_046b8c),
    (0x046B90, i_046b90),
    (0x046B98, i_046b98),
    (0x046B9A, i_046b9a),
    (0x046BA2, i_046ba2),
    (0x046BA4, i_046ba4),
    (0x046BAC, i_046bac),
    (0x046BAE, i_046bae),
    (0x046BB6, i_046bb6),
    (0x046BBA, i_046bba),
    (0x046BBE, i_046bbe),
    (0x046BC0, i_046bc0),
    (0x046BC4, i_046bc4),
    (0x046BC6, i_046bc6),
    (0x046BC8, i_046bc8),
    (0x046BCC, i_046bcc),
    (0x046BD0, i_046bd0),
    (0x046BD4, i_046bd4),
    (0x046BD8, i_046bd8),
    (0x046BDA, i_046bda),
    (0x046BDE, i_046bde),
    (0x046BE2, i_046be2),
    (0x046BE6, i_046be6),
    (0x046BE8, i_046be8),
    (0x046BEC, i_046bec),
    (0x046BF4, i_046bf4),
    (0x046BF6, i_046bf6),
    (0x046BFA, i_046bfa),
    (0x046C02, i_046c02),
    (0x046C04, i_046c04),
    (0x046C08, i_046c08),
    (0x046C10, i_046c10),
    (0x046C12, i_046c12),
    (0x046C16, i_046c16),
    (0x046C1E, i_046c1e),
    (0x046C22, i_046c22),
    (0x046C26, i_046c26),
    (0x046C28, i_046c28),
    (0x046C2C, i_046c2c),
    (0x046C2E, i_046c2e),
    (0x046C32, i_046c32),
    (0x046C34, i_046c34),
    (0x046C36, i_046c36),
    (0x046C3A, i_046c3a),
    (0x046C3E, i_046c3e),
    (0x046C42, i_046c42),
    (0x046C46, i_046c46),
    (0x046C48, i_046c48),
    (0x046C4C, i_046c4c),
    (0x046C50, i_046c50),
    (0x046C54, i_046c54),
    (0x046C56, i_046c56),
    (0x046C5A, i_046c5a),
    (0x046C62, i_046c62),
    (0x046C64, i_046c64),
    (0x046C68, i_046c68),
    (0x046C70, i_046c70),
    (0x046C72, i_046c72),
    (0x046C76, i_046c76),
    (0x046C7E, i_046c7e),
    (0x046C80, i_046c80),
    (0x046C84, i_046c84),
    (0x046C8C, i_046c8c),
    (0x046C90, i_046c90),
    (0x046C94, i_046c94),
    (0x046C96, i_046c96),
    (0x046C9C, i_046c9c),
    (0x046C9E, i_046c9e),
    (0x046CA2, i_046ca2),
    (0x046CA4, i_046ca4),
    (0x046CA8, i_046ca8),
    (0x046CAA, i_046caa),
    (0x046CAC, i_046cac),
    (0x046CB0, i_046cb0),
    (0x046CB4, i_046cb4),
    (0x046CB8, i_046cb8),
    (0x046CBC, i_046cbc),
    (0x046CBE, i_046cbe),
    (0x046CC2, i_046cc2),
    (0x046CC6, i_046cc6),
    (0x046CCA, i_046cca),
    (0x046CCC, i_046ccc),
    (0x046CD0, i_046cd0),
    (0x046CD8, i_046cd8),
    (0x046CDA, i_046cda),
    (0x046CDE, i_046cde),
    (0x046CE6, i_046ce6),
    (0x046CE8, i_046ce8),
    (0x046CEC, i_046cec),
    (0x046CF4, i_046cf4),
    (0x046CF6, i_046cf6),
    (0x046CFA, i_046cfa),
    (0x046D02, i_046d02),
    (0x046D06, i_046d06),
    (0x046D0A, i_046d0a),
    (0x046D0C, i_046d0c),
    (0x046D0E, i_046d0e),
    (0x046D12, i_046d12),
    (0x046D14, i_046d14),
    (0x046D16, i_046d16),
    (0x046D18, i_046d18),
    (0x046D1C, i_046d1c),
    (0x046D20, i_046d20),
    (0x046D24, i_046d24),
    (0x046D28, i_046d28),
    (0x046D2C, i_046d2c),
    (0x046D30, i_046d30),
    (0x046D32, i_046d32),
    (0x046D36, i_046d36),
    (0x046D38, i_046d38),
    (0x046D3A, i_046d3a),
    (0x046D3E, i_046d3e),
    (0x046D42, i_046d42),
    (0x046D46, i_046d46),
    (0x046D4A, i_046d4a),
    (0x046D4C, i_046d4c),
    (0x046D50, i_046d50),
    (0x046D54, i_046d54),
    (0x046D58, i_046d58),
    (0x046D5A, i_046d5a),
    (0x046D5E, i_046d5e),
    (0x046D66, i_046d66),
    (0x046D68, i_046d68),
    (0x046D6C, i_046d6c),
    (0x046D74, i_046d74),
    (0x046D76, i_046d76),
    (0x046D7A, i_046d7a),
    (0x046D82, i_046d82),
    (0x046D84, i_046d84),
    (0x046D88, i_046d88),
    (0x046D90, i_046d90),
    (0x046D94, i_046d94),
    (0x046D98, i_046d98),
    (0x046D9A, i_046d9a),
    (0x046D9E, i_046d9e),
    (0x046DA0, i_046da0),
    (0x046DA4, i_046da4),
    (0x046DA6, i_046da6),
    (0x046DA8, i_046da8),
    (0x046DAC, i_046dac),
    (0x046DB0, i_046db0),
    (0x046DB4, i_046db4),
    (0x046DB8, i_046db8),
    (0x046DBA, i_046dba),
    (0x046DBE, i_046dbe),
    (0x046DC2, i_046dc2),
    (0x046DC6, i_046dc6),
    (0x046DC8, i_046dc8),
    (0x046DCC, i_046dcc),
    (0x046DD4, i_046dd4),
    (0x046DD6, i_046dd6),
    (0x046DDA, i_046dda),
    (0x046DE2, i_046de2),
    (0x046DE4, i_046de4),
    (0x046DE8, i_046de8),
    (0x046DF0, i_046df0),
    (0x046DF2, i_046df2),
    (0x046DF6, i_046df6),
    (0x046DFE, i_046dfe),
    (0x046E02, i_046e02),
    (0x046E06, i_046e06),
    (0x046E08, i_046e08),
    (0x046E0C, i_046e0c),
    (0x046E10, i_046e10),
    (0x046E12, i_046e12),
    (0x046E16, i_046e16),
    (0x046E1A, i_046e1a),
    (0x046E1C, i_046e1c),
    (0x046E20, i_046e20),
    (0x046E28, i_046e28),
    (0x046E2A, i_046e2a),
    (0x046E32, i_046e32),
    (0x046E34, i_046e34),
    (0x046E3C, i_046e3c),
    (0x046E3E, i_046e3e),
    (0x046E46, i_046e46),
    (0x046E4A, i_046e4a),
    (0x046E52, i_046e52),
    (0x046E54, i_046e54),
    (0x046E5C, i_046e5c),
    (0x046E5E, i_046e5e),
    (0x046E66, i_046e66),
    (0x046E68, i_046e68),
    (0x046E70, i_046e70),
    (0x046E72, i_046e72),
    (0x046E76, i_046e76),
    (0x046E78, i_046e78),
    (0x046E7A, i_046e7a),
    (0x046E7C, i_046e7c),
    (0x046E80, i_046e80),
    (0x046E84, i_046e84),
    (0x046E88, i_046e88),
    (0x046E8C, i_046e8c),
    (0x046E90, i_046e90),
    (0x046E94, i_046e94),
    (0x046E96, i_046e96),
    (0x046E9A, i_046e9a),
    (0x046E9C, i_046e9c),
    (0x046E9E, i_046e9e),
    (0x046EA2, i_046ea2),
    (0x046EA6, i_046ea6),
    (0x046EAA, i_046eaa),
    (0x046EAE, i_046eae),
    (0x046EB0, i_046eb0),
    (0x046EB4, i_046eb4),
    (0x046EB8, i_046eb8),
    (0x046EBC, i_046ebc),
    (0x046EBE, i_046ebe),
    (0x046EC2, i_046ec2),
    (0x046ECA, i_046eca),
    (0x046ECC, i_046ecc),
    (0x046ED0, i_046ed0),
    (0x046ED8, i_046ed8),
    (0x046EDA, i_046eda),
    (0x046EDE, i_046ede),
    (0x046EE6, i_046ee6),
    (0x046EE8, i_046ee8),
    (0x046EEC, i_046eec),
    (0x046EF4, i_046ef4),
    (0x046EF8, i_046ef8),
    (0x046EFC, i_046efc),
    (0x046EFE, i_046efe),
    (0x046F02, i_046f02),
    (0x046F04, i_046f04),
    (0x046F08, i_046f08),
    (0x046F0A, i_046f0a),
    (0x046F0C, i_046f0c),
    (0x046F10, i_046f10),
    (0x046F14, i_046f14),
    (0x046F18, i_046f18),
    (0x046F1C, i_046f1c),
    (0x046F1E, i_046f1e),
    (0x046F22, i_046f22),
    (0x046F26, i_046f26),
    (0x046F2A, i_046f2a),
    (0x046F2C, i_046f2c),
    (0x046F30, i_046f30),
    (0x046F38, i_046f38),
    (0x046F3A, i_046f3a),
    (0x046F3E, i_046f3e),
    (0x046F46, i_046f46),
    (0x046F48, i_046f48),
    (0x046F4C, i_046f4c),
    (0x046F54, i_046f54),
    (0x046F56, i_046f56),
    (0x046F5A, i_046f5a),
    (0x046F62, i_046f62),
    (0x046F66, i_046f66),
    (0x046F6A, i_046f6a),
    (0x046F6C, i_046f6c),
    (0x046F70, i_046f70),
    (0x046F74, i_046f74),
    (0x046F76, i_046f76),
    (0x046F7A, i_046f7a),
    (0x046F7E, i_046f7e),
    (0x046F80, i_046f80),
    (0x046F84, i_046f84),
    (0x046F8C, i_046f8c),
    (0x046F8E, i_046f8e),
    (0x046F96, i_046f96),
    (0x046F98, i_046f98),
    (0x046FA0, i_046fa0),
    (0x046FA2, i_046fa2),
    (0x046FAA, i_046faa),
    (0x046FAE, i_046fae),
    (0x046FB6, i_046fb6),
    (0x046FB8, i_046fb8),
    (0x046FC0, i_046fc0),
    (0x046FC2, i_046fc2),
    (0x046FCA, i_046fca),
    (0x046FCC, i_046fcc),
    (0x046FD4, i_046fd4),
    (0x046FD6, i_046fd6),
    (0x046FD8, i_046fd8),
    (0x046FDE, i_046fde),
    (0x046FE2, i_046fe2),
    (0x046FE6, i_046fe6),
    (0x046FEA, i_046fea),
    (0x046FEE, i_046fee),
    (0x046FF4, i_046ff4),
    (0x046FFA, i_046ffa),
    (0x047000, i_047000),
    (0x047004, i_047004),
    (0x047006, i_047006),
    (0x04700A, i_04700a),
    (0x04700E, i_04700e),
    (0x047012, i_047012),
    (0x047016, i_047016),
    (0x04701A, i_04701a),
    (0x04701E, i_04701e),
    (0x047022, i_047022),
    (0x047028, i_047028),
    (0x04702C, i_04702c),
    (0x047032, i_047032),
    (0x047036, i_047036),
    (0x047038, i_047038),
    (0x04703A, i_04703a),
    (0x04703E, i_04703e),
    (0x047044, i_047044),
    (0x047048, i_047048),
    (0x04704C, i_04704c),
    (0x04704E, i_04704e),
    (0x047050, i_047050),
    (0x047054, i_047054),
    (0x047058, i_047058),
    (0x04705C, i_04705c),
    (0x047060, i_047060),
    (0x047064, i_047064),
    (0x047068, i_047068),
    (0x04706C, i_04706c),
    (0x047072, i_047072),
    (0x047076, i_047076),
    (0x04707C, i_04707c),
    (0x04707E, i_04707e),
    (0x047080, i_047080),
    (0x047086, i_047086),
    (0x04708A, i_04708a),
    (0x04708C, i_04708c),
    (0x047092, i_047092),
    (0x047096, i_047096),
    (0x04709C, i_04709c),
    (0x04709E, i_04709e),
    (0x0470A0, i_0470a0),
    (0x0470A6, i_0470a6),
    (0x0470AA, i_0470aa),
    (0x0470AC, i_0470ac),
    (0x0470B2, i_0470b2),
    (0x0470B6, i_0470b6),
    (0x0470BC, i_0470bc),
    (0x0470BE, i_0470be),
    (0x0470C0, i_0470c0),
    (0x0470C4, i_0470c4),
    (0x0470C8, i_0470c8),
    (0x0470CE, i_0470ce),
    (0x0470D4, i_0470d4),
    (0x0470D8, i_0470d8),
    (0x0470DE, i_0470de),
    (0x0470E4, i_0470e4),
    (0x0470EA, i_0470ea),
    (0x0470EE, i_0470ee),
    (0x0470F0, i_0470f0),
    (0x0470F4, i_0470f4),
    (0x0470F8, i_0470f8),
    (0x0470FC, i_0470fc),
    (0x047100, i_047100),
    (0x047104, i_047104),
    (0x047108, i_047108),
    (0x04710E, i_04710e),
];
