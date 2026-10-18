// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Grunt_PickRand_2f3712_08dec2  ($08DEC2, 22 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DEC2: move.w #$3, d0
pub fn i_08dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DEC6: jsr $5ea1c.l
pub fn i_08dec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DECC);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DECC: lea.l $2f3712.l, a0
pub fn i_08decc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DED2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3712;
    20
}
/// $08DED2: movea.l #$ffffffff, a1
pub fn i_08ded2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DED8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ded8  ($08DED8, 8 B, thunks/jsr_abs_thunks.c)
/// $08DED8: jsr $772.l
pub fn i_08ded8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DEDE);
    cpu.pc = 0x772;
    16
}
/// $08DEDE: rts
pub fn i_08dede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Runner_08dee0  ($08DEE0, 98 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DEE0: move.w #$60, $22(a6)
pub fn i_08dee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DEE6: move.w #$150, $24(a6)
pub fn i_08dee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DEEC: clr.w $26(a6)
pub fn i_08deec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEF0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DEF0: move.b #$10, $99(a6)
pub fn i_08def0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DEF6: move.b #$1, $98(a6)
pub fn i_08def6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEFC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DEFC: cmpi.b #$0, $99(a6)
pub fn i_08defc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF02;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08DF02: bne.w $8df0c
pub fn i_08df02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8DF0C; return 10; }
    8
}
/// $08DF06: move.b #$10, $99(a6)
pub fn i_08df06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DF0C: jsr $8f108(pc)
pub fn i_08df0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF10);
    cpu.pc = 0x8F108;
    16
}
/// $08DF10: jsr $8f002(pc)
pub fn i_08df10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF14);
    cpu.pc = 0x8F002;
    16
}
/// $08DF14: jsr $8f010(pc)
pub fn i_08df14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF18);
    cpu.pc = 0x8F010;
    16
}
/// $08DF18: move.w #$f9, d1
pub fn i_08df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF1C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DF1C: jsr $236e.l
pub fn i_08df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF22);
    cpu.pc = 0x236E;
    16
}
/// $08DF22: lea.l $2f3cc0.l, a0
pub fn i_08df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3CC0;
    20
}
/// $08DF28: jsr $28cd4.l
pub fn i_08df28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF2E);
    cpu.pc = 0x28CD4;
    16
}
/// $08DF2E: lea.l $8df34(pc), a1
pub fn i_08df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF34;
    16
}
/// $08DF32: move.l a1, (a6)
pub fn i_08df32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DF34: jsr $2783a.l
pub fn i_08df34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF3A);
    cpu.pc = 0x2783A;
    16
}
/// $08DF3A: jsr $8d2b0(pc)
pub fn i_08df3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF3E);
    cpu.pc = 0x8D2B0;
    16
}
/// $08DF3E: jsr $8efb0(pc)
pub fn i_08df3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF42);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08df42  ($08DF42, 8 B, thunks/jsr_abs_thunks.c)
/// $08DF42: jsr $28d70.l
pub fn i_08df42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF48);
    cpu.pc = 0x28D70;
    16
}
/// $08DF48: rts
pub fn i_08df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl153_08df4a  ($08DF4A, 32 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF4A: jsr $8f108(pc)
pub fn i_08df4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF4E);
    cpu.pc = 0x8F108;
    16
}
/// $08DF4E: jsr $8f002(pc)
pub fn i_08df4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF52);
    cpu.pc = 0x8F002;
    16
}
/// $08DF52: jsr $8f010(pc)
pub fn i_08df52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF56);
    cpu.pc = 0x8F010;
    16
}
/// $08DF56: jsr $8f070(pc)
pub fn i_08df56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF5A);
    cpu.pc = 0x8F070;
    16
}
/// $08DF5A: jsr $8e172(pc)
pub fn i_08df5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF5E);
    cpu.pc = 0x8E172;
    16
}
/// $08DF5E: jsr $27cee.l
pub fn i_08df5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF64);
    cpu.pc = 0x27CEE;
    16
}
/// $08DF64: jsr $28d70.l
pub fn i_08df64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF6A);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_08df6a  ($08DF6A, 8 B, kernel/task_handlers.c)
/// $08DF6A: lea.l $8df72(pc), a1
pub fn i_08df6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF72;
    16
}
/// $08DF6E: move.l a1, (a6)
pub fn i_08df6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DF70: rts
pub fn i_08df70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Run_08df72  ($08DF72, 24 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF72: jsr $8d2b0(pc)
pub fn i_08df72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF76);
    cpu.pc = 0x8D2B0;
    16
}
/// $08DF76: jsr $2783a.l
pub fn i_08df76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF7C);
    cpu.pc = 0x2783A;
    16
}
/// $08DF7C: jsr $8d184(pc)
pub fn i_08df7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF80);
    cpu.pc = 0x8D184;
    16
}
/// $08DF80: bcc.w $8df8c
pub fn i_08df80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DF8C; return 10; }
    8
}
/// $08DF84: jmp $518.l
pub fn i_08df84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF8A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08df8a  ($08DF8A, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF8A: rts
pub fn i_08df8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08df8c  ($08DF8C, 8 B, thunks/jsr_abs_thunks.c)
/// $08DF8C: jsr $28d70.l
pub fn i_08df8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF92);
    cpu.pc = 0x28D70;
    16
}
/// $08DF92: rts
pub fn i_08df92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl154_08df94  ($08DF94, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF94: jsr $8f108(pc)
pub fn i_08df94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF98);
    cpu.pc = 0x8F108;
    16
}
/// $08DF98: jsr $8f002(pc)
pub fn i_08df98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF9C);
    cpu.pc = 0x8F002;
    16
}
/// $08DF9C: jsr $8f010(pc)
pub fn i_08df9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DFA0);
    cpu.pc = 0x8F010;
    16
}
/// $08DFA0: jsr $8f070(pc)
pub fn i_08dfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DFA4);
    cpu.pc = 0x8F070;
    16
}
/// $08DFA4: jsr $27cee.l
pub fn i_08dfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFAA);
    cpu.pc = 0x27CEE;
    16
}
/// $08DFAA: jsr $28d70.l
pub fn i_08dfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFB0);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_08dfb0  ($08DFB0, 8 B, kernel/task_handlers.c)
/// $08DFB0: lea.l $8dfb8(pc), a1
pub fn i_08dfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DFB8;
    16
}
/// $08DFB4: move.l a1, (a6)
pub fn i_08dfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DFB6: rts
pub fn i_08dfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Rand_Stand_08dfb8  ($08DFB8, 114 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DFB8: lea.l $2f3e4a.l, a0
pub fn i_08dfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E4A;
    20
}
/// $08DFBE: jsr $28cd4.l
pub fn i_08dfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFC4);
    cpu.pc = 0x28CD4;
    16
}
/// $08DFC4: lea.l $8dfca(pc), a1
pub fn i_08dfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DFCA;
    16
}
/// $08DFC8: move.l a1, (a6)
pub fn i_08dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DFCA: jsr $2783a.l
pub fn i_08dfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFD0);
    cpu.pc = 0x2783A;
    16
}
/// $08DFD0: jsr $28d70.l
pub fn i_08dfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFD6);
    cpu.pc = 0x28D70;
    16
}
/// $08DFD6: bcc.w $8e01a
pub fn i_08dfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E01A; return 10; }
    8
}
/// $08DFDA: move.w #$60, d0
pub fn i_08dfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DFDE: jsr $8efce(pc)
pub fn i_08dfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DFE2);
    cpu.pc = 0x8EFCE;
    16
}
/// $08DFE2: bcc.w $8e016
pub fn i_08dfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E016; return 10; }
    8
}
/// $08DFE6: move.w #$f, d0
pub fn i_08dfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DFEA: jsr $5ea1c.l
pub fn i_08dfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFF0);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DFF0: move.b #$20, $99(a6)
pub fn i_08dff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DFF6: add.b d0, $99(a6)
pub fn i_08dff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFFA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08DFFA: move.b #$1, $98(a6)
pub fn i_08dffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E000;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E000: jsr $8f002(pc)
pub fn i_08e000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E004;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E004);
    cpu.pc = 0x8F002;
    16
}
/// $08E004: jsr $8f010(pc)
pub fn i_08e004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E008;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E008);
    cpu.pc = 0x8F010;
    16
}
/// $08E008: jsr $8e172(pc)
pub fn i_08e008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E00C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E00C);
    cpu.pc = 0x8E172;
    16
}
/// $08E00C: lea.l $8df72(pc), a1
pub fn i_08e00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF72;
    16
}
/// $08E010: move.l a1, (a6)
pub fn i_08e010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E012: bra.w $8e01a
pub fn i_08e012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E016;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E01A;
    10
}
/// $08E016: jsr $8e09c(pc)
pub fn i_08e016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E01A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E01A);
    cpu.pc = 0x8E09C;
    16
}
/// $08E01A: jsr $8d184(pc)
pub fn i_08e01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E01E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E01E);
    cpu.pc = 0x8D184;
    16
}
/// $08E01E: bcc.w $8e028
pub fn i_08e01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E022;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E028; return 10; }
    8
}
/// $08E022: jmp $518.l
pub fn i_08e022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E028;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E028: rts
pub fn i_08e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E02A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Rand_Stand2_08e02a  ($08E02A, 54 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E02A: lea.l $2f3e56.l, a0
pub fn i_08e02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E030;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E56;
    20
}
/// $08E030: jsr $28cd4.l
pub fn i_08e030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E036);
    cpu.pc = 0x28CD4;
    16
}
/// $08E036: lea.l $8e03c(pc), a1
pub fn i_08e036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E03A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E03C;
    16
}
/// $08E03A: move.l a1, (a6)
pub fn i_08e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E03C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E03C: jsr $2783a.l
pub fn i_08e03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E042);
    cpu.pc = 0x2783A;
    16
}
/// $08E042: jsr $28d70.l
pub fn i_08e042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E048);
    cpu.pc = 0x28D70;
    16
}
/// $08E048: bcc.w $8e050
pub fn i_08e048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E04C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E050; return 10; }
    8
}
/// $08E04C: jsr $8e09c(pc)
pub fn i_08e04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E050;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E050);
    cpu.pc = 0x8E09C;
    16
}
/// $08E050: jsr $8d184(pc)
pub fn i_08e050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E054;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E054);
    cpu.pc = 0x8D184;
    16
}
/// $08E054: bcc.w $8e05e
pub fn i_08e054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E058;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E05E; return 10; }
    8
}
/// $08E058: jmp $518.l
pub fn i_08e058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E05E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E05E: rts
pub fn i_08e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E060;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Rand_Walk_08e060  ($08E060, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E060: lea.l $2f3e0c.l, a0
pub fn i_08e060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E066;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E0C;
    20
}
/// $08E066: jsr $28cd4.l
pub fn i_08e066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E06C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E06C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E06C: lea.l $8e072(pc), a1
pub fn i_08e06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E070;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E072;
    16
}
/// $08E070: move.l a1, (a6)
pub fn i_08e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E072;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E072: jsr $2783a.l
pub fn i_08e072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E078);
    cpu.pc = 0x2783A;
    16
}
/// $08E078: jsr $8d2b0(pc)
pub fn i_08e078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E07C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E07C);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E07C: jsr $28d70.l
pub fn i_08e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E082);
    cpu.pc = 0x28D70;
    16
}
/// $08E082: bcc.w $8e08c
pub fn i_08e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E086;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E08C; return 10; }
    8
}
/// $08E086: lea.l $8dfb8(pc), a1
pub fn i_08e086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E08A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DFB8;
    16
}
/// $08E08A: move.l a1, (a6)
pub fn i_08e08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E08C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E08C: jsr $8d184(pc)
pub fn i_08e08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E090;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E090);
    cpu.pc = 0x8D184;
    16
}
/// $08E090: bcc.w $8e09a
pub fn i_08e090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E094;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E09A; return 10; }
    8
}
/// $08E094: jmp $518.l
pub fn i_08e094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E09A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E09A: rts
pub fn i_08e09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E09C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_PickRand_2f3722_08e09c  ($08E09C, 22 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E09C: move.w #$3, d0
pub fn i_08e09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E0A0: jsr $5ea1c.l
pub fn i_08e0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0A6);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E0A6: lea.l $2f3722.l, a0
pub fn i_08e0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3722;
    20
}
/// $08E0AC: movea.l #$ffffffff, a1
pub fn i_08e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0B2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08e0b2  ($08E0B2, 8 B, thunks/jsr_abs_thunks.c)
/// $08E0B2: jsr $772.l
pub fn i_08e0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0B8);
    cpu.pc = 0x772;
    16
}
/// $08E0B8: rts
pub fn i_08e0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Spawn_08e0ba  ($08E0BA, 74 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E0BA: move.w #$80, $22(a6)
pub fn i_08e0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E0C0: move.w #$150, $24(a6)
pub fn i_08e0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E0C6: clr.w $26(a6)
pub fn i_08e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0CA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E0CA: move.b #$1, $98(a6)
pub fn i_08e0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E0D0: move.b #$2, $9a(a6)
pub fn i_08e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E0D6: move.b #$8, $99(a6)
pub fn i_08e0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E0DC: jsr $8f108(pc)
pub fn i_08e0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0E0);
    cpu.pc = 0x8F108;
    16
}
/// $08E0E0: jsr $8f002(pc)
pub fn i_08e0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0E4);
    cpu.pc = 0x8F002;
    16
}
/// $08E0E4: jsr $8f010(pc)
pub fn i_08e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0E8);
    cpu.pc = 0x8F010;
    16
}
/// $08E0E8: jsr $8f070(pc)
pub fn i_08e0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0EC);
    cpu.pc = 0x8F070;
    16
}
/// $08E0EC: jsr $27cee.l
pub fn i_08e0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0F2);
    cpu.pc = 0x27CEE;
    16
}
/// $08E0F2: jsr $28d70.l
pub fn i_08e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0F8);
    cpu.pc = 0x28D70;
    16
}
/// $08E0F8: lea.l $8e10c(pc), a1
pub fn i_08e0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E10C;
    16
}
/// $08E0FC: move.l a1, (a6)
pub fn i_08e0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E0FE: lea.l $2f3e0c.l, a0
pub fn i_08e0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E104;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E0C;
    20
}
// ===== JsrAbsThunk_08e104  ($08E104, 8 B, thunks/jsr_abs_thunks.c)
/// $08E104: jsr $28cd4.l
pub fn i_08e104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E10A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E10A);
    cpu.pc = 0x28CD4;
    16
}
/// $08E10A: rts
pub fn i_08e10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E10C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Run_08e10c  ($08E10C, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E10C: jsr $8d2b0(pc)
pub fn i_08e10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E110;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E110);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E110: jsr $28d70.l
pub fn i_08e110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E116);
    cpu.pc = 0x28D70;
    16
}
/// $08E116: bcc.w $8e162
pub fn i_08e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E11A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E162; return 10; }
    8
}
/// $08E11A: move.w #$60, d0
pub fn i_08e11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E11E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E11E: jsr $8efce(pc)
pub fn i_08e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E122;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E122);
    cpu.pc = 0x8EFCE;
    16
}
/// $08E122: bcc.w $8e156
pub fn i_08e122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E126;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E156; return 10; }
    8
}
/// $08E126: move.w #$f, d0
pub fn i_08e126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E12A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E12A: jsr $5ea1c.l
pub fn i_08e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E130);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E130: move.b #$20, $99(a6)
pub fn i_08e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E136;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E136: add.b d0, $99(a6)
pub fn i_08e136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E13A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08E13A: move.b #$1, $98(a6)
pub fn i_08e13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E140;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E140: jsr $8f002(pc)
pub fn i_08e140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E144;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E144);
    cpu.pc = 0x8F002;
    16
}
/// $08E144: jsr $8f010(pc)
pub fn i_08e144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E148;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E148);
    cpu.pc = 0x8F010;
    16
}
/// $08E148: jsr $8e172(pc)
pub fn i_08e148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E14C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E14C);
    cpu.pc = 0x8E172;
    16
}
/// $08E14C: lea.l $8df72(pc), a1
pub fn i_08e14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E150;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF72;
    16
}
/// $08E150: move.l a1, (a6)
pub fn i_08e150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E152;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E152: bra.w $8e162
pub fn i_08e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E156;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E162;
    10
}
/// $08E156: lea.l $2f3e0c.l, a0
pub fn i_08e156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E15C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E0C;
    20
}
/// $08E15C: jsr $28cd4.l
pub fn i_08e15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E162);
    cpu.pc = 0x28CD4;
    16
}
/// $08E162: jsr $8d184(pc)
pub fn i_08e162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E166;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E166);
    cpu.pc = 0x8D184;
    16
}
/// $08E166: bcc.w $8e170
pub fn i_08e166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E16A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E170; return 10; }
    8
}
/// $08E16A: jmp $518.l
pub fn i_08e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E170;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E170: rts
pub fn i_08e170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_LoadMapByDir_08e172  ($08E172, 34 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E172: clr.w d0
pub fn i_08e172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E174;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08E174: move.b $99(a6), d0
pub fn i_08e174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E178;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08E178: lsr.b #$4, d0
pub fn i_08e178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E17A;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $08E17A: andi.b #$7, d0
pub fn i_08e17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E17E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08E17E: movea.l #$2f3cdc, a0
pub fn i_08e17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E184;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F3CDC;
    cpu.a[0] = v1;
    4
}
/// $08E184: lsl.w #$2, d0
pub fn i_08e184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E186;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08E186: movea.l (a0, d0.w), a0
pub fn i_08e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E18A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08E18A: cmpa.l #$ffffffff, a0
pub fn i_08e18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E190;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08E190: beq.w $8e19a
pub fn i_08e190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E194;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8E19A; return 10; }
    8
}
// ===== JsrAbsThunk_08e194  ($08E194, 8 B, thunks/jsr_abs_thunks.c)
/// $08E194: jsr $28cd4.l
pub fn i_08e194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E19A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E19A);
    cpu.pc = 0x28CD4;
    16
}
/// $08E19A: rts
pub fn i_08e19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E19C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl160_08e19c  ($08E19C, 86 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E19C: jsr $8f108(pc)
pub fn i_08e19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1A0);
    cpu.pc = 0x8F108;
    16
}
/// $08E1A0: jsr $8f002(pc)
pub fn i_08e1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1A4);
    cpu.pc = 0x8F002;
    16
}
/// $08E1A4: jsr $8f02c(pc)
pub fn i_08e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1A8);
    cpu.pc = 0x8F02C;
    16
}
/// $08E1A8: move.w #$fd, d1
pub fn i_08e1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1AC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E1AC: jsr $236e.l
pub fn i_08e1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1B2);
    cpu.pc = 0x236E;
    16
}
/// $08E1B2: lea.l $2f3e86.l, a0
pub fn i_08e1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E86;
    20
}
/// $08E1B8: jsr $28cd4.l
pub fn i_08e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1BE);
    cpu.pc = 0x28CD4;
    16
}
/// $08E1BE: lea.l $8e1c4(pc), a1
pub fn i_08e1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E1C4;
    16
}
/// $08E1C2: move.l a1, (a6)
pub fn i_08e1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E1C4: jsr $8f040(pc)
pub fn i_08e1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1C8);
    cpu.pc = 0x8F040;
    16
}
/// $08E1C8: bcc.w $8e1de
pub fn i_08e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E1DE; return 10; }
    8
}
/// $08E1CC: lea.l $2f3e92.l, a0
pub fn i_08e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E92;
    20
}
/// $08E1D2: jsr $28cd4.l
pub fn i_08e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1D8);
    cpu.pc = 0x28CD4;
    16
}
/// $08E1D8: lea.l $8e1de(pc), a1
pub fn i_08e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E1DE;
    16
}
/// $08E1DC: move.l a1, (a6)
pub fn i_08e1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E1DE: jsr $2783a.l
pub fn i_08e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1E4);
    cpu.pc = 0x2783A;
    16
}
/// $08E1E4: jsr $8d1e8(pc)
pub fn i_08e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1E8);
    cpu.pc = 0x8D1E8;
    16
}
/// $08E1E8: bcc.w $8e1f2
pub fn i_08e1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E1F2; return 10; }
    8
}
/// $08E1EC: jmp $518.l
pub fn i_08e1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08e1f2  ($08E1F2, 8 B, thunks/jsr_abs_thunks.c)
/// $08E1F2: jsr $28d70.l
pub fn i_08e1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1F8);
    cpu.pc = 0x28D70;
    16
}
/// $08E1F8: rts
pub fn i_08e1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sentry_Init_08e1fa  ($08E1FA, 134 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E1FA: move.b #$1, $98(a6)
pub fn i_08e1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E200;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E200: move.b #$20, $99(a6)
pub fn i_08e200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E206;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E206: move.b #$14, $9a(a6)
pub fn i_08e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E20C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E20C: move.w #$40, $22(a6)
pub fn i_08e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E212;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E212: move.w #$180, $24(a6)
pub fn i_08e212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E218;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E218: clr.w $26(a6)
pub fn i_08e218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E21C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E21C: jsr $8f002(pc)
pub fn i_08e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E220;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E220);
    cpu.pc = 0x8F002;
    16
}
/// $08E220: jsr $8f010(pc)
pub fn i_08e220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E224;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E224);
    cpu.pc = 0x8F010;
    16
}
/// $08E224: eori.b #$1, $3a(a6)
pub fn i_08e224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E22A;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E22A: move.w #$fe, d1
pub fn i_08e22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E22E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E22E: jsr $236e.l
pub fn i_08e22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E234);
    cpu.pc = 0x236E;
    16
}
/// $08E234: lea.l $2f3e9e.l, a0
pub fn i_08e234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E23A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E9E;
    20
}
/// $08E23A: jsr $28cd4.l
pub fn i_08e23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E240);
    cpu.pc = 0x28CD4;
    16
}
/// $08E240: move.w #$ffff, $38(a6)
pub fn i_08e240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E246;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E246: bset.b #$6, $12(a6)
pub fn i_08e246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E24C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E24C: lea.l $8e252(pc), a1
pub fn i_08e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E250;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E252;
    16
}
/// $08E250: move.l a1, (a6)
pub fn i_08e250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E252;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E252: jsr $8efb0(pc)
pub fn i_08e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E256;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E256);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E256: jsr $28d70.l
pub fn i_08e256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E25C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E25C);
    cpu.pc = 0x28D70;
    16
}
/// $08E25C: bcc.w $8e286
pub fn i_08e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E260;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E286; return 10; }
    8
}
/// $08E260: cmpi.b #$0, $98(a6)
pub fn i_08e260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E266;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E266: bne.w $8e274
pub fn i_08e266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E26A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E274; return 10; }
    8
}
/// $08E26A: subi.w #$48, $22(a6)
pub fn i_08e26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E270;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08E270: bra.w $8e27a
pub fn i_08e270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E274;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E27A;
    10
}
/// $08E274: addi.w #$48, $22(a6)
pub fn i_08e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E27A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08E27A: move.b $9a(a6), $70(a6)
pub fn i_08e27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E280;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetTaskHandler_08e280  ($08E280, 8 B, kernel/task_handlers.c)
/// $08E280: lea.l $8e288(pc), a1
pub fn i_08e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E284;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E288;
    16
}
/// $08E284: move.l a1, (a6)
pub fn i_08e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E286;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E286: rts
pub fn i_08e286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sentry_Wait_08e288  ($08E288, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E288: subq.b #$1, $70(a6)
pub fn i_08e288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E28C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08E28C: bne.w $8e2a2
pub fn i_08e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E290;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E2A2; return 10; }
    8
}
/// $08E290: lea.l $2f3e9e.l, a0
pub fn i_08e290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E296;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E9E;
    20
}
/// $08E296: jsr $28cd4.l
pub fn i_08e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E29C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E29C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E29C: lea.l $8e252(pc), a1
pub fn i_08e29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E252;
    16
}
/// $08E2A0: move.l a1, (a6)
pub fn i_08e2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E2A2: jsr $8d2f8(pc)
pub fn i_08e2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2A6);
    cpu.pc = 0x8D2F8;
    16
}
// ===== JsrPcThunk_08e2a6  ($08E2A6, 6 B, thunks/jsr_pc_thunks.c)
/// $08E2A6: jsr $8efb0(pc)
pub fn i_08e2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2AA);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E2AA: rts
pub fn i_08e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl180_08e2ac  ($08E2AC, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E2AC: jsr $8f108(pc)
pub fn i_08e2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2B0);
    cpu.pc = 0x8F108;
    16
}
/// $08E2B0: jsr $8f002(pc)
pub fn i_08e2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2B4);
    cpu.pc = 0x8F002;
    16
}
/// $08E2B4: jsr $8f02c(pc)
pub fn i_08e2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2B8);
    cpu.pc = 0x8F02C;
    16
}
/// $08E2B8: move.w #$124, d1
pub fn i_08e2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2BC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E2BC: jsr $236e.l
pub fn i_08e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E2C2);
    cpu.pc = 0x236E;
    16
}
/// $08E2C2: lea.l $2f3f4c.l, a0
pub fn i_08e2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3F4C;
    20
}
/// $08E2C8: jsr $28cd4.l
pub fn i_08e2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E2CE);
    cpu.pc = 0x28CD4;
    16
}
/// $08E2CE: move.w #$ffff, $38(a6)
pub fn i_08e2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E2D4: bset.b #$6, $12(a6)
pub fn i_08e2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2DA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E2DA: lea.l $8e2f0(pc), a1
pub fn i_08e2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E2F0;
    16
}
/// $08E2DE: move.l a1, (a6)
pub fn i_08e2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E2E0: jsr $8f040(pc)
pub fn i_08e2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2E4);
    cpu.pc = 0x8F040;
    16
}
/// $08E2E4: bcc.w $8e2ee
pub fn i_08e2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E2EE; return 10; }
    8
}
// ===== SetTaskHandler_08e2e8  ($08E2E8, 8 B, kernel/task_handlers.c)
/// $08E2E8: lea.l $8e2f0(pc), a1
pub fn i_08e2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E2F0;
    16
}
/// $08E2EC: move.l a1, (a6)
pub fn i_08e2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E2EE: rts
pub fn i_08e2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl180_Run_08e2f0  ($08E2F0, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E2F0: jsr $2783a.l
pub fn i_08e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E2F6);
    cpu.pc = 0x2783A;
    16
}
/// $08E2F6: jsr $8efb0(pc)
pub fn i_08e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2FA);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E2FA: jsr $28d70.l
pub fn i_08e2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E300);
    cpu.pc = 0x28D70;
    16
}
/// $08E300: bcc.w $8e30a
pub fn i_08e300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E304;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E30A; return 10; }
    8
}
/// $08E304: jmp $518.l
pub fn i_08e304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E30A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E30A: rts
pub fn i_08e30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E30C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Tmpl181_08e30c  ($08E30C, 174 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E30C: jsr $8f002(pc)
pub fn i_08e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E310;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E310);
    cpu.pc = 0x8F002;
    16
}
/// $08E310: jsr $8f010(pc)
pub fn i_08e310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E314;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E314);
    cpu.pc = 0x8F010;
    16
}
/// $08E314: asr.w $28(a6)
pub fn i_08e314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E318;
    cpu.ir = 0xE0EE;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = do_shift_rotate(cpu, v2, 1, Size::Word, 0, false);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08E318: move.w #$125, d1
