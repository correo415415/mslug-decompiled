// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $057780: lea.l $578dc(pc), a1
pub fn i_057780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x578DC;
    16
}
/// $057784: move.l a1, (a6)
pub fn i_057784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57786;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057786: btst.b #$6, $72(a6)
pub fn i_057786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5778C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05778C: beq.w $57796
pub fn i_05778c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57790;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57796; return 10; }
    8
}
/// $057790: lea.l $58a50(pc), a1
pub fn i_057790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57794;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A50;
    16
}
/// $057794: move.l a1, (a6)
pub fn i_057794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57796;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057796: btst.b #$5, $72(a6)
pub fn i_057796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5779C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05779C: beq.w $577a6
pub fn i_05779c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x577A6; return 10; }
    8
}
/// $0577A0: lea.l $5804c(pc), a1
pub fn i_0577a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $0577A4: move.l a1, (a6)
pub fn i_0577a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0577A6: bra.w $577b0
pub fn i_0577a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x577B0;
    10
}
/// $0577AA: lea.l $57582(pc), a1
pub fn i_0577aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57582;
    16
}
/// $0577AE: move.l a1, (a6)
pub fn i_0577ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0577B0: jsr $49fd0.l
pub fn i_0577b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x577B6);
    cpu.pc = 0x49FD0;
    16
}
// ===== JsrPcThunk_0577b6  ($0577B6, 6 B, thunks/jsr_pc_thunks.c)
/// $0577B6: jsr $56e1e(pc)
pub fn i_0577b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x577BA);
    cpu.pc = 0x56E1E;
    16
}
/// $0577BA: rts
pub fn i_0577ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SlugApproach_0577bc  ($0577BC, 124 B, enemies/soldiers/helpers_056acc.s)
/// $0577BC: bsr.w $56acc
pub fn i_0577bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577C0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x577C0);
    cpu.pc = 0x56ACC;
    18
}
/// $0577C0: bsr.w $56b92
pub fn i_0577c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x577C4);
    cpu.pc = 0x56B92;
    18
}
/// $0577C4: jsr $28d70.l
pub fn i_0577c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x577CA);
    cpu.pc = 0x28D70;
    16
}
/// $0577CA: move.w $80(a6), d1
pub fn i_0577ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577CE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0577CE: mulu.w d1, d1
pub fn i_0577ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577D0;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0577D0: lsr.l #$8, d1
pub fn i_0577d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577D2;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $0577D2: jsr $5e9b6.l
pub fn i_0577d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x577D8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0577D8: move.w d0, -(a7)
pub fn i_0577d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577DA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0577DA: andi.w #$ff, d0
pub fn i_0577da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577DE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0577DE: cmp.w d1, d0
pub fn i_0577de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E0;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0577E0: bcc.w $577e8
pub fn i_0577e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x577E8; return 10; }
    8
}
/// $0577E4: move.b (a7), d0
pub fn i_0577e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E6;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0577E6: cmp.w d1, d0
pub fn i_0577e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E8;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0577E8: addq.w #$2, a7
pub fn i_0577e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577EA;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0577EA: bcc.w $577f4
pub fn i_0577ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x577F4; return 10; }
    8
}
/// $0577EE: lea.l $5799e(pc), a1
pub fn i_0577ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5799E;
    16
}
/// $0577F2: move.l a1, (a6)
pub fn i_0577f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0577F4: btst.b #$0, $72(a6)
pub fn i_0577f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577FA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0577FA: bne.w $57804
pub fn i_0577fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57804; return 10; }
    8
}
/// $0577FE: lea.l $585f6(pc), a1
pub fn i_0577fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57802;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $057802: move.l a1, (a6)
pub fn i_057802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57804;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057804: tst.b $78(a6)
pub fn i_057804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57808;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $057808: bne.w $57812
pub fn i_057808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5780C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57812; return 10; }
    8
}
/// $05780C: lea.l $5799e(pc), a1
pub fn i_05780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57810;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5799E;
    16
}
/// $057810: move.l a1, (a6)
pub fn i_057810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57812;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057812: btst.b #$6, $72(a6)
pub fn i_057812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57818;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057818: beq.w $57822
pub fn i_057818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5781C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57822; return 10; }
    8
}
/// $05781C: lea.l $58a50(pc), a1
pub fn i_05781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57820;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A50;
    16
}
/// $057820: move.l a1, (a6)
pub fn i_057820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57822;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057822: btst.b #$5, $72(a6)
pub fn i_057822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57828;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057828: beq.w $57832
pub fn i_057828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5782C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57832; return 10; }
    8
}
/// $05782C: lea.l $5804c(pc), a1
pub fn i_05782c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57830;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $057830: move.l a1, (a6)
pub fn i_057830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57832;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057832: jsr $49fd0.l
pub fn i_057832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57838);
    cpu.pc = 0x49FD0;
    16
}
// ===== JsrPcThunk_057838  ($057838, 6 B, thunks/jsr_pc_thunks.c)
/// $057838: jsr $56e1e(pc)
pub fn i_057838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5783C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5783C);
    cpu.pc = 0x56E1E;
    16
}
/// $05783C: rts
pub fn i_05783c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5783E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnJumpIn_05783e  ($05783E, 66 B, enemies/soldiers/helpers_056acc.s)
/// $05783E: bsr.w $570a8
pub fn i_05783e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57842;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57842);
    cpu.pc = 0x570A8;
    18
}
/// $057842: bsr.w $5719c
pub fn i_057842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57846;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57846);
    cpu.pc = 0x5719C;
    18
}
/// $057846: lea.l $2b69d8.l, a0
pub fn i_057846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5784C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B69D8;
    20
}
/// $05784C: jsr $28cd4.l
pub fn i_05784c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57852);
    cpu.pc = 0x28CD4;
    16
}
/// $057852: move.w #$fe67, d0
pub fn i_057852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57856;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057856: jsr $5dca4.l
pub fn i_057856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5785C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5785C);
    cpu.pc = 0x5DCA4;
    16
}
/// $05785C: move.w d0, $28(a6)
pub fn i_05785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57860;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057860: move.w #$190, $2a(a6)
pub fn i_057860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57866;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057866: move.w #$ffd8, $2e(a6)
pub fn i_057866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5786C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05786C: move.w #$0, $2c(a6)
pub fn i_05786c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57872;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057872: clr.w $2a(a6)
pub fn i_057872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57876;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $057876: lea.l $58a82(pc), a1
pub fn i_057876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5787A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A82;
    16
}
/// $05787A: move.l a1, (a6)
pub fn i_05787a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5787C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05787C: bra.w $58a82
pub fn i_05787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57880;
    cpu.ir = 0x6000;
    cpu.pc = 0x58A82;
    10
}
// ===== Soldier_Leap_057880  ($057880, 542 B, enemies/soldiers/helpers_056acc.s)
/// $057880: bsr.w $570a8
pub fn i_057880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57884;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57884);
    cpu.pc = 0x570A8;
    18
}
/// $057884: bsr.w $5720e
pub fn i_057884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57888;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57888);
    cpu.pc = 0x5720E;
    18
}
/// $057888: bsr.w $5721e
pub fn i_057888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5788C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5788C);
    cpu.pc = 0x5721E;
    18
}
/// $05788C: bsr.w $57216
pub fn i_05788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57890;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57890);
    cpu.pc = 0x57216;
    18
}
/// $057890: lea.l $2b69aa.l, a0
pub fn i_057890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57896;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B69AA;
    20
}
/// $057896: jsr $28cd4.l
pub fn i_057896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5789C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5789C);
    cpu.pc = 0x28CD4;
    16
}
/// $05789C: move.w #$fe67, d0
pub fn i_05789c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0578A0: jsr $5dca4.l
pub fn i_0578a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x578A6);
    cpu.pc = 0x5DCA4;
    16
}
/// $0578A6: move.w d0, $28(a6)
pub fn i_0578a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578AA: move.w #$190, $2a(a6)
pub fn i_0578aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578B0: move.w #$ffd8, $2e(a6)
pub fn i_0578b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578B6: move.w #$0, $2c(a6)
pub fn i_0578b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578BC: clr.w $2a(a6)
pub fn i_0578bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578C0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0578C0: bsr.w $56b92
pub fn i_0578c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x578C4);
    cpu.pc = 0x56B92;
    18
}
/// $0578C4: bsr.w $56eda
pub fn i_0578c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578C8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x578C8);
    cpu.pc = 0x56EDA;
    18
}
/// $0578C8: bcc.w $57908
pub fn i_0578c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57908; return 10; }
    8
}
/// $0578CC: lea.l $2b6fee.l, a0
pub fn i_0578cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6FEE;
    20
}
/// $0578D2: jsr $28cd4.l
pub fn i_0578d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x578D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0578D8: bra.w $57a5a
pub fn i_0578d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A5A;
    10
}
/// $057908: move.l $7a(a6), d0
pub fn i_057908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5790C;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05790C: beq.w $5793e
pub fn i_05790c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57910;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5793E; return 10; }
    8
}
/// $057910: cmpi.l #$100440, d0
pub fn i_057910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57916;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $057916: bne.w $5792e
pub fn i_057916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5791A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5792E; return 10; }
    8
}
/// $05791A: lea.l $2b71dc.l, a0
pub fn i_05791a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57920;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B71DC;
    20
}
/// $057920: move.l a0, $4c(a6)
pub fn i_057920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57924;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057924: jsr $283ca.l
pub fn i_057924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5792A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5792A);
    cpu.pc = 0x283CA;
    16
}
/// $05792A: bra.w $5793e
pub fn i_05792a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5792E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5793E;
    10
}
/// $05792E: lea.l $2b7230.l, a0
pub fn i_05792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57934;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7230;
    20
}
/// $057934: move.l a0, $4c(a6)
pub fn i_057934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57938;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057938: jsr $283ca.l
pub fn i_057938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5793E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5793E);
    cpu.pc = 0x283CA;
    16
}
/// $05793E: lea.l $57944(pc), a1
pub fn i_05793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57944;
    16
}
/// $057942: move.l a1, (a6)
pub fn i_057942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057944: jsr $27d50.l
pub fn i_057944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5794A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5794A);
    cpu.pc = 0x27D50;
    16
}
/// $05794A: bcc.w $57954
pub fn i_05794a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5794E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57954; return 10; }
    8
}
/// $05794E: lea.l $58464(pc), a1
pub fn i_05794e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57952;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58464;
    16
}
/// $057952: move.l a1, (a6)
pub fn i_057952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57954;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057954: jsr $28d70.l
pub fn i_057954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5795A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5795A);
    cpu.pc = 0x28D70;
    16
}
/// $05795A: bsr.w $56e4a
pub fn i_05795a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5795E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5795E);
    cpu.pc = 0x56E4A;
    18
}
/// $05795E: bcc.w $57994
pub fn i_05795e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57962;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57994; return 10; }
    8
}
/// $057962: tst.w $2a(a6)
pub fn i_057962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57966;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $057966: bpl.w $57994
pub fn i_057966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5796A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57994; return 10; }
    8
}
/// $05796A: jsr $283d8.l
pub fn i_05796a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57970);
    cpu.pc = 0x283D8;
    16
}
/// $057970: btst.b #$1, $13(a6)
pub fn i_057970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57976;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057976: beq.w $57994
pub fn i_057976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5797A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57994; return 10; }
    8
}
/// $05797A: movea.l $7a(a6), a0
pub fn i_05797a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5797E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05797E: move.w $24(a0), d0
pub fn i_05797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57982;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057982: addi.w #$10, d0
pub fn i_057982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57986;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057986: cmp.w $24(a6), d0
pub fn i_057986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5798A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05798A: bpl.w $57994
pub fn i_05798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5798E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57994; return 10; }
    8
}
/// $05798E: lea.l $57b06(pc), a1
pub fn i_05798e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57992;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57B06;
    16
}
/// $057992: move.l a1, (a6)
pub fn i_057992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57994;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057994: jsr $49fd0.l
pub fn i_057994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5799A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5799A);
    cpu.pc = 0x49FD0;
    16
}
/// $05799A: bra.w $56e1e
pub fn i_05799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5799E;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
/// $05799E: lea.l $2b6f40.l, a0
pub fn i_05799e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6F40;
    20
}
/// $0579A4: jsr $28cd4.l
pub fn i_0579a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x579AA);
    cpu.pc = 0x28CD4;
    16
}
/// $0579AA: cmpi.b #$ff, $9d(a6)
pub fn i_0579aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579B0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0579B0: bne.w $579d8
pub fn i_0579b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x579D8; return 10; }
    8
}
/// $0579B4: move.w #$fdde, d0
pub fn i_0579b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579B8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFDDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0579B8: jsr $5dca4.l
pub fn i_0579b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x579BE);
    cpu.pc = 0x5DCA4;
    16
}
/// $0579BE: move.w d0, $28(a6)
pub fn i_0579be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579C2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579C2: move.w #$438, $2a(a6)
pub fn i_0579c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x438;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579C8: move.w #$ffb8, $2e(a6)
pub fn i_0579c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579CE: move.w #$0, $2c(a6)
pub fn i_0579ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579D4: bra.w $57a54
pub fn i_0579d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A54;
    10
}
/// $0579D8: cmpi.b #$1, $9d(a6)
pub fn i_0579d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0579DE: bne.w $57a06
pub fn i_0579de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57A06; return 10; }
    8
}
/// $0579E2: move.w #$fe67, d0
pub fn i_0579e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0579E6: jsr $5dca4.l
pub fn i_0579e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x579EC);
    cpu.pc = 0x5DCA4;
    16
}
/// $0579EC: move.w d0, $28(a6)
pub fn i_0579ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579F0: move.w #$7f8, $2a(a6)
pub fn i_0579f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579F6: move.w #$ff9a, $2e(a6)
pub fn i_0579f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579FC: move.w #$0, $2c(a6)
pub fn i_0579fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A02;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A02: bra.w $57a54
pub fn i_057a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A06;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A54;
    10
}
/// $057A06: cmpi.b #$2, $9d(a6)
pub fn i_057a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A0C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $057A0C: bne.w $57a34
pub fn i_057a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57A34; return 10; }
    8
}
/// $057A10: move.w #$fedc, d0
pub fn i_057a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A14;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEDC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057A14: jsr $5dca4.l
pub fn i_057a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A1A);
    cpu.pc = 0x5DCA4;
    16
}
/// $057A1A: move.w d0, $28(a6)
pub fn i_057a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A1E: move.w #$7fc, $2a(a6)
pub fn i_057a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A24: move.w #$ffb7, $2e(a6)
pub fn i_057a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A2A: move.w #$0, $2c(a6)
pub fn i_057a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A30: bra.w $57a54
pub fn i_057a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A34;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A54;
    10
}
/// $057A34: move.w #$fd56, d0
pub fn i_057a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A38;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFD56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057A38: jsr $5dca4.l
pub fn i_057a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A3E);
    cpu.pc = 0x5DCA4;
    16
}
/// $057A3E: move.w d0, $28(a6)
pub fn i_057a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A42: move.w #$7f8, $2a(a6)
pub fn i_057a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A48: move.w #$ff56, $2e(a6)
pub fn i_057a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A4E: move.w #$0, $2c(a6)
pub fn i_057a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A54;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A54: jsr $283ca.l
pub fn i_057a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A5A);
    cpu.pc = 0x283CA;
    16
}
/// $057A5A: lea.l $57a60(pc), a1
pub fn i_057a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57A60;
    16
}
/// $057A5E: move.l a1, (a6)
pub fn i_057a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057A60: jsr $27d50.l
pub fn i_057a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A66);
    cpu.pc = 0x27D50;
    16
}
/// $057A66: bcc.w $57a70
pub fn i_057a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57A70; return 10; }
    8
}
/// $057A6A: lea.l $58454(pc), a1
pub fn i_057a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58454;
    16
}
/// $057A6E: move.l a1, (a6)
pub fn i_057a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057A70: jsr $28d70.l
pub fn i_057a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A76);
    cpu.pc = 0x28D70;
    16
}
/// $057A76: tst.w $2a(a6)
pub fn i_057a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A7A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $057A7A: bpl.w $57a84
pub fn i_057a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A7E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57A84; return 10; }
    8
}
/// $057A7E: jsr $283d8.l
pub fn i_057a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A84);
    cpu.pc = 0x283D8;
    16
}
/// $057A84: btst.b #$1, $13(a6)
pub fn i_057a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A8A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057A8A: beq.w $57a94
pub fn i_057a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57A94; return 10; }
    8
}
/// $057A8E: lea.l $57a9e(pc), a1
pub fn i_057a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57A9E;
    16
}
/// $057A92: move.l a1, (a6)
pub fn i_057a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057A94: jsr $49fd0.l
pub fn i_057a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A9A);
    cpu.pc = 0x49FD0;
    16
}
/// $057A9A: bra.w $56e1e
pub fn i_057a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_LeapLand_057a9e  ($057A9E, 70 B, enemies/soldiers/helpers_056acc.s)
/// $057A9E: lea.l $2b6fba.l, a0
pub fn i_057a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6FBA;
    20
}
/// $057AA4: jsr $28cd4.l
pub fn i_057aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AAA);
    cpu.pc = 0x28CD4;
    16
}
/// $057AAA: jsr $283ca.l
pub fn i_057aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AB0);
    cpu.pc = 0x283CA;
    16
}
/// $057AB0: lea.l $57ab6(pc), a1
pub fn i_057ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57AB6;
    16
}
/// $057AB4: move.l a1, (a6)
pub fn i_057ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057AB6: jsr $27d50.l
pub fn i_057ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57ABC);
    cpu.pc = 0x27D50;
    16
}
/// $057ABC: scs.b $78(a6)
pub fn i_057abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AC0;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $057AC0: jsr $28d70.l
pub fn i_057ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AC6);
    cpu.pc = 0x28D70;
    16
}
/// $057AC6: jsr $5e8da.l
pub fn i_057ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57ACC);
    cpu.pc = 0x5E8DA;
    16
}
/// $057ACC: tst.b $78(a6)
pub fn i_057acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AD0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $057AD0: beq.w $57ada
pub fn i_057ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57ADA; return 10; }
    8
}
/// $057AD4: lea.l $58454(pc), a1
pub fn i_057ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58454;
    16
}
/// $057AD8: move.l a1, (a6)
pub fn i_057ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ADA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057ADA: jsr $49fd0.l
pub fn i_057ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AE0);
    cpu.pc = 0x49FD0;
    16
}
/// $057AE0: bra.w $56e1e
pub fn i_057ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_HitCheckTail_057ae4  ($057AE4, 34 B, enemies/soldiers/helpers_056acc.s)
/// $057AE4: jsr $2870a.l
pub fn i_057ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AEA);
    cpu.pc = 0x2870A;
    16
}
/// $057AEA: bcc.w $57b04
pub fn i_057aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57B04; return 10; }
    8
}
/// $057AEE: cmpi.b #$1c, $58(a6)
pub fn i_057aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AF4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1C, Size::Byte, false);
    16
}
/// $057AF4: bne.w $57afe
pub fn i_057af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57AFE; return 10; }
    8
}
/// $057AF8: jsr $49fd0.l
pub fn i_057af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AFE);
    cpu.pc = 0x49FD0;
    16
}
/// $057AFE: bclr.b #$3, $13(a6)
pub fn i_057afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B04;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057B04: rts
pub fn i_057b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabLatch_057b06  ($057B06, 174 B, enemies/soldiers/helpers_056acc.s)
/// $057B06: lea.l $2b6a02.l, a0
pub fn i_057b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6A02;
    20
}
/// $057B0C: jsr $28cd4.l
pub fn i_057b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57B12);
    cpu.pc = 0x28CD4;
    16
}
/// $057B12: lea.l $2b67de.l, a0
pub fn i_057b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B67DE;
    20
}
/// $057B18: move.l a0, $48(a6)
pub fn i_057b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B1C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057B1C: bclr.b #$3, $13(a6)
pub fn i_057b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B22;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057B22: lea.l $ffff.w, a0
pub fn i_057b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B26;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $057B26: move.l a0, $4c(a6)
pub fn i_057b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B2A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057B2A: jsr $283ca.l
pub fn i_057b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57B30);
    cpu.pc = 0x283CA;
    16
}
/// $057B30: clr.w $80(a6)
pub fn i_057b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B34;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $057B34: bsr.w $56e4a
pub fn i_057b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B38;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57B38);
    cpu.pc = 0x56E4A;
    18
}
/// $057B38: bcs.w $57b44
pub fn i_057b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B3C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57B44; return 10; }
    8
}
/// $057B3C: bra.w $57fc6
pub fn i_057b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B40;
    cpu.ir = 0x6000;
    cpu.pc = 0x57FC6;
    10
}
/// $057B44: movea.l $7a(a6), a0
pub fn i_057b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B48;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057B48: move.w $22(a6), d0
pub fn i_057b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B4C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B4C: move.w $24(a6), d1
pub fn i_057b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B50;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B50: sub.w $22(a0), d0
pub fn i_057b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B54;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B54: sub.w $24(a0), d1
pub fn i_057b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B58;
    cpu.ir = 0x9268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B58: move.w d0, $88(a6)
pub fn i_057b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B5C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B5C: move.w d1, $8a(a6)
pub fn i_057b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B60;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B60: lea.l $57b66(pc), a1
pub fn i_057b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57B66;
    16
}
/// $057B64: move.l a1, (a6)
pub fn i_057b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057B66: bsr.w $56e4a
pub fn i_057b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B6A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57B6A);
    cpu.pc = 0x56E4A;
    18
}
/// $057B6A: bcs.w $57b74
pub fn i_057b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B6E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57B74; return 10; }
    8
}
/// $057B6E: lea.l $57fc6(pc), a1
pub fn i_057b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057B72: move.l a1, (a6)
pub fn i_057b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057B74: movea.l $7a(a6), a0
pub fn i_057b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B78;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057B78: move.w $38(a0), d0
pub fn i_057b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B7C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B7C: andi.w #$ffe3, d0
pub fn i_057b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B80;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057B80: ori.w #$18, d0
pub fn i_057b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B84;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057B84: move.w d0, $38(a6)
pub fn i_057b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B88;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B88: move.w $22(a0), d0
pub fn i_057b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B8C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B8C: move.w $24(a0), d1
pub fn i_057b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B90;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B90: add.w $88(a6), d0
pub fn i_057b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B94;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B94: add.w $8a(a6), d1
pub fn i_057b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B98;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B98: move.w d0, $22(a6)
pub fn i_057b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B9C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B9C: move.w d1, $24(a6)
pub fn i_057b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BA0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057BA0: jsr $28d70.l
pub fn i_057ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57BA6);
    cpu.pc = 0x28D70;
    16
}
/// $057BA6: bcc.w $57bb0
pub fn i_057ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57BB0; return 10; }
    8
}
/// $057BAA: lea.l $57bb4(pc), a1
pub fn i_057baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57BB4;
    16
}
/// $057BAE: move.l a1, (a6)
pub fn i_057bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057BB0: jmp $57ae4(pc)
pub fn i_057bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BB4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabSlideToAnchor_057bb4  ($057BB4, 244 B, enemies/soldiers/helpers_056acc.s)
/// $057BB4: lea.l $2b6a60.l, a0
pub fn i_057bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6A60;
    20
}
/// $057BBA: jsr $28cd4.l
pub fn i_057bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57BC0);
    cpu.pc = 0x28CD4;
    16
}
/// $057BC0: bclr.b #$0, $3a(a6)
pub fn i_057bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BC6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057BC6: bsr.w $56e4a
pub fn i_057bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BCA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57BCA);
    cpu.pc = 0x56E4A;
    18
}
/// $057BCA: bcc.w $57fc6
pub fn i_057bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057BCE: move.b d0, $76(a6)
pub fn i_057bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BD2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057BD2: movea.l $7a(a6), a0
pub fn i_057bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BD6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057BD6: jsr $8f82a.l
pub fn i_057bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57BDC);
    cpu.pc = 0x8F82A;
    16
}
/// $057BDC: bcc.w $57fc6
pub fn i_057bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057BE0: sub.w $22(a6), d0
pub fn i_057be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BE4;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057BE4: sub.w $24(a6), d1
pub fn i_057be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BE8;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057BE8: neg.w d0
pub fn i_057be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BEA;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $057BEA: neg.w d1
pub fn i_057bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BEC;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $057BEC: move.w d0, $88(a6)
pub fn i_057bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BF0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057BF0: move.w d1, $8a(a6)
pub fn i_057bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BF4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057BF4: lea.l $57bfa(pc), a1
pub fn i_057bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57BFA;
    16
}
/// $057BF8: move.l a1, (a6)
pub fn i_057bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057BFA: move.w $88(a6), d0
pub fn i_057bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BFE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057BFE: beq.w $57c0e
pub fn i_057bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57C0E; return 10; }
    8
}
/// $057C02: asr.w #$5, d0
pub fn i_057c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C04;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $057C04: bne.w $57c0a
pub fn i_057c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C0A; return 10; }
    8
}
/// $057C08: moveq #$1, d0
pub fn i_057c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C0A;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $057C0A: sub.w d0, $88(a6)
pub fn i_057c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C0E;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $057C0E: move.w $8a(a6), d0
pub fn i_057c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057C12: beq.w $57c22
pub fn i_057c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57C22; return 10; }
    8
}
/// $057C16: asr.w #$5, d0
pub fn i_057c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C18;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $057C18: bne.w $57c1e
pub fn i_057c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C1C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C1E; return 10; }
    8
}
/// $057C1C: moveq #$1, d0
pub fn i_057c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C1E;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $057C1E: sub.w d0, $8a(a6)
pub fn i_057c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C22;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $057C22: move.b $76(a6), d0
pub fn i_057c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C26;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $057C26: movea.l $7a(a6), a0
pub fn i_057c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C2A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057C2A: jsr $8f82a.l
pub fn i_057c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57C30);
    cpu.pc = 0x8F82A;
    16
}
/// $057C30: bcs.w $57c3e
pub fn i_057c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C34;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57C3E; return 10; }
    8
}
/// $057C34: lea.l $57fc6(pc), a1
pub fn i_057c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057C38: move.l a1, (a6)
pub fn i_057c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C3A: bra.w $57c5e
pub fn i_057c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x57C5E;
    10
}
/// $057C3E: add.w $88(a6), d0
pub fn i_057c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C42;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057C42: add.w $8a(a6), d1
pub fn i_057c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C46;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057C46: move.w d0, $22(a6)
pub fn i_057c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C4A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057C4A: move.w d1, $24(a6)
pub fn i_057c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C4E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057C4E: move.w $38(a0), d0
pub fn i_057c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C52;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057C52: andi.w #$ffe3, d0
pub fn i_057c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C56;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057C56: ori.w #$18, d0
pub fn i_057c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C5A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057C5A: move.w d0, $38(a6)
pub fn i_057c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057C5E: jsr $28d70.l
pub fn i_057c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57C64);
    cpu.pc = 0x28D70;
    16
}
/// $057C64: move.w $88(a6), d0
pub fn i_057c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C68;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057C68: or.w $8a(a6), d0
pub fn i_057c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C6C;
    cpu.ir = 0x806E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $057C6C: bne.w $57ca4
