// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $057752: cmp.w d1, d0
pub fn i_057752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57754;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $057754: bcc.w $5775c
pub fn i_057754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57758;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5775C; return 10; }
    8
}
/// $057758: move.b (a7), d0
pub fn i_057758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5775A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05775A: cmp.w d1, d0
pub fn i_05775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5775C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05775C: addq.w #$2, a7
pub fn i_05775c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5775E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05775E: bcc.w $57768
pub fn i_05775e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57762;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57768; return 10; }
    8
}
/// $057762: lea.l $578dc(pc), a1
pub fn i_057762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x578DC;
    16
}
/// $057766: move.l a1, (a6)
pub fn i_057766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057768: btst.b #$0, $72(a6)
pub fn i_057768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5776E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05776E: bne.w $57778
pub fn i_05776e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57772;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57778; return 10; }
    8
}
/// $057772: lea.l $585f6(pc), a1
pub fn i_057772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57776;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $057776: move.l a1, (a6)
pub fn i_057776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57778;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057778: tst.b $78(a6)
pub fn i_057778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5777C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05777C: bne.w $57786
pub fn i_05777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57780;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57786; return 10; }
    8
}
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
// ===== VRAM_FixLayerAutoclear_05A824  ($05A824, 150 B, video/fix_layer_autoclear_05a824.s)
/// $05A824: moveq #$0, d2
pub fn i_05a824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A826;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A826: move.w #$1, $3c0004.l
pub fn i_05a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A82E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0004, v1 as u16);
    16
}
/// $05A82E: move.w #$8200, $3c0000.l
pub fn i_05a82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A836;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x8200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05A836: movea.l #$3c0002, a0
pub fn i_05a836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A83C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x3C0002;
    cpu.a[0] = v1;
    4
}
/// $05A83C: move.w #$17c, d0
pub fn i_05a83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A840;
    cpu.ir = 0x303C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A840: move.w d2, (a0)
pub fn i_05a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A842;
    cpu.ir = 0x3082;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $05A842: move.b d0, d0
pub fn i_05a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A844;
    cpu.ir = 0x1000;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05A844: dbra d0, $5a840
pub fn i_05a844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A848;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5A848; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5A848; return 14; }
    cpu.pc = 0x5A840;
    10
}
/// $05A848: move.w #$0, d1
pub fn i_05a848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A84C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A84C: moveq #$0, d2
pub fn i_05a84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A84E;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A84E: move.w #$1, $3c0004.l
pub fn i_05a84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A856;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0004, v1 as u16);
    16
}
/// $05A856: move.w #$0, $3c0000.l
pub fn i_05a856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A85E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05A85E: movea.l #$3c0002, a0
pub fn i_05a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A864;
    cpu.ir = 0x207C;
    let v1: u32 = 0x3C0002;
    cpu.a[0] = v1;
    4
}
/// $05A864: move.w #$2f9f, d0
pub fn i_05a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A868;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2F9F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A868: nop
pub fn i_05a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A86A;
    cpu.ir = 0x4E71;
    4
}
/// $05A86A: nop
pub fn i_05a86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A86C;
    cpu.ir = 0x4E71;
    4
}
/// $05A86C: move.w d1, (a0)
pub fn i_05a86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A86E;
    cpu.ir = 0x3081;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $05A86E: nop
pub fn i_05a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A870;
    cpu.ir = 0x4E71;
    4
}
/// $05A870: nop
pub fn i_05a870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A872;
    cpu.ir = 0x4E71;
    4
}
/// $05A872: move.w d2, (a0)
pub fn i_05a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A874;
    cpu.ir = 0x3082;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $05A874: dbra d0, $5a868
pub fn i_05a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A878;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5A878; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5A878; return 14; }
    cpu.pc = 0x5A868;
    10
}
/// $05A878: clr.w $10e1fa.l
pub fn i_05a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A87E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1FA) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1FA, r2 as u16);
    16
}
/// $05A87E: clr.w $10e1f6.l
pub fn i_05a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A884;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1F6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1F6, r2 as u16);
    16
}
/// $05A884: clr.w $10e1f8.l
pub fn i_05a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A88A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1F8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1F8, r2 as u16);
    16
}
/// $05A88A: lea.l $108080.l, a5
pub fn i_05a88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A890;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A890: clr.b $616c(a5)
pub fn i_05a890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A894;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x616C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A894: clr.w $6168(a5)
pub fn i_05a894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A898;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x6168);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A898: clr.w $616a(a5)
pub fn i_05a898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A89C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x616A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A89C: lea.l $614c(a5), a0
pub fn i_05a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8A0;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05A8A0: clr.b $8(a0)
pub fn i_05a8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8A4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A8A4: move.w #$ffff, $4(a0)
pub fn i_05a8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8AA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A8AA: lea.l $6158(a5), a0
pub fn i_05a8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8AE;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05A8AE: clr.b $8(a0)
pub fn i_05a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8B2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A8B2: move.w #$ffff, $4(a0)
pub fn i_05a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8B8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A8B8: rts
pub fn i_05a8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VRAM_FixAutoclear_Reset_05a9ba  ($05A9BA, 28 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A9BA: lea.l $108080.l, a5
pub fn i_05a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9C0;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A9C0: clr.b $10e1ec.l
pub fn i_05a9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9C6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E1EC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E1EC, r2 as u8);
    16
}
/// $05A9C6: clr.w $4254(a5)
pub fn i_05a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9CA;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x4254);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A9CA: clr.w $6148(a5)
pub fn i_05a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9CE;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x6148);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A9CE: move.w #$348, $614a(a5)
pub fn i_05a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9D4;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x348;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A9D4: rts
pub fn i_05a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_Dispatch_05A9D6  ($05A9D6, 192 B, video/sprite_dispatch_dual_05a9d6.s)
/// $05A9D6: andi.b #$3, d5
pub fn i_05a9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9DA;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9DA: ori.b #$4, d5
pub fn i_05a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9DE;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9DE: bra.w $5a9e6
pub fn i_05a9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A9E6;
    10
}
/// $05A9E2: andi.b #$3, d5
pub fn i_05a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9E6;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9E6: jsr $5b1b2(pc)
pub fn i_05a9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5A9EA);
    cpu.pc = 0x5B1B2;
    16
}
/// $05A9EA: move a1, usp
pub fn i_05a9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9EC;
    cpu.ir = 0x4E61;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.usp = cpu.a[1];
    4
}
/// $05A9EC: lea.l $108080.l, a5
pub fn i_05a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A9F2: move.l a0, d7
pub fn i_05a9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F4;
    cpu.ir = 0x2E08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A9F4: bmi.w $5aa92
pub fn i_05a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05A9F8: swap d1
pub fn i_05a9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9FA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A9FA: move.w $4254(a5), d1
pub fn i_05a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9FE;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A9FE: cmpi.w #$11d0, d1
pub fn i_05a9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA02;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x11D0, Size::Word, false);
    8
}
/// $05AA02: bcc.w $5aa92
pub fn i_05aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05AA06: move.w $6148(a5), d6
pub fn i_05aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA0A;
    cpu.ir = 0x3C2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA0A: move.w $614a(a5), d7