pub fn i_08e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E31C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E31C: jsr $236e.l
pub fn i_08e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E322);
    cpu.pc = 0x236E;
    16
}
/// $08E322: move.w #$ffff, $38(a6)
pub fn i_08e322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E328;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E328: bset.b #$6, $12(a6)
pub fn i_08e328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E32E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E32E: move.w $28(a6), $74(a6)
pub fn i_08e32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E334;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08E334: lea.l $2f3fc6.l, a0
pub fn i_08e334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E33A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3FC6;
    20
}
/// $08E33A: jsr $28cd4.l
pub fn i_08e33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E340);
    cpu.pc = 0x28CD4;
    16
}
/// $08E340: clr.w $28(a6)
pub fn i_08e340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E344;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E344: cmpi.b #$0, $98(a6)
pub fn i_08e344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E34A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E34A: bne.w $8e358
pub fn i_08e34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E34E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E358; return 10; }
    8
}
/// $08E34E: move.w #$fff8, $2c(a6)
pub fn i_08e34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E354;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E354: bra.w $8e35e
pub fn i_08e354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E358;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E35E;
    10
}
/// $08E358: move.w #$8, $2c(a6)
pub fn i_08e358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E35E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E35E: jsr $8f002(pc)
pub fn i_08e35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E362;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E362);
    cpu.pc = 0x8F002;
    16
}
/// $08E362: lea.l $8e368(pc), a1
pub fn i_08e362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E366;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E368;
    16
}
/// $08E366: move.l a1, (a6)
pub fn i_08e366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E368;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E368: jsr $8d2b0(pc)
pub fn i_08e368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E36C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E36C);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E36C: jsr $2783a.l
pub fn i_08e36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E372);
    cpu.pc = 0x2783A;
    16
}
/// $08E372: move.w $2c(a6), d0
pub fn i_08e372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E376;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E376: add.w d0, $28(a6)
pub fn i_08e376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E37A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E37A: cmpi.b #$0, $98(a6)
pub fn i_08e37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E380;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E380: bne.w $8e39a
pub fn i_08e380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E384;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E39A; return 10; }
    8
}
/// $08E384: move.w $28(a6), d0
pub fn i_08e384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E388;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E388: cmp.w $74(a6), d0
pub fn i_08e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E38C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08E38C: bgt.w $8e396
pub fn i_08e38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E390;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E396; return 10; }
    8
}
/// $08E390: lea.l $8e3c4(pc), a1
pub fn i_08e390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E394;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E3C4;
    16
}
/// $08E394: move.l a1, (a6)
pub fn i_08e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E396;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E396: bra.w $8e3ac
pub fn i_08e396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E39A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E3AC;
    10
}
/// $08E39A: move.w $28(a6), d0
pub fn i_08e39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E39E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E39E: cmp.w $74(a6), d0
pub fn i_08e39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3A2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08E3A2: blt.w $8e3ac
pub fn i_08e3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3A6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E3AC; return 10; }
    8
}
/// $08E3A6: lea.l $8e3c4(pc), a1
pub fn i_08e3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E3C4;
    16
}
/// $08E3AA: move.l a1, (a6)
pub fn i_08e3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E3AC: jsr $8d184(pc)
pub fn i_08e3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E3B0);
    cpu.pc = 0x8D184;
    16
}
/// $08E3B0: bcc.w $8e3bc
pub fn i_08e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E3BC; return 10; }
    8
}
/// $08E3B4: jmp $518.l
pub fn i_08e3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3BA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e3ba  ($08E3BA, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E3BA: rts
pub fn i_08e3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e3bc  ($08E3BC, 8 B, thunks/jsr_abs_thunks.c)
/// $08E3BC: jsr $28d70.l
pub fn i_08e3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E3C2);
    cpu.pc = 0x28D70;
    16
}
/// $08E3C2: rts
pub fn i_08e3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Pause_08e3c4  ($08E3C4, 64 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E3C4: move.w #$7, d0
pub fn i_08e3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E3C8: jsr $5ea1c.l
pub fn i_08e3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E3CE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E3CE: add.w d0, $70(a6)
pub fn i_08e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3D2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E3D2: lea.l $8e3d8(pc), a1
pub fn i_08e3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E3D8;
    16
}
/// $08E3D6: move.l a1, (a6)
pub fn i_08e3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E3D8: subq.w #$1, $70(a6)
pub fn i_08e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3DC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E3DC: cmpi.w #$0, $70(a6)
pub fn i_08e3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E3E2: bgt.w $8e3ec
pub fn i_08e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3E6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E3EC; return 10; }
    8
}
/// $08E3E6: lea.l $8e40e(pc), a1
pub fn i_08e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E40E;
    16
}
/// $08E3EA: move.l a1, (a6)
pub fn i_08e3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E3EC: jsr $8d2b0(pc)
pub fn i_08e3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E3F0);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E3F0: jsr $2783a.l
pub fn i_08e3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E3F6);
    cpu.pc = 0x2783A;
    16
}
/// $08E3F6: jsr $8d184(pc)
pub fn i_08e3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E3FA);
    cpu.pc = 0x8D184;
    16
}
/// $08E3FA: bcc.w $8e406
pub fn i_08e3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E406; return 10; }
    8
}
/// $08E3FE: jmp $518.l
pub fn i_08e3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E404;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e404  ($08E404, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E404: rts
pub fn i_08e404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E406;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e406  ($08E406, 8 B, thunks/jsr_abs_thunks.c)
/// $08E406: jsr $28d70.l
pub fn i_08e406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E40C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E40C);
    cpu.pc = 0x28D70;
    16
}
/// $08E40C: rts
pub fn i_08e40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E40E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Back_08e40e  ($08E40E, 138 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E40E: move.w #$7, d0
pub fn i_08e40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E412;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E412: jsr $5ea1c.l
pub fn i_08e412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E418);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E418: addi.w #$c, d0
pub fn i_08e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E41C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08E41C: move.w d0, $70(a6)
pub fn i_08e41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E420;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E420: neg.w $2c(a6)
pub fn i_08e420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E424;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E424: lea.l $2f3fe2.l, a0
pub fn i_08e424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E42A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3FE2;
    20
}
/// $08E42A: jsr $28cd4.l
pub fn i_08e42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E430);
    cpu.pc = 0x28CD4;
    16
}
/// $08E430: lea.l $8e436(pc), a1
pub fn i_08e430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E436;
    16
}
/// $08E434: move.l a1, (a6)
pub fn i_08e434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E436;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E436: jsr $8d2b0(pc)
pub fn i_08e436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E43A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E43A);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E43A: jsr $2783a.l
pub fn i_08e43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E440);
    cpu.pc = 0x2783A;
    16
}
/// $08E440: move.w $2c(a6), d0
pub fn i_08e440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E444;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E444: add.w d0, $28(a6)
pub fn i_08e444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E448;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E448: subq.w #$1, $70(a6)
pub fn i_08e448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E44C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E44C: cmpi.w #$0, $70(a6)
pub fn i_08e44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E452;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E452: bgt.w $8e45c
pub fn i_08e452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E456;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E45C; return 10; }
    8
}
/// $08E456: lea.l $8e4a2(pc), a1
pub fn i_08e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E45A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4A2;
    16
}
/// $08E45A: move.l a1, (a6)
pub fn i_08e45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E45C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E45C: cmpi.b #$0, $98(a6)
pub fn i_08e45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E462;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E462: bne.w $8e47a
pub fn i_08e462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E466;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E47A; return 10; }
    8
}
/// $08E466: cmpi.w #$0, $28(a6)
pub fn i_08e466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E46C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E46C: blt.w $8e476
pub fn i_08e46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E470;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E476; return 10; }
    8
}
/// $08E470: lea.l $8e4a2(pc), a1
pub fn i_08e470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E474;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4A2;
    16
}
/// $08E474: move.l a1, (a6)
pub fn i_08e474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E476;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E476: bra.w $8e48a
pub fn i_08e476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E47A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E48A;
    10
}
/// $08E47A: cmpi.w #$0, $28(a6)
pub fn i_08e47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E480;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E480: bgt.w $8e48a
pub fn i_08e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E484;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E48A; return 10; }
    8
}
/// $08E484: lea.l $8e4a2(pc), a1
pub fn i_08e484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E488;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4A2;
    16
}
/// $08E488: move.l a1, (a6)
pub fn i_08e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E48A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E48A: jsr $8d184(pc)
pub fn i_08e48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E48E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E48E);
    cpu.pc = 0x8D184;
    16
}
/// $08E48E: bcc.w $8e49a
pub fn i_08e48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E492;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E49A; return 10; }
    8
}
/// $08E492: jmp $518.l
pub fn i_08e492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E498;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e498  ($08E498, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E498: rts
pub fn i_08e498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E49A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e49a  ($08E49A, 8 B, thunks/jsr_abs_thunks.c)
/// $08E49A: jsr $28d70.l
pub fn i_08e49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4A0);
    cpu.pc = 0x28D70;
    16
}
/// $08E4A0: rts
pub fn i_08e4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Turn_08e4a2  ($08E4A2, 64 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E4A2: lea.l $2f3fee.l, a0
pub fn i_08e4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3FEE;
    20
}
/// $08E4A8: jsr $28cd4.l
pub fn i_08e4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4AE);
    cpu.pc = 0x28CD4;
    16
}
/// $08E4AE: lea.l $8e4b4(pc), a1
pub fn i_08e4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4B4;
    16
}
/// $08E4B2: move.l a1, (a6)
pub fn i_08e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E4B4: jsr $2783a.l
pub fn i_08e4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4BA);
    cpu.pc = 0x2783A;
    16
}
/// $08E4BA: jsr $28d70.l
pub fn i_08e4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4C0);
    cpu.pc = 0x28D70;
    16
}
/// $08E4C0: bcc.w $8e4d4
pub fn i_08e4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E4D4; return 10; }
    8
}
/// $08E4C4: neg.w $74(a6)
pub fn i_08e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4C8;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E4C8: eori.b #$1, $98(a6)
pub fn i_08e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4CE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E4CE: lea.l $8e334(pc), a1
pub fn i_08e4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E334;
    16
}
/// $08E4D2: move.l a1, (a6)
pub fn i_08e4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E4D4: jsr $8d184(pc)
pub fn i_08e4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4D8);
    cpu.pc = 0x8D184;
    16
}
/// $08E4D8: bcc.w $8e4e4
pub fn i_08e4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E4E4; return 10; }
    8
}
/// $08E4DC: jmp $518.l
pub fn i_08e4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4E2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e4e2  ($08E4E2, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E4E2: rts
pub fn i_08e4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0008E4E4  ($08E4E4, 2 B, thunks/trivial_rts.c)
/// $08E4E4: rts
pub fn i_08e4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_Tmpl182_08e4e6  ($08E4E6, 24 B, enemies/critters_rings_08e4e6.s)
/// $08E4E6: move.w #$c000, $38(a6)
pub fn i_08e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E4EC: jsr $8f002(pc)
pub fn i_08e4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4F0);
    cpu.pc = 0x8F002;
    16
}
/// $08E4F0: jsr $8f010(pc)
pub fn i_08e4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4F4);
    cpu.pc = 0x8F010;
    16
}
/// $08E4F4: move.w #$12a, d1
pub fn i_08e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E4F8: jsr $236e.l
pub fn i_08e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4FE);
    cpu.pc = 0x236E;
    16
}
// ===== Bobber_Descend_08e4fe  ($08E4FE, 70 B, enemies/critters_rings_08e4e6.s)
/// $08E4FE: move.w #$4, $2e(a6)
pub fn i_08e4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E504;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E504: move.w #$6, $70(a6)
pub fn i_08e504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E50A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E50A: move.w #$7, d0
pub fn i_08e50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E50E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E50E: jsr $5ea1c.l
pub fn i_08e50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E514);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E514: add.w d0, $70(a6)
pub fn i_08e514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E518;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E518: lea.l $2f4036.l, a0
pub fn i_08e518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E51E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4036;
    20
}
/// $08E51E: jsr $28cd4.l
pub fn i_08e51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E524);
    cpu.pc = 0x28CD4;
    16
}
/// $08E524: lea.l $8e52a(pc), a1
pub fn i_08e524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E528;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E52A;
    16
}
/// $08E528: move.l a1, (a6)
pub fn i_08e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E52A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E52A: jsr $2783a.l
pub fn i_08e52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E530);
    cpu.pc = 0x2783A;
    16
}
/// $08E530: jsr $8d34e(pc)
pub fn i_08e530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E534;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E534);
    cpu.pc = 0x8D34E;
    16
}
/// $08E534: jsr $28d70.l
pub fn i_08e534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E53A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E53A);
    cpu.pc = 0x28D70;
    16
}
/// $08E53A: cmpi.w #$1d0, $24(a6)
pub fn i_08e53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E540;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1D0, Size::Word, false);
    16
}
/// $08E540: blt.w $8e54c
pub fn i_08e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E544;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E54C; return 10; }
    8
}
// ===== SetTaskHandler_08e544  ($08E544, 8 B, kernel/task_handlers.c)
/// $08E544: lea.l $8e566(pc), a1
pub fn i_08e544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E566;
    16
}
/// $08E548: move.l a1, (a6)
pub fn i_08e548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E54A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E54A: rts
pub fn i_08e54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E54C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_DescendEnd_08e54c  ($08E54C, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E54C: subq.w #$1, $70(a6)
pub fn i_08e54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E550;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E550: cmpi.w #$0, $70(a6)
pub fn i_08e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E556;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E556: bhi.w $8e560
pub fn i_08e556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E55A;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8E560; return 10; }
    8
}
/// $08E55A: lea.l $8e566(pc), a1
pub fn i_08e55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E55E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E566;
    16
}
/// $08E55E: move.l a1, (a6)
pub fn i_08e55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E560;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08e560  ($08E560, 6 B, thunks/jsr_pc_thunks.c)
/// $08E560: jsr $8efb0(pc)
pub fn i_08e560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E564;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E564);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E564: rts
pub fn i_08e564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E566;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_Ascend_08e566  ($08E566, 70 B, enemies/critters_rings_08e4e6.s)
/// $08E566: move.w #$fffc, $2e(a6)
pub fn i_08e566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E56C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E56C: move.w #$6, $70(a6)
pub fn i_08e56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E572;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E572: move.w #$7, d0
pub fn i_08e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E576;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E576: jsr $5ea1c.l
pub fn i_08e576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E57C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E57C);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E57C: add.w d0, $70(a6)
pub fn i_08e57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E580;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E580: lea.l $2f4036.l, a0
pub fn i_08e580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E586;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4036;
    20
}
/// $08E586: jsr $28cd4.l
pub fn i_08e586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E58C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E58C: lea.l $8e592(pc), a1
pub fn i_08e58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E590;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E592;
    16
}
/// $08E590: move.l a1, (a6)
pub fn i_08e590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E592;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E592: jsr $2783a.l
pub fn i_08e592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E598;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E598);
    cpu.pc = 0x2783A;
    16
}
/// $08E598: jsr $8d34e(pc)
pub fn i_08e598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E59C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E59C);
    cpu.pc = 0x8D34E;
    16
}
/// $08E59C: jsr $28d70.l
pub fn i_08e59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E5A2);
    cpu.pc = 0x28D70;
    16
}
/// $08E5A2: cmpi.w #$160, $24(a6)
pub fn i_08e5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5A8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $08E5A8: bge.w $8e5b4
pub fn i_08e5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5AC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8E5B4; return 10; }
    8
}
// ===== SetTaskHandler_08e5ac  ($08E5AC, 8 B, kernel/task_handlers.c)
/// $08E5AC: lea.l $8e4fe(pc), a1
pub fn i_08e5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4FE;
    16
}
/// $08E5B0: move.l a1, (a6)
pub fn i_08e5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E5B2: rts
pub fn i_08e5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_AscendEnd_08e5b4  ($08E5B4, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E5B4: subq.w #$1, $70(a6)
pub fn i_08e5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E5B8: cmpi.w #$0, $70(a6)
pub fn i_08e5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5BE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E5BE: bhi.w $8e5c8
pub fn i_08e5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C2;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8E5C8; return 10; }
    8
}
/// $08E5C2: lea.l $8e4fe(pc), a1
pub fn i_08e5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4FE;
    16
}
/// $08E5C6: move.l a1, (a6)
pub fn i_08e5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08e5c8  ($08E5C8, 6 B, thunks/jsr_pc_thunks.c)
/// $08E5C8: jsr $8efb0(pc)
pub fn i_08e5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5CC);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E5CC: rts
pub fn i_08e5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Tmpl183_08e5ce  ($08E5CE, 76 B, enemies/critters_rings_08e4e6.s)
/// $08E5CE: jsr $8f108(pc)
pub fn i_08e5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5D2);
    cpu.pc = 0x8F108;
    16
}
/// $08E5D2: jsr $8f002(pc)
pub fn i_08e5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5D6);
    cpu.pc = 0x8F002;
    16
}
/// $08E5D6: jsr $8f010(pc)
pub fn i_08e5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5DA);
    cpu.pc = 0x8F010;
    16
}
/// $08E5DA: move.w #$fd00, $2a(a6)
pub fn i_08e5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E5E0: move.w #$10, $2e(a6)
pub fn i_08e5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E5E6: move.w #$124, d1
pub fn i_08e5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E5EA: jsr $236e.l
pub fn i_08e5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E5F0);
    cpu.pc = 0x236E;
    16
}
/// $08E5F0: lea.l $2f4072.l, a0
pub fn i_08e5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4072;
    20
}
/// $08E5F6: jsr $28cd4.l
pub fn i_08e5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E5FC);
    cpu.pc = 0x28CD4;
    16
}
/// $08E5FC: jsr $27cee.l
pub fn i_08e5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E602);
    cpu.pc = 0x27CEE;
    16
}
/// $08E602: jsr $28d70.l
pub fn i_08e602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E608);
    cpu.pc = 0x28D70;
    16
}
/// $08E608: move.w #$f, d0
pub fn i_08e608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E60C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E60C: jsr $5ea1c.l
pub fn i_08e60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E612);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E612: addi.w #$c, d0
pub fn i_08e612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E616;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08E616: move.w d0, $70(a6)
pub fn i_08e616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E61A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08e61a  ($08E61A, 8 B, kernel/task_handlers.c)
/// $08E61A: lea.l $8e622(pc), a1
pub fn i_08e61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E61E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E622;
    16
}
/// $08E61E: move.l a1, (a6)
pub fn i_08e61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E620;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E620: rts
pub fn i_08e620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E622;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Jump_08e622  ($08E622, 50 B, enemies/critters_rings_08e4e6.s)
/// $08E622: subq.w #$1, $70(a6)
pub fn i_08e622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E626;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E626: cmpi.w #$0, $70(a6)
pub fn i_08e626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E62C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E62C: bgt.w $8e636
pub fn i_08e62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E630;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E636; return 10; }
    8
}
/// $08E630: lea.l $8e65c(pc), a1
pub fn i_08e630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E634;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E65C;
    16
}
/// $08E634: move.l a1, (a6)
pub fn i_08e634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E636;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E636: cmpi.w #$40, $2a(a6)
pub fn i_08e636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E63C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Word, false);
    16
}
/// $08E63C: blt.w $8e646
pub fn i_08e63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E640;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E646; return 10; }
    8
}
/// $08E640: lea.l $8e65c(pc), a1
pub fn i_08e640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E644;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E65C;
    16
}
/// $08E644: move.l a1, (a6)
pub fn i_08e644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E646;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E646: jsr $8d34e(pc)
pub fn i_08e646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E64A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E64A);
    cpu.pc = 0x8D34E;
    16
}
/// $08E64A: jsr $2783a.l
pub fn i_08e64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E650);
    cpu.pc = 0x2783A;
    16
}
/// $08E650: jsr $8efb0(pc)
pub fn i_08e650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E654;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E654);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08e654  ($08E654, 8 B, thunks/jsr_abs_thunks.c)
/// $08E654: jsr $28d70.l
pub fn i_08e654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E65A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E65A);
    cpu.pc = 0x28D70;
    16
}
/// $08E65A: rts
pub fn i_08e65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E65C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Hop_08e65c  ($08E65C, 84 B, enemies/critters_rings_08e4e6.s)
/// $08E65C: move.w #$3f, d0
pub fn i_08e65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E660;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E660: jsr $5ea1c.l
pub fn i_08e660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E666);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E666: addi.w #$40, d0
pub fn i_08e666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E66A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08E66A: neg.w d0
pub fn i_08e66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E66C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08E66C: move.w d0, $2a(a6)
pub fn i_08e66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E670;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E670: move.w #$4, $2e(a6)
pub fn i_08e670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E676;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E676: move.w #$b, $70(a6)
pub fn i_08e676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E67C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E67C: lea.l $2f407e.l, a0
pub fn i_08e67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E682;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F407E;
    20
}
/// $08E682: jsr $28cd4.l
pub fn i_08e682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E688);
    cpu.pc = 0x28CD4;
    16
}
/// $08E688: lea.l $8e68e(pc), a1
pub fn i_08e688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E68C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E68E;
    16
}
/// $08E68C: move.l a1, (a6)
pub fn i_08e68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E68E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E68E: subq.w #$1, $70(a6)
pub fn i_08e68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E692;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E692: cmpi.w #$0, $70(a6)
pub fn i_08e692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E698;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E698: bgt.w $8e6a2
pub fn i_08e698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E69C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E6A2; return 10; }
    8
}
/// $08E69C: lea.l $8e6b8(pc), a1
pub fn i_08e69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E6B8;
    16
}
/// $08E6A0: move.l a1, (a6)
pub fn i_08e6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E6A2: jsr $8d34e(pc)
pub fn i_08e6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6A6);
    cpu.pc = 0x8D34E;
    16
}
/// $08E6A6: jsr $2783a.l
pub fn i_08e6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6AC);
    cpu.pc = 0x2783A;
    16
}
/// $08E6AC: jsr $8efb0(pc)
pub fn i_08e6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6B0);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08e6b0  ($08E6B0, 8 B, thunks/jsr_abs_thunks.c)
/// $08E6B0: jsr $28d70.l
pub fn i_08e6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6B6);
    cpu.pc = 0x28D70;
    16
}
/// $08E6B6: rts
pub fn i_08e6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Fall_08e6b8  ($08E6B8, 32 B, enemies/critters_rings_08e4e6.s)
/// $08E6B8: lea.l $2f408a.l, a0
pub fn i_08e6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F408A;
    20
}
/// $08E6BE: jsr $28cd4.l
pub fn i_08e6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6C4);
    cpu.pc = 0x28CD4;
    16
}
/// $08E6C4: lea.l $8e6ca(pc), a1
pub fn i_08e6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E6CA;
    16
}
/// $08E6C8: move.l a1, (a6)
pub fn i_08e6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E6CA: jsr $8d2f8(pc)
pub fn i_08e6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6CE);
    cpu.pc = 0x8D2F8;
    16
}
/// $08E6CE: jsr $2783a.l
pub fn i_08e6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6D4);
    cpu.pc = 0x2783A;
    16
}
/// $08E6D4: jsr $8efb0(pc)
pub fn i_08e6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6D8);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08e6d8  ($08E6D8, 8 B, thunks/jsr_abs_thunks.c)
/// $08E6D8: jsr $28d70.l
pub fn i_08e6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6DE);
    cpu.pc = 0x28D70;
    16
}
/// $08E6DE: rts
pub fn i_08e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Runner_Tmpl184_08e6e0  ($08E6E0, 46 B, enemies/critters_rings_08e4e6.s)
/// $08E6E0: jsr $8f108(pc)
pub fn i_08e6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6E4);
    cpu.pc = 0x8F108;
    16
}
/// $08E6E4: jsr $8f002(pc)
pub fn i_08e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6E8);
    cpu.pc = 0x8F002;
    16
}
/// $08E6E8: jsr $8f010(pc)
pub fn i_08e6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6EC);
    cpu.pc = 0x8F010;
    16
}
/// $08E6EC: move.w #$128, d1
pub fn i_08e6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E6F0: jsr $236e.l
pub fn i_08e6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6F6);
    cpu.pc = 0x236E;
    16
}
/// $08E6F6: lea.l $2f4096.l, a0
pub fn i_08e6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4096;
    20
}
/// $08E6FC: jsr $28cd4.l
pub fn i_08e6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E702);
    cpu.pc = 0x28CD4;
    16
}
/// $08E702: jsr $27cee.l
pub fn i_08e702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E708);
    cpu.pc = 0x27CEE;
    16
}
/// $08E708: jsr $28d70.l
pub fn i_08e708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E70E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E70E);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_08e70e  ($08E70E, 8 B, kernel/task_handlers.c)
/// $08E70E: lea.l $8e716(pc), a1
pub fn i_08e70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E712;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E716;
    16
}
/// $08E712: move.l a1, (a6)
pub fn i_08e712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E714;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E714: rts
pub fn i_08e714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E716;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Runner_Run_08e716  ($08E716, 24 B, enemies/critters_rings_08e4e6.s)
/// $08E716: jsr $8d2b0(pc)
pub fn i_08e716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E71A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E71A);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E71A: jsr $2783a.l
pub fn i_08e71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E720);
    cpu.pc = 0x2783A;
    16
}
/// $08E720: jsr $8d184(pc)
pub fn i_08e720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E724;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E724);
    cpu.pc = 0x8D184;
    16
}
/// $08E724: bcc.w $8e730
pub fn i_08e724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E728;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E730; return 10; }
    8
}
/// $08E728: jmp $518.l
pub fn i_08e728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E72E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e72e  ($08E72E, 2 B, enemies/critters_rings_08e4e6.s)
/// $08E72E: rts
pub fn i_08e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E730;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e730  ($08E730, 8 B, thunks/jsr_abs_thunks.c)
/// $08E730: jsr $28d70.l
pub fn i_08e730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E736);
    cpu.pc = 0x28D70;
    16
}
/// $08E736: rts
pub fn i_08e736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E738;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Spawn3_08e738  ($08E738, 14 B, enemies/critters_rings_08e4e6.s)
/// $08E738: move.b #$3, $9a(a6)
pub fn i_08e738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E73E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E73E: clr.w $5c(a6)
pub fn i_08e73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E742;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E742: bra.w $8e750
pub fn i_08e742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E746;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E750;
    10
}
// ===== Nest_Tmpl185_08e746  ($08E746, 132 B, enemies/critters_rings_08e4e6.s)
/// $08E746: jsr $8f108(pc)
pub fn i_08e746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E74A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E74A);
    cpu.pc = 0x8F108;
    16
}
/// $08E74A: move.w #$1, $5c(a6)
pub fn i_08e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E750;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E750: jsr $8f002(pc)
pub fn i_08e750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E754;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E754);
    cpu.pc = 0x8F002;
    16
}
/// $08E754: move.w #$125, d1
pub fn i_08e754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E758;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E758: jsr $236e.l
pub fn i_08e758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E75E);
    cpu.pc = 0x236E;
    16
}
/// $08E75E: lea.l $2f41f2.l, a0
pub fn i_08e75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E764;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F41F2;
    20
}
/// $08E764: jsr $28cd4.l
pub fn i_08e764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E76A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E76A);
    cpu.pc = 0x28CD4;
    16
}
/// $08E76A: lea.l $2f414a.l, a0
pub fn i_08e76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E770;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F414A;
    20
}
/// $08E770: move.l a0, $48(a6)
pub fn i_08e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E774;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08E774: bclr.b #$3, $13(a6)
pub fn i_08e774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E77A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E77A: move.w #$64, $66(a6)
pub fn i_08e77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E780;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E780: subq.b #$1, $9a(a6)
pub fn i_08e780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E784;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x9A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08E784: cmpi.b #$0, $9a(a6)
pub fn i_08e784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E78A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E78A: blt.w $8e7a0
pub fn i_08e78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E78E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E7A0; return 10; }
    8
}
/// $08E78E: lea.l $8eb56(pc), a1
pub fn i_08e78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E792;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB56;
    16
}
/// $08E792: jsr $4ae.l
pub fn i_08e792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E798);
    cpu.pc = 0x4AE;
    16
}
/// $08E798: jsr $5dd02.l
pub fn i_08e798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E79E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E79E);
    cpu.pc = 0x5DD02;
    16
}
/// $08E79E: bra.b $8e780
pub fn i_08e79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7A0;
    cpu.ir = 0x60E0;
    cpu.pc = 0x8E780;
    10
}
/// $08E7A0: lea.l $8e7a6(pc), a1
pub fn i_08e7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E7A6;
    16
}
/// $08E7A4: move.l a1, (a6)
pub fn i_08e7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E7A6: jsr $2783a.l
pub fn i_08e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7AC);
    cpu.pc = 0x2783A;
    16
}
/// $08E7AC: jsr $28d70.l
pub fn i_08e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7B2);
    cpu.pc = 0x28D70;
    16
}
/// $08E7B2: jsr $2870a.l
pub fn i_08e7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7B8);
    cpu.pc = 0x2870A;
    16
}
/// $08E7B8: bcc.w $8e7c2
pub fn i_08e7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E7C2; return 10; }
    8
}
/// $08E7BC: lea.l $8e7de(pc), a1
pub fn i_08e7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E7DE;
    16
}
/// $08E7C0: move.l a1, (a6)
pub fn i_08e7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E7C2: jsr $8d184(pc)
pub fn i_08e7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E7C6);
    cpu.pc = 0x8D184;
    16
}
/// $08E7C6: bcc.w $8e7d8
pub fn i_08e7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E7D8; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e7ca  ($08E7CA, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E7CA: jsr $5b6.l
pub fn i_08e7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7D0);
    cpu.pc = 0x5B6;
    16
}
/// $08E7D0: jmp $518.l
pub fn i_08e7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E7D6: rts
pub fn i_08e7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e7d8  ($08E7D8, 6 B, thunks/jsr_pc_thunks.c)
/// $08E7D8: jsr $8ea50(pc)
pub fn i_08e7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E7DC);
    cpu.pc = 0x8EA50;
    16
}
/// $08E7DC: rts
pub fn i_08e7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage1_Hit_08e7de  ($08E7DE, 48 B, enemies/critters_rings_08e4e6.s)
/// $08E7DE: lea.l $2f4202.l, a0
pub fn i_08e7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4202;
    20
}
/// $08E7E4: jsr $28cd4.l
pub fn i_08e7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7EA);
    cpu.pc = 0x28CD4;
    16
}
/// $08E7EA: lea.l $8e7f0(pc), a1
pub fn i_08e7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E7F0;
    16
}
/// $08E7EE: move.l a1, (a6)
pub fn i_08e7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E7F0: jsr $2783a.l
pub fn i_08e7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7F6);
    cpu.pc = 0x2783A;
    16
}
/// $08E7F6: jsr $28d70.l
pub fn i_08e7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7FC);
    cpu.pc = 0x28D70;
    16
}
/// $08E7FC: bcc.w $8e806
pub fn i_08e7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E800;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E806; return 10; }
    8
}
/// $08E800: lea.l $8e822(pc), a1
pub fn i_08e800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E804;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E822;
    16
}
/// $08E804: move.l a1, (a6)
pub fn i_08e804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E806;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E806: jsr $8d184(pc)
pub fn i_08e806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E80A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E80A);
    cpu.pc = 0x8D184;
    16
}
/// $08E80A: bcc.w $8e81c
pub fn i_08e80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E80E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E81C; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e80e  ($08E80E, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E80E: jsr $5b6.l
pub fn i_08e80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E814);
    cpu.pc = 0x5B6;
    16
}
/// $08E814: jmp $518.l
pub fn i_08e814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E81A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E81A: rts
pub fn i_08e81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E81C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e81c  ($08E81C, 6 B, thunks/jsr_pc_thunks.c)
/// $08E81C: jsr $8ea50(pc)
pub fn i_08e81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E820;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E820);
    cpu.pc = 0x8EA50;
    16
}
/// $08E820: rts
pub fn i_08e820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E822;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage2_08e822  ($08E822, 54 B, enemies/critters_rings_08e4e6.s)
/// $08E822: bclr.b #$3, $13(a6)
pub fn i_08e822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E828;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E828: move.w #$64, $66(a6)
pub fn i_08e828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E82E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E82E: lea.l $8e834(pc), a1
pub fn i_08e82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E832;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E834;
    16
}
/// $08E832: move.l a1, (a6)
pub fn i_08e832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E834;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E834: jsr $2783a.l
pub fn i_08e834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E83A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E83A);
    cpu.pc = 0x2783A;
    16
}
/// $08E83A: jsr $28d70.l
pub fn i_08e83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E840);
    cpu.pc = 0x28D70;
    16
}
/// $08E840: jsr $2870a.l
pub fn i_08e840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E846);
    cpu.pc = 0x2870A;
    16
}
/// $08E846: bcc.w $8e850
pub fn i_08e846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E84A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E850; return 10; }
    8
}
/// $08E84A: lea.l $8e86c(pc), a1
pub fn i_08e84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E84E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E86C;
    16
}
/// $08E84E: move.l a1, (a6)
pub fn i_08e84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E850;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E850: jsr $8d184(pc)
pub fn i_08e850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E854;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E854);
    cpu.pc = 0x8D184;
    16
}
/// $08E854: bcc.w $8e866
pub fn i_08e854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E858;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E866; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e858  ($08E858, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E858: jsr $5b6.l
pub fn i_08e858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E85E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E85E);
    cpu.pc = 0x5B6;
    16
}
/// $08E85E: jmp $518.l
pub fn i_08e85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E864;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E864: rts
pub fn i_08e864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E866;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e866  ($08E866, 6 B, thunks/jsr_pc_thunks.c)
/// $08E866: jsr $8ea50(pc)
pub fn i_08e866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E86A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E86A);
    cpu.pc = 0x8EA50;
    16
}
/// $08E86A: rts
pub fn i_08e86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E86C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage2_Hit_08e86c  ($08E86C, 48 B, enemies/critters_rings_08e4e6.s)
/// $08E86C: lea.l $2f4284.l, a0
pub fn i_08e86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E872;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4284;
    20
}
/// $08E872: jsr $28cd4.l
pub fn i_08e872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E878);
    cpu.pc = 0x28CD4;
    16
}
/// $08E878: lea.l $8e87e(pc), a1
pub fn i_08e878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E87C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E87E;
    16
}
/// $08E87C: move.l a1, (a6)
pub fn i_08e87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E87E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E87E: jsr $2783a.l
pub fn i_08e87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E884);
    cpu.pc = 0x2783A;
    16
}
/// $08E884: jsr $28d70.l
pub fn i_08e884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E88A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E88A);
    cpu.pc = 0x28D70;
    16
}
/// $08E88A: bcc.w $8e894
pub fn i_08e88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E88E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E894; return 10; }
    8
}
/// $08E88E: lea.l $8e8b0(pc), a1
pub fn i_08e88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E8B0;
    16
}
/// $08E892: move.l a1, (a6)
pub fn i_08e892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E894;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E894: jsr $8d184(pc)
pub fn i_08e894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E898;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E898);
    cpu.pc = 0x8D184;
    16
}
/// $08E898: bcc.w $8e8aa
pub fn i_08e898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E89C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E8AA; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e89c  ($08E89C, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E89C: jsr $5b6.l
pub fn i_08e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8A2);
    cpu.pc = 0x5B6;
    16
}
/// $08E8A2: jmp $518.l
pub fn i_08e8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E8A8: rts
pub fn i_08e8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e8aa  ($08E8AA, 6 B, thunks/jsr_pc_thunks.c)
/// $08E8AA: jsr $8ea50(pc)
pub fn i_08e8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E8AE);
    cpu.pc = 0x8EA50;
    16
}
/// $08E8AE: rts
pub fn i_08e8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage3_08e8b0  ($08E8B0, 54 B, enemies/critters_rings_08e4e6.s)
/// $08E8B0: bclr.b #$3, $13(a6)
pub fn i_08e8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8B6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E8B6: move.w #$64, $66(a6)
pub fn i_08e8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E8BC: lea.l $8e8c2(pc), a1
pub fn i_08e8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E8C2;
    16
}
/// $08E8C0: move.l a1, (a6)
pub fn i_08e8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E8C2: jsr $2783a.l
pub fn i_08e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8C8);
    cpu.pc = 0x2783A;
    16
}
/// $08E8C8: jsr $28d70.l
pub fn i_08e8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8CE);
    cpu.pc = 0x28D70;
    16
}
/// $08E8CE: jsr $2870a.l
pub fn i_08e8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8D4);
    cpu.pc = 0x2870A;
    16
}
/// $08E8D4: bcc.w $8e8de
pub fn i_08e8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E8DE; return 10; }
    8
}
/// $08E8D8: lea.l $8e8fa(pc), a1
pub fn i_08e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E8FA;
    16
}
/// $08E8DC: move.l a1, (a6)
pub fn i_08e8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E8DE: jsr $8d184(pc)
pub fn i_08e8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E8E2);
    cpu.pc = 0x8D184;
    16
}
/// $08E8E2: bcc.w $8e8f4
pub fn i_08e8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E8F4; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e8e6  ($08E8E6, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E8E6: jsr $5b6.l
pub fn i_08e8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8EC);
    cpu.pc = 0x5B6;
    16
}
/// $08E8EC: jmp $518.l
pub fn i_08e8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E8F2: rts
pub fn i_08e8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e8f4  ($08E8F4, 6 B, thunks/jsr_pc_thunks.c)
/// $08E8F4: jsr $8ea50(pc)
pub fn i_08e8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E8F8);
    cpu.pc = 0x8EA50;
    16
}
/// $08E8F8: rts
pub fn i_08e8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage3_Hit_08e8fa  ($08E8FA, 48 B, enemies/critters_rings_08e4e6.s)
/// $08E8FA: lea.l $2f4306.l, a0
pub fn i_08e8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E900;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4306;
    20
}
/// $08E900: jsr $28cd4.l
pub fn i_08e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E906);
    cpu.pc = 0x28CD4;
    16
}
/// $08E906: lea.l $8e90c(pc), a1
pub fn i_08e906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E90A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E90C;
    16
}
/// $08E90A: move.l a1, (a6)
pub fn i_08e90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E90C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E90C: jsr $2783a.l
pub fn i_08e90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E912);
    cpu.pc = 0x2783A;
    16
}
/// $08E912: jsr $28d70.l
pub fn i_08e912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E918);
    cpu.pc = 0x28D70;
    16
}
/// $08E918: bcc.w $8e922
pub fn i_08e918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E91C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E922; return 10; }
    8
}
/// $08E91C: lea.l $8e93e(pc), a1
pub fn i_08e91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E920;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E93E;
    16
}
/// $08E920: move.l a1, (a6)
pub fn i_08e920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E922;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E922: jsr $8d184(pc)
pub fn i_08e922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E926;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E926);
    cpu.pc = 0x8D184;
    16
}
/// $08E926: bcc.w $8e938
pub fn i_08e926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E92A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E938; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e92a  ($08E92A, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E92A: jsr $5b6.l
pub fn i_08e92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E930);
    cpu.pc = 0x5B6;
    16
}
/// $08E930: jmp $518.l
pub fn i_08e930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E936;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E936: rts
pub fn i_08e936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E938;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e938  ($08E938, 6 B, thunks/jsr_pc_thunks.c)
/// $08E938: jsr $8ea50(pc)
pub fn i_08e938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E93C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E93C);
    cpu.pc = 0x8EA50;
    16
}
/// $08E93C: rts
pub fn i_08e93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E93E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage4_08e93e  ($08E93E, 54 B, enemies/critters_rings_08e4e6.s)
/// $08E93E: bclr.b #$3, $13(a6)
pub fn i_08e93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E944;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E944: move.w #$64, $66(a6)
pub fn i_08e944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E94A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E94A: lea.l $8e950(pc), a1
pub fn i_08e94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E94E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E950;
    16
}
/// $08E94E: move.l a1, (a6)
pub fn i_08e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E950: jsr $2783a.l
pub fn i_08e950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E956);
    cpu.pc = 0x2783A;
    16
}
/// $08E956: jsr $28d70.l
pub fn i_08e956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E95C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E95C);
    cpu.pc = 0x28D70;
    16
}
/// $08E95C: jsr $2870a.l
pub fn i_08e95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E962);
    cpu.pc = 0x2870A;
    16
}
/// $08E962: bcc.w $8e96c
pub fn i_08e962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E966;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E96C; return 10; }
    8
}
/// $08E966: lea.l $8e988(pc), a1
pub fn i_08e966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E96A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E988;
    16
}
/// $08E96A: move.l a1, (a6)
pub fn i_08e96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E96C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E96C: jsr $8d184(pc)
pub fn i_08e96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E970;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E970);
    cpu.pc = 0x8D184;
    16
}
/// $08E970: bcc.w $8e982
pub fn i_08e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E974;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E982; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e974  ($08E974, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E974: jsr $5b6.l
pub fn i_08e974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E97A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E97A);
    cpu.pc = 0x5B6;
    16
}
/// $08E97A: jmp $518.l
pub fn i_08e97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E980;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E980: rts
pub fn i_08e980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E982;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e982  ($08E982, 6 B, thunks/jsr_pc_thunks.c)
/// $08E982: jsr $8ea50(pc)
pub fn i_08e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E986;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E986);
    cpu.pc = 0x8EA50;
    16
}
/// $08E986: rts
pub fn i_08e986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E988;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Destroyed_08e988  ($08E988, 56 B, enemies/critters_rings_08e4e6.s)
/// $08E988: lea.l $ffff.w, a0
pub fn i_08e988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E98C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08E98C: move.l a0, $48(a6)
pub fn i_08e98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E990;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08E990: lea.l $2f4388.l, a0
pub fn i_08e990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E996;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4388;
    20
}
/// $08E996: jsr $28cd4.l
pub fn i_08e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E99C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E99C: lea.l $8e9a2(pc), a1
pub fn i_08e99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E9A2;
    16
}
/// $08E9A0: move.l a1, (a6)
pub fn i_08e9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E9A2: jsr $2783a.l
pub fn i_08e9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9A8);
    cpu.pc = 0x2783A;
    16
}
/// $08E9A8: jsr $28d70.l
pub fn i_08e9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9AE);
    cpu.pc = 0x28D70;
    16
}
/// $08E9AE: bcc.w $8e9b8
pub fn i_08e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E9B8; return 10; }
    8
}
/// $08E9B2: lea.l $8e9d4(pc), a1
pub fn i_08e9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E9D4;
    16
}
/// $08E9B6: move.l a1, (a6)
pub fn i_08e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E9B8: jsr $8d184(pc)
pub fn i_08e9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9BC);
    cpu.pc = 0x8D184;
    16
}
/// $08E9BC: bcc.w $8e9ce
pub fn i_08e9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E9CE; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e9c0  ($08E9C0, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E9C0: jsr $5b6.l
pub fn i_08e9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9C6);
    cpu.pc = 0x5B6;
    16
}
/// $08E9C6: jmp $518.l
pub fn i_08e9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E9CC: rts
pub fn i_08e9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e9ce  ($08E9CE, 6 B, thunks/jsr_pc_thunks.c)
/// $08E9CE: jsr $8ea50(pc)
pub fn i_08e9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9D2);
    cpu.pc = 0x8EA50;
    16
}
/// $08E9D2: rts
pub fn i_08e9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_DestroyedWait_08e9d4  ($08E9D4, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E9D4: jsr $2783a.l
pub fn i_08e9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9DA);
    cpu.pc = 0x2783A;
    16
}
/// $08E9DA: jsr $28d70.l
pub fn i_08e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9E0);
    cpu.pc = 0x28D70;
    16
}
/// $08E9E0: jsr $8d184(pc)
pub fn i_08e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9E4);
    cpu.pc = 0x8D184;
    16
}
/// $08E9E4: bcc.w $8e9f6
pub fn i_08e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E9F6; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e9e8  ($08E9E8, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E9E8: jsr $5b6.l
pub fn i_08e9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9EE);
    cpu.pc = 0x5B6;
    16
}
/// $08E9EE: jmp $518.l
pub fn i_08e9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9F4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E9F4: rts
pub fn i_08e9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_08e9f6  ($08E9F6, 6 B, thunks/jsr_pc_thunks.c)
/// $08E9F6: jsr $8ea50(pc)
pub fn i_08e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9FA);
    cpu.pc = 0x8EA50;
    16
}
/// $08E9FA: rts
pub fn i_08e9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_FlyOff_08e9fc  ($08E9FC, 26 B, enemies/critters_rings_08e4e6.s)
/// $08E9FC: lea.l $ffff.w, a0
pub fn i_08e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA00;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08EA00: move.l a0, $48(a6)
pub fn i_08ea00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA04;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08EA04: move.w #$ff80, $2a(a6)
pub fn i_08ea04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EA0A: move.w #$ffc0, $2e(a6)
pub fn i_08ea0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EA10: lea.l $8ea16(pc), a1
pub fn i_08ea10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EA16;
    16
}
/// $08EA14: move.l a1, (a6)
pub fn i_08ea14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Nest_FlyOff_Run_08ea16  ($08EA16, 42 B, enemies/critters_rings_08e4e6.s)
/// $08EA16: jsr $27c8c.l
pub fn i_08ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA1C);
    cpu.pc = 0x27C8C;
    16
}
/// $08EA1C: bcc.w $8ea2c
pub fn i_08ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EA2C; return 10; }
    8
}
/// $08EA20: jsr $5b6.l
pub fn i_08ea20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA26);
    cpu.pc = 0x5B6;
    16
}
/// $08EA26: jmp $518.l
pub fn i_08ea26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA2C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EA2C: jsr $2783a.l
pub fn i_08ea2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA32);
    cpu.pc = 0x2783A;
    16
}
/// $08EA32: jsr $28d70.l
pub fn i_08ea32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA38);
    cpu.pc = 0x28D70;
    16
}
/// $08EA38: jsr $8d184(pc)
pub fn i_08ea38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EA3C);
    cpu.pc = 0x8D184;
    16
}
/// $08EA3C: bcc.w $8ea4e
pub fn i_08ea3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EA4E; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08ea40  ($08EA40, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EA40: jsr $5b6.l
pub fn i_08ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA46);
    cpu.pc = 0x5B6;
    16
}
/// $08EA46: jmp $518.l
pub fn i_08ea46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA4C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EA4C: rts
pub fn i_08ea4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0008EA4E  ($08EA4E, 2 B, thunks/trivial_rts.c)
/// $08EA4E: rts
pub fn i_08ea4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_DieIfParentGone_08ea50  ($08EA50, 28 B, enemies/critters_rings_08e4e6.s)
/// $08EA50: cmpi.w #$0, $5c(a6)
pub fn i_08ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA56;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EA56: beq.w $8ea5c
pub fn i_08ea56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8EA5C; return 10; }
    8
}
/// $08EA5A: rts
pub fn i_08ea5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08EA5C: movea.l $c(a6), a1
pub fn i_08ea5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA60;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EA60: cmpi.l #$ffffffff, $48(a1)
pub fn i_08ea60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA68;
    cpu.ir = 0xCA9;
    let ad2 = cpu.a[1].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $08EA68: bne.w $8ea72
