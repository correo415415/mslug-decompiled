// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $06873C: bset.b #$6, $12(a6)
pub fn i_06873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68742;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $068742: lea.l $2c84fa.l, a0
pub fn i_068742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68748;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C84FA;
    20
}
/// $068748: jsr $28cd4.l
pub fn i_068748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6874E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6874E);
    cpu.pc = 0x28CD4;
    16
}
/// $06874E: lea.l $68754(pc), a1
pub fn i_06874e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68752;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68754;
    16
}
/// $068752: move.l a1, (a6)
pub fn i_068752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68754;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068754: jsr $68a70(pc)
pub fn i_068754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68758;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68758);
    cpu.pc = 0x68A70;
    16
}
// ===== JsrPcThunk_068758  ($068758, 6 B, thunks/jsr_pc_thunks.c)
/// $068758: jsr $68ab8(pc)
pub fn i_068758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6875C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6875C);
    cpu.pc = 0x68AB8;
    16
}
/// $06875C: rts
pub fn i_06875c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6875E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_Nose_06875e  ($06875E, 80 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06875E: clr.w $82(a6)
pub fn i_06875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68762;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $068762: clr.w $84(a6)
pub fn i_068762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68766;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $068766: move.w #$1b7, d1
pub fn i_068766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6876A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06876A: jsr $236e.l
pub fn i_06876a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68770);
    cpu.pc = 0x236E;
    16
}
/// $068770: bset.b #$6, $12(a6)
pub fn i_068770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68776;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $068776: lea.l $2c84ee.l, a0
pub fn i_068776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6877C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C84EE;
    20
}
/// $06877C: jsr $28cd4.l
pub fn i_06877c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68782);
    cpu.pc = 0x28CD4;
    16
}
/// $068782: lea.l $68788(pc), a1
pub fn i_068782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68786;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68788;
    16
}
/// $068786: move.l a1, (a6)
pub fn i_068786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68788;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068788: cmpi.w #$180, $22(a6)
pub fn i_068788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6878E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $06878E: bgt.w $687aa
pub fn i_06878e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68792;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x687AA; return 10; }
    8
}
/// $068792: jsr $2783a.l
pub fn i_068792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68798);
    cpu.pc = 0x2783A;
    16
}
/// $068798: lea.l $2c84c2.l, a1
pub fn i_068798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6879E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C84C2;
    20
}
/// $06879E: jsr $43fac.l
pub fn i_06879e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x687A4);
    cpu.pc = 0x43FAC;
    16
}
/// $0687A4: lea.l $687aa(pc), a1
pub fn i_0687a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x687AA;
    16
}
/// $0687A8: move.l a1, (a6)
pub fn i_0687a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0687AA: jsr $68a70(pc)
pub fn i_0687aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x687AE);
    cpu.pc = 0x68A70;
    16
}
// ===== JsrPcThunk_0687ae  ($0687AE, 6 B, thunks/jsr_pc_thunks.c)
/// $0687AE: jsr $68ab8(pc)
pub fn i_0687ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x687B2);
    cpu.pc = 0x68AB8;
    16
}
/// $0687B2: rts
pub fn i_0687b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_Hatch_0687b4  ($0687B4, 20 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0687B4: clr.w $86(a6)
pub fn i_0687b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687B8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0687B8: move.w #$ffff, $84(a6)
pub fn i_0687b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0687BE: move.w #$1b7, d1
pub fn i_0687be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687C2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0687C2: jsr $236e.l
pub fn i_0687c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x687C8);
    cpu.pc = 0x236E;
    16
}
// ===== S5Airship_HatchClosed_0687c8  ($0687C8, 60 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0687C8: move.w #$8, $82(a6)
pub fn i_0687c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0687CE: lea.l $2c85e0.l, a0
pub fn i_0687ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C85E0;
    20
}
/// $0687D4: jsr $28cd4.l
pub fn i_0687d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x687DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0687DA: lea.l $687e0(pc), a1
pub fn i_0687da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x687E0;
    16
}
/// $0687DE: move.l a1, (a6)
pub fn i_0687de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0687E0: jsr $68a70(pc)
pub fn i_0687e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x687E4);
    cpu.pc = 0x68A70;
    16
}
/// $0687E4: jsr $68b14(pc)
pub fn i_0687e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x687E8);
    cpu.pc = 0x68B14;
    16
}
/// $0687E8: jsr $68ab8(pc)
pub fn i_0687e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x687EC);
    cpu.pc = 0x68AB8;
    16
}
/// $0687EC: movea.l $c(a6), a0
pub fn i_0687ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687F0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0687F0: cmpi.b #$1, $20(a0)
pub fn i_0687f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687F6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0687F6: bne.w $68800
pub fn i_0687f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68800; return 10; }
    8
}
/// $0687FA: lea.l $68804(pc), a1
pub fn i_0687fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x687FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68804;
    16
}
/// $0687FE: move.l a1, (a6)
pub fn i_0687fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68800;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068800: bra.w $6894a
pub fn i_068800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68804;
    cpu.ir = 0x6000;
    cpu.pc = 0x6894A;
    10
}
// ===== S5Airship_HatchOpen_068804  ($068804, 64 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068804: move.w #$10a8, d0
pub fn i_068804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68808;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068808: jsr $2352.l
pub fn i_068808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6880E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6880E);
    cpu.pc = 0x2352;
    16
}
/// $06880E: lea.l $2c85e0.l, a0
pub fn i_06880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68814;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C85E0;
    20
}
/// $068814: jsr $28cd4.l
pub fn i_068814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6881A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6881A);
    cpu.pc = 0x28CD4;
    16
}
/// $06881A: lea.l $68820(pc), a1
pub fn i_06881a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6881E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68820;
    16
}
/// $06881E: move.l a1, (a6)
pub fn i_06881e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068820: jsr $68a70(pc)
pub fn i_068820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68824;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68824);
    cpu.pc = 0x68A70;
    16
}
/// $068824: jsr $68b14(pc)
pub fn i_068824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68828;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68828);
    cpu.pc = 0x68B14;
    16
}
/// $068828: jsr $68ab8(pc)
pub fn i_068828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6882C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6882C);
    cpu.pc = 0x68AB8;
    16
}
/// $06882C: addq.w #$1, $82(a6)
pub fn i_06882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68830;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068830: cmpi.w #$28, $82(a6)
pub fn i_068830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68836;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x28, Size::Word, false);
    16
}
/// $068836: bne.w $68840
pub fn i_068836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6883A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68840; return 10; }
    8
}
/// $06883A: lea.l $68844(pc), a1
pub fn i_06883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6883E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68844;
    16
}
/// $06883E: move.l a1, (a6)
pub fn i_06883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68840;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068840: bra.w $6894a
pub fn i_068840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68844;
    cpu.ir = 0x6000;
    cpu.pc = 0x6894A;
    10
}
// ===== S5Airship_HatchAnim_068844  ($068844, 96 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068844: movea.l $c(a6), a0
pub fn i_068844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68848;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $068848: move.b #$2, $20(a0)
pub fn i_068848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6884E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06884E: subq.w #$1, $82(a6)
pub fn i_06884e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68852;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068852: move.b $3b(a6), d0
pub fn i_068852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68856;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $068856: andi.w #$f, d0
pub fn i_068856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6885A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06885A: move.w d0, $86(a6)
pub fn i_06885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6885E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06885E: movea.l #$2c8626, a0
pub fn i_06885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68864;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C8626;
    cpu.a[0] = v1;
    4
}
/// $068864: lsl.w #$2, d0
pub fn i_068864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68866;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $068866: movea.l (a0, d0.w), a0
pub fn i_068866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6886A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06886A: cmpa.l #$ffffffff, a0
pub fn i_06886a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68870;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $068870: beq.w $6887a
pub fn i_068870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68874;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6887A; return 10; }
    8
}
/// $068874: jsr $28cd4.l
pub fn i_068874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6887A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6887A);
    cpu.pc = 0x28CD4;
    16
}
/// $06887A: lea.l $68880(pc), a1
pub fn i_06887a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6887E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68880;
    16
}
/// $06887E: move.l a1, (a6)
pub fn i_06887e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68880;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068880: jsr $68a70(pc)
pub fn i_068880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68884;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68884);
    cpu.pc = 0x68A70;
    16
}
/// $068884: jsr $68b14(pc)
pub fn i_068884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68888;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68888);
    cpu.pc = 0x68B14;
    16
}
/// $068888: jsr $68ab8(pc)
pub fn i_068888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6888C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6888C);
    cpu.pc = 0x68AB8;
    16
}
/// $06888C: movea.l $c(a6), a0
pub fn i_06888c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68890;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $068890: cmpi.b #$3, $20(a0)
pub fn i_068890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68896;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $068896: bne.w $688a0
pub fn i_068896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6889A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x688A0; return 10; }
    8
}
/// $06889A: lea.l $688a4(pc), a1
pub fn i_06889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6889E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x688A4;
    16
}
/// $06889E: move.l a1, (a6)
pub fn i_06889e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0688A0: bra.w $6894a
pub fn i_0688a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x6894A;
    10
}
// ===== S5Airship_HatchClose_0688a4  ($0688A4, 84 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0688A4: move.w $86(a6), d0
pub fn i_0688a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688A8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0688A8: movea.l #$2c8686, a0
pub fn i_0688a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688AE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C8686;
    cpu.a[0] = v1;
    4
}
/// $0688AE: lsl.w #$2, d0
pub fn i_0688ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688B0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0688B0: movea.l (a0, d0.w), a0
pub fn i_0688b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688B4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0688B4: cmpa.l #$ffffffff, a0
pub fn i_0688b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688BA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0688BA: beq.w $688c4
pub fn i_0688ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x688C4; return 10; }
    8
}
/// $0688BE: jsr $28cd4.l
pub fn i_0688be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x688C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0688C4: lea.l $688ca(pc), a1
pub fn i_0688c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x688CA;
    16
}
/// $0688C8: move.l a1, (a6)
pub fn i_0688c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0688CA: jsr $68a70(pc)
pub fn i_0688ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x688CE);
    cpu.pc = 0x68A70;
    16
}
/// $0688CE: jsr $68b14(pc)
pub fn i_0688ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x688D2);
    cpu.pc = 0x68B14;
    16
}
/// $0688D2: jsr $68ab8(pc)
pub fn i_0688d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x688D6);
    cpu.pc = 0x68AB8;
    16
}
/// $0688D6: subq.w #$1, $82(a6)
pub fn i_0688d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688DA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0688DA: cmpi.w #$8, $82(a6)
pub fn i_0688da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688E0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $0688E0: bne.w $688f4
pub fn i_0688e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x688F4; return 10; }
    8
}
/// $0688E4: movea.l $c(a6), a0
pub fn i_0688e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0688E8: move.b #$4, $20(a0)
pub fn i_0688e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688EE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0688EE: lea.l $687c8(pc), a1
pub fn i_0688ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x687C8;
    16
}
/// $0688F2: move.l a1, (a6)
pub fn i_0688f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0688F4: bra.w $6894a
pub fn i_0688f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6894A;
    10
}
// ===== S5Airship_HatchFinal_0688f8  ($0688F8, 74 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0688F8: move.w $86(a6), d0
pub fn i_0688f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x688FC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0688FC: movea.l #$2c8686, a0
pub fn i_0688fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68902;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C8686;
    cpu.a[0] = v1;
    4
}
/// $068902: lsl.w #$2, d0
pub fn i_068902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68904;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $068904: movea.l (a0, d0.w), a0
pub fn i_068904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68908;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $068908: cmpa.l #$ffffffff, a0
pub fn i_068908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6890E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06890E: beq.w $68918
pub fn i_06890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68912;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68918; return 10; }
    8
}
/// $068912: jsr $28cd4.l
pub fn i_068912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68918);
    cpu.pc = 0x28CD4;
    16
}
/// $068918: lea.l $6891e(pc), a1
pub fn i_068918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6891C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6891E;
    16
}
/// $06891C: move.l a1, (a6)
pub fn i_06891c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6891E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06891E: jsr $68a70(pc)
pub fn i_06891e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68922;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68922);
    cpu.pc = 0x68A70;
    16
}
/// $068922: cmpi.w #$0, $82(a6)
pub fn i_068922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68928;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $068928: blt.w $68930
pub fn i_068928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6892C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x68930; return 10; }
    8
}
/// $06892C: jsr $68b14(pc)
pub fn i_06892c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68930;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68930);
    cpu.pc = 0x68B14;
    16
}
/// $068930: jsr $68ab8(pc)
pub fn i_068930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68934;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68934);
    cpu.pc = 0x68AB8;
    16
}
/// $068934: subq.w #$1, $82(a6)
pub fn i_068934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68938;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068938: cmpi.w #$fff8, $82(a6)
pub fn i_068938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6893E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFF8, Size::Word, false);
    16
}
/// $06893E: bgt.w $68948
pub fn i_06893e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68942;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68948; return 10; }
    8
}
// ===== SetTaskHandler_068942  ($068942, 8 B, kernel/task_handlers.c)
/// $068942: lea.l $6895c(pc), a1
pub fn i_068942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6895C;
    16
}
/// $068946: move.l a1, (a6)
pub fn i_068946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68948;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068948: rts
pub fn i_068948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6894A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_SealCheck_06894a  ($06894A, 10 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06894A: tst.b $106ed3.l
pub fn i_06894a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68950;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $068950: bne.w $6895a
pub fn i_068950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68954;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6895A; return 10; }
    8
}
// ===== SetTaskHandler_068954  ($068954, 8 B, kernel/task_handlers.c)
/// $068954: lea.l $688f8(pc), a1
pub fn i_068954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68958;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x688F8;
    16
}
/// $068958: move.l a1, (a6)
pub fn i_068958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6895A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06895A: rts
pub fn i_06895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6895C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_06895c  ($06895C, 14 B, kernel/jsr5b6_jmp518.c)
/// $06895C: jsr $5b6.l
pub fn i_06895c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68962);
    cpu.pc = 0x5B6;
    16
}
/// $068962: jmp $518.l
pub fn i_068962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68968;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $068968: rts
pub fn i_068968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6896A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camera0_RelinkAndWrapScroll_06896A  ($06896A, 112 B, video/camera_relink_wrap_scroll_06896a.s)
/// $06896A: move.w $72(a6), d0
pub fn i_06896a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6896E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06896E: subq.w #$8, d0
pub fn i_06896e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68970;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $068970: cmp.w $106f50.l, d0
pub fn i_068970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68976;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $068976: bgt.w $689e0
pub fn i_068976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6897A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x689E0; return 10; }
    8
}
/// $06897A: lea.l $106f6c.l, a0
pub fn i_06897a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68980;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $068980: lea.l $2c84ba.l, a1
pub fn i_068980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68986;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C84BA;
    20
}
/// $068986: lea.l $20a000.l, a2
pub fn i_068986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6898C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x20A000;
    20
}
/// $06898C: move.l a2, $78(a0)
pub fn i_06898c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68990;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068990: move.w $106f50.l, d0
pub fn i_068990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68996;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $068996: sub.w $72(a6), d0
pub fn i_068996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6899A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06899A: addi.w #$fec0, d0
pub fn i_06899a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6899E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFEC0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06899E: move.w $106f54.l, d1
pub fn i_06899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689A4;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F54) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0689A4: addi.w #$30, d1
pub fn i_0689a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689A8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0689A8: addi.w #$ffc0, d1
pub fn i_0689a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689AC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFC0, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0689AC: jsr $51b3e.l
pub fn i_0689ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x689B2);
    cpu.pc = 0x51B3E;
    16
}
/// $0689B2: lea.l $106f6c.l, a0
pub fn i_0689b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $0689B8: jsr $43df4.l
pub fn i_0689b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x689BE);
    cpu.pc = 0x43DF4;
    16
}
/// $0689BE: lea.l $106f6c.l, a0
pub fn i_0689be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $0689C4: move.b #$f, d0
pub fn i_0689c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689C8;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0689C8: move.b #$0, d1
pub fn i_0689c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689CC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0689CC: jsr $51b1c.l
pub fn i_0689cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x689D2);
    cpu.pc = 0x51B1C;
    16
}
/// $0689D2: subi.w #$140, $106f50.l
pub fn i_0689d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689DA;
    cpu.ir = 0x479;
    let v1 = bus.read16(0x106F50) as u32;
    let r2 = sub_with_flags(cpu, v1, 0x140, Size::Word, true);
    bus.write16(0x106F50, r2 as u16);
    20
}
// ===== SetXN_0689da  ($0689DA, 6 B, thunks/ccr_helpers.c)
/// $0689DA: ori.b #$11, ccr
pub fn i_0689da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689DE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0689DE: rts
pub fn i_0689de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0689e0  ($0689E0, 6 B, thunks/ccr_helpers.c)
/// $0689E0: andi.b #$ee, ccr
pub fn i_0689e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689E4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0689E4: rts
pub fn i_0689e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_CameraHook_0689e6  ($0689E6, 104 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0689E6: move.w $72(a6), d0
pub fn i_0689e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689EA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0689EA: subq.w #$8, d0
pub fn i_0689ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689EC;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0689EC: cmp.w $106f50.l, d0
pub fn i_0689ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689F2;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $0689F2: bgt.w $68a54
pub fn i_0689f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689F6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68A54; return 10; }
    8
}
/// $0689F6: lea.l $106f6c.l, a0
pub fn i_0689f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x689FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $0689FC: lea.l $2c84ba.l, a1
pub fn i_0689fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A02;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C84BA;
    20
}
/// $068A02: lea.l $20a000.l, a2
pub fn i_068a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A08;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x20A000;
    20
}
/// $068A08: move.l a2, $78(a0)
pub fn i_068a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A0C;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068A0C: move.w $106f50.l, d0
pub fn i_068a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A12;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $068A12: sub.w $72(a6), d0
pub fn i_068a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A16;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $068A16: addi.w #$fec0, d0
pub fn i_068a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A1A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFEC0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068A1A: move.w $106f54.l, d1
pub fn i_068a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A20;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F54) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $068A20: addi.w #$30, d1
pub fn i_068a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A24;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068A24: addi.w #$ffc0, d1
pub fn i_068a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A28;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFC0, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068A28: jsr $51b3e.l
pub fn i_068a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68A2E);
    cpu.pc = 0x51B3E;
    16
}
/// $068A2E: lea.l $106f6c.l, a0
pub fn i_068a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $068A34: jsr $43df4.l
pub fn i_068a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68A3A);
    cpu.pc = 0x43DF4;
    16
}
/// $068A3A: lea.l $106f6c.l, a0
pub fn i_068a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $068A40: move.b #$f, d0
pub fn i_068a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A44;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $068A44: move.b #$0, d1
pub fn i_068a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A48;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $068A48: jsr $51b1c.l
pub fn i_068a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68A4E);
    cpu.pc = 0x51B1C;
    16
}
// ===== SetXN_068a4e  ($068A4E, 6 B, thunks/ccr_helpers.c)
/// $068A4E: ori.b #$11, ccr
pub fn i_068a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A52;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $068A52: rts
pub fn i_068a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_068a54  ($068A54, 6 B, thunks/ccr_helpers.c)
/// $068A54: andi.b #$ee, ccr
pub fn i_068a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A58;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068A58: rts
pub fn i_068a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_ActiveFlag_068a5a  ($068A5A, 14 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068A5A: jsr $68af4(pc)
pub fn i_068a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68A5E);
    cpu.pc = 0x68AF4;
    16
}
/// $068A5E: bcc.w $68a68
pub fn i_068a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68A68; return 10; }
    8
}
/// $068A62: bset.b #$6, $13(a6)
pub fn i_068a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A68;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrAbsThunk_068a68  ($068A68, 8 B, thunks/jsr_abs_thunks.c)
/// $068A68: jsr $2783a.l
pub fn i_068a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68A6E);
    cpu.pc = 0x2783A;
    16
}
/// $068A6E: rts
pub fn i_068a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_FollowParent_068a70  ($068A70, 32 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068A70: jsr $5e506.l
pub fn i_068a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68A76);
    cpu.pc = 0x5E506;
    16
}
/// $068A76: move.w $80(a6), d0
pub fn i_068a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A7A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068A7A: add.w d0, $22(a6)
pub fn i_068a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A7E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068A7E: move.w $82(a6), d0
pub fn i_068a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A82;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068A82: add.w d0, $24(a6)
pub fn i_068a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A86;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068A86: move.w $84(a6), d0
pub fn i_068a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A8A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068A8A: add.w d0, $38(a6)
pub fn i_068a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A8E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068A8E: rts
pub fn i_068a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_FollowGrandparent_068a90  ($068A90, 40 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068A90: jsr $5e506.l
pub fn i_068a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68A96);
    cpu.pc = 0x5E506;
    16
}
/// $068A96: move.w $80(a6), d0
pub fn i_068a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A9A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068A9A: add.w d0, $22(a6)
pub fn i_068a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A9E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068A9E: movea.l $c(a0), a0
pub fn i_068a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AA2;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $068AA2: move.w $24(a0), d0
pub fn i_068aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AA6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068AA6: add.w $82(a6), d0
pub fn i_068aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AAA;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $068AAA: move.w d0, $24(a6)
pub fn i_068aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AAE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068AAE: move.w $84(a6), d0
pub fn i_068aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AB2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068AB2: add.w d0, $38(a6)
pub fn i_068ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AB6;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068AB6: rts
pub fn i_068ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_PastRightEdge_068ab8  ($068AB8, 10 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068AB8: cmpi.w #$1a0, $22(a6)
pub fn i_068ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68ABE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1A0, Size::Word, false);
    16
}
/// $068ABE: bgt.w $68aca
pub fn i_068abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AC2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68ACA; return 10; }
    8
}
// ===== JsrAbsThunk_068ac2  ($068AC2, 8 B, thunks/jsr_abs_thunks.c)
/// $068AC2: jsr $28d70.l
pub fn i_068ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68AC8);
    cpu.pc = 0x28D70;
    16
}
/// $068AC8: rts
pub fn i_068ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68ACA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_068aca  ($068ACA, 6 B, thunks/ccr_helpers.c)
/// $068ACA: andi.b #$ee, ccr
pub fn i_068aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68ACE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068ACE: rts
pub fn i_068ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_Jitter_068ad0  ($068AD0, 36 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068AD0: jsr $68af4(pc)
pub fn i_068ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68AD4);
    cpu.pc = 0x68AF4;
    16
}
/// $068AD4: bcs.w $68ada
pub fn i_068ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AD8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68ADA; return 10; }
    8
}
/// $068AD8: rts
pub fn i_068ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68ADA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $068ADA: jsr $5e9b6.l
pub fn i_068ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68AE0);
    cpu.pc = 0x5E9B6;
    16
}
/// $068AE0: move.w #$ffff, d1
pub fn i_068ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AE4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068AE4: andi.w #$3, d0
pub fn i_068ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AE8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068AE8: bne.w $68aee
pub fn i_068ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68AEE; return 10; }
    8
}
/// $068AEC: clr.w d1
pub fn i_068aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AEE;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $068AEE: move.w d1, $82(a6)
pub fn i_068aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AF2;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068AF2: rts
pub fn i_068af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_IsActive_068af4  ($068AF4, 20 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068AF4: tst.w $106f5e.l
pub fn i_068af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AFA;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $068AFA: beq.w $68b0e
pub fn i_068afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68AFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68B0E; return 10; }
    8
}
/// $068AFE: tst.l $106f60.l
pub fn i_068afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B04;
    cpu.ir = 0x4AB9;
    let v2 = bus.read32(0x106F60);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $068B04: beq.w $68b0e