pub fn i_05aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA0E;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA0E: cmp.w d7, d6
pub fn i_05aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA10;
    cpu.ir = 0xBC47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05AA10: bcc.w $5aa92
pub fn i_05aa10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05AA14: btst.b #$2, d5
pub fn i_05aa14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA18;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05AA18: bne.w $5aa2e
pub fn i_05aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA1C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AA2E; return 10; }
    8
}
/// $05AA1C: lea.l $5428(a5), a4
pub fn i_05aa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA20;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[4] = ad1;
    16
}
/// $05AA20: move.l d1, (a4, d6.w)
pub fn i_05aa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA24;
    cpu.ir = 0x2981;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05AA24: addq.w #$4, d6
pub fn i_05aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA26;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AA26: move.w d6, $6148(a5)
pub fn i_05aa26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA2A;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA2A: bra.w $5aa3c
pub fn i_05aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA3C;
    10
}
/// $05AA2E: subq.w #$4, d7
pub fn i_05aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA30;
    cpu.ir = 0x5947;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AA30: lea.l $5428(a5), a4
pub fn i_05aa30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA34;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[4] = ad1;
    16
}
/// $05AA34: move.l d1, (a4, d7.w)
pub fn i_05aa34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA38;
    cpu.ir = 0x2981;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05AA38: move.w d7, $614a(a5)
pub fn i_05aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA3C;
    cpu.ir = 0x3B47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA3C: move.w d5, -(a7)
pub fn i_05aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA3E;
    cpu.ir = 0x3F05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05AA3E: lsl.w #$8, d2
pub fn i_05aa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA40;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AA40: lea.l $4258(a5), a3
pub fn i_05aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA44;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05AA44: lea.l $11d0(a3), a2
pub fn i_05aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA48;
    cpu.ir = 0x45EB;
    let ad1 = cpu.a[3].wrapping_add(0x11D0);
    cpu.a[2] = ad1;
    16
}
/// $05AA48: adda.w d1, a3
pub fn i_05aa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4A;
    cpu.ir = 0xD6C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AA4A: move.b d3, d7
pub fn i_05aa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4C;
    cpu.ir = 0x1E03;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AA4C: and.b d4, d7
pub fn i_05aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4E;
    cpu.ir = 0xCE04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 & cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AA4E: addq.b #$1, d7
pub fn i_05aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA50;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AA50: sne.b d7
pub fn i_05aa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA52;
    cpu.ir = 0x56C7;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[7] & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AA52: andi.w #$4, d7
pub fn i_05aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA56;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x4) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AA56: andi.b #$3, d5
pub fn i_05aa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AA5A: or.b d5, d7
pub fn i_05aa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5C;
    cpu.ir = 0x8E05;
    let v1 = cpu.d[5] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AA5C: add.w d7, d7
pub fn i_05aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5E;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AA5E: add.w d7, d7
pub fn i_05aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA60;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AA60: jsr $5aa96(pc, d7.w)
pub fn i_05aa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA64;
    cpu.ir = 0x4EBB;
    let ad1 = 0x5AA96_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.push32(bus, 0x5AA64);
    cpu.jump_to(ad1);
    16
}
/// $05AA64: move.w (a7)+, d5
pub fn i_05aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA66;
    cpu.ir = 0x3A1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05AA66: lea.l $4258(a5), a2
pub fn i_05aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA6A;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AA6A: suba.l a2, a3
pub fn i_05aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA6C;
    cpu.ir = 0x97CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_sub(v1);
    8
}
/// $05AA6C: move.w $4254(a5), d0
pub fn i_05aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA70;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA70: cmp.w a3, d0
pub fn i_05aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA72;
    cpu.ir = 0xB04B;
    let v1 = cpu.a[3] as i16 as i32 as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05AA72: beq.w $5aa7e
pub fn i_05aa72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AA7E; return 10; }
    8
}
/// $05AA76: move.w a3, $4254(a5)
pub fn i_05aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA7A;
    cpu.ir = 0x3B4B;
    let v1 = cpu.a[3] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA7A: bra.w $5aa92
pub fn i_05aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA92;
    10
}
/// $05AA7E: btst.b #$2, d5
pub fn i_05aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA82;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05AA82: bne.w $5aa8e
pub fn i_05aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AA8E; return 10; }
    8
}
/// $05AA86: subq.w #$4, $6148(a5)
pub fn i_05aa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA8A;
    cpu.ir = 0x596D;
    let ad1 = cpu.a[5].wrapping_add(0x6148);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05AA8A: bra.w $5aa92
