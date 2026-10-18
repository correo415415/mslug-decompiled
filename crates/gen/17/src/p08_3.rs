// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0850A8: movea.l #$ffffffff, a0
pub fn i_0850a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850AE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0850AE: lea.l $2e9860.l, a0
pub fn i_0850ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $0850B4: jsr $5dd5c.l
pub fn i_0850b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850BA);
    cpu.pc = 0x5DD5C;
    16
}
/// $0850BA: bcc.w $850c4
pub fn i_0850ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x850C4; return 10; }
    8
}
/// $0850BE: jmp $518.l
pub fn i_0850be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0850C4: rts
pub fn i_0850c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0850c6  ($0850C6, 102 B, enemies/squad/rescue_squad_084836.s)
/// $0850C6: lea.l $2e78a4.l, a0
pub fn i_0850c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78A4;
    20
}
/// $0850CC: jsr $28cd4.l
pub fn i_0850cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850D2);
    cpu.pc = 0x28CD4;
    16
}
/// $0850D2: move.w #$14, $72(a6)
pub fn i_0850d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0850D8: lea.l $850de(pc), a1
pub fn i_0850d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x850DE;
    16
}
/// $0850DC: move.l a1, (a6)
pub fn i_0850dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0850DE: jsr $2783a.l
pub fn i_0850de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850E4);
    cpu.pc = 0x2783A;
    16
}
/// $0850E4: jsr $28d70.l
pub fn i_0850e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850EA);
    cpu.pc = 0x28D70;
    16
}
/// $0850EA: jsr $85f44(pc)
pub fn i_0850ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x850EE);
    cpu.pc = 0x85F44;
    16
}
/// $0850EE: subq.w #$1, $72(a6)
pub fn i_0850ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850F2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0850F2: bpl.w $8510e
pub fn i_0850f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850F6;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8510E; return 10; }
    8
}
/// $0850F6: jsr $85f60(pc)
pub fn i_0850f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x850FA);
    cpu.pc = 0x85F60;
    16
}
/// $0850FA: bcc.w $85108
pub fn i_0850fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85108; return 10; }
    8
}
/// $0850FE: lea.l $84fca(pc), a1
pub fn i_0850fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85102;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84FCA;
    16
}
/// $085102: move.l a1, (a6)
pub fn i_085102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85104;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085104: bra.w $8510e
pub fn i_085104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85108;
    cpu.ir = 0x6000;
    cpu.pc = 0x8510E;
    10
}
/// $085108: lea.l $84f5e(pc), a1
pub fn i_085108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8510C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84F5E;
    16
}
/// $08510C: move.l a1, (a6)
pub fn i_08510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8510E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08510E: movea.l #$ffffffff, a0
pub fn i_08510e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85114;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $085114: lea.l $2e9860.l, a0
pub fn i_085114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8511A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $08511A: jsr $5dd5c.l
pub fn i_08511a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85120);
    cpu.pc = 0x5DD5C;
    16
}
/// $085120: bcc.w $8512a
pub fn i_085120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85124;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8512A; return 10; }
    8
}
/// $085124: jmp $518.l
pub fn i_085124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8512A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08512A: rts
pub fn i_08512a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8512C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_08512c  ($08512C, 8 B, kernel/task_handlers.c)
/// $08512C: lea.l $85134(pc), a1
pub fn i_08512c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85130;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85134;
    16
}
/// $085130: move.l a1, (a6)
pub fn i_085130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85132;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085132: rts
pub fn i_085132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85134;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_PlatformSpawn_085134  ($085134, 90 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085134: lea.l $676ba.l, a1
pub fn i_085134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8513A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x676BA;
    20
}
/// $08513A: jsr $4ae.l
pub fn i_08513a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85140);
    cpu.pc = 0x4AE;
    16
}
/// $085140: jsr $5dd02.l
pub fn i_085140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85146);
    cpu.pc = 0x5DD02;
    16
}
/// $085146: move.w #$160, $22(a0)
pub fn i_085146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8514C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08514C: movem.l a0, -(a7)
pub fn i_08514c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85150;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $085150: move.w #$0, d0
pub fn i_085150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85154;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085154: move.w #$320, d1
pub fn i_085154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85158;
    cpu.ir = 0x323C;
    let v1: u32 = 0x320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085158: jsr $44022.l
pub fn i_085158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8515E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8515E);
    cpu.pc = 0x44022;
    16
}
/// $08515E: movem.l (a7)+, a0
pub fn i_08515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85162;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $085162: move.w d1, $24(a0)
pub fn i_085162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85166;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085166: move.l #$0, $98(a0)
pub fn i_085166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8516E;
    cpu.ir = 0x217C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08516E: move.l #$0, $9c(a0)
pub fn i_08516e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85176;
    cpu.ir = 0x217C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085176: move.b #$0, $98(a0)
pub fn i_085176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8517C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08517C: move.b #$10, $99(a0)
pub fn i_08517c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85182;
    cpu.ir = 0x117C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085182: move.b #$1, $11(a0)
pub fn i_085182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85188;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085188: jmp $518.l
pub fn i_085188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8518E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08518e  ($08518E, 2 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08518E: rts
pub fn i_08518e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Init_085190  ($085190, 88 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085190: move.w #$1c5, d1
pub fn i_085190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85194;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085194: jsr $236e.l
pub fn i_085194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8519A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8519A);
    cpu.pc = 0x236E;
    16
}
/// $08519A: lea.l $2c029a.l, a0
pub fn i_08519a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C029A;
    20
}
/// $0851A0: jsr $799de.l
pub fn i_0851a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851A6);
    cpu.pc = 0x799DE;
    16
}
/// $0851A6: move.w d0, $66(a6)
pub fn i_0851a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0851AA: lea.l $85394(pc), a1
pub fn i_0851aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85394;
    16
}
/// $0851AE: jsr $4ae.l
pub fn i_0851ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851B4);
    cpu.pc = 0x4AE;
    16
}
/// $0851B4: jsr $5dd22.l
pub fn i_0851b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851BA);
    cpu.pc = 0x5DD22;
    16
}
/// $0851BA: move.b #$0, $20(a6)
pub fn i_0851ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851C0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0851C0: move.b #$0, $21(a6)
pub fn i_0851c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0851C6: move.b #$0, $89(a6)
pub fn i_0851c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0851CC: lea.l $851d2(pc), a1
pub fn i_0851cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x851D2;
    16
}
/// $0851D0: move.l a1, (a6)
pub fn i_0851d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0851D2: move.l $106f5c.l, d0
pub fn i_0851d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851D8;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0851D8: swap d0
pub fn i_0851d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851DA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0851DA: cmpi.w #$1f0, d0
pub fn i_0851da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851DE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1F0, Size::Word, false);
    8
}
/// $0851DE: bgt.w $851f0
pub fn i_0851de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851E2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x851F0; return 10; }
    8
}
/// $0851E2: clr.b $10e39a.l
pub fn i_0851e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851E8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
// ===== JsrAbsThunk_0851e8  ($0851E8, 8 B, thunks/jsr_abs_thunks.c)
/// $0851E8: jsr $2783a.l
pub fn i_0851e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851EE);
    cpu.pc = 0x2783A;
    16
}
/// $0851EE: rts
pub fn i_0851ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Approach_0851f0  ($0851F0, 120 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0851F0: lea.l $85608(pc), a1
pub fn i_0851f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85608;
    16
}
/// $0851F4: jsr $4ae.l
pub fn i_0851f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851FA);
    cpu.pc = 0x4AE;
    16
}
/// $0851FA: lea.l $2e93fa.l, a0
pub fn i_0851fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85200;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E93FA;
    20
}
/// $085200: move.l a0, $48(a6)
pub fn i_085200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85204;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085204: lea.l $2e78ce.l, a0
pub fn i_085204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8520A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78CE;
    20
}
/// $08520A: jsr $28cd4.l
pub fn i_08520a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85210;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85210);
    cpu.pc = 0x28CD4;
    16
}
/// $085210: lea.l $85216(pc), a1
pub fn i_085210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85214;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85216;
    16
}
/// $085214: move.l a1, (a6)
pub fn i_085214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85216;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085216: clr.b $10e39a.l
pub fn i_085216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8521C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08521C: jsr $2783a.l
pub fn i_08521c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85222);
    cpu.pc = 0x2783A;
    16
}
/// $085222: move.b #$0, $89(a6)
pub fn i_085222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85228;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085228: jsr $2870a.l
pub fn i_085228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8522E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8522E);
    cpu.pc = 0x2870A;
    16
}
/// $08522E: bcc.w $85268
pub fn i_08522e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85232;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85268; return 10; }
    8
}
/// $085232: move.b #$ff, $89(a6)
pub fn i_085232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85238;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085238: lea.l $2e78b4.l, a0
pub fn i_085238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8523E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78B4;
    20
}
/// $08523E: jsr $28cd4.l
pub fn i_08523e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85244;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85244);
    cpu.pc = 0x28CD4;
    16
}
/// $085244: lea.l $5e766.l, a0
pub fn i_085244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8524A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08524A: jsr $5e798.l
pub fn i_08524a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85250);
    cpu.pc = 0x5E798;
    16
}
/// $085250: bclr.b #$3, $13(a6)
pub fn i_085250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85256;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $085256: lea.l $2e994e.l, a1
pub fn i_085256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8525C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E994E;
    20
}
/// $08525C: jsr $77c7e.l
pub fn i_08525c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85262;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85262);
    cpu.pc = 0x77C7E;
    16
}
/// $085262: lea.l $85270(pc), a1
pub fn i_085262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85266;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85270;
    16
}
/// $085266: move.l a1, (a6)
pub fn i_085266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85268;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_085268  ($085268, 8 B, thunks/jsr_abs_thunks.c)
/// $085268: jsr $28d70.l
pub fn i_085268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8526E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8526E);
    cpu.pc = 0x28D70;
    16
}
/// $08526E: rts
pub fn i_08526e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Fight_085270  ($085270, 192 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085270: move.w #$102d, d0
pub fn i_085270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85274;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085274: jsr $2352.l
pub fn i_085274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8527A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8527A);
    cpu.pc = 0x2352;
    16
}
/// $08527A: lea.l $2ea858.l, a2
pub fn i_08527a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85280;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA858;
    20
}
/// $085280: jsr $5022a.l
pub fn i_085280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85286);
    cpu.pc = 0x5022A;
    16
}
/// $085286: lea.l $8528c(pc), a1
pub fn i_085286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8528A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8528C;
    16
}
/// $08528A: move.l a1, (a6)
pub fn i_08528a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8528C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08528C: clr.b $10e39a.l
pub fn i_08528c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85292;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $085292: jsr $2783a.l
pub fn i_085292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85298);
    cpu.pc = 0x2783A;
    16
}
/// $085298: move.b #$0, $89(a6)
pub fn i_085298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8529E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08529E: jsr $2870a.l
pub fn i_08529e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852A4);
    cpu.pc = 0x2870A;
    16
}
/// $0852A4: bcc.w $852cc
pub fn i_0852a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x852CC; return 10; }
    8
}
/// $0852A8: move.b #$ff, $89(a6)
pub fn i_0852a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0852AE: lea.l $2e78de.l, a0
pub fn i_0852ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78DE;
    20
}
/// $0852B4: jsr $28cd4.l
pub fn i_0852b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0852BA: lea.l $5e766.l, a0
pub fn i_0852ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0852C0: jsr $5e798.l
pub fn i_0852c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852C6);
    cpu.pc = 0x5E798;
    16
}
/// $0852C6: bclr.b #$3, $13(a6)
pub fn i_0852c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852CC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0852CC: jsr $28758.l
pub fn i_0852cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852D2);
    cpu.pc = 0x28758;
    16
}
/// $0852D2: bcc.w $85330
pub fn i_0852d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85330; return 10; }
    8
}
/// $0852D6: lea.l $ffff.w, a0
pub fn i_0852d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852DA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0852DA: move.l a0, $48(a6)
pub fn i_0852da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852DE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0852DE: move.b #$ff, $20(a6)
pub fn i_0852de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0852E4: lea.l $2eaf02.l, a1
pub fn i_0852e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAF02;
    20
}
/// $0852EA: jsr $43fac.l
pub fn i_0852ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852F0);
    cpu.pc = 0x43FAC;
    16
}
/// $0852F0: lea.l $2e9960.l, a1
pub fn i_0852f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9960;
    20
}
/// $0852F6: jsr $77c7e.l
pub fn i_0852f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852FC);
    cpu.pc = 0x77C7E;
    16
}
/// $0852FC: lea.l $2e9972.l, a1
pub fn i_0852fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85302;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9972;
    20
}
/// $085302: jsr $77c7e.l
pub fn i_085302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85308);
    cpu.pc = 0x77C7E;
    16
}
/// $085308: move.b #$ff, $74(a6)
pub fn i_085308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8530E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08530E: lea.l $2e9790.l, a0
pub fn i_08530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85314;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9790;
    20
}
/// $085314: move.l a0, $4c(a6)
pub fn i_085314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85318;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085318: jsr $283ca.l
pub fn i_085318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8531E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8531E);
    cpu.pc = 0x283CA;
    16
}
/// $08531E: jsr $283ca.l
pub fn i_08531e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85324);
    cpu.pc = 0x283CA;
    16
}
/// $085324: jsr $283d8.l
pub fn i_085324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8532A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8532A);
    cpu.pc = 0x283D8;
    16
}
/// $08532A: lea.l $85338(pc), a1
pub fn i_08532a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8532E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85338;
    16
}
/// $08532E: move.l a1, (a6)
pub fn i_08532e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85330;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_085330  ($085330, 8 B, thunks/jsr_abs_thunks.c)
/// $085330: jsr $28d70.l
pub fn i_085330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85336);
    cpu.pc = 0x28D70;
    16
}
/// $085336: rts
pub fn i_085336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Wreck_085338  ($085338, 92 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085338: lea.l $ffff.w, a0
pub fn i_085338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8533C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08533C: move.l a0, $4c(a6)
pub fn i_08533c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85340;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085340: jsr $283ca.l
pub fn i_085340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85346);
    cpu.pc = 0x283CA;
    16
}
/// $085346: move.w #$1028, d0
pub fn i_085346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8534A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08534A: jsr $2352.l
pub fn i_08534a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85350);
    cpu.pc = 0x2352;
    16
}
/// $085350: jsr $85b7e(pc)
pub fn i_085350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85354;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85354);
    cpu.pc = 0x85B7E;
    16
}
/// $085354: move.w #$74, d0
pub fn i_085354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85358;
    cpu.ir = 0x303C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085358: move.b #$0, $74(a6)
pub fn i_085358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8535E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08535E: lea.l $ec6c8.l, a1
pub fn i_08535e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85364;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEC6C8;
    20
}
/// $085364: jsr $4429e.l
pub fn i_085364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8536A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8536A);
    cpu.pc = 0x4429E;
    16
}
/// $08536A: lea.l $2ea86c.l, a2
pub fn i_08536a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85370;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA86C;
    20
}
/// $085370: jsr $5022a.l
pub fn i_085370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85376);
    cpu.pc = 0x5022A;
    16
}
/// $085376: lea.l $8537c(pc), a1
pub fn i_085376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8537A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8537C;
    16
}
/// $08537A: move.l a1, (a6)
pub fn i_08537a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8537C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08537C: clr.b $10e39a.l
pub fn i_08537c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85382;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $085382: cmpi.b #$ff, $21(a6)
pub fn i_085382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85388;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085388: bne.w $85392
pub fn i_085388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8538C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85392; return 10; }
    8
}
/// $08538C: jmp $518.l
pub fn i_08538c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85392;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085392: rts
pub fn i_085392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85394;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Init_085394  ($085394, 118 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085394: lea.l $856aa(pc), a1
pub fn i_085394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85398;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x856AA;
    16
}
/// $085398: jsr $4ae.l
pub fn i_085398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8539E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8539E);
    cpu.pc = 0x4AE;
    16
}
/// $08539E: movea.l $c(a6), a1
pub fn i_08539e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853A2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0853A2: move.w $22(a1), $22(a0)
pub fn i_0853a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853A8;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0853A8: move.w $24(a1), $24(a0)
pub fn i_0853a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853AE;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0853AE: move.w #$100, d0
pub fn i_0853ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0853B2: move.w #$2f6, d1
pub fn i_0853b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2F6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0853B6: jsr $44022.l
pub fn i_0853b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x853BC);
    cpu.pc = 0x44022;
    16
}
/// $0853BC: move.w d0, $22(a6)
pub fn i_0853bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0853C0: move.w d1, $24(a6)
pub fn i_0853c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853C4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0853C4: move.w #$1c8, d1
pub fn i_0853c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0853C8: jsr $236e.l
pub fn i_0853c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x853CE);
    cpu.pc = 0x236E;
    16
}
/// $0853CE: lea.l $2e7908.l, a0
pub fn i_0853ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7908;
    20
}
/// $0853D4: jsr $28cd4.l
pub fn i_0853d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x853DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0853DA: move.w #$100, $38(a6)
pub fn i_0853da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0853E0: move.l #$2eb0b8, $60(a6)
pub fn i_0853e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853E8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EB0B8;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0853E8: lea.l $853ee(pc), a1
pub fn i_0853e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x853EE;
    16
}
/// $0853EC: move.l a1, (a6)
pub fn i_0853ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0853EE: move.l $106f5c.l, d0
pub fn i_0853ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853F4;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0853F4: swap d0
pub fn i_0853f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853F6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0853F6: cmpi.w #$1f0, d0
pub fn i_0853f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853FA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1F0, Size::Word, false);
    8
}
/// $0853FA: bgt.w $85412
pub fn i_0853fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853FE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85412; return 10; }
    8
}
/// $0853FE: jsr $28998.l
pub fn i_0853fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85404;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85404);
    cpu.pc = 0x28998;
    16
}
/// $085404: jsr $2783a.l
pub fn i_085404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8540A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8540A);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08540a  ($08540A, 8 B, thunks/jsr_abs_thunks.c)
/// $08540A: jsr $28d70.l
pub fn i_08540a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85410);
    cpu.pc = 0x28D70;
    16
}
/// $085410: rts
pub fn i_085410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Active_085412  ($085412, 106 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085412: jsr $267e2.l
pub fn i_085412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85418);
    cpu.pc = 0x267E2;
    16
}
/// $085418: bclr.b #$1, $12(a6)
pub fn i_085418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8541E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08541E: lea.l $ffff.w, a0
pub fn i_08541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85422;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $085422: move.l a0, $4c(a6)
pub fn i_085422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85426;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085426: jsr $283ca.l
pub fn i_085426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8542C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8542C);
    cpu.pc = 0x283CA;
    16
}
/// $08542C: lea.l $85526(pc), a1
pub fn i_08542c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85430;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85526;
    16
}
/// $085430: jsr $4ae.l
pub fn i_085430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85436);
    cpu.pc = 0x4AE;
    16
}
/// $085436: lea.l $8543c(pc), a1
pub fn i_085436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8543A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8543C;
    16
}
/// $08543A: move.l a1, (a6)
pub fn i_08543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8543C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08543C: jsr $28998.l
pub fn i_08543c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85442);
    cpu.pc = 0x28998;
    16
}
/// $085442: jsr $27cee.l
pub fn i_085442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85448);
    cpu.pc = 0x27CEE;
    16
}
/// $085448: jsr $85d32(pc)
pub fn i_085448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8544C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8544C);
    cpu.pc = 0x85D32;
    16
}
/// $08544C: jsr $85ebc(pc)
pub fn i_08544c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85450;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85450);
    cpu.pc = 0x85EBC;
    16
}
/// $085450: jsr $28d70.l
pub fn i_085450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85456;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85456);
    cpu.pc = 0x28D70;
    16
}
/// $085456: jsr $85da8(pc)
pub fn i_085456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8545A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8545A);
    cpu.pc = 0x85DA8;
    16
}
/// $08545A: movea.l $c(a6), a0
pub fn i_08545a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8545E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08545E: cmpi.b #$ff, $20(a0)
pub fn i_08545e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85464;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085464: bne.w $85482
pub fn i_085464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85468;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85482; return 10; }
    8
}
/// $085468: lea.l $ffff.w, a0
pub fn i_085468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8546C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08546C: move.l a0, $48(a6)
pub fn i_08546c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85470;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085470: lea.l $2e9984.l, a1
pub fn i_085470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85476;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9984;
    20
}
/// $085476: jsr $77c7e.l
pub fn i_085476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8547C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8547C);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetTaskHandler_08547c  ($08547C, 8 B, kernel/task_handlers.c)
/// $08547C: lea.l $85484(pc), a1
pub fn i_08547c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85480;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85484;
    16
}
/// $085480: move.l a1, (a6)
pub fn i_085480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85482;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085482: rts
pub fn i_085482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85484;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Death_085484  ($085484, 100 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085484: lea.l $ffff.w, a0
pub fn i_085484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85488;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $085488: move.l a0, $48(a6)
pub fn i_085488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8548C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08548C: move.w #$1033, d0
pub fn i_08548c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85490;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085490: jsr $2352.l
pub fn i_085490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85496);
    cpu.pc = 0x2352;
    16
}
/// $085496: lea.l $2e9996.l, a1
pub fn i_085496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8549C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9996;
    20
}
/// $08549C: jsr $77c7e.l
pub fn i_08549c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854A2);
    cpu.pc = 0x77C7E;
    16
}
/// $0854A2: jsr $267e2.l
pub fn i_0854a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854A8);
    cpu.pc = 0x267E2;
    16
}
/// $0854A8: move.w #$ffe0, $2e(a6)
pub fn i_0854a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0854AE: lea.l $2e7940.l, a0
pub fn i_0854ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7940;
    20
}
/// $0854B4: jsr $28cd4.l
pub fn i_0854b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0854BA: lea.l $854c0(pc), a1
pub fn i_0854ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x854C0;
    16
}
/// $0854BE: move.l a1, (a6)
pub fn i_0854be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0854C0: jsr $27cee.l
pub fn i_0854c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854C6);
    cpu.pc = 0x27CEE;
    16
}
/// $0854C6: jsr $28d70.l
pub fn i_0854c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854CC);
    cpu.pc = 0x28D70;
    16
}
/// $0854CC: cmpi.w #$20, $24(a6)
pub fn i_0854cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $0854D2: bgt.w $854e6
pub fn i_0854d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854D6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x854E6; return 10; }
    8
}
/// $0854D6: movea.l $c(a6), a0
pub fn i_0854d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854DA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0854DA: move.b #$ff, $21(a0)
pub fn i_0854da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854E0;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0854E0: jmp $518.l
pub fn i_0854e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854E6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0854E6: rts
pub fn i_0854e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SmokePuff_0854e8  ($0854E8, 62 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0854E8: move.w #$2, $72(a6)
pub fn i_0854e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0854EE: lea.l $854f4(pc), a1
pub fn i_0854ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x854F4;
    16
}
/// $0854F2: move.l a1, (a6)
pub fn i_0854f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0854F4: jsr $2783a.l
pub fn i_0854f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854FA);
    cpu.pc = 0x2783A;
    16
}
/// $0854FA: lea.l $2e96ec.l, a0
pub fn i_0854fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85500;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E96EC;
    20
}
/// $085500: move.l a0, $4c(a6)
pub fn i_085500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85504;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085504: jsr $283ca.l
pub fn i_085504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8550A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8550A);
    cpu.pc = 0x283CA;
    16
}
/// $08550A: jsr $283ca.l
pub fn i_08550a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85510);
    cpu.pc = 0x283CA;
    16
}
/// $085510: jsr $283d8.l
pub fn i_085510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85516);
    cpu.pc = 0x283D8;
    16
}
/// $085516: subq.w #$1, $72(a6)
pub fn i_085516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8551A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08551A: bpl.w $85524
pub fn i_08551a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8551E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x85524; return 10; }
    8
}
/// $08551E: jmp $518.l
pub fn i_08551e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85524;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085524: rts
pub fn i_085524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85526;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_TurretMount_085526  ($085526, 94 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085526: lea.l $2e7a10.l, a0
pub fn i_085526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8552C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7A10;
    20
}
/// $08552C: jsr $28cd4.l
pub fn i_08552c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85532);
    cpu.pc = 0x28CD4;
    16
}
/// $085532: lea.l $85538(pc), a1
pub fn i_085532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85536;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85538;
    16
}
/// $085536: move.l a1, (a6)
pub fn i_085536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85538;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085538: movea.l $c(a6), a0
pub fn i_085538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8553C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08553C: move.w $22(a0), $22(a6)
pub fn i_08553c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85542;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $085542: move.w $24(a0), $24(a6)
pub fn i_085542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85548;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $085548: jsr $28d70.l
pub fn i_085548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8554E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8554E);
    cpu.pc = 0x28D70;
    16
}
/// $08554E: jsr $2870a.l
pub fn i_08554e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85554;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85554);
    cpu.pc = 0x2870A;
    16
}
/// $085554: bcc.w $8556a
pub fn i_085554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85558;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8556A; return 10; }
    8
}
/// $085558: lea.l $5e766.l, a0
pub fn i_085558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8555E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08555E: jsr $5e798.l
pub fn i_08555e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85564);
    cpu.pc = 0x5E798;
    16
}
/// $085564: bclr.b #$3, $13(a6)
pub fn i_085564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8556A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08556A: movea.l $c(a6), a0
pub fn i_08556a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8556E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08556E: movea.l $c(a0), a0
pub fn i_08556e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85572;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085572: cmpi.b #$ff, $21(a0)
pub fn i_085572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85578;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085578: bne.w $85582
pub fn i_085578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8557C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85582; return 10; }
    8
}
/// $08557C: jmp $518.l
pub fn i_08557c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85582;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085582: rts
pub fn i_085582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85584;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SoldierDropper_085584  ($085584, 126 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085584: move.w #$180, d0
pub fn i_085584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85588;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085588: move.w #$258, d1
pub fn i_085588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8558C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08558C: jsr $44022.l
pub fn i_08558c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85592;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85592);
    cpu.pc = 0x44022;
    16
}
/// $085592: move.w d0, $22(a6)
pub fn i_085592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85596;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085596: move.w d1, $24(a6)
pub fn i_085596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8559A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08559A: move.w #$0, $72(a6)
pub fn i_08559a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0855A0: lea.l $855a6(pc), a1
pub fn i_0855a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x855A6;
    16
}
/// $0855A4: move.l a1, (a6)
pub fn i_0855a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0855A6: jsr $2783a.l
pub fn i_0855a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855AC);
    cpu.pc = 0x2783A;
    16
}
/// $0855AC: movea.l $c(a6), a0
pub fn i_0855ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855B0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0855B0: cmpi.b #$ff, $20(a0)
pub fn i_0855b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855B6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0855B6: bne.w $855c2
pub fn i_0855b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x855C2; return 10; }
    8
}
/// $0855BA: jmp $518.l
pub fn i_0855ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855C0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0855C0: rts
pub fn i_0855c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0855C2: jsr $6f0.l
pub fn i_0855c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855C8);
    cpu.pc = 0x6F0;
    16
}
/// $0855C8: bcs.w $85606
pub fn i_0855c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85606; return 10; }
    8
}
/// $0855CC: cmpi.w #$0, $72(a6)
pub fn i_0855cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0855D2: ble.w $855de
pub fn i_0855d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855D6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x855DE; return 10; }
    8
}
/// $0855D6: subq.w #$1, $72(a6)
pub fn i_0855d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855DA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0855DA: bra.w $85606
pub fn i_0855da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x85606;
    10
}
/// $0855DE: lea.l $483e2.l, a1
pub fn i_0855de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x483E2;
    20
}
/// $0855E4: jsr $4ae.l
pub fn i_0855e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855EA);
    cpu.pc = 0x4AE;
    16
}
/// $0855EA: jsr $5dd22.l
pub fn i_0855ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0855F0: move.b #$0, $11(a0)
pub fn i_0855f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855F6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0855F6: lea.l $2c0420.l, a0
pub fn i_0855f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0420;
    20
}
/// $0855FC: jsr $799de.l
pub fn i_0855fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85602);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_085602  ($085602, 6 B, kernel/task_setters_w.c)
/// $085602: move.w d0, $72(a6)
pub fn i_085602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85606;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085606: rts
pub fn i_085606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85608;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SoldierDropperB_085608  ($085608, 156 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085608: move.w #$180, d0
pub fn i_085608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8560C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08560C: move.w #$2ff, d1
pub fn i_08560c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85610;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085610: jsr $44022.l
pub fn i_085610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85616);
    cpu.pc = 0x44022;
    16
}
/// $085616: move.w d0, $22(a6)
pub fn i_085616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8561A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08561A: move.w d1, $24(a6)
pub fn i_08561a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8561E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08561E: move.w #$0, $72(a6)
pub fn i_08561e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85624;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085624: lea.l $8562a(pc), a1
pub fn i_085624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85628;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8562A;
    16
}
/// $085628: move.l a1, (a6)
pub fn i_085628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8562A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08562A: jsr $2783a.l
pub fn i_08562a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85630);
    cpu.pc = 0x2783A;
    16
}
/// $085630: movea.l $c(a6), a0
pub fn i_085630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85634;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085634: cmpi.b #$ff, $20(a0)
pub fn i_085634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8563A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08563A: bne.w $85646
pub fn i_08563a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8563E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85646; return 10; }
    8
}
/// $08563E: jmp $518.l
pub fn i_08563e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85644;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085644: rts
pub fn i_085644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85646;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $085646: jsr $6f0.l
pub fn i_085646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8564C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8564C);
    cpu.pc = 0x6F0;
    16
}
/// $08564C: bcs.w $856a8
pub fn i_08564c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85650;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x856A8; return 10; }
    8
}
/// $085650: cmpi.w #$0, $72(a6)
pub fn i_085650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85656;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085656: ble.w $85662
pub fn i_085656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8565A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85662; return 10; }
    8
}
/// $08565A: subq.w #$1, $72(a6)
pub fn i_08565a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8565E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08565E: bra.w $856a8
pub fn i_08565e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85662;
    cpu.ir = 0x6000;
    cpu.pc = 0x856A8;
    10
}
/// $085662: lea.l $483e2.l, a1
pub fn i_085662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85668;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x483E2;
    20
}
/// $085668: jsr $4ae.l
pub fn i_085668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8566E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8566E);
    cpu.pc = 0x4AE;
    16
}
/// $08566E: jsr $5dd22.l
pub fn i_08566e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85674;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85674);
    cpu.pc = 0x5DD22;
    16
}
/// $085674: move.b #$0, $11(a0)
pub fn i_085674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8567A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08567A: move.b #$f, $98(a0)
pub fn i_08567a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85680;
    cpu.ir = 0x117C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085680: move.b #$0, $99(a0)