pub fn i_08ea68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EA72; return 10; }
    8
}
// ===== SetTaskHandler_08ea6c  ($08EA6C, 8 B, kernel/task_handlers.c)
/// $08EA6C: lea.l $8ea16(pc), a1
pub fn i_08ea6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EA16;
    16
}
/// $08EA70: move.l a1, (a6)
pub fn i_08ea70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EA72: rts
pub fn i_08ea72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest2_Tmpl187_08ea74  ($08EA74, 106 B, enemies/critters_rings_08e4e6.s)
/// $08EA74: jsr $8f108(pc)
pub fn i_08ea74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EA78);
    cpu.pc = 0x8F108;
    16
}
/// $08EA78: lea.l $2f419e.l, a0
pub fn i_08ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F419E;
    20
}
/// $08EA7E: move.l a0, $48(a6)
pub fn i_08ea7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA82;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08EA82: bclr.b #$3, $13(a6)
pub fn i_08ea82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08EA88: move.w #$64, $66(a6)
pub fn i_08ea88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EA8E: clr.b $20(a6)
pub fn i_08ea8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA92;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EA92: subq.b #$1, $98(a6)
pub fn i_08ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA96;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EA96: cmpi.b #$0, $98(a6)
pub fn i_08ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA9C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EA9C: blt.w $8eab2
pub fn i_08ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAA0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EAB2; return 10; }
    8
}
/// $08EAA0: lea.l $8eb56(pc), a1
pub fn i_08eaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB56;
    16
}
/// $08EAA4: jsr $4ae.l
pub fn i_08eaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAAA);
    cpu.pc = 0x4AE;
    16
}
/// $08EAAA: jsr $5dd02.l
pub fn i_08eaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAB0);
    cpu.pc = 0x5DD02;
    16
}
/// $08EAB0: bra.b $8ea92
pub fn i_08eab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB2;
    cpu.ir = 0x60E0;
    cpu.pc = 0x8EA92;
    10
}
/// $08EAB2: lea.l $8eab8(pc), a1
pub fn i_08eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EAB8;
    16
}
/// $08EAB6: move.l a1, (a6)
pub fn i_08eab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EAB8: jsr $2783a.l
pub fn i_08eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EABE);
    cpu.pc = 0x2783A;
    16
}
/// $08EABE: jsr $2870a.l
pub fn i_08eabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAC4);
    cpu.pc = 0x2870A;
    16
}
/// $08EAC4: bcc.w $8eace
pub fn i_08eac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EACE; return 10; }
    8
}
/// $08EAC8: lea.l $8eaee(pc), a1
pub fn i_08eac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EACC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EAEE;
    16
}
/// $08EACC: move.l a1, (a6)
pub fn i_08eacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EACE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EACE: jsr $8d184(pc)
pub fn i_08eace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EAD2);
    cpu.pc = 0x8D184;
    16
}
/// $08EAD2: bcc.w $8eaec
pub fn i_08ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EAEC; return 10; }
    8
}
/// $08EAD6: lea.l $ffff.w, a0
pub fn i_08ead6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EADA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08EADA: move.l a0, $48(a6)
pub fn i_08eada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EADE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Jsr5B6ThenJmpScheduler_08eade  ($08EADE, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EADE: jsr $5b6.l
pub fn i_08eade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAE4);
    cpu.pc = 0x5B6;
    16
}
/// $08EAE4: jmp $518.l
pub fn i_08eae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAEA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EAEA: rts
pub fn i_08eaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0008EAEC  ($08EAEC, 2 B, thunks/trivial_rts.c)
/// $08EAEC: rts
pub fn i_08eaec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest2_Hit_08eaee  ($08EAEE, 12 B, enemies/critters_rings_08e4e6.s)
/// $08EAEE: move.b #$1, $20(a6)
pub fn i_08eaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EAF4: move.w #$14, $70(a6)
pub fn i_08eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08eafa  ($08EAFA, 8 B, kernel/task_handlers.c)
/// $08EAFA: lea.l $8eb02(pc), a1
pub fn i_08eafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB02;
    16
}
/// $08EAFE: move.l a1, (a6)
pub fn i_08eafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EB00: rts
pub fn i_08eb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest2_HitWait_08eb02  ($08EB02, 68 B, enemies/critters_rings_08e4e6.s)
/// $08EB02: clr.b $20(a6)
pub fn i_08eb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB06;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EB06: lea.l $8eb0c(pc), a1
pub fn i_08eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB0C;
    16
}
/// $08EB0A: move.l a1, (a6)
pub fn i_08eb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EB0C: jsr $2783a.l
pub fn i_08eb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB12);
    cpu.pc = 0x2783A;
    16
}
/// $08EB12: subq.w #$1, $70(a6)
pub fn i_08eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB16;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08EB16: cmpi.w #$0, $70(a6)
pub fn i_08eb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB1C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EB1C: bgt.w $8eb36
pub fn i_08eb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB20;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8EB36; return 10; }
    8
}
/// $08EB20: bclr.b #$3, $13(a6)
pub fn i_08eb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB26;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08EB26: move.w #$64, $66(a6)
pub fn i_08eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EB2C: clr.b $20(a6)
pub fn i_08eb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB30;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EB30: lea.l $8eab8(pc), a1
pub fn i_08eb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EAB8;
    16
}
/// $08EB34: move.l a1, (a6)
pub fn i_08eb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EB36: jsr $8d184(pc)
pub fn i_08eb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EB3A);
    cpu.pc = 0x8D184;
    16
}
/// $08EB3A: bcc.w $8eb54
pub fn i_08eb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EB54; return 10; }
    8
}
/// $08EB3E: lea.l $ffff.w, a0
pub fn i_08eb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB42;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08EB42: move.l a0, $48(a6)
pub fn i_08eb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB46;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Jsr5B6ThenJmpScheduler_08eb46  ($08EB46, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EB46: jsr $5b6.l
pub fn i_08eb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB4C);
    cpu.pc = 0x5B6;
    16
}
/// $08EB4C: jmp $518.l
pub fn i_08eb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB52;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EB52: rts
pub fn i_08eb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0008EB54  ($08EB54, 2 B, thunks/trivial_rts.c)
/// $08EB54: rts
pub fn i_08eb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swarmer_Init_08eb56  ($08EB56, 22 B, enemies/critters_rings_08e4e6.s)
/// $08EB56: move.w #$125, d1
pub fn i_08eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB5A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EB5A: jsr $236e.l
pub fn i_08eb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB60);
    cpu.pc = 0x236E;
    16
}
/// $08EB60: lea.l $2f440a.l, a0
pub fn i_08eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F440A;
    20
}
/// $08EB66: jsr $28cd4.l
pub fn i_08eb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB6C);
    cpu.pc = 0x28CD4;
    16
}
// ===== Swarmer_AimParent_08eb6c  ($08EB6C, 42 B, enemies/critters_rings_08e4e6.s)
/// $08EB6C: movea.l $c(a6), a1
pub fn i_08eb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB70;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EB70: move.w $22(a1), d0
pub fn i_08eb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB74;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EB74: subi.w #$c, d0
pub fn i_08eb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB78;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EB78: move.w $24(a1), d1
pub fn i_08eb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB7C;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EB7C: subi.w #$18, d1
pub fn i_08eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB80;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EB80: sub.w $22(a6), d0
pub fn i_08eb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB84;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EB84: sub.w $24(a6), d1
pub fn i_08eb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB88;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EB88: jsr $5e018.l
pub fn i_08eb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB8E);
    cpu.pc = 0x5E018;
    16
}
/// $08EB8E: move.w d0, $34(a6)
pub fn i_08eb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB92;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EB92: bra.w $8eba4
pub fn i_08eb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB96;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EBA4;
    10
}
// ===== Swarmer_AimRandom_08eb96  ($08EB96, 180 B, enemies/critters_rings_08e4e6.s)
/// $08EB96: move.w #$ff, d0
pub fn i_08eb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB9A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EB9A: jsr $5ea1c.l
pub fn i_08eb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBA0);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EBA0: move.w d0, $34(a6)
pub fn i_08eba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBA4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBA4: move.w #$ff, d0
pub fn i_08eba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EBA8: jsr $5ea1c.l
pub fn i_08eba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBAE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EBAE: move.w d0, d1
pub fn i_08ebae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBB0;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EBB0: addi.w #$180, d1
pub fn i_08ebb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBB4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x180, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EBB4: move.w $34(a6), d0
pub fn i_08ebb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EBB8: jsr $13c0e.l
pub fn i_08ebb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBBE);
    cpu.pc = 0x13C0E;
    16
}
/// $08EBBE: move.w d1, $28(a6)
pub fn i_08ebbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBC2;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBC2: move.w d2, $2a(a6)
pub fn i_08ebc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBC6;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBC6: cmpi.w #$0, $28(a6)
pub fn i_08ebc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBCC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EBCC: bge.w $8ebd8
pub fn i_08ebcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBD0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8EBD8; return 10; }
    8
}
/// $08EBD0: clr.b $3a(a6)
pub fn i_08ebd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBD4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EBD4: bra.w $8ebde
pub fn i_08ebd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EBDE;
    10
}
/// $08EBD8: move.b #$1, $3a(a6)
pub fn i_08ebd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBDE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EBDE: move.w #$3, d0
pub fn i_08ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EBE2: jsr $5ea1c.l
pub fn i_08ebe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBE8);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EBE8: addi.w #$1, d0
pub fn i_08ebe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBEC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EBEC: move.w d0, $70(a6)
pub fn i_08ebec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBF0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBF0: lea.l $8ebf6(pc), a1
pub fn i_08ebf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EBF6;
    16
}
/// $08EBF4: move.l a1, (a6)
pub fn i_08ebf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EBF6: jsr $8d2f8(pc)
pub fn i_08ebf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EBFA);
    cpu.pc = 0x8D2F8;
    16
}
/// $08EBFA: jsr $2783a.l
pub fn i_08ebfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC00);
    cpu.pc = 0x2783A;
    16
}
/// $08EC00: subq.w #$1, $70(a6)
pub fn i_08ec00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC04;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08EC04: cmpi.w #$0, $70(a6)
pub fn i_08ec04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC0A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EC0A: bgt.w $8ec30
pub fn i_08ec0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC0E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8EC30; return 10; }
    8
}
/// $08EC0E: move.w #$3, d0
pub fn i_08ec0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC12;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EC12: jsr $5ea1c.l
pub fn i_08ec12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC18);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EC18: cmpi.w #$0, d0
pub fn i_08ec18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC1C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $08EC1C: bne.w $8ec2a
pub fn i_08ec1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EC2A; return 10; }
    8
}
/// $08EC20: lea.l $8eb6c(pc), a1
pub fn i_08ec20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB6C;
    16
}
/// $08EC24: move.l a1, (a6)
pub fn i_08ec24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EC26: bra.w $8ec30
pub fn i_08ec26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EC30;
    10
}
/// $08EC2A: lea.l $8eb96(pc), a1
pub fn i_08ec2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB96;
    16
}
/// $08EC2E: move.l a1, (a6)
pub fn i_08ec2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EC30: movea.l $c(a6), a1
pub fn i_08ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC34;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EC34: cmpi.b #$0, $20(a1)
pub fn i_08ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC3A;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EC3A: beq.w $8ec44
pub fn i_08ec3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8EC44; return 10; }
    8
}
/// $08EC3E: lea.l $8ec50(pc), a1
pub fn i_08ec3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EC50;
    16
}
/// $08EC42: move.l a1, (a6)
pub fn i_08ec42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EC44: jsr $28d70.l
pub fn i_08ec44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC4A);
    cpu.pc = 0x28D70;
    16
}
// ===== JsrPcThunk_08ec4a  ($08EC4A, 6 B, thunks/jsr_pc_thunks.c)
/// $08EC4A: jsr $8efb0(pc)
pub fn i_08ec4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EC4E);
    cpu.pc = 0x8EFB0;
    16
}
/// $08EC4E: rts
pub fn i_08ec4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swarmer_Flee_08ec50  ($08EC50, 132 B, enemies/critters_rings_08e4e6.s)
/// $08EC50: movea.l $c(a6), a1
pub fn i_08ec50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC54;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EC54: move.w $22(a6), d0
pub fn i_08ec54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC58;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EC58: addi.w #$c, d0
pub fn i_08ec58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC5C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EC5C: move.w $24(a6), d1
pub fn i_08ec5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC60;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EC60: addi.w #$18, d1
pub fn i_08ec60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC64;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EC64: sub.w $22(a1), d0
pub fn i_08ec64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC68;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EC68: sub.w $24(a1), d1
pub fn i_08ec68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC6C;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EC6C: jsr $5e018.l
pub fn i_08ec6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC72);
    cpu.pc = 0x5E018;
    16
}
/// $08EC72: move.w d0, $34(a6)
pub fn i_08ec72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC76;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EC76: move.w #$300, d1
pub fn i_08ec76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC7A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EC7A: move.w $34(a6), d0
pub fn i_08ec7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC7E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EC7E: jsr $13c0e.l
pub fn i_08ec7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC84);
    cpu.pc = 0x13C0E;
    16
}
/// $08EC84: move.w d1, $28(a6)
pub fn i_08ec84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC88;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EC88: move.w d2, $2a(a6)
pub fn i_08ec88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC8C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EC8C: cmpi.w #$0, $28(a6)
pub fn i_08ec8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC92;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EC92: bge.w $8ec9e
pub fn i_08ec92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC96;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8EC9E; return 10; }
    8
}
/// $08EC96: clr.b $3a(a6)
pub fn i_08ec96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC9A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EC9A: bra.w $8eca4
pub fn i_08ec9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8ECA4;
    10
}
/// $08EC9E: move.b #$1, $3a(a6)
pub fn i_08ec9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECA4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ECA4: move.w #$4, $70(a6)
pub fn i_08eca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ECAA: lea.l $8ecb0(pc), a1
pub fn i_08ecaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ECB0;
    16
}
/// $08ECAE: move.l a1, (a6)
pub fn i_08ecae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ECB0: jsr $8d2f8(pc)
pub fn i_08ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECB4);
    cpu.pc = 0x8D2F8;
    16
}
/// $08ECB4: jsr $2783a.l
pub fn i_08ecb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECBA);
    cpu.pc = 0x2783A;
    16
}
/// $08ECBA: subq.w #$1, $70(a6)
pub fn i_08ecba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECBE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08ECBE: cmpi.w #$0, $70(a6)
pub fn i_08ecbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08ECC4: bgt.w $8ecce
pub fn i_08ecc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECC8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8ECCE; return 10; }
    8
}
/// $08ECC8: lea.l $8eb6c(pc), a1
pub fn i_08ecc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB6C;
    16
}
/// $08ECCC: move.l a1, (a6)
pub fn i_08eccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ECCE: jsr $28d70.l
pub fn i_08ecce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECD4);
    cpu.pc = 0x28D70;
    16
}
// ===== JsrPcThunk_08ecd4  ($08ECD4, 6 B, thunks/jsr_pc_thunks.c)
/// $08ECD4: jsr $8efb0(pc)
pub fn i_08ecd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECD8);
    cpu.pc = 0x8EFB0;
    16
}
/// $08ECD8: rts
pub fn i_08ecd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swarmer_Tmpl186_08ecda  ($08ECDA, 30 B, enemies/critters_rings_08e4e6.s)
/// $08ECDA: jsr $8f108(pc)
pub fn i_08ecda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECDE);
    cpu.pc = 0x8F108;
    16
}
/// $08ECDE: jsr $8f002(pc)
pub fn i_08ecde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECE2);
    cpu.pc = 0x8F002;
    16
}
/// $08ECE2: move.w #$125, d1
pub fn i_08ece2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ECE6: jsr $236e.l
pub fn i_08ece6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECEC);
    cpu.pc = 0x236E;
    16
}
/// $08ECEC: lea.l $2f440a.l, a0
pub fn i_08ecec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F440A;
    20
}
/// $08ECF2: jsr $28cd4.l
pub fn i_08ecf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECF8);
    cpu.pc = 0x28CD4;
    16
}
// ===== Swarmer_Wander_08ecf8  ($08ECF8, 132 B, enemies/critters_rings_08e4e6.s)
/// $08ECF8: move.w #$ff, d0
pub fn i_08ecf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ECFC: jsr $5ea1c.l
pub fn i_08ecfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED02);
    cpu.pc = 0x5EA1C;
    16
}
/// $08ED02: move.w d0, $34(a6)
pub fn i_08ed02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED06;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED06: move.w #$ff, d0
pub fn i_08ed06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED0A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED0A: jsr $5ea1c.l
pub fn i_08ed0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED10);
    cpu.pc = 0x5EA1C;
    16
}
/// $08ED10: move.w d0, d1
pub fn i_08ed10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED12;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED12: addi.w #$100, d1
pub fn i_08ed12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED16;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08ED16: move.w $34(a6), d0
pub fn i_08ed16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08ED1A: jsr $13c0e.l
pub fn i_08ed1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED20);
    cpu.pc = 0x13C0E;
    16
}
/// $08ED20: move.w d1, $28(a6)
pub fn i_08ed20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED24;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED24: move.w d2, $2a(a6)
pub fn i_08ed24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED28;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED28: move.w #$3, d0
pub fn i_08ed28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED2C: jsr $5ea1c.l
pub fn i_08ed2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED32);
    cpu.pc = 0x5EA1C;
    16
}
/// $08ED32: addi.w #$2, d0
pub fn i_08ed32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED36;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08ED36: move.w d0, $70(a6)
pub fn i_08ed36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED3A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED3A: cmpi.w #$0, $28(a6)
pub fn i_08ed3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED40;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08ED40: bge.w $8ed4c
pub fn i_08ed40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED44;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8ED4C; return 10; }
    8
}
/// $08ED44: clr.b $3a(a6)
pub fn i_08ed44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED48;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08ED48: bra.w $8ed52
pub fn i_08ed48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8ED52;
    10
}
/// $08ED4C: move.b #$1, $3a(a6)
pub fn i_08ed4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ED52: lea.l $8ed58(pc), a1
pub fn i_08ed52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ED58;
    16
}
/// $08ED56: move.l a1, (a6)
pub fn i_08ed56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ED58: jsr $8d2f8(pc)
pub fn i_08ed58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ED5C);
    cpu.pc = 0x8D2F8;
    16
}
/// $08ED5C: jsr $2783a.l
pub fn i_08ed5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED62);
    cpu.pc = 0x2783A;
    16
}
/// $08ED62: subq.w #$1, $70(a6)
pub fn i_08ed62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED66;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08ED66: cmpi.w #$0, $70(a6)
pub fn i_08ed66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED6C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08ED6C: bgt.w $8ed76
pub fn i_08ed6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED70;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8ED76; return 10; }
    8
}
/// $08ED70: lea.l $8ecf8(pc), a1
pub fn i_08ed70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ECF8;
    16
}
/// $08ED74: move.l a1, (a6)
pub fn i_08ed74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ED76: jsr $28d70.l
pub fn i_08ed76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED7C);
    cpu.pc = 0x28D70;
    16
}
// ===== JsrPcThunk_08ed7c  ($08ED7C, 6 B, thunks/jsr_pc_thunks.c)
/// $08ED7C: jsr $8efb0(pc)
pub fn i_08ed7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ED80);
    cpu.pc = 0x8EFB0;
    16
}
/// $08ED80: rts
pub fn i_08ed80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Static_Tmpl188_08ed82  ($08ED82, 58 B, enemies/critters_rings_08e4e6.s)
/// $08ED82: move.w #$0, $38(a6)
pub fn i_08ed82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED88: jsr $8f002(pc)
pub fn i_08ed88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ED8C);
    cpu.pc = 0x8F002;
    16
}
/// $08ED8C: move.w #$12a, d1
pub fn i_08ed8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED90;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED90: jsr $236e.l
pub fn i_08ed90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED96);
    cpu.pc = 0x236E;
    16
}
/// $08ED96: lea.l $2f4426.l, a0
pub fn i_08ed96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4426;
    20
}
/// $08ED9C: jsr $28cd4.l
pub fn i_08ed9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDA2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EDA2: lea.l $8eda8(pc), a1
pub fn i_08eda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EDA8;
    16
}
/// $08EDA6: move.l a1, (a6)
pub fn i_08eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EDA8: jsr $2783a.l
pub fn i_08eda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDAE);
    cpu.pc = 0x2783A;
    16
}
/// $08EDAE: jsr $8d184(pc)
pub fn i_08edae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EDB2);
    cpu.pc = 0x8D184;
    16
}
/// $08EDB2: bcc.w $8edbe
pub fn i_08edb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EDBE; return 10; }
    8
}
/// $08EDB6: jmp $518.l
pub fn i_08edb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDBC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08edbc  ($08EDBC, 2 B, enemies/critters_rings_08e4e6.s)
/// $08EDBC: rts
pub fn i_08edbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08edbe  ($08EDBE, 8 B, thunks/jsr_abs_thunks.c)
/// $08EDBE: jsr $28d70.l
pub fn i_08edbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDC4);
    cpu.pc = 0x28D70;
    16
}
/// $08EDC4: rts
pub fn i_08edc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CamProp_Tmpl189_08edc6  ($08EDC6, 16 B, enemies/critters_rings_08e4e6.s)
/// $08EDC6: lea.l $2f4442.l, a0
pub fn i_08edc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4442;
    20
}
/// $08EDCC: jsr $28cd4.l
pub fn i_08edcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDD2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EDD2: bra.w $8ede2
pub fn i_08edd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EDE2;
    10
}
// ===== CamProp_Tmpl190_08edd6  ($08EDD6, 54 B, enemies/critters_rings_08e4e6.s)
/// $08EDD6: lea.l $2f44ae.l, a0
pub fn i_08edd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F44AE;
    20
}
/// $08EDDC: jsr $28cd4.l
pub fn i_08eddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDE2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EDE2: move.w #$0, $38(a6)
pub fn i_08ede2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EDE8: move.w #$f4, d1
pub fn i_08ede8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDEC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EDEC: jsr $236e.l
pub fn i_08edec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDF2);
    cpu.pc = 0x236E;
    16
}
/// $08EDF2: lea.l $8edf8(pc), a1
pub fn i_08edf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EDF8;
    16
}
/// $08EDF6: move.l a1, (a6)
pub fn i_08edf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EDF8: jsr $4407a.l
pub fn i_08edf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDFE);
    cpu.pc = 0x4407A;
    16
}
/// $08EDFE: jsr $8d184(pc)
pub fn i_08edfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE02);
    cpu.pc = 0x8D184;
    16
}
/// $08EE02: bcc.w $8ee0e
pub fn i_08ee02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EE0E; return 10; }
    8
}
/// $08EE06: jmp $518.l
pub fn i_08ee06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE0C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08ee0c  ($08EE0C, 2 B, enemies/critters_rings_08e4e6.s)
/// $08EE0C: rts
pub fn i_08ee0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08ee0e  ($08EE0E, 8 B, thunks/jsr_abs_thunks.c)
/// $08EE0E: jsr $28d70.l
pub fn i_08ee0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE14);
    cpu.pc = 0x28D70;
    16
}
/// $08EE14: rts
pub fn i_08ee14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Lob_Tmpl191_08ee16  ($08EE16, 92 B, enemies/critters_rings_08e4e6.s)
/// $08EE16: jsr $8f002(pc)
pub fn i_08ee16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE1A);
    cpu.pc = 0x8F002;
    16
}
/// $08EE1A: jsr $8f010(pc)
pub fn i_08ee1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE1E);
    cpu.pc = 0x8F010;
    16
}
/// $08EE1E: clr.w d0
pub fn i_08ee1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE20;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08EE20: move.b $99(a6), d0
pub fn i_08ee20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE24;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08EE24: lsl.w #$5, d0
pub fn i_08ee24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE26;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08EE26: neg.w d0
pub fn i_08ee26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE28;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08EE28: move.w d0, $2a(a6)
pub fn i_08ee28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE2C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EE2C: move.w #$0, $38(a6)
pub fn i_08ee2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE32;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EE32: move.w #$f4, d1
pub fn i_08ee32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE36;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EE36: jsr $236e.l
pub fn i_08ee36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE3C);
    cpu.pc = 0x236E;
    16
}
/// $08EE3C: lea.l $2f44ca.l, a0
pub fn i_08ee3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F44CA;
    20
}
/// $08EE42: jsr $28cd4.l
pub fn i_08ee42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE48);
    cpu.pc = 0x28CD4;
    16
}
/// $08EE48: lea.l $8ee4e(pc), a1
pub fn i_08ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EE4E;
    16
}
/// $08EE4C: move.l a1, (a6)
pub fn i_08ee4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EE4E: jsr $8d2f8(pc)
pub fn i_08ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE52);
    cpu.pc = 0x8D2F8;
    16
}
/// $08EE52: cmpi.w #$190, $24(a6)
pub fn i_08ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE58;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x190, Size::Word, false);
    16
}
/// $08EE58: bgt.w $8ee6e
pub fn i_08ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE5C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8EE6E; return 10; }
    8
}
/// $08EE5C: lea.l $2f44d6.l, a0
pub fn i_08ee5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F44D6;
    20
}
/// $08EE62: jsr $28cd4.l
pub fn i_08ee62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE68);
    cpu.pc = 0x28CD4;
    16
}
/// $08EE68: lea.l $8ee7a(pc), a1
pub fn i_08ee68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EE7A;
    16
}
/// $08EE6C: move.l a1, (a6)
pub fn i_08ee6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EE6E: jsr $8efb0(pc)
pub fn i_08ee6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE72);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08ee72  ($08EE72, 8 B, thunks/jsr_abs_thunks.c)
/// $08EE72: jsr $28d70.l
pub fn i_08ee72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE78);
    cpu.pc = 0x28D70;
    16
}
/// $08EE78: rts
pub fn i_08ee78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Lob_Landed_08ee7a  ($08EE7A, 28 B, enemies/critters_rings_08e4e6.s)
/// $08EE7A: jsr $4407a.l
pub fn i_08ee7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE80);
    cpu.pc = 0x4407A;
    16
}
/// $08EE80: jsr $8efb0(pc)
pub fn i_08ee80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE84);
    cpu.pc = 0x8EFB0;
    16
}
/// $08EE84: jsr $28d70.l
pub fn i_08ee84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE8A);
    cpu.pc = 0x28D70;
    16
}
/// $08EE8A: bcc.w $8ee94
pub fn i_08ee8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EE94; return 10; }
    8
}
/// $08EE8E: jmp $518.l
pub fn i_08ee8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE94;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EE94: rts
pub fn i_08ee94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shard_V0_08ee96  ($08EE96, 22 B, enemies/critters_rings_08e4e6.s)
/// $08EE96: lea.l $2f40c6.l, a0
pub fn i_08ee96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40C6;
    20
}
/// $08EE9C: jsr $28cd4.l
pub fn i_08ee9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEA2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EEA2: move.b #$0, $74(a6)
pub fn i_08eea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEA8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EEA8: bra.w $8eed4
pub fn i_08eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EED4;
    10
}
// ===== Shard_V1_08eeac  ($08EEAC, 22 B, enemies/critters_rings_08e4e6.s)
/// $08EEAC: lea.l $2f40f2.l, a0
pub fn i_08eeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40F2;
    20
}
/// $08EEB2: jsr $28cd4.l
pub fn i_08eeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEB8);
    cpu.pc = 0x28CD4;
    16
}
/// $08EEB8: move.b #$1, $74(a6)
pub fn i_08eeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EEBE: bra.w $8eed4
pub fn i_08eebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EED4;
    10
}
// ===== Shard_V2_08eec2  ($08EEC2, 118 B, enemies/critters_rings_08e4e6.s)
/// $08EEC2: lea.l $2f411e.l, a0
pub fn i_08eec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F411E;
    20
}
/// $08EEC8: jsr $28cd4.l
pub fn i_08eec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EECE);
    cpu.pc = 0x28CD4;
    16
}
/// $08EECE: move.b #$2, $74(a6)
pub fn i_08eece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EED4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EED4: move.w #$1f, d0
pub fn i_08eed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EED8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EED8: jsr $5ea1c.l
pub fn i_08eed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEDE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EEDE: neg.w d0
pub fn i_08eede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEE0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08EEE0: move.w d0, $2a(a6)
pub fn i_08eee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEE4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EEE4: move.w #$1f, d0
pub fn i_08eee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEE8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EEE8: jsr $5ea1c.l
pub fn i_08eee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEEE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EEEE: subi.w #$30, d0
pub fn i_08eeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEF2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EEF2: move.w d0, $2e(a6)
pub fn i_08eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEF6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EEF6: jsr $8f002(pc)
pub fn i_08eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EEFA);
    cpu.pc = 0x8F002;
    16
}
/// $08EEFA: move.w #$ad, d1
pub fn i_08eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEFE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EEFE: jsr $236e.l
pub fn i_08eefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF04);
    cpu.pc = 0x236E;
    16
}
/// $08EF04: movea.l $c(a6), a1
pub fn i_08ef04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF08;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EF08: movea.l $c(a1), a2
pub fn i_08ef08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF0C;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08EF0C: move.w $24(a2), $5c(a6)
pub fn i_08ef0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF12;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08EF12: lea.l $8ef18(pc), a1
pub fn i_08ef12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EF18;
    16
}
/// $08EF16: move.l a1, (a6)
pub fn i_08ef16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EF18: jsr $8d34e(pc)
pub fn i_08ef18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EF1C);
    cpu.pc = 0x8D34E;
    16
}
/// $08EF1C: move.w $5c(a6), d0
pub fn i_08ef1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF20;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EF20: cmp.w $24(a6), d0
pub fn i_08ef20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF24;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08EF24: blt.w $8ef2e
pub fn i_08ef24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF28;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EF2E; return 10; }
    8
}
/// $08EF28: lea.l $8ef40(pc), a1
pub fn i_08ef28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EF40;
    16
}
/// $08EF2C: move.l a1, (a6)
pub fn i_08ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EF2E: jsr $2783a.l
pub fn i_08ef2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF34);
    cpu.pc = 0x2783A;
    16
}
/// $08EF34: jsr $8efb0(pc)
pub fn i_08ef34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EF38);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08ef38  ($08EF38, 8 B, thunks/jsr_abs_thunks.c)
/// $08EF38: jsr $28d70.l
pub fn i_08ef38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF3E);
    cpu.pc = 0x28D70;
    16
}
/// $08EF3E: rts
pub fn i_08ef3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shard_Land_08ef40  ($08EF40, 112 B, enemies/critters_rings_08e4e6.s)
/// $08EF40: cmpi.b #$0, $74(a6)
pub fn i_08ef40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF46;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EF46: bne.w $8ef5a
pub fn i_08ef46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF4A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EF5A; return 10; }
    8
}
/// $08EF4A: lea.l $2f40d2.l, a0
pub fn i_08ef4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40D2;
    20
}
/// $08EF50: jsr $28cd4.l
pub fn i_08ef50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF56);
    cpu.pc = 0x28CD4;
    16
}
/// $08EF56: bra.w $8ef8e
pub fn i_08ef56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EF8E;
    10
}
/// $08EF5A: cmpi.b #$0, $74(a6)
pub fn i_08ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF60;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EF60: bne.w $8ef74
pub fn i_08ef60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EF74; return 10; }
    8
}
/// $08EF64: lea.l $2f40fe.l, a0
pub fn i_08ef64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40FE;
    20
}
/// $08EF6A: jsr $28cd4.l
pub fn i_08ef6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF70);
    cpu.pc = 0x28CD4;
    16
}
/// $08EF70: bra.w $8ef8e
pub fn i_08ef70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF74;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EF8E;
    10
}
/// $08EF74: cmpi.b #$0, $74(a6)
pub fn i_08ef74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF7A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EF7A: bne.w $8ef8e
pub fn i_08ef7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EF8E; return 10; }
    8
}
/// $08EF7E: lea.l $2f412a.l, a0
pub fn i_08ef7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F412A;
    20
}
/// $08EF84: jsr $28cd4.l
pub fn i_08ef84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF8A);
    cpu.pc = 0x28CD4;
    16
}
/// $08EF8A: bra.w $8ef8e
pub fn i_08ef8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EF8E;
    10
}
/// $08EF8E: lea.l $8ef94(pc), a1
pub fn i_08ef8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EF94;
    16
}
/// $08EF92: move.l a1, (a6)
pub fn i_08ef92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EF94: jsr $2783a.l
pub fn i_08ef94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF9A);
    cpu.pc = 0x2783A;
    16
}
/// $08EF9A: jsr $8efb0(pc)
pub fn i_08ef9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EF9E);
    cpu.pc = 0x8EFB0;
    16
}
/// $08EF9E: jsr $28d70.l
pub fn i_08ef9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFA4);
    cpu.pc = 0x28D70;
    16
}
/// $08EFA4: bcc.w $8efae
pub fn i_08efa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EFAE; return 10; }
    8
}
/// $08EFA8: jmp $518.l
pub fn i_08efa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EFAE: rts
pub fn i_08efae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_GroundKill_08efb0  ($08EFB0, 16 B, enemies/critters_rings_08e4e6.s)
/// $08EFB0: movea.l #$ffffffff, a0
pub fn i_08efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFB6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08EFB6: jsr $5dd5c.l
pub fn i_08efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFBC);
    cpu.pc = 0x5DD5C;
    16
}
/// $08EFBC: bcc.w $8efcc
pub fn i_08efbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EFCC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08efc0  ($08EFC0, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EFC0: jsr $5b6.l
pub fn i_08efc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFC6);
    cpu.pc = 0x5B6;
    16
}
/// $08EFC6: jmp $518.l
pub fn i_08efc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EFCC: rts
pub fn i_08efcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_PlayerNearX_08efce  ($08EFCE, 40 B, enemies/critters_rings_08e4e6.s)
/// $08EFCE: lea.l $100440.l, a1
pub fn i_08efce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFD4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $08EFD4: move.w $22(a6), d1
pub fn i_08efd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFD8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EFD8: sub.w $22(a1), d1
pub fn i_08efd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFDC;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EFDC: cmp.w d0, d1
pub fn i_08efdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFDE;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $08EFDE: blt.w $8effc
pub fn i_08efde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFE2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EFFC; return 10; }
    8
}
/// $08EFE2: lea.l $1004e0.l, a1
pub fn i_08efe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $08EFE8: move.w $22(a6), d1
pub fn i_08efe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFEC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EFEC: sub.w $22(a1), d1
pub fn i_08efec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFF0;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EFF0: cmp.w d0, d1
pub fn i_08eff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFF2;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $08EFF2: blt.w $8effc
pub fn i_08eff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFF6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EFFC; return 10; }
    8
}
// ===== ClearXN_08eff6  ($08EFF6, 6 B, thunks/ccr_helpers.c)
/// $08EFF6: andi.b #$ee, ccr
pub fn i_08eff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFFA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08EFFA: rts
pub fn i_08effa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08effc  ($08EFFC, 6 B, thunks/ccr_helpers.c)
/// $08EFFC: ori.b #$11, ccr
pub fn i_08effc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F000;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F000: rts
pub fn i_08f000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F002;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_FacingFromParam_08f002  ($08F002, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F002: andi.b #$1, $98(a6)
pub fn i_08f002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F008;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08F008: move.b $98(a6), $3a(a6)
pub fn i_08f008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F00E;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08F00E: rts
pub fn i_08f00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F010;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_VelXFromParam_08f010  ($08F010, 28 B, enemies/critters_rings_08e4e6.s)
/// $08F010: clr.w d0
pub fn i_08f010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F012;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F012: move.b $99(a6), d0
pub fn i_08f012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F016;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F016: lsl.w #$5, d0
pub fn i_08f016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F018;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F018: move.w d0, $28(a6)
pub fn i_08f018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F01C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F01C: cmpi.b #$0, $3a(a6)
pub fn i_08f01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F022;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F022: bne.w $8f02a
pub fn i_08f022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F026;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F02A; return 10; }
    8
}
/// $08F026: neg.w $28(a6)
pub fn i_08f026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02A;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F02A: rts
pub fn i_08f02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollTarget_08f02c  ($08F02C, 20 B, enemies/critters_rings_08e4e6.s)
/// $08F02C: clr.w d0
pub fn i_08f02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F02E: move.b $99(a6), d0
pub fn i_08f02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F032;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F032: move.w $106f50.l, $72(a6)
pub fn i_08f032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F03A;
    cpu.ir = 0x3D79;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad3, v2 as u16);
    24
}
/// $08F03A: add.w d0, $72(a6)
pub fn i_08f03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F03E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F03E: rts
pub fn i_08f03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F040;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollReached_08f040  ($08F040, 10 B, enemies/critters_rings_08e4e6.s)
/// $08F040: cmpi.b #$0, $99(a6)
pub fn i_08f040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F046;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F046: bne.w $8f050
pub fn i_08f046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F04A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F050; return 10; }
    8
}
// ===== ClearXN_08f04a  ($08F04A, 6 B, thunks/ccr_helpers.c)
/// $08F04A: andi.b #$ee, ccr
pub fn i_08f04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F04E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F04E: rts
pub fn i_08f04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F050;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollReached_Cmp_08f050  ($08F050, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F050: move.w $106f50.l, d0
pub fn i_08f050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F056;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $08F056: cmp.w $72(a6), d0
pub fn i_08f056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F05A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F05A: bcs.w $8f068
pub fn i_08f05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F05E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F068; return 10; }
    8
}
// ===== SetXN_08f05e  ($08F05E, 6 B, thunks/ccr_helpers.c)
/// $08F05E: ori.b #$11, ccr
pub fn i_08f05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F062;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F062: rts
pub fn i_08f062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F064;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollReached_Yes_08f064  ($08F064, 4 B, enemies/critters_rings_08e4e6.s)
/// $08F064: bra.w $8f06e
pub fn i_08f064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F068;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F06E;
    10
}
// ===== ClearXN_08f068  ($08F068, 6 B, thunks/ccr_helpers.c)
/// $08F068: andi.b #$ee, ccr
pub fn i_08f068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F06C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F06C: rts
pub fn i_08f06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F06E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rts_08f06e  ($08F06E, 2 B, enemies/critters_rings_08e4e6.s)
/// $08F06E: rts
pub fn i_08f06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Snd_ByParam9A_Base_08f070  ($08F070, 12 B, enemies/critters_rings_08e4e6.s)
/// $08F070: clr.w d0
pub fn i_08f070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F072;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F072: move.b $9a(a6), d0
pub fn i_08f072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F076;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F076: addi.w #$fa, d0
pub fn i_08f076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F07A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFA, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F07A: move.w d0, d1
pub fn i_08f07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F07C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_08f07c  ($08F07C, 8 B, thunks/jsr_abs_thunks.c)
/// $08F07C: jsr $236e.l
pub fn i_08f07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F082);
    cpu.pc = 0x236E;
    16
}
/// $08F082: rts
pub fn i_08f082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F084;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Snd_ByParam9A_A_08f084  ($08F084, 68 B, enemies/critters_rings_08e4e6.s)
/// $08F084: cmpi.b #$0, $9a(a6)
pub fn i_08f084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F08A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F08A: bne.w $8f098
pub fn i_08f08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F08E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F098; return 10; }
    8
}
/// $08F08E: move.w #$14a, d1
pub fn i_08f08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F092;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F092: jsr $236e.l
pub fn i_08f092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F098);
    cpu.pc = 0x236E;
    16
}
/// $08F098: cmpi.b #$1, $9a(a6)
pub fn i_08f098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F09E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08F09E: bne.w $8f0ac
pub fn i_08f09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0AC; return 10; }
    8
}
/// $08F0A2: move.w #$f6, d1
pub fn i_08f0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0A6: jsr $236e.l
pub fn i_08f0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0AC);
    cpu.pc = 0x236E;
    16
}
/// $08F0AC: cmpi.b #$1, $9a(a6)
pub fn i_08f0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0B2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08F0B2: bne.w $8f0c4
pub fn i_08f0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0C4; return 10; }
    8
}
/// $08F0B6: move.w #$13d, d1
pub fn i_08f0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x13D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0BA: jsr $236e.l
pub fn i_08f0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0C0);
    cpu.pc = 0x236E;
    16
}
/// $08F0C0: bra.w $8f0ce
pub fn i_08f0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F0CE;
    10
}
/// $08F0C4: move.w #$14a, d1
pub fn i_08f0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_08f0c8  ($08F0C8, 8 B, thunks/jsr_abs_thunks.c)
/// $08F0C8: jsr $236e.l
pub fn i_08f0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0CE);
    cpu.pc = 0x236E;
    16
}
/// $08F0CE: rts
pub fn i_08f0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Snd_ByParam9A_B_08f0d0  ($08F0D0, 48 B, enemies/critters_rings_08e4e6.s)
/// $08F0D0: cmpi.b #$0, $9a(a6)
pub fn i_08f0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0D6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F0D6: bne.w $8f0e4
pub fn i_08f0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0E4; return 10; }
    8
}
/// $08F0DA: move.w #$149, d1
pub fn i_08f0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x149;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0DE: jsr $236e.l
pub fn i_08f0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0E4);
    cpu.pc = 0x236E;
    16
}
/// $08F0E4: cmpi.b #$1, $9a(a6)
pub fn i_08f0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0EA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08F0EA: bne.w $8f0fc
pub fn i_08f0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0FC; return 10; }
    8
}
/// $08F0EE: move.w #$f7, d1
pub fn i_08f0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0F2: jsr $236e.l
pub fn i_08f0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0F8);
    cpu.pc = 0x236E;
    16
}
/// $08F0F8: bra.w $8f106
pub fn i_08f0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F106;
    10
}
/// $08F0FC: move.w #$149, d1
pub fn i_08f0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F100;
    cpu.ir = 0x323C;
    let v1: u32 = 0x149;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_08f100  ($08F100, 8 B, thunks/jsr_abs_thunks.c)
