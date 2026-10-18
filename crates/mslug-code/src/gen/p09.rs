// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $090000: move.w #$10, $8c(a6)
pub fn i_090000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90006;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090006: move.w #$40, $8e(a6)
pub fn i_090006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9000C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09000C: move.w #$0, $80(a6)
pub fn i_09000c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90012;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090012: move.w #$0, $84(a6)
pub fn i_090012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90018;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090018: move.w #$2000, d0
pub fn i_090018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9001C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09001C: jsr $28134.l
pub fn i_09001c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90022);
    cpu.pc = 0x28134;
    16
}
/// $090022: andi.w #$ffe3, $38(a6)
pub fn i_090022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90028;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090028: ori.w #$0, $38(a6)
pub fn i_090028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9002E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09002E: move.w #$10e1, d0
pub fn i_09002e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90032;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090032: jsr $2352.l
pub fn i_090032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90038);
    cpu.pc = 0x2352;
    16
}
/// $090038: lea.l $2f5056.l, a0
pub fn i_090038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9003E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5056;
    20
}
/// $09003E: jsr $28cd4.l
pub fn i_09003e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90044);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_090044  ($090044, 8 B, kernel/task_handlers.c)
/// $090044: lea.l $90098(pc), a1
pub fn i_090044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90048;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90098;
    16
}
/// $090048: move.l a1, (a6)
pub fn i_090048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9004A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09004A: rts
pub fn i_09004a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9004C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Sprite_MidB_09004c  ($09004C, 68 B, mission/gameover_continue_08f6d2.s)
/// $09004C: move.w #$dd, d1
pub fn i_09004c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90050;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090050: jsr $236e.l
pub fn i_090050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90056;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90056);
    cpu.pc = 0x236E;
    16
}
/// $090056: move.w #$0, $8c(a6)
pub fn i_090056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9005C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09005C: move.w #$20, $8e(a6)
pub fn i_09005c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90062;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090062: move.w #$1, $80(a6)
pub fn i_090062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90068;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090068: move.w #$0, $84(a6)
pub fn i_090068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9006E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09006E: move.w #$0, d0
pub fn i_09006e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90072;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090072: jsr $28134.l
pub fn i_090072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90078);
    cpu.pc = 0x28134;
    16
}
/// $090078: andi.w #$ffe3, $38(a6)
pub fn i_090078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9007E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09007E: ori.w #$0, $38(a6)
pub fn i_09007e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90084;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090084: lea.l $2f5062.l, a0
pub fn i_090084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9008A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5062;
    20
}
/// $09008A: jsr $28cd4.l
pub fn i_09008a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90090);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_090090  ($090090, 8 B, kernel/task_handlers.c)
/// $090090: lea.l $90098(pc), a1
pub fn i_090090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90094;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90098;
    16
}
/// $090094: move.l a1, (a6)
pub fn i_090094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90096;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090096: rts
pub fn i_090096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90098;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Sprite_FollowParent_090098  ($090098, 68 B, mission/gameover_continue_08f6d2.s)
/// $090098: movea.l $c(a6), a0
pub fn i_090098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9009C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09009C: move.w $22(a0), d0
pub fn i_09009c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900A0;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0900A0: add.w $8c(a6), d0
pub fn i_0900a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900A4;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0900A4: move.w d0, $22(a6)
pub fn i_0900a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0900A8: move.w $24(a0), d0
pub fn i_0900a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900AC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0900AC: add.w $8e(a6), d0
pub fn i_0900ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900B0;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0900B0: move.w d0, $24(a6)
pub fn i_0900b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900B4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0900B4: jsr $28d70.l
pub fn i_0900b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x900BA);
    cpu.pc = 0x28D70;
    16
}
/// $0900BA: tst.w $80(a6)
pub fn i_0900ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900BE;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0900BE: bne.w $900e2
pub fn i_0900be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x900E2; return 10; }
    8
}
/// $0900C2: tst.w $84(a6)
pub fn i_0900c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900C6;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0900C6: bne.w $900e2
pub fn i_0900c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x900E2; return 10; }
    8
}
/// $0900CA: move.w #$ffff, $84(a6)
pub fn i_0900ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0900D0: move.w $14(a6), d1
pub fn i_0900d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0900D4: move.w #$de, d2
pub fn i_0900d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900D8;
    cpu.ir = 0x343C;
    let v1: u32 = 0xDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0900D8: moveq #$2, d3
pub fn i_0900d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900DA;
    cpu.ir = 0x7602;
    cpu.d[3] = 0x2;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0900DA: moveq #$1, d4
pub fn i_0900da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900DC;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_0900dc  ($0900DC, 8 B, thunks/jsr_abs_thunks.c)
/// $0900DC: jsr $2c30.l
pub fn i_0900dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x900E2);
    cpu.pc = 0x2C30;
    16
}
/// $0900E2: rts
pub fn i_0900e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Prop_A_0900e4  ($0900E4, 128 B, mission/gameover_continue_08f6d2.s)
/// $0900E4: move.w #$be, d1
pub fn i_0900e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900E8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xBE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0900E8: jsr $236e.l
pub fn i_0900e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x900EE);
    cpu.pc = 0x236E;
    16
}
/// $0900EE: move.w #$ae, $22(a6)
pub fn i_0900ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xAE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0900F4: move.w #$148, $24(a6)
pub fn i_0900f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x148;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0900FA: move.w #$4000, d0
pub fn i_0900fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0900FE: jsr $28134.l
pub fn i_0900fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90104;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90104);
    cpu.pc = 0x28134;
    16
}
/// $090104: andi.w #$ffe3, $38(a6)
pub fn i_090104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9010A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09010A: ori.w #$0, $38(a6)
pub fn i_09010a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90110;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090110: move.w #$0, $82(a6)
pub fn i_090110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90116;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090116: lea.l $2f5328.l, a0
pub fn i_090116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9011C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5328;
    20
}
/// $09011C: jsr $28cd4.l
pub fn i_09011c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90122;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90122);
    cpu.pc = 0x28CD4;
    16
}
/// $090122: lea.l $90128(pc), a1
pub fn i_090122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90126;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90128;
    16
}
/// $090126: move.l a1, (a6)
pub fn i_090126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90128;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090128: jsr $910d6(pc)
pub fn i_090128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9012C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9012C);
    cpu.pc = 0x910D6;
    16
}
/// $09012C: bcc.w $90138
pub fn i_09012c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90130;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x90138; return 10; }
    8
}
/// $090130: jmp $518.l
pub fn i_090130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90136;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $090138: cmpi.w #$10, $82(a6)
pub fn i_090138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9013E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $09013E: bge.w $90164
pub fn i_09013e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90142;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x90164; return 10; }
    8
}
/// $090142: lea.l $2f4b68.l, a0
pub fn i_090142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90148;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4B68;
    20
}
/// $090148: move.w $82(a6), d0
pub fn i_090148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9014C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09014C: add.w d0, d0
pub fn i_09014c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9014E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09014E: add.w d0, d0
pub fn i_09014e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90150;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090150: move.w (a0, d0.w), d1
pub fn i_090150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90154;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090154: add.w d1, $22(a6)
pub fn i_090154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90158;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090158: move.w $2(a0, d0.w), d1
pub fn i_090158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9015C;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09015C: add.w d1, $24(a6)
pub fn i_09015c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90160;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090160: addq.w #$1, $82(a6)
pub fn i_090160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90164;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrAbsThunk_090164  ($090164, 8 B, thunks/jsr_abs_thunks.c)
/// $090164: jsr $28d70.l
pub fn i_090164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9016A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9016A);
    cpu.pc = 0x28D70;
    16
}
/// $09016A: rts
pub fn i_09016a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9016C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Prop_B_09016c  ($09016C, 394 B, mission/gameover_continue_08f6d2.s)
/// $09016C: move.w #$bd, d1
pub fn i_09016c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90170;
    cpu.ir = 0x323C;
    let v1: u32 = 0xBD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090170: jsr $236e.l
pub fn i_090170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90176;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90176);
    cpu.pc = 0x236E;
    16
}
/// $090176: move.w #$ac, $22(a6)
pub fn i_090176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9017C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xAC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09017C: move.w #$148, $24(a6)
pub fn i_09017c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90182;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x148;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090182: move.w #$8000, d0
pub fn i_090182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90186;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090186: jsr $28134.l
pub fn i_090186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9018C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9018C);
    cpu.pc = 0x28134;
    16
}
/// $09018C: andi.w #$ffe3, $38(a6)
pub fn i_09018c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90192;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090192: ori.w #$0, $38(a6)
pub fn i_090192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90198;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090198: move.w #$0, $82(a6)
pub fn i_090198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9019E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09019E: move.w #$0, $86(a6)
pub fn i_09019e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0901A4: lea.l $2f531c.l, a0
pub fn i_0901a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F531C;
    20
}
/// $0901AA: jsr $28cd4.l
pub fn i_0901aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x901B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0901B0: lea.l $901b6(pc), a1
pub fn i_0901b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x901B6;
    16
}
/// $0901B4: move.l a1, (a6)
pub fn i_0901b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0901B6: jsr $910d6(pc)
pub fn i_0901b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x901BA);
    cpu.pc = 0x910D6;
    16
}
/// $0901BA: bcc.w $901c6
pub fn i_0901ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x901C6; return 10; }
    8
}
/// $0901BE: jmp $518.l
pub fn i_0901be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0901C6: cmpi.w #$10, $82(a6)
pub fn i_0901c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $0901CC: bge.w $90248
pub fn i_0901cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901D0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x90248; return 10; }
    8
}
/// $0901D0: cmpi.w #$a, $82(a6)
pub fn i_0901d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Word, false);
    16
}
/// $0901D6: bne.w $90222
pub fn i_0901d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90222; return 10; }
    8
}
/// $0901DA: move.w $14(a6), d1
pub fn i_0901da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901DE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0901DE: move.w #$100, d2
pub fn i_0901de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901E2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0901E2: moveq #$2, d3
pub fn i_0901e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901E4;
    cpu.ir = 0x7602;
    cpu.d[3] = 0x2;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0901E4: moveq #$3, d4
pub fn i_0901e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901E6;
    cpu.ir = 0x7803;
    cpu.d[4] = 0x3;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0901E6: jsr $2c30.l
pub fn i_0901e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x901EC);
    cpu.pc = 0x2C30;
    16
}
/// $0901EC: move.w #$10, d1
pub fn i_0901ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0901F0: move.w #$19b, d2
pub fn i_0901f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901F4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0901F4: moveq #$2, d3
pub fn i_0901f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901F6;
    cpu.ir = 0x7602;
    cpu.d[3] = 0x2;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0901F6: moveq #$3, d4
pub fn i_0901f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901F8;
    cpu.ir = 0x7803;
    cpu.d[4] = 0x3;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0901F8: jsr $2c26.l
pub fn i_0901f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x901FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x901FE);
    cpu.pc = 0x2C26;
    16
}
/// $0901FE: move.w #$11, d1
pub fn i_0901fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90202;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090202: move.w #$19c, d2
pub fn i_090202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90206;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090206: moveq #$2, d3
pub fn i_090206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90208;
    cpu.ir = 0x7602;
    cpu.d[3] = 0x2;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090208: moveq #$3, d4
pub fn i_090208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9020A;
    cpu.ir = 0x7803;
    cpu.d[4] = 0x3;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09020A: jsr $2c26.l
pub fn i_09020a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90210;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90210);
    cpu.pc = 0x2C26;
    16
}
/// $090210: move.w #$12, d1
pub fn i_090210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90214;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090214: move.w #$19d, d2
pub fn i_090214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90218;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090218: moveq #$2, d3
pub fn i_090218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9021A;
    cpu.ir = 0x7602;
    cpu.d[3] = 0x2;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09021A: moveq #$3, d4
pub fn i_09021a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9021C;
    cpu.ir = 0x7803;
    cpu.d[4] = 0x3;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09021C: jsr $2c26.l
pub fn i_09021c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90222);
    cpu.pc = 0x2C26;
    16
}
/// $090222: lea.l $2f4ba8.l, a0
pub fn i_090222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90228;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4BA8;
    20
}
/// $090228: move.w $82(a6), d0
pub fn i_090228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9022C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09022C: add.w d0, d0
pub fn i_09022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9022E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09022E: add.w d0, d0
pub fn i_09022e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90230;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090230: move.w (a0, d0.w), d1
pub fn i_090230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90234;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090234: add.w d1, $22(a6)
pub fn i_090234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90238;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090238: move.w $2(a0, d0.w), d1
pub fn i_090238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9023C;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09023C: add.w d1, $24(a6)
pub fn i_09023c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90240;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090240: addq.w #$1, $82(a6)
pub fn i_090240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90244;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090244: bra.w $902f6
pub fn i_090244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90248;
    cpu.ir = 0x6000;
    cpu.pc = 0x902F6;
    10
}
/// $090248: cmpi.w #$4, $86(a6)
pub fn i_090248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9024E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $09024E: bge.w $902f6
pub fn i_09024e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90252;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x902F6; return 10; }
    8
}
/// $090252: cmpi.w #$0, $86(a6)
pub fn i_090252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90258;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $090258: bne.w $90282
pub fn i_090258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9025C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90282; return 10; }
    8
}
/// $09025C: lea.l $902fe(pc), a1
pub fn i_09025c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90260;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x902FE;
    16
}
/// $090260: jsr $4ae.l
pub fn i_090260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90266);
    cpu.pc = 0x4AE;
    16
}
/// $090266: jsr $5dd02.l
pub fn i_090266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9026C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9026C);
    cpu.pc = 0x5DD02;
    16
}
/// $09026C: move.w #$101, d1
pub fn i_09026c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90270;
    cpu.ir = 0x323C;
    let v1: u32 = 0x101;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090270: jsr $236e.l
pub fn i_090270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90276;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90276);
    cpu.pc = 0x236E;
    16
}
/// $090276: lea.l $2f4a38.l, a0
pub fn i_090276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9027C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4A38;
    20
}
/// $09027C: jsr $2b58.l
pub fn i_09027c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90282;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90282);
    cpu.pc = 0x2B58;
    16
}
/// $090282: cmpi.w #$2, $86(a6)
pub fn i_090282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90288;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $090288: bne.w $902d4
pub fn i_090288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9028C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x902D4; return 10; }
    8
}
/// $09028C: move.w $14(a6), d1
pub fn i_09028c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90290;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090290: move.w #$100, d2
pub fn i_090290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90294;
    cpu.ir = 0x343C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090294: moveq #$1, d3
pub fn i_090294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90296;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090296: moveq #$6, d4
pub fn i_090296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90298;
    cpu.ir = 0x7806;
    cpu.d[4] = 0x6;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090298: jsr $2c30.l
pub fn i_090298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9029E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9029E);
    cpu.pc = 0x2C30;
    16
}
/// $09029E: move.w #$10, d1
pub fn i_09029e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902A2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0902A2: move.w #$19b, d2
pub fn i_0902a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902A6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0902A6: moveq #$1, d3
pub fn i_0902a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902A8;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0902A8: moveq #$6, d4
pub fn i_0902a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902AA;
    cpu.ir = 0x7806;
    cpu.d[4] = 0x6;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0902AA: jsr $2c26.l
pub fn i_0902aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x902B0);
    cpu.pc = 0x2C26;
    16
}
/// $0902B0: move.w #$11, d1
pub fn i_0902b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0902B4: move.w #$19c, d2
pub fn i_0902b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902B8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0902B8: moveq #$1, d3
pub fn i_0902b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902BA;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0902BA: moveq #$6, d4
pub fn i_0902ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902BC;
    cpu.ir = 0x7806;
    cpu.d[4] = 0x6;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0902BC: jsr $2c26.l
pub fn i_0902bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x902C2);
    cpu.pc = 0x2C26;
    16
}
/// $0902C2: move.w #$12, d1
pub fn i_0902c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0902C6: move.w #$19d, d2
pub fn i_0902c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902CA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0902CA: moveq #$1, d3
pub fn i_0902ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902CC;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0902CC: moveq #$6, d4
pub fn i_0902cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902CE;
    cpu.ir = 0x7806;
    cpu.d[4] = 0x6;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0902CE: jsr $2c26.l