pub fn i_05aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA92;
    10
}
/// $05AA8E: addq.w #$4, $614a(a5)
pub fn i_05aa8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA92;
    cpu.ir = 0x586D;
    let ad1 = cpu.a[5].wrapping_add(0x614A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05AA92: move usp, a1
pub fn i_05aa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA94;
    cpu.ir = 0x4E69;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[1] = cpu.usp;
    4
}
/// $05AA94: rts
pub fn i_05aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteDispatchJT_05AA96  ($05AA96, 1820 B, video/sprite_queue_render_05aa96.s)
/// $05AA96: bra.w $5aab6
pub fn i_05aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AAB6;
    10
}
/// $05AA9A: bra.w $5ab34
pub fn i_05aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AB34;
    10
}
/// $05AA9E: bra.w $5abb8
pub fn i_05aa9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5ABB8;
    10
}
/// $05AAA2: bra.w $5ac44
pub fn i_05aaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AC44;
    10
}
/// $05AAA6: bra.w $5acd6
pub fn i_05aaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5ACD6;
    10
}
/// $05AAAA: bra.w $5ae00
pub fn i_05aaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AE00;
    10
}
/// $05AAAE: bra.w $5af30
pub fn i_05aaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AF30;
    10
}
/// $05AAB2: bra.w $5b06e
pub fn i_05aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B06E;
    10
}
/// $05AAB6: cmpa.l a2, a3
pub fn i_05aab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AAB8: bcc.w $5ab32
pub fn i_05aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AABC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB32; return 10; }
    8
}
/// $05AABC: move.b (a0)+, -(a7)
pub fn i_05aabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AABE;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AABE: move.b (a0)+, d2
pub fn i_05aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC0;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AAC0: eor.b d5, d2
pub fn i_05aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC2;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AAC2: move.w #$fff, d4
pub fn i_05aac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC6;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAC6: moveq #$0, d5
pub fn i_05aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC8;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAC8: move.l (a0)+, d1
pub fn i_05aac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACA;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AACA: moveq #$0, d5
pub fn i_05aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACC;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AACC: move.b (a0)+, d5
pub fn i_05aacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACE;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AACE: swap d5
pub fn i_05aace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD0;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAD0: move.b (a0)+, d5
pub fn i_05aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD2;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AAD2: move.w d5, d6
pub fn i_05aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD4;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAD4: swap d6
pub fn i_05aad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD6;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAD6: move.w d5, d6
pub fn i_05aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD8;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAD8: add.w d0, d1
pub fn i_05aad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AADA: swap d1
pub fn i_05aada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADC;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AADC: swap d0
pub fn i_05aadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AADE: add.w d0, d1
pub fn i_05aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE0;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AAE0: swap d0
pub fn i_05aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAE2: andi.w #$1ff, d1
pub fn i_05aae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AAE6: lsl.l #$7, d1
pub fn i_05aae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE8;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AAE8: or.l d6, d1
pub fn i_05aae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEA;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAEA: clr.w d5
pub fn i_05aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEC;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AAEC: swap d5
pub fn i_05aaec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEE;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAEE: subq.b #$1, d5
pub fn i_05aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF0;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AAF0: add.w d6, d6
pub fn i_05aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF2;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AAF2: move.w #$800, d3
pub fn i_05aaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF6;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAF6: cmpi.w #$f800, d1
pub fn i_05aaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAFA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AAFA: bcc.w $5ab06
pub fn i_05aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB06; return 10; }
    8
}
/// $05AAFE: cmpi.w #$a000, d1
pub fn i_05aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB02;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AB02: bcc.w $5ab14
pub fn i_05ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB14; return 10; }
    8
}
/// $05AB06: move.l a0, (a3)+
pub fn i_05ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB08;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB08: move.w d2, (a3)+
pub fn i_05ab08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0A;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB0A: move.w d4, (a3)+
pub fn i_05ab0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0C;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB0C: move.l d1, (a3)+
pub fn i_05ab0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0E;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB0E: cmpa.l a2, a3
pub fn i_05ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB10;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB10: bcc.w $5ab20
pub fn i_05ab10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB20; return 10; }
    8
}
/// $05AB14: add.w d3, d1
pub fn i_05ab14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB16;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB16: adda.w d6, a0
pub fn i_05ab16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB18;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AB18: dbra d5, $5aaf6
pub fn i_05ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB1C;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AB1C; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AB1C; return 14; }
    cpu.pc = 0x5AAF6;
    10
}
/// $05AB1C: subq.b #$1, (a7)
pub fn i_05ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB1E;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AB1E: bne.b $5aac6
pub fn i_05ab1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB20;
    cpu.ir = 0x66A6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AAC6; return 10; }
    8
}
/// $05AB20: addq.w #$2, a7
pub fn i_05ab20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB22;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AB22: lea.l $4258(a5), a2
pub fn i_05ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB26;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AB26: cmpa.l a2, a3
pub fn i_05ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB28;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB28: beq.w $5ab32
pub fn i_05ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB2C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AB32; return 10; }
    8
}
/// $05AB2C: ori.b #$80, -$6(a3)
pub fn i_05ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB32;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AB32: rts
pub fn i_05ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AB34: cmpa.l a2, a3
pub fn i_05ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB36;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB36: bcc.w $5abb6
pub fn i_05ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ABB6; return 10; }
    8
}
/// $05AB3A: move.b (a0)+, -(a7)
pub fn i_05ab3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3C;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AB3C: move.b (a0)+, d2
pub fn i_05ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3E;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB3E: eor.b d5, d2
pub fn i_05ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB40;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AB40: move.w #$fff, d4
pub fn i_05ab40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB44;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB44: moveq #$0, d5
pub fn i_05ab44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB46;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB46: move.l (a0)+, d1
pub fn i_05ab46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB48;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB48: moveq #$0, d5
pub fn i_05ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB4A: move.b (a0)+, d5
pub fn i_05ab4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4C;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB4C: swap d5
pub fn i_05ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB4E: move.b (a0)+, d5
pub fn i_05ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB50;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB50: swap d1
pub fn i_05ab50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB52;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB52: not.w d1
pub fn i_05ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB54;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AB54: swap d1
pub fn i_05ab54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB56;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB56: move.w d5, d6
pub fn i_05ab56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB58;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB58: swap d6
pub fn i_05ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5A;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB5A: move.w d5, d6
pub fn i_05ab5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5C;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB5C: add.w d0, d1
pub fn i_05ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5E;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB5E: swap d1
pub fn i_05ab5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB60;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB60: swap d0
pub fn i_05ab60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB62;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB62: add.w d0, d1
pub fn i_05ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB64;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB64: swap d0
pub fn i_05ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB66;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB66: andi.w #$1ff, d1
pub fn i_05ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AB6A: lsl.l #$7, d1
pub fn i_05ab6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6C;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AB6C: or.l d6, d1
pub fn i_05ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6E;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB6E: clr.w d5
pub fn i_05ab6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB70;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AB70: swap d5
pub fn i_05ab70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB72;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB72: subq.b #$1, d5
pub fn i_05ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB74;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AB74: add.w d6, d6
pub fn i_05ab74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB76;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB76: move.w #$800, d3
pub fn i_05ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB7A;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB7A: sub.w d3, d1
pub fn i_05ab7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB7C;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB7C: cmpi.w #$f800, d1
pub fn i_05ab7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB80;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AB80: bcc.w $5ab8c
pub fn i_05ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB8C; return 10; }
    8
}
/// $05AB84: cmpi.w #$a000, d1
pub fn i_05ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB88;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AB88: bcc.w $5ab9a
pub fn i_05ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB9A; return 10; }
    8
}
/// $05AB8C: move.l a0, (a3)+
pub fn i_05ab8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB8E;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB8E: move.w d2, (a3)+
pub fn i_05ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB90;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB90: move.w d4, (a3)+
pub fn i_05ab90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB92;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB92: move.l d1, (a3)+
pub fn i_05ab92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB94;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB94: cmpa.l a2, a3
pub fn i_05ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB96;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB96: bcc.w $5aba4
pub fn i_05ab96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ABA4; return 10; }
    8
}
/// $05AB9A: adda.w d6, a0
pub fn i_05ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB9C;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AB9C: dbra d5, $5ab7a
pub fn i_05ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA0;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ABA0; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ABA0; return 14; }
    cpu.pc = 0x5AB7A;
    10
}
/// $05ABA0: subq.b #$1, (a7)
pub fn i_05aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA2;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ABA2: bne.b $5ab44
pub fn i_05aba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA4;
    cpu.ir = 0x66A0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AB44; return 10; }
    8
}
/// $05ABA4: addq.w #$2, a7
pub fn i_05aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA6;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ABA6: lea.l $4258(a5), a2
pub fn i_05aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABAA;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ABAA: cmpa.l a2, a3
pub fn i_05abaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABAC;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ABAC: beq.w $5abb6
pub fn i_05abac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ABB6; return 10; }
    8
}
/// $05ABB0: ori.b #$80, -$6(a3)
pub fn i_05abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB6;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ABB6: rts
pub fn i_05abb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05ABB8: cmpa.l a2, a3
pub fn i_05abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABBA;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ABBA: bcc.w $5ac42
pub fn i_05abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC42; return 10; }
    8
}
/// $05ABBE: move.b (a0)+, -(a7)
pub fn i_05abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC0;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05ABC0: move.b (a0)+, d2
pub fn i_05abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC2;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABC2: eor.b d5, d2
pub fn i_05abc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC4;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ABC4: move.w #$1fff, d4
pub fn i_05abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABC8: moveq #$0, d5
pub fn i_05abc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCA;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABCA: move.l (a0)+, d1
pub fn i_05abca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCC;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ABCC: moveq #$0, d5
pub fn i_05abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCE;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABCE: move.b (a0)+, d5
pub fn i_05abce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD0;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABD0: swap d5
pub fn i_05abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD2;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABD2: move.b (a0)+, d5
pub fn i_05abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD4;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABD4: neg.w d1
pub fn i_05abd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD6;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABD6: move.w d5, d6
pub fn i_05abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD8;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABD8: lsl.w #$4, d6
pub fn i_05abd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDA;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05ABDA: subq.w #$1, d6
pub fn i_05abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDC;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABDC: add.w d6, d1
pub fn i_05abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDE;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABDE: move.w d5, d6
pub fn i_05abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE0;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABE0: swap d6
pub fn i_05abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE2;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABE2: move.w d5, d6
pub fn i_05abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE4;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABE4: add.w d0, d1
pub fn i_05abe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE6;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABE6: swap d1
pub fn i_05abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE8;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABE8: swap d0
pub fn i_05abe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABEA: add.w d0, d1
pub fn i_05abea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEC;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABEC: swap d0
pub fn i_05abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABEE: andi.w #$1ff, d1
pub fn i_05abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ABF2: lsl.l #$7, d1
pub fn i_05abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF4;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05ABF4: or.l d6, d1
pub fn i_05abf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF6;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABF6: clr.w d5
pub fn i_05abf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF8;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABF8: swap d5
pub fn i_05abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABFA: subq.b #$1, d5
pub fn i_05abfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFC;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ABFC: add.w d6, d6
pub fn i_05abfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFE;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABFE: adda.w d6, a0
pub fn i_05abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC00;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC00: move.w #$800, d3
pub fn i_05ac00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC04;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC04: cmpi.w #$f800, d1
pub fn i_05ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC08;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AC08: bcc.w $5ac14
pub fn i_05ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC14; return 10; }
    8
}
/// $05AC0C: cmpi.w #$a000, d1
pub fn i_05ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC10;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AC10: bcc.w $5ac22
pub fn i_05ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC22; return 10; }
    8
}
/// $05AC14: move.l a0, (a3)+
pub fn i_05ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC16;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC16: move.w d2, (a3)+
pub fn i_05ac16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC18;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AC18: move.w d4, (a3)+
pub fn i_05ac18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1A;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AC1A: move.l d1, (a3)+
pub fn i_05ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1C;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC1C: cmpa.l a2, a3
pub fn i_05ac1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1E;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC1E: bcc.w $5ac30
pub fn i_05ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC30; return 10; }
    8
}
/// $05AC22: add.w d3, d1
pub fn i_05ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC24;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC24: adda.w d6, a0
pub fn i_05ac24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC26;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC26: dbra d5, $5ac04
pub fn i_05ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2A;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AC2A; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AC2A; return 14; }
    cpu.pc = 0x5AC04;
    10
}
/// $05AC2A: suba.w d6, a0
pub fn i_05ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2C;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC2C: subq.b #$1, (a7)
pub fn i_05ac2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2E;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AC2E: bne.b $5abc8
pub fn i_05ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC30;
    cpu.ir = 0x6698;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5ABC8; return 10; }
    8
}
/// $05AC30: addq.w #$2, a7
pub fn i_05ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC32;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AC32: lea.l $4258(a5), a2
pub fn i_05ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC36;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AC36: cmpa.l a2, a3
pub fn i_05ac36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC38;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC38: beq.w $5ac42
pub fn i_05ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AC42; return 10; }
    8
}
/// $05AC3C: ori.b #$80, -$6(a3)
pub fn i_05ac3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC42;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AC42: rts
pub fn i_05ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AC44: cmpa.l a2, a3
pub fn i_05ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC46;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC46: bcc.w $5acd4
pub fn i_05ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACD4; return 10; }
    8
}
/// $05AC4A: move.b (a0)+, -(a7)
pub fn i_05ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4C;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AC4C: move.b (a0)+, d2
pub fn i_05ac4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4E;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC4E: eor.b d5, d2
pub fn i_05ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC50;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AC50: move.w #$1fff, d4
pub fn i_05ac50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC54;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC54: moveq #$0, d5
pub fn i_05ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC56;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC56: move.l (a0)+, d1
pub fn i_05ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC58;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC58: moveq #$0, d5
pub fn i_05ac58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC5A: move.b (a0)+, d5
pub fn i_05ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5C;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC5C: swap d5
pub fn i_05ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC5E: move.b (a0)+, d5
pub fn i_05ac5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC60;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC60: swap d1
pub fn i_05ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC62;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC62: not.w d1
pub fn i_05ac62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC64;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC64: swap d1
pub fn i_05ac64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC66;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC66: neg.w d1
pub fn i_05ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC68;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC68: move.w d5, d6
pub fn i_05ac68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6A;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC6A: lsl.w #$4, d6
pub fn i_05ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6C;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AC6C: subq.w #$1, d6
pub fn i_05ac6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6E;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC6E: add.w d6, d1
pub fn i_05ac6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC70;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC70: move.w d5, d6
pub fn i_05ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC72;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC72: swap d6
pub fn i_05ac72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC74;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC74: move.w d5, d6
pub fn i_05ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC76;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC76: add.w d0, d1
pub fn i_05ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC78;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC78: swap d1
pub fn i_05ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7A;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC7A: swap d0
pub fn i_05ac7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC7C: add.w d0, d1
pub fn i_05ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7E;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC7E: swap d0
pub fn i_05ac7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC80;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC80: andi.w #$1ff, d1
pub fn i_05ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC84;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AC84: lsl.l #$7, d1
pub fn i_05ac84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC86;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AC86: or.l d6, d1
pub fn i_05ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC88;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC88: clr.w d5
pub fn i_05ac88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8A;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC8A: swap d5
pub fn i_05ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8C;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC8C: subq.b #$1, d5
pub fn i_05ac8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8E;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AC8E: add.w d6, d6
pub fn i_05ac8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC90;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC90: adda.w d6, a0
pub fn i_05ac90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC92;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC92: move.w #$800, d3
pub fn i_05ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC96;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC96: sub.w d3, d1
pub fn i_05ac96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC98;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC98: cmpi.w #$f800, d1
pub fn i_05ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC9C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AC9C: bcc.w $5aca8
pub fn i_05ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACA8; return 10; }
    8
}
/// $05ACA0: cmpi.w #$a000, d1
pub fn i_05aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05ACA4: bcc.w $5acb6
pub fn i_05aca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACB6; return 10; }
    8
}
/// $05ACA8: move.l a0, (a3)+
pub fn i_05aca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAA;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ACAA: move.w d2, (a3)+
pub fn i_05acaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAC;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ACAC: move.w d4, (a3)+
pub fn i_05acac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAE;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ACAE: move.l d1, (a3)+
pub fn i_05acae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB0;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ACB0: cmpa.l a2, a3
pub fn i_05acb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB2;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACB2: bcc.w $5acc2
pub fn i_05acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACC2; return 10; }
    8
}
/// $05ACB6: adda.w d6, a0
pub fn i_05acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB8;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ACB8: dbra d5, $5ac96
pub fn i_05acb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACBC;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ACBC; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ACBC; return 14; }
    cpu.pc = 0x5AC96;
    10
}
/// $05ACBC: suba.w d6, a0
pub fn i_05acbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACBE;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ACBE: subq.b #$1, (a7)
pub fn i_05acbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC0;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ACC0: bne.b $5ac54
pub fn i_05acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC2;
    cpu.ir = 0x6692;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AC54; return 10; }
    8
}
/// $05ACC2: addq.w #$2, a7
pub fn i_05acc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC4;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ACC4: lea.l $4258(a5), a2
pub fn i_05acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC8;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ACC8: cmpa.l a2, a3
pub fn i_05acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACCA;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACCA: beq.w $5acd4
pub fn i_05acca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ACD4; return 10; }
    8
}
/// $05ACCE: ori.b #$80, -$6(a3)
pub fn i_05acce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD4;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ACD4: rts
pub fn i_05acd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05ACD6: cmpa.l a2, a3
pub fn i_05acd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACD8: bcc.w $5adfe
pub fn i_05acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADFE; return 10; }
    8
}
/// $05ACDC: move.b (a0)+, -(a7)
pub fn i_05acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACDE;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05ACDE: move.b (a0)+, d2
pub fn i_05acde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE0;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ACE0: eor.b d5, d2
pub fn i_05ace0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE2;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ACE2: andi.w #$ff, d4
pub fn i_05ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE6;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ACE6: move.w d4, d7
pub fn i_05ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE8;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ACE8: addq.w #$1, d7
pub fn i_05ace8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEA;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ACEA: swap d7
pub fn i_05acea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEC;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ACEC: move.w d3, d7
pub fn i_05acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEE;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ACEE: andi.w #$ff, d7
pub fn i_05acee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF2;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ACF2: addq.w #$1, d7
pub fn i_05acf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF4;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ACF4: lsl.w #$4, d4
pub fn i_05acf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF6;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05ACF6: eori.b #$f0, d4
pub fn i_05acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACFA;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05ACFA: move.b d4, $616e(a5)
pub fn i_05acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACFE;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05ACFE: move.b d3, d4
pub fn i_05acfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD00;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD00: andi.b #$f0, d4
pub fn i_05ad00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD04;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AD04: move.w d4, d6
pub fn i_05ad04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD06;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD06: subi.w #$100, d4
pub fn i_05ad06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0A;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD0A: swap d4
pub fn i_05ad0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD0C: move.w d6, d4
pub fn i_05ad0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0E;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD0E: andi.w #$f, d3
pub fn i_05ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD12;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD12: swap d2
pub fn i_05ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD14;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD14: move.w d3, d2
pub fn i_05ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD16;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD16: swap d2
pub fn i_05ad16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD18;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD18: moveq #$0, d5
pub fn i_05ad18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD1A: move.l (a0)+, d1
pub fn i_05ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1C;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AD1C: moveq #$0, d5
pub fn i_05ad1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1E;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD1E: move.b (a0)+, d5
pub fn i_05ad1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD20;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AD20: swap d5
pub fn i_05ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD22;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD22: move.b (a0)+, d5
pub fn i_05ad22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD24;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AD24: moveq #$0, d3
pub fn i_05ad24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD26;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD26: move.l d1, d6
pub fn i_05ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD28;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD28: muls.w d7, d6
pub fn i_05ad28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2A;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD2A: spl.b d3
pub fn i_05ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2C;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AD2C: add.l d3, d6
pub fn i_05ad2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2E;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05AD2E: asr.l #$8, d6
pub fn i_05ad2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD30;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05AD30: swap d7
pub fn i_05ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD32;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD32: swap d1
pub fn i_05ad32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD34;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD34: muls.w d7, d1
pub fn i_05ad34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD36;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD36: move.b d1, $616d(a5)
pub fn i_05ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3A;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AD3A: lsl.l #$8, d1
pub fn i_05ad3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3C;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05AD3C: move.w d6, d1
pub fn i_05ad3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3E;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD3E: swap d7
pub fn i_05ad3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD40;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD40: move.w d5, d6
pub fn i_05ad40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD42;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD42: cmpi.b #$11, d6
pub fn i_05ad42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD46;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05AD46: bcc.w $5ad60
pub fn i_05ad46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AD60; return 10; }
    8
}
/// $05AD4A: subq.b #$1, d6
pub fn i_05ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4C;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AD4C: lsl.b #$4, d6
pub fn i_05ad4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4E;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05AD4E: swap d2
pub fn i_05ad4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD50;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD50: or.b d2, d6
pub fn i_05ad50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD52;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AD52: lea.l $5a8ba(pc), a4
pub fn i_05ad52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD56;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05AD56: move.b (a4, d6.w), d6
pub fn i_05ad56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD5A;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05AD5A: swap d2
pub fn i_05ad5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD5C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD5C: bra.w $5ad66
pub fn i_05ad5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD60;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AD66;
    10
}
/// $05AD60: swap d2
pub fn i_05ad60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD62;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD62: move.w d2, d6
pub fn i_05ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD64;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD64: swap d2
pub fn i_05ad64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD66;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD66: andi.b #$f0, d4
pub fn i_05ad66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6A;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AD6A: or.b d4, d6
pub fn i_05ad6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6C;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AD6C: move.b d6, d4
pub fn i_05ad6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6E;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD6E: swap d4
pub fn i_05ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD70;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD70: move.b d6, d4
pub fn i_05ad70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD72;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD72: swap d4
pub fn i_05ad72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD74;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD74: move.w d7, d6
pub fn i_05ad74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD76;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD76: mulu.w d5, d6
pub fn i_05ad76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD78;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD78: addi.w #$ff, d6
pub fn i_05ad78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD7C;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD7C: lsr.w #$8, d6
pub fn i_05ad7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD7E;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AD7E: swap d6
pub fn i_05ad7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD80;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD80: move.w d5, d6
pub fn i_05ad80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD82;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD82: add.w d0, d1
pub fn i_05ad82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD84;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD84: swap d1
pub fn i_05ad84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD86;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD86: swap d0
pub fn i_05ad86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD88;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD88: add.w d0, d1
pub fn i_05ad88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD8A;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD8A: swap d0
pub fn i_05ad8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD8C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD8C: andi.w #$1ff, d1
pub fn i_05ad8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD90;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD90: lsl.l #$7, d1
pub fn i_05ad90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD92;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AD92: or.l d6, d1
pub fn i_05ad92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD94;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD94: clr.w d5
pub fn i_05ad94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD96;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AD96: swap d5
pub fn i_05ad96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD98;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD98: subq.b #$1, d5
pub fn i_05ad98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9A;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AD9A: add.w d6, d6
pub fn i_05ad9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9C;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD9C: movea.l d4, a4
pub fn i_05ad9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9E;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05AD9E: move.b $616e(a5), d3
pub fn i_05ad9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADA2;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05ADA2: sub.b d3, $616d(a5)
pub fn i_05ada2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADA6;
    cpu.ir = 0x972D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05ADA6: bcc.w $5adb2