/// $08F100: jsr $236e.l
pub fn i_08f100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F106);
    cpu.pc = 0x236E;
    16
}
/// $08F106: rts
pub fn i_08f106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F108;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prio_Set8018_08f108  ($08F108, 20 B, enemies/critters_rings_08e4e6.s)
/// $08F108: move.w #$8000, $38(a6)
pub fn i_08f108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F10E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F10E: andi.w #$ffe3, $38(a6)
pub fn i_08f10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F114;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08F114: ori.w #$18, $38(a6)
pub fn i_08f114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F11A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08F11A: rts
pub fn i_08f11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F11C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08f11c  ($08F11C, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F11C: movea.l $8(a6), a1
pub fn i_08f11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F120;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08F120: move.b $10(a6), d0
pub fn i_08f120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F124;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F124: cmp.b $10(a1), d0
pub fn i_08f124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F128;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08F128: bcs.w $8f132
pub fn i_08f128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F12C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F132; return 10; }
    8
}
// ===== ClearXN_08f12c  ($08F12C, 6 B, thunks/ccr_helpers.c)
/// $08F12C: andi.b #$ee, ccr
pub fn i_08f12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F130;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F130: rts
pub fn i_08f130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F132;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08f132  ($08F132, 6 B, thunks/ccr_helpers.c)
/// $08F132: ori.b #$11, ccr
pub fn i_08f132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F136;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F136: rts
pub fn i_08f136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F138;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ring_Reset_08f138  ($08F138, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F138: clr.w (a0)
pub fn i_08f138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F13A;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $08F13A: clr.w $2(a0)
pub fn i_08f13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F13E;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F13E: clr.w $4(a0)
pub fn i_08f13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F142;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F142: clr.w $6(a0)
pub fn i_08f142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F146;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F146: rts
pub fn i_08f146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ring_Compact_08f148  ($08F148, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F148: move.w $4(a0), (a0)
pub fn i_08f148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F14C;
    cpu.ir = 0x30A8;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v3 as u16);
    16
}
/// $08F14C: move.w $2(a0), $4(a0)
pub fn i_08f14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F152;
    cpu.ir = 0x3168;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F152: clr.w $6(a0)