pub fn i_0902ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x902D4);
    cpu.pc = 0x2C26;
    16
}
/// $0902D4: lea.l $2f4be8.l, a0
pub fn i_0902d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4BE8;
    20
}
/// $0902DA: move.w $86(a6), d0
pub fn i_0902da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0902DE: add.w d0, d0
pub fn i_0902de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902E0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0902E0: add.w d0, d0
pub fn i_0902e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902E2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0902E2: move.w (a0, d0.w), d1
pub fn i_0902e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902E6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0902E6: add.w d1, $22(a6)
pub fn i_0902e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902EA;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0902EA: move.w $2(a0, d0.w), d1
pub fn i_0902ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902EE;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0902EE: add.w d1, $24(a6)
pub fn i_0902ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902F2;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0902F2: addq.w #$1, $86(a6)
pub fn i_0902f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902F6;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrAbsThunk_0902f6  ($0902F6, 8 B, thunks/jsr_abs_thunks.c)
/// $0902F6: jsr $28d70.l
pub fn i_0902f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x902FC);
    cpu.pc = 0x28D70;
    16
}
/// $0902FC: rts
pub fn i_0902fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x902FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Prop_C_0902fe  ($0902FE, 80 B, mission/gameover_continue_08f6d2.s)
/// $0902FE: move.w #$ff, d1
pub fn i_0902fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90302;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090302: jsr $236e.l
pub fn i_090302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90308);
    cpu.pc = 0x236E;
    16
}
/// $090308: move.w #$98, $22(a6)
pub fn i_090308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9030E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x98;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09030E: move.w #$198, $24(a6)
pub fn i_09030e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90314;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090314: move.w #$e000, d0
pub fn i_090314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90318;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090318: jsr $28134.l
pub fn i_090318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9031E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9031E);
    cpu.pc = 0x28134;
    16
}
/// $09031E: andi.w #$ffe3, $38(a6)
pub fn i_09031e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90324;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090324: ori.w #$0, $38(a6)
pub fn i_090324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9032A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09032A: lea.l $2f5368.l, a0
pub fn i_09032a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90330;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5368;
    20
}
/// $090330: jsr $28cd4.l
pub fn i_090330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90336);
    cpu.pc = 0x28CD4;
    16
}
/// $090336: lea.l $9033c(pc), a1
pub fn i_090336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9033A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9033C;
    16
}
/// $09033A: move.l a1, (a6)
pub fn i_09033a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9033C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09033C: jsr $28d70.l
pub fn i_09033c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90342);
    cpu.pc = 0x28D70;
    16
}
/// $090342: bcc.w $9034c
pub fn i_090342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90346;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9034C; return 10; }
    8
}
/// $090346: jmp $518.l
pub fn i_090346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9034C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09034C: rts
pub fn i_09034c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9034E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Zoom_09034e  ($09034E, 260 B, mission/gameover_continue_08f6d2.s)
/// $09034E: move.w #$df, d1
pub fn i_09034e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90352;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090352: jsr $236e.l
pub fn i_090352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90358);
    cpu.pc = 0x236E;
    16
}
/// $090358: move.w #$c000, d0
pub fn i_090358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9035C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09035C: jsr $28134.l
pub fn i_09035c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90362);
    cpu.pc = 0x28134;
    16
}
/// $090362: andi.w #$ffe3, $38(a6)
pub fn i_090362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90368;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090368: ori.w #$1c, $38(a6)
pub fn i_090368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9036E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09036E: moveq #$0, d0
pub fn i_09036e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90370;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090370: move.b d0, $21(a6)
pub fn i_090370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90374;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090374: move.b d0, $20(a6)
pub fn i_090374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90378;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090378: move.w #$ec, $88(a6)
pub fn i_090378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9037E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09037E: move.w #$14c, $8a(a6)
pub fn i_09037e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90384;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090384: move.w #$f0, $8c(a6)
pub fn i_090384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9038A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09038A: move.w d0, $8e(a6)
pub fn i_09038a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9038E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09038E: move.w d0, $86(a6)
pub fn i_09038e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90392;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090392: lea.l $2f50b4.l, a0
pub fn i_090392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90398;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F50B4;
    20
}
/// $090398: jsr $28cd4.l
pub fn i_090398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9039E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9039E);
    cpu.pc = 0x28CD4;
    16
}
/// $09039E: lea.l $903a4(pc), a1
pub fn i_09039e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x903A4;
    16
}
/// $0903A2: move.l a1, (a6)
pub fn i_0903a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0903A4: move.w $88(a6), d0
pub fn i_0903a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903A8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0903A8: sub.w $8c(a6), d0
pub fn i_0903a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903AC;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0903AC: move.w d0, $22(a6)
pub fn i_0903ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0903B0: move.w $8a(a6), d0
pub fn i_0903b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0903B4: sub.w $8e(a6), d0
pub fn i_0903b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903B8;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0903B8: move.w d0, $24(a6)
pub fn i_0903b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0903BC: jsr $28d70.l
pub fn i_0903bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x903C2);
    cpu.pc = 0x28D70;
    16
}
/// $0903C2: cmpi.w #$20, $82(a6)
pub fn i_0903c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903C8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $0903C8: bgt.w $9044c
pub fn i_0903c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903CC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9044C; return 10; }
    8
}
/// $0903CC: cmpi.w #$24, $8c(a6)
pub fn i_0903cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Word, false);
    16
}
/// $0903D2: bge.w $903f0
pub fn i_0903d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903D6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x903F0; return 10; }
    8
}
/// $0903D6: tst.b $21(a6)
pub fn i_0903d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903DA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0903DA: bne.w $903f0
pub fn i_0903da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x903F0; return 10; }
    8
}
/// $0903DE: move.b #$ff, $21(a6)
pub fn i_0903de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0903E4: move.w #$158, $8a(a6)
pub fn i_0903e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x158;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0903EA: move.w #$c, $8e(a6)
pub fn i_0903ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0903F0: move.w $8c(a6), d0
pub fn i_0903f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903F4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0903F4: lsr.w #$1, d0
pub fn i_0903f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903F6;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0903F6: tst.w d0
pub fn i_0903f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903F8;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0903F8: bne.w $903fe
pub fn i_0903f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x903FE; return 10; }
    8
}
/// $0903FC: moveq #$1, d0
pub fn i_0903fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x903FE;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0903FE: sub.w d0, $8c(a6)
pub fn i_0903fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90402;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090402: bpl.w $90410
pub fn i_090402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90406;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x90410; return 10; }
    8
}
/// $090406: move.w #$ec, $22(a6)
pub fn i_090406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9040C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09040C: clr.w $8c(a6)
pub fn i_09040c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90410;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $090410: move.w $8e(a6), d0
pub fn i_090410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90414;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090414: lsr.w #$2, d0
pub fn i_090414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90416;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $090416: tst.w d0
pub fn i_090416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90418;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $090418: bne.w $9041e
pub fn i_090418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9041C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9041E; return 10; }
    8
}
/// $09041C: moveq #$1, d0
pub fn i_09041c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9041E;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09041E: sub.w d0, $8e(a6)
pub fn i_09041e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90422;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090422: bpl.w $90430
pub fn i_090422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90426;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x90430; return 10; }
    8
}
/// $090426: move.w #$158, $24(a6)
pub fn i_090426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9042C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x158;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09042C: clr.w $8e(a6)
pub fn i_09042c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90430;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $090430: tst.b $21(a6)
pub fn i_090430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90434;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $090434: beq.w $9044c
pub fn i_090434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90438;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9044C; return 10; }
    8
}
/// $090438: addq.w #$1, $86(a6)
pub fn i_090438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9043C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09043C: cmpi.w #$5, $86(a6)
pub fn i_09043c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90442;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $090442: blt.w $9044c
pub fn i_090442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90446;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x9044C; return 10; }
    8
}
/// $090446: lea.l $90452(pc), a1
pub fn i_090446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9044A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90452;
    16
}
/// $09044A: move.l a1, (a6)
pub fn i_09044a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9044C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09044C: subq.w #$1, $82(a6)
pub fn i_09044c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90450;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090450: rts
pub fn i_090450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90452;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Zoom_Path_090452  ($090452, 116 B, mission/gameover_continue_08f6d2.s)
/// $090452: move.w #$df, d1
pub fn i_090452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90456;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090456: jsr $236e.l
pub fn i_090456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9045C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9045C);
    cpu.pc = 0x236E;
    16
}
/// $09045C: move.w #$ec, $22(a6)
pub fn i_09045c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90462;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090462: move.w #$158, $24(a6)
pub fn i_090462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90468;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x158;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090468: move.w #$0, $86(a6)
pub fn i_090468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9046E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09046E: lea.l $90474(pc), a1
pub fn i_09046e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90472;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90474;
    16
}
/// $090472: move.l a1, (a6)
pub fn i_090472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90474;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090474: cmpi.w #$9, $86(a6)
pub fn i_090474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9047A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x9, Size::Word, false);
    16
}
/// $09047A: bge.w $9049c
pub fn i_09047a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9047E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x9049C; return 10; }
    8
}
/// $09047E: lea.l $2f4b24.l, a0
pub fn i_09047e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90484;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4B24;
    20
}
/// $090484: move.w $86(a6), d0
pub fn i_090484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90488;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090488: add.w d0, d0
pub fn i_090488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9048A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09048A: add.w d0, d0
pub fn i_09048a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9048C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09048C: move.w (a0, d0.w), d1
pub fn i_09048c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90490;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090490: add.w d1, $22(a6)
pub fn i_090490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90494;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090494: move.w $2(a0, d0.w), d1
pub fn i_090494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90498;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090498: add.w d1, $24(a6)
pub fn i_090498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9049C;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09049C: jsr $28d70.l
pub fn i_09049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x904A2);
    cpu.pc = 0x28D70;
    16
}
/// $0904A2: addq.w #$1, $86(a6)
pub fn i_0904a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904A6;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0904A6: cmpi.w #$d, $86(a6)
pub fn i_0904a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904AC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $0904AC: blt.w $904c0
pub fn i_0904ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904B0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x904C0; return 10; }
    8
}
/// $0904B0: movea.l $c(a6), a0
pub fn i_0904b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904B4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0904B4: move.b #$1, $20(a0)
pub fn i_0904b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904BA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0904BA: lea.l $904c6(pc), a1
pub fn i_0904ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x904C6;
    16
}
/// $0904BE: move.l a1, (a6)
pub fn i_0904be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0904C0: subq.w #$1, $82(a6)
pub fn i_0904c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904C4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0904C4: rts
pub fn i_0904c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Zoom_Wait_0904c6  ($0904C6, 8 B, mission/gameover_continue_08f6d2.s)
/// $0904C6: subq.w #$1, $82(a6)
pub fn i_0904c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904CA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0904CA: bne.w $904dc
pub fn i_0904ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x904DC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0904ce  ($0904CE, 14 B, kernel/jsr5b6_jmp518.c)
/// $0904CE: jsr $5b6.l
pub fn i_0904ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x904D4);
    cpu.pc = 0x5B6;
    16
}
/// $0904D4: jmp $518.l
pub fn i_0904d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_0904dc  ($0904DC, 8 B, thunks/jsr_abs_thunks.c)
/// $0904DC: jsr $28d70.l
pub fn i_0904dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x904E2);
    cpu.pc = 0x28D70;
    16
}
/// $0904E2: rts
pub fn i_0904e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Shake_0904e4  ($0904E4, 104 B, mission/gameover_continue_08f6d2.s)
/// $0904E4: move.w #$df, d1
pub fn i_0904e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904E8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0904E8: jsr $236e.l
pub fn i_0904e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x904EE);
    cpu.pc = 0x236E;
    16
}
/// $0904EE: move.w #$c000, d0
pub fn i_0904ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904F2;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0904F2: jsr $28134.l
pub fn i_0904f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x904F8);
    cpu.pc = 0x28134;
    16
}
/// $0904F8: andi.w #$ffe3, $38(a6)
pub fn i_0904f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x904FE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0904FE: ori.w #$1c, $38(a6)
pub fn i_0904fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90504;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090504: move.w #$c8, $22(a6)
pub fn i_090504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9050A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09050A: move.w #$140, $24(a6)
pub fn i_09050a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90510;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090510: move.w $22(a6), $88(a6)
pub fn i_090510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90516;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $090516: move.w $24(a6), $8a(a6)
pub fn i_090516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9051C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09051C: move.w #$0, $86(a6)
pub fn i_09051c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90522;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090522: lea.l $90592(pc), a1
pub fn i_090522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90526;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90592;
    16
}
/// $090526: jsr $4ae.l
pub fn i_090526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9052C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9052C);
    cpu.pc = 0x4AE;
    16
}
/// $09052C: jsr $5dd02.l
pub fn i_09052c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90532);
    cpu.pc = 0x5DD02;
    16
}
/// $090532: lea.l $2f50c4.l, a0
pub fn i_090532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90538;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F50C4;
    20
}
/// $090538: jsr $28cd4.l
pub fn i_090538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9053E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9053E);
    cpu.pc = 0x28CD4;
    16
}
/// $09053E: lea.l $90544(pc), a1
pub fn i_09053e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90542;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90544;
    16
}
/// $090542: move.l a1, (a6)
pub fn i_090542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90544;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090544: subq.w #$1, $82(a6)
pub fn i_090544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90548;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090548: bne.w $9055a
pub fn i_090548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9054C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9055A; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_09054c  ($09054C, 14 B, kernel/jsr5b6_jmp518.c)
/// $09054C: jsr $5b6.l
pub fn i_09054c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90552);
    cpu.pc = 0x5B6;
    16
}
/// $090552: jmp $518.l
pub fn i_090552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90558;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== GO_Shake_Step_09055a  ($09055A, 48 B, mission/gameover_continue_08f6d2.s)
/// $09055A: lea.l $2f4b48.l, a0
pub fn i_09055a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90560;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4B48;
    20
}
/// $090560: move.w $86(a6), d0
pub fn i_090560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90564;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090564: add.w d0, d0
pub fn i_090564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90566;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090566: add.w d0, d0
pub fn i_090566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90568;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090568: move.w (a0, d0.w), d1
pub fn i_090568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9056C;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09056C: add.w $88(a6), d1
pub fn i_09056c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90570;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090570: move.w d1, $22(a6)
pub fn i_090570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90574;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090574: move.w $2(a0, d0.w), d1
pub fn i_090574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90578;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090578: add.w $8a(a6), d1
pub fn i_090578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9057C;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09057C: move.w d1, $24(a6)
pub fn i_09057c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90580;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090580: addq.w #$2, $86(a6)
pub fn i_090580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90584;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090584: andi.w #$7, $86(a6)
pub fn i_090584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9058A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== JsrAbsThunk_09058a  ($09058A, 8 B, thunks/jsr_abs_thunks.c)
/// $09058A: jsr $28d70.l
pub fn i_09058a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90590);
    cpu.pc = 0x28D70;
    16
}
/// $090590: rts
pub fn i_090590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90592;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Flash_090592  ($090592, 40 B, mission/gameover_continue_08f6d2.s)
/// $090592: move.w #$e0, d1
pub fn i_090592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90596;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090596: jsr $236e.l
pub fn i_090596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9059C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9059C);
    cpu.pc = 0x236E;
    16
}
/// $09059C: move.w #$a8, $22(a6)
pub fn i_09059c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0905A2: move.w #$180, $24(a6)
pub fn i_0905a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0905A8: lea.l $2f50fc.l, a0
pub fn i_0905a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F50FC;
    20
}
/// $0905AE: jsr $28cd4.l
pub fn i_0905ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x905B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0905B4: lea.l $905ba(pc), a1
pub fn i_0905b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x905BA;
    16
}
/// $0905B8: move.l a1, (a6)
pub fn i_0905b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_0905ba  ($0905BA, 8 B, thunks/jsr_abs_thunks.c)
/// $0905BA: jsr $28d70.l
pub fn i_0905ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x905C0);
    cpu.pc = 0x28D70;
    16
}
/// $0905C0: rts
pub fn i_0905c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Banner_0905c2  ($0905C2, 88 B, mission/gameover_continue_08f6d2.s)
/// $0905C2: move.w #$e2, d1
pub fn i_0905c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0905C6: jsr $236e.l
pub fn i_0905c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x905CC);
    cpu.pc = 0x236E;
    16
}
/// $0905CC: move.w #$f000, d0
pub fn i_0905cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0905D0: jsr $28134.l
pub fn i_0905d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x905D6);
    cpu.pc = 0x28134;
    16
}
/// $0905D6: andi.w #$ffe3, $38(a6)
pub fn i_0905d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905DC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0905DC: ori.w #$1c, $38(a6)
pub fn i_0905dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905E2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0905E2: move.w #$a0, $22(a6)
pub fn i_0905e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0905E8: move.w #$180, $24(a6)
pub fn i_0905e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0905EE: lea.l $2f506e.l, a0
pub fn i_0905ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F506E;
    20
}
/// $0905F4: jsr $28cd4.l
pub fn i_0905f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x905FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0905FA: lea.l $90600(pc), a1
pub fn i_0905fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x905FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90600;
    16
}
/// $0905FE: move.l a1, (a6)
pub fn i_0905fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90600;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090600: subq.w #$1, $82(a6)
pub fn i_090600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90604;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090604: bne.w $90628
pub fn i_090604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90608;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90628; return 10; }
    8
}
/// $090608: move.w #$10df, d0
pub fn i_090608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9060C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09060C: jsr $2352.l
pub fn i_09060c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90612);
    cpu.pc = 0x2352;
    16
}
/// $090612: movea.l $c(a6), a0
pub fn i_090612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90616;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $090616: clr.b $21(a0)
pub fn i_090616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9061A;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_09061a  ($09061A, 14 B, kernel/jsr5b6_jmp518.c)
/// $09061A: jsr $5b6.l
pub fn i_09061a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90620);
    cpu.pc = 0x5B6;
    16
}
/// $090620: jmp $518.l
pub fn i_090620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90626;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_090628  ($090628, 8 B, thunks/jsr_abs_thunks.c)
/// $090628: jsr $28d70.l
pub fn i_090628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9062E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9062E);
    cpu.pc = 0x28D70;
    16
}
/// $09062E: rts
pub fn i_09062e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90630;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Banner_B_090630  ($090630, 70 B, mission/gameover_continue_08f6d2.s)
/// $090630: move.w #$e2, d1
pub fn i_090630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90634;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090634: jsr $236e.l
pub fn i_090634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9063A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9063A);
    cpu.pc = 0x236E;
    16
}
/// $09063A: move.w #$f000, d0
pub fn i_09063a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9063E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09063E: jsr $28134.l
pub fn i_09063e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90644);
    cpu.pc = 0x28134;
    16
}
/// $090644: andi.w #$ffe3, $38(a6)
pub fn i_090644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9064A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09064A: ori.w #$1c, $38(a6)
pub fn i_09064a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90650;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090650: move.w #$a0, $22(a6)
pub fn i_090650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90656;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090656: move.w #$180, $24(a6)
pub fn i_090656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9065C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09065C: lea.l $2f506e.l, a0
pub fn i_09065c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90662;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F506E;
    20
}
/// $090662: jsr $28cd4.l
pub fn i_090662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90668);
    cpu.pc = 0x28CD4;
    16
}
/// $090668: lea.l $9066e(pc), a1
pub fn i_090668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9066C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9066E;
    16
}
/// $09066C: move.l a1, (a6)
pub fn i_09066c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9066E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09066E: jsr $910d6(pc)
pub fn i_09066e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90672;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90672);
    cpu.pc = 0x910D6;
    16
}
/// $090672: bcc.w $90684
pub fn i_090672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90676;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x90684; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_090676  ($090676, 14 B, kernel/jsr5b6_jmp518.c)
/// $090676: jsr $5b6.l
pub fn i_090676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9067C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9067C);
    cpu.pc = 0x5B6;
    16
}
/// $09067C: jmp $518.l
pub fn i_09067c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90682;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_090684  ($090684, 8 B, thunks/jsr_abs_thunks.c)
/// $090684: jsr $28d70.l
pub fn i_090684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9068A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9068A);
    cpu.pc = 0x28D70;
    16
}
/// $09068A: rts
pub fn i_09068a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9068C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Banner_C_09068c  ($09068C, 62 B, mission/gameover_continue_08f6d2.s)
/// $09068C: move.w #$e1, d1
pub fn i_09068c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90690;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090690: jsr $236e.l
pub fn i_090690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90696);
    cpu.pc = 0x236E;
    16
}
/// $090696: move.w #$0, d0
pub fn i_090696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9069A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09069A: jsr $28134.l
pub fn i_09069a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x906A0);
    cpu.pc = 0x28134;
    16
}
/// $0906A0: andi.w #$ffe3, $38(a6)
pub fn i_0906a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906A6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0906A6: ori.w #$1c, $38(a6)
pub fn i_0906a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906AC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0906AC: move.w #$a0, $22(a6)
pub fn i_0906ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0906B2: move.w #$180, $24(a6)
pub fn i_0906b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0906B8: lea.l $2f507a.l, a0
pub fn i_0906b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F507A;
    20
}
/// $0906BE: jsr $28cd4.l
pub fn i_0906be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x906C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0906C4: lea.l $906ca(pc), a1
pub fn i_0906c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x906CA;
    16
}
/// $0906C8: move.l a1, (a6)
pub fn i_0906c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== GO_Banner_Tick_0906ca  ($0906CA, 8 B, mission/gameover_continue_08f6d2.s)
/// $0906CA: subq.w #$1, $82(a6)
pub fn i_0906ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906CE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0906CE: bne.w $906e0
pub fn i_0906ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x906E0; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0906d2  ($0906D2, 14 B, kernel/jsr5b6_jmp518.c)
/// $0906D2: jsr $5b6.l
pub fn i_0906d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x906D8);
    cpu.pc = 0x5B6;
    16
}
/// $0906D8: jmp $518.l
pub fn i_0906d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906DE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_0906e0  ($0906E0, 8 B, thunks/jsr_abs_thunks.c)
/// $0906E0: jsr $28d70.l
pub fn i_0906e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x906E6);
    cpu.pc = 0x28D70;
    16
}
/// $0906E6: rts
pub fn i_0906e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Banner_D_0906e8  ($0906E8, 64 B, mission/gameover_continue_08f6d2.s)
/// $0906E8: move.w #$e2, d1
pub fn i_0906e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906EC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0906EC: jsr $236e.l
pub fn i_0906ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x906F2);
    cpu.pc = 0x236E;
    16
}
/// $0906F2: move.w #$0, d0
pub fn i_0906f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0906F6: jsr $28134.l
pub fn i_0906f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x906FC);
    cpu.pc = 0x28134;
    16
}
/// $0906FC: andi.w #$ffe3, $38(a6)
pub fn i_0906fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90702;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090702: ori.w #$1c, $38(a6)
pub fn i_090702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90708;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090708: move.w #$a0, $22(a6)
pub fn i_090708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9070E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09070E: move.w #$180, $24(a6)
pub fn i_09070e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90714;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090714: lea.l $2f5086.l, a0
pub fn i_090714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9071A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5086;
    20
}
/// $09071A: jsr $28cd4.l
pub fn i_09071a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90720);
    cpu.pc = 0x28CD4;
    16
}
/// $090720: lea.l $906ca(pc), a1
pub fn i_090720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90724;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x906CA;
    16
}
/// $090724: move.l a1, (a6)
pub fn i_090724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90726;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090726: bra.b $906ca
pub fn i_090726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90728;
    cpu.ir = 0x60A2;
    cpu.pc = 0x906CA;
    10
}
// ===== GO_Letter_V0_090728  ($090728, 70 B, mission/gameover_continue_08f6d2.s)
/// $090728: move.w #$c5, d1
pub fn i_090728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9072C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09072C: jsr $236e.l
pub fn i_09072c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90732;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90732);
    cpu.pc = 0x236E;
    16
}
/// $090732: move.w #$4000, d0
pub fn i_090732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90736;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090736: jsr $28134.l
pub fn i_090736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9073C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9073C);
    cpu.pc = 0x28134;
    16
}
/// $09073C: andi.w #$ffe3, $38(a6)
pub fn i_09073c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90742;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090742: ori.w #$1c, $38(a6)
pub fn i_090742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90748;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090748: clr.b $21(a6)
pub fn i_090748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9074C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09074C: move.w #$0, $80(a6)
pub fn i_09074c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90752;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090752: move.w #$b7, $7c(a6)
pub fn i_090752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90758;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090758: move.w #$bf, $7e(a6)
pub fn i_090758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9075E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09075E: lea.l $2f4d9e.l, a0
pub fn i_09075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90764;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4D9E;
    20
}
/// $090764: jsr $28cd4.l
pub fn i_090764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9076A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9076A);
    cpu.pc = 0x28CD4;
    16
}
/// $09076A: bra.w $90b08
pub fn i_09076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9076E;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V1_09076e  ($09076E, 70 B, mission/gameover_continue_08f6d2.s)
/// $09076E: move.w #$c7, d1
pub fn i_09076e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90772;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090772: jsr $236e.l
pub fn i_090772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90778);
    cpu.pc = 0x236E;
    16
}
/// $090778: move.w #$4000, d0
pub fn i_090778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9077C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09077C: jsr $28134.l
pub fn i_09077c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90782);
    cpu.pc = 0x28134;
    16
}
/// $090782: andi.w #$ffe3, $38(a6)
pub fn i_090782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90788;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090788: ori.w #$18, $38(a6)
pub fn i_090788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9078E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09078E: clr.b $21(a6)
pub fn i_09078e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90792;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090792: move.w #$1, $80(a6)
pub fn i_090792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90798;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090798: move.w #$b9, $7c(a6)
pub fn i_090798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9079E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09079E: move.w #$c1, $7e(a6)
pub fn i_09079e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0907A4: lea.l $2f4daa.l, a0
pub fn i_0907a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4DAA;
    20
}
/// $0907AA: jsr $28cd4.l
pub fn i_0907aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x907B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0907B0: bra.w $90b08
pub fn i_0907b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V2_0907b4  ($0907B4, 70 B, mission/gameover_continue_08f6d2.s)
/// $0907B4: move.w #$c5, d1
pub fn i_0907b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0907B8: jsr $236e.l
pub fn i_0907b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x907BE);
    cpu.pc = 0x236E;
    16
}
/// $0907BE: move.w #$4000, d0
pub fn i_0907be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0907C2: jsr $28134.l
pub fn i_0907c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x907C8);
    cpu.pc = 0x28134;
    16
}
/// $0907C8: andi.w #$ffe3, $38(a6)
pub fn i_0907c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907CE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0907CE: ori.w #$14, $38(a6)
pub fn i_0907ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907D4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0907D4: clr.b $21(a6)
pub fn i_0907d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907D8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0907D8: move.w #$2, $80(a6)
pub fn i_0907d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0907DE: move.w #$b7, $7c(a6)
pub fn i_0907de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0907E4: move.w #$bf, $7e(a6)
pub fn i_0907e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0907EA: lea.l $2f4e08.l, a0
pub fn i_0907ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E08;
    20
}
/// $0907F0: jsr $28cd4.l
pub fn i_0907f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x907F6);
    cpu.pc = 0x28CD4;
    16
}
/// $0907F6: bra.w $90b08
pub fn i_0907f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V3_0907fa  ($0907FA, 70 B, mission/gameover_continue_08f6d2.s)
/// $0907FA: move.w #$c7, d1
pub fn i_0907fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x907FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0907FE: jsr $236e.l
pub fn i_0907fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90804);
    cpu.pc = 0x236E;
    16
}
/// $090804: move.w #$4000, d0
pub fn i_090804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90808;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090808: jsr $28134.l
pub fn i_090808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9080E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9080E);
    cpu.pc = 0x28134;
    16
}
/// $09080E: andi.w #$ffe3, $38(a6)
pub fn i_09080e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90814;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090814: ori.w #$10, $38(a6)
pub fn i_090814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9081A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09081A: clr.b $21(a6)
pub fn i_09081a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9081E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09081E: move.w #$3, $80(a6)
pub fn i_09081e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90824;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090824: move.w #$b9, $7c(a6)
pub fn i_090824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9082A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09082A: move.w #$c1, $7e(a6)
pub fn i_09082a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90830;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090830: lea.l $2f4e14.l, a0
pub fn i_090830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90836;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E14;
    20
}
/// $090836: jsr $28cd4.l
pub fn i_090836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9083C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9083C);
    cpu.pc = 0x28CD4;
    16
}
/// $09083C: bra.w $90b08
pub fn i_09083c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90840;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V4_090840  ($090840, 78 B, mission/gameover_continue_08f6d2.s)
/// $090840: move.w #$c5, d1
pub fn i_090840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90844;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090844: jsr $236e.l
pub fn i_090844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9084A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9084A);
    cpu.pc = 0x236E;
    16
}
/// $09084A: move.w #$4000, d0
pub fn i_09084a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9084E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09084E: jsr $28134.l
pub fn i_09084e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90854);
    cpu.pc = 0x28134;
    16
}
/// $090854: andi.w #$ffe3, $38(a6)
pub fn i_090854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9085A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09085A: ori.w #$1c, $38(a6)
pub fn i_09085a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90860;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090860: move.b #$ff, $21(a6)
pub fn i_090860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90866;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090866: move.w #$4, $80(a6)
pub fn i_090866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9086C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09086C: move.w #$b7, $7c(a6)
pub fn i_09086c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90872;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090872: move.w #$bf, $7e(a6)
pub fn i_090872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90878;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090878: move.w #$20, $86(a6)
pub fn i_090878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9087E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09087E: lea.l $2f4e42.l, a0
pub fn i_09087e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90884;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E42;
    20
}
/// $090884: jsr $28cd4.l
pub fn i_090884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9088A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9088A);
    cpu.pc = 0x28CD4;
    16
}
/// $09088A: bra.w $90b08
pub fn i_09088a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9088E;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V5_09088e  ($09088E, 70 B, mission/gameover_continue_08f6d2.s)
/// $09088E: move.w #$c5, d1
pub fn i_09088e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90892;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090892: jsr $236e.l
pub fn i_090892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90898;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90898);
    cpu.pc = 0x236E;
    16
}
/// $090898: move.w #$4000, d0
pub fn i_090898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9089C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09089C: jsr $28134.l
pub fn i_09089c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x908A2);
    cpu.pc = 0x28134;
    16
}
/// $0908A2: andi.w #$ffe3, $38(a6)
pub fn i_0908a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908A8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0908A8: ori.w #$18, $38(a6)
pub fn i_0908a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908AE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0908AE: clr.b $21(a6)
pub fn i_0908ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908B2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0908B2: move.w #$5, $80(a6)
pub fn i_0908b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0908B8: move.w #$b7, $7c(a6)
pub fn i_0908b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0908BE: move.w #$bf, $7e(a6)
pub fn i_0908be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0908C4: lea.l $2f4e4e.l, a0
pub fn i_0908c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E4E;
    20
}
/// $0908CA: jsr $28cd4.l
pub fn i_0908ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x908D0);
    cpu.pc = 0x28CD4;
    16
}
/// $0908D0: bra.w $90b08
pub fn i_0908d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V6_0908d4  ($0908D4, 70 B, mission/gameover_continue_08f6d2.s)
/// $0908D4: move.w #$c5, d1
pub fn i_0908d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0908D8: jsr $236e.l
pub fn i_0908d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x908DE);
    cpu.pc = 0x236E;
    16
}
/// $0908DE: move.w #$4000, d0
pub fn i_0908de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0908E2: jsr $28134.l
pub fn i_0908e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x908E8);
    cpu.pc = 0x28134;
    16
}
/// $0908E8: andi.w #$ffe3, $38(a6)
pub fn i_0908e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908EE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0908EE: ori.w #$14, $38(a6)
pub fn i_0908ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908F4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0908F4: clr.b $21(a6)
pub fn i_0908f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908F8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0908F8: move.w #$6, $80(a6)
pub fn i_0908f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x908FE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0908FE: move.w #$b7, $7c(a6)
pub fn i_0908fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90904;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090904: move.w #$bf, $7e(a6)
pub fn i_090904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9090A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09090A: lea.l $2f4e5a.l, a0
pub fn i_09090a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90910;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E5A;
    20
}
/// $090910: jsr $28cd4.l
pub fn i_090910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90916;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90916);
    cpu.pc = 0x28CD4;
    16
}
/// $090916: bra.w $90b08
pub fn i_090916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9091A;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V7_09091a  ($09091A, 78 B, mission/gameover_continue_08f6d2.s)
/// $09091A: move.w #$c6, d1
pub fn i_09091a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9091E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09091E: jsr $236e.l
pub fn i_09091e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90924);
    cpu.pc = 0x236E;
    16
}
/// $090924: move.w #$4000, d0
pub fn i_090924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90928;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090928: jsr $28134.l
pub fn i_090928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9092E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9092E);
    cpu.pc = 0x28134;
    16
}
/// $09092E: andi.w #$ffe3, $38(a6)
pub fn i_09092e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90934;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090934: ori.w #$10, $38(a6)
pub fn i_090934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9093A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09093A: move.b #$ff, $21(a6)
pub fn i_09093a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90940;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090940: move.w #$7, $80(a6)
pub fn i_090940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90946;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090946: move.w #$b8, $7c(a6)
pub fn i_090946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9094C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09094C: move.w #$c0, $7e(a6)
pub fn i_09094c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90952;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090952: move.w #$20, $86(a6)
pub fn i_090952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90958;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090958: lea.l $2f4e66.l, a0
pub fn i_090958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9095E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E66;
    20
}
/// $09095E: jsr $28cd4.l
pub fn i_09095e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90964);
    cpu.pc = 0x28CD4;
    16
}
/// $090964: bra.w $90b08
pub fn i_090964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90968;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V8_090968  ($090968, 70 B, mission/gameover_continue_08f6d2.s)
/// $090968: move.w #$c5, d1
pub fn i_090968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9096C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09096C: jsr $236e.l
pub fn i_09096c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90972);
    cpu.pc = 0x236E;
    16
}
/// $090972: move.w #$4000, d0
pub fn i_090972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90976;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090976: jsr $28134.l
pub fn i_090976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9097C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9097C);
    cpu.pc = 0x28134;
    16
}
/// $09097C: andi.w #$ffe3, $38(a6)
pub fn i_09097c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90982;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090982: ori.w #$10, $38(a6)
pub fn i_090982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90988;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090988: clr.b $21(a6)
pub fn i_090988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9098C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09098C: move.w #$8, $80(a6)
pub fn i_09098c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90992;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090992: move.w #$b7, $7c(a6)
pub fn i_090992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90998;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090998: move.w #$bf, $7e(a6)
pub fn i_090998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9099E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09099E: lea.l $2f4e72.l, a0
pub fn i_09099e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E72;
    20
}
/// $0909A4: jsr $28cd4.l
pub fn i_0909a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x909AA);
    cpu.pc = 0x28CD4;
    16
}
/// $0909AA: bra.w $90b08
pub fn i_0909aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V9_0909ae  ($0909AE, 70 B, mission/gameover_continue_08f6d2.s)
/// $0909AE: move.w #$c9, d1
pub fn i_0909ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909B2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0909B2: jsr $236e.l
pub fn i_0909b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x909B8);
    cpu.pc = 0x236E;
    16
}
/// $0909B8: move.w #$4000, d0
pub fn i_0909b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0909BC: jsr $28134.l
pub fn i_0909bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x909C2);
    cpu.pc = 0x28134;
    16
}
/// $0909C2: andi.w #$ffe3, $38(a6)
pub fn i_0909c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909C8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0909C8: ori.w #$c, $38(a6)
pub fn i_0909c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909CE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0909CE: clr.b $21(a6)
pub fn i_0909ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909D2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0909D2: move.w #$9, $80(a6)
pub fn i_0909d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0909D8: move.w #$bb, $7c(a6)
pub fn i_0909d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0909DE: move.w #$c3, $7e(a6)
pub fn i_0909de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0909E4: lea.l $2f4e7e.l, a0
pub fn i_0909e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E7E;
    20
}
/// $0909EA: jsr $28cd4.l
pub fn i_0909ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x909F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0909F0: bra.w $90b08
pub fn i_0909f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V10_0909f4  ($0909F4, 70 B, mission/gameover_continue_08f6d2.s)
/// $0909F4: move.w #$c8, d1
pub fn i_0909f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909F8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0909F8: jsr $236e.l
pub fn i_0909f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x909FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x909FE);
    cpu.pc = 0x236E;
    16
}
/// $0909FE: move.w #$4000, d0
pub fn i_0909fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A02;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090A02: jsr $28134.l
pub fn i_090a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90A08);
    cpu.pc = 0x28134;
    16
}
/// $090A08: andi.w #$ffe3, $38(a6)
pub fn i_090a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A0E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090A0E: ori.w #$10, $38(a6)
pub fn i_090a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A14;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090A14: clr.b $21(a6)
pub fn i_090a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A18;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090A18: move.w #$a, $80(a6)
pub fn i_090a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090A1E: move.w #$ba, $7c(a6)
pub fn i_090a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090A24: move.w #$c2, $7e(a6)
pub fn i_090a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090A2A: lea.l $2f4e8a.l, a0
pub fn i_090a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E8A;
    20
}
/// $090A30: jsr $28cd4.l
pub fn i_090a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90A36);
    cpu.pc = 0x28CD4;
    16
}
/// $090A36: bra.w $90b08
pub fn i_090a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V11_090a3a  ($090A3A, 70 B, mission/gameover_continue_08f6d2.s)
/// $090A3A: move.w #$c6, d1
pub fn i_090a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A3E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090A3E: jsr $236e.l
pub fn i_090a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90A44);
    cpu.pc = 0x236E;
    16
}
/// $090A44: move.w #$4000, d0
pub fn i_090a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090A48: jsr $28134.l
pub fn i_090a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90A4E);
    cpu.pc = 0x28134;
    16
}
/// $090A4E: andi.w #$ffe3, $38(a6)
pub fn i_090a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A54;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090A54: ori.w #$10, $38(a6)
pub fn i_090a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A5A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090A5A: clr.b $21(a6)
pub fn i_090a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A5E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090A5E: move.w #$b, $80(a6)
pub fn i_090a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A64;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090A64: move.w #$b8, $7c(a6)
pub fn i_090a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090A6A: move.w #$c0, $7e(a6)
pub fn i_090a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090A70: lea.l $2f4e96.l, a0
pub fn i_090a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4E96;
    20
}
/// $090A76: jsr $28cd4.l
pub fn i_090a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90A7C);
    cpu.pc = 0x28CD4;
    16
}
/// $090A7C: bra.w $90b08
pub fn i_090a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A80;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V12_090a80  ($090A80, 70 B, mission/gameover_continue_08f6d2.s)
/// $090A80: move.w #$c5, d1
pub fn i_090a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A84;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090A84: jsr $236e.l
pub fn i_090a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90A8A);
    cpu.pc = 0x236E;
    16
}
/// $090A8A: move.w #$4000, d0
pub fn i_090a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090A8E: jsr $28134.l
pub fn i_090a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90A94);
    cpu.pc = 0x28134;
    16
}
/// $090A94: andi.w #$ffe3, $38(a6)
pub fn i_090a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A9A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090A9A: ori.w #$8, $38(a6)
pub fn i_090a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AA0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090AA0: clr.b $21(a6)
pub fn i_090aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AA4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090AA4: move.w #$c, $80(a6)
pub fn i_090aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090AAA: move.w #$b7, $7c(a6)
pub fn i_090aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090AB0: move.w #$bf, $7e(a6)
pub fn i_090ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AB6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090AB6: lea.l $2f4ea2.l, a0
pub fn i_090ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90ABC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4EA2;
    20
}
/// $090ABC: jsr $28cd4.l
pub fn i_090abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90AC2);
    cpu.pc = 0x28CD4;
    16
}
/// $090AC2: bra.w $90b08
pub fn i_090ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B08;
    10
}
// ===== GO_Letter_V13_090ac6  ($090AC6, 324 B, mission/gameover_continue_08f6d2.s)
/// $090AC6: move.w #$c5, d1
pub fn i_090ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90ACA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090ACA: jsr $236e.l
pub fn i_090aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90AD0);
    cpu.pc = 0x236E;
    16
}
/// $090AD0: move.w #$4000, d0
pub fn i_090ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AD4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090AD4: jsr $28134.l
pub fn i_090ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90ADA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90ADA);
    cpu.pc = 0x28134;
    16
}
/// $090ADA: andi.w #$ffe3, $38(a6)
pub fn i_090ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AE0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090AE0: ori.w #$4, $38(a6)
pub fn i_090ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AE6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090AE6: clr.b $21(a6)
pub fn i_090ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AEA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090AEA: move.w #$d, $80(a6)
pub fn i_090aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AF0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090AF0: move.w #$b7, $7c(a6)
pub fn i_090af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AF6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090AF6: move.w #$bf, $7e(a6)
pub fn i_090af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90AFC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090AFC: lea.l $2f4eae.l, a0
pub fn i_090afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4EAE;
    20
}
/// $090B02: jsr $28cd4.l
pub fn i_090b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90B08);
    cpu.pc = 0x28CD4;
    16
}
/// $090B08: clr.b $20(a6)
pub fn i_090b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B0C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090B0C: move.w #$1, $82(a6)
pub fn i_090b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B12;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090B12: lea.l $90b18(pc), a1
pub fn i_090b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90B18;
    16
}
/// $090B16: move.l a1, (a6)
pub fn i_090b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090B18: jsr $910a2(pc)
pub fn i_090b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90B1C);
    cpu.pc = 0x910A2;
    16
}
/// $090B1C: bcs.w $90bb0
pub fn i_090b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B20;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x90BB0; return 10; }
    8
}
/// $090B20: movea.l $c(a6), a0
pub fn i_090b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B24;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $090B24: lea.l $2f4a46.l, a1
pub fn i_090b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B2A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F4A46;
    20
}
/// $090B2A: lea.l $2f4a7c.l, a2
pub fn i_090b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B30;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F4A7C;
    20
}
/// $090B30: move.w $80(a6), d2
pub fn i_090b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B34;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090B34: add.w d2, d2
pub fn i_090b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B36;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090B36: add.w d2, d2
pub fn i_090b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B38;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090B38: movea.l (a2, d2.w), a2
pub fn i_090b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B3C;
    cpu.ir = 0x2472;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $090B3C: moveq #$0, d2