pub fn i_068b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68B0E; return 10; }
    8
}
// ===== SetXN_068b08  ($068B08, 6 B, thunks/ccr_helpers.c)
/// $068B08: ori.b #$11, ccr
pub fn i_068b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B0C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $068B0C: rts
pub fn i_068b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_068b0e  ($068B0E, 6 B, thunks/ccr_helpers.c)
/// $068B0E: andi.b #$ee, ccr
pub fn i_068b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B12;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068B12: rts
pub fn i_068b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_ShadowA_068b14  ($068B14, 42 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068B14: move.w #$4000, d0
pub fn i_068b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068B18: jsr $28134.l
pub fn i_068b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68B1E);
    cpu.pc = 0x28134;
    16
}
/// $068B1E: andi.w #$ffe3, $38(a6)
pub fn i_068b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B24;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $068B24: ori.w #$8, $38(a6)
pub fn i_068b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B2A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $068B2A: move.w $22(a6), d0
pub fn i_068b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068B2E: move.w $24(a6), d1
pub fn i_068b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B32;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068B32: subi.w #$20, d0
pub fn i_068b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B36;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068B36: addi.w #$1f, d1
pub fn i_068b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B3A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068B3A: move.w #$40, d2
pub fn i_068b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B3E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_068b3e  ($068B3E, 8 B, thunks/jsr_abs_thunks.c)
/// $068B3E: jsr $99812.l
pub fn i_068b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68B44);
    cpu.pc = 0x99812;
    16
}
/// $068B44: rts
pub fn i_068b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00068B46  ($068B46, 2 B, thunks/trivial_rts.c)
/// $068B46: rts
pub fn i_068b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_ShadowB_068b48  ($068B48, 20 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068B48: move.w $22(a6), d0
pub fn i_068b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B4C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068B4C: move.w $24(a6), d1
pub fn i_068b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B50;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068B50: subi.w #$38, d0
pub fn i_068b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B54;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x38, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068B54: addi.w #$1f, d1
pub fn i_068b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B58;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068B58: move.w #$70, d2
pub fn i_068b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B5C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_068b5c  ($068B5C, 8 B, thunks/jsr_abs_thunks.c)
/// $068B5C: jsr $99812.l
pub fn i_068b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68B62);
    cpu.pc = 0x99812;
    16
}
/// $068B62: rts
pub fn i_068b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_ShadowC_068b64  ($068B64, 54 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068B64: jsr $68af4(pc)
pub fn i_068b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68B68);
    cpu.pc = 0x68AF4;
    16
}
/// $068B68: bcs.w $68b72
pub fn i_068b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B6C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68B72; return 10; }
    8
}
/// $068B6C: clr.w d1
pub fn i_068b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B6E;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $068B6E: bra.w $68b86
pub fn i_068b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B72;
    cpu.ir = 0x6000;
    cpu.pc = 0x68B86;
    10
}
/// $068B72: jsr $5e9b6.l
pub fn i_068b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68B78);
    cpu.pc = 0x5E9B6;
    16
}
/// $068B78: move.w #$ffff, d1
pub fn i_068b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068B7C: andi.w #$3, d0
pub fn i_068b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B80;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068B80: bne.w $68b86
pub fn i_068b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68B86; return 10; }
    8
}
/// $068B84: clr.w d1
pub fn i_068b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B86;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $068B86: add.w $24(a6), d1
pub fn i_068b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B8A;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $068B8A: addi.w #$1e, d1
pub fn i_068b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B8E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068B8E: move.w $22(a6), d0
pub fn i_068b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B92;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068B92: subi.w #$38, d0
pub fn i_068b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B96;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x38, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068B96: move.w #$140, d2
pub fn i_068b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68B9A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_068b9a  ($068B9A, 8 B, thunks/jsr_abs_thunks.c)
/// $068B9A: jsr $99812.l
pub fn i_068b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68BA0);
    cpu.pc = 0x99812;
    16
}
/// $068BA0: rts
pub fn i_068ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_DropPow_068ba2  ($068BA2, 96 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068BA2: cmpi.w #$0, $76(a6)
pub fn i_068ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BA8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $068BA8: blt.w $68c00
pub fn i_068ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BAC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x68C00; return 10; }
    8
}
/// $068BAC: lea.l $2ba618.l, a0
pub fn i_068bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA618;
    20
}
/// $068BB2: jsr $799de.l
pub fn i_068bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68BB8);
    cpu.pc = 0x799DE;
    16
}
/// $068BB8: move.w d0, $78(a6)
pub fn i_068bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BBC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068BBC: jsr $5e9b6.l
pub fn i_068bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68BC2);
    cpu.pc = 0x5E9B6;
    16
}
/// $068BC2: andi.w #$ff, d0
pub fn i_068bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BC6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068BC6: cmp.w $78(a6), d0
pub fn i_068bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BCA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $068BCA: bgt.w $68c00
pub fn i_068bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BCE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68C00; return 10; }
    8
}
/// $068BCE: lea.l $3fec6.l, a1
pub fn i_068bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BD4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3FEC6;
    20
}
/// $068BD4: jsr $4ae.l
pub fn i_068bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68BDA);
    cpu.pc = 0x4AE;
    16
}
/// $068BDA: jsr $5dd02.l
pub fn i_068bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68BE0);
    cpu.pc = 0x5DD02;
    16
}
/// $068BE0: move.b $1.w, $98(a0)
pub fn i_068be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BE6;
    cpu.ir = 0x1178;
    let v2 = bus.read8(0x1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad3, v2 as u8);
    20
}
/// $068BE6: clr.b $99(a0)
pub fn i_068be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BEA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $068BEA: move.b $1.w, $9a(a0)
pub fn i_068bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BF0;
    cpu.ir = 0x1178;
    let v2 = bus.read8(0x1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad3, v2 as u8);
    20
}
/// $068BF0: move.w #$140, $22(a0)
pub fn i_068bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BF6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068BF6: move.w #$140, $24(a0)
pub fn i_068bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68BFC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068BFC: subq.w #$1, $76(a6)
pub fn i_068bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C00;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068C00: rts
pub fn i_068c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_HitCheck_068c02  ($068C02, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068C02: movea.l $8(a6), a1
pub fn i_068c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C06;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068C06: move.b $10(a6), d0
pub fn i_068c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C0A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $068C0A: cmp.b $10(a1), d0
pub fn i_068c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C0E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $068C0E: bcs.w $68c18
pub fn i_068c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C12;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68C18; return 10; }
    8
}
// ===== ClearXN_068c12  ($068C12, 6 B, thunks/ccr_helpers.c)
/// $068C12: andi.b #$ee, ccr
pub fn i_068c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C16;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068C16: rts
pub fn i_068c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_068c18  ($068C18, 6 B, thunks/ccr_helpers.c)
/// $068C18: ori.b #$11, ccr
pub fn i_068c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C1C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $068C1C: rts
pub fn i_068c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_Tmpl35_068c1e  ($068C1E, 302 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068C1E: clr.b $7d(a6)
pub fn i_068c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C22;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $068C22: clr.b $7e(a6)
pub fn i_068c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C26;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $068C26: bra.w $68c42
pub fn i_068c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x68C42;
    10
}
/// $068C2A: clr.b $7d(a6)
pub fn i_068c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $068C2E: move.b #$1, $7e(a6)
pub fn i_068c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $068C34: bra.w $68c42
pub fn i_068c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C38;
    cpu.ir = 0x6000;
    cpu.pc = 0x68C42;
    10
}
/// $068C38: move.b #$1, $7d(a6)
pub fn i_068c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $068C3E: clr.b $7e(a6)
pub fn i_068c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C42;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $068C42: jsr $69d5e(pc)
pub fn i_068c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68C46);
    cpu.pc = 0x69D5E;
    16
}
/// $068C46: jsr $5e7c0.l
pub fn i_068c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68C4C);
    cpu.pc = 0x5E7C0;
    16
}
/// $068C4C: jsr $267e2.l
pub fn i_068c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68C52);
    cpu.pc = 0x267E2;
    16
}
/// $068C52: move.w #$3, $1c(a6)
pub fn i_068c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068C58: jsr $138fe.l
pub fn i_068c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68C5E);
    cpu.pc = 0x138FE;
    16
}
/// $068C5E: lea.l $2b7f76.l, a0
pub fn i_068c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7F76;
    20
}
/// $068C64: jsr $799de.l
pub fn i_068c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68C6A);
    cpu.pc = 0x799DE;
    16
}
/// $068C6A: move.w d0, $76(a6)
pub fn i_068c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C6E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068C6E: lea.l $2b7ff8.l, a0
pub fn i_068c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7FF8;
    20
}
/// $068C74: jsr $799de.l
pub fn i_068c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68C7A);
    cpu.pc = 0x799DE;
    16
}
/// $068C7A: move.w d0, $78(a6)
pub fn i_068c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C7E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068C7E: move.w #$8000, d0
pub fn i_068c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C82;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068C82: jsr $28134.l
pub fn i_068c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68C88);
    cpu.pc = 0x28134;
    16
}
/// $068C88: andi.w #$ffe3, $38(a6)
pub fn i_068c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C8E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $068C8E: ori.w #$14, $38(a6)
pub fn i_068c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C94;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $068C94: move.w #$64, $66(a6)
pub fn i_068c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68C9A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068C9A: move.l #$2c86e0, $48(a6)
pub fn i_068c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CA2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C86E0;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068CA2: move.l #$2c8b18, $60(a6)
pub fn i_068ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CAA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8B18;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068CAA: clr.b $7a(a6)
pub fn i_068caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CAE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $068CAE: lea.l $69880(pc), a1
pub fn i_068cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69880;
    16
}
/// $068CB2: jsr $4ae.l
pub fn i_068cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68CB8);
    cpu.pc = 0x4AE;
    16
}
/// $068CB8: jsr $5dd02.l
pub fn i_068cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68CBE);
    cpu.pc = 0x5DD02;
    16
}
/// $068CBE: move.l a0, $88(a6)
pub fn i_068cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CC2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068CC2: move.b $98(a6), $98(a0)
pub fn i_068cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CC8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $068CC8: tst.b $99(a6)
pub fn i_068cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CCC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $068CCC: bne.w $68cea
pub fn i_068ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68CEA; return 10; }
    8
}
/// $068CD0: lea.l $69a32(pc), a1
pub fn i_068cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69A32;
    16
}
/// $068CD4: jsr $4ae.l
pub fn i_068cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68CDA);
    cpu.pc = 0x4AE;
    16
}
/// $068CDA: jsr $5dd02.l
pub fn i_068cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68CE0);
    cpu.pc = 0x5DD02;
    16
}
/// $068CE0: move.b $9b(a6), $9b(a0)
pub fn i_068ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CE6;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $068CE6: move.l a0, $8c(a6)
pub fn i_068ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CEA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068CEA: move.w #$0, d0
pub fn i_068cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CEE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068CEE: move.w #$e0, d1
pub fn i_068cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CF2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068CF2: cmpi.w #$100, $22(a6)
pub fn i_068cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CF8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $068CF8: bgt.w $68d04
pub fn i_068cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68CFC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68D04; return 10; }
    8
}
/// $068CFC: move.w #$1, d0
pub fn i_068cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068D00: move.w #$60, d1
pub fn i_068d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D04;
    cpu.ir = 0x323C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068D04: move.b d0, $3a(a6)