pub fn i_08f152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F156;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F156: rts
pub fn i_08f156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F158;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rings_InitAll_08f158  ($08F158, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F158: lea.l $10e2f2.l, a0
pub fn i_08f158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F15E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F15E: bsr.b $8f138
pub fn i_08f15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F160;
    cpu.ir = 0x61D8;
    cpu.push32(bus, 0x8F160);
    cpu.pc = 0x8F138;
    18
}
/// $08F160: lea.l $10e33a.l, a0
pub fn i_08f160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F166;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F166: bsr.b $8f138
pub fn i_08f166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F168;
    cpu.ir = 0x61D0;
    cpu.push32(bus, 0x8F168);
    cpu.pc = 0x8F138;
    18
}
/// $08F168: lea.l $10e362.l, a0
pub fn i_08f168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F16E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F16E: bsr.b $8f138
pub fn i_08f16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F170;
    cpu.ir = 0x61C8;
    cpu.push32(bus, 0x8F170);
    cpu.pc = 0x8F138;
    18
}
/// $08F170: rts
pub fn i_08f170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rings_CompactAll_08f172  ($08F172, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F172: lea.l $10e2f2.l, a0
pub fn i_08f172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F178;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F178: bsr.b $8f148
pub fn i_08f178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F17A;
    cpu.ir = 0x61CE;
    cpu.push32(bus, 0x8F17A);
    cpu.pc = 0x8F148;
    18
}
/// $08F17A: lea.l $10e33a.l, a0
pub fn i_08f17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F180;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F180: bsr.b $8f148
pub fn i_08f180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F182;
    cpu.ir = 0x61C6;
    cpu.push32(bus, 0x8F182);
    cpu.pc = 0x8F148;
    18
}
/// $08F182: lea.l $10e362.l, a0
pub fn i_08f182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F188;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F188: bsr.b $8f148
pub fn i_08f188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F18A;
    cpu.ir = 0x61BE;
    cpu.push32(bus, 0x8F18A);
    cpu.pc = 0x8F148;
    18
}
/// $08F18A: rts
pub fn i_08f18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F18C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Zone_Tmpl152_08f18c  ($08F18C, 82 B, enemies/critters_rings_08e4e6.s)
/// $08F18C: move.w $22(a6), d0
pub fn i_08f18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F190;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F190: add.w $106f50.l, d0
pub fn i_08f190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F196;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F196: move.w d0, $22(a6)
pub fn i_08f196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F19A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F19A: clr.w d1
pub fn i_08f19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F19C;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F19C: move.b $98(a6), d1
pub fn i_08f19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A0;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F1A0: lsl.w #$4, d1
pub fn i_08f1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A2;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08F1A2: add.w d1, d0
pub fn i_08f1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A4;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F1A4: move.w d0, $70(a6)
pub fn i_08f1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F1A8: move.w $24(a6), d0
pub fn i_08f1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F1AC: subi.w #$200, d0
pub fn i_08f1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1B0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F1B0: neg.w d0
pub fn i_08f1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1B2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F1B2: add.w $106f54.l, d0
pub fn i_08f1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1B8;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F1B8: move.w d0, $24(a6)
pub fn i_08f1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F1BC: clr.w d1
pub fn i_08f1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1BE;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F1BE: move.b $99(a6), d1
pub fn i_08f1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1C2;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F1C2: lsl.w #$4, d1
pub fn i_08f1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1C4;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08F1C4: add.w d1, d0
pub fn i_08f1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1C6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F1C6: move.w d0, $72(a6)
pub fn i_08f1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1CA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F1CA: lea.l $8f1d0(pc), a1
pub fn i_08f1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8F1D0;
    16
}
/// $08F1CE: move.l a1, (a6)
pub fn i_08f1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08F1D0: move.w $70(a6), d0
pub fn i_08f1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1D4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F1D4: cmp.w $106f50.l, d0
pub fn i_08f1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1DA;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $08F1DA: bcc.w $8f1ec
pub fn i_08f1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F1EC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08f1de  ($08F1DE, 14 B, kernel/jsr5b6_jmp518.c)
/// $08F1DE: jsr $5b6.l
pub fn i_08f1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F1E4);
    cpu.pc = 0x5B6;
    16
}
/// $08F1E4: jmp $518.l
pub fn i_08f1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08F1EA: rts
pub fn i_08f1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_Push_08f1ec  ($08F1EC, 72 B, enemies/critters_rings_08e4e6.s)
/// $08F1EC: lea.l $10e2f2.l, a0
pub fn i_08f1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F1F2: lea.l $8(a0), a1
pub fn i_08f1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1F6;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F1F6: adda.w $2(a0), a1
pub fn i_08f1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1FA;
    cpu.ir = 0xD2E8;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $08F1FA: move.w $22(a6), (a1)
pub fn i_08f1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1FE;
    cpu.ir = 0x32AE;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v3 as u16);
    16
}
/// $08F1FE: move.w $24(a6), $2(a1)
pub fn i_08f1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F204;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F204: move.w $70(a6), $4(a1)
pub fn i_08f204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F20A;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F20A: move.w $72(a6), $6(a1)
pub fn i_08f20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F210;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x6);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F210: cmpi.w #$4, $6(a0)
pub fn i_08f210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F216;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F216: bcc.w $8f232
pub fn i_08f216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F21A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F232; return 10; }
    8
}
/// $08F21A: move.w $2(a0), d7
pub fn i_08f21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F21E;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F21E: addq.w #$8, d7
pub fn i_08f21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F220;
    cpu.ir = 0x5047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F220: cmpi.w #$40, d7
pub fn i_08f220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F224;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F224: bcs.w $8f22a
pub fn i_08f224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F228;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F22A; return 10; }
    8
}
/// $08F228: clr.w d7
pub fn i_08f228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F22A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F22A: move.w d7, $2(a0)
pub fn i_08f22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F22E;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F22E: addq.w #$1, $6(a0)
pub fn i_08f22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F232;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F232: rts
pub fn i_08f232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F234;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_PushOffset_08f234  ($08F234, 108 B, enemies/critters_rings_08e4e6.s)
/// $08F234: move.w $22(a6), d0
pub fn i_08f234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F238;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F238: move.w $24(a6), d1
pub fn i_08f238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F23C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F23C: add.w $106f50.l, d0
pub fn i_08f23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F242;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F242: subi.w #$200, d1
pub fn i_08f242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F246;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F246: neg.w d1
pub fn i_08f246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F248;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F248: add.w $106f54.l, d1
pub fn i_08f248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F24E;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F24E: move.w d0, d2
pub fn i_08f24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F250;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F250: move.w d1, d3
pub fn i_08f250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F252;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F252: add.w (a0), d0
pub fn i_08f252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F254;
    cpu.ir = 0xD050;
    let v2 = bus.read16(cpu.a[0]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08F254: add.w $2(a0), d2
pub fn i_08f254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F258;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F258: add.w $4(a0), d1
pub fn i_08f258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F25C;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F25C: add.w $6(a0), d3
pub fn i_08f25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F260;
    cpu.ir = 0xD668;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F260: lea.l $10e2f2.l, a0
pub fn i_08f260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F266: lea.l $8(a0), a1
pub fn i_08f266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F26A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F26A: adda.w $2(a0), a1
pub fn i_08f26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F26E;
    cpu.ir = 0xD2E8;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $08F26E: move.w d0, (a1)
pub fn i_08f26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F270;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $08F270: move.w d1, $2(a1)
pub fn i_08f270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F274;
    cpu.ir = 0x3341;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F274: move.w d2, $4(a1)
pub fn i_08f274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F278;
    cpu.ir = 0x3342;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F278: move.w d3, $6(a1)
pub fn i_08f278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F27C;
    cpu.ir = 0x3343;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F27C: cmpi.w #$4, $6(a0)
pub fn i_08f27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F282;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F282: bcc.w $8f29e
pub fn i_08f282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F286;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F29E; return 10; }
    8
}
/// $08F286: move.w $2(a0), d7
pub fn i_08f286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F28A;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F28A: addq.w #$8, d7
pub fn i_08f28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F28C;
    cpu.ir = 0x5047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F28C: cmpi.w #$40, d7
pub fn i_08f28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F290;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F290: bcs.w $8f296
pub fn i_08f290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F294;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F296; return 10; }
    8
}
/// $08F294: clr.w d7
pub fn i_08f294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F296;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F296: move.w d7, $2(a0)
pub fn i_08f296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F29A;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F29A: addq.w #$1, $6(a0)
pub fn i_08f29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F29E;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F29E: rts
pub fn i_08f29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_HitTest_08f2a0  ($08F2A0, 78 B, enemies/critters_rings_08e4e6.s)
/// $08F2A0: move.w $22(a6), d0
pub fn i_08f2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2A4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F2A4: add.w $106f50.l, d0
pub fn i_08f2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2AA;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F2AA: move.w $24(a6), d1
pub fn i_08f2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2AE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F2AE: subi.w #$200, d1
pub fn i_08f2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2B2;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F2B2: neg.w d1
pub fn i_08f2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2B4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F2B4: add.w $106f54.l, d1
pub fn i_08f2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2BA;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F2BA: lea.l $10e2f2.l, a0
pub fn i_08f2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F2C0: lea.l $8(a0), a1
pub fn i_08f2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C4;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F2C4: move.w (a0), d7
pub fn i_08f2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C6;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F2C6: cmp.w $4(a0), d7
pub fn i_08f2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2CA;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F2CA: beq.w $8f302
pub fn i_08f2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F302; return 10; }
    8
}
/// $08F2CE: cmp.w (a1, d7.w), d0
pub fn i_08f2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2D2;
    cpu.ir = 0xB071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2D2: bcs.w $8f2f4
pub fn i_08f2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2D6: cmp.w $4(a1, d7.w), d0
pub fn i_08f2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2DA;
    cpu.ir = 0xB071;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2DA: bhi.w $8f2f4
pub fn i_08f2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2DE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2DE: cmp.w $2(a1, d7.w), d1
pub fn i_08f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2E2;
    cpu.ir = 0xB271;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2E2: bcs.w $8f2f4
pub fn i_08f2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2E6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2E6: cmp.w $6(a1, d7.w), d1
pub fn i_08f2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2EA;
    cpu.ir = 0xB271;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2EA: bhi.w $8f2f4
pub fn i_08f2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2EE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8F2F4; return 10; }
    8
}
// ===== SetC_08f2ee  ($08F2EE, 6 B, thunks/ccr_helpers.c)
/// $08F2EE: ori.b #$1, ccr
pub fn i_08f2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F2F2: rts
pub fn i_08f2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_HitTest_Next_08f2f4  ($08F2F4, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F2F4: addq.w #$8, d7
pub fn i_08f2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F6;
    cpu.ir = 0x5047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F2F6: cmpi.w #$40, d7
pub fn i_08f2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2FA;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F2FA: bcs.w $8f300
pub fn i_08f2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2FE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F300; return 10; }
    8
}
/// $08F2FE: clr.w d7
pub fn i_08f2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F300;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F300: bra.b $8f2c6
pub fn i_08f300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F302;
    cpu.ir = 0x60C4;
    cpu.pc = 0x8F2C6;
    10
}
// ===== ClearC_08f302  ($08F302, 6 B, thunks/ccr_helpers.c)
/// $08F302: andi.b #$fe, ccr
pub fn i_08f302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F306;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F306: rts
pub fn i_08f306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F308;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_PushCapped_08F308  ($08F308, 60 B, kernel/pos_ring_push_capped_08f308.s)
/// $08F308: lea.l $10e33a.l, a0
pub fn i_08f308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F30E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F30E: lea.l $8(a0), a1
pub fn i_08f30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F312;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F312: adda.w $2(a0), a1
pub fn i_08f312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F316;
    cpu.ir = 0xD2E8;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $08F316: move.w $22(a6), (a1)
pub fn i_08f316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F31A;
    cpu.ir = 0x32AE;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v3 as u16);
    16
}
/// $08F31A: move.w $24(a6), $2(a1)
pub fn i_08f31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F320;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F320: cmpi.w #$4, $6(a0)
pub fn i_08f320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F326;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F326: bcc.w $8f342
pub fn i_08f326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F32A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F342; return 10; }
    8
}
/// $08F32A: move.w $2(a0), d7
pub fn i_08f32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F32E;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F32E: addq.w #$4, d7
pub fn i_08f32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F330;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F330: cmpi.w #$20, d7
pub fn i_08f330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F334;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $08F334: bcs.w $8f33a
pub fn i_08f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F338;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F33A; return 10; }
    8
}
/// $08F338: clr.w d7
pub fn i_08f338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F33A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F33A: move.w d7, $2(a0)
pub fn i_08f33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F33E;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F33E: addq.w #$1, $6(a0)
pub fn i_08f33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F342;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F342: rts
pub fn i_08f342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F344;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_FindNear_08f344  ($08F344, 72 B, enemies/critters_rings_08e4e6.s)
/// $08F344: lea.l $10e33a.l, a0
pub fn i_08f344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F34A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F34A: lea.l $8(a0), a1
pub fn i_08f34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F34E;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F34E: move.w (a0), d7
pub fn i_08f34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F350;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F350: cmp.w $4(a0), d7
pub fn i_08f350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F354;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F354: beq.w $8f3a0
pub fn i_08f354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F358;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F3A0; return 10; }
    8
}
/// $08F358: move.w (a1, d7.w), d0
pub fn i_08f358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F35C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F35C: move.w $2(a1, d7.w), d1
pub fn i_08f35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F360;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F360: sub.w $22(a6), d0
pub fn i_08f360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F364;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F364: smi.b d2
pub fn i_08f364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F366;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $08F366: sub.w $24(a6), d1
pub fn i_08f366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F36A;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F36A: addi.w #$40, d0
pub fn i_08f36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F36E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F36E: addi.w #$30, d1
pub fn i_08f36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F372;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F372: cmpi.w #$80, d0
pub fn i_08f372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F376;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $08F376: bcc.w $8f392
pub fn i_08f376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F37A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F392; return 10; }
    8
}
/// $08F37A: cmpi.w #$60, d1
pub fn i_08f37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F37E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $08F37E: bcc.w $8f392
pub fn i_08f37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F382;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F392; return 10; }
    8
}
/// $08F382: btst.b #$0, $3a(a6)
pub fn i_08f382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F388;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08F388: seq.b d0
pub fn i_08f388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F38A;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $08F38A: eor.b d2, d0
pub fn i_08f38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F38C;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetC_08f38c  ($08F38C, 6 B, thunks/ccr_helpers.c)
/// $08F38C: ori.b #$1, ccr
pub fn i_08f38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F390;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F390: rts
pub fn i_08f390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_FindNear_Next_08f392  ($08F392, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F392: addq.w #$4, d7
pub fn i_08f392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F394;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F394: cmpi.w #$20, d7
pub fn i_08f394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F398;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $08F398: bcs.w $8f39e
pub fn i_08f398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F39C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F39E; return 10; }
    8
}
/// $08F39C: clr.w d7
pub fn i_08f39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F39E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F39E: bra.b $8f350
pub fn i_08f39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A0;
    cpu.ir = 0x60B0;
    cpu.pc = 0x8F350;
    10
}
// ===== ClearC_08f3a0  ($08F3A0, 6 B, thunks/ccr_helpers.c)
/// $08F3A0: andi.b #$fe, ccr
pub fn i_08f3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F3A4: rts
pub fn i_08f3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_NewId_08f3a6  ($08F3A6, 24 B, enemies/critters_rings_08e4e6.s)
/// $08F3A6: move.b $10e2f0.l, d0
pub fn i_08f3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3AC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E2F0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F3AC: addq.b #$1, d0
pub fn i_08f3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3AE;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F3AE: andi.b #$3f, d0
pub fn i_08f3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3B2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08F3B2: addi.b #$40, d0
pub fn i_08f3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3B6;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08F3B6: move.b d0, $10e2f0.l
pub fn i_08f3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3BC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2F0, v1 as u8);
    16
}
/// $08F3BC: rts
pub fn i_08f3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_Register_08f3be  ($08F3BE, 178 B, enemies/critters_rings_08e4e6.s)
/// $08F3BE: lea.l $10e362.l, a0
pub fn i_08f3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F3C4: lea.l $8(a0), a1
pub fn i_08f3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3C8;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F3C8: move.b #$fe, d3
pub fn i_08f3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3CC;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F3CC: move.w (a0), d7
pub fn i_08f3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3CE;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F3CE: cmp.w $4(a0), d7
pub fn i_08f3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3D2;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F3D2: beq.w $8f3f8
pub fn i_08f3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F3F8; return 10; }
    8
}
/// $08F3D6: cmp.b $4(a1, d7.w), d2
pub fn i_08f3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3DA;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F3DA: bne.w $8f3ea
pub fn i_08f3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F3EA; return 10; }
    8
}
/// $08F3DE: move.b $5(a1, d7.w), d4
pub fn i_08f3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3E2;
    cpu.ir = 0x1831;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F3E2: bmi.w $8f3ea
pub fn i_08f3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3E6;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x8F3EA; return 10; }
    8
}
/// $08F3E6: move.b #$ff, d3
pub fn i_08f3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3EA;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F3EA: addq.w #$6, d7
pub fn i_08f3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3EC;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F3EC: cmpi.w #$30, d7
pub fn i_08f3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F0;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F3F0: bcs.w $8f3f6
pub fn i_08f3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F3F6; return 10; }
    8
}
/// $08F3F4: clr.w d7
pub fn i_08f3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F6;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F3F6: bra.b $8f3ce
pub fn i_08f3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F8;
    cpu.ir = 0x60D6;
    cpu.pc = 0x8F3CE;
    10
}
/// $08F3F8: move.w $22(a6), d5
pub fn i_08f3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3FC;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F3FC: subi.w #$10, d5
pub fn i_08f3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F400;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F400: subi.w #$130, d5
pub fn i_08f400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F404;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x130, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F404: bcc.w $8f46e
pub fn i_08f404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F408;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F46E; return 10; }
    8
}
/// $08F408: cmp.w $2(a0), d7
pub fn i_08f408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F40C;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F40C: beq.w $8f43c
pub fn i_08f40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F410;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F43C; return 10; }
    8
}
/// $08F410: cmp.b $4(a1, d7.w), d2
pub fn i_08f410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F414;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F414: bne.w $8f42e
pub fn i_08f414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F418;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F42E; return 10; }
    8
}
/// $08F418: move.b $5(a1, d7.w), d5
pub fn i_08f418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F41C;
    cpu.ir = 0x1A31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F41C: bpl.w $8f42c
pub fn i_08f41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F420;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8F42C; return 10; }
    8
}
/// $08F420: move.w d0, (a1, d7.w)
pub fn i_08f420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F424;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $08F424: move.w d1, $2(a1, d7.w)
pub fn i_08f424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F428;
    cpu.ir = 0x3381;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $08F428: move.b d4, $5(a1, d7.w)
pub fn i_08f428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F42C;
    cpu.ir = 0x1384;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F42C: rts
pub fn i_08f42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F42E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08F42E: addq.w #$6, d7
pub fn i_08f42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F430;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F430: cmpi.w #$30, d7
pub fn i_08f430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F434;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F434: bcs.w $8f43a
pub fn i_08f434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F438;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F43A; return 10; }
    8
}
/// $08F438: clr.w d7
pub fn i_08f438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F43A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F43A: bra.b $8f408
pub fn i_08f43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F43C;
    cpu.ir = 0x60CC;
    cpu.pc = 0x8F408;
    10
}
/// $08F43C: adda.w d7, a1
pub fn i_08f43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F43E;
    cpu.ir = 0xD2C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $08F43E: move.w d0, (a1)
pub fn i_08f43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F440;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $08F440: move.w d1, $2(a1)
pub fn i_08f440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F444;
    cpu.ir = 0x3341;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F444: move.b d2, $4(a1)
pub fn i_08f444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F448;
    cpu.ir = 0x1342;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F448: move.b d3, $5(a1)
pub fn i_08f448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F44C;
    cpu.ir = 0x1343;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F44C: cmpi.w #$4, $6(a0)