pub fn i_090b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B3E;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090B3E: move.w $84(a0), d2
pub fn i_090b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B42;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090B42: add.w d2, d2
pub fn i_090b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B44;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090B44: add.w d2, d2
pub fn i_090b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B46;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090B46: adda.l d2, a2
pub fn i_090b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B48;
    cpu.ir = 0xD5C2;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $090B48: move.w (a1), d1
pub fn i_090b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B4A;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $090B4A: add.w (a2), d1
pub fn i_090b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B4C;
    cpu.ir = 0xD252;
    let v2 = bus.read16(cpu.a[2]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $090B4C: add.w $22(a0), d1
pub fn i_090b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B50;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090B50: move.w d1, $22(a6)
pub fn i_090b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B54;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090B54: move.w $2(a1), d1
pub fn i_090b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B58;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090B58: add.w $2(a2), d1
pub fn i_090b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B5C;
    cpu.ir = 0xD26A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090B5C: add.w $24(a0), d1
pub fn i_090b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B60;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090B60: move.w d1, $24(a6)
pub fn i_090b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B64;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090B64: tst.b $21(a6)
pub fn i_090b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B68;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $090B68: beq.w $90b7c
pub fn i_090b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x90B7C; return 10; }
    8
}
/// $090B6C: tst.w $86(a6)
pub fn i_090b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B70;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $090B70: beq.w $90b7c
pub fn i_090b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x90B7C; return 10; }
    8
}
/// $090B74: subq.w #$1, $86(a6)
pub fn i_090b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B78;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090B78: bra.w $90b82
pub fn i_090b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x90B82;
    10
}
/// $090B7C: jsr $28d70.l
pub fn i_090b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90B82);
    cpu.pc = 0x28D70;
    16
}
/// $090B82: movea.l $c(a6), a0
pub fn i_090b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B86;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $090B86: cmpi.b #$1, $21(a0)
pub fn i_090b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B8C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $090B8C: bne.w $90bb0
pub fn i_090b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B90;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90BB0; return 10; }
    8
}
/// $090B90: tst.b $20(a6)
pub fn i_090b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B94;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $090B94: bne.w $90bb0
pub fn i_090b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90BB0; return 10; }
    8
}
/// $090B98: move.b #$80, $20(a6)
pub fn i_090b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90B9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090B9E: move.w $14(a6), d1
pub fn i_090b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BA2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090BA2: move.w $7c(a6), d2
pub fn i_090ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BA6;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090BA6: moveq #$1, d3
pub fn i_090ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BA8;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090BA8: moveq #$2, d4
pub fn i_090ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BAA;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090BAA: jsr $2c30.l
pub fn i_090baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90BB0);
    cpu.pc = 0x2C30;
    16
}
/// $090BB0: jsr $910f0(pc)
pub fn i_090bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90BB4);
    cpu.pc = 0x910F0;
    16
}
/// $090BB4: bcc.w $90bd2
pub fn i_090bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x90BD2; return 10; }
    8
}
/// $090BB8: cmpi.b #$ff, $20(a6)
pub fn i_090bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BBE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $090BBE: beq.w $90bd2
pub fn i_090bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x90BD2; return 10; }
    8
}
/// $090BC2: move.b #$ff, $20(a6)
pub fn i_090bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BC8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090BC8: move.w $7e(a6), d1
pub fn i_090bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BCC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090BCC: jsr $236e.l
pub fn i_090bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90BD2);
    cpu.pc = 0x236E;
    16
}
/// $090BD2: jsr $910f0(pc)
pub fn i_090bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90BD6);
    cpu.pc = 0x910F0;
    16
}
/// $090BD6: bcc.w $90c10
pub fn i_090bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x90C10; return 10; }
    8
}
/// $090BDA: tst.w $82(a6)
pub fn i_090bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BDE;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $090BDE: beq.w $90c12
pub fn i_090bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x90C12; return 10; }
    8
}
/// $090BE2: subq.w #$1, $82(a6)
pub fn i_090be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BE6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090BE6: bne.w $90c10
pub fn i_090be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90C10; return 10; }
    8
}
/// $090BEA: move.b #$fc, $32(a6)
pub fn i_090bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BF0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090BF0: move.b #$fc, $33(a6)
pub fn i_090bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090BF6: cmpi.w #$5, $80(a6)
pub fn i_090bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90BFC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $090BFC: bne.w $90c10
pub fn i_090bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90C10; return 10; }
    8
}
/// $090C00: lea.l $90c30(pc), a1
pub fn i_090c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90C30;
    16
}
/// $090C04: jsr $4ae.l
pub fn i_090c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90C0A);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_090c0a  ($090C0A, 8 B, thunks/jsr_abs_thunks.c)
/// $090C0A: jsr $5dd02.l
pub fn i_090c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90C10);
    cpu.pc = 0x5DD02;
    16
}
/// $090C10: rts
pub fn i_090c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Letter_FadeOut_090c12  ($090C12, 30 B, mission/gameover_continue_08f6d2.s)
/// $090C12: subi.b #$12, $32(a6)
pub fn i_090c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C18;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x12, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $090C18: subi.b #$12, $33(a6)
pub fn i_090c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C1E;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x12, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $090C1E: cmpi.b #$20, $32(a6)
pub fn i_090c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C24;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Byte, false);
    16
}
/// $090C24: bhi.w $90c2e
pub fn i_090c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C28;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x90C2E; return 10; }
    8
}
/// $090C28: jmp $518.l
pub fn i_090c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C2E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $090C2E: rts
pub fn i_090c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Glow_090c30  ($090C30, 138 B, mission/gameover_continue_08f6d2.s)
/// $090C30: move.w #$b6, d1
pub fn i_090c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C34;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090C34: jsr $236e.l
pub fn i_090c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90C3A);
    cpu.pc = 0x236E;
    16
}
/// $090C3A: subi.w #$6a, $22(a6)
pub fn i_090c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C40;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x6A, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $090C40: addi.w #$30, $24(a6)
pub fn i_090c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C46;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $090C46: move.b #$15, $32(a6)
pub fn i_090c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090C4C: move.b #$15, $33(a6)
pub fn i_090c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090C52: move.w #$1, $82(a6)
pub fn i_090c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090C58: move.w #$1, $86(a6)
pub fn i_090c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C5E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090C5E: moveq #$2, d0
pub fn i_090c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C60;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090C60: jsr $523c6.l
pub fn i_090c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90C66);
    cpu.pc = 0x523C6;
    16
}
/// $090C66: move.w #$e000, d0
pub fn i_090c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090C6A: jsr $28134.l
pub fn i_090c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90C70);
    cpu.pc = 0x28134;
    16
}
/// $090C70: andi.w #$ffe3, $38(a6)
pub fn i_090c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C76;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090C76: ori.w #$0, $38(a6)
pub fn i_090c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C7C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090C7C: lea.l $2f5338.l, a0
pub fn i_090c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5338;
    20
}
/// $090C82: jsr $28cd4.l
pub fn i_090c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90C88);
    cpu.pc = 0x28CD4;
    16
}
/// $090C88: lea.l $90c8e(pc), a1
pub fn i_090c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90C8E;
    16
}
/// $090C8C: move.l a1, (a6)
pub fn i_090c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090C8E: jsr $28d70.l
pub fn i_090c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90C94);
    cpu.pc = 0x28D70;
    16
}
/// $090C94: subq.w #$1, $86(a6)
pub fn i_090c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C98;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090C98: bne.w $90cb8
pub fn i_090c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90C9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90CB8; return 10; }
    8
}
/// $090C9C: move.w $82(a6), $86(a6)
pub fn i_090c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CA2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $090CA2: cmpi.b #$ff, $32(a6)
pub fn i_090ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CA8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $090CA8: beq.w $90cb8
pub fn i_090ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x90CB8; return 10; }
    8
}
/// $090CAC: addi.b #$12, $32(a6)
pub fn i_090cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CB2;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x12, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $090CB2: addi.b #$12, $33(a6)
pub fn i_090cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CB8;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x12, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $090CB8: rts
pub fn i_090cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Glow_Drift_090cba  ($090CBA, 246 B, mission/gameover_continue_08f6d2.s)
/// $090CBA: move.w #$b6, d1
pub fn i_090cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CBE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090CBE: jsr $236e.l
pub fn i_090cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90CC4);
    cpu.pc = 0x236E;
    16
}
/// $090CC4: move.w $22(a6), d0
pub fn i_090cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CC8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090CC8: addi.w #$10, d0
pub fn i_090cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CCC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $090CCC: move.w d0, $8c(a6)
pub fn i_090ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CD0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090CD0: move.w $24(a6), d0
pub fn i_090cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CD4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090CD4: addi.w #$40, d0
pub fn i_090cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CD8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $090CD8: move.w d0, $8e(a6)
pub fn i_090cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CDC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090CDC: subq.w #$8, $22(a6)
pub fn i_090cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CE0;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090CE0: addi.w #$58, $24(a6)
pub fn i_090ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CE6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x58, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $090CE6: move.b #$ff, $32(a6)
pub fn i_090ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090CEC: move.b #$ff, $33(a6)
pub fn i_090cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CF2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090CF2: move.w #$1, $82(a6)
pub fn i_090cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090CF8: move.w #$1, $86(a6)
pub fn i_090cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90CFE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090CFE: moveq #$1, d0
pub fn i_090cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D00;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090D00: jsr $523da.l
pub fn i_090d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90D06);
    cpu.pc = 0x523DA;
    16
}
/// $090D06: move.w #$e000, d0
pub fn i_090d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D0A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090D0A: jsr $28134.l
pub fn i_090d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90D10);
    cpu.pc = 0x28134;
    16
}
/// $090D10: andi.w #$ffe3, $38(a6)
pub fn i_090d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D16;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090D16: ori.w #$0, $38(a6)
pub fn i_090d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D1C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090D1C: lea.l $2f5338.l, a0
pub fn i_090d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5338;
    20
}
/// $090D22: jsr $28cd4.l
pub fn i_090d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90D28);
    cpu.pc = 0x28CD4;
    16
}
/// $090D28: lea.l $90d2e(pc), a1
pub fn i_090d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90D2E;
    16
}
/// $090D2C: move.l a1, (a6)
pub fn i_090d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090D2E: jsr $28d70.l
pub fn i_090d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90D34);
    cpu.pc = 0x28D70;
    16
}
/// $090D34: move.w $8c(a6), d0
pub fn i_090d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D38;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090D38: cmp.w $22(a6), d0
pub fn i_090d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D3C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $090D3C: beq.w $90d56
pub fn i_090d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x90D56; return 10; }
    8
}
/// $090D40: move.w $8c(a6), d0
pub fn i_090d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D44;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090D44: sub.w $22(a6), d0
pub fn i_090d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D48;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090D48: asr.w #$1, d0
pub fn i_090d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D4A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $090D4A: tst.w d0
pub fn i_090d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D4C;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $090D4C: bne.w $90d52
pub fn i_090d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90D52; return 10; }
    8
}
/// $090D50: moveq #$1, d0
pub fn i_090d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D52;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090D52: add.w d0, $22(a6)
pub fn i_090d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D56;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090D56: move.w $8e(a6), d0
pub fn i_090d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D5A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090D5A: cmp.w $24(a6), d0
pub fn i_090d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D5E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $090D5E: beq.w $90d7a
pub fn i_090d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x90D7A; return 10; }
    8
}
/// $090D62: move.w $8e(a6), d0
pub fn i_090d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D66;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090D66: sub.w $24(a6), d0
pub fn i_090d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D6A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090D6A: asr.w #$1, d0
pub fn i_090d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D6C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $090D6C: tst.w d0
pub fn i_090d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D6E;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $090D6E: bne.w $90d76
pub fn i_090d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90D76; return 10; }
    8
}
/// $090D72: move.w #$ffff, d0
pub fn i_090d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D76;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090D76: add.w d0, $24(a6)
pub fn i_090d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D7A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090D7A: subq.w #$1, $86(a6)
pub fn i_090d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D7E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $090D7E: bne.w $90cb8
pub fn i_090d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D82;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90CB8; return 10; }
    8
}
/// $090D82: move.w $82(a6), $86(a6)
pub fn i_090d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D88;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $090D88: subi.b #$10, $32(a6)
pub fn i_090d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D8E;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $090D8E: subi.b #$10, $33(a6)
pub fn i_090d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D94;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $090D94: cmpi.b #$1f, $32(a6)
pub fn i_090d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D9A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Byte, false);
    16
}
/// $090D9A: bhi.w $90dae
pub fn i_090d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90D9E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x90DAE; return 10; }
    8
}
/// $090D9E: movea.l $c(a6), a0
pub fn i_090d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DA2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $090DA2: move.b #$1, $20(a0)
pub fn i_090da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DA8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090DA8: jmp $518.l
pub fn i_090da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $090DAE: rts
pub fn i_090dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Figure_A_090db0  ($090DB0, 50 B, mission/gameover_continue_08f6d2.s)
/// $090DB0: move.w #$c4, d1
pub fn i_090db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090DB4: jsr $236e.l
pub fn i_090db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90DBA);
    cpu.pc = 0x236E;
    16
}
/// $090DBA: clr.b $21(a6)
pub fn i_090dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DBE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090DBE: clr.b $20(a6)
pub fn i_090dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DC2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090DC2: move.w #$2000, d0
pub fn i_090dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090DC6: jsr $28134.l
pub fn i_090dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90DCC);
    cpu.pc = 0x28134;
    16
}
/// $090DCC: move.w #$b5, $7c(a6)
pub fn i_090dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090DD2: lea.l $2f4eba.l, a0
pub fn i_090dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4EBA;
    20
}
/// $090DD8: jsr $28cd4.l
pub fn i_090dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90DDE);
    cpu.pc = 0x28CD4;
    16
}
/// $090DDE: bra.w $90e1c
pub fn i_090dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x90E1C;
    10
}
// ===== GO_Figure_B_090de2  ($090DE2, 148 B, mission/gameover_continue_08f6d2.s)
/// $090DE2: move.w #$c4, d1
pub fn i_090de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090DE6: jsr $236e.l
pub fn i_090de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90DEC);
    cpu.pc = 0x236E;
    16
}
/// $090DEC: clr.b $21(a6)
pub fn i_090dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DF0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090DF0: clr.b $20(a6)
pub fn i_090df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DF4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090DF4: move.w #$0, d0
pub fn i_090df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090DF8: jsr $28134.l
pub fn i_090df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90DFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90DFE);
    cpu.pc = 0x28134;
    16
}
/// $090DFE: andi.w #$ffe3, $38(a6)
pub fn i_090dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E04;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090E04: ori.w #$0, $38(a6)
pub fn i_090e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E0A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090E0A: move.w #$b5, $7c(a6)
pub fn i_090e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090E10: lea.l $2f4eec.l, a0
pub fn i_090e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4EEC;
    20
}
/// $090E16: jsr $28cd4.l
pub fn i_090e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90E1C);
    cpu.pc = 0x28CD4;
    16
}
/// $090E1C: lea.l $90e22(pc), a1
pub fn i_090e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E22;
    16
}
/// $090E20: move.l a1, (a6)
pub fn i_090e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090E22: jsr $910a2(pc)
pub fn i_090e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90E26);
    cpu.pc = 0x910A2;
    16
}
/// $090E26: bcs.w $90e6e
pub fn i_090e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E2A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x90E6E; return 10; }
    8
}
/// $090E2A: movea.l $c(a6), a0
pub fn i_090e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E2E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $090E2E: move.w $22(a0), $22(a6)
pub fn i_090e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E34;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $090E34: move.w $24(a0), $24(a6)
pub fn i_090e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E3A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $090E3A: jsr $28d70.l
pub fn i_090e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90E40);
    cpu.pc = 0x28D70;
    16
}
/// $090E40: movea.l $c(a6), a0
pub fn i_090e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E44;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $090E44: cmpi.b #$1, $21(a0)
pub fn i_090e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E4A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $090E4A: bne.w $90e6e
pub fn i_090e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90E6E; return 10; }
    8
}
/// $090E4E: tst.b $21(a6)
pub fn i_090e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E52;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $090E52: bne.w $90e6e
pub fn i_090e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x90E6E; return 10; }
    8
}
/// $090E56: move.b #$ff, $21(a6)
pub fn i_090e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090E5C: move.w $14(a6), d1
pub fn i_090e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E60;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090E60: move.w $7c(a6), d2
pub fn i_090e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E64;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090E64: moveq #$1, d3
pub fn i_090e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E66;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090E66: moveq #$2, d4
pub fn i_090e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E68;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $090E68: jsr $2c30.l
pub fn i_090e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90E6E);
    cpu.pc = 0x2C30;
    16
}
/// $090E6E: jsr $9110a(pc)
pub fn i_090e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90E72);
    cpu.pc = 0x9110A;
    16
}
/// $090E72: bcc.w $90e7c
pub fn i_090e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x90E7C; return 10; }
    8
}
// ===== SetTaskHandler_090e76  ($090E76, 8 B, kernel/task_handlers.c)
/// $090E76: lea.l $90e7e(pc), a1
pub fn i_090e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E7E;
    16
}
/// $090E7A: move.l a1, (a6)
pub fn i_090e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E7C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090E7C: rts
pub fn i_090e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_090e7e  ($090E7E, 8 B, kernel/jmp_to_scheduler.c)
/// $090E7E: jmp $518.l
pub fn i_090e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E84;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== GO_Figure_Part_090e86  ($090E86, 114 B, mission/gameover_continue_08f6d2.s)
/// $090E86: move.w #$bc, d1
pub fn i_090e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E8A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090E8A: jsr $236e.l
pub fn i_090e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90E90);
    cpu.pc = 0x236E;
    16
}
/// $090E90: clr.b $21(a6)
pub fn i_090e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E94;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090E94: clr.b $20(a6)
pub fn i_090e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E98;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090E98: move.w #$0, d0
pub fn i_090e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90E9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090E9C: jsr $28134.l
pub fn i_090e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90EA2);
    cpu.pc = 0x28134;
    16
}
/// $090EA2: andi.w #$ffe3, $38(a6)
pub fn i_090ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EA8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090EA8: ori.w #$0, $38(a6)
pub fn i_090ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EAE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090EAE: move.w #$bc, $7c(a6)
pub fn i_090eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090EB4: lea.l $2f4ef8.l, a0
pub fn i_090eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4EF8;
    20
}
/// $090EBA: jsr $28cd4.l
pub fn i_090eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90EC0);
    cpu.pc = 0x28CD4;
    16
}
/// $090EC0: lea.l $90ec6(pc), a1
pub fn i_090ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90EC6;
    16
}
/// $090EC4: move.l a1, (a6)
pub fn i_090ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090EC6: jsr $910a2(pc)
pub fn i_090ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90ECA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90ECA);
    cpu.pc = 0x910A2;
    16
}
/// $090ECA: bcs.w $90efe
pub fn i_090eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90ECE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x90EFE; return 10; }
    8
}
/// $090ECE: movea.l $c(a6), a1
pub fn i_090ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90ED2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $090ED2: lea.l $2f4a4a.l, a0
pub fn i_090ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90ED8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4A4A;
    20
}
/// $090ED8: move.w $80(a6), d0
pub fn i_090ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EDC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090EDC: add.w d0, d0
pub fn i_090edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EDE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090EDE: add.w d0, d0
pub fn i_090ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EE0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090EE0: move.w (a0, d0.w), d1
pub fn i_090ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EE4;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090EE4: add.w $22(a1), d1
pub fn i_090ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EE8;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090EE8: move.w d1, $22(a6)
pub fn i_090ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EEC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090EEC: move.w $2(a0, d0.w), d1
pub fn i_090eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EF0;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090EF0: add.w $24(a1), d1
pub fn i_090ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EF4;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090EF4: move.w d1, $24(a6)
pub fn i_090ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EF8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_090ef8  ($090EF8, 8 B, thunks/jsr_abs_thunks.c)
/// $090EF8: jsr $28d70.l
pub fn i_090ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90EFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90EFE);
    cpu.pc = 0x28D70;
    16
}
/// $090EFE: rts
pub fn i_090efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Scroller_A_090f00  ($090F00, 104 B, mission/gameover_continue_08f6d2.s)
/// $090F00: move.w #$c4, d1
pub fn i_090f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F04;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090F04: jsr $236e.l
pub fn i_090f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90F0A);
    cpu.pc = 0x236E;
    16
}
/// $090F0A: clr.b $21(a6)
pub fn i_090f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F0E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090F0E: clr.b $20(a6)
pub fn i_090f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F12;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090F12: move.w #$0, d0
pub fn i_090f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F16;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090F16: jsr $28134.l
pub fn i_090f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90F1C);
    cpu.pc = 0x28134;
    16
}
/// $090F1C: andi.w #$ffe3, $38(a6)
pub fn i_090f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F22;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090F22: ori.w #$10, $38(a6)
pub fn i_090f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F28;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090F28: move.w #$200, $28(a6)
pub fn i_090f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F2E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090F2E: lea.l $2f4a72.l, a0
pub fn i_090f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4A72;
    20
}
/// $090F34: move.w $80(a6), d0
pub fn i_090f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F38;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090F38: add.w d0, d0
pub fn i_090f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F3A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090F3A: move.w (a0, d0.w), d1
pub fn i_090f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F3E;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090F3E: add.w $22(a6), d1
pub fn i_090f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F42;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090F42: move.w d1, $22(a6)
pub fn i_090f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F46;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090F46: move.w #$b5, $7c(a6)
pub fn i_090f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F4C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090F4C: move.w #$10, $82(a6)
pub fn i_090f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090F52: lea.l $2f4ed4.l, a0
pub fn i_090f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4ED4;
    20
}
/// $090F58: jsr $28cd4.l
pub fn i_090f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90F5E);
    cpu.pc = 0x28CD4;
    16
}
/// $090F5E: lea.l $90fce(pc), a1
pub fn i_090f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90FCE;
    16
}
/// $090F62: move.l a1, (a6)
pub fn i_090f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $090F64: bra.w $90fce
pub fn i_090f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F68;
    cpu.ir = 0x6000;
    cpu.pc = 0x90FCE;
    10
}
// ===== GO_Scroller_B_090f68  ($090F68, 102 B, mission/gameover_continue_08f6d2.s)
/// $090F68: move.w #$c4, d1
pub fn i_090f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F6C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090F6C: jsr $236e.l
pub fn i_090f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90F72);
    cpu.pc = 0x236E;
    16
}
/// $090F72: move.b #$1, $21(a6)
pub fn i_090f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $090F78: clr.b $20(a6)
pub fn i_090f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F7C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $090F7C: move.w #$0, d0
pub fn i_090f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F80;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $090F80: jsr $28134.l
pub fn i_090f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90F86);
    cpu.pc = 0x28134;
    16
}
/// $090F86: andi.w #$ffe3, $38(a6)
pub fn i_090f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F8C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090F8C: ori.w #$c, $38(a6)
pub fn i_090f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F92;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $090F92: move.w #$180, $28(a6)
pub fn i_090f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F98;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090F98: lea.l $2f4a78.l, a0
pub fn i_090f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90F9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4A78;
    20
}
/// $090F9E: move.w $80(a6), d0
pub fn i_090f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FA2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090FA2: add.w d0, d0
pub fn i_090fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FA4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $090FA4: move.w (a0, d0.w), d1
pub fn i_090fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FA8;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $090FA8: add.w $22(a6), d1
pub fn i_090fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FAC;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $090FAC: move.w d1, $22(a6)
pub fn i_090fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FB0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090FB0: move.w #$b5, $7c(a6)
pub fn i_090fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FB6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090FB6: move.w #$10, $82(a6)
pub fn i_090fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FBC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $090FBC: lea.l $2f4ee0.l, a0
pub fn i_090fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4EE0;
    20
}
/// $090FC2: jsr $28cd4.l
pub fn i_090fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90FC8);
    cpu.pc = 0x28CD4;
    16
}
/// $090FC8: lea.l $90fce(pc), a1
pub fn i_090fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90FCE;
    16
}
/// $090FCC: move.l a1, (a6)
pub fn i_090fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== GO_Scroller_Run_090fce  ($090FCE, 192 B, mission/gameover_continue_08f6d2.s)
/// $090FCE: jsr $910a2(pc)
pub fn i_090fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x90FD2);
    cpu.pc = 0x910A2;
    16
}
/// $090FD2: bcs.w $91086
pub fn i_090fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FD6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x91086; return 10; }
    8
}
/// $090FD6: movea.l $c(a6), a0
pub fn i_090fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FDA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $090FDA: move.w $24(a0), $24(a6)
pub fn i_090fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FE0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $090FE0: addi.w #$10, $24(a6)
pub fn i_090fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FE6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $090FE6: cmpi.b #$a0, $32(a6)
pub fn i_090fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FEC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Byte, false);
    16
}
/// $090FEC: ble.w $9100c
pub fn i_090fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FF0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9100C; return 10; }
    8
}
/// $090FF0: move.w $22(a6), d0
pub fn i_090ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FF4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $090FF4: cmpi.w #$20, d0
pub fn i_090ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FF8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $090FF8: blt.w $9100c
pub fn i_090ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90FFC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x9100C; return 10; }
    8
}
/// $090FFC: andi.w #$1, d0
pub fn i_090ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91000;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $091000: beq.w $9100c
pub fn i_091000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91004;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9100C; return 10; }
    8
}
/// $091004: subq.b #$2, $32(a6)
pub fn i_091004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91008;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $091008: subq.b #$2, $33(a6)
pub fn i_091008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9100C;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09100C: move.w $28(a6), d0
pub fn i_09100c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91010;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $091010: move.w d0, d1
pub fn i_091010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91012;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091012: asr.w #$8, d1
pub fn i_091012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91014;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $091014: add.b d0, $26(a6)
pub fn i_091014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91018;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $091018: moveq #$0, d0
pub fn i_091018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9101A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09101A: addx.w d0, d1
pub fn i_09101a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9101C;
    cpu.ir = 0xD340;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09101C: add.w d1, $22(a6)