pub fn i_085680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85686;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085686: move.b #$0, $9a(a0)
pub fn i_085686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8568C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08568C: move.b #$1, $9b(a0)
pub fn i_08568c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85692;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085692: move.b #$1e, $9c(a0)
pub fn i_085692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85698;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085698: lea.l $2c0420.l, a0
pub fn i_085698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8569E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0420;
    20
}
/// $08569E: jsr $799de.l
pub fn i_08569e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856A4);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_0856a4  ($0856A4, 6 B, kernel/task_setters_w.c)
/// $0856A4: move.w d0, $72(a6)
pub fn i_0856a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0856A8: rts
pub fn i_0856a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Wheel_0856aa  ($0856AA, 144 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0856AA: move.w #$1d0, d1
pub fn i_0856aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0856AE: jsr $236e.l
pub fn i_0856ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856B4);
    cpu.pc = 0x236E;
    16
}
/// $0856B4: move.w #$1d1, d1
pub fn i_0856b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0856B8: jsr $236e.l
pub fn i_0856b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856BE);
    cpu.pc = 0x236E;
    16
}
/// $0856BE: addi.w #$30, $22(a6)
pub fn i_0856be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856C4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0856C4: addi.w #$61, $24(a6)
pub fn i_0856c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856CA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x61, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0856CA: clr.l d0
pub fn i_0856ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856CC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0856CC: move.b d0, $88(a6)
pub fn i_0856cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856D0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0856D0: movea.l #$2e7bde, a0
pub fn i_0856d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856D6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BDE;
    cpu.a[0] = v1;
    4
}
/// $0856D6: lsl.w #$2, d0
pub fn i_0856d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856D8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0856D8: movea.l (a0, d0.w), a0
pub fn i_0856d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856DC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0856DC: cmpa.l #$ffffffff, a0
pub fn i_0856dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856E2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0856E2: beq.w $856ec
pub fn i_0856e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x856EC; return 10; }
    8
}
/// $0856E6: jsr $28cd4.l
pub fn i_0856e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856EC);
    cpu.pc = 0x28CD4;
    16
}
/// $0856EC: move.w #$0, $38(a6)
pub fn i_0856ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0856F2: lea.l $856f8(pc), a1
pub fn i_0856f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x856F8;
    16
}
/// $0856F6: move.l a1, (a6)
pub fn i_0856f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0856F8: jsr $2783a.l
pub fn i_0856f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856FE);
    cpu.pc = 0x2783A;
    16
}
/// $0856FE: move.w $16(a6), $14(a6)
pub fn i_0856fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85704;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $085704: movea.l $c(a6), a0
pub fn i_085704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85708;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085708: movea.l $c(a0), a0
pub fn i_085708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8570C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08570C: cmpi.b #$ff, $89(a0)
pub fn i_08570c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85712;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085712: bne.w $8571c
pub fn i_085712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85716;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8571C; return 10; }
    8
}
/// $085716: move.w $18(a6), $14(a6)
pub fn i_085716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8571C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08571C: jsr $85c32(pc)
pub fn i_08571c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85720;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85720);
    cpu.pc = 0x85C32;
    16
}
/// $085720: movea.l $c(a6), a0
pub fn i_085720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85724;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085724: movea.l $c(a0), a0
pub fn i_085724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85728;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085728: cmpi.b #$ff, $20(a0)
pub fn i_085728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8572E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08572E: bne.w $85738
pub fn i_08572e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85732;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85738; return 10; }
    8
}
/// $085732: jmp $518.l
pub fn i_085732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85738;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085738: rts
pub fn i_085738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8573A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SplashFx_08573a  ($08573A, 64 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08573A: move.w #$1b3, d1
pub fn i_08573a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8573E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08573E: jsr $236e.l
pub fn i_08573e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85744);
    cpu.pc = 0x236E;
    16
}
/// $085744: lea.l $2e7950.l, a0
pub fn i_085744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8574A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7950;
    20
}
/// $08574A: jsr $28cd4.l
pub fn i_08574a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85750);
    cpu.pc = 0x28CD4;
    16
}
/// $085750: move.w #$0, $38(a6)
pub fn i_085750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85756;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085756: jsr $267e2.l
pub fn i_085756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8575C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8575C);
    cpu.pc = 0x267E2;
    16
}
/// $08575C: lea.l $85762(pc), a1
pub fn i_08575c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85760;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85762;
    16
}
/// $085760: move.l a1, (a6)
pub fn i_085760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85762;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085762: jsr $2783a.l
pub fn i_085762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85768;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85768);
    cpu.pc = 0x2783A;
    16
}
/// $085768: jsr $28d70.l
pub fn i_085768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8576E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8576E);
    cpu.pc = 0x28D70;
    16
}
/// $08576E: bcc.w $85778
pub fn i_08576e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85772;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85778; return 10; }
    8
}
/// $085772: jmp $518.l
pub fn i_085772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85778;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085778: rts
pub fn i_085778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8577A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Rail_SpawnRow_08577a  ($08577A, 56 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08577A: clr.b d0
pub fn i_08577a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8577C;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08577C: movem.w d0, -(a7)
pub fn i_08577c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85780;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $085780: lea.l $857b4(pc), a1
pub fn i_085780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x857B4;
    16
}
/// $085784: jsr $4ae.l
pub fn i_085784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8578A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8578A);
    cpu.pc = 0x4AE;
    16
}
/// $08578A: jsr $5dd22.l
pub fn i_08578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85790);
    cpu.pc = 0x5DD22;
    16
}
/// $085790: movem.w (a7)+, d0
pub fn i_085790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85794;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085794: move.b d0, $21(a0)
pub fn i_085794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85798;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085798: addi.w #$10, $22(a6)
pub fn i_085798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8579E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08579E: addq.b #$1, d0
pub fn i_08579e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857A0;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0857A0: cmpi.b #$14, d0
pub fn i_0857a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857A4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x14, Size::Byte, false);
    8
}
/// $0857A4: blt.b $8577c
pub fn i_0857a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857A6;
    cpu.ir = 0x6DD6;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8577C; return 10; }
    8
}
/// $0857A6: lea.l $857ac(pc), a1
pub fn i_0857a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x857AC;
    16
}
/// $0857AA: move.l a1, (a6)
pub fn i_0857aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0857AC: jmp $518.l
pub fn i_0857ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857B2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0857b2  ($0857B2, 2 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0857B2: rts
pub fn i_0857b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Rail_Segment_0857b4  ($0857B4, 62 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0857B4: lea.l $857ba(pc), a1
pub fn i_0857b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x857BA;
    16
}
/// $0857B8: move.l a1, (a6)
pub fn i_0857b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0857BA: jsr $2783a.l
pub fn i_0857ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857C0);
    cpu.pc = 0x2783A;
    16
}
/// $0857C0: movea.l #$ffffffff, a0
pub fn i_0857c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0857C6: lea.l $2e986a.l, a0
pub fn i_0857c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E986A;
    20
}
/// $0857CC: jsr $5dd5c.l
pub fn i_0857cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857D2);
    cpu.pc = 0x5DD5C;
    16
}
/// $0857D2: bcs.w $857ea
pub fn i_0857d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x857EA; return 10; }
    8
}
/// $0857D6: lea.l $2eb104.l, a0
pub fn i_0857d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB104;
    20
}
/// $0857DC: jsr $5e086.l
pub fn i_0857dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857E2);
    cpu.pc = 0x5E086;
    16
}
/// $0857E2: bcs.w $857f0
pub fn i_0857e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857E6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x857F0; return 10; }
    8
}
/// $0857E6: jsr $85d04(pc)
pub fn i_0857e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x857EA);
    cpu.pc = 0x85D04;
    16
}
/// $0857EA: jmp $518.l
pub fn i_0857ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0857F0: rts
pub fn i_0857f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Water_Spawn2_0857f2  ($0857F2, 44 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0857F2: lea.l $85820(pc), a1
pub fn i_0857f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85820;
    16
}
/// $0857F6: jsr $4ae.l
pub fn i_0857f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857FC);
    cpu.pc = 0x4AE;
    16
}
/// $0857FC: jsr $5dd22.l
pub fn i_0857fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85802);
    cpu.pc = 0x5DD22;
    16
}
/// $085802: lea.l $8582a(pc), a1
pub fn i_085802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85806;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8582A;
    16
}
/// $085806: jsr $4ae.l
pub fn i_085806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8580C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8580C);
    cpu.pc = 0x4AE;
    16
}
/// $08580C: jsr $5dd22.l
pub fn i_08580c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85812);
    cpu.pc = 0x5DD22;
    16
}
/// $085812: lea.l $85818(pc), a1
pub fn i_085812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85818;
    16
}
/// $085816: move.l a1, (a6)
pub fn i_085816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85818;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085818: jmp $518.l
pub fn i_085818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8581E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08581e  ($08581E, 2 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08581E: rts
pub fn i_08581e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85820;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Water_InitP1_085820  ($085820, 10 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085820: move.b #$0, $20(a6)
pub fn i_085820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85826;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085826: bra.w $85830
pub fn i_085826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8582A;
    cpu.ir = 0x6000;
    cpu.pc = 0x85830;
    10
}
// ===== M4_Water_InitP2_08582a  ($08582A, 166 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08582A: move.b #$1, $20(a6)
pub fn i_08582a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85830;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085830: lea.l $2e79fa.l, a0
pub fn i_085830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85836;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E79FA;
    20
}
/// $085836: jsr $28cd4.l
pub fn i_085836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8583C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8583C);
    cpu.pc = 0x28CD4;
    16
}
/// $08583C: lea.l $85842(pc), a1
pub fn i_08583c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85840;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85842;
    16
}
/// $085840: move.l a1, (a6)
pub fn i_085840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85842;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085842: jsr $2783a.l
pub fn i_085842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85848;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85848);
    cpu.pc = 0x2783A;
    16
}
/// $085848: jsr $28d70.l
pub fn i_085848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8584E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8584E);
    cpu.pc = 0x28D70;
    16
}
/// $08584E: movea.l #$ffffffff, a0
pub fn i_08584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85854;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $085854: lea.l $2e986a.l, a0
pub fn i_085854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8585A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E986A;
    20
}
/// $08585A: jsr $5dd5c.l
pub fn i_08585a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85860);
    cpu.pc = 0x5DD5C;
    16
}
/// $085860: bcc.w $8586c
pub fn i_085860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85864;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8586C; return 10; }
    8
}
/// $085864: jmp $518.l
pub fn i_085864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8586A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08586A: rts
pub fn i_08586a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8586C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08586C: jsr $6f0.l
pub fn i_08586c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85872;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85872);
    cpu.pc = 0x6F0;
    16
}
/// $085872: bcs.w $858ce
pub fn i_085872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85876;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x858CE; return 10; }
    8
}
/// $085876: lea.l $100440.l, a0
pub fn i_085876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8587C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $08587C: movea.l a0, a4
pub fn i_08587c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8587E;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $08587E: cmpi.b #$0, $20(a6)
pub fn i_08587e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85884;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $085884: beq.w $85890
pub fn i_085884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85888;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85890; return 10; }
    8
}
/// $085888: lea.l $1004e0.l, a0
pub fn i_085888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8588E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $08588E: movea.l a0, a4
pub fn i_08588e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85890;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $085890: lea.l $2eb10e.l, a1
pub fn i_085890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85896;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EB10E;
    20
}
/// $085896: jsr $5e260.l
pub fn i_085896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8589C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8589C);
    cpu.pc = 0x5E260;
    16
}
/// $08589C: bcs.w $858ce
pub fn i_08589c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858A0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x858CE; return 10; }
    8
}
/// $0858A0: cmpi.w #$0, $28(a4)
pub fn i_0858a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858A6;
    cpu.ir = 0xC6C;
    let ad2 = cpu.a[4].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0858A6: beq.w $858ce
pub fn i_0858a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x858CE; return 10; }
    8
}
/// $0858AA: movem.l a4, -(a7)
pub fn i_0858aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858AE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0858AE: lea.l $8573a(pc), a1
pub fn i_0858ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8573A;
    16
}
/// $0858B2: jsr $4ae.l
pub fn i_0858b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x858B8);
    cpu.pc = 0x4AE;
    16
}
/// $0858B8: movem.l (a7)+, a4
pub fn i_0858b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858BC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0858BC: move.w $22(a4), $22(a0)
pub fn i_0858bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858C2;
    cpu.ir = 0x316C;
    let ad2 = cpu.a[4].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0858C2: move.w $24(a4), d0