pub fn i_08f44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F452;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F452: bcc.w $8f46e
pub fn i_08f452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F456;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F46E; return 10; }
    8
}
/// $08F456: move.w $2(a0), d7
pub fn i_08f456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F45A;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F45A: addq.w #$6, d7
pub fn i_08f45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F45C;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F45C: cmpi.w #$30, d7
pub fn i_08f45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F460;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F460: bcs.w $8f466
pub fn i_08f460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F464;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F466; return 10; }
    8
}
/// $08F464: clr.w d7
pub fn i_08f464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F466;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F466: move.w d7, $2(a0)
pub fn i_08f466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F46A;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F46A: addq.w #$1, $6(a0)
pub fn i_08f46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F46E;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F46E: rts
pub fn i_08f46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F470;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_FindPending_08f470  ($08F470, 108 B, enemies/critters_rings_08e4e6.s)
/// $08F470: lea.l $10e362.l, a0
pub fn i_08f470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F476;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F476: lea.l $8(a0), a1
pub fn i_08f476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F47A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F47A: move.w (a0), d7
pub fn i_08f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F47C;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F47C: cmpa.l #$100440, a6
pub fn i_08f47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F482;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F482: beq.w $8f4bc
pub fn i_08f482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F486;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4BC; return 10; }
    8
}
/// $08F486: cmpa.l #$1004e0, a6
pub fn i_08f486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F48C;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F48C: beq.w $8f4bc
pub fn i_08f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F490;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4BC; return 10; }
    8
}
/// $08F490: cmp.w $4(a0), d7
pub fn i_08f490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F494;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F494: beq.w $8f4b8
pub fn i_08f494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F498;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4B8; return 10; }
    8
}
/// $08F498: tst.b $4(a1, d7.w)
pub fn i_08f498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F49C;
    cpu.ir = 0x4A31;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $08F49C: bpl.w $8f4aa
pub fn i_08f49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4A0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8F4AA; return 10; }
    8
}
/// $08F4A0: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4A6;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F4A6: beq.w $8f4e2
pub fn i_08f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4E2; return 10; }
    8
}
/// $08F4AA: addq.w #$6, d7
pub fn i_08f4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4AC;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4AC: cmpi.w #$30, d7
pub fn i_08f4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B0;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F4B0: bcs.w $8f4b6
pub fn i_08f4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F4B6; return 10; }
    8
}
/// $08F4B4: clr.w d7
pub fn i_08f4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B6;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4B6: bra.b $8f490
pub fn i_08f4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B8;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F490;
    10
}
/// $08F4B8: bra.w $8f4dc
pub fn i_08f4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F4DC;
    10
}
/// $08F4BC: cmp.w $4(a0), d7
pub fn i_08f4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4C0;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F4C0: beq.w $8f4dc
pub fn i_08f4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4DC; return 10; }
    8
}
/// $08F4C4: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4CA;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F4CA: beq.w $8f4e2
pub fn i_08f4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4E2; return 10; }
    8
}
/// $08F4CE: addq.w #$6, d7
pub fn i_08f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D0;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4D0: cmpi.w #$30, d7
pub fn i_08f4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D4;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F4D4: bcs.w $8f4da
pub fn i_08f4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F4DA; return 10; }
    8
}
/// $08F4D8: clr.w d7
pub fn i_08f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4DA;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4DA: bra.b $8f4bc
pub fn i_08f4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4DC;
    cpu.ir = 0x60E0;
    cpu.pc = 0x8F4BC;
    10
}
// ===== ClearC_08f4dc  ($08F4DC, 6 B, thunks/ccr_helpers.c)
/// $08F4DC: andi.b #$fe, ccr
pub fn i_08f4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F4E0: rts
pub fn i_08f4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_InRange_08f4e2  ($08F4E2, 56 B, enemies/critters_rings_08e4e6.s)
/// $08F4E2: lea.l (a1, d7.w), a2
pub fn i_08f4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E6;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $08F4E6: move.w (a2), d0
pub fn i_08f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E8;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F4E8: move.w $2(a2), d1
pub fn i_08f4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4EC;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F4EC: moveq #$ff, d2
pub fn i_08f4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4EE;
    cpu.ir = 0x74FF;
    cpu.d[2] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F4EE: move.b $4(a2), d2
pub fn i_08f4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F2;
    cpu.ir = 0x142A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F4F2: move.w $22(a6), d4
pub fn i_08f4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F6;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F4F6: sub.w d0, d4
pub fn i_08f4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F8;
    cpu.ir = 0x9840;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F4F8: addi.w #$18, d4
pub fn i_08f4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4FC;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F4FC: cmpi.w #$30, d4
pub fn i_08f4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F500;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F500: bcc.w $8f51a
pub fn i_08f500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F504;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F51A; return 10; }
    8
}
/// $08F504: move.w $24(a6), d4
pub fn i_08f504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F508;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F508: sub.w d1, d4
pub fn i_08f508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F50A;
    cpu.ir = 0x9841;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F50A: addi.w #$20, d4
pub fn i_08f50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F50E;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F50E: cmpi.w #$40, d4
pub fn i_08f50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F512;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F512: bcc.w $8f51a
pub fn i_08f512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F516;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F51A; return 10; }
    8
}
/// $08F516: andi.w #$ff, d2
pub fn i_08f516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F51A;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetC_08f51a  ($08F51A, 6 B, thunks/ccr_helpers.c)
/// $08F51A: ori.b #$1, ccr
pub fn i_08f51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F51E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F51E: rts
pub fn i_08f51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F520;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_08f520  ($08F520, 84 B, enemies/critters_rings_08e4e6.s)
/// $08F520: lea.l $10e362.l, a0
pub fn i_08f520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F526;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F526: lea.l $8(a0), a1
pub fn i_08f526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F52A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F52A: move.w (a0), d7
pub fn i_08f52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F52C;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F52C: move.b #$0, d3
pub fn i_08f52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F530;
    cpu.ir = 0x163C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F530: cmpa.l #$100440, a6
pub fn i_08f530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F536;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F536: beq.w $8f54c
pub fn i_08f536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F53A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F54C; return 10; }
    8
}
/// $08F53A: move.b #$1, d3
pub fn i_08f53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F53E;
    cpu.ir = 0x163C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F53E: cmpa.l #$1004e0, a6
pub fn i_08f53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F544;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F544: beq.w $8f54c
pub fn i_08f544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F548;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F54C; return 10; }
    8
}
/// $08F548: move.b #$2, d3
pub fn i_08f548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F54C;
    cpu.ir = 0x163C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F54C: cmp.w $4(a0), d7
pub fn i_08f54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F550;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F550: beq.w $8f574
pub fn i_08f550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F554;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F574; return 10; }
    8
}
/// $08F554: cmp.b $4(a1, d7.w), d2
pub fn i_08f554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F558;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F558: bne.w $8f566
pub fn i_08f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F55C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F566; return 10; }
    8
}
/// $08F55C: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F562;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F562: beq.w $8f57a
pub fn i_08f562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F566;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F57A; return 10; }
    8
}
/// $08F566: addq.w #$6, d7
pub fn i_08f566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F568;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F568: cmpi.w #$30, d7
pub fn i_08f568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F56C;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F56C: bcs.w $8f572
pub fn i_08f56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F570;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F572; return 10; }
    8
}
/// $08F570: clr.w d7
pub fn i_08f570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F572;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F572: bra.b $8f54c
pub fn i_08f572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F574;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F54C;
    10
}
// ===== ClearC_08f574  ($08F574, 6 B, thunks/ccr_helpers.c)
/// $08F574: andi.b #$fe, ccr
pub fn i_08f574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F578;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F578: rts
pub fn i_08f578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F57A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_Mark_08f57a  ($08F57A, 30 B, enemies/critters_rings_08e4e6.s)
/// $08F57A: move.b #$ff, $5(a1, d7.w)
pub fn i_08f57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F580;
    cpu.ir = 0x13BC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F580: move.w $4(a0), d7
pub fn i_08f580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F584;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F584: cmp.w $2(a0), d7
pub fn i_08f584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F588;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F588: beq.w $8f5ac
pub fn i_08f588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F58C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F5AC; return 10; }
    8
}
/// $08F58C: cmp.b $4(a1, d7.w), d2
pub fn i_08f58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F590;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F590: bne.w $8f59e
pub fn i_08f590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F594;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F59E; return 10; }
    8
}
/// $08F594: move.b d3, $5(a1, d7.w)
pub fn i_08f594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F598;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
// ===== SetC_08f598  ($08F598, 6 B, thunks/ccr_helpers.c)
/// $08F598: ori.b #$1, ccr
pub fn i_08f598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F59C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F59C: rts
pub fn i_08f59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F59E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_Next_08f59e  ($08F59E, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F59E: addq.w #$6, d7
pub fn i_08f59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A0;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5A0: cmpi.w #$30, d7
pub fn i_08f5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A4;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F5A4: bcs.w $8f5aa
pub fn i_08f5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F5AA; return 10; }
    8
}
/// $08F5A8: clr.w d7
pub fn i_08f5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5AA;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5AA: bra.b $8f584
pub fn i_08f5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5AC;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F584;
    10
}
// ===== TargetRing_ClaimById_Append_08f5ac  ($08F5AC, 42 B, enemies/critters_rings_08e4e6.s)
/// $08F5AC: move.b d2, $4(a1, d7.w)
pub fn i_08f5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5B0;
    cpu.ir = 0x1382;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F5B0: move.b d3, $5(a1, d7.w)
pub fn i_08f5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5B4;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F5B4: cmpi.w #$4, $6(a0)
pub fn i_08f5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5BA;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F5BA: bcc.w $8f5d6
pub fn i_08f5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F5D6; return 10; }
    8
}
/// $08F5BE: move.w $2(a0), d7
pub fn i_08f5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C2;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F5C2: addq.w #$6, d7
pub fn i_08f5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C4;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5C4: cmpi.w #$30, d7
pub fn i_08f5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C8;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F5C8: bcs.w $8f5ce
pub fn i_08f5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F5CE; return 10; }
    8
}
/// $08F5CC: clr.w d7
pub fn i_08f5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5CE;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5CE: move.w d7, $2(a0)
pub fn i_08f5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5D2;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F5D2: addq.w #$1, $6(a0)
pub fn i_08f5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5D6;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetC_08f5d6  ($08F5D6, 6 B, thunks/ccr_helpers.c)
/// $08F5D6: ori.b #$1, ccr
pub fn i_08f5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5DA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F5DA: rts
pub fn i_08f5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimByKey_08f5dc  ($08F5DC, 50 B, enemies/critters_rings_08e4e6.s)
/// $08F5DC: lea.l $10e362.l, a0
pub fn i_08f5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F5E2: lea.l $8(a0), a1
pub fn i_08f5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5E6;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F5E6: move.w (a0), d7
pub fn i_08f5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5E8;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F5E8: cmp.w $4(a0), d7
pub fn i_08f5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5EC;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F5EC: beq.w $8f60e
pub fn i_08f5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F60E; return 10; }
    8
}
/// $08F5F0: cmp.b $4(a1, d7.w), d0
pub fn i_08f5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5F4;
    cpu.ir = 0xB031;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F5F4: bne.w $8f600
pub fn i_08f5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F600; return 10; }
    8
}
/// $08F5F8: tst.b $5(a1, d7.w)
pub fn i_08f5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5FC;
    cpu.ir = 0x4A31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $08F5FC: bpl.w $8f614
pub fn i_08f5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F600;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8F614; return 10; }
    8
}
/// $08F600: addq.w #$6, d7
pub fn i_08f600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F602;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F602: cmpi.w #$30, d7
pub fn i_08f602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F606;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F606: bcs.w $8f60c
pub fn i_08f606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F60A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F60C; return 10; }
    8
}
/// $08F60A: clr.w d7
pub fn i_08f60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F60C;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F60C: bra.b $8f5e8
pub fn i_08f60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F60E;
    cpu.ir = 0x60DA;
    cpu.pc = 0x8F5E8;
    10
}
// ===== ClearC_08f60e  ($08F60E, 6 B, thunks/ccr_helpers.c)
/// $08F60E: andi.b #$fe, ccr
pub fn i_08f60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F612;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F612: rts
pub fn i_08f612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F614;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimByKey_Found_08f614  ($08F614, 66 B, enemies/critters_rings_08e4e6.s)
/// $08F614: move.b d0, d2
pub fn i_08f614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F616;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F616: move.w (a1, d7.w), d0
pub fn i_08f616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F61A;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F61A: move.w $2(a1, d7.w), d1
pub fn i_08f61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F61E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F61E: move.b #$0, d3
pub fn i_08f61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F622;
    cpu.ir = 0x163C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F622: cmpa.l #$100440, a6
pub fn i_08f622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F628;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F628: beq.w $8f63e
pub fn i_08f628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F62C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F63E; return 10; }
    8
}
/// $08F62C: move.b #$1, d3
pub fn i_08f62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F630;
    cpu.ir = 0x163C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F630: cmpa.l #$1004e0, a6
pub fn i_08f630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F636;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F636: beq.w $8f63e
pub fn i_08f636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F63A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F63E; return 10; }
    8
}
/// $08F63A: move.b #$2, d3
pub fn i_08f63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F63E;
    cpu.ir = 0x163C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F63E: move.w $4(a0), d7
pub fn i_08f63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F642;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F642: cmp.w $2(a0), d7
pub fn i_08f642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F646;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F646: beq.w $8f66a
pub fn i_08f646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F64A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F66A; return 10; }
    8
}
/// $08F64A: cmp.b $4(a1, d7.w), d2
pub fn i_08f64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F64E;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F64E: bne.w $8f65c
pub fn i_08f64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F652;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F65C; return 10; }
    8
}
/// $08F652: move.b d3, $5(a1, d7.w)
pub fn i_08f652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F656;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
// ===== SetC_08f656  ($08F656, 6 B, thunks/ccr_helpers.c)
/// $08F656: ori.b #$1, ccr
pub fn i_08f656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F65A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F65A: rts
pub fn i_08f65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F65C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimByKey_Next_08f65c  ($08F65C, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F65C: addq.w #$6, d7
pub fn i_08f65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F65E;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F65E: cmpi.w #$30, d7
pub fn i_08f65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F662;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F662: bcs.w $8f668
pub fn i_08f662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F666;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F668; return 10; }
    8
}
/// $08F666: clr.w d7
pub fn i_08f666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F668;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F668: bra.b $8f642
pub fn i_08f668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F66A;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F642;
    10
}
// ===== TargetRing_ClaimByKey_Append_08f66a  ($08F66A, 44 B, enemies/critters_rings_08e4e6.s)
/// $08F66A: move.b d2, $4(a1, d7.w)
pub fn i_08f66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F66E;
    cpu.ir = 0x1382;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F66E: move.b #$ff, $5(a1, d7.w)
pub fn i_08f66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F674;
    cpu.ir = 0x13BC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F674: cmpi.w #$4, $6(a0)
pub fn i_08f674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F67A;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F67A: bcc.w $8f696
pub fn i_08f67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F67E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F696; return 10; }
    8
}
/// $08F67E: move.w $2(a0), d7
pub fn i_08f67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F682;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F682: addq.w #$6, d7
pub fn i_08f682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F684;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F684: cmpi.w #$30, d7
pub fn i_08f684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F688;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F688: bcs.w $8f68e
pub fn i_08f688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F68C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F68E; return 10; }
    8
}
/// $08F68C: clr.w d7
pub fn i_08f68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F68E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F68E: move.w d7, $2(a0)
pub fn i_08f68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F692;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F692: addq.w #$1, $6(a0)
pub fn i_08f692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F696;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetC_08f696  ($08F696, 6 B, thunks/ccr_helpers.c)
/// $08F696: ori.b #$1, ccr
pub fn i_08f696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F69A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F69A: rts
pub fn i_08f69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F69C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_SndByState_08f69c  ($08F69C, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F69C: move.w #$134, d1
pub fn i_08f69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6A0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x134;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F6A0: tst.b d4
pub fn i_08f6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6A2;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F6A2: beq.w $8f6b4
pub fn i_08f6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F6B4; return 10; }
    8
}
/// $08F6A6: move.w #$152, d1
pub fn i_08f6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x152;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F6AA: subq.b #$1, d4
pub fn i_08f6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6AC;
    cpu.ir = 0x5304;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F6AC: beq.w $8f6b4
pub fn i_08f6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F6B4; return 10; }
    8
}
/// $08F6B0: move.w #$e, d1
pub fn i_08f6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F6B4: rts
pub fn i_08f6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08f6b6  ($08F6B6, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F6B6: movea.l $8(a6), a1
pub fn i_08f6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6BA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08F6BA: move.b $10(a6), d0
pub fn i_08f6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F6BE: cmp.b $10(a1), d0
pub fn i_08f6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6C2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08F6C2: bcs.w $8f6cc
pub fn i_08f6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F6CC; return 10; }
    8
}
// ===== ClearXN_08f6c6  ($08F6C6, 6 B, thunks/ccr_helpers.c)
/// $08F6C6: andi.b #$ee, ccr
pub fn i_08f6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F6CA: rts
pub fn i_08f6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08f6cc  ($08F6CC, 6 B, thunks/ccr_helpers.c)
/// $08F6CC: ori.b #$11, ccr
pub fn i_08f6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6D0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F6D0: rts
pub fn i_08f6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_MaskF0F0_08f6d2  ($08F6D2, 8 B, mission/gameover_continue_08f6d2.s)
/// $08F6D2: move.w #$f0f0, $96(a6)
pub fn i_08f6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0F0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F6D8: rts
pub fn i_08f6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_MaskCurF0_08f6da  ($08F6DA, 24 B, mission/gameover_continue_08f6d2.s)
/// $08F6DA: move.b $106f28.l, d0
pub fn i_08f6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F6E0: not.b d0
pub fn i_08f6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E2;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F6E2: andi.w #$1, d0
pub fn i_08f6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6E6: addi.w #$96, d0
pub fn i_08f6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6EA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6EA: move.b #$f0, (a6, d0.w)
pub fn i_08f6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F0;
    cpu.ir = 0x1DBC;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F6F0: rts
pub fn i_08f6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_SetLowNibble_08f6f2  ($08F6F2, 34 B, mission/gameover_continue_08f6d2.s)
/// $08F6F2: move.b $106f28.l, d0
pub fn i_08f6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F6F8: not.b d0
pub fn i_08f6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6FA;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F6FA: andi.w #$1, d0
pub fn i_08f6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6FE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6FE: addi.w #$96, d0
pub fn i_08f6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F702;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F702: lsl.b #$4, d1
pub fn i_08f702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F704;
    cpu.ir = 0xE909;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $08F704: move.b (a6, d0.w), d2
pub fn i_08f704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F708;
    cpu.ir = 0x1436;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F708: andi.b #$f0, d2
pub fn i_08f708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F70C;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08F70C: or.b d2, d1
pub fn i_08f70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F70E;
    cpu.ir = 0x8202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F70E: or.b d1, (a6, d0.w)
pub fn i_08f70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F712;
    cpu.ir = 0x8336;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $08F712: rts
pub fn i_08f712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F714;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_TestMaskCur_08f714  ($08F714, 24 B, mission/gameover_continue_08f6d2.s)
/// $08F714: move.b $106f28.l, d0
pub fn i_08f714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F71A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F71A: andi.w #$1, d0
pub fn i_08f71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F71E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F71E: addi.w #$96, d0
pub fn i_08f71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F722;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F722: move.b (a6, d0.w), d0
pub fn i_08f722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F726;
    cpu.ir = 0x1036;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F726: and.b d0, d1
pub fn i_08f726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F728;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F728: beq.w $8f732
pub fn i_08f728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F72C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F732; return 10; }
    8
}
// ===== SetXN_08f72c  ($08F72C, 6 B, thunks/ccr_helpers.c)
/// $08F72C: ori.b #$11, ccr
pub fn i_08f72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F730;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F730: rts
pub fn i_08f730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F732;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_08f732  ($08F732, 6 B, thunks/ccr_helpers.c)
/// $08F732: andi.b #$ee, ccr
pub fn i_08f732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F736;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F736: rts
pub fn i_08f736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F738;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anchor_GetWorldPos_08f738  ($08F738, 54 B, mission/gameover_continue_08f6d2.s)
/// $08F738: move.l $70(a0), d1
pub fn i_08f738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F73C;
    cpu.ir = 0x2228;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08F73C: bmi.w $8f774
pub fn i_08f73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F740;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x8F774; return 10; }
    8
}
/// $08F740: movea.l d1, a1
pub fn i_08f740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F742;
    cpu.ir = 0x2241;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08F742: movea.l (a1), a2