pub fn i_09101c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91020;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $091020: cmpi.w #$110, $22(a6)
pub fn i_091020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91026;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $091026: ble.w $91042
pub fn i_091026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9102A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x91042; return 10; }
    8
}
/// $09102A: move.b #$ff, $32(a6)
pub fn i_09102a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91030;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $091030: move.b #$ff, $33(a6)
pub fn i_091030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91036;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $091036: move.w #$10, $22(a6)
pub fn i_091036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9103C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09103C: move.w #$0, $26(a6)
pub fn i_09103c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91042;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091042: tst.w $82(a6)
pub fn i_091042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91046;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $091046: beq.w $91052
pub fn i_091046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9104A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x91052; return 10; }
    8
}
/// $09104A: subq.w #$1, $82(a6)
pub fn i_09104a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9104E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09104E: bra.w $91058
pub fn i_09104e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91052;
    cpu.ir = 0x6000;
    cpu.pc = 0x91058;
    10
}
/// $091052: jsr $28d70.l
pub fn i_091052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91058);
    cpu.pc = 0x28D70;
    16
}
/// $091058: movea.l $c(a6), a0
pub fn i_091058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9105C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09105C: cmpi.b #$1, $21(a0)
pub fn i_09105c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91062;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $091062: bne.w $91086
pub fn i_091062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91066;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x91086; return 10; }
    8
}
/// $091066: tst.b $21(a6)
pub fn i_091066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9106A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09106A: bne.w $91086
pub fn i_09106a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9106E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x91086; return 10; }
    8
}
/// $09106E: move.b #$ff, $21(a6)
pub fn i_09106e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91074;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $091074: move.w $14(a6), d1
pub fn i_091074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91078;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $091078: move.w $7c(a6), d2
pub fn i_091078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9107C;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09107C: moveq #$1, d3
pub fn i_09107c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9107E;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09107E: moveq #$2, d4
pub fn i_09107e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91080;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $091080: jsr $2c30.l
pub fn i_091080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91086;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91086);
    cpu.pc = 0x2C30;
    16
}
/// $091086: jsr $9110a(pc)
pub fn i_091086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9108A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9108A);
    cpu.pc = 0x9110A;
    16
}
/// $09108A: bcc.w $91094
pub fn i_09108a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9108E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x91094; return 10; }
    8
}
// ===== SetTaskHandler_09108e  ($09108E, 8 B, kernel/task_handlers.c)
/// $09108E: lea.l $90e7e(pc), a1
pub fn i_09108e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91092;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E7E;
    16
}
/// $091092: move.l a1, (a6)
pub fn i_091092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91094;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $091094: rts
pub fn i_091094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91096;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_CopyParam84_091096  ($091096, 12 B, mission/gameover_continue_08f6d2.s)
/// $091096: movea.l $c(a6), a0
pub fn i_091096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9109A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09109A: move.w $84(a6), $84(a0)
pub fn i_09109a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910A0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x84);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0910A0: rts
pub fn i_0910a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_ParentState21Is2_0910a2  ($0910A2, 14 B, mission/gameover_continue_08f6d2.s)
/// $0910A2: movea.l $c(a6), a0
pub fn i_0910a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910A6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0910A6: cmpi.b #$2, $21(a0)
pub fn i_0910a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910AC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0910AC: bne.w $910b6
pub fn i_0910ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x910B6; return 10; }
    8
}
// ===== SetXN_0910b0  ($0910B0, 6 B, thunks/ccr_helpers.c)
/// $0910B0: ori.b #$11, ccr
pub fn i_0910b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910B4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0910B4: rts
pub fn i_0910b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0910b6  ($0910B6, 6 B, thunks/ccr_helpers.c)
/// $0910B6: andi.b #$ee, ccr
pub fn i_0910b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910BA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0910BA: rts
pub fn i_0910ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_ParentState20Is3_0910d6  ($0910D6, 14 B, mission/gameover_continue_08f6d2.s)
/// $0910D6: movea.l $c(a6), a0
pub fn i_0910d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910DA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0910DA: cmpi.b #$3, $20(a0)
pub fn i_0910da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910E0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $0910E0: bne.w $910ea
pub fn i_0910e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x910EA; return 10; }
    8
}
// ===== SetXN_0910e4  ($0910E4, 6 B, thunks/ccr_helpers.c)
/// $0910E4: ori.b #$11, ccr
pub fn i_0910e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0910E8: rts
pub fn i_0910e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0910ea  ($0910EA, 6 B, thunks/ccr_helpers.c)
/// $0910EA: andi.b #$ee, ccr
pub fn i_0910ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0910EE: rts
pub fn i_0910ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_ParentState21Is3_0910f0  ($0910F0, 14 B, mission/gameover_continue_08f6d2.s)
/// $0910F0: movea.l $c(a6), a0
pub fn i_0910f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910F4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0910F4: cmpi.b #$3, $21(a0)
pub fn i_0910f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910FA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $0910FA: bne.w $91104
pub fn i_0910fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x91104; return 10; }
    8
}
// ===== SetXN_0910fe  ($0910FE, 6 B, thunks/ccr_helpers.c)
/// $0910FE: ori.b #$11, ccr
pub fn i_0910fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91102;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $091102: rts
pub fn i_091102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91104;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_091104  ($091104, 6 B, thunks/ccr_helpers.c)
/// $091104: andi.b #$ee, ccr
pub fn i_091104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91108;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $091108: rts
pub fn i_091108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9110A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_ParentState21Is4_09110a  ($09110A, 14 B, mission/gameover_continue_08f6d2.s)
/// $09110A: movea.l $c(a6), a0
pub fn i_09110a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9110E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09110E: cmpi.b #$4, $21(a0)
pub fn i_09110e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91114;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $091114: bne.w $9111e
pub fn i_091114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91118;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9111E; return 10; }
    8
}
// ===== SetXN_091118  ($091118, 6 B, thunks/ccr_helpers.c)
/// $091118: ori.b #$11, ccr
pub fn i_091118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9111C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09111C: rts
pub fn i_09111c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9111E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_09111e  ($09111E, 6 B, thunks/ccr_helpers.c)
/// $09111E: andi.b #$ee, ccr
pub fn i_09111e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91122;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $091122: rts
pub fn i_091122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91124;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Particle_Add_091124  ($091124, 4 B, mission/gameover_continue_08f6d2.s)
/// $091124: lea.l $91130(pc), a1
pub fn i_091124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91128;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91130;
    16
}
// ===== JsrAbsThunk_091128  ($091128, 8 B, thunks/jsr_abs_thunks.c)
/// $091128: jsr $6fe.l
pub fn i_091128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9112E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9112E);
    cpu.pc = 0x6FE;
    16
}
/// $09112E: rts
pub fn i_09112e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91130;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Particle_091130  ($091130, 346 B, mission/gameover_continue_08f6d2.s)
/// $091130: jsr $5e9b6.l
pub fn i_091130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91136);
    cpu.pc = 0x5E9B6;
    16
}
/// $091136: andi.w #$1, d0
pub fn i_091136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9113A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09113A: beq.w $911e6
pub fn i_09113a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9113E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x911E6; return 10; }
    8
}
/// $09113E: move.w #$e0, d1
pub fn i_09113e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91142;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091142: jsr $236e.l
pub fn i_091142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91148);
    cpu.pc = 0x236E;
    16
}
/// $091148: move.w #$20, $22(a6)
pub fn i_091148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9114E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09114E: move.w #$128, $24(a6)
pub fn i_09114e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91154;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091154: jsr $5e9b6.l
pub fn i_091154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9115A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9115A);
    cpu.pc = 0x5E9B6;
    16
}
/// $09115A: andi.w #$f, d0
pub fn i_09115a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9115E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09115E: subq.w #$8, d0
pub fn i_09115e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91160;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $091160: add.w d0, $22(a6)
pub fn i_091160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91164;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $091164: move.w #$200, $28(a6)
pub fn i_091164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9116A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09116A: move.w #$c00, $2a(a6)
pub fn i_09116a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91170;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091170: move.w #$a0, $2e(a6)
pub fn i_091170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91176;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091176: move.w #$0, $82(a6)
pub fn i_091176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9117C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09117C: jsr $5e9b6.l
pub fn i_09117c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91182);
    cpu.pc = 0x5E9B6;
    16
}
/// $091182: andi.w #$f0, d0
pub fn i_091182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91186;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $091186: subi.w #$80, d0
pub fn i_091186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9118A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09118A: add.w d0, $28(a6)
pub fn i_09118a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9118E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09118E: jsr $5e9b6.l
pub fn i_09118e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91194);
    cpu.pc = 0x5E9B6;
    16
}
/// $091194: andi.w #$30, d0
pub fn i_091194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91198;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x30) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $091198: add.w d0, $28(a6)
pub fn i_091198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9119C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09119C: move.w #$2000, d0
pub fn i_09119c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0911A0: jsr $28134.l
pub fn i_0911a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x911A6);
    cpu.pc = 0x28134;
    16
}
/// $0911A6: andi.w #$ffe3, $38(a6)
pub fn i_0911a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911AC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0911AC: ori.w #$0, $38(a6)
pub fn i_0911ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911B2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0911B2: jsr $5e9b6.l
pub fn i_0911b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x911B8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0911B8: andi.w #$3, d0
pub fn i_0911b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911BC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0911BC: beq.w $911d0
pub fn i_0911bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x911D0; return 10; }
    8
}
/// $0911C0: lea.l $2f512a.l, a0
pub fn i_0911c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F512A;
    20
}
/// $0911C6: jsr $28cd4.l
pub fn i_0911c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x911CC);
    cpu.pc = 0x28CD4;
    16
}
/// $0911CC: bra.w $911dc
pub fn i_0911cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x911DC;
    10
}
/// $0911D0: lea.l $2f5276.l, a0
pub fn i_0911d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5276;
    20
}
/// $0911D6: jsr $28cd4.l
pub fn i_0911d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x911DC);
    cpu.pc = 0x28CD4;
    16
}
/// $0911DC: lea.l $9128a(pc), a1
pub fn i_0911dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9128A;
    16
}
/// $0911E0: move.l a1, (a6)
pub fn i_0911e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0911E2: bra.w $9128a
pub fn i_0911e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x9128A;
    10
}
/// $0911E6: move.w #$e0, d1
pub fn i_0911e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0911EA: jsr $236e.l
pub fn i_0911ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x911F0);
    cpu.pc = 0x236E;
    16
}
/// $0911F0: move.w #$20, $22(a6)
pub fn i_0911f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0911F6: move.w #$128, $24(a6)
pub fn i_0911f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x911FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0911FC: jsr $5e9b6.l
pub fn i_0911fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91202);
    cpu.pc = 0x5E9B6;
    16
}
/// $091202: andi.w #$f, d0
pub fn i_091202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91206;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $091206: subq.w #$8, d0
pub fn i_091206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91208;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $091208: add.w d0, $22(a6)
pub fn i_091208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9120C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09120C: move.w #$300, $28(a6)
pub fn i_09120c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91212;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091212: move.w #$c00, $2a(a6)
pub fn i_091212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91218;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091218: move.w #$90, $2e(a6)
pub fn i_091218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9121E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09121E: move.w #$0, $82(a6)
pub fn i_09121e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91224;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091224: jsr $5e9b6.l
pub fn i_091224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9122A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9122A);
    cpu.pc = 0x5E9B6;
    16
}
/// $09122A: andi.w #$f0, d0
pub fn i_09122a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9122E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09122E: subi.w #$80, d0
pub fn i_09122e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91232;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $091232: add.w d0, $28(a6)
pub fn i_091232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91236;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $091236: jsr $5e9b6.l
pub fn i_091236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9123C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9123C);
    cpu.pc = 0x5E9B6;
    16
}
/// $09123C: andi.w #$30, d0
pub fn i_09123c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91240;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x30) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $091240: add.w d0, $28(a6)
pub fn i_091240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91244;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $091244: move.w #$2000, d0
pub fn i_091244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91248;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091248: jsr $28134.l
pub fn i_091248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9124E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9124E);
    cpu.pc = 0x28134;
    16
}
/// $09124E: andi.w #$ffe3, $38(a6)
pub fn i_09124e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91254;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $091254: ori.w #$0, $38(a6)
pub fn i_091254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9125A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09125A: jsr $5e9b6.l
pub fn i_09125a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91260;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91260);
    cpu.pc = 0x5E9B6;
    16
}
/// $091260: andi.w #$3, d0
pub fn i_091260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91264;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $091264: beq.w $91278
pub fn i_091264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91268;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x91278; return 10; }
    8
}
/// $091268: lea.l $2f51d0.l, a0
pub fn i_091268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9126E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F51D0;
    20
}
/// $09126E: jsr $28cd4.l
pub fn i_09126e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91274);
    cpu.pc = 0x28CD4;
    16
}
/// $091274: bra.w $91284
pub fn i_091274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91278;
    cpu.ir = 0x6000;
    cpu.pc = 0x91284;
    10
}
/// $091278: lea.l $2f5276.l, a0
pub fn i_091278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9127E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5276;
    20
}
/// $09127E: jsr $28cd4.l
pub fn i_09127e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91284;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91284);
    cpu.pc = 0x28CD4;
    16
}
/// $091284: lea.l $9128a(pc), a1
pub fn i_091284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91288;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9128A;
    16
}
/// $091288: move.l a1, (a6)
pub fn i_091288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9128A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== GO_Particle_Run_09128a  ($09128A, 166 B, mission/gameover_continue_08f6d2.s)
/// $09128A: movea.l $c(a6), a0
pub fn i_09128a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9128E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09128E: movea.l $c(a0), a0
pub fn i_09128e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91292;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $091292: cmpi.b #$2, $20(a0)
pub fn i_091292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91298;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $091298: beq.w $91328
pub fn i_091298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9129C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x91328; return 10; }
    8
}
/// $09129C: move.w $28(a6), d0
pub fn i_09129c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912A0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0912A0: move.w d0, d1
pub fn i_0912a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912A2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0912A2: asr.w #$8, d1
pub fn i_0912a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912A4;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0912A4: add.b d0, $26(a6)
pub fn i_0912a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912A8;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0912A8: moveq #$0, d0
pub fn i_0912a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912AA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0912AA: addx.w d0, d1
pub fn i_0912aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912AC;
    cpu.ir = 0xD340;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0912AC: add.w d1, $22(a6)
pub fn i_0912ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912B0;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0912B0: move.w $28(a6), d0
pub fn i_0912b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0912B4: move.w d0, d1
pub fn i_0912b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912B6;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0912B6: asr.w #$8, d1
pub fn i_0912b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912B8;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0912B8: add.b d0, $26(a6)
pub fn i_0912b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912BC;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0912BC: moveq #$0, d0
pub fn i_0912bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912BE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0912BE: addx.w d0, d1
pub fn i_0912be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912C0;
    cpu.ir = 0xD340;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0912C0: add.w d1, $22(a6)
pub fn i_0912c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912C4;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0912C4: move.w $2e(a6), d0
pub fn i_0912c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912C8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0912C8: sub.w d0, $2a(a6)
pub fn i_0912c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912CC;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0912CC: move.w $2a(a6), d0
pub fn i_0912cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0912D0: move.w d0, d1
pub fn i_0912d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912D2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0912D2: asr.w #$8, d1
pub fn i_0912d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912D4;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0912D4: add.b d0, $27(a6)
pub fn i_0912d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912D8;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0912D8: moveq #$0, d0
pub fn i_0912d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912DA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0912DA: addx.w d0, d1
pub fn i_0912da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912DC;
    cpu.ir = 0xD340;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0912DC: add.w d1, $24(a6)
pub fn i_0912dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912E0;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0912E0: move.w $2e(a6), d0
pub fn i_0912e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912E4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0912E4: sub.w d0, $2a(a6)
pub fn i_0912e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912E8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0912E8: move.w $2a(a6), d0
pub fn i_0912e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912EC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0912EC: move.w d0, d1
pub fn i_0912ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912EE;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0912EE: asr.w #$8, d1
pub fn i_0912ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912F0;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0912F0: add.b d0, $27(a6)
pub fn i_0912f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912F4;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0912F4: moveq #$0, d0
pub fn i_0912f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912F6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0912F6: addx.w d0, d1
pub fn i_0912f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912F8;
    cpu.ir = 0xD340;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0912F8: add.w d1, $24(a6)
pub fn i_0912f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x912FC;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0912FC: jsr $28d70.l
pub fn i_0912fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91302);
    cpu.pc = 0x28D70;
    16
}
/// $091302: addq.w #$1, $82(a6)
pub fn i_091302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91306;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $091306: cmpi.w #$1, $82(a6)
pub fn i_091306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9130C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $09130C: blt.w $9131e
pub fn i_09130c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91310;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x9131E; return 10; }
    8
}
/// $091310: move.w #$0, $82(a6)
pub fn i_091310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91316;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091316: subq.b #$6, $32(a6)
pub fn i_091316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9131A;
    cpu.ir = 0x5D2E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 6, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09131A: subq.b #$6, $33(a6)