pub fn i_0858c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858C6;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0858C6: subi.w #$8, d0
pub fn i_0858c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858CA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0858CA: move.w d0, $24(a0)
pub fn i_0858ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858CE;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0858CE: rts
pub fn i_0858ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Debris_Init_0858d0  ($0858D0, 34 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0858D0: move.w #$1c9, d1
pub fn i_0858d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0858D4: jsr $236e.l
pub fn i_0858d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x858DA);
    cpu.pc = 0x236E;
    16
}
/// $0858DA: move.w #$0, $38(a6)
pub fn i_0858da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0858E0: jsr $267e2.l
pub fn i_0858e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x858E6);
    cpu.pc = 0x267E2;
    16
}
/// $0858E6: move.b $99(a6), d0
pub fn i_0858e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0858EA: andi.b #$1, d0
pub fn i_0858ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858EE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0858EE: move.b d0, $3a(a6)
pub fn i_0858ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858F2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== M4_Debris_Idle_0858f2  ($0858F2, 108 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0858F2: clr.l d0
pub fn i_0858f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858F4;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0858F4: move.b $98(a6), d0
pub fn i_0858f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858F8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0858F8: andi.b #$3, d0
pub fn i_0858f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858FC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0858FC: movea.l #$2e7b2e, a0
pub fn i_0858fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85902;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7B2E;
    cpu.a[0] = v1;
    4
}
/// $085902: lsl.w #$2, d0
pub fn i_085902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85904;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085904: movea.l (a0, d0.w), a0
pub fn i_085904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85908;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085908: cmpa.l #$ffffffff, a0
pub fn i_085908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8590E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08590E: beq.w $85918
pub fn i_08590e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85912;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85918; return 10; }
    8
}
/// $085912: jsr $28cd4.l
pub fn i_085912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85918);
    cpu.pc = 0x28CD4;
    16
}
/// $085918: lea.l $8591e(pc), a1
pub fn i_085918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8591C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8591E;
    16
}
/// $08591C: move.l a1, (a6)
pub fn i_08591c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8591E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08591E: jsr $2783a.l
pub fn i_08591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85924);
    cpu.pc = 0x2783A;
    16
}
/// $085924: jsr $28d70.l
pub fn i_085924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8592A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8592A);
    cpu.pc = 0x28D70;
    16
}
/// $08592A: lea.l $2eb118.l, a0
pub fn i_08592a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85930;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB118;
    20
}
/// $085930: jsr $5e086.l
pub fn i_085930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85936);
    cpu.pc = 0x5E086;
    16
}
/// $085936: bcs.w $85940
pub fn i_085936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8593A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85940; return 10; }
    8
}
/// $08593A: lea.l $8595e(pc), a1
pub fn i_08593a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8593E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8595E;
    16
}
/// $08593E: move.l a1, (a6)
pub fn i_08593e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85940;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085940: movea.l #$ffffffff, a0
pub fn i_085940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85946;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $085946: lea.l $2e9874.l, a0
pub fn i_085946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8594C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9874;
    20
}
/// $08594C: jsr $5dd5c.l
pub fn i_08594c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85952);
    cpu.pc = 0x5DD5C;
    16
}
/// $085952: bcc.w $8595c
pub fn i_085952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85956;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8595C; return 10; }
    8
}
/// $085956: jmp $518.l
pub fn i_085956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8595C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08595C: rts
pub fn i_08595c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8595E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Debris_Hit_08595e  ($08595E, 118 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08595E: clr.l d0
pub fn i_08595e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85960;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085960: move.b $98(a6), d0
pub fn i_085960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85964;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085964: andi.b #$3, d0
pub fn i_085964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85968;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085968: movea.l #$2e7b3e, a0
pub fn i_085968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8596E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7B3E;
    cpu.a[0] = v1;
    4
}
/// $08596E: lsl.w #$2, d0
pub fn i_08596e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85970;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085970: movea.l (a0, d0.w), a0
pub fn i_085970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85974;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085974: cmpa.l #$ffffffff, a0
pub fn i_085974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8597A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08597A: beq.w $85984
pub fn i_08597a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8597E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85984; return 10; }
    8
}
/// $08597E: jsr $28cd4.l
pub fn i_08597e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85984);
    cpu.pc = 0x28CD4;
    16
}
/// $085984: lea.l $8598a(pc), a1
pub fn i_085984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85988;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8598A;
    16
}
/// $085988: move.l a1, (a6)
pub fn i_085988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8598A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08598A: jsr $2783a.l
pub fn i_08598a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85990);
    cpu.pc = 0x2783A;
    16
}
/// $085990: jsr $28d70.l
pub fn i_085990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85996);
    cpu.pc = 0x28D70;
    16
}
/// $085996: bcc.w $859b6
pub fn i_085996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8599A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x859B6; return 10; }
    8
}
/// $08599A: lea.l $8595e(pc), a1
pub fn i_08599a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8599E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8595E;
    16
}
/// $08599E: move.l a1, (a6)
pub fn i_08599e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0859A0: lea.l $2eb118.l, a0
pub fn i_0859a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB118;
    20
}
/// $0859A6: jsr $5e086.l
pub fn i_0859a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x859AC);
    cpu.pc = 0x5E086;
    16
}
/// $0859AC: bcc.w $859b6
pub fn i_0859ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x859B6; return 10; }
    8
}
/// $0859B0: lea.l $858f2(pc), a1
pub fn i_0859b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x858F2;
    16
}
/// $0859B4: move.l a1, (a6)
pub fn i_0859b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0859B6: movea.l #$ffffffff, a0
pub fn i_0859b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859BC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0859BC: lea.l $2e9874.l, a0
pub fn i_0859bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9874;
    20
}
/// $0859C2: jsr $5dd5c.l
pub fn i_0859c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x859C8);
    cpu.pc = 0x5DD5C;
    16
}
/// $0859C8: bcc.w $859d2
pub fn i_0859c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x859D2; return 10; }
    8
}
/// $0859CC: jmp $518.l
pub fn i_0859cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859D2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0859D2: rts
pub fn i_0859d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_CamFloor_Init_0859d4  ($0859D4, 52 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0859D4: move.b #$0, $20(a6)
pub fn i_0859d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0859DA: move.w #$430, $10816a.l
pub fn i_0859da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859E2;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x430;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v1 as u16);
    16
}
/// $0859E2: move.w #$430, $10816e.l
pub fn i_0859e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859EA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x430;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v1 as u16);
    16
}
/// $0859EA: move.w #$a0, $22(a6)
pub fn i_0859ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0859F0: btst.b #$0, $100001.l
pub fn i_0859f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859F8;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0859F8: beq.w $85a08
pub fn i_0859f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85A08; return 10; }
    8
}
/// $0859FC: lea.l $2e7bfe.l, a0
pub fn i_0859fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7BFE;
    20
}
/// $085A02: jsr $28cd4.l
pub fn i_085a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A08);
    cpu.pc = 0x28CD4;
    16
}
// ===== M4_CamFloor_Step_085a08  ($085A08, 190 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085A08: clr.l d0
pub fn i_085a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A0A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085A0A: move.b $20(a6), d0
pub fn i_085a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A0E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085A0E: asl.l #$1, d0
pub fn i_085a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A10;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $085A10: lea.l $ec882.l, a0
pub fn i_085a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xEC882;
    20
}
/// $085A16: move.w (a0, d0.w), d1
pub fn i_085a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A1A;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $085A1A: cmpi.w #$ffff, d1
pub fn i_085a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A1E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $085A1E: bne.w $85a2a
pub fn i_085a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A22;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85A2A; return 10; }
    8
}
/// $085A22: jmp $518.l
pub fn i_085a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A28;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085A28: rts
pub fn i_085a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $085A2A: jsr $44022.l
pub fn i_085a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A30);
    cpu.pc = 0x44022;
    16
}
/// $085A30: move.w d1, $24(a6)
pub fn i_085a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A34;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A34: lea.l $85a3a(pc), a1
pub fn i_085a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85A3A;
    16
}
/// $085A38: move.l a1, (a6)
pub fn i_085a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085A3A: move.w $22(a6), d0
pub fn i_085a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A3E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085A3E: move.w $24(a6), d1
pub fn i_085a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A42;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085A42: movem.w d0, -(a7)
pub fn i_085a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A46;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $085A46: movem.w d1, -(a7)
pub fn i_085a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A4A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $085A4A: move.w #$0, $22(a6)
pub fn i_085a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A50: move.w #$fc, $24(a6)
pub fn i_085a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A56: lea.l $2e97e4.l, a0
pub fn i_085a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A5C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E97E4;
    20
}
/// $085A5C: move.l a0, $4c(a6)
pub fn i_085a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A60;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085A60: jsr $283ca.l
pub fn i_085a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A66);
    cpu.pc = 0x283CA;
    16
}
/// $085A66: jsr $283ca.l
pub fn i_085a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A6C);
    cpu.pc = 0x283CA;
    16
}
/// $085A6C: jsr $283d8.l
pub fn i_085a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A72);
    cpu.pc = 0x283D8;
    16
}
/// $085A72: movem.w (a7)+, d1
pub fn i_085a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A76;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085A76: movem.w (a7)+, d0
pub fn i_085a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A7A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085A7A: move.w d0, $22(a6)
pub fn i_085a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A7E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A7E: move.w d1, $24(a6)
pub fn i_085a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A82;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A82: jsr $2783a.l
pub fn i_085a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A88);
    cpu.pc = 0x2783A;
    16
}
/// $085A88: btst.b #$0, $100001.l
pub fn i_085a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A90;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $085A90: beq.w $85a9a
pub fn i_085a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85A9A; return 10; }
    8
}
/// $085A94: jsr $28d70.l
pub fn i_085a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A9A);
    cpu.pc = 0x28D70;
    16
}
/// $085A9A: jsr $85ace(pc)
pub fn i_085a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85A9E);
    cpu.pc = 0x85ACE;
    16
}
/// $085A9E: bcc.w $85acc
pub fn i_085a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85ACC; return 10; }
    8
}
/// $085AA2: move.w $24(a6), d1
pub fn i_085aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AA6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085AA6: jsr $4400e.l
pub fn i_085aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85AAC);
    cpu.pc = 0x4400E;
    16
}
/// $085AAC: subi.w #$100, d1
pub fn i_085aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AB0;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085AB0: bpl.w $85ab6
pub fn i_085ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AB4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x85AB6; return 10; }
    8
}
/// $085AB4: clr.w d1
pub fn i_085ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AB6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085AB6: move.w d1, $10816a.l
pub fn i_085ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ABC;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v1 as u16);
    16
}
/// $085ABC: move.w d1, $10816e.l
pub fn i_085abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AC2;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v1 as u16);
    16
}
/// $085AC2: addq.b #$1, $20(a6)
pub fn i_085ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AC6;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetTaskHandler_085ac6  ($085AC6, 8 B, kernel/task_handlers.c)
/// $085AC6: lea.l $85a08(pc), a1
pub fn i_085ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ACA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85A08;
    16
}
/// $085ACA: move.l a1, (a6)
pub fn i_085aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ACC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085ACC: rts
pub fn i_085acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ACE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_PlayersBelowY_085ace  ($085ACE, 142 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085ACE: jsr $5e55c.l
pub fn i_085ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85AD4);
    cpu.pc = 0x5E55C;
    16
}
/// $085AD4: cmpi.b #$0, d0
pub fn i_085ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AD8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $085AD8: beq.w $85b62
pub fn i_085ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ADC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085ADC: cmpi.b #$1, d0
pub fn i_085adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AE0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $085AE0: bne.w $85b02
pub fn i_085ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85B02; return 10; }
    8
}
/// $085AE4: lea.l $100440.l, a0
pub fn i_085ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085AEA: jsr $85b68(pc)
pub fn i_085aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85AEE);
    cpu.pc = 0x85B68;
    16
}
/// $085AEE: bcc.w $85b62
pub fn i_085aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085AF2: move.w $24(a0), d1
pub fn i_085af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AF6;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085AF6: cmp.w $24(a6), d1
pub fn i_085af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AFA;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085AFA: blt.w $85b62
pub fn i_085afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AFE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085AFE: bra.w $85b5c
pub fn i_085afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B02;
    cpu.ir = 0x6000;
    cpu.pc = 0x85B5C;
    10
}
/// $085B02: cmpi.b #$2, d0
pub fn i_085b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B06;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $085B06: bne.w $85b28
pub fn i_085b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85B28; return 10; }
    8
}
/// $085B0A: lea.l $1004e0.l, a0
pub fn i_085b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $085B10: jsr $85b68(pc)
pub fn i_085b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85B14);
    cpu.pc = 0x85B68;
    16
}
/// $085B14: bcc.w $85b62
pub fn i_085b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B18: move.w $24(a0), d1
pub fn i_085b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B1C;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085B1C: cmp.w $24(a6), d1
pub fn i_085b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B20;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085B20: blt.w $85b62
pub fn i_085b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B24;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B24: bra.w $85b5c
pub fn i_085b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B28;
    cpu.ir = 0x6000;
    cpu.pc = 0x85B5C;
    10
}
/// $085B28: lea.l $100440.l, a0
pub fn i_085b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085B2E: jsr $85b68(pc)
pub fn i_085b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85B32);
    cpu.pc = 0x85B68;
    16
}
/// $085B32: bcc.w $85b62
pub fn i_085b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B36: move.w $24(a0), d1
pub fn i_085b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B3A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085B3A: cmp.w $24(a6), d1
pub fn i_085b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B3E;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085B3E: blt.w $85b62
pub fn i_085b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B42;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B42: lea.l $1004e0.l, a0
pub fn i_085b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $085B48: jsr $85b68(pc)
pub fn i_085b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85B4C);
    cpu.pc = 0x85B68;
    16
}
/// $085B4C: bcc.w $85b62
pub fn i_085b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B50: move.w $24(a0), d1
pub fn i_085b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B54;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085B54: cmp.w $24(a6), d1
pub fn i_085b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B58;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085B58: blt.w $85b62
pub fn i_085b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B5C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
// ===== SetXN_085b5c  ($085B5C, 6 B, thunks/ccr_helpers.c)
/// $085B5C: ori.b #$11, ccr
pub fn i_085b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B60;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085B60: rts
pub fn i_085b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085b62  ($085B62, 6 B, thunks/ccr_helpers.c)
/// $085B62: andi.b #$ee, ccr
pub fn i_085b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B66;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085B66: rts
pub fn i_085b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SlotAlive_085b68  ($085B68, 10 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085B68: jsr $32e08.l
pub fn i_085b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85B6E);
    cpu.pc = 0x32E08;
    16
}
/// $085B6E: bcs.w $85b78
pub fn i_085b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B72;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85B78; return 10; }
    8
}
// ===== SetXN_085b72  ($085B72, 6 B, thunks/ccr_helpers.c)
/// $085B72: ori.b #$11, ccr
pub fn i_085b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B76;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085B76: rts
pub fn i_085b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085b78  ($085B78, 6 B, thunks/ccr_helpers.c)
/// $085B78: andi.b #$ee, ccr
pub fn i_085b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B7C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085B7C: rts
pub fn i_085b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SetupSprites4_085b7e  ($085B7E, 82 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085B7E: move.w #$4e, d1
pub fn i_085b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B82;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B82: move.w #$1f8, d2
pub fn i_085b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B86;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B86: move.w #$ffff, d3
pub fn i_085b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B8A;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B8A: move.w #$1, d4
pub fn i_085b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B8E;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B8E: jsr $2c26.l
pub fn i_085b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85B94);
    cpu.pc = 0x2C26;
    16
}
/// $085B94: move.w #$4f, d1
pub fn i_085b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B98;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B98: move.w #$1f9, d2
pub fn i_085b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B9C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B9C: move.w #$ffff, d3
pub fn i_085b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BA0;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BA0: move.w #$1, d4
pub fn i_085ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BA4;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BA4: jsr $2c26.l
pub fn i_085ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85BAA);
    cpu.pc = 0x2C26;
    16
}
/// $085BAA: move.w #$50, d1
pub fn i_085baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BAE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BAE: move.w #$1fa, d2
pub fn i_085bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BB2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1FA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BB2: move.w #$ffff, d3
pub fn i_085bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BB6;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BB6: move.w #$1, d4
pub fn i_085bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BBA;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BBA: jsr $2c26.l
pub fn i_085bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85BC0);
    cpu.pc = 0x2C26;
    16
}
/// $085BC0: move.w #$51, d1
pub fn i_085bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BC4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x51;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BC4: move.w #$1fb, d2
pub fn i_085bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BC8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1FB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BC8: move.w #$ffff, d3
pub fn i_085bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BCC;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BCC: move.w #$1, d4
pub fn i_085bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BD0;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_085bd0  ($085BD0, 8 B, thunks/jsr_abs_thunks.c)
/// $085BD0: jsr $2c26.l
pub fn i_085bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85BD6);
    cpu.pc = 0x2C26;
    16
}
/// $085BD6: rts
pub fn i_085bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SetupSprites4B_085bd8  ($085BD8, 82 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085BD8: move.w #$4e, d1
pub fn i_085bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BDC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BDC: move.w #$1f7, d2
pub fn i_085bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BE0;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BE0: move.w #$ffff, d3
pub fn i_085be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BE4;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BE4: move.w #$1, d4
pub fn i_085be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BE8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BE8: jsr $2c26.l
pub fn i_085be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85BEE);
    cpu.pc = 0x2C26;
    16
}
/// $085BEE: move.w #$4f, d1
pub fn i_085bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BF2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BF2: move.w #$1fc, d2
pub fn i_085bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BF6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BF6: move.w #$ffff, d3
pub fn i_085bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BFA;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BFA: move.w #$1, d4
pub fn i_085bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BFE;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BFE: jsr $2c26.l
pub fn i_085bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C04);
    cpu.pc = 0x2C26;
    16
}
/// $085C04: move.w #$50, d1
pub fn i_085c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C08;
    cpu.ir = 0x323C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085C08: move.w #$1fd, d2
pub fn i_085c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C0C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085C0C: move.w #$ffff, d3
pub fn i_085c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C10;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085C10: move.w #$1, d4
pub fn i_085c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C14;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085C14: jsr $2c26.l
pub fn i_085c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C1A);
    cpu.pc = 0x2C26;
    16
}
/// $085C1A: move.w #$51, d1
pub fn i_085c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C1E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x51;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085C1E: move.w #$1fe, d2
pub fn i_085c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C22;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1FE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085C22: move.w #$ffff, d3
pub fn i_085c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C26;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085C26: move.w #$1, d4
pub fn i_085c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C2A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_085c2a  ($085C2A, 8 B, thunks/jsr_abs_thunks.c)
/// $085C2A: jsr $2c26.l
pub fn i_085c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C30);
    cpu.pc = 0x2C26;
    16
}
/// $085C30: rts
pub fn i_085c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Wheel_Anim_085c32  ($085C32, 202 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085C32: movea.l $c(a6), a0
pub fn i_085c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C36;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085C36: cmpi.w #$0, $2a(a0)
pub fn i_085c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C3C;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085C3C: beq.w $85cd6
pub fn i_085c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85CD6; return 10; }
    8
}
/// $085C40: clr.l d0
pub fn i_085c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C42;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085C42: move.w $2a(a0), d0
pub fn i_085c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C46;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085C46: btst.b #$f, d0
pub fn i_085c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C4A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085C4A: bne.w $85c86
pub fn i_085c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85C86; return 10; }
    8
}
/// $085C4E: move.b $88(a6), d0
pub fn i_085c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C52;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085C52: subq.b #$1, d0
pub fn i_085c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C54;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $085C54: andi.b #$3, d0
pub fn i_085c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C58;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085C58: move.b d0, $88(a6)
pub fn i_085c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085C5C: clr.l d0
pub fn i_085c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C5E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085C5E: move.b $88(a6), d0
pub fn i_085c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C62;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085C62: andi.b #$3, d0
pub fn i_085c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C66;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085C66: movea.l #$2e7bee, a0
pub fn i_085c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C6C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BEE;
    cpu.a[0] = v1;
    4
}
/// $085C6C: lsl.w #$2, d0
pub fn i_085c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C6E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085C6E: movea.l (a0, d0.w), a0
pub fn i_085c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C72;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085C72: cmpa.l #$ffffffff, a0
pub fn i_085c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C78;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $085C78: beq.w $85c82
pub fn i_085c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C7C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85C82; return 10; }
    8
}
/// $085C7C: jsr $28cd4.l
pub fn i_085c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C82);
    cpu.pc = 0x28CD4;
    16
}
/// $085C82: bra.w $85cfc
pub fn i_085c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C86;
    cpu.ir = 0x6000;
    cpu.pc = 0x85CFC;
    10
}
/// $085C86: jsr $28d70.l
pub fn i_085c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C8C);
    cpu.pc = 0x28D70;
    16
}
/// $085C8C: bcc.w $85d02
pub fn i_085c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85D02; return 10; }
    8
}
/// $085C90: move.b $88(a6), d0
pub fn i_085c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C94;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085C94: addq.b #$1, d0
pub fn i_085c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C96;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $085C96: andi.b #$3, d0
pub fn i_085c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C9A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085C9A: move.b d0, $88(a6)
pub fn i_085c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C9E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085C9E: clr.l d0
pub fn i_085c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CA0;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085CA0: move.b $88(a6), d0
pub fn i_085ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085CA4: andi.b #$3, d0
pub fn i_085ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CA8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085CA8: movea.l #$2e7bde, a0
pub fn i_085ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CAE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BDE;
    cpu.a[0] = v1;
    4
}
/// $085CAE: lsl.w #$2, d0
pub fn i_085cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CB0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085CB0: movea.l (a0, d0.w), a0
pub fn i_085cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CB4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085CB4: cmpa.l #$ffffffff, a0
pub fn i_085cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CBA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $085CBA: beq.w $85cc4
pub fn i_085cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CBE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85CC4; return 10; }
    8
}
/// $085CBE: jsr $28cd4.l
pub fn i_085cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85CC4);
    cpu.pc = 0x28CD4;
    16
}
/// $085CC4: bra.w $85d02
pub fn i_085cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x85D02;
    10
}
/// $085CC8: move.b $88(a6), d0
pub fn i_085cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CCC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085CCC: addq.b #$1, d0
pub fn i_085ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CCE;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $085CCE: andi.b #$3, d0
pub fn i_085cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CD2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085CD2: move.b d0, $88(a6)
pub fn i_085cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CD6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085CD6: clr.l d0
pub fn i_085cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CD8;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085CD8: move.b $88(a6), d0
pub fn i_085cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CDC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085CDC: andi.b #$3, d0
pub fn i_085cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CE0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085CE0: movea.l #$2e7bde, a0
pub fn i_085ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CE6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BDE;
    cpu.a[0] = v1;
    4
}
/// $085CE6: lsl.w #$2, d0
pub fn i_085ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CE8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085CE8: movea.l (a0, d0.w), a0
pub fn i_085ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CEC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085CEC: cmpa.l #$ffffffff, a0
pub fn i_085cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CF2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $085CF2: beq.w $85cfc
pub fn i_085cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85CFC; return 10; }
    8
}
/// $085CF6: jsr $28cd4.l
pub fn i_085cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85CFC);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_085cfc  ($085CFC, 8 B, thunks/jsr_abs_thunks.c)
/// $085CFC: jsr $28d70.l
pub fn i_085cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D02);
    cpu.pc = 0x28D70;
    16
}
/// $085D02: rts
pub fn i_085d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Rail_Blit_085d04  ($085D04, 30 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085D04: lea.l $2eaa10.l, a0
pub fn i_085d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA10;
    20
}
/// $085D0A: clr.l d0
pub fn i_085d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D0C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085D0C: move.b $21(a6), d0
pub fn i_085d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D10;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085D10: cmpi.b #$14, d0
pub fn i_085d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D14;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x14, Size::Byte, false);
    8
}
/// $085D14: blt.w $85d1c
pub fn i_085d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D18;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85D1C; return 10; }
    8
}
/// $085D18: move.b #$0, d0
pub fn i_085d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D1C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $085D1C: asl.l #$2, d0
pub fn i_085d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D1E;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $085D1E: movea.l (a0, d0.w), a2
pub fn i_085d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D22;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_085d22  ($085D22, 8 B, thunks/jsr_abs_thunks.c)
/// $085D22: jsr $5022a.l
pub fn i_085d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D28);
    cpu.pc = 0x5022A;
    16
}
/// $085D28: rts
pub fn i_085d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_085d2a  ($085D2A, 8 B, thunks/jsr_abs_thunks.c)
/// $085D2A: jsr $8b558.l
pub fn i_085d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D30);
    cpu.pc = 0x8B558;
    16
}
/// $085D30: rts
pub fn i_085d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_EdgeCheck_085d32  ($085D32, 106 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085D32: move.w $22(a6), d0
pub fn i_085d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D36;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085D36: move.w $24(a6), d1
pub fn i_085d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D3A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085D3A: jsr $4400e.l
pub fn i_085d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D40);
    cpu.pc = 0x4400E;
    16
}
/// $085D40: clr.l d0
pub fn i_085d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D42;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085D42: move.w $2a(a6), d0
pub fn i_085d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085D46: btst.b #$f, d0
pub fn i_085d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D4A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085D4A: bne.w $85d5a
pub fn i_085d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85D5A; return 10; }
    8
}
/// $085D4E: cmpi.w #$2d0, d1
pub fn i_085d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D52;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2D0, Size::Word, false);
    8
}
/// $085D52: bgt.w $85da2
pub fn i_085d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D56;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85DA2; return 10; }
    8
}
/// $085D56: bra.w $85d9c
pub fn i_085d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x85D9C;
    10
}
/// $085D5A: cmpi.w #$310, d1
pub fn i_085d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D5E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x310, Size::Word, false);
    8
}
/// $085D5E: blt.w $85da2
pub fn i_085d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D62;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85DA2; return 10; }
    8
}
/// $085D62: lea.l $2e99a8.l, a1
pub fn i_085d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E99A8;
    20
}
/// $085D68: jsr $77c7e.l
pub fn i_085d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D6E);
    cpu.pc = 0x77C7E;
    16
}
/// $085D6E: subi.w #$8, $24(a0)
pub fn i_085d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D74;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $085D74: lea.l $2e99ba.l, a1
pub fn i_085d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D7A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E99BA;
    20
}
/// $085D7A: jsr $77c7e.l
pub fn i_085d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D80);
    cpu.pc = 0x77C7E;
    16
}
/// $085D80: subi.w #$8, $38(a0)
pub fn i_085d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D86;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $085D86: move.b #$a, $75(a6)
pub fn i_085d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D8C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085D8C: lea.l $854e8(pc), a1
pub fn i_085d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x854E8;
    16
}
/// $085D90: jsr $4ae.l
pub fn i_085d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D96);
    cpu.pc = 0x4AE;
    16
}
/// $085D96: jsr $5dd02.l
pub fn i_085d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D9C);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetXN_085d9c  ($085D9C, 6 B, thunks/ccr_helpers.c)
/// $085D9C: ori.b #$11, ccr
pub fn i_085d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085DA0: rts
pub fn i_085da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085da2  ($085DA2, 6 B, thunks/ccr_helpers.c)
/// $085DA2: andi.b #$ee, ccr
pub fn i_085da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085DA6: rts
pub fn i_085da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Knockback_085da8  ($085DA8, 102 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085DA8: lea.l $2e94a2.l, a0
pub fn i_085da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E94A2;
    20
}
/// $085DAE: move.l a0, $48(a6)
pub fn i_085dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DB2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085DB2: move.w #$7fff, $66(a6)
pub fn i_085db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085DB8: jsr $2870a.l
pub fn i_085db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DBE);
    cpu.pc = 0x2870A;
    16
}
/// $085DBE: bcc.w $85e14
pub fn i_085dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85E14; return 10; }
    8
}
/// $085DC2: lea.l $2c031c.l, a0
pub fn i_085dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C031C;
    20
}
/// $085DC8: jsr $799de.l
pub fn i_085dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DCE);
    cpu.pc = 0x799DE;
    16
}
/// $085DCE: move.w d0, $2a(a6)
pub fn i_085dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DD2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085DD2: lea.l $2e791e.l, a0
pub fn i_085dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E791E;
    20
}
/// $085DD8: jsr $28cd4.l
pub fn i_085dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DDE);
    cpu.pc = 0x28CD4;
    16
}
/// $085DDE: bclr.b #$3, $13(a6)
pub fn i_085dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DE4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $085DE4: move.w #$108d, d0
pub fn i_085de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DE8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085DE8: jsr $2352.l
pub fn i_085de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DEE);
    cpu.pc = 0x2352;
    16
}
/// $085DEE: move.w $22(a6), d0
pub fn i_085dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085DF2: move.w $24(a6), d1
pub fn i_085df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DF6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085DF6: jsr $4400e.l
pub fn i_085df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DFC);
    cpu.pc = 0x4400E;
    16
}
/// $085DFC: cmpi.w #$2d0, d1
pub fn i_085dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E00;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2D0, Size::Word, false);
    8
}
/// $085E00: ble.w $85e2a
pub fn i_085e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E04;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85E2A; return 10; }
    8
}
/// $085E04: move.b $78(a6), d0
pub fn i_085e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085E08: asl.b #$1, d0
pub fn i_085e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E0A;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $085E0A: bclr.b #$0, d0
pub fn i_085e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E0E;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
// ===== SetTaskB_085e0e  ($085E0E, 6 B, kernel/task_setters_b.c)
/// $085E0E: move.b d0, $78(a6)
pub fn i_085e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E12;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085E12: rts
pub fn i_085e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Recoil_085e14  ($085E14, 52 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085E14: move.w $22(a6), d0
pub fn i_085e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E18;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085E18: move.w $24(a6), d1
pub fn i_085e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E1C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085E1C: jsr $4400e.l
pub fn i_085e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85E22);
    cpu.pc = 0x4400E;
    16
}
/// $085E22: cmpi.w #$310, d1
pub fn i_085e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E26;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x310, Size::Word, false);
    8
}
/// $085E26: bge.w $85e4e
pub fn i_085e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E2A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x85E4E; return 10; }
    8
}
/// $085E2A: lea.l $2c039e.l, a0
pub fn i_085e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C039E;
    20
}
/// $085E30: jsr $799de.l
pub fn i_085e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85E36);
    cpu.pc = 0x799DE;
    16
}
/// $085E36: move.w d0, d0
pub fn i_085e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E38;
    cpu.ir = 0x3000;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085E38: neg.w d0
pub fn i_085e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E3A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085E3A: move.w d0, $2a(a6)
pub fn i_085e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E3E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085E3E: move.b $78(a6), d0
pub fn i_085e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E42;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085E42: asl.b #$1, d0
pub fn i_085e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E44;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $085E44: bclr.b #$0, d0
pub fn i_085e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E48;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
// ===== SetTaskB_085e48  ($085E48, 6 B, kernel/task_setters_b.c)
/// $085E48: move.b d0, $78(a6)
pub fn i_085e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E4C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085E4C: rts
pub fn i_085e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_RecoilStop_085e4e  ($085E4E, 22 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085E4E: move.b $78(a6), d0
pub fn i_085e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E52;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085E52: asl.b #$1, d0
pub fn i_085e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E54;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $085E54: bset.b #$0, d0
pub fn i_085e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E58;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $085E58: move.b d0, $78(a6)
pub fn i_085e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085E5C: move.w #$0, $2a(a6)
pub fn i_085e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085E62: rts
pub fn i_085e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_PickTargetSlot_085e64  ($085E64, 76 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085E64: jsr $5e1aa.l
pub fn i_085e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85E6A);
    cpu.pc = 0x5E1AA;
    16
}
/// $085E6A: bcs.w $85eb6
pub fn i_085e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E6E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85EB6; return 10; }
    8
}
/// $085E6E: cmpi.w #$1, d0
pub fn i_085e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E72;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $085E72: bne.w $85e80
pub fn i_085e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85E80; return 10; }
    8
}
/// $085E76: lea.l $100440.l, a0
pub fn i_085e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085E7C: bra.w $85eb0
pub fn i_085e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E80;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EB0;
    10
}
/// $085E80: cmpi.w #$2, d0
pub fn i_085e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E84;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $085E84: bne.w $85e92
pub fn i_085e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E88;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85E92; return 10; }
    8
}
/// $085E88: lea.l $1004e0.l, a0
pub fn i_085e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $085E8E: bra.w $85eb0
pub fn i_085e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E92;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EB0;
    10
}
/// $085E92: move.b $106f28.l, d0
pub fn i_085e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E98;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $085E98: btst.b #$7, d0
pub fn i_085e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E9C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085E9C: bne.w $85eaa
pub fn i_085e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85EAA; return 10; }
    8
}
/// $085EA0: lea.l $100440.l, a0
pub fn i_085ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085EA6: bra.w $85eb0
pub fn i_085ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EB0;
    10
}
/// $085EAA: lea.l $1004e0.l, a0
pub fn i_085eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
// ===== SetXN_085eb0  ($085EB0, 6 B, thunks/ccr_helpers.c)
/// $085EB0: ori.b #$11, ccr
pub fn i_085eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EB4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085EB4: rts
pub fn i_085eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085eb6  ($085EB6, 6 B, thunks/ccr_helpers.c)
/// $085EB6: andi.b #$ee, ccr
pub fn i_085eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EBA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085EBA: rts
pub fn i_085eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Shake_085ebc  ($085EBC, 44 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085EBC: cmpi.b #$0, $75(a6)
pub fn i_085ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EC2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $085EC2: ble.w $85ee6
pub fn i_085ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EC6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85EE6; return 10; }
    8
}
/// $085EC6: subq.b #$1, $75(a6)
pub fn i_085ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ECA;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $085ECA: move.w #$1, d1
pub fn i_085eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ECE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085ECE: move.b $75(a6), d0
pub fn i_085ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ED2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085ED2: btst.b #$0, d0
pub fn i_085ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ED6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085ED6: bne.w $85ee2
pub fn i_085ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85EE2; return 10; }
    8
}
/// $085EDA: add.w d1, $24(a6)
pub fn i_085eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EDE;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $085EDE: bra.w $85ee6
pub fn i_085ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EE6;
    10
}
/// $085EE2: sub.w d1, $24(a6)
pub fn i_085ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EE6;
    cpu.ir = 0x936E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $085EE6: rts