pub fn i_08f742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F744;
    cpu.ir = 0x2451;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[2] = v2;
    12
}
/// $08F744: andi.w #$ff, d0
pub fn i_08f744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F748;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F748: add.w d0, d0
pub fn i_08f748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F74A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x08DEC2, i_08dec2),
    (0x08DEC6, i_08dec6),
    (0x08DECC, i_08decc),
    (0x08DED2, i_08ded2),
    (0x08DED8, i_08ded8),
    (0x08DEDE, i_08dede),
    (0x08DEE0, i_08dee0),
    (0x08DEE6, i_08dee6),
    (0x08DEEC, i_08deec),
    (0x08DEF0, i_08def0),
    (0x08DEF6, i_08def6),
    (0x08DEFC, i_08defc),
    (0x08DF02, i_08df02),
    (0x08DF06, i_08df06),
    (0x08DF0C, i_08df0c),
    (0x08DF10, i_08df10),
    (0x08DF14, i_08df14),
    (0x08DF18, i_08df18),
    (0x08DF1C, i_08df1c),
    (0x08DF22, i_08df22),
    (0x08DF28, i_08df28),
    (0x08DF2E, i_08df2e),
    (0x08DF32, i_08df32),
    (0x08DF34, i_08df34),
    (0x08DF3A, i_08df3a),
    (0x08DF3E, i_08df3e),
    (0x08DF42, i_08df42),
    (0x08DF48, i_08df48),
    (0x08DF4A, i_08df4a),
    (0x08DF4E, i_08df4e),
    (0x08DF52, i_08df52),
    (0x08DF56, i_08df56),
    (0x08DF5A, i_08df5a),
    (0x08DF5E, i_08df5e),
    (0x08DF64, i_08df64),
    (0x08DF6A, i_08df6a),
    (0x08DF6E, i_08df6e),
    (0x08DF70, i_08df70),
    (0x08DF72, i_08df72),
    (0x08DF76, i_08df76),
    (0x08DF7C, i_08df7c),
    (0x08DF80, i_08df80),
    (0x08DF84, i_08df84),
    (0x08DF8A, i_08df8a),
    (0x08DF8C, i_08df8c),
    (0x08DF92, i_08df92),
    (0x08DF94, i_08df94),
    (0x08DF98, i_08df98),
    (0x08DF9C, i_08df9c),
    (0x08DFA0, i_08dfa0),
    (0x08DFA4, i_08dfa4),
    (0x08DFAA, i_08dfaa),
    (0x08DFB0, i_08dfb0),
    (0x08DFB4, i_08dfb4),
    (0x08DFB6, i_08dfb6),
    (0x08DFB8, i_08dfb8),
    (0x08DFBE, i_08dfbe),
    (0x08DFC4, i_08dfc4),
    (0x08DFC8, i_08dfc8),
    (0x08DFCA, i_08dfca),
    (0x08DFD0, i_08dfd0),
    (0x08DFD6, i_08dfd6),
    (0x08DFDA, i_08dfda),
    (0x08DFDE, i_08dfde),
    (0x08DFE2, i_08dfe2),
    (0x08DFE6, i_08dfe6),
    (0x08DFEA, i_08dfea),
    (0x08DFF0, i_08dff0),
    (0x08DFF6, i_08dff6),
    (0x08DFFA, i_08dffa),
    (0x08E000, i_08e000),
    (0x08E004, i_08e004),
    (0x08E008, i_08e008),
    (0x08E00C, i_08e00c),
    (0x08E010, i_08e010),
    (0x08E012, i_08e012),
    (0x08E016, i_08e016),
    (0x08E01A, i_08e01a),
    (0x08E01E, i_08e01e),
    (0x08E022, i_08e022),
    (0x08E028, i_08e028),
    (0x08E02A, i_08e02a),
    (0x08E030, i_08e030),
    (0x08E036, i_08e036),
    (0x08E03A, i_08e03a),
    (0x08E03C, i_08e03c),
    (0x08E042, i_08e042),
    (0x08E048, i_08e048),
    (0x08E04C, i_08e04c),
    (0x08E050, i_08e050),
    (0x08E054, i_08e054),
    (0x08E058, i_08e058),
    (0x08E05E, i_08e05e),
    (0x08E060, i_08e060),
    (0x08E066, i_08e066),
    (0x08E06C, i_08e06c),
    (0x08E070, i_08e070),
    (0x08E072, i_08e072),
    (0x08E078, i_08e078),
    (0x08E07C, i_08e07c),
    (0x08E082, i_08e082),
    (0x08E086, i_08e086),
    (0x08E08A, i_08e08a),
    (0x08E08C, i_08e08c),
    (0x08E090, i_08e090),
    (0x08E094, i_08e094),
    (0x08E09A, i_08e09a),
    (0x08E09C, i_08e09c),
    (0x08E0A0, i_08e0a0),
    (0x08E0A6, i_08e0a6),
    (0x08E0AC, i_08e0ac),
    (0x08E0B2, i_08e0b2),
    (0x08E0B8, i_08e0b8),
    (0x08E0BA, i_08e0ba),
    (0x08E0C0, i_08e0c0),
    (0x08E0C6, i_08e0c6),
    (0x08E0CA, i_08e0ca),
    (0x08E0D0, i_08e0d0),
    (0x08E0D6, i_08e0d6),
    (0x08E0DC, i_08e0dc),
    (0x08E0E0, i_08e0e0),
    (0x08E0E4, i_08e0e4),
    (0x08E0E8, i_08e0e8),
    (0x08E0EC, i_08e0ec),
    (0x08E0F2, i_08e0f2),
    (0x08E0F8, i_08e0f8),
    (0x08E0FC, i_08e0fc),
    (0x08E0FE, i_08e0fe),
    (0x08E104, i_08e104),
    (0x08E10A, i_08e10a),
    (0x08E10C, i_08e10c),
    (0x08E110, i_08e110),
    (0x08E116, i_08e116),
    (0x08E11A, i_08e11a),
    (0x08E11E, i_08e11e),
    (0x08E122, i_08e122),
    (0x08E126, i_08e126),
    (0x08E12A, i_08e12a),
    (0x08E130, i_08e130),
    (0x08E136, i_08e136),
    (0x08E13A, i_08e13a),
    (0x08E140, i_08e140),
    (0x08E144, i_08e144),
    (0x08E148, i_08e148),
    (0x08E14C, i_08e14c),
    (0x08E150, i_08e150),
    (0x08E152, i_08e152),
    (0x08E156, i_08e156),
    (0x08E15C, i_08e15c),
    (0x08E162, i_08e162),
    (0x08E166, i_08e166),
    (0x08E16A, i_08e16a),
    (0x08E170, i_08e170),
    (0x08E172, i_08e172),
    (0x08E174, i_08e174),
    (0x08E178, i_08e178),
    (0x08E17A, i_08e17a),
    (0x08E17E, i_08e17e),
    (0x08E184, i_08e184),
    (0x08E186, i_08e186),
    (0x08E18A, i_08e18a),
    (0x08E190, i_08e190),
    (0x08E194, i_08e194),
    (0x08E19A, i_08e19a),
    (0x08E19C, i_08e19c),
    (0x08E1A0, i_08e1a0),
    (0x08E1A4, i_08e1a4),
    (0x08E1A8, i_08e1a8),
    (0x08E1AC, i_08e1ac),
    (0x08E1B2, i_08e1b2),
    (0x08E1B8, i_08e1b8),
    (0x08E1BE, i_08e1be),
    (0x08E1C2, i_08e1c2),
    (0x08E1C4, i_08e1c4),
    (0x08E1C8, i_08e1c8),
    (0x08E1CC, i_08e1cc),
    (0x08E1D2, i_08e1d2),
    (0x08E1D8, i_08e1d8),
    (0x08E1DC, i_08e1dc),
    (0x08E1DE, i_08e1de),
    (0x08E1E4, i_08e1e4),
    (0x08E1E8, i_08e1e8),
    (0x08E1EC, i_08e1ec),
    (0x08E1F2, i_08e1f2),
    (0x08E1F8, i_08e1f8),
    (0x08E1FA, i_08e1fa),
    (0x08E200, i_08e200),
    (0x08E206, i_08e206),
    (0x08E20C, i_08e20c),
    (0x08E212, i_08e212),
    (0x08E218, i_08e218),
    (0x08E21C, i_08e21c),
    (0x08E220, i_08e220),
    (0x08E224, i_08e224),
    (0x08E22A, i_08e22a),
    (0x08E22E, i_08e22e),
    (0x08E234, i_08e234),
    (0x08E23A, i_08e23a),
    (0x08E240, i_08e240),
    (0x08E246, i_08e246),
    (0x08E24C, i_08e24c),
    (0x08E250, i_08e250),
    (0x08E252, i_08e252),
    (0x08E256, i_08e256),
    (0x08E25C, i_08e25c),
    (0x08E260, i_08e260),
    (0x08E266, i_08e266),
    (0x08E26A, i_08e26a),
    (0x08E270, i_08e270),
    (0x08E274, i_08e274),
    (0x08E27A, i_08e27a),
    (0x08E280, i_08e280),
    (0x08E284, i_08e284),
    (0x08E286, i_08e286),
    (0x08E288, i_08e288),
    (0x08E28C, i_08e28c),
    (0x08E290, i_08e290),
    (0x08E296, i_08e296),
    (0x08E29C, i_08e29c),
    (0x08E2A0, i_08e2a0),
    (0x08E2A2, i_08e2a2),
    (0x08E2A6, i_08e2a6),
    (0x08E2AA, i_08e2aa),
    (0x08E2AC, i_08e2ac),
    (0x08E2B0, i_08e2b0),
    (0x08E2B4, i_08e2b4),
    (0x08E2B8, i_08e2b8),
    (0x08E2BC, i_08e2bc),
    (0x08E2C2, i_08e2c2),
    (0x08E2C8, i_08e2c8),
    (0x08E2CE, i_08e2ce),
    (0x08E2D4, i_08e2d4),
    (0x08E2DA, i_08e2da),
    (0x08E2DE, i_08e2de),
    (0x08E2E0, i_08e2e0),
    (0x08E2E4, i_08e2e4),
    (0x08E2E8, i_08e2e8),
    (0x08E2EC, i_08e2ec),
    (0x08E2EE, i_08e2ee),
    (0x08E2F0, i_08e2f0),
    (0x08E2F6, i_08e2f6),
    (0x08E2FA, i_08e2fa),
    (0x08E300, i_08e300),
    (0x08E304, i_08e304),
    (0x08E30A, i_08e30a),
    (0x08E30C, i_08e30c),
    (0x08E310, i_08e310),
    (0x08E314, i_08e314),
    (0x08E318, i_08e318),
    (0x08E31C, i_08e31c),
    (0x08E322, i_08e322),
    (0x08E328, i_08e328),
    (0x08E32E, i_08e32e),
    (0x08E334, i_08e334),
    (0x08E33A, i_08e33a),
    (0x08E340, i_08e340),
    (0x08E344, i_08e344),
    (0x08E34A, i_08e34a),
    (0x08E34E, i_08e34e),
    (0x08E354, i_08e354),
    (0x08E358, i_08e358),
    (0x08E35E, i_08e35e),
    (0x08E362, i_08e362),
    (0x08E366, i_08e366),
    (0x08E368, i_08e368),
    (0x08E36C, i_08e36c),
    (0x08E372, i_08e372),
    (0x08E376, i_08e376),
    (0x08E37A, i_08e37a),
    (0x08E380, i_08e380),
    (0x08E384, i_08e384),
    (0x08E388, i_08e388),
    (0x08E38C, i_08e38c),
    (0x08E390, i_08e390),
    (0x08E394, i_08e394),
    (0x08E396, i_08e396),
    (0x08E39A, i_08e39a),
    (0x08E39E, i_08e39e),
    (0x08E3A2, i_08e3a2),
    (0x08E3A6, i_08e3a6),
    (0x08E3AA, i_08e3aa),
    (0x08E3AC, i_08e3ac),
    (0x08E3B0, i_08e3b0),
    (0x08E3B4, i_08e3b4),
    (0x08E3BA, i_08e3ba),
    (0x08E3BC, i_08e3bc),
    (0x08E3C2, i_08e3c2),
    (0x08E3C4, i_08e3c4),
    (0x08E3C8, i_08e3c8),
    (0x08E3CE, i_08e3ce),
    (0x08E3D2, i_08e3d2),
    (0x08E3D6, i_08e3d6),
    (0x08E3D8, i_08e3d8),
    (0x08E3DC, i_08e3dc),
    (0x08E3E2, i_08e3e2),
    (0x08E3E6, i_08e3e6),
    (0x08E3EA, i_08e3ea),
    (0x08E3EC, i_08e3ec),
    (0x08E3F0, i_08e3f0),
    (0x08E3F6, i_08e3f6),
    (0x08E3FA, i_08e3fa),
    (0x08E3FE, i_08e3fe),
    (0x08E404, i_08e404),
    (0x08E406, i_08e406),
    (0x08E40C, i_08e40c),
    (0x08E40E, i_08e40e),
    (0x08E412, i_08e412),
    (0x08E418, i_08e418),
    (0x08E41C, i_08e41c),
    (0x08E420, i_08e420),
    (0x08E424, i_08e424),
    (0x08E42A, i_08e42a),
    (0x08E430, i_08e430),
    (0x08E434, i_08e434),
    (0x08E436, i_08e436),
    (0x08E43A, i_08e43a),
    (0x08E440, i_08e440),
    (0x08E444, i_08e444),
    (0x08E448, i_08e448),
    (0x08E44C, i_08e44c),
    (0x08E452, i_08e452),
    (0x08E456, i_08e456),
    (0x08E45A, i_08e45a),
    (0x08E45C, i_08e45c),
    (0x08E462, i_08e462),
    (0x08E466, i_08e466),
    (0x08E46C, i_08e46c),
    (0x08E470, i_08e470),
    (0x08E474, i_08e474),
    (0x08E476, i_08e476),
    (0x08E47A, i_08e47a),
    (0x08E480, i_08e480),
    (0x08E484, i_08e484),
    (0x08E488, i_08e488),
    (0x08E48A, i_08e48a),
    (0x08E48E, i_08e48e),
    (0x08E492, i_08e492),
    (0x08E498, i_08e498),
    (0x08E49A, i_08e49a),
    (0x08E4A0, i_08e4a0),
    (0x08E4A2, i_08e4a2),
    (0x08E4A8, i_08e4a8),
    (0x08E4AE, i_08e4ae),
    (0x08E4B2, i_08e4b2),
    (0x08E4B4, i_08e4b4),
    (0x08E4BA, i_08e4ba),
    (0x08E4C0, i_08e4c0),
    (0x08E4C4, i_08e4c4),
    (0x08E4C8, i_08e4c8),
    (0x08E4CE, i_08e4ce),
    (0x08E4D2, i_08e4d2),
    (0x08E4D4, i_08e4d4),
    (0x08E4D8, i_08e4d8),
    (0x08E4DC, i_08e4dc),
    (0x08E4E2, i_08e4e2),
    (0x08E4E4, i_08e4e4),
    (0x08E4E6, i_08e4e6),
    (0x08E4EC, i_08e4ec),
    (0x08E4F0, i_08e4f0),
    (0x08E4F4, i_08e4f4),
    (0x08E4F8, i_08e4f8),
    (0x08E4FE, i_08e4fe),
    (0x08E504, i_08e504),
    (0x08E50A, i_08e50a),
    (0x08E50E, i_08e50e),
    (0x08E514, i_08e514),
    (0x08E518, i_08e518),
    (0x08E51E, i_08e51e),
    (0x08E524, i_08e524),
    (0x08E528, i_08e528),
    (0x08E52A, i_08e52a),
    (0x08E530, i_08e530),
    (0x08E534, i_08e534),
    (0x08E53A, i_08e53a),
    (0x08E540, i_08e540),
    (0x08E544, i_08e544),
    (0x08E548, i_08e548),
    (0x08E54A, i_08e54a),
    (0x08E54C, i_08e54c),
    (0x08E550, i_08e550),
    (0x08E556, i_08e556),
    (0x08E55A, i_08e55a),
    (0x08E55E, i_08e55e),
    (0x08E560, i_08e560),
    (0x08E564, i_08e564),
    (0x08E566, i_08e566),
    (0x08E56C, i_08e56c),
    (0x08E572, i_08e572),
    (0x08E576, i_08e576),
    (0x08E57C, i_08e57c),
    (0x08E580, i_08e580),
    (0x08E586, i_08e586),
    (0x08E58C, i_08e58c),
    (0x08E590, i_08e590),
    (0x08E592, i_08e592),
    (0x08E598, i_08e598),
    (0x08E59C, i_08e59c),
    (0x08E5A2, i_08e5a2),
    (0x08E5A8, i_08e5a8),
    (0x08E5AC, i_08e5ac),
    (0x08E5B0, i_08e5b0),
    (0x08E5B2, i_08e5b2),
    (0x08E5B4, i_08e5b4),
    (0x08E5B8, i_08e5b8),
    (0x08E5BE, i_08e5be),
    (0x08E5C2, i_08e5c2),
    (0x08E5C6, i_08e5c6),
    (0x08E5C8, i_08e5c8),
    (0x08E5CC, i_08e5cc),
    (0x08E5CE, i_08e5ce),
    (0x08E5D2, i_08e5d2),
    (0x08E5D6, i_08e5d6),
    (0x08E5DA, i_08e5da),
    (0x08E5E0, i_08e5e0),
    (0x08E5E6, i_08e5e6),
    (0x08E5EA, i_08e5ea),
    (0x08E5F0, i_08e5f0),
    (0x08E5F6, i_08e5f6),
    (0x08E5FC, i_08e5fc),
    (0x08E602, i_08e602),
    (0x08E608, i_08e608),
    (0x08E60C, i_08e60c),
    (0x08E612, i_08e612),
    (0x08E616, i_08e616),
    (0x08E61A, i_08e61a),
    (0x08E61E, i_08e61e),
    (0x08E620, i_08e620),
    (0x08E622, i_08e622),
    (0x08E626, i_08e626),
    (0x08E62C, i_08e62c),
    (0x08E630, i_08e630),
    (0x08E634, i_08e634),
    (0x08E636, i_08e636),
    (0x08E63C, i_08e63c),
    (0x08E640, i_08e640),
    (0x08E644, i_08e644),
    (0x08E646, i_08e646),
    (0x08E64A, i_08e64a),
    (0x08E650, i_08e650),
    (0x08E654, i_08e654),
    (0x08E65A, i_08e65a),
    (0x08E65C, i_08e65c),
    (0x08E660, i_08e660),
    (0x08E666, i_08e666),
    (0x08E66A, i_08e66a),
    (0x08E66C, i_08e66c),
    (0x08E670, i_08e670),
    (0x08E676, i_08e676),
    (0x08E67C, i_08e67c),
    (0x08E682, i_08e682),
    (0x08E688, i_08e688),
    (0x08E68C, i_08e68c),
    (0x08E68E, i_08e68e),
    (0x08E692, i_08e692),
    (0x08E698, i_08e698),
    (0x08E69C, i_08e69c),
    (0x08E6A0, i_08e6a0),
    (0x08E6A2, i_08e6a2),
    (0x08E6A6, i_08e6a6),
    (0x08E6AC, i_08e6ac),
    (0x08E6B0, i_08e6b0),
    (0x08E6B6, i_08e6b6),
    (0x08E6B8, i_08e6b8),
    (0x08E6BE, i_08e6be),
    (0x08E6C4, i_08e6c4),
    (0x08E6C8, i_08e6c8),
    (0x08E6CA, i_08e6ca),
    (0x08E6CE, i_08e6ce),
    (0x08E6D4, i_08e6d4),
    (0x08E6D8, i_08e6d8),
    (0x08E6DE, i_08e6de),
    (0x08E6E0, i_08e6e0),
    (0x08E6E4, i_08e6e4),
    (0x08E6E8, i_08e6e8),
    (0x08E6EC, i_08e6ec),
    (0x08E6F0, i_08e6f0),
    (0x08E6F6, i_08e6f6),
    (0x08E6FC, i_08e6fc),
    (0x08E702, i_08e702),
    (0x08E708, i_08e708),
    (0x08E70E, i_08e70e),
    (0x08E712, i_08e712),
    (0x08E714, i_08e714),
    (0x08E716, i_08e716),
    (0x08E71A, i_08e71a),
    (0x08E720, i_08e720),
    (0x08E724, i_08e724),
    (0x08E728, i_08e728),
    (0x08E72E, i_08e72e),
    (0x08E730, i_08e730),
    (0x08E736, i_08e736),
    (0x08E738, i_08e738),
    (0x08E73E, i_08e73e),
    (0x08E742, i_08e742),
    (0x08E746, i_08e746),
    (0x08E74A, i_08e74a),
    (0x08E750, i_08e750),
    (0x08E754, i_08e754),
    (0x08E758, i_08e758),
    (0x08E75E, i_08e75e),
    (0x08E764, i_08e764),
    (0x08E76A, i_08e76a),
    (0x08E770, i_08e770),
    (0x08E774, i_08e774),
    (0x08E77A, i_08e77a),
    (0x08E780, i_08e780),
    (0x08E784, i_08e784),
    (0x08E78A, i_08e78a),
    (0x08E78E, i_08e78e),
    (0x08E792, i_08e792),
    (0x08E798, i_08e798),
    (0x08E79E, i_08e79e),
    (0x08E7A0, i_08e7a0),
    (0x08E7A4, i_08e7a4),
    (0x08E7A6, i_08e7a6),
    (0x08E7AC, i_08e7ac),
    (0x08E7B2, i_08e7b2),
    (0x08E7B8, i_08e7b8),
    (0x08E7BC, i_08e7bc),
    (0x08E7C0, i_08e7c0),
    (0x08E7C2, i_08e7c2),
    (0x08E7C6, i_08e7c6),
    (0x08E7CA, i_08e7ca),
    (0x08E7D0, i_08e7d0),
    (0x08E7D6, i_08e7d6),
    (0x08E7D8, i_08e7d8),
    (0x08E7DC, i_08e7dc),
    (0x08E7DE, i_08e7de),
    (0x08E7E4, i_08e7e4),
    (0x08E7EA, i_08e7ea),
    (0x08E7EE, i_08e7ee),
    (0x08E7F0, i_08e7f0),
    (0x08E7F6, i_08e7f6),
    (0x08E7FC, i_08e7fc),
    (0x08E800, i_08e800),
    (0x08E804, i_08e804),
    (0x08E806, i_08e806),
    (0x08E80A, i_08e80a),
    (0x08E80E, i_08e80e),
    (0x08E814, i_08e814),
    (0x08E81A, i_08e81a),
    (0x08E81C, i_08e81c),
    (0x08E820, i_08e820),
    (0x08E822, i_08e822),
    (0x08E828, i_08e828),
    (0x08E82E, i_08e82e),
    (0x08E832, i_08e832),
    (0x08E834, i_08e834),
    (0x08E83A, i_08e83a),
    (0x08E840, i_08e840),
    (0x08E846, i_08e846),
    (0x08E84A, i_08e84a),
    (0x08E84E, i_08e84e),
    (0x08E850, i_08e850),
    (0x08E854, i_08e854),
    (0x08E858, i_08e858),
    (0x08E85E, i_08e85e),
    (0x08E864, i_08e864),
    (0x08E866, i_08e866),
    (0x08E86A, i_08e86a),
    (0x08E86C, i_08e86c),
    (0x08E872, i_08e872),
    (0x08E878, i_08e878),
    (0x08E87C, i_08e87c),
    (0x08E87E, i_08e87e),
    (0x08E884, i_08e884),
    (0x08E88A, i_08e88a),
    (0x08E88E, i_08e88e),
    (0x08E892, i_08e892),
    (0x08E894, i_08e894),
    (0x08E898, i_08e898),
    (0x08E89C, i_08e89c),
    (0x08E8A2, i_08e8a2),
    (0x08E8A8, i_08e8a8),
    (0x08E8AA, i_08e8aa),
    (0x08E8AE, i_08e8ae),
    (0x08E8B0, i_08e8b0),
    (0x08E8B6, i_08e8b6),
    (0x08E8BC, i_08e8bc),
    (0x08E8C0, i_08e8c0),
    (0x08E8C2, i_08e8c2),
    (0x08E8C8, i_08e8c8),
    (0x08E8CE, i_08e8ce),
    (0x08E8D4, i_08e8d4),
    (0x08E8D8, i_08e8d8),
    (0x08E8DC, i_08e8dc),
    (0x08E8DE, i_08e8de),
    (0x08E8E2, i_08e8e2),
    (0x08E8E6, i_08e8e6),
    (0x08E8EC, i_08e8ec),
    (0x08E8F2, i_08e8f2),
    (0x08E8F4, i_08e8f4),
    (0x08E8F8, i_08e8f8),
    (0x08E8FA, i_08e8fa),
    (0x08E900, i_08e900),
    (0x08E906, i_08e906),
    (0x08E90A, i_08e90a),
    (0x08E90C, i_08e90c),
    (0x08E912, i_08e912),
    (0x08E918, i_08e918),
    (0x08E91C, i_08e91c),
    (0x08E920, i_08e920),
    (0x08E922, i_08e922),
    (0x08E926, i_08e926),
    (0x08E92A, i_08e92a),
    (0x08E930, i_08e930),
    (0x08E936, i_08e936),
    (0x08E938, i_08e938),
    (0x08E93C, i_08e93c),
    (0x08E93E, i_08e93e),
    (0x08E944, i_08e944),
    (0x08E94A, i_08e94a),
    (0x08E94E, i_08e94e),
    (0x08E950, i_08e950),
    (0x08E956, i_08e956),
    (0x08E95C, i_08e95c),
    (0x08E962, i_08e962),
    (0x08E966, i_08e966),
    (0x08E96A, i_08e96a),
    (0x08E96C, i_08e96c),
    (0x08E970, i_08e970),
    (0x08E974, i_08e974),
    (0x08E97A, i_08e97a),
    (0x08E980, i_08e980),
    (0x08E982, i_08e982),
    (0x08E986, i_08e986),
    (0x08E988, i_08e988),
    (0x08E98C, i_08e98c),
    (0x08E990, i_08e990),
    (0x08E996, i_08e996),
    (0x08E99C, i_08e99c),
    (0x08E9A0, i_08e9a0),
    (0x08E9A2, i_08e9a2),
    (0x08E9A8, i_08e9a8),
    (0x08E9AE, i_08e9ae),
    (0x08E9B2, i_08e9b2),
    (0x08E9B6, i_08e9b6),
    (0x08E9B8, i_08e9b8),
    (0x08E9BC, i_08e9bc),
    (0x08E9C0, i_08e9c0),
    (0x08E9C6, i_08e9c6),
    (0x08E9CC, i_08e9cc),
    (0x08E9CE, i_08e9ce),
    (0x08E9D2, i_08e9d2),
    (0x08E9D4, i_08e9d4),
    (0x08E9DA, i_08e9da),
    (0x08E9E0, i_08e9e0),
    (0x08E9E4, i_08e9e4),
    (0x08E9E8, i_08e9e8),
    (0x08E9EE, i_08e9ee),
    (0x08E9F4, i_08e9f4),
    (0x08E9F6, i_08e9f6),
    (0x08E9FA, i_08e9fa),
    (0x08E9FC, i_08e9fc),
    (0x08EA00, i_08ea00),
    (0x08EA04, i_08ea04),
    (0x08EA0A, i_08ea0a),
    (0x08EA10, i_08ea10),
    (0x08EA14, i_08ea14),
    (0x08EA16, i_08ea16),
    (0x08EA1C, i_08ea1c),
    (0x08EA20, i_08ea20),
    (0x08EA26, i_08ea26),
    (0x08EA2C, i_08ea2c),
    (0x08EA32, i_08ea32),
    (0x08EA38, i_08ea38),
    (0x08EA3C, i_08ea3c),
    (0x08EA40, i_08ea40),
    (0x08EA46, i_08ea46),
    (0x08EA4C, i_08ea4c),
    (0x08EA4E, i_08ea4e),
    (0x08EA50, i_08ea50),
    (0x08EA56, i_08ea56),
    (0x08EA5A, i_08ea5a),
    (0x08EA5C, i_08ea5c),
    (0x08EA60, i_08ea60),
    (0x08EA68, i_08ea68),
    (0x08EA6C, i_08ea6c),
    (0x08EA70, i_08ea70),
    (0x08EA72, i_08ea72),
    (0x08EA74, i_08ea74),
    (0x08EA78, i_08ea78),
    (0x08EA7E, i_08ea7e),
    (0x08EA82, i_08ea82),
    (0x08EA88, i_08ea88),
    (0x08EA8E, i_08ea8e),
    (0x08EA92, i_08ea92),
    (0x08EA96, i_08ea96),
    (0x08EA9C, i_08ea9c),
    (0x08EAA0, i_08eaa0),
    (0x08EAA4, i_08eaa4),
    (0x08EAAA, i_08eaaa),
    (0x08EAB0, i_08eab0),
    (0x08EAB2, i_08eab2),
    (0x08EAB6, i_08eab6),
    (0x08EAB8, i_08eab8),
    (0x08EABE, i_08eabe),
    (0x08EAC4, i_08eac4),
    (0x08EAC8, i_08eac8),
    (0x08EACC, i_08eacc),
    (0x08EACE, i_08eace),
    (0x08EAD2, i_08ead2),
    (0x08EAD6, i_08ead6),
    (0x08EADA, i_08eada),
    (0x08EADE, i_08eade),
    (0x08EAE4, i_08eae4),
    (0x08EAEA, i_08eaea),
    (0x08EAEC, i_08eaec),
    (0x08EAEE, i_08eaee),
    (0x08EAF4, i_08eaf4),
    (0x08EAFA, i_08eafa),
    (0x08EAFE, i_08eafe),
    (0x08EB00, i_08eb00),
    (0x08EB02, i_08eb02),
    (0x08EB06, i_08eb06),
    (0x08EB0A, i_08eb0a),
    (0x08EB0C, i_08eb0c),
    (0x08EB12, i_08eb12),
    (0x08EB16, i_08eb16),
    (0x08EB1C, i_08eb1c),
    (0x08EB20, i_08eb20),
    (0x08EB26, i_08eb26),
    (0x08EB2C, i_08eb2c),
    (0x08EB30, i_08eb30),
    (0x08EB34, i_08eb34),
    (0x08EB36, i_08eb36),
    (0x08EB3A, i_08eb3a),
    (0x08EB3E, i_08eb3e),
    (0x08EB42, i_08eb42),
    (0x08EB46, i_08eb46),
    (0x08EB4C, i_08eb4c),
    (0x08EB52, i_08eb52),
    (0x08EB54, i_08eb54),
    (0x08EB56, i_08eb56),
    (0x08EB5A, i_08eb5a),
    (0x08EB60, i_08eb60),
    (0x08EB66, i_08eb66),
    (0x08EB6C, i_08eb6c),
    (0x08EB70, i_08eb70),
    (0x08EB74, i_08eb74),
    (0x08EB78, i_08eb78),
    (0x08EB7C, i_08eb7c),
    (0x08EB80, i_08eb80),
    (0x08EB84, i_08eb84),
    (0x08EB88, i_08eb88),
    (0x08EB8E, i_08eb8e),
    (0x08EB92, i_08eb92),
    (0x08EB96, i_08eb96),
    (0x08EB9A, i_08eb9a),
    (0x08EBA0, i_08eba0),
    (0x08EBA4, i_08eba4),
    (0x08EBA8, i_08eba8),
    (0x08EBAE, i_08ebae),
    (0x08EBB0, i_08ebb0),
    (0x08EBB4, i_08ebb4),
    (0x08EBB8, i_08ebb8),
    (0x08EBBE, i_08ebbe),
    (0x08EBC2, i_08ebc2),
    (0x08EBC6, i_08ebc6),
    (0x08EBCC, i_08ebcc),
    (0x08EBD0, i_08ebd0),
    (0x08EBD4, i_08ebd4),
    (0x08EBD8, i_08ebd8),
    (0x08EBDE, i_08ebde),
    (0x08EBE2, i_08ebe2),
    (0x08EBE8, i_08ebe8),
    (0x08EBEC, i_08ebec),
    (0x08EBF0, i_08ebf0),
    (0x08EBF4, i_08ebf4),
    (0x08EBF6, i_08ebf6),
    (0x08EBFA, i_08ebfa),
    (0x08EC00, i_08ec00),
    (0x08EC04, i_08ec04),
    (0x08EC0A, i_08ec0a),
    (0x08EC0E, i_08ec0e),
    (0x08EC12, i_08ec12),
    (0x08EC18, i_08ec18),
    (0x08EC1C, i_08ec1c),
    (0x08EC20, i_08ec20),
    (0x08EC24, i_08ec24),
    (0x08EC26, i_08ec26),
    (0x08EC2A, i_08ec2a),
    (0x08EC2E, i_08ec2e),
    (0x08EC30, i_08ec30),
    (0x08EC34, i_08ec34),
    (0x08EC3A, i_08ec3a),
    (0x08EC3E, i_08ec3e),
    (0x08EC42, i_08ec42),
    (0x08EC44, i_08ec44),
    (0x08EC4A, i_08ec4a),
    (0x08EC4E, i_08ec4e),
    (0x08EC50, i_08ec50),
    (0x08EC54, i_08ec54),
    (0x08EC58, i_08ec58),
    (0x08EC5C, i_08ec5c),
    (0x08EC60, i_08ec60),
    (0x08EC64, i_08ec64),
    (0x08EC68, i_08ec68),
    (0x08EC6C, i_08ec6c),
    (0x08EC72, i_08ec72),
    (0x08EC76, i_08ec76),
    (0x08EC7A, i_08ec7a),
    (0x08EC7E, i_08ec7e),
    (0x08EC84, i_08ec84),
    (0x08EC88, i_08ec88),
    (0x08EC8C, i_08ec8c),
    (0x08EC92, i_08ec92),
    (0x08EC96, i_08ec96),
    (0x08EC9A, i_08ec9a),
    (0x08EC9E, i_08ec9e),
    (0x08ECA4, i_08eca4),
    (0x08ECAA, i_08ecaa),
    (0x08ECAE, i_08ecae),
    (0x08ECB0, i_08ecb0),
    (0x08ECB4, i_08ecb4),
    (0x08ECBA, i_08ecba),
    (0x08ECBE, i_08ecbe),
    (0x08ECC4, i_08ecc4),
    (0x08ECC8, i_08ecc8),
    (0x08ECCC, i_08eccc),
    (0x08ECCE, i_08ecce),
    (0x08ECD4, i_08ecd4),
    (0x08ECD8, i_08ecd8),
    (0x08ECDA, i_08ecda),
    (0x08ECDE, i_08ecde),
    (0x08ECE2, i_08ece2),
    (0x08ECE6, i_08ece6),
    (0x08ECEC, i_08ecec),
    (0x08ECF2, i_08ecf2),
    (0x08ECF8, i_08ecf8),
    (0x08ECFC, i_08ecfc),
    (0x08ED02, i_08ed02),
    (0x08ED06, i_08ed06),
    (0x08ED0A, i_08ed0a),
    (0x08ED10, i_08ed10),
    (0x08ED12, i_08ed12),
    (0x08ED16, i_08ed16),
    (0x08ED1A, i_08ed1a),
    (0x08ED20, i_08ed20),
    (0x08ED24, i_08ed24),
    (0x08ED28, i_08ed28),
    (0x08ED2C, i_08ed2c),
    (0x08ED32, i_08ed32),
    (0x08ED36, i_08ed36),
    (0x08ED3A, i_08ed3a),
    (0x08ED40, i_08ed40),
    (0x08ED44, i_08ed44),
    (0x08ED48, i_08ed48),
    (0x08ED4C, i_08ed4c),
    (0x08ED52, i_08ed52),
    (0x08ED56, i_08ed56),
    (0x08ED58, i_08ed58),
    (0x08ED5C, i_08ed5c),
    (0x08ED62, i_08ed62),
    (0x08ED66, i_08ed66),
    (0x08ED6C, i_08ed6c),
    (0x08ED70, i_08ed70),
    (0x08ED74, i_08ed74),
    (0x08ED76, i_08ed76),
    (0x08ED7C, i_08ed7c),
    (0x08ED80, i_08ed80),
    (0x08ED82, i_08ed82),
    (0x08ED88, i_08ed88),
    (0x08ED8C, i_08ed8c),
    (0x08ED90, i_08ed90),
    (0x08ED96, i_08ed96),
    (0x08ED9C, i_08ed9c),
    (0x08EDA2, i_08eda2),
    (0x08EDA6, i_08eda6),
    (0x08EDA8, i_08eda8),
    (0x08EDAE, i_08edae),
    (0x08EDB2, i_08edb2),
    (0x08EDB6, i_08edb6),
    (0x08EDBC, i_08edbc),
    (0x08EDBE, i_08edbe),
    (0x08EDC4, i_08edc4),
    (0x08EDC6, i_08edc6),
    (0x08EDCC, i_08edcc),
    (0x08EDD2, i_08edd2),
    (0x08EDD6, i_08edd6),
    (0x08EDDC, i_08eddc),
    (0x08EDE2, i_08ede2),
    (0x08EDE8, i_08ede8),
    (0x08EDEC, i_08edec),
    (0x08EDF2, i_08edf2),
    (0x08EDF6, i_08edf6),
    (0x08EDF8, i_08edf8),
    (0x08EDFE, i_08edfe),
    (0x08EE02, i_08ee02),
    (0x08EE06, i_08ee06),
    (0x08EE0C, i_08ee0c),
    (0x08EE0E, i_08ee0e),
    (0x08EE14, i_08ee14),
    (0x08EE16, i_08ee16),
    (0x08EE1A, i_08ee1a),
    (0x08EE1E, i_08ee1e),
    (0x08EE20, i_08ee20),
    (0x08EE24, i_08ee24),
    (0x08EE26, i_08ee26),
    (0x08EE28, i_08ee28),
    (0x08EE2C, i_08ee2c),
    (0x08EE32, i_08ee32),
    (0x08EE36, i_08ee36),
    (0x08EE3C, i_08ee3c),
    (0x08EE42, i_08ee42),
    (0x08EE48, i_08ee48),
    (0x08EE4C, i_08ee4c),
    (0x08EE4E, i_08ee4e),
    (0x08EE52, i_08ee52),
    (0x08EE58, i_08ee58),
    (0x08EE5C, i_08ee5c),
    (0x08EE62, i_08ee62),
    (0x08EE68, i_08ee68),
    (0x08EE6C, i_08ee6c),
    (0x08EE6E, i_08ee6e),
    (0x08EE72, i_08ee72),
    (0x08EE78, i_08ee78),
    (0x08EE7A, i_08ee7a),
    (0x08EE80, i_08ee80),
    (0x08EE84, i_08ee84),
    (0x08EE8A, i_08ee8a),
    (0x08EE8E, i_08ee8e),
    (0x08EE94, i_08ee94),
    (0x08EE96, i_08ee96),
    (0x08EE9C, i_08ee9c),
    (0x08EEA2, i_08eea2),
    (0x08EEA8, i_08eea8),
    (0x08EEAC, i_08eeac),
    (0x08EEB2, i_08eeb2),
    (0x08EEB8, i_08eeb8),
    (0x08EEBE, i_08eebe),
    (0x08EEC2, i_08eec2),
    (0x08EEC8, i_08eec8),
    (0x08EECE, i_08eece),
    (0x08EED4, i_08eed4),
    (0x08EED8, i_08eed8),
    (0x08EEDE, i_08eede),
    (0x08EEE0, i_08eee0),
    (0x08EEE4, i_08eee4),
    (0x08EEE8, i_08eee8),
    (0x08EEEE, i_08eeee),
    (0x08EEF2, i_08eef2),
    (0x08EEF6, i_08eef6),
    (0x08EEFA, i_08eefa),
    (0x08EEFE, i_08eefe),
    (0x08EF04, i_08ef04),
    (0x08EF08, i_08ef08),
    (0x08EF0C, i_08ef0c),
    (0x08EF12, i_08ef12),
    (0x08EF16, i_08ef16),
    (0x08EF18, i_08ef18),
    (0x08EF1C, i_08ef1c),
    (0x08EF20, i_08ef20),
    (0x08EF24, i_08ef24),
    (0x08EF28, i_08ef28),
    (0x08EF2C, i_08ef2c),
    (0x08EF2E, i_08ef2e),
    (0x08EF34, i_08ef34),
    (0x08EF38, i_08ef38),
    (0x08EF3E, i_08ef3e),
    (0x08EF40, i_08ef40),
    (0x08EF46, i_08ef46),
    (0x08EF4A, i_08ef4a),
    (0x08EF50, i_08ef50),
    (0x08EF56, i_08ef56),
    (0x08EF5A, i_08ef5a),
    (0x08EF60, i_08ef60),
    (0x08EF64, i_08ef64),
    (0x08EF6A, i_08ef6a),
    (0x08EF70, i_08ef70),
    (0x08EF74, i_08ef74),
    (0x08EF7A, i_08ef7a),
    (0x08EF7E, i_08ef7e),
    (0x08EF84, i_08ef84),
    (0x08EF8A, i_08ef8a),
    (0x08EF8E, i_08ef8e),
    (0x08EF92, i_08ef92),
    (0x08EF94, i_08ef94),
    (0x08EF9A, i_08ef9a),
    (0x08EF9E, i_08ef9e),
    (0x08EFA4, i_08efa4),
    (0x08EFA8, i_08efa8),
    (0x08EFAE, i_08efae),
    (0x08EFB0, i_08efb0),
    (0x08EFB6, i_08efb6),
    (0x08EFBC, i_08efbc),
    (0x08EFC0, i_08efc0),
    (0x08EFC6, i_08efc6),
    (0x08EFCC, i_08efcc),
    (0x08EFCE, i_08efce),
    (0x08EFD4, i_08efd4),
    (0x08EFD8, i_08efd8),
    (0x08EFDC, i_08efdc),
    (0x08EFDE, i_08efde),
    (0x08EFE2, i_08efe2),
    (0x08EFE8, i_08efe8),
    (0x08EFEC, i_08efec),
    (0x08EFF0, i_08eff0),
    (0x08EFF2, i_08eff2),
    (0x08EFF6, i_08eff6),
    (0x08EFFA, i_08effa),
    (0x08EFFC, i_08effc),
    (0x08F000, i_08f000),
    (0x08F002, i_08f002),
    (0x08F008, i_08f008),
    (0x08F00E, i_08f00e),
    (0x08F010, i_08f010),
    (0x08F012, i_08f012),
    (0x08F016, i_08f016),
    (0x08F018, i_08f018),
    (0x08F01C, i_08f01c),
    (0x08F022, i_08f022),
    (0x08F026, i_08f026),
    (0x08F02A, i_08f02a),
    (0x08F02C, i_08f02c),
    (0x08F02E, i_08f02e),
    (0x08F032, i_08f032),
    (0x08F03A, i_08f03a),
    (0x08F03E, i_08f03e),
    (0x08F040, i_08f040),
    (0x08F046, i_08f046),
    (0x08F04A, i_08f04a),
    (0x08F04E, i_08f04e),
    (0x08F050, i_08f050),
    (0x08F056, i_08f056),
    (0x08F05A, i_08f05a),
    (0x08F05E, i_08f05e),
    (0x08F062, i_08f062),
    (0x08F064, i_08f064),
    (0x08F068, i_08f068),
    (0x08F06C, i_08f06c),
    (0x08F06E, i_08f06e),
    (0x08F070, i_08f070),
    (0x08F072, i_08f072),
    (0x08F076, i_08f076),
    (0x08F07A, i_08f07a),
    (0x08F07C, i_08f07c),
    (0x08F082, i_08f082),
    (0x08F084, i_08f084),
    (0x08F08A, i_08f08a),
    (0x08F08E, i_08f08e),
    (0x08F092, i_08f092),
    (0x08F098, i_08f098),
    (0x08F09E, i_08f09e),
    (0x08F0A2, i_08f0a2),
    (0x08F0A6, i_08f0a6),
    (0x08F0AC, i_08f0ac),
    (0x08F0B2, i_08f0b2),
    (0x08F0B6, i_08f0b6),
    (0x08F0BA, i_08f0ba),
    (0x08F0C0, i_08f0c0),
    (0x08F0C4, i_08f0c4),
    (0x08F0C8, i_08f0c8),
    (0x08F0CE, i_08f0ce),
    (0x08F0D0, i_08f0d0),
    (0x08F0D6, i_08f0d6),
    (0x08F0DA, i_08f0da),
    (0x08F0DE, i_08f0de),
    (0x08F0E4, i_08f0e4),
    (0x08F0EA, i_08f0ea),
    (0x08F0EE, i_08f0ee),
    (0x08F0F2, i_08f0f2),
    (0x08F0F8, i_08f0f8),
    (0x08F0FC, i_08f0fc),
    (0x08F100, i_08f100),
    (0x08F106, i_08f106),
    (0x08F108, i_08f108),
    (0x08F10E, i_08f10e),
    (0x08F114, i_08f114),
    (0x08F11A, i_08f11a),
    (0x08F11C, i_08f11c),
    (0x08F120, i_08f120),
    (0x08F124, i_08f124),
    (0x08F128, i_08f128),
    (0x08F12C, i_08f12c),
    (0x08F130, i_08f130),
    (0x08F132, i_08f132),
    (0x08F136, i_08f136),
    (0x08F138, i_08f138),
    (0x08F13A, i_08f13a),
    (0x08F13E, i_08f13e),
    (0x08F142, i_08f142),
    (0x08F146, i_08f146),
    (0x08F148, i_08f148),
    (0x08F14C, i_08f14c),
    (0x08F152, i_08f152),
    (0x08F156, i_08f156),
    (0x08F158, i_08f158),
    (0x08F15E, i_08f15e),
    (0x08F160, i_08f160),
    (0x08F166, i_08f166),
    (0x08F168, i_08f168),
    (0x08F16E, i_08f16e),
    (0x08F170, i_08f170),
    (0x08F172, i_08f172),
    (0x08F178, i_08f178),
    (0x08F17A, i_08f17a),
    (0x08F180, i_08f180),
    (0x08F182, i_08f182),
    (0x08F188, i_08f188),
    (0x08F18A, i_08f18a),
    (0x08F18C, i_08f18c),
    (0x08F190, i_08f190),
    (0x08F196, i_08f196),
    (0x08F19A, i_08f19a),
    (0x08F19C, i_08f19c),
    (0x08F1A0, i_08f1a0),
    (0x08F1A2, i_08f1a2),
    (0x08F1A4, i_08f1a4),
    (0x08F1A8, i_08f1a8),
    (0x08F1AC, i_08f1ac),
    (0x08F1B0, i_08f1b0),
    (0x08F1B2, i_08f1b2),
    (0x08F1B8, i_08f1b8),
    (0x08F1BC, i_08f1bc),
    (0x08F1BE, i_08f1be),
    (0x08F1C2, i_08f1c2),
    (0x08F1C4, i_08f1c4),
    (0x08F1C6, i_08f1c6),
    (0x08F1CA, i_08f1ca),
    (0x08F1CE, i_08f1ce),
    (0x08F1D0, i_08f1d0),
    (0x08F1D4, i_08f1d4),
    (0x08F1DA, i_08f1da),
    (0x08F1DE, i_08f1de),
    (0x08F1E4, i_08f1e4),
    (0x08F1EA, i_08f1ea),
    (0x08F1EC, i_08f1ec),
    (0x08F1F2, i_08f1f2),
    (0x08F1F6, i_08f1f6),
    (0x08F1FA, i_08f1fa),
    (0x08F1FE, i_08f1fe),
    (0x08F204, i_08f204),
    (0x08F20A, i_08f20a),
    (0x08F210, i_08f210),
    (0x08F216, i_08f216),
    (0x08F21A, i_08f21a),
    (0x08F21E, i_08f21e),
    (0x08F220, i_08f220),
    (0x08F224, i_08f224),
    (0x08F228, i_08f228),
    (0x08F22A, i_08f22a),
    (0x08F22E, i_08f22e),
    (0x08F232, i_08f232),
    (0x08F234, i_08f234),
    (0x08F238, i_08f238),
    (0x08F23C, i_08f23c),
    (0x08F242, i_08f242),
    (0x08F246, i_08f246),
    (0x08F248, i_08f248),
    (0x08F24E, i_08f24e),
    (0x08F250, i_08f250),
    (0x08F252, i_08f252),
    (0x08F254, i_08f254),
    (0x08F258, i_08f258),
    (0x08F25C, i_08f25c),
    (0x08F260, i_08f260),
    (0x08F266, i_08f266),
    (0x08F26A, i_08f26a),
    (0x08F26E, i_08f26e),
    (0x08F270, i_08f270),
    (0x08F274, i_08f274),
    (0x08F278, i_08f278),
    (0x08F27C, i_08f27c),
    (0x08F282, i_08f282),
    (0x08F286, i_08f286),
    (0x08F28A, i_08f28a),
    (0x08F28C, i_08f28c),
    (0x08F290, i_08f290),
    (0x08F294, i_08f294),
    (0x08F296, i_08f296),
    (0x08F29A, i_08f29a),
    (0x08F29E, i_08f29e),
    (0x08F2A0, i_08f2a0),
    (0x08F2A4, i_08f2a4),
    (0x08F2AA, i_08f2aa),
    (0x08F2AE, i_08f2ae),
    (0x08F2B2, i_08f2b2),
    (0x08F2B4, i_08f2b4),
    (0x08F2BA, i_08f2ba),
    (0x08F2C0, i_08f2c0),
    (0x08F2C4, i_08f2c4),
    (0x08F2C6, i_08f2c6),
    (0x08F2CA, i_08f2ca),
    (0x08F2CE, i_08f2ce),
    (0x08F2D2, i_08f2d2),
    (0x08F2D6, i_08f2d6),
    (0x08F2DA, i_08f2da),
    (0x08F2DE, i_08f2de),
    (0x08F2E2, i_08f2e2),
    (0x08F2E6, i_08f2e6),
    (0x08F2EA, i_08f2ea),
    (0x08F2EE, i_08f2ee),
    (0x08F2F2, i_08f2f2),
    (0x08F2F4, i_08f2f4),
    (0x08F2F6, i_08f2f6),
    (0x08F2FA, i_08f2fa),
    (0x08F2FE, i_08f2fe),
    (0x08F300, i_08f300),
    (0x08F302, i_08f302),
    (0x08F306, i_08f306),
    (0x08F308, i_08f308),
    (0x08F30E, i_08f30e),
    (0x08F312, i_08f312),
    (0x08F316, i_08f316),
    (0x08F31A, i_08f31a),
    (0x08F320, i_08f320),
    (0x08F326, i_08f326),
    (0x08F32A, i_08f32a),
    (0x08F32E, i_08f32e),
    (0x08F330, i_08f330),
    (0x08F334, i_08f334),
    (0x08F338, i_08f338),
    (0x08F33A, i_08f33a),
    (0x08F33E, i_08f33e),
    (0x08F342, i_08f342),
    (0x08F344, i_08f344),
    (0x08F34A, i_08f34a),
    (0x08F34E, i_08f34e),
    (0x08F350, i_08f350),
    (0x08F354, i_08f354),
    (0x08F358, i_08f358),
    (0x08F35C, i_08f35c),
    (0x08F360, i_08f360),
    (0x08F364, i_08f364),
    (0x08F366, i_08f366),
    (0x08F36A, i_08f36a),
    (0x08F36E, i_08f36e),
    (0x08F372, i_08f372),
    (0x08F376, i_08f376),
    (0x08F37A, i_08f37a),
    (0x08F37E, i_08f37e),
    (0x08F382, i_08f382),
    (0x08F388, i_08f388),
    (0x08F38A, i_08f38a),
    (0x08F38C, i_08f38c),
    (0x08F390, i_08f390),
    (0x08F392, i_08f392),
    (0x08F394, i_08f394),
    (0x08F398, i_08f398),
    (0x08F39C, i_08f39c),
    (0x08F39E, i_08f39e),
    (0x08F3A0, i_08f3a0),
    (0x08F3A4, i_08f3a4),
    (0x08F3A6, i_08f3a6),
    (0x08F3AC, i_08f3ac),
    (0x08F3AE, i_08f3ae),
    (0x08F3B2, i_08f3b2),
    (0x08F3B6, i_08f3b6),
    (0x08F3BC, i_08f3bc),
    (0x08F3BE, i_08f3be),
    (0x08F3C4, i_08f3c4),
    (0x08F3C8, i_08f3c8),
    (0x08F3CC, i_08f3cc),
    (0x08F3CE, i_08f3ce),
    (0x08F3D2, i_08f3d2),
    (0x08F3D6, i_08f3d6),
    (0x08F3DA, i_08f3da),
    (0x08F3DE, i_08f3de),
    (0x08F3E2, i_08f3e2),
    (0x08F3E6, i_08f3e6),
    (0x08F3EA, i_08f3ea),
    (0x08F3EC, i_08f3ec),
    (0x08F3F0, i_08f3f0),
    (0x08F3F4, i_08f3f4),
    (0x08F3F6, i_08f3f6),
    (0x08F3F8, i_08f3f8),
    (0x08F3FC, i_08f3fc),
    (0x08F400, i_08f400),
    (0x08F404, i_08f404),
    (0x08F408, i_08f408),
    (0x08F40C, i_08f40c),
    (0x08F410, i_08f410),
    (0x08F414, i_08f414),
    (0x08F418, i_08f418),
    (0x08F41C, i_08f41c),
    (0x08F420, i_08f420),
    (0x08F424, i_08f424),
    (0x08F428, i_08f428),
    (0x08F42C, i_08f42c),
    (0x08F42E, i_08f42e),
    (0x08F430, i_08f430),
    (0x08F434, i_08f434),
    (0x08F438, i_08f438),
    (0x08F43A, i_08f43a),
    (0x08F43C, i_08f43c),
    (0x08F43E, i_08f43e),
    (0x08F440, i_08f440),
    (0x08F444, i_08f444),
    (0x08F448, i_08f448),
    (0x08F44C, i_08f44c),
    (0x08F452, i_08f452),
    (0x08F456, i_08f456),
    (0x08F45A, i_08f45a),
    (0x08F45C, i_08f45c),
    (0x08F460, i_08f460),
    (0x08F464, i_08f464),
    (0x08F466, i_08f466),
    (0x08F46A, i_08f46a),
    (0x08F46E, i_08f46e),
    (0x08F470, i_08f470),
    (0x08F476, i_08f476),
    (0x08F47A, i_08f47a),
    (0x08F47C, i_08f47c),
    (0x08F482, i_08f482),
    (0x08F486, i_08f486),
    (0x08F48C, i_08f48c),
    (0x08F490, i_08f490),
    (0x08F494, i_08f494),
    (0x08F498, i_08f498),
    (0x08F49C, i_08f49c),
    (0x08F4A0, i_08f4a0),
    (0x08F4A6, i_08f4a6),
    (0x08F4AA, i_08f4aa),
    (0x08F4AC, i_08f4ac),
    (0x08F4B0, i_08f4b0),
    (0x08F4B4, i_08f4b4),
    (0x08F4B6, i_08f4b6),
    (0x08F4B8, i_08f4b8),
    (0x08F4BC, i_08f4bc),
    (0x08F4C0, i_08f4c0),
    (0x08F4C4, i_08f4c4),
    (0x08F4CA, i_08f4ca),
    (0x08F4CE, i_08f4ce),
    (0x08F4D0, i_08f4d0),
    (0x08F4D4, i_08f4d4),
    (0x08F4D8, i_08f4d8),
    (0x08F4DA, i_08f4da),
    (0x08F4DC, i_08f4dc),
    (0x08F4E0, i_08f4e0),
    (0x08F4E2, i_08f4e2),
    (0x08F4E6, i_08f4e6),
    (0x08F4E8, i_08f4e8),
    (0x08F4EC, i_08f4ec),
    (0x08F4EE, i_08f4ee),
    (0x08F4F2, i_08f4f2),
    (0x08F4F6, i_08f4f6),
    (0x08F4F8, i_08f4f8),
    (0x08F4FC, i_08f4fc),
    (0x08F500, i_08f500),
    (0x08F504, i_08f504),
    (0x08F508, i_08f508),
    (0x08F50A, i_08f50a),
    (0x08F50E, i_08f50e),
    (0x08F512, i_08f512),
    (0x08F516, i_08f516),
    (0x08F51A, i_08f51a),
    (0x08F51E, i_08f51e),
    (0x08F520, i_08f520),
    (0x08F526, i_08f526),
    (0x08F52A, i_08f52a),
    (0x08F52C, i_08f52c),
    (0x08F530, i_08f530),
    (0x08F536, i_08f536),
    (0x08F53A, i_08f53a),
    (0x08F53E, i_08f53e),
    (0x08F544, i_08f544),
    (0x08F548, i_08f548),
    (0x08F54C, i_08f54c),
    (0x08F550, i_08f550),
    (0x08F554, i_08f554),
    (0x08F558, i_08f558),
    (0x08F55C, i_08f55c),
    (0x08F562, i_08f562),
    (0x08F566, i_08f566),
    (0x08F568, i_08f568),
    (0x08F56C, i_08f56c),
    (0x08F570, i_08f570),
    (0x08F572, i_08f572),
    (0x08F574, i_08f574),
    (0x08F578, i_08f578),
    (0x08F57A, i_08f57a),
    (0x08F580, i_08f580),
    (0x08F584, i_08f584),
    (0x08F588, i_08f588),
    (0x08F58C, i_08f58c),
    (0x08F590, i_08f590),
    (0x08F594, i_08f594),
    (0x08F598, i_08f598),
    (0x08F59C, i_08f59c),
    (0x08F59E, i_08f59e),
    (0x08F5A0, i_08f5a0),
    (0x08F5A4, i_08f5a4),
    (0x08F5A8, i_08f5a8),
    (0x08F5AA, i_08f5aa),
    (0x08F5AC, i_08f5ac),
    (0x08F5B0, i_08f5b0),
    (0x08F5B4, i_08f5b4),
    (0x08F5BA, i_08f5ba),
    (0x08F5BE, i_08f5be),
    (0x08F5C2, i_08f5c2),
    (0x08F5C4, i_08f5c4),
    (0x08F5C8, i_08f5c8),
    (0x08F5CC, i_08f5cc),
    (0x08F5CE, i_08f5ce),
    (0x08F5D2, i_08f5d2),
    (0x08F5D6, i_08f5d6),
    (0x08F5DA, i_08f5da),
    (0x08F5DC, i_08f5dc),
    (0x08F5E2, i_08f5e2),
    (0x08F5E6, i_08f5e6),
    (0x08F5E8, i_08f5e8),
    (0x08F5EC, i_08f5ec),
    (0x08F5F0, i_08f5f0),
    (0x08F5F4, i_08f5f4),
    (0x08F5F8, i_08f5f8),
    (0x08F5FC, i_08f5fc),
    (0x08F600, i_08f600),
    (0x08F602, i_08f602),
    (0x08F606, i_08f606),
    (0x08F60A, i_08f60a),
    (0x08F60C, i_08f60c),
    (0x08F60E, i_08f60e),
    (0x08F612, i_08f612),
    (0x08F614, i_08f614),
    (0x08F616, i_08f616),
    (0x08F61A, i_08f61a),
    (0x08F61E, i_08f61e),
    (0x08F622, i_08f622),
    (0x08F628, i_08f628),
    (0x08F62C, i_08f62c),
    (0x08F630, i_08f630),
    (0x08F636, i_08f636),
    (0x08F63A, i_08f63a),
    (0x08F63E, i_08f63e),
    (0x08F642, i_08f642),
    (0x08F646, i_08f646),
    (0x08F64A, i_08f64a),
    (0x08F64E, i_08f64e),
    (0x08F652, i_08f652),
    (0x08F656, i_08f656),
    (0x08F65A, i_08f65a),
    (0x08F65C, i_08f65c),
    (0x08F65E, i_08f65e),
    (0x08F662, i_08f662),
    (0x08F666, i_08f666),
    (0x08F668, i_08f668),
    (0x08F66A, i_08f66a),
    (0x08F66E, i_08f66e),
    (0x08F674, i_08f674),
    (0x08F67A, i_08f67a),
    (0x08F67E, i_08f67e),
    (0x08F682, i_08f682),
    (0x08F684, i_08f684),
    (0x08F688, i_08f688),
    (0x08F68C, i_08f68c),
    (0x08F68E, i_08f68e),
    (0x08F692, i_08f692),
    (0x08F696, i_08f696),
    (0x08F69A, i_08f69a),
    (0x08F69C, i_08f69c),
    (0x08F6A0, i_08f6a0),
    (0x08F6A2, i_08f6a2),
    (0x08F6A6, i_08f6a6),
    (0x08F6AA, i_08f6aa),
    (0x08F6AC, i_08f6ac),
    (0x08F6B0, i_08f6b0),
    (0x08F6B4, i_08f6b4),
    (0x08F6B6, i_08f6b6),
    (0x08F6BA, i_08f6ba),
    (0x08F6BE, i_08f6be),
    (0x08F6C2, i_08f6c2),
    (0x08F6C6, i_08f6c6),
    (0x08F6CA, i_08f6ca),
    (0x08F6CC, i_08f6cc),
    (0x08F6D0, i_08f6d0),
    (0x08F6D2, i_08f6d2),
    (0x08F6D8, i_08f6d8),
    (0x08F6DA, i_08f6da),
    (0x08F6E0, i_08f6e0),
    (0x08F6E2, i_08f6e2),
    (0x08F6E6, i_08f6e6),
    (0x08F6EA, i_08f6ea),
    (0x08F6F0, i_08f6f0),
    (0x08F6F2, i_08f6f2),
    (0x08F6F8, i_08f6f8),
    (0x08F6FA, i_08f6fa),
    (0x08F6FE, i_08f6fe),
    (0x08F702, i_08f702),
    (0x08F704, i_08f704),
    (0x08F708, i_08f708),
    (0x08F70C, i_08f70c),
    (0x08F70E, i_08f70e),
    (0x08F712, i_08f712),
    (0x08F714, i_08f714),
    (0x08F71A, i_08f71a),
    (0x08F71E, i_08f71e),
    (0x08F722, i_08f722),
    (0x08F726, i_08f726),
    (0x08F728, i_08f728),
    (0x08F72C, i_08f72c),
    (0x08F730, i_08f730),
    (0x08F732, i_08f732),
    (0x08F736, i_08f736),
    (0x08F738, i_08f738),
    (0x08F73C, i_08f73c),
    (0x08F740, i_08f740),
    (0x08F742, i_08f742),
    (0x08F744, i_08f744),
    (0x08F748, i_08f748),
];