pub fn i_09131a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9131E;
    cpu.ir = 0x5D2E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 6, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09131E: cmpi.w #$120, $24(a6)
pub fn i_09131e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91324;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $091324: bgt.w $9132e
pub fn i_091324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91328;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9132E; return 10; }
    8
}
/// $091328: jmp $518.l
pub fn i_091328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9132E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09132E: rts
pub fn i_09132e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91330;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_091330  ($091330, 8 B, kernel/task_handlers.c)
/// $091330: lea.l $91338(pc), a1
pub fn i_091330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91334;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91338;
    16
}
/// $091334: move.l a1, (a6)
pub fn i_091334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91336;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $091336: rts
pub fn i_091336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_Spawn_091338  ($091338, 108 B, mission/gameover_continue_08f6d2.s)
/// $091338: move.w #$90, $22(a6)
pub fn i_091338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9133E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09133E: move.w #$1c0, $24(a6)
pub fn i_09133e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91344;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091344: clr.b $20(a6)
pub fn i_091344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91348;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $091348: clr.b $21(a6)
pub fn i_091348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9134C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09134C: lea.l $91578(pc), a1
pub fn i_09134c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91350;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91578;
    16
}
/// $091350: jsr $4ae.l
pub fn i_091350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91356);
    cpu.pc = 0x4AE;
    16
}
/// $091356: lea.l $91514(pc), a1
pub fn i_091356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9135A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91514;
    16
}
/// $09135A: jsr $4ae.l
pub fn i_09135a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91360);
    cpu.pc = 0x4AE;
    16
}
/// $091360: lea.l $9148c(pc), a1
pub fn i_091360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91364;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9148C;
    16
}
/// $091364: jsr $4ae.l
pub fn i_091364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9136A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9136A);
    cpu.pc = 0x4AE;
    16
}
/// $09136A: jsr $5dd02.l
pub fn i_09136a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91370);
    cpu.pc = 0x5DD02;
    16
}
/// $091370: lea.l $9144a(pc), a1
pub fn i_091370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91374;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9144A;
    16
}
/// $091374: jsr $4ae.l
pub fn i_091374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9137A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9137A);
    cpu.pc = 0x4AE;
    16
}
/// $09137A: jsr $5dd02.l
pub fn i_09137a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91380);
    cpu.pc = 0x5DD02;
    16
}
/// $091380: lea.l $91408(pc), a1
pub fn i_091380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91384;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91408;
    16
}
/// $091384: jsr $4ae.l
pub fn i_091384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9138A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9138A);
    cpu.pc = 0x4AE;
    16
}
/// $09138A: jsr $5dd02.l
pub fn i_09138a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91390);
    cpu.pc = 0x5DD02;
    16
}
/// $091390: lea.l $914ce(pc), a1
pub fn i_091390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91394;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x914CE;
    16
}
/// $091394: jsr $4ae.l
pub fn i_091394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9139A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9139A);
    cpu.pc = 0x4AE;
    16
}
/// $09139A: jsr $5dd02.l
pub fn i_09139a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x913A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x913A0);
    cpu.pc = 0x5DD02;
    16
}
/// $0913A0: jsr $91618(pc)
pub fn i_0913a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x913A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x913A4);
    cpu.pc = 0x91618;
    16
}
// ===== SetTaskHandler_0913a4  ($0913A4, 8 B, kernel/task_handlers.c)
/// $0913A4: lea.l $913aa(pc), a1
pub fn i_0913a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x913A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x913AA;
    16
}
/// $0913A8: move.l a1, (a6)
pub fn i_0913a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x913AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0913AA: rts
pub fn i_0913aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x913AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_Sprite_Mid_091408  ($091408, 66 B, mission/gameover_continue_08f6d2.s)
/// $091408: move.w #$de, d1
pub fn i_091408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9140C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09140C: jsr $236e.l
pub fn i_09140c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91412;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91412);
    cpu.pc = 0x236E;
    16
}
/// $091412: move.w #$a0, $22(a6)
pub fn i_091412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91418;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091418: move.w #$180, $24(a6)
pub fn i_091418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9141E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09141E: move.w #$2000, d0
pub fn i_09141e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91422;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091422: jsr $28134.l
pub fn i_091422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91428);
    cpu.pc = 0x28134;
    16
}
/// $091428: andi.w #$ffe3, $38(a6)
pub fn i_091428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9142E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09142E: ori.w #$0, $38(a6)
pub fn i_09142e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91434;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $091434: lea.l $2f5056.l, a0
pub fn i_091434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9143A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5056;
    20
}
/// $09143A: jsr $28cd4.l
pub fn i_09143a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91440);
    cpu.pc = 0x28CD4;
    16
}
/// $091440: lea.l $9150c(pc), a1
pub fn i_091440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91444;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9150C;
    16
}
/// $091444: move.l a1, (a6)
pub fn i_091444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91446;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $091446: bra.w $9150c
pub fn i_091446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9144A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9150C;
    10
}
// ===== Continue_Sprite_Right_09144a  ($09144A, 66 B, mission/gameover_continue_08f6d2.s)
/// $09144A: move.w #$d9, d1
pub fn i_09144a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9144E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09144E: jsr $236e.l
pub fn i_09144e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91454);
    cpu.pc = 0x236E;
    16
}
/// $091454: move.w #$130, $22(a6)
pub fn i_091454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9145A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x130;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09145A: move.w #$180, $24(a6)
pub fn i_09145a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91460;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091460: move.w #$d000, d0
pub fn i_091460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91464;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091464: jsr $28134.l
pub fn i_091464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9146A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9146A);
    cpu.pc = 0x28134;
    16
}
/// $09146A: andi.w #$ffe3, $38(a6)
pub fn i_09146a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91470;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $091470: ori.w #$0, $38(a6)
pub fn i_091470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91476;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $091476: lea.l $2f503e.l, a0
pub fn i_091476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9147C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F503E;
    20
}
/// $09147C: jsr $28cd4.l
pub fn i_09147c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91482);
    cpu.pc = 0x28CD4;
    16
}
/// $091482: lea.l $9150c(pc), a1
pub fn i_091482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91486;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9150C;
    16
}
/// $091486: move.l a1, (a6)
pub fn i_091486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91488;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $091488: bra.w $9150c
pub fn i_091488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9148C;
    cpu.ir = 0x6000;
    cpu.pc = 0x9150C;
    10
}
// ===== Continue_Sprite_Left_09148c  ($09148C, 66 B, mission/gameover_continue_08f6d2.s)
/// $09148C: move.w #$da, d1
pub fn i_09148c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91490;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091490: jsr $236e.l
pub fn i_091490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91496);
    cpu.pc = 0x236E;
    16
}
/// $091496: move.w #$20, $22(a6)
pub fn i_091496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9149C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09149C: move.w #$138, $24(a6)
pub fn i_09149c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x138;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0914A2: move.w #$e000, d0
pub fn i_0914a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0914A6: jsr $28134.l
pub fn i_0914a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x914AC);
    cpu.pc = 0x28134;
    16
}
/// $0914AC: andi.w #$ffe3, $38(a6)
pub fn i_0914ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914B2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0914B2: ori.w #$0, $38(a6)
pub fn i_0914b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914B8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0914B8: lea.l $2f504a.l, a0
pub fn i_0914b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F504A;
    20
}
/// $0914BE: jsr $28cd4.l
pub fn i_0914be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x914C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0914C4: lea.l $9150c(pc), a1
pub fn i_0914c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9150C;
    16
}
/// $0914C8: move.l a1, (a6)
pub fn i_0914c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0914CA: bra.w $9150c
pub fn i_0914ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x9150C;
    10
}
// ===== Continue_Sprite_MidB_0914ce  ($0914CE, 62 B, mission/gameover_continue_08f6d2.s)
/// $0914CE: move.w #$dd, d1
pub fn i_0914ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0914D2: jsr $236e.l
pub fn i_0914d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x914D8);
    cpu.pc = 0x236E;
    16
}
/// $0914D8: move.w #$90, $22(a6)
pub fn i_0914d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0914DE: move.w #$160, $24(a6)
pub fn i_0914de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0914E4: move.w #$0, d0
pub fn i_0914e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914E8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0914E8: jsr $28134.l
pub fn i_0914e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x914EE);
    cpu.pc = 0x28134;
    16
}
/// $0914EE: andi.w #$ffe3, $38(a6)
pub fn i_0914ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914F4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0914F4: ori.w #$0, $38(a6)
pub fn i_0914f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914FA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0914FA: lea.l $2f5062.l, a0
pub fn i_0914fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91500;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5062;
    20
}
/// $091500: jsr $28cd4.l
pub fn i_091500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91506;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91506);
    cpu.pc = 0x28CD4;
    16
}
/// $091506: lea.l $9150c(pc), a1
pub fn i_091506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9150A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9150C;
    16
}
/// $09150A: move.l a1, (a6)
pub fn i_09150a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9150C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_09150c  ($09150C, 8 B, thunks/jsr_abs_thunks.c)
/// $09150C: jsr $28d70.l
pub fn i_09150c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91512);
    cpu.pc = 0x28D70;
    16
}
/// $091512: rts
pub fn i_091512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_Text_Init_091514  ($091514, 24 B, mission/gameover_continue_08f6d2.s)
/// $091514: move.w #$1, $82(a6)
pub fn i_091514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9151A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09151A: move.w #$10, $86(a6)
pub fn i_09151a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91520;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091520: move.w #$0, $84(a6)
pub fn i_091520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91526;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $091526: lea.l $9152c(pc), a1
pub fn i_091526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9152A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9152C;
    16
}
/// $09152A: move.l a1, (a6)
pub fn i_09152a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9152C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Continue_Text_Clear_09152c  ($09152C, 36 B, mission/gameover_continue_08f6d2.s)
/// $09152C: movea.l #$7158, a1
pub fn i_09152c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91532;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7158;
    cpu.a[1] = v1;
    4
}
/// $091532: move.w #$3480, d0
pub fn i_091532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91536;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3480;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091536: move.w #$14, d1
pub fn i_091536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9153A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09153A: moveq #$2, d2
pub fn i_09153a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9153C;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09153C: jsr $5da9c.l
pub fn i_09153c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x91542);
    cpu.pc = 0x5DA9C;
    16
}
/// $091542: subq.w #$1, $82(a6)
pub fn i_091542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91546;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $091546: bne.w $91556
pub fn i_091546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9154A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x91556; return 10; }
    8
}
/// $09154A: move.w $86(a6), $82(a6)
pub fn i_09154a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91550;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== SetTaskHandler_091550  ($091550, 8 B, kernel/task_handlers.c)
/// $091550: lea.l $91558(pc), a1
pub fn i_091550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91554;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91558;
    16
}
/// $091554: move.l a1, (a6)
pub fn i_091554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91556;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $091556: rts
pub fn i_091556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91558;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_Text_Blink_091558  ($091558, 24 B, mission/gameover_continue_08f6d2.s)
/// $091558: jsr $52328.l
pub fn i_091558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9155E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9155E);
    cpu.pc = 0x52328;
    16
}
/// $09155E: jsr $915dc(pc)
pub fn i_09155e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91562;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x91562);
    cpu.pc = 0x915DC;
    16
}
/// $091562: subq.w #$1, $82(a6)
pub fn i_091562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91566;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $091566: bne.w $91576
pub fn i_091566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9156A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x91576; return 10; }
    8
}
/// $09156A: move.w $86(a6), $82(a6)
pub fn i_09156a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91570;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== SetTaskHandler_091570  ($091570, 8 B, kernel/task_handlers.c)
/// $091570: lea.l $9152c(pc), a1
pub fn i_091570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91574;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9152C;
    16
}
/// $091574: move.l a1, (a6)
pub fn i_091574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91576;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $091576: rts
pub fn i_091576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91578;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_Text_Countdown_091578  ($091578, 92 B, mission/gameover_continue_08f6d2.s)
/// $091578: movea.l #$7236, a1
pub fn i_091578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9157E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7236;
    cpu.a[1] = v1;
    4
}
/// $09157E: lea.l $2f4cb2.l, a2
pub fn i_09157e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91584;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F4CB2;
    20
}
/// $091584: move.w #$5300, d0
pub fn i_091584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91588;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $091588: jsr $5dad8.l
pub fn i_091588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9158E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9158E);
    cpu.pc = 0x5DAD8;
    16
}
/// $09158E: lea.l $91594(pc), a1
pub fn i_09158e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91592;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91594;
    16
}
/// $091592: move.l a1, (a6)
pub fn i_091592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91594;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $091594: movea.l #$72b6, a1
pub fn i_091594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9159A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x72B6;
    cpu.a[1] = v1;
    4
}
/// $09159A: move.w #$5300, d0
pub fn i_09159a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9159E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09159E: move.b $10fdda.l, d3
pub fn i_09159e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915A4;
    cpu.ir = 0x1639;
    let v2 = bus.read8(0x10FDDA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0915A4: move.b d3, d4
pub fn i_0915a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915A6;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0915A6: lsr.b #$4, d3
pub fn i_0915a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915A8;
    cpu.ir = 0xE80B;
    let h1 = cpu.d[3] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    14
}
/// $0915A8: addi.b #$30, d3
pub fn i_0915a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915AC;
    cpu.ir = 0x603;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0915AC: or.b d3, d0
pub fn i_0915ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915AE;
    cpu.ir = 0x8003;
    let v1 = cpu.d[3] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0915AE: moveq #$1, d1
pub fn i_0915ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915B0;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0915B0: moveq #$1, d2
pub fn i_0915b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915B2;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0915B2: move.l d4, -(a7)
pub fn i_0915b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915B4;
    cpu.ir = 0x2F04;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0915B4: jsr $5da56.l
pub fn i_0915b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x915BA);
    cpu.pc = 0x5DA56;
    16
}
/// $0915BA: move.l (a7)+, d4
pub fn i_0915ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915BC;
    cpu.ir = 0x281F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0915BC: movea.l #$72d6, a1
pub fn i_0915bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915C2;
    cpu.ir = 0x227C;
    let v1: u32 = 0x72D6;
    cpu.a[1] = v1;
    4
}
/// $0915C2: move.w #$5300, d0
pub fn i_0915c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0915C6: andi.b #$f, d4
pub fn i_0915c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915CA;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0915CA: addi.b #$30, d4
pub fn i_0915ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915CE;
    cpu.ir = 0x604;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0915CE: or.b d4, d0
pub fn i_0915ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915D0;
    cpu.ir = 0x8004;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0915D0: moveq #$1, d1
pub fn i_0915d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915D2;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0915D2: moveq #$1, d2
pub fn i_0915d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915D4;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_0915d4  ($0915D4, 8 B, thunks/jsr_abs_thunks.c)
/// $0915D4: jsr $5da56.l
pub fn i_0915d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x915DA);
    cpu.pc = 0x5DA56;
    16
}
/// $0915DA: rts
pub fn i_0915da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_Text_DrawRows_0915dc  ($0915DC, 60 B, mission/gameover_continue_08f6d2.s)
/// $0915DC: lea.l $2f4cb8.l, a0
pub fn i_0915dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4CB8;
    20
}
/// $0915E2: add.w d0, d0
pub fn i_0915e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915E4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0915E4: add.w d0, d0
pub fn i_0915e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915E6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0915E6: movea.l (a0, d0.w), a4
pub fn i_0915e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915EA;
    cpu.ir = 0x2870;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    18
}
/// $0915EA: movea.l #$7158, a1
pub fn i_0915ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915F0;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7158;
    cpu.a[1] = v1;
    4
}
/// $0915F0: moveq #$0, d4
pub fn i_0915f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915F2;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0915F2: move.w (a4, d4.w), d0
pub fn i_0915f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915F6;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0915F6: cmpi.w #$ffff, d0
pub fn i_0915f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915FA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $0915FA: beq.w $91616
pub fn i_0915fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x915FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x91616; return 10; }
    8
}
/// $0915FE: move.l a1, -(a7)
pub fn i_0915fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91600;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $091600: moveq #$1, d1
pub fn i_091600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91602;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $091602: moveq #$2, d2
pub fn i_091602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91604;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $091604: jsr $5da56.l
pub fn i_091604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9160A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9160A);
    cpu.pc = 0x5DA56;
    16
}
/// $09160A: movea.l (a7)+, a1
pub fn i_09160a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9160C;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $09160C: adda.l #$20, a1
pub fn i_09160c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91612;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $091612: addq.w #$2, d4
pub fn i_091612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91614;
    cpu.ir = 0x5444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $091614: bra.b $915f2
pub fn i_091614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91616;
    cpu.ir = 0x60DC;
    cpu.pc = 0x915F2;
    10
}
/// $091616: rts
pub fn i_091616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91618;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_Text_DrawTitle_091618  ($091618, 16 B, mission/gameover_continue_08f6d2.s)
/// $091618: movea.l #$711b, a1
pub fn i_091618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9161E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x711B;
    cpu.a[1] = v1;
    4
}
/// $09161E: lea.l $2f4c98.l, a2
pub fn i_09161e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91624;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F4C98;
    20
}
/// $091624: move.w #$2300, d0
pub fn i_091624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91628;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_091628  ($091628, 8 B, thunks/jsr_abs_thunks.c)
/// $091628: jsr $5dad8.l
pub fn i_091628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9162E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9162E);
    cpu.pc = 0x5DAD8;
    16
}
/// $09162E: rts
pub fn i_09162e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91630;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09196E: cmpi.w #$150, $106f50.l
pub fn i_09196e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91976;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x150, Size::Word, false);
    20
}
/// $091976: scs.b d0
pub fn i_091976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91978;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $091978: lea.l $9197e(pc), a1
pub fn i_091978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9197C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9197E;
    16
}
/// $09197C: rts
pub fn i_09197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9197E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0919B0: cmpi.w #$2e0, $106f50.l
pub fn i_0919b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919B8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x2E0, Size::Word, false);
    20
}
/// $0919B8: scs.b d0
pub fn i_0919b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919BA;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0919BA: lea.l $919c0(pc), a1
pub fn i_0919ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x919C0;
    16
}
/// $0919BE: rts
pub fn i_0919be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0919C8: cmpi.w #$590, $106f50.l
pub fn i_0919c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919D0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x590, Size::Word, false);
    20
}
/// $0919D0: scs.b d0
pub fn i_0919d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919D2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0919D2: lea.l $919d8(pc), a1
pub fn i_0919d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x919D8;
    16
}
/// $0919D6: rts
pub fn i_0919d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x919D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $091A0A: cmpi.w #$5e0, $106f50.l
pub fn i_091a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A12;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x5E0, Size::Word, false);
    20
}
/// $091A12: scs.b d0
pub fn i_091a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A14;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $091A14: lea.l $91a1a(pc), a1
pub fn i_091a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91A1A;
    16
}
/// $091A18: rts
pub fn i_091a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $091A4E: cmpi.w #$600, $106f50.l
pub fn i_091a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A56;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x600, Size::Word, false);
    20
}
/// $091A56: scs.b d0
pub fn i_091a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A58;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $091A58: lea.l $91a5e(pc), a1
pub fn i_091a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91A5E;
    16
}
/// $091A5C: rts
pub fn i_091a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $091A60: cmpi.w #$0, $106e88.l
pub fn i_091a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A68;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106E88) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    20
}
/// $091A68: sne.b d0
pub fn i_091a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A6A;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $091A6A: lea.l $91a70(pc), a1
pub fn i_091a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91A70;
    16
}
/// $091A6E: rts
pub fn i_091a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SelectPositive_TwoSlots_0967C0  ($0967C0, 62 B, frontend/attract_sub_helpers_0967c0.s)
/// $0967C0: lea.l $100440.l, a0
pub fn i_0967c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $0967C6: jsr $5e2d8.l
pub fn i_0967c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x967CC);
    cpu.pc = 0x5E2D8;
    16
}
/// $0967CC: move.l d0, d2
pub fn i_0967cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967CE;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967CE: move.l d1, d3
pub fn i_0967ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967D0;
    cpu.ir = 0x2601;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967D0: lea.l $1004e0.l, a0
pub fn i_0967d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $0967D6: jsr $5e2d8.l
pub fn i_0967d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x967DC);
    cpu.pc = 0x5E2D8;
    16
}
/// $0967DC: tst.l d0
pub fn i_0967dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967DE;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967DE: bpl.w $967e4
pub fn i_0967de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967E2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967E4; return 10; }
    8
}
/// $0967E2: move.l d2, d0
pub fn i_0967e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967E4;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967E4: tst.l d2
pub fn i_0967e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967E6;
    cpu.ir = 0x4A82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967E6: bpl.w $967ec
pub fn i_0967e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967EA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967EC; return 10; }
    8
}
/// $0967EA: move.l d0, d2
pub fn i_0967ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967EC;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967EC: tst.l d1
pub fn i_0967ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967EE;
    cpu.ir = 0x4A81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967EE: bpl.w $967f4
pub fn i_0967ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967F2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967F4; return 10; }
    8
}
/// $0967F2: move.l d3, d1
pub fn i_0967f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967F4;
    cpu.ir = 0x2203;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967F4: tst.l d3
pub fn i_0967f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967F6;
    cpu.ir = 0x4A83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967F6: bpl.w $967fc
pub fn i_0967f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967FA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967FC; return 10; }
    8
}
/// $0967FA: move.l d1, d3
pub fn i_0967fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967FC;
    cpu.ir = 0x2601;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967FC: rts
pub fn i_0967fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State0_Handler_0967FE  ($0967FE, 66 B, frontend/attract_state_machines_0967fe.s)
/// $0967FE: move.b #$0, d0
pub fn i_0967fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96802;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $096802: move.b d0, $21(a6)
pub fn i_096802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96806;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $096806: jsr $43568.l
pub fn i_096806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9680C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9680C);
    cpu.pc = 0x43568;
    16
}
/// $09680C: move.b #$0, $10e39e.l
pub fn i_09680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96814;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $096814: move.w #$0, $70(a6)
pub fn i_096814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9681A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09681A: lea.l $96820(pc), a1
pub fn i_09681a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9681E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96820;
    16
}
/// $09681E: move.l a1, (a6)
pub fn i_09681e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $096820: jsr $967c0(pc)
pub fn i_096820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96824;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96824);
    cpu.pc = 0x967C0;
    16
}
/// $096824: jsr $436de.l
pub fn i_096824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9682A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9682A);
    cpu.pc = 0x436DE;
    16
}
/// $09682A: jsr $969c2(pc)
pub fn i_09682a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9682E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9682E);
    cpu.pc = 0x969C2;
    16
}
/// $09682E: jsr $96a80(pc)
pub fn i_09682e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96832;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96832);
    cpu.pc = 0x96A80;
    16
}
/// $096832: jsr $96b24(pc)
pub fn i_096832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96836;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96836);
    cpu.pc = 0x96B24;
    16
}
/// $096836: move.b #$1, $10e39a.l
pub fn i_096836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9683E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $09683E: rts
pub fn i_09683e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AttractCuller_Cam0_0969C2  ($0969C2, 76 B, frontend/attract_sub_helpers_0967c0.s)
/// $0969C2: move.b $21(a6), d0
pub fn i_0969c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0969C6: jsr $96b7e(pc)
pub fn i_0969c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x969CA);
    cpu.pc = 0x96B7E;
    16
}
/// $0969CA: clr.l d4
pub fn i_0969ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969CC;
    cpu.ir = 0x4284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = r2;
    4
}
/// $0969CC: move.w $70(a6), d4
pub fn i_0969cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969D0;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0969D0: cmpi.w #$ffff, (a0, d4.w)
pub fn i_0969d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969D6;
    cpu.ir = 0xC70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $0969D6: beq.w $96a0c
pub fn i_0969d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96A0C; return 10; }
    8
}
/// $0969DA: move.w $2(a0, d4.w), d0
pub fn i_0969da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969DE;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0969DE: move.l $106f50.l, d1
pub fn i_0969de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969E4;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F50);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0969E4: swap d1
pub fn i_0969e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969E6;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0969E6: addi.w #$140, d1
pub fn i_0969e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969EA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x140, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0969EA: sub.w d1, d0
pub fn i_0969ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969EC;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0969EC: cmpi.w #$0, d0
pub fn i_0969ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969F0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0969F0: bge.w $96a0c
pub fn i_0969f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969F4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x96A0C; return 10; }
    8
}
/// $0969F4: movea.l a0, a1
pub fn i_0969f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969F6;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $0969F6: andi.l #$ffff, d4
pub fn i_0969f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969FC;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $0969FC: adda.l d4, a1
pub fn i_0969fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969FE;
    cpu.ir = 0xD3C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0969FE: jsr $5dcce.l
pub fn i_0969fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96A04);
    cpu.pc = 0x5DCCE;
    16
}
/// $096A04: addi.w #$14, $70(a6)
pub fn i_096a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A0A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $096A0A: bra.b $969c2
pub fn i_096a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A0C;
    cpu.ir = 0x60B6;
    cpu.pc = 0x969C2;
    10
}
/// $096A0C: rts
pub fn i_096a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Viewport_CoordToScreen_096A5A  ($096A5A, 38 B, frontend/attract_sub_helpers_0967c0.s)
/// $096A5A: move.l $106f50.l, d2
pub fn i_096a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A60;
    cpu.ir = 0x2439;
    let v2 = bus.read32(0x106F50);
    cpu.d[2] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096A60: asr.l #$8, d2