pub fn i_05ada6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADB2; return 10; }
    8
}
/// $05ADAA: swap d4
pub fn i_05adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAC;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ADAC: tst.w d4
pub fn i_05adac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAE;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05ADAE: bmi.w $5adde
pub fn i_05adae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5ADDE; return 10; }
    8
}
/// $05ADB2: move.w d4, d3
pub fn i_05adb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB4;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ADB4: andi.w #$f00, d3
pub fn i_05adb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB8;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ADB8: addi.w #$100, d3
pub fn i_05adb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADBC;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ADBC: lsr.w #$1, d3
pub fn i_05adbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADBE;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05ADBE: cmpi.w #$f800, d1
pub fn i_05adbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADC2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05ADC2: bcc.w $5adce
pub fn i_05adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADCE; return 10; }
    8
}
/// $05ADC6: cmpi.w #$a000, d1
pub fn i_05adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADCA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05ADCA: bcc.w $5addc
pub fn i_05adca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADDC; return 10; }
    8
}
/// $05ADCE: move.l a0, (a3)+
pub fn i_05adce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD0;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ADD0: move.w d2, (a3)+
pub fn i_05add0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD2;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ADD2: move.w d4, (a3)+
pub fn i_05add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD4;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ADD4: move.l d1, (a3)+
pub fn i_05add4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD6;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ADD6: cmpa.l a2, a3
pub fn i_05add6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ADD8: bcc.w $5adec
pub fn i_05add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADEC; return 10; }
    8
}
/// $05ADDC: add.w d3, d1
pub fn i_05addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADDE;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ADDE: move.l a4, d4
pub fn i_05adde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE0;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ADE0: adda.w d6, a0
pub fn i_05ade0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE2;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ADE2: dbra d5, $5ad9e
pub fn i_05ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE6;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ADE6; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ADE6; return 14; }
    cpu.pc = 0x5AD9E;
    10
}
/// $05ADE6: subq.b #$1, (a7)
pub fn i_05ade6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE8;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ADE8: bne.w $5ad18
pub fn i_05ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AD18; return 10; }
    8
}
/// $05ADEC: addq.w #$2, a7
pub fn i_05adec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADEE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ADEE: lea.l $4258(a5), a2
pub fn i_05adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF2;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x057752, i_057752),
    (0x057754, i_057754),
    (0x057758, i_057758),
    (0x05775A, i_05775a),
    (0x05775C, i_05775c),
    (0x05775E, i_05775e),
    (0x057762, i_057762),
    (0x057766, i_057766),
    (0x057768, i_057768),
    (0x05776E, i_05776e),
    (0x057772, i_057772),
    (0x057776, i_057776),
    (0x057778, i_057778),
    (0x05777C, i_05777c),
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
    (0x05804C, i_05804c),
    (0x058052, i_058052),
    (0x058056, i_058056),
    (0x05805C, i_05805c),
    (0x058062, i_058062),
    (0x058068, i_058068),
    (0x05806C, i_05806c),
    (0x058070, i_058070),
    (0x058072, i_058072),
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
    (0x05A824, i_05a824),
    (0x05A826, i_05a826),
    (0x05A82E, i_05a82e),
    (0x05A836, i_05a836),
    (0x05A83C, i_05a83c),
    (0x05A840, i_05a840),
    (0x05A842, i_05a842),
    (0x05A844, i_05a844),
    (0x05A848, i_05a848),
    (0x05A84C, i_05a84c),
    (0x05A84E, i_05a84e),
    (0x05A856, i_05a856),
    (0x05A85E, i_05a85e),
    (0x05A864, i_05a864),
    (0x05A868, i_05a868),
    (0x05A86A, i_05a86a),
    (0x05A86C, i_05a86c),
    (0x05A86E, i_05a86e),
    (0x05A870, i_05a870),
    (0x05A872, i_05a872),
    (0x05A874, i_05a874),
    (0x05A878, i_05a878),
    (0x05A87E, i_05a87e),
    (0x05A884, i_05a884),
    (0x05A88A, i_05a88a),
    (0x05A890, i_05a890),
    (0x05A894, i_05a894),
    (0x05A898, i_05a898),
    (0x05A89C, i_05a89c),
    (0x05A8A0, i_05a8a0),
    (0x05A8A4, i_05a8a4),
    (0x05A8AA, i_05a8aa),
    (0x05A8AE, i_05a8ae),
    (0x05A8B2, i_05a8b2),
    (0x05A8B8, i_05a8b8),
    (0x05A9BA, i_05a9ba),
    (0x05A9C0, i_05a9c0),
    (0x05A9C6, i_05a9c6),
    (0x05A9CA, i_05a9ca),
    (0x05A9CE, i_05a9ce),
    (0x05A9D4, i_05a9d4),
    (0x05A9D6, i_05a9d6),
    (0x05A9DA, i_05a9da),
    (0x05A9DE, i_05a9de),
    (0x05A9E2, i_05a9e2),
    (0x05A9E6, i_05a9e6),
    (0x05A9EA, i_05a9ea),
    (0x05A9EC, i_05a9ec),
    (0x05A9F2, i_05a9f2),
    (0x05A9F4, i_05a9f4),
    (0x05A9F8, i_05a9f8),
    (0x05A9FA, i_05a9fa),
    (0x05A9FE, i_05a9fe),
    (0x05AA02, i_05aa02),
    (0x05AA06, i_05aa06),
    (0x05AA0A, i_05aa0a),
    (0x05AA0E, i_05aa0e),
    (0x05AA10, i_05aa10),
    (0x05AA14, i_05aa14),
    (0x05AA18, i_05aa18),
    (0x05AA1C, i_05aa1c),
    (0x05AA20, i_05aa20),
    (0x05AA24, i_05aa24),
    (0x05AA26, i_05aa26),
    (0x05AA2A, i_05aa2a),
    (0x05AA2E, i_05aa2e),
    (0x05AA30, i_05aa30),
    (0x05AA34, i_05aa34),
    (0x05AA38, i_05aa38),
    (0x05AA3C, i_05aa3c),
    (0x05AA3E, i_05aa3e),
    (0x05AA40, i_05aa40),
    (0x05AA44, i_05aa44),
    (0x05AA48, i_05aa48),
    (0x05AA4A, i_05aa4a),
    (0x05AA4C, i_05aa4c),
    (0x05AA4E, i_05aa4e),
    (0x05AA50, i_05aa50),
    (0x05AA52, i_05aa52),
    (0x05AA56, i_05aa56),
    (0x05AA5A, i_05aa5a),
    (0x05AA5C, i_05aa5c),
    (0x05AA5E, i_05aa5e),
    (0x05AA60, i_05aa60),
    (0x05AA64, i_05aa64),
    (0x05AA66, i_05aa66),
    (0x05AA6A, i_05aa6a),
    (0x05AA6C, i_05aa6c),
    (0x05AA70, i_05aa70),
    (0x05AA72, i_05aa72),
    (0x05AA76, i_05aa76),
    (0x05AA7A, i_05aa7a),
    (0x05AA7E, i_05aa7e),
    (0x05AA82, i_05aa82),
    (0x05AA86, i_05aa86),
    (0x05AA8A, i_05aa8a),
    (0x05AA8E, i_05aa8e),
    (0x05AA92, i_05aa92),
    (0x05AA94, i_05aa94),
    (0x05AA96, i_05aa96),
    (0x05AA9A, i_05aa9a),
    (0x05AA9E, i_05aa9e),
    (0x05AAA2, i_05aaa2),
    (0x05AAA6, i_05aaa6),
    (0x05AAAA, i_05aaaa),
    (0x05AAAE, i_05aaae),
    (0x05AAB2, i_05aab2),
    (0x05AAB6, i_05aab6),
    (0x05AAB8, i_05aab8),
    (0x05AABC, i_05aabc),
    (0x05AABE, i_05aabe),
    (0x05AAC0, i_05aac0),
    (0x05AAC2, i_05aac2),
    (0x05AAC6, i_05aac6),
    (0x05AAC8, i_05aac8),
    (0x05AACA, i_05aaca),
    (0x05AACC, i_05aacc),
    (0x05AACE, i_05aace),
    (0x05AAD0, i_05aad0),
    (0x05AAD2, i_05aad2),
    (0x05AAD4, i_05aad4),
    (0x05AAD6, i_05aad6),
    (0x05AAD8, i_05aad8),
    (0x05AADA, i_05aada),
    (0x05AADC, i_05aadc),
    (0x05AADE, i_05aade),
    (0x05AAE0, i_05aae0),
    (0x05AAE2, i_05aae2),
    (0x05AAE6, i_05aae6),
    (0x05AAE8, i_05aae8),
    (0x05AAEA, i_05aaea),
    (0x05AAEC, i_05aaec),
    (0x05AAEE, i_05aaee),
    (0x05AAF0, i_05aaf0),
    (0x05AAF2, i_05aaf2),
    (0x05AAF6, i_05aaf6),
    (0x05AAFA, i_05aafa),
    (0x05AAFE, i_05aafe),
    (0x05AB02, i_05ab02),
    (0x05AB06, i_05ab06),
    (0x05AB08, i_05ab08),
    (0x05AB0A, i_05ab0a),
    (0x05AB0C, i_05ab0c),
    (0x05AB0E, i_05ab0e),
    (0x05AB10, i_05ab10),
    (0x05AB14, i_05ab14),
    (0x05AB16, i_05ab16),
    (0x05AB18, i_05ab18),
    (0x05AB1C, i_05ab1c),
    (0x05AB1E, i_05ab1e),
    (0x05AB20, i_05ab20),
    (0x05AB22, i_05ab22),
    (0x05AB26, i_05ab26),
    (0x05AB28, i_05ab28),
    (0x05AB2C, i_05ab2c),
    (0x05AB32, i_05ab32),
    (0x05AB34, i_05ab34),
    (0x05AB36, i_05ab36),
    (0x05AB3A, i_05ab3a),
    (0x05AB3C, i_05ab3c),
    (0x05AB3E, i_05ab3e),
    (0x05AB40, i_05ab40),
    (0x05AB44, i_05ab44),
    (0x05AB46, i_05ab46),
    (0x05AB48, i_05ab48),
    (0x05AB4A, i_05ab4a),
    (0x05AB4C, i_05ab4c),
    (0x05AB4E, i_05ab4e),
    (0x05AB50, i_05ab50),
    (0x05AB52, i_05ab52),
    (0x05AB54, i_05ab54),
    (0x05AB56, i_05ab56),
    (0x05AB58, i_05ab58),
    (0x05AB5A, i_05ab5a),
    (0x05AB5C, i_05ab5c),
    (0x05AB5E, i_05ab5e),
    (0x05AB60, i_05ab60),
    (0x05AB62, i_05ab62),
    (0x05AB64, i_05ab64),
    (0x05AB66, i_05ab66),
    (0x05AB6A, i_05ab6a),
    (0x05AB6C, i_05ab6c),
    (0x05AB6E, i_05ab6e),
    (0x05AB70, i_05ab70),
    (0x05AB72, i_05ab72),
    (0x05AB74, i_05ab74),
    (0x05AB76, i_05ab76),
    (0x05AB7A, i_05ab7a),
    (0x05AB7C, i_05ab7c),
    (0x05AB80, i_05ab80),
    (0x05AB84, i_05ab84),
    (0x05AB88, i_05ab88),
    (0x05AB8C, i_05ab8c),
    (0x05AB8E, i_05ab8e),
    (0x05AB90, i_05ab90),
    (0x05AB92, i_05ab92),
    (0x05AB94, i_05ab94),
    (0x05AB96, i_05ab96),
    (0x05AB9A, i_05ab9a),
    (0x05AB9C, i_05ab9c),
    (0x05ABA0, i_05aba0),
    (0x05ABA2, i_05aba2),
    (0x05ABA4, i_05aba4),
    (0x05ABA6, i_05aba6),
    (0x05ABAA, i_05abaa),
    (0x05ABAC, i_05abac),
    (0x05ABB0, i_05abb0),
    (0x05ABB6, i_05abb6),
    (0x05ABB8, i_05abb8),
    (0x05ABBA, i_05abba),
    (0x05ABBE, i_05abbe),
    (0x05ABC0, i_05abc0),
    (0x05ABC2, i_05abc2),
    (0x05ABC4, i_05abc4),
    (0x05ABC8, i_05abc8),
    (0x05ABCA, i_05abca),
    (0x05ABCC, i_05abcc),
    (0x05ABCE, i_05abce),
    (0x05ABD0, i_05abd0),
    (0x05ABD2, i_05abd2),
    (0x05ABD4, i_05abd4),
    (0x05ABD6, i_05abd6),
    (0x05ABD8, i_05abd8),
    (0x05ABDA, i_05abda),
    (0x05ABDC, i_05abdc),
    (0x05ABDE, i_05abde),
    (0x05ABE0, i_05abe0),
    (0x05ABE2, i_05abe2),
    (0x05ABE4, i_05abe4),
    (0x05ABE6, i_05abe6),
    (0x05ABE8, i_05abe8),
    (0x05ABEA, i_05abea),
    (0x05ABEC, i_05abec),
    (0x05ABEE, i_05abee),
    (0x05ABF2, i_05abf2),
    (0x05ABF4, i_05abf4),
    (0x05ABF6, i_05abf6),
    (0x05ABF8, i_05abf8),
    (0x05ABFA, i_05abfa),
    (0x05ABFC, i_05abfc),
    (0x05ABFE, i_05abfe),
    (0x05AC00, i_05ac00),
    (0x05AC04, i_05ac04),
    (0x05AC08, i_05ac08),
    (0x05AC0C, i_05ac0c),
    (0x05AC10, i_05ac10),
    (0x05AC14, i_05ac14),
    (0x05AC16, i_05ac16),
    (0x05AC18, i_05ac18),
    (0x05AC1A, i_05ac1a),
    (0x05AC1C, i_05ac1c),
    (0x05AC1E, i_05ac1e),
    (0x05AC22, i_05ac22),
    (0x05AC24, i_05ac24),
    (0x05AC26, i_05ac26),
    (0x05AC2A, i_05ac2a),
    (0x05AC2C, i_05ac2c),
    (0x05AC2E, i_05ac2e),
    (0x05AC30, i_05ac30),
    (0x05AC32, i_05ac32),
    (0x05AC36, i_05ac36),
    (0x05AC38, i_05ac38),
    (0x05AC3C, i_05ac3c),
    (0x05AC42, i_05ac42),
    (0x05AC44, i_05ac44),
    (0x05AC46, i_05ac46),
    (0x05AC4A, i_05ac4a),
    (0x05AC4C, i_05ac4c),
    (0x05AC4E, i_05ac4e),
    (0x05AC50, i_05ac50),
    (0x05AC54, i_05ac54),
    (0x05AC56, i_05ac56),
    (0x05AC58, i_05ac58),
    (0x05AC5A, i_05ac5a),
    (0x05AC5C, i_05ac5c),
    (0x05AC5E, i_05ac5e),
    (0x05AC60, i_05ac60),
    (0x05AC62, i_05ac62),
    (0x05AC64, i_05ac64),
    (0x05AC66, i_05ac66),
    (0x05AC68, i_05ac68),
    (0x05AC6A, i_05ac6a),
    (0x05AC6C, i_05ac6c),
    (0x05AC6E, i_05ac6e),
    (0x05AC70, i_05ac70),
    (0x05AC72, i_05ac72),
    (0x05AC74, i_05ac74),
    (0x05AC76, i_05ac76),
    (0x05AC78, i_05ac78),
    (0x05AC7A, i_05ac7a),
    (0x05AC7C, i_05ac7c),
    (0x05AC7E, i_05ac7e),
    (0x05AC80, i_05ac80),
    (0x05AC84, i_05ac84),
    (0x05AC86, i_05ac86),
    (0x05AC88, i_05ac88),
    (0x05AC8A, i_05ac8a),
    (0x05AC8C, i_05ac8c),
    (0x05AC8E, i_05ac8e),
    (0x05AC90, i_05ac90),
    (0x05AC92, i_05ac92),
    (0x05AC96, i_05ac96),
    (0x05AC98, i_05ac98),
    (0x05AC9C, i_05ac9c),
    (0x05ACA0, i_05aca0),
    (0x05ACA4, i_05aca4),
    (0x05ACA8, i_05aca8),
    (0x05ACAA, i_05acaa),
    (0x05ACAC, i_05acac),
    (0x05ACAE, i_05acae),
    (0x05ACB0, i_05acb0),
    (0x05ACB2, i_05acb2),
    (0x05ACB6, i_05acb6),
    (0x05ACB8, i_05acb8),
    (0x05ACBC, i_05acbc),
    (0x05ACBE, i_05acbe),
    (0x05ACC0, i_05acc0),
    (0x05ACC2, i_05acc2),
    (0x05ACC4, i_05acc4),
    (0x05ACC8, i_05acc8),
    (0x05ACCA, i_05acca),
    (0x05ACCE, i_05acce),
    (0x05ACD4, i_05acd4),
    (0x05ACD6, i_05acd6),
    (0x05ACD8, i_05acd8),
    (0x05ACDC, i_05acdc),
    (0x05ACDE, i_05acde),
    (0x05ACE0, i_05ace0),
    (0x05ACE2, i_05ace2),
    (0x05ACE6, i_05ace6),
    (0x05ACE8, i_05ace8),
    (0x05ACEA, i_05acea),
    (0x05ACEC, i_05acec),
    (0x05ACEE, i_05acee),
    (0x05ACF2, i_05acf2),
    (0x05ACF4, i_05acf4),
    (0x05ACF6, i_05acf6),
    (0x05ACFA, i_05acfa),
    (0x05ACFE, i_05acfe),
    (0x05AD00, i_05ad00),
    (0x05AD04, i_05ad04),
    (0x05AD06, i_05ad06),
    (0x05AD0A, i_05ad0a),
    (0x05AD0C, i_05ad0c),
    (0x05AD0E, i_05ad0e),
    (0x05AD12, i_05ad12),
    (0x05AD14, i_05ad14),
    (0x05AD16, i_05ad16),
    (0x05AD18, i_05ad18),
    (0x05AD1A, i_05ad1a),
    (0x05AD1C, i_05ad1c),
    (0x05AD1E, i_05ad1e),
    (0x05AD20, i_05ad20),
    (0x05AD22, i_05ad22),
    (0x05AD24, i_05ad24),
    (0x05AD26, i_05ad26),
    (0x05AD28, i_05ad28),
    (0x05AD2A, i_05ad2a),
    (0x05AD2C, i_05ad2c),
    (0x05AD2E, i_05ad2e),
    (0x05AD30, i_05ad30),
    (0x05AD32, i_05ad32),
    (0x05AD34, i_05ad34),
    (0x05AD36, i_05ad36),
    (0x05AD3A, i_05ad3a),
    (0x05AD3C, i_05ad3c),
    (0x05AD3E, i_05ad3e),
    (0x05AD40, i_05ad40),
    (0x05AD42, i_05ad42),
    (0x05AD46, i_05ad46),
    (0x05AD4A, i_05ad4a),
    (0x05AD4C, i_05ad4c),
    (0x05AD4E, i_05ad4e),
    (0x05AD50, i_05ad50),
    (0x05AD52, i_05ad52),
    (0x05AD56, i_05ad56),
    (0x05AD5A, i_05ad5a),
    (0x05AD5C, i_05ad5c),
    (0x05AD60, i_05ad60),
    (0x05AD62, i_05ad62),
    (0x05AD64, i_05ad64),
    (0x05AD66, i_05ad66),
    (0x05AD6A, i_05ad6a),
    (0x05AD6C, i_05ad6c),
    (0x05AD6E, i_05ad6e),
    (0x05AD70, i_05ad70),
    (0x05AD72, i_05ad72),
    (0x05AD74, i_05ad74),
    (0x05AD76, i_05ad76),
    (0x05AD78, i_05ad78),
    (0x05AD7C, i_05ad7c),
    (0x05AD7E, i_05ad7e),
    (0x05AD80, i_05ad80),
    (0x05AD82, i_05ad82),
    (0x05AD84, i_05ad84),
    (0x05AD86, i_05ad86),
    (0x05AD88, i_05ad88),
    (0x05AD8A, i_05ad8a),
    (0x05AD8C, i_05ad8c),
    (0x05AD90, i_05ad90),
    (0x05AD92, i_05ad92),
    (0x05AD94, i_05ad94),
    (0x05AD96, i_05ad96),
    (0x05AD98, i_05ad98),
    (0x05AD9A, i_05ad9a),
    (0x05AD9C, i_05ad9c),
    (0x05AD9E, i_05ad9e),
    (0x05ADA2, i_05ada2),
    (0x05ADA6, i_05ada6),
    (0x05ADAA, i_05adaa),
    (0x05ADAC, i_05adac),
    (0x05ADAE, i_05adae),
    (0x05ADB2, i_05adb2),
    (0x05ADB4, i_05adb4),
    (0x05ADB8, i_05adb8),
    (0x05ADBC, i_05adbc),
    (0x05ADBE, i_05adbe),
    (0x05ADC2, i_05adc2),
    (0x05ADC6, i_05adc6),
    (0x05ADCA, i_05adca),
    (0x05ADCE, i_05adce),
    (0x05ADD0, i_05add0),
    (0x05ADD2, i_05add2),
    (0x05ADD4, i_05add4),
    (0x05ADD6, i_05add6),
    (0x05ADD8, i_05add8),
    (0x05ADDC, i_05addc),
    (0x05ADDE, i_05adde),
    (0x05ADE0, i_05ade0),
    (0x05ADE2, i_05ade2),
    (0x05ADE6, i_05ade6),
    (0x05ADE8, i_05ade8),
    (0x05ADEC, i_05adec),
    (0x05ADEE, i_05adee),
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
];