pub fn i_085ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_WobbleArm_085ee8  ($085EE8, 32 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085EE8: cmpi.w #$0, $72(a6)
pub fn i_085ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EEE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085EEE: bgt.w $85f06
pub fn i_085eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EF2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85F06; return 10; }
    8
}
/// $085EF2: jsr $5e9b6.l
pub fn i_085ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85EF8);
    cpu.pc = 0x5E9B6;
    16
}
/// $085EF8: andi.w #$1f, d0
pub fn i_085ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EFC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085EFC: bne.w $85f06
pub fn i_085efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85F06; return 10; }
    8
}
/// $085F00: move.w #$2, $72(a6)
pub fn i_085f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085F06: rts
pub fn i_085f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_WobbleStep_085f08  ($085F08, 60 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F08: cmpi.w #$0, $72(a6)
pub fn i_085f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F0E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085F0E: ble.w $85f42
pub fn i_085f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F12;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85F42; return 10; }
    8
}
/// $085F12: subq.w #$1, $72(a6)
pub fn i_085f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F16;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $085F16: move.w #$100, d1
pub fn i_085f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F1A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085F1A: move.w $72(a6), d0
pub fn i_085f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F1E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F1E: btst.b #$0, d0
pub fn i_085f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F22;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085F22: bne.w $85f28
pub fn i_085f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85F28; return 10; }
    8
}
/// $085F26: neg.w d1
pub fn i_085f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F28;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085F28: move.w $2a(a6), d2
pub fn i_085f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F2C;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F2C: movem.w d2, -(a7)
pub fn i_085f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F30;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    cpu.a[7] = a;
    20
}
/// $085F30: move.w d1, $2a(a6)
pub fn i_085f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F34;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085F34: jsr $27cee.l
pub fn i_085f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85F3A);
    cpu.pc = 0x27CEE;
    16
}
/// $085F3A: movem.w (a7)+, d2
pub fn i_085f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F3E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085F3E: move.w d2, $2a(a6)
pub fn i_085f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F42;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085F42: rts
pub fn i_085f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_HitboxParams_085f44  ($085F44, 20 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F44: move.w $22(a6), d0
pub fn i_085f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F48;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F48: subi.w #$60, d0
pub fn i_085f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F4C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x60, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085F4C: move.w $24(a6), d1
pub fn i_085f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F50;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F50: subi.w #$0, d1
pub fn i_085f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F54;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x0, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085F54: move.w #$c0, d2
pub fn i_085f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F58;
    cpu.ir = 0x343C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_085f58  ($085F58, 8 B, thunks/jsr_abs_thunks.c)
/// $085F58: jsr $99812.l
pub fn i_085f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85F5E);
    cpu.pc = 0x99812;
    16
}
/// $085F5E: rts
pub fn i_085f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_AltitudeCheck_085f60  ($085F60, 50 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F60: move.w $22(a6), d0
pub fn i_085f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F64: move.w $24(a6), d1
pub fn i_085f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F68;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F68: jsr $44022.l
pub fn i_085f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85F6E);
    cpu.pc = 0x44022;
    16
}
/// $085F6E: clr.w d2
pub fn i_085f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F70;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085F70: clr.w d3
pub fn i_085f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F72;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085F72: move.b $98(a6), d2
pub fn i_085f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F76;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085F76: move.b $99(a6), d3
pub fn i_085f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F7A;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085F7A: asl.w #$4, d2
pub fn i_085f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F7C;
    cpu.ir = 0xE942;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $085F7C: asl.w #$4, d3
pub fn i_085f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F7E;
    cpu.ir = 0xE943;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $085F7E: clr.l d4
pub fn i_085f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F80;
    cpu.ir = 0x4284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = r2;
    4
}
/// $085F80: move.w $2a(a6), d4
pub fn i_085f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F84;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F84: btst.b #$f, d4
pub fn i_085f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F88;
    cpu.ir = 0x804;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085F88: beq.w $85f9e
pub fn i_085f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85F9E; return 10; }
    8
}
/// $085F8C: cmp.w d1, d3
pub fn i_085f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F8E;
    cpu.ir = 0xB641;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $085F8E: ble.w $85f98
pub fn i_085f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F92;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85F98; return 10; }
    8
}
// ===== SetXN_085f92  ($085F92, 6 B, thunks/ccr_helpers.c)
/// $085F92: ori.b #$11, ccr
pub fn i_085f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F96;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085F96: rts
pub fn i_085f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085f98  ($085F98, 6 B, thunks/ccr_helpers.c)
/// $085F98: andi.b #$ee, ccr
pub fn i_085f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F9C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085F9C: rts
pub fn i_085f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_AltitudeCheckUp_085f9e  ($085F9E, 6 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F9E: cmp.w d1, d2
pub fn i_085f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FA0;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $085FA0: bge.w $85faa
pub fn i_085fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FA4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x85FAA; return 10; }
    8
}
// ===== ClearXN_085fa4  ($085FA4, 6 B, thunks/ccr_helpers.c)
/// $085FA4: andi.b #$ee, ccr
pub fn i_085fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FA8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085FA8: rts
pub fn i_085fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_085faa  ($085FAA, 6 B, thunks/ccr_helpers.c)
/// $085FAA: ori.b #$11, ccr
pub fn i_085faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FAE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085FAE: rts
pub fn i_085fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_RandomDropSpawn_085fb0  ($085FB0, 108 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085FB0: jsr $5e9b6.l
pub fn i_085fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FB6);
    cpu.pc = 0x5E9B6;
    16
}
/// $085FB6: andi.b #$7, d0
pub fn i_085fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FBA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085FBA: bne.w $8601a
pub fn i_085fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8601A; return 10; }
    8
}
/// $085FBE: move.w #$2, d0
pub fn i_085fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FC2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085FC2: jsr $5e9e4.l
pub fn i_085fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FC8);
    cpu.pc = 0x5E9E4;
    16
}
/// $085FC8: addi.w #$1, d0
pub fn i_085fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FCC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085FCC: cmpi.w #$0, d0
pub fn i_085fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FD0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $085FD0: beq.w $8601a
pub fn i_085fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8601A; return 10; }
    8
}
/// $085FD4: movem.w d0, -(a7)
pub fn i_085fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FD8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $085FD8: lea.l $3fec6.l, a1
pub fn i_085fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FDE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3FEC6;
    20
}
/// $085FDE: jsr $4ae.l
pub fn i_085fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FE4);
    cpu.pc = 0x4AE;
    16
}
/// $085FE4: jsr $5dd22.l
pub fn i_085fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FEA);
    cpu.pc = 0x5DD22;
    16
}
/// $085FEA: movem.l a0, -(a7)
pub fn i_085fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FEE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $085FEE: move.w #$200, $24(a0)
pub fn i_085fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FF4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085FF4: move.w #$30, d0
pub fn i_085ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085FF8: jsr $5e9e4.l
pub fn i_085ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FFE);
    cpu.pc = 0x5E9E4;
    16
}
/// $085FFE: btst.b #$0, d0
pub fn i_085ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86002;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $086002: beq.w $86008
pub fn i_086002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86006;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86008; return 10; }
    8
}
/// $086006: neg.w d0
pub fn i_086006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86008;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $086008: movem.l (a7)+, a0
pub fn i_086008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8600C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08600C: add.w d0, $22(a0)
pub fn i_08600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86010;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $086010: movem.w (a7)+, d0
pub fn i_086010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86014;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $086014: subi.w #$1, d0
pub fn i_086014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86018;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $086018: bra.b $85fcc
pub fn i_086018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8601A;
    cpu.ir = 0x60B2;
    cpu.pc = 0x85FCC;
    10
}
/// $08601A: rts
pub fn i_08601a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8601C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_PhaseJingle_08601c  ($08601C, 24 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08601C: move.b $21(a6), d0
pub fn i_08601c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86020;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086020: cmpi.b #$1, d0
pub fn i_086020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86024;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $086024: beq.w $86030
pub fn i_086024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86028;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86030; return 10; }
    8
}
/// $086028: cmpi.b #$5, d0
pub fn i_086028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8602C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $08602C: bne.w $8603c
pub fn i_08602c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86030;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8603C; return 10; }
    8
}
/// $086030: move.w #$1026, d0
pub fn i_086030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86034;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1026;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_086034  ($086034, 8 B, thunks/jsr_abs_thunks.c)
/// $086034: jsr $2352.l
pub fn i_086034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8603A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8603A);
    cpu.pc = 0x2352;
    16
}
/// $08603A: rts
pub fn i_08603a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8603C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_PhaseJingle8_08603c  ($08603C, 12 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08603C: cmpi.b #$8, d0
pub fn i_08603c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86040;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $086040: bne.w $8604e
pub fn i_086040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86044;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8604E; return 10; }
    8
}
/// $086044: move.w #$1032, d0
pub fn i_086044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86048;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_086048  ($086048, 8 B, thunks/jsr_abs_thunks.c)
/// $086048: jsr $2352.l
pub fn i_086048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8604E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8604E);
    cpu.pc = 0x2352;
    16
}
/// $08604E: rts
pub fn i_08604e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86050;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SineBob_086050  ($086050, 30 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086050: addi.w #$8, $34(a6)
pub fn i_086050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86056;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086056: andi.w #$ff, $34(a6)
pub fn i_086056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8605C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08605C: move.w $34(a6), d0
pub fn i_08605c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86060;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $086060: move.w #$80, d1
pub fn i_086060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86064;
    cpu.ir = 0x323C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086064: jsr $13c0e.l
pub fn i_086064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8606A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8606A);
    cpu.pc = 0x13C0E;
    16
}
/// $08606A: move.w d2, $2a(a6)
pub fn i_08606a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8606E;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_08606e  ($08606E, 8 B, thunks/jsr_abs_thunks.c)
/// $08606E: jsr $27cee.l
pub fn i_08606e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86074);
    cpu.pc = 0x27CEE;
    16
}
/// $086074: rts
pub fn i_086074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86076;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnGuardList_086076  ($086076, 110 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086076: lea.l $2eaf1c.l, a1
pub fn i_086076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8607C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAF1C;
    20
}
/// $08607C: cmpi.b #$b, $106ece.l
pub fn i_08607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86084;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xB, Size::Byte, false);
    20
}
/// $086084: bne.w $8608e
pub fn i_086084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86088;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8608E; return 10; }
    8
}
/// $086088: lea.l $2eaf7e.l, a1
pub fn i_086088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8608E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAF7E;
    20
}
/// $08608E: clr.l d0
pub fn i_08608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86090;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086090: movem.l a1, -(a7)
pub fn i_086090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86094;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086094: movem.l d0, -(a7)
pub fn i_086094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86098;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086098: lea.l $83c02(pc), a1
pub fn i_086098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8609C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83C02;
    16
}
/// $08609C: jsr $6fe.l
pub fn i_08609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860A2);
    cpu.pc = 0x6FE;
    16
}
/// $0860A2: jsr $5dd02.l
pub fn i_0860a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860A8);
    cpu.pc = 0x5DD02;
    16
}
/// $0860A8: movem.l (a7)+, d0
pub fn i_0860a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860AC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0860AC: movem.l (a7)+, a1
pub fn i_0860ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860B0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0860B0: move.w (a1, d0.w), d1
pub fn i_0860b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860B4;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0860B4: add.w d1, $22(a0)
pub fn i_0860b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860B8;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0860B8: move.w $2(a1, d0.w), d1
pub fn i_0860b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860BC;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0860BC: add.w d1, $24(a0)
pub fn i_0860bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860C0;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0860C0: move.w $4(a1, d0.w), $38(a0)
pub fn i_0860c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860C6;
    cpu.ir = 0x3171;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0860C6: move.b $6(a1, d0.w), $21(a0)
pub fn i_0860c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860CC;
    cpu.ir = 0x1171;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    22
}
/// $0860CC: move.b $7(a1, d0.w), d1
pub fn i_0860cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860D0;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x7).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0860D0: or.b d1, $3a(a0)
pub fn i_0860d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860D4;
    cpu.ir = 0x8328;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0860D4: addi.l #$8, d0
pub fn i_0860d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860DA;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $0860DA: cmpi.w #$ffff, (a1, d0.w)
pub fn i_0860da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E0;
    cpu.ir = 0xC71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $0860E0: bne.b $86090
pub fn i_0860e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E2;
    cpu.ir = 0x66AE;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86090; return 10; }
    8
}
/// $0860E2: rts
pub fn i_0860e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_Spawn45Children_0860e4  ($0860E4, 40 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0860E4: clr.b d0
pub fn i_0860e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E6;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0860E6: movem.w d0, -(a7)
pub fn i_0860e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860EA;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $0860EA: lea.l $845ec(pc), a1
pub fn i_0860ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845EC;
    16
}
/// $0860EE: jsr $6fe.l
pub fn i_0860ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860F4);
    cpu.pc = 0x6FE;
    16
}
/// $0860F4: jsr $5dd02.l
pub fn i_0860f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860FA);
    cpu.pc = 0x5DD02;
    16
}
/// $0860FA: movem.w (a7)+, d0
pub fn i_0860fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860FE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0860FE: move.b d0, $21(a0)
pub fn i_0860fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86102;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086102: addq.b #$1, d0
pub fn i_086102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86104;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086104: cmpi.b #$2d, d0
pub fn i_086104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86108;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2D, Size::Byte, false);
    8
}
/// $086108: blt.b $860e6
pub fn i_086108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8610A;
    cpu.ir = 0x6DDC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x860E6; return 10; }
    8
}
/// $08610A: rts
pub fn i_08610a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8610C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnStepList_08610c  ($08610C, 138 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08610C: lea.l $2eafe2.l, a1
pub fn i_08610c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86112;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAFE2;
    20
}
/// $086112: clr.l d0
pub fn i_086112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86114;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086114: cmpi.l #$38, d0
pub fn i_086114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8611A;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x38, Size::Long, false);
    8
}
/// $08611A: bge.w $86142
pub fn i_08611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8611E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x86142; return 10; }
    8
}
/// $08611E: movem.l a1, -(a7)
pub fn i_08611e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86122;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086122: movem.l d0, -(a7)
pub fn i_086122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86126;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086126: lea.l $84836(pc), a1
pub fn i_086126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8612A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84836;
    16
}
/// $08612A: jsr $6fe.l
pub fn i_08612a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86130);
    cpu.pc = 0x6FE;
    16
}
/// $086130: jsr $5dd02.l
pub fn i_086130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86136);
    cpu.pc = 0x5DD02;
    16
}
/// $086136: movem.l (a7)+, d0
pub fn i_086136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8613A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08613A: movem.l (a7)+, a1
pub fn i_08613a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8613E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08613E: bra.w $86162
pub fn i_08613e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86142;
    cpu.ir = 0x6000;
    cpu.pc = 0x86162;
    10
}
/// $086142: movem.l a1, -(a7)
pub fn i_086142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86146;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086146: movem.l d0, -(a7)
pub fn i_086146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8614A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $08614A: lea.l $84836(pc), a1
pub fn i_08614a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8614E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84836;
    16
}
/// $08614E: jsr $4ae.l
pub fn i_08614e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86154);
    cpu.pc = 0x4AE;
    16
}
/// $086154: jsr $5dd22.l
pub fn i_086154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8615A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8615A);
    cpu.pc = 0x5DD22;
    16
}
/// $08615A: movem.l (a7)+, d0
pub fn i_08615a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8615E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08615E: movem.l (a7)+, a1
pub fn i_08615e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86162;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $086162: move.w (a1, d0.w), d1
pub fn i_086162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86166;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $086166: add.w d1, $22(a0)
pub fn i_086166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8616A;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08616A: move.w $2(a1, d0.w), d1
pub fn i_08616a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8616E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08616E: add.w d1, $24(a0)
pub fn i_08616e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86172;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $086172: move.w $4(a1, d0.w), $38(a0)
pub fn i_086172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86178;
    cpu.ir = 0x3171;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    22
}
/// $086178: move.b $6(a1, d0.w), $21(a0)
pub fn i_086178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8617E;
    cpu.ir = 0x1171;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    22
}
/// $08617E: move.b $7(a1, d0.w), d1
pub fn i_08617e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86182;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x7).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $086182: or.b d1, $3a(a0)
pub fn i_086182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86186;
    cpu.ir = 0x8328;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $086186: addi.l #$8, d0
pub fn i_086186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8618C;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08618C: cmpi.w #$ffff, (a1, d0.w)
pub fn i_08618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86192;
    cpu.ir = 0xC71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $086192: bne.b $86114