pub fn i_057c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57CA4; return 10; }
    8
}
/// $057C70: lea.l $57fc6(pc), a1
pub fn i_057c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057C74: move.l a1, (a6)
pub fn i_057c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C76: move.b $76(a6), d0
pub fn i_057c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C7A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $057C7A: cmpi.b #$0, d0
pub fn i_057c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C7E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $057C7E: bne.w $57c88
pub fn i_057c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C82;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C88; return 10; }
    8
}
/// $057C82: lea.l $57d04(pc), a1
pub fn i_057c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57D04;
    16
}
/// $057C86: move.l a1, (a6)
pub fn i_057c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C88: cmpi.b #$1, d0
pub fn i_057c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C8C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $057C8C: bne.w $57c96
pub fn i_057c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C90;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C96; return 10; }
    8
}
/// $057C90: lea.l $57d6e(pc), a1
pub fn i_057c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57D6E;
    16
}
/// $057C94: move.l a1, (a6)
pub fn i_057c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C96: cmpi.b #$2, d0
pub fn i_057c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C9A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $057C9A: bne.w $57ca4
pub fn i_057c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57CA4; return 10; }
    8
}
/// $057C9E: lea.l $57e0a(pc), a1
pub fn i_057c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57E0A;
    16
}
/// $057CA2: move.l a1, (a6)
pub fn i_057ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057CA4: jmp $57ae4(pc)
pub fn i_057ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CA8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabHoldFlag_057ca8  ($057CA8, 16 B, enemies/soldiers/helpers_056acc.s)
/// $057CA8: tst.b $106ed3.l
pub fn i_057ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CAE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $057CAE: bne.w $57cc0
pub fn i_057cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57CC0; return 10; }
    8
}
/// $057CB2: bset.b #$3, $13(a6)
pub fn i_057cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CB8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_057cb8  ($057CB8, 8 B, kernel/task_handlers.c)
/// $057CB8: lea.l $57f4e(pc), a1
pub fn i_057cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F4E;
    16
}
/// $057CBC: move.l a1, (a6)
pub fn i_057cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057CBE: rts
pub fn i_057cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabFollowPlayer_057cc0  ($057CC0, 60 B, enemies/soldiers/helpers_056acc.s)
/// $057CC0: move.b $76(a6), d0
pub fn i_057cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CC4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $057CC4: movea.l $7a(a6), a0
pub fn i_057cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057CC8: jsr $8f884.l
pub fn i_057cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57CCE);
    cpu.pc = 0x8F884;
    16
}
/// $057CCE: bcc.w $57cee
pub fn i_057cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57CEE; return 10; }
    8
}
/// $057CD2: move.w d0, $22(a6)
pub fn i_057cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CD6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057CD6: move.w d1, $24(a6)
pub fn i_057cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CDA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057CDA: move.w $38(a0), d0
pub fn i_057cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CDE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057CDE: andi.w #$ffe3, d0
pub fn i_057cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CE2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057CE2: ori.w #$18, d0
pub fn i_057ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CE6;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057CE6: move.w d0, $38(a6)
pub fn i_057ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CEA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057CEA: bra.w $57cf4
pub fn i_057cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x57CF4;
    10
}
/// $057CEE: lea.l $57fc6(pc), a1
pub fn i_057cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057CF2: move.l a1, (a6)
pub fn i_057cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057CF4: bsr.w $56f10
pub fn i_057cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CF8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57CF8);
    cpu.pc = 0x56F10;
    18
}
/// $057CF8: bcc.w $57d02
pub fn i_057cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57D02; return 10; }
    8
}
// ===== SetTaskHandler_057cfc  ($057CFC, 8 B, kernel/task_handlers.c)
/// $057CFC: lea.l $57f4e(pc), a1
pub fn i_057cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F4E;
    16
}
/// $057D00: move.l a1, (a6)
pub fn i_057d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D02: rts
pub fn i_057d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabPlayer_057d04  ($057D04, 290 B, enemies/soldiers/states_057d04.s)
/// $057D04: bclr.b #$0, $3a(a6)
pub fn i_057d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D0A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057D0A: movea.l $7a(a6), a0
pub fn i_057d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D0E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057D0E: move.w #$0, d0
pub fn i_057d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D12;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057D12: jsr $8f85c.l
pub fn i_057d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D18);
    cpu.pc = 0x8F85C;
    16
}
/// $057D18: bcc.w $57fc6
pub fn i_057d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057D1C: lea.l $2b6d64.l, a0
pub fn i_057d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6D64;
    20
}
/// $057D22: jsr $28cd4.l
pub fn i_057d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D28);
    cpu.pc = 0x28CD4;
    16
}
/// $057D28: clr.b $20(a6)
pub fn i_057d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D2C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $057D2C: lea.l $57d32(pc), a1
pub fn i_057d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57D32;
    16
}
/// $057D30: move.l a1, (a6)
pub fn i_057d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D32: bsr.w $57ca8
pub fn i_057d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D36;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57D36);
    cpu.pc = 0x57CA8;
    18
}
/// $057D36: jsr $56f64(pc)
pub fn i_057d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x57D3A);
    cpu.pc = 0x56F64;
    16
}
/// $057D3A: bcc.w $57d44
pub fn i_057d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57D44; return 10; }
    8
}
/// $057D3E: lea.l $57ece(pc), a1
pub fn i_057d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57ECE;
    16
}
/// $057D42: move.l a1, (a6)
pub fn i_057d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D44: jsr $28d70.l
pub fn i_057d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D4A);
    cpu.pc = 0x28D70;
    16
}
/// $057D4A: bcc.w $57d54
pub fn i_057d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57D54; return 10; }
    8
}
/// $057D4E: lea.l $57fc6(pc), a1
pub fn i_057d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057D52: move.l a1, (a6)
pub fn i_057d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D54: tst.b $20(a6)
pub fn i_057d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D58;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $057D58: beq.w $57d6a
pub fn i_057d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D5C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57D6A; return 10; }
    8
}
/// $057D5C: movea.l $7a(a6), a0
pub fn i_057d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057D60: move.w #$0, d0
pub fn i_057d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D64;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057D64: jsr $8f8c2.l
pub fn i_057d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D6A);
    cpu.pc = 0x8F8C2;
    16
}
/// $057D6A: jmp $57ae4(pc)
pub fn i_057d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D6E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabBreakA_057ece  ($057ECE, 20 B, enemies/soldiers/states_057d04.s)
/// $057ECE: move.l #$57d1c, $5c(a6)
pub fn i_057ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ED6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x57D1C;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057ED6: move.l #$2b6c32, $92(a6)
pub fn i_057ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EDE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6C32;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EDE: bra.w $57f06
pub fn i_057ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F06;
    10
}
// ===== Soldier_GrabBreakB_057ee2  ($057EE2, 20 B, enemies/soldiers/states_057d04.s)
/// $057EE2: move.l #$57d86, $5c(a6)
pub fn i_057ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EEA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x57D86;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EEA: move.l #$2b6c32, $92(a6)
pub fn i_057eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EF2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6C32;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EF2: bra.w $57f06
pub fn i_057ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F06;
    10
}
// ===== Soldier_GrabBreakC_057ef6  ($057EF6, 16 B, enemies/soldiers/states_057d04.s)
/// $057EF6: move.l #$57e26, $5c(a6)
pub fn i_057ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EFE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x57E26;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EFE: move.l #$2b6bc0, $92(a6)
pub fn i_057efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F06;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6BC0;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Soldier_GrabBreak_057f06  ($057F06, 72 B, enemies/soldiers/states_057d04.s)
/// $057F06: movea.l $92(a6), a0
pub fn i_057f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F0A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057F0A: jsr $28cd4.l
pub fn i_057f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F10);
    cpu.pc = 0x28CD4;
    16
}
/// $057F10: lea.l $57f16(pc), a1
pub fn i_057f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F16;
    16
}
/// $057F14: move.l a1, (a6)
pub fn i_057f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057F16: bsr.w $57ca8
pub fn i_057f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F1A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57F1A);
    cpu.pc = 0x57CA8;
    18
}
/// $057F1A: jsr $28d70.l
pub fn i_057f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F20);
    cpu.pc = 0x28D70;
    16
}
/// $057F20: bcc.w $57f3a
pub fn i_057f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57F3A; return 10; }
    8
}
/// $057F24: bsr.w $56f64
pub fn i_057f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F28;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57F28);
    cpu.pc = 0x56F64;
    18
}
/// $057F28: bcc.w $57f36
pub fn i_057f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57F36; return 10; }
    8
}
/// $057F2C: lea.l $57f06(pc), a1
pub fn i_057f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F06;
    16
}
/// $057F30: move.l a1, (a6)
pub fn i_057f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057F32: bra.w $57f3a
pub fn i_057f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F36;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F3A;
    10
}
/// $057F36: move.l $5c(a6), (a6)
pub fn i_057f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F3A;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $057F3A: jsr $2870a.l
pub fn i_057f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F40);
    cpu.pc = 0x2870A;
    16
}
/// $057F40: bcc.w $57f4a
pub fn i_057f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57F4A; return 10; }
    8
}
/// $057F44: lea.l $57e7a(pc), a1
pub fn i_057f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57E7A;
    16
}
/// $057F48: move.l a1, (a6)
pub fn i_057f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057F4A: jmp $57ae4(pc)
pub fn i_057f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F4E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabThrownA_057f4e  ($057F4E, 120 B, enemies/soldiers/states_057d04.s)
/// $057F4E: lea.l $2b6ca4.l, a0
pub fn i_057f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6CA4;
    20
}
/// $057F54: jsr $28cd4.l
pub fn i_057f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F5A);
    cpu.pc = 0x28CD4;
    16
}
/// $057F5A: movea.l $7a(a6), a0
pub fn i_057f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057F5E: move.w $22(a0), d0
pub fn i_057f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F62;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057F62: sub.w $22(a6), d0
pub fn i_057f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F66;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057F66: bpl.w $57f74
pub fn i_057f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F6A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57F74; return 10; }
    8
}
/// $057F6A: bclr.b #$0, $3a(a6)
pub fn i_057f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057F70: bra.w $57f7a
pub fn i_057f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F74;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F7A;
    10
}
/// $057F74: bset.b #$0, $3a(a6)
pub fn i_057f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F7A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057F7A: move.w #$333, d0
pub fn i_057f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F7E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x333;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057F7E: jsr $5dca4.l
pub fn i_057f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F84);
    cpu.pc = 0x5DCA4;
    16
}
/// $057F84: move.w d0, $28(a6)
pub fn i_057f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F88;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F88: move.w #$32a, $2a(a6)
pub fn i_057f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F8E: move.w #$ffaf, $2e(a6)
pub fn i_057f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F94: move.w #$0, $2c(a6)
pub fn i_057f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F9A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F9A: lea.l $57fa0(pc), a1
pub fn i_057f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FA0;
    16
}
/// $057F9E: move.l a1, (a6)
pub fn i_057f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057FA0: jsr $2831e.l
pub fn i_057fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FA6);
    cpu.pc = 0x2831E;
    16
}
/// $057FA6: scs.b $78(a6)
pub fn i_057fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FAA;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $057FAA: bcc.w $57fb4
pub fn i_057faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FB4; return 10; }
    8
}
/// $057FAE: lea.l $588ae(pc), a1
pub fn i_057fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $057FB2: move.l a1, (a6)
pub fn i_057fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057FB4: jsr $28d70.l
pub fn i_057fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FBA);
    cpu.pc = 0x28D70;
    16
}
/// $057FBA: jsr $49fd0.l
pub fn i_057fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FC0);
    cpu.pc = 0x49FD0;
    16
}
/// $057FC0: bsr.w $56e1e
pub fn i_057fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FC4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57FC4);
    cpu.pc = 0x56E1E;
    18
}
/// $057FC4: rts
pub fn i_057fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabThrownB_057fc6  ($057FC6, 134 B, enemies/soldiers/states_057d04.s)
/// $057FC6: lea.l $ffff.w, a0
pub fn i_057fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FCA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $057FCA: move.l a0, $4c(a6)
pub fn i_057fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FCE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057FCE: jsr $283ca.l
pub fn i_057fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FD4);
    cpu.pc = 0x283CA;
    16
}
/// $057FD4: movea.l $7a(a6), a0
pub fn i_057fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FD8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057FD8: move.w $22(a0), d0
pub fn i_057fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FDC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057FDC: sub.w $22(a6), d0
pub fn i_057fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FE0;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057FE0: bpl.w $57fee
pub fn i_057fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FE4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57FEE; return 10; }
    8
}
/// $057FE4: bclr.b #$0, $3a(a6)
pub fn i_057fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FEA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057FEA: bra.w $57ff4
pub fn i_057fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x57FF4;
    10
}
/// $057FEE: bset.b #$0, $3a(a6)
pub fn i_057fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FF4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057FF4: lea.l $2b6d00.l, a0
pub fn i_057ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6D00;
    20
}
/// $057FFA: jsr $28cd4.l
pub fn i_057ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58000);
    cpu.pc = 0x28CD4;
    16
}
/// $058000: move.w #$266, d0
pub fn i_058000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58004;
    cpu.ir = 0x303C;
    let v1: u32 = 0x266;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $058004: jsr $5dca4.l
pub fn i_058004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5800A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5800A);
    cpu.pc = 0x5DCA4;
    16
}
/// $05800A: move.w d0, $28(a6)
pub fn i_05800a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5800E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05800E: move.w #$65e, $2a(a6)
pub fn i_05800e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58014;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x65E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058014: move.w #$ff5d, $2e(a6)
pub fn i_058014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5801A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05801A: move.w #$0, $2c(a6)
pub fn i_05801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58020;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058020: lea.l $58026(pc), a1
pub fn i_058020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58024;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58026;
    16
}
/// $058024: move.l a1, (a6)
pub fn i_058024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58026;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058026: jsr $2831e.l
pub fn i_058026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5802C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5802C);
    cpu.pc = 0x2831E;
    16
}
/// $05802C: scs.b $78(a6)
pub fn i_05802c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58030;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058030: bcc.w $5803a
pub fn i_058030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58034;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5803A; return 10; }
    8
}
/// $058034: lea.l $58464(pc), a1
pub fn i_058034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58038;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58464;
    16
}
/// $058038: move.l a1, (a6)
pub fn i_058038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5803A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05803A: jsr $28d70.l
pub fn i_05803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58040);
    cpu.pc = 0x28D70;
    16
}
/// $058040: jsr $49fd0.l
pub fn i_058040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58046;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58046);
    cpu.pc = 0x49FD0;
    16
}
/// $058046: bsr.w $56e1e
pub fn i_058046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5804A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5804A);
    cpu.pc = 0x56E1E;
    18
}
/// $05804A: rts
pub fn i_05804a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5804C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_RunToward_05804c  ($05804C, 42 B, enemies/soldiers/states_057d04.s)
/// $05804C: btst.b #$4, $74(a6)
pub fn i_05804c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58052;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058052: bne.w $58aa8
pub fn i_058052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58056;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58AA8; return 10; }
    8
}
/// $058056: lea.l $29b7c8.l, a0
pub fn i_058056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5805C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B7C8;
    20
}
/// $05805C: jsr $28cd4.l
pub fn i_05805c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58062);
    cpu.pc = 0x28CD4;
    16
}
/// $058062: move.w #$100, $36(a6)
pub fn i_058062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58068;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058068: bsr.w $56f8a
pub fn i_058068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5806C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5806C);
    cpu.pc = 0x56F8A;
    18
}
/// $05806C: lea.l $5809e(pc), a1
pub fn i_05806c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58070;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5809E;
    16
}
/// $058070: move.l a1, (a6)
pub fn i_058070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58072;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058072: bra.w $5809e
pub fn i_058072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58076;
    cpu.ir = 0x6000;
    cpu.pc = 0x5809E;
    10
}
// ===== Soldier_RunByTable_058076  ($058076, 40 B, enemies/soldiers/states_057d04.s)
/// $058076: lea.l $2b5d14.l, a0
pub fn i_058076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5807C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5D14;
    20
}
/// $05807C: jsr $28cd4.l
pub fn i_05807c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58082);
    cpu.pc = 0x28CD4;
    16
}
/// $058082: lea.l $2b756e.l, a0
pub fn i_058082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58088;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B756E;
    20
}
/// $058088: jsr $799de.l
pub fn i_058088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5808E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5808E);
    cpu.pc = 0x799DE;
    16
}
/// $05808E: neg.w d0
pub fn i_05808e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58090;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058090: move.w d0, $36(a6)
pub fn i_058090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58094;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058094: bsr.w $56f8a
pub fn i_058094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58098;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58098);
    cpu.pc = 0x56F8A;
    18
}
/// $058098: lea.l $5809e(pc), a1
pub fn i_058098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5809C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5809E;
    16
}
/// $05809C: move.l a1, (a6)
pub fn i_05809c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5809E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Run_Loop_05809e  ($05809E, 166 B, enemies/soldiers/states_057d04.s)
/// $05809E: bsr.w $56acc
pub fn i_05809e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580A2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x580A2);
    cpu.pc = 0x56ACC;
    18
}
/// $0580A2: bsr.w $56b92
pub fn i_0580a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x580A6);
    cpu.pc = 0x56B92;
    18
}
/// $0580A6: move.w $28(a6), d0
pub fn i_0580a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580AA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0580AA: asr.w #$4, d0
pub fn i_0580aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580AC;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0580AC: sub.w d0, $28(a6)
pub fn i_0580ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580B0;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0580B0: jsr $28d70.l
pub fn i_0580b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x580B6);
    cpu.pc = 0x28D70;
    16
}
/// $0580B6: bcc.w $580c0
pub fn i_0580b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580C0; return 10; }
    8
}
/// $0580BA: lea.l $58144(pc), a1
pub fn i_0580ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $0580BE: move.l a1, (a6)
pub fn i_0580be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0580C0: bsr.w $56fec
pub fn i_0580c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x580C4);
    cpu.pc = 0x56FEC;
    18
}
/// $0580C4: jsr $5e9b6.l
pub fn i_0580c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x580CA);
    cpu.pc = 0x5E9B6;
    16
}
/// $0580CA: move.w d0, -(a7)
pub fn i_0580ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580CC;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0580CC: andi.w #$ff, d0
pub fn i_0580cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580D0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0580D0: cmp.w $8a(a6), d0
pub fn i_0580d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580D4;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0580D4: bcc.w $580de
pub fn i_0580d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580DE; return 10; }
    8
}
/// $0580D8: move.b (a7), d0
pub fn i_0580d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580DA;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0580DA: cmp.w $8a(a6), d0
pub fn i_0580da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580DE;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0580DE: addq.w #$2, a7
pub fn i_0580de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580E0;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0580E0: bcc.w $580ea
pub fn i_0580e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580EA; return 10; }
    8
}
/// $0580E4: lea.l $58658(pc), a1
pub fn i_0580e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58658;
    16
}
/// $0580E8: move.l a1, (a6)
pub fn i_0580e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0580EA: jsr $56e36(pc)
pub fn i_0580ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x580EE);
    cpu.pc = 0x56E36;
    16
}
/// $0580EE: bcc.w $580f8
pub fn i_0580ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580F8; return 10; }
    8
}
/// $0580F2: lea.l $59062(pc), a1
pub fn i_0580f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59062;
    16
}
/// $0580F6: move.l a1, (a6)
pub fn i_0580f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0580F8: tst.b $78(a6)
pub fn i_0580f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580FC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0580FC: bne.w $58106
pub fn i_0580fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58100;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58106; return 10; }
    8
}
/// $058100: lea.l $58412(pc), a1
pub fn i_058100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58104;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058104: move.l a1, (a6)
pub fn i_058104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58106;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058106: jsr $5e9b6.l
pub fn i_058106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5810C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5810C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05810C: move.w d0, -(a7)
pub fn i_05810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5810E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05810E: andi.w #$ff, d0
pub fn i_05810e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58112;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058112: cmp.w $88(a6), d0
pub fn i_058112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58116;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058116: bcc.w $58120
pub fn i_058116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5811A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58120; return 10; }
    8
}
/// $05811A: move.b (a7), d0
pub fn i_05811a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5811C;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05811C: cmp.w $88(a6), d0
pub fn i_05811c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58120;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058120: addq.w #$2, a7
pub fn i_058120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58122;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058122: bcc.w $5812c
pub fn i_058122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58126;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5812C; return 10; }
    8
}
/// $058126: lea.l $588f6(pc), a1
pub fn i_058126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5812A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588F6;
    16
}
/// $05812A: move.l a1, (a6)
pub fn i_05812a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5812C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05812C: jsr $574e8(pc)
pub fn i_05812c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58130;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58130);
    cpu.pc = 0x574E8;
    16
}
/// $058130: bcc.w $5813a
pub fn i_058130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58134;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5813A; return 10; }
    8
}
/// $058134: lea.l $585ae(pc), a1
pub fn i_058134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58138;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585AE;
    16
}
/// $058138: move.l a1, (a6)
pub fn i_058138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5813A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05813A: jsr $49fd0.l
pub fn i_05813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58140);
    cpu.pc = 0x49FD0;
    16
}
/// $058140: bra.w $56e1e
pub fn i_058140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58144;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_Idle_058144  ($058144, 648 B, enemies/soldiers/states_057d04.s)
/// $058144: tst.w $34(a6)
pub fn i_058144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58148;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $058148: bne.w $5815c
pub fn i_058148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5814C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5815C; return 10; }
    8
}
/// $05814C: lea.l $29b816.l, a0
pub fn i_05814c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58152;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $058152: jsr $28cd4.l
pub fn i_058152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58158);
    cpu.pc = 0x28CD4;
    16
}
/// $058158: bra.w $58188
pub fn i_058158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5815C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58188;
    10
}
/// $05815C: spl.b d0
pub fn i_05815c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5815E;
    cpu.ir = 0x5AC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05815E: btst.b #$0, $3a(a6)
pub fn i_05815e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58164;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058164: spl.b d1
pub fn i_058164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58166;
    cpu.ir = 0x5AC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $058166: eor.w d0, d1