pub fn i_068d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D08;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $068D08: move.b d0, $70(a6)
pub fn i_068d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D0C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $068D0C: move.w d1, $72(a6)
pub fn i_068d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D10;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068D10: lea.l $723d2.l, a1
pub fn i_068d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D16;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723D2;
    20
}
/// $068D16: jsr $4ae.l
pub fn i_068d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68D1C);
    cpu.pc = 0x4AE;
    16
}
/// $068D1C: jsr $5dd02.l
pub fn i_068d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68D22);
    cpu.pc = 0x5DD02;
    16
}
/// $068D22: clr.w $98(a0)
pub fn i_068d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D26;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $068D26: lea.l $7773e.l, a1
pub fn i_068d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D2C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7773E;
    20
}
/// $068D2C: jsr $4ae.l
pub fn i_068d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68D32);
    cpu.pc = 0x4AE;
    16
}
/// $068D32: jsr $5dd02.l
pub fn i_068d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68D38);
    cpu.pc = 0x5DD02;
    16
}
/// $068D38: move.l #$2c89cc, $4c(a6)
pub fn i_068d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D40;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C89CC;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068D40: tst.b $9a(a6)
pub fn i_068d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D44;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $068D44: beq.w $68ddc
pub fn i_068d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68DDC; return 10; }
    8
}
/// $068D48: bra.w $68f3a
pub fn i_068d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x68F3A;
    10
}
// ===== Tank_Fall_068d4c  ($068D4C, 144 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068D4C: jsr $267e2.l
pub fn i_068d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68D52);
    cpu.pc = 0x267E2;
    16
}
/// $068D52: cmpi.l #$ffffffff, $8c(a6)
pub fn i_068d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D5A;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $068D5A: beq.w $68d84
pub fn i_068d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68D84; return 10; }
    8
}
/// $068D5E: move.w #$0, d0
pub fn i_068d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068D62: cmpi.w #$e, d0
pub fn i_068d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D66;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068D66: ble.w $68d76
pub fn i_068d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D6A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68D76; return 10; }
    8
}
/// $068D6A: nop
pub fn i_068d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D6C;
    cpu.ir = 0x4E71;
    4
}
/// $068D6C: nop
pub fn i_068d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D6E;
    cpu.ir = 0x4E71;
    4
}
/// $068D6E: cmpi.w #$e, d0
pub fn i_068d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D72;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068D72: nop
pub fn i_068d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D74;
    cpu.ir = 0x4E71;
    4
}
/// $068D74: trap #$f
pub fn i_068d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D76;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068D76: movea.l $8c(a6), a1
pub fn i_068d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D7A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068D7A: lea.l $2c8bc2.l, a0
pub fn i_068d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $068D80: jsr $69d76(pc)
pub fn i_068d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68D84);
    cpu.pc = 0x69D76;
    16
}
/// $068D84: move.w #$0, d0
pub fn i_068d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D88;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068D88: cmpi.w #$a, d0
pub fn i_068d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D8C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068D8C: ble.w $68d9c
pub fn i_068d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D90;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68D9C; return 10; }
    8
}
/// $068D90: nop
pub fn i_068d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D92;
    cpu.ir = 0x4E71;
    4
}
/// $068D92: nop
pub fn i_068d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D94;
    cpu.ir = 0x4E71;
    4
}
/// $068D94: cmpi.w #$a, d0
pub fn i_068d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D98;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068D98: nop
pub fn i_068d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D9A;
    cpu.ir = 0x4E71;
    4
}
/// $068D9A: trap #$f
pub fn i_068d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68D9C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068D9C: movea.l $88(a6), a1
pub fn i_068d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DA0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068DA0: lea.l $2c8b96.l, a0
pub fn i_068da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $068DA6: jsr $69d76(pc)
pub fn i_068da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68DAA);
    cpu.pc = 0x69D76;
    16
}
/// $068DAA: lea.l $2c8cce.l, a0
pub fn i_068daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8CCE;
    20
}
/// $068DB0: jsr $28cd4.l
pub fn i_068db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68DB6);
    cpu.pc = 0x28CD4;
    16
}
/// $068DB6: lea.l $68dbc(pc), a1
pub fn i_068db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68DBC;
    16
}
/// $068DBA: move.l a1, (a6)
pub fn i_068dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068DBC: jsr $28998.l
pub fn i_068dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68DC2);
    cpu.pc = 0x28998;
    16
}
/// $068DC2: jsr $27c8c.l
pub fn i_068dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68DC8);
    cpu.pc = 0x27C8C;
    16
}
/// $068DC8: bcc.w $68dd2
pub fn i_068dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DCC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68DD2; return 10; }
    8
}
/// $068DCC: lea.l $68f3a(pc), a1
pub fn i_068dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68F3A;
    16
}
/// $068DD0: move.l a1, (a6)
pub fn i_068dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068DD2: jsr $28d70.l
pub fn i_068dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68DD8);
    cpu.pc = 0x28D70;
    16
}
/// $068DD8: bra.w $697ea
pub fn i_068dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x697EA;
    10
}
// ===== Tank_Drive_068ddc  ($068DDC, 350 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068DDC: tst.b $9a(a6)
pub fn i_068ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DE0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $068DE0: bne.w $68f3a
pub fn i_068de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68F3A; return 10; }
    8
}
/// $068DE4: move.w #$109e, d0
pub fn i_068de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DE8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068DE8: jsr $2352.l
pub fn i_068de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68DEE);
    cpu.pc = 0x2352;
    16
}
/// $068DEE: jsr $267e2.l
pub fn i_068dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68DF4);
    cpu.pc = 0x267E2;
    16
}
/// $068DF4: move.w #$300, d0
pub fn i_068df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068DF8: jsr $69dd4(pc)
pub fn i_068df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68DFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68DFC);
    cpu.pc = 0x69DD4;
    16
}
/// $068DFC: move.w d0, $28(a6)
pub fn i_068dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E00;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068E00: clr.w $96(a6)
pub fn i_068e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E04;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $068E04: jsr $69ddc(pc)
pub fn i_068e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68E08);
    cpu.pc = 0x69DDC;
    16
}
/// $068E08: bcs.w $68e7c
pub fn i_068e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E0C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68E7C; return 10; }
    8
}
/// $068E0C: cmpi.l #$ffffffff, $8c(a6)
pub fn i_068e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E14;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $068E14: beq.w $68e3e
pub fn i_068e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68E3E; return 10; }
    8
}
/// $068E18: move.w #$1, d0
pub fn i_068e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068E1C: cmpi.w #$e, d0
pub fn i_068e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E20;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068E20: ble.w $68e30
pub fn i_068e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E24;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68E30; return 10; }
    8
}
/// $068E24: nop
pub fn i_068e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E26;
    cpu.ir = 0x4E71;
    4
}
/// $068E26: nop
pub fn i_068e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E28;
    cpu.ir = 0x4E71;
    4
}
/// $068E28: cmpi.w #$e, d0
pub fn i_068e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E2C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068E2C: nop
pub fn i_068e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E2E;
    cpu.ir = 0x4E71;
    4
}
/// $068E2E: trap #$f
pub fn i_068e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E30;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068E30: movea.l $8c(a6), a1
pub fn i_068e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E34;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068E34: lea.l $2c8bc2.l, a0
pub fn i_068e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $068E3A: jsr $69d76(pc)
pub fn i_068e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68E3E);
    cpu.pc = 0x69D76;
    16
}
/// $068E3E: move.w #$1, d0
pub fn i_068e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068E42: cmpi.w #$a, d0
pub fn i_068e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E46;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068E46: ble.w $68e56
pub fn i_068e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E4A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68E56; return 10; }
    8
}
/// $068E4A: nop
pub fn i_068e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E4C;
    cpu.ir = 0x4E71;
    4
}
/// $068E4C: nop
pub fn i_068e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E4E;
    cpu.ir = 0x4E71;
    4
}
/// $068E4E: cmpi.w #$a, d0
pub fn i_068e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E52;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068E52: nop
pub fn i_068e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E54;
    cpu.ir = 0x4E71;
    4
}
/// $068E54: trap #$f
pub fn i_068e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E56;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068E56: movea.l $88(a6), a1
pub fn i_068e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E5A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068E5A: lea.l $2c8b96.l, a0
pub fn i_068e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $068E60: jsr $69d76(pc)
pub fn i_068e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68E64);
    cpu.pc = 0x69D76;
    16
}
/// $068E64: lea.l $2c8d6a.l, a0
pub fn i_068e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8D6A;
    20
}
/// $068E6A: jsr $28cd4.l
pub fn i_068e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68E70);
    cpu.pc = 0x28CD4;
    16
}
/// $068E70: move.l #$2c8e8a, $90(a6)
pub fn i_068e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E78;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8E8A;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068E78: bra.w $68ee8
pub fn i_068e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x68EE8;
    10
}
/// $068E7C: cmpi.l #$ffffffff, $8c(a6)
pub fn i_068e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E84;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $068E84: beq.w $68eae
pub fn i_068e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E88;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68EAE; return 10; }
    8
}
/// $068E88: move.w #$2, d0
pub fn i_068e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068E8C: cmpi.w #$e, d0
pub fn i_068e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E90;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068E90: ble.w $68ea0
pub fn i_068e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E94;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68EA0; return 10; }
    8
}
/// $068E94: nop
pub fn i_068e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E96;
    cpu.ir = 0x4E71;
    4
}
/// $068E96: nop
pub fn i_068e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E98;
    cpu.ir = 0x4E71;
    4
}
/// $068E98: cmpi.w #$e, d0
pub fn i_068e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E9C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068E9C: nop
pub fn i_068e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E9E;
    cpu.ir = 0x4E71;
    4
}
/// $068E9E: trap #$f
pub fn i_068e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EA0;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068EA0: movea.l $8c(a6), a1
pub fn i_068ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EA4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068EA4: lea.l $2c8bc2.l, a0
pub fn i_068ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $068EAA: jsr $69d76(pc)
pub fn i_068eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68EAE);
    cpu.pc = 0x69D76;
    16
}
/// $068EAE: move.w #$1, d0
pub fn i_068eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EB2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068EB2: cmpi.w #$a, d0
pub fn i_068eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EB6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068EB6: ble.w $68ec6
pub fn i_068eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EBA;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68EC6; return 10; }
    8
}
/// $068EBA: nop
pub fn i_068eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EBC;
    cpu.ir = 0x4E71;
    4
}
/// $068EBC: nop
pub fn i_068ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EBE;
    cpu.ir = 0x4E71;
    4
}
/// $068EBE: cmpi.w #$a, d0
pub fn i_068ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EC2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068EC2: nop
pub fn i_068ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EC4;
    cpu.ir = 0x4E71;
    4
}
/// $068EC4: trap #$f
pub fn i_068ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EC6;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068EC6: movea.l $88(a6), a1
pub fn i_068ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68ECA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068ECA: lea.l $2c8b96.l, a0
pub fn i_068eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68ED0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $068ED0: jsr $69d76(pc)
pub fn i_068ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68ED4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68ED4);
    cpu.pc = 0x69D76;
    16
}
/// $068ED4: lea.l $2c8eba.l, a0
pub fn i_068ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8EBA;
    20
}
/// $068EDA: jsr $28cd4.l
pub fn i_068eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68EE0);
    cpu.pc = 0x28CD4;
    16
}
/// $068EE0: move.l #$2c8f80, $90(a6)
pub fn i_068ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EE8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8F80;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068EE8: jsr $69e36(pc)
pub fn i_068ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68EEC);
    cpu.pc = 0x69E36;
    16
}
/// $068EEC: lea.l $68ef2(pc), a1
pub fn i_068eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68EF2;
    16
}
/// $068EF0: move.l a1, (a6)
pub fn i_068ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068EF2: jsr $28998.l
pub fn i_068ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68EF8);
    cpu.pc = 0x28998;
    16
}
/// $068EF8: jsr $27afc.l
pub fn i_068ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68EFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68EFE);
    cpu.pc = 0x27AFC;
    16
}
/// $068EFE: bcc.w $68f08
pub fn i_068efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F02;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68F08; return 10; }
    8
}
/// $068F02: lea.l $6900a(pc), a1
pub fn i_068f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6900A;
    16
}
/// $068F06: move.l a1, (a6)
pub fn i_068f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068F08: jsr $28d70.l
pub fn i_068f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68F0E);
    cpu.pc = 0x28D70;
    16
}
/// $068F0E: jsr $69df4(pc)
pub fn i_068f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68F12);
    cpu.pc = 0x69DF4;
    16
}
/// $068F12: bcc.w $68f1c
pub fn i_068f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68F1C; return 10; }
    8
}
/// $068F16: lea.l $68fe4(pc), a1
pub fn i_068f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68FE4;
    16
}
/// $068F1A: move.l a1, (a6)
pub fn i_068f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068F1C: jsr $69e14(pc)
pub fn i_068f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68F20);
    cpu.pc = 0x69E14;
    16
}
/// $068F20: bcc.w $68f2a
pub fn i_068f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68F2A; return 10; }
    8
}
/// $068F24: lea.l $6900a(pc), a1
pub fn i_068f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6900A;
    16
}
/// $068F28: move.l a1, (a6)
pub fn i_068f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068F2A: jsr $283ca.l
pub fn i_068f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68F30);
    cpu.pc = 0x283CA;
    16
}
/// $068F30: jsr $283d8.l
pub fn i_068f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68F36);
    cpu.pc = 0x283D8;
    16
}
/// $068F36: bra.w $697da
pub fn i_068f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x697DA;
    10
}
// ===== Tank_Idle_068f3a  ($068F3A, 170 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068F3A: move.w #$f, $74(a6)
pub fn i_068f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F40;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068F40: cmpi.l #$ffffffff, $8c(a6)
pub fn i_068f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F48;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $068F48: beq.w $68f72
pub fn i_068f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68F72; return 10; }
    8
}
/// $068F4C: move.w #$0, d0
pub fn i_068f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F50;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068F50: cmpi.w #$e, d0
pub fn i_068f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F54;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068F54: ble.w $68f64
pub fn i_068f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F58;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68F64; return 10; }
    8
}
/// $068F58: nop
pub fn i_068f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F5A;
    cpu.ir = 0x4E71;
    4
}
/// $068F5A: nop
pub fn i_068f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F5C;
    cpu.ir = 0x4E71;
    4
}
/// $068F5C: cmpi.w #$e, d0
pub fn i_068f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F60;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $068F60: nop
pub fn i_068f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F62;
    cpu.ir = 0x4E71;
    4
}
/// $068F62: trap #$f
pub fn i_068f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F64;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068F64: movea.l $8c(a6), a1
pub fn i_068f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F68;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068F68: lea.l $2c8bc2.l, a0
pub fn i_068f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $068F6E: jsr $69d76(pc)
pub fn i_068f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68F72);
    cpu.pc = 0x69D76;
    16
}
/// $068F72: move.w #$0, d0
pub fn i_068f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068F76: cmpi.w #$a, d0
pub fn i_068f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F7A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068F7A: ble.w $68f8a
pub fn i_068f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F7E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68F8A; return 10; }
    8
}
/// $068F7E: nop
pub fn i_068f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F80;
    cpu.ir = 0x4E71;
    4
}
/// $068F80: nop
pub fn i_068f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F82;
    cpu.ir = 0x4E71;
    4
}
/// $068F82: cmpi.w #$a, d0
pub fn i_068f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F86;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $068F86: nop
pub fn i_068f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F88;
    cpu.ir = 0x4E71;
    4
}
/// $068F88: trap #$f
pub fn i_068f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F8A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $068F8A: movea.l $88(a6), a1
pub fn i_068f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F8E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $068F8E: lea.l $2c8b96.l, a0
pub fn i_068f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $068F94: jsr $69d76(pc)
pub fn i_068f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68F98);
    cpu.pc = 0x69D76;
    16
}
/// $068F98: lea.l $2c8cce.l, a0
pub fn i_068f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68F9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8CCE;
    20
}
/// $068F9E: jsr $28cd4.l
pub fn i_068f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68FA4);
    cpu.pc = 0x28CD4;
    16
}
/// $068FA4: lea.l $68faa(pc), a1
pub fn i_068fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68FAA;
    16
}
/// $068FA8: move.l a1, (a6)
pub fn i_068fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068FAA: jsr $28998.l
pub fn i_068faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68FB0);
    cpu.pc = 0x28998;
    16
}
/// $068FB0: jsr $2783a.l
pub fn i_068fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68FB6);
    cpu.pc = 0x2783A;
    16
}
/// $068FB6: jsr $28d70.l
pub fn i_068fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68FBC);
    cpu.pc = 0x28D70;
    16
}
/// $068FBC: tst.b $9a(a6)
pub fn i_068fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FC0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $068FC0: bne.w $68fd2
pub fn i_068fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68FD2; return 10; }
    8
}
/// $068FC4: subq.w #$1, $74(a6)
pub fn i_068fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FC8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $068FC8: bne.w $68fd2
pub fn i_068fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FCC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68FD2; return 10; }
    8
}
/// $068FCC: lea.l $690ea(pc), a1
pub fn i_068fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x690EA;
    16
}
/// $068FD0: move.l a1, (a6)
pub fn i_068fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068FD2: jsr $69e14(pc)
pub fn i_068fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68FD6);
    cpu.pc = 0x69E14;
    16
}
/// $068FD6: bcc.w $68fe0
pub fn i_068fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68FE0; return 10; }
    8
}
/// $068FDA: lea.l $69190(pc), a1
pub fn i_068fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69190;
    16
}
/// $068FDE: move.l a1, (a6)
pub fn i_068fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068FE0: bra.w $697da
pub fn i_068fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x697DA;
    10
}
// ===== Tank_Turn_068fe4  ($068FE4, 38 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068FE4: jsr $69ddc(pc)
pub fn i_068fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68FE8);
    cpu.pc = 0x69DDC;
    16
}
/// $068FE8: bcs.w $68ff8
pub fn i_068fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FEC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68FF8; return 10; }
    8
}
/// $068FEC: move.l #$2c8e8a, $90(a6)
pub fn i_068fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FF4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8E8A;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $068FF4: bra.w $69000
pub fn i_068ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68FF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x69000;
    10
}
/// $068FF8: move.l #$2c8f80, $90(a6)
pub fn i_068ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69000;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8F80;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069000: eori.b #$1, $70(a6)
pub fn i_069000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69006;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069006: bra.w $69026
pub fn i_069006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6900A;
    cpu.ir = 0x6000;
    cpu.pc = 0x69026;
    10
}
// ===== Tank_Brake_06900a  ($06900A, 224 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06900A: jsr $69ddc(pc)
pub fn i_06900a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6900E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6900E);
    cpu.pc = 0x69DDC;
    16
}
/// $06900E: bcs.w $6901e
pub fn i_06900e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69012;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6901E; return 10; }
    8
}
/// $069012: move.l #$2c8e8a, $90(a6)
pub fn i_069012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6901A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8E8A;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06901A: bra.w $69026
pub fn i_06901a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6901E;
    cpu.ir = 0x6000;
    cpu.pc = 0x69026;
    10
}
/// $06901E: move.l #$2c8f80, $90(a6)
pub fn i_06901e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69026;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8F80;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069026: move.w $28(a6), d0
pub fn i_069026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6902A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06902A: asr.w #$4, d0
pub fn i_06902a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6902C;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06902C: clr.w $96(a6)
pub fn i_06902c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69030;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $069030: neg.w d0
pub fn i_069030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69032;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $069032: move.w d0, $2c(a6)
pub fn i_069032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69036;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069036: move.w #$109e, d0
pub fn i_069036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6903A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06903A: jsr $2222.l
pub fn i_06903a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69040);
    cpu.pc = 0x2222;
    16
}
/// $069040: lea.l $69046(pc), a1
pub fn i_069040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69044;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69046;
    16
}
/// $069044: move.l a1, (a6)
pub fn i_069044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69046;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069046: jsr $28998.l
pub fn i_069046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6904C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6904C);
    cpu.pc = 0x28998;
    16
}
/// $06904C: jsr $27afc.l
pub fn i_06904c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69052);
    cpu.pc = 0x27AFC;
    16
}
/// $069052: cmpi.w #$0, $2c(a6)
pub fn i_069052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69058;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $069058: beq.w $690ce
pub fn i_069058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6905C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x690CE; return 10; }
    8
}
/// $06905C: cmpi.w #$0, $28(a6)
pub fn i_06905c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69062;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $069062: bne.w $690ce
pub fn i_069062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69066;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x690CE; return 10; }
    8
}
/// $069066: clr.w $2c(a6)
pub fn i_069066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6906A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06906A: cmpi.l #$ffffffff, $8c(a6)
pub fn i_06906a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69072;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $069072: beq.w $6909c
pub fn i_069072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69076;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6909C; return 10; }
    8
}
/// $069076: move.w #$3, d0
pub fn i_069076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6907A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06907A: cmpi.w #$e, d0
pub fn i_06907a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6907E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06907E: ble.w $6908e
pub fn i_06907e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69082;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6908E; return 10; }
    8
}
/// $069082: nop
pub fn i_069082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69084;
    cpu.ir = 0x4E71;
    4
}
/// $069084: nop
pub fn i_069084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69086;
    cpu.ir = 0x4E71;
    4
}
/// $069086: cmpi.w #$e, d0
pub fn i_069086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6908A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06908A: nop
pub fn i_06908a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6908C;
    cpu.ir = 0x4E71;
    4
}
/// $06908C: trap #$f
pub fn i_06908c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6908E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06908E: movea.l $8c(a6), a1
pub fn i_06908e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69092;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069092: lea.l $2c8bc2.l, a0
pub fn i_069092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69098;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069098: jsr $69d76(pc)
pub fn i_069098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6909C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6909C);
    cpu.pc = 0x69D76;
    16
}
/// $06909C: move.w #$2, d0
pub fn i_06909c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0690A0: cmpi.w #$a, d0
pub fn i_0690a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690A4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $0690A4: ble.w $690b4
pub fn i_0690a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690A8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x690B4; return 10; }
    8
}
/// $0690A8: nop
pub fn i_0690a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690AA;
    cpu.ir = 0x4E71;
    4
}
/// $0690AA: nop
pub fn i_0690aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690AC;
    cpu.ir = 0x4E71;
    4
}
/// $0690AC: cmpi.w #$a, d0
pub fn i_0690ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690B0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $0690B0: nop
pub fn i_0690b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690B2;
    cpu.ir = 0x4E71;
    4
}
/// $0690B2: trap #$f
pub fn i_0690b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690B4;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0690B4: movea.l $88(a6), a1
pub fn i_0690b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690B8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0690B8: lea.l $2c8b96.l, a0
pub fn i_0690b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $0690BE: jsr $69d76(pc)
pub fn i_0690be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x690C2);
    cpu.pc = 0x69D76;
    16
}
/// $0690C2: lea.l $2c8fb0.l, a0
pub fn i_0690c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8FB0;
    20
}
/// $0690C8: jsr $28cd4.l
pub fn i_0690c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x690CE);
    cpu.pc = 0x28CD4;
    16
}
/// $0690CE: jsr $69e36(pc)
pub fn i_0690ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x690D2);
    cpu.pc = 0x69E36;
    16
}
/// $0690D2: jsr $28d70.l
pub fn i_0690d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x690D8);
    cpu.pc = 0x28D70;
    16
}
/// $0690D8: bcc.w $690e2
pub fn i_0690d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x690E2; return 10; }
    8
}
/// $0690DC: lea.l $68f3a(pc), a1
pub fn i_0690dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68F3A;
    16
}
/// $0690E0: move.l a1, (a6)
pub fn i_0690e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0690E2: jsr $69e14(pc)
pub fn i_0690e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x690E6);
    cpu.pc = 0x69E14;
    16
}
/// $0690E6: bra.w $697da
pub fn i_0690e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x697DA;
    10
}
// ===== Tank_Resume_0690ea  ($0690EA, 166 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0690EA: tst.b $9a(a6)
pub fn i_0690ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690EE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0690EE: bne.w $68f3a
pub fn i_0690ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68F3A; return 10; }
    8
}
/// $0690F2: jsr $69e14(pc)
pub fn i_0690f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x690F6);
    cpu.pc = 0x69E14;
    16
}
/// $0690F6: bcs.w $69190
pub fn i_0690f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x690FA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x69190; return 10; }
    8
}
/// $0690FA: cmpi.l #$ffffffff, $8c(a6)
pub fn i_0690fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69102;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $069102: beq.w $6912c
pub fn i_069102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69106;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6912C; return 10; }
    8
}
/// $069106: move.w #$4, d0
pub fn i_069106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6910A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06910A: cmpi.w #$e, d0
pub fn i_06910a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6910E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06910E: ble.w $6911e
pub fn i_06910e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69112;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6911E; return 10; }
    8
}
/// $069112: nop
pub fn i_069112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69114;
    cpu.ir = 0x4E71;
    4
}
/// $069114: nop
pub fn i_069114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69116;
    cpu.ir = 0x4E71;
    4
}
/// $069116: cmpi.w #$e, d0
pub fn i_069116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6911A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06911A: nop
pub fn i_06911a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6911C;
    cpu.ir = 0x4E71;
    4
}
/// $06911C: trap #$f
pub fn i_06911c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6911E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06911E: movea.l $8c(a6), a1
pub fn i_06911e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69122;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069122: lea.l $2c8bc2.l, a0
pub fn i_069122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69128;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069128: jsr $69d76(pc)
pub fn i_069128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6912C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6912C);
    cpu.pc = 0x69D76;
    16
}
/// $06912C: move.w #$3, d0
pub fn i_06912c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69130;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069130: cmpi.w #$a, d0
pub fn i_069130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69134;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069134: ble.w $69144
pub fn i_069134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69138;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69144; return 10; }
    8
}
/// $069138: nop
pub fn i_069138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6913A;
    cpu.ir = 0x4E71;
    4
}
/// $06913A: nop
pub fn i_06913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6913C;
    cpu.ir = 0x4E71;
    4
}
/// $06913C: cmpi.w #$a, d0
pub fn i_06913c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69140;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069140: nop
pub fn i_069140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69142;
    cpu.ir = 0x4E71;
    4
}
/// $069142: trap #$f
pub fn i_069142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69144;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069144: movea.l $88(a6), a1
pub fn i_069144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69148;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069148: lea.l $2c8b96.l, a0
pub fn i_069148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6914E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $06914E: jsr $69d76(pc)
pub fn i_06914e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69152;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69152);
    cpu.pc = 0x69D76;
    16
}
/// $069152: lea.l $2c90f4.l, a0
pub fn i_069152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69158;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C90F4;
    20
}
/// $069158: jsr $28cd4.l
pub fn i_069158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6915E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6915E);
    cpu.pc = 0x28CD4;
    16
}
/// $06915E: lea.l $69164(pc), a1
pub fn i_06915e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69162;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69164;
    16
}
/// $069162: move.l a1, (a6)
pub fn i_069162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69164;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069164: jsr $28998.l
pub fn i_069164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6916A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6916A);
    cpu.pc = 0x28998;
    16
}
/// $06916A: jsr $2783a.l
pub fn i_06916a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69170);
    cpu.pc = 0x2783A;
    16
}
/// $069170: jsr $28d70.l
pub fn i_069170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69176;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69176);
    cpu.pc = 0x28D70;
    16
}
/// $069176: bcc.w $69180
pub fn i_069176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6917A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69180; return 10; }
    8
}
/// $06917A: lea.l $68ddc(pc), a1
pub fn i_06917a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6917E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68DDC;
    16
}
/// $06917E: move.l a1, (a6)
pub fn i_06917e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69180;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069180: jsr $283ca.l
pub fn i_069180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69186;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69186);
    cpu.pc = 0x283CA;
    16
}
/// $069186: jsr $283d8.l
pub fn i_069186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6918C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6918C);
    cpu.pc = 0x283D8;
    16
}
/// $06918C: bra.w $697da
pub fn i_06918c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69190;
    cpu.ir = 0x6000;
    cpu.pc = 0x697DA;
    10
}
// ===== Tank_AimPlayer_069190  ($069190, 326 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069190: cmpi.b #$0, $7a(a6)
pub fn i_069190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69196;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $069196: bne.w $692e6
pub fn i_069196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6919A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x692E6; return 10; }
    8
}
/// $06919A: move.b #$1, d0
pub fn i_06919a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6919E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06919E: bra.w $691cc
pub fn i_06919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x691CC;
    10
}
/// $0691A2: lea.l $2c8b74.l, a0
pub fn i_0691a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B74;
    20
}
/// $0691A8: jsr $5e0d4.l
pub fn i_0691a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x691AE);
    cpu.pc = 0x5E0D4;
    16
}
/// $0691AE: bcs.w $691ca
pub fn i_0691ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691B2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x691CA; return 10; }
    8
}
/// $0691B2: move.w $22(a6), d0
pub fn i_0691b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691B6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0691B6: sub.w $22(a0), d0
pub fn i_0691b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691BA;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0691BA: cmpi.w #$a0, d0
pub fn i_0691ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691BE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0691BE: bgt.w $691ca
pub fn i_0691be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691C2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x691CA; return 10; }
    8
}
/// $0691C2: move.b #$1, d0
pub fn i_0691c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691C6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0691C6: bra.w $691cc
pub fn i_0691c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x691CC;
    10
}
/// $0691CA: clr.b d0
pub fn i_0691ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691CC;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0691CC: move.b d0, $7b(a6)
pub fn i_0691cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691D0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0691D0: beq.w $6923c
pub fn i_0691d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6923C; return 10; }
    8
}
/// $0691D4: cmpi.l #$ffffffff, $8c(a6)
pub fn i_0691d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691DC;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0691DC: beq.w $69206
pub fn i_0691dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69206; return 10; }
    8
}
/// $0691E0: move.w #$5, d0
pub fn i_0691e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0691E4: cmpi.w #$e, d0
pub fn i_0691e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691E8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $0691E8: ble.w $691f8
pub fn i_0691e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691EC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x691F8; return 10; }
    8
}
/// $0691EC: nop
pub fn i_0691ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691EE;
    cpu.ir = 0x4E71;
    4
}
/// $0691EE: nop
pub fn i_0691ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691F0;
    cpu.ir = 0x4E71;
    4
}
/// $0691F0: cmpi.w #$e, d0
pub fn i_0691f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691F4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $0691F4: nop
pub fn i_0691f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691F6;
    cpu.ir = 0x4E71;
    4
}
/// $0691F6: trap #$f
pub fn i_0691f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691F8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0691F8: movea.l $8c(a6), a1
pub fn i_0691f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x691FC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0691FC: lea.l $2c8bc2.l, a0
pub fn i_0691fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69202;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069202: jsr $69d76(pc)
pub fn i_069202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69206;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69206);
    cpu.pc = 0x69D76;
    16
}
/// $069206: move.w #$5, d0
pub fn i_069206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6920A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06920A: cmpi.w #$a, d0
pub fn i_06920a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6920E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $06920E: ble.w $6921e
pub fn i_06920e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69212;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6921E; return 10; }
    8
}
/// $069212: nop
pub fn i_069212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69214;
    cpu.ir = 0x4E71;
    4
}
/// $069214: nop
pub fn i_069214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69216;
    cpu.ir = 0x4E71;
    4
}
/// $069216: cmpi.w #$a, d0
pub fn i_069216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6921A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $06921A: nop
pub fn i_06921a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6921C;
    cpu.ir = 0x4E71;
    4
}
/// $06921C: trap #$f
pub fn i_06921c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6921E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06921E: movea.l $88(a6), a1
pub fn i_06921e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69222;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069222: lea.l $2c8b96.l, a0
pub fn i_069222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69228;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $069228: jsr $69d76(pc)
pub fn i_069228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6922C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6922C);
    cpu.pc = 0x69D76;
    16
}
/// $06922C: lea.l $2c922a.l, a0
pub fn i_06922c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69232;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C922A;
    20
}
/// $069232: jsr $28cd4.l
pub fn i_069232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69238);
    cpu.pc = 0x28CD4;
    16
}
/// $069238: bra.w $692a0
pub fn i_069238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6923C;
    cpu.ir = 0x6000;
    cpu.pc = 0x692A0;
    10
}
/// $06923C: cmpi.l #$ffffffff, $8c(a6)
pub fn i_06923c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69244;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $069244: beq.w $6926e
pub fn i_069244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69248;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6926E; return 10; }
    8
}
/// $069248: move.w #$6, d0
pub fn i_069248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6924C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06924C: cmpi.w #$e, d0
pub fn i_06924c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69250;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069250: ble.w $69260
pub fn i_069250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69254;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69260; return 10; }
    8
}
/// $069254: nop
pub fn i_069254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69256;
    cpu.ir = 0x4E71;
    4
}
/// $069256: nop
pub fn i_069256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69258;
    cpu.ir = 0x4E71;
    4
}
/// $069258: cmpi.w #$e, d0
pub fn i_069258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6925C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06925C: nop
pub fn i_06925c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6925E;
    cpu.ir = 0x4E71;
    4
}
/// $06925E: trap #$f
pub fn i_06925e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69260;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069260: movea.l $8c(a6), a1
pub fn i_069260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69264;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069264: lea.l $2c8bc2.l, a0
pub fn i_069264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6926A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $06926A: jsr $69d76(pc)
pub fn i_06926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6926E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6926E);
    cpu.pc = 0x69D76;
    16
}
/// $06926E: move.w #$5, d0
pub fn i_06926e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69272;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069272: cmpi.w #$a, d0
pub fn i_069272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69276;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069276: ble.w $69286
pub fn i_069276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6927A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69286; return 10; }
    8
}
/// $06927A: nop
pub fn i_06927a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6927C;
    cpu.ir = 0x4E71;
    4
}
/// $06927C: nop
pub fn i_06927c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6927E;
    cpu.ir = 0x4E71;
    4
}
/// $06927E: cmpi.w #$a, d0
pub fn i_06927e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69282;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069282: nop
pub fn i_069282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69284;
    cpu.ir = 0x4E71;
    4
}
/// $069284: trap #$f
pub fn i_069284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69286;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069286: movea.l $88(a6), a1
pub fn i_069286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6928A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06928A: lea.l $2c8b96.l, a0
pub fn i_06928a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69290;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $069290: jsr $69d76(pc)
pub fn i_069290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69294;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69294);
    cpu.pc = 0x69D76;
    16
}
/// $069294: lea.l $2c92d8.l, a0
pub fn i_069294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6929A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C92D8;
    20
}
/// $06929A: jsr $28cd4.l
pub fn i_06929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x692A0);
    cpu.pc = 0x28CD4;
    16
}
/// $0692A0: move.w #$32, $74(a6)
pub fn i_0692a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0692A6: lea.l $692ac(pc), a1
pub fn i_0692a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x692AC;
    16
}
/// $0692AA: move.l a1, (a6)
pub fn i_0692aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0692AC: jsr $28998.l
pub fn i_0692ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x692B2);
    cpu.pc = 0x28998;
    16
}
/// $0692B2: jsr $2783a.l
pub fn i_0692b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x692B8);
    cpu.pc = 0x2783A;
    16
}
/// $0692B8: jsr $28d70.l
pub fn i_0692b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x692BE);
    cpu.pc = 0x28D70;
    16
}
/// $0692BE: subq.w #$1, $74(a6)
pub fn i_0692be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692C2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0692C2: bne.w $692cc
pub fn i_0692c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x692CC; return 10; }
    8
}
/// $0692C6: lea.l $692d6(pc), a1
pub fn i_0692c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x692D6;
    16
}
/// $0692CA: move.l a1, (a6)
pub fn i_0692ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0692CC: bclr.b #$3, $13(a6)
pub fn i_0692cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692D2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0692D2: bra.w $697da
pub fn i_0692d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x697DA;
    10
}
// ===== Tank_RngReload_0692d6  ($0692D6, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0692D6: lea.l $2b807a.l, a0
pub fn i_0692d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B807A;
    20
}
/// $0692DC: jsr $799de.l
pub fn i_0692dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x692E2);
    cpu.pc = 0x799DE;
    16
}
/// $0692E2: move.b d0, $7a(a6)
pub fn i_0692e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692E6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Tank_FireBurst_0692e6  ($0692E6, 328 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0692E6: lea.l $2b7ff8.l, a0
pub fn i_0692e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7FF8;
    20
}
/// $0692EC: jsr $799de.l
pub fn i_0692ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x692F2);
    cpu.pc = 0x799DE;
    16
}
/// $0692F2: move.w d0, $78(a6)
pub fn i_0692f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692F6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0692F6: cmpi.b #$0, $7b(a6)
pub fn i_0692f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x692FC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0692FC: bne.w $69368
pub fn i_0692fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69300;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x69368; return 10; }
    8
}
/// $069300: cmpi.l #$ffffffff, $8c(a6)
pub fn i_069300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69308;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $069308: beq.w $69332
pub fn i_069308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6930C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69332; return 10; }
    8
}
/// $06930C: move.w #$8, d0
pub fn i_06930c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69310;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069310: cmpi.w #$e, d0
pub fn i_069310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69314;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069314: ble.w $69324
pub fn i_069314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69318;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69324; return 10; }
    8
}
/// $069318: nop
pub fn i_069318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6931A;
    cpu.ir = 0x4E71;
    4
}
/// $06931A: nop
pub fn i_06931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6931C;
    cpu.ir = 0x4E71;
    4
}
/// $06931C: cmpi.w #$e, d0
pub fn i_06931c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69320;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069320: nop
pub fn i_069320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69322;
    cpu.ir = 0x4E71;
    4
}
/// $069322: trap #$f
pub fn i_069322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69324;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069324: movea.l $8c(a6), a1
pub fn i_069324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69328;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069328: lea.l $2c8bc2.l, a0
pub fn i_069328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6932E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $06932E: jsr $69d76(pc)
pub fn i_06932e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69332;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69332);
    cpu.pc = 0x69D76;
    16
}
/// $069332: move.w #$6, d0
pub fn i_069332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69336;
    cpu.ir = 0x303C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069336: cmpi.w #$a, d0