pub fn i_086192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86194;
    cpu.ir = 0x6680;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86114; return 10; }
    8
}
/// $086194: rts
pub fn i_086194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86196;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnTenEscorts_086196  ($086196, 362 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086196: lea.l $77fd6.l, a1
pub fn i_086196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8619C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $08619C: jsr $4ae.l
pub fn i_08619c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861A2);
    cpu.pc = 0x4AE;
    16
}
/// $0861A2: jsr $5dd22.l
pub fn i_0861a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861A8);
    cpu.pc = 0x5DD22;
    16
}
/// $0861A8: addi.w #$30, $22(a0)
pub fn i_0861a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861AE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861AE: addi.w #$20, $24(a0)
pub fn i_0861ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861B4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861B4: move.w #$c000, $38(a0)
pub fn i_0861b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0861BA: lea.l $77fd6.l, a1
pub fn i_0861ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0861C0: jsr $4ae.l
pub fn i_0861c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861C6);
    cpu.pc = 0x4AE;
    16
}
/// $0861C6: jsr $5dd22.l
pub fn i_0861c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861CC);
    cpu.pc = 0x5DD22;
    16
}
/// $0861CC: addi.w #$20, $22(a0)
pub fn i_0861cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861D2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861D2: addi.w #$50, $24(a0)
pub fn i_0861d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861D8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861D8: move.w #$c000, $38(a0)
pub fn i_0861d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861DE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0861DE: lea.l $77fd6.l, a1
pub fn i_0861de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0861E4: jsr $4ae.l
pub fn i_0861e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861EA);
    cpu.pc = 0x4AE;
    16
}
/// $0861EA: jsr $5dd22.l
pub fn i_0861ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0861F0: addi.w #$40, $22(a0)
pub fn i_0861f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861F6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861F6: addi.w #$70, $24(a0)
pub fn i_0861f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861FC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861FC: move.w #$c000, $38(a0)
pub fn i_0861fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86202;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086202: lea.l $77fd6.l, a1
pub fn i_086202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86208;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086208: jsr $4ae.l
pub fn i_086208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8620E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8620E);
    cpu.pc = 0x4AE;
    16
}
/// $08620E: jsr $5dd22.l
pub fn i_08620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86214);
    cpu.pc = 0x5DD22;
    16
}
/// $086214: addi.w #$60, $22(a0)
pub fn i_086214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8621A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08621A: addi.w #$10, $24(a0)
pub fn i_08621a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86220;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086220: move.w #$c000, $38(a0)
pub fn i_086220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86226;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086226: lea.l $77fd6.l, a1
pub fn i_086226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8622C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $08622C: jsr $4ae.l
pub fn i_08622c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86232);
    cpu.pc = 0x4AE;
    16
}
/// $086232: jsr $5dd22.l
pub fn i_086232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86238);
    cpu.pc = 0x5DD22;
    16
}
/// $086238: addi.w #$80, $22(a0)
pub fn i_086238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8623E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08623E: addi.w #$40, $24(a0)
pub fn i_08623e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86244;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086244: move.w #$c000, $38(a0)
pub fn i_086244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8624A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08624A: lea.l $77fd6.l, a1
pub fn i_08624a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86250;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086250: jsr $4ae.l
pub fn i_086250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86256);
    cpu.pc = 0x4AE;
    16
}
/// $086256: jsr $5dd22.l
pub fn i_086256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8625C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8625C);
    cpu.pc = 0x5DD22;
    16
}
/// $08625C: addi.w #$90, $22(a0)
pub fn i_08625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86262;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086262: addi.w #$70, $24(a0)
pub fn i_086262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86268;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086268: move.w #$c000, $38(a0)
pub fn i_086268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8626E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08626E: lea.l $77fd6.l, a1
pub fn i_08626e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86274;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086274: jsr $4ae.l
pub fn i_086274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8627A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8627A);
    cpu.pc = 0x4AE;
    16
}
/// $08627A: jsr $5dd22.l
pub fn i_08627a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86280);
    cpu.pc = 0x5DD22;
    16
}
/// $086280: addi.w #$b0, $22(a0)
pub fn i_086280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86286;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xB0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086286: addi.w #$20, $24(a0)
pub fn i_086286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8628C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08628C: move.w #$c000, $38(a0)
pub fn i_08628c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86292;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086292: lea.l $77fd6.l, a1
pub fn i_086292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86298;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086298: jsr $4ae.l
pub fn i_086298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8629E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8629E);
    cpu.pc = 0x4AE;
    16
}
/// $08629E: jsr $5dd22.l
pub fn i_08629e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862A4);
    cpu.pc = 0x5DD22;
    16
}
/// $0862A4: addi.w #$d0, $22(a0)
pub fn i_0862a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xD0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862AA: addi.w #$60, $24(a0)
pub fn i_0862aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862B0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862B0: move.w #$c000, $38(a0)
pub fn i_0862b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0862B6: lea.l $77fd6.l, a1
pub fn i_0862b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0862BC: jsr $4ae.l
pub fn i_0862bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862C2);
    cpu.pc = 0x4AE;
    16
}
/// $0862C2: jsr $5dd22.l
pub fn i_0862c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862C8);
    cpu.pc = 0x5DD22;
    16
}
/// $0862C8: addi.w #$e0, $22(a0)
pub fn i_0862c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862CE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862CE: addi.w #$20, $24(a0)
pub fn i_0862ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862D4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862D4: move.w #$c000, $38(a0)
pub fn i_0862d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862DA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0862DA: lea.l $77fd6.l, a1
pub fn i_0862da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862E0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0862E0: jsr $4ae.l
pub fn i_0862e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862E6);
    cpu.pc = 0x4AE;
    16
}
/// $0862E6: jsr $5dd22.l
pub fn i_0862e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862EC);
    cpu.pc = 0x5DD22;
    16
}
/// $0862EC: addi.w #$f0, $22(a0)
pub fn i_0862ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862F2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862F2: addi.w #$70, $24(a0)
pub fn i_0862f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862F8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862F8: move.w #$c000, $38(a0)
pub fn i_0862f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862FE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0862FE: rts
pub fn i_0862fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86300;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_Spawn4Finale_086300  ($086300, 40 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086300: clr.b d0
pub fn i_086300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86302;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086302: movem.w d0, -(a7)
pub fn i_086302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86306;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $086306: lea.l $846e0(pc), a1
pub fn i_086306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8630A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x846E0;
    16
}
/// $08630A: jsr $6fe.l
pub fn i_08630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86310);
    cpu.pc = 0x6FE;
    16
}
/// $086310: jsr $5dd02.l
pub fn i_086310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86316);
    cpu.pc = 0x5DD02;
    16
}
/// $086316: movem.w (a7)+, d0
pub fn i_086316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8631A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08631A: move.b d0, $21(a0)
pub fn i_08631a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8631E;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08631E: addq.b #$1, d0
pub fn i_08631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86320;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086320: cmpi.b #$4, d0
pub fn i_086320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86324;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $086324: blt.b $86302
pub fn i_086324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86326;
    cpu.ir = 0x6DDC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x86302; return 10; }
    8
}
/// $086326: rts
pub fn i_086326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86328;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnRow8_086328  ($086328, 60 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086328: clr.b d0
pub fn i_086328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8632A;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08632A: move.w $22(a6), d1
pub fn i_08632a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8632E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08632E: movem.l d0, -(a7)
pub fn i_08632e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86332;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086332: movem.l d1, -(a7)
pub fn i_086332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86336;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086336: lea.l $83fd0(pc), a1
pub fn i_086336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8633A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83FD0;
    16
}
/// $08633A: jsr $4ae.l
pub fn i_08633a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86340);
    cpu.pc = 0x4AE;
    16
}
/// $086340: jsr $5dd22.l
pub fn i_086340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86346);
    cpu.pc = 0x5DD22;
    16
}
/// $086346: movem.l (a7)+, d1
pub fn i_086346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8634A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08634A: movem.l (a7)+, d0
pub fn i_08634a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8634E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08634E: move.b d0, $21(a0)
pub fn i_08634e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86352;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086352: move.w d1, $22(a0)
pub fn i_086352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86356;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086356: addq.b #$1, d0
pub fn i_086356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86358;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086358: addi.w #$20, d1
pub fn i_086358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8635C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08635C: cmpi.b #$8, d0
pub fn i_08635c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86360;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $086360: blt.b $8632e
pub fn i_086360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86362;
    cpu.ir = 0x6DCC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8632E; return 10; }
    8
}
/// $086362: rts
pub fn i_086362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86364;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnRow9_086364  ($086364, 60 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086364: clr.b d0
pub fn i_086364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86366;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086366: move.w $22(a6), d1
pub fn i_086366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8636A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08636A: movem.l d0, -(a7)
pub fn i_08636a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8636E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $08636E: movem.l d1, -(a7)
pub fn i_08636e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86372;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086372: lea.l $84282(pc), a1
pub fn i_086372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86376;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84282;
    16
}
/// $086376: jsr $4ae.l
pub fn i_086376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8637C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8637C);
    cpu.pc = 0x4AE;
    16
}
/// $08637C: jsr $5dd22.l
pub fn i_08637c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86382);
    cpu.pc = 0x5DD22;
    16
}
/// $086382: movem.l (a7)+, d1
pub fn i_086382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86386;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $086386: movem.l (a7)+, d0
pub fn i_086386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8638A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08638A: move.b d0, $21(a0)
pub fn i_08638a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8638E;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08638E: move.w d1, $22(a0)
pub fn i_08638e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86392;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086392: addq.b #$1, d0
pub fn i_086392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86394;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086394: addi.w #$10, d1
pub fn i_086394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86398;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $086398: cmpi.b #$9, d0
pub fn i_086398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8639C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $08639C: blt.b $8636a
pub fn i_08639c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8639E;
    cpu.ir = 0x6DCC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8636A; return 10; }
    8
}
/// $08639E: rts
pub fn i_08639e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_BobY1_0863a0  ($0863A0, 30 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0863A0: addq.b #$1, $72(a6)
pub fn i_0863a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863A4;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0863A4: btst.b #$0, $72(a6)
pub fn i_0863a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863AA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0863AA: bne.w $863b6
pub fn i_0863aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x863B6; return 10; }
    8
}
/// $0863AE: subi.w #$1, $24(a6)
pub fn i_0863ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863B4;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0863B4: rts
pub fn i_0863b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0863B6: addi.w #$1, $24(a6)
pub fn i_0863b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863BC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0863BC: rts
pub fn i_0863bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TickIfState2_0863be  ($0863BE, 16 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0863BE: cmpi.b #$2, $21(a6)
pub fn i_0863be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0863C4: bne.w $863d4
pub fn i_0863c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x863D4; return 10; }
    8
}
/// $0863C8: jsr $283ca.l
pub fn i_0863c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863CE);
    cpu.pc = 0x283CA;
    16
}
// ===== JsrAbsThunk_0863ce  ($0863CE, 8 B, thunks/jsr_abs_thunks.c)
/// $0863CE: jsr $283d8.l
pub fn i_0863ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863D4);
    cpu.pc = 0x283D8;
    16
}
/// $0863D4: rts
pub fn i_0863d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0863d6  ($0863D6, 14 B, kernel/state_dispatch_stubs.c)
/// $0863D6: lea.l $2e9c8c.l, a2
pub fn i_0863d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863DC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C8C;
    20
}
/// $0863DC: jsr $5022a.l
pub fn i_0863dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863E2);
    cpu.pc = 0x5022A;
    16
}
/// $0863E2: rts
pub fn i_0863e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0863e4  ($0863E4, 14 B, kernel/state_dispatch_stubs.c)
/// $0863E4: lea.l $2e9c50.l, a2
pub fn i_0863e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863EA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C50;
    20
}
/// $0863EA: jsr $5022a.l
pub fn i_0863ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863F0);
    cpu.pc = 0x5022A;
    16
}
/// $0863F0: rts
pub fn i_0863f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0863f2  ($0863F2, 14 B, kernel/state_dispatch_stubs.c)
/// $0863F2: lea.l $2e9c64.l, a2
pub fn i_0863f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863F8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C64;
    20
}
/// $0863F8: jsr $5022a.l
pub fn i_0863f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863FE);
    cpu.pc = 0x5022A;
    16
}
/// $0863FE: rts
pub fn i_0863fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086400  ($086400, 14 B, kernel/state_dispatch_stubs.c)
/// $086400: lea.l $2e9c78.l, a2
pub fn i_086400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86406;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C78;
    20
}
/// $086406: jsr $5022a.l
pub fn i_086406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8640C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8640C);
    cpu.pc = 0x5022A;
    16
}
/// $08640C: rts
pub fn i_08640c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8640E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08640e  ($08640E, 14 B, kernel/state_dispatch_stubs.c)
/// $08640E: lea.l $2e9ca0.l, a2
pub fn i_08640e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86414;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9CA0;
    20
}
/// $086414: jsr $5022a.l
pub fn i_086414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8641A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8641A);
    cpu.pc = 0x5022A;
    16
}
/// $08641A: rts
pub fn i_08641a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8641C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08641c  ($08641C, 14 B, kernel/state_dispatch_stubs.c)
/// $08641C: lea.l $2e9cb4.l, a2
pub fn i_08641c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86422;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9CB4;
    20
}
/// $086422: jsr $5022a.l
pub fn i_086422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86428);
    cpu.pc = 0x5022A;
    16
}
/// $086428: rts
pub fn i_086428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8642A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08642a  ($08642A, 14 B, kernel/state_dispatch_stubs.c)
/// $08642A: lea.l $2e9f20.l, a2
pub fn i_08642a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86430;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9F20;
    20
}
/// $086430: jsr $5022a.l
pub fn i_086430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86436);
    cpu.pc = 0x5022A;
    16
}
/// $086436: rts
pub fn i_086436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86438;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086438  ($086438, 14 B, kernel/state_dispatch_stubs.c)
/// $086438: lea.l $2ea7a4.l, a2
pub fn i_086438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8643E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7A4;
    20
}
/// $08643E: jsr $5022a.l
pub fn i_08643e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86444);
    cpu.pc = 0x5022A;
    16
}
/// $086444: rts
pub fn i_086444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086446  ($086446, 14 B, kernel/state_dispatch_stubs.c)
/// $086446: lea.l $2ea7b8.l, a2
pub fn i_086446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8644C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7B8;
    20
}
/// $08644C: jsr $5022a.l
pub fn i_08644c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86452);
    cpu.pc = 0x5022A;
    16
}
/// $086452: rts
pub fn i_086452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086454  ($086454, 14 B, kernel/state_dispatch_stubs.c)
/// $086454: lea.l $2ea7cc.l, a2
pub fn i_086454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8645A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7CC;
    20
}
/// $08645A: jsr $5022a.l
pub fn i_08645a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86460);
    cpu.pc = 0x5022A;
    16
}
/// $086460: rts
pub fn i_086460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86462;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086462  ($086462, 14 B, kernel/state_dispatch_stubs.c)
/// $086462: lea.l $2ea7e0.l, a2
pub fn i_086462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86468;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7E0;
    20
}
/// $086468: jsr $5022a.l
pub fn i_086468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8646E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8646E);
    cpu.pc = 0x5022A;
    16
}
/// $08646E: rts
pub fn i_08646e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86470;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086470  ($086470, 14 B, kernel/state_dispatch_stubs.c)
/// $086470: lea.l $2ea7f4.l, a2
pub fn i_086470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86476;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7F4;
    20
}
/// $086476: jsr $5022a.l
pub fn i_086476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8647C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8647C);
    cpu.pc = 0x5022A;
    16
}
/// $08647C: rts
pub fn i_08647c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8647E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08647e  ($08647E, 14 B, kernel/state_dispatch_stubs.c)
/// $08647E: lea.l $2ea808.l, a2
pub fn i_08647e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86484;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA808;
    20
}
/// $086484: jsr $5022a.l
pub fn i_086484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8648A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8648A);
    cpu.pc = 0x5022A;
    16
}
/// $08648A: rts
pub fn i_08648a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8648C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08648c  ($08648C, 14 B, kernel/state_dispatch_stubs.c)
/// $08648C: lea.l $2ea81c.l, a2
pub fn i_08648c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86492;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA81C;
    20
}
/// $086492: jsr $5022a.l
pub fn i_086492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86498);
    cpu.pc = 0x5022A;
    16
}
/// $086498: rts
pub fn i_086498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8649A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08649a  ($08649A, 14 B, kernel/state_dispatch_stubs.c)
/// $08649A: lea.l $2ea830.l, a2
pub fn i_08649a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864A0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA830;
    20
}
/// $0864A0: jsr $5022a.l
pub fn i_0864a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864A6);
    cpu.pc = 0x5022A;
    16
}
/// $0864A6: rts
pub fn i_0864a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0864a8  ($0864A8, 14 B, kernel/state_dispatch_stubs.c)
/// $0864A8: lea.l $2ea844.l, a2
pub fn i_0864a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864AE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA844;
    20
}
/// $0864AE: jsr $5022a.l
pub fn i_0864ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864B4);
    cpu.pc = 0x5022A;
    16
}
/// $0864B4: rts
pub fn i_0864b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_A_0864b6  ($0864B6, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0864B6: lea.l $2eaa60.l, a0
pub fn i_0864b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA60;
    20
}
/// $0864BC: clr.l d0
pub fn i_0864bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864BE;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0864BE: move.b $21(a6), d0
pub fn i_0864be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864C2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0864C2: asl.l #$4, d0
pub fn i_0864c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864C4;
    cpu.ir = 0xE980;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, true);
    cpu.d[0] = r;
    14
}
/// $0864C4: movea.l $4(a0, d0.w), a2
pub fn i_0864c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864C8;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_0864c8  ($0864C8, 8 B, thunks/jsr_abs_thunks.c)
/// $0864C8: jsr $5022a.l
pub fn i_0864c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864CE);
    cpu.pc = 0x5022A;
    16
}
/// $0864CE: rts
pub fn i_0864ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_B_0864d0  ($0864D0, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0864D0: lea.l $2eaa60.l, a0
pub fn i_0864d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA60;
    20
}
/// $0864D6: clr.l d0
pub fn i_0864d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864D8;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0864D8: move.b $21(a6), d0
pub fn i_0864d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864DC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0864DC: asl.l #$4, d0
pub fn i_0864dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864DE;
    cpu.ir = 0xE980;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, true);
    cpu.d[0] = r;
    14
}
/// $0864DE: movea.l $c(a0, d0.w), a2
pub fn i_0864de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864E2;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0xC).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_0864e2  ($0864E2, 8 B, thunks/jsr_abs_thunks.c)
/// $0864E2: jsr $5022a.l
pub fn i_0864e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864E8);
    cpu.pc = 0x5022A;
    16
}
/// $0864E8: rts
pub fn i_0864e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_C_0864ea  ($0864EA, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0864EA: lea.l $2eaae0.l, a0
pub fn i_0864ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAAE0;
    20
}
/// $0864F0: clr.l d0
pub fn i_0864f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0864F2: move.b $21(a6), d0
pub fn i_0864f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0864F6: asl.l #$2, d0
pub fn i_0864f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F8;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $0864F8: movea.l (a0, d0.w), a2
pub fn i_0864f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864FC;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_0864fc  ($0864FC, 8 B, thunks/jsr_abs_thunks.c)
/// $0864FC: jsr $5022a.l
pub fn i_0864fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86502);
    cpu.pc = 0x5022A;
    16
}
/// $086502: rts
pub fn i_086502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86504;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_D_086504  ($086504, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086504: lea.l $2eab04.l, a0
pub fn i_086504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8650A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAB04;
    20
}
/// $08650A: clr.l d0
pub fn i_08650a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8650C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08650C: move.b $21(a6), d0
pub fn i_08650c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86510;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086510: asl.l #$2, d0
pub fn i_086510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86512;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $086512: movea.l (a0, d0.w), a2
pub fn i_086512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86516;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_086516  ($086516, 8 B, thunks/jsr_abs_thunks.c)
/// $086516: jsr $5022a.l
pub fn i_086516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8651C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8651C);
    cpu.pc = 0x5022A;
    16
}
/// $08651C: rts
pub fn i_08651c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8651E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_E_08651e  ($08651E, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08651E: lea.l $2eab24.l, a0
pub fn i_08651e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86524;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAB24;
    20
}
/// $086524: clr.l d0
pub fn i_086524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86526;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086526: move.b $21(a6), d0
pub fn i_086526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8652A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08652A: asl.l #$3, d0
pub fn i_08652a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8652C;
    cpu.ir = 0xE780;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, true);
    cpu.d[0] = r;
    12
}
/// $08652C: movea.l (a0, d0.w), a2
pub fn i_08652c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86530;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_086530  ($086530, 8 B, thunks/jsr_abs_thunks.c)
/// $086530: jsr $5022a.l
pub fn i_086530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86536);
    cpu.pc = 0x5022A;
    16
}
/// $086536: rts
pub fn i_086536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86538;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_F_086538  ($086538, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086538: lea.l $2eab24.l, a0
pub fn i_086538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8653E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAB24;
    20
}
/// $08653E: clr.l d0
pub fn i_08653e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86540;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086540: move.b $21(a6), d0
pub fn i_086540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86544;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086544: asl.l #$3, d0
pub fn i_086544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86546;
    cpu.ir = 0xE780;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, true);
    cpu.d[0] = r;
    12
}
/// $086546: movea.l $4(a0, d0.w), a2
pub fn i_086546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8654A;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_08654a  ($08654A, 8 B, thunks/jsr_abs_thunks.c)
/// $08654A: jsr $5022a.l
pub fn i_08654a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86550);
    cpu.pc = 0x5022A;
    16
}
/// $086550: rts
pub fn i_086550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86552;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_086552  ($086552, 16 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086552: movea.l $8(a6), a1
pub fn i_086552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86556;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086556: move.b $10(a6), d0
pub fn i_086556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8655A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08655A: cmp.b $10(a1), d0
pub fn i_08655a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8655E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08655E: bcs.w $86568
pub fn i_08655e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86562;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x86568; return 10; }
    8
}
// ===== ClearXN_086562  ($086562, 6 B, thunks/ccr_helpers.c)
/// $086562: andi.b #$ee, ccr
pub fn i_086562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86566;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $086566: rts
pub fn i_086566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86568;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_086568  ($086568, 6 B, thunks/ccr_helpers.c)
/// $086568: ori.b #$11, ccr
pub fn i_086568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8656C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08656C: rts
pub fn i_08656c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8656E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScreenNuke_Init_08656e  ($08656E, 72 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08656E: lea.l $2ecec4.l, a0
pub fn i_08656e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86574;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECEC4;
    20
}
/// $086574: move.l a0, $4c(a6)
pub fn i_086574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86578;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086578: jsr $283ca.l
pub fn i_086578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8657E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8657E);
    cpu.pc = 0x283CA;
    16
}
/// $08657E: bra.w $86596
pub fn i_08657e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86582;
    cpu.ir = 0x6000;
    cpu.pc = 0x86596;
    10
}
/// $086582: bra.w $86596
pub fn i_086582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86586;
    cpu.ir = 0x6000;
    cpu.pc = 0x86596;
    10
}
/// $086586: lea.l $2ecd80.l, a0
pub fn i_086586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8658C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECD80;
    20
}
/// $08658C: move.l a0, $4c(a6)
pub fn i_08658c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86590;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086590: jsr $283ca.l
pub fn i_086590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86596);
    cpu.pc = 0x283CA;
    16
}
/// $086596: move.w #$0, $22(a6)
pub fn i_086596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8659C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08659C: move.w #$100, $24(a6)
pub fn i_08659c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0865A2: jsr $283ca.l
pub fn i_0865a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865A8);
    cpu.pc = 0x283CA;
    16
}
/// $0865A8: jsr $283d8.l
pub fn i_0865a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865AE);
    cpu.pc = 0x283D8;
    16
}
/// $0865AE: move.b #$1, $10e39e.l
pub fn i_0865ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865B6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
// ===== SetTaskHandler_0865b6  ($0865B6, 8 B, kernel/task_handlers.c)
/// $0865B6: lea.l $865be(pc), a1
pub fn i_0865b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x865BE;
    16
}
/// $0865BA: move.l a1, (a6)
pub fn i_0865ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0865BC: rts
pub fn i_0865bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScreenNuke_Clear_0865be  ($0865BE, 28 B, enemies/bosses/fort_scene4_0865be.s)
/// $0865BE: move.b #$0, $10e39e.l
pub fn i_0865be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865C6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $0865C6: lea.l $ffff.w, a0
pub fn i_0865c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865CA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0865CA: move.l a0, $4c(a6)
pub fn i_0865ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865CE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0865CE: jsr $283ca.l
pub fn i_0865ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865D4);
    cpu.pc = 0x283CA;
    16
}
/// $0865D4: jmp $518.l
pub fn i_0865d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0865da  ($0865DA, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $0865DA: rts
pub fn i_0865da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_Init_V0_0865dc  ($0865DC, 72 B, enemies/bosses/fort_scene4_0865be.s)
/// $0865DC: movea.l $3c(a6), a1
pub fn i_0865dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865E0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0865E0: jsr $2942a.l
pub fn i_0865e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865E6);
    cpu.pc = 0x2942A;
    16
}
/// $0865E6: move.b #$0, $20(a6)
pub fn i_0865e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0865EC: jsr $884a6(pc)
pub fn i_0865ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x865F0);
    cpu.pc = 0x884A6;
    16
}
/// $0865F0: lea.l $2edcb0.l, a1
pub fn i_0865f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDCB0;
    20
}
/// $0865F6: move.l a1, $78(a6)
pub fn i_0865f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865FA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0865FA: lea.l $edc8c.l, a1
pub fn i_0865fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86600;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDC8C;
    20
}
/// $086600: move.l a1, $7c(a6)
pub fn i_086600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86604;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086604: lea.l $86e4a(pc), a1
pub fn i_086604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86608;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86E4A;
    16
}
/// $086608: jsr $4ae.l
pub fn i_086608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8660E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8660E);
    cpu.pc = 0x4AE;
    16
}
/// $08660E: jsr $5dd22.l
pub fn i_08660e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86614;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86614);
    cpu.pc = 0x5DD22;
    16
}
/// $086614: addi.w #$50, $22(a0)
pub fn i_086614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8661A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08661A: addi.w #$10, $24(a0)
pub fn i_08661a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86620;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086620: bra.w $86738
pub fn i_086620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86624;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V1_086624  ($086624, 66 B, enemies/bosses/fort_scene4_0865be.s)
/// $086624: movea.l $3c(a6), a1
pub fn i_086624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86628;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086628: jsr $2942a.l
pub fn i_086628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8662E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8662E);
    cpu.pc = 0x2942A;
    16
}
/// $08662E: move.b #$1, $20(a6)
pub fn i_08662e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86634;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086634: jsr $885b4(pc)
pub fn i_086634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86638;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86638);
    cpu.pc = 0x885B4;
    16
}
/// $086638: lea.l $2edcba.l, a1
pub fn i_086638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8663E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDCBA;
    20
}
/// $08663E: move.l a1, $78(a6)
pub fn i_08663e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86642;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086642: lea.l $edcd2.l, a1
pub fn i_086642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86648;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDCD2;
    20
}
/// $086648: move.l a1, $7c(a6)
pub fn i_086648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8664C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08664C: lea.l $86e74(pc), a1
pub fn i_08664c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86E74;
    16
}
/// $086650: jsr $4ae.l
pub fn i_086650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86656);
    cpu.pc = 0x4AE;
    16
}
/// $086656: jsr $5dd22.l
pub fn i_086656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8665C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8665C);
    cpu.pc = 0x5DD22;
    16
}
/// $08665C: addi.w #$50, $22(a0)
pub fn i_08665c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86662;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086662: bra.w $86738
pub fn i_086662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86666;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V2_086666  ($086666, 72 B, enemies/bosses/fort_scene4_0865be.s)
/// $086666: movea.l $3c(a6), a1
pub fn i_086666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8666A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08666A: jsr $2942a.l
pub fn i_08666a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86670;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86670);
    cpu.pc = 0x2942A;
    16
}
/// $086670: move.b #$2, $20(a6)
pub fn i_086670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86676;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086676: jsr $886c2(pc)
pub fn i_086676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8667A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8667A);
    cpu.pc = 0x886C2;
    16
}
/// $08667A: lea.l $2edcfe.l, a1
pub fn i_08667a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86680;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDCFE;
    20
}
/// $086680: move.l a1, $78(a6)
pub fn i_086680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86684;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086684: lea.l $edd2a.l, a1
pub fn i_086684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8668A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDD2A;
    20
}
/// $08668A: move.l a1, $7c(a6)
pub fn i_08668a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8668E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08668E: lea.l $86e9e(pc), a1
pub fn i_08668e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86692;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86E9E;
    16
}
/// $086692: jsr $4ae.l
pub fn i_086692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86698);
    cpu.pc = 0x4AE;
    16
}
/// $086698: jsr $5dd22.l
pub fn i_086698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8669E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8669E);
    cpu.pc = 0x5DD22;
    16
}
/// $08669E: addi.w #$50, $22(a0)
pub fn i_08669e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866A4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0866A4: addi.w #$10, $24(a0)
pub fn i_0866a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0866AA: bra.w $86738
pub fn i_0866aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V3_0866ae  ($0866AE, 66 B, enemies/bosses/fort_scene4_0865be.s)
/// $0866AE: movea.l $3c(a6), a1
pub fn i_0866ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866B2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0866B2: jsr $2942a.l
pub fn i_0866b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866B8);
    cpu.pc = 0x2942A;
    16
}
/// $0866B8: move.b #$3, $20(a6)
pub fn i_0866b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0866BE: jsr $887d0(pc)
pub fn i_0866be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x866C2);
    cpu.pc = 0x887D0;
    16
}
/// $0866C2: lea.l $2edd0c.l, a1
pub fn i_0866c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866C8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD0C;
    20
}
/// $0866C8: move.l a1, $78(a6)
pub fn i_0866c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866CC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0866CC: lea.l $edda6.l, a1
pub fn i_0866cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDDA6;
    20
}
/// $0866D2: move.l a1, $7c(a6)
pub fn i_0866d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866D6;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0866D6: lea.l $86ec8(pc), a1
pub fn i_0866d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86EC8;
    16
}
/// $0866DA: jsr $4ae.l
pub fn i_0866da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866E0);
    cpu.pc = 0x4AE;
    16
}
/// $0866E0: jsr $5dd22.l
pub fn i_0866e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866E6);
    cpu.pc = 0x5DD22;
    16
}
/// $0866E6: addi.w #$50, $22(a0)
pub fn i_0866e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866EC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0866EC: bra.w $86738
pub fn i_0866ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V4_0866f0  ($0866F0, 228 B, enemies/bosses/fort_scene4_0865be.s)
/// $0866F0: movea.l $3c(a6), a1
pub fn i_0866f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866F4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0866F4: jsr $2942a.l
pub fn i_0866f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866FA);
    cpu.pc = 0x2942A;
    16
}
/// $0866FA: move.b #$4, $20(a6)
pub fn i_0866fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86700;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086700: jsr $8891a(pc)
pub fn i_086700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86704;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86704);
    cpu.pc = 0x8891A;
    16
}
/// $086704: lea.l $2edd20.l, a1
pub fn i_086704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8670A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD20;
    20
}
/// $08670A: move.l a1, $78(a6)
pub fn i_08670a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8670E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08670E: lea.l $ede22.l, a1
pub fn i_08670e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86714;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE22;
    20
}
/// $086714: move.l a1, $7c(a6)
pub fn i_086714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86718;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086718: lea.l $86ef2(pc), a1
pub fn i_086718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8671C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86EF2;
    16
}
/// $08671C: jsr $4ae.l
pub fn i_08671c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86722;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86722);
    cpu.pc = 0x4AE;
    16
}
/// $086722: jsr $5dd22.l
pub fn i_086722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86728);
    cpu.pc = 0x5DD22;
    16
}
/// $086728: addi.w #$50, $22(a0)
pub fn i_086728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8672E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08672E: addi.w #$10, $24(a0)
pub fn i_08672e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86734;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086734: bra.w $86738
pub fn i_086734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86738;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
/// $086738: move.w #$c0, $70(a6)
pub fn i_086738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8673E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08673E: move.w #$140, $66(a6)
pub fn i_08673e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86744;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086744: move.b #$0, $21(a6)
pub fn i_086744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8674A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08674A: lea.l $2ec41e.l, a0
pub fn i_08674a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86750;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC41E;
    20
}
/// $086750: move.l a0, $48(a6)
pub fn i_086750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86754;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086754: lea.l $8675a(pc), a1
pub fn i_086754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86758;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8675A;
    16
}
/// $086758: move.l a1, (a6)
pub fn i_086758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8675A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08675A: cmpi.w #$ffc0, $22(a6)
pub fn i_08675a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86760;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFC0, Size::Word, false);
    16
}
/// $086760: bgt.w $8676c
pub fn i_086760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86764;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8676C; return 10; }
    8
}
/// $086764: lea.l $ffff.w, a0
pub fn i_086764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86768;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086768: move.l a0, $48(a6)
pub fn i_086768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8676C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08676C: jsr $2783a.l
pub fn i_08676c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86772);
    cpu.pc = 0x2783A;
    16
}
/// $086772: jsr $2870a.l
pub fn i_086772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86778);
    cpu.pc = 0x2870A;
    16
}
/// $086778: bcc.w $8678e
pub fn i_086778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8677C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8678E; return 10; }
    8
}
/// $08677C: lea.l $5e766.l, a0
pub fn i_08677c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86782;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $086782: jsr $5e770.l
pub fn i_086782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86788);
    cpu.pc = 0x5E770;
    16
}
/// $086788: bclr.b #$3, $13(a6)
pub fn i_086788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8678E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08678E: cmpi.w #$c0, $22(a6)
pub fn i_08678e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86794;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Word, false);
    16
}
/// $086794: blt.w $8679e
pub fn i_086794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86798;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8679E; return 10; }
    8
}
/// $086798: move.w #$140, $66(a6)
pub fn i_086798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8679E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08679E: cmpi.w #$0, $66(a6)
pub fn i_08679e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0867A4: bgt.w $867bc
pub fn i_0867a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867A8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x867BC; return 10; }
    8
}
/// $0867A8: lea.l $ffff.w, a0
pub fn i_0867a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867AC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0867AC: move.l a0, $48(a6)
pub fn i_0867ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0867B0: move.b #$ff, $21(a6)
pub fn i_0867b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0867B6: lea.l $867d4(pc), a1
pub fn i_0867b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x867D4;
    16
}
/// $0867BA: move.l a1, (a6)
pub fn i_0867ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0867BC: jsr $4fa70.l
pub fn i_0867bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867C2);
    cpu.pc = 0x4FA70;
    16
}
/// $0867C2: bcc.w $867d2
pub fn i_0867c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x867D2; return 10; }
    8
}
/// $0867C6: move.b #$ff, $21(a6)
pub fn i_0867c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0867CC: jmp $518.l
pub fn i_0867cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867D2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0867D2: rts
pub fn i_0867d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_Destroyed_0867d4  ($0867D4, 120 B, enemies/bosses/fort_scene4_0865be.s)
/// $0867D4: jsr $2783a.l
pub fn i_0867d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867DA);
    cpu.pc = 0x2783A;
    16
}
/// $0867DA: move.w #$1028, d0
pub fn i_0867da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0867DE: jsr $2352.l
pub fn i_0867de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867E4);
    cpu.pc = 0x2352;
    16
}
/// $0867E4: lea.l $2ecf68.l, a1
pub fn i_0867e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECF68;
    20
}
/// $0867EA: jsr $77c7e.l
pub fn i_0867ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867F0);
    cpu.pc = 0x77C7E;
    16
}
/// $0867F0: lea.l $2ecf7a.l, a1
pub fn i_0867f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECF7A;
    20
}
/// $0867F6: jsr $77c7e.l
pub fn i_0867f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867FC);
    cpu.pc = 0x77C7E;
    16
}
/// $0867FC: lea.l $2ed04a.l, a1
pub fn i_0867fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86802;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED04A;
    20
}
/// $086802: jsr $77c7e.l
pub fn i_086802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86808);
    cpu.pc = 0x77C7E;
    16
}
/// $086808: jsr $88a28(pc)
pub fn i_086808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8680C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8680C);
    cpu.pc = 0x88A28;
    16
}
/// $08680C: movea.l $78(a6), a1
pub fn i_08680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86810;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086810: jsr $43fac.l
pub fn i_086810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86816);
    cpu.pc = 0x43FAC;
    16
}
/// $086816: jsr $434dc.l
pub fn i_086816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8681C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8681C);
    cpu.pc = 0x434DC;
    16
}
/// $08681C: lea.l $2ecc88.l, a0
pub fn i_08681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86822;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECC88;
    20
}
/// $086822: move.l a0, $4c(a6)
pub fn i_086822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86826;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086826: jsr $283ca.l
pub fn i_086826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8682C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8682C);
    cpu.pc = 0x283CA;
    16
}
/// $08682C: jsr $283ca.l
pub fn i_08682c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86832);
    cpu.pc = 0x283CA;
    16
}
/// $086832: jsr $283d8.l
pub fn i_086832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86838);
    cpu.pc = 0x283D8;
    16
}
/// $086838: movea.l $7c(a6), a1
pub fn i_086838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8683C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08683C: move.w #$82, d0
pub fn i_08683c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86840;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086840: move.b #$0, $82(a6)
pub fn i_086840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86846;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086846: jsr $4429e.l
pub fn i_086846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8684C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8684C);
    cpu.pc = 0x4429E;
    16
}
// ===== SetTaskHandler_08684c  ($08684C, 8 B, kernel/task_handlers.c)
/// $08684C: lea.l $86854(pc), a1
pub fn i_08684c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86850;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86854;
    16
}
/// $086850: move.l a1, (a6)
pub fn i_086850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86852;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086852: rts
pub fn i_086852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86854;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_Idle_086854  ($086854, 20 B, enemies/bosses/fort_scene4_0865be.s)
/// $086854: lea.l $ffff.w, a0
pub fn i_086854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86858;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086858: move.l a0, $4c(a6)
pub fn i_086858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8685C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08685C: jsr $283ca.l
pub fn i_08685c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86862);
    cpu.pc = 0x283CA;
    16
}
/// $086862: jmp $518.l
pub fn i_086862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86868;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_086868  ($086868, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $086868: rts
pub fn i_086868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8686A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Init_08686a  ($08686A, 314 B, enemies/bosses/fort_scene4_0865be.s)
/// $08686A: move.w #$e3, d1
pub fn i_08686a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8686E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08686E: jsr $236e.l
pub fn i_08686e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86874);
    cpu.pc = 0x236E;
    16
}
/// $086874: move.b #$0, $21(a6)
pub fn i_086874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8687A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08687A: bra.w $868e2
pub fn i_08687a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8687E;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $08687E: move.w #$e4, d1
pub fn i_08687e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86882;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086882: jsr $236e.l
pub fn i_086882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86888);
    cpu.pc = 0x236E;
    16
}
/// $086888: move.b #$30, $21(a6)
pub fn i_086888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8688E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08688E: bra.w $868e2
pub fn i_08688e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86892;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $086892: move.w #$e3, d1
pub fn i_086892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86896;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086896: jsr $236e.l
pub fn i_086896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8689C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8689C);
    cpu.pc = 0x236E;
    16
}
/// $08689C: move.b #$60, $21(a6)
pub fn i_08689c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868A2: bra.w $868e2
pub fn i_0868a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $0868A6: move.w #$e3, d1
pub fn i_0868a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0868AA: jsr $236e.l
pub fn i_0868aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x868B0);
    cpu.pc = 0x236E;
    16
}
/// $0868B0: move.b #$1, $21(a6)
pub fn i_0868b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868B6: bra.w $868e2
pub fn i_0868b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $0868BA: move.w #$e4, d1
pub fn i_0868ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868BE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0868BE: jsr $236e.l
pub fn i_0868be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x868C4);
    cpu.pc = 0x236E;
    16
}
/// $0868C4: move.b #$31, $21(a6)
pub fn i_0868c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868CA: bra.w $868e2
pub fn i_0868ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $0868CE: move.w #$e3, d1
pub fn i_0868ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0868D2: jsr $236e.l
pub fn i_0868d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x868D8);
    cpu.pc = 0x236E;
    16
}
/// $0868D8: move.b #$61, $21(a6)
pub fn i_0868d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x61;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868DE: bra.w $868e2
pub fn i_0868de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $0868E2: move.b #$ff, $32(a6)
pub fn i_0868e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868E8: move.b #$ff, $33(a6)
pub fn i_0868e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868EE: move.b #$0, $3a(a6)
pub fn i_0868ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868F4: move.w #$28, d0
pub fn i_0868f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0868F8: move.w d0, $66(a6)
pub fn i_0868f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0868FC: move.w d0, $80(a6)
pub fn i_0868fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86900;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086900: move.w #$8000, $38(a6)
pub fn i_086900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86906;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086906: jsr $267e2.l
pub fn i_086906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8690C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8690C);
    cpu.pc = 0x267E2;
    16
}
/// $08690C: jsr $27cee.l
pub fn i_08690c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86912);
    cpu.pc = 0x27CEE;
    16
}
/// $086912: clr.l d0
pub fn i_086912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86914;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086914: move.b $21(a6), d0
pub fn i_086914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86918;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086918: asr.l #$4, d0
pub fn i_086918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8691A;
    cpu.ir = 0xE880;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[0] = r;
    14
}
/// $08691A: movea.l #$2eb2c0, a0
pub fn i_08691a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86920;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB2C0;
    cpu.a[0] = v1;
    4
}
/// $086920: lsl.w #$2, d0
pub fn i_086920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86922;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086922: movea.l (a0, d0.w), a0
pub fn i_086922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86926;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086926: cmpa.l #$ffffffff, a0
pub fn i_086926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8692C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08692C: beq.w $86936
pub fn i_08692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86930;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86936; return 10; }
    8
}
/// $086930: jsr $28cd4.l
pub fn i_086930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86936);
    cpu.pc = 0x28CD4;
    16
}
/// $086936: move.w $72(a6), d0
pub fn i_086936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8693A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08693A: move.b d0, $44(a6)
pub fn i_08693a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8693E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08693E: lea.l $86944(pc), a1
pub fn i_08693e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86944;
    16
}
/// $086942: move.l a1, (a6)
pub fn i_086942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086944: movea.l $c(a6), a0
pub fn i_086944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86948;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086948: cmpi.b #$ff, $21(a0)
pub fn i_086948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8694E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08694E: beq.w $86990
pub fn i_08694e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86952;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86990; return 10; }
    8
}
/// $086952: jsr $2783a.l
pub fn i_086952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86958);
    cpu.pc = 0x2783A;
    16
}
/// $086958: jsr $28d70.l
pub fn i_086958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8695E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8695E);
    cpu.pc = 0x28D70;
    16
}
/// $08695E: jsr $2870a.l
pub fn i_08695e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86964);
    cpu.pc = 0x2870A;
    16
}
/// $086964: bcc.w $8697c
pub fn i_086964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86968;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8697C; return 10; }
    8
}
/// $086968: move.w #$108d, d0
pub fn i_086968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8696C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08696C: jsr $2352.l
pub fn i_08696c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86972);
    cpu.pc = 0x2352;
    16
}
/// $086972: bclr.b #$3, $13(a6)
pub fn i_086972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86978;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086978: jsr $8848c(pc)
pub fn i_086978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8697C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8697C);
    cpu.pc = 0x8848C;
    16
}
/// $08697C: cmpi.w #$1e, $66(a6)
pub fn i_08697c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86982;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $086982: bgt.w $869a2
pub fn i_086982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86986;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x869A2; return 10; }
    8
}
/// $086986: lea.l $869a4(pc), a1
pub fn i_086986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8698A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x869A4;
    16
}
/// $08698A: move.l a1, (a6)
pub fn i_08698a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8698C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08698C: bra.w $869a2
pub fn i_08698c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86990;
    cpu.ir = 0x6000;
    cpu.pc = 0x869A2;
    10
}
/// $086990: lea.l $2edd48.l, a1
pub fn i_086990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86996;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086996: jsr $43fac.l
pub fn i_086996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8699C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8699C);
    cpu.pc = 0x43FAC;
    16
}
/// $08699C: jmp $518.l
pub fn i_08699c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0869A2: rts
pub fn i_0869a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Damaged1_0869a4  ($0869A4, 140 B, enemies/bosses/fort_scene4_0865be.s)
/// $0869A4: clr.l d0
pub fn i_0869a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869A6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0869A6: move.b $21(a6), d0
pub fn i_0869a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869AA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0869AA: asr.l #$4, d0
pub fn i_0869aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869AC;
    cpu.ir = 0xE880;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[0] = r;
    14
}
/// $0869AC: addq.l #$1, d0
pub fn i_0869ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869AE;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $0869AE: movea.l #$2eb2c0, a0
pub fn i_0869ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB2C0;
    cpu.a[0] = v1;
    4
}
/// $0869B4: lsl.w #$2, d0
pub fn i_0869b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869B6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0869B6: movea.l (a0, d0.w), a0
pub fn i_0869b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869BA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0869BA: cmpa.l #$ffffffff, a0
pub fn i_0869ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869C0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0869C0: beq.w $869ca
pub fn i_0869c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x869CA; return 10; }
    8
}
/// $0869C4: jsr $28cd4.l
pub fn i_0869c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0869CA: lea.l $869d0(pc), a1
pub fn i_0869ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x869D0;
    16
}
/// $0869CE: move.l a1, (a6)
pub fn i_0869ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0869D0: movea.l $c(a6), a0
pub fn i_0869d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869D4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0869D4: cmpi.b #$ff, $21(a0)
pub fn i_0869d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869DA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0869DA: beq.w $86a1c
pub fn i_0869da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86A1C; return 10; }
    8
}
/// $0869DE: jsr $2783a.l
pub fn i_0869de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869E4);
    cpu.pc = 0x2783A;
    16
}
/// $0869E4: jsr $28d70.l
pub fn i_0869e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869EA);
    cpu.pc = 0x28D70;
    16
}
/// $0869EA: jsr $2870a.l
pub fn i_0869ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869F0);
    cpu.pc = 0x2870A;
    16
}
/// $0869F0: bcc.w $86a08
pub fn i_0869f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86A08; return 10; }
    8
}
/// $0869F4: move.w #$108d, d0
pub fn i_0869f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0869F8: jsr $2352.l
pub fn i_0869f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869FE);
    cpu.pc = 0x2352;
    16
}
/// $0869FE: bclr.b #$3, $13(a6)
pub fn i_0869fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A04;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086A04: jsr $8848c(pc)
pub fn i_086a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86A08);
    cpu.pc = 0x8848C;
    16
}
/// $086A08: cmpi.w #$1a, $66(a6)
pub fn i_086a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A0E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1A, Size::Word, false);
    16
}
/// $086A0E: bgt.w $86a2e
pub fn i_086a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A12;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x86A2E; return 10; }
    8
}
/// $086A12: lea.l $86a30(pc), a1
pub fn i_086a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86A30;
    16
}
/// $086A16: move.l a1, (a6)
pub fn i_086a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086A18: bra.w $86a2e
pub fn i_086a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x86A2E;
    10
}
/// $086A1C: lea.l $2edd48.l, a1
pub fn i_086a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A22;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086A22: jsr $43fac.l
pub fn i_086a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A28);
    cpu.pc = 0x43FAC;
    16
}
/// $086A28: jmp $518.l
pub fn i_086a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A2E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086A2E: rts
pub fn i_086a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Damaged2_086a30  ($086A30, 140 B, enemies/bosses/fort_scene4_0865be.s)
/// $086A30: clr.l d0
pub fn i_086a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A32;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086A32: move.b $21(a6), d0
pub fn i_086a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A36;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086A36: asr.l #$4, d0
pub fn i_086a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A38;
    cpu.ir = 0xE880;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[0] = r;
    14
}
/// $086A38: addq.l #$2, d0
pub fn i_086a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A3A;
    cpu.ir = 0x5480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $086A3A: movea.l #$2eb2c0, a0