pub fn i_058166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58168;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058168: bne.w $5817c
pub fn i_058168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5816C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5817C; return 10; }
    8
}
/// $05816C: lea.l $2b5f28.l, a0
pub fn i_05816c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58172;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5F28;
    20
}
/// $058172: jsr $28cd4.l
pub fn i_058172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58178;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58178);
    cpu.pc = 0x28CD4;
    16
}
/// $058178: bra.w $58188
pub fn i_058178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5817C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58188;
    10
}
/// $05817C: lea.l $2b5f28.l, a0
pub fn i_05817c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58182;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5F28;
    20
}
/// $058182: jsr $28cd4.l
pub fn i_058182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58188);
    cpu.pc = 0x28CD4;
    16
}
/// $058188: clr.w $28(a6)
pub fn i_058188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5818C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05818C: clr.w $2a(a6)
pub fn i_05818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58190;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058190: lea.l $58196(pc), a1
pub fn i_058190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58194;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58196;
    16
}
/// $058194: move.l a1, (a6)
pub fn i_058194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58196;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058196: bsr.w $56acc
pub fn i_058196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5819A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5819A);
    cpu.pc = 0x56ACC;
    18
}
/// $05819A: bsr.w $56b92
pub fn i_05819a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5819E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5819E);
    cpu.pc = 0x56B92;
    18
}
/// $05819E: jsr $28d70.l
pub fn i_05819e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x581A4);
    cpu.pc = 0x28D70;
    16
}
/// $0581A4: bsr.w $56fec
pub fn i_0581a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581A8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x581A8);
    cpu.pc = 0x56FEC;
    18
}
/// $0581A8: btst.b #$0, $72(a6)
pub fn i_0581a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581AE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0581AE: beq.w $5828c
pub fn i_0581ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5828C; return 10; }
    8
}
/// $0581B2: move.w $80(a6), d1
pub fn i_0581b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581B6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0581B6: cmp.w $86(a6), d1
pub fn i_0581b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581BA;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0581BA: bcc.w $581c2
pub fn i_0581ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581C2; return 10; }
    8
}
/// $0581BE: move.w $86(a6), d1
pub fn i_0581be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581C2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0581C2: jsr $5e9b6.l
pub fn i_0581c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x581C8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0581C8: move.w d0, -(a7)
pub fn i_0581c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581CA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0581CA: andi.w #$ff, d0
pub fn i_0581ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581CE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0581CE: cmp.w d1, d0
pub fn i_0581ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D0;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581D0: bcc.w $581d8
pub fn i_0581d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581D8; return 10; }
    8
}
/// $0581D4: move.b (a7), d0
pub fn i_0581d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D6;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0581D6: cmp.w d1, d0
pub fn i_0581d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D8;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581D8: addq.w #$2, a7
pub fn i_0581d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581DA;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0581DA: bcc.w $581e4
pub fn i_0581da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581E4; return 10; }
    8
}
/// $0581DE: lea.l $58500(pc), a1
pub fn i_0581de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58500;
    16
}
/// $0581E2: move.l a1, (a6)
pub fn i_0581e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0581E4: mulu.w d1, d1
pub fn i_0581e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E6;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0581E6: lsr.l #$8, d1
pub fn i_0581e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E8;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $0581E8: jsr $5e9b6.l
pub fn i_0581e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x581EE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0581EE: move.w d0, -(a7)
pub fn i_0581ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581F0;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0581F0: andi.w #$ff, d0
pub fn i_0581f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581F4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0581F4: cmp.w d1, d0
pub fn i_0581f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581F6;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581F6: bcc.w $581fe
pub fn i_0581f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581FE; return 10; }
    8
}
/// $0581FA: move.b (a7), d0
pub fn i_0581fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581FC;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0581FC: cmp.w d1, d0
pub fn i_0581fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581FE;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581FE: addq.w #$2, a7
pub fn i_0581fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58200;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058200: bcc.w $5820a
pub fn i_058200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58204;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5820A; return 10; }
    8
}
/// $058204: lea.l $58076(pc), a1
pub fn i_058204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58208;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58076;
    16
}
/// $058208: move.l a1, (a6)
pub fn i_058208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5820A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05820A: mulu.w d1, d1
pub fn i_05820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5820C;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05820C: lsr.l #$8, d1
pub fn i_05820c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5820E;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $05820E: jsr $5e9b6.l
pub fn i_05820e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58214);
    cpu.pc = 0x5E9B6;
    16
}
/// $058214: move.w d0, -(a7)
pub fn i_058214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58216;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058216: andi.w #$ff, d0
pub fn i_058216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5821A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05821A: cmp.w d1, d0
pub fn i_05821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5821C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05821C: bcc.w $58224
pub fn i_05821c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58220;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58224; return 10; }
    8
}
/// $058220: move.b (a7), d0
pub fn i_058220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58222;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058222: cmp.w d1, d0
pub fn i_058222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58224;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058224: addq.w #$2, a7
pub fn i_058224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58226;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058226: bcc.w $58230
pub fn i_058226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5822A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58230; return 10; }
    8
}
/// $05822A: lea.l $585f6(pc), a1
pub fn i_05822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5822E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $05822E: move.l a1, (a6)
pub fn i_05822e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58230;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058230: move.w $82(a6), d1
pub fn i_058230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58234;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058234: cmp.w $84(a6), d1
pub fn i_058234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58238;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058238: bcc.w $58240
pub fn i_058238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5823C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58240; return 10; }
    8
}
/// $05823C: move.w $84(a6), d1
pub fn i_05823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58240;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058240: jsr $5e9b6.l
pub fn i_058240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58246);
    cpu.pc = 0x5E9B6;
    16
}
/// $058246: move.w d0, -(a7)
pub fn i_058246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58248;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058248: andi.w #$ff, d0
pub fn i_058248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5824C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05824C: cmp.w d1, d0
pub fn i_05824c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5824E;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05824E: bcc.w $58256
pub fn i_05824e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58252;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58256; return 10; }
    8
}
/// $058252: move.b (a7), d0
pub fn i_058252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58254;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058254: cmp.w d1, d0
pub fn i_058254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58256;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058256: addq.w #$2, a7
pub fn i_058256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58258;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058258: bcc.w $58262
pub fn i_058258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5825C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58262; return 10; }
    8
}
/// $05825C: lea.l $584c4(pc), a1
pub fn i_05825c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58260;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x584C4;
    16
}
/// $058260: move.l a1, (a6)
pub fn i_058260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58262;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058262: mulu.w d1, d1
pub fn i_058262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58264;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $058264: lsr.l #$8, d1
pub fn i_058264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58266;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $058266: jsr $5e9b6.l
pub fn i_058266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5826C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5826C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05826C: move.w d0, -(a7)
pub fn i_05826c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5826E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05826E: andi.w #$ff, d0
pub fn i_05826e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58272;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058272: cmp.w d1, d0
pub fn i_058272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58274;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058274: bcc.w $5827c
pub fn i_058274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58278;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5827C; return 10; }
    8
}
/// $058278: move.b (a7), d0
pub fn i_058278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5827A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05827A: cmp.w d1, d0
pub fn i_05827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5827C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05827C: addq.w #$2, a7
pub fn i_05827c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5827E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05827E: bcc.w $58288
pub fn i_05827e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58282;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58288; return 10; }
    8
}
/// $058282: lea.l $57558(pc), a1
pub fn i_058282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58286;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $058286: move.l a1, (a6)
pub fn i_058286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58288;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058288: bra.w $582d8
pub fn i_058288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5828C;
    cpu.ir = 0x6000;
    cpu.pc = 0x582D8;
    10
}
/// $05828C: jsr $5e9b6.l
pub fn i_05828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58292);
    cpu.pc = 0x5E9B6;
    16
}
/// $058292: move.w d0, -(a7)
pub fn i_058292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58294;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058294: andi.w #$ff, d0
pub fn i_058294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58298;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058298: cmp.w $80(a6), d0
pub fn i_058298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5829C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05829C: bcc.w $582a6
pub fn i_05829c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582A6; return 10; }
    8
}
/// $0582A0: move.b (a7), d0
pub fn i_0582a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A2;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0582A2: cmp.w $80(a6), d0
pub fn i_0582a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582A6: addq.w #$2, a7
pub fn i_0582a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A8;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0582A8: bcc.w $582b2
pub fn i_0582a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582B2; return 10; }
    8
}
/// $0582AC: lea.l $57558(pc), a1
pub fn i_0582ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $0582B0: move.l a1, (a6)
pub fn i_0582b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0582B2: jsr $5e9b6.l
pub fn i_0582b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x582B8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0582B8: move.w d0, -(a7)
pub fn i_0582b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582BA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0582BA: andi.w #$ff, d0
pub fn i_0582ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582BE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0582BE: cmp.w $82(a6), d0
pub fn i_0582be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582C2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582C2: bcc.w $582cc
pub fn i_0582c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582CC; return 10; }
    8
}
/// $0582C6: move.b (a7), d0
pub fn i_0582c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582C8;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0582C8: cmp.w $82(a6), d0
pub fn i_0582c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582CC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582CC: addq.w #$2, a7
pub fn i_0582cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582CE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0582CE: bcc.w $582d8
pub fn i_0582ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582D8; return 10; }
    8
}
/// $0582D2: lea.l $583cc(pc), a1
pub fn i_0582d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x583CC;
    16
}
/// $0582D6: move.l a1, (a6)
pub fn i_0582d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0582D8: jsr $5e9b6.l
pub fn i_0582d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x582DE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0582DE: move.w d0, -(a7)
pub fn i_0582de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582E0;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0582E0: andi.w #$ff, d0
pub fn i_0582e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0582E4: cmp.w $86(a6), d0
pub fn i_0582e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582E8;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582E8: bcc.w $582f2
pub fn i_0582e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582F2; return 10; }
    8
}
/// $0582EC: move.b (a7), d0
pub fn i_0582ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582EE;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0582EE: cmp.w $86(a6), d0
pub fn i_0582ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582F2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582F2: addq.w #$2, a7
pub fn i_0582f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582F4;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0582F4: bcc.w $582fe
pub fn i_0582f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582FE; return 10; }
    8
}
/// $0582F8: lea.l $585f6(pc), a1
pub fn i_0582f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $0582FC: move.l a1, (a6)
pub fn i_0582fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0582FE: jsr $5e9b6.l
pub fn i_0582fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58304);
    cpu.pc = 0x5E9B6;
    16
}
/// $058304: move.w d0, -(a7)
pub fn i_058304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58306;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058306: andi.w #$ff, d0
pub fn i_058306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5830A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05830A: cmp.w $84(a6), d0
pub fn i_05830a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5830E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05830E: bcc.w $58318
pub fn i_05830e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58312;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58318; return 10; }
    8
}
/// $058312: move.b (a7), d0
pub fn i_058312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58314;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058314: cmp.w $84(a6), d0
pub fn i_058314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58318;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058318: addq.w #$2, a7
pub fn i_058318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5831A;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05831A: bcc.w $58324
pub fn i_05831a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5831E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58324; return 10; }
    8
}
/// $05831E: lea.l $57558(pc), a1
pub fn i_05831e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58322;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $058322: move.l a1, (a6)
pub fn i_058322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58324;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058324: move.w $34(a6), d0
pub fn i_058324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58328;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058328: bne.w $58352
pub fn i_058328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5832C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58352; return 10; }
    8
}
/// $05832C: jsr $5e9b6.l
pub fn i_05832c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58332);
    cpu.pc = 0x5E9B6;
    16
}
/// $058332: move.w d0, -(a7)
pub fn i_058332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58334;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058334: andi.w #$ff, d0
pub fn i_058334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58338;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058338: cmp.w $8a(a6), d0
pub fn i_058338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5833C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05833C: bcc.w $58346
pub fn i_05833c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58340;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58346; return 10; }
    8
}
/// $058340: move.b (a7), d0
pub fn i_058340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58342;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058342: cmp.w $8a(a6), d0
pub fn i_058342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58346;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058346: addq.w #$2, a7
pub fn i_058346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58348;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058348: bcc.w $58352
pub fn i_058348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5834C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58352; return 10; }
    8
}
/// $05834C: lea.l $58658(pc), a1
pub fn i_05834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58350;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58658;
    16
}
/// $058350: move.l a1, (a6)
pub fn i_058350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58352;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058352: jsr $56e36(pc)
pub fn i_058352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58356;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58356);
    cpu.pc = 0x56E36;
    16
}
/// $058356: bcc.w $58360
pub fn i_058356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5835A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58360; return 10; }
    8
}
/// $05835A: lea.l $59062(pc), a1
pub fn i_05835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5835E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59062;
    16
}
/// $05835E: move.l a1, (a6)
pub fn i_05835e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58360;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058360: jsr $56fa0(pc)
pub fn i_058360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58364;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58364);
    cpu.pc = 0x56FA0;
    16
}
/// $058364: bcc.w $5836e
pub fn i_058364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58368;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5836E; return 10; }
    8
}
/// $058368: lea.l $58968(pc), a1
pub fn i_058368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5836C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58968;
    16
}
/// $05836C: move.l a1, (a6)
pub fn i_05836c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5836E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05836E: btst.b #$0, $73(a6)
pub fn i_05836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58374;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058374: beq.w $5837e
pub fn i_058374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58378;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5837E; return 10; }
    8
}
/// $058378: lea.l $58e08(pc), a1
pub fn i_058378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5837C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58E08;
    16
}
/// $05837C: move.l a1, (a6)
pub fn i_05837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5837E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05837E: jsr $5e9b6.l
pub fn i_05837e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58384);
    cpu.pc = 0x5E9B6;
    16
}
/// $058384: move.w d0, -(a7)
pub fn i_058384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58386;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058386: andi.w #$ff, d0
pub fn i_058386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5838A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05838A: cmp.w $88(a6), d0
pub fn i_05838a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5838E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05838E: bcc.w $58398
pub fn i_05838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58392;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58398; return 10; }
    8
}
/// $058392: move.b (a7), d0
pub fn i_058392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58394;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058394: cmp.w $88(a6), d0
pub fn i_058394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58398;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058398: addq.w #$2, a7
pub fn i_058398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5839A;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05839A: bcc.w $583a4
pub fn i_05839a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5839E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x583A4; return 10; }
    8
}
/// $05839E: lea.l $588f6(pc), a1
pub fn i_05839e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588F6;
    16
}
/// $0583A2: move.l a1, (a6)
pub fn i_0583a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583A4: jsr $574e8(pc)
pub fn i_0583a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x583A8);
    cpu.pc = 0x574E8;
    16
}
/// $0583A8: bcc.w $583b2
pub fn i_0583a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x583B2; return 10; }
    8
}
/// $0583AC: lea.l $585ae(pc), a1
pub fn i_0583ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585AE;
    16
}
/// $0583B0: move.l a1, (a6)
pub fn i_0583b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583B2: tst.b $78(a6)
pub fn i_0583b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583B6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0583B6: bne.w $583c0
pub fn i_0583b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x583C0; return 10; }
    8
}
/// $0583BA: lea.l $58412(pc), a1
pub fn i_0583ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0583BE: move.l a1, (a6)
pub fn i_0583be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583C0: jsr $49fd0.l
pub fn i_0583c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583C6);
    cpu.pc = 0x49FD0;
    16
}
/// $0583C6: bsr.w $56e1e
pub fn i_0583c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583CA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x583CA);
    cpu.pc = 0x56E1E;
    18
}
/// $0583CA: rts
pub fn i_0583ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_IdleFidget_0583cc  ($0583CC, 70 B, enemies/soldiers/states_057d04.s)
/// $0583CC: jsr $5e9b6.l
pub fn i_0583cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583D2);
    cpu.pc = 0x5E9B6;
    16
}
/// $0583D2: andi.b #$f, d0
pub fn i_0583d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583D6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0583D6: bne.w $585f6
pub fn i_0583d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x585F6; return 10; }
    8
}
/// $0583DA: lea.l $29bf34.l, a0
pub fn i_0583da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BF34;
    20
}
/// $0583E0: jsr $28cd4.l
pub fn i_0583e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0583E6: lea.l $583ec(pc), a1
pub fn i_0583e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x583EC;
    16
}
/// $0583EA: move.l a1, (a6)
pub fn i_0583ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583EC: jsr $2783a.l
pub fn i_0583ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583F2);
    cpu.pc = 0x2783A;
    16
}
/// $0583F2: bsr.w $56b92
pub fn i_0583f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583F6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x583F6);
    cpu.pc = 0x56B92;
    18
}
/// $0583F6: jsr $28d70.l
pub fn i_0583f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583FC);
    cpu.pc = 0x28D70;
    16
}
/// $0583FC: bcc.w $58406
pub fn i_0583fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58400;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58406; return 10; }
    8
}
/// $058400: lea.l $58144(pc), a1
pub fn i_058400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58404;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $058404: move.l a1, (a6)
pub fn i_058404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58406;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058406: jsr $49fd0.l
pub fn i_058406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5840C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5840C);
    cpu.pc = 0x49FD0;
    16
}
/// $05840C: bsr.w $56e1e
pub fn i_05840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58410;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58410);
    cpu.pc = 0x56E1E;
    18
}
/// $058410: rts
pub fn i_058410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Hurt_058412  ($058412, 18 B, enemies/soldiers/states_057d04.s)
/// $058412: lea.l $2b5b92.l, a0
pub fn i_058412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58418;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5B92;
    20
}
/// $058418: jsr $28cd4.l
pub fn i_058418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5841E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5841E);
    cpu.pc = 0x28CD4;
    16
}
/// $05841E: lea.l $58424(pc), a1
pub fn i_05841e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58422;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58424;
    16
}
/// $058422: move.l a1, (a6)
pub fn i_058422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58424;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Hurt_Loop_058424  ($058424, 64 B, enemies/soldiers/states_057d04.s)
/// $058424: move.w $28(a6), d0
pub fn i_058424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58428;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058428: asr.w #$6, d0
pub fn i_058428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5842A;
    cpu.ir = 0xEC40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05842A: sub.w d0, $28(a6)