pub fn i_069336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6933A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $06933A: ble.w $6934a
pub fn i_06933a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6933E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6934A; return 10; }
    8
}
/// $06933E: nop
pub fn i_06933e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69340;
    cpu.ir = 0x4E71;
    4
}
/// $069340: nop
pub fn i_069340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69342;
    cpu.ir = 0x4E71;
    4
}
/// $069342: cmpi.w #$a, d0
pub fn i_069342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69346;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069346: nop
pub fn i_069346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69348;
    cpu.ir = 0x4E71;
    4
}
/// $069348: trap #$f
pub fn i_069348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6934A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06934A: movea.l $88(a6), a1
pub fn i_06934a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6934E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06934E: lea.l $2c8b96.l, a0
pub fn i_06934e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69354;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $069354: jsr $69d76(pc)
pub fn i_069354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69358;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69358);
    cpu.pc = 0x69D76;
    16
}
/// $069358: lea.l $2c94a8.l, a0
pub fn i_069358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6935E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C94A8;
    20
}
/// $06935E: jsr $28cd4.l
pub fn i_06935e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69364;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69364);
    cpu.pc = 0x28CD4;
    16
}
/// $069364: bra.w $693cc
pub fn i_069364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69368;
    cpu.ir = 0x6000;
    cpu.pc = 0x693CC;
    10
}
/// $069368: cmpi.l #$ffffffff, $8c(a6)
pub fn i_069368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69370;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $069370: beq.w $6939a
pub fn i_069370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69374;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6939A; return 10; }
    8
}
/// $069374: move.w #$9, d0
pub fn i_069374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69378;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069378: cmpi.w #$e, d0
pub fn i_069378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6937C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06937C: ble.w $6938c
pub fn i_06937c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69380;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6938C; return 10; }
    8
}
/// $069380: nop
pub fn i_069380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69382;
    cpu.ir = 0x4E71;
    4
}
/// $069382: nop
pub fn i_069382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69384;
    cpu.ir = 0x4E71;
    4
}
/// $069384: cmpi.w #$e, d0
pub fn i_069384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69388;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069388: nop
pub fn i_069388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6938A;
    cpu.ir = 0x4E71;
    4
}
/// $06938A: trap #$f
pub fn i_06938a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6938C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06938C: movea.l $8c(a6), a1
pub fn i_06938c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69390;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069390: lea.l $2c8bc2.l, a0
pub fn i_069390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69396;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069396: jsr $69d76(pc)
pub fn i_069396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6939A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6939A);
    cpu.pc = 0x69D76;
    16
}
/// $06939A: move.w #$6, d0
pub fn i_06939a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6939E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06939E: cmpi.w #$a, d0
pub fn i_06939e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693A2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $0693A2: ble.w $693b2
pub fn i_0693a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693A6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x693B2; return 10; }
    8
}
/// $0693A6: nop
pub fn i_0693a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693A8;
    cpu.ir = 0x4E71;
    4
}
/// $0693A8: nop
pub fn i_0693a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693AA;
    cpu.ir = 0x4E71;
    4
}
/// $0693AA: cmpi.w #$a, d0
pub fn i_0693aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693AE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $0693AE: nop
pub fn i_0693ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693B0;
    cpu.ir = 0x4E71;
    4
}
/// $0693B0: trap #$f
pub fn i_0693b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693B2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0693B2: movea.l $88(a6), a1
pub fn i_0693b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693B6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0693B6: lea.l $2c8b96.l, a0
pub fn i_0693b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $0693BC: jsr $69d76(pc)
pub fn i_0693bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x693C0);
    cpu.pc = 0x69D76;
    16
}
/// $0693C0: lea.l $2c961e.l, a0
pub fn i_0693c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C961E;
    20
}
/// $0693C6: jsr $28cd4.l
pub fn i_0693c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x693CC);
    cpu.pc = 0x28CD4;
    16
}
/// $0693CC: lea.l $693d2(pc), a1
pub fn i_0693cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x693D2;
    16
}
/// $0693D0: move.l a1, (a6)
pub fn i_0693d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0693D2: jsr $28998.l
pub fn i_0693d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x693D8);
    cpu.pc = 0x28998;
    16
}
/// $0693D8: jsr $2783a.l
pub fn i_0693d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x693DE);
    cpu.pc = 0x2783A;
    16
}
/// $0693DE: jsr $28d70.l
pub fn i_0693de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x693E4);
    cpu.pc = 0x28D70;
    16
}
/// $0693E4: bcc.w $693f8
pub fn i_0693e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x693F8; return 10; }
    8
}
/// $0693E8: cmpi.b #$0, $7a(a6)
pub fn i_0693e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693EE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0693EE: bgt.w $693f8
pub fn i_0693ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693F2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x693F8; return 10; }
    8
}
/// $0693F2: lea.l $6942e(pc), a1
pub fn i_0693f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6942E;
    16
}
/// $0693F6: move.l a1, (a6)
pub fn i_0693f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0693F8: cmpi.b #$0, $7a(a6)
pub fn i_0693f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x693FE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0693FE: ble.w $69424
pub fn i_0693fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69402;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69424; return 10; }
    8
}
/// $069402: subq.w #$1, $78(a6)
pub fn i_069402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69406;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069406: cmpi.w #$0, $78(a6)
pub fn i_069406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6940C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06940C: bgt.w $69424
pub fn i_06940c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69410;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x69424; return 10; }
    8
}
/// $069410: subq.b #$1, $7a(a6)
pub fn i_069410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69414;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $069414: cmpi.b #$0, $7a(a6)
pub fn i_069414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6941A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06941A: ble.w $69424
pub fn i_06941a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6941E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69424; return 10; }
    8
}
/// $06941E: lea.l $692e6(pc), a1
pub fn i_06941e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69422;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x692E6;
    16
}
/// $069422: move.l a1, (a6)
pub fn i_069422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69424;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069424: bclr.b #$3, $13(a6)
pub fn i_069424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6942A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06942A: bra.w $697da
pub fn i_06942a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6942E;
    cpu.ir = 0x6000;
    cpu.pc = 0x697DA;
    10
}
// ===== Tank_FireDone_06942e  ($06942E, 174 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06942E: move.b #$0, $7a(a6)
pub fn i_06942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69434;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $069434: lea.l $2b7f76.l, a0
pub fn i_069434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6943A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7F76;
    20
}
/// $06943A: jsr $799de.l
pub fn i_06943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69440);
    cpu.pc = 0x799DE;
    16
}
/// $069440: move.w d0, $76(a6)
pub fn i_069440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69444;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069444: cmpi.b #$0, $7b(a6)
pub fn i_069444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6944A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06944A: beq.w $68f3a
pub fn i_06944a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6944E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68F3A; return 10; }
    8
}
/// $06944E: clr.b $7b(a6)
pub fn i_06944e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69452;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $069452: cmpi.l #$ffffffff, $8c(a6)
pub fn i_069452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6945A;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $06945A: beq.w $69484
pub fn i_06945a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6945E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69484; return 10; }
    8
}
/// $06945E: move.w #$7, d0
pub fn i_06945e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69462;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069462: cmpi.w #$e, d0
pub fn i_069462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69466;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069466: ble.w $69476
pub fn i_069466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6946A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69476; return 10; }
    8
}
/// $06946A: nop
pub fn i_06946a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6946C;
    cpu.ir = 0x4E71;
    4
}
/// $06946C: nop
pub fn i_06946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6946E;
    cpu.ir = 0x4E71;
    4
}
/// $06946E: cmpi.w #$e, d0
pub fn i_06946e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69472;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069472: nop
pub fn i_069472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69474;
    cpu.ir = 0x4E71;
    4
}
/// $069474: trap #$f
pub fn i_069474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69476;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069476: movea.l $8c(a6), a1
pub fn i_069476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6947A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06947A: lea.l $2c8bc2.l, a0
pub fn i_06947a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69480;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069480: jsr $69d76(pc)
pub fn i_069480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69484;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69484);
    cpu.pc = 0x69D76;
    16
}
/// $069484: move.w #$0, d0
pub fn i_069484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69488;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069488: cmpi.w #$a, d0
pub fn i_069488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6948C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $06948C: ble.w $6949c
pub fn i_06948c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69490;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6949C; return 10; }
    8
}
/// $069490: nop
pub fn i_069490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69492;
    cpu.ir = 0x4E71;
    4
}
/// $069492: nop
pub fn i_069492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69494;
    cpu.ir = 0x4E71;
    4
}
/// $069494: cmpi.w #$a, d0
pub fn i_069494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69498;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069498: nop
pub fn i_069498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6949A;
    cpu.ir = 0x4E71;
    4
}
/// $06949A: trap #$f
pub fn i_06949a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6949C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06949C: movea.l $88(a6), a1
pub fn i_06949c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694A0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0694A0: lea.l $2c8b96.l, a0
pub fn i_0694a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $0694A6: jsr $69d76(pc)
pub fn i_0694a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x694AA);
    cpu.pc = 0x69D76;
    16
}
/// $0694AA: lea.l $2c93fa.l, a0
pub fn i_0694aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C93FA;
    20
}
/// $0694B0: jsr $28cd4.l
pub fn i_0694b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x694B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0694B6: lea.l $694bc(pc), a1
pub fn i_0694b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x694BC;
    16
}
/// $0694BA: move.l a1, (a6)
pub fn i_0694ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0694BC: jsr $28998.l
pub fn i_0694bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x694C2);
    cpu.pc = 0x28998;
    16
}
/// $0694C2: jsr $2783a.l
pub fn i_0694c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x694C8);
    cpu.pc = 0x2783A;
    16
}
/// $0694C8: jsr $28d70.l
pub fn i_0694c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x694CE);
    cpu.pc = 0x28D70;
    16
}
/// $0694CE: bcc.w $694d8
pub fn i_0694ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x694D8; return 10; }
    8
}
/// $0694D2: lea.l $68f3a(pc), a1
pub fn i_0694d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68F3A;
    16
}
/// $0694D6: move.l a1, (a6)
pub fn i_0694d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0694D8: bra.w $697da
pub fn i_0694d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x697DA;
    10
}
// ===== Tank_Die_0694dc  ($0694DC, 444 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0694DC: lea.l $77fd6.l, a1
pub fn i_0694dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0694E2: jsr $4ae.l
pub fn i_0694e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x694E8);
    cpu.pc = 0x4AE;
    16
}
/// $0694E8: jsr $5dd02.l
pub fn i_0694e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x694EE);
    cpu.pc = 0x5DD02;
    16
}
/// $0694EE: move.w #$20, d0
pub fn i_0694ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694F2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0694F2: jsr $69dbc(pc)
pub fn i_0694f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x694F6);
    cpu.pc = 0x69DBC;
    16
}
/// $0694F6: add.w d0, $22(a0)
pub fn i_0694f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694FA;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0694FA: cmpi.l #$ffffffff, $8c(a6)
pub fn i_0694fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69502;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $069502: beq.w $6952c
pub fn i_069502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69506;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6952C; return 10; }
    8
}
/// $069506: move.w #$d, d0
pub fn i_069506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6950A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06950A: cmpi.w #$e, d0
pub fn i_06950a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6950E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06950E: ble.w $6951e
pub fn i_06950e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69512;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6951E; return 10; }
    8
}
/// $069512: nop
pub fn i_069512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69514;
    cpu.ir = 0x4E71;
    4
}
/// $069514: nop
pub fn i_069514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69516;
    cpu.ir = 0x4E71;
    4
}
/// $069516: cmpi.w #$e, d0
pub fn i_069516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6951A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06951A: nop
pub fn i_06951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6951C;
    cpu.ir = 0x4E71;
    4
}
/// $06951C: trap #$f
pub fn i_06951c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6951E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06951E: movea.l $8c(a6), a1
pub fn i_06951e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69522;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069522: lea.l $2c8bc2.l, a0
pub fn i_069522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69528;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069528: jsr $69d76(pc)
pub fn i_069528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6952C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6952C);
    cpu.pc = 0x69D76;
    16
}
/// $06952C: move.w #$8, d0
pub fn i_06952c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69530;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069530: cmpi.w #$a, d0
pub fn i_069530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69534;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069534: ble.w $69544
pub fn i_069534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69538;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69544; return 10; }
    8
}
/// $069538: nop
pub fn i_069538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6953A;
    cpu.ir = 0x4E71;
    4
}
/// $06953A: nop
pub fn i_06953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6953C;
    cpu.ir = 0x4E71;
    4
}
/// $06953C: cmpi.w #$a, d0
pub fn i_06953c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69540;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069540: nop
pub fn i_069540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69542;
    cpu.ir = 0x4E71;
    4
}
/// $069542: trap #$f
pub fn i_069542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69544;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069544: movea.l $88(a6), a1
pub fn i_069544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69548;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069548: lea.l $2c8b96.l, a0
pub fn i_069548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6954E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $06954E: jsr $69d76(pc)
pub fn i_06954e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69552;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69552);
    cpu.pc = 0x69D76;
    16
}
/// $069552: lea.l $2c97ca.l, a0
pub fn i_069552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69558;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C97CA;
    20
}
/// $069558: jsr $28cd4.l
pub fn i_069558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6955E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6955E);
    cpu.pc = 0x28CD4;
    16
}
/// $06955E: bra.w $6962e
pub fn i_06955e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69562;
    cpu.ir = 0x6000;
    cpu.pc = 0x6962E;
    10
}
/// $069562: cmpi.l #$ffffffff, $8c(a6)
pub fn i_069562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6956A;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $06956A: beq.w $69594
pub fn i_06956a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6956E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69594; return 10; }
    8
}
/// $06956E: move.w #$a, d0
pub fn i_06956e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69572;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069572: cmpi.w #$e, d0
pub fn i_069572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69576;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069576: ble.w $69586
pub fn i_069576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6957A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69586; return 10; }
    8
}
/// $06957A: nop
pub fn i_06957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6957C;
    cpu.ir = 0x4E71;
    4
}
/// $06957C: nop
pub fn i_06957c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6957E;
    cpu.ir = 0x4E71;
    4
}
/// $06957E: cmpi.w #$e, d0
pub fn i_06957e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69582;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069582: nop
pub fn i_069582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69584;
    cpu.ir = 0x4E71;
    4
}
/// $069584: trap #$f
pub fn i_069584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69586;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069586: movea.l $8c(a6), a1
pub fn i_069586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6958A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06958A: lea.l $2c8bc2.l, a0
pub fn i_06958a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69590;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069590: jsr $69d76(pc)
pub fn i_069590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69594;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69594);
    cpu.pc = 0x69D76;
    16
}
/// $069594: move.w #$8, d0
pub fn i_069594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69598;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069598: cmpi.w #$a, d0
pub fn i_069598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6959C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $06959C: ble.w $695ac
pub fn i_06959c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695A0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x695AC; return 10; }
    8
}
/// $0695A0: nop
pub fn i_0695a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695A2;
    cpu.ir = 0x4E71;
    4
}
/// $0695A2: nop
pub fn i_0695a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695A4;
    cpu.ir = 0x4E71;
    4
}
/// $0695A4: cmpi.w #$a, d0
pub fn i_0695a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695A8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $0695A8: nop
pub fn i_0695a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695AA;
    cpu.ir = 0x4E71;
    4
}
/// $0695AA: trap #$f
pub fn i_0695aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695AC;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0695AC: movea.l $88(a6), a1
pub fn i_0695ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695B0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0695B0: lea.l $2c8b96.l, a0
pub fn i_0695b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $0695B6: jsr $69d76(pc)
pub fn i_0695b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x695BA);
    cpu.pc = 0x69D76;
    16
}
/// $0695BA: lea.l $2c9710.l, a0
pub fn i_0695ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9710;
    20
}
/// $0695C0: jsr $28cd4.l
pub fn i_0695c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x695C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0695C6: bra.w $6962e
pub fn i_0695c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6962E;
    10
}
/// $0695CA: cmpi.l #$ffffffff, $8c(a6)
pub fn i_0695ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695D2;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0695D2: beq.w $695fc
pub fn i_0695d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x695FC; return 10; }
    8
}
/// $0695D6: move.w #$b, d0
pub fn i_0695d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0695DA: cmpi.w #$e, d0
pub fn i_0695da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695DE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $0695DE: ble.w $695ee
pub fn i_0695de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695E2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x695EE; return 10; }
    8
}
/// $0695E2: nop
pub fn i_0695e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695E4;
    cpu.ir = 0x4E71;
    4
}
/// $0695E4: nop
pub fn i_0695e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695E6;
    cpu.ir = 0x4E71;
    4
}
/// $0695E6: cmpi.w #$e, d0
pub fn i_0695e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695EA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $0695EA: nop
pub fn i_0695ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695EC;
    cpu.ir = 0x4E71;
    4
}
/// $0695EC: trap #$f
pub fn i_0695ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695EE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0695EE: movea.l $8c(a6), a1
pub fn i_0695ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695F2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0695F2: lea.l $2c8bc2.l, a0
pub fn i_0695f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $0695F8: jsr $69d76(pc)
pub fn i_0695f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x695FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x695FC);
    cpu.pc = 0x69D76;
    16
}
/// $0695FC: move.w #$8, d0
pub fn i_0695fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69600;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069600: cmpi.w #$a, d0
pub fn i_069600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69604;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069604: ble.w $69614
pub fn i_069604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69608;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69614; return 10; }
    8
}
/// $069608: nop
pub fn i_069608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6960A;
    cpu.ir = 0x4E71;
    4
}
/// $06960A: nop
pub fn i_06960a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6960C;
    cpu.ir = 0x4E71;
    4
}
/// $06960C: cmpi.w #$a, d0
pub fn i_06960c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69610;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069610: nop
pub fn i_069610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69612;
    cpu.ir = 0x4E71;
    4
}
/// $069612: trap #$f
pub fn i_069612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69614;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069614: movea.l $88(a6), a1
pub fn i_069614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69618;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069618: lea.l $2c8b96.l, a0
pub fn i_069618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6961E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $06961E: jsr $69d76(pc)
pub fn i_06961e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69622;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69622);
    cpu.pc = 0x69D76;
    16
}
/// $069622: lea.l $2c97ca.l, a0
pub fn i_069622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69628;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C97CA;
    20
}
/// $069628: jsr $28cd4.l
pub fn i_069628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6962E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6962E);
    cpu.pc = 0x28CD4;
    16
}
/// $06962E: bclr.b #$3, $13(a6)
pub fn i_06962e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69634;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069634: move.w #$109e, d0
pub fn i_069634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69638;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069638: jsr $2222.l
pub fn i_069638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6963E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6963E);
    cpu.pc = 0x2222;
    16
}
/// $06963E: lea.l $69644(pc), a1
pub fn i_06963e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69642;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69644;
    16
}
/// $069642: move.l a1, (a6)
pub fn i_069642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69644;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069644: jsr $28998.l
pub fn i_069644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6964A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6964A);
    cpu.pc = 0x28998;
    16
}
/// $06964A: jsr $2783a.l
pub fn i_06964a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69650);
    cpu.pc = 0x2783A;
    16
}
/// $069650: jsr $28d70.l
pub fn i_069650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69656);
    cpu.pc = 0x28D70;
    16
}
/// $069656: bcc.w $69660
pub fn i_069656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6965A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69660; return 10; }
    8
}
/// $06965A: lea.l $690ea(pc), a1
pub fn i_06965a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6965E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x690EA;
    16
}
/// $06965E: move.l a1, (a6)
pub fn i_06965e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69660;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069660: jsr $69ee8(pc)
pub fn i_069660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69664;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69664);
    cpu.pc = 0x69EE8;
    16
}
/// $069664: jsr $69e14(pc)
pub fn i_069664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69668;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69668);
    cpu.pc = 0x69E14;
    16
}
/// $069668: jsr $27eba.l
pub fn i_069668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6966E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6966E);
    cpu.pc = 0x27EBA;
    16
}
/// $06966E: bcc.w $69678
pub fn i_06966e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69672;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69678; return 10; }
    8
}
/// $069672: lea.l $68d4c(pc), a1
pub fn i_069672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69676;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68D4C;
    16
}
/// $069676: move.l a1, (a6)
pub fn i_069676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69678;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069678: jsr $2870a.l
pub fn i_069678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6967E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6967E);
    cpu.pc = 0x2870A;
    16
}
/// $06967E: bcc.w $69694
pub fn i_06967e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69682;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69694; return 10; }
    8
}
/// $069682: bclr.b #$3, $13(a6)
pub fn i_069682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69688;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069688: lea.l $5e766.l, a0
pub fn i_069688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6968E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06968E: jsr $5e770.l
pub fn i_06968e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69694);
    cpu.pc = 0x5E770;
    16
}
/// $069694: bra.w $6982c
pub fn i_069694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69698;
    cpu.ir = 0x6000;
    cpu.pc = 0x6982C;
    10
}
// ===== Tank_Wreck_069698  ($069698, 302 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069698: lea.l $698aa(pc), a1
pub fn i_069698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6969C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x698AA;
    16
}
/// $06969C: move.l $88(a6), d0
pub fn i_06969c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696A0;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0696A0: jsr $69da4(pc)
pub fn i_0696a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x696A4);
    cpu.pc = 0x69DA4;
    16
}
/// $0696A4: cmpi.l #$ffffffff, $8c(a6)
pub fn i_0696a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696AC;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0696AC: beq.w $696d6
pub fn i_0696ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x696D6; return 10; }
    8
}
/// $0696B0: move.w #$c, d0
pub fn i_0696b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0696B4: cmpi.w #$e, d0
pub fn i_0696b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696B8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $0696B8: ble.w $696c8
pub fn i_0696b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696BC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x696C8; return 10; }
    8
}
/// $0696BC: nop
pub fn i_0696bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696BE;
    cpu.ir = 0x4E71;
    4
}
/// $0696BE: nop
pub fn i_0696be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696C0;
    cpu.ir = 0x4E71;
    4
}
/// $0696C0: cmpi.w #$e, d0
pub fn i_0696c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696C4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $0696C4: nop
pub fn i_0696c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696C6;
    cpu.ir = 0x4E71;
    4
}
/// $0696C6: trap #$f
pub fn i_0696c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696C8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0696C8: movea.l $8c(a6), a1
pub fn i_0696c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696CC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0696CC: lea.l $2c8bc2.l, a0
pub fn i_0696cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $0696D2: jsr $69d76(pc)
pub fn i_0696d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x696D6);
    cpu.pc = 0x69D76;
    16
}
/// $0696D6: move.w #$a, d0
pub fn i_0696d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0696DA: cmpi.w #$a, d0
pub fn i_0696da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696DE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $0696DE: ble.w $696ee
pub fn i_0696de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696E2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x696EE; return 10; }
    8
}
/// $0696E2: nop
pub fn i_0696e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696E4;
    cpu.ir = 0x4E71;
    4
}
/// $0696E4: nop
pub fn i_0696e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696E6;
    cpu.ir = 0x4E71;
    4
}
/// $0696E6: cmpi.w #$a, d0
pub fn i_0696e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696EA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $0696EA: nop
pub fn i_0696ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696EC;
    cpu.ir = 0x4E71;
    4
}
/// $0696EC: trap #$f
pub fn i_0696ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696EE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0696EE: movea.l $88(a6), a1
pub fn i_0696ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696F2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0696F2: lea.l $2c8b96.l, a0
pub fn i_0696f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $0696F8: jsr $69d76(pc)
pub fn i_0696f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x696FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x696FC);
    cpu.pc = 0x69D76;
    16
}
/// $0696FC: lea.l $2c9938.l, a0
pub fn i_0696fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69702;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9938;
    20
}
/// $069702: jsr $28cd4.l
pub fn i_069702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69708);
    cpu.pc = 0x28CD4;
    16
}
/// $069708: bra.w $6977c
pub fn i_069708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6970C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6977C;
    10
}
/// $06970C: lea.l $69a18(pc), a1
pub fn i_06970c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69710;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69A18;
    16
}
/// $069710: move.l $88(a6), d0
pub fn i_069710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69714;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069714: jsr $69da4(pc)
pub fn i_069714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69718;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69718);
    cpu.pc = 0x69DA4;
    16
}
/// $069718: cmpi.l #$ffffffff, $8c(a6)
pub fn i_069718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69720;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $069720: beq.w $6974a
pub fn i_069720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69724;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6974A; return 10; }
    8
}
/// $069724: move.w #$d, d0
pub fn i_069724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69728;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069728: cmpi.w #$e, d0
pub fn i_069728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6972C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $06972C: ble.w $6973c
pub fn i_06972c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69730;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6973C; return 10; }
    8
}
/// $069730: nop
pub fn i_069730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69732;
    cpu.ir = 0x4E71;
    4
}
/// $069732: nop
pub fn i_069732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69734;
    cpu.ir = 0x4E71;
    4
}
/// $069734: cmpi.w #$e, d0
pub fn i_069734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69738;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $069738: nop
pub fn i_069738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6973A;
    cpu.ir = 0x4E71;
    4
}
/// $06973A: trap #$f
pub fn i_06973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6973C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06973C: movea.l $8c(a6), a1
pub fn i_06973c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69740;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069740: lea.l $2c8bc2.l, a0
pub fn i_069740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69746;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BC2;
    20
}
/// $069746: jsr $69d76(pc)
pub fn i_069746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6974A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6974A);
    cpu.pc = 0x69D76;
    16
}
/// $06974A: move.w #$a, d0
pub fn i_06974a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6974E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06974E: cmpi.w #$a, d0
pub fn i_06974e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69752;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $069752: ble.w $69762
pub fn i_069752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69756;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69762; return 10; }
    8
}
/// $069756: nop
pub fn i_069756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69758;
    cpu.ir = 0x4E71;
    4
}
/// $069758: nop
pub fn i_069758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6975A;
    cpu.ir = 0x4E71;
    4
}
/// $06975A: cmpi.w #$a, d0
pub fn i_06975a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6975E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $06975E: nop
pub fn i_06975e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69760;
    cpu.ir = 0x4E71;
    4
}
/// $069760: trap #$f
pub fn i_069760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69762;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $069762: movea.l $88(a6), a1
pub fn i_069762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69766;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $069766: lea.l $2c8b96.l, a0
pub fn i_069766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6976C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B96;
    20
}
/// $06976C: jsr $69d76(pc)
pub fn i_06976c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69770;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69770);
    cpu.pc = 0x69D76;
    16
}
/// $069770: lea.l $2c99de.l, a0
pub fn i_069770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69776;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C99DE;
    20
}
/// $069776: jsr $28cd4.l
pub fn i_069776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6977C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6977C);
    cpu.pc = 0x28CD4;
    16
}
/// $06977C: move.w #$0, $94(a6)
pub fn i_06977c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69782;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069782: bclr.b #$1, $12(a6)
pub fn i_069782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69788;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069788: lea.l $6978e(pc), a1
pub fn i_069788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6978C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6978E;
    16
}
/// $06978C: move.l a1, (a6)
pub fn i_06978c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6978E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06978E: jsr $2783a.l
pub fn i_06978e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69794;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69794);
    cpu.pc = 0x2783A;
    16
}
/// $069794: jsr $69e90(pc)
pub fn i_069794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69798;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69798);
    cpu.pc = 0x69E90;
    16
}
/// $069798: jsr $28d70.l
pub fn i_069798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6979E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6979E);
    cpu.pc = 0x28D70;
    16
}
/// $06979E: bcc.w $697b0
pub fn i_06979e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x697B0; return 10; }
    8
}
/// $0697A2: tst.b $7d(a6)
pub fn i_0697a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697A6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0697A6: beq.w $697b0
pub fn i_0697a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x697B0; return 10; }
    8
}
/// $0697AA: lea.l $697ce(pc), a1
pub fn i_0697aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x697CE;
    16
}
/// $0697AE: move.l a1, (a6)
pub fn i_0697ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0697B0: movea.l #$ffffffff, a0
pub fn i_0697b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697B6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0697B6: lea.l $2c8b6c.l, a0
pub fn i_0697b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B6C;
    20
}
/// $0697BC: jsr $5dd5c.l
pub fn i_0697bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x697C2);
    cpu.pc = 0x5DD5C;
    16
}
/// $0697C2: bcc.w $697cc
pub fn i_0697c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x697CC; return 10; }
    8
}
// ===== SetTaskHandler_0697c6  ($0697C6, 8 B, kernel/task_handlers.c)
/// $0697C6: lea.l $6986a(pc), a1
pub fn i_0697c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6986A;
    16
}
/// $0697CA: move.l a1, (a6)
pub fn i_0697ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0697CC: rts
pub fn i_0697cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_WreckBlast_0697ce  ($0697CE, 10 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0697CE: jsr $69f12(pc)
pub fn i_0697ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x697D2);
    cpu.pc = 0x69F12;
    16
}
/// $0697D2: jmp $518.l
pub fn i_0697d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697D8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0697d8  ($0697D8, 2 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0697D8: rts
pub fn i_0697d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_Tail_0697da  ($0697DA, 136 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0697DA: jsr $27eba.l
pub fn i_0697da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x697E0);
    cpu.pc = 0x27EBA;
    16
}
/// $0697E0: bcc.w $697ea
pub fn i_0697e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x697EA; return 10; }
    8
}
/// $0697E4: lea.l $68d4c(pc), a1
pub fn i_0697e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68D4C;
    16
}
/// $0697E8: move.l a1, (a6)
pub fn i_0697e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0697EA: jsr $69ee8(pc)
pub fn i_0697ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x697EE);
    cpu.pc = 0x69EE8;
    16
}
/// $0697EE: jsr $2870a.l
pub fn i_0697ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x697F4);
    cpu.pc = 0x2870A;
    16
}
/// $0697F4: bcc.w $6982c
pub fn i_0697f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6982C; return 10; }
    8
}
/// $0697F8: lea.l $5e766.l, a0
pub fn i_0697f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x697FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0697FE: jsr $5e770.l
pub fn i_0697fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69804);
    cpu.pc = 0x5E770;
    16
}
/// $069804: bclr.b #$3, $13(a6)
pub fn i_069804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6980A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06980A: lea.l $5e766.l, a0
pub fn i_06980a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69810;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $069810: jsr $5e770.l
pub fn i_069810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69816);
    cpu.pc = 0x5E770;
    16
}
/// $069816: lea.l $69562(pc), a1
pub fn i_069816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6981A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69562;
    16
}
/// $06981A: move.l a1, (a6)
pub fn i_06981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6981C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06981C: jsr $5e844.l
pub fn i_06981c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69822);
    cpu.pc = 0x5E844;
    16
}
/// $069822: bcc.w $6982c
pub fn i_069822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69826;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6982C; return 10; }
    8
}
/// $069826: lea.l $695ca(pc), a1
pub fn i_069826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6982A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x695CA;
    16
}
/// $06982A: move.l a1, (a6)
pub fn i_06982a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6982C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06982C: jsr $28758.l
pub fn i_06982c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69832);
    cpu.pc = 0x28758;
    16
}
/// $069832: bcc.w $6984c
pub fn i_069832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69836;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6984C; return 10; }
    8
}
/// $069836: lea.l $69698(pc), a1
pub fn i_069836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6983A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69698;
    16
}
/// $06983A: move.l a1, (a6)
pub fn i_06983a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6983C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06983C: jsr $5e844.l
pub fn i_06983c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69842;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69842);
    cpu.pc = 0x5E844;
    16
}
/// $069842: bcc.w $6984c
pub fn i_069842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69846;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6984C; return 10; }
    8
}
/// $069846: lea.l $6970c(pc), a1
pub fn i_069846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6984A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6970C;
    16
}
/// $06984A: move.l a1, (a6)
pub fn i_06984a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6984C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06984C: movea.l #$ffffffff, a0
pub fn i_06984c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69852;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $069852: lea.l $2c8b6c.l, a0
pub fn i_069852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69858;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B6C;
    20
}
/// $069858: jsr $5dd5c.l
pub fn i_069858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6985E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6985E);
    cpu.pc = 0x5DD5C;
    16
}
/// $06985E: bcc.w $69868
pub fn i_06985e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69862;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69868; return 10; }
    8
}
// ===== SetTaskHandler_069862  ($069862, 8 B, kernel/task_handlers.c)
/// $069862: lea.l $6986a(pc), a1
pub fn i_069862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6986A;
    16
}
/// $069866: move.l a1, (a6)
pub fn i_069866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069868: rts
pub fn i_069868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6986A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06986a  ($06986A, 8 B, kernel/jmp_to_scheduler.c)
/// $06986A: jmp $518.l
pub fn i_06986a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69870;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $069870: rts
pub fn i_069870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69872;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_069872  ($069872, 14 B, kernel/jsr5b6_jmp518.c)
/// $069872: jsr $5b6.l
pub fn i_069872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69878);
    cpu.pc = 0x5B6;
    16
}
/// $069878: jmp $518.l
pub fn i_069878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6987E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06987E: rts
pub fn i_06987e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69880;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_Driver_069880  ($069880, 34 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069880: move.w #$1d, d1
pub fn i_069880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69884;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069884: jsr $236e.l
pub fn i_069884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6988A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6988A);
    cpu.pc = 0x236E;
    16
}
/// $06988A: lea.l $69890(pc), a1
pub fn i_06988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6988E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69890;
    16
}
/// $06988E: move.l a1, (a6)
pub fn i_06988e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69890;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069890: jsr $69ebe(pc)
pub fn i_069890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69894;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69894);
    cpu.pc = 0x69EBE;
    16
}
/// $069894: jsr $69ed4(pc)
pub fn i_069894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69898;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69898);
    cpu.pc = 0x69ED4;
    16
}
/// $069898: movea.l $c(a6), a0
pub fn i_069898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6989C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06989C: move.l $5c(a0), $5c(a6)
pub fn i_06989c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698A2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
// ===== JsrAbsThunk_0698a2  ($0698A2, 8 B, thunks/jsr_abs_thunks.c)
/// $0698A2: jsr $28d70.l
pub fn i_0698a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x698A8);
    cpu.pc = 0x28D70;
    16
}
/// $0698A8: rts
pub fn i_0698a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_DriverBail_0698aa  ($0698AA, 114 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0698AA: move.b $98(a6), d0
pub fn i_0698aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698AE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0698AE: cmpi.b #$2, d0
pub fn i_0698ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698B2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $0698B2: blt.w $698bc
pub fn i_0698b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698B6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x698BC; return 10; }
    8
}
/// $0698B6: jsr $5e9b6.l
pub fn i_0698b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x698BC);
    cpu.pc = 0x5E9B6;
    16
}
/// $0698BC: andi.b #$1, d0
pub fn i_0698bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698C0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0698C0: move.b d0, $98(a6)
pub fn i_0698c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698C4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0698C4: cmpi.b #$0, d0
pub fn i_0698c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698C8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0698C8: bne.w $698dc
pub fn i_0698c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x698DC; return 10; }
    8
}
/// $0698CC: lea.l $2c9b2c.l, a0
pub fn i_0698cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9B2C;
    20
}
/// $0698D2: jsr $28cd4.l
pub fn i_0698d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x698D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0698D8: bra.w $698e8
pub fn i_0698d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x698E8;
    10
}
/// $0698DC: lea.l $2c9b88.l, a0
pub fn i_0698dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9B88;
    20
}
/// $0698E2: jsr $28cd4.l
pub fn i_0698e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x698E8);
    cpu.pc = 0x28CD4;
    16
}
/// $0698E8: lea.l $698ee(pc), a1
pub fn i_0698e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x698EE;
    16
}
/// $0698EC: move.l a1, (a6)
pub fn i_0698ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0698EE: jsr $69ebe(pc)
pub fn i_0698ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x698F2);
    cpu.pc = 0x69EBE;
    16
}
/// $0698F2: jsr $69ed4(pc)
pub fn i_0698f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x698F6);
    cpu.pc = 0x69ED4;
    16
}
/// $0698F6: move.b #$0, $44(a6)
pub fn i_0698f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0698FC: jsr $28d70.l
pub fn i_0698fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69902;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69902);
    cpu.pc = 0x28D70;
    16
}
/// $069902: bcc.w $6990c
pub fn i_069902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69906;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6990C; return 10; }
    8
}
/// $069906: lea.l $699b2(pc), a1
pub fn i_069906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6990A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x699B2;
    16
}
/// $06990A: move.l a1, (a6)
pub fn i_06990a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6990C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06990C: jsr $2870a.l
pub fn i_06990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69912);
    cpu.pc = 0x2870A;
    16
}
/// $069912: bcc.w $69922
pub fn i_069912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69916;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69922; return 10; }
    8
}
/// $069916: lea.l $69924(pc), a1
pub fn i_069916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6991A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69924;
    16
}
/// $06991A: move.l a1, (a6)
pub fn i_06991a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6991C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_06991c  ($06991C, 8 B, thunks/jsr_abs_thunks.c)
/// $06991C: jsr $519be.l
pub fn i_06991c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69922);
    cpu.pc = 0x519BE;
    16
}
/// $069922: rts
pub fn i_069922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69924;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_DriverHit_069924  ($069924, 58 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069924: move.w #$0, $72(a6)
pub fn i_069924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6992A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06992A: lea.l $2c997c.l, a0
pub fn i_06992a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69930;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C997C;
    20
}
/// $069930: jsr $28cd4.l
pub fn i_069930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69936);
    cpu.pc = 0x28CD4;
    16
}
/// $069936: lea.l $6993c(pc), a1
pub fn i_069936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6993A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6993C;
    16
}
/// $06993A: move.l a1, (a6)
pub fn i_06993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6993C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06993C: jsr $69ebe(pc)
pub fn i_06993c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69940;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69940);
    cpu.pc = 0x69EBE;
    16
}
/// $069940: jsr $69ed4(pc)
pub fn i_069940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69944;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69944);
    cpu.pc = 0x69ED4;
    16
}
/// $069944: movea.l $c(a6), a0
pub fn i_069944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69948;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $069948: move.b #$0, $44(a6)
pub fn i_069948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6994E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06994E: cmpi.w #$0, $94(a0)
pub fn i_06994e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69954;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $069954: bne.w $6995e
pub fn i_069954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69958;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6995E; return 10; }
    8
}
/// $069958: lea.l $69966(pc), a1
pub fn i_069958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6995C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69966;
    16
}
/// $06995C: move.l a1, (a6)
pub fn i_06995c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6995E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_06995e  ($06995E, 8 B, thunks/jsr_abs_thunks.c)
/// $06995E: jsr $28d70.l
pub fn i_06995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69964);
    cpu.pc = 0x28D70;
    16
}
/// $069964: rts
pub fn i_069964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69966;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_DriverDie_069966  ($069966, 68 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069966: jsr $4a0d4.l
pub fn i_069966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6996C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6996C);
    cpu.pc = 0x4A0D4;
    16
}
/// $06996C: jsr $267e2.l
pub fn i_06996c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69972);
    cpu.pc = 0x267E2;
    16
}
/// $069972: lea.l $4acfe.l, a0
pub fn i_069972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69978;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x4ACFE;
    20
}
/// $069978: jsr $28cd4.l
pub fn i_069978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6997E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6997E);
    cpu.pc = 0x28CD4;
    16
}
/// $06997E: lea.l $69984(pc), a1
pub fn i_06997e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69982;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69984;
    16
}
/// $069982: move.l a1, (a6)
pub fn i_069982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69984;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069984: jsr $2783a.l
pub fn i_069984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6998A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6998A);
    cpu.pc = 0x2783A;
    16
}
/// $06998A: jsr $28d70.l
pub fn i_06998a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69990);
    cpu.pc = 0x28D70;
    16
}
/// $069990: bcc.w $6999a
pub fn i_069990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69994;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6999A; return 10; }
    8
}
/// $069994: lea.l $69872(pc), a1
pub fn i_069994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69998;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69872;
    16
}
/// $069998: move.l a1, (a6)
pub fn i_069998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6999A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06999A: movea.l #$ffffffff, a0
pub fn i_06999a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699A0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0699A0: jsr $5dd56.l
pub fn i_0699a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x699A6);
    cpu.pc = 0x5DD56;
    16
}
/// $0699A6: bcc.w $699b0
pub fn i_0699a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x699B0; return 10; }
    8
}
// ===== SetTaskHandler_0699aa  ($0699AA, 8 B, kernel/task_handlers.c)
/// $0699AA: lea.l $69872(pc), a1
pub fn i_0699aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69872;
    16
}
/// $0699AE: move.l a1, (a6)
pub fn i_0699ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0699B0: rts
pub fn i_0699b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_DriverToSoldier_0699b2  ($0699B2, 102 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0699B2: move.w #$8000, d0
pub fn i_0699b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0699B6: jsr $28134.l
pub fn i_0699b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x699BC);
    cpu.pc = 0x28134;
    16
}
/// $0699BC: andi.w #$ffe3, $38(a6)
pub fn i_0699bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699C2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0699C2: ori.w #$18, $38(a6)
pub fn i_0699c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699C8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0699C8: addq.w #$8, $24(a6)
pub fn i_0699c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699CC;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0699CC: cmpi.b #$0, $98(a6)
pub fn i_0699cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699D2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0699D2: bne.w $69a08
pub fn i_0699d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x69A08; return 10; }
    8
}
/// $0699D6: eori.b #$1, $3a(a6)
pub fn i_0699d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699DC;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0699DC: move.w #$ff78, d0
pub fn i_0699dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0699E0: jsr $5dca4.l
pub fn i_0699e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x699E6);
    cpu.pc = 0x5DCA4;
    16
}
/// $0699E6: move.w d0, $28(a6)
pub fn i_0699e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699EA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0699EA: move.w #$21c, $2a(a6)
pub fn i_0699ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x21C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0699F0: move.w #$ffb8, $2e(a6)
pub fn i_0699f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0699F6: move.w #$0, $2c(a6)
pub fn i_0699f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x699FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0699FC: jsr $4a0d4.l
pub fn i_0699fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A02);
    cpu.pc = 0x4A0D4;
    16
}
/// $069A02: jmp $58f82.l
pub fn i_069a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A08;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x58F82;
    8
}
/// $069A08: jsr $4a0d4.l
pub fn i_069a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A0E);
    cpu.pc = 0x4A0D4;
    16
}
/// $069A0E: addq.w #$4, $24(a6)
pub fn i_069a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A12;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069A12: jmp $5724e.l
pub fn i_069a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A18;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5724E;
    8
}
// ===== Tank_DriverWreck_069a18  ($069A18, 26 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069A18: jsr $69ebe(pc)
pub fn i_069a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69A1C);
    cpu.pc = 0x69EBE;
    16
}
/// $069A1C: jsr $69ed4(pc)
pub fn i_069a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69A20);
    cpu.pc = 0x69ED4;
    16
}
/// $069A20: jsr $28d70.l
pub fn i_069a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A26);
    cpu.pc = 0x28D70;
    16
}
/// $069A26: jsr $4a166.l
pub fn i_069a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A2C);
    cpu.pc = 0x4A166;
    16
}
/// $069A2C: addq.w #$2, $38(a6)
pub fn i_069a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A30;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069A30: rts
pub fn i_069a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_Turret_069a32  ($069A32, 174 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069A32: jsr $69d5e(pc)
pub fn i_069a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69A36);
    cpu.pc = 0x69D5E;
    16
}
/// $069A36: move.w #$3, $1c(a6)
pub fn i_069a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A3C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069A3C: jsr $138fe.l
pub fn i_069a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A42);
    cpu.pc = 0x138FE;
    16
}
/// $069A42: lea.l $2b7df0.l, a0
pub fn i_069a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7DF0;
    20
}
/// $069A48: jsr $799de.l
pub fn i_069a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A4E);
    cpu.pc = 0x799DE;
    16
}
/// $069A4E: move.w d0, $66(a6)
pub fn i_069a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A52;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069A52: lea.l $69a58(pc), a1
pub fn i_069a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69A58;
    16
}
/// $069A56: move.l a1, (a6)
pub fn i_069a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069A58: jsr $69eaa(pc)
pub fn i_069a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69A5C);
    cpu.pc = 0x69EAA;
    16
}
/// $069A5C: jsr $28d70.l
pub fn i_069a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A62);
    cpu.pc = 0x28D70;
    16
}
/// $069A62: jsr $283ca.l
pub fn i_069a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A68);
    cpu.pc = 0x283CA;
    16
}
/// $069A68: jsr $283d8.l
pub fn i_069a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A6E);
    cpu.pc = 0x283D8;
    16
}
/// $069A6E: jsr $2870a.l
pub fn i_069a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A74);
    cpu.pc = 0x2870A;
    16
}
/// $069A74: bcc.w $69a90
pub fn i_069a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69A90; return 10; }
    8
}
/// $069A78: lea.l $5e766.l, a0
pub fn i_069a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $069A7E: jsr $5e770.l
pub fn i_069a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69A84);
    cpu.pc = 0x5E770;
    16
}
/// $069A84: bclr.b #$3, $13(a6)
pub fn i_069a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A8A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069A8A: bclr.b #$0, $13(a6)
pub fn i_069a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069A90: cmpi.b #$33, $20(a6)
pub fn i_069a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A96;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x33, Size::Byte, false);
    16
}
/// $069A96: beq.w $69ac6
pub fn i_069a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69A9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69AC6; return 10; }
    8
}
/// $069A9A: cmpi.b #$77, $20(a6)
pub fn i_069a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AA0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x77, Size::Byte, false);
    16
}
/// $069AA0: beq.w $69ac6
pub fn i_069aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69AC6; return 10; }
    8
}
/// $069AA4: cmpi.w #$0, $66(a6)
pub fn i_069aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AAA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $069AAA: bgt.w $69ac6
pub fn i_069aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AAE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x69AC6; return 10; }
    8
}
/// $069AAE: move.b #$33, $20(a6)
pub fn i_069aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AB4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $069AB4: move.w #$102d, d0
pub fn i_069ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AB8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069AB8: jsr $2352.l
pub fn i_069ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69ABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69ABE);
    cpu.pc = 0x2352;
    16
}
/// $069ABE: lea.l $694dc(pc), a1
pub fn i_069abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x694DC;
    16
}
/// $069AC2: jsr $69db4(pc)
pub fn i_069ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69AC6);
    cpu.pc = 0x69DB4;
    16
}
/// $069AC6: cmpi.b #$77, $20(a6)
pub fn i_069ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69ACC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x77, Size::Byte, false);
    16
}
/// $069ACC: bne.w $69ad6
pub fn i_069acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x69AD6; return 10; }
    8
}
/// $069AD0: lea.l $69ae8(pc), a1
pub fn i_069ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69AE8;
    16
}
/// $069AD4: move.l a1, (a6)
pub fn i_069ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069AD6: jsr $5e45a.l
pub fn i_069ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69ADC);
    cpu.pc = 0x5E45A;
    16
}
/// $069ADC: bcc.w $69ae6
pub fn i_069adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69AE6; return 10; }
    8
}
// ===== SetTaskHandler_069ae0  ($069AE0, 8 B, kernel/task_handlers.c)
/// $069AE0: lea.l $69872(pc), a1
pub fn i_069ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69872;
    16
}
/// $069AE4: move.l a1, (a6)
pub fn i_069ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069AE6: rts
pub fn i_069ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_TurretFree_069ae8  ($069AE8, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069AE8: movea.l $c(a6), a0
pub fn i_069ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $069AEC: move.l #$ffffffff, $8c(a0)
pub fn i_069aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AF4;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069AF4: bra.w $69872
pub fn i_069af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x69872;
    10
}
// ===== Tank_Debris_069af8  ($069AF8, 72 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069AF8: lea.l $2c8bfe.l, a0
pub fn i_069af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69AFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8BFE;
    20
}
/// $069AFE: bra.w $69b08
pub fn i_069afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B02;
    cpu.ir = 0x6000;
    cpu.pc = 0x69B08;
    10
}
/// $069B02: lea.l $2c8c16.l, a0
pub fn i_069b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8C16;
    20
}
/// $069B08: move.l a0, $5c(a6)
pub fn i_069b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B0C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069B0C: lea.l $2b7e72.l, a0
pub fn i_069b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7E72;
    20
}
/// $069B12: jsr $799de.l
pub fn i_069b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B18);
    cpu.pc = 0x799DE;
    16
}
/// $069B18: movea.l $5c(a6), a0
pub fn i_069b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B1C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $069B1C: andi.w #$3, d0
pub fn i_069b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B20;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $069B20: move.w d0, d1
pub fn i_069b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B22;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069B22: add.w d1, d1
pub fn i_069b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B24;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $069B24: add.w d1, d0
pub fn i_069b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B26;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $069B26: lsl.w #$1, d0
pub fn i_069b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B28;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $069B28: move.w (a0, d0.w), $28(a6)
pub fn i_069b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B2E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    22
}
/// $069B2E: move.w $2(a0, d0.w), $2e(a6)
pub fn i_069b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B34;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $069B34: move.w $4(a0, d0.w), $2a(a6)
pub fn i_069b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B3A;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $069B3A: jmp $6dbd4.l
pub fn i_069b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B40;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DBD4;
    8
}
// ===== Tank_Smoke_069b40  ($069B40, 98 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069B40: move.w #$8, d1
pub fn i_069b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069B44: jsr $236e.l
pub fn i_069b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B4A);
    cpu.pc = 0x236E;
    16
}
/// $069B4A: lea.l $2c9bbe.l, a0
pub fn i_069b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9BBE;
    20
}
/// $069B50: jsr $28cd4.l
pub fn i_069b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B56);
    cpu.pc = 0x28CD4;
    16
}
/// $069B56: bra.w $69b76
pub fn i_069b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x69B76;
    10
}
/// $069B5A: eori.b #$1, $3a(a6)
pub fn i_069b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B60;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069B60: move.w #$8, d1
pub fn i_069b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B64;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069B64: jsr $236e.l
pub fn i_069b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B6A);
    cpu.pc = 0x236E;
    16
}
/// $069B6A: lea.l $2c9c60.l, a0
pub fn i_069b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9C60;
    20
}
/// $069B70: jsr $28cd4.l
pub fn i_069b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B76);
    cpu.pc = 0x28CD4;
    16
}
/// $069B76: move.w #$4000, d0
pub fn i_069b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B7A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069B7A: jsr $28134.l
pub fn i_069b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B80);
    cpu.pc = 0x28134;
    16
}
/// $069B80: andi.w #$ffe3, $38(a6)
pub fn i_069b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B86;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069B86: ori.w #$1c, $38(a6)
pub fn i_069b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B8C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069B8C: lea.l $69b92(pc), a1
pub fn i_069b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69B92;
    16
}
/// $069B90: move.l a1, (a6)
pub fn i_069b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069B92: jsr $2783a.l
pub fn i_069b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B98);
    cpu.pc = 0x2783A;
    16
}
/// $069B98: jsr $28d70.l
pub fn i_069b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69B9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69B9E);
    cpu.pc = 0x28D70;
    16
}
/// $069B9E: bcc.w $69ba8
pub fn i_069b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69BA8; return 10; }
    8
}
// ===== SetTaskHandler_069ba2  ($069BA2, 8 B, kernel/task_handlers.c)
/// $069BA2: lea.l $69872(pc), a1
pub fn i_069ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69872;
    16
}
/// $069BA6: move.l a1, (a6)
pub fn i_069ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069BA8: rts
pub fn i_069ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SmokeExplode_069baa  ($069BAA, 36 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069BAA: move.w #$4000, d0
pub fn i_069baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069BAE: jsr $28134.l
pub fn i_069bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69BB4);
    cpu.pc = 0x28134;
    16
}
/// $069BB4: andi.w #$ffe3, $38(a6)
pub fn i_069bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BBA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069BBA: ori.w #$0, $38(a6)
pub fn i_069bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BC0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069BC0: move.l #$ffffffff, $48(a6)
pub fn i_069bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BC8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069BC8: jmp $77efe.l
pub fn i_069bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77EFE;
    8
}
// ===== Tank_Missile_069bce  ($069BCE, 142 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069BCE: bset.b #$4, $6b(a6)
pub fn i_069bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BD4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069BD4: move.w #$23, d1
pub fn i_069bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069BD8: jsr $236e.l
pub fn i_069bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69BDE);
    cpu.pc = 0x236E;
    16
}
/// $069BDE: move.w #$c, $82(a6)
pub fn i_069bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069BE4: move.w #$5, $66(a6)
pub fn i_069be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069BEA: move.w #$d000, d0
pub fn i_069bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BEE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069BEE: jsr $28134.l
pub fn i_069bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69BF4);
    cpu.pc = 0x28134;
    16
}
/// $069BF4: andi.w #$ffe3, $38(a6)
pub fn i_069bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69BFA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069BFA: ori.w #$14, $38(a6)
pub fn i_069bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C00;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069C00: lea.l $2b7ef4.l, a0
pub fn i_069c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7EF4;
    20
}
/// $069C06: jsr $799de.l
pub fn i_069c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69C0C);
    cpu.pc = 0x799DE;
    16
}
/// $069C0C: move.w d0, $36(a6)
pub fn i_069c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C10;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069C10: lsr.w #$8, d0
pub fn i_069c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C12;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $069C12: lea.l $2c8c2e.l, a0
pub fn i_069c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8C2E;
    20
}
/// $069C18: move.b (a0, d0.w), $7f(a6)
pub fn i_069c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C1E;
    cpu.ir = 0x1D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad4, v3 as u8);
    22
}
/// $069C1E: lea.l $2c8c3e.l, a0
pub fn i_069c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8C3E;
    20
}
/// $069C24: add.w d0, d0
pub fn i_069c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C26;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $069C26: move.w (a0, d0.w), $76(a6)
pub fn i_069c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C2C;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    22
}
/// $069C2C: move.w #$5a, $74(a6)
pub fn i_069c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C32;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069C32: lea.l $2c9cf8.l, a0
pub fn i_069c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C9CF8;
    20
}
/// $069C38: jsr $28cd4.l
pub fn i_069c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69C3E);
    cpu.pc = 0x28CD4;
    16
}
/// $069C3E: jsr $69e5c(pc)
pub fn i_069c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69C42);
    cpu.pc = 0x69E5C;
    16
}
/// $069C42: lea.l $69c48(pc), a1
pub fn i_069c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69C48;
    16
}
/// $069C46: move.l a1, (a6)
pub fn i_069c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069C48: subq.w #$1, $76(a6)
pub fn i_069c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C4C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069C4C: cmpi.w #$0, $76(a6)
pub fn i_069c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C52;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $069C52: bgt.w $69c5c
pub fn i_069c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C56;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x69C5C; return 10; }
    8
}
/// $069C56: lea.l $69cd0(pc), a1
pub fn i_069c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69CD0;
    16
}
/// $069C5A: move.l a1, (a6)
pub fn i_069c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Tank_MissileFly_069c5c  ($069C5C, 108 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069C5C: subq.w #$1, $74(a6)
pub fn i_069c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C60;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069C60: cmpi.w #$0, $74(a6)
pub fn i_069c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C66;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $069C66: bgt.w $69c70
pub fn i_069c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C6A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x69C70; return 10; }
    8
}
/// $069C6A: lea.l $69c5c(pc), a1
pub fn i_069c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69C5C;
    16
}
/// $069C6E: move.l a1, (a6)
pub fn i_069c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069C70: jsr $27cee.l
pub fn i_069c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69C76);
    cpu.pc = 0x27CEE;
    16
}
/// $069C76: bcc.w $69c80
pub fn i_069c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69C80; return 10; }
    8
}
/// $069C7A: lea.l $69d34(pc), a1
pub fn i_069c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69D34;
    16
}
/// $069C7E: move.l a1, (a6)
pub fn i_069c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069C80: jsr $28d70.l
pub fn i_069c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69C86);
    cpu.pc = 0x28D70;
    16
}
/// $069C86: jsr $283d8.l
pub fn i_069c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69C8C);
    cpu.pc = 0x283D8;
    16
}
/// $069C8C: btst.b #$1, $13(a6)
pub fn i_069c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C92;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $069C92: beq.w $69c9c
pub fn i_069c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69C9C; return 10; }
    8
}
/// $069C96: lea.l $69d34(pc), a1
pub fn i_069c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69D34;
    16
}
/// $069C9A: move.l a1, (a6)
pub fn i_069c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069C9C: bclr.b #$3, $13(a6)
pub fn i_069c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CA2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $069CA2: jsr $28758.l
pub fn i_069ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69CA8);
    cpu.pc = 0x28758;
    16
}
/// $069CA8: bcc.w $69cb2
pub fn i_069ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69CB2; return 10; }
    8
}
/// $069CAC: lea.l $69d34(pc), a1
pub fn i_069cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69D34;
    16
}
/// $069CB0: move.l a1, (a6)
pub fn i_069cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069CB2: movea.l #$ffffffff, a0
pub fn i_069cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CB8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $069CB8: lea.l $2c8b7c.l, a0
pub fn i_069cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8B7C;
    20
}
/// $069CBE: jsr $5dd56.l
pub fn i_069cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69CC4);
    cpu.pc = 0x5DD56;
    16
}
/// $069CC4: bcc.w $69cce
pub fn i_069cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69CCE; return 10; }
    8
}
// ===== SetTaskHandler_069cc8  ($069CC8, 8 B, kernel/task_handlers.c)
/// $069CC8: lea.l $69872(pc), a1
pub fn i_069cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69872;
    16
}
/// $069CCC: move.l a1, (a6)
pub fn i_069ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069CCE: rts
pub fn i_069cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_MissileHome_069cd0  ($069CD0, 100 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069CD0: move.w #$258, $76(a6)
pub fn i_069cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069CD6: jsr $5e1ea.l
pub fn i_069cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69CDC);
    cpu.pc = 0x5E1EA;
    16
}
/// $069CDC: move.l a0, $84(a6)
pub fn i_069cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CE0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069CE0: lea.l $69ce6(pc), a1
pub fn i_069ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69CE6;
    16
}
/// $069CE4: move.l a1, (a6)
pub fn i_069ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069CE6: addq.w #$1, $76(a6)
pub fn i_069ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CEA;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069CEA: move.w $76(a6), d0
pub fn i_069cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CEE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $069CEE: cmp.b $7f(a6), d0
pub fn i_069cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CF2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $069CF2: ble.w $69d30
pub fn i_069cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CF6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69D30; return 10; }
    8
}
/// $069CF6: clr.w $76(a6)
pub fn i_069cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CFA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $069CFA: movea.l $84(a6), a0
pub fn i_069cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69CFE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $069CFE: jsr $5e070.l
pub fn i_069cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69D04);
    cpu.pc = 0x5E070;
    16
}
/// $069D04: addq.w #$8, d0
pub fn i_069d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D06;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $069D06: andi.w #$ff, d0
pub fn i_069d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D0A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $069D0A: lsr.w #$3, d0
pub fn i_069d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D0C;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $069D0C: cmp.w $82(a6), d0
pub fn i_069d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D10;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $069D10: beq.w $69d26
pub fn i_069d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69D26; return 10; }
    8
}
/// $069D14: move.w #$1, d1
pub fn i_069d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D18;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069D18: sub.w $82(a6), d0
pub fn i_069d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D1C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $069D1C: bcc.w $69d22
pub fn i_069d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69D22; return 10; }
    8
}
/// $069D20: neg.w d1
pub fn i_069d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D22;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $069D22: add.w d1, $82(a6)
pub fn i_069d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D26;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069D26: andi.w #$1f, $82(a6)
pub fn i_069d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D2C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069D2C: jsr $69e5c(pc)
pub fn i_069d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69D30);
    cpu.pc = 0x69E5C;
    16
}
/// $069D30: bra.w $69c5c
pub fn i_069d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D34;
    cpu.ir = 0x6000;
    cpu.pc = 0x69C5C;
    10
}
// ===== Tank_MissileExplode_069d34  ($069D34, 42 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069D34: jsr $13600.l
pub fn i_069d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69D3A);
    cpu.pc = 0x13600;
    16
}
/// $069D3A: move.w #$4000, d0
pub fn i_069d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D3E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069D3E: jsr $28134.l
pub fn i_069d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69D44);
    cpu.pc = 0x28134;
    16
}
/// $069D44: andi.w #$ffe3, $38(a6)
pub fn i_069d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D4A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069D4A: ori.w #$0, $38(a6)
pub fn i_069d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D50;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $069D50: move.l #$ffffffff, $48(a6)
pub fn i_069d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D58;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $069D58: jmp $77f6a.l
pub fn i_069d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D5E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Tank_EngineSound_069d5e  ($069D5E, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069D5E: move.w #$1b, d1
pub fn i_069d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D62;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069D62: tst.b $9b(a6)
pub fn i_069d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D66;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $069D66: beq.w $69d6e
pub fn i_069d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D6A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69D6E; return 10; }
    8
}
/// $069D6A: move.w #$1c, d1
pub fn i_069d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D6E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_069d6e  ($069D6E, 8 B, thunks/jsr_abs_thunks.c)
/// $069D6E: jsr $236e.l
pub fn i_069d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69D74);
    cpu.pc = 0x236E;
    16
}
/// $069D74: rts
pub fn i_069d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SetSpriteByIndex_069d76  ($069D76, 46 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069D76: move.l a1, d2
pub fn i_069d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D78;
    cpu.ir = 0x2409;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $069D78: cmpi.l #$ffffffff, d2
pub fn i_069d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D7E;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $069D7E: beq.w $69da2
pub fn i_069d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69DA2; return 10; }
    8
}
/// $069D82: move.l a6, -(a7)
pub fn i_069d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D84;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $069D84: andi.w #$ff, d0
pub fn i_069d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D88;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $069D88: movea.l a1, a6
pub fn i_069d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D8A;
    cpu.ir = 0x2C49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $069D8A: lsl.w #$2, d0