pub fn i_096a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A62;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $096A62: asr.l #$8, d2
pub fn i_096a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A64;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $096A64: sub.w d2, d0
pub fn i_096a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A66;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $096A66: addq.w #$1, d0
pub fn i_096a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A68;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $096A68: move.l $106f54.l, d3
pub fn i_096a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A6E;
    cpu.ir = 0x2639;
    let v2 = bus.read32(0x106F54);
    cpu.d[3] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096A6E: asr.l #$8, d3
pub fn i_096a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A70;
    cpu.ir = 0xE083;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[3] = r;
    22
}
/// $096A70: asr.l #$8, d3
pub fn i_096a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A72;
    cpu.ir = 0xE083;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[3] = r;
    22
}
/// $096A72: sub.w d3, d1
pub fn i_096a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A74;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $096A74: move.w #$180, d4
pub fn i_096a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A78;
    cpu.ir = 0x383C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $096A78: sub.w d1, d4
pub fn i_096a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A7A;
    cpu.ir = 0x9841;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $096A7A: move.w d4, d1
pub fn i_096a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A7C;
    cpu.ir = 0x3204;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $096A7C: addq.w #$1, d1
pub fn i_096a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A7E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $096A7E: rts
pub fn i_096a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debug_DrawHUDVars_096A80  ($096A80, 164 B, video/debug_draw_hud_vars_096a80.s)
/// $096A80: btst.b #$0, $100001.l
pub fn i_096a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A88;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $096A88: beq.w $96b22
pub fn i_096a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96B22; return 10; }
    8
}
/// $096A8C: movea.l #$7146, a1
pub fn i_096a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A92;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7146;
    cpu.a[1] = v1;
    4
}
/// $096A92: move.w $106e88.l, d0
pub fn i_096a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A98;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E88) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $096A98: jsr $5d6c2.l
pub fn i_096a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96A9E);
    cpu.pc = 0x5D6C2;
    16
}
/// $096A9E: movea.w #$72e3, a1
pub fn i_096a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AA2;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72E3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AA2: move.w $106f5c.l, d0
pub fn i_096aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AA8;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $096AA8: jsr $5d6c2.l
pub fn i_096aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AAE);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AAE: movea.w #$7383, a1
pub fn i_096aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AB2;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7383;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AB2: move.l $106f50.l, d0
pub fn i_096ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AB8;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096AB8: swap d0
pub fn i_096ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ABA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096ABA: jsr $5d6c2.l
pub fn i_096aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AC0);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AC0: movea.w #$7423, a1
pub fn i_096ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AC4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7423;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AC4: move.l $106f54.l, d0
pub fn i_096ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ACA;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F54);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096ACA: swap d0
pub fn i_096aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ACC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096ACC: jsr $5d6c2.l
pub fn i_096acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AD2);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AD2: movea.w #$7384, a1
pub fn i_096ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AD6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7384;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AD6: move.l $106f50.l, d0
pub fn i_096ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ADC;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096ADC: swap d0
pub fn i_096adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ADE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096ADE: andi.l #$ffff, d0
pub fn i_096ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AE4;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $096AE4: jsr $5d8f2.l
pub fn i_096ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AEA);
    cpu.pc = 0x5D8F2;
    16
}
/// $096AEA: jsr $5d6c2.l
pub fn i_096aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AF0);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AF0: movea.w #$7424, a1
pub fn i_096af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AF4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7424;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AF4: move.l $106f54.l, d0
pub fn i_096af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AFA;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F54);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096AFA: swap d0
pub fn i_096afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AFC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096AFC: andi.l #$ffff, d0
pub fn i_096afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B02;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $096B02: jsr $5d8f2.l
pub fn i_096b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B08);
    cpu.pc = 0x5D8F2;
    16
}
/// $096B08: jsr $5d6c2.l
pub fn i_096b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B0E);
    cpu.pc = 0x5D6C2;
    16
}
/// $096B0E: movea.w #$7385, a1
pub fn i_096b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B12;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7385;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096B12: move.b $10e39c.l, d0
pub fn i_096b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B18;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $096B18: andi.w #$ff, d0
pub fn i_096b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B1C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $096B1C: jsr $5d6c2.l
pub fn i_096b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B22);
    cpu.pc = 0x5D6C2;
    16
}
/// $096B22: rts
pub fn i_096b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugTriggers_TwoBits_096B24  ($096B24, 90 B, frontend/attract_sub_helpers_0967c0.s)
/// $096B24: btst.b #$2, $100001.l
pub fn i_096b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B2C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $096B2C: beq.w $96b50
pub fn i_096b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96B50; return 10; }
    8
}
/// $096B30: jsr $5cd18.l
pub fn i_096b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B36);
    cpu.pc = 0x5CD18;
    16
}
/// $096B36: bcc.w $96b50
pub fn i_096b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B50; return 10; }
    8
}
/// $096B3A: jsr $5d11c.l
pub fn i_096b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B40);
    cpu.pc = 0x5D11C;
    16
}
/// $096B40: bcc.w $96b50
pub fn i_096b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B50; return 10; }
    8
}
/// $096B44: lea.l $86586.l, a1
pub fn i_096b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B4A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86586;
    20
}
/// $096B4A: jsr $4ae.l
pub fn i_096b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B50);
    cpu.pc = 0x4AE;
    16
}
/// $096B50: btst.b #$7, $100001.l
pub fn i_096b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B58;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $096B58: beq.w $96b7c
pub fn i_096b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B5C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96B7C; return 10; }
    8
}
/// $096B5C: jsr $5cd18.l
pub fn i_096b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B62);
    cpu.pc = 0x5CD18;
    16
}
/// $096B62: bcc.w $96b7c
pub fn i_096b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B7C; return 10; }
    8
}
/// $096B66: jsr $5d11c.l
pub fn i_096b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B6C);
    cpu.pc = 0x5D11C;
    16
}
/// $096B6C: bcc.w $96b7c
pub fn i_096b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B7C; return 10; }
    8
}
/// $096B70: lea.l $86582.l, a1
pub fn i_096b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86582;
    20
}
/// $096B76: jsr $4ae.l
pub fn i_096b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B7C);
    cpu.pc = 0x4AE;
    16
}
/// $096B7C: rts
pub fn i_096b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClampAndLookup8_096B7E  ($096B7E, 62 B, frontend/attract_sub_helpers_0967c0.s)
/// $096B7E: cmpi.b #$7, d0
pub fn i_096b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B82;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x7, Size::Byte, false);
    8
}
/// $096B82: ble.w $96b8a
pub fn i_096b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B86;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x96B8A; return 10; }
    8
}
/// $096B86: move.b #$7, d0
pub fn i_096b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B8A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $096B8A: andi.l #$ff, d0
pub fn i_096b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B90;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $096B90: asl.l #$2, d0
pub fn i_096b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B92;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $096B92: lea.l $96b9c(pc), a0
pub fn i_096b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B96;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x96B9C;
    16
}
/// $096B96: movea.l (a0, d0.w), a0
pub fn i_096b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B9A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $096B9A: rts
pub fn i_096b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_LoadDefaults_09773c  ($09773C, 134 B, frontend/hiscore_memcard_options_09773c.s)
/// $09773C: lea.l $2f53b2.l, a0
pub fn i_09773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97742;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F53B2;
    20
}
/// $097742: lea.l $100002.l, a1
pub fn i_097742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97748;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $097748: move.b (a0)+, (a1)+
pub fn i_097748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9774A;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09774A: move.b (a0)+, (a1)+
pub fn i_09774a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9774C;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09774C: move.l (a0)+, (a1)+
pub fn i_09774c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9774E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09774E: move.w (a0)+, (a1)+
pub fn i_09774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97750;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097750: move.w (a0)+, (a1)+
pub fn i_097750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97752;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097752: move.w (a0)+, (a1)+
pub fn i_097752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97754;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097754: move.b (a0)+, (a1)+
pub fn i_097754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97756;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097756: move.b (a0)+, (a1)+
pub fn i_097756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97758;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097758: move.l (a0)+, (a1)+
pub fn i_097758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9775A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09775A: move.w (a0)+, (a1)+
pub fn i_09775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9775C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09775C: move.w (a0)+, (a1)+
pub fn i_09775c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9775E;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09775E: move.w (a0)+, (a1)+
pub fn i_09775e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97760;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097760: move.b (a0)+, (a1)+
pub fn i_097760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97762;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097762: move.b (a0)+, (a1)+
pub fn i_097762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97764;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097764: move.l (a0)+, (a1)+
pub fn i_097764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97766;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $097766: move.w (a0)+, (a1)+
pub fn i_097766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97768;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097768: move.w (a0)+, (a1)+
pub fn i_097768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9776A;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09776A: move.w (a0)+, (a1)+
pub fn i_09776a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9776C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09776C: move.b (a0)+, (a1)+
pub fn i_09776c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9776E;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09776E: move.b (a0)+, (a1)+
pub fn i_09776e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97770;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097770: move.l (a0)+, (a1)+
pub fn i_097770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97772;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $097772: move.w (a0)+, (a1)+
pub fn i_097772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97774;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097774: move.w (a0)+, (a1)+
pub fn i_097774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97776;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097776: move.w (a0)+, (a1)+
pub fn i_097776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97778;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097778: move.b (a0)+, (a1)+
pub fn i_097778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9777A;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09777A: move.b (a0)+, (a1)+
pub fn i_09777a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9777C;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09777C: move.l (a0)+, (a1)+
pub fn i_09777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9777E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09777E: move.w (a0)+, (a1)+
pub fn i_09777e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97780;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097780: move.w (a0)+, (a1)+
pub fn i_097780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97782;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097782: move.w (a0)+, (a1)+
pub fn i_097782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97784;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097784: move.b (a0)+, (a1)+
pub fn i_097784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97786;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097786: move.b (a0)+, (a1)+
pub fn i_097786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97788;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097788: move.l (a0)+, (a1)+
pub fn i_097788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9778A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09778A: move.w (a0)+, (a1)+
pub fn i_09778a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9778C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09778C: move.w (a0)+, (a1)+
pub fn i_09778c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9778E;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09778E: move.w (a0)+, (a1)+
pub fn i_09778e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97790;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097790: move.b (a0)+, (a1)+
pub fn i_097790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97792;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097792: move.b (a0)+, (a1)+
pub fn i_097792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97794;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097794: move.l (a0)+, (a1)+
pub fn i_097794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97796;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $097796: move.w (a0)+, (a1)+
pub fn i_097796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97798;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097798: move.w (a0)+, (a1)+
pub fn i_097798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9779A;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09779A: move.w (a0)+, (a1)+
pub fn i_09779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9779C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09779C: move.b (a0)+, (a1)+
pub fn i_09779c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9779E;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09779E: move.b (a0)+, (a1)+
pub fn i_09779e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A0;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977A0: move.l (a0)+, (a1)+
pub fn i_0977a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A2;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0977A2: move.w (a0)+, (a1)+
pub fn i_0977a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A4;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977A4: move.w (a0)+, (a1)+
pub fn i_0977a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A6;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977A6: move.w (a0)+, (a1)+
pub fn i_0977a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A8;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977A8: move.b (a0)+, (a1)+
pub fn i_0977a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977AA;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977AA: move.b (a0)+, (a1)+
pub fn i_0977aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977AC;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977AC: move.l (a0)+, (a1)+
pub fn i_0977ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977AE;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0977AE: move.w (a0)+, (a1)+
pub fn i_0977ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B0;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977B0: move.w (a0)+, (a1)+
pub fn i_0977b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B2;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977B2: move.w (a0)+, (a1)+
pub fn i_0977b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B4;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977B4: move.b (a0)+, (a1)+
pub fn i_0977b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B6;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977B6: move.b (a0)+, (a1)+
pub fn i_0977b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B8;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977B8: move.l (a0)+, (a1)+
pub fn i_0977b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977BA;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0977BA: move.w (a0)+, (a1)+
pub fn i_0977ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977BC;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977BC: move.w (a0)+, (a1)+
pub fn i_0977bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977BE;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977BE: move.w (a0)+, (a1)+
pub fn i_0977be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977C0;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977C0: rts
pub fn i_0977c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_Tpl_Loader_0977ea  ($0977EA, 96 B, frontend/hiscore_memcard_options_09773c.s)
/// $0977EA: lea.l $9794c(pc), a1
pub fn i_0977ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9794C;
    16
}
/// $0977EE: jsr $4ae.l
pub fn i_0977ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x977F4);
    cpu.pc = 0x4AE;
    16
}
/// $0977F4: move.w #$3c, $80(a6)
pub fn i_0977f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0977FA: lea.l $100002.l, a1
pub fn i_0977fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97800;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $097800: moveq #$0, d1
pub fn i_097800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97802;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097802: move.b d1, (a1)
pub fn i_097802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97804;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097804: move.b d1, (a1)
pub fn i_097804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97806;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097806: move.b d1, (a1)
pub fn i_097806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97808;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097808: move.b d1, (a1)
pub fn i_097808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9780A;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $09780A: move.b d1, (a1)
pub fn i_09780a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9780C;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $09780C: move.b d1, (a1)
pub fn i_09780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9780E;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $09780E: move.b d1, (a1)
pub fn i_09780e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97810;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097810: move.b d1, (a1)
pub fn i_097810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97812;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097812: move.b d1, (a1)
pub fn i_097812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97814;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097814: move.b d1, (a1)
pub fn i_097814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97816;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097816: move.b #$b, d0
pub fn i_097816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9781A;
    cpu.ir = 0x103C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09781A: jsr $43568.l