pub fn i_086a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A40;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB2C0;
    cpu.a[0] = v1;
    4
}
/// $086A40: lsl.w #$2, d0
pub fn i_086a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A42;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086A42: movea.l (a0, d0.w), a0
pub fn i_086a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A46;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086A46: cmpa.l #$ffffffff, a0
pub fn i_086a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A4C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $086A4C: beq.w $86a56
pub fn i_086a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86A56; return 10; }
    8
}
/// $086A50: jsr $28cd4.l
pub fn i_086a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A56);
    cpu.pc = 0x28CD4;
    16
}
/// $086A56: lea.l $86a5c(pc), a1
pub fn i_086a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86A5C;
    16
}
/// $086A5A: move.l a1, (a6)
pub fn i_086a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086A5C: movea.l $c(a6), a0
pub fn i_086a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086A60: cmpi.b #$ff, $21(a0)
pub fn i_086a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A66;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086A66: beq.w $86aa8
pub fn i_086a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A6A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86AA8; return 10; }
    8
}
/// $086A6A: jsr $2783a.l
pub fn i_086a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A70);
    cpu.pc = 0x2783A;
    16
}
/// $086A70: jsr $28d70.l
pub fn i_086a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A76);
    cpu.pc = 0x28D70;
    16
}
/// $086A76: jsr $2870a.l
pub fn i_086a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A7C);
    cpu.pc = 0x2870A;
    16
}
/// $086A7C: bcc.w $86a94
pub fn i_086a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86A94; return 10; }
    8
}
/// $086A80: move.w #$108d, d0
pub fn i_086a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086A84: jsr $2352.l
pub fn i_086a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A8A);
    cpu.pc = 0x2352;
    16
}
/// $086A8A: bclr.b #$3, $13(a6)
pub fn i_086a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086A90: jsr $8848c(pc)
pub fn i_086a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86A94);
    cpu.pc = 0x8848C;
    16
}
/// $086A94: cmpi.w #$d, $66(a6)
pub fn i_086a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A9A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $086A9A: bgt.w $86aba
pub fn i_086a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A9E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x86ABA; return 10; }
    8
}
/// $086A9E: lea.l $86abc(pc), a1
pub fn i_086a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86ABC;
    16
}
/// $086AA2: move.l a1, (a6)
pub fn i_086aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086AA4: bra.w $86aba
pub fn i_086aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86ABA;
    10
}
/// $086AA8: lea.l $2edd48.l, a1
pub fn i_086aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AAE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086AAE: jsr $43fac.l
pub fn i_086aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AB4);
    cpu.pc = 0x43FAC;
    16
}
/// $086AB4: jmp $518.l
pub fn i_086ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ABA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086ABA: rts
pub fn i_086aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ABC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Critical_086abc  ($086ABC, 232 B, enemies/bosses/fort_scene4_0865be.s)
/// $086ABC: lea.l $86ac2(pc), a1
pub fn i_086abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86AC2;
    16
}
/// $086AC0: move.l a1, (a6)
pub fn i_086ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086AC2: movea.l $c(a6), a0
pub fn i_086ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AC6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086AC6: cmpi.b #$ff, $21(a0)
pub fn i_086ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ACC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086ACC: beq.w $86b90
pub fn i_086acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86B90; return 10; }
    8
}
/// $086AD0: jsr $2783a.l
pub fn i_086ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AD6);
    cpu.pc = 0x2783A;
    16
}
/// $086AD6: jsr $28d70.l
pub fn i_086ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86ADC);
    cpu.pc = 0x28D70;
    16
}
/// $086ADC: jsr $2870a.l
pub fn i_086adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AE2);
    cpu.pc = 0x2870A;
    16
}
/// $086AE2: bcc.w $86afa
pub fn i_086ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86AFA; return 10; }
    8
}
/// $086AE6: move.w #$108d, d0
pub fn i_086ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086AEA: jsr $2352.l
pub fn i_086aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AF0);
    cpu.pc = 0x2352;
    16
}
/// $086AF0: bclr.b #$3, $13(a6)
pub fn i_086af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AF6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086AF6: jsr $8848c(pc)
pub fn i_086af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86AFA);
    cpu.pc = 0x8848C;
    16
}
/// $086AFA: jsr $28758.l
pub fn i_086afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B00);
    cpu.pc = 0x28758;
    16
}
/// $086B00: bcc.w $86ba2
pub fn i_086b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B04;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86BA2; return 10; }
    8
}
/// $086B04: bclr.b #$0, $13(a6)
pub fn i_086b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B0A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086B0A: lea.l $ffff.w, a0
pub fn i_086b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B0E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086B0E: move.l a0, $48(a6)
pub fn i_086b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B12;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086B12: btst.b #$0, $21(a6)
pub fn i_086b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B18;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $086B18: beq.w $86b26
pub fn i_086b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86B26; return 10; }
    8
}
/// $086B1C: lea.l $86ba4(pc), a1
pub fn i_086b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86BA4;
    16
}
/// $086B20: move.l a1, (a6)
pub fn i_086b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086B22: bra.w $86ba2
pub fn i_086b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B26;
    cpu.ir = 0x6000;
    cpu.pc = 0x86BA2;
    10
}
/// $086B26: move.l #$100, d0
pub fn i_086b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B2C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086B2C: jsr $51a28.l
pub fn i_086b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B32);
    cpu.pc = 0x51A28;
    16
}
/// $086B32: move.w #$102f, d0
pub fn i_086b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B36;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086B36: jsr $2352.l
pub fn i_086b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B3C);
    cpu.pc = 0x2352;
    16
}
/// $086B3C: lea.l $78066.l, a1
pub fn i_086b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B42;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78066;
    20
}
/// $086B42: jsr $4ae.l
pub fn i_086b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B48);
    cpu.pc = 0x4AE;
    16
}
/// $086B48: jsr $5dd22.l
pub fn i_086b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B4E);
    cpu.pc = 0x5DD22;
    16
}
/// $086B4E: addi.w #$10, $24(a0)
pub fn i_086b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B54;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086B54: movea.l $74(a6), a2
pub fn i_086b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B58;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $086B58: jsr $5022a.l
pub fn i_086b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B5E);
    cpu.pc = 0x5022A;
    16
}
/// $086B5E: lea.l $2ed05c.l, a1
pub fn i_086b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B64;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED05C;
    20
}
/// $086B64: jsr $77c7e.l
pub fn i_086b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B6A);
    cpu.pc = 0x77C7E;
    16
}
/// $086B6A: addi.w #$10, $24(a0)
pub fn i_086b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B70;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086B70: movea.l $7c(a6), a1
pub fn i_086b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086B74: move.l a1, d0
pub fn i_086b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B76;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086B76: cmpi.l #$ffffffff, d0
pub fn i_086b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B7C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $086B7C: beq.w $86b90
pub fn i_086b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86B90; return 10; }
    8
}
/// $086B80: move.w #$82, d0
pub fn i_086b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086B84: move.b #$0, $82(a6)
pub fn i_086b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B8A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086B8A: jsr $4429e.l
pub fn i_086b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B90);
    cpu.pc = 0x4429E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0850A8, i_0850a8),
    (0x0850AE, i_0850ae),
    (0x0850B4, i_0850b4),
    (0x0850BA, i_0850ba),
    (0x0850BE, i_0850be),
    (0x0850C4, i_0850c4),
    (0x0850C6, i_0850c6),
    (0x0850CC, i_0850cc),
    (0x0850D2, i_0850d2),
    (0x0850D8, i_0850d8),
    (0x0850DC, i_0850dc),
    (0x0850DE, i_0850de),
    (0x0850E4, i_0850e4),
    (0x0850EA, i_0850ea),
    (0x0850EE, i_0850ee),
    (0x0850F2, i_0850f2),
    (0x0850F6, i_0850f6),
    (0x0850FA, i_0850fa),
    (0x0850FE, i_0850fe),
    (0x085102, i_085102),
    (0x085104, i_085104),
    (0x085108, i_085108),
    (0x08510C, i_08510c),
    (0x08510E, i_08510e),
    (0x085114, i_085114),
    (0x08511A, i_08511a),
    (0x085120, i_085120),
    (0x085124, i_085124),
    (0x08512A, i_08512a),
    (0x08512C, i_08512c),
    (0x085130, i_085130),
    (0x085132, i_085132),
    (0x085134, i_085134),
    (0x08513A, i_08513a),
    (0x085140, i_085140),
    (0x085146, i_085146),
    (0x08514C, i_08514c),
    (0x085150, i_085150),
    (0x085154, i_085154),
    (0x085158, i_085158),
    (0x08515E, i_08515e),
    (0x085162, i_085162),
    (0x085166, i_085166),
    (0x08516E, i_08516e),
    (0x085176, i_085176),
    (0x08517C, i_08517c),
    (0x085182, i_085182),
    (0x085188, i_085188),
    (0x08518E, i_08518e),
    (0x085190, i_085190),
    (0x085194, i_085194),
    (0x08519A, i_08519a),
    (0x0851A0, i_0851a0),
    (0x0851A6, i_0851a6),
    (0x0851AA, i_0851aa),
    (0x0851AE, i_0851ae),
    (0x0851B4, i_0851b4),
    (0x0851BA, i_0851ba),
    (0x0851C0, i_0851c0),
    (0x0851C6, i_0851c6),
    (0x0851CC, i_0851cc),
    (0x0851D0, i_0851d0),
    (0x0851D2, i_0851d2),
    (0x0851D8, i_0851d8),
    (0x0851DA, i_0851da),
    (0x0851DE, i_0851de),
    (0x0851E2, i_0851e2),
    (0x0851E8, i_0851e8),
    (0x0851EE, i_0851ee),
    (0x0851F0, i_0851f0),
    (0x0851F4, i_0851f4),
    (0x0851FA, i_0851fa),
    (0x085200, i_085200),
    (0x085204, i_085204),
    (0x08520A, i_08520a),
    (0x085210, i_085210),
    (0x085214, i_085214),
    (0x085216, i_085216),
    (0x08521C, i_08521c),
    (0x085222, i_085222),
    (0x085228, i_085228),
    (0x08522E, i_08522e),
    (0x085232, i_085232),
    (0x085238, i_085238),
    (0x08523E, i_08523e),
    (0x085244, i_085244),
    (0x08524A, i_08524a),
    (0x085250, i_085250),
    (0x085256, i_085256),
    (0x08525C, i_08525c),
    (0x085262, i_085262),
    (0x085266, i_085266),
    (0x085268, i_085268),
    (0x08526E, i_08526e),
    (0x085270, i_085270),
    (0x085274, i_085274),
    (0x08527A, i_08527a),
    (0x085280, i_085280),
    (0x085286, i_085286),
    (0x08528A, i_08528a),
    (0x08528C, i_08528c),
    (0x085292, i_085292),
    (0x085298, i_085298),
    (0x08529E, i_08529e),
    (0x0852A4, i_0852a4),
    (0x0852A8, i_0852a8),
    (0x0852AE, i_0852ae),
    (0x0852B4, i_0852b4),
    (0x0852BA, i_0852ba),
    (0x0852C0, i_0852c0),
    (0x0852C6, i_0852c6),
    (0x0852CC, i_0852cc),
    (0x0852D2, i_0852d2),
    (0x0852D6, i_0852d6),
    (0x0852DA, i_0852da),
    (0x0852DE, i_0852de),
    (0x0852E4, i_0852e4),
    (0x0852EA, i_0852ea),
    (0x0852F0, i_0852f0),
    (0x0852F6, i_0852f6),
    (0x0852FC, i_0852fc),
    (0x085302, i_085302),
    (0x085308, i_085308),
    (0x08530E, i_08530e),
    (0x085314, i_085314),
    (0x085318, i_085318),
    (0x08531E, i_08531e),
    (0x085324, i_085324),
    (0x08532A, i_08532a),
    (0x08532E, i_08532e),
    (0x085330, i_085330),
    (0x085336, i_085336),
    (0x085338, i_085338),
    (0x08533C, i_08533c),
    (0x085340, i_085340),
    (0x085346, i_085346),
    (0x08534A, i_08534a),
    (0x085350, i_085350),
    (0x085354, i_085354),
    (0x085358, i_085358),
    (0x08535E, i_08535e),
    (0x085364, i_085364),
    (0x08536A, i_08536a),
    (0x085370, i_085370),
    (0x085376, i_085376),
    (0x08537A, i_08537a),
    (0x08537C, i_08537c),
    (0x085382, i_085382),
    (0x085388, i_085388),
    (0x08538C, i_08538c),
    (0x085392, i_085392),
    (0x085394, i_085394),
    (0x085398, i_085398),
    (0x08539E, i_08539e),
    (0x0853A2, i_0853a2),
    (0x0853A8, i_0853a8),
    (0x0853AE, i_0853ae),
    (0x0853B2, i_0853b2),
    (0x0853B6, i_0853b6),
    (0x0853BC, i_0853bc),
    (0x0853C0, i_0853c0),
    (0x0853C4, i_0853c4),
    (0x0853C8, i_0853c8),
    (0x0853CE, i_0853ce),
    (0x0853D4, i_0853d4),
    (0x0853DA, i_0853da),
    (0x0853E0, i_0853e0),
    (0x0853E8, i_0853e8),
    (0x0853EC, i_0853ec),
    (0x0853EE, i_0853ee),
    (0x0853F4, i_0853f4),
    (0x0853F6, i_0853f6),
    (0x0853FA, i_0853fa),
    (0x0853FE, i_0853fe),
    (0x085404, i_085404),
    (0x08540A, i_08540a),
    (0x085410, i_085410),
    (0x085412, i_085412),
    (0x085418, i_085418),
    (0x08541E, i_08541e),
    (0x085422, i_085422),
    (0x085426, i_085426),
    (0x08542C, i_08542c),
    (0x085430, i_085430),
    (0x085436, i_085436),
    (0x08543A, i_08543a),
    (0x08543C, i_08543c),
    (0x085442, i_085442),
    (0x085448, i_085448),
    (0x08544C, i_08544c),
    (0x085450, i_085450),
    (0x085456, i_085456),
    (0x08545A, i_08545a),
    (0x08545E, i_08545e),
    (0x085464, i_085464),
    (0x085468, i_085468),
    (0x08546C, i_08546c),
    (0x085470, i_085470),
    (0x085476, i_085476),
    (0x08547C, i_08547c),
    (0x085480, i_085480),
    (0x085482, i_085482),
    (0x085484, i_085484),
    (0x085488, i_085488),
    (0x08548C, i_08548c),
    (0x085490, i_085490),
    (0x085496, i_085496),
    (0x08549C, i_08549c),
    (0x0854A2, i_0854a2),
    (0x0854A8, i_0854a8),
    (0x0854AE, i_0854ae),
    (0x0854B4, i_0854b4),
    (0x0854BA, i_0854ba),
    (0x0854BE, i_0854be),
    (0x0854C0, i_0854c0),
    (0x0854C6, i_0854c6),
    (0x0854CC, i_0854cc),
    (0x0854D2, i_0854d2),
    (0x0854D6, i_0854d6),
    (0x0854DA, i_0854da),
    (0x0854E0, i_0854e0),
    (0x0854E6, i_0854e6),
    (0x0854E8, i_0854e8),
    (0x0854EE, i_0854ee),
    (0x0854F2, i_0854f2),
    (0x0854F4, i_0854f4),
    (0x0854FA, i_0854fa),
    (0x085500, i_085500),
    (0x085504, i_085504),
    (0x08550A, i_08550a),
    (0x085510, i_085510),
    (0x085516, i_085516),
    (0x08551A, i_08551a),
    (0x08551E, i_08551e),
    (0x085524, i_085524),
    (0x085526, i_085526),
    (0x08552C, i_08552c),
    (0x085532, i_085532),
    (0x085536, i_085536),
    (0x085538, i_085538),
    (0x08553C, i_08553c),
    (0x085542, i_085542),
    (0x085548, i_085548),
    (0x08554E, i_08554e),
    (0x085554, i_085554),
    (0x085558, i_085558),
    (0x08555E, i_08555e),
    (0x085564, i_085564),
    (0x08556A, i_08556a),
    (0x08556E, i_08556e),
    (0x085572, i_085572),
    (0x085578, i_085578),
    (0x08557C, i_08557c),
    (0x085582, i_085582),
    (0x085584, i_085584),
    (0x085588, i_085588),
    (0x08558C, i_08558c),
    (0x085592, i_085592),
    (0x085596, i_085596),
    (0x08559A, i_08559a),
    (0x0855A0, i_0855a0),
    (0x0855A4, i_0855a4),
    (0x0855A6, i_0855a6),
    (0x0855AC, i_0855ac),
    (0x0855B0, i_0855b0),
    (0x0855B6, i_0855b6),
    (0x0855BA, i_0855ba),
    (0x0855C0, i_0855c0),
    (0x0855C2, i_0855c2),
    (0x0855C8, i_0855c8),
    (0x0855CC, i_0855cc),
    (0x0855D2, i_0855d2),
    (0x0855D6, i_0855d6),
    (0x0855DA, i_0855da),
    (0x0855DE, i_0855de),
    (0x0855E4, i_0855e4),
    (0x0855EA, i_0855ea),
    (0x0855F0, i_0855f0),
    (0x0855F6, i_0855f6),
    (0x0855FC, i_0855fc),
    (0x085602, i_085602),
    (0x085606, i_085606),
    (0x085608, i_085608),
    (0x08560C, i_08560c),
    (0x085610, i_085610),
    (0x085616, i_085616),
    (0x08561A, i_08561a),
    (0x08561E, i_08561e),
    (0x085624, i_085624),
    (0x085628, i_085628),
    (0x08562A, i_08562a),
    (0x085630, i_085630),
    (0x085634, i_085634),
    (0x08563A, i_08563a),
    (0x08563E, i_08563e),
    (0x085644, i_085644),
    (0x085646, i_085646),
    (0x08564C, i_08564c),
    (0x085650, i_085650),
    (0x085656, i_085656),
    (0x08565A, i_08565a),
    (0x08565E, i_08565e),
    (0x085662, i_085662),
    (0x085668, i_085668),
    (0x08566E, i_08566e),
    (0x085674, i_085674),
    (0x08567A, i_08567a),
    (0x085680, i_085680),
    (0x085686, i_085686),
    (0x08568C, i_08568c),
    (0x085692, i_085692),
    (0x085698, i_085698),
    (0x08569E, i_08569e),
    (0x0856A4, i_0856a4),
    (0x0856A8, i_0856a8),
    (0x0856AA, i_0856aa),
    (0x0856AE, i_0856ae),
    (0x0856B4, i_0856b4),
    (0x0856B8, i_0856b8),
    (0x0856BE, i_0856be),
    (0x0856C4, i_0856c4),
    (0x0856CA, i_0856ca),
    (0x0856CC, i_0856cc),
    (0x0856D0, i_0856d0),
    (0x0856D6, i_0856d6),
    (0x0856D8, i_0856d8),
    (0x0856DC, i_0856dc),
    (0x0856E2, i_0856e2),
    (0x0856E6, i_0856e6),
    (0x0856EC, i_0856ec),
    (0x0856F2, i_0856f2),
    (0x0856F6, i_0856f6),
    (0x0856F8, i_0856f8),
    (0x0856FE, i_0856fe),
    (0x085704, i_085704),
    (0x085708, i_085708),
    (0x08570C, i_08570c),
    (0x085712, i_085712),
    (0x085716, i_085716),
    (0x08571C, i_08571c),
    (0x085720, i_085720),
    (0x085724, i_085724),
    (0x085728, i_085728),
    (0x08572E, i_08572e),
    (0x085732, i_085732),
    (0x085738, i_085738),
    (0x08573A, i_08573a),
    (0x08573E, i_08573e),
    (0x085744, i_085744),
    (0x08574A, i_08574a),
    (0x085750, i_085750),
    (0x085756, i_085756),
    (0x08575C, i_08575c),
    (0x085760, i_085760),
    (0x085762, i_085762),
    (0x085768, i_085768),
    (0x08576E, i_08576e),
    (0x085772, i_085772),
    (0x085778, i_085778),
    (0x08577A, i_08577a),
    (0x08577C, i_08577c),
    (0x085780, i_085780),
    (0x085784, i_085784),
    (0x08578A, i_08578a),
    (0x085790, i_085790),
    (0x085794, i_085794),
    (0x085798, i_085798),
    (0x08579E, i_08579e),
    (0x0857A0, i_0857a0),
    (0x0857A4, i_0857a4),
    (0x0857A6, i_0857a6),
    (0x0857AA, i_0857aa),
    (0x0857AC, i_0857ac),
    (0x0857B2, i_0857b2),
    (0x0857B4, i_0857b4),
    (0x0857B8, i_0857b8),
    (0x0857BA, i_0857ba),
    (0x0857C0, i_0857c0),
    (0x0857C6, i_0857c6),
    (0x0857CC, i_0857cc),
    (0x0857D2, i_0857d2),
    (0x0857D6, i_0857d6),
    (0x0857DC, i_0857dc),
    (0x0857E2, i_0857e2),
    (0x0857E6, i_0857e6),
    (0x0857EA, i_0857ea),
    (0x0857F0, i_0857f0),
    (0x0857F2, i_0857f2),
    (0x0857F6, i_0857f6),
    (0x0857FC, i_0857fc),
    (0x085802, i_085802),
    (0x085806, i_085806),
    (0x08580C, i_08580c),
    (0x085812, i_085812),
    (0x085816, i_085816),
    (0x085818, i_085818),
    (0x08581E, i_08581e),
    (0x085820, i_085820),
    (0x085826, i_085826),
    (0x08582A, i_08582a),
    (0x085830, i_085830),
    (0x085836, i_085836),
    (0x08583C, i_08583c),
    (0x085840, i_085840),
    (0x085842, i_085842),
    (0x085848, i_085848),
    (0x08584E, i_08584e),
    (0x085854, i_085854),
    (0x08585A, i_08585a),
    (0x085860, i_085860),
    (0x085864, i_085864),
    (0x08586A, i_08586a),
    (0x08586C, i_08586c),
    (0x085872, i_085872),
    (0x085876, i_085876),
    (0x08587C, i_08587c),
    (0x08587E, i_08587e),
    (0x085884, i_085884),
    (0x085888, i_085888),
    (0x08588E, i_08588e),
    (0x085890, i_085890),
    (0x085896, i_085896),
    (0x08589C, i_08589c),
    (0x0858A0, i_0858a0),
    (0x0858A6, i_0858a6),
    (0x0858AA, i_0858aa),
    (0x0858AE, i_0858ae),
    (0x0858B2, i_0858b2),
    (0x0858B8, i_0858b8),
    (0x0858BC, i_0858bc),
    (0x0858C2, i_0858c2),
    (0x0858C6, i_0858c6),
    (0x0858CA, i_0858ca),
    (0x0858CE, i_0858ce),
    (0x0858D0, i_0858d0),
    (0x0858D4, i_0858d4),
    (0x0858DA, i_0858da),
    (0x0858E0, i_0858e0),
    (0x0858E6, i_0858e6),
    (0x0858EA, i_0858ea),
    (0x0858EE, i_0858ee),
    (0x0858F2, i_0858f2),
    (0x0858F4, i_0858f4),
    (0x0858F8, i_0858f8),
    (0x0858FC, i_0858fc),
    (0x085902, i_085902),
    (0x085904, i_085904),
    (0x085908, i_085908),
    (0x08590E, i_08590e),
    (0x085912, i_085912),
    (0x085918, i_085918),
    (0x08591C, i_08591c),
    (0x08591E, i_08591e),
    (0x085924, i_085924),
    (0x08592A, i_08592a),
    (0x085930, i_085930),
    (0x085936, i_085936),
    (0x08593A, i_08593a),
    (0x08593E, i_08593e),
    (0x085940, i_085940),
    (0x085946, i_085946),
    (0x08594C, i_08594c),
    (0x085952, i_085952),
    (0x085956, i_085956),
    (0x08595C, i_08595c),
    (0x08595E, i_08595e),
    (0x085960, i_085960),
    (0x085964, i_085964),
    (0x085968, i_085968),
    (0x08596E, i_08596e),
    (0x085970, i_085970),
    (0x085974, i_085974),
    (0x08597A, i_08597a),
    (0x08597E, i_08597e),
    (0x085984, i_085984),
    (0x085988, i_085988),
    (0x08598A, i_08598a),
    (0x085990, i_085990),
    (0x085996, i_085996),
    (0x08599A, i_08599a),
    (0x08599E, i_08599e),
    (0x0859A0, i_0859a0),
    (0x0859A6, i_0859a6),
    (0x0859AC, i_0859ac),
    (0x0859B0, i_0859b0),
    (0x0859B4, i_0859b4),
    (0x0859B6, i_0859b6),
    (0x0859BC, i_0859bc),
    (0x0859C2, i_0859c2),
    (0x0859C8, i_0859c8),
    (0x0859CC, i_0859cc),
    (0x0859D2, i_0859d2),
    (0x0859D4, i_0859d4),
    (0x0859DA, i_0859da),
    (0x0859E2, i_0859e2),
    (0x0859EA, i_0859ea),
    (0x0859F0, i_0859f0),
    (0x0859F8, i_0859f8),
    (0x0859FC, i_0859fc),
    (0x085A02, i_085a02),
    (0x085A08, i_085a08),
    (0x085A0A, i_085a0a),
    (0x085A0E, i_085a0e),
    (0x085A10, i_085a10),
    (0x085A16, i_085a16),
    (0x085A1A, i_085a1a),
    (0x085A1E, i_085a1e),
    (0x085A22, i_085a22),
    (0x085A28, i_085a28),
    (0x085A2A, i_085a2a),
    (0x085A30, i_085a30),
    (0x085A34, i_085a34),
    (0x085A38, i_085a38),
    (0x085A3A, i_085a3a),
    (0x085A3E, i_085a3e),
    (0x085A42, i_085a42),
    (0x085A46, i_085a46),
    (0x085A4A, i_085a4a),
    (0x085A50, i_085a50),
    (0x085A56, i_085a56),
    (0x085A5C, i_085a5c),
    (0x085A60, i_085a60),
    (0x085A66, i_085a66),
    (0x085A6C, i_085a6c),
    (0x085A72, i_085a72),
    (0x085A76, i_085a76),
    (0x085A7A, i_085a7a),
    (0x085A7E, i_085a7e),
    (0x085A82, i_085a82),
    (0x085A88, i_085a88),
    (0x085A90, i_085a90),
    (0x085A94, i_085a94),
    (0x085A9A, i_085a9a),
    (0x085A9E, i_085a9e),
    (0x085AA2, i_085aa2),
    (0x085AA6, i_085aa6),
    (0x085AAC, i_085aac),
    (0x085AB0, i_085ab0),
    (0x085AB4, i_085ab4),
    (0x085AB6, i_085ab6),
    (0x085ABC, i_085abc),
    (0x085AC2, i_085ac2),
    (0x085AC6, i_085ac6),
    (0x085ACA, i_085aca),
    (0x085ACC, i_085acc),
    (0x085ACE, i_085ace),
    (0x085AD4, i_085ad4),
    (0x085AD8, i_085ad8),
    (0x085ADC, i_085adc),
    (0x085AE0, i_085ae0),
    (0x085AE4, i_085ae4),
    (0x085AEA, i_085aea),
    (0x085AEE, i_085aee),
    (0x085AF2, i_085af2),
    (0x085AF6, i_085af6),
    (0x085AFA, i_085afa),
    (0x085AFE, i_085afe),
    (0x085B02, i_085b02),
    (0x085B06, i_085b06),
    (0x085B0A, i_085b0a),
    (0x085B10, i_085b10),
    (0x085B14, i_085b14),
    (0x085B18, i_085b18),
    (0x085B1C, i_085b1c),
    (0x085B20, i_085b20),
    (0x085B24, i_085b24),
    (0x085B28, i_085b28),
    (0x085B2E, i_085b2e),
    (0x085B32, i_085b32),
    (0x085B36, i_085b36),
    (0x085B3A, i_085b3a),
    (0x085B3E, i_085b3e),
    (0x085B42, i_085b42),
    (0x085B48, i_085b48),
    (0x085B4C, i_085b4c),
    (0x085B50, i_085b50),
    (0x085B54, i_085b54),
    (0x085B58, i_085b58),
    (0x085B5C, i_085b5c),
    (0x085B60, i_085b60),
    (0x085B62, i_085b62),
    (0x085B66, i_085b66),
    (0x085B68, i_085b68),
    (0x085B6E, i_085b6e),
    (0x085B72, i_085b72),
    (0x085B76, i_085b76),
    (0x085B78, i_085b78),
    (0x085B7C, i_085b7c),
    (0x085B7E, i_085b7e),
    (0x085B82, i_085b82),
    (0x085B86, i_085b86),
    (0x085B8A, i_085b8a),
    (0x085B8E, i_085b8e),
    (0x085B94, i_085b94),
    (0x085B98, i_085b98),
    (0x085B9C, i_085b9c),
    (0x085BA0, i_085ba0),
    (0x085BA4, i_085ba4),
    (0x085BAA, i_085baa),
    (0x085BAE, i_085bae),
    (0x085BB2, i_085bb2),
    (0x085BB6, i_085bb6),
    (0x085BBA, i_085bba),
    (0x085BC0, i_085bc0),
    (0x085BC4, i_085bc4),
    (0x085BC8, i_085bc8),
    (0x085BCC, i_085bcc),
    (0x085BD0, i_085bd0),
    (0x085BD6, i_085bd6),
    (0x085BD8, i_085bd8),
    (0x085BDC, i_085bdc),
    (0x085BE0, i_085be0),
    (0x085BE4, i_085be4),
    (0x085BE8, i_085be8),
    (0x085BEE, i_085bee),
    (0x085BF2, i_085bf2),
    (0x085BF6, i_085bf6),
    (0x085BFA, i_085bfa),
    (0x085BFE, i_085bfe),
    (0x085C04, i_085c04),
    (0x085C08, i_085c08),
    (0x085C0C, i_085c0c),
    (0x085C10, i_085c10),
    (0x085C14, i_085c14),
    (0x085C1A, i_085c1a),
    (0x085C1E, i_085c1e),
    (0x085C22, i_085c22),
    (0x085C26, i_085c26),
    (0x085C2A, i_085c2a),
    (0x085C30, i_085c30),
    (0x085C32, i_085c32),
    (0x085C36, i_085c36),
    (0x085C3C, i_085c3c),
    (0x085C40, i_085c40),
    (0x085C42, i_085c42),
    (0x085C46, i_085c46),
    (0x085C4A, i_085c4a),
    (0x085C4E, i_085c4e),
    (0x085C52, i_085c52),
    (0x085C54, i_085c54),
    (0x085C58, i_085c58),
    (0x085C5C, i_085c5c),
    (0x085C5E, i_085c5e),
    (0x085C62, i_085c62),
    (0x085C66, i_085c66),
    (0x085C6C, i_085c6c),
    (0x085C6E, i_085c6e),
    (0x085C72, i_085c72),
    (0x085C78, i_085c78),
    (0x085C7C, i_085c7c),
    (0x085C82, i_085c82),
    (0x085C86, i_085c86),
    (0x085C8C, i_085c8c),
    (0x085C90, i_085c90),
    (0x085C94, i_085c94),
    (0x085C96, i_085c96),
    (0x085C9A, i_085c9a),
    (0x085C9E, i_085c9e),
    (0x085CA0, i_085ca0),
    (0x085CA4, i_085ca4),
    (0x085CA8, i_085ca8),
    (0x085CAE, i_085cae),
    (0x085CB0, i_085cb0),
    (0x085CB4, i_085cb4),
    (0x085CBA, i_085cba),
    (0x085CBE, i_085cbe),
    (0x085CC4, i_085cc4),
    (0x085CC8, i_085cc8),
    (0x085CCC, i_085ccc),
    (0x085CCE, i_085cce),
    (0x085CD2, i_085cd2),
    (0x085CD6, i_085cd6),
    (0x085CD8, i_085cd8),
    (0x085CDC, i_085cdc),
    (0x085CE0, i_085ce0),
    (0x085CE6, i_085ce6),
    (0x085CE8, i_085ce8),
    (0x085CEC, i_085cec),
    (0x085CF2, i_085cf2),
    (0x085CF6, i_085cf6),
    (0x085CFC, i_085cfc),
    (0x085D02, i_085d02),
    (0x085D04, i_085d04),
    (0x085D0A, i_085d0a),
    (0x085D0C, i_085d0c),
    (0x085D10, i_085d10),
    (0x085D14, i_085d14),
    (0x085D18, i_085d18),
    (0x085D1C, i_085d1c),
    (0x085D1E, i_085d1e),
    (0x085D22, i_085d22),
    (0x085D28, i_085d28),
    (0x085D2A, i_085d2a),
    (0x085D30, i_085d30),
    (0x085D32, i_085d32),
    (0x085D36, i_085d36),
    (0x085D3A, i_085d3a),
    (0x085D40, i_085d40),
    (0x085D42, i_085d42),
    (0x085D46, i_085d46),
    (0x085D4A, i_085d4a),
    (0x085D4E, i_085d4e),
    (0x085D52, i_085d52),
    (0x085D56, i_085d56),
    (0x085D5A, i_085d5a),
    (0x085D5E, i_085d5e),
    (0x085D62, i_085d62),
    (0x085D68, i_085d68),
    (0x085D6E, i_085d6e),
    (0x085D74, i_085d74),
    (0x085D7A, i_085d7a),
    (0x085D80, i_085d80),
    (0x085D86, i_085d86),
    (0x085D8C, i_085d8c),
    (0x085D90, i_085d90),
    (0x085D96, i_085d96),
    (0x085D9C, i_085d9c),
    (0x085DA0, i_085da0),
    (0x085DA2, i_085da2),
    (0x085DA6, i_085da6),
    (0x085DA8, i_085da8),
    (0x085DAE, i_085dae),
    (0x085DB2, i_085db2),
    (0x085DB8, i_085db8),
    (0x085DBE, i_085dbe),
    (0x085DC2, i_085dc2),
    (0x085DC8, i_085dc8),
    (0x085DCE, i_085dce),
    (0x085DD2, i_085dd2),
    (0x085DD8, i_085dd8),
    (0x085DDE, i_085dde),
    (0x085DE4, i_085de4),
    (0x085DE8, i_085de8),
    (0x085DEE, i_085dee),
    (0x085DF2, i_085df2),
    (0x085DF6, i_085df6),
    (0x085DFC, i_085dfc),
    (0x085E00, i_085e00),
    (0x085E04, i_085e04),
    (0x085E08, i_085e08),
    (0x085E0A, i_085e0a),
    (0x085E0E, i_085e0e),
    (0x085E12, i_085e12),
    (0x085E14, i_085e14),
    (0x085E18, i_085e18),
    (0x085E1C, i_085e1c),
    (0x085E22, i_085e22),
    (0x085E26, i_085e26),
    (0x085E2A, i_085e2a),
    (0x085E30, i_085e30),
    (0x085E36, i_085e36),
    (0x085E38, i_085e38),
    (0x085E3A, i_085e3a),
    (0x085E3E, i_085e3e),
    (0x085E42, i_085e42),
    (0x085E44, i_085e44),
    (0x085E48, i_085e48),
    (0x085E4C, i_085e4c),
    (0x085E4E, i_085e4e),
    (0x085E52, i_085e52),
    (0x085E54, i_085e54),
    (0x085E58, i_085e58),
    (0x085E5C, i_085e5c),
    (0x085E62, i_085e62),
    (0x085E64, i_085e64),
    (0x085E6A, i_085e6a),
    (0x085E6E, i_085e6e),
    (0x085E72, i_085e72),
    (0x085E76, i_085e76),
    (0x085E7C, i_085e7c),
    (0x085E80, i_085e80),
    (0x085E84, i_085e84),
    (0x085E88, i_085e88),
    (0x085E8E, i_085e8e),
    (0x085E92, i_085e92),
    (0x085E98, i_085e98),
    (0x085E9C, i_085e9c),
    (0x085EA0, i_085ea0),
    (0x085EA6, i_085ea6),
    (0x085EAA, i_085eaa),
    (0x085EB0, i_085eb0),
    (0x085EB4, i_085eb4),
    (0x085EB6, i_085eb6),
    (0x085EBA, i_085eba),
    (0x085EBC, i_085ebc),
    (0x085EC2, i_085ec2),
    (0x085EC6, i_085ec6),
    (0x085ECA, i_085eca),
    (0x085ECE, i_085ece),
    (0x085ED2, i_085ed2),
    (0x085ED6, i_085ed6),
    (0x085EDA, i_085eda),
    (0x085EDE, i_085ede),
    (0x085EE2, i_085ee2),
    (0x085EE6, i_085ee6),
    (0x085EE8, i_085ee8),
    (0x085EEE, i_085eee),
    (0x085EF2, i_085ef2),
    (0x085EF8, i_085ef8),
    (0x085EFC, i_085efc),
    (0x085F00, i_085f00),
    (0x085F06, i_085f06),
    (0x085F08, i_085f08),
    (0x085F0E, i_085f0e),
    (0x085F12, i_085f12),
    (0x085F16, i_085f16),
    (0x085F1A, i_085f1a),
    (0x085F1E, i_085f1e),
    (0x085F22, i_085f22),
    (0x085F26, i_085f26),
    (0x085F28, i_085f28),
    (0x085F2C, i_085f2c),
    (0x085F30, i_085f30),
    (0x085F34, i_085f34),
    (0x085F3A, i_085f3a),
    (0x085F3E, i_085f3e),
    (0x085F42, i_085f42),
    (0x085F44, i_085f44),
    (0x085F48, i_085f48),
    (0x085F4C, i_085f4c),
    (0x085F50, i_085f50),
    (0x085F54, i_085f54),
    (0x085F58, i_085f58),
    (0x085F5E, i_085f5e),
    (0x085F60, i_085f60),
    (0x085F64, i_085f64),
    (0x085F68, i_085f68),
    (0x085F6E, i_085f6e),
    (0x085F70, i_085f70),
    (0x085F72, i_085f72),
    (0x085F76, i_085f76),
    (0x085F7A, i_085f7a),
    (0x085F7C, i_085f7c),
    (0x085F7E, i_085f7e),
    (0x085F80, i_085f80),
    (0x085F84, i_085f84),
    (0x085F88, i_085f88),
    (0x085F8C, i_085f8c),
    (0x085F8E, i_085f8e),
    (0x085F92, i_085f92),
    (0x085F96, i_085f96),
    (0x085F98, i_085f98),
    (0x085F9C, i_085f9c),
    (0x085F9E, i_085f9e),
    (0x085FA0, i_085fa0),
    (0x085FA4, i_085fa4),
    (0x085FA8, i_085fa8),
    (0x085FAA, i_085faa),
    (0x085FAE, i_085fae),
    (0x085FB0, i_085fb0),
    (0x085FB6, i_085fb6),
    (0x085FBA, i_085fba),
    (0x085FBE, i_085fbe),
    (0x085FC2, i_085fc2),
    (0x085FC8, i_085fc8),
    (0x085FCC, i_085fcc),
    (0x085FD0, i_085fd0),
    (0x085FD4, i_085fd4),
    (0x085FD8, i_085fd8),
    (0x085FDE, i_085fde),
    (0x085FE4, i_085fe4),
    (0x085FEA, i_085fea),
    (0x085FEE, i_085fee),
    (0x085FF4, i_085ff4),
    (0x085FF8, i_085ff8),
    (0x085FFE, i_085ffe),
    (0x086002, i_086002),
    (0x086006, i_086006),
    (0x086008, i_086008),
    (0x08600C, i_08600c),
    (0x086010, i_086010),
    (0x086014, i_086014),
    (0x086018, i_086018),
    (0x08601A, i_08601a),
    (0x08601C, i_08601c),
    (0x086020, i_086020),
    (0x086024, i_086024),
    (0x086028, i_086028),
    (0x08602C, i_08602c),
    (0x086030, i_086030),
    (0x086034, i_086034),
    (0x08603A, i_08603a),
    (0x08603C, i_08603c),
    (0x086040, i_086040),
    (0x086044, i_086044),
    (0x086048, i_086048),
    (0x08604E, i_08604e),
    (0x086050, i_086050),
    (0x086056, i_086056),
    (0x08605C, i_08605c),
    (0x086060, i_086060),
    (0x086064, i_086064),
    (0x08606A, i_08606a),
    (0x08606E, i_08606e),
    (0x086074, i_086074),
    (0x086076, i_086076),
    (0x08607C, i_08607c),
    (0x086084, i_086084),
    (0x086088, i_086088),
    (0x08608E, i_08608e),
    (0x086090, i_086090),
    (0x086094, i_086094),
    (0x086098, i_086098),
    (0x08609C, i_08609c),
    (0x0860A2, i_0860a2),
    (0x0860A8, i_0860a8),
    (0x0860AC, i_0860ac),
    (0x0860B0, i_0860b0),
    (0x0860B4, i_0860b4),
    (0x0860B8, i_0860b8),
    (0x0860BC, i_0860bc),
    (0x0860C0, i_0860c0),
    (0x0860C6, i_0860c6),
    (0x0860CC, i_0860cc),
    (0x0860D0, i_0860d0),
    (0x0860D4, i_0860d4),
    (0x0860DA, i_0860da),
    (0x0860E0, i_0860e0),
    (0x0860E2, i_0860e2),
    (0x0860E4, i_0860e4),
    (0x0860E6, i_0860e6),
    (0x0860EA, i_0860ea),
    (0x0860EE, i_0860ee),
    (0x0860F4, i_0860f4),
    (0x0860FA, i_0860fa),
    (0x0860FE, i_0860fe),
    (0x086102, i_086102),
    (0x086104, i_086104),
    (0x086108, i_086108),
    (0x08610A, i_08610a),
    (0x08610C, i_08610c),
    (0x086112, i_086112),
    (0x086114, i_086114),
    (0x08611A, i_08611a),
    (0x08611E, i_08611e),
    (0x086122, i_086122),
    (0x086126, i_086126),
    (0x08612A, i_08612a),
    (0x086130, i_086130),
    (0x086136, i_086136),
    (0x08613A, i_08613a),
    (0x08613E, i_08613e),
    (0x086142, i_086142),
    (0x086146, i_086146),
    (0x08614A, i_08614a),
    (0x08614E, i_08614e),
    (0x086154, i_086154),
    (0x08615A, i_08615a),
    (0x08615E, i_08615e),
    (0x086162, i_086162),
    (0x086166, i_086166),
    (0x08616A, i_08616a),
    (0x08616E, i_08616e),
    (0x086172, i_086172),
    (0x086178, i_086178),
    (0x08617E, i_08617e),
    (0x086182, i_086182),
    (0x086186, i_086186),
    (0x08618C, i_08618c),
    (0x086192, i_086192),
    (0x086194, i_086194),
    (0x086196, i_086196),
    (0x08619C, i_08619c),
    (0x0861A2, i_0861a2),
    (0x0861A8, i_0861a8),
    (0x0861AE, i_0861ae),
    (0x0861B4, i_0861b4),
    (0x0861BA, i_0861ba),
    (0x0861C0, i_0861c0),
    (0x0861C6, i_0861c6),
    (0x0861CC, i_0861cc),
    (0x0861D2, i_0861d2),
    (0x0861D8, i_0861d8),
    (0x0861DE, i_0861de),
    (0x0861E4, i_0861e4),
    (0x0861EA, i_0861ea),
    (0x0861F0, i_0861f0),
    (0x0861F6, i_0861f6),
    (0x0861FC, i_0861fc),
    (0x086202, i_086202),
    (0x086208, i_086208),
    (0x08620E, i_08620e),
    (0x086214, i_086214),
    (0x08621A, i_08621a),
    (0x086220, i_086220),
    (0x086226, i_086226),
    (0x08622C, i_08622c),
    (0x086232, i_086232),
    (0x086238, i_086238),
    (0x08623E, i_08623e),
    (0x086244, i_086244),
    (0x08624A, i_08624a),
    (0x086250, i_086250),
    (0x086256, i_086256),
    (0x08625C, i_08625c),
    (0x086262, i_086262),
    (0x086268, i_086268),
    (0x08626E, i_08626e),
    (0x086274, i_086274),
    (0x08627A, i_08627a),
    (0x086280, i_086280),
    (0x086286, i_086286),
    (0x08628C, i_08628c),
    (0x086292, i_086292),
    (0x086298, i_086298),
    (0x08629E, i_08629e),
    (0x0862A4, i_0862a4),
    (0x0862AA, i_0862aa),
    (0x0862B0, i_0862b0),
    (0x0862B6, i_0862b6),
    (0x0862BC, i_0862bc),
    (0x0862C2, i_0862c2),
    (0x0862C8, i_0862c8),
    (0x0862CE, i_0862ce),
    (0x0862D4, i_0862d4),
    (0x0862DA, i_0862da),
    (0x0862E0, i_0862e0),
    (0x0862E6, i_0862e6),
    (0x0862EC, i_0862ec),
    (0x0862F2, i_0862f2),
    (0x0862F8, i_0862f8),
    (0x0862FE, i_0862fe),
    (0x086300, i_086300),
    (0x086302, i_086302),
    (0x086306, i_086306),
    (0x08630A, i_08630a),
    (0x086310, i_086310),
    (0x086316, i_086316),
    (0x08631A, i_08631a),
    (0x08631E, i_08631e),
    (0x086320, i_086320),
    (0x086324, i_086324),
    (0x086326, i_086326),
    (0x086328, i_086328),
    (0x08632A, i_08632a),
    (0x08632E, i_08632e),
    (0x086332, i_086332),
    (0x086336, i_086336),
    (0x08633A, i_08633a),
    (0x086340, i_086340),
    (0x086346, i_086346),
    (0x08634A, i_08634a),
    (0x08634E, i_08634e),
    (0x086352, i_086352),
    (0x086356, i_086356),
    (0x086358, i_086358),
    (0x08635C, i_08635c),
    (0x086360, i_086360),
    (0x086362, i_086362),
    (0x086364, i_086364),
    (0x086366, i_086366),
    (0x08636A, i_08636a),
    (0x08636E, i_08636e),
    (0x086372, i_086372),
    (0x086376, i_086376),
    (0x08637C, i_08637c),
    (0x086382, i_086382),
    (0x086386, i_086386),
    (0x08638A, i_08638a),
    (0x08638E, i_08638e),
    (0x086392, i_086392),
    (0x086394, i_086394),
    (0x086398, i_086398),
    (0x08639C, i_08639c),
    (0x08639E, i_08639e),
    (0x0863A0, i_0863a0),
    (0x0863A4, i_0863a4),
    (0x0863AA, i_0863aa),
    (0x0863AE, i_0863ae),
    (0x0863B4, i_0863b4),
    (0x0863B6, i_0863b6),
    (0x0863BC, i_0863bc),
    (0x0863BE, i_0863be),
    (0x0863C4, i_0863c4),
    (0x0863C8, i_0863c8),
    (0x0863CE, i_0863ce),
    (0x0863D4, i_0863d4),
    (0x0863D6, i_0863d6),
    (0x0863DC, i_0863dc),
    (0x0863E2, i_0863e2),
    (0x0863E4, i_0863e4),
    (0x0863EA, i_0863ea),
    (0x0863F0, i_0863f0),
    (0x0863F2, i_0863f2),
    (0x0863F8, i_0863f8),
    (0x0863FE, i_0863fe),
    (0x086400, i_086400),
    (0x086406, i_086406),
    (0x08640C, i_08640c),
    (0x08640E, i_08640e),
    (0x086414, i_086414),
    (0x08641A, i_08641a),
    (0x08641C, i_08641c),
    (0x086422, i_086422),
    (0x086428, i_086428),
    (0x08642A, i_08642a),
    (0x086430, i_086430),
    (0x086436, i_086436),
    (0x086438, i_086438),
    (0x08643E, i_08643e),
    (0x086444, i_086444),
    (0x086446, i_086446),
    (0x08644C, i_08644c),
    (0x086452, i_086452),
    (0x086454, i_086454),
    (0x08645A, i_08645a),
    (0x086460, i_086460),
    (0x086462, i_086462),
    (0x086468, i_086468),
    (0x08646E, i_08646e),
    (0x086470, i_086470),
    (0x086476, i_086476),
    (0x08647C, i_08647c),
    (0x08647E, i_08647e),
    (0x086484, i_086484),
    (0x08648A, i_08648a),
    (0x08648C, i_08648c),
    (0x086492, i_086492),
    (0x086498, i_086498),
    (0x08649A, i_08649a),
    (0x0864A0, i_0864a0),
    (0x0864A6, i_0864a6),
    (0x0864A8, i_0864a8),
    (0x0864AE, i_0864ae),
    (0x0864B4, i_0864b4),
    (0x0864B6, i_0864b6),
    (0x0864BC, i_0864bc),
    (0x0864BE, i_0864be),
    (0x0864C2, i_0864c2),
    (0x0864C4, i_0864c4),
    (0x0864C8, i_0864c8),
    (0x0864CE, i_0864ce),
    (0x0864D0, i_0864d0),
    (0x0864D6, i_0864d6),
    (0x0864D8, i_0864d8),
    (0x0864DC, i_0864dc),
    (0x0864DE, i_0864de),
    (0x0864E2, i_0864e2),
    (0x0864E8, i_0864e8),
    (0x0864EA, i_0864ea),
    (0x0864F0, i_0864f0),
    (0x0864F2, i_0864f2),
    (0x0864F6, i_0864f6),
    (0x0864F8, i_0864f8),
    (0x0864FC, i_0864fc),
    (0x086502, i_086502),
    (0x086504, i_086504),
    (0x08650A, i_08650a),
    (0x08650C, i_08650c),
    (0x086510, i_086510),
    (0x086512, i_086512),
    (0x086516, i_086516),
    (0x08651C, i_08651c),
    (0x08651E, i_08651e),
    (0x086524, i_086524),
    (0x086526, i_086526),
    (0x08652A, i_08652a),
    (0x08652C, i_08652c),
    (0x086530, i_086530),
    (0x086536, i_086536),
    (0x086538, i_086538),
    (0x08653E, i_08653e),
    (0x086540, i_086540),
    (0x086544, i_086544),
    (0x086546, i_086546),
    (0x08654A, i_08654a),
    (0x086550, i_086550),
    (0x086552, i_086552),
    (0x086556, i_086556),
    (0x08655A, i_08655a),
    (0x08655E, i_08655e),
    (0x086562, i_086562),
    (0x086566, i_086566),
    (0x086568, i_086568),
    (0x08656C, i_08656c),
    (0x08656E, i_08656e),
    (0x086574, i_086574),
    (0x086578, i_086578),
    (0x08657E, i_08657e),
    (0x086582, i_086582),
    (0x086586, i_086586),
    (0x08658C, i_08658c),
    (0x086590, i_086590),
    (0x086596, i_086596),
    (0x08659C, i_08659c),
    (0x0865A2, i_0865a2),
    (0x0865A8, i_0865a8),
    (0x0865AE, i_0865ae),
    (0x0865B6, i_0865b6),
    (0x0865BA, i_0865ba),
    (0x0865BC, i_0865bc),
    (0x0865BE, i_0865be),
    (0x0865C6, i_0865c6),
    (0x0865CA, i_0865ca),
    (0x0865CE, i_0865ce),
    (0x0865D4, i_0865d4),
    (0x0865DA, i_0865da),
    (0x0865DC, i_0865dc),
    (0x0865E0, i_0865e0),
    (0x0865E6, i_0865e6),
    (0x0865EC, i_0865ec),
    (0x0865F0, i_0865f0),
    (0x0865F6, i_0865f6),
    (0x0865FA, i_0865fa),
    (0x086600, i_086600),
    (0x086604, i_086604),
    (0x086608, i_086608),
    (0x08660E, i_08660e),
    (0x086614, i_086614),
    (0x08661A, i_08661a),
    (0x086620, i_086620),
    (0x086624, i_086624),
    (0x086628, i_086628),
    (0x08662E, i_08662e),
    (0x086634, i_086634),
    (0x086638, i_086638),
    (0x08663E, i_08663e),
    (0x086642, i_086642),
    (0x086648, i_086648),
    (0x08664C, i_08664c),
    (0x086650, i_086650),
    (0x086656, i_086656),
    (0x08665C, i_08665c),
    (0x086662, i_086662),
    (0x086666, i_086666),
    (0x08666A, i_08666a),
    (0x086670, i_086670),
    (0x086676, i_086676),
    (0x08667A, i_08667a),
    (0x086680, i_086680),
    (0x086684, i_086684),
    (0x08668A, i_08668a),
    (0x08668E, i_08668e),
    (0x086692, i_086692),
    (0x086698, i_086698),
    (0x08669E, i_08669e),
    (0x0866A4, i_0866a4),
    (0x0866AA, i_0866aa),
    (0x0866AE, i_0866ae),
    (0x0866B2, i_0866b2),
    (0x0866B8, i_0866b8),
    (0x0866BE, i_0866be),
    (0x0866C2, i_0866c2),
    (0x0866C8, i_0866c8),
    (0x0866CC, i_0866cc),
    (0x0866D2, i_0866d2),
    (0x0866D6, i_0866d6),
    (0x0866DA, i_0866da),
    (0x0866E0, i_0866e0),
    (0x0866E6, i_0866e6),
    (0x0866EC, i_0866ec),
    (0x0866F0, i_0866f0),
    (0x0866F4, i_0866f4),
    (0x0866FA, i_0866fa),
    (0x086700, i_086700),
    (0x086704, i_086704),
    (0x08670A, i_08670a),
    (0x08670E, i_08670e),
    (0x086714, i_086714),
    (0x086718, i_086718),
    (0x08671C, i_08671c),
    (0x086722, i_086722),
    (0x086728, i_086728),
    (0x08672E, i_08672e),
    (0x086734, i_086734),
    (0x086738, i_086738),
    (0x08673E, i_08673e),
    (0x086744, i_086744),
    (0x08674A, i_08674a),
    (0x086750, i_086750),
    (0x086754, i_086754),
    (0x086758, i_086758),
    (0x08675A, i_08675a),
    (0x086760, i_086760),
    (0x086764, i_086764),
    (0x086768, i_086768),
    (0x08676C, i_08676c),
    (0x086772, i_086772),
    (0x086778, i_086778),
    (0x08677C, i_08677c),
    (0x086782, i_086782),
    (0x086788, i_086788),
    (0x08678E, i_08678e),
    (0x086794, i_086794),
    (0x086798, i_086798),
    (0x08679E, i_08679e),
    (0x0867A4, i_0867a4),
    (0x0867A8, i_0867a8),
    (0x0867AC, i_0867ac),
    (0x0867B0, i_0867b0),
    (0x0867B6, i_0867b6),
    (0x0867BA, i_0867ba),
    (0x0867BC, i_0867bc),
    (0x0867C2, i_0867c2),
    (0x0867C6, i_0867c6),
    (0x0867CC, i_0867cc),
    (0x0867D2, i_0867d2),
    (0x0867D4, i_0867d4),
    (0x0867DA, i_0867da),
    (0x0867DE, i_0867de),
    (0x0867E4, i_0867e4),
    (0x0867EA, i_0867ea),
    (0x0867F0, i_0867f0),
    (0x0867F6, i_0867f6),
    (0x0867FC, i_0867fc),
    (0x086802, i_086802),
    (0x086808, i_086808),
    (0x08680C, i_08680c),
    (0x086810, i_086810),
    (0x086816, i_086816),
    (0x08681C, i_08681c),
    (0x086822, i_086822),
    (0x086826, i_086826),
    (0x08682C, i_08682c),
    (0x086832, i_086832),
    (0x086838, i_086838),
    (0x08683C, i_08683c),
    (0x086840, i_086840),
    (0x086846, i_086846),
    (0x08684C, i_08684c),
    (0x086850, i_086850),
    (0x086852, i_086852),
    (0x086854, i_086854),
    (0x086858, i_086858),
    (0x08685C, i_08685c),
    (0x086862, i_086862),
    (0x086868, i_086868),
    (0x08686A, i_08686a),
    (0x08686E, i_08686e),
    (0x086874, i_086874),
    (0x08687A, i_08687a),
    (0x08687E, i_08687e),
    (0x086882, i_086882),
    (0x086888, i_086888),
    (0x08688E, i_08688e),
    (0x086892, i_086892),
    (0x086896, i_086896),
    (0x08689C, i_08689c),
    (0x0868A2, i_0868a2),
    (0x0868A6, i_0868a6),
    (0x0868AA, i_0868aa),
    (0x0868B0, i_0868b0),
    (0x0868B6, i_0868b6),
    (0x0868BA, i_0868ba),
    (0x0868BE, i_0868be),
    (0x0868C4, i_0868c4),
    (0x0868CA, i_0868ca),
    (0x0868CE, i_0868ce),
    (0x0868D2, i_0868d2),
    (0x0868D8, i_0868d8),
    (0x0868DE, i_0868de),
    (0x0868E2, i_0868e2),
    (0x0868E8, i_0868e8),
    (0x0868EE, i_0868ee),
    (0x0868F4, i_0868f4),
    (0x0868F8, i_0868f8),
    (0x0868FC, i_0868fc),
    (0x086900, i_086900),
    (0x086906, i_086906),
    (0x08690C, i_08690c),
    (0x086912, i_086912),
    (0x086914, i_086914),
    (0x086918, i_086918),
    (0x08691A, i_08691a),
    (0x086920, i_086920),
    (0x086922, i_086922),
    (0x086926, i_086926),
    (0x08692C, i_08692c),
    (0x086930, i_086930),
    (0x086936, i_086936),
    (0x08693A, i_08693a),
    (0x08693E, i_08693e),
    (0x086942, i_086942),
    (0x086944, i_086944),
    (0x086948, i_086948),
    (0x08694E, i_08694e),
    (0x086952, i_086952),
    (0x086958, i_086958),
    (0x08695E, i_08695e),
    (0x086964, i_086964),
    (0x086968, i_086968),
    (0x08696C, i_08696c),
    (0x086972, i_086972),
    (0x086978, i_086978),
    (0x08697C, i_08697c),
    (0x086982, i_086982),
    (0x086986, i_086986),
    (0x08698A, i_08698a),
    (0x08698C, i_08698c),
    (0x086990, i_086990),
    (0x086996, i_086996),
    (0x08699C, i_08699c),
    (0x0869A2, i_0869a2),
    (0x0869A4, i_0869a4),
    (0x0869A6, i_0869a6),
    (0x0869AA, i_0869aa),
    (0x0869AC, i_0869ac),
    (0x0869AE, i_0869ae),
    (0x0869B4, i_0869b4),
    (0x0869B6, i_0869b6),
    (0x0869BA, i_0869ba),
    (0x0869C0, i_0869c0),
    (0x0869C4, i_0869c4),
    (0x0869CA, i_0869ca),
    (0x0869CE, i_0869ce),
    (0x0869D0, i_0869d0),
    (0x0869D4, i_0869d4),
    (0x0869DA, i_0869da),
    (0x0869DE, i_0869de),
    (0x0869E4, i_0869e4),
    (0x0869EA, i_0869ea),
    (0x0869F0, i_0869f0),
    (0x0869F4, i_0869f4),
    (0x0869F8, i_0869f8),
    (0x0869FE, i_0869fe),
    (0x086A04, i_086a04),
    (0x086A08, i_086a08),
    (0x086A0E, i_086a0e),
    (0x086A12, i_086a12),
    (0x086A16, i_086a16),
    (0x086A18, i_086a18),
    (0x086A1C, i_086a1c),
    (0x086A22, i_086a22),
    (0x086A28, i_086a28),
    (0x086A2E, i_086a2e),
    (0x086A30, i_086a30),
    (0x086A32, i_086a32),
    (0x086A36, i_086a36),
    (0x086A38, i_086a38),
    (0x086A3A, i_086a3a),
    (0x086A40, i_086a40),
    (0x086A42, i_086a42),
    (0x086A46, i_086a46),
    (0x086A4C, i_086a4c),
    (0x086A50, i_086a50),
    (0x086A56, i_086a56),
    (0x086A5A, i_086a5a),
    (0x086A5C, i_086a5c),
    (0x086A60, i_086a60),
    (0x086A66, i_086a66),
    (0x086A6A, i_086a6a),
    (0x086A70, i_086a70),
    (0x086A76, i_086a76),
    (0x086A7C, i_086a7c),
    (0x086A80, i_086a80),
    (0x086A84, i_086a84),
    (0x086A8A, i_086a8a),
    (0x086A90, i_086a90),
    (0x086A94, i_086a94),
    (0x086A9A, i_086a9a),
    (0x086A9E, i_086a9e),
    (0x086AA2, i_086aa2),
    (0x086AA4, i_086aa4),
    (0x086AA8, i_086aa8),
    (0x086AAE, i_086aae),
    (0x086AB4, i_086ab4),
    (0x086ABA, i_086aba),
    (0x086ABC, i_086abc),
    (0x086AC0, i_086ac0),
    (0x086AC2, i_086ac2),
    (0x086AC6, i_086ac6),
    (0x086ACC, i_086acc),
    (0x086AD0, i_086ad0),
    (0x086AD6, i_086ad6),
    (0x086ADC, i_086adc),
    (0x086AE2, i_086ae2),
    (0x086AE6, i_086ae6),
    (0x086AEA, i_086aea),
    (0x086AF0, i_086af0),
    (0x086AF6, i_086af6),
    (0x086AFA, i_086afa),
    (0x086B00, i_086b00),
    (0x086B04, i_086b04),
    (0x086B0A, i_086b0a),
    (0x086B0E, i_086b0e),
    (0x086B12, i_086b12),
    (0x086B18, i_086b18),
    (0x086B1C, i_086b1c),
    (0x086B20, i_086b20),
    (0x086B22, i_086b22),
    (0x086B26, i_086b26),
    (0x086B2C, i_086b2c),
    (0x086B32, i_086b32),
    (0x086B36, i_086b36),
    (0x086B3C, i_086b3c),
    (0x086B42, i_086b42),
    (0x086B48, i_086b48),
    (0x086B4E, i_086b4e),
    (0x086B54, i_086b54),
    (0x086B58, i_086b58),
    (0x086B5E, i_086b5e),
    (0x086B64, i_086b64),
    (0x086B6A, i_086b6a),
    (0x086B70, i_086b70),
    (0x086B74, i_086b74),
    (0x086B76, i_086b76),
    (0x086B7C, i_086b7c),
    (0x086B80, i_086b80),
    (0x086B84, i_086b84),
    (0x086B8A, i_086b8a),
];