pub fn i_069d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D8C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $069D8C: movea.l (a0, d0.w), a0
pub fn i_069d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D90;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $069D90: cmpa.l #$ffffffff, a0
pub fn i_069d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D96;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $069D96: beq.w $69da0
pub fn i_069d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69D9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69DA0; return 10; }
    8
}
/// $069D9A: jsr $28cd4.l
pub fn i_069d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69DA0);
    cpu.pc = 0x28CD4;
    16
}
/// $069DA0: movea.l (a7)+, a6
pub fn i_069da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DA2;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $069DA2: rts
pub fn i_069da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SetHandlerIfValid_069da4  ($069DA4, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069DA4: cmpi.l #$ffffffff, d0
pub fn i_069da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DAA;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $069DAA: beq.w $69db2
pub fn i_069daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69DB2; return 10; }
    8
}
/// $069DAE: movea.l d0, a0
pub fn i_069dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DB0;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $069DB0: move.l a1, (a0)
pub fn i_069db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DB2;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069DB2: rts
pub fn i_069db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SetParentHandler_069db4  ($069DB4, 8 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069DB4: movea.l $c(a6), a0
pub fn i_069db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DB8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $069DB8: move.l a1, (a0)
pub fn i_069db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DBA;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $069DBA: rts
pub fn i_069dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_MirrorByFacing_069dbc  ($069DBC, 12 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069DBC: cmpi.b #$1, $3a(a6)
pub fn i_069dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DC2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $069DC2: beq.w $69dce
pub fn i_069dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69DCE; return 10; }
    8
}
/// $069DC6: neg.w d0
pub fn i_069dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DC8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_069dc8  ($069DC8, 6 B, thunks/ccr_helpers.c)
/// $069DC8: ori.b #$11, ccr
pub fn i_069dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DCC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $069DCC: rts
pub fn i_069dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_069dce  ($069DCE, 6 B, thunks/ccr_helpers.c)
/// $069DCE: andi.b #$ee, ccr
pub fn i_069dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DD2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $069DD2: rts
pub fn i_069dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_MirrorByDir_069dd4  ($069DD4, 8 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069DD4: cmpi.b #$1, $70(a6)
pub fn i_069dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DDA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $069DDA: bra.b $69dc2
pub fn i_069dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DDC;
    cpu.ir = 0x60E6;
    cpu.pc = 0x69DC2;
    10
}
// ===== Tank_FacingMatchesDir_069ddc  ($069DDC, 12 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069DDC: move.b $3a(a6), d0
pub fn i_069ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DE0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $069DE0: cmp.b $70(a6), d0
pub fn i_069de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DE4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $069DE4: bne.w $69dee
pub fn i_069de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x69DEE; return 10; }
    8
}
// ===== ClearXN_069de8  ($069DE8, 6 B, thunks/ccr_helpers.c)
/// $069DE8: andi.b #$ee, ccr
pub fn i_069de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DEC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $069DEC: rts
pub fn i_069dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_069dee  ($069DEE, 6 B, thunks/ccr_helpers.c)
/// $069DEE: ori.b #$11, ccr
pub fn i_069dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DF2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $069DF2: rts
pub fn i_069df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_PastRangeX_069df4  ($069DF4, 32 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069DF4: move.w $22(a6), d1
pub fn i_069df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DF8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $069DF8: move.w #$0, d0
pub fn i_069df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69DFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069DFC: jsr $69dd4(pc)
pub fn i_069dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69E00);
    cpu.pc = 0x69DD4;
    16
}
/// $069E00: bcs.w $69e0c
pub fn i_069e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E04;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x69E0C; return 10; }
    8
}
/// $069E04: add.w $72(a6), d0
pub fn i_069e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E08;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $069E08: cmp.w d1, d0
pub fn i_069e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E0A;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $069E0A: rts
pub fn i_069e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $069E0C: add.w $72(a6), d0
pub fn i_069e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E10;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $069E10: cmp.w d0, d1
pub fn i_069e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E12;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $069E12: rts
pub fn i_069e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_CooldownTick_069e14  ($069E14, 22 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069E14: subq.w #$1, $76(a6)
pub fn i_069e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E18;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069E18: bgt.w $69e30
pub fn i_069e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E1C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x69E30; return 10; }
    8
}
/// $069E1C: clr.w $76(a6)
pub fn i_069e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E20;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $069E20: cmpi.w #$120, $22(a6)
pub fn i_069e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E26;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $069E26: bgt.w $69e30
pub fn i_069e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E2A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x69E30; return 10; }
    8
}
// ===== SetXN_069e2a  ($069E2A, 6 B, thunks/ccr_helpers.c)
/// $069E2A: ori.b #$11, ccr
pub fn i_069e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E2E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $069E2E: rts
pub fn i_069e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_069e30  ($069E30, 6 B, thunks/ccr_helpers.c)
/// $069E30: andi.b #$ee, ccr
pub fn i_069e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E34;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $069E34: rts
pub fn i_069e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_AnimFrame_069e36  ($069E36, 38 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069E36: clr.l d0
pub fn i_069e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E38;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $069E38: move.w $96(a6), d0
pub fn i_069e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E3C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $069E3C: addq.w #$1, $96(a6)
pub fn i_069e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E40;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069E40: lsr.w #$4, d0
pub fn i_069e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E42;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $069E42: cmpi.w #$3, d0
pub fn i_069e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E46;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $069E46: ble.w $69e4e
pub fn i_069e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E4A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69E4E; return 10; }
    8
}
/// $069E4A: move.w #$3, d0
pub fn i_069e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E4E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069E4E: lsl.w #$2, d0
pub fn i_069e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E50;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $069E50: movea.l $90(a6), a0
pub fn i_069e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E54;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $069E54: move.l (a0, d0.w), $5c(a6)
pub fn i_069e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E5A;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $069E5A: rts
pub fn i_069e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_VelFromAngle_069e5c  ($069E5C, 52 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069E5C: move.w $82(a6), d0
pub fn i_069e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E60;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $069E60: andi.w #$1f, d0
pub fn i_069e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E64;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $069E64: lsl.w #$4, d0
pub fn i_069e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E66;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $069E66: lea.l $2c07ac.l, a1
pub fn i_069e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $069E6C: lea.l $2c072c.l, a2
pub fn i_069e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E72;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $069E72: move.w (a1, d0.w), d1
pub fn i_069e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E76;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $069E76: move.w (a2, d0.w), d2
pub fn i_069e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E7A;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $069E7A: move.w $36(a6), d0
pub fn i_069e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E7E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $069E7E: muls.w d0, d1
pub fn i_069e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E80;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $069E80: muls.w d0, d2
pub fn i_069e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E82;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $069E82: asr.l #$8, d1
pub fn i_069e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E84;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $069E84: asr.l #$8, d2
pub fn i_069e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E86;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $069E86: move.w d1, $28(a6)
pub fn i_069e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E8A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069E8A: move.w d2, $2a(a6)
pub fn i_069e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E8E;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069E8E: rts
pub fn i_069e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_RecoilTick_069e90  ($069E90, 20 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069E90: cmpi.w #$0, $94(a6)
pub fn i_069e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E96;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $069E96: ble.w $69ea8
pub fn i_069e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E9A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x69EA8; return 10; }
    8
}
/// $069E9A: subq.w #$1, $94(a6)
pub fn i_069e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69E9E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069E9E: addi.b #$1, $44(a6)
pub fn i_069e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EA4;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetXN_069ea4  ($069EA4, 6 B, thunks/ccr_helpers.c)
/// $069EA4: ori.b #$11, ccr
pub fn i_069ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EA8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $069EA8: rts
pub fn i_069ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_CopyParentAnim_069eaa  ($069EAA, 20 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069EAA: jsr $5e506.l
pub fn i_069eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69EB0);
    cpu.pc = 0x5E506;
    16
}
/// $069EB0: move.b $44(a0), $44(a6)
pub fn i_069eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EB6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad4, v3 as u8);
    20
}
/// $069EB6: move.l $5c(a0), $5c(a6)
pub fn i_069eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EBC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $069EBC: rts
pub fn i_069ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_CopyParentAnimPrio_069ebe  ($069EBE, 22 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069EBE: jsr $5e506.l
pub fn i_069ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69EC4);
    cpu.pc = 0x5E506;
    16
}
/// $069EC4: move.w $72(a6), d0
pub fn i_069ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EC8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $069EC8: add.w d0, $38(a6)
pub fn i_069ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69ECC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069ECC: move.b $44(a0), $44(a6)
pub fn i_069ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69ED2;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad4, v3 as u8);
    20
}
/// $069ED2: rts
pub fn i_069ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69ED4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_OffsetAbove_069ed4  ($069ED4, 20 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069ED4: addi.w #$18, $24(a6)
pub fn i_069ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EDA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $069EDA: move.w #$fff0, d0
pub fn i_069eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069EDE: jsr $69dbc(pc)
pub fn i_069ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69EE2);
    cpu.pc = 0x69DBC;
    16
}
/// $069EE2: add.w d0, $22(a6)
pub fn i_069ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EE6;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069EE6: rts
pub fn i_069ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_ItemFlag_069ee8  ($069EE8, 8 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069EE8: tst.b $9c(a6)
pub fn i_069ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EEC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $069EEC: beq.w $69ef6
pub fn i_069eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x69EF6; return 10; }
    8
}
// ===== ClrRamWord_069ef0  ($069EF0, 8 B, thunks/clr_ram_word.c)
/// $069EF0: clr.b $10e39a.l
pub fn i_069ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EF6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $069EF6: rts
pub fn i_069ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SpawnExplosion_069ef8  ($069EF8, 26 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069EF8: lea.l $77fd6.l, a1
pub fn i_069ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69EFE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $069EFE: jsr $4ae.l
pub fn i_069efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F04);
    cpu.pc = 0x4AE;
    16
}
/// $069F04: jsr $5dd02.l
pub fn i_069f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F0A);
    cpu.pc = 0x5DD02;
    16
}
/// $069F0A: move.w #$4000, $38(a0)
pub fn i_069f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F10;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $069F10: rts
pub fn i_069f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_DeathBlast_069f12  ($069F12, 70 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069F12: move.w #$1033, d0
pub fn i_069f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F16;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069F16: jsr $2352.l
pub fn i_069f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F1C);
    cpu.pc = 0x2352;
    16
}
/// $069F1C: lea.l $77fd6.l, a1
pub fn i_069f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F22;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $069F22: jsr $4ae.l
pub fn i_069f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F28);
    cpu.pc = 0x4AE;
    16
}
/// $069F28: jsr $5dd02.l
pub fn i_069f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F2E);
    cpu.pc = 0x5DD02;
    16
}
/// $069F2E: move.w #$10, d0
pub fn i_069f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F32;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069F32: movem.w d0, -(a7)
pub fn i_069f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F36;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $069F36: lea.l $62536.l, a1
pub fn i_069f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x62536;
    20
}
/// $069F3C: jsr $4ae.l
pub fn i_069f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F42);
    cpu.pc = 0x4AE;
    16
}
/// $069F42: jsr $5dd02.l
pub fn i_069f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F48);
    cpu.pc = 0x5DD02;
    16
}
/// $069F48: addi.w #$18, $24(a0)
pub fn i_069f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F4E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $069F4E: movem.w (a7)+, d0
pub fn i_069f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F52;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $069F52: subq.w #$1, d0
pub fn i_069f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F54;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $069F54: bcc.b $69f32
pub fn i_069f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F56;
    cpu.ir = 0x64DC;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x69F32; return 10; }
    8
}
/// $069F56: rts
pub fn i_069f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_DropItem_069f58  ($069F58, 8 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069F58: move.b $9d(a6), d0
pub fn i_069f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F5C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $069F5C: move.w #$9e, d1
pub fn i_069f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F60;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_069f60  ($069F60, 8 B, thunks/jsr_abs_thunks.c)
/// $069F60: jsr $9a7aa.l
pub fn i_069f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F66);
    cpu.pc = 0x9A7AA;
    16
}
/// $069F66: rts
pub fn i_069f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SpawnSmokeMissile_069f68  ($069F68, 80 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069F68: move.w #$1064, d0
pub fn i_069f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F6C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069F6C: jsr $2352.l
pub fn i_069f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F72);
    cpu.pc = 0x2352;
    16
}
/// $069F72: lea.l $69b5a(pc), a1
pub fn i_069f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69B5A;
    16
}
/// $069F76: jsr $4ae.l
pub fn i_069f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F7C);
    cpu.pc = 0x4AE;
    16
}
/// $069F7C: jsr $5dd02.l
pub fn i_069f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F82);
    cpu.pc = 0x5DD02;
    16
}
/// $069F82: addi.w #$30, $24(a0)
pub fn i_069f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F88;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $069F88: move.w #$28, d0
pub fn i_069f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069F8C: jsr $69dbc(pc)
pub fn i_069f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69F90);
    cpu.pc = 0x69DBC;
    16
}
/// $069F90: add.w d0, $22(a0)
pub fn i_069f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F94;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069F94: lea.l $69bce(pc), a1
pub fn i_069f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69BCE;
    16
}
/// $069F98: jsr $4ae.l
pub fn i_069f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69F9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69F9E);
    cpu.pc = 0x4AE;
    16
}
/// $069F9E: jsr $5dd02.l
pub fn i_069f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69FA4);
    cpu.pc = 0x5DD02;
    16
}
/// $069FA4: addi.w #$3e, $24(a0)
pub fn i_069fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FAA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x3E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $069FAA: move.w #$28, d0
pub fn i_069faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069FAE: jsr $69dbc(pc)
pub fn i_069fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69FB2);
    cpu.pc = 0x69DBC;
    16
}
/// $069FB2: add.w d0, $22(a0)
pub fn i_069fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FB6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069FB6: rts
pub fn i_069fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SpawnSmokeDebris_069fb8  ($069FB8, 62 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069FB8: move.w #$1064, d0
pub fn i_069fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069FBC: jsr $2352.l
pub fn i_069fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69FC2);
    cpu.pc = 0x2352;
    16
}
/// $069FC2: lea.l $69b40(pc), a1
pub fn i_069fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69B40;
    16
}
/// $069FC6: jsr $4ae.l
pub fn i_069fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69FCC);
    cpu.pc = 0x4AE;
    16
}
/// $069FCC: jsr $5dd02.l
pub fn i_069fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69FD2);
    cpu.pc = 0x5DD02;
    16
}
/// $069FD2: lea.l $69af8(pc), a1
pub fn i_069fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x69AF8;
    16
}
/// $069FD6: jsr $4ae.l
pub fn i_069fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69FDC);
    cpu.pc = 0x4AE;
    16
}
/// $069FDC: jsr $5dd02.l
pub fn i_069fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x69FE2);
    cpu.pc = 0x5DD02;
    16
}
/// $069FE2: addi.w #$28, $24(a0)
pub fn i_069fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FE8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $069FE8: move.w #$20, d0
pub fn i_069fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069FEC: jsr $69dbc(pc)
pub fn i_069fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x69FF0);
    cpu.pc = 0x69DBC;
    16
}
/// $069FF0: add.w d0, $22(a0)
pub fn i_069ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FF4;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $069FF4: rts
pub fn i_069ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tank_SetPrio4000_069ff6  ($069FF6, 10 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $069FF6: move.w #$4000, d0
pub fn i_069ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69FFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $069FFA: jsr $28134.l
pub fn i_069ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A000);
    cpu.pc = 0x28134;
    16
}
// ===== Tank_SetAttackByVariant_06a000  ($06A000, 60 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A000: andi.w #$ffe3, $38(a6)
pub fn i_06a000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A006;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06A006: ori.w #$14, $38(a6)
pub fn i_06a006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A00C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06A00C: tst.b $7d(a6)
pub fn i_06a00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A010;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A010: bne.w $6a042
pub fn i_06a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A014;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6A042; return 10; }
    8
}
/// $06A014: move.l #$2c8a74, $4c(a6)
pub fn i_06a014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A01C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8A74;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06A01C: jsr $283ca.l
pub fn i_06a01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A022);
    cpu.pc = 0x283CA;
    16
}
/// $06A022: jsr $283d8.l
pub fn i_06a022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A028;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A028);
    cpu.pc = 0x283D8;
    16
}
/// $06A028: lea.l $2c8c7e.l, a1
pub fn i_06a028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A02E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C8C7E;
    20
}
/// $06A02E: tst.b $7e(a6)
pub fn i_06a02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A032;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A032: beq.w $6a03c
pub fn i_06a032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A036;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A03C; return 10; }
    8
}
/// $06A036: lea.l $2c8ca6.l, a1
pub fn i_06a036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A03C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C8CA6;
    20
}
// ===== JsrAbsThunk_06a03c  ($06A03C, 8 B, thunks/jsr_abs_thunks.c)
/// $06A03C: jsr $43fac.l
pub fn i_06a03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A042);
    cpu.pc = 0x43FAC;
    16
}
/// $06A042: rts
pub fn i_06a042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A044;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x06873C, i_06873c),
    (0x068742, i_068742),
    (0x068748, i_068748),
    (0x06874E, i_06874e),
    (0x068752, i_068752),
    (0x068754, i_068754),
    (0x068758, i_068758),
    (0x06875C, i_06875c),
    (0x06875E, i_06875e),
    (0x068762, i_068762),
    (0x068766, i_068766),
    (0x06876A, i_06876a),
    (0x068770, i_068770),
    (0x068776, i_068776),
    (0x06877C, i_06877c),
    (0x068782, i_068782),
    (0x068786, i_068786),
    (0x068788, i_068788),
    (0x06878E, i_06878e),
    (0x068792, i_068792),
    (0x068798, i_068798),
    (0x06879E, i_06879e),
    (0x0687A4, i_0687a4),
    (0x0687A8, i_0687a8),
    (0x0687AA, i_0687aa),
    (0x0687AE, i_0687ae),
    (0x0687B2, i_0687b2),
    (0x0687B4, i_0687b4),
    (0x0687B8, i_0687b8),
    (0x0687BE, i_0687be),
    (0x0687C2, i_0687c2),
    (0x0687C8, i_0687c8),
    (0x0687CE, i_0687ce),
    (0x0687D4, i_0687d4),
    (0x0687DA, i_0687da),
    (0x0687DE, i_0687de),
    (0x0687E0, i_0687e0),
    (0x0687E4, i_0687e4),
    (0x0687E8, i_0687e8),
    (0x0687EC, i_0687ec),
    (0x0687F0, i_0687f0),
    (0x0687F6, i_0687f6),
    (0x0687FA, i_0687fa),
    (0x0687FE, i_0687fe),
    (0x068800, i_068800),
    (0x068804, i_068804),
    (0x068808, i_068808),
    (0x06880E, i_06880e),
    (0x068814, i_068814),
    (0x06881A, i_06881a),
    (0x06881E, i_06881e),
    (0x068820, i_068820),
    (0x068824, i_068824),
    (0x068828, i_068828),
    (0x06882C, i_06882c),
    (0x068830, i_068830),
    (0x068836, i_068836),
    (0x06883A, i_06883a),
    (0x06883E, i_06883e),
    (0x068840, i_068840),
    (0x068844, i_068844),
    (0x068848, i_068848),
    (0x06884E, i_06884e),
    (0x068852, i_068852),
    (0x068856, i_068856),
    (0x06885A, i_06885a),
    (0x06885E, i_06885e),
    (0x068864, i_068864),
    (0x068866, i_068866),
    (0x06886A, i_06886a),
    (0x068870, i_068870),
    (0x068874, i_068874),
    (0x06887A, i_06887a),
    (0x06887E, i_06887e),
    (0x068880, i_068880),
    (0x068884, i_068884),
    (0x068888, i_068888),
    (0x06888C, i_06888c),
    (0x068890, i_068890),
    (0x068896, i_068896),
    (0x06889A, i_06889a),
    (0x06889E, i_06889e),
    (0x0688A0, i_0688a0),
    (0x0688A4, i_0688a4),
    (0x0688A8, i_0688a8),
    (0x0688AE, i_0688ae),
    (0x0688B0, i_0688b0),
    (0x0688B4, i_0688b4),
    (0x0688BA, i_0688ba),
    (0x0688BE, i_0688be),
    (0x0688C4, i_0688c4),
    (0x0688C8, i_0688c8),
    (0x0688CA, i_0688ca),
    (0x0688CE, i_0688ce),
    (0x0688D2, i_0688d2),
    (0x0688D6, i_0688d6),
    (0x0688DA, i_0688da),
    (0x0688E0, i_0688e0),
    (0x0688E4, i_0688e4),
    (0x0688E8, i_0688e8),
    (0x0688EE, i_0688ee),
    (0x0688F2, i_0688f2),
    (0x0688F4, i_0688f4),
    (0x0688F8, i_0688f8),
    (0x0688FC, i_0688fc),
    (0x068902, i_068902),
    (0x068904, i_068904),
    (0x068908, i_068908),
    (0x06890E, i_06890e),
    (0x068912, i_068912),
    (0x068918, i_068918),
    (0x06891C, i_06891c),
    (0x06891E, i_06891e),
    (0x068922, i_068922),
    (0x068928, i_068928),
    (0x06892C, i_06892c),
    (0x068930, i_068930),
    (0x068934, i_068934),
    (0x068938, i_068938),
    (0x06893E, i_06893e),
    (0x068942, i_068942),
    (0x068946, i_068946),
    (0x068948, i_068948),
    (0x06894A, i_06894a),
    (0x068950, i_068950),
    (0x068954, i_068954),
    (0x068958, i_068958),
    (0x06895A, i_06895a),
    (0x06895C, i_06895c),
    (0x068962, i_068962),
    (0x068968, i_068968),
    (0x06896A, i_06896a),
    (0x06896E, i_06896e),
    (0x068970, i_068970),
    (0x068976, i_068976),
    (0x06897A, i_06897a),
    (0x068980, i_068980),
    (0x068986, i_068986),
    (0x06898C, i_06898c),
    (0x068990, i_068990),
    (0x068996, i_068996),
    (0x06899A, i_06899a),
    (0x06899E, i_06899e),
    (0x0689A4, i_0689a4),
    (0x0689A8, i_0689a8),
    (0x0689AC, i_0689ac),
    (0x0689B2, i_0689b2),
    (0x0689B8, i_0689b8),
    (0x0689BE, i_0689be),
    (0x0689C4, i_0689c4),
    (0x0689C8, i_0689c8),
    (0x0689CC, i_0689cc),
    (0x0689D2, i_0689d2),
    (0x0689DA, i_0689da),
    (0x0689DE, i_0689de),
    (0x0689E0, i_0689e0),
    (0x0689E4, i_0689e4),
    (0x0689E6, i_0689e6),
    (0x0689EA, i_0689ea),
    (0x0689EC, i_0689ec),
    (0x0689F2, i_0689f2),
    (0x0689F6, i_0689f6),
    (0x0689FC, i_0689fc),
    (0x068A02, i_068a02),
    (0x068A08, i_068a08),
    (0x068A0C, i_068a0c),
    (0x068A12, i_068a12),
    (0x068A16, i_068a16),
    (0x068A1A, i_068a1a),
    (0x068A20, i_068a20),
    (0x068A24, i_068a24),
    (0x068A28, i_068a28),
    (0x068A2E, i_068a2e),
    (0x068A34, i_068a34),
    (0x068A3A, i_068a3a),
    (0x068A40, i_068a40),
    (0x068A44, i_068a44),
    (0x068A48, i_068a48),
    (0x068A4E, i_068a4e),
    (0x068A52, i_068a52),
    (0x068A54, i_068a54),
    (0x068A58, i_068a58),
    (0x068A5A, i_068a5a),
    (0x068A5E, i_068a5e),
    (0x068A62, i_068a62),
    (0x068A68, i_068a68),
    (0x068A6E, i_068a6e),
    (0x068A70, i_068a70),
    (0x068A76, i_068a76),
    (0x068A7A, i_068a7a),
    (0x068A7E, i_068a7e),
    (0x068A82, i_068a82),
    (0x068A86, i_068a86),
    (0x068A8A, i_068a8a),
    (0x068A8E, i_068a8e),
    (0x068A90, i_068a90),
    (0x068A96, i_068a96),
    (0x068A9A, i_068a9a),
    (0x068A9E, i_068a9e),
    (0x068AA2, i_068aa2),
    (0x068AA6, i_068aa6),
    (0x068AAA, i_068aaa),
    (0x068AAE, i_068aae),
    (0x068AB2, i_068ab2),
    (0x068AB6, i_068ab6),
    (0x068AB8, i_068ab8),
    (0x068ABE, i_068abe),
    (0x068AC2, i_068ac2),
    (0x068AC8, i_068ac8),
    (0x068ACA, i_068aca),
    (0x068ACE, i_068ace),
    (0x068AD0, i_068ad0),
    (0x068AD4, i_068ad4),
    (0x068AD8, i_068ad8),
    (0x068ADA, i_068ada),
    (0x068AE0, i_068ae0),
    (0x068AE4, i_068ae4),
    (0x068AE8, i_068ae8),
    (0x068AEC, i_068aec),
    (0x068AEE, i_068aee),
    (0x068AF2, i_068af2),
    (0x068AF4, i_068af4),
    (0x068AFA, i_068afa),
    (0x068AFE, i_068afe),
    (0x068B04, i_068b04),
    (0x068B08, i_068b08),
    (0x068B0C, i_068b0c),
    (0x068B0E, i_068b0e),
    (0x068B12, i_068b12),
    (0x068B14, i_068b14),
    (0x068B18, i_068b18),
    (0x068B1E, i_068b1e),
    (0x068B24, i_068b24),
    (0x068B2A, i_068b2a),
    (0x068B2E, i_068b2e),
    (0x068B32, i_068b32),
    (0x068B36, i_068b36),
    (0x068B3A, i_068b3a),
    (0x068B3E, i_068b3e),
    (0x068B44, i_068b44),
    (0x068B46, i_068b46),
    (0x068B48, i_068b48),
    (0x068B4C, i_068b4c),
    (0x068B50, i_068b50),
    (0x068B54, i_068b54),
    (0x068B58, i_068b58),
    (0x068B5C, i_068b5c),
    (0x068B62, i_068b62),
    (0x068B64, i_068b64),
    (0x068B68, i_068b68),
    (0x068B6C, i_068b6c),
    (0x068B6E, i_068b6e),
    (0x068B72, i_068b72),
    (0x068B78, i_068b78),
    (0x068B7C, i_068b7c),
    (0x068B80, i_068b80),
    (0x068B84, i_068b84),
    (0x068B86, i_068b86),
    (0x068B8A, i_068b8a),
    (0x068B8E, i_068b8e),
    (0x068B92, i_068b92),
    (0x068B96, i_068b96),
    (0x068B9A, i_068b9a),
    (0x068BA0, i_068ba0),
    (0x068BA2, i_068ba2),
    (0x068BA8, i_068ba8),
    (0x068BAC, i_068bac),
    (0x068BB2, i_068bb2),
    (0x068BB8, i_068bb8),
    (0x068BBC, i_068bbc),
    (0x068BC2, i_068bc2),
    (0x068BC6, i_068bc6),
    (0x068BCA, i_068bca),
    (0x068BCE, i_068bce),
    (0x068BD4, i_068bd4),
    (0x068BDA, i_068bda),
    (0x068BE0, i_068be0),
    (0x068BE6, i_068be6),
    (0x068BEA, i_068bea),
    (0x068BF0, i_068bf0),
    (0x068BF6, i_068bf6),
    (0x068BFC, i_068bfc),
    (0x068C00, i_068c00),
    (0x068C02, i_068c02),
    (0x068C06, i_068c06),
    (0x068C0A, i_068c0a),
    (0x068C0E, i_068c0e),
    (0x068C12, i_068c12),
    (0x068C16, i_068c16),
    (0x068C18, i_068c18),
    (0x068C1C, i_068c1c),
    (0x068C1E, i_068c1e),
    (0x068C22, i_068c22),
    (0x068C26, i_068c26),
    (0x068C2A, i_068c2a),
    (0x068C2E, i_068c2e),
    (0x068C34, i_068c34),
    (0x068C38, i_068c38),
    (0x068C3E, i_068c3e),
    (0x068C42, i_068c42),
    (0x068C46, i_068c46),
    (0x068C4C, i_068c4c),
    (0x068C52, i_068c52),
    (0x068C58, i_068c58),
    (0x068C5E, i_068c5e),
    (0x068C64, i_068c64),
    (0x068C6A, i_068c6a),
    (0x068C6E, i_068c6e),
    (0x068C74, i_068c74),
    (0x068C7A, i_068c7a),
    (0x068C7E, i_068c7e),
    (0x068C82, i_068c82),
    (0x068C88, i_068c88),
    (0x068C8E, i_068c8e),
    (0x068C94, i_068c94),
    (0x068C9A, i_068c9a),
    (0x068CA2, i_068ca2),
    (0x068CAA, i_068caa),
    (0x068CAE, i_068cae),
    (0x068CB2, i_068cb2),
    (0x068CB8, i_068cb8),
    (0x068CBE, i_068cbe),
    (0x068CC2, i_068cc2),
    (0x068CC8, i_068cc8),
    (0x068CCC, i_068ccc),
    (0x068CD0, i_068cd0),
    (0x068CD4, i_068cd4),
    (0x068CDA, i_068cda),
    (0x068CE0, i_068ce0),
    (0x068CE6, i_068ce6),
    (0x068CEA, i_068cea),
    (0x068CEE, i_068cee),
    (0x068CF2, i_068cf2),
    (0x068CF8, i_068cf8),
    (0x068CFC, i_068cfc),
    (0x068D00, i_068d00),
    (0x068D04, i_068d04),
    (0x068D08, i_068d08),
    (0x068D0C, i_068d0c),
    (0x068D10, i_068d10),
    (0x068D16, i_068d16),
    (0x068D1C, i_068d1c),
    (0x068D22, i_068d22),
    (0x068D26, i_068d26),
    (0x068D2C, i_068d2c),
    (0x068D32, i_068d32),
    (0x068D38, i_068d38),
    (0x068D40, i_068d40),
    (0x068D44, i_068d44),
    (0x068D48, i_068d48),
    (0x068D4C, i_068d4c),
    (0x068D52, i_068d52),
    (0x068D5A, i_068d5a),
    (0x068D5E, i_068d5e),
    (0x068D62, i_068d62),
    (0x068D66, i_068d66),
    (0x068D6A, i_068d6a),
    (0x068D6C, i_068d6c),
    (0x068D6E, i_068d6e),
    (0x068D72, i_068d72),
    (0x068D74, i_068d74),
    (0x068D76, i_068d76),
    (0x068D7A, i_068d7a),
    (0x068D80, i_068d80),
    (0x068D84, i_068d84),
    (0x068D88, i_068d88),
    (0x068D8C, i_068d8c),
    (0x068D90, i_068d90),
    (0x068D92, i_068d92),
    (0x068D94, i_068d94),
    (0x068D98, i_068d98),
    (0x068D9A, i_068d9a),
    (0x068D9C, i_068d9c),
    (0x068DA0, i_068da0),
    (0x068DA6, i_068da6),
    (0x068DAA, i_068daa),
    (0x068DB0, i_068db0),
    (0x068DB6, i_068db6),
    (0x068DBA, i_068dba),
    (0x068DBC, i_068dbc),
    (0x068DC2, i_068dc2),
    (0x068DC8, i_068dc8),
    (0x068DCC, i_068dcc),
    (0x068DD0, i_068dd0),
    (0x068DD2, i_068dd2),
    (0x068DD8, i_068dd8),
    (0x068DDC, i_068ddc),
    (0x068DE0, i_068de0),
    (0x068DE4, i_068de4),
    (0x068DE8, i_068de8),
    (0x068DEE, i_068dee),
    (0x068DF4, i_068df4),
    (0x068DF8, i_068df8),
    (0x068DFC, i_068dfc),
    (0x068E00, i_068e00),
    (0x068E04, i_068e04),
    (0x068E08, i_068e08),
    (0x068E0C, i_068e0c),
    (0x068E14, i_068e14),
    (0x068E18, i_068e18),
    (0x068E1C, i_068e1c),
    (0x068E20, i_068e20),
    (0x068E24, i_068e24),
    (0x068E26, i_068e26),
    (0x068E28, i_068e28),
    (0x068E2C, i_068e2c),
    (0x068E2E, i_068e2e),
    (0x068E30, i_068e30),
    (0x068E34, i_068e34),
    (0x068E3A, i_068e3a),
    (0x068E3E, i_068e3e),
    (0x068E42, i_068e42),
    (0x068E46, i_068e46),
    (0x068E4A, i_068e4a),
    (0x068E4C, i_068e4c),
    (0x068E4E, i_068e4e),
    (0x068E52, i_068e52),
    (0x068E54, i_068e54),
    (0x068E56, i_068e56),
    (0x068E5A, i_068e5a),
    (0x068E60, i_068e60),
    (0x068E64, i_068e64),
    (0x068E6A, i_068e6a),
    (0x068E70, i_068e70),
    (0x068E78, i_068e78),
    (0x068E7C, i_068e7c),
    (0x068E84, i_068e84),
    (0x068E88, i_068e88),
    (0x068E8C, i_068e8c),
    (0x068E90, i_068e90),
    (0x068E94, i_068e94),
    (0x068E96, i_068e96),
    (0x068E98, i_068e98),
    (0x068E9C, i_068e9c),
    (0x068E9E, i_068e9e),
    (0x068EA0, i_068ea0),
    (0x068EA4, i_068ea4),
    (0x068EAA, i_068eaa),
    (0x068EAE, i_068eae),
    (0x068EB2, i_068eb2),
    (0x068EB6, i_068eb6),
    (0x068EBA, i_068eba),
    (0x068EBC, i_068ebc),
    (0x068EBE, i_068ebe),
    (0x068EC2, i_068ec2),
    (0x068EC4, i_068ec4),
    (0x068EC6, i_068ec6),
    (0x068ECA, i_068eca),
    (0x068ED0, i_068ed0),
    (0x068ED4, i_068ed4),
    (0x068EDA, i_068eda),
    (0x068EE0, i_068ee0),
    (0x068EE8, i_068ee8),
    (0x068EEC, i_068eec),
    (0x068EF0, i_068ef0),
    (0x068EF2, i_068ef2),
    (0x068EF8, i_068ef8),
    (0x068EFE, i_068efe),
    (0x068F02, i_068f02),
    (0x068F06, i_068f06),
    (0x068F08, i_068f08),
    (0x068F0E, i_068f0e),
    (0x068F12, i_068f12),
    (0x068F16, i_068f16),
    (0x068F1A, i_068f1a),
    (0x068F1C, i_068f1c),
    (0x068F20, i_068f20),
    (0x068F24, i_068f24),
    (0x068F28, i_068f28),
    (0x068F2A, i_068f2a),
    (0x068F30, i_068f30),
    (0x068F36, i_068f36),
    (0x068F3A, i_068f3a),
    (0x068F40, i_068f40),
    (0x068F48, i_068f48),
    (0x068F4C, i_068f4c),
    (0x068F50, i_068f50),
    (0x068F54, i_068f54),
    (0x068F58, i_068f58),
    (0x068F5A, i_068f5a),
    (0x068F5C, i_068f5c),
    (0x068F60, i_068f60),
    (0x068F62, i_068f62),
    (0x068F64, i_068f64),
    (0x068F68, i_068f68),
    (0x068F6E, i_068f6e),
    (0x068F72, i_068f72),
    (0x068F76, i_068f76),
    (0x068F7A, i_068f7a),
    (0x068F7E, i_068f7e),
    (0x068F80, i_068f80),
    (0x068F82, i_068f82),
    (0x068F86, i_068f86),
    (0x068F88, i_068f88),
    (0x068F8A, i_068f8a),
    (0x068F8E, i_068f8e),
    (0x068F94, i_068f94),
    (0x068F98, i_068f98),
    (0x068F9E, i_068f9e),
    (0x068FA4, i_068fa4),
    (0x068FA8, i_068fa8),
    (0x068FAA, i_068faa),
    (0x068FB0, i_068fb0),
    (0x068FB6, i_068fb6),
    (0x068FBC, i_068fbc),
    (0x068FC0, i_068fc0),
    (0x068FC4, i_068fc4),
    (0x068FC8, i_068fc8),
    (0x068FCC, i_068fcc),
    (0x068FD0, i_068fd0),
    (0x068FD2, i_068fd2),
    (0x068FD6, i_068fd6),
    (0x068FDA, i_068fda),
    (0x068FDE, i_068fde),
    (0x068FE0, i_068fe0),
    (0x068FE4, i_068fe4),
    (0x068FE8, i_068fe8),
    (0x068FEC, i_068fec),
    (0x068FF4, i_068ff4),
    (0x068FF8, i_068ff8),
    (0x069000, i_069000),
    (0x069006, i_069006),
    (0x06900A, i_06900a),
    (0x06900E, i_06900e),
    (0x069012, i_069012),
    (0x06901A, i_06901a),
    (0x06901E, i_06901e),
    (0x069026, i_069026),
    (0x06902A, i_06902a),
    (0x06902C, i_06902c),
    (0x069030, i_069030),
    (0x069032, i_069032),
    (0x069036, i_069036),
    (0x06903A, i_06903a),
    (0x069040, i_069040),
    (0x069044, i_069044),
    (0x069046, i_069046),
    (0x06904C, i_06904c),
    (0x069052, i_069052),
    (0x069058, i_069058),
    (0x06905C, i_06905c),
    (0x069062, i_069062),
    (0x069066, i_069066),
    (0x06906A, i_06906a),
    (0x069072, i_069072),
    (0x069076, i_069076),
    (0x06907A, i_06907a),
    (0x06907E, i_06907e),
    (0x069082, i_069082),
    (0x069084, i_069084),
    (0x069086, i_069086),
    (0x06908A, i_06908a),
    (0x06908C, i_06908c),
    (0x06908E, i_06908e),
    (0x069092, i_069092),
    (0x069098, i_069098),
    (0x06909C, i_06909c),
    (0x0690A0, i_0690a0),
    (0x0690A4, i_0690a4),
    (0x0690A8, i_0690a8),
    (0x0690AA, i_0690aa),
    (0x0690AC, i_0690ac),
    (0x0690B0, i_0690b0),
    (0x0690B2, i_0690b2),
    (0x0690B4, i_0690b4),
    (0x0690B8, i_0690b8),
    (0x0690BE, i_0690be),
    (0x0690C2, i_0690c2),
    (0x0690C8, i_0690c8),
    (0x0690CE, i_0690ce),
    (0x0690D2, i_0690d2),
    (0x0690D8, i_0690d8),
    (0x0690DC, i_0690dc),
    (0x0690E0, i_0690e0),
    (0x0690E2, i_0690e2),
    (0x0690E6, i_0690e6),
    (0x0690EA, i_0690ea),
    (0x0690EE, i_0690ee),
    (0x0690F2, i_0690f2),
    (0x0690F6, i_0690f6),
    (0x0690FA, i_0690fa),
    (0x069102, i_069102),
    (0x069106, i_069106),
    (0x06910A, i_06910a),
    (0x06910E, i_06910e),
    (0x069112, i_069112),
    (0x069114, i_069114),
    (0x069116, i_069116),
    (0x06911A, i_06911a),
    (0x06911C, i_06911c),
    (0x06911E, i_06911e),
    (0x069122, i_069122),
    (0x069128, i_069128),
    (0x06912C, i_06912c),
    (0x069130, i_069130),
    (0x069134, i_069134),
    (0x069138, i_069138),
    (0x06913A, i_06913a),
    (0x06913C, i_06913c),
    (0x069140, i_069140),
    (0x069142, i_069142),
    (0x069144, i_069144),
    (0x069148, i_069148),
    (0x06914E, i_06914e),
    (0x069152, i_069152),
    (0x069158, i_069158),
    (0x06915E, i_06915e),
    (0x069162, i_069162),
    (0x069164, i_069164),
    (0x06916A, i_06916a),
    (0x069170, i_069170),
    (0x069176, i_069176),
    (0x06917A, i_06917a),
    (0x06917E, i_06917e),
    (0x069180, i_069180),
    (0x069186, i_069186),
    (0x06918C, i_06918c),
    (0x069190, i_069190),
    (0x069196, i_069196),
    (0x06919A, i_06919a),
    (0x06919E, i_06919e),
    (0x0691A2, i_0691a2),
    (0x0691A8, i_0691a8),
    (0x0691AE, i_0691ae),
    (0x0691B2, i_0691b2),
    (0x0691B6, i_0691b6),
    (0x0691BA, i_0691ba),
    (0x0691BE, i_0691be),
    (0x0691C2, i_0691c2),
    (0x0691C6, i_0691c6),
    (0x0691CA, i_0691ca),
    (0x0691CC, i_0691cc),
    (0x0691D0, i_0691d0),
    (0x0691D4, i_0691d4),
    (0x0691DC, i_0691dc),
    (0x0691E0, i_0691e0),
    (0x0691E4, i_0691e4),
    (0x0691E8, i_0691e8),
    (0x0691EC, i_0691ec),
    (0x0691EE, i_0691ee),
    (0x0691F0, i_0691f0),
    (0x0691F4, i_0691f4),
    (0x0691F6, i_0691f6),
    (0x0691F8, i_0691f8),
    (0x0691FC, i_0691fc),
    (0x069202, i_069202),
    (0x069206, i_069206),
    (0x06920A, i_06920a),
    (0x06920E, i_06920e),
    (0x069212, i_069212),
    (0x069214, i_069214),
    (0x069216, i_069216),
    (0x06921A, i_06921a),
    (0x06921C, i_06921c),
    (0x06921E, i_06921e),
    (0x069222, i_069222),
    (0x069228, i_069228),
    (0x06922C, i_06922c),
    (0x069232, i_069232),
    (0x069238, i_069238),
    (0x06923C, i_06923c),
    (0x069244, i_069244),
    (0x069248, i_069248),
    (0x06924C, i_06924c),
    (0x069250, i_069250),
    (0x069254, i_069254),
    (0x069256, i_069256),
    (0x069258, i_069258),
    (0x06925C, i_06925c),
    (0x06925E, i_06925e),
    (0x069260, i_069260),
    (0x069264, i_069264),
    (0x06926A, i_06926a),
    (0x06926E, i_06926e),
    (0x069272, i_069272),
    (0x069276, i_069276),
    (0x06927A, i_06927a),
    (0x06927C, i_06927c),
    (0x06927E, i_06927e),
    (0x069282, i_069282),
    (0x069284, i_069284),
    (0x069286, i_069286),
    (0x06928A, i_06928a),
    (0x069290, i_069290),
    (0x069294, i_069294),
    (0x06929A, i_06929a),
    (0x0692A0, i_0692a0),
    (0x0692A6, i_0692a6),
    (0x0692AA, i_0692aa),
    (0x0692AC, i_0692ac),
    (0x0692B2, i_0692b2),
    (0x0692B8, i_0692b8),
    (0x0692BE, i_0692be),
    (0x0692C2, i_0692c2),
    (0x0692C6, i_0692c6),
    (0x0692CA, i_0692ca),
    (0x0692CC, i_0692cc),
    (0x0692D2, i_0692d2),
    (0x0692D6, i_0692d6),
    (0x0692DC, i_0692dc),
    (0x0692E2, i_0692e2),
    (0x0692E6, i_0692e6),
    (0x0692EC, i_0692ec),
    (0x0692F2, i_0692f2),
    (0x0692F6, i_0692f6),
    (0x0692FC, i_0692fc),
    (0x069300, i_069300),
    (0x069308, i_069308),
    (0x06930C, i_06930c),
    (0x069310, i_069310),
    (0x069314, i_069314),
    (0x069318, i_069318),
    (0x06931A, i_06931a),
    (0x06931C, i_06931c),
    (0x069320, i_069320),
    (0x069322, i_069322),
    (0x069324, i_069324),
    (0x069328, i_069328),
    (0x06932E, i_06932e),
    (0x069332, i_069332),
    (0x069336, i_069336),
    (0x06933A, i_06933a),
    (0x06933E, i_06933e),
    (0x069340, i_069340),
    (0x069342, i_069342),
    (0x069346, i_069346),
    (0x069348, i_069348),
    (0x06934A, i_06934a),
    (0x06934E, i_06934e),
    (0x069354, i_069354),
    (0x069358, i_069358),
    (0x06935E, i_06935e),
    (0x069364, i_069364),
    (0x069368, i_069368),
    (0x069370, i_069370),
    (0x069374, i_069374),
    (0x069378, i_069378),
    (0x06937C, i_06937c),
    (0x069380, i_069380),
    (0x069382, i_069382),
    (0x069384, i_069384),
    (0x069388, i_069388),
    (0x06938A, i_06938a),
    (0x06938C, i_06938c),
    (0x069390, i_069390),
    (0x069396, i_069396),
    (0x06939A, i_06939a),
    (0x06939E, i_06939e),
    (0x0693A2, i_0693a2),
    (0x0693A6, i_0693a6),
    (0x0693A8, i_0693a8),
    (0x0693AA, i_0693aa),
    (0x0693AE, i_0693ae),
    (0x0693B0, i_0693b0),
    (0x0693B2, i_0693b2),
    (0x0693B6, i_0693b6),
    (0x0693BC, i_0693bc),
    (0x0693C0, i_0693c0),
    (0x0693C6, i_0693c6),
    (0x0693CC, i_0693cc),
    (0x0693D0, i_0693d0),
    (0x0693D2, i_0693d2),
    (0x0693D8, i_0693d8),
    (0x0693DE, i_0693de),
    (0x0693E4, i_0693e4),
    (0x0693E8, i_0693e8),
    (0x0693EE, i_0693ee),
    (0x0693F2, i_0693f2),
    (0x0693F6, i_0693f6),
    (0x0693F8, i_0693f8),
    (0x0693FE, i_0693fe),
    (0x069402, i_069402),
    (0x069406, i_069406),
    (0x06940C, i_06940c),
    (0x069410, i_069410),
    (0x069414, i_069414),
    (0x06941A, i_06941a),
    (0x06941E, i_06941e),
    (0x069422, i_069422),
    (0x069424, i_069424),
    (0x06942A, i_06942a),
    (0x06942E, i_06942e),
    (0x069434, i_069434),
    (0x06943A, i_06943a),
    (0x069440, i_069440),
    (0x069444, i_069444),
    (0x06944A, i_06944a),
    (0x06944E, i_06944e),
    (0x069452, i_069452),
    (0x06945A, i_06945a),
    (0x06945E, i_06945e),
    (0x069462, i_069462),
    (0x069466, i_069466),
    (0x06946A, i_06946a),
    (0x06946C, i_06946c),
    (0x06946E, i_06946e),
    (0x069472, i_069472),
    (0x069474, i_069474),
    (0x069476, i_069476),
    (0x06947A, i_06947a),
    (0x069480, i_069480),
    (0x069484, i_069484),
    (0x069488, i_069488),
    (0x06948C, i_06948c),
    (0x069490, i_069490),
    (0x069492, i_069492),
    (0x069494, i_069494),
    (0x069498, i_069498),
    (0x06949A, i_06949a),
    (0x06949C, i_06949c),
    (0x0694A0, i_0694a0),
    (0x0694A6, i_0694a6),
    (0x0694AA, i_0694aa),
    (0x0694B0, i_0694b0),
    (0x0694B6, i_0694b6),
    (0x0694BA, i_0694ba),
    (0x0694BC, i_0694bc),
    (0x0694C2, i_0694c2),
    (0x0694C8, i_0694c8),
    (0x0694CE, i_0694ce),
    (0x0694D2, i_0694d2),
    (0x0694D6, i_0694d6),
    (0x0694D8, i_0694d8),
    (0x0694DC, i_0694dc),
    (0x0694E2, i_0694e2),
    (0x0694E8, i_0694e8),
    (0x0694EE, i_0694ee),
    (0x0694F2, i_0694f2),
    (0x0694F6, i_0694f6),
    (0x0694FA, i_0694fa),
    (0x069502, i_069502),
    (0x069506, i_069506),
    (0x06950A, i_06950a),
    (0x06950E, i_06950e),
    (0x069512, i_069512),
    (0x069514, i_069514),
    (0x069516, i_069516),
    (0x06951A, i_06951a),
    (0x06951C, i_06951c),
    (0x06951E, i_06951e),
    (0x069522, i_069522),
    (0x069528, i_069528),
    (0x06952C, i_06952c),
    (0x069530, i_069530),
    (0x069534, i_069534),
    (0x069538, i_069538),
    (0x06953A, i_06953a),
    (0x06953C, i_06953c),
    (0x069540, i_069540),
    (0x069542, i_069542),
    (0x069544, i_069544),
    (0x069548, i_069548),
    (0x06954E, i_06954e),
    (0x069552, i_069552),
    (0x069558, i_069558),
    (0x06955E, i_06955e),
    (0x069562, i_069562),
    (0x06956A, i_06956a),
    (0x06956E, i_06956e),
    (0x069572, i_069572),
    (0x069576, i_069576),
    (0x06957A, i_06957a),
    (0x06957C, i_06957c),
    (0x06957E, i_06957e),
    (0x069582, i_069582),
    (0x069584, i_069584),
    (0x069586, i_069586),
    (0x06958A, i_06958a),
    (0x069590, i_069590),
    (0x069594, i_069594),
    (0x069598, i_069598),
    (0x06959C, i_06959c),
    (0x0695A0, i_0695a0),
    (0x0695A2, i_0695a2),
    (0x0695A4, i_0695a4),
    (0x0695A8, i_0695a8),
    (0x0695AA, i_0695aa),
    (0x0695AC, i_0695ac),
    (0x0695B0, i_0695b0),
    (0x0695B6, i_0695b6),
    (0x0695BA, i_0695ba),
    (0x0695C0, i_0695c0),
    (0x0695C6, i_0695c6),
    (0x0695CA, i_0695ca),
    (0x0695D2, i_0695d2),
    (0x0695D6, i_0695d6),
    (0x0695DA, i_0695da),
    (0x0695DE, i_0695de),
    (0x0695E2, i_0695e2),
    (0x0695E4, i_0695e4),
    (0x0695E6, i_0695e6),
    (0x0695EA, i_0695ea),
    (0x0695EC, i_0695ec),
    (0x0695EE, i_0695ee),
    (0x0695F2, i_0695f2),
    (0x0695F8, i_0695f8),
    (0x0695FC, i_0695fc),
    (0x069600, i_069600),
    (0x069604, i_069604),
    (0x069608, i_069608),
    (0x06960A, i_06960a),
    (0x06960C, i_06960c),
    (0x069610, i_069610),
    (0x069612, i_069612),
    (0x069614, i_069614),
    (0x069618, i_069618),
    (0x06961E, i_06961e),
    (0x069622, i_069622),
    (0x069628, i_069628),
    (0x06962E, i_06962e),
    (0x069634, i_069634),
    (0x069638, i_069638),
    (0x06963E, i_06963e),
    (0x069642, i_069642),
    (0x069644, i_069644),
    (0x06964A, i_06964a),
    (0x069650, i_069650),
    (0x069656, i_069656),
    (0x06965A, i_06965a),
    (0x06965E, i_06965e),
    (0x069660, i_069660),
    (0x069664, i_069664),
    (0x069668, i_069668),
    (0x06966E, i_06966e),
    (0x069672, i_069672),
    (0x069676, i_069676),
    (0x069678, i_069678),
    (0x06967E, i_06967e),
    (0x069682, i_069682),
    (0x069688, i_069688),
    (0x06968E, i_06968e),
    (0x069694, i_069694),
    (0x069698, i_069698),
    (0x06969C, i_06969c),
    (0x0696A0, i_0696a0),
    (0x0696A4, i_0696a4),
    (0x0696AC, i_0696ac),
    (0x0696B0, i_0696b0),
    (0x0696B4, i_0696b4),
    (0x0696B8, i_0696b8),
    (0x0696BC, i_0696bc),
    (0x0696BE, i_0696be),
    (0x0696C0, i_0696c0),
    (0x0696C4, i_0696c4),
    (0x0696C6, i_0696c6),
    (0x0696C8, i_0696c8),
    (0x0696CC, i_0696cc),
    (0x0696D2, i_0696d2),
    (0x0696D6, i_0696d6),
    (0x0696DA, i_0696da),
    (0x0696DE, i_0696de),
    (0x0696E2, i_0696e2),
    (0x0696E4, i_0696e4),
    (0x0696E6, i_0696e6),
    (0x0696EA, i_0696ea),
    (0x0696EC, i_0696ec),
    (0x0696EE, i_0696ee),
    (0x0696F2, i_0696f2),
    (0x0696F8, i_0696f8),
    (0x0696FC, i_0696fc),
    (0x069702, i_069702),
    (0x069708, i_069708),
    (0x06970C, i_06970c),
    (0x069710, i_069710),
    (0x069714, i_069714),
    (0x069718, i_069718),
    (0x069720, i_069720),
    (0x069724, i_069724),
    (0x069728, i_069728),
    (0x06972C, i_06972c),
    (0x069730, i_069730),
    (0x069732, i_069732),
    (0x069734, i_069734),
    (0x069738, i_069738),
    (0x06973A, i_06973a),
    (0x06973C, i_06973c),
    (0x069740, i_069740),
    (0x069746, i_069746),
    (0x06974A, i_06974a),
    (0x06974E, i_06974e),
    (0x069752, i_069752),
    (0x069756, i_069756),
    (0x069758, i_069758),
    (0x06975A, i_06975a),
    (0x06975E, i_06975e),
    (0x069760, i_069760),
    (0x069762, i_069762),
    (0x069766, i_069766),
    (0x06976C, i_06976c),
    (0x069770, i_069770),
    (0x069776, i_069776),
    (0x06977C, i_06977c),
    (0x069782, i_069782),
    (0x069788, i_069788),
    (0x06978C, i_06978c),
    (0x06978E, i_06978e),
    (0x069794, i_069794),
    (0x069798, i_069798),
    (0x06979E, i_06979e),
    (0x0697A2, i_0697a2),
    (0x0697A6, i_0697a6),
    (0x0697AA, i_0697aa),
    (0x0697AE, i_0697ae),
    (0x0697B0, i_0697b0),
    (0x0697B6, i_0697b6),
    (0x0697BC, i_0697bc),
    (0x0697C2, i_0697c2),
    (0x0697C6, i_0697c6),
    (0x0697CA, i_0697ca),
    (0x0697CC, i_0697cc),
    (0x0697CE, i_0697ce),
    (0x0697D2, i_0697d2),
    (0x0697D8, i_0697d8),
    (0x0697DA, i_0697da),
    (0x0697E0, i_0697e0),
    (0x0697E4, i_0697e4),
    (0x0697E8, i_0697e8),
    (0x0697EA, i_0697ea),
    (0x0697EE, i_0697ee),
    (0x0697F4, i_0697f4),
    (0x0697F8, i_0697f8),
    (0x0697FE, i_0697fe),
    (0x069804, i_069804),
    (0x06980A, i_06980a),
    (0x069810, i_069810),
    (0x069816, i_069816),
    (0x06981A, i_06981a),
    (0x06981C, i_06981c),
    (0x069822, i_069822),
    (0x069826, i_069826),
    (0x06982A, i_06982a),
    (0x06982C, i_06982c),
    (0x069832, i_069832),
    (0x069836, i_069836),
    (0x06983A, i_06983a),
    (0x06983C, i_06983c),
    (0x069842, i_069842),
    (0x069846, i_069846),
    (0x06984A, i_06984a),
    (0x06984C, i_06984c),
    (0x069852, i_069852),
    (0x069858, i_069858),
    (0x06985E, i_06985e),
    (0x069862, i_069862),
    (0x069866, i_069866),
    (0x069868, i_069868),
    (0x06986A, i_06986a),
    (0x069870, i_069870),
    (0x069872, i_069872),
    (0x069878, i_069878),
    (0x06987E, i_06987e),
    (0x069880, i_069880),
    (0x069884, i_069884),
    (0x06988A, i_06988a),
    (0x06988E, i_06988e),
    (0x069890, i_069890),
    (0x069894, i_069894),
    (0x069898, i_069898),
    (0x06989C, i_06989c),
    (0x0698A2, i_0698a2),
    (0x0698A8, i_0698a8),
    (0x0698AA, i_0698aa),
    (0x0698AE, i_0698ae),
    (0x0698B2, i_0698b2),
    (0x0698B6, i_0698b6),
    (0x0698BC, i_0698bc),
    (0x0698C0, i_0698c0),
    (0x0698C4, i_0698c4),
    (0x0698C8, i_0698c8),
    (0x0698CC, i_0698cc),
    (0x0698D2, i_0698d2),
    (0x0698D8, i_0698d8),
    (0x0698DC, i_0698dc),
    (0x0698E2, i_0698e2),
    (0x0698E8, i_0698e8),
    (0x0698EC, i_0698ec),
    (0x0698EE, i_0698ee),
    (0x0698F2, i_0698f2),
    (0x0698F6, i_0698f6),
    (0x0698FC, i_0698fc),
    (0x069902, i_069902),
    (0x069906, i_069906),
    (0x06990A, i_06990a),
    (0x06990C, i_06990c),
    (0x069912, i_069912),
    (0x069916, i_069916),
    (0x06991A, i_06991a),
    (0x06991C, i_06991c),
    (0x069922, i_069922),
    (0x069924, i_069924),
    (0x06992A, i_06992a),
    (0x069930, i_069930),
    (0x069936, i_069936),
    (0x06993A, i_06993a),
    (0x06993C, i_06993c),
    (0x069940, i_069940),
    (0x069944, i_069944),
    (0x069948, i_069948),
    (0x06994E, i_06994e),
    (0x069954, i_069954),
    (0x069958, i_069958),
    (0x06995C, i_06995c),
    (0x06995E, i_06995e),
    (0x069964, i_069964),
    (0x069966, i_069966),
    (0x06996C, i_06996c),
    (0x069972, i_069972),
    (0x069978, i_069978),
    (0x06997E, i_06997e),
    (0x069982, i_069982),
    (0x069984, i_069984),
    (0x06998A, i_06998a),
    (0x069990, i_069990),
    (0x069994, i_069994),
    (0x069998, i_069998),
    (0x06999A, i_06999a),
    (0x0699A0, i_0699a0),
    (0x0699A6, i_0699a6),
    (0x0699AA, i_0699aa),
    (0x0699AE, i_0699ae),
    (0x0699B0, i_0699b0),
    (0x0699B2, i_0699b2),
    (0x0699B6, i_0699b6),
    (0x0699BC, i_0699bc),
    (0x0699C2, i_0699c2),
    (0x0699C8, i_0699c8),
    (0x0699CC, i_0699cc),
    (0x0699D2, i_0699d2),
    (0x0699D6, i_0699d6),
    (0x0699DC, i_0699dc),
    (0x0699E0, i_0699e0),
    (0x0699E6, i_0699e6),
    (0x0699EA, i_0699ea),
    (0x0699F0, i_0699f0),
    (0x0699F6, i_0699f6),
    (0x0699FC, i_0699fc),
    (0x069A02, i_069a02),
    (0x069A08, i_069a08),
    (0x069A0E, i_069a0e),
    (0x069A12, i_069a12),
    (0x069A18, i_069a18),
    (0x069A1C, i_069a1c),
    (0x069A20, i_069a20),
    (0x069A26, i_069a26),
    (0x069A2C, i_069a2c),
    (0x069A30, i_069a30),
    (0x069A32, i_069a32),
    (0x069A36, i_069a36),
    (0x069A3C, i_069a3c),
    (0x069A42, i_069a42),
    (0x069A48, i_069a48),
    (0x069A4E, i_069a4e),
    (0x069A52, i_069a52),
    (0x069A56, i_069a56),
    (0x069A58, i_069a58),
    (0x069A5C, i_069a5c),
    (0x069A62, i_069a62),
    (0x069A68, i_069a68),
    (0x069A6E, i_069a6e),
    (0x069A74, i_069a74),
    (0x069A78, i_069a78),
    (0x069A7E, i_069a7e),
    (0x069A84, i_069a84),
    (0x069A8A, i_069a8a),
    (0x069A90, i_069a90),
    (0x069A96, i_069a96),
    (0x069A9A, i_069a9a),
    (0x069AA0, i_069aa0),
    (0x069AA4, i_069aa4),
    (0x069AAA, i_069aaa),
    (0x069AAE, i_069aae),
    (0x069AB4, i_069ab4),
    (0x069AB8, i_069ab8),
    (0x069ABE, i_069abe),
    (0x069AC2, i_069ac2),
    (0x069AC6, i_069ac6),
    (0x069ACC, i_069acc),
    (0x069AD0, i_069ad0),
    (0x069AD4, i_069ad4),
    (0x069AD6, i_069ad6),
    (0x069ADC, i_069adc),
    (0x069AE0, i_069ae0),
    (0x069AE4, i_069ae4),
    (0x069AE6, i_069ae6),
    (0x069AE8, i_069ae8),
    (0x069AEC, i_069aec),
    (0x069AF4, i_069af4),
    (0x069AF8, i_069af8),
    (0x069AFE, i_069afe),
    (0x069B02, i_069b02),
    (0x069B08, i_069b08),
    (0x069B0C, i_069b0c),
    (0x069B12, i_069b12),
    (0x069B18, i_069b18),
    (0x069B1C, i_069b1c),
    (0x069B20, i_069b20),
    (0x069B22, i_069b22),
    (0x069B24, i_069b24),
    (0x069B26, i_069b26),
    (0x069B28, i_069b28),
    (0x069B2E, i_069b2e),
    (0x069B34, i_069b34),
    (0x069B3A, i_069b3a),
    (0x069B40, i_069b40),
    (0x069B44, i_069b44),
    (0x069B4A, i_069b4a),
    (0x069B50, i_069b50),
    (0x069B56, i_069b56),
    (0x069B5A, i_069b5a),
    (0x069B60, i_069b60),
    (0x069B64, i_069b64),
    (0x069B6A, i_069b6a),
    (0x069B70, i_069b70),
    (0x069B76, i_069b76),
    (0x069B7A, i_069b7a),
    (0x069B80, i_069b80),
    (0x069B86, i_069b86),
    (0x069B8C, i_069b8c),
    (0x069B90, i_069b90),
    (0x069B92, i_069b92),
    (0x069B98, i_069b98),
    (0x069B9E, i_069b9e),
    (0x069BA2, i_069ba2),
    (0x069BA6, i_069ba6),
    (0x069BA8, i_069ba8),
    (0x069BAA, i_069baa),
    (0x069BAE, i_069bae),
    (0x069BB4, i_069bb4),
    (0x069BBA, i_069bba),
    (0x069BC0, i_069bc0),
    (0x069BC8, i_069bc8),
    (0x069BCE, i_069bce),
    (0x069BD4, i_069bd4),
    (0x069BD8, i_069bd8),
    (0x069BDE, i_069bde),
    (0x069BE4, i_069be4),
    (0x069BEA, i_069bea),
    (0x069BEE, i_069bee),
    (0x069BF4, i_069bf4),
    (0x069BFA, i_069bfa),
    (0x069C00, i_069c00),
    (0x069C06, i_069c06),
    (0x069C0C, i_069c0c),
    (0x069C10, i_069c10),
    (0x069C12, i_069c12),
    (0x069C18, i_069c18),
    (0x069C1E, i_069c1e),
    (0x069C24, i_069c24),
    (0x069C26, i_069c26),
    (0x069C2C, i_069c2c),
    (0x069C32, i_069c32),
    (0x069C38, i_069c38),
    (0x069C3E, i_069c3e),
    (0x069C42, i_069c42),
    (0x069C46, i_069c46),
    (0x069C48, i_069c48),
    (0x069C4C, i_069c4c),
    (0x069C52, i_069c52),
    (0x069C56, i_069c56),
    (0x069C5A, i_069c5a),
    (0x069C5C, i_069c5c),
    (0x069C60, i_069c60),
    (0x069C66, i_069c66),
    (0x069C6A, i_069c6a),
    (0x069C6E, i_069c6e),
    (0x069C70, i_069c70),
    (0x069C76, i_069c76),
    (0x069C7A, i_069c7a),
    (0x069C7E, i_069c7e),
    (0x069C80, i_069c80),
    (0x069C86, i_069c86),
    (0x069C8C, i_069c8c),
    (0x069C92, i_069c92),
    (0x069C96, i_069c96),
    (0x069C9A, i_069c9a),
    (0x069C9C, i_069c9c),
    (0x069CA2, i_069ca2),
    (0x069CA8, i_069ca8),
    (0x069CAC, i_069cac),
    (0x069CB0, i_069cb0),
    (0x069CB2, i_069cb2),
    (0x069CB8, i_069cb8),
    (0x069CBE, i_069cbe),
    (0x069CC4, i_069cc4),
    (0x069CC8, i_069cc8),
    (0x069CCC, i_069ccc),
    (0x069CCE, i_069cce),
    (0x069CD0, i_069cd0),
    (0x069CD6, i_069cd6),
    (0x069CDC, i_069cdc),
    (0x069CE0, i_069ce0),
    (0x069CE4, i_069ce4),
    (0x069CE6, i_069ce6),
    (0x069CEA, i_069cea),
    (0x069CEE, i_069cee),
    (0x069CF2, i_069cf2),
    (0x069CF6, i_069cf6),
    (0x069CFA, i_069cfa),
    (0x069CFE, i_069cfe),
    (0x069D04, i_069d04),
    (0x069D06, i_069d06),
    (0x069D0A, i_069d0a),
    (0x069D0C, i_069d0c),
    (0x069D10, i_069d10),
    (0x069D14, i_069d14),
    (0x069D18, i_069d18),
    (0x069D1C, i_069d1c),
    (0x069D20, i_069d20),
    (0x069D22, i_069d22),
    (0x069D26, i_069d26),
    (0x069D2C, i_069d2c),
    (0x069D30, i_069d30),
    (0x069D34, i_069d34),
    (0x069D3A, i_069d3a),
    (0x069D3E, i_069d3e),
    (0x069D44, i_069d44),
    (0x069D4A, i_069d4a),
    (0x069D50, i_069d50),
    (0x069D58, i_069d58),
    (0x069D5E, i_069d5e),
    (0x069D62, i_069d62),
    (0x069D66, i_069d66),
    (0x069D6A, i_069d6a),
    (0x069D6E, i_069d6e),
    (0x069D74, i_069d74),
    (0x069D76, i_069d76),
    (0x069D78, i_069d78),
    (0x069D7E, i_069d7e),
    (0x069D82, i_069d82),
    (0x069D84, i_069d84),
    (0x069D88, i_069d88),
    (0x069D8A, i_069d8a),
    (0x069D8C, i_069d8c),
    (0x069D90, i_069d90),
    (0x069D96, i_069d96),
    (0x069D9A, i_069d9a),
    (0x069DA0, i_069da0),
    (0x069DA2, i_069da2),
    (0x069DA4, i_069da4),
    (0x069DAA, i_069daa),
    (0x069DAE, i_069dae),
    (0x069DB0, i_069db0),
    (0x069DB2, i_069db2),
    (0x069DB4, i_069db4),
    (0x069DB8, i_069db8),
    (0x069DBA, i_069dba),
    (0x069DBC, i_069dbc),
    (0x069DC2, i_069dc2),
    (0x069DC6, i_069dc6),
    (0x069DC8, i_069dc8),
    (0x069DCC, i_069dcc),
    (0x069DCE, i_069dce),
    (0x069DD2, i_069dd2),
    (0x069DD4, i_069dd4),
    (0x069DDA, i_069dda),
    (0x069DDC, i_069ddc),
    (0x069DE0, i_069de0),
    (0x069DE4, i_069de4),
    (0x069DE8, i_069de8),
    (0x069DEC, i_069dec),
    (0x069DEE, i_069dee),
    (0x069DF2, i_069df2),
    (0x069DF4, i_069df4),
    (0x069DF8, i_069df8),
    (0x069DFC, i_069dfc),
    (0x069E00, i_069e00),
    (0x069E04, i_069e04),
    (0x069E08, i_069e08),
    (0x069E0A, i_069e0a),
    (0x069E0C, i_069e0c),
    (0x069E10, i_069e10),
    (0x069E12, i_069e12),
    (0x069E14, i_069e14),
    (0x069E18, i_069e18),
    (0x069E1C, i_069e1c),
    (0x069E20, i_069e20),
    (0x069E26, i_069e26),
    (0x069E2A, i_069e2a),
    (0x069E2E, i_069e2e),
    (0x069E30, i_069e30),
    (0x069E34, i_069e34),
    (0x069E36, i_069e36),
    (0x069E38, i_069e38),
    (0x069E3C, i_069e3c),
    (0x069E40, i_069e40),
    (0x069E42, i_069e42),
    (0x069E46, i_069e46),
    (0x069E4A, i_069e4a),
    (0x069E4E, i_069e4e),
    (0x069E50, i_069e50),
    (0x069E54, i_069e54),
    (0x069E5A, i_069e5a),
    (0x069E5C, i_069e5c),
    (0x069E60, i_069e60),
    (0x069E64, i_069e64),
    (0x069E66, i_069e66),
    (0x069E6C, i_069e6c),
    (0x069E72, i_069e72),
    (0x069E76, i_069e76),
    (0x069E7A, i_069e7a),
    (0x069E7E, i_069e7e),
    (0x069E80, i_069e80),
    (0x069E82, i_069e82),
    (0x069E84, i_069e84),
    (0x069E86, i_069e86),
    (0x069E8A, i_069e8a),
    (0x069E8E, i_069e8e),
    (0x069E90, i_069e90),
    (0x069E96, i_069e96),
    (0x069E9A, i_069e9a),
    (0x069E9E, i_069e9e),
    (0x069EA4, i_069ea4),
    (0x069EA8, i_069ea8),
    (0x069EAA, i_069eaa),
    (0x069EB0, i_069eb0),
    (0x069EB6, i_069eb6),
    (0x069EBC, i_069ebc),
    (0x069EBE, i_069ebe),
    (0x069EC4, i_069ec4),
    (0x069EC8, i_069ec8),
    (0x069ECC, i_069ecc),
    (0x069ED2, i_069ed2),
    (0x069ED4, i_069ed4),
    (0x069EDA, i_069eda),
    (0x069EDE, i_069ede),
    (0x069EE2, i_069ee2),
    (0x069EE6, i_069ee6),
    (0x069EE8, i_069ee8),
    (0x069EEC, i_069eec),
    (0x069EF0, i_069ef0),
    (0x069EF6, i_069ef6),
    (0x069EF8, i_069ef8),
    (0x069EFE, i_069efe),
    (0x069F04, i_069f04),
    (0x069F0A, i_069f0a),
    (0x069F10, i_069f10),
    (0x069F12, i_069f12),
    (0x069F16, i_069f16),
    (0x069F1C, i_069f1c),
    (0x069F22, i_069f22),
    (0x069F28, i_069f28),
    (0x069F2E, i_069f2e),
    (0x069F32, i_069f32),
    (0x069F36, i_069f36),
    (0x069F3C, i_069f3c),
    (0x069F42, i_069f42),
    (0x069F48, i_069f48),
    (0x069F4E, i_069f4e),
    (0x069F52, i_069f52),
    (0x069F54, i_069f54),
    (0x069F56, i_069f56),
    (0x069F58, i_069f58),
    (0x069F5C, i_069f5c),
    (0x069F60, i_069f60),
    (0x069F66, i_069f66),
    (0x069F68, i_069f68),
    (0x069F6C, i_069f6c),
    (0x069F72, i_069f72),
    (0x069F76, i_069f76),
    (0x069F7C, i_069f7c),
    (0x069F82, i_069f82),
    (0x069F88, i_069f88),
    (0x069F8C, i_069f8c),
    (0x069F90, i_069f90),
    (0x069F94, i_069f94),
    (0x069F98, i_069f98),
    (0x069F9E, i_069f9e),
    (0x069FA4, i_069fa4),
    (0x069FAA, i_069faa),
    (0x069FAE, i_069fae),
    (0x069FB2, i_069fb2),
    (0x069FB6, i_069fb6),
    (0x069FB8, i_069fb8),
    (0x069FBC, i_069fbc),
    (0x069FC2, i_069fc2),
    (0x069FC6, i_069fc6),
    (0x069FCC, i_069fcc),
    (0x069FD2, i_069fd2),
    (0x069FD6, i_069fd6),
    (0x069FDC, i_069fdc),
    (0x069FE2, i_069fe2),
    (0x069FE8, i_069fe8),
    (0x069FEC, i_069fec),
    (0x069FF0, i_069ff0),
    (0x069FF4, i_069ff4),
    (0x069FF6, i_069ff6),
    (0x069FFA, i_069ffa),
    (0x06A000, i_06a000),
    (0x06A006, i_06a006),
    (0x06A00C, i_06a00c),
    (0x06A010, i_06a010),
    (0x06A014, i_06a014),
    (0x06A01C, i_06a01c),
    (0x06A022, i_06a022),
    (0x06A028, i_06a028),
    (0x06A02E, i_06a02e),
    (0x06A032, i_06a032),
    (0x06A036, i_06a036),
    (0x06A03C, i_06a03c),
    (0x06A042, i_06a042),
];