pub fn i_09781a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97820);
    cpu.pc = 0x43568;
    16
}
/// $097820: moveq #$0, d0
pub fn i_097820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97822;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097822: moveq #$0, d1
pub fn i_097822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97824;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097824: jsr $437da.l
pub fn i_097824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9782A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9782A);
    cpu.pc = 0x437DA;
    16
}
/// $09782A: clr.b $21(a6)
pub fn i_09782a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9782E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09782E: move.w #$0, $70(a6)
pub fn i_09782e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97834;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097834: movea.l #$719b, a1
pub fn i_097834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9783A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x719B;
    cpu.a[1] = v1;
    4
}
/// $09783A: lea.l $2f54a0.l, a2
pub fn i_09783a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97840;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F54A0;
    20
}
/// $097840: move.w #$4300, d0
pub fn i_097840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97844;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097844: jsr $5dad8.l
pub fn i_097844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9784A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9784A);
    cpu.pc = 0x5DAD8;
    16
}
// ===== SetTaskHandler_09784a  ($09784A, 8 B, kernel/task_handlers.c)
/// $09784A: lea.l $97852(pc), a1
pub fn i_09784a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9784E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97852;
    16
}
/// $09784E: move.l a1, (a6)
pub fn i_09784e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97850;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097850: rts
pub fn i_097850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97852;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_WaitLogo_097852  ($097852, 42 B, frontend/hiscore_memcard_options_09773c.s)
/// $097852: tst.b $21(a6)
pub fn i_097852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97856;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $097856: bne.w $9785c
pub fn i_097856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9785A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9785C; return 10; }
    8
}
/// $09785A: rts
pub fn i_09785a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9785C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09785C: clr.b $20(a6)
pub fn i_09785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97860;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $097860: movea.l #$70a6, a1
pub fn i_097860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97866;
    cpu.ir = 0x227C;
    let v1: u32 = 0x70A6;
    cpu.a[1] = v1;
    4
}
/// $097866: move.l a1, $76(a6)
pub fn i_097866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9786A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09786A: move.l #$100002, $72(a6)
pub fn i_09786a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97872;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x100002;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $097872: cmpi.b #$ff, $21(a6)
pub fn i_097872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97878;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $097878: bne.w $97884
pub fn i_097878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9787C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97884; return 10; }
    8
}
// ===== SetTaskHandler_09787c  ($09787C, 8 B, kernel/task_handlers.c)
/// $09787C: lea.l $9788c(pc), a1
pub fn i_09787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97880;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9788C;
    16
}
/// $097880: move.l a1, (a6)
pub fn i_097880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97882;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097882: rts
pub fn i_097882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97884;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_097884  ($097884, 8 B, kernel/task_handlers.c)
/// $097884: lea.l $978fa(pc), a1
pub fn i_097884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97888;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x978FA;
    16
}
/// $097888: move.l a1, (a6)
pub fn i_097888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9788A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09788A: rts
pub fn i_09788a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9788C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_Logo_FadeIn_09794c  ($09794C, 164 B, frontend/hiscore_memcard_options_09773c.s)
/// $09794C: move.b #$ff, $21(a6)
pub fn i_09794c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97952;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097952: move.b #$0, $32(a6)
pub fn i_097952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97958;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097958: move.w #$0, $70(a6)
pub fn i_097958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9795E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09795E: bra.w $97974
pub fn i_09795e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97962;
    cpu.ir = 0x6000;
    cpu.pc = 0x97974;
    10
}
/// $097974: move.w #$15f, d1
pub fn i_097974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97978;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097978: jsr $236e.l
pub fn i_097978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9797E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9797E);
    cpu.pc = 0x236E;
    16
}
/// $09797E: move.w #$a0, $22(a6)
pub fn i_09797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97984;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097984: move.w #$180, $24(a6)
pub fn i_097984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9798A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09798A: move.b #$0, $20(a6)
pub fn i_09798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97990;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097990: jsr $5e9b6.l
pub fn i_097990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97996);
    cpu.pc = 0x5E9B6;
    16
}
/// $097996: andi.w #$1, d0
pub fn i_097996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9799A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09799A: movea.l #$2f5504, a0
pub fn i_09799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979A0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F5504;
    cpu.a[0] = v1;
    4
}
/// $0979A0: lsl.w #$2, d0
pub fn i_0979a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979A2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0979A2: movea.l (a0, d0.w), a0
pub fn i_0979a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0979A6: cmpa.l #$ffffffff, a0
pub fn i_0979a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979AC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0979AC: beq.w $979b6
pub fn i_0979ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x979B6; return 10; }
    8
}
/// $0979B0: jsr $28cd4.l
pub fn i_0979b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x979B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0979B6: lea.l $979bc(pc), a1
pub fn i_0979b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x979BC;
    16
}
/// $0979BA: move.l a1, (a6)
pub fn i_0979ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0979BC: tst.b $20(a6)
pub fn i_0979bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979C0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0979C0: bne.w $979f0
pub fn i_0979c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x979F0; return 10; }
    8
}
/// $0979C4: addi.b #$20, $32(a6)
pub fn i_0979c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979CA;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $0979CA: addi.w #$20, $70(a6)
pub fn i_0979ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979D0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0979D0: cmpi.w #$ff, $70(a6)
pub fn i_0979d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    16
}
/// $0979D6: bcs.w $979f0
pub fn i_0979d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x979F0; return 10; }
    8
}
/// $0979DA: move.b #$ff, $32(a6)
pub fn i_0979da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0979E0: move.b #$ff, $20(a6)
pub fn i_0979e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0979E6: movea.l $c(a6), a0
pub fn i_0979e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0979EA: move.b $21(a6), $21(a0)
pub fn i_0979ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979F0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== JsrAbsThunk_0979f0  ($0979F0, 8 B, thunks/jsr_abs_thunks.c)
/// $0979F0: jsr $28d70.l
pub fn i_0979f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x979F6);
    cpu.pc = 0x28D70;
    16
}
/// $0979F6: rts
pub fn i_0979f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawHeaderA_097a48  ($097A48, 6 B, frontend/hiscore_memcard_options_09773c.s)
/// $097A48: movea.l #$7063, a1
pub fn i_097a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A4E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1;
    4
}
// ===== JsrPcThunk_097a4e  ($097A4E, 6 B, thunks/jsr_pc_thunks.c)
/// $097A4E: jsr $97a60(pc)
pub fn i_097a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97A52);
    cpu.pc = 0x97A60;
    16
}
/// $097A52: rts
pub fn i_097a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_TryEnter_P1_097a60  ($097A60, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $097A60: lea.l $106e94.l, a0
pub fn i_097a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E94;
    20
}
/// $097A66: jsr $51aa4.l
pub fn i_097a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A6C);
    cpu.pc = 0x51AA4;
    16
}
/// $097A6C: moveq #$1, d1
pub fn i_097a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A6E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
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
    (0x090000, i_090000),
    (0x090006, i_090006),
    (0x09000C, i_09000c),
    (0x090012, i_090012),
    (0x090018, i_090018),
    (0x09001C, i_09001c),
    (0x090022, i_090022),
    (0x090028, i_090028),
    (0x09002E, i_09002e),
    (0x090032, i_090032),
    (0x090038, i_090038),
    (0x09003E, i_09003e),
    (0x090044, i_090044),
    (0x090048, i_090048),
    (0x09004A, i_09004a),
    (0x09004C, i_09004c),
    (0x090050, i_090050),
    (0x090056, i_090056),
    (0x09005C, i_09005c),
    (0x090062, i_090062),
    (0x090068, i_090068),
    (0x09006E, i_09006e),
    (0x090072, i_090072),
    (0x090078, i_090078),
    (0x09007E, i_09007e),
    (0x090084, i_090084),
    (0x09008A, i_09008a),
    (0x090090, i_090090),
    (0x090094, i_090094),
    (0x090096, i_090096),
    (0x090098, i_090098),
    (0x09009C, i_09009c),
    (0x0900A0, i_0900a0),
    (0x0900A4, i_0900a4),
    (0x0900A8, i_0900a8),
    (0x0900AC, i_0900ac),
    (0x0900B0, i_0900b0),
    (0x0900B4, i_0900b4),
    (0x0900BA, i_0900ba),
    (0x0900BE, i_0900be),
    (0x0900C2, i_0900c2),
    (0x0900C6, i_0900c6),
    (0x0900CA, i_0900ca),
    (0x0900D0, i_0900d0),
    (0x0900D4, i_0900d4),
    (0x0900D8, i_0900d8),
    (0x0900DA, i_0900da),
    (0x0900DC, i_0900dc),
    (0x0900E2, i_0900e2),
    (0x0900E4, i_0900e4),
    (0x0900E8, i_0900e8),
    (0x0900EE, i_0900ee),
    (0x0900F4, i_0900f4),
    (0x0900FA, i_0900fa),
    (0x0900FE, i_0900fe),
    (0x090104, i_090104),
    (0x09010A, i_09010a),
    (0x090110, i_090110),
    (0x090116, i_090116),
    (0x09011C, i_09011c),
    (0x090122, i_090122),
    (0x090126, i_090126),
    (0x090128, i_090128),
    (0x09012C, i_09012c),
    (0x090130, i_090130),
    (0x090138, i_090138),
    (0x09013E, i_09013e),
    (0x090142, i_090142),
    (0x090148, i_090148),
    (0x09014C, i_09014c),
    (0x09014E, i_09014e),
    (0x090150, i_090150),
    (0x090154, i_090154),
    (0x090158, i_090158),
    (0x09015C, i_09015c),
    (0x090160, i_090160),
    (0x090164, i_090164),
    (0x09016A, i_09016a),
    (0x09016C, i_09016c),
    (0x090170, i_090170),
    (0x090176, i_090176),
    (0x09017C, i_09017c),
    (0x090182, i_090182),
    (0x090186, i_090186),
    (0x09018C, i_09018c),
    (0x090192, i_090192),
    (0x090198, i_090198),
    (0x09019E, i_09019e),
    (0x0901A4, i_0901a4),
    (0x0901AA, i_0901aa),
    (0x0901B0, i_0901b0),
    (0x0901B4, i_0901b4),
    (0x0901B6, i_0901b6),
    (0x0901BA, i_0901ba),
    (0x0901BE, i_0901be),
    (0x0901C6, i_0901c6),
    (0x0901CC, i_0901cc),
    (0x0901D0, i_0901d0),
    (0x0901D6, i_0901d6),
    (0x0901DA, i_0901da),
    (0x0901DE, i_0901de),
    (0x0901E2, i_0901e2),
    (0x0901E4, i_0901e4),
    (0x0901E6, i_0901e6),
    (0x0901EC, i_0901ec),
    (0x0901F0, i_0901f0),
    (0x0901F4, i_0901f4),
    (0x0901F6, i_0901f6),
    (0x0901F8, i_0901f8),
    (0x0901FE, i_0901fe),
    (0x090202, i_090202),
    (0x090206, i_090206),
    (0x090208, i_090208),
    (0x09020A, i_09020a),
    (0x090210, i_090210),
    (0x090214, i_090214),
    (0x090218, i_090218),
    (0x09021A, i_09021a),
    (0x09021C, i_09021c),
    (0x090222, i_090222),
    (0x090228, i_090228),
    (0x09022C, i_09022c),
    (0x09022E, i_09022e),
    (0x090230, i_090230),
    (0x090234, i_090234),
    (0x090238, i_090238),
    (0x09023C, i_09023c),
    (0x090240, i_090240),
    (0x090244, i_090244),
    (0x090248, i_090248),
    (0x09024E, i_09024e),
    (0x090252, i_090252),
    (0x090258, i_090258),
    (0x09025C, i_09025c),
    (0x090260, i_090260),
    (0x090266, i_090266),
    (0x09026C, i_09026c),
    (0x090270, i_090270),
    (0x090276, i_090276),
    (0x09027C, i_09027c),
    (0x090282, i_090282),
    (0x090288, i_090288),
    (0x09028C, i_09028c),
    (0x090290, i_090290),
    (0x090294, i_090294),
    (0x090296, i_090296),
    (0x090298, i_090298),
    (0x09029E, i_09029e),
    (0x0902A2, i_0902a2),
    (0x0902A6, i_0902a6),
    (0x0902A8, i_0902a8),
    (0x0902AA, i_0902aa),
    (0x0902B0, i_0902b0),
    (0x0902B4, i_0902b4),
    (0x0902B8, i_0902b8),
    (0x0902BA, i_0902ba),
    (0x0902BC, i_0902bc),
    (0x0902C2, i_0902c2),
    (0x0902C6, i_0902c6),
    (0x0902CA, i_0902ca),
    (0x0902CC, i_0902cc),
    (0x0902CE, i_0902ce),
    (0x0902D4, i_0902d4),
    (0x0902DA, i_0902da),
    (0x0902DE, i_0902de),
    (0x0902E0, i_0902e0),
    (0x0902E2, i_0902e2),
    (0x0902E6, i_0902e6),
    (0x0902EA, i_0902ea),
    (0x0902EE, i_0902ee),
    (0x0902F2, i_0902f2),
    (0x0902F6, i_0902f6),
    (0x0902FC, i_0902fc),
    (0x0902FE, i_0902fe),
    (0x090302, i_090302),
    (0x090308, i_090308),
    (0x09030E, i_09030e),
    (0x090314, i_090314),
    (0x090318, i_090318),
    (0x09031E, i_09031e),
    (0x090324, i_090324),
    (0x09032A, i_09032a),
    (0x090330, i_090330),
    (0x090336, i_090336),
    (0x09033A, i_09033a),
    (0x09033C, i_09033c),
    (0x090342, i_090342),
    (0x090346, i_090346),
    (0x09034C, i_09034c),
    (0x09034E, i_09034e),
    (0x090352, i_090352),
    (0x090358, i_090358),
    (0x09035C, i_09035c),
    (0x090362, i_090362),
    (0x090368, i_090368),
    (0x09036E, i_09036e),
    (0x090370, i_090370),
    (0x090374, i_090374),
    (0x090378, i_090378),
    (0x09037E, i_09037e),
    (0x090384, i_090384),
    (0x09038A, i_09038a),
    (0x09038E, i_09038e),
    (0x090392, i_090392),
    (0x090398, i_090398),
    (0x09039E, i_09039e),
    (0x0903A2, i_0903a2),
    (0x0903A4, i_0903a4),
    (0x0903A8, i_0903a8),
    (0x0903AC, i_0903ac),
    (0x0903B0, i_0903b0),
    (0x0903B4, i_0903b4),
    (0x0903B8, i_0903b8),
    (0x0903BC, i_0903bc),
    (0x0903C2, i_0903c2),
    (0x0903C8, i_0903c8),
    (0x0903CC, i_0903cc),
    (0x0903D2, i_0903d2),
    (0x0903D6, i_0903d6),
    (0x0903DA, i_0903da),
    (0x0903DE, i_0903de),
    (0x0903E4, i_0903e4),
    (0x0903EA, i_0903ea),
    (0x0903F0, i_0903f0),
    (0x0903F4, i_0903f4),
    (0x0903F6, i_0903f6),
    (0x0903F8, i_0903f8),
    (0x0903FC, i_0903fc),
    (0x0903FE, i_0903fe),
    (0x090402, i_090402),
    (0x090406, i_090406),
    (0x09040C, i_09040c),
    (0x090410, i_090410),
    (0x090414, i_090414),
    (0x090416, i_090416),
    (0x090418, i_090418),
    (0x09041C, i_09041c),
    (0x09041E, i_09041e),
    (0x090422, i_090422),
    (0x090426, i_090426),
    (0x09042C, i_09042c),
    (0x090430, i_090430),
    (0x090434, i_090434),
    (0x090438, i_090438),
    (0x09043C, i_09043c),
    (0x090442, i_090442),
    (0x090446, i_090446),
    (0x09044A, i_09044a),
    (0x09044C, i_09044c),
    (0x090450, i_090450),
    (0x090452, i_090452),
    (0x090456, i_090456),
    (0x09045C, i_09045c),
    (0x090462, i_090462),
    (0x090468, i_090468),
    (0x09046E, i_09046e),
    (0x090472, i_090472),
    (0x090474, i_090474),
    (0x09047A, i_09047a),
    (0x09047E, i_09047e),
    (0x090484, i_090484),
    (0x090488, i_090488),
    (0x09048A, i_09048a),
    (0x09048C, i_09048c),
    (0x090490, i_090490),
    (0x090494, i_090494),
    (0x090498, i_090498),
    (0x09049C, i_09049c),
    (0x0904A2, i_0904a2),
    (0x0904A6, i_0904a6),
    (0x0904AC, i_0904ac),
    (0x0904B0, i_0904b0),
    (0x0904B4, i_0904b4),
    (0x0904BA, i_0904ba),
    (0x0904BE, i_0904be),
    (0x0904C0, i_0904c0),
    (0x0904C4, i_0904c4),
    (0x0904C6, i_0904c6),
    (0x0904CA, i_0904ca),
    (0x0904CE, i_0904ce),
    (0x0904D4, i_0904d4),
    (0x0904DC, i_0904dc),
    (0x0904E2, i_0904e2),
    (0x0904E4, i_0904e4),
    (0x0904E8, i_0904e8),
    (0x0904EE, i_0904ee),
    (0x0904F2, i_0904f2),
    (0x0904F8, i_0904f8),
    (0x0904FE, i_0904fe),
    (0x090504, i_090504),
    (0x09050A, i_09050a),
    (0x090510, i_090510),
    (0x090516, i_090516),
    (0x09051C, i_09051c),
    (0x090522, i_090522),
    (0x090526, i_090526),
    (0x09052C, i_09052c),
    (0x090532, i_090532),
    (0x090538, i_090538),
    (0x09053E, i_09053e),
    (0x090542, i_090542),
    (0x090544, i_090544),
    (0x090548, i_090548),
    (0x09054C, i_09054c),
    (0x090552, i_090552),
    (0x09055A, i_09055a),
    (0x090560, i_090560),
    (0x090564, i_090564),
    (0x090566, i_090566),
    (0x090568, i_090568),
    (0x09056C, i_09056c),
    (0x090570, i_090570),
    (0x090574, i_090574),
    (0x090578, i_090578),
    (0x09057C, i_09057c),
    (0x090580, i_090580),
    (0x090584, i_090584),
    (0x09058A, i_09058a),
    (0x090590, i_090590),
    (0x090592, i_090592),
    (0x090596, i_090596),
    (0x09059C, i_09059c),
    (0x0905A2, i_0905a2),
    (0x0905A8, i_0905a8),
    (0x0905AE, i_0905ae),
    (0x0905B4, i_0905b4),
    (0x0905B8, i_0905b8),
    (0x0905BA, i_0905ba),
    (0x0905C0, i_0905c0),
    (0x0905C2, i_0905c2),
    (0x0905C6, i_0905c6),
    (0x0905CC, i_0905cc),
    (0x0905D0, i_0905d0),
    (0x0905D6, i_0905d6),
    (0x0905DC, i_0905dc),
    (0x0905E2, i_0905e2),
    (0x0905E8, i_0905e8),
    (0x0905EE, i_0905ee),
    (0x0905F4, i_0905f4),
    (0x0905FA, i_0905fa),
    (0x0905FE, i_0905fe),
    (0x090600, i_090600),
    (0x090604, i_090604),
    (0x090608, i_090608),
    (0x09060C, i_09060c),
    (0x090612, i_090612),
    (0x090616, i_090616),
    (0x09061A, i_09061a),
    (0x090620, i_090620),
    (0x090628, i_090628),
    (0x09062E, i_09062e),
    (0x090630, i_090630),
    (0x090634, i_090634),
    (0x09063A, i_09063a),
    (0x09063E, i_09063e),
    (0x090644, i_090644),
    (0x09064A, i_09064a),
    (0x090650, i_090650),
    (0x090656, i_090656),
    (0x09065C, i_09065c),
    (0x090662, i_090662),
    (0x090668, i_090668),
    (0x09066C, i_09066c),
    (0x09066E, i_09066e),
    (0x090672, i_090672),
    (0x090676, i_090676),
    (0x09067C, i_09067c),
    (0x090684, i_090684),
    (0x09068A, i_09068a),
    (0x09068C, i_09068c),
    (0x090690, i_090690),
    (0x090696, i_090696),
    (0x09069A, i_09069a),
    (0x0906A0, i_0906a0),
    (0x0906A6, i_0906a6),
    (0x0906AC, i_0906ac),
    (0x0906B2, i_0906b2),
    (0x0906B8, i_0906b8),
    (0x0906BE, i_0906be),
    (0x0906C4, i_0906c4),
    (0x0906C8, i_0906c8),
    (0x0906CA, i_0906ca),
    (0x0906CE, i_0906ce),
    (0x0906D2, i_0906d2),
    (0x0906D8, i_0906d8),
    (0x0906E0, i_0906e0),
    (0x0906E6, i_0906e6),
    (0x0906E8, i_0906e8),
    (0x0906EC, i_0906ec),
    (0x0906F2, i_0906f2),
    (0x0906F6, i_0906f6),
    (0x0906FC, i_0906fc),
    (0x090702, i_090702),
    (0x090708, i_090708),
    (0x09070E, i_09070e),
    (0x090714, i_090714),
    (0x09071A, i_09071a),
    (0x090720, i_090720),
    (0x090724, i_090724),
    (0x090726, i_090726),
    (0x090728, i_090728),
    (0x09072C, i_09072c),
    (0x090732, i_090732),
    (0x090736, i_090736),
    (0x09073C, i_09073c),
    (0x090742, i_090742),
    (0x090748, i_090748),
    (0x09074C, i_09074c),
    (0x090752, i_090752),
    (0x090758, i_090758),
    (0x09075E, i_09075e),
    (0x090764, i_090764),
    (0x09076A, i_09076a),
    (0x09076E, i_09076e),
    (0x090772, i_090772),
    (0x090778, i_090778),
    (0x09077C, i_09077c),
    (0x090782, i_090782),
    (0x090788, i_090788),
    (0x09078E, i_09078e),
    (0x090792, i_090792),
    (0x090798, i_090798),
    (0x09079E, i_09079e),
    (0x0907A4, i_0907a4),
    (0x0907AA, i_0907aa),
    (0x0907B0, i_0907b0),
    (0x0907B4, i_0907b4),
    (0x0907B8, i_0907b8),
    (0x0907BE, i_0907be),
    (0x0907C2, i_0907c2),
    (0x0907C8, i_0907c8),
    (0x0907CE, i_0907ce),
    (0x0907D4, i_0907d4),
    (0x0907D8, i_0907d8),
    (0x0907DE, i_0907de),
    (0x0907E4, i_0907e4),
    (0x0907EA, i_0907ea),
    (0x0907F0, i_0907f0),
    (0x0907F6, i_0907f6),
    (0x0907FA, i_0907fa),
    (0x0907FE, i_0907fe),
    (0x090804, i_090804),
    (0x090808, i_090808),
    (0x09080E, i_09080e),
    (0x090814, i_090814),
    (0x09081A, i_09081a),
    (0x09081E, i_09081e),
    (0x090824, i_090824),
    (0x09082A, i_09082a),
    (0x090830, i_090830),
    (0x090836, i_090836),
    (0x09083C, i_09083c),
    (0x090840, i_090840),
    (0x090844, i_090844),
    (0x09084A, i_09084a),
    (0x09084E, i_09084e),
    (0x090854, i_090854),
    (0x09085A, i_09085a),
    (0x090860, i_090860),
    (0x090866, i_090866),
    (0x09086C, i_09086c),
    (0x090872, i_090872),
    (0x090878, i_090878),
    (0x09087E, i_09087e),
    (0x090884, i_090884),
    (0x09088A, i_09088a),
    (0x09088E, i_09088e),
    (0x090892, i_090892),
    (0x090898, i_090898),
    (0x09089C, i_09089c),
    (0x0908A2, i_0908a2),
    (0x0908A8, i_0908a8),
    (0x0908AE, i_0908ae),
    (0x0908B2, i_0908b2),
    (0x0908B8, i_0908b8),
    (0x0908BE, i_0908be),
    (0x0908C4, i_0908c4),
    (0x0908CA, i_0908ca),
    (0x0908D0, i_0908d0),
    (0x0908D4, i_0908d4),
    (0x0908D8, i_0908d8),
    (0x0908DE, i_0908de),
    (0x0908E2, i_0908e2),
    (0x0908E8, i_0908e8),
    (0x0908EE, i_0908ee),
    (0x0908F4, i_0908f4),
    (0x0908F8, i_0908f8),
    (0x0908FE, i_0908fe),
    (0x090904, i_090904),
    (0x09090A, i_09090a),
    (0x090910, i_090910),
    (0x090916, i_090916),
    (0x09091A, i_09091a),
    (0x09091E, i_09091e),
    (0x090924, i_090924),
    (0x090928, i_090928),
    (0x09092E, i_09092e),
    (0x090934, i_090934),
    (0x09093A, i_09093a),
    (0x090940, i_090940),
    (0x090946, i_090946),
    (0x09094C, i_09094c),
    (0x090952, i_090952),
    (0x090958, i_090958),
    (0x09095E, i_09095e),
    (0x090964, i_090964),
    (0x090968, i_090968),
    (0x09096C, i_09096c),
    (0x090972, i_090972),
    (0x090976, i_090976),
    (0x09097C, i_09097c),
    (0x090982, i_090982),
    (0x090988, i_090988),
    (0x09098C, i_09098c),
    (0x090992, i_090992),
    (0x090998, i_090998),
    (0x09099E, i_09099e),
    (0x0909A4, i_0909a4),
    (0x0909AA, i_0909aa),
    (0x0909AE, i_0909ae),
    (0x0909B2, i_0909b2),
    (0x0909B8, i_0909b8),
    (0x0909BC, i_0909bc),
    (0x0909C2, i_0909c2),
    (0x0909C8, i_0909c8),
    (0x0909CE, i_0909ce),
    (0x0909D2, i_0909d2),
    (0x0909D8, i_0909d8),
    (0x0909DE, i_0909de),
    (0x0909E4, i_0909e4),
    (0x0909EA, i_0909ea),
    (0x0909F0, i_0909f0),
    (0x0909F4, i_0909f4),
    (0x0909F8, i_0909f8),
    (0x0909FE, i_0909fe),
    (0x090A02, i_090a02),
    (0x090A08, i_090a08),
    (0x090A0E, i_090a0e),
    (0x090A14, i_090a14),
    (0x090A18, i_090a18),
    (0x090A1E, i_090a1e),
    (0x090A24, i_090a24),
    (0x090A2A, i_090a2a),
    (0x090A30, i_090a30),
    (0x090A36, i_090a36),
    (0x090A3A, i_090a3a),
    (0x090A3E, i_090a3e),
    (0x090A44, i_090a44),
    (0x090A48, i_090a48),
    (0x090A4E, i_090a4e),
    (0x090A54, i_090a54),
    (0x090A5A, i_090a5a),
    (0x090A5E, i_090a5e),
    (0x090A64, i_090a64),
    (0x090A6A, i_090a6a),
    (0x090A70, i_090a70),
    (0x090A76, i_090a76),
    (0x090A7C, i_090a7c),
    (0x090A80, i_090a80),
    (0x090A84, i_090a84),
    (0x090A8A, i_090a8a),
    (0x090A8E, i_090a8e),
    (0x090A94, i_090a94),
    (0x090A9A, i_090a9a),
    (0x090AA0, i_090aa0),
    (0x090AA4, i_090aa4),
    (0x090AAA, i_090aaa),
    (0x090AB0, i_090ab0),
    (0x090AB6, i_090ab6),
    (0x090ABC, i_090abc),
    (0x090AC2, i_090ac2),
    (0x090AC6, i_090ac6),
    (0x090ACA, i_090aca),
    (0x090AD0, i_090ad0),
    (0x090AD4, i_090ad4),
    (0x090ADA, i_090ada),
    (0x090AE0, i_090ae0),
    (0x090AE6, i_090ae6),
    (0x090AEA, i_090aea),
    (0x090AF0, i_090af0),
    (0x090AF6, i_090af6),
    (0x090AFC, i_090afc),
    (0x090B02, i_090b02),
    (0x090B08, i_090b08),
    (0x090B0C, i_090b0c),
    (0x090B12, i_090b12),
    (0x090B16, i_090b16),
    (0x090B18, i_090b18),
    (0x090B1C, i_090b1c),
    (0x090B20, i_090b20),
    (0x090B24, i_090b24),
    (0x090B2A, i_090b2a),
    (0x090B30, i_090b30),
    (0x090B34, i_090b34),
    (0x090B36, i_090b36),
    (0x090B38, i_090b38),
    (0x090B3C, i_090b3c),
    (0x090B3E, i_090b3e),
    (0x090B42, i_090b42),
    (0x090B44, i_090b44),
    (0x090B46, i_090b46),
    (0x090B48, i_090b48),
    (0x090B4A, i_090b4a),
    (0x090B4C, i_090b4c),
    (0x090B50, i_090b50),
    (0x090B54, i_090b54),
    (0x090B58, i_090b58),
    (0x090B5C, i_090b5c),
    (0x090B60, i_090b60),
    (0x090B64, i_090b64),
    (0x090B68, i_090b68),
    (0x090B6C, i_090b6c),
    (0x090B70, i_090b70),
    (0x090B74, i_090b74),
    (0x090B78, i_090b78),
    (0x090B7C, i_090b7c),
    (0x090B82, i_090b82),
    (0x090B86, i_090b86),
    (0x090B8C, i_090b8c),
    (0x090B90, i_090b90),
    (0x090B94, i_090b94),
    (0x090B98, i_090b98),
    (0x090B9E, i_090b9e),
    (0x090BA2, i_090ba2),
    (0x090BA6, i_090ba6),
    (0x090BA8, i_090ba8),
    (0x090BAA, i_090baa),
    (0x090BB0, i_090bb0),
    (0x090BB4, i_090bb4),
    (0x090BB8, i_090bb8),
    (0x090BBE, i_090bbe),
    (0x090BC2, i_090bc2),
    (0x090BC8, i_090bc8),
    (0x090BCC, i_090bcc),
    (0x090BD2, i_090bd2),
    (0x090BD6, i_090bd6),
    (0x090BDA, i_090bda),
    (0x090BDE, i_090bde),
    (0x090BE2, i_090be2),
    (0x090BE6, i_090be6),
    (0x090BEA, i_090bea),
    (0x090BF0, i_090bf0),
    (0x090BF6, i_090bf6),
    (0x090BFC, i_090bfc),
    (0x090C00, i_090c00),
    (0x090C04, i_090c04),
    (0x090C0A, i_090c0a),
    (0x090C10, i_090c10),
    (0x090C12, i_090c12),
    (0x090C18, i_090c18),
    (0x090C1E, i_090c1e),
    (0x090C24, i_090c24),
    (0x090C28, i_090c28),
    (0x090C2E, i_090c2e),
    (0x090C30, i_090c30),
    (0x090C34, i_090c34),
    (0x090C3A, i_090c3a),
    (0x090C40, i_090c40),
    (0x090C46, i_090c46),
    (0x090C4C, i_090c4c),
    (0x090C52, i_090c52),
    (0x090C58, i_090c58),
    (0x090C5E, i_090c5e),
    (0x090C60, i_090c60),
    (0x090C66, i_090c66),
    (0x090C6A, i_090c6a),
    (0x090C70, i_090c70),
    (0x090C76, i_090c76),
    (0x090C7C, i_090c7c),
    (0x090C82, i_090c82),
    (0x090C88, i_090c88),
    (0x090C8C, i_090c8c),
    (0x090C8E, i_090c8e),
    (0x090C94, i_090c94),
    (0x090C98, i_090c98),
    (0x090C9C, i_090c9c),
    (0x090CA2, i_090ca2),
    (0x090CA8, i_090ca8),
    (0x090CAC, i_090cac),
    (0x090CB2, i_090cb2),
    (0x090CB8, i_090cb8),
    (0x090CBA, i_090cba),
    (0x090CBE, i_090cbe),
    (0x090CC4, i_090cc4),
    (0x090CC8, i_090cc8),
    (0x090CCC, i_090ccc),
    (0x090CD0, i_090cd0),
    (0x090CD4, i_090cd4),
    (0x090CD8, i_090cd8),
    (0x090CDC, i_090cdc),
    (0x090CE0, i_090ce0),
    (0x090CE6, i_090ce6),
    (0x090CEC, i_090cec),
    (0x090CF2, i_090cf2),
    (0x090CF8, i_090cf8),
    (0x090CFE, i_090cfe),
    (0x090D00, i_090d00),
    (0x090D06, i_090d06),
    (0x090D0A, i_090d0a),
    (0x090D10, i_090d10),
    (0x090D16, i_090d16),
    (0x090D1C, i_090d1c),
    (0x090D22, i_090d22),
    (0x090D28, i_090d28),
    (0x090D2C, i_090d2c),
    (0x090D2E, i_090d2e),
    (0x090D34, i_090d34),
    (0x090D38, i_090d38),
    (0x090D3C, i_090d3c),
    (0x090D40, i_090d40),
    (0x090D44, i_090d44),
    (0x090D48, i_090d48),
    (0x090D4A, i_090d4a),
    (0x090D4C, i_090d4c),
    (0x090D50, i_090d50),
    (0x090D52, i_090d52),
    (0x090D56, i_090d56),
    (0x090D5A, i_090d5a),
    (0x090D5E, i_090d5e),
    (0x090D62, i_090d62),
    (0x090D66, i_090d66),
    (0x090D6A, i_090d6a),
    (0x090D6C, i_090d6c),
    (0x090D6E, i_090d6e),
    (0x090D72, i_090d72),
    (0x090D76, i_090d76),
    (0x090D7A, i_090d7a),
    (0x090D7E, i_090d7e),
    (0x090D82, i_090d82),
    (0x090D88, i_090d88),
    (0x090D8E, i_090d8e),
    (0x090D94, i_090d94),
    (0x090D9A, i_090d9a),
    (0x090D9E, i_090d9e),
    (0x090DA2, i_090da2),
    (0x090DA8, i_090da8),
    (0x090DAE, i_090dae),
    (0x090DB0, i_090db0),
    (0x090DB4, i_090db4),
    (0x090DBA, i_090dba),
    (0x090DBE, i_090dbe),
    (0x090DC2, i_090dc2),
    (0x090DC6, i_090dc6),
    (0x090DCC, i_090dcc),
    (0x090DD2, i_090dd2),
    (0x090DD8, i_090dd8),
    (0x090DDE, i_090dde),
    (0x090DE2, i_090de2),
    (0x090DE6, i_090de6),
    (0x090DEC, i_090dec),
    (0x090DF0, i_090df0),
    (0x090DF4, i_090df4),
    (0x090DF8, i_090df8),
    (0x090DFE, i_090dfe),
    (0x090E04, i_090e04),
    (0x090E0A, i_090e0a),
    (0x090E10, i_090e10),
    (0x090E16, i_090e16),
    (0x090E1C, i_090e1c),
    (0x090E20, i_090e20),
    (0x090E22, i_090e22),
    (0x090E26, i_090e26),
    (0x090E2A, i_090e2a),
    (0x090E2E, i_090e2e),
    (0x090E34, i_090e34),
    (0x090E3A, i_090e3a),
    (0x090E40, i_090e40),
    (0x090E44, i_090e44),
    (0x090E4A, i_090e4a),
    (0x090E4E, i_090e4e),
    (0x090E52, i_090e52),
    (0x090E56, i_090e56),
    (0x090E5C, i_090e5c),
    (0x090E60, i_090e60),
    (0x090E64, i_090e64),
    (0x090E66, i_090e66),
    (0x090E68, i_090e68),
    (0x090E6E, i_090e6e),
    (0x090E72, i_090e72),
    (0x090E76, i_090e76),
    (0x090E7A, i_090e7a),
    (0x090E7C, i_090e7c),
    (0x090E7E, i_090e7e),
    (0x090E86, i_090e86),
    (0x090E8A, i_090e8a),
    (0x090E90, i_090e90),
    (0x090E94, i_090e94),
    (0x090E98, i_090e98),
    (0x090E9C, i_090e9c),
    (0x090EA2, i_090ea2),
    (0x090EA8, i_090ea8),
    (0x090EAE, i_090eae),
    (0x090EB4, i_090eb4),
    (0x090EBA, i_090eba),
    (0x090EC0, i_090ec0),
    (0x090EC4, i_090ec4),
    (0x090EC6, i_090ec6),
    (0x090ECA, i_090eca),
    (0x090ECE, i_090ece),
    (0x090ED2, i_090ed2),
    (0x090ED8, i_090ed8),
    (0x090EDC, i_090edc),
    (0x090EDE, i_090ede),
    (0x090EE0, i_090ee0),
    (0x090EE4, i_090ee4),
    (0x090EE8, i_090ee8),
    (0x090EEC, i_090eec),
    (0x090EF0, i_090ef0),
    (0x090EF4, i_090ef4),
    (0x090EF8, i_090ef8),
    (0x090EFE, i_090efe),
    (0x090F00, i_090f00),
    (0x090F04, i_090f04),
    (0x090F0A, i_090f0a),
    (0x090F0E, i_090f0e),
    (0x090F12, i_090f12),
    (0x090F16, i_090f16),
    (0x090F1C, i_090f1c),
    (0x090F22, i_090f22),
    (0x090F28, i_090f28),
    (0x090F2E, i_090f2e),
    (0x090F34, i_090f34),
    (0x090F38, i_090f38),
    (0x090F3A, i_090f3a),
    (0x090F3E, i_090f3e),
    (0x090F42, i_090f42),
    (0x090F46, i_090f46),
    (0x090F4C, i_090f4c),
    (0x090F52, i_090f52),
    (0x090F58, i_090f58),
    (0x090F5E, i_090f5e),
    (0x090F62, i_090f62),
    (0x090F64, i_090f64),
    (0x090F68, i_090f68),
    (0x090F6C, i_090f6c),
    (0x090F72, i_090f72),
    (0x090F78, i_090f78),
    (0x090F7C, i_090f7c),
    (0x090F80, i_090f80),
    (0x090F86, i_090f86),
    (0x090F8C, i_090f8c),
    (0x090F92, i_090f92),
    (0x090F98, i_090f98),
    (0x090F9E, i_090f9e),
    (0x090FA2, i_090fa2),
    (0x090FA4, i_090fa4),
    (0x090FA8, i_090fa8),
    (0x090FAC, i_090fac),
    (0x090FB0, i_090fb0),
    (0x090FB6, i_090fb6),
    (0x090FBC, i_090fbc),
    (0x090FC2, i_090fc2),
    (0x090FC8, i_090fc8),
    (0x090FCC, i_090fcc),
    (0x090FCE, i_090fce),
    (0x090FD2, i_090fd2),
    (0x090FD6, i_090fd6),
    (0x090FDA, i_090fda),
    (0x090FE0, i_090fe0),
    (0x090FE6, i_090fe6),
    (0x090FEC, i_090fec),
    (0x090FF0, i_090ff0),
    (0x090FF4, i_090ff4),
    (0x090FF8, i_090ff8),
    (0x090FFC, i_090ffc),
    (0x091000, i_091000),
    (0x091004, i_091004),
    (0x091008, i_091008),
    (0x09100C, i_09100c),
    (0x091010, i_091010),
    (0x091012, i_091012),
    (0x091014, i_091014),
    (0x091018, i_091018),
    (0x09101A, i_09101a),
    (0x09101C, i_09101c),
    (0x091020, i_091020),
    (0x091026, i_091026),
    (0x09102A, i_09102a),
    (0x091030, i_091030),
    (0x091036, i_091036),
    (0x09103C, i_09103c),
    (0x091042, i_091042),
    (0x091046, i_091046),
    (0x09104A, i_09104a),
    (0x09104E, i_09104e),
    (0x091052, i_091052),
    (0x091058, i_091058),
    (0x09105C, i_09105c),
    (0x091062, i_091062),
    (0x091066, i_091066),
    (0x09106A, i_09106a),
    (0x09106E, i_09106e),
    (0x091074, i_091074),
    (0x091078, i_091078),
    (0x09107C, i_09107c),
    (0x09107E, i_09107e),
    (0x091080, i_091080),
    (0x091086, i_091086),
    (0x09108A, i_09108a),
    (0x09108E, i_09108e),
    (0x091092, i_091092),
    (0x091094, i_091094),
    (0x091096, i_091096),
    (0x09109A, i_09109a),
    (0x0910A0, i_0910a0),
    (0x0910A2, i_0910a2),
    (0x0910A6, i_0910a6),
    (0x0910AC, i_0910ac),
    (0x0910B0, i_0910b0),
    (0x0910B4, i_0910b4),
    (0x0910B6, i_0910b6),
    (0x0910BA, i_0910ba),
    (0x0910D6, i_0910d6),
    (0x0910DA, i_0910da),
    (0x0910E0, i_0910e0),
    (0x0910E4, i_0910e4),
    (0x0910E8, i_0910e8),
    (0x0910EA, i_0910ea),
    (0x0910EE, i_0910ee),
    (0x0910F0, i_0910f0),
    (0x0910F4, i_0910f4),
    (0x0910FA, i_0910fa),
    (0x0910FE, i_0910fe),
    (0x091102, i_091102),
    (0x091104, i_091104),
    (0x091108, i_091108),
    (0x09110A, i_09110a),
    (0x09110E, i_09110e),
    (0x091114, i_091114),
    (0x091118, i_091118),
    (0x09111C, i_09111c),
    (0x09111E, i_09111e),
    (0x091122, i_091122),
    (0x091124, i_091124),
    (0x091128, i_091128),
    (0x09112E, i_09112e),
    (0x091130, i_091130),
    (0x091136, i_091136),
    (0x09113A, i_09113a),
    (0x09113E, i_09113e),
    (0x091142, i_091142),
    (0x091148, i_091148),
    (0x09114E, i_09114e),
    (0x091154, i_091154),
    (0x09115A, i_09115a),
    (0x09115E, i_09115e),
    (0x091160, i_091160),
    (0x091164, i_091164),
    (0x09116A, i_09116a),
    (0x091170, i_091170),
    (0x091176, i_091176),
    (0x09117C, i_09117c),
    (0x091182, i_091182),
    (0x091186, i_091186),
    (0x09118A, i_09118a),
    (0x09118E, i_09118e),
    (0x091194, i_091194),
    (0x091198, i_091198),
    (0x09119C, i_09119c),
    (0x0911A0, i_0911a0),
    (0x0911A6, i_0911a6),
    (0x0911AC, i_0911ac),
    (0x0911B2, i_0911b2),
    (0x0911B8, i_0911b8),
    (0x0911BC, i_0911bc),
    (0x0911C0, i_0911c0),
    (0x0911C6, i_0911c6),
    (0x0911CC, i_0911cc),
    (0x0911D0, i_0911d0),
    (0x0911D6, i_0911d6),
    (0x0911DC, i_0911dc),
    (0x0911E0, i_0911e0),
    (0x0911E2, i_0911e2),
    (0x0911E6, i_0911e6),
    (0x0911EA, i_0911ea),
    (0x0911F0, i_0911f0),
    (0x0911F6, i_0911f6),
    (0x0911FC, i_0911fc),
    (0x091202, i_091202),
    (0x091206, i_091206),
    (0x091208, i_091208),
    (0x09120C, i_09120c),
    (0x091212, i_091212),
    (0x091218, i_091218),
    (0x09121E, i_09121e),
    (0x091224, i_091224),
    (0x09122A, i_09122a),
    (0x09122E, i_09122e),
    (0x091232, i_091232),
    (0x091236, i_091236),
    (0x09123C, i_09123c),
    (0x091240, i_091240),
    (0x091244, i_091244),
    (0x091248, i_091248),
    (0x09124E, i_09124e),
    (0x091254, i_091254),
    (0x09125A, i_09125a),
    (0x091260, i_091260),
    (0x091264, i_091264),
    (0x091268, i_091268),
    (0x09126E, i_09126e),
    (0x091274, i_091274),
    (0x091278, i_091278),
    (0x09127E, i_09127e),
    (0x091284, i_091284),
    (0x091288, i_091288),
    (0x09128A, i_09128a),
    (0x09128E, i_09128e),
    (0x091292, i_091292),
    (0x091298, i_091298),
    (0x09129C, i_09129c),
    (0x0912A0, i_0912a0),
    (0x0912A2, i_0912a2),
    (0x0912A4, i_0912a4),
    (0x0912A8, i_0912a8),
    (0x0912AA, i_0912aa),
    (0x0912AC, i_0912ac),
    (0x0912B0, i_0912b0),
    (0x0912B4, i_0912b4),
    (0x0912B6, i_0912b6),
    (0x0912B8, i_0912b8),
    (0x0912BC, i_0912bc),
    (0x0912BE, i_0912be),
    (0x0912C0, i_0912c0),
    (0x0912C4, i_0912c4),
    (0x0912C8, i_0912c8),
    (0x0912CC, i_0912cc),
    (0x0912D0, i_0912d0),
    (0x0912D2, i_0912d2),
    (0x0912D4, i_0912d4),
    (0x0912D8, i_0912d8),
    (0x0912DA, i_0912da),
    (0x0912DC, i_0912dc),
    (0x0912E0, i_0912e0),
    (0x0912E4, i_0912e4),
    (0x0912E8, i_0912e8),
    (0x0912EC, i_0912ec),
    (0x0912EE, i_0912ee),
    (0x0912F0, i_0912f0),
    (0x0912F4, i_0912f4),
    (0x0912F6, i_0912f6),
    (0x0912F8, i_0912f8),
    (0x0912FC, i_0912fc),
    (0x091302, i_091302),
    (0x091306, i_091306),
    (0x09130C, i_09130c),
    (0x091310, i_091310),
    (0x091316, i_091316),
    (0x09131A, i_09131a),
    (0x09131E, i_09131e),
    (0x091324, i_091324),
    (0x091328, i_091328),
    (0x09132E, i_09132e),
    (0x091330, i_091330),
    (0x091334, i_091334),
    (0x091336, i_091336),
    (0x091338, i_091338),
    (0x09133E, i_09133e),
    (0x091344, i_091344),
    (0x091348, i_091348),
    (0x09134C, i_09134c),
    (0x091350, i_091350),
    (0x091356, i_091356),
    (0x09135A, i_09135a),
    (0x091360, i_091360),
    (0x091364, i_091364),
    (0x09136A, i_09136a),
    (0x091370, i_091370),
    (0x091374, i_091374),
    (0x09137A, i_09137a),
    (0x091380, i_091380),
    (0x091384, i_091384),
    (0x09138A, i_09138a),
    (0x091390, i_091390),
    (0x091394, i_091394),
    (0x09139A, i_09139a),
    (0x0913A0, i_0913a0),
    (0x0913A4, i_0913a4),
    (0x0913A8, i_0913a8),
    (0x0913AA, i_0913aa),
    (0x091408, i_091408),
    (0x09140C, i_09140c),
    (0x091412, i_091412),
    (0x091418, i_091418),
    (0x09141E, i_09141e),
    (0x091422, i_091422),
    (0x091428, i_091428),
    (0x09142E, i_09142e),
    (0x091434, i_091434),
    (0x09143A, i_09143a),
    (0x091440, i_091440),
    (0x091444, i_091444),
    (0x091446, i_091446),
    (0x09144A, i_09144a),
    (0x09144E, i_09144e),
    (0x091454, i_091454),
    (0x09145A, i_09145a),
    (0x091460, i_091460),
    (0x091464, i_091464),
    (0x09146A, i_09146a),
    (0x091470, i_091470),
    (0x091476, i_091476),
    (0x09147C, i_09147c),
    (0x091482, i_091482),
    (0x091486, i_091486),
    (0x091488, i_091488),
    (0x09148C, i_09148c),
    (0x091490, i_091490),
    (0x091496, i_091496),
    (0x09149C, i_09149c),
    (0x0914A2, i_0914a2),
    (0x0914A6, i_0914a6),
    (0x0914AC, i_0914ac),
    (0x0914B2, i_0914b2),
    (0x0914B8, i_0914b8),
    (0x0914BE, i_0914be),
    (0x0914C4, i_0914c4),
    (0x0914C8, i_0914c8),
    (0x0914CA, i_0914ca),
    (0x0914CE, i_0914ce),
    (0x0914D2, i_0914d2),
    (0x0914D8, i_0914d8),
    (0x0914DE, i_0914de),
    (0x0914E4, i_0914e4),
    (0x0914E8, i_0914e8),
    (0x0914EE, i_0914ee),
    (0x0914F4, i_0914f4),
    (0x0914FA, i_0914fa),
    (0x091500, i_091500),
    (0x091506, i_091506),
    (0x09150A, i_09150a),
    (0x09150C, i_09150c),
    (0x091512, i_091512),
    (0x091514, i_091514),
    (0x09151A, i_09151a),
    (0x091520, i_091520),
    (0x091526, i_091526),
    (0x09152A, i_09152a),
    (0x09152C, i_09152c),
    (0x091532, i_091532),
    (0x091536, i_091536),
    (0x09153A, i_09153a),
    (0x09153C, i_09153c),
    (0x091542, i_091542),
    (0x091546, i_091546),
    (0x09154A, i_09154a),
    (0x091550, i_091550),
    (0x091554, i_091554),
    (0x091556, i_091556),
    (0x091558, i_091558),
    (0x09155E, i_09155e),
    (0x091562, i_091562),
    (0x091566, i_091566),
    (0x09156A, i_09156a),
    (0x091570, i_091570),
    (0x091574, i_091574),
    (0x091576, i_091576),
    (0x091578, i_091578),
    (0x09157E, i_09157e),
    (0x091584, i_091584),
    (0x091588, i_091588),
    (0x09158E, i_09158e),
    (0x091592, i_091592),
    (0x091594, i_091594),
    (0x09159A, i_09159a),
    (0x09159E, i_09159e),
    (0x0915A4, i_0915a4),
    (0x0915A6, i_0915a6),
    (0x0915A8, i_0915a8),
    (0x0915AC, i_0915ac),
    (0x0915AE, i_0915ae),
    (0x0915B0, i_0915b0),
    (0x0915B2, i_0915b2),
    (0x0915B4, i_0915b4),
    (0x0915BA, i_0915ba),
    (0x0915BC, i_0915bc),
    (0x0915C2, i_0915c2),
    (0x0915C6, i_0915c6),
    (0x0915CA, i_0915ca),
    (0x0915CE, i_0915ce),
    (0x0915D0, i_0915d0),
    (0x0915D2, i_0915d2),
    (0x0915D4, i_0915d4),
    (0x0915DA, i_0915da),
    (0x0915DC, i_0915dc),
    (0x0915E2, i_0915e2),
    (0x0915E4, i_0915e4),
    (0x0915E6, i_0915e6),
    (0x0915EA, i_0915ea),
    (0x0915F0, i_0915f0),
    (0x0915F2, i_0915f2),
    (0x0915F6, i_0915f6),
    (0x0915FA, i_0915fa),
    (0x0915FE, i_0915fe),
    (0x091600, i_091600),
    (0x091602, i_091602),
    (0x091604, i_091604),
    (0x09160A, i_09160a),
    (0x09160C, i_09160c),
    (0x091612, i_091612),
    (0x091614, i_091614),
    (0x091616, i_091616),
    (0x091618, i_091618),
    (0x09161E, i_09161e),
    (0x091624, i_091624),
    (0x091628, i_091628),
    (0x09162E, i_09162e),
    (0x09196E, i_09196e),
    (0x091976, i_091976),
    (0x091978, i_091978),
    (0x09197C, i_09197c),
    (0x0919B0, i_0919b0),
    (0x0919B8, i_0919b8),
    (0x0919BA, i_0919ba),
    (0x0919BE, i_0919be),
    (0x0919C8, i_0919c8),
    (0x0919D0, i_0919d0),
    (0x0919D2, i_0919d2),
    (0x0919D6, i_0919d6),
    (0x091A0A, i_091a0a),
    (0x091A12, i_091a12),
    (0x091A14, i_091a14),
    (0x091A18, i_091a18),
    (0x091A4E, i_091a4e),
    (0x091A56, i_091a56),
    (0x091A58, i_091a58),
    (0x091A5C, i_091a5c),
    (0x091A60, i_091a60),
    (0x091A68, i_091a68),
    (0x091A6A, i_091a6a),
    (0x091A6E, i_091a6e),
    (0x0967C0, i_0967c0),
    (0x0967C6, i_0967c6),
    (0x0967CC, i_0967cc),
    (0x0967CE, i_0967ce),
    (0x0967D0, i_0967d0),
    (0x0967D6, i_0967d6),
    (0x0967DC, i_0967dc),
    (0x0967DE, i_0967de),
    (0x0967E2, i_0967e2),
    (0x0967E4, i_0967e4),
    (0x0967E6, i_0967e6),
    (0x0967EA, i_0967ea),
    (0x0967EC, i_0967ec),
    (0x0967EE, i_0967ee),
    (0x0967F2, i_0967f2),
    (0x0967F4, i_0967f4),
    (0x0967F6, i_0967f6),
    (0x0967FA, i_0967fa),
    (0x0967FC, i_0967fc),
    (0x0967FE, i_0967fe),
    (0x096802, i_096802),
    (0x096806, i_096806),
    (0x09680C, i_09680c),
    (0x096814, i_096814),
    (0x09681A, i_09681a),
    (0x09681E, i_09681e),
    (0x096820, i_096820),
    (0x096824, i_096824),
    (0x09682A, i_09682a),
    (0x09682E, i_09682e),
    (0x096832, i_096832),
    (0x096836, i_096836),
    (0x09683E, i_09683e),
    (0x0969C2, i_0969c2),
    (0x0969C6, i_0969c6),
    (0x0969CA, i_0969ca),
    (0x0969CC, i_0969cc),
    (0x0969D0, i_0969d0),
    (0x0969D6, i_0969d6),
    (0x0969DA, i_0969da),
    (0x0969DE, i_0969de),
    (0x0969E4, i_0969e4),
    (0x0969E6, i_0969e6),
    (0x0969EA, i_0969ea),
    (0x0969EC, i_0969ec),
    (0x0969F0, i_0969f0),
    (0x0969F4, i_0969f4),
    (0x0969F6, i_0969f6),
    (0x0969FC, i_0969fc),
    (0x0969FE, i_0969fe),
    (0x096A04, i_096a04),
    (0x096A0A, i_096a0a),
    (0x096A0C, i_096a0c),
    (0x096A5A, i_096a5a),
    (0x096A60, i_096a60),
    (0x096A62, i_096a62),
    (0x096A64, i_096a64),
    (0x096A66, i_096a66),
    (0x096A68, i_096a68),
    (0x096A6E, i_096a6e),
    (0x096A70, i_096a70),
    (0x096A72, i_096a72),
    (0x096A74, i_096a74),
    (0x096A78, i_096a78),
    (0x096A7A, i_096a7a),
    (0x096A7C, i_096a7c),
    (0x096A7E, i_096a7e),
    (0x096A80, i_096a80),
    (0x096A88, i_096a88),
    (0x096A8C, i_096a8c),
    (0x096A92, i_096a92),
    (0x096A98, i_096a98),
    (0x096A9E, i_096a9e),
    (0x096AA2, i_096aa2),
    (0x096AA8, i_096aa8),
    (0x096AAE, i_096aae),
    (0x096AB2, i_096ab2),
    (0x096AB8, i_096ab8),
    (0x096ABA, i_096aba),
    (0x096AC0, i_096ac0),
    (0x096AC4, i_096ac4),
    (0x096ACA, i_096aca),
    (0x096ACC, i_096acc),
    (0x096AD2, i_096ad2),
    (0x096AD6, i_096ad6),
    (0x096ADC, i_096adc),
    (0x096ADE, i_096ade),
    (0x096AE4, i_096ae4),
    (0x096AEA, i_096aea),
    (0x096AF0, i_096af0),
    (0x096AF4, i_096af4),
    (0x096AFA, i_096afa),
    (0x096AFC, i_096afc),
    (0x096B02, i_096b02),
    (0x096B08, i_096b08),
    (0x096B0E, i_096b0e),
    (0x096B12, i_096b12),
    (0x096B18, i_096b18),
    (0x096B1C, i_096b1c),
    (0x096B22, i_096b22),
    (0x096B24, i_096b24),
    (0x096B2C, i_096b2c),
    (0x096B30, i_096b30),
    (0x096B36, i_096b36),
    (0x096B3A, i_096b3a),
    (0x096B40, i_096b40),
    (0x096B44, i_096b44),
    (0x096B4A, i_096b4a),
    (0x096B50, i_096b50),
    (0x096B58, i_096b58),
    (0x096B5C, i_096b5c),
    (0x096B62, i_096b62),
    (0x096B66, i_096b66),
    (0x096B6C, i_096b6c),
    (0x096B70, i_096b70),
    (0x096B76, i_096b76),
    (0x096B7C, i_096b7c),
    (0x096B7E, i_096b7e),
    (0x096B82, i_096b82),
    (0x096B86, i_096b86),
    (0x096B8A, i_096b8a),
    (0x096B90, i_096b90),
    (0x096B92, i_096b92),
    (0x096B96, i_096b96),
    (0x096B9A, i_096b9a),
    (0x09773C, i_09773c),
    (0x097742, i_097742),
    (0x097748, i_097748),
    (0x09774A, i_09774a),
    (0x09774C, i_09774c),
    (0x09774E, i_09774e),
    (0x097750, i_097750),
    (0x097752, i_097752),
    (0x097754, i_097754),
    (0x097756, i_097756),
    (0x097758, i_097758),
    (0x09775A, i_09775a),
    (0x09775C, i_09775c),
    (0x09775E, i_09775e),
    (0x097760, i_097760),
    (0x097762, i_097762),
    (0x097764, i_097764),
    (0x097766, i_097766),
    (0x097768, i_097768),
    (0x09776A, i_09776a),
    (0x09776C, i_09776c),
    (0x09776E, i_09776e),
    (0x097770, i_097770),
    (0x097772, i_097772),
    (0x097774, i_097774),
    (0x097776, i_097776),
    (0x097778, i_097778),
    (0x09777A, i_09777a),
    (0x09777C, i_09777c),
    (0x09777E, i_09777e),
    (0x097780, i_097780),
    (0x097782, i_097782),
    (0x097784, i_097784),
    (0x097786, i_097786),
    (0x097788, i_097788),
    (0x09778A, i_09778a),
    (0x09778C, i_09778c),
    (0x09778E, i_09778e),
    (0x097790, i_097790),
    (0x097792, i_097792),
    (0x097794, i_097794),
    (0x097796, i_097796),
    (0x097798, i_097798),
    (0x09779A, i_09779a),
    (0x09779C, i_09779c),
    (0x09779E, i_09779e),
    (0x0977A0, i_0977a0),
    (0x0977A2, i_0977a2),
    (0x0977A4, i_0977a4),
    (0x0977A6, i_0977a6),
    (0x0977A8, i_0977a8),
    (0x0977AA, i_0977aa),
    (0x0977AC, i_0977ac),
    (0x0977AE, i_0977ae),
    (0x0977B0, i_0977b0),
    (0x0977B2, i_0977b2),
    (0x0977B4, i_0977b4),
    (0x0977B6, i_0977b6),
    (0x0977B8, i_0977b8),
    (0x0977BA, i_0977ba),
    (0x0977BC, i_0977bc),
    (0x0977BE, i_0977be),
    (0x0977C0, i_0977c0),
    (0x0977EA, i_0977ea),
    (0x0977EE, i_0977ee),
    (0x0977F4, i_0977f4),
    (0x0977FA, i_0977fa),
    (0x097800, i_097800),
    (0x097802, i_097802),
    (0x097804, i_097804),
    (0x097806, i_097806),
    (0x097808, i_097808),
    (0x09780A, i_09780a),
    (0x09780C, i_09780c),
    (0x09780E, i_09780e),
    (0x097810, i_097810),
    (0x097812, i_097812),
    (0x097814, i_097814),
    (0x097816, i_097816),
    (0x09781A, i_09781a),
    (0x097820, i_097820),
    (0x097822, i_097822),
    (0x097824, i_097824),
    (0x09782A, i_09782a),
    (0x09782E, i_09782e),
    (0x097834, i_097834),
    (0x09783A, i_09783a),
    (0x097840, i_097840),
    (0x097844, i_097844),
    (0x09784A, i_09784a),
    (0x09784E, i_09784e),
    (0x097850, i_097850),
    (0x097852, i_097852),
    (0x097856, i_097856),
    (0x09785A, i_09785a),
    (0x09785C, i_09785c),
    (0x097860, i_097860),
    (0x097866, i_097866),
    (0x09786A, i_09786a),
    (0x097872, i_097872),
    (0x097878, i_097878),
    (0x09787C, i_09787c),
    (0x097880, i_097880),
    (0x097882, i_097882),
    (0x097884, i_097884),
    (0x097888, i_097888),
    (0x09788A, i_09788a),
    (0x09794C, i_09794c),
    (0x097952, i_097952),
    (0x097958, i_097958),
    (0x09795E, i_09795e),
    (0x097974, i_097974),
    (0x097978, i_097978),
    (0x09797E, i_09797e),
    (0x097984, i_097984),
    (0x09798A, i_09798a),
    (0x097990, i_097990),
    (0x097996, i_097996),
    (0x09799A, i_09799a),
    (0x0979A0, i_0979a0),
    (0x0979A2, i_0979a2),
    (0x0979A6, i_0979a6),
    (0x0979AC, i_0979ac),
    (0x0979B0, i_0979b0),
    (0x0979B6, i_0979b6),
    (0x0979BA, i_0979ba),
    (0x0979BC, i_0979bc),
    (0x0979C0, i_0979c0),
    (0x0979C4, i_0979c4),
    (0x0979CA, i_0979ca),
    (0x0979D0, i_0979d0),
    (0x0979D6, i_0979d6),
    (0x0979DA, i_0979da),
    (0x0979E0, i_0979e0),
    (0x0979E6, i_0979e6),
    (0x0979EA, i_0979ea),
    (0x0979F0, i_0979f0),
    (0x0979F6, i_0979f6),
    (0x097A48, i_097a48),
    (0x097A4E, i_097a4e),
    (0x097A52, i_097a52),
    (0x097A60, i_097a60),
    (0x097A66, i_097a66),
    (0x097A6C, i_097a6c),
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