pub fn i_05842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5842E;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05842E: jsr $28364.l
pub fn i_05842e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58434);
    cpu.pc = 0x28364;
    16
}
/// $058434: scs.b $78(a6)
pub fn i_058434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58438;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058438: bcc.w $58442
pub fn i_058438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5843C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58442; return 10; }
    8
}
/// $05843C: lea.l $58464(pc), a1
pub fn i_05843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58440;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58464;
    16
}
/// $058440: move.l a1, (a6)
pub fn i_058440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58442;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058442: jsr $28d70.l
pub fn i_058442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58448);
    cpu.pc = 0x28D70;
    16
}
/// $058448: jsr $49fd0.l
pub fn i_058448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5844E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5844E);
    cpu.pc = 0x49FD0;
    16
}
/// $05844E: bsr.w $56e1e
pub fn i_05844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58452;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58452);
    cpu.pc = 0x56E1E;
    18
}
/// $058452: rts
pub fn i_058452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Land_058464  ($058464, 96 B, enemies/soldiers/states_057d04.s)
/// $058464: move.w $28(a6), d0
pub fn i_058464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58468;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058468: bpl.w $5846e
pub fn i_058468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5846C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5846E; return 10; }
    8
}
/// $05846C: neg.w d0
pub fn i_05846c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5846E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05846E: move.w $2a(a6), d1
pub fn i_05846e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58472;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058472: bpl.w $58478
pub fn i_058472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58476;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x58478; return 10; }
    8
}
/// $058476: neg.w d1
pub fn i_058476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58478;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058478: cmp.w d0, d1
pub fn i_058478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5847A;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05847A: bcs.w $5804c
pub fn i_05847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5847E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5804C; return 10; }
    8
}
/// $05847E: lea.l $2b5c22.l, a0
pub fn i_05847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58484;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C22;
    20
}
/// $058484: jsr $28cd4.l
pub fn i_058484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5848A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5848A);
    cpu.pc = 0x28CD4;
    16
}
/// $05848A: clr.w $28(a6)
pub fn i_05848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5848E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05848E: lea.l $58494(pc), a1
pub fn i_05848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58494;
    16
}
/// $058492: move.l a1, (a6)
pub fn i_058492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58494;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058494: bsr.w $56acc
pub fn i_058494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58498;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58498);
    cpu.pc = 0x56ACC;
    18
}
/// $058498: jsr $28d70.l
pub fn i_058498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5849E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5849E);
    cpu.pc = 0x28D70;
    16
}
/// $05849E: bcc.w $584ba
pub fn i_05849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x584BA; return 10; }
    8
}
/// $0584A2: tst.b $78(a6)
pub fn i_0584a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584A6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0584A6: beq.w $584b4
pub fn i_0584a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x584B4; return 10; }
    8
}
/// $0584AA: lea.l $57558(pc), a1
pub fn i_0584aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $0584AE: move.l a1, (a6)
pub fn i_0584ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0584B0: bra.w $584ba
pub fn i_0584b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x584BA;
    10
}
/// $0584B4: lea.l $58412(pc), a1
pub fn i_0584b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0584B8: move.l a1, (a6)
pub fn i_0584b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0584BA: jsr $49fd0.l
pub fn i_0584ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x584C0);
    cpu.pc = 0x49FD0;
    16
}
/// $0584C0: bra.w $56e1e
pub fn i_0584c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_StepRight_0584c4  ($0584C4, 60 B, enemies/soldiers/states_057d04.s)
/// $0584C4: tst.w $34(a6)
pub fn i_0584c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584C8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0584C8: bne.w $584dc
pub fn i_0584c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x584DC; return 10; }
    8
}
/// $0584CC: lea.l $2b5c56.l, a0
pub fn i_0584cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C56;
    20
}
/// $0584D2: jsr $28cd4.l
pub fn i_0584d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x584D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0584D8: bra.w $584e8
pub fn i_0584d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x584E8;
    10
}
/// $0584DC: lea.l $2b5f56.l, a0
pub fn i_0584dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5F56;
    20
}
/// $0584E2: jsr $28cd4.l
pub fn i_0584e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x584E8);
    cpu.pc = 0x28CD4;
    16
}
/// $0584E8: move.w #$80, $36(a6)
pub fn i_0584e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0584EE: bsr.w $56f8a
pub fn i_0584ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584F2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x584F2);
    cpu.pc = 0x56F8A;
    18
}
/// $0584F2: clr.w $2a(a6)
pub fn i_0584f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0584F6: lea.l $58538(pc), a1
pub fn i_0584f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58538;
    16
}
/// $0584FA: move.l a1, (a6)
pub fn i_0584fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0584FC: bra.w $58538
pub fn i_0584fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58500;
    cpu.ir = 0x6000;
    cpu.pc = 0x58538;
    10
}
// ===== Soldier_StepLeft_058500  ($058500, 56 B, enemies/soldiers/states_057d04.s)
/// $058500: tst.w $34(a6)
pub fn i_058500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58504;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $058504: bne.w $58518
pub fn i_058504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58508;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58518; return 10; }
    8
}
/// $058508: lea.l $2b5ca8.l, a0
pub fn i_058508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5850E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5CA8;
    20
}
/// $05850E: jsr $28cd4.l
pub fn i_05850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58514);
    cpu.pc = 0x28CD4;
    16
}
/// $058514: bra.w $58524
pub fn i_058514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58518;
    cpu.ir = 0x6000;
    cpu.pc = 0x58524;
    10
}
/// $058518: lea.l $2b5fd0.l, a0
pub fn i_058518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5851E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5FD0;
    20
}
/// $05851E: jsr $28cd4.l
pub fn i_05851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58524);
    cpu.pc = 0x28CD4;
    16
}
/// $058524: move.w #$ff80, $36(a6)
pub fn i_058524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5852A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05852A: bsr.w $56f8a
pub fn i_05852a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5852E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5852E);
    cpu.pc = 0x56F8A;
    18
}
/// $05852E: clr.w $2a(a6)
pub fn i_05852e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58532;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058532: lea.l $58538(pc), a1
pub fn i_058532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58536;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58538;
    16
}
/// $058536: move.l a1, (a6)
pub fn i_058536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58538;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Step_Loop_058538  ($058538, 118 B, enemies/soldiers/states_057d04.s)
/// $058538: bsr.w $56acc
pub fn i_058538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5853C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5853C);
    cpu.pc = 0x56ACC;
    18
}
/// $05853C: bsr.w $56b92
pub fn i_05853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58540;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58540);
    cpu.pc = 0x56B92;
    18
}
/// $058540: jsr $28d70.l
pub fn i_058540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58546;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58546);
    cpu.pc = 0x28D70;
    16
}
/// $058546: bcc.w $58550
pub fn i_058546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5854A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58550; return 10; }
    8
}
/// $05854A: lea.l $58144(pc), a1
pub fn i_05854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5854E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $05854E: move.l a1, (a6)
pub fn i_05854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58550;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058550: bsr.w $56fec
pub fn i_058550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58554;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58554);
    cpu.pc = 0x56FEC;
    18
}
/// $058554: jsr $5e9b6.l
pub fn i_058554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5855A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5855A);
    cpu.pc = 0x5E9B6;
    16
}
/// $05855A: move.w d0, -(a7)
pub fn i_05855a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5855C;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05855C: andi.w #$ff, d0
pub fn i_05855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58560;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058560: cmp.w $88(a6), d0
pub fn i_058560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58564;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058564: bcc.w $5856e
pub fn i_058564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58568;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5856E; return 10; }
    8
}
/// $058568: move.b (a7), d0
pub fn i_058568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5856A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05856A: cmp.w $88(a6), d0
pub fn i_05856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5856E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05856E: addq.w #$2, a7
pub fn i_05856e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58570;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058570: bcc.w $5857a
pub fn i_058570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58574;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5857A; return 10; }
    8
}
/// $058574: lea.l $588f6(pc), a1
pub fn i_058574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58578;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588F6;
    16
}
/// $058578: move.l a1, (a6)
pub fn i_058578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5857A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05857A: tst.b $78(a6)
pub fn i_05857a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5857E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05857E: bne.w $58588
pub fn i_05857e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58582;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58588; return 10; }
    8
}
/// $058582: lea.l $58412(pc), a1
pub fn i_058582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58586;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058586: move.l a1, (a6)
pub fn i_058586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58588;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058588: jsr $56fa0(pc)
pub fn i_058588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5858C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5858C);
    cpu.pc = 0x56FA0;
    16
}
/// $05858C: bcc.w $58596
pub fn i_05858c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58590;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58596; return 10; }
    8
}
/// $058590: lea.l $58968(pc), a1
pub fn i_058590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58594;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58968;
    16
}
/// $058594: move.l a1, (a6)
pub fn i_058594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58596;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058596: jsr $574e8(pc)
pub fn i_058596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5859A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5859A);
    cpu.pc = 0x574E8;
    16
}
/// $05859A: bcc.w $585a4
pub fn i_05859a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5859E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x585A4; return 10; }
    8
}
/// $05859E: lea.l $585ae(pc), a1
pub fn i_05859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585AE;
    16
}
/// $0585A2: move.l a1, (a6)
pub fn i_0585a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0585A4: jsr $49fd0.l
pub fn i_0585a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585AA);
    cpu.pc = 0x49FD0;
    16
}
/// $0585AA: bra.w $56e1e
pub fn i_0585aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_MeleeAttack_0585ae  ($0585AE, 72 B, enemies/soldiers/states_057d04.s)
/// $0585AE: clr.w $28(a6)
pub fn i_0585ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585B2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0585B2: lea.l $2b70d2.l, a0
pub fn i_0585b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B70D2;
    20
}
/// $0585B8: jsr $28cd4.l
pub fn i_0585b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585BE);
    cpu.pc = 0x28CD4;
    16
}
/// $0585BE: lea.l $57494(pc), a0
pub fn i_0585be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x57494;
    16
}
/// $0585C2: move.l a0, $4c(a6)
pub fn i_0585c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585C6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0585C6: jsr $283ca.l
pub fn i_0585c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585CC);
    cpu.pc = 0x283CA;
    16
}
/// $0585CC: jsr $283ca.l
pub fn i_0585cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585D2);
    cpu.pc = 0x283CA;
    16
}
/// $0585D2: lea.l $585d8(pc), a1
pub fn i_0585d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585D8;
    16
}
/// $0585D6: move.l a1, (a6)
pub fn i_0585d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0585D8: bsr.w $56acc
pub fn i_0585d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x585DC);
    cpu.pc = 0x56ACC;
    18
}
/// $0585DC: jsr $28d70.l
pub fn i_0585dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585E2);
    cpu.pc = 0x28D70;
    16
}
/// $0585E2: bcc.w $585ec
pub fn i_0585e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x585EC; return 10; }
    8
}
/// $0585E6: lea.l $58144(pc), a1
pub fn i_0585e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $0585EA: move.l a1, (a6)
pub fn i_0585ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0585EC: jsr $49fd0.l
pub fn i_0585ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585F2);
    cpu.pc = 0x49FD0;
    16
}
/// $0585F2: bra.w $56e1e
pub fn i_0585f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_Brake_0585f6  ($0585F6, 98 B, enemies/soldiers/states_057d04.s)
/// $0585F6: lea.l $2b5cfa.l, a0
pub fn i_0585f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5CFA;
    20
}
/// $0585FC: jsr $28cd4.l
pub fn i_0585fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58602);
    cpu.pc = 0x28CD4;
    16
}
/// $058602: lea.l $58608(pc), a1
pub fn i_058602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58606;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58608;
    16
}
/// $058606: move.l a1, (a6)
pub fn i_058606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58608;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058608: bsr.w $56b92
pub fn i_058608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5860C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5860C);
    cpu.pc = 0x56B92;
    18
}
/// $05860C: move.w $28(a6), d0
pub fn i_05860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58610;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058610: asr.w #$4, d0
pub fn i_058610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58612;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $058612: sub.w d0, $28(a6)
pub fn i_058612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58616;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $058616: bsr.w $56acc
pub fn i_058616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5861A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5861A);
    cpu.pc = 0x56ACC;
    18
}
/// $05861A: jsr $28d70.l
pub fn i_05861a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58620);
    cpu.pc = 0x28D70;
    16
}
/// $058620: bcc.w $5863e
pub fn i_058620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58624;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5863E; return 10; }
    8
}
/// $058624: btst.b #$0, $72(a6)
pub fn i_058624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5862A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05862A: beq.w $58638
pub fn i_05862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5862E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58638; return 10; }
    8
}
/// $05862E: lea.l $5804c(pc), a1
pub fn i_05862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58632;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $058632: move.l a1, (a6)
pub fn i_058632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58634;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058634: bra.w $5863e
pub fn i_058634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58638;
    cpu.ir = 0x6000;
    cpu.pc = 0x5863E;
    10
}
/// $058638: lea.l $57558(pc), a1
pub fn i_058638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5863C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $05863C: move.l a1, (a6)
pub fn i_05863c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5863E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05863E: tst.b $78(a6)
pub fn i_05863e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58642;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058642: bne.w $5864c
pub fn i_058642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58646;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5864C; return 10; }
    8
}
/// $058646: lea.l $58412(pc), a1
pub fn i_058646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5864A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $05864A: move.l a1, (a6)
pub fn i_05864a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5864C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05864C: jsr $49fd0.l
pub fn i_05864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58652);
    cpu.pc = 0x49FD0;
    16
}
/// $058652: bsr.w $56e1e
pub fn i_058652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58656;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58656);
    cpu.pc = 0x56E1E;
    18
}
/// $058656: rts
pub fn i_058656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58658;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Flee_058658  ($058658, 96 B, enemies/soldiers/states_057d04.s)
/// $058658: btst.b #$4, $74(a6)
pub fn i_058658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5865E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05865E: bne.w $58aa8
pub fn i_05865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58662;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58AA8; return 10; }
    8
}
/// $058662: lea.l $29b956.l, a0
pub fn i_058662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58668;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B956;
    20
}
/// $058668: jsr $28cd4.l
pub fn i_058668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5866E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5866E);
    cpu.pc = 0x28CD4;
    16
}
/// $05866E: move.w $28(a6), d0
pub fn i_05866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58672;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058672: add.w d0, d0
pub fn i_058672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58674;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $058674: move.w d0, $28(a6)
pub fn i_058674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58678;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058678: clr.w $2a(a6)
pub fn i_058678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5867C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05867C: lea.l $58682(pc), a1
pub fn i_05867c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58680;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58682;
    16
}
/// $058680: move.l a1, (a6)
pub fn i_058680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58682;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058682: move.w $28(a6), d0
pub fn i_058682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58686;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058686: asr.w #$5, d0
pub fn i_058686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58688;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $058688: sub.w d0, $28(a6)
pub fn i_058688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5868C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05868C: bsr.w $56acc
pub fn i_05868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58690;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58690);
    cpu.pc = 0x56ACC;
    18
}
/// $058690: bsr.w $56b92
pub fn i_058690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58694;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58694);
    cpu.pc = 0x56B92;
    18
}
/// $058694: jsr $28d70.l
pub fn i_058694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5869A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5869A);
    cpu.pc = 0x28D70;
    16
}
/// $05869A: bcc.w $586ac
pub fn i_05869a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5869E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x586AC; return 10; }
    8
}
/// $05869E: tst.b $78(a6)
pub fn i_05869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0586A2: beq.w $586ac
pub fn i_0586a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x586AC; return 10; }
    8
}
/// $0586A6: lea.l $586b8(pc), a1
pub fn i_0586a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x586B8;
    16
}
/// $0586AA: move.l a1, (a6)
pub fn i_0586aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0586AC: jsr $49fd0.l
pub fn i_0586ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586B2);
    cpu.pc = 0x49FD0;
    16
}
/// $0586B2: bsr.w $56e1e
pub fn i_0586b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586B6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x586B6);
    cpu.pc = 0x56E1E;
    18
}
/// $0586B6: rts
pub fn i_0586b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_FleeStop_0586b8  ($0586B8, 230 B, enemies/soldiers/states_057d04.s)
/// $0586B8: lea.l $2b5c10.l, a0
pub fn i_0586b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C10;
    20
}
/// $0586BE: jsr $28cd4.l
pub fn i_0586be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0586C4: clr.w $28(a6)
pub fn i_0586c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586C8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0586C8: lea.l $586ce(pc), a1
pub fn i_0586c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x586CE;
    16
}
/// $0586CC: move.l a1, (a6)
pub fn i_0586cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0586CE: bsr.w $56acc
pub fn i_0586ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586D2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x586D2);
    cpu.pc = 0x56ACC;
    18
}
/// $0586D2: bsr.w $56b92
pub fn i_0586d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x586D6);
    cpu.pc = 0x56B92;
    18
}
/// $0586D6: jsr $28d70.l
pub fn i_0586d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586DC);
    cpu.pc = 0x28D70;
    16
}
/// $0586DC: move.w $8a(a6), d1
pub fn i_0586dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586E0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0586E0: neg.w d1
pub fn i_0586e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586E2;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0586E2: addi.w #$100, d1
pub fn i_0586e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586E6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0586E6: jsr $5e9b6.l
pub fn i_0586e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586EC);
    cpu.pc = 0x5E9B6;
    16
}
/// $0586EC: move.w d0, -(a7)
pub fn i_0586ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586EE;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0586EE: andi.w #$ff, d0
pub fn i_0586ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586F2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0586F2: cmp.w d1, d0
pub fn i_0586f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586F4;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0586F4: bcc.w $586fc
pub fn i_0586f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x586FC; return 10; }
    8
}
/// $0586F8: move.b (a7), d0
pub fn i_0586f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586FA;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0586FA: cmp.w d1, d0
pub fn i_0586fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586FC;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0586FC: addq.w #$2, a7
pub fn i_0586fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586FE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0586FE: bcc.w $58708
pub fn i_0586fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58702;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58708; return 10; }
    8
}
/// $058702: lea.l $588ae(pc), a1
pub fn i_058702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58706;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $058706: move.l a1, (a6)
pub fn i_058706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58708;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058708: btst.b #$0, $72(a6)
pub fn i_058708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5870E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05870E: beq.w $58762
pub fn i_05870e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58712;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58762; return 10; }
    8
}
/// $058712: jsr $5e9b6.l
pub fn i_058712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58718);
    cpu.pc = 0x5E9B6;
    16
}
/// $058718: move.w d0, -(a7)
pub fn i_058718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5871A;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05871A: andi.w #$ff, d0
pub fn i_05871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5871E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05871E: cmp.w $82(a6), d0
pub fn i_05871e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58722;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058722: bcc.w $5872c
pub fn i_058722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58726;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5872C; return 10; }
    8
}
/// $058726: move.b (a7), d0
pub fn i_058726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58728;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058728: cmp.w $82(a6), d0
pub fn i_058728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5872C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05872C: addq.w #$2, a7
pub fn i_05872c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5872E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05872E: bcc.w $58738
pub fn i_05872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58732;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58738; return 10; }
    8
}
/// $058732: lea.l $5879e(pc), a1
pub fn i_058732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58736;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5879E;
    16
}
/// $058736: move.l a1, (a6)
pub fn i_058736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58738;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058738: move.w $80(a6), d1
pub fn i_058738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5873C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05873C: jsr $5e9b6.l
pub fn i_05873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58742);
    cpu.pc = 0x5E9B6;
    16
}
/// $058742: move.w d0, -(a7)
pub fn i_058742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58744;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058744: andi.w #$ff, d0
pub fn i_058744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58748;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058748: cmp.w d1, d0
pub fn i_058748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5874A;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05874A: bcc.w $58752
pub fn i_05874a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5874E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58752; return 10; }
    8
}
/// $05874E: move.b (a7), d0
pub fn i_05874e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58750;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058750: cmp.w d1, d0
pub fn i_058750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58752;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058752: addq.w #$2, a7
pub fn i_058752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58754;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058754: bcc.w $5875e
pub fn i_058754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58758;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5875E; return 10; }
    8
}
/// $058758: lea.l $587ac(pc), a1
pub fn i_058758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5875C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x587AC;
    16
}
/// $05875C: move.l a1, (a6)
pub fn i_05875c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5875E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05875E: bra.w $58768
pub fn i_05875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58762;
    cpu.ir = 0x6000;
    cpu.pc = 0x58768;
    10
}
/// $058762: lea.l $587ac(pc), a1
pub fn i_058762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x587AC;
    16
}
/// $058766: move.l a1, (a6)
pub fn i_058766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058768: jsr $56e36(pc)
pub fn i_058768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5876C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5876C);
    cpu.pc = 0x56E36;
    16
}
/// $05876C: bcc.w $58776
pub fn i_05876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58770;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58776; return 10; }
    8
}
/// $058770: lea.l $591c8(pc), a1
pub fn i_058770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58774;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x591C8;
    16
}
/// $058774: move.l a1, (a6)
pub fn i_058774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58776;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058776: jsr $56fa0(pc)
pub fn i_058776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5877A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5877A);
    cpu.pc = 0x56FA0;
    16
}
/// $05877A: bcc.w $58784
pub fn i_05877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5877E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58784; return 10; }
    8
}
/// $05877E: lea.l $589f8(pc), a1
pub fn i_05877e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58782;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x589F8;
    16
}
/// $058782: move.l a1, (a6)
pub fn i_058782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58784;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058784: tst.b $78(a6)
pub fn i_058784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58788;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058788: bne.w $58792
pub fn i_058788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5878C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58792; return 10; }
    8
}
/// $05878C: lea.l $58412(pc), a1
pub fn i_05878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58790;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058790: move.l a1, (a6)
pub fn i_058790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58792;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058792: jsr $49ff2.l
pub fn i_058792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58798);
    cpu.pc = 0x49FF2;
    16
}
/// $058798: bsr.w $56e1e
pub fn i_058798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5879C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5879C);
    cpu.pc = 0x56E1E;
    18
}
/// $05879C: rts
pub fn i_05879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5879E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_RetreatJmp_05879e  ($05879E, 14 B, enemies/soldiers/states_057d04.s)
/// $05879E: btst.b #$0, $72(a6)
pub fn i_05879e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587A4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0587A4: bne.w $587b6
pub fn i_0587a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x587B6; return 10; }
    8
}
/// $0587A8: bra.w $587e4
pub fn i_0587a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x587E4;
    10
}
// ===== Soldier_Retreat_0587ac  ($0587AC, 84 B, enemies/soldiers/states_057d04.s)
/// $0587AC: btst.b #$0, $72(a6)
pub fn i_0587ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0587B2: bne.w $587e4
pub fn i_0587b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x587E4; return 10; }
    8
}
/// $0587B6: lea.l $2b5bcc.l, a0
pub fn i_0587b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5BCC;
    20
}
/// $0587BC: jsr $28cd4.l
pub fn i_0587bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x587C2);
    cpu.pc = 0x28CD4;
    16
}
/// $0587C2: move.w #$100, d0
pub fn i_0587c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0587C6: btst.b #$0, $72(a6)
pub fn i_0587c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587CC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0587CC: bne.w $587d2
pub fn i_0587cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x587D2; return 10; }
    8
}
/// $0587D0: add.w d0, d0
pub fn i_0587d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0587D2: move.w d0, $36(a6)
pub fn i_0587d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587D6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0587D6: bsr.w $56f8a
pub fn i_0587d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587DA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x587DA);
    cpu.pc = 0x56F8A;
    18
}
/// $0587DA: lea.l $58800(pc), a1
pub fn i_0587da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58800;
    16
}
/// $0587DE: move.l a1, (a6)
pub fn i_0587de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0587E0: bra.w $58800
pub fn i_0587e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x58800;
    10
}
/// $0587E4: lea.l $2b5bcc.l, a0
pub fn i_0587e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5BCC;
    20
}
/// $0587EA: jsr $28cd4.l
pub fn i_0587ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x587F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0587F0: move.w #$ff00, $36(a6)
pub fn i_0587f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0587F6: bsr.w $56f8a
pub fn i_0587f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587FA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x587FA);
    cpu.pc = 0x56F8A;
    18
}
/// $0587FA: lea.l $58800(pc), a1
pub fn i_0587fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58800;
    16
}
/// $0587FE: move.l a1, (a6)
pub fn i_0587fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58800;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Retreat_Loop_058800  ($058800, 174 B, enemies/soldiers/states_057d04.s)
/// $058800: jsr $282d8.l
pub fn i_058800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58806);
    cpu.pc = 0x282D8;
    16
}
/// $058806: bcc.w $58810
pub fn i_058806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5880A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58810; return 10; }
    8
}
/// $05880A: lea.l $588ae(pc), a1
pub fn i_05880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5880E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $05880E: move.l a1, (a6)
pub fn i_05880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58810;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058810: jsr $27eba.l
pub fn i_058810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58816);
    cpu.pc = 0x27EBA;
    16
}
/// $058816: scc.b $78(a6)
pub fn i_058816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5881A;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $05881A: bsr.w $56b92
pub fn i_05881a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5881E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5881E);
    cpu.pc = 0x56B92;
    18
}
/// $05881E: move.w $34(a6), d0
pub fn i_05881e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58822;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058822: beq.w $5882c
pub fn i_058822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58826;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5882C; return 10; }
    8
}
/// $058826: lea.l $588ae(pc), a1
pub fn i_058826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5882A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $05882A: move.l a1, (a6)
pub fn i_05882a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5882C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05882C: jsr $28d70.l
pub fn i_05882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58832);
    cpu.pc = 0x28D70;
    16
}
/// $058832: bcc.w $5883c
pub fn i_058832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58836;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5883C; return 10; }
    8
}
/// $058836: lea.l $586b8(pc), a1
pub fn i_058836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5883A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x586B8;
    16
}
/// $05883A: move.l a1, (a6)
pub fn i_05883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5883C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05883C: move.w $8a(a6), d1
pub fn i_05883c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58840;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058840: neg.w d1
pub fn i_058840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58842;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058842: addi.w #$100, d1
pub fn i_058842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58846;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058846: jsr $5e9b6.l
pub fn i_058846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5884C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5884C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05884C: move.w d0, -(a7)
pub fn i_05884c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5884E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05884E: andi.w #$ff, d0
pub fn i_05884e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58852;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058852: cmp.w d1, d0
pub fn i_058852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58854;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058854: bcc.w $5885c
pub fn i_058854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58858;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5885C; return 10; }
    8
}
/// $058858: move.b (a7), d0
pub fn i_058858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5885A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05885A: cmp.w d1, d0
pub fn i_05885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5885C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05885C: addq.w #$2, a7
pub fn i_05885c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5885E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05885E: bcc.w $58868
pub fn i_05885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58862;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58868; return 10; }
    8
}
/// $058862: lea.l $588ae(pc), a1
pub fn i_058862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $058866: move.l a1, (a6)
pub fn i_058866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058868: btst.b #$5, $5a(a6)
pub fn i_058868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5886E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05886E: beq.w $58878
pub fn i_05886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58872;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58878; return 10; }
    8
}
/// $058872: lea.l $588ae(pc), a1
pub fn i_058872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58876;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $058876: move.l a1, (a6)
pub fn i_058876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58878;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058878: jsr $56e36(pc)
pub fn i_058878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5887C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5887C);
    cpu.pc = 0x56E36;
    16
}
/// $05887C: bcc.w $58886
pub fn i_05887c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58880;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58886; return 10; }
    8
}
/// $058880: lea.l $591c8(pc), a1
pub fn i_058880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58884;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x591C8;
    16
}
/// $058884: move.l a1, (a6)
pub fn i_058884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58886;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058886: jsr $56fa0(pc)
pub fn i_058886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5888A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5888A);
    cpu.pc = 0x56FA0;
    16
}
/// $05888A: bcc.w $58894
pub fn i_05888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5888E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58894; return 10; }
    8
}
/// $05888E: lea.l $589f8(pc), a1
pub fn i_05888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x589F8;
    16
}
/// $058892: move.l a1, (a6)
pub fn i_058892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58894;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058894: tst.b $78(a6)
pub fn i_058894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58898;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058898: bne.w $588a2
pub fn i_058898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5889C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x588A2; return 10; }
    8
}
/// $05889C: lea.l $58412(pc), a1
pub fn i_05889c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0588A0: move.l a1, (a6)
pub fn i_0588a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588A2: jsr $49ff2.l
pub fn i_0588a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588A8);
    cpu.pc = 0x49FF2;
    16
}
/// $0588A8: bsr.w $56e1e
pub fn i_0588a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588AC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x588AC);
    cpu.pc = 0x56E1E;
    18
}
/// $0588AC: rts
pub fn i_0588ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Stand_0588ae  ($0588AE, 72 B, enemies/soldiers/states_057d04.s)
/// $0588AE: lea.l $29bd84.l, a0
pub fn i_0588ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BD84;
    20
}
/// $0588B4: jsr $28cd4.l
pub fn i_0588b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0588BA: clr.w $28(a6)
pub fn i_0588ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588BE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0588BE: lea.l $588c4(pc), a1
pub fn i_0588be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588C4;
    16
}
/// $0588C2: move.l a1, (a6)
pub fn i_0588c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588C4: jsr $56acc(pc)
pub fn i_0588c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x588C8);
    cpu.pc = 0x56ACC;
    16
}
/// $0588C8: bsr.w $56b92
pub fn i_0588c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588CC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x588CC);
    cpu.pc = 0x56B92;
    18
}
/// $0588CC: jsr $28d70.l
pub fn i_0588cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588D2);
    cpu.pc = 0x28D70;
    16
}
/// $0588D2: bcc.w $588dc
pub fn i_0588d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x588DC; return 10; }
    8
}
/// $0588D6: lea.l $57558(pc), a1
pub fn i_0588d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $0588DA: move.l a1, (a6)
pub fn i_0588da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588DC: tst.b $78(a6)
pub fn i_0588dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588E0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0588E0: bne.w $588ea
pub fn i_0588e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x588EA; return 10; }
    8
}
/// $0588E4: lea.l $58412(pc), a1
pub fn i_0588e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0588E8: move.l a1, (a6)
pub fn i_0588e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588EA: jsr $49fd0.l
pub fn i_0588ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588F0);
    cpu.pc = 0x49FD0;
    16
}
/// $0588F0: bsr.w $56e1e
pub fn i_0588f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x588F4);
    cpu.pc = 0x56E1E;
    18
}
/// $0588F4: rts
pub fn i_0588f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Jump_0588f6  ($0588F6, 114 B, enemies/soldiers/states_057d04.s)
/// $0588F6: move.w $84(a6), d0
pub fn i_0588f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0588FA: add.w $82(a6), d0
pub fn i_0588fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588FE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0588FE: sub.w $86(a6), d0
pub fn i_0588fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58902;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058902: sub.w $80(a6), d0
pub fn i_058902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58906;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058906: asr.w #$1, d0
pub fn i_058906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58908;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $058908: btst.b #$0, $3a(a6)
pub fn i_058908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5890E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05890E: seq.b d1
pub fn i_05890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58910;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $058910: ext.w d1
pub fn i_058910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58912;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $058912: eor.w d0, d1
pub fn i_058912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58914;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058914: move.w d1, $28(a6)
pub fn i_058914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58918;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058918: tst.w d0
pub fn i_058918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5891A;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05891A: bmi.w $5893e
pub fn i_05891a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5891E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5893E; return 10; }
    8
}
/// $05891E: lea.l $2b5d58.l, a0
pub fn i_05891e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58924;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5D58;
    20
}
/// $058924: jsr $28cd4.l
pub fn i_058924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5892A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5892A);
    cpu.pc = 0x28CD4;
    16
}
/// $05892A: move.w $88(a6), d0
pub fn i_05892a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5892E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05892E: lsl.w #$3, d0
pub fn i_05892e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58930;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058930: move.w d0, $2a(a6)
pub fn i_058930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58934;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058934: lea.l $58424(pc), a1
pub fn i_058934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58938;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58424;
    16
}
/// $058938: move.l a1, (a6)
pub fn i_058938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5893A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05893A: bra.w $58424
pub fn i_05893a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5893E;
    cpu.ir = 0x6000;
    cpu.pc = 0x58424;
    10
}
/// $05893E: lea.l $2b5e68.l, a0
pub fn i_05893e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58944;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5E68;
    20
}
/// $058944: jsr $28cd4.l
pub fn i_058944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5894A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5894A);
    cpu.pc = 0x28CD4;
    16
}
/// $05894A: move.w $88(a6), d0
pub fn i_05894a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5894E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05894E: lsl.w #$3, d0
pub fn i_05894e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58950;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058950: move.w d0, $2a(a6)
pub fn i_058950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58954;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058954: move.w #$ff00, $36(a6)
pub fn i_058954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5895A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05895A: bsr.w $56f8a
pub fn i_05895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5895E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5895E);
    cpu.pc = 0x56F8A;
    18
}
/// $05895E: lea.l $58424(pc), a1
pub fn i_05895e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58962;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58424;
    16
}
/// $058962: move.l a1, (a6)
pub fn i_058962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58964;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058964: bra.w $58424
pub fn i_058964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58968;
    cpu.ir = 0x6000;
    cpu.pc = 0x58424;
    10
}
// ===== Soldier_Surrender_058968  ($058968, 144 B, enemies/soldiers/states_057d04.s)
/// $058968: move.b $75(a6), d0
pub fn i_058968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5896C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05896C: lea.l $29ba70.l, a0
pub fn i_05896c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58972;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BA70;
    20
}
/// $058972: cmpi.b #$3, d0
pub fn i_058972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58976;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $058976: bne.w $58994
pub fn i_058976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5897A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58994; return 10; }
    8
}
/// $05897A: lea.l $29bbec.l, a0
pub fn i_05897a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58980;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BBEC;
    20
}
/// $058980: jsr $5e9b6.l
pub fn i_058980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58986;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58986);
    cpu.pc = 0x5E9B6;
    16
}
/// $058986: bmi.w $58990
pub fn i_058986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5898A;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x58990; return 10; }
    8
}
/// $05898A: lea.l $29bc64.l, a0
pub fn i_05898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58990;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BC64;
    20
}
/// $058990: bra.w $589b0
pub fn i_058990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58994;
    cpu.ir = 0x6000;
    cpu.pc = 0x589B0;
    10
}
/// $058994: cmpi.b #$1, d0
pub fn i_058994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58998;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $058998: bne.w $589a2
pub fn i_058998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5899C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x589A2; return 10; }
    8
}
/// $05899C: lea.l $29bbec.l, a0
pub fn i_05899c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BBEC;
    20
}
/// $0589A2: cmpi.b #$2, d0
pub fn i_0589a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589A6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $0589A6: bne.w $589b0
pub fn i_0589a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x589B0; return 10; }
    8
}
/// $0589AA: lea.l $29bc64.l, a0
pub fn i_0589aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BC64;
    20
}
/// $0589B0: jsr $28cd4.l
pub fn i_0589b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x589B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0589B6: move.l $7a(a6), d0
pub fn i_0589b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589BA;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0589BA: bsr.w $57044
pub fn i_0589ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589BE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x589BE);
    cpu.pc = 0x57044;
    18
}
/// $0589BE: clr.w $28(a6)
pub fn i_0589be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589C2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0589C2: lea.l $589c8(pc), a1
pub fn i_0589c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x589C8;
    16
}
/// $0589C6: move.l a1, (a6)
pub fn i_0589c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0589C8: bsr.w $56fa0
pub fn i_0589c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589CC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x589CC);
    cpu.pc = 0x56FA0;
    18
}
/// $0589CC: bsr.w $56acc
pub fn i_0589cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589D0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x589D0);
    cpu.pc = 0x56ACC;
    18
}
/// $0589D0: jsr $28d70.l
pub fn i_0589d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x589D6);
    cpu.pc = 0x28D70;
    16
}
/// $0589D6: bcc.w $589e0
pub fn i_0589d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x589E0; return 10; }
    8
}
/// $0589DA: lea.l $58144(pc), a1
pub fn i_0589da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $0589DE: move.l a1, (a6)
pub fn i_0589de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0589E0: tst.b $78(a6)
pub fn i_0589e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589E4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0589E4: bne.w $589ee
pub fn i_0589e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x589EE; return 10; }
    8
}
/// $0589E8: lea.l $58412(pc), a1
pub fn i_0589e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0589EC: move.l a1, (a6)
pub fn i_0589ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0589EE: jsr $49fd0.l
pub fn i_0589ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x589F4);
    cpu.pc = 0x49FD0;
    16
}
/// $0589F4: bra.w $56e1e
pub fn i_0589f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_SurrenderFlee_0589f8  ($0589F8, 294 B, enemies/soldiers/states_057d04.s)
/// $0589F8: move.b $75(a6), d0
pub fn i_0589f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x589FC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0589FC: cmpi.b #$0, d0
pub fn i_0589fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A00;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $058A00: bne.w $587ac
pub fn i_058a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x587AC; return 10; }
    8
}
/// $058A04: lea.l $29bb4c.l, a0
pub fn i_058a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BB4C;
    20
}
/// $058A0A: jsr $28cd4.l
pub fn i_058a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58A10);
    cpu.pc = 0x28CD4;
    16
}
/// $058A10: move.b #$6, $5c(a6)
pub fn i_058a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $058A16: clr.w $28(a6)
pub fn i_058a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A1A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058A1A: lea.l $58a20(pc), a1
pub fn i_058a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A20;
    16
}
/// $058A1E: move.l a1, (a6)
pub fn i_058a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058A20: bsr.w $56fa0
pub fn i_058a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A24;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58A24);
    cpu.pc = 0x56FA0;
    18
}
/// $058A24: bsr.w $56acc
pub fn i_058a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A28;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58A28);
    cpu.pc = 0x56ACC;
    18
}
/// $058A28: jsr $28d70.l
pub fn i_058a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58A2E);
    cpu.pc = 0x28D70;
    16
}
/// $058A2E: bcc.w $58a38
pub fn i_058a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58A38; return 10; }
    8
}
/// $058A32: lea.l $586b8(pc), a1
pub fn i_058a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x586B8;
    16
}
/// $058A36: move.l a1, (a6)
pub fn i_058a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058A38: tst.b $78(a6)
pub fn i_058a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A3C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058A3C: bne.w $58a46
pub fn i_058a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58A46; return 10; }
    8
}
/// $058A40: lea.l $58412(pc), a1
pub fn i_058a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058A44: move.l a1, (a6)
pub fn i_058a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058A46: jsr $49ff2.l
pub fn i_058a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58A4C);
    cpu.pc = 0x49FF2;
    16
}
/// $058A4C: bra.w $56e1e
pub fn i_058a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A50;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
/// $058A50: lea.l $2b5d58.l, a0
pub fn i_058a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A56;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5D58;
    20
}
/// $058A56: jsr $28cd4.l
pub fn i_058a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58A5C);
    cpu.pc = 0x28CD4;
    16
}
/// $058A5C: move.w #$fe67, d0
pub fn i_058a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A60;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $058A60: jsr $5dca4.l
pub fn i_058a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58A66);
    cpu.pc = 0x5DCA4;
    16
}
/// $058A66: move.w d0, $28(a6)
pub fn i_058a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A6A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058A6A: move.w #$65e, $2a(a6)
pub fn i_058a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x65E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058A70: move.w #$ff5d, $2e(a6)
pub fn i_058a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A76;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058A76: move.w #$0, $2c(a6)
pub fn i_058a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058A7C: lea.l $58a82(pc), a1
pub fn i_058a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A82;
    16
}
/// $058A80: move.l a1, (a6)
pub fn i_058a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058A82: jsr $2831e.l
pub fn i_058a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58A88);
    cpu.pc = 0x2831E;
    16
}
/// $058A88: scs.b $78(a6)
pub fn i_058a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A8C;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058A8C: bcc.w $58a96
pub fn i_058a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58A96; return 10; }
    8
}
/// $058A90: lea.l $58464(pc), a1
pub fn i_058a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58464;
    16
}
/// $058A94: move.l a1, (a6)
pub fn i_058a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058A96: jsr $28d70.l
pub fn i_058a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58A9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58A9C);
    cpu.pc = 0x28D70;
    16
}
/// $058A9C: jsr $49fd0.l
pub fn i_058a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58AA2);
    cpu.pc = 0x49FD0;
    16
}
/// $058AA2: bsr.w $56e1e
pub fn i_058aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AA6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58AA6);
    cpu.pc = 0x56E1E;
    18
}
/// $058AA6: rts
pub fn i_058aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $058AA8: bsr.w $56b92
pub fn i_058aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AAC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58AAC);
    cpu.pc = 0x56B92;
    18
}
/// $058AAC: btst.b #$0, $72(a6)
pub fn i_058aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AB2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058AB2: beq.w $585f6
pub fn i_058ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x585F6; return 10; }
    8
}
/// $058AB6: move.w #$190, $36(a6)
pub fn i_058ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ABC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058ABC: bsr.w $56f8a
pub fn i_058abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AC0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58AC0);
    cpu.pc = 0x56F8A;
    18
}
/// $058AC0: lea.l $29b9ce.l, a0
pub fn i_058ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B9CE;
    20
}
/// $058AC6: jsr $28cd4.l
pub fn i_058ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58ACC);
    cpu.pc = 0x28CD4;
    16
}
/// $058ACC: lea.l $58ad2(pc), a1
pub fn i_058acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58AD2;
    16
}
/// $058AD0: move.l a1, (a6)
pub fn i_058ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058AD2: move.w $28(a6), d0
pub fn i_058ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AD6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058AD6: beq.w $58ae8
pub fn i_058ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ADA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58AE8; return 10; }
    8
}
/// $058ADA: asr.w #$4, d0
pub fn i_058ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ADC;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $058ADC: bne.w $58ae4
pub fn i_058adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58AE4; return 10; }
    8
}
/// $058AE0: move.w #$1, d0
pub fn i_058ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $058AE4: sub.w d0, $28(a6)
pub fn i_058ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AE8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $058AE8: bsr.w $56acc
pub fn i_058ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AEC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58AEC);
    cpu.pc = 0x56ACC;
    18
}
/// $058AEC: jsr $28d70.l
pub fn i_058aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58AF2);
    cpu.pc = 0x28D70;
    16
}
/// $058AF2: bcc.w $58b12
pub fn i_058af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58B12; return 10; }
    8
}
/// $058AF6: move.b $74(a6), d0
pub fn i_058af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AFA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $058AFA: andi.b #$7, d0
pub fn i_058afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58AFE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $058AFE: beq.w $58b0c
pub fn i_058afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58B0C; return 10; }
    8
}
/// $058B02: lea.l $588ae(pc), a1
pub fn i_058b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $058B06: move.l a1, (a6)
pub fn i_058b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058B08: bra.w $58b12
pub fn i_058b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58B12;
    10
}
/// $058B0C: lea.l $591c8(pc), a1
pub fn i_058b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x591C8;
    16
}
/// $058B10: move.l a1, (a6)
pub fn i_058b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058B12: jsr $49fd0.l
pub fn i_058b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B18);
    cpu.pc = 0x49FD0;
    16
}
/// $058B18: bsr.w $56e1e
pub fn i_058b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B1C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58B1C);
    cpu.pc = 0x56E1E;
    18
}
/// $058B1C: rts
pub fn i_058b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ThrowGrenadeA_058b1e  ($058B1E, 64 B, enemies/soldiers/states_057d04.s)
/// $058B1E: lea.l $2b60ce.l, a0
pub fn i_058b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B60CE;
    20
}
/// $058B24: jsr $28cd4.l
pub fn i_058b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B2A);
    cpu.pc = 0x28CD4;
    16
}
/// $058B2A: lea.l $58b30(pc), a1
pub fn i_058b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B30;
    16
}
/// $058B2E: move.l a1, (a6)
pub fn i_058b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058B30: clr.w $2e(a6)
pub fn i_058b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B34;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058B34: clr.w $2a(a6)
pub fn i_058b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B38;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058B38: move.b $77(a6), d0
pub fn i_058b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B3C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $058B3C: jsr $77190.l
pub fn i_058b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B42);
    cpu.pc = 0x77190;
    16
}
/// $058B42: jsr $28d70.l
pub fn i_058b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B48);
    cpu.pc = 0x28D70;
    16
}
/// $058B48: bcc.w $58b52
pub fn i_058b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58B52; return 10; }
    8
}
/// $058B4C: lea.l $58b5e(pc), a1
pub fn i_058b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B5E;
    16
}
/// $058B50: move.l a1, (a6)
pub fn i_058b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058B52: jsr $49fd0.l
pub fn i_058b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B58);
    cpu.pc = 0x49FD0;
    16
}
/// $058B58: bsr.w $56e1e
pub fn i_058b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B5C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58B5C);
    cpu.pc = 0x56E1E;
    18
}
/// $058B5C: rts
pub fn i_058b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ThrowGrenadeA_Loop_058b5e  ($058B5E, 120 B, enemies/soldiers/states_057d04.s)
/// $058B5E: lea.l $2b63f6.l, a0
pub fn i_058b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B63F6;
    20
}
/// $058B64: jsr $28cd4.l
pub fn i_058b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B6A);
    cpu.pc = 0x28CD4;
    16
}
/// $058B6A: clr.w $2e(a6)
pub fn i_058b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B6E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058B6E: move.w #$154, $2a(a6)
pub fn i_058b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x154;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058B74: lea.l $58b7a(pc), a1
pub fn i_058b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B7A;
    16
}
/// $058B78: move.l a1, (a6)
pub fn i_058b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058B7A: jsr $56b92(pc)
pub fn i_058b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58B7E);
    cpu.pc = 0x56B92;
    16
}
/// $058B7E: move.b $77(a6), d0
pub fn i_058b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B82;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $058B82: jsr $77190.l
pub fn i_058b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B88);
    cpu.pc = 0x77190;
    16
}
/// $058B88: move.w d0, -(a7)
pub fn i_058b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B8A;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058B8A: jsr $28d70.l
pub fn i_058b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58B90);
    cpu.pc = 0x28D70;
    16
}
/// $058B90: bcc.w $58ba8
pub fn i_058b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58BA8; return 10; }
    8
}
/// $058B94: lea.l $58b5e(pc), a1
pub fn i_058b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B5E;
    16
}
/// $058B98: move.l a1, (a6)
pub fn i_058b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058B9A: jsr $56fa0(pc)
pub fn i_058b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58B9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58B9E);
    cpu.pc = 0x56FA0;
    16
}
/// $058B9E: bcc.w $58ba8
pub fn i_058b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58BA8; return 10; }
    8
}
/// $058BA2: lea.l $58d78(pc), a1
pub fn i_058ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58D78;
    16
}
/// $058BA6: move.l a1, (a6)
pub fn i_058ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058BA8: move.w (a7)+, d0
pub fn i_058ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BAA;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $058BAA: bpl.w $58bb8
pub fn i_058baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BAE;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x58BB8; return 10; }
    8
}
/// $058BAE: lea.l $58412(pc), a1
pub fn i_058bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058BB2: move.l a1, (a6)
pub fn i_058bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058BB4: bra.w $58bca
pub fn i_058bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x58BCA;
    10
}
/// $058BB8: subi.w #$24, d0
pub fn i_058bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BBC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x24, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058BBC: sub.w $24(a6), d0
pub fn i_058bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BC0;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058BC0: bcc.w $58bca
pub fn i_058bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58BCA; return 10; }
    8
}
/// $058BC4: lea.l $58bd6(pc), a1
pub fn i_058bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58BD6;
    16
}
/// $058BC8: move.l a1, (a6)
pub fn i_058bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058BCA: jsr $49fd0.l
pub fn i_058bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58BD0);
    cpu.pc = 0x49FD0;
    16
}
/// $058BD0: bsr.w $56e1e
pub fn i_058bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BD4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58BD4);
    cpu.pc = 0x56E1E;
    18
}
/// $058BD4: rts
pub fn i_058bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ThrowGrenadeRecover_058bd6  ($058BD6, 56 B, enemies/soldiers/states_057d04.s)
/// $058BD6: lea.l $2b6124.l, a0
pub fn i_058bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6124;
    20
}
/// $058BDC: jsr $28cd4.l
pub fn i_058bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58BE2);
    cpu.pc = 0x28CD4;
    16
}
/// $058BE2: lea.l $58be8(pc), a1
pub fn i_058be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58BE8;
    16
}
/// $058BE6: move.l a1, (a6)
pub fn i_058be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BE8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058BE8: jsr $2783a.l
pub fn i_058be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58BEE);
    cpu.pc = 0x2783A;
    16
}
/// $058BEE: bsr.w $56b92
pub fn i_058bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BF2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58BF2);
    cpu.pc = 0x56B92;
    18
}
/// $058BF2: jsr $28d70.l
pub fn i_058bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58BF8);
    cpu.pc = 0x28D70;
    16
}
/// $058BF8: bcc.w $58c02
pub fn i_058bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58BFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58C02; return 10; }
    8
}
/// $058BFC: lea.l $58c0e(pc), a1
pub fn i_058bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58C0E;
    16
}
/// $058C00: move.l a1, (a6)
pub fn i_058c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058C02: jsr $49fd0.l
pub fn i_058c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58C08);
    cpu.pc = 0x49FD0;
    16
}
/// $058C08: bsr.w $56e1e
pub fn i_058c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C0C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58C0C);
    cpu.pc = 0x56E1E;
    18
}
/// $058C0C: rts
pub fn i_058c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_HopBack_058c0e  ($058C0E, 128 B, enemies/soldiers/states_057d04.s)
/// $058C0E: clr.b $78(a6)
pub fn i_058c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C12;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $058C12: lea.l $2b6204.l, a0
pub fn i_058c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6204;
    20
}
/// $058C18: jsr $28cd4.l
pub fn i_058c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58C1E);
    cpu.pc = 0x28CD4;
    16
}
/// $058C1E: move.w #$fe67, d0
pub fn i_058c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C22;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $058C22: jsr $5dca4.l
pub fn i_058c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58C28);
    cpu.pc = 0x5DCA4;
    16
}
/// $058C28: move.w d0, $28(a6)
pub fn i_058c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C2C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058C2C: move.w #$32a, $2a(a6)
pub fn i_058c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C32;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058C32: move.w #$ffaf, $2e(a6)
pub fn i_058c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058C38: move.w #$0, $2c(a6)
pub fn i_058c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058C3E: lea.l $58c44(pc), a1
pub fn i_058c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58C44;
    16
}
/// $058C42: move.l a1, (a6)
pub fn i_058c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058C44: move.w $28(a6), d0
pub fn i_058c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C48;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058C48: beq.w $58c58
pub fn i_058c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58C58; return 10; }
    8
}
/// $058C4C: asr.w #$4, d0
pub fn i_058c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C4E;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $058C4E: seq.b d1
pub fn i_058c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C50;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $058C50: ext.w d1
pub fn i_058c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C52;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $058C52: sub.w d1, d0
pub fn i_058c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C54;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $058C54: sub.w d0, $28(a6)
pub fn i_058c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C58;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $058C58: jsr $56acc(pc)
pub fn i_058c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58C5C);
    cpu.pc = 0x56ACC;
    16
}
/// $058C5C: bsr.w $56b92
pub fn i_058c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C60;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58C60);
    cpu.pc = 0x56B92;
    18
}
/// $058C60: jsr $28d70.l
pub fn i_058c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58C66);
    cpu.pc = 0x28D70;
    16
}
/// $058C66: bcc.w $58c82
pub fn i_058c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58C82; return 10; }
    8
}
/// $058C6A: tst.b $78(a6)
pub fn i_058c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C6E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058C6E: bne.w $58c7c
pub fn i_058c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58C7C; return 10; }
    8
}
/// $058C72: lea.l $58412(pc), a1
pub fn i_058c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058C76: move.l a1, (a6)
pub fn i_058c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058C78: bra.w $58c82
pub fn i_058c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58C82;
    10
}
/// $058C7C: lea.l $58144(pc), a1
pub fn i_058c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $058C80: move.l a1, (a6)
pub fn i_058c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058C82: jsr $49fd0.l
pub fn i_058c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58C88);
    cpu.pc = 0x49FD0;
    16
}
/// $058C88: bsr.w $56e1e
pub fn i_058c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C8C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58C8C);
    cpu.pc = 0x56E1E;
    18
}
/// $058C8C: rts
pub fn i_058c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ThrowGrenadeB_058c8e  ($058C8E, 64 B, enemies/soldiers/states_057d04.s)
/// $058C8E: lea.l $2b6284.l, a0
pub fn i_058c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6284;
    20
}
/// $058C94: jsr $28cd4.l
pub fn i_058c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58C9A);
    cpu.pc = 0x28CD4;
    16
}
/// $058C9A: clr.w $2a(a6)
pub fn i_058c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58C9E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058C9E: clr.w $2e(a6)
pub fn i_058c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CA2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058CA2: lea.l $58ca8(pc), a1
pub fn i_058ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58CA8;
    16
}
/// $058CA6: move.l a1, (a6)
pub fn i_058ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058CA8: move.b $77(a6), d0
pub fn i_058ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CAC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $058CAC: jsr $77190.l
pub fn i_058cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58CB2);
    cpu.pc = 0x77190;
    16
}
/// $058CB2: jsr $28d70.l
pub fn i_058cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58CB8);
    cpu.pc = 0x28D70;
    16
}
/// $058CB8: bcc.w $58cc2
pub fn i_058cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58CC2; return 10; }
    8
}
/// $058CBC: lea.l $58cce(pc), a1
pub fn i_058cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58CCE;
    16
}
/// $058CC0: move.l a1, (a6)
pub fn i_058cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058CC2: jsr $49fd0.l
pub fn i_058cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58CC8);
    cpu.pc = 0x49FD0;
    16
}
/// $058CC8: bsr.w $56e1e
pub fn i_058cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CCC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58CCC);
    cpu.pc = 0x56E1E;
    18
}
/// $058CCC: rts
pub fn i_058ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ThrowGrenadeB_Loop_058cce  ($058CCE, 118 B, enemies/soldiers/states_057d04.s)
/// $058CCE: lea.l $2b648a.l, a0
pub fn i_058cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B648A;
    20
}
/// $058CD4: jsr $28cd4.l
pub fn i_058cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58CDA);
    cpu.pc = 0x28CD4;
    16
}
/// $058CDA: clr.w $2e(a6)
pub fn i_058cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CDE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058CDE: move.w #$feac, $2a(a6)
pub fn i_058cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEAC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058CE4: lea.l $58cea(pc), a1
pub fn i_058ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58CEA;
    16
}
/// $058CE8: move.l a1, (a6)
pub fn i_058ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058CEA: jsr $56b92(pc)
pub fn i_058cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58CEE);
    cpu.pc = 0x56B92;
    16
}
/// $058CEE: move.b $77(a6), d0
pub fn i_058cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CF2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $058CF2: jsr $77190.l
pub fn i_058cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58CF8);
    cpu.pc = 0x77190;
    16
}
/// $058CF8: move.w d0, -(a7)
pub fn i_058cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58CFA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058CFA: jsr $28d70.l
pub fn i_058cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D00);
    cpu.pc = 0x28D70;
    16
}
/// $058D00: bcc.w $58d18
pub fn i_058d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D04;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58D18; return 10; }
    8
}
/// $058D04: lea.l $58cce(pc), a1
pub fn i_058d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58CCE;
    16
}
/// $058D08: move.l a1, (a6)
pub fn i_058d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058D0A: jsr $56fa0(pc)
pub fn i_058d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58D0E);
    cpu.pc = 0x56FA0;
    16
}
/// $058D0E: bcc.w $58d18
pub fn i_058d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58D18; return 10; }
    8
}
/// $058D12: lea.l $58d78(pc), a1
pub fn i_058d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58D78;
    16
}
/// $058D16: move.l a1, (a6)
pub fn i_058d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058D18: move.w (a7)+, d0
pub fn i_058d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D1A;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $058D1A: bpl.w $58d24
pub fn i_058d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D1E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x58D24; return 10; }
    8
}
/// $058D1E: lea.l $58412(pc), a1
pub fn i_058d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058D22: move.l a1, (a6)
pub fn i_058d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058D24: jsr $27eba.l
pub fn i_058d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D2A);
    cpu.pc = 0x27EBA;
    16
}
/// $058D2A: scc.b $78(a6)
pub fn i_058d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D2E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058D2E: bcs.w $58d38
pub fn i_058d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D32;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x58D38; return 10; }
    8
}
/// $058D32: lea.l $58d44(pc), a1
pub fn i_058d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58D44;
    16
}
/// $058D36: move.l a1, (a6)
pub fn i_058d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058D38: jsr $49fd0.l
pub fn i_058d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D3E);
    cpu.pc = 0x49FD0;
    16
}
/// $058D3E: bsr.w $56e1e
pub fn i_058d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D42;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58D42);
    cpu.pc = 0x56E1E;
    18
}
/// $058D42: rts
pub fn i_058d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ThrowGrenadeBRecover_058d44  ($058D44, 52 B, enemies/soldiers/states_057d04.s)
/// $058D44: lea.l $2b63b4.l, a0
pub fn i_058d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D4A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B63B4;
    20
}
/// $058D4A: jsr $28cd4.l
pub fn i_058d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D50);
    cpu.pc = 0x28CD4;
    16
}
/// $058D50: lea.l $58d56(pc), a1
pub fn i_058d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58D56;
    16
}
/// $058D54: move.l a1, (a6)
pub fn i_058d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058D56: jsr $2783a.l
pub fn i_058d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D5C);
    cpu.pc = 0x2783A;
    16
}
/// $058D5C: jsr $28d70.l
pub fn i_058d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D62);
    cpu.pc = 0x28D70;
    16
}
/// $058D62: bcc.w $58d6c
pub fn i_058d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58D6C; return 10; }
    8
}
/// $058D66: lea.l $58144(pc), a1
pub fn i_058d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $058D6A: move.l a1, (a6)
pub fn i_058d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058D6C: jsr $49fd0.l
pub fn i_058d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D72);
    cpu.pc = 0x49FD0;
    16
}
/// $058D72: bsr.w $56e1e
pub fn i_058d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D76;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58D76);
    cpu.pc = 0x56E1E;
    18
}
/// $058D76: rts
pub fn i_058d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ThrowGrenadeAim_058d78  ($058D78, 128 B, enemies/soldiers/states_057d04.s)
/// $058D78: btst.b #$0, $72(a6)
pub fn i_058d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D7E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058D7E: beq.w $58d98
pub fn i_058d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58D98; return 10; }
    8
}
/// $058D82: lea.l $2b6596.l, a0
pub fn i_058d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6596;
    20
}
/// $058D88: jsr $28cd4.l
pub fn i_058d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58D8E);
    cpu.pc = 0x28CD4;
    16
}
/// $058D8E: move.b #$7, $5c(a6)
pub fn i_058d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D94;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $058D94: bra.w $58daa
pub fn i_058d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D98;
    cpu.ir = 0x6000;
    cpu.pc = 0x58DAA;
    10
}
/// $058D98: lea.l $2b6644.l, a0
pub fn i_058d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58D9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6644;
    20
}
/// $058D9E: jsr $28cd4.l
pub fn i_058d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58DA4);
    cpu.pc = 0x28CD4;
    16
}
/// $058DA4: move.b #$8, $5c(a6)
pub fn i_058da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $058DAA: lea.l $58db0(pc), a1
pub fn i_058daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58DB0;
    16
}
/// $058DAE: move.l a1, (a6)
pub fn i_058dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058DB0: move.b $77(a6), d0
pub fn i_058db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DB4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $058DB4: move.w $2a(a6), -(a7)
pub fn i_058db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DB8;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $058DB8: clr.w $2a(a6)
pub fn i_058db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DBC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058DBC: clr.w $2e(a6)
pub fn i_058dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DC0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058DC0: jsr $77190.l
pub fn i_058dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58DC6);
    cpu.pc = 0x77190;
    16
}
/// $058DC6: move.w (a7)+, $2a(a6)
pub fn i_058dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DCA;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    16
}
/// $058DCA: jsr $28d70.l
pub fn i_058dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58DD0);
    cpu.pc = 0x28D70;
    16
}
/// $058DD0: bcc.w $58dec
pub fn i_058dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58DEC; return 10; }
    8
}
/// $058DD4: tst.w $2a(a6)
pub fn i_058dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DD8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $058DD8: bmi.w $58de6
pub fn i_058dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DDC;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x58DE6; return 10; }
    8
}
/// $058DDC: lea.l $58b5e(pc), a1
pub fn i_058ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B5E;
    16
}
/// $058DE0: move.l a1, (a6)
pub fn i_058de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058DE2: bra.w $58dec
pub fn i_058de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x58DEC;
    10
}
/// $058DE6: lea.l $58cce(pc), a1
pub fn i_058de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58CCE;
    16
}
/// $058DEA: move.l a1, (a6)
pub fn i_058dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058DEC: jsr $49fd0.l
pub fn i_058dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58DF2);
    cpu.pc = 0x49FD0;
    16
}
/// $058DF2: bsr.w $56e1e
pub fn i_058df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DF6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58DF6);
    cpu.pc = 0x56E1E;
    18
}
/// $058DF6: rts
pub fn i_058df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58DF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TauntInit_058e08  ($058E08, 12 B, enemies/soldiers/states_057d04.s)
/// $058E08: clr.w $30(a6)
pub fn i_058e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E0C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058E0C: clr.w $28(a6)
pub fn i_058e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E10;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058E10: clr.w $2a(a6)
pub fn i_058e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E14;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== Soldier_Taunt_058e14  ($058E14, 190 B, enemies/soldiers/states_057d04.s)
/// $058E14: jsr $5e9b6.l
pub fn i_058e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58E1A);
    cpu.pc = 0x5E9B6;
    16
}
/// $058E1A: andi.w #$c, d0
pub fn i_058e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E1E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058E1E: lea.l $58df8(pc), a0
pub fn i_058e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E22;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x58DF8;
    16
}
/// $058E22: movea.l (a0, d0.w), a0
pub fn i_058e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E26;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $058E26: jsr $28cd4.l
pub fn i_058e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58E2C);
    cpu.pc = 0x28CD4;
    16
}
/// $058E2C: lea.l $58e32(pc), a1
pub fn i_058e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58E32;
    16
}
/// $058E30: move.l a1, (a6)
pub fn i_058e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058E32: jsr $56acc(pc)
pub fn i_058e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58E36);
    cpu.pc = 0x56ACC;
    16
}
/// $058E36: jsr $56b92(pc)
pub fn i_058e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58E3A);
    cpu.pc = 0x56B92;
    16
}
/// $058E3A: jsr $28d70.l
pub fn i_058e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58E40);
    cpu.pc = 0x28D70;
    16
}
/// $058E40: bcc.w $58e4a
pub fn i_058e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58E4A; return 10; }
    8
}
/// $058E44: lea.l $58e14(pc), a1
pub fn i_058e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58E14;
    16
}
/// $058E48: move.l a1, (a6)
pub fn i_058e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058E4A: btst.b #$0, $73(a6)
pub fn i_058e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E50;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058E50: beq.w $58e5e
pub fn i_058e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E54;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58E5E; return 10; }
    8
}
/// $058E54: move.b #$1, $30(a6)
pub fn i_058e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E5A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write8(ad2, v1 as u8);
    12
}
/// $058E5A: bra.w $58ec6
pub fn i_058e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x58EC6;
    10
}
/// $058E5E: tst.b $30(a6)
pub fn i_058e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E62;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058E62: beq.w $58e6c
pub fn i_058e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58E6C; return 10; }
    8
}
/// $058E66: lea.l $58ed2(pc), a1
pub fn i_058e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58ED2;
    16
}
/// $058E6A: move.l a1, (a6)
pub fn i_058e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058E6C: move.w $80(a6), d1
pub fn i_058e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E70;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058E70: jsr $5e9b6.l
pub fn i_058e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58E76);
    cpu.pc = 0x5E9B6;
    16
}
/// $058E76: move.w d0, -(a7)
pub fn i_058e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E78;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058E78: andi.w #$ff, d0
pub fn i_058e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E7C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058E7C: cmp.w d1, d0
pub fn i_058e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E7E;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058E7E: bcc.w $58e86
pub fn i_058e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58E86; return 10; }
    8
}
/// $058E82: move.b (a7), d0
pub fn i_058e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E84;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058E84: cmp.w d1, d0
pub fn i_058e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E86;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058E86: addq.w #$2, a7
pub fn i_058e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E88;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058E88: bcc.w $58ea6
pub fn i_058e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58EA6; return 10; }
    8
}
/// $058E8C: btst.b #$0, $72(a6)
pub fn i_058e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E92;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058E92: beq.w $58ea0
pub fn i_058e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58EA0; return 10; }
    8
}
/// $058E96: lea.l $58ed2(pc), a1
pub fn i_058e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58ED2;
    16
}
/// $058E9A: move.l a1, (a6)
pub fn i_058e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058E9C: bra.w $58ea6
pub fn i_058e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x58EA6;
    10
}
/// $058EA0: lea.l $585f6(pc), a1
pub fn i_058ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $058EA4: move.l a1, (a6)
pub fn i_058ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058EA6: btst.b #$3, $73(a6)
pub fn i_058ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EAC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058EAC: beq.w $58eb6
pub fn i_058eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58EB6; return 10; }
    8
}
/// $058EB0: lea.l $58ed2(pc), a1
pub fn i_058eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58ED2;
    16
}
/// $058EB4: move.l a1, (a6)
pub fn i_058eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058EB6: btst.b #$2, $73(a6)
pub fn i_058eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EBC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058EBC: beq.w $58ec6
pub fn i_058ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58EC6; return 10; }
    8
}
/// $058EC0: lea.l $585f6(pc), a1
pub fn i_058ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $058EC4: move.l a1, (a6)
pub fn i_058ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058EC6: jsr $49fd0.l
pub fn i_058ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58ECC);
    cpu.pc = 0x49FD0;
    16
}
/// $058ECC: bsr.w $56e1e
pub fn i_058ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ED0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58ED0);
    cpu.pc = 0x56E1E;
    18
}
/// $058ED0: rts
pub fn i_058ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ED2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TauntEnd_058ed2  ($058ED2, 76 B, enemies/soldiers/states_057d04.s)
/// $058ED2: lea.l $29bfc4.l, a0
pub fn i_058ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58ED8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFC4;
    20
}
/// $058ED8: jsr $28cd4.l
pub fn i_058ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58EDE);
    cpu.pc = 0x28CD4;
    16
}
/// $058EDE: lea.l $58ee4(pc), a1
pub fn i_058ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58EE4;
    16
}
/// $058EE2: move.l a1, (a6)
pub fn i_058ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058EE4: jsr $2783a.l
pub fn i_058ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58EEA);
    cpu.pc = 0x2783A;
    16
}
/// $058EEA: bsr.w $56b92
pub fn i_058eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EEE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58EEE);
    cpu.pc = 0x56B92;
    18
}
/// $058EEE: jsr $28d70.l
pub fn i_058eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58EF4);
    cpu.pc = 0x28D70;
    16
}
/// $058EF4: bcc.w $58f12
pub fn i_058ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58F12; return 10; }
    8
}
/// $058EF8: btst.b #$0, $72(a6)
pub fn i_058ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58EFE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058EFE: beq.w $58f0c
pub fn i_058efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58F0C; return 10; }
    8
}
/// $058F02: lea.l $585f6(pc), a1
pub fn i_058f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $058F06: move.l a1, (a6)
pub fn i_058f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058F08: bra.w $58f12
pub fn i_058f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58F12;
    10
}
/// $058F0C: lea.l $57558(pc), a1
pub fn i_058f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $058F10: move.l a1, (a6)
pub fn i_058f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058F12: jsr $49fd0.l
pub fn i_058f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58F18);
    cpu.pc = 0x49FD0;
    16
}
/// $058F18: bsr.w $56e1e
pub fn i_058f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F1C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58F1C);
    cpu.pc = 0x56E1E;
    18
}
/// $058F1C: rts
pub fn i_058f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $058F82: bset.b #$1, $12(a6)
pub fn i_058f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F88;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $058F88: bset.b #$4, $72(a6)
pub fn i_058f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F8E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $058F8E: move.b #$1, $98(a6)
pub fn i_058f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F94;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $058F94: jsr $27f60.l
pub fn i_058f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58F9A);
    cpu.pc = 0x27F60;
    16
}
/// $058F9A: scc.b $78(a6)
pub fn i_058f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58F9E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058F9E: bra.w $59062
pub fn i_058f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x59062;
    10
}
/// $058FC2: bset.b #$1, $12(a6)
pub fn i_058fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FC8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $058FC8: bset.b #$4, $72(a6)
pub fn i_058fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FCE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $058FCE: move.b #$1, $98(a6)
pub fn i_058fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FD4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $058FD4: jsr $27f60.l
pub fn i_058fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58FDA);
    cpu.pc = 0x27F60;
    16
}
/// $058FDA: scc.b $78(a6)
pub fn i_058fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FDE;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058FDE: bra.w $591de
pub fn i_058fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x591DE;
    10
}
/// $058FE2: bset.b #$1, $12(a6)
pub fn i_058fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FE8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $058FE8: bclr.b #$4, $72(a6)
pub fn i_058fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FEE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $058FEE: move.b #$1, $98(a6)
pub fn i_058fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $058FF4: jsr $27f60.l
pub fn i_058ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58FFA);
    cpu.pc = 0x27F60;
    16
}
/// $058FFA: scc.b $78(a6)
pub fn i_058ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58FFE;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058FFE: bra.w $59062
pub fn i_058ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59002;
    cpu.ir = 0x6000;
    cpu.pc = 0x59062;
    10
}
/// $059042: bset.b #$1, $12(a6)
pub fn i_059042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59048;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $059048: bclr.b #$4, $72(a6)
pub fn i_059048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5904E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05904E: move.b #$1, $98(a6)
pub fn i_05904e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59054;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059054: jsr $27f60.l
pub fn i_059054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5905A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5905A);
    cpu.pc = 0x27F60;
    16
}
/// $05905A: scc.b $78(a6)
pub fn i_05905a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5905E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $05905E: bra.w $59228
pub fn i_05905e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59062;
    cpu.ir = 0x6000;
    cpu.pc = 0x59228;
    10
}
// ===== Soldier_SpawnFaceTarget_059062  ($059062, 36 B, enemies/soldiers/states_057d04.s)
/// $059062: move.w #$ffff, d0
pub fn i_059062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59066;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059066: jsr $2352.l
pub fn i_059066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5906C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5906C);
    cpu.pc = 0x2352;
    16
}
/// $05906C: cmpi.w #$a0, $22(a6)
pub fn i_05906c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59072;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $059072: bmi.w $59080
pub fn i_059072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59076;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x59080; return 10; }
    8
}
/// $059076: bset.b #$0, $3a(a6)
pub fn i_059076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5907C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05907C: bra.w $59086
pub fn i_05907c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59080;
    cpu.ir = 0x6000;
    cpu.pc = 0x59086;
    10
}
/// $059080: bclr.b #$0, $3a(a6)
pub fn i_059080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59086;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Soldier_SpawnEnter_059086  ($059086, 192 B, enemies/soldiers/states_057d04.s)
/// $059086: btst.b #$4, $72(a6)
pub fn i_059086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5908C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05908C: bne.w $590a0
pub fn i_05908c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59090;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x590A0; return 10; }
    8
}
/// $059090: lea.l $29b744.l, a0
pub fn i_059090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59096;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $059096: jsr $28cd4.l
pub fn i_059096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5909C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5909C);
    cpu.pc = 0x28CD4;
    16
}
/// $05909C: bra.w $590ac
pub fn i_05909c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x590AC;
    10
}
/// $0590A0: lea.l $2b604a.l, a0
pub fn i_0590a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B604A;
    20
}
/// $0590A6: jsr $28cd4.l
pub fn i_0590a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x590AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0590AC: lea.l $2b75d0.l, a0
pub fn i_0590ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B75D0;
    20
}
/// $0590B2: jsr $799de.l
pub fn i_0590b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x590B8);
    cpu.pc = 0x799DE;
    16
}
/// $0590B8: move.w d0, $36(a6)
pub fn i_0590b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0590BC: bsr.w $56f8a
pub fn i_0590bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590C0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x590C0);
    cpu.pc = 0x56F8A;
    18
}
/// $0590C0: clr.w $2a(a6)
pub fn i_0590c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590C4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0590C4: lea.l $590ca(pc), a1
pub fn i_0590c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x590CA;
    16
}
/// $0590C8: move.l a1, (a6)
pub fn i_0590c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0590CA: jsr $56acc(pc)
pub fn i_0590ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x590CE);
    cpu.pc = 0x56ACC;
    16
}
/// $0590CE: bsr.w $56b92
pub fn i_0590ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590D2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x590D2);
    cpu.pc = 0x56B92;
    18
}
/// $0590D2: jsr $28d70.l
pub fn i_0590d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x590D8);
    cpu.pc = 0x28D70;
    16
}
/// $0590D8: btst.b #$0, $72(a6)
pub fn i_0590d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590DE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0590DE: beq.w $5910c
pub fn i_0590de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5910C; return 10; }
    8
}
/// $0590E2: jsr $5e9b6.l
pub fn i_0590e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x590E8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0590E8: move.w d0, -(a7)
pub fn i_0590e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590EA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0590EA: andi.w #$ff, d0
pub fn i_0590ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590EE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0590EE: cmp.w $80(a6), d0
pub fn i_0590ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590F2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0590F2: bcc.w $590fc
pub fn i_0590f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x590FC; return 10; }
    8
}
/// $0590F6: move.b (a7), d0
pub fn i_0590f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590F8;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0590F8: cmp.w $80(a6), d0
pub fn i_0590f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590FC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0590FC: addq.w #$2, a7
pub fn i_0590fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x590FE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0590FE: bcc.w $59108
pub fn i_0590fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59102;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x59108; return 10; }
    8
}
/// $059102: lea.l $59228(pc), a1
pub fn i_059102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59106;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59228;
    16
}
/// $059106: move.l a1, (a6)
pub fn i_059106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59108;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059108: bra.w $5911c
pub fn i_059108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5910C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5911C;
    10
}
/// $05910C: btst.b #$6, $72(a6)
pub fn i_05910c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59112;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $059112: beq.w $5911c
pub fn i_059112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59116;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5911C; return 10; }
    8
}
/// $059116: lea.l $5925a(pc), a1
pub fn i_059116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5911A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5925A;
    16
}
/// $05911A: move.l a1, (a6)
pub fn i_05911a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5911C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05911C: btst.b #$5, $72(a6)
pub fn i_05911c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59122;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $059122: beq.w $5912c
pub fn i_059122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59126;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5912C; return 10; }
    8
}
/// $059126: lea.l $59146(pc), a1
pub fn i_059126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5912A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59146;
    16
}
/// $05912A: move.l a1, (a6)
pub fn i_05912a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5912C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05912C: tst.b $78(a6)
pub fn i_05912c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59130;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $059130: bne.w $5913a
pub fn i_059130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59134;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5913A; return 10; }
    8
}
/// $059134: lea.l $5925a(pc), a1
pub fn i_059134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59138;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5925A;
    16
}
/// $059138: move.l a1, (a6)
pub fn i_059138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5913A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05913A: jsr $49fd0.l
pub fn i_05913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59140);
    cpu.pc = 0x49FD0;
    16
}
/// $059140: bsr.w $56e1e
pub fn i_059140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59144;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x59144);
    cpu.pc = 0x56E1E;
    18
}
/// $059144: rts
pub fn i_059144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59146;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnWait_059146  ($059146, 130 B, enemies/soldiers/states_057d04.s)
/// $059146: btst.b #$0, $72(a6)
pub fn i_059146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5914C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05914C: bne.w $59228
pub fn i_05914c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59150;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59228; return 10; }
    8
}
/// $059150: move.b #$b4, $5c(a6)
pub fn i_059150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59156;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059156: lea.l $2b6ef8.l, a0
pub fn i_059156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5915C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6EF8;
    20
}
/// $05915C: jsr $28cd4.l
pub fn i_05915c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59162);
    cpu.pc = 0x28CD4;
    16
}
/// $059162: lea.l $59168(pc), a1
pub fn i_059162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59166;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59168;
    16
}
/// $059166: move.l a1, (a6)
pub fn i_059166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59168;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059168: jsr $56acc(pc)
pub fn i_059168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5916C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5916C);
    cpu.pc = 0x56ACC;
    16
}
/// $05916C: bsr.w $56b92
pub fn i_05916c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59170;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x59170);
    cpu.pc = 0x56B92;
    18
}
/// $059170: jsr $28d70.l
pub fn i_059170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59176;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59176);
    cpu.pc = 0x28D70;
    16
}
/// $059176: btst.b #$0, $72(a6)
pub fn i_059176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5917C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05917C: beq.w $59190
pub fn i_05917c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59180;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x59190; return 10; }
    8
}
/// $059180: btst.b #$0, $73(a6)
pub fn i_059180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59186;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $059186: bne.w $59190
pub fn i_059186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5918A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59190; return 10; }
    8
}
/// $05918A: lea.l $59228(pc), a1
pub fn i_05918a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5918E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59228;
    16
}
/// $05918E: move.l a1, (a6)
pub fn i_05918e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59190;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059190: btst.b #$5, $72(a6)
pub fn i_059190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59196;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $059196: bne.w $591a0
pub fn i_059196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5919A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x591A0; return 10; }
    8
}
/// $05919A: lea.l $59086(pc), a1
pub fn i_05919a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5919E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59086;
    16
}
/// $05919E: move.l a1, (a6)
pub fn i_05919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0591A0: tst.b $78(a6)
pub fn i_0591a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591A4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0591A4: bne.w $591ae
pub fn i_0591a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x591AE; return 10; }
    8
}
/// $0591A8: lea.l $592b2(pc), a1
pub fn i_0591a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x592B2;
    16
}
/// $0591AC: move.l a1, (a6)
pub fn i_0591ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0591AE: subq.b #$1, $5c(a6)
pub fn i_0591ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591B2;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0591B2: bne.w $591bc
pub fn i_0591b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x591BC; return 10; }
    8
}
/// $0591B6: lea.l $59228(pc), a1
pub fn i_0591b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59228;
    16
}
/// $0591BA: move.l a1, (a6)
pub fn i_0591ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0591BC: jsr $49fd0.l
pub fn i_0591bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x591C2);
    cpu.pc = 0x49FD0;
    16
}
/// $0591C2: bsr.w $56e1e
pub fn i_0591c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591C6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x591C6);
    cpu.pc = 0x56E1E;
    18
}
/// $0591C6: rts
pub fn i_0591c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnStand_0591c8  ($0591C8, 44 B, enemies/soldiers/states_057d04.s)
/// $0591C8: lea.l $29bd84.l, a0
pub fn i_0591c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BD84;
    20
}
/// $0591CE: jsr $28cd4.l
pub fn i_0591ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x591D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0591D4: lea.l $591f4(pc), a1
pub fn i_0591d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x591D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x591F4;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x057780, i_057780),
    (0x057784, i_057784),
    (0x057786, i_057786),
    (0x05778C, i_05778c),
    (0x057790, i_057790),
    (0x057794, i_057794),
    (0x057796, i_057796),
    (0x05779C, i_05779c),
    (0x0577A0, i_0577a0),
    (0x0577A4, i_0577a4),
    (0x0577A6, i_0577a6),
    (0x0577AA, i_0577aa),
    (0x0577AE, i_0577ae),
    (0x0577B0, i_0577b0),
    (0x0577B6, i_0577b6),
    (0x0577BA, i_0577ba),
    (0x0577BC, i_0577bc),
    (0x0577C0, i_0577c0),
    (0x0577C4, i_0577c4),
    (0x0577CA, i_0577ca),
    (0x0577CE, i_0577ce),
    (0x0577D0, i_0577d0),
    (0x0577D2, i_0577d2),
    (0x0577D8, i_0577d8),
    (0x0577DA, i_0577da),
    (0x0577DE, i_0577de),
    (0x0577E0, i_0577e0),
    (0x0577E4, i_0577e4),
    (0x0577E6, i_0577e6),
    (0x0577E8, i_0577e8),
    (0x0577EA, i_0577ea),
    (0x0577EE, i_0577ee),
    (0x0577F2, i_0577f2),
    (0x0577F4, i_0577f4),
    (0x0577FA, i_0577fa),
    (0x0577FE, i_0577fe),
    (0x057802, i_057802),
    (0x057804, i_057804),
    (0x057808, i_057808),
    (0x05780C, i_05780c),
    (0x057810, i_057810),
    (0x057812, i_057812),
    (0x057818, i_057818),
    (0x05781C, i_05781c),
    (0x057820, i_057820),
    (0x057822, i_057822),
    (0x057828, i_057828),
    (0x05782C, i_05782c),
    (0x057830, i_057830),
    (0x057832, i_057832),
    (0x057838, i_057838),
    (0x05783C, i_05783c),
    (0x05783E, i_05783e),
    (0x057842, i_057842),
    (0x057846, i_057846),
    (0x05784C, i_05784c),
    (0x057852, i_057852),
    (0x057856, i_057856),
    (0x05785C, i_05785c),
    (0x057860, i_057860),
    (0x057866, i_057866),
    (0x05786C, i_05786c),
    (0x057872, i_057872),
    (0x057876, i_057876),
    (0x05787A, i_05787a),
    (0x05787C, i_05787c),
    (0x057880, i_057880),
    (0x057884, i_057884),
    (0x057888, i_057888),
    (0x05788C, i_05788c),
    (0x057890, i_057890),
    (0x057896, i_057896),
    (0x05789C, i_05789c),
    (0x0578A0, i_0578a0),
    (0x0578A6, i_0578a6),
    (0x0578AA, i_0578aa),
    (0x0578B0, i_0578b0),
    (0x0578B6, i_0578b6),
    (0x0578BC, i_0578bc),
    (0x0578C0, i_0578c0),
    (0x0578C4, i_0578c4),
    (0x0578C8, i_0578c8),
    (0x0578CC, i_0578cc),
    (0x0578D2, i_0578d2),
    (0x0578D8, i_0578d8),
    (0x057908, i_057908),
    (0x05790C, i_05790c),
    (0x057910, i_057910),
    (0x057916, i_057916),
    (0x05791A, i_05791a),
    (0x057920, i_057920),
    (0x057924, i_057924),
    (0x05792A, i_05792a),
    (0x05792E, i_05792e),
    (0x057934, i_057934),
    (0x057938, i_057938),
    (0x05793E, i_05793e),
    (0x057942, i_057942),
    (0x057944, i_057944),
    (0x05794A, i_05794a),
    (0x05794E, i_05794e),
    (0x057952, i_057952),
    (0x057954, i_057954),
    (0x05795A, i_05795a),
    (0x05795E, i_05795e),
    (0x057962, i_057962),
    (0x057966, i_057966),
    (0x05796A, i_05796a),
    (0x057970, i_057970),
    (0x057976, i_057976),
    (0x05797A, i_05797a),
    (0x05797E, i_05797e),
    (0x057982, i_057982),
    (0x057986, i_057986),
    (0x05798A, i_05798a),
    (0x05798E, i_05798e),
    (0x057992, i_057992),
    (0x057994, i_057994),
    (0x05799A, i_05799a),
    (0x05799E, i_05799e),
    (0x0579A4, i_0579a4),
    (0x0579AA, i_0579aa),
    (0x0579B0, i_0579b0),
    (0x0579B4, i_0579b4),
    (0x0579B8, i_0579b8),
    (0x0579BE, i_0579be),
    (0x0579C2, i_0579c2),
    (0x0579C8, i_0579c8),
    (0x0579CE, i_0579ce),
    (0x0579D4, i_0579d4),
    (0x0579D8, i_0579d8),
    (0x0579DE, i_0579de),
    (0x0579E2, i_0579e2),
    (0x0579E6, i_0579e6),
    (0x0579EC, i_0579ec),
    (0x0579F0, i_0579f0),
    (0x0579F6, i_0579f6),
    (0x0579FC, i_0579fc),
    (0x057A02, i_057a02),
    (0x057A06, i_057a06),
    (0x057A0C, i_057a0c),
    (0x057A10, i_057a10),
    (0x057A14, i_057a14),
    (0x057A1A, i_057a1a),
    (0x057A1E, i_057a1e),
    (0x057A24, i_057a24),
    (0x057A2A, i_057a2a),
    (0x057A30, i_057a30),
    (0x057A34, i_057a34),
    (0x057A38, i_057a38),
    (0x057A3E, i_057a3e),
    (0x057A42, i_057a42),
    (0x057A48, i_057a48),
    (0x057A4E, i_057a4e),
    (0x057A54, i_057a54),
    (0x057A5A, i_057a5a),
    (0x057A5E, i_057a5e),
    (0x057A60, i_057a60),
    (0x057A66, i_057a66),
    (0x057A6A, i_057a6a),
    (0x057A6E, i_057a6e),
    (0x057A70, i_057a70),
    (0x057A76, i_057a76),
    (0x057A7A, i_057a7a),
    (0x057A7E, i_057a7e),
    (0x057A84, i_057a84),
    (0x057A8A, i_057a8a),
    (0x057A8E, i_057a8e),
    (0x057A92, i_057a92),
    (0x057A94, i_057a94),
    (0x057A9A, i_057a9a),
    (0x057A9E, i_057a9e),
    (0x057AA4, i_057aa4),
    (0x057AAA, i_057aaa),
    (0x057AB0, i_057ab0),
    (0x057AB4, i_057ab4),
    (0x057AB6, i_057ab6),
    (0x057ABC, i_057abc),
    (0x057AC0, i_057ac0),
    (0x057AC6, i_057ac6),
    (0x057ACC, i_057acc),
    (0x057AD0, i_057ad0),
    (0x057AD4, i_057ad4),
    (0x057AD8, i_057ad8),
    (0x057ADA, i_057ada),
    (0x057AE0, i_057ae0),
    (0x057AE4, i_057ae4),
    (0x057AEA, i_057aea),
    (0x057AEE, i_057aee),
    (0x057AF4, i_057af4),
    (0x057AF8, i_057af8),
    (0x057AFE, i_057afe),
    (0x057B04, i_057b04),
    (0x057B06, i_057b06),
    (0x057B0C, i_057b0c),
    (0x057B12, i_057b12),
    (0x057B18, i_057b18),
    (0x057B1C, i_057b1c),
    (0x057B22, i_057b22),
    (0x057B26, i_057b26),
    (0x057B2A, i_057b2a),
    (0x057B30, i_057b30),
    (0x057B34, i_057b34),
    (0x057B38, i_057b38),
    (0x057B3C, i_057b3c),
    (0x057B44, i_057b44),
    (0x057B48, i_057b48),
    (0x057B4C, i_057b4c),
    (0x057B50, i_057b50),
    (0x057B54, i_057b54),
    (0x057B58, i_057b58),
    (0x057B5C, i_057b5c),
    (0x057B60, i_057b60),
    (0x057B64, i_057b64),
    (0x057B66, i_057b66),
    (0x057B6A, i_057b6a),
    (0x057B6E, i_057b6e),
    (0x057B72, i_057b72),
    (0x057B74, i_057b74),
    (0x057B78, i_057b78),
    (0x057B7C, i_057b7c),
    (0x057B80, i_057b80),
    (0x057B84, i_057b84),
    (0x057B88, i_057b88),
    (0x057B8C, i_057b8c),
    (0x057B90, i_057b90),
    (0x057B94, i_057b94),
    (0x057B98, i_057b98),
    (0x057B9C, i_057b9c),
    (0x057BA0, i_057ba0),
    (0x057BA6, i_057ba6),
    (0x057BAA, i_057baa),
    (0x057BAE, i_057bae),
    (0x057BB0, i_057bb0),
    (0x057BB4, i_057bb4),
    (0x057BBA, i_057bba),
    (0x057BC0, i_057bc0),
    (0x057BC6, i_057bc6),
    (0x057BCA, i_057bca),
    (0x057BCE, i_057bce),
    (0x057BD2, i_057bd2),
    (0x057BD6, i_057bd6),
    (0x057BDC, i_057bdc),
    (0x057BE0, i_057be0),
    (0x057BE4, i_057be4),
    (0x057BE8, i_057be8),
    (0x057BEA, i_057bea),
    (0x057BEC, i_057bec),
    (0x057BF0, i_057bf0),
    (0x057BF4, i_057bf4),
    (0x057BF8, i_057bf8),
    (0x057BFA, i_057bfa),
    (0x057BFE, i_057bfe),
    (0x057C02, i_057c02),
    (0x057C04, i_057c04),
    (0x057C08, i_057c08),
    (0x057C0A, i_057c0a),
    (0x057C0E, i_057c0e),
    (0x057C12, i_057c12),
    (0x057C16, i_057c16),
    (0x057C18, i_057c18),
    (0x057C1C, i_057c1c),
    (0x057C1E, i_057c1e),
    (0x057C22, i_057c22),
    (0x057C26, i_057c26),
    (0x057C2A, i_057c2a),
    (0x057C30, i_057c30),
    (0x057C34, i_057c34),
    (0x057C38, i_057c38),
    (0x057C3A, i_057c3a),
    (0x057C3E, i_057c3e),
    (0x057C42, i_057c42),
    (0x057C46, i_057c46),
    (0x057C4A, i_057c4a),
    (0x057C4E, i_057c4e),
    (0x057C52, i_057c52),
    (0x057C56, i_057c56),
    (0x057C5A, i_057c5a),
    (0x057C5E, i_057c5e),
    (0x057C64, i_057c64),
    (0x057C68, i_057c68),
    (0x057C6C, i_057c6c),
    (0x057C70, i_057c70),
    (0x057C74, i_057c74),
    (0x057C76, i_057c76),
    (0x057C7A, i_057c7a),
    (0x057C7E, i_057c7e),
    (0x057C82, i_057c82),
    (0x057C86, i_057c86),
    (0x057C88, i_057c88),
    (0x057C8C, i_057c8c),
    (0x057C90, i_057c90),
    (0x057C94, i_057c94),
    (0x057C96, i_057c96),
    (0x057C9A, i_057c9a),
    (0x057C9E, i_057c9e),
    (0x057CA2, i_057ca2),
    (0x057CA4, i_057ca4),
    (0x057CA8, i_057ca8),
    (0x057CAE, i_057cae),
    (0x057CB2, i_057cb2),
    (0x057CB8, i_057cb8),
    (0x057CBC, i_057cbc),
    (0x057CBE, i_057cbe),
    (0x057CC0, i_057cc0),
    (0x057CC4, i_057cc4),
    (0x057CC8, i_057cc8),
    (0x057CCE, i_057cce),
    (0x057CD2, i_057cd2),
    (0x057CD6, i_057cd6),
    (0x057CDA, i_057cda),
    (0x057CDE, i_057cde),
    (0x057CE2, i_057ce2),
    (0x057CE6, i_057ce6),
    (0x057CEA, i_057cea),
    (0x057CEE, i_057cee),
    (0x057CF2, i_057cf2),
    (0x057CF4, i_057cf4),
    (0x057CF8, i_057cf8),
    (0x057CFC, i_057cfc),
    (0x057D00, i_057d00),
    (0x057D02, i_057d02),
    (0x057D04, i_057d04),
    (0x057D0A, i_057d0a),
    (0x057D0E, i_057d0e),
    (0x057D12, i_057d12),
    (0x057D18, i_057d18),
    (0x057D1C, i_057d1c),
    (0x057D22, i_057d22),
    (0x057D28, i_057d28),
    (0x057D2C, i_057d2c),
    (0x057D30, i_057d30),
    (0x057D32, i_057d32),
    (0x057D36, i_057d36),
    (0x057D3A, i_057d3a),
    (0x057D3E, i_057d3e),
    (0x057D42, i_057d42),
    (0x057D44, i_057d44),
    (0x057D4A, i_057d4a),
    (0x057D4E, i_057d4e),
    (0x057D52, i_057d52),
    (0x057D54, i_057d54),
    (0x057D58, i_057d58),
    (0x057D5C, i_057d5c),
    (0x057D60, i_057d60),
    (0x057D64, i_057d64),
    (0x057D6A, i_057d6a),
    (0x057ECE, i_057ece),
    (0x057ED6, i_057ed6),
    (0x057EDE, i_057ede),
    (0x057EE2, i_057ee2),
    (0x057EEA, i_057eea),
    (0x057EF2, i_057ef2),
    (0x057EF6, i_057ef6),
    (0x057EFE, i_057efe),
    (0x057F06, i_057f06),
    (0x057F0A, i_057f0a),
    (0x057F10, i_057f10),
    (0x057F14, i_057f14),
    (0x057F16, i_057f16),
    (0x057F1A, i_057f1a),
    (0x057F20, i_057f20),
    (0x057F24, i_057f24),
    (0x057F28, i_057f28),
    (0x057F2C, i_057f2c),
    (0x057F30, i_057f30),
    (0x057F32, i_057f32),
    (0x057F36, i_057f36),
    (0x057F3A, i_057f3a),
    (0x057F40, i_057f40),
    (0x057F44, i_057f44),
    (0x057F48, i_057f48),
    (0x057F4A, i_057f4a),
    (0x057F4E, i_057f4e),
    (0x057F54, i_057f54),
    (0x057F5A, i_057f5a),
    (0x057F5E, i_057f5e),
    (0x057F62, i_057f62),
    (0x057F66, i_057f66),
    (0x057F6A, i_057f6a),
    (0x057F70, i_057f70),
    (0x057F74, i_057f74),
    (0x057F7A, i_057f7a),
    (0x057F7E, i_057f7e),
    (0x057F84, i_057f84),
    (0x057F88, i_057f88),
    (0x057F8E, i_057f8e),
    (0x057F94, i_057f94),
    (0x057F9A, i_057f9a),
    (0x057F9E, i_057f9e),
    (0x057FA0, i_057fa0),
    (0x057FA6, i_057fa6),
    (0x057FAA, i_057faa),
    (0x057FAE, i_057fae),
    (0x057FB2, i_057fb2),
    (0x057FB4, i_057fb4),
    (0x057FBA, i_057fba),
    (0x057FC0, i_057fc0),
    (0x057FC4, i_057fc4),
    (0x057FC6, i_057fc6),
    (0x057FCA, i_057fca),
    (0x057FCE, i_057fce),
    (0x057FD4, i_057fd4),
    (0x057FD8, i_057fd8),
    (0x057FDC, i_057fdc),
    (0x057FE0, i_057fe0),
    (0x057FE4, i_057fe4),
    (0x057FEA, i_057fea),
    (0x057FEE, i_057fee),
    (0x057FF4, i_057ff4),
    (0x057FFA, i_057ffa),
    (0x058000, i_058000),
    (0x058004, i_058004),
    (0x05800A, i_05800a),
    (0x05800E, i_05800e),
    (0x058014, i_058014),
    (0x05801A, i_05801a),
    (0x058020, i_058020),
    (0x058024, i_058024),
    (0x058026, i_058026),
    (0x05802C, i_05802c),
    (0x058030, i_058030),
    (0x058034, i_058034),
    (0x058038, i_058038),
    (0x05803A, i_05803a),
    (0x058040, i_058040),
    (0x058046, i_058046),
    (0x05804A, i_05804a),
    (0x05804C, i_05804c),
    (0x058052, i_058052),
    (0x058056, i_058056),
    (0x05805C, i_05805c),
    (0x058062, i_058062),
    (0x058068, i_058068),
    (0x05806C, i_05806c),
    (0x058070, i_058070),
    (0x058072, i_058072),
    (0x058076, i_058076),
    (0x05807C, i_05807c),
    (0x058082, i_058082),
    (0x058088, i_058088),
    (0x05808E, i_05808e),
    (0x058090, i_058090),
    (0x058094, i_058094),
    (0x058098, i_058098),
    (0x05809C, i_05809c),
    (0x05809E, i_05809e),
    (0x0580A2, i_0580a2),
    (0x0580A6, i_0580a6),
    (0x0580AA, i_0580aa),
    (0x0580AC, i_0580ac),
    (0x0580B0, i_0580b0),
    (0x0580B6, i_0580b6),
    (0x0580BA, i_0580ba),
    (0x0580BE, i_0580be),
    (0x0580C0, i_0580c0),
    (0x0580C4, i_0580c4),
    (0x0580CA, i_0580ca),
    (0x0580CC, i_0580cc),
    (0x0580D0, i_0580d0),
    (0x0580D4, i_0580d4),
    (0x0580D8, i_0580d8),
    (0x0580DA, i_0580da),
    (0x0580DE, i_0580de),
    (0x0580E0, i_0580e0),
    (0x0580E4, i_0580e4),
    (0x0580E8, i_0580e8),
    (0x0580EA, i_0580ea),
    (0x0580EE, i_0580ee),
    (0x0580F2, i_0580f2),
    (0x0580F6, i_0580f6),
    (0x0580F8, i_0580f8),
    (0x0580FC, i_0580fc),
    (0x058100, i_058100),
    (0x058104, i_058104),
    (0x058106, i_058106),
    (0x05810C, i_05810c),
    (0x05810E, i_05810e),
    (0x058112, i_058112),
    (0x058116, i_058116),
    (0x05811A, i_05811a),
    (0x05811C, i_05811c),
    (0x058120, i_058120),
    (0x058122, i_058122),
    (0x058126, i_058126),
    (0x05812A, i_05812a),
    (0x05812C, i_05812c),
    (0x058130, i_058130),
    (0x058134, i_058134),
    (0x058138, i_058138),
    (0x05813A, i_05813a),
    (0x058140, i_058140),
    (0x058144, i_058144),
    (0x058148, i_058148),
    (0x05814C, i_05814c),
    (0x058152, i_058152),
    (0x058158, i_058158),
    (0x05815C, i_05815c),
    (0x05815E, i_05815e),
    (0x058164, i_058164),
    (0x058166, i_058166),
    (0x058168, i_058168),
    (0x05816C, i_05816c),
    (0x058172, i_058172),
    (0x058178, i_058178),
    (0x05817C, i_05817c),
    (0x058182, i_058182),
    (0x058188, i_058188),
    (0x05818C, i_05818c),
    (0x058190, i_058190),
    (0x058194, i_058194),
    (0x058196, i_058196),
    (0x05819A, i_05819a),
    (0x05819E, i_05819e),
    (0x0581A4, i_0581a4),
    (0x0581A8, i_0581a8),
    (0x0581AE, i_0581ae),
    (0x0581B2, i_0581b2),
    (0x0581B6, i_0581b6),
    (0x0581BA, i_0581ba),
    (0x0581BE, i_0581be),
    (0x0581C2, i_0581c2),
    (0x0581C8, i_0581c8),
    (0x0581CA, i_0581ca),
    (0x0581CE, i_0581ce),
    (0x0581D0, i_0581d0),
    (0x0581D4, i_0581d4),
    (0x0581D6, i_0581d6),
    (0x0581D8, i_0581d8),
    (0x0581DA, i_0581da),
    (0x0581DE, i_0581de),
    (0x0581E2, i_0581e2),
    (0x0581E4, i_0581e4),
    (0x0581E6, i_0581e6),
    (0x0581E8, i_0581e8),
    (0x0581EE, i_0581ee),
    (0x0581F0, i_0581f0),
    (0x0581F4, i_0581f4),
    (0x0581F6, i_0581f6),
    (0x0581FA, i_0581fa),
    (0x0581FC, i_0581fc),
    (0x0581FE, i_0581fe),
    (0x058200, i_058200),
    (0x058204, i_058204),
    (0x058208, i_058208),
    (0x05820A, i_05820a),
    (0x05820C, i_05820c),
    (0x05820E, i_05820e),
    (0x058214, i_058214),
    (0x058216, i_058216),
    (0x05821A, i_05821a),
    (0x05821C, i_05821c),
    (0x058220, i_058220),
    (0x058222, i_058222),
    (0x058224, i_058224),
    (0x058226, i_058226),
    (0x05822A, i_05822a),
    (0x05822E, i_05822e),
    (0x058230, i_058230),
    (0x058234, i_058234),
    (0x058238, i_058238),
    (0x05823C, i_05823c),
    (0x058240, i_058240),
    (0x058246, i_058246),
    (0x058248, i_058248),
    (0x05824C, i_05824c),
    (0x05824E, i_05824e),
    (0x058252, i_058252),
    (0x058254, i_058254),
    (0x058256, i_058256),
    (0x058258, i_058258),
    (0x05825C, i_05825c),
    (0x058260, i_058260),
    (0x058262, i_058262),
    (0x058264, i_058264),
    (0x058266, i_058266),
    (0x05826C, i_05826c),
    (0x05826E, i_05826e),
    (0x058272, i_058272),
    (0x058274, i_058274),
    (0x058278, i_058278),
    (0x05827A, i_05827a),
    (0x05827C, i_05827c),
    (0x05827E, i_05827e),
    (0x058282, i_058282),
    (0x058286, i_058286),
    (0x058288, i_058288),
    (0x05828C, i_05828c),
    (0x058292, i_058292),
    (0x058294, i_058294),
    (0x058298, i_058298),
    (0x05829C, i_05829c),
    (0x0582A0, i_0582a0),
    (0x0582A2, i_0582a2),
    (0x0582A6, i_0582a6),
    (0x0582A8, i_0582a8),
    (0x0582AC, i_0582ac),
    (0x0582B0, i_0582b0),
    (0x0582B2, i_0582b2),
    (0x0582B8, i_0582b8),
    (0x0582BA, i_0582ba),
    (0x0582BE, i_0582be),
    (0x0582C2, i_0582c2),
    (0x0582C6, i_0582c6),
    (0x0582C8, i_0582c8),
    (0x0582CC, i_0582cc),
    (0x0582CE, i_0582ce),
    (0x0582D2, i_0582d2),
    (0x0582D6, i_0582d6),
    (0x0582D8, i_0582d8),
    (0x0582DE, i_0582de),
    (0x0582E0, i_0582e0),
    (0x0582E4, i_0582e4),
    (0x0582E8, i_0582e8),
    (0x0582EC, i_0582ec),
    (0x0582EE, i_0582ee),
    (0x0582F2, i_0582f2),
    (0x0582F4, i_0582f4),
    (0x0582F8, i_0582f8),
    (0x0582FC, i_0582fc),
    (0x0582FE, i_0582fe),
    (0x058304, i_058304),
    (0x058306, i_058306),
    (0x05830A, i_05830a),
    (0x05830E, i_05830e),
    (0x058312, i_058312),
    (0x058314, i_058314),
    (0x058318, i_058318),
    (0x05831A, i_05831a),
    (0x05831E, i_05831e),
    (0x058322, i_058322),
    (0x058324, i_058324),
    (0x058328, i_058328),
    (0x05832C, i_05832c),
    (0x058332, i_058332),
    (0x058334, i_058334),
    (0x058338, i_058338),
    (0x05833C, i_05833c),
    (0x058340, i_058340),
    (0x058342, i_058342),
    (0x058346, i_058346),
    (0x058348, i_058348),
    (0x05834C, i_05834c),
    (0x058350, i_058350),
    (0x058352, i_058352),
    (0x058356, i_058356),
    (0x05835A, i_05835a),
    (0x05835E, i_05835e),
    (0x058360, i_058360),
    (0x058364, i_058364),
    (0x058368, i_058368),
    (0x05836C, i_05836c),
    (0x05836E, i_05836e),
    (0x058374, i_058374),
    (0x058378, i_058378),
    (0x05837C, i_05837c),
    (0x05837E, i_05837e),
    (0x058384, i_058384),
    (0x058386, i_058386),
    (0x05838A, i_05838a),
    (0x05838E, i_05838e),
    (0x058392, i_058392),
    (0x058394, i_058394),
    (0x058398, i_058398),
    (0x05839A, i_05839a),
    (0x05839E, i_05839e),
    (0x0583A2, i_0583a2),
    (0x0583A4, i_0583a4),
    (0x0583A8, i_0583a8),
    (0x0583AC, i_0583ac),
    (0x0583B0, i_0583b0),
    (0x0583B2, i_0583b2),
    (0x0583B6, i_0583b6),
    (0x0583BA, i_0583ba),
    (0x0583BE, i_0583be),
    (0x0583C0, i_0583c0),
    (0x0583C6, i_0583c6),
    (0x0583CA, i_0583ca),
    (0x0583CC, i_0583cc),
    (0x0583D2, i_0583d2),
    (0x0583D6, i_0583d6),
    (0x0583DA, i_0583da),
    (0x0583E0, i_0583e0),
    (0x0583E6, i_0583e6),
    (0x0583EA, i_0583ea),
    (0x0583EC, i_0583ec),
    (0x0583F2, i_0583f2),
    (0x0583F6, i_0583f6),
    (0x0583FC, i_0583fc),
    (0x058400, i_058400),
    (0x058404, i_058404),
    (0x058406, i_058406),
    (0x05840C, i_05840c),
    (0x058410, i_058410),
    (0x058412, i_058412),
    (0x058418, i_058418),
    (0x05841E, i_05841e),
    (0x058422, i_058422),
    (0x058424, i_058424),
    (0x058428, i_058428),
    (0x05842A, i_05842a),
    (0x05842E, i_05842e),
    (0x058434, i_058434),
    (0x058438, i_058438),
    (0x05843C, i_05843c),
    (0x058440, i_058440),
    (0x058442, i_058442),
    (0x058448, i_058448),
    (0x05844E, i_05844e),
    (0x058452, i_058452),
    (0x058464, i_058464),
    (0x058468, i_058468),
    (0x05846C, i_05846c),
    (0x05846E, i_05846e),
    (0x058472, i_058472),
    (0x058476, i_058476),
    (0x058478, i_058478),
    (0x05847A, i_05847a),
    (0x05847E, i_05847e),
    (0x058484, i_058484),
    (0x05848A, i_05848a),
    (0x05848E, i_05848e),
    (0x058492, i_058492),
    (0x058494, i_058494),
    (0x058498, i_058498),
    (0x05849E, i_05849e),
    (0x0584A2, i_0584a2),
    (0x0584A6, i_0584a6),
    (0x0584AA, i_0584aa),
    (0x0584AE, i_0584ae),
    (0x0584B0, i_0584b0),
    (0x0584B4, i_0584b4),
    (0x0584B8, i_0584b8),
    (0x0584BA, i_0584ba),
    (0x0584C0, i_0584c0),
    (0x0584C4, i_0584c4),
    (0x0584C8, i_0584c8),
    (0x0584CC, i_0584cc),
    (0x0584D2, i_0584d2),
    (0x0584D8, i_0584d8),
    (0x0584DC, i_0584dc),
    (0x0584E2, i_0584e2),
    (0x0584E8, i_0584e8),
    (0x0584EE, i_0584ee),
    (0x0584F2, i_0584f2),
    (0x0584F6, i_0584f6),
    (0x0584FA, i_0584fa),
    (0x0584FC, i_0584fc),
    (0x058500, i_058500),
    (0x058504, i_058504),
    (0x058508, i_058508),
    (0x05850E, i_05850e),
    (0x058514, i_058514),
    (0x058518, i_058518),
    (0x05851E, i_05851e),
    (0x058524, i_058524),
    (0x05852A, i_05852a),
    (0x05852E, i_05852e),
    (0x058532, i_058532),
    (0x058536, i_058536),
    (0x058538, i_058538),
    (0x05853C, i_05853c),
    (0x058540, i_058540),
    (0x058546, i_058546),
    (0x05854A, i_05854a),
    (0x05854E, i_05854e),
    (0x058550, i_058550),
    (0x058554, i_058554),
    (0x05855A, i_05855a),
    (0x05855C, i_05855c),
    (0x058560, i_058560),
    (0x058564, i_058564),
    (0x058568, i_058568),
    (0x05856A, i_05856a),
    (0x05856E, i_05856e),
    (0x058570, i_058570),
    (0x058574, i_058574),
    (0x058578, i_058578),
    (0x05857A, i_05857a),
    (0x05857E, i_05857e),
    (0x058582, i_058582),
    (0x058586, i_058586),
    (0x058588, i_058588),
    (0x05858C, i_05858c),
    (0x058590, i_058590),
    (0x058594, i_058594),
    (0x058596, i_058596),
    (0x05859A, i_05859a),
    (0x05859E, i_05859e),
    (0x0585A2, i_0585a2),
    (0x0585A4, i_0585a4),
    (0x0585AA, i_0585aa),
    (0x0585AE, i_0585ae),
    (0x0585B2, i_0585b2),
    (0x0585B8, i_0585b8),
    (0x0585BE, i_0585be),
    (0x0585C2, i_0585c2),
    (0x0585C6, i_0585c6),
    (0x0585CC, i_0585cc),
    (0x0585D2, i_0585d2),
    (0x0585D6, i_0585d6),
    (0x0585D8, i_0585d8),
    (0x0585DC, i_0585dc),
    (0x0585E2, i_0585e2),
    (0x0585E6, i_0585e6),
    (0x0585EA, i_0585ea),
    (0x0585EC, i_0585ec),
    (0x0585F2, i_0585f2),
    (0x0585F6, i_0585f6),
    (0x0585FC, i_0585fc),
    (0x058602, i_058602),
    (0x058606, i_058606),
    (0x058608, i_058608),
    (0x05860C, i_05860c),
    (0x058610, i_058610),
    (0x058612, i_058612),
    (0x058616, i_058616),
    (0x05861A, i_05861a),
    (0x058620, i_058620),
    (0x058624, i_058624),
    (0x05862A, i_05862a),
    (0x05862E, i_05862e),
    (0x058632, i_058632),
    (0x058634, i_058634),
    (0x058638, i_058638),
    (0x05863C, i_05863c),
    (0x05863E, i_05863e),
    (0x058642, i_058642),
    (0x058646, i_058646),
    (0x05864A, i_05864a),
    (0x05864C, i_05864c),
    (0x058652, i_058652),
    (0x058656, i_058656),
    (0x058658, i_058658),
    (0x05865E, i_05865e),
    (0x058662, i_058662),
    (0x058668, i_058668),
    (0x05866E, i_05866e),
    (0x058672, i_058672),
    (0x058674, i_058674),
    (0x058678, i_058678),
    (0x05867C, i_05867c),
    (0x058680, i_058680),
    (0x058682, i_058682),
    (0x058686, i_058686),
    (0x058688, i_058688),
    (0x05868C, i_05868c),
    (0x058690, i_058690),
    (0x058694, i_058694),
    (0x05869A, i_05869a),
    (0x05869E, i_05869e),
    (0x0586A2, i_0586a2),
    (0x0586A6, i_0586a6),
    (0x0586AA, i_0586aa),
    (0x0586AC, i_0586ac),
    (0x0586B2, i_0586b2),
    (0x0586B6, i_0586b6),
    (0x0586B8, i_0586b8),
    (0x0586BE, i_0586be),
    (0x0586C4, i_0586c4),
    (0x0586C8, i_0586c8),
    (0x0586CC, i_0586cc),
    (0x0586CE, i_0586ce),
    (0x0586D2, i_0586d2),
    (0x0586D6, i_0586d6),
    (0x0586DC, i_0586dc),
    (0x0586E0, i_0586e0),
    (0x0586E2, i_0586e2),
    (0x0586E6, i_0586e6),
    (0x0586EC, i_0586ec),
    (0x0586EE, i_0586ee),
    (0x0586F2, i_0586f2),
    (0x0586F4, i_0586f4),
    (0x0586F8, i_0586f8),
    (0x0586FA, i_0586fa),
    (0x0586FC, i_0586fc),
    (0x0586FE, i_0586fe),
    (0x058702, i_058702),
    (0x058706, i_058706),
    (0x058708, i_058708),
    (0x05870E, i_05870e),
    (0x058712, i_058712),
    (0x058718, i_058718),
    (0x05871A, i_05871a),
    (0x05871E, i_05871e),
    (0x058722, i_058722),
    (0x058726, i_058726),
    (0x058728, i_058728),
    (0x05872C, i_05872c),
    (0x05872E, i_05872e),
    (0x058732, i_058732),
    (0x058736, i_058736),
    (0x058738, i_058738),
    (0x05873C, i_05873c),
    (0x058742, i_058742),
    (0x058744, i_058744),
    (0x058748, i_058748),
    (0x05874A, i_05874a),
    (0x05874E, i_05874e),
    (0x058750, i_058750),
    (0x058752, i_058752),
    (0x058754, i_058754),
    (0x058758, i_058758),
    (0x05875C, i_05875c),
    (0x05875E, i_05875e),
    (0x058762, i_058762),
    (0x058766, i_058766),
    (0x058768, i_058768),
    (0x05876C, i_05876c),
    (0x058770, i_058770),
    (0x058774, i_058774),
    (0x058776, i_058776),
    (0x05877A, i_05877a),
    (0x05877E, i_05877e),
    (0x058782, i_058782),
    (0x058784, i_058784),
    (0x058788, i_058788),
    (0x05878C, i_05878c),
    (0x058790, i_058790),
    (0x058792, i_058792),
    (0x058798, i_058798),
    (0x05879C, i_05879c),
    (0x05879E, i_05879e),
    (0x0587A4, i_0587a4),
    (0x0587A8, i_0587a8),
    (0x0587AC, i_0587ac),
    (0x0587B2, i_0587b2),
    (0x0587B6, i_0587b6),
    (0x0587BC, i_0587bc),
    (0x0587C2, i_0587c2),
    (0x0587C6, i_0587c6),
    (0x0587CC, i_0587cc),
    (0x0587D0, i_0587d0),
    (0x0587D2, i_0587d2),
    (0x0587D6, i_0587d6),
    (0x0587DA, i_0587da),
    (0x0587DE, i_0587de),
    (0x0587E0, i_0587e0),
    (0x0587E4, i_0587e4),
    (0x0587EA, i_0587ea),
    (0x0587F0, i_0587f0),
    (0x0587F6, i_0587f6),
    (0x0587FA, i_0587fa),
    (0x0587FE, i_0587fe),
    (0x058800, i_058800),
    (0x058806, i_058806),
    (0x05880A, i_05880a),
    (0x05880E, i_05880e),
    (0x058810, i_058810),
    (0x058816, i_058816),
    (0x05881A, i_05881a),
    (0x05881E, i_05881e),
    (0x058822, i_058822),
    (0x058826, i_058826),
    (0x05882A, i_05882a),
    (0x05882C, i_05882c),
    (0x058832, i_058832),
    (0x058836, i_058836),
    (0x05883A, i_05883a),
    (0x05883C, i_05883c),
    (0x058840, i_058840),
    (0x058842, i_058842),
    (0x058846, i_058846),
    (0x05884C, i_05884c),
    (0x05884E, i_05884e),
    (0x058852, i_058852),
    (0x058854, i_058854),
    (0x058858, i_058858),
    (0x05885A, i_05885a),
    (0x05885C, i_05885c),
    (0x05885E, i_05885e),
    (0x058862, i_058862),
    (0x058866, i_058866),
    (0x058868, i_058868),
    (0x05886E, i_05886e),
    (0x058872, i_058872),
    (0x058876, i_058876),
    (0x058878, i_058878),
    (0x05887C, i_05887c),
    (0x058880, i_058880),
    (0x058884, i_058884),
    (0x058886, i_058886),
    (0x05888A, i_05888a),
    (0x05888E, i_05888e),
    (0x058892, i_058892),
    (0x058894, i_058894),
    (0x058898, i_058898),
    (0x05889C, i_05889c),
    (0x0588A0, i_0588a0),
    (0x0588A2, i_0588a2),
    (0x0588A8, i_0588a8),
    (0x0588AC, i_0588ac),
    (0x0588AE, i_0588ae),
    (0x0588B4, i_0588b4),
    (0x0588BA, i_0588ba),
    (0x0588BE, i_0588be),
    (0x0588C2, i_0588c2),
    (0x0588C4, i_0588c4),
    (0x0588C8, i_0588c8),
    (0x0588CC, i_0588cc),
    (0x0588D2, i_0588d2),
    (0x0588D6, i_0588d6),
    (0x0588DA, i_0588da),
    (0x0588DC, i_0588dc),
    (0x0588E0, i_0588e0),
    (0x0588E4, i_0588e4),
    (0x0588E8, i_0588e8),
    (0x0588EA, i_0588ea),
    (0x0588F0, i_0588f0),
    (0x0588F4, i_0588f4),
    (0x0588F6, i_0588f6),
    (0x0588FA, i_0588fa),
    (0x0588FE, i_0588fe),
    (0x058902, i_058902),
    (0x058906, i_058906),
    (0x058908, i_058908),
    (0x05890E, i_05890e),
    (0x058910, i_058910),
    (0x058912, i_058912),
    (0x058914, i_058914),
    (0x058918, i_058918),
    (0x05891A, i_05891a),
    (0x05891E, i_05891e),
    (0x058924, i_058924),
    (0x05892A, i_05892a),
    (0x05892E, i_05892e),
    (0x058930, i_058930),
    (0x058934, i_058934),
    (0x058938, i_058938),
    (0x05893A, i_05893a),
    (0x05893E, i_05893e),
    (0x058944, i_058944),
    (0x05894A, i_05894a),
    (0x05894E, i_05894e),
    (0x058950, i_058950),
    (0x058954, i_058954),
    (0x05895A, i_05895a),
    (0x05895E, i_05895e),
    (0x058962, i_058962),
    (0x058964, i_058964),
    (0x058968, i_058968),
    (0x05896C, i_05896c),
    (0x058972, i_058972),
    (0x058976, i_058976),
    (0x05897A, i_05897a),
    (0x058980, i_058980),
    (0x058986, i_058986),
    (0x05898A, i_05898a),
    (0x058990, i_058990),
    (0x058994, i_058994),
    (0x058998, i_058998),
    (0x05899C, i_05899c),
    (0x0589A2, i_0589a2),
    (0x0589A6, i_0589a6),
    (0x0589AA, i_0589aa),
    (0x0589B0, i_0589b0),
    (0x0589B6, i_0589b6),
    (0x0589BA, i_0589ba),
    (0x0589BE, i_0589be),
    (0x0589C2, i_0589c2),
    (0x0589C6, i_0589c6),
    (0x0589C8, i_0589c8),
    (0x0589CC, i_0589cc),
    (0x0589D0, i_0589d0),
    (0x0589D6, i_0589d6),
    (0x0589DA, i_0589da),
    (0x0589DE, i_0589de),
    (0x0589E0, i_0589e0),
    (0x0589E4, i_0589e4),
    (0x0589E8, i_0589e8),
    (0x0589EC, i_0589ec),
    (0x0589EE, i_0589ee),
    (0x0589F4, i_0589f4),
    (0x0589F8, i_0589f8),
    (0x0589FC, i_0589fc),
    (0x058A00, i_058a00),
    (0x058A04, i_058a04),
    (0x058A0A, i_058a0a),
    (0x058A10, i_058a10),
    (0x058A16, i_058a16),
    (0x058A1A, i_058a1a),
    (0x058A1E, i_058a1e),
    (0x058A20, i_058a20),
    (0x058A24, i_058a24),
    (0x058A28, i_058a28),
    (0x058A2E, i_058a2e),
    (0x058A32, i_058a32),
    (0x058A36, i_058a36),
    (0x058A38, i_058a38),
    (0x058A3C, i_058a3c),
    (0x058A40, i_058a40),
    (0x058A44, i_058a44),
    (0x058A46, i_058a46),
    (0x058A4C, i_058a4c),
    (0x058A50, i_058a50),
    (0x058A56, i_058a56),
    (0x058A5C, i_058a5c),
    (0x058A60, i_058a60),
    (0x058A66, i_058a66),
    (0x058A6A, i_058a6a),
    (0x058A70, i_058a70),
    (0x058A76, i_058a76),
    (0x058A7C, i_058a7c),
    (0x058A80, i_058a80),
    (0x058A82, i_058a82),
    (0x058A88, i_058a88),
    (0x058A8C, i_058a8c),
    (0x058A90, i_058a90),
    (0x058A94, i_058a94),
    (0x058A96, i_058a96),
    (0x058A9C, i_058a9c),
    (0x058AA2, i_058aa2),
    (0x058AA6, i_058aa6),
    (0x058AA8, i_058aa8),
    (0x058AAC, i_058aac),
    (0x058AB2, i_058ab2),
    (0x058AB6, i_058ab6),
    (0x058ABC, i_058abc),
    (0x058AC0, i_058ac0),
    (0x058AC6, i_058ac6),
    (0x058ACC, i_058acc),
    (0x058AD0, i_058ad0),
    (0x058AD2, i_058ad2),
    (0x058AD6, i_058ad6),
    (0x058ADA, i_058ada),
    (0x058ADC, i_058adc),
    (0x058AE0, i_058ae0),
    (0x058AE4, i_058ae4),
    (0x058AE8, i_058ae8),
    (0x058AEC, i_058aec),
    (0x058AF2, i_058af2),
    (0x058AF6, i_058af6),
    (0x058AFA, i_058afa),
    (0x058AFE, i_058afe),
    (0x058B02, i_058b02),
    (0x058B06, i_058b06),
    (0x058B08, i_058b08),
    (0x058B0C, i_058b0c),
    (0x058B10, i_058b10),
    (0x058B12, i_058b12),
    (0x058B18, i_058b18),
    (0x058B1C, i_058b1c),
    (0x058B1E, i_058b1e),
    (0x058B24, i_058b24),
    (0x058B2A, i_058b2a),
    (0x058B2E, i_058b2e),
    (0x058B30, i_058b30),
    (0x058B34, i_058b34),
    (0x058B38, i_058b38),
    (0x058B3C, i_058b3c),
    (0x058B42, i_058b42),
    (0x058B48, i_058b48),
    (0x058B4C, i_058b4c),
    (0x058B50, i_058b50),
    (0x058B52, i_058b52),
    (0x058B58, i_058b58),
    (0x058B5C, i_058b5c),
    (0x058B5E, i_058b5e),
    (0x058B64, i_058b64),
    (0x058B6A, i_058b6a),
    (0x058B6E, i_058b6e),
    (0x058B74, i_058b74),
    (0x058B78, i_058b78),
    (0x058B7A, i_058b7a),
    (0x058B7E, i_058b7e),
    (0x058B82, i_058b82),
    (0x058B88, i_058b88),
    (0x058B8A, i_058b8a),
    (0x058B90, i_058b90),
    (0x058B94, i_058b94),
    (0x058B98, i_058b98),
    (0x058B9A, i_058b9a),
    (0x058B9E, i_058b9e),
    (0x058BA2, i_058ba2),
    (0x058BA6, i_058ba6),
    (0x058BA8, i_058ba8),
    (0x058BAA, i_058baa),
    (0x058BAE, i_058bae),
    (0x058BB2, i_058bb2),
    (0x058BB4, i_058bb4),
    (0x058BB8, i_058bb8),
    (0x058BBC, i_058bbc),
    (0x058BC0, i_058bc0),
    (0x058BC4, i_058bc4),
    (0x058BC8, i_058bc8),
    (0x058BCA, i_058bca),
    (0x058BD0, i_058bd0),
    (0x058BD4, i_058bd4),
    (0x058BD6, i_058bd6),
    (0x058BDC, i_058bdc),
    (0x058BE2, i_058be2),
    (0x058BE6, i_058be6),
    (0x058BE8, i_058be8),
    (0x058BEE, i_058bee),
    (0x058BF2, i_058bf2),
    (0x058BF8, i_058bf8),
    (0x058BFC, i_058bfc),
    (0x058C00, i_058c00),
    (0x058C02, i_058c02),
    (0x058C08, i_058c08),
    (0x058C0C, i_058c0c),
    (0x058C0E, i_058c0e),
    (0x058C12, i_058c12),
    (0x058C18, i_058c18),
    (0x058C1E, i_058c1e),
    (0x058C22, i_058c22),
    (0x058C28, i_058c28),
    (0x058C2C, i_058c2c),
    (0x058C32, i_058c32),
    (0x058C38, i_058c38),
    (0x058C3E, i_058c3e),
    (0x058C42, i_058c42),
    (0x058C44, i_058c44),
    (0x058C48, i_058c48),
    (0x058C4C, i_058c4c),
    (0x058C4E, i_058c4e),
    (0x058C50, i_058c50),
    (0x058C52, i_058c52),
    (0x058C54, i_058c54),
    (0x058C58, i_058c58),
    (0x058C5C, i_058c5c),
    (0x058C60, i_058c60),
    (0x058C66, i_058c66),
    (0x058C6A, i_058c6a),
    (0x058C6E, i_058c6e),
    (0x058C72, i_058c72),
    (0x058C76, i_058c76),
    (0x058C78, i_058c78),
    (0x058C7C, i_058c7c),
    (0x058C80, i_058c80),
    (0x058C82, i_058c82),
    (0x058C88, i_058c88),
    (0x058C8C, i_058c8c),
    (0x058C8E, i_058c8e),
    (0x058C94, i_058c94),
    (0x058C9A, i_058c9a),
    (0x058C9E, i_058c9e),
    (0x058CA2, i_058ca2),
    (0x058CA6, i_058ca6),
    (0x058CA8, i_058ca8),
    (0x058CAC, i_058cac),
    (0x058CB2, i_058cb2),
    (0x058CB8, i_058cb8),
    (0x058CBC, i_058cbc),
    (0x058CC0, i_058cc0),
    (0x058CC2, i_058cc2),
    (0x058CC8, i_058cc8),
    (0x058CCC, i_058ccc),
    (0x058CCE, i_058cce),
    (0x058CD4, i_058cd4),
    (0x058CDA, i_058cda),
    (0x058CDE, i_058cde),
    (0x058CE4, i_058ce4),
    (0x058CE8, i_058ce8),
    (0x058CEA, i_058cea),
    (0x058CEE, i_058cee),
    (0x058CF2, i_058cf2),
    (0x058CF8, i_058cf8),
    (0x058CFA, i_058cfa),
    (0x058D00, i_058d00),
    (0x058D04, i_058d04),
    (0x058D08, i_058d08),
    (0x058D0A, i_058d0a),
    (0x058D0E, i_058d0e),
    (0x058D12, i_058d12),
    (0x058D16, i_058d16),
    (0x058D18, i_058d18),
    (0x058D1A, i_058d1a),
    (0x058D1E, i_058d1e),
    (0x058D22, i_058d22),
    (0x058D24, i_058d24),
    (0x058D2A, i_058d2a),
    (0x058D2E, i_058d2e),
    (0x058D32, i_058d32),
    (0x058D36, i_058d36),
    (0x058D38, i_058d38),
    (0x058D3E, i_058d3e),
    (0x058D42, i_058d42),
    (0x058D44, i_058d44),
    (0x058D4A, i_058d4a),
    (0x058D50, i_058d50),
    (0x058D54, i_058d54),
    (0x058D56, i_058d56),
    (0x058D5C, i_058d5c),
    (0x058D62, i_058d62),
    (0x058D66, i_058d66),
    (0x058D6A, i_058d6a),
    (0x058D6C, i_058d6c),
    (0x058D72, i_058d72),
    (0x058D76, i_058d76),
    (0x058D78, i_058d78),
    (0x058D7E, i_058d7e),
    (0x058D82, i_058d82),
    (0x058D88, i_058d88),
    (0x058D8E, i_058d8e),
    (0x058D94, i_058d94),
    (0x058D98, i_058d98),
    (0x058D9E, i_058d9e),
    (0x058DA4, i_058da4),
    (0x058DAA, i_058daa),
    (0x058DAE, i_058dae),
    (0x058DB0, i_058db0),
    (0x058DB4, i_058db4),
    (0x058DB8, i_058db8),
    (0x058DBC, i_058dbc),
    (0x058DC0, i_058dc0),
    (0x058DC6, i_058dc6),
    (0x058DCA, i_058dca),
    (0x058DD0, i_058dd0),
    (0x058DD4, i_058dd4),
    (0x058DD8, i_058dd8),
    (0x058DDC, i_058ddc),
    (0x058DE0, i_058de0),
    (0x058DE2, i_058de2),
    (0x058DE6, i_058de6),
    (0x058DEA, i_058dea),
    (0x058DEC, i_058dec),
    (0x058DF2, i_058df2),
    (0x058DF6, i_058df6),
    (0x058E08, i_058e08),
    (0x058E0C, i_058e0c),
    (0x058E10, i_058e10),
    (0x058E14, i_058e14),
    (0x058E1A, i_058e1a),
    (0x058E1E, i_058e1e),
    (0x058E22, i_058e22),
    (0x058E26, i_058e26),
    (0x058E2C, i_058e2c),
    (0x058E30, i_058e30),
    (0x058E32, i_058e32),
    (0x058E36, i_058e36),
    (0x058E3A, i_058e3a),
    (0x058E40, i_058e40),
    (0x058E44, i_058e44),
    (0x058E48, i_058e48),
    (0x058E4A, i_058e4a),
    (0x058E50, i_058e50),
    (0x058E54, i_058e54),
    (0x058E5A, i_058e5a),
    (0x058E5E, i_058e5e),
    (0x058E62, i_058e62),
    (0x058E66, i_058e66),
    (0x058E6A, i_058e6a),
    (0x058E6C, i_058e6c),
    (0x058E70, i_058e70),
    (0x058E76, i_058e76),
    (0x058E78, i_058e78),
    (0x058E7C, i_058e7c),
    (0x058E7E, i_058e7e),
    (0x058E82, i_058e82),
    (0x058E84, i_058e84),
    (0x058E86, i_058e86),
    (0x058E88, i_058e88),
    (0x058E8C, i_058e8c),
    (0x058E92, i_058e92),
    (0x058E96, i_058e96),
    (0x058E9A, i_058e9a),
    (0x058E9C, i_058e9c),
    (0x058EA0, i_058ea0),
    (0x058EA4, i_058ea4),
    (0x058EA6, i_058ea6),
    (0x058EAC, i_058eac),
    (0x058EB0, i_058eb0),
    (0x058EB4, i_058eb4),
    (0x058EB6, i_058eb6),
    (0x058EBC, i_058ebc),
    (0x058EC0, i_058ec0),
    (0x058EC4, i_058ec4),
    (0x058EC6, i_058ec6),
    (0x058ECC, i_058ecc),
    (0x058ED0, i_058ed0),
    (0x058ED2, i_058ed2),
    (0x058ED8, i_058ed8),
    (0x058EDE, i_058ede),
    (0x058EE2, i_058ee2),
    (0x058EE4, i_058ee4),
    (0x058EEA, i_058eea),
    (0x058EEE, i_058eee),
    (0x058EF4, i_058ef4),
    (0x058EF8, i_058ef8),
    (0x058EFE, i_058efe),
    (0x058F02, i_058f02),
    (0x058F06, i_058f06),
    (0x058F08, i_058f08),
    (0x058F0C, i_058f0c),
    (0x058F10, i_058f10),
    (0x058F12, i_058f12),
    (0x058F18, i_058f18),
    (0x058F1C, i_058f1c),
    (0x058F82, i_058f82),
    (0x058F88, i_058f88),
    (0x058F8E, i_058f8e),
    (0x058F94, i_058f94),
    (0x058F9A, i_058f9a),
    (0x058F9E, i_058f9e),
    (0x058FC2, i_058fc2),
    (0x058FC8, i_058fc8),
    (0x058FCE, i_058fce),
    (0x058FD4, i_058fd4),
    (0x058FDA, i_058fda),
    (0x058FDE, i_058fde),
    (0x058FE2, i_058fe2),
    (0x058FE8, i_058fe8),
    (0x058FEE, i_058fee),
    (0x058FF4, i_058ff4),
    (0x058FFA, i_058ffa),
    (0x058FFE, i_058ffe),
    (0x059042, i_059042),
    (0x059048, i_059048),
    (0x05904E, i_05904e),
    (0x059054, i_059054),
    (0x05905A, i_05905a),
    (0x05905E, i_05905e),
    (0x059062, i_059062),
    (0x059066, i_059066),
    (0x05906C, i_05906c),
    (0x059072, i_059072),
    (0x059076, i_059076),
    (0x05907C, i_05907c),
    (0x059080, i_059080),
    (0x059086, i_059086),
    (0x05908C, i_05908c),
    (0x059090, i_059090),
    (0x059096, i_059096),
    (0x05909C, i_05909c),
    (0x0590A0, i_0590a0),
    (0x0590A6, i_0590a6),
    (0x0590AC, i_0590ac),
    (0x0590B2, i_0590b2),
    (0x0590B8, i_0590b8),
    (0x0590BC, i_0590bc),
    (0x0590C0, i_0590c0),
    (0x0590C4, i_0590c4),
    (0x0590C8, i_0590c8),
    (0x0590CA, i_0590ca),
    (0x0590CE, i_0590ce),
    (0x0590D2, i_0590d2),
    (0x0590D8, i_0590d8),
    (0x0590DE, i_0590de),
    (0x0590E2, i_0590e2),
    (0x0590E8, i_0590e8),
    (0x0590EA, i_0590ea),
    (0x0590EE, i_0590ee),
    (0x0590F2, i_0590f2),
    (0x0590F6, i_0590f6),
    (0x0590F8, i_0590f8),
    (0x0590FC, i_0590fc),
    (0x0590FE, i_0590fe),
    (0x059102, i_059102),
    (0x059106, i_059106),
    (0x059108, i_059108),
    (0x05910C, i_05910c),
    (0x059112, i_059112),
    (0x059116, i_059116),
    (0x05911A, i_05911a),
    (0x05911C, i_05911c),
    (0x059122, i_059122),
    (0x059126, i_059126),
    (0x05912A, i_05912a),
    (0x05912C, i_05912c),
    (0x059130, i_059130),
    (0x059134, i_059134),
    (0x059138, i_059138),
    (0x05913A, i_05913a),
    (0x059140, i_059140),
    (0x059144, i_059144),
    (0x059146, i_059146),
    (0x05914C, i_05914c),
    (0x059150, i_059150),
    (0x059156, i_059156),
    (0x05915C, i_05915c),
    (0x059162, i_059162),
    (0x059166, i_059166),
    (0x059168, i_059168),
    (0x05916C, i_05916c),
    (0x059170, i_059170),
    (0x059176, i_059176),
    (0x05917C, i_05917c),
    (0x059180, i_059180),
    (0x059186, i_059186),
    (0x05918A, i_05918a),
    (0x05918E, i_05918e),
    (0x059190, i_059190),
    (0x059196, i_059196),
    (0x05919A, i_05919a),
    (0x05919E, i_05919e),
    (0x0591A0, i_0591a0),
    (0x0591A4, i_0591a4),
    (0x0591A8, i_0591a8),
    (0x0591AC, i_0591ac),
    (0x0591AE, i_0591ae),
    (0x0591B2, i_0591b2),
    (0x0591B6, i_0591b6),
    (0x0591BA, i_0591ba),
    (0x0591BC, i_0591bc),
    (0x0591C2, i_0591c2),
    (0x0591C6, i_0591c6),
    (0x0591C8, i_0591c8),
    (0x0591CE, i_0591ce),
    (0x0591D4, i_0591d4),
];
