// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $060000: jmp $518.l
pub fn i_060000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60006;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $060006: rts
pub fn i_060006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60008;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable22_Destroyed_060008  ($060008, 72 B, props/late_props_turrets_05e000.s)
/// $060008: lea.l $2c1b2a.l, a0
pub fn i_060008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6000E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1B2A;
    20
}
/// $06000E: jsr $28cd4.l
pub fn i_06000e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60014);
    cpu.pc = 0x28CD4;
    16
}
/// $060014: lea.l $6001a(pc), a1
pub fn i_060014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60018;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6001A;
    16
}
/// $060018: move.l a1, (a6)
pub fn i_060018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6001A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06001A: jsr $2783a.l
pub fn i_06001a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60020;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60020);
    cpu.pc = 0x2783A;
    16
}
/// $060020: jsr $28d70.l
pub fn i_060020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60026);
    cpu.pc = 0x28D70;
    16
}
/// $060026: bcc.w $60030
pub fn i_060026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6002A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60030; return 10; }
    8
}
/// $06002A: lea.l $5fe26(pc), a1
pub fn i_06002a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6002E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $06002E: move.l a1, (a6)
pub fn i_06002e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60030;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060030: jsr $2870a.l
pub fn i_060030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60036);
    cpu.pc = 0x2870A;
    16
}
/// $060036: bcc.w $60040
pub fn i_060036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6003A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60040; return 10; }
    8
}
/// $06003A: lea.l $60210(pc), a1
pub fn i_06003a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6003E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $06003E: move.l a1, (a6)
pub fn i_06003e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60040;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060040: jsr $6037c(pc)
pub fn i_060040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60044;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60044);
    cpu.pc = 0x6037C;
    16
}
/// $060044: bcc.w $6004e
pub fn i_060044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60048;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6004E; return 10; }
    8
}
/// $060048: jmp $518.l
pub fn i_060048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6004E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06004E: rts
pub fn i_06004e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60050;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Tmpl23_060050  ($060050, 10 B, props/late_props_turrets_05e000.s)
/// $060050: cmpi.w #$140, $22(a6)
pub fn i_060050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60056;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $060056: blt.w $60062
pub fn i_060056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6005A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x60062; return 10; }
    8
}
// ===== JsrAbsThunk_06005a  ($06005A, 8 B, thunks/jsr_abs_thunks.c)
/// $06005A: jsr $2783a.l
pub fn i_06005a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60060;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60060);
    cpu.pc = 0x2783A;
    16
}
/// $060060: rts
pub fn i_060060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60062;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable23_Init_060062  ($060062, 146 B, props/late_props_turrets_05e000.s)
/// $060062: move.w #$e, d1
pub fn i_060062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60066;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060066: jsr $236e.l
pub fn i_060066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6006C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6006C);
    cpu.pc = 0x236E;
    16
}
/// $06006C: move.b #$ff, $32(a6)
pub fn i_06006c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60072;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060072: move.b #$ff, $33(a6)
pub fn i_060072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60078;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060078: move.b $98(a6), d0
pub fn i_060078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6007C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06007C: andi.b #$1, d0
pub fn i_06007c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60080;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $060080: move.b d0, $3a(a6)
pub fn i_060080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60084;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060084: move.w #$8000, $38(a6)
pub fn i_060084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6008A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06008A: move.w #$1, $66(a6)
pub fn i_06008a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60090;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060090: jsr $267e2.l
pub fn i_060090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60096);
    cpu.pc = 0x267E2;
    16
}
/// $060096: lea.l $2c1d44.l, a0
pub fn i_060096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6009C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1D44;
    20
}
/// $06009C: jsr $28cd4.l
pub fn i_06009c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x600A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0600A2: lea.l $600a8(pc), a1
pub fn i_0600a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x600A8;
    16
}
/// $0600A6: move.l a1, (a6)
pub fn i_0600a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0600A8: jsr $2783a.l
pub fn i_0600a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x600AE);
    cpu.pc = 0x2783A;
    16
}
/// $0600AE: jsr $28d70.l
pub fn i_0600ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x600B4);
    cpu.pc = 0x28D70;
    16
}
/// $0600B4: jsr $5e0d4.l
pub fn i_0600b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x600BA);
    cpu.pc = 0x5E0D4;
    16
}
/// $0600BA: move.w $22(a0), d0
pub fn i_0600ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600BE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0600BE: sub.w $22(a6), d0
pub fn i_0600be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600C2;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0600C2: jsr $603b4(pc)
pub fn i_0600c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x600C6);
    cpu.pc = 0x603B4;
    16
}
/// $0600C6: cmpi.w #$20, d0
pub fn i_0600c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600CA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $0600CA: bgt.w $600d4
pub fn i_0600ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600CE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x600D4; return 10; }
    8
}
/// $0600CE: lea.l $600f4(pc), a1
pub fn i_0600ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x600F4;
    16
}
/// $0600D2: move.l a1, (a6)
pub fn i_0600d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0600D4: jsr $2870a.l
pub fn i_0600d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x600DA);
    cpu.pc = 0x2870A;
    16
}
/// $0600DA: bcc.w $600e4
pub fn i_0600da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x600E4; return 10; }
    8
}
/// $0600DE: lea.l $60210(pc), a1
pub fn i_0600de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $0600E2: move.l a1, (a6)
pub fn i_0600e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0600E4: jsr $6037c(pc)
pub fn i_0600e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x600E8);
    cpu.pc = 0x6037C;
    16
}
/// $0600E8: bcc.w $600f2
pub fn i_0600e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x600F2; return 10; }
    8
}
/// $0600EC: jmp $518.l
pub fn i_0600ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0600F2: rts
pub fn i_0600f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable23_Destroyed_0600f4  ($0600F4, 72 B, props/late_props_turrets_05e000.s)
/// $0600F4: lea.l $2c1d88.l, a0
pub fn i_0600f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x600FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1D88;
    20
}
/// $0600FA: jsr $28cd4.l
pub fn i_0600fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60100);
    cpu.pc = 0x28CD4;
    16
}
/// $060100: lea.l $60106(pc), a1
pub fn i_060100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60104;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60106;
    16
}
/// $060104: move.l a1, (a6)
pub fn i_060104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60106;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060106: jsr $2783a.l
pub fn i_060106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6010C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6010C);
    cpu.pc = 0x2783A;
    16
}
/// $06010C: jsr $28d70.l
pub fn i_06010c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60112;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60112);
    cpu.pc = 0x28D70;
    16
}
/// $060112: bcc.w $6011c
pub fn i_060112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60116;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6011C; return 10; }
    8
}
/// $060116: lea.l $5fe26(pc), a1
pub fn i_060116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6011A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $06011A: move.l a1, (a6)
pub fn i_06011a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6011C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06011C: jsr $2870a.l
pub fn i_06011c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60122;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60122);
    cpu.pc = 0x2870A;
    16
}
/// $060122: bcc.w $6012c
pub fn i_060122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60126;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6012C; return 10; }
    8
}
/// $060126: lea.l $60210(pc), a1
pub fn i_060126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6012A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $06012A: move.l a1, (a6)
pub fn i_06012a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6012C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06012C: jsr $6037c(pc)
pub fn i_06012c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60130;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60130);
    cpu.pc = 0x6037C;
    16
}
/// $060130: bcc.w $6013a
pub fn i_060130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60134;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6013A; return 10; }
    8
}
/// $060134: jmp $518.l
pub fn i_060134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6013A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06013A: rts
pub fn i_06013a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6013C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Tmpl24_06013c  ($06013C, 70 B, props/late_props_turrets_05e000.s)
/// $06013C: lea.l $2c1d98.l, a0
pub fn i_06013c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60142;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1D98;
    20
}
/// $060142: jsr $28cd4.l
pub fn i_060142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60148);
    cpu.pc = 0x28CD4;
    16
}
/// $060148: bra.w $60178
pub fn i_060148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6014C;
    cpu.ir = 0x6000;
    cpu.pc = 0x60178;
    10
}
/// $060178: cmpi.w #$140, $22(a6)
pub fn i_060178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6017E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $06017E: blt.w $6018a
pub fn i_06017e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60182;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6018A; return 10; }
    8
}
// ===== JsrAbsThunk_060182  ($060182, 8 B, thunks/jsr_abs_thunks.c)
/// $060182: jsr $2783a.l
pub fn i_060182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60188);
    cpu.pc = 0x2783A;
    16
}
/// $060188: rts
pub fn i_060188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6018A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable24_Init_06018a  ($06018A, 134 B, props/late_props_turrets_05e000.s)
/// $06018A: move.w #$e, d1
pub fn i_06018a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6018E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06018E: jsr $236e.l
pub fn i_06018e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60194);
    cpu.pc = 0x236E;
    16
}
/// $060194: move.b #$ff, $32(a6)
pub fn i_060194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6019A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06019A: move.b #$ff, $33(a6)
pub fn i_06019a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0601A0: move.b $98(a6), d0
pub fn i_0601a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601A4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0601A4: andi.b #$1, d0
pub fn i_0601a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601A8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0601A8: move.b d0, $3a(a6)
pub fn i_0601a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601AC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0601AC: move.w #$8000, $38(a6)
pub fn i_0601ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0601B2: move.w #$1, $66(a6)
pub fn i_0601b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0601B8: jsr $267e2.l
pub fn i_0601b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x601BE);
    cpu.pc = 0x267E2;
    16
}
/// $0601BE: lea.l $601c4(pc), a1
pub fn i_0601be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x601C4;
    16
}
/// $0601C2: move.l a1, (a6)
pub fn i_0601c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0601C4: jsr $2783a.l
pub fn i_0601c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x601CA);
    cpu.pc = 0x2783A;
    16
}
/// $0601CA: jsr $28d70.l
pub fn i_0601ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x601D0);
    cpu.pc = 0x28D70;
    16
}
/// $0601D0: jsr $5e0d4.l
pub fn i_0601d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x601D6);
    cpu.pc = 0x5E0D4;
    16
}
/// $0601D6: move.w $22(a0), d0
pub fn i_0601d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601DA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0601DA: sub.w $22(a6), d0
pub fn i_0601da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601DE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0601DE: jsr $603b4(pc)
pub fn i_0601de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x601E2);
    cpu.pc = 0x603B4;
    16
}
/// $0601E2: cmpi.w #$20, d0
pub fn i_0601e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601E6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $0601E6: bgt.w $601f0
pub fn i_0601e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x601F0; return 10; }
    8
}
/// $0601EA: lea.l $5fe26(pc), a1
pub fn i_0601ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $0601EE: move.l a1, (a6)
pub fn i_0601ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0601F0: jsr $2870a.l
pub fn i_0601f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x601F6);
    cpu.pc = 0x2870A;
    16
}
/// $0601F6: bcc.w $60200
pub fn i_0601f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60200; return 10; }
    8
}
/// $0601FA: lea.l $60210(pc), a1
pub fn i_0601fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x601FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $0601FE: move.l a1, (a6)
pub fn i_0601fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60200;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060200: jsr $6037c(pc)
pub fn i_060200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60204;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60204);
    cpu.pc = 0x6037C;
    16
}
/// $060204: bcc.w $6020e
pub fn i_060204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60208;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6020E; return 10; }
    8
}
/// $060208: jmp $518.l
pub fn i_060208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6020E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06020E: rts
pub fn i_06020e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60210;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_SpawnPiece_060210  ($060210, 78 B, props/late_props_turrets_05e000.s)
/// $060210: jsr $519be.l
pub fn i_060210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60216;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60216);
    cpu.pc = 0x519BE;
    16
}
/// $060216: move.w #$2, d0
pub fn i_060216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6021A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06021A: jsr $5e9e4.l
pub fn i_06021a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60220);
    cpu.pc = 0x5E9E4;
    16
}
/// $060220: movea.l #$2c20b8, a0
pub fn i_060220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60226;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C20B8;
    cpu.a[0] = v1;
    4
}
/// $060226: lsl.w #$2, d0
pub fn i_060226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60228;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $060228: movea.l (a0, d0.w), a0
pub fn i_060228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6022C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06022C: cmpa.l #$ffffffff, a0
pub fn i_06022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60232;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $060232: beq.w $6023c
pub fn i_060232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60236;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6023C; return 10; }
    8
}
/// $060236: jsr $28cd4.l
pub fn i_060236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6023C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6023C);
    cpu.pc = 0x28CD4;
    16
}
/// $06023C: lea.l $60242(pc), a1
pub fn i_06023c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60240;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60242;
    16
}
/// $060240: move.l a1, (a6)
pub fn i_060240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60242;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060242: jsr $2783a.l
pub fn i_060242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60248);
    cpu.pc = 0x2783A;
    16
}
/// $060248: jsr $28d70.l
pub fn i_060248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6024E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6024E);
    cpu.pc = 0x28D70;
    16
}
/// $06024E: bcc.w $6025c
pub fn i_06024e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60252;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6025C; return 10; }
    8
}
/// $060252: jsr $603c4(pc)
pub fn i_060252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60256;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60256);
    cpu.pc = 0x603C4;
    16
}
/// $060256: jmp $518.l
pub fn i_060256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6025C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06025C: rts
pub fn i_06025c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6025E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Shard_06025e  ($06025E, 196 B, props/late_props_turrets_05e000.s)
/// $06025E: move.w #$15d, d1
pub fn i_06025e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60262;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060262: jsr $236e.l
pub fn i_060262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60268);
    cpu.pc = 0x236E;
    16
}
/// $060268: move.w #$164, d1
pub fn i_060268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6026C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x164;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06026C: jsr $236e.l
pub fn i_06026c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60272);
    cpu.pc = 0x236E;
    16
}
/// $060272: move.w #$165, d1
pub fn i_060272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60276;
    cpu.ir = 0x323C;
    let v1: u32 = 0x165;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060276: jsr $236e.l
pub fn i_060276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6027C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6027C);
    cpu.pc = 0x236E;
    16
}
/// $06027C: move.b #$ff, $32(a6)
pub fn i_06027c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60282;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060282: move.b #$ff, $33(a6)
pub fn i_060282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60288;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060288: move.w #$d000, $38(a6)
pub fn i_060288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6028E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06028E: movea.l $c(a6), a0
pub fn i_06028e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60292;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $060292: clr.b $3a(a6)
pub fn i_060292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60296;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $060296: lea.l $2c2168.l, a0
pub fn i_060296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6029C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2168;
    20
}
/// $06029C: move.l a0, $4c(a6)
pub fn i_06029c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602A0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0602A0: jsr $283ca.l
pub fn i_0602a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x602A6);
    cpu.pc = 0x283CA;
    16
}
/// $0602A6: move.w #$0, $70(a6)
pub fn i_0602a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0602AC: lea.l $2c72c0.l, a0
pub fn i_0602ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C72C0;
    20
}
/// $0602B2: jsr $28cd4.l
pub fn i_0602b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x602B8);
    cpu.pc = 0x28CD4;
    16
}
/// $0602B8: lea.l $602be(pc), a1
pub fn i_0602b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x602BE;
    16
}
/// $0602BC: move.l a1, (a6)
pub fn i_0602bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0602BE: jsr $27cee.l
pub fn i_0602be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x602C4);
    cpu.pc = 0x27CEE;
    16
}
/// $0602C4: bcs.w $60302
pub fn i_0602c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x60302; return 10; }
    8
}
/// $0602C8: jsr $60322(pc)
pub fn i_0602c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x602CC);
    cpu.pc = 0x60322;
    16
}
/// $0602CC: move.w $28(a6), d0
pub fn i_0602cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0602D0: move.w $2a(a6), d1
pub fn i_0602d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0602D4: asr.w #$4, d0
pub fn i_0602d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602D6;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0602D6: asr.w #$4, d1
pub fn i_0602d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602D8;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0602D8: jsr $5e018.l
pub fn i_0602d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x602DE);
    cpu.pc = 0x5E018;
    16
}
/// $0602DE: lsr.w #$3, d0
pub fn i_0602de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602E0;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0602E0: move.w d0, $30(a6)
pub fn i_0602e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602E4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0602E4: jsr $28d70.l
pub fn i_0602e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x602EA);
    cpu.pc = 0x28D70;
    16
}
/// $0602EA: jsr $283d8.l
pub fn i_0602ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x602F0);
    cpu.pc = 0x283D8;
    16
}
/// $0602F0: btst.b #$1, $13(a6)
pub fn i_0602f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602F6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0602F6: bne.w $60302
pub fn i_0602f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60302; return 10; }
    8
}
/// $0602FA: jsr $60398(pc)
pub fn i_0602fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x602FE);
    cpu.pc = 0x60398;
    16
}
/// $0602FE: bcc.w $60320
pub fn i_0602fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60302;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60320; return 10; }
    8
}
/// $060302: lea.l $77eda.l, a1
pub fn i_060302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60308;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EDA;
    20
}
/// $060308: jsr $4ae.l
pub fn i_060308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6030E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6030E);
    cpu.pc = 0x4AE;
    16
}
/// $06030E: jsr $5dd02.l
pub fn i_06030e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60314);
    cpu.pc = 0x5DD02;
    16
}
/// $060314: move.w #$4000, $38(a0)
pub fn i_060314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6031A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06031A: jmp $518.l
pub fn i_06031a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60320;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $060320: rts
pub fn i_060320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60322;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_ShardVel_060322  ($060322, 42 B, props/late_props_turrets_05e000.s)
/// $060322: move.w $70(a6), d0
pub fn i_060322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60326;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $060326: andi.l #$3, d0
pub fn i_060326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6032C;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $06032C: asl.b #$1, d0
pub fn i_06032c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6032E;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $06032E: move.w $16(a6, d0.w), $14(a6)
pub fn i_06032e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60334;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $060334: addi.w #$1, $70(a6)
pub fn i_060334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6033A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06033A: cmpi.w #$3, $70(a6)
pub fn i_06033a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60340;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $060340: blt.w $6034a
pub fn i_060340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60344;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6034A; return 10; }
    8
}
/// $060344: move.w #$0, $70(a6)
pub fn i_060344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6034A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06034A: rts
pub fn i_06034a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6034C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_SpawnShards_06034c  ($06034C, 48 B, props/late_props_turrets_05e000.s)
/// $06034C: lea.l $6025e(pc), a1
pub fn i_06034c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60350;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6025E;
    16
}
/// $060350: jsr $4ae.l
pub fn i_060350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60356);
    cpu.pc = 0x4AE;
    16
}
/// $060356: jsr $5dd02.l
pub fn i_060356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6035C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6035C);
    cpu.pc = 0x5DD02;
    16
}
/// $06035C: addi.w #$18, $24(a0)
pub fn i_06035c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60362;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $060362: move.w $28(a6), $28(a0)
pub fn i_060362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60368;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $060368: move.w #$400, $2a(a0)
pub fn i_060368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6036E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06036E: move.w #$0, $2c(a0)
pub fn i_06036e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60374;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060374: addi.w #$ffe0, $2e(a0)
pub fn i_060374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6037A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06037A: rts
pub fn i_06037a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6037C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C21BC_06037c  ($06037C, 16 B, props/late_props_turrets_05e000.s)
/// $06037C: lea.l $2c21bc.l, a0
pub fn i_06037c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60382;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C21BC;
    20
}
/// $060382: jsr $5dd56.l
pub fn i_060382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60388;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60388);
    cpu.pc = 0x5DD56;
    16
}
/// $060388: bcc.w $60392
pub fn i_060388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6038C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60392; return 10; }
    8
}
// ===== SetC_06038c  ($06038C, 6 B, thunks/ccr_helpers.c)
/// $06038C: ori.b #$1, ccr
pub fn i_06038c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60390;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $060390: rts
pub fn i_060390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_060392  ($060392, 6 B, thunks/ccr_helpers.c)
/// $060392: andi.b #$fe, ccr
pub fn i_060392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60396;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $060396: rts
pub fn i_060396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60398;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C21C6_060398  ($060398, 16 B, props/late_props_turrets_05e000.s)
/// $060398: lea.l $2c21c6.l, a0
pub fn i_060398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6039E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C21C6;
    20
}
/// $06039E: jsr $5dd5c.l
pub fn i_06039e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x603A4);
    cpu.pc = 0x5DD5C;
    16
}
/// $0603A4: bcc.w $603ae
pub fn i_0603a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x603AE; return 10; }
    8
}
// ===== SetC_0603a8  ($0603A8, 6 B, thunks/ccr_helpers.c)
/// $0603A8: ori.b #$1, ccr
pub fn i_0603a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603AC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0603AC: rts
pub fn i_0603ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_0603ae  ($0603AE, 6 B, thunks/ccr_helpers.c)
/// $0603AE: andi.b #$fe, ccr
pub fn i_0603ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603B2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0603B2: rts
pub fn i_0603b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Facing_FromAngleHi_0603b4  ($0603B4, 16 B, props/late_props_turrets_05e000.s)
/// $0603B4: move.w d0, d1
pub fn i_0603b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603B6;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0603B6: asr.w #$8, d1
pub fn i_0603b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603B8;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0603B8: btst.b #$7, d1
pub fn i_0603b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603BC;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0603BC: beq.w $603c2
pub fn i_0603bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x603C2; return 10; }
    8
}
/// $0603C0: neg.w d0
pub fn i_0603c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0603C2: rts
pub fn i_0603c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_PieceRng_0603c4  ($0603C4, 64 B, props/late_props_turrets_05e000.s)
/// $0603C4: subq.b #$1, $99(a6)
pub fn i_0603c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603C8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0603C8: bmi.w $60402
pub fn i_0603c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603CC;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x60402; return 10; }
    8
}
/// $0603CC: jsr $5e9b6.l
pub fn i_0603cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x603D2);
    cpu.pc = 0x5E9B6;
    16
}
/// $0603D2: andi.l #$3, d0
pub fn i_0603d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603D8;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $0603D8: cmpi.l #$3, d0
pub fn i_0603d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603DE;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Long, false);
    8
}
/// $0603DE: bne.w $603e4
pub fn i_0603de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x603E4; return 10; }
    8
}
/// $0603E2: moveq #$2, d0
pub fn i_0603e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603E4;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0603E4: asl.l #$2, d0
pub fn i_0603e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603E6;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $0603E6: lea.l $2c21d0.l, a0
pub fn i_0603e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C21D0;
    20
}
/// $0603EC: movea.l (a0, d0.w), a1
pub fn i_0603ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603F0;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0603F0: jsr $4ae.l
pub fn i_0603f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x603F6);
    cpu.pc = 0x4AE;
    16
}
/// $0603F6: jsr $5dd02.l
pub fn i_0603f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x603FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x603FC);
    cpu.pc = 0x5DD02;
    16
}
/// $0603FC: move.b $99(a6), $99(a0)
pub fn i_0603fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60402;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    20
}
/// $060402: rts
pub fn i_060402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60404;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Stub_060404  ($060404, 16 B, props/late_props_turrets_05e000.s)
/// $060404: movea.l $8(a6), a1
pub fn i_060404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60408;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $060408: move.b $10(a6), d0
pub fn i_060408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6040C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06040C: cmp.b $10(a1), d0
pub fn i_06040c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60410;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $060410: bcs.w $6041a
pub fn i_060410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60414;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6041A; return 10; }
    8
}
// ===== ClearXN_060414  ($060414, 6 B, thunks/ccr_helpers.c)
/// $060414: andi.b #$ee, ccr
pub fn i_060414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60418;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $060418: rts
pub fn i_060418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6041A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06041a  ($06041A, 6 B, thunks/ccr_helpers.c)
/// $06041A: ori.b #$11, ccr
pub fn i_06041a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6041E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06041E: rts
pub fn i_06041e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60420;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sign_Tmpl28_060420  ($060420, 266 B, props/late_props_turrets_05e000.s)
/// $060420: ori.b #$1, $3a(a6)
pub fn i_060420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60426;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060426: lea.l $2c21dc.l, a0
pub fn i_060426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6042C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C21DC;
    20
}
/// $06042C: jsr $28cd4.l
pub fn i_06042c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60432);
    cpu.pc = 0x28CD4;
    16
}
/// $060432: lea.l $573a2.l, a0
pub fn i_060432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60438;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x573A2;
    20
}
/// $060438: move.l a0, $70(a6)
pub fn i_060438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6043C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06043C: bra.w $604bc
pub fn i_06043c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60440;
    cpu.ir = 0x6000;
    cpu.pc = 0x604BC;
    10
}
/// $060440: ori.b #$1, $3a(a6)
pub fn i_060440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60446;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060446: lea.l $2c21dc.l, a0
pub fn i_060446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6044C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C21DC;
    20
}
/// $06044C: jsr $28cd4.l
pub fn i_06044c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60452);
    cpu.pc = 0x28CD4;
    16
}
/// $060452: lea.l $572be.l, a0
pub fn i_060452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60458;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x572BE;
    20
}
/// $060458: move.l a0, $70(a6)
pub fn i_060458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6045C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06045C: bra.w $604bc
pub fn i_06045c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60460;
    cpu.ir = 0x6000;
    cpu.pc = 0x604BC;
    10
}
/// $060460: ori.b #$1, $3a(a6)
pub fn i_060460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60466;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060466: lea.l $2c09ac.l, a0
pub fn i_060466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6046C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C09AC;
    20
}
/// $06046C: jsr $28cd4.l
pub fn i_06046c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60472);
    cpu.pc = 0x28CD4;
    16
}
/// $060472: lea.l $573a2.l, a0
pub fn i_060472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60478;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x573A2;
    20
}
/// $060478: move.l a0, $70(a6)
pub fn i_060478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6047C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06047C: bra.w $604bc
pub fn i_06047c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60480;
    cpu.ir = 0x6000;
    cpu.pc = 0x604BC;
    10
}
/// $0604BC: move.w #$e, d1
pub fn i_0604bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0604C0: jsr $236e.l
pub fn i_0604c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x604C6);
    cpu.pc = 0x236E;
    16
}
/// $0604C6: move.b #$ff, $32(a6)
pub fn i_0604c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0604CC: move.b #$ff, $33(a6)
pub fn i_0604cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0604D2: move.w #$8000, $38(a6)
pub fn i_0604d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0604D8: jsr $267e2.l
pub fn i_0604d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x604DE);
    cpu.pc = 0x267E2;
    16
}
/// $0604DE: lea.l $604e4(pc), a1
pub fn i_0604de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x604E4;
    16
}
/// $0604E2: move.l a1, (a6)
pub fn i_0604e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0604E4: jsr $2783a.l
pub fn i_0604e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x604EA);
    cpu.pc = 0x2783A;
    16
}
/// $0604EA: jsr $28d70.l
pub fn i_0604ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x604F0);
    cpu.pc = 0x28D70;
    16
}
/// $0604F0: bcc.w $6051a
pub fn i_0604f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6051A; return 10; }
    8
}
/// $0604F4: movea.l $70(a6), a1
pub fn i_0604f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604F8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0604F8: jsr $4498e.l
pub fn i_0604f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x604FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x604FE);
    cpu.pc = 0x4498E;
    16
}
/// $0604FE: jsr $5dd02.l
pub fn i_0604fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60504);
    cpu.pc = 0x5DD02;
    16
}
/// $060504: move.l $98(a6), $98(a0)
pub fn i_060504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6050A;
    cpu.ir = 0x216E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $06050A: move.l $9c(a6), $9c(a0)
pub fn i_06050a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60510;
    cpu.ir = 0x216E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x9C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $060510: lea.l $6052a(pc), a1
pub fn i_060510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60514;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6052A;
    16
}
/// $060514: move.l a1, (a6)
pub fn i_060514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60516;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060516: bra.w $60528
pub fn i_060516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6051A;
    cpu.ir = 0x6000;
    cpu.pc = 0x60528;
    10
}
/// $06051A: jsr $6053e(pc)
pub fn i_06051a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6051E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6051E);
    cpu.pc = 0x6053E;
    16
}
/// $06051E: bcc.w $60528
pub fn i_06051e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60522;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60528; return 10; }
    8
}
/// $060522: jmp $518.l
pub fn i_060522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60528;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $060528: rts
pub fn i_060528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6052A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sign_Idle_06052a  ($06052A, 18 B, props/late_props_turrets_05e000.s)
/// $06052A: jsr $2783a.l
pub fn i_06052a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60530);
    cpu.pc = 0x2783A;
    16
}
/// $060530: jsr $28d70.l
pub fn i_060530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60536);
    cpu.pc = 0x28D70;
    16
}
/// $060536: jmp $518.l
pub fn i_060536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6053C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Sign_Rts_06053c  ($06053C, 2 B, props/late_props_turrets_05e000.s)
/// $06053C: rts
pub fn i_06053c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6053E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C2314_06053e  ($06053E, 16 B, props/late_props_turrets_05e000.s)
/// $06053E: lea.l $2c2314.l, a0
pub fn i_06053e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60544;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2314;
    20
}
/// $060544: jsr $5dd5c.l
pub fn i_060544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6054A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6054A);
    cpu.pc = 0x5DD5C;
    16
}
/// $06054A: bcc.w $60554
pub fn i_06054a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6054E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60554; return 10; }
    8
}
// ===== SetC_06054e  ($06054E, 6 B, thunks/ccr_helpers.c)
/// $06054E: ori.b #$1, ccr
pub fn i_06054e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60552;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $060552: rts
pub fn i_060552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60554;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_060554  ($060554, 6 B, thunks/ccr_helpers.c)
/// $060554: andi.b #$fe, ccr
pub fn i_060554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60558;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $060558: rts
pub fn i_060558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6055A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sign_Free_06055a  ($06055A, 16 B, props/late_props_turrets_05e000.s)
/// $06055A: movea.l $8(a6), a1
pub fn i_06055a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6055E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06055E: move.b $10(a6), d0
pub fn i_06055e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60562;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $060562: cmp.b $10(a1), d0
pub fn i_060562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60566;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $060566: bcs.w $60570
pub fn i_060566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6056A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x60570; return 10; }
    8
}
// ===== ClearXN_06056a  ($06056A, 6 B, thunks/ccr_helpers.c)
/// $06056A: andi.b #$ee, ccr
pub fn i_06056a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6056E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06056E: rts
pub fn i_06056e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60570;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_060570  ($060570, 6 B, thunks/ccr_helpers.c)
/// $060570: ori.b #$11, ccr
pub fn i_060570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60574;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $060574: rts
pub fn i_060574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60576;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HomingMarker_Targets_060576  ($060576, 196 B, props/late_props_turrets_05e000.s)
/// $060576: ori.b #$0, d0
pub fn i_060576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6057A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06057A: ori.b #$0, d0
pub fn i_06057a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6057E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06057E: ori.b #$0, d0
pub fn i_06057e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60582;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $060582: ori.b #$0, d0
pub fn i_060582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60586;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $060586: dc.w $fe00
pub fn i_060586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60588;
    cpu.ir = 0xFE00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== ItemProp_Tmpl114_06063a  ($06063A, 34 B, props/late_props_turrets_05e000.s)
/// $06063A: lea.l $2c25fa.l, a1
pub fn i_06063a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60640;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C25FA;
    20
}
/// $060640: jsr $43fac.l
pub fn i_060640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60646);
    cpu.pc = 0x43FAC;
    16
}
/// $060646: move.w #$e, d1
pub fn i_060646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6064A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06064A: jsr $236e.l
pub fn i_06064a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60650);
    cpu.pc = 0x236E;
    16
}
/// $060650: move.l #$2507fa, $3c(a6)
pub fn i_060650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60658;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2507FA;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $060658: bra.w $606e6
pub fn i_060658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6065C;
    cpu.ir = 0x6000;
    cpu.pc = 0x606E6;
    10
}
// ===== ItemProp_Tmpl115_06065c  ($06065C, 34 B, props/late_props_turrets_05e000.s)
/// $06065C: lea.l $2c2606.l, a1
pub fn i_06065c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60662;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2606;
    20
}
/// $060662: jsr $43fac.l
pub fn i_060662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60668);
    cpu.pc = 0x43FAC;
    16
}
/// $060668: move.w #$e, d1
pub fn i_060668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6066C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06066C: jsr $236e.l
pub fn i_06066c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60672;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60672);
    cpu.pc = 0x236E;
    16
}
/// $060672: move.l #$25086e, $3c(a6)
pub fn i_060672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6067A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x25086E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06067A: bra.w $606e6
pub fn i_06067a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6067E;
    cpu.ir = 0x6000;
    cpu.pc = 0x606E6;
    10
}
// ===== ItemProp_Tmpl116_06067e  ($06067E, 34 B, props/late_props_turrets_05e000.s)
/// $06067E: lea.l $2c2622.l, a1
pub fn i_06067e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60684;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2622;
    20
}
/// $060684: jsr $43fac.l
pub fn i_060684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6068A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6068A);
    cpu.pc = 0x43FAC;
    16
}
/// $06068A: move.w #$e, d1
pub fn i_06068a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6068E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06068E: jsr $236e.l
pub fn i_06068e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60694);
    cpu.pc = 0x236E;
    16
}
/// $060694: move.l #$250838, $3c(a6)
pub fn i_060694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6069C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x250838;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06069C: bra.w $606e6
pub fn i_06069c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x606E6;
    10
}
// ===== ItemProp_Tmpl117_0606a0  ($0606A0, 34 B, props/late_props_turrets_05e000.s)
/// $0606A0: lea.l $2c2614.l, a1
pub fn i_0606a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606A6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2614;
    20
}
/// $0606A6: jsr $43fac.l
pub fn i_0606a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x606AC);
    cpu.pc = 0x43FAC;
    16
}
/// $0606AC: move.w #$e, d1
pub fn i_0606ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0606B0: jsr $236e.l
pub fn i_0606b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x606B6);
    cpu.pc = 0x236E;
    16
}
/// $0606B6: move.l #$25084a, $3c(a6)
pub fn i_0606b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606BE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x25084A;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0606BE: bra.w $606e6
pub fn i_0606be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x606E6;
    10
}
// ===== ItemProp_Tmpl118_0606c2  ($0606C2, 34 B, props/late_props_turrets_05e000.s)
/// $0606C2: lea.l $2c2630.l, a1
pub fn i_0606c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606C8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2630;
    20
}
/// $0606C8: jsr $43fac.l
pub fn i_0606c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x606CE);
    cpu.pc = 0x43FAC;
    16
}
/// $0606CE: move.w #$e, d1
pub fn i_0606ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0606D2: jsr $236e.l
pub fn i_0606d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x606D8);
    cpu.pc = 0x236E;
    16
}
/// $0606D8: move.l #$25085c, $3c(a6)
pub fn i_0606d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606E0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x25085C;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0606E0: bra.w $606e6
pub fn i_0606e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x606E6;
    10
}
// ===== ItemProp_Rts_0606e4  ($0606E4, 2 B, props/late_props_turrets_05e000.s)
/// $0606E4: nop
pub fn i_0606e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606E6;
    cpu.ir = 0x4E71;
    4
}
// ===== SetTaskHandler_0606e6  ($0606E6, 8 B, kernel/task_handlers.c)
/// $0606E6: lea.l $606ee(pc), a1
pub fn i_0606e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x606EE;
    16
}
/// $0606EA: move.l a1, (a6)
pub fn i_0606ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0606EC: rts
pub fn i_0606ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ItemProp_Idle_0606ee  ($0606EE, 16 B, props/late_props_turrets_05e000.s)
/// $0606EE: jsr $2783a.l
pub fn i_0606ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x606F4);
    cpu.pc = 0x2783A;
    16
}
/// $0606F4: jsr $5ca2a.l
pub fn i_0606f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x606FA);
    cpu.pc = 0x5CA2A;
    16
}
/// $0606FA: jmp $60706(pc)
pub fn i_0606fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606FE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x60706;
    8
}
// ===== ItemProp_Wait_0606fe  ($0606FE, 30 B, props/late_props_turrets_05e000.s)
/// $0606FE: dc.w $ffd0
pub fn i_0606fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60700;
    cpu.ir = 0xFFD0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $060706: lea.l $606fe(pc), a0
pub fn i_060706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6070A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x606FE;
    16
}
/// $06070A: jsr $5dd5c.l
pub fn i_06070a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60710);
    cpu.pc = 0x5DD5C;
    16
}
/// $060710: bcc.w $6071a
pub fn i_060710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60714;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6071A; return 10; }
    8
}
/// $060714: jmp $518.l
pub fn i_060714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6071A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06071A: rts
pub fn i_06071a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6071C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_CommonInit_06071c  ($06071C, 64 B, props/late_props_turrets_05e000.s)
/// $06071C: move.w #$6c, d1
pub fn i_06071c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60720;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060720: jsr $236e.l
pub fn i_060720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60726;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60726);
    cpu.pc = 0x236E;
    16
}
/// $060726: move.w #$8000, d0
pub fn i_060726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6072A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06072A: jsr $28134.l
pub fn i_06072a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60730;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60730);
    cpu.pc = 0x28134;
    16
}
/// $060730: andi.w #$ffe3, $38(a6)
pub fn i_060730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60736;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $060736: ori.w #$c, $38(a6)
pub fn i_060736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6073C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06073C: lea.l $2c231e.l, a0
pub fn i_06073c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60742;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C231E;
    20
}
/// $060742: move.l a0, $48(a6)
pub fn i_060742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60746;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $060746: lea.l $2bf414.l, a0
pub fn i_060746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6074C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF414;
    20
}
/// $06074C: jsr $799de.l
pub fn i_06074c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60752;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60752);
    cpu.pc = 0x799DE;
    16
}
/// $060752: move.w d0, $66(a6)
pub fn i_060752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60756;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060756: lea.l $2c252e.l, a1
pub fn i_060756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6075C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C252E;
    20
}
// ===== JsrAbsThunk_06075c  ($06075C, 8 B, thunks/jsr_abs_thunks.c)
/// $06075C: jsr $43fac.l
pub fn i_06075c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60762;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60762);
    cpu.pc = 0x43FAC;
    16
}
/// $060762: rts
pub fn i_060762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60764;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Tmpl110_060764  ($060764, 40 B, props/late_props_turrets_05e000.s)
/// $060764: move.b #$2, $70(a6)
pub fn i_060764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6076A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06076A: bra.w $6077c
pub fn i_06076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6076E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6077C;
    10
}
/// $06077C: jsr $6071c(pc)
pub fn i_06077c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60780;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60780);
    cpu.pc = 0x6071C;
    16
}
/// $060780: lea.l $2c2416.l, a0
pub fn i_060780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60786;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2416;
    20
}
/// $060786: jsr $28cd4.l
pub fn i_060786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6078C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6078C);
    cpu.pc = 0x28CD4;
    16
}
// ===== Obstacle_Idle_06078c  ($06078C, 68 B, props/late_props_turrets_05e000.s)
/// $06078C: bclr.b #$3, $13(a6)
pub fn i_06078c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60792;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060792: lea.l $60798(pc), a1
pub fn i_060792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60796;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60798;
    16
}
/// $060796: move.l a1, (a6)
pub fn i_060796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60798;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060798: jsr $2783a.l
pub fn i_060798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6079E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6079E);
    cpu.pc = 0x2783A;
    16
}
/// $06079E: jsr $28d70.l
pub fn i_06079e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x607A4);
    cpu.pc = 0x28D70;
    16
}
/// $0607A4: jsr $2870a.l
pub fn i_0607a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x607AA);
    cpu.pc = 0x2870A;
    16
}
/// $0607AA: bcc.w $607b4
pub fn i_0607aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x607B4; return 10; }
    8
}
/// $0607AE: lea.l $607d0(pc), a1
pub fn i_0607ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x607D0;
    16
}
/// $0607B2: move.l a1, (a6)
pub fn i_0607b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0607B4: bclr.b #$3, $13(a6)
pub fn i_0607b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0607BA: jsr $28758.l
pub fn i_0607ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x607C0);
    cpu.pc = 0x28758;
    16
}
/// $0607C0: bcc.w $607ca
pub fn i_0607c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x607CA; return 10; }
    8
}
/// $0607C4: lea.l $60814(pc), a1
pub fn i_0607c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60814;
    16
}
/// $0607C8: move.l a1, (a6)
pub fn i_0607c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0607CA: bsr.w $60706
pub fn i_0607ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607CE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x607CE);
    cpu.pc = 0x60706;
    18
}
/// $0607CE: rts
pub fn i_0607ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Damaged_0607d0  ($0607D0, 68 B, props/late_props_turrets_05e000.s)
/// $0607D0: lea.l $2c2422.l, a0
pub fn i_0607d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2422;
    20
}
/// $0607D6: jsr $28cd4.l
pub fn i_0607d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x607DC);
    cpu.pc = 0x28CD4;
    16
}
/// $0607DC: lea.l $607e2(pc), a1
pub fn i_0607dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x607E2;
    16
}
/// $0607E0: move.l a1, (a6)
pub fn i_0607e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0607E2: jsr $2783a.l
pub fn i_0607e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x607E8);
    cpu.pc = 0x2783A;
    16
}
/// $0607E8: jsr $28d70.l
pub fn i_0607e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x607EE);
    cpu.pc = 0x28D70;
    16
}
/// $0607EE: bcc.w $607f8
pub fn i_0607ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x607F8; return 10; }
    8
}
/// $0607F2: lea.l $6078c(pc), a1
pub fn i_0607f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6078C;
    16
}
/// $0607F6: move.l a1, (a6)
pub fn i_0607f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0607F8: bclr.b #$3, $13(a6)
pub fn i_0607f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x607FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0607FE: jsr $28758.l
pub fn i_0607fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60804);
    cpu.pc = 0x28758;
    16
}
/// $060804: bcc.w $6080e
pub fn i_060804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60808;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6080E; return 10; }
    8
}
/// $060808: lea.l $60814(pc), a1
pub fn i_060808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6080C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60814;
    16
}
/// $06080C: move.l a1, (a6)
pub fn i_06080c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6080E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06080E: bsr.w $60706
pub fn i_06080e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60812;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x60812);
    cpu.pc = 0x60706;
    18
}
/// $060812: rts
pub fn i_060812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60814;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Destroyed_060814  ($060814, 114 B, props/late_props_turrets_05e000.s)
/// $060814: jsr $2783a.l
pub fn i_060814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6081A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6081A);
    cpu.pc = 0x2783A;
    16
}
/// $06081A: move.b $70(a6), d0
pub fn i_06081a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6081E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06081E: bne.w $6082c
pub fn i_06081e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60822;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6082C; return 10; }
    8
}
/// $060822: lea.l $2c2572.l, a1
pub fn i_060822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60828;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2572;
    20
}
/// $060828: bra.w $6084c
pub fn i_060828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6082C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6084C;
    10
}
/// $06082C: cmpi.b #$1, d0
pub fn i_06082c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60830;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $060830: bne.w $6083e
pub fn i_060830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60834;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6083E; return 10; }
    8
}
/// $060834: lea.l $2c2550.l, a1
pub fn i_060834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6083A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2550;
    20
}
/// $06083A: bra.w $6084c
pub fn i_06083a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6083E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6084C;
    10
}
/// $06083E: cmpi.b #$2, d0
pub fn i_06083e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60842;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $060842: bne.w $6084c
pub fn i_060842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60846;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6084C; return 10; }
    8
}
/// $060846: lea.l $2c2550.l, a1
pub fn i_060846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6084C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2550;
    20
}
/// $06084C: jsr $43fac.l
pub fn i_06084c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60852);
    cpu.pc = 0x43FAC;
    16
}
/// $060852: bclr.b #$0, $13(a6)
pub fn i_060852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60858;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060858: lea.l $2c244e.l, a0
pub fn i_060858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6085E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C244E;
    20
}
/// $06085E: jsr $28cd4.l
pub fn i_06085e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60864;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60864);
    cpu.pc = 0x28CD4;
    16
}
/// $060864: lea.l $6086a(pc), a1
pub fn i_060864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60868;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6086A;
    16
}
/// $060868: move.l a1, (a6)
pub fn i_060868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6086A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06086A: jsr $2783a.l
pub fn i_06086a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60870);
    cpu.pc = 0x2783A;
    16
}
/// $060870: jsr $28d70.l
pub fn i_060870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60876);
    cpu.pc = 0x28D70;
    16
}
/// $060876: bcc.w $60880
pub fn i_060876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6087A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60880; return 10; }
    8
}
/// $06087A: lea.l $60886(pc), a1
pub fn i_06087a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6087E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60886;
    16
}
/// $06087E: move.l a1, (a6)
pub fn i_06087e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60880;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060880: bsr.w $60706
pub fn i_060880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60884;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x60884);
    cpu.pc = 0x60706;
    18
}
/// $060884: rts
pub fn i_060884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Rearm_060886  ($060886, 28 B, props/late_props_turrets_05e000.s)
/// $060886: bclr.b #$3, $13(a6)
pub fn i_060886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6088C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06088C: jsr $2783a.l
pub fn i_06088c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60892);
    cpu.pc = 0x2783A;
    16
}
/// $060892: lea.l $2bf496.l, a0
pub fn i_060892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60898;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF496;
    20
}
/// $060898: jsr $799de.l
pub fn i_060898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6089E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6089E);
    cpu.pc = 0x799DE;
    16
}
/// $06089E: move.w d0, $66(a6)
pub fn i_06089e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Obstacle_IdleB_0608a2  ($0608A2, 62 B, props/late_props_turrets_05e000.s)
/// $0608A2: lea.l $608a8(pc), a1
pub fn i_0608a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x608A8;
    16
}
/// $0608A6: move.l a1, (a6)
pub fn i_0608a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0608A8: jsr $2783a.l
pub fn i_0608a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x608AE);
    cpu.pc = 0x2783A;
    16
}
/// $0608AE: jsr $28d70.l
pub fn i_0608ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x608B4);
    cpu.pc = 0x28D70;
    16
}
/// $0608B4: jsr $2870a.l
pub fn i_0608b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x608BA);
    cpu.pc = 0x2870A;
    16
}
/// $0608BA: bcc.w $608c4
pub fn i_0608ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x608C4; return 10; }
    8
}
/// $0608BE: lea.l $608e0(pc), a1
pub fn i_0608be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x608E0;
    16
}
/// $0608C2: move.l a1, (a6)
pub fn i_0608c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0608C4: bclr.b #$3, $13(a6)
pub fn i_0608c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0608CA: jsr $28758.l
pub fn i_0608ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x608D0);
    cpu.pc = 0x28758;
    16
}
/// $0608D0: bcc.w $608da
pub fn i_0608d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x608DA; return 10; }
    8
}
/// $0608D4: lea.l $60924(pc), a1
pub fn i_0608d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60924;
    16
}
/// $0608D8: move.l a1, (a6)
pub fn i_0608d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0608DA: bsr.w $60706
pub fn i_0608da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608DE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x608DE);
    cpu.pc = 0x60706;
    18
}
/// $0608DE: rts
pub fn i_0608de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_DamagedB_0608e0  ($0608E0, 68 B, props/late_props_turrets_05e000.s)
/// $0608E0: lea.l $2c2438.l, a0
pub fn i_0608e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2438;
    20
}
/// $0608E6: jsr $28cd4.l
pub fn i_0608e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x608EC);
    cpu.pc = 0x28CD4;
    16
}
/// $0608EC: lea.l $608f2(pc), a1
pub fn i_0608ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x608F2;
    16
}
/// $0608F0: move.l a1, (a6)
pub fn i_0608f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0608F2: jsr $2783a.l
pub fn i_0608f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x608F8);
    cpu.pc = 0x2783A;
    16
}
/// $0608F8: jsr $28d70.l
pub fn i_0608f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x608FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x608FE);
    cpu.pc = 0x28D70;
    16
}
/// $0608FE: bcc.w $60908
pub fn i_0608fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60902;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60908; return 10; }
    8
}
/// $060902: lea.l $608a2(pc), a1
pub fn i_060902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60906;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x608A2;
    16
}
/// $060906: move.l a1, (a6)
pub fn i_060906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60908;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060908: bclr.b #$3, $13(a6)
pub fn i_060908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6090E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06090E: jsr $28758.l
pub fn i_06090e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60914);
    cpu.pc = 0x28758;
    16
}
/// $060914: bcc.w $6091e
pub fn i_060914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60918;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6091E; return 10; }
    8
}
/// $060918: lea.l $60924(pc), a1
pub fn i_060918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6091C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60924;
    16
}
/// $06091C: move.l a1, (a6)
pub fn i_06091c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6091E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06091E: bsr.w $60706
pub fn i_06091e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60922;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x60922);
    cpu.pc = 0x60706;
    18
}
/// $060922: rts
pub fn i_060922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60924;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_DestroyedB_060924  ($060924, 120 B, props/late_props_turrets_05e000.s)
/// $060924: bclr.b #$0, $13(a6)
pub fn i_060924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6092A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06092A: jsr $2783a.l
pub fn i_06092a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60930);
    cpu.pc = 0x2783A;
    16
}
/// $060930: lea.l $2c2504.l, a0
pub fn i_060930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60936;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2504;
    20
}
/// $060936: jsr $28cd4.l
pub fn i_060936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6093C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6093C);
    cpu.pc = 0x28CD4;
    16
}
/// $06093C: move.b $70(a6), d0
pub fn i_06093c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60940;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $060940: bne.w $6094e
pub fn i_060940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60944;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6094E; return 10; }
    8
}
/// $060944: lea.l $2c2594.l, a1
pub fn i_060944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6094A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2594;
    20
}
/// $06094A: bra.w $6096e
pub fn i_06094a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6094E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6096E;
    10
}
/// $06094E: cmpi.b #$1, d0
pub fn i_06094e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60952;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $060952: bne.w $60960
pub fn i_060952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60956;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60960; return 10; }
    8
}
/// $060956: lea.l $2c25b6.l, a1
pub fn i_060956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6095C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C25B6;
    20
}
/// $06095C: bra.w $6096e
pub fn i_06095c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60960;
    cpu.ir = 0x6000;
    cpu.pc = 0x6096E;
    10
}
/// $060960: cmpi.b #$2, d0
pub fn i_060960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60964;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $060964: bne.w $6096e
pub fn i_060964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60968;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6096E; return 10; }
    8
}
/// $060968: lea.l $2c25d8.l, a1
pub fn i_060968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6096E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C25D8;
    20
}
/// $06096E: jsr $43fac.l
pub fn i_06096e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60974);
    cpu.pc = 0x43FAC;
    16
}
/// $060974: jsr $8f308.l
pub fn i_060974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6097A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6097A);
    cpu.pc = 0x8F308;
    16
}
/// $06097A: lea.l $60980(pc), a1
pub fn i_06097a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6097E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60980;
    16
}
/// $06097E: move.l a1, (a6)
pub fn i_06097e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60980;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060980: jsr $2783a.l
pub fn i_060980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60986;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60986);
    cpu.pc = 0x2783A;
    16
}
/// $060986: jsr $28d70.l
pub fn i_060986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6098C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6098C);
    cpu.pc = 0x28D70;
    16
}
/// $06098C: bcc.w $60996
pub fn i_06098c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60990;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60996; return 10; }
    8
}
/// $060990: lea.l $6099c(pc), a1
pub fn i_060990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60994;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6099C;
    16
}
/// $060994: move.l a1, (a6)
pub fn i_060994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60996;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060996: bsr.w $60706
pub fn i_060996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6099A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x6099A);
    cpu.pc = 0x60706;
    18
}
/// $06099A: rts
pub fn i_06099a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6099C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Wreck_06099c  ($06099C, 38 B, props/late_props_turrets_05e000.s)
/// $06099C: lea.l $ffff.w, a0
pub fn i_06099c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609A0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0609A0: move.l a0, $48(a6)
pub fn i_0609a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609A4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0609A4: bclr.b #$3, $13(a6)
pub fn i_0609a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609AA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0609AA: lea.l $609b0(pc), a1
pub fn i_0609aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x609B0;
    16
}
/// $0609AE: move.l a1, (a6)
pub fn i_0609ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0609B0: jsr $2783a.l
pub fn i_0609b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x609B6);
    cpu.pc = 0x2783A;
    16
}
/// $0609B6: jsr $28d70.l
pub fn i_0609b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x609BC);
    cpu.pc = 0x28D70;
    16
}
/// $0609BC: bsr.w $60706
pub fn i_0609bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609C0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x609C0);
    cpu.pc = 0x60706;
    18
}
/// $0609C0: rts
pub fn i_0609c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_SpawnChild_0609c2  ($0609C2, 10 B, props/late_props_turrets_05e000.s)
/// $0609C2: lea.l $609d4(pc), a1
pub fn i_0609c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x609D4;
    16
}
/// $0609C6: jsr $4ae.l
pub fn i_0609c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x609CC);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_0609cc  ($0609CC, 8 B, thunks/jsr_abs_thunks.c)
/// $0609CC: jsr $5dd02.l
pub fn i_0609cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x609D2);
    cpu.pc = 0x5DD02;
    16
}
/// $0609D2: rts
pub fn i_0609d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Smoke_0609d4  ($0609D4, 58 B, props/late_props_turrets_05e000.s)
/// $0609D4: move.w #$6c, d1
pub fn i_0609d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0609D8: jsr $236e.l
pub fn i_0609d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x609DE);
    cpu.pc = 0x236E;
    16
}
/// $0609DE: lea.l $2dd37e.l, a0
pub fn i_0609de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $0609E4: jsr $28cd4.l
pub fn i_0609e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x609EA);
    cpu.pc = 0x28CD4;
    16
}
/// $0609EA: move.w #$2000, $38(a6)
pub fn i_0609ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0609F0: lea.l $609f6(pc), a1
pub fn i_0609f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x609F6;
    16
}
/// $0609F4: move.l a1, (a6)
pub fn i_0609f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0609F6: jsr $2783a.l
pub fn i_0609f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x609FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x609FC);
    cpu.pc = 0x2783A;
    16
}
/// $0609FC: jsr $28d70.l
pub fn i_0609fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60A02);
    cpu.pc = 0x28D70;
    16
}
/// $060A02: bcc.w $60a0c
pub fn i_060a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60A0C; return 10; }
    8
}
/// $060A06: jmp $518.l
pub fn i_060a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A0C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $060A0C: rts
pub fn i_060a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Tmpl111_060a0e  ($060A0E, 62 B, props/late_props_turrets_05e000.s)
/// $060A0E: move.w #$82, d1
pub fn i_060a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A12;
    cpu.ir = 0x323C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060A12: jsr $236e.l
pub fn i_060a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60A18);
    cpu.pc = 0x236E;
    16
}
/// $060A18: move.w #$f000, $38(a6)
pub fn i_060a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060A1E: lea.l $2c26b4.l, a0
pub fn i_060a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C26B4;
    20
}
/// $060A24: jsr $28cd4.l
pub fn i_060a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60A2A);
    cpu.pc = 0x28CD4;
    16
}
/// $060A2A: lea.l $60a30(pc), a1
pub fn i_060a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60A30;
    16
}
/// $060A2E: move.l a1, (a6)
pub fn i_060a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060A30: jsr $2783a.l
pub fn i_060a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60A36);
    cpu.pc = 0x2783A;
    16
}
/// $060A36: jsr $28d70.l
pub fn i_060a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60A3C);
    cpu.pc = 0x28D70;
    16
}
/// $060A3C: bclr.b #$3, $13(a6)
pub fn i_060a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A42;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060A42: bclr.b #$0, $13(a6)
pub fn i_060a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A48;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060A48: bra.w $60706
pub fn i_060a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x60706;
    10
}
// ===== Obstacle_Tmpl094_060a4c  ($060A4C, 198 B, props/late_props_turrets_05e000.s)
/// $060A4C: move.b #$2, $70(a6)
pub fn i_060a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060A52: bra.w $60a64
pub fn i_060a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A56;
    cpu.ir = 0x6000;
    cpu.pc = 0x60A64;
    10
}
/// $060A64: move.w #$82, d1
pub fn i_060a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A68;
    cpu.ir = 0x323C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060A68: jsr $236e.l
pub fn i_060a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60A6E);
    cpu.pc = 0x236E;
    16
}
/// $060A6E: lea.l $2c27a6.l, a1
pub fn i_060a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A74;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C27A6;
    20
}
/// $060A74: cmpi.b #$1, $70(a6)
pub fn i_060a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A7A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $060A7A: bne.w $60a88
pub fn i_060a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60A88; return 10; }
    8
}
/// $060A7E: lea.l $2c2796.l, a1
pub fn i_060a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A84;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2796;
    20
}
/// $060A84: bra.w $60a98
pub fn i_060a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A88;
    cpu.ir = 0x6000;
    cpu.pc = 0x60A98;
    10
}
/// $060A88: cmpi.b #$2, $70(a6)
pub fn i_060a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A8E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $060A8E: bne.w $60a98
pub fn i_060a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60A98; return 10; }
    8
}
/// $060A92: lea.l $2c27b6.l, a1
pub fn i_060a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A98;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C27B6;
    20
}
/// $060A98: jsr $43fac.l
pub fn i_060a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60A9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60A9E);
    cpu.pc = 0x43FAC;
    16
}
/// $060A9E: lea.l $2bf518.l, a0
pub fn i_060a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF518;
    20
}
/// $060AA4: jsr $799de.l
pub fn i_060aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60AAA);
    cpu.pc = 0x799DE;
    16
}
/// $060AAA: move.w d0, $66(a6)
pub fn i_060aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AAE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060AAE: move.w #$8000, d0
pub fn i_060aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AB2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060AB2: jsr $28134.l
pub fn i_060ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60AB8);
    cpu.pc = 0x28134;
    16
}
/// $060AB8: andi.w #$ffe3, $38(a6)
pub fn i_060ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60ABE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $060ABE: ori.w #$c, $38(a6)
pub fn i_060abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AC4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $060AC4: lea.l $2c26b4.l, a0
pub fn i_060ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60ACA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C26B4;
    20
}
/// $060ACA: jsr $28cd4.l
pub fn i_060aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60AD0);
    cpu.pc = 0x28CD4;
    16
}
/// $060AD0: lea.l $60ad6(pc), a1
pub fn i_060ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60AD6;
    16
}
/// $060AD4: move.l a1, (a6)
pub fn i_060ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060AD6: jsr $2783a.l
pub fn i_060ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60ADC);
    cpu.pc = 0x2783A;
    16
}
/// $060ADC: jsr $28758.l
pub fn i_060adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60AE2);
    cpu.pc = 0x28758;
    16
}
/// $060AE2: bcc.w $60af0
pub fn i_060ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60AF0; return 10; }
    8
}
/// $060AE6: lea.l $60b40(pc), a1
pub fn i_060ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60B40;
    16
}
/// $060AEA: move.l a1, (a6)
pub fn i_060aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060AEC: bra.w $60b06
pub fn i_060aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x60B06;
    10
}
/// $060AF0: jsr $2870a.l
pub fn i_060af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60AF6);
    cpu.pc = 0x2870A;
    16
}
/// $060AF6: bcc.w $60b06
pub fn i_060af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60AFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60B06; return 10; }
    8
}
/// $060AFA: lea.l $2c26c6.l, a0
pub fn i_060afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C26C6;
    20
}
/// $060B00: jsr $28cd4.l
pub fn i_060b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60B06);
    cpu.pc = 0x28CD4;
    16
}
/// $060B06: jsr $28d70.l
pub fn i_060b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60B0C);
    cpu.pc = 0x28D70;
    16
}
/// $060B0C: bsr.w $60706
pub fn i_060b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B10;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x60B10);
    cpu.pc = 0x60706;
    18
}
/// $060B10: rts
pub fn i_060b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle094_Register_060b12  ($060B12, 44 B, props/late_props_turrets_05e000.s)
/// $060B12: lea.l $2c27c6.l, a1
pub fn i_060b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B18;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C27C6;
    20
}
/// $060B18: cmpi.b #$1, $70(a6)
pub fn i_060b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B1E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $060B1E: bne.w $60b28
pub fn i_060b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B22;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60B28; return 10; }
    8
}
/// $060B22: lea.l $2c27d6.l, a1
pub fn i_060b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B28;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C27D6;
    20
}
/// $060B28: cmpi.b #$2, $70(a6)
pub fn i_060b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B2E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $060B2E: bne.w $60b38
pub fn i_060b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60B38; return 10; }
    8
}
/// $060B32: lea.l $2c27e6.l, a1
pub fn i_060b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C27E6;
    20
}
/// $060B38: jmp $43fac.l
pub fn i_060b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B3E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x43FAC;
    8
}
// ===== Obstacle_Rts_060b3e  ($060B3E, 2 B, props/late_props_turrets_05e000.s)
/// $060B3E: rts
pub fn i_060b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle_Flames_060b40  ($060B40, 52 B, props/late_props_turrets_05e000.s)
/// $060B40: lea.l $2c26ea.l, a0
pub fn i_060b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C26EA;
    20
}
/// $060B46: jsr $28cd4.l
pub fn i_060b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60B4C);
    cpu.pc = 0x28CD4;
    16
}
/// $060B4C: lea.l $60b52(pc), a1
pub fn i_060b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60B52;
    16
}
/// $060B50: move.l a1, (a6)
pub fn i_060b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060B52: jsr $2783a.l
pub fn i_060b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60B58);
    cpu.pc = 0x2783A;
    16
}
/// $060B58: jsr $28d70.l
pub fn i_060b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60B5E);
    cpu.pc = 0x28D70;
    16
}
/// $060B5E: bcc.w $60b6e
pub fn i_060b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60B6E; return 10; }
    8
}
/// $060B62: jsr $5b6.l
pub fn i_060b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60B68);
    cpu.pc = 0x5B6;
    16
}
/// $060B68: jmp $518.l
pub fn i_060b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $060B6E: bsr.w $60706
pub fn i_060b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B72;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x60B72);
    cpu.pc = 0x60706;
    18
}
/// $060B72: rts
pub fn i_060b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $060B7A: move.b #$1, $70(a6)
pub fn i_060b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060B80: bra.w $60b88
pub fn i_060b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B84;
    cpu.ir = 0x6000;
    cpu.pc = 0x60B88;
    10
}
/// $060B88: moveq #$0, d0
pub fn i_060b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B8A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $060B8A: move.b $98(a6), d0
pub fn i_060b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B8E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $060B8E: cmpi.b #$3, d0
pub fn i_060b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B92;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $060B92: bcs.w $60b98
pub fn i_060b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B96;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x60B98; return 10; }
    8
}
/// $060B96: moveq #$0, d0
pub fn i_060b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B98;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $060B98: add.w d0, d0
pub fn i_060b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B9A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $060B9A: lea.l $60b74(pc), a0
pub fn i_060b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60B9E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x60B74;
    16
}
/// $060B9E: move.w (a0, d0.w), d1
pub fn i_060b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BA2;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $060BA2: jsr $236e.l
pub fn i_060ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60BA8);
    cpu.pc = 0x236E;
    16
}
/// $060BA8: lea.l $2c29bc.l, a0
pub fn i_060ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C29BC;
    20
}
/// $060BAE: jsr $28cd4.l
pub fn i_060bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60BB4);
    cpu.pc = 0x28CD4;
    16
}
/// $060BB4: tst.b $70(a6)
pub fn i_060bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BB8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $060BB8: beq.w $60bc6
pub fn i_060bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BBC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x60BC6; return 10; }
    8
}
/// $060BBC: lea.l $2c28fc.l, a1
pub fn i_060bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C28FC;
    20
}
/// $060BC2: bra.w $60bcc
pub fn i_060bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x60BCC;
    10
}
/// $060BC6: lea.l $2c28bc.l, a1
pub fn i_060bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BCC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C28BC;
    20
}
/// $060BCC: jsr $43fac.l
pub fn i_060bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60BD2);
    cpu.pc = 0x43FAC;
    16
}
/// $060BD2: move.w #$64, $66(a6)
pub fn i_060bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BD8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060BD8: move.w #$8000, d0
pub fn i_060bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BDC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060BDC: jsr $28134.l
pub fn i_060bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60BE2);
    cpu.pc = 0x28134;
    16
}
/// $060BE2: andi.w #$ffe3, $38(a6)
pub fn i_060be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BE8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $060BE8: ori.w #$c, $38(a6)
pub fn i_060be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BEE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_060bee  ($060BEE, 8 B, kernel/task_handlers.c)
/// $060BEE: lea.l $60bf6(pc), a1
pub fn i_060bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60BF6;
    16
}
/// $060BF2: move.l a1, (a6)
pub fn i_060bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060BF4: rts
pub fn i_060bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle095_Idle_060bf6  ($060BF6, 74 B, props/late_props_turrets_05e000.s)
/// $060BF6: jsr $2783a.l
pub fn i_060bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60BFC);
    cpu.pc = 0x2783A;
    16
}
/// $060BFC: jsr $28758.l
pub fn i_060bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C02);
    cpu.pc = 0x28758;
    16
}
/// $060C02: bcc.w $60c0e
pub fn i_060c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60C0E; return 10; }
    8
}
/// $060C06: bsr.w $60c6a
pub fn i_060c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C0A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x60C0A);
    cpu.pc = 0x60C6A;
    18
}
/// $060C0A: bra.w $60c36
pub fn i_060c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x60C36;
    10
}
/// $060C0E: jsr $2870a.l
pub fn i_060c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C14);
    cpu.pc = 0x2870A;
    16
}
/// $060C14: bcc.w $60c36
pub fn i_060c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60C36; return 10; }
    8
}
/// $060C18: cmpi.b #$4, $58(a6)
pub fn i_060c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C1E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $060C1E: bne.w $60c2a
pub fn i_060c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C22;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60C2A; return 10; }
    8
}
/// $060C22: bsr.w $60c6a
pub fn i_060c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C26;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x60C26);
    cpu.pc = 0x60C6A;
    18
}
/// $060C26: bra.w $60c36
pub fn i_060c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x60C36;
    10
}
/// $060C2A: lea.l $2c29ce.l, a0
pub fn i_060c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C29CE;
    20
}
/// $060C30: jsr $28cd4.l
pub fn i_060c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C36);
    cpu.pc = 0x28CD4;
    16
}
/// $060C36: jsr $28d70.l
pub fn i_060c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C3C);
    cpu.pc = 0x28D70;
    16
}
/// $060C3C: bra.w $60706
pub fn i_060c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C40;
    cpu.ir = 0x6000;
    cpu.pc = 0x60706;
    10
}
// ===== Obstacle_FlushMusicJmp77FD6_060c40  ($060C40, 42 B, props/late_props_turrets_05e000.s)
/// $060C40: jsr $2783a.l
pub fn i_060c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C46);
    cpu.pc = 0x2783A;
    16
}
/// $060C46: jsr $28d70.l
pub fn i_060c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C4C);
    cpu.pc = 0x28D70;
    16
}
/// $060C4C: bcc.w $60c66
pub fn i_060c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60C66; return 10; }
    8
}
/// $060C50: jsr $13600.l
pub fn i_060c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C56);
    cpu.pc = 0x13600;
    16
}
/// $060C56: move.w #$109f, d0
pub fn i_060c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C5A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060C5A: jsr $2352.l
pub fn i_060c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C60);
    cpu.pc = 0x2352;
    16
}
/// $060C60: jmp $77fd6.l
pub fn i_060c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C66;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
/// $060C66: bra.w $60706
pub fn i_060c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x60706;
    10
}
// ===== Obstacle095_Wreck_060c6a  ($060C6A, 40 B, props/late_props_turrets_05e000.s)
/// $060C6A: move.w $22(a6), d0
pub fn i_060c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C6E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $060C6E: sub.w $54(a6), d0
pub fn i_060c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C72;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $060C72: bmi.w $60c86
pub fn i_060c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C76;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x60C86; return 10; }
    8
}
/// $060C76: lea.l $2c2a14.l, a0
pub fn i_060c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2A14;
    20
}
/// $060C7C: jsr $28cd4.l
pub fn i_060c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C82);
    cpu.pc = 0x28CD4;
    16
}
/// $060C82: bra.w $60c92
pub fn i_060c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C86;
    cpu.ir = 0x6000;
    cpu.pc = 0x60C92;
    10
}
/// $060C86: lea.l $2c2aca.l, a0
pub fn i_060c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2ACA;
    20
}
/// $060C8C: jsr $28cd4.l
pub fn i_060c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C92);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_060c92  ($060C92, 8 B, kernel/task_handlers.c)
/// $060C92: lea.l $60c40(pc), a1
pub fn i_060c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60C40;
    16
}
/// $060C96: move.l a1, (a6)
pub fn i_060c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060C98: rts
pub fn i_060c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Obstacle095_Register_060c9a  ($060C9A, 30 B, props/late_props_turrets_05e000.s)
/// $060C9A: tst.b $70(a6)
pub fn i_060c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C9E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $060C9E: beq.w $60cac
pub fn i_060c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x60CAC; return 10; }
    8
}
/// $060CA2: lea.l $2c297c.l, a1
pub fn i_060ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CA8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C297C;
    20
}
/// $060CA8: bra.w $60cb2
pub fn i_060ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x60CB2;
    10
}
/// $060CAC: lea.l $2c293c.l, a1
pub fn i_060cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CB2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C293C;
    20
}
/// $060CB2: jmp $43fac.l
pub fn i_060cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x43FAC;
    8
}
// ===== Obstacle095_Stub_060cb8  ($060CB8, 16 B, props/late_props_turrets_05e000.s)
/// $060CB8: movea.l $8(a6), a1
pub fn i_060cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CBC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $060CBC: move.b $10(a6), d0
pub fn i_060cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CC0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $060CC0: cmp.b $10(a1), d0
pub fn i_060cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CC4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $060CC4: bcs.w $60cce
pub fn i_060cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CC8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x60CCE; return 10; }
    8
}
// ===== ClearXN_060cc8  ($060CC8, 6 B, thunks/ccr_helpers.c)
/// $060CC8: andi.b #$ee, ccr
pub fn i_060cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CCC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $060CCC: rts
pub fn i_060ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_060cce  ($060CCE, 6 B, thunks/ccr_helpers.c)
/// $060CCE: ori.b #$11, ccr
pub fn i_060cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CD2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $060CD2: rts
pub fn i_060cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Tmpl07C_060cd4  ($060CD4, 94 B, props/late_props_turrets_05e000.s)
/// $060CD4: move.w #$0, $80(a6)
pub fn i_060cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CDA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060CDA: bra.w $60cee
pub fn i_060cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x60CEE;
    10
}
/// $060CEE: move.w #$4f, d1
pub fn i_060cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CF2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060CF2: jsr $236e.l
pub fn i_060cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60CF8);
    cpu.pc = 0x236E;
    16
}
/// $060CF8: moveq #$0, d0
pub fn i_060cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CFA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $060CFA: move.b d0, $20(a6)
pub fn i_060cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60CFE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060CFE: move.b d0, $21(a6)
pub fn i_060cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D02;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060D02: lea.l $2c2b80.l, a0
pub fn i_060d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2B80;
    20
}
/// $060D08: move.w $80(a6), d0
pub fn i_060d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D0C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $060D0C: add.w d0, d0
pub fn i_060d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D0E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $060D0E: add.w d0, d0
pub fn i_060d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D10;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $060D10: movea.l (a0, d0.w), a0
pub fn i_060d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D14;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $060D14: jsr (a0)
pub fn i_060d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D16;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x60D16);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $060D16: lea.l $2bdc06.l, a0
pub fn i_060d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDC06;
    20
}
/// $060D1C: jsr $799de.l
pub fn i_060d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D22);
    cpu.pc = 0x799DE;
    16
}
/// $060D22: move.w d0, $66(a6)
pub fn i_060d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D26;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060D26: bclr.b #$1, $12(a6)
pub fn i_060d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D2C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060D2C: jsr $267e2.l
pub fn i_060d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D32);
    cpu.pc = 0x267E2;
    16
}
// ===== SetTaskHandler_060d32  ($060D32, 8 B, kernel/task_handlers.c)
/// $060D32: lea.l $60d3a(pc), a1
pub fn i_060d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60D3A;
    16
}
/// $060D36: move.l a1, (a6)
pub fn i_060d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060D38: rts
pub fn i_060d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Idle_060d3a  ($060D3A, 94 B, props/late_props_turrets_05e000.s)
/// $060D3A: move.w $38(a6), d0
pub fn i_060d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D3E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $060D3E: addq.w #$1, d0
pub fn i_060d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D40;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $060D40: jsr $28134.l
pub fn i_060d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D46);
    cpu.pc = 0x28134;
    16
}
/// $060D46: lea.l $29c536.l, a0
pub fn i_060d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C536;
    20
}
/// $060D4C: jsr $28cd4.l
pub fn i_060d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D52);
    cpu.pc = 0x28CD4;
    16
}
/// $060D52: lea.l $60d58(pc), a1
pub fn i_060d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60D58;
    16
}
/// $060D56: move.l a1, (a6)
pub fn i_060d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060D58: jsr $2783a.l
pub fn i_060d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D5E);
    cpu.pc = 0x2783A;
    16
}
/// $060D5E: jsr $28d70.l
pub fn i_060d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D64);
    cpu.pc = 0x28D70;
    16
}
/// $060D64: jsr $2870a.l
pub fn i_060d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D6A);
    cpu.pc = 0x2870A;
    16
}
/// $060D6A: bcc.w $60d80
pub fn i_060d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60D80; return 10; }
    8
}
/// $060D6E: lea.l $5e766.l, a0
pub fn i_060d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $060D74: jsr $5e770.l
pub fn i_060d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D7A);
    cpu.pc = 0x5E770;
    16
}
/// $060D7A: bclr.b #$3, $13(a6)
pub fn i_060d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D80;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060D80: jsr $28758.l
pub fn i_060d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60D86);
    cpu.pc = 0x28758;
    16
}
/// $060D86: bcc.w $60d90
pub fn i_060d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60D90; return 10; }
    8
}
/// $060D8A: lea.l $60da0(pc), a1
pub fn i_060d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60DA0;
    16
}
/// $060D8E: move.l a1, (a6)
pub fn i_060d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060D90: jsr $60df0(pc)
pub fn i_060d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60D94);
    cpu.pc = 0x60DF0;
    16
}
/// $060D94: bcc.w $60d9e
pub fn i_060d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60D9E; return 10; }
    8
}
// ===== SetTaskHandler_060d98  ($060D98, 8 B, kernel/task_handlers.c)
/// $060D98: lea.l $60de8(pc), a1
pub fn i_060d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60DE8;
    16
}
/// $060D9C: move.l a1, (a6)
pub fn i_060d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60D9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060D9E: rts
pub fn i_060d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Destroyed_060da0  ($060DA0, 64 B, props/late_props_turrets_05e000.s)
/// $060DA0: bclr.b #$1, $12(a6)
pub fn i_060da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DA6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $060DA6: lea.l $2c2b8c.l, a0
pub fn i_060da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2B8C;
    20
}
/// $060DAC: move.w $80(a6), d0
pub fn i_060dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DB0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $060DB0: add.w d0, d0
pub fn i_060db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DB2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $060DB2: add.w d0, d0
pub fn i_060db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DB4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $060DB4: movea.l (a0, d0.w), a0
pub fn i_060db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DB8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $060DB8: jsr (a0)
pub fn i_060db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DBA;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x60DBA);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $060DBA: lea.l $29c54c.l, a0
pub fn i_060dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C54C;
    20
}
/// $060DC0: jsr $28cd4.l
pub fn i_060dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60DC6);
    cpu.pc = 0x28CD4;
    16
}
/// $060DC6: lea.l $60dcc(pc), a1
pub fn i_060dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60DCC;
    16
}
/// $060DCA: move.l a1, (a6)
pub fn i_060dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060DCC: jsr $2783a.l
pub fn i_060dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60DD2);
    cpu.pc = 0x2783A;
    16
}
/// $060DD2: jsr $28d70.l
pub fn i_060dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60DD8);
    cpu.pc = 0x28D70;
    16
}
/// $060DD8: jsr $60df0(pc)
pub fn i_060dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60DDC);
    cpu.pc = 0x60DF0;
    16
}
/// $060DDC: bcc.w $60de6
pub fn i_060ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60DE6; return 10; }
    8
}
// ===== SetTaskHandler_060de0  ($060DE0, 8 B, kernel/task_handlers.c)
/// $060DE0: lea.l $60de8(pc), a1
pub fn i_060de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60DE8;
    16
}
/// $060DE4: move.l a1, (a6)
pub fn i_060de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060DE6: rts
pub fn i_060de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_060de8  ($060DE8, 8 B, kernel/jmp_to_scheduler.c)
/// $060DE8: jmp $518.l
pub fn i_060de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DEE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Offworld_Box2C2C60_060df0  ($060DF0, 16 B, props/late_props_turrets_05e000.s)
/// $060DF0: lea.l $2c2c60.l, a0
pub fn i_060df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2C60;
    20
}
/// $060DF6: jsr $5dd5c.l
pub fn i_060df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60DFC);
    cpu.pc = 0x5DD5C;
    16
}
/// $060DFC: bcc.w $60e06
pub fn i_060dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60E06; return 10; }
    8
}
// ===== SetC_060e00  ($060E00, 6 B, thunks/ccr_helpers.c)
/// $060E00: ori.b #$1, ccr
pub fn i_060e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E04;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $060E04: rts
pub fn i_060e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_060e06  ($060E06, 6 B, thunks/ccr_helpers.c)
/// $060E06: andi.b #$fe, ccr
pub fn i_060e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E0A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $060E0A: rts
pub fn i_060e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Register0_060e0c  ($060E0C, 6 B, props/late_props_turrets_05e000.s)
/// $060E0C: lea.l $2c2b98.l, a1
pub fn i_060e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2B98;
    20
}
// ===== JsrAbsThunk_060e12  ($060E12, 8 B, thunks/jsr_abs_thunks.c)
/// $060E12: jsr $43fac.l
pub fn i_060e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E18);
    cpu.pc = 0x43FAC;
    16
}
/// $060E18: rts
pub fn i_060e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Register1_060e1a  ($060E1A, 6 B, props/late_props_turrets_05e000.s)
/// $060E1A: lea.l $2c2bc4.l, a1
pub fn i_060e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2BC4;
    20
}
// ===== JsrAbsThunk_060e20  ($060E20, 8 B, thunks/jsr_abs_thunks.c)
/// $060E20: jsr $43fac.l
pub fn i_060e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E26);
    cpu.pc = 0x43FAC;
    16
}
/// $060E26: rts
pub fn i_060e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Register2_060e28  ($060E28, 6 B, props/late_props_turrets_05e000.s)
/// $060E28: lea.l $2c2bf0.l, a1
pub fn i_060e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E2E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2BF0;
    20
}
// ===== JsrAbsThunk_060e2e  ($060E2E, 8 B, thunks/jsr_abs_thunks.c)
/// $060E2E: jsr $43fac.l
pub fn i_060e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E34);
    cpu.pc = 0x43FAC;
    16
}
/// $060E34: rts
pub fn i_060e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Register3_060e36  ($060E36, 6 B, props/late_props_turrets_05e000.s)
/// $060E36: lea.l $2c2c28.l, a1
pub fn i_060e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C2C28;
    20
}
// ===== JsrAbsThunk_060e3c  ($060E3C, 8 B, thunks/jsr_abs_thunks.c)
/// $060E3C: jsr $43fac.l
pub fn i_060e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E42);
    cpu.pc = 0x43FAC;
    16
}
/// $060E42: rts
pub fn i_060e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00060E44  ($060E44, 2 B, thunks/trivial_rts.c)
/// $060E44: rts
pub fn i_060e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Stub_060e46  ($060E46, 16 B, props/late_props_turrets_05e000.s)
/// $060E46: movea.l $8(a6), a1
pub fn i_060e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E4A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $060E4A: move.b $10(a6), d0
pub fn i_060e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E4E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $060E4E: cmp.b $10(a1), d0
pub fn i_060e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E52;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $060E52: bcs.w $60e5c
pub fn i_060e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E56;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x60E5C; return 10; }
    8
}
// ===== ClearXN_060e56  ($060E56, 6 B, thunks/ccr_helpers.c)
/// $060E56: andi.b #$ee, ccr
pub fn i_060e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E5A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $060E5A: rts
pub fn i_060e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_060e5c  ($060E5C, 6 B, thunks/ccr_helpers.c)
/// $060E5C: ori.b #$11, ccr
pub fn i_060e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E60;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $060E60: rts
pub fn i_060e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Tmpl08E_060e62  ($060E62, 162 B, props/late_props_turrets_05e000.s)
/// $060E62: move.w #$187, d1
pub fn i_060e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E66;
    cpu.ir = 0x323C;
    let v1: u32 = 0x187;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060E66: jsr $236e.l
pub fn i_060e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E6C);
    cpu.pc = 0x236E;
    16
}
/// $060E6C: move.w #$188, d1
pub fn i_060e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E70;
    cpu.ir = 0x323C;
    let v1: u32 = 0x188;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060E70: jsr $236e.l
pub fn i_060e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E76);
    cpu.pc = 0x236E;
    16
}
/// $060E76: move.w #$18a, d1
pub fn i_060e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E7A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060E7A: jsr $236e.l
pub fn i_060e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E80);
    cpu.pc = 0x236E;
    16
}
/// $060E80: move.w #$10e8, d0
pub fn i_060e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $060E84: jsr $2352.l
pub fn i_060e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E8A);
    cpu.pc = 0x2352;
    16
}
/// $060E8A: move.b #$0, $7c(a6)
pub fn i_060e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060E90: move.w #$8000, $38(a6)
pub fn i_060e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E96;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060E96: jsr $267e2.l
pub fn i_060e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60E9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60E9C);
    cpu.pc = 0x267E2;
    16
}
/// $060E9C: jsr $27cee.l
pub fn i_060e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60EA2);
    cpu.pc = 0x27CEE;
    16
}
/// $060EA2: move.b $98(a6), d0
pub fn i_060ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EA6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $060EA6: andi.l #$3, d0
pub fn i_060ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EAC;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $060EAC: move.b d0, $3a(a6)
pub fn i_060eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EB0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060EB0: move.w $22(a6), $72(a6)
pub fn i_060eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EB6;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $060EB6: move.w $24(a6), $74(a6)
pub fn i_060eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EBC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    20
}
/// $060EBC: jsr $6179e(pc)
pub fn i_060ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60EC0);
    cpu.pc = 0x6179E;
    16
}
/// $060EC0: lea.l $2bdd0a.l, a0
pub fn i_060ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDD0A;
    20
}
/// $060EC6: jsr $799de.l
pub fn i_060ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60ECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60ECC);
    cpu.pc = 0x799DE;
    16
}
/// $060ECC: move.w d0, $66(a6)
pub fn i_060ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60ED0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060ED0: lea.l $2bde0e.l, a0
pub fn i_060ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60ED6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDE0E;
    20
}
/// $060ED6: jsr $799de.l
pub fn i_060ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60EDC);
    cpu.pc = 0x799DE;
    16
}
/// $060EDC: move.w d0, $78(a6)
pub fn i_060edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EE0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $060EE0: move.b #$ff, $7b(a6)
pub fn i_060ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060EE6: lea.l $2c32d6.l, a0
pub fn i_060ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C32D6;
    20
}
/// $060EEC: move.l a0, $48(a6)
pub fn i_060eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EF0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $060EF0: move.b #$0, $20(a6)
pub fn i_060ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060EF6: jsr $61952(pc)
pub fn i_060ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60EFA);
    cpu.pc = 0x61952;
    16
}
/// $060EFA: lea.l $61254(pc), a1
pub fn i_060efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60EFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61254;
    16
}
/// $060EFE: jsr $4ae.l
pub fn i_060efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60F04);
    cpu.pc = 0x4AE;
    16
}
// ===== AimTurret_Active_060f04  ($060F04, 132 B, props/late_props_turrets_05e000.s)
/// $060F04: lea.l $60f0a(pc), a1
pub fn i_060f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60F0A;
    16
}
/// $060F08: move.l a1, (a6)
pub fn i_060f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060F0A: jsr $6179e(pc)
pub fn i_060f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F0E);
    cpu.pc = 0x6179E;
    16
}
/// $060F0E: jsr $6164c(pc)
pub fn i_060f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F12);
    cpu.pc = 0x6164C;
    16
}
/// $060F12: jsr $61580(pc)
pub fn i_060f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F16);
    cpu.pc = 0x61580;
    16
}
/// $060F16: jsr $617d2(pc)
pub fn i_060f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F1A);
    cpu.pc = 0x617D2;
    16
}
/// $060F1A: jsr $614b6(pc)
pub fn i_060f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F1E);
    cpu.pc = 0x614B6;
    16
}
/// $060F1E: jsr $28d70.l
pub fn i_060f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60F24);
    cpu.pc = 0x28D70;
    16
}
/// $060F24: jsr $28758.l
pub fn i_060f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60F2A);
    cpu.pc = 0x28758;
    16
}
/// $060F2A: bcc.w $60f44
pub fn i_060f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60F44; return 10; }
    8
}
/// $060F2E: lea.l $6100c(pc), a1
pub fn i_060f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6100C;
    16
}
/// $060F32: move.l a1, (a6)
pub fn i_060f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060F34: cmpi.b #$ff, $9a(a6)
pub fn i_060f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F3A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $060F3A: bne.w $60f44
pub fn i_060f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F3E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60F44; return 10; }
    8
}
/// $060F3E: lea.l $610f4(pc), a1
pub fn i_060f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x610F4;
    16
}
/// $060F42: move.l a1, (a6)
pub fn i_060f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060F44: jsr $618b0(pc)
pub fn i_060f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F48);
    cpu.pc = 0x618B0;
    16
}
/// $060F48: jsr $6174a(pc)
pub fn i_060f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F4C);
    cpu.pc = 0x6174A;
    16
}
/// $060F4C: bcc.w $60f56
pub fn i_060f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60F56; return 10; }
    8
}
/// $060F50: lea.l $60f88(pc), a1
pub fn i_060f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60F88;
    16
}
/// $060F54: move.l a1, (a6)
pub fn i_060f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060F56: jsr $6197c(pc)
pub fn i_060f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F5A);
    cpu.pc = 0x6197C;
    16
}
/// $060F5A: bcc.w $60f64
pub fn i_060f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60F64; return 10; }
    8
}
/// $060F5E: lea.l $61204(pc), a1
pub fn i_060f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61204;
    16
}
/// $060F62: move.l a1, (a6)
pub fn i_060f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060F64: movea.l #$ffffffff, a0
pub fn i_060f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F6A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $060F6A: lea.l $2c33a0.l, a0
pub fn i_060f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C33A0;
    20
}
/// $060F70: jsr $5dd5c.l
pub fn i_060f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60F76);
    cpu.pc = 0x5DD5C;
    16
}
/// $060F76: bcc.w $60f86
pub fn i_060f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60F86; return 10; }
    8
}
/// $060F7A: move.b #$ff, $20(a6)
pub fn i_060f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $060F80: jmp $518.l
pub fn i_060f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F86;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $060F86: rts
pub fn i_060f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_ActiveB_060f88  ($060F88, 132 B, props/late_props_turrets_05e000.s)
/// $060F88: lea.l $60f8e(pc), a1
pub fn i_060f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60F8E;
    16
}
/// $060F8C: move.l a1, (a6)
pub fn i_060f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060F8E: jsr $6179e(pc)
pub fn i_060f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F92);
    cpu.pc = 0x6179E;
    16
}
/// $060F92: jsr $6161e(pc)
pub fn i_060f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F96);
    cpu.pc = 0x6161E;
    16
}
/// $060F96: jsr $61580(pc)
pub fn i_060f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F9A);
    cpu.pc = 0x61580;
    16
}
/// $060F9A: jsr $617d2(pc)
pub fn i_060f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60F9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60F9E);
    cpu.pc = 0x617D2;
    16
}
/// $060F9E: jsr $614b6(pc)
pub fn i_060f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60FA2);
    cpu.pc = 0x614B6;
    16
}
/// $060FA2: jsr $28d70.l
pub fn i_060fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60FA8);
    cpu.pc = 0x28D70;
    16
}
/// $060FA8: jsr $28758.l
pub fn i_060fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60FAE);
    cpu.pc = 0x28758;
    16
}
/// $060FAE: bcc.w $60fc8
pub fn i_060fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60FC8; return 10; }
    8
}
/// $060FB2: lea.l $6100c(pc), a1
pub fn i_060fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6100C;
    16
}
/// $060FB6: move.l a1, (a6)
pub fn i_060fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060FB8: cmpi.b #$ff, $9a(a6)
pub fn i_060fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FBE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $060FBE: bne.w $60fc8
pub fn i_060fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FC2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x60FC8; return 10; }
    8
}
/// $060FC2: lea.l $610f4(pc), a1
pub fn i_060fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x610F4;
    16
}
/// $060FC6: move.l a1, (a6)
pub fn i_060fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060FC8: jsr $618b0(pc)
pub fn i_060fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60FCC);
    cpu.pc = 0x618B0;
    16
}
/// $060FCC: jsr $61774(pc)
pub fn i_060fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60FD0);
    cpu.pc = 0x61774;
    16
}
/// $060FD0: bcs.w $60fda
pub fn i_060fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FD4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x60FDA; return 10; }
    8
}
/// $060FD4: lea.l $60f04(pc), a1
pub fn i_060fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60F04;
    16
}
/// $060FD8: move.l a1, (a6)
pub fn i_060fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060FDA: jsr $6197c(pc)
pub fn i_060fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x60FDE);
    cpu.pc = 0x6197C;
    16
}
/// $060FDE: bcc.w $60fe8
pub fn i_060fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60FE8; return 10; }
    8
}
/// $060FE2: lea.l $61204(pc), a1
pub fn i_060fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61204;
    16
}
/// $060FE6: move.l a1, (a6)
pub fn i_060fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FE8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $060FE8: movea.l #$ffffffff, a0
pub fn i_060fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FEE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $060FEE: lea.l $2c33a0.l, a0
pub fn i_060fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C33A0;
    20
}
/// $060FF4: jsr $5dd5c.l
pub fn i_060ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60FFA);
    cpu.pc = 0x5DD5C;
    16
}
/// $060FFA: bcc.w $6100a
pub fn i_060ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60FFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6100A; return 10; }
    8
}
/// $060FFE: move.b #$ff, $20(a6)
pub fn i_060ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61004;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061004: jmp $518.l
pub fn i_061004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6100A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06100A: rts
pub fn i_06100a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6100C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Die_06100c  ($06100C, 232 B, props/late_props_turrets_05e000.s)
/// $06100C: move.w #$1ff, $74(a6)
pub fn i_06100c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61012;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061012: lea.l $2c2cd8.l, a0
pub fn i_061012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61018;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2CD8;
    20
}
/// $061018: jsr $28cd4.l
pub fn i_061018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6101E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6101E);
    cpu.pc = 0x28CD4;
    16
}
/// $06101E: lea.l $61024(pc), a1
pub fn i_06101e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61024;
    16
}
/// $061022: move.l a1, (a6)
pub fn i_061022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61024;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061024: jsr $616ca(pc)
pub fn i_061024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61028;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61028);
    cpu.pc = 0x616CA;
    16
}
/// $061028: jsr $61580(pc)
pub fn i_061028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6102C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6102C);
    cpu.pc = 0x61580;
    16
}
/// $06102C: jsr $617d2(pc)
pub fn i_06102c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61030;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61030);
    cpu.pc = 0x617D2;
    16
}
/// $061030: jsr $28d70.l
pub fn i_061030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61036);
    cpu.pc = 0x28D70;
    16
}
/// $061036: cmpi.w #$1b0, $24(a6)
pub fn i_061036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6103C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1B0, Size::Word, false);
    16
}
/// $06103C: blt.w $61088
pub fn i_06103c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61040;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x61088; return 10; }
    8
}
/// $061040: lea.l $77fd6.l, a1
pub fn i_061040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61046;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $061046: jsr $4ae.l
pub fn i_061046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6104C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6104C);
    cpu.pc = 0x4AE;
    16
}
/// $06104C: jsr $5dd02.l
pub fn i_06104c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61052);
    cpu.pc = 0x5DD02;
    16
}
/// $061052: subi.w #$10, $24(a0)
pub fn i_061052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61058;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $061058: lea.l $2c3406.l, a1
pub fn i_061058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6105E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C3406;
    20
}
/// $06105E: jsr $77c7e.l
pub fn i_06105e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61064;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61064);
    cpu.pc = 0x77C7E;
    16
}
/// $061064: move.w #$102f, d0
pub fn i_061064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61068;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061068: jsr $2352.l
pub fn i_061068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6106E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6106E);
    cpu.pc = 0x2352;
    16
}
/// $06106E: move.w #$10e8, d0
pub fn i_06106e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61072;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061072: jsr $2222.l
pub fn i_061072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61078);
    cpu.pc = 0x2222;
    16
}
/// $061078: move.b #$ff, $20(a6)
pub fn i_061078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6107E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06107E: jsr $61482(pc)
pub fn i_06107e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61082;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61082);
    cpu.pc = 0x61482;
    16
}
/// $061082: jmp $518.l
pub fn i_061082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61088;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $061088: move.b $106f28.l, d0
pub fn i_061088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6108E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $06108E: andi.b #$3, d0
pub fn i_06108e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61092;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $061092: bne.w $610bc
pub fn i_061092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61096;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x610BC; return 10; }
    8
}
/// $061096: lea.l $2c33b4.l, a1
pub fn i_061096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6109C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C33B4;
    20
}
/// $06109C: jsr $77c7e.l
pub fn i_06109c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x610A2);
    cpu.pc = 0x77C7E;
    16
}
/// $0610A2: move.w #$28, d0
pub fn i_0610a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0610A6: btst.b #$0, $3a(a6)
pub fn i_0610a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610AC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0610AC: beq.w $610b2
pub fn i_0610ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x610B2; return 10; }
    8
}
/// $0610B0: neg.w d0
pub fn i_0610b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610B2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0610B2: add.w d0, $22(a0)
pub fn i_0610b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610B6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0610B6: subi.w #$18, $24(a0)
pub fn i_0610b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610BC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0610BC: movea.l #$ffffffff, a0
pub fn i_0610bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610C2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0610C2: lea.l $2c33a0.l, a0
pub fn i_0610c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C33A0;
    20
}
/// $0610C8: jsr $5dd5c.l
pub fn i_0610c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x610CE);
    cpu.pc = 0x5DD5C;
    16
}
/// $0610CE: bcc.w $610f2
pub fn i_0610ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x610F2; return 10; }
    8
}
/// $0610D2: move.w #$102f, d0
pub fn i_0610d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610D6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0610D6: jsr $2352.l
pub fn i_0610d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x610DC);
    cpu.pc = 0x2352;
    16
}
/// $0610DC: move.w #$10e8, d0
pub fn i_0610dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0610E0: jsr $2222.l
pub fn i_0610e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x610E6);
    cpu.pc = 0x2222;
    16
}
/// $0610E6: move.b #$ff, $20(a6)
pub fn i_0610e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0610EC: jmp $518.l
pub fn i_0610ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0610F2: rts
pub fn i_0610f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_DieB_0610f4  ($0610F4, 272 B, props/late_props_turrets_05e000.s)
/// $0610F4: move.w #$140, $72(a6)
pub fn i_0610f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x610FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0610FA: move.w #$160, $74(a6)
pub fn i_0610fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61100;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061100: move.b $9c(a6), d1
pub fn i_061100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61104;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061104: asl.b #$1, d1
pub fn i_061104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61106;
    cpu.ir = 0xE301;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $061106: move.b d1, $9c(a6)
pub fn i_061106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6110A;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06110A: lea.l $2c2cd8.l, a0
pub fn i_06110a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61110;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2CD8;
    20
}
/// $061110: jsr $28cd4.l
pub fn i_061110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61116);
    cpu.pc = 0x28CD4;
    16
}
/// $061116: lea.l $6111c(pc), a1
pub fn i_061116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6111A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6111C;
    16
}
/// $06111A: move.l a1, (a6)
pub fn i_06111a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6111C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06111C: jsr $6164c(pc)
pub fn i_06111c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61120;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61120);
    cpu.pc = 0x6164C;
    16
}
/// $061120: jsr $61580(pc)
pub fn i_061120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61124;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61124);
    cpu.pc = 0x61580;
    16
}
/// $061124: jsr $617d2(pc)
pub fn i_061124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61128;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61128);
    cpu.pc = 0x617D2;
    16
}
/// $061128: jsr $28d70.l
pub fn i_061128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6112E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6112E);
    cpu.pc = 0x28D70;
    16
}
/// $06112E: move.w $22(a6), d0
pub fn i_06112e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61132;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061132: move.w $24(a6), d1
pub fn i_061132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61136;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061136: jsr $4400e.l
pub fn i_061136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6113C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6113C);
    cpu.pc = 0x4400E;
    16
}
/// $06113C: cmpi.w #$3e0, d0
pub fn i_06113c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61140;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3E0, Size::Word, false);
    8
}
/// $061140: blt.w $61198
pub fn i_061140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61144;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x61198; return 10; }
    8
}
/// $061144: lea.l $77fd6.l, a1
pub fn i_061144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6114A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06114A: jsr $4ae.l
pub fn i_06114a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61150);
    cpu.pc = 0x4AE;
    16
}
/// $061150: jsr $5dd02.l
pub fn i_061150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61156);
    cpu.pc = 0x5DD02;
    16
}
/// $061156: subi.w #$10, $24(a0)
pub fn i_061156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6115C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06115C: lea.l $2c3406.l, a1
pub fn i_06115c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61162;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C3406;
    20
}
/// $061162: jsr $77c7e.l
pub fn i_061162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61168);
    cpu.pc = 0x77C7E;
    16
}
/// $061168: move.w #$102f, d0
pub fn i_061168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6116C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06116C: jsr $2352.l
pub fn i_06116c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61172;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61172);
    cpu.pc = 0x2352;
    16
}
/// $061172: move.w #$10e8, d0
pub fn i_061172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61176;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061176: jsr $2222.l
pub fn i_061176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6117C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6117C);
    cpu.pc = 0x2222;
    16
}
/// $06117C: move.b #$ff, $20(a6)
pub fn i_06117c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61182;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061182: lea.l $612a8(pc), a1
pub fn i_061182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61186;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x612A8;
    16
}
/// $061186: jsr $4ae.l
pub fn i_061186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6118C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6118C);
    cpu.pc = 0x4AE;
    16
}
/// $06118C: jsr $5dd02.l
pub fn i_06118c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61192);
    cpu.pc = 0x5DD02;
    16
}
/// $061192: jmp $518.l
pub fn i_061192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61198;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $061198: move.b $106f28.l, d0
pub fn i_061198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6119E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $06119E: andi.b #$3, d0
pub fn i_06119e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611A2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0611A2: bne.w $611cc
pub fn i_0611a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x611CC; return 10; }
    8
}
/// $0611A6: lea.l $2c33b4.l, a1
pub fn i_0611a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C33B4;
    20
}
/// $0611AC: jsr $77c7e.l
pub fn i_0611ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x611B2);
    cpu.pc = 0x77C7E;
    16
}
/// $0611B2: move.w #$28, d0
pub fn i_0611b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0611B6: btst.b #$0, $3a(a6)
pub fn i_0611b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611BC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0611BC: beq.w $611c2
pub fn i_0611bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x611C2; return 10; }
    8
}
/// $0611C0: neg.w d0
pub fn i_0611c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0611C2: add.w d0, $22(a0)
pub fn i_0611c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611C6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0611C6: subi.w #$18, $24(a0)
pub fn i_0611c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611CC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0611CC: movea.l #$ffffffff, a0
pub fn i_0611cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611D2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0611D2: lea.l $2c33a0.l, a0
pub fn i_0611d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C33A0;
    20
}
/// $0611D8: jsr $5dd5c.l
pub fn i_0611d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x611DE);
    cpu.pc = 0x5DD5C;
    16
}
/// $0611DE: bcc.w $61202
pub fn i_0611de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61202; return 10; }
    8
}
/// $0611E2: move.w #$102f, d0
pub fn i_0611e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0611E6: jsr $2352.l
pub fn i_0611e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x611EC);
    cpu.pc = 0x2352;
    16
}
/// $0611EC: move.w #$10e8, d0
pub fn i_0611ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611F0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0611F0: jsr $2222.l
pub fn i_0611f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x611F6);
    cpu.pc = 0x2222;
    16
}
/// $0611F6: move.b #$ff, $20(a6)
pub fn i_0611f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x611FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0611FC: jmp $518.l
pub fn i_0611fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61202;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $061202: rts
pub fn i_061202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61204;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Dormant_061204  ($061204, 80 B, props/late_props_turrets_05e000.s)
/// $061204: move.w #$1ff, $74(a6)
pub fn i_061204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6120A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06120A: lea.l $61210(pc), a1
pub fn i_06120a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6120E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61210;
    16
}
/// $06120E: move.l a1, (a6)
pub fn i_06120e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61210;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061210: jsr $616ca(pc)
pub fn i_061210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61214;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61214);
    cpu.pc = 0x616CA;
    16
}
/// $061214: jsr $61580(pc)
pub fn i_061214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61218;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61218);
    cpu.pc = 0x61580;
    16
}
/// $061218: jsr $617d2(pc)
pub fn i_061218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6121C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6121C);
    cpu.pc = 0x617D2;
    16
}
/// $06121C: jsr $614b6(pc)
pub fn i_06121c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61220;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61220);
    cpu.pc = 0x614B6;
    16
}
/// $061220: jsr $28d70.l
pub fn i_061220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61226);
    cpu.pc = 0x28D70;
    16
}
/// $061226: movea.l #$ffffffff, a0
pub fn i_061226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6122C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06122C: lea.l $2c33a0.l, a0
pub fn i_06122c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61232;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C33A0;
    20
}
/// $061232: jsr $5dd56.l
pub fn i_061232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61238);
    cpu.pc = 0x5DD56;
    16
}
/// $061238: bcc.w $61252
pub fn i_061238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6123C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61252; return 10; }
    8
}
/// $06123C: move.b #$ff, $20(a6)
pub fn i_06123c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61242;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061242: move.w #$10e8, d0
pub fn i_061242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61246;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061246: jsr $2222.l
pub fn i_061246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6124C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6124C);
    cpu.pc = 0x2222;
    16
}
/// $06124C: jmp $518.l
pub fn i_06124c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61252;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $061252: rts
pub fn i_061252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61254;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Gun_061254  ($061254, 84 B, props/late_props_turrets_05e000.s)
/// $061254: move.w #$189, d1
pub fn i_061254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61258;
    cpu.ir = 0x323C;
    let v1: u32 = 0x189;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061258: jsr $236e.l
pub fn i_061258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6125E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6125E);
    cpu.pc = 0x236E;
    16
}
/// $06125E: move.w #$18b, d1
pub fn i_06125e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61262;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061262: jsr $236e.l
pub fn i_061262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61268);
    cpu.pc = 0x236E;
    16
}
/// $061268: move.b #$0, $46(a6)
pub fn i_061268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6126E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06126E: move.b #$0, $3b(a6)
pub fn i_06126e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61274;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061274: lea.l $6127a(pc), a1
pub fn i_061274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6127A;
    16
}
/// $061278: move.l a1, (a6)
pub fn i_061278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6127A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06127A: jsr $5e4ee.l
pub fn i_06127a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61280);
    cpu.pc = 0x5E4EE;
    16
}
/// $061280: movea.l $c(a6), a0
pub fn i_061280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61284;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $061284: move.b $3a(a0), $3a(a6)
pub fn i_061284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6128A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06128A: jsr $615f6(pc)
pub fn i_06128a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6128E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6128E);
    cpu.pc = 0x615F6;
    16
}
/// $06128E: jsr $614e6(pc)
pub fn i_06128e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61292;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x61292);
    cpu.pc = 0x614E6;
    16
}
/// $061292: movea.l $c(a6), a0
pub fn i_061292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61296;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $061296: cmpi.b #$ff, $20(a0)
pub fn i_061296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6129C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $06129C: bne.w $612a6
pub fn i_06129c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x612A6; return 10; }
    8
}
/// $0612A0: jmp $518.l
pub fn i_0612a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612A6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0612A6: rts
pub fn i_0612a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_GunAttack_0612a8  ($0612A8, 62 B, props/late_props_turrets_05e000.s)
/// $0612A8: move.w #$4, $86(a6)
pub fn i_0612a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0612AE: lea.l $612b4(pc), a1
pub fn i_0612ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x612B4;
    16
}
/// $0612B2: move.l a1, (a6)
pub fn i_0612b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0612B4: jsr $2783a.l
pub fn i_0612b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x612BA);
    cpu.pc = 0x2783A;
    16
}
/// $0612BA: lea.l $2c3282.l, a0
pub fn i_0612ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3282;
    20
}
/// $0612C0: move.l a0, $4c(a6)
pub fn i_0612c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0612C4: jsr $283ca.l
pub fn i_0612c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x612CA);
    cpu.pc = 0x283CA;
    16
}
/// $0612CA: jsr $283ca.l
pub fn i_0612ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x612D0);
    cpu.pc = 0x283CA;
    16
}
/// $0612D0: jsr $283d8.l
pub fn i_0612d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x612D6);
    cpu.pc = 0x283D8;
    16
}
/// $0612D6: subq.w #$1, $86(a6)
pub fn i_0612d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612DA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0612DA: bpl.w $612e4
pub fn i_0612da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612DE;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x612E4; return 10; }
    8
}
/// $0612DE: jmp $518.l
pub fn i_0612de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612E4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0612E4: rts
pub fn i_0612e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Barrel_0612e6  ($0612E6, 94 B, props/late_props_turrets_05e000.s)
/// $0612E6: move.w #$189, d1
pub fn i_0612e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x189;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0612EA: jsr $236e.l
pub fn i_0612ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x612F0);
    cpu.pc = 0x236E;
    16
}
/// $0612F0: jsr $5e4ee.l
pub fn i_0612f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x612F6);
    cpu.pc = 0x5E4EE;
    16
}
/// $0612F6: subq.w #$1, $38(a6)
pub fn i_0612f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612FA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0612FA: movea.l $c(a6), a0
pub fn i_0612fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0612FE: move.b $3a(a0), $3a(a6)
pub fn i_0612fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61304;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $061304: lea.l $2c2d86.l, a0
pub fn i_061304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6130A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2D86;
    20
}
/// $06130A: jsr $28cd4.l
pub fn i_06130a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61310);
    cpu.pc = 0x28CD4;
    16
}
/// $061310: lea.l $61316(pc), a1
pub fn i_061310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61314;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61316;
    16
}
/// $061314: move.l a1, (a6)
pub fn i_061314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61316;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061316: jsr $2783a.l
pub fn i_061316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6131C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6131C);
    cpu.pc = 0x2783A;
    16
}
/// $06131C: jsr $28d70.l
pub fn i_06131c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61322);
    cpu.pc = 0x28D70;
    16
}
/// $061322: bcs.w $6133c
pub fn i_061322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61326;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6133C; return 10; }
    8
}
/// $061326: movea.l #$ffffffff, a0
pub fn i_061326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6132C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06132C: lea.l $2c33aa.l, a0
pub fn i_06132c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61332;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C33AA;
    20
}
/// $061332: jsr $5dd56.l
pub fn i_061332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61338);
    cpu.pc = 0x5DD56;
    16
}
/// $061338: bcc.w $61342
pub fn i_061338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6133C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61342; return 10; }
    8
}
/// $06133C: jmp $518.l
pub fn i_06133c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61342;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $061342: rts
pub fn i_061342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61344;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Shell_061344  ($061344, 270 B, props/late_props_turrets_05e000.s)
/// $061344: move.w #$109b, d0
pub fn i_061344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61348;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061348: jsr $2352.l
pub fn i_061348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6134E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6134E);
    cpu.pc = 0x2352;
    16
}
/// $06134E: move.w #$187, d1
pub fn i_06134e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61352;
    cpu.ir = 0x323C;
    let v1: u32 = 0x187;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061352: jsr $236e.l
pub fn i_061352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61358);
    cpu.pc = 0x236E;
    16
}
/// $061358: lea.l $2c2df8.l, a0
pub fn i_061358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6135E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C2DF8;
    20
}
/// $06135E: jsr $28cd4.l
pub fn i_06135e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61364;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61364);
    cpu.pc = 0x28CD4;
    16
}
/// $061364: jsr $267e2.l
pub fn i_061364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6136A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6136A);
    cpu.pc = 0x267E2;
    16
}
/// $06136A: lea.l $2bdf94.l, a0
pub fn i_06136a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61370;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDF94;
    20
}
/// $061370: jsr $799de.l
pub fn i_061370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61376);
    cpu.pc = 0x799DE;
    16
}
/// $061376: move.w d0, d0
pub fn i_061376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61378;
    cpu.ir = 0x3000;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061378: neg.w d0
pub fn i_061378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6137A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06137A: move.w d0, $2e(a6)
pub fn i_06137a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6137E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06137E: movea.l $c(a6), a0
pub fn i_06137e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61382;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $061382: move.w $28(a0), $28(a6)
pub fn i_061382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61388;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $061388: move.w $22(a0), $22(a6)
pub fn i_061388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6138E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06138E: move.w $24(a0), $24(a6)
pub fn i_06138e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61394;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $061394: subi.w #$10, $24(a6)
pub fn i_061394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6139A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06139A: subq.w #$1, $38(a6)
pub fn i_06139a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6139E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06139E: move.w #$3c, $66(a6)
pub fn i_06139e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0613A4: lea.l $2c334c.l, a0
pub fn i_0613a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C334C;
    20
}
/// $0613AA: move.l a0, $48(a6)
pub fn i_0613aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613AE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0613AE: lea.l $2c322e.l, a0
pub fn i_0613ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C322E;
    20
}
/// $0613B4: move.l a0, $4c(a6)
pub fn i_0613b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613B8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0613B8: jsr $283ca.l
pub fn i_0613b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x613BE);
    cpu.pc = 0x283CA;
    16
}
/// $0613BE: lea.l $613c4(pc), a1
pub fn i_0613be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x613C4;
    16
}
/// $0613C2: move.l a1, (a6)
pub fn i_0613c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0613C4: jsr $27d50.l
pub fn i_0613c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x613CA);
    cpu.pc = 0x27D50;
    16
}
/// $0613CA: bcc.w $613d4
pub fn i_0613ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x613D4; return 10; }
    8
}
/// $0613CE: lea.l $61452(pc), a1
pub fn i_0613ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61452;
    16
}
/// $0613D2: move.l a1, (a6)
pub fn i_0613d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0613D4: jsr $28d70.l
pub fn i_0613d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x613DA);
    cpu.pc = 0x28D70;
    16
}
/// $0613DA: movea.l $c(a6), a0
pub fn i_0613da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613DE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0613DE: cmpi.b #$ff, $20(a0)
pub fn i_0613de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613E4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0613E4: bne.w $613ee
pub fn i_0613e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x613EE; return 10; }
    8
}
/// $0613E8: lea.l $61452(pc), a1
pub fn i_0613e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61452;
    16
}
/// $0613EC: move.l a1, (a6)
pub fn i_0613ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0613EE: jsr $2870a.l
pub fn i_0613ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x613F4);
    cpu.pc = 0x2870A;
    16
}
/// $0613F4: bcc.w $61408
pub fn i_0613f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61408; return 10; }
    8
}
/// $0613F8: bclr.b #$3, $13(a6)
pub fn i_0613f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x613FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0613FE: move.w #$108d, d0
pub fn i_0613fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61402;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061402: jsr $2352.l
pub fn i_061402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61408);
    cpu.pc = 0x2352;
    16
}
/// $061408: jsr $28758.l
pub fn i_061408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6140E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6140E);
    cpu.pc = 0x28758;
    16
}
/// $06140E: bcs.w $61422
pub fn i_06140e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61412;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x61422; return 10; }
    8
}
/// $061412: jsr $283d8.l
pub fn i_061412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61418);
    cpu.pc = 0x283D8;
    16
}
/// $061418: btst.b #$1, $13(a6)
pub fn i_061418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6141E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06141E: beq.w $61428
pub fn i_06141e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61422;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61428; return 10; }
    8
}
/// $061422: lea.l $61452(pc), a1
pub fn i_061422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61426;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61452;
    16
}
/// $061426: move.l a1, (a6)
pub fn i_061426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61428;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061428: clr.w d0
pub fn i_061428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6142A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06142A: sub.w $28(a6), d0
pub fn i_06142a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6142E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06142E: asr.w #$4, d0
pub fn i_06142e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61430;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $061430: move.w d0, $2c(a6)
pub fn i_061430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61434;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061434: movea.l #$ffffffff, a0
pub fn i_061434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6143A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06143A: lea.l $2c33aa.l, a0
pub fn i_06143a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61440;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C33AA;
    20
}
/// $061440: jsr $5dd56.l
pub fn i_061440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61446);
    cpu.pc = 0x5DD56;
    16
}
/// $061446: bcc.w $61450
pub fn i_061446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6144A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x61450; return 10; }
    8
}
/// $06144A: jmp $518.l
pub fn i_06144a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61450;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $061450: rts
pub fn i_061450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61452;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_ShellBurst_061452  ($061452, 46 B, props/late_props_turrets_05e000.s)
/// $061452: move.w #$1022, d0
pub fn i_061452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61456;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061456: jsr $2352.l
pub fn i_061456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6145C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6145C);
    cpu.pc = 0x2352;
    16
}
/// $06145C: lea.l $77f6a.l, a1
pub fn i_06145c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61462;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $061462: jsr $4ae.l
pub fn i_061462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61468);
    cpu.pc = 0x4AE;
    16
}
/// $061468: jsr $5dd02.l
pub fn i_061468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6146E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6146E);
    cpu.pc = 0x5DD02;
    16
}
/// $06146E: lea.l $61474(pc), a1
pub fn i_06146e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61472;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61474;
    16
}
/// $061472: move.l a1, (a6)
pub fn i_061472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61474;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $061474: jsr $28d70.l
pub fn i_061474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6147A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6147A);
    cpu.pc = 0x28D70;
    16
}
/// $06147A: jmp $518.l
pub fn i_06147a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61480;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== AimTurret_Rts_061480  ($061480, 2 B, props/late_props_turrets_05e000.s)
/// $061480: rts
pub fn i_061480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_DropItem_061482  ($061482, 46 B, props/late_props_turrets_05e000.s)
/// $061482: move.w $22(a6), d0
pub fn i_061482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61486;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061486: movem.w d0, -(a7)
pub fn i_061486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6148A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06148A: move.w #$10, d0
pub fn i_06148a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6148E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06148E: btst.b #$0, $3a(a6)
pub fn i_06148e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61494;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $061494: beq.w $6149a
pub fn i_061494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61498;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6149A; return 10; }
    8
}
/// $061498: neg.w d0
pub fn i_061498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6149A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06149A: add.w d0, $22(a6)
pub fn i_06149a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6149E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06149E: move.b $9d(a6), d0
pub fn i_06149e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614A2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0614A2: move.w #$9e, d1
pub fn i_0614a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0614A6: jsr $9a7cc.l
pub fn i_0614a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x614AC);
    cpu.pc = 0x9A7CC;
    16
}
/// $0614AC: movem.w (a7)+, d0
pub fn i_0614ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614B0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetTaskW_0614b0  ($0614B0, 6 B, kernel/task_setters_w.c)
/// $0614B0: move.w d0, $22(a6)
pub fn i_0614b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614B4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0614B4: rts
pub fn i_0614b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Cooldown_0614b6  ($0614B6, 40 B, props/late_props_turrets_05e000.s)
/// $0614B6: clr.l d0
pub fn i_0614b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614B8;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0614B8: move.b $71(a6), d0
pub fn i_0614b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614BC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0614BC: cmpi.b #$6, d0
pub fn i_0614bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614C0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $0614C0: bls.w $614c8
pub fn i_0614c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614C4;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x614C8; return 10; }
    8
}
/// $0614C4: move.b #$0, d0
pub fn i_0614c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614C8;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0614C8: movea.l #$2c2cbc, a0
pub fn i_0614c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614CE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C2CBC;
    cpu.a[0] = v1;
    4
}
/// $0614CE: lsl.w #$2, d0
pub fn i_0614ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614D0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0614D0: movea.l (a0, d0.w), a0
pub fn i_0614d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614D4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0614D4: cmpa.l #$ffffffff, a0
pub fn i_0614d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614DA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0614DA: beq.w $614e4
pub fn i_0614da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x614E4; return 10; }
    8
}
// ===== JsrAbsThunk_0614de  ($0614DE, 8 B, thunks/jsr_abs_thunks.c)
/// $0614DE: jsr $28cd4.l
pub fn i_0614de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x614E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0614E4: rts
pub fn i_0614e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_AnimScript_0614e6  ($0614E6, 146 B, props/late_props_turrets_05e000.s)
/// $0614E6: movea.l $c(a6), a0
pub fn i_0614e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0614EA: clr.l d0
pub fn i_0614ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614EC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0614EC: move.b $71(a0), d0
pub fn i_0614ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614F0;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0614F0: cmpi.b #$6, d0
pub fn i_0614f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614F4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $0614F4: bls.w $614fc
pub fn i_0614f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614F8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x614FC; return 10; }
    8
}
/// $0614F8: move.b #$0, d0
pub fn i_0614f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614FC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0614FC: asl.l #$2, d0
pub fn i_0614fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x614FE;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $0614FE: lea.l $2c3212.l, a0
pub fn i_0614fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61504;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3212;
    20
}
/// $061504: movea.l (a0, d0.w), a1
pub fn i_061504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61508;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $061508: move.l a1, $3c(a6)
pub fn i_061508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6150C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06150C: subq.b #$1, $46(a6)
pub fn i_06150c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61510;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $061510: cmpi.b #$0, $46(a6)
pub fn i_061510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61516;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $061516: bgt.w $6154c
pub fn i_061516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6151A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6154C; return 10; }
    8
}
/// $06151A: addq.b #$1, $3b(a6)
pub fn i_06151a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6151E;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06151E: movea.l $3c(a6), a0
pub fn i_06151e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61522;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $061522: clr.l d0
pub fn i_061522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61524;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $061524: move.b $3b(a6), d0
pub fn i_061524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61528;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061528: asl.l #$3, d0
pub fn i_061528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6152A;
    cpu.ir = 0xE780;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, true);
    cpu.d[0] = r;
    12
}
/// $06152A: movea.l (a0, d0.w), a1
pub fn i_06152a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6152E;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06152E: move.l a1, d5
pub fn i_06152e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61530;
    cpu.ir = 0x2A09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $061530: cmpi.l #$ffffffff, d5
pub fn i_061530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61536;
    cpu.ir = 0xC85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $061536: bne.w $61540
pub fn i_061536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6153A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x61540; return 10; }
    8
}
/// $06153A: move.b #$0, $3b(a6)
pub fn i_06153a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61540;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061540: move.b $3b(a6), d0
pub fn i_061540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61544;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061544: asl.l #$3, d0
pub fn i_061544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61546;
    cpu.ir = 0xE780;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, true);
    cpu.d[0] = r;
    12
}
/// $061546: move.b $4(a0, d0.w), $46(a6)
pub fn i_061546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6154C;
    cpu.ir = 0x1D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad4, v3 as u8);
    22
}
/// $06154C: movea.l $3c(a6), a0
pub fn i_06154c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61550;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $061550: clr.l d0
pub fn i_061550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61552;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $061552: move.b $3b(a6), d0
pub fn i_061552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61556;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061556: asl.l #$3, d0
pub fn i_061556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61558;
    cpu.ir = 0xE780;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, true);
    cpu.d[0] = r;
    12
}
/// $061558: movea.l (a0, d0.w), a0
pub fn i_061558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6155C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06155C: move.b $3a(a6), d5
pub fn i_06155c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61560;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061560: andi.b #$3, d5
pub fn i_061560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61564;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $061564: move.l $22(a6), d0
pub fn i_061564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61568;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $061568: move.w $38(a6), d1
pub fn i_061568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6156C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06156C: move.w $14(a6), d2
pub fn i_06156c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61570;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061570: move.b $33(a6), d3
pub fn i_061570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61574;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061574: move.b $32(a6), d4
pub fn i_061574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61578;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
// ===== JsrAbsThunk_061578  ($061578, 8 B, thunks/jsr_abs_thunks.c)
/// $061578: jsr $5a9e2.l
pub fn i_061578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6157E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6157E);
    cpu.pc = 0x5A9E2;
    16
}
/// $06157E: rts
pub fn i_06157e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61580;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_HitCheck_061580  ($061580, 118 B, props/late_props_turrets_05e000.s)
/// $061580: addq.b #$1, $70(a6)
pub fn i_061580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61584;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $061584: move.b $70(a6), d0
pub fn i_061584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61588;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061588: cmpi.b #$2, d0
pub fn i_061588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6158C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $06158C: bne.w $615a8
pub fn i_06158c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61590;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x615A8; return 10; }
    8
}
/// $061590: move.w $18(a6), d0
pub fn i_061590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61594;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061594: move.w $16(a6), $18(a6)
pub fn i_061594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6159A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06159A: move.w d0, $16(a6)
pub fn i_06159a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6159E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06159E: move.w d0, $14(a6)
pub fn i_06159e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0615A2: move.b #$0, $70(a6)
pub fn i_0615a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615A8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0615A8: move.b $7c(a6), d0
pub fn i_0615a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615AC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0615AC: asl.b #$1, d0
pub fn i_0615ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615AE;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $0615AE: move.b d0, $7c(a6)
pub fn i_0615ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615B2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0615B2: jsr $2870a.l
pub fn i_0615b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x615B8);
    cpu.pc = 0x2870A;
    16
}
/// $0615B8: bcc.w $615d2
pub fn i_0615b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x615D2; return 10; }
    8
}
/// $0615BC: bset.b #$0, $7c(a6)
pub fn i_0615bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615C2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0615C2: bclr.b #$3, $13(a6)
pub fn i_0615c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615C8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0615C8: move.w #$108d, d0
pub fn i_0615c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615CC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0615CC: jsr $2352.l
pub fn i_0615cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x615D2);
    cpu.pc = 0x2352;
    16
}
/// $0615D2: btst.b #$0, $7c(a6)
pub fn i_0615d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615D8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0615D8: beq.w $615ee
pub fn i_0615d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x615EE; return 10; }
    8
}
/// $0615DC: btst.b #$1, $7c(a6)
pub fn i_0615dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615E2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0615E2: bne.w $615ee
pub fn i_0615e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x615EE; return 10; }
    8
}
/// $0615E6: move.w $1a(a6), $14(a6)
pub fn i_0615e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615EC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0615EC: rts
pub fn i_0615ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0615EE: move.w $16(a6), $14(a6)
pub fn i_0615ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615F4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0615F4: rts
pub fn i_0615f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_GunOffset_0615f6  ($0615F6, 40 B, props/late_props_turrets_05e000.s)
/// $0615F6: movea.l $c(a6), a0
pub fn i_0615f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x615FA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0615FA: btst.b #$0, $7c(a0)
pub fn i_0615fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61600;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $061600: beq.w $61616
pub fn i_061600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61604;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61616; return 10; }
    8
}
/// $061604: btst.b #$1, $7c(a0)
pub fn i_061604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6160A;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06160A: bne.w $61616
pub fn i_06160a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6160E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x61616; return 10; }
    8
}
/// $06160E: move.w $18(a6), $14(a6)
pub fn i_06160e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61614;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $061614: rts
pub fn i_061614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61616;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061616: move.w $16(a6), $14(a6)
pub fn i_061616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6161C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06161C: rts
pub fn i_06161c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6161E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_SinCos40_06161e  ($06161E, 38 B, props/late_props_turrets_05e000.s)
/// $06161E: subi.w #$4, $34(a6)
pub fn i_06161e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61624;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $061624: andi.w #$ff, $34(a6)
pub fn i_061624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6162A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06162A: move.w $34(a6), d0
pub fn i_06162a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6162E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06162E: move.w #$40, d1
pub fn i_06162e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61632;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061632: jsr $13c0e.l
pub fn i_061632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61638;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61638);
    cpu.pc = 0x13C0E;
    16
}
/// $061638: asl.w #$1, d1
pub fn i_061638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6163A;
    cpu.ir = 0xE341;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06163A: move.w #$0, $28(a6)
pub fn i_06163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61640;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061640: move.w d2, $2a(a6)
pub fn i_061640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61644;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_061644  ($061644, 8 B, thunks/jsr_abs_thunks.c)
/// $061644: jsr $27cee.l
pub fn i_061644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6164A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6164A);
    cpu.pc = 0x27CEE;
    16
}
/// $06164A: rts
pub fn i_06164a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6164C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_AimAtPlayer_06164c  ($06164C, 118 B, props/late_props_turrets_05e000.s)
/// $06164C: move.w $72(a6), d0
pub fn i_06164c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61650;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061650: move.w $74(a6), d1
pub fn i_061650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61654;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061654: sub.w $22(a6), d0
pub fn i_061654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61658;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $061658: sub.w $24(a6), d1
pub fn i_061658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6165C;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06165C: asr.w #$1, d0
pub fn i_06165c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6165E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06165E: asr.w #$1, d1
pub fn i_06165e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61660;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $061660: jsr $5e018.l
pub fn i_061660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61666);
    cpu.pc = 0x5E018;
    16
}
/// $061666: clr.w d1
pub fn i_061666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61668;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $061668: move.b $9c(a6), d1
pub fn i_061668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6166C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06166C: andi.w #$1f, d1
pub fn i_06166c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61670;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $061670: btst.b #$0, $3a(a6)
pub fn i_061670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61676;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $061676: beq.w $6167c
pub fn i_061676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6167A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6167C; return 10; }
    8
}
/// $06167A: neg.w d1
pub fn i_06167a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6167C;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06167C: add.w d1, d0
pub fn i_06167c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6167E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06167E: andi.w #$ff, d0
pub fn i_06167e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61682;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $061682: move.w d0, $34(a6)
pub fn i_061682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61686;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061686: move.w $72(a6), d2
pub fn i_061686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6168A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06168A: move.w $74(a6), d3
pub fn i_06168a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6168E;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06168E: move.w $22(a6), d0
pub fn i_06168e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61692;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061692: move.w $24(a6), d1
pub fn i_061692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61696;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061696: jsr $5e23a.l
pub fn i_061696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6169C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6169C);
    cpu.pc = 0x5E23A;
    16
}
/// $06169C: asl.w #$3, d0
pub fn i_06169c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6169E;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06169E: move.w d0, d1
pub fn i_06169e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616A0;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0616A0: move.w $34(a6), d0
pub fn i_0616a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616A4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0616A4: andi.w #$ff, d0
pub fn i_0616a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616A8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0616A8: jsr $13c0e.l
pub fn i_0616a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x616AE);
    cpu.pc = 0x13C0E;
    16
}
/// $0616AE: sub.w $28(a6), d1
pub fn i_0616ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616B2;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0616B2: sub.w $2a(a6), d2
pub fn i_0616b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616B6;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0616B6: asr.w #$3, d1
pub fn i_0616b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616B8;
    cpu.ir = 0xE641;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0616B8: asr.w #$4, d2
pub fn i_0616b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616BA;
    cpu.ir = 0xE842;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0616BA: move.w d1, $2c(a6)
pub fn i_0616ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616BE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0616BE: move.w d2, $2e(a6)
pub fn i_0616be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616C2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_0616c2  ($0616C2, 8 B, thunks/jsr_abs_thunks.c)
/// $0616C2: jsr $27cee.l
pub fn i_0616c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x616C8);
    cpu.pc = 0x27CEE;
    16
}
/// $0616C8: rts
pub fn i_0616c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_AimAtPlayerB_0616ca  ($0616CA, 120 B, props/late_props_turrets_05e000.s)
/// $0616CA: move.w $72(a6), d0
pub fn i_0616ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0616CE: move.w $74(a6), d1
pub fn i_0616ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616D2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0616D2: sub.w $22(a6), d0
pub fn i_0616d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616D6;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0616D6: sub.w $24(a6), d1
pub fn i_0616d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616DA;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0616DA: asr.w #$1, d0
pub fn i_0616da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616DC;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0616DC: asr.w #$1, d1
pub fn i_0616dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616DE;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0616DE: jsr $5e018.l
pub fn i_0616de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x616E4);
    cpu.pc = 0x5E018;
    16
}
/// $0616E4: clr.w d1
pub fn i_0616e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616E6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0616E6: move.b $9c(a6), d1
pub fn i_0616e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616EA;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0616EA: andi.w #$1f, d1
pub fn i_0616ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616EE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0616EE: btst.b #$0, $3a(a6)
pub fn i_0616ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616F4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0616F4: beq.w $616fa
pub fn i_0616f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x616FA; return 10; }
    8
}
/// $0616F8: neg.w d1
pub fn i_0616f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616FA;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0616FA: add.w d1, d0
pub fn i_0616fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616FC;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0616FC: andi.w #$ff, d0
pub fn i_0616fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61700;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $061700: move.w d0, $34(a6)
pub fn i_061700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61704;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061704: move.w $72(a6), d2
pub fn i_061704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61708;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061708: move.w $74(a6), d3
pub fn i_061708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6170C;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06170C: move.w $22(a6), d0
pub fn i_06170c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61710;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061710: move.w $24(a6), d1
pub fn i_061710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61714;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061714: jsr $5e23a.l
pub fn i_061714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6171A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6171A);
    cpu.pc = 0x5E23A;
    16
}
/// $06171A: asl.w #$4, d0
pub fn i_06171a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6171C;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06171C: move.w d0, d1
pub fn i_06171c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6171E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06171E: move.w $34(a6), d0
pub fn i_06171e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61722;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061722: andi.w #$ff, d0
pub fn i_061722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61726;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $061726: jsr $13c0e.l
pub fn i_061726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6172C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6172C);
    cpu.pc = 0x13C0E;
    16
}
/// $06172C: sub.w $28(a6), d1
pub fn i_06172c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61730;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $061730: sub.w $2a(a6), d2
pub fn i_061730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61734;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $061734: asr.w #$4, d1
pub fn i_061734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61736;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $061736: asr.w #$4, d2
pub fn i_061736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61738;
    cpu.ir = 0xE842;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $061738: move.w d1, $2c(a6)
pub fn i_061738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6173C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06173C: move.w #$10, $2e(a6)
pub fn i_06173c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61742;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_061742  ($061742, 8 B, thunks/jsr_abs_thunks.c)
/// $061742: jsr $27cee.l
pub fn i_061742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61748);
    cpu.pc = 0x27CEE;
    16
}
/// $061748: rts
pub fn i_061748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6174A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_StepAngle_06174a  ($06174A, 30 B, props/late_props_turrets_05e000.s)
/// $06174A: move.w $72(a6), d2
pub fn i_06174a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6174E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06174E: move.w $74(a6), d3
pub fn i_06174e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61752;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061752: move.w $22(a6), d0
pub fn i_061752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61756;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061756: move.w $24(a6), d1
pub fn i_061756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6175A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06175A: jsr $5e23a.l
pub fn i_06175a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61760);
    cpu.pc = 0x5E23A;
    16
}
/// $061760: cmpi.w #$8, d0
pub fn i_061760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61764;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $061764: bgt.w $6176e
pub fn i_061764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61768;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6176E; return 10; }
    8
}
// ===== SetXN_061768  ($061768, 6 B, thunks/ccr_helpers.c)
/// $061768: ori.b #$11, ccr
pub fn i_061768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6176C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06176C: rts
pub fn i_06176c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6176E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06176e  ($06176E, 6 B, thunks/ccr_helpers.c)
/// $06176E: andi.b #$ee, ccr
pub fn i_06176e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61772;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $061772: rts
pub fn i_061772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61774;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_StepAngleB_061774  ($061774, 30 B, props/late_props_turrets_05e000.s)
/// $061774: move.w $72(a6), d2
pub fn i_061774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61778;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061778: move.w $74(a6), d3
pub fn i_061778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6177C;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06177C: move.w $22(a6), d0
pub fn i_06177c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61780;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061780: move.w $24(a6), d1
pub fn i_061780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61784;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061784: jsr $5e23a.l
pub fn i_061784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6178A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6178A);
    cpu.pc = 0x5E23A;
    16
}
/// $06178A: cmpi.w #$10, d0
pub fn i_06178a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6178E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10, Size::Word, false);
    8
}
/// $06178E: bgt.w $61798
pub fn i_06178e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61792;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x61798; return 10; }
    8
}
// ===== SetXN_061792  ($061792, 6 B, thunks/ccr_helpers.c)
/// $061792: ori.b #$11, ccr
pub fn i_061792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61796;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $061796: rts
pub fn i_061796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61798;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_061798  ($061798, 6 B, thunks/ccr_helpers.c)
/// $061798: andi.b #$ee, ccr
pub fn i_061798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6179C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06179C: rts
pub fn i_06179c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6179E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Acquire_06179e  ($06179E, 52 B, props/late_props_turrets_05e000.s)
/// $06179E: move.b $99(a6), d0
pub fn i_06179e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617A2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0617A2: andi.b #$f, d0
pub fn i_0617a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617A6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0617A6: move.w #$100, d1
pub fn i_0617a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0617AA: cmpi.b #$0, d0
pub fn i_0617aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617AE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0617AE: ble.w $617ba
pub fn i_0617ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617B2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x617BA; return 10; }
    8
}
/// $0617B2: addi.w #$10, d1
pub fn i_0617b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617B6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0617B6: subq.b #$1, d0
pub fn i_0617b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617B8;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0617B8: bra.b $617aa
pub fn i_0617b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617BA;
    cpu.ir = 0x60F0;
    cpu.pc = 0x617AA;
    10
}
/// $0617BA: move.w d1, $74(a6)
pub fn i_0617ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617BE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0617BE: jsr $5e0d4.l
pub fn i_0617be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x617C4);
    cpu.pc = 0x5E0D4;
    16
}
/// $0617C4: bcs.w $617d0
pub fn i_0617c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x617D0; return 10; }
    8
}
/// $0617C8: move.w $22(a0), d1
pub fn i_0617c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617CC;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0617CC: move.w d1, $72(a6)
pub fn i_0617cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617D0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0617D0: rts
pub fn i_0617d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Anim_0617d2  ($0617D2, 222 B, props/late_props_turrets_05e000.s)
/// $0617D2: move.w $28(a6), d0
pub fn i_0617d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617D6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0617D6: btst.b #$0, $3a(a6)
pub fn i_0617d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617DC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0617DC: beq.w $61848
pub fn i_0617dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61848; return 10; }
    8
}
/// $0617E0: cmpi.w #$ff40, d0
pub fn i_0617e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617E4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF40, Size::Word, false);
    8
}
/// $0617E4: bge.w $617f0
pub fn i_0617e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617E8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x617F0; return 10; }
    8
}
/// $0617E8: move.b #$6, $71(a6)
pub fn i_0617e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0617EE: rts
pub fn i_0617ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0617F0: cmpi.w #$ff80, d0
pub fn i_0617f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617F4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF80, Size::Word, false);
    8
}
/// $0617F4: bge.w $61800
pub fn i_0617f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617F8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61800; return 10; }
    8
}
/// $0617F8: move.b #$5, $71(a6)
pub fn i_0617f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x617FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0617FE: rts
pub fn i_0617fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61800;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061800: cmpi.w #$ffc0, d0
pub fn i_061800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61804;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFC0, Size::Word, false);
    8
}
/// $061804: bge.w $61810
pub fn i_061804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61808;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61810; return 10; }
    8
}
/// $061808: move.b #$4, $71(a6)
pub fn i_061808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6180E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06180E: rts
pub fn i_06180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61810;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061810: cmpi.w #$40, d0
pub fn i_061810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61814;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $061814: bge.w $61820
pub fn i_061814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61818;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61820; return 10; }
    8
}
/// $061818: move.b #$3, $71(a6)
pub fn i_061818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6181E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06181E: rts
pub fn i_06181e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61820;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061820: cmpi.w #$80, d0
pub fn i_061820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61824;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $061824: bge.w $61830
pub fn i_061824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61828;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61830; return 10; }
    8
}
/// $061828: move.b #$2, $71(a6)
pub fn i_061828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6182E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06182E: rts
pub fn i_06182e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61830;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061830: cmpi.w #$c0, d0
pub fn i_061830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61834;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $061834: bge.w $61840
pub fn i_061834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61838;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61840; return 10; }
    8
}
/// $061838: move.b #$1, $71(a6)
pub fn i_061838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6183E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06183E: rts
pub fn i_06183e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061840: move.b #$0, $71(a6)
pub fn i_061840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61846;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061846: rts
pub fn i_061846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61848;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061848: cmpi.w #$ff40, d0
pub fn i_061848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6184C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF40, Size::Word, false);
    8
}
/// $06184C: bge.w $61858
pub fn i_06184c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61850;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61858; return 10; }
    8
}
/// $061850: move.b #$0, $71(a6)
pub fn i_061850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61856;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061856: rts
pub fn i_061856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61858;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061858: cmpi.w #$ff80, d0
pub fn i_061858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6185C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF80, Size::Word, false);
    8
}
/// $06185C: bge.w $61868
pub fn i_06185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61860;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61868; return 10; }
    8
}
/// $061860: move.b #$1, $71(a6)
pub fn i_061860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61866;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061866: rts
pub fn i_061866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61868;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061868: cmpi.w #$ffc0, d0
pub fn i_061868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6186C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFC0, Size::Word, false);
    8
}
/// $06186C: bge.w $61878
pub fn i_06186c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61870;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61878; return 10; }
    8
}
/// $061870: move.b #$2, $71(a6)
pub fn i_061870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61876;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061876: rts
pub fn i_061876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61878;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061878: cmpi.w #$40, d0
pub fn i_061878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6187C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $06187C: bge.w $61888
pub fn i_06187c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61880;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61888; return 10; }
    8
}
/// $061880: move.b #$3, $71(a6)
pub fn i_061880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61886;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061886: rts
pub fn i_061886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61888;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061888: cmpi.w #$80, d0
pub fn i_061888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6188C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $06188C: bge.w $61898
pub fn i_06188c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61890;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x61898; return 10; }
    8
}
/// $061890: move.b #$4, $71(a6)
pub fn i_061890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61896;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061896: rts
pub fn i_061896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61898;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $061898: cmpi.w #$c0, d0
pub fn i_061898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6189C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $06189C: bge.w $618a8
pub fn i_06189c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618A0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x618A8; return 10; }
    8
}
/// $0618A0: move.b #$5, $71(a6)
pub fn i_0618a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0618A6: rts
pub fn i_0618a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0618A8: move.b #$6, $71(a6)
pub fn i_0618a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0618AE: rts
pub fn i_0618ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_RngDifficulty_0618b0  ($0618B0, 90 B, props/late_props_turrets_05e000.s)
/// $0618B0: cmpi.w #$0, $78(a6)
pub fn i_0618b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618B6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0618B6: bgt.w $6194c
pub fn i_0618b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618BA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6194C; return 10; }
    8
}
/// $0618BA: cmpi.b #$ff, $7b(a6)
pub fn i_0618ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618C0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0618C0: bne.w $618e4
pub fn i_0618c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x618E4; return 10; }
    8
}
/// $0618C4: lea.l $2bde90.l, a0
pub fn i_0618c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDE90;
    20
}
/// $0618CA: jsr $799de.l
pub fn i_0618ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x618D0);
    cpu.pc = 0x799DE;
    16
}
/// $0618D0: move.b d0, $7a(a6)
pub fn i_0618d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618D4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0618D4: lea.l $2bdf12.l, a0
pub fn i_0618d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDF12;
    20
}
/// $0618DA: jsr $799de.l
pub fn i_0618da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x618E0);
    cpu.pc = 0x799DE;
    16
}
/// $0618E0: move.b d0, $7b(a6)
pub fn i_0618e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618E4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0618E4: cmpi.b #$0, $7a(a6)
pub fn i_0618e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618EA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0618EA: bgt.w $61946
pub fn i_0618ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618EE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x61946; return 10; }
    8
}
/// $0618EE: cmpi.b #$0, $7b(a6)
pub fn i_0618ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618F4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0618F4: bne.w $61910
pub fn i_0618f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x61910; return 10; }
    8
}
/// $0618F8: move.b #$ff, $7b(a6)
pub fn i_0618f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x618FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0618FE: lea.l $2bdd8c.l, a0
pub fn i_0618fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61904;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDD8C;
    20
}
/// $061904: jsr $799de.l
pub fn i_061904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6190A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6190A);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_06190a  ($06190A, 6 B, kernel/task_setters_w.c)
/// $06190A: move.w d0, $78(a6)
pub fn i_06190a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6190E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06190E: rts
pub fn i_06190e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61910;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_SpawnPair_061910  ($061910, 54 B, props/late_props_turrets_05e000.s)
/// $061910: lea.l $61344(pc), a1
pub fn i_061910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61914;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x61344;
    16
}
/// $061914: jsr $4ae.l
pub fn i_061914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6191A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6191A);
    cpu.pc = 0x4AE;
    16
}
/// $06191A: jsr $5dd02.l
pub fn i_06191a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61920;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61920);
    cpu.pc = 0x5DD02;
    16
}
/// $061920: lea.l $612e6(pc), a1
pub fn i_061920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61924;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x612E6;
    16
}
/// $061924: jsr $4ae.l
pub fn i_061924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6192A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6192A);
    cpu.pc = 0x4AE;
    16
}
/// $06192A: jsr $5dd02.l
pub fn i_06192a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61930);
    cpu.pc = 0x5DD02;
    16
}
/// $061930: lea.l $2bde90.l, a0
pub fn i_061930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61936;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDE90;
    20
}
/// $061936: jsr $799de.l
pub fn i_061936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6193C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6193C);
    cpu.pc = 0x799DE;
    16
}
/// $06193C: move.b d0, $7a(a6)
pub fn i_06193c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61940;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $061940: subq.b #$1, $7b(a6)
pub fn i_061940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61944;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $061944: rts
pub fn i_061944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61946;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Rts_061946  ($061946, 6 B, props/late_props_turrets_05e000.s)
/// $061946: subq.b #$1, $7a(a6)
pub fn i_061946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6194A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06194A: rts
pub fn i_06194a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6194C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_RtsB_06194c  ($06194C, 6 B, props/late_props_turrets_05e000.s)
/// $06194C: subq.w #$1, $78(a6)
pub fn i_06194c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61950;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $061950: rts
pub fn i_061950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61952;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_ScrollGate_061952  ($061952, 36 B, props/late_props_turrets_05e000.s)
/// $061952: clr.w d0
pub fn i_061952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61954;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $061954: move.b $9a(a6), d0
pub fn i_061954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61958;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061958: asl.w #$6, d0
pub fn i_061958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6195A;
    cpu.ir = 0xED40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $06195A: andi.w #$7fff, d0
pub fn i_06195a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6195E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7FFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06195E: move.w d0, $80(a6)
pub fn i_06195e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61962;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061962: clr.w d0
pub fn i_061962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61964;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $061964: move.b $9b(a6), d0
pub fn i_061964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61968;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $061968: asl.w #$4, d0
pub fn i_061968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6196A;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06196A: move.w d0, $82(a6)
pub fn i_06196a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6196E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06196E: move.l $106f50.l, d0
pub fn i_06196e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61974;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $061974: swap d0
pub fn i_061974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61976;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== SetTaskW_061976  ($061976, 6 B, kernel/task_setters_w.c)
/// $061976: move.w d0, $84(a6)
pub fn i_061976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6197A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06197A: rts
pub fn i_06197a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6197C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_ScrollGateB_06197c  ($06197C, 38 B, props/late_props_turrets_05e000.s)
/// $06197C: subi.w #$2, $80(a6)
pub fn i_06197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61982;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $061982: cmpi.w #$0, $80(a6)
pub fn i_061982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61988;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $061988: ble.w $619a8
pub fn i_061988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6198C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x619A8; return 10; }
    8
}
/// $06198C: move.l $106f50.l, d0
pub fn i_06198c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61992;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $061992: swap d0
pub fn i_061992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61994;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $061994: move.w $84(a6), d1
pub fn i_061994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61998;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $061998: sub.w d1, d0
pub fn i_061998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6199A;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06199A: cmp.w $82(a6), d0
pub fn i_06199a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6199E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06199E: bgt.w $619a8
pub fn i_06199e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619A2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x619A8; return 10; }
    8
}
// ===== ClearXN_0619a2  ($0619A2, 6 B, thunks/ccr_helpers.c)
/// $0619A2: andi.b #$ee, ccr
pub fn i_0619a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619A6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0619A6: rts
pub fn i_0619a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0619a8  ($0619A8, 6 B, thunks/ccr_helpers.c)
/// $0619A8: ori.b #$11, ccr
pub fn i_0619a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619AC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0619AC: rts
pub fn i_0619ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimTurret_Stub_0619ae  ($0619AE, 16 B, props/late_props_turrets_05e000.s)
/// $0619AE: movea.l $8(a6), a1
pub fn i_0619ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619B2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0619B2: move.b $10(a6), d0
pub fn i_0619b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619B6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0619B6: cmp.b $10(a1), d0
pub fn i_0619b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619BA;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0619BA: bcs.w $619c4
pub fn i_0619ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x619C4; return 10; }
    8
}
// ===== ClearXN_0619be  ($0619BE, 6 B, thunks/ccr_helpers.c)
/// $0619BE: andi.b #$ee, ccr
pub fn i_0619be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619C2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0619C2: rts
pub fn i_0619c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0619c4  ($0619C4, 6 B, thunks/ccr_helpers.c)
/// $0619C4: ori.b #$11, ccr
pub fn i_0619c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619C8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0619C8: rts
pub fn i_0619c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GroundNest_Tmpl32_0619ca  ($0619CA, 216 B, props/late_props_turrets_05e000.s)
/// $0619CA: clr.b $83(a6)
pub fn i_0619ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619CE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x83);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0619CE: clr.b $84(a6)
pub fn i_0619ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619D2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0619D2: bra.w $619ee
pub fn i_0619d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x619EE;
    10
}
/// $0619EE: jsr $5e7c0.l
pub fn i_0619ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x619F4);
    cpu.pc = 0x5E7C0;
    16
}
/// $0619F4: jsr $267e2.l
pub fn i_0619f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x619FA);
    cpu.pc = 0x267E2;
    16
}
/// $0619FA: move.w #$f2, d1
pub fn i_0619fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x619FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0619FE: tst.b $9e(a6)
pub fn i_0619fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A02;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $061A02: beq.w $61a0a
pub fn i_061a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A06;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x61A0A; return 10; }
    8
}
/// $061A06: move.w #$1cc, d1
pub fn i_061a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A0A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061A0A: jsr $236e.l
pub fn i_061a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A10);
    cpu.pc = 0x236E;
    16
}
/// $061A10: move.w #$2, $1c(a6)
pub fn i_061a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A16;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061A16: jsr $138fe.l
pub fn i_061a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A1C);
    cpu.pc = 0x138FE;
    16
}
/// $061A1C: move.w #$8000, d0
pub fn i_061a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $061A20: jsr $28134.l
pub fn i_061a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A26);
    cpu.pc = 0x28134;
    16
}
/// $061A26: andi.w #$ffe3, $38(a6)
pub fn i_061a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A2C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $061A2C: ori.w #$8, $38(a6)
pub fn i_061a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A32;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $061A32: lea.l $2b7a22.l, a0
pub fn i_061a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7A22;
    20
}
/// $061A38: jsr $799de.l
pub fn i_061a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A3E);
    cpu.pc = 0x799DE;
    16
}
/// $061A3E: move.w d0, $66(a6)
pub fn i_061a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061A42: lea.l $2b7b26.l, a0
pub fn i_061a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7B26;
    20
}
/// $061A48: jsr $799de.l
pub fn i_061a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A4E);
    cpu.pc = 0x799DE;
    16
}
/// $061A4E: move.w d0, $74(a6)
pub fn i_061a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A52;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061A52: move.l #$2c3458, $48(a6)
pub fn i_061a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A5A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C3458;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $061A5A: move.l #$2c3704, $60(a6)
pub fn i_061a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A62;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C3704;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $061A62: move.w #$a0, $72(a6)
pub fn i_061a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $061A68: clr.b $77(a6)
pub fn i_061a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A6C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $061A6C: lea.l $723d2.l, a1
pub fn i_061a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A72;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723D2;
    20
}
/// $061A72: jsr $4ae.l
pub fn i_061a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A78);
    cpu.pc = 0x4AE;
    16
}
/// $061A78: jsr $5dd02.l
pub fn i_061a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A7E);
    cpu.pc = 0x5DD02;
    16
}
/// $061A7E: clr.w $98(a0)
pub fn i_061a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A82;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $061A82: lea.l $7773e.l, a1
pub fn i_061a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A88;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7773E;
    20
}
/// $061A88: jsr $4ae.l
pub fn i_061a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A8E);
    cpu.pc = 0x4AE;
    16
}
/// $061A8E: jsr $5dd02.l
pub fn i_061a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x61A94);
    cpu.pc = 0x5DD02;
    16
}
/// $061A94: bset.b #$2, $6b(a6)
pub fn i_061a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A9A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $061A9A: move.l #$2c3554, $4c(a6)
pub fn i_061a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AA2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C3554;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== LateProp_Init_061aa2  ($061AA2, 200 B, props/late_props_turrets_05e000.s)
/// $061AA2: tst.b $98(a6)
pub fn i_061aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61AA6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x060000, i_060000),
    (0x060006, i_060006),
    (0x060008, i_060008),
    (0x06000E, i_06000e),
    (0x060014, i_060014),
    (0x060018, i_060018),
    (0x06001A, i_06001a),
    (0x060020, i_060020),
    (0x060026, i_060026),
    (0x06002A, i_06002a),
    (0x06002E, i_06002e),
    (0x060030, i_060030),
    (0x060036, i_060036),
    (0x06003A, i_06003a),
    (0x06003E, i_06003e),
    (0x060040, i_060040),
    (0x060044, i_060044),
    (0x060048, i_060048),
    (0x06004E, i_06004e),
    (0x060050, i_060050),
    (0x060056, i_060056),
    (0x06005A, i_06005a),
    (0x060060, i_060060),
    (0x060062, i_060062),
    (0x060066, i_060066),
    (0x06006C, i_06006c),
    (0x060072, i_060072),
    (0x060078, i_060078),
    (0x06007C, i_06007c),
    (0x060080, i_060080),
    (0x060084, i_060084),
    (0x06008A, i_06008a),
    (0x060090, i_060090),
    (0x060096, i_060096),
    (0x06009C, i_06009c),
    (0x0600A2, i_0600a2),
    (0x0600A6, i_0600a6),
    (0x0600A8, i_0600a8),
    (0x0600AE, i_0600ae),
    (0x0600B4, i_0600b4),
    (0x0600BA, i_0600ba),
    (0x0600BE, i_0600be),
    (0x0600C2, i_0600c2),
    (0x0600C6, i_0600c6),
    (0x0600CA, i_0600ca),
    (0x0600CE, i_0600ce),
    (0x0600D2, i_0600d2),
    (0x0600D4, i_0600d4),
    (0x0600DA, i_0600da),
    (0x0600DE, i_0600de),
    (0x0600E2, i_0600e2),
    (0x0600E4, i_0600e4),
    (0x0600E8, i_0600e8),
    (0x0600EC, i_0600ec),
    (0x0600F2, i_0600f2),
    (0x0600F4, i_0600f4),
    (0x0600FA, i_0600fa),
    (0x060100, i_060100),
    (0x060104, i_060104),
    (0x060106, i_060106),
    (0x06010C, i_06010c),
    (0x060112, i_060112),
    (0x060116, i_060116),
    (0x06011A, i_06011a),
    (0x06011C, i_06011c),
    (0x060122, i_060122),
    (0x060126, i_060126),
    (0x06012A, i_06012a),
    (0x06012C, i_06012c),
    (0x060130, i_060130),
    (0x060134, i_060134),
    (0x06013A, i_06013a),
    (0x06013C, i_06013c),
    (0x060142, i_060142),
    (0x060148, i_060148),
    (0x060178, i_060178),
    (0x06017E, i_06017e),
    (0x060182, i_060182),
    (0x060188, i_060188),
    (0x06018A, i_06018a),
    (0x06018E, i_06018e),
    (0x060194, i_060194),
    (0x06019A, i_06019a),
    (0x0601A0, i_0601a0),
    (0x0601A4, i_0601a4),
    (0x0601A8, i_0601a8),
    (0x0601AC, i_0601ac),
    (0x0601B2, i_0601b2),
    (0x0601B8, i_0601b8),
    (0x0601BE, i_0601be),
    (0x0601C2, i_0601c2),
    (0x0601C4, i_0601c4),
    (0x0601CA, i_0601ca),
    (0x0601D0, i_0601d0),
    (0x0601D6, i_0601d6),
    (0x0601DA, i_0601da),
    (0x0601DE, i_0601de),
    (0x0601E2, i_0601e2),
    (0x0601E6, i_0601e6),
    (0x0601EA, i_0601ea),
    (0x0601EE, i_0601ee),
    (0x0601F0, i_0601f0),
    (0x0601F6, i_0601f6),
    (0x0601FA, i_0601fa),
    (0x0601FE, i_0601fe),
    (0x060200, i_060200),
    (0x060204, i_060204),
    (0x060208, i_060208),
    (0x06020E, i_06020e),
    (0x060210, i_060210),
    (0x060216, i_060216),
    (0x06021A, i_06021a),
    (0x060220, i_060220),
    (0x060226, i_060226),
    (0x060228, i_060228),
    (0x06022C, i_06022c),
    (0x060232, i_060232),
    (0x060236, i_060236),
    (0x06023C, i_06023c),
    (0x060240, i_060240),
    (0x060242, i_060242),
    (0x060248, i_060248),
    (0x06024E, i_06024e),
    (0x060252, i_060252),
    (0x060256, i_060256),
    (0x06025C, i_06025c),
    (0x06025E, i_06025e),
    (0x060262, i_060262),
    (0x060268, i_060268),
    (0x06026C, i_06026c),
    (0x060272, i_060272),
    (0x060276, i_060276),
    (0x06027C, i_06027c),
    (0x060282, i_060282),
    (0x060288, i_060288),
    (0x06028E, i_06028e),
    (0x060292, i_060292),
    (0x060296, i_060296),
    (0x06029C, i_06029c),
    (0x0602A0, i_0602a0),
    (0x0602A6, i_0602a6),
    (0x0602AC, i_0602ac),
    (0x0602B2, i_0602b2),
    (0x0602B8, i_0602b8),
    (0x0602BC, i_0602bc),
    (0x0602BE, i_0602be),
    (0x0602C4, i_0602c4),
    (0x0602C8, i_0602c8),
    (0x0602CC, i_0602cc),
    (0x0602D0, i_0602d0),
    (0x0602D4, i_0602d4),
    (0x0602D6, i_0602d6),
    (0x0602D8, i_0602d8),
    (0x0602DE, i_0602de),
    (0x0602E0, i_0602e0),
    (0x0602E4, i_0602e4),
    (0x0602EA, i_0602ea),
    (0x0602F0, i_0602f0),
    (0x0602F6, i_0602f6),
    (0x0602FA, i_0602fa),
    (0x0602FE, i_0602fe),
    (0x060302, i_060302),
    (0x060308, i_060308),
    (0x06030E, i_06030e),
    (0x060314, i_060314),
    (0x06031A, i_06031a),
    (0x060320, i_060320),
    (0x060322, i_060322),
    (0x060326, i_060326),
    (0x06032C, i_06032c),
    (0x06032E, i_06032e),
    (0x060334, i_060334),
    (0x06033A, i_06033a),
    (0x060340, i_060340),
    (0x060344, i_060344),
    (0x06034A, i_06034a),
    (0x06034C, i_06034c),
    (0x060350, i_060350),
    (0x060356, i_060356),
    (0x06035C, i_06035c),
    (0x060362, i_060362),
    (0x060368, i_060368),
    (0x06036E, i_06036e),
    (0x060374, i_060374),
    (0x06037A, i_06037a),
    (0x06037C, i_06037c),
    (0x060382, i_060382),
    (0x060388, i_060388),
    (0x06038C, i_06038c),
    (0x060390, i_060390),
    (0x060392, i_060392),
    (0x060396, i_060396),
    (0x060398, i_060398),
    (0x06039E, i_06039e),
    (0x0603A4, i_0603a4),
    (0x0603A8, i_0603a8),
    (0x0603AC, i_0603ac),
    (0x0603AE, i_0603ae),
    (0x0603B2, i_0603b2),
    (0x0603B4, i_0603b4),
    (0x0603B6, i_0603b6),
    (0x0603B8, i_0603b8),
    (0x0603BC, i_0603bc),
    (0x0603C0, i_0603c0),
    (0x0603C2, i_0603c2),
    (0x0603C4, i_0603c4),
    (0x0603C8, i_0603c8),
    (0x0603CC, i_0603cc),
    (0x0603D2, i_0603d2),
    (0x0603D8, i_0603d8),
    (0x0603DE, i_0603de),
    (0x0603E2, i_0603e2),
    (0x0603E4, i_0603e4),
    (0x0603E6, i_0603e6),
    (0x0603EC, i_0603ec),
    (0x0603F0, i_0603f0),
    (0x0603F6, i_0603f6),
    (0x0603FC, i_0603fc),
    (0x060402, i_060402),
    (0x060404, i_060404),
    (0x060408, i_060408),
    (0x06040C, i_06040c),
    (0x060410, i_060410),
    (0x060414, i_060414),
    (0x060418, i_060418),
    (0x06041A, i_06041a),
    (0x06041E, i_06041e),
    (0x060420, i_060420),
    (0x060426, i_060426),
    (0x06042C, i_06042c),
    (0x060432, i_060432),
    (0x060438, i_060438),
    (0x06043C, i_06043c),
    (0x060440, i_060440),
    (0x060446, i_060446),
    (0x06044C, i_06044c),
    (0x060452, i_060452),
    (0x060458, i_060458),
    (0x06045C, i_06045c),
    (0x060460, i_060460),
    (0x060466, i_060466),
    (0x06046C, i_06046c),
    (0x060472, i_060472),
    (0x060478, i_060478),
    (0x06047C, i_06047c),
    (0x0604BC, i_0604bc),
    (0x0604C0, i_0604c0),
    (0x0604C6, i_0604c6),
    (0x0604CC, i_0604cc),
    (0x0604D2, i_0604d2),
    (0x0604D8, i_0604d8),
    (0x0604DE, i_0604de),
    (0x0604E2, i_0604e2),
    (0x0604E4, i_0604e4),
    (0x0604EA, i_0604ea),
    (0x0604F0, i_0604f0),
    (0x0604F4, i_0604f4),
    (0x0604F8, i_0604f8),
    (0x0604FE, i_0604fe),
    (0x060504, i_060504),
    (0x06050A, i_06050a),
    (0x060510, i_060510),
    (0x060514, i_060514),
    (0x060516, i_060516),
    (0x06051A, i_06051a),
    (0x06051E, i_06051e),
    (0x060522, i_060522),
    (0x060528, i_060528),
    (0x06052A, i_06052a),
    (0x060530, i_060530),
    (0x060536, i_060536),
    (0x06053C, i_06053c),
    (0x06053E, i_06053e),
    (0x060544, i_060544),
    (0x06054A, i_06054a),
    (0x06054E, i_06054e),
    (0x060552, i_060552),
    (0x060554, i_060554),
    (0x060558, i_060558),
    (0x06055A, i_06055a),
    (0x06055E, i_06055e),
    (0x060562, i_060562),
    (0x060566, i_060566),
    (0x06056A, i_06056a),
    (0x06056E, i_06056e),
    (0x060570, i_060570),
    (0x060574, i_060574),
    (0x060576, i_060576),
    (0x06057A, i_06057a),
    (0x06057E, i_06057e),
    (0x060582, i_060582),
    (0x060586, i_060586),
    (0x06063A, i_06063a),
    (0x060640, i_060640),
    (0x060646, i_060646),
    (0x06064A, i_06064a),
    (0x060650, i_060650),
    (0x060658, i_060658),
    (0x06065C, i_06065c),
    (0x060662, i_060662),
    (0x060668, i_060668),
    (0x06066C, i_06066c),
    (0x060672, i_060672),
    (0x06067A, i_06067a),
    (0x06067E, i_06067e),
    (0x060684, i_060684),
    (0x06068A, i_06068a),
    (0x06068E, i_06068e),
    (0x060694, i_060694),
    (0x06069C, i_06069c),
    (0x0606A0, i_0606a0),
    (0x0606A6, i_0606a6),
    (0x0606AC, i_0606ac),
    (0x0606B0, i_0606b0),
    (0x0606B6, i_0606b6),
    (0x0606BE, i_0606be),
    (0x0606C2, i_0606c2),
    (0x0606C8, i_0606c8),
    (0x0606CE, i_0606ce),
    (0x0606D2, i_0606d2),
    (0x0606D8, i_0606d8),
    (0x0606E0, i_0606e0),
    (0x0606E4, i_0606e4),
    (0x0606E6, i_0606e6),
    (0x0606EA, i_0606ea),
    (0x0606EC, i_0606ec),
    (0x0606EE, i_0606ee),
    (0x0606F4, i_0606f4),
    (0x0606FA, i_0606fa),
    (0x0606FE, i_0606fe),
    (0x060706, i_060706),
    (0x06070A, i_06070a),
    (0x060710, i_060710),
    (0x060714, i_060714),
    (0x06071A, i_06071a),
    (0x06071C, i_06071c),
    (0x060720, i_060720),
    (0x060726, i_060726),
    (0x06072A, i_06072a),
    (0x060730, i_060730),
    (0x060736, i_060736),
    (0x06073C, i_06073c),
    (0x060742, i_060742),
    (0x060746, i_060746),
    (0x06074C, i_06074c),
    (0x060752, i_060752),
    (0x060756, i_060756),
    (0x06075C, i_06075c),
    (0x060762, i_060762),
    (0x060764, i_060764),
    (0x06076A, i_06076a),
    (0x06077C, i_06077c),
    (0x060780, i_060780),
    (0x060786, i_060786),
    (0x06078C, i_06078c),
    (0x060792, i_060792),
    (0x060796, i_060796),
    (0x060798, i_060798),
    (0x06079E, i_06079e),
    (0x0607A4, i_0607a4),
    (0x0607AA, i_0607aa),
    (0x0607AE, i_0607ae),
    (0x0607B2, i_0607b2),
    (0x0607B4, i_0607b4),
    (0x0607BA, i_0607ba),
    (0x0607C0, i_0607c0),
    (0x0607C4, i_0607c4),
    (0x0607C8, i_0607c8),
    (0x0607CA, i_0607ca),
    (0x0607CE, i_0607ce),
    (0x0607D0, i_0607d0),
    (0x0607D6, i_0607d6),
    (0x0607DC, i_0607dc),
    (0x0607E0, i_0607e0),
    (0x0607E2, i_0607e2),
    (0x0607E8, i_0607e8),
    (0x0607EE, i_0607ee),
    (0x0607F2, i_0607f2),
    (0x0607F6, i_0607f6),
    (0x0607F8, i_0607f8),
    (0x0607FE, i_0607fe),
    (0x060804, i_060804),
    (0x060808, i_060808),
    (0x06080C, i_06080c),
    (0x06080E, i_06080e),
    (0x060812, i_060812),
    (0x060814, i_060814),
    (0x06081A, i_06081a),
    (0x06081E, i_06081e),
    (0x060822, i_060822),
    (0x060828, i_060828),
    (0x06082C, i_06082c),
    (0x060830, i_060830),
    (0x060834, i_060834),
    (0x06083A, i_06083a),
    (0x06083E, i_06083e),
    (0x060842, i_060842),
    (0x060846, i_060846),
    (0x06084C, i_06084c),
    (0x060852, i_060852),
    (0x060858, i_060858),
    (0x06085E, i_06085e),
    (0x060864, i_060864),
    (0x060868, i_060868),
    (0x06086A, i_06086a),
    (0x060870, i_060870),
    (0x060876, i_060876),
    (0x06087A, i_06087a),
    (0x06087E, i_06087e),
    (0x060880, i_060880),
    (0x060884, i_060884),
    (0x060886, i_060886),
    (0x06088C, i_06088c),
    (0x060892, i_060892),
    (0x060898, i_060898),
    (0x06089E, i_06089e),
    (0x0608A2, i_0608a2),
    (0x0608A6, i_0608a6),
    (0x0608A8, i_0608a8),
    (0x0608AE, i_0608ae),
    (0x0608B4, i_0608b4),
    (0x0608BA, i_0608ba),
    (0x0608BE, i_0608be),
    (0x0608C2, i_0608c2),
    (0x0608C4, i_0608c4),
    (0x0608CA, i_0608ca),
    (0x0608D0, i_0608d0),
    (0x0608D4, i_0608d4),
    (0x0608D8, i_0608d8),
    (0x0608DA, i_0608da),
    (0x0608DE, i_0608de),
    (0x0608E0, i_0608e0),
    (0x0608E6, i_0608e6),
    (0x0608EC, i_0608ec),
    (0x0608F0, i_0608f0),
    (0x0608F2, i_0608f2),
    (0x0608F8, i_0608f8),
    (0x0608FE, i_0608fe),
    (0x060902, i_060902),
    (0x060906, i_060906),
    (0x060908, i_060908),
    (0x06090E, i_06090e),
    (0x060914, i_060914),
    (0x060918, i_060918),
    (0x06091C, i_06091c),
    (0x06091E, i_06091e),
    (0x060922, i_060922),
    (0x060924, i_060924),
    (0x06092A, i_06092a),
    (0x060930, i_060930),
    (0x060936, i_060936),
    (0x06093C, i_06093c),
    (0x060940, i_060940),
    (0x060944, i_060944),
    (0x06094A, i_06094a),
    (0x06094E, i_06094e),
    (0x060952, i_060952),
    (0x060956, i_060956),
    (0x06095C, i_06095c),
    (0x060960, i_060960),
    (0x060964, i_060964),
    (0x060968, i_060968),
    (0x06096E, i_06096e),
    (0x060974, i_060974),
    (0x06097A, i_06097a),
    (0x06097E, i_06097e),
    (0x060980, i_060980),
    (0x060986, i_060986),
    (0x06098C, i_06098c),
    (0x060990, i_060990),
    (0x060994, i_060994),
    (0x060996, i_060996),
    (0x06099A, i_06099a),
    (0x06099C, i_06099c),
    (0x0609A0, i_0609a0),
    (0x0609A4, i_0609a4),
    (0x0609AA, i_0609aa),
    (0x0609AE, i_0609ae),
    (0x0609B0, i_0609b0),
    (0x0609B6, i_0609b6),
    (0x0609BC, i_0609bc),
    (0x0609C0, i_0609c0),
    (0x0609C2, i_0609c2),
    (0x0609C6, i_0609c6),
    (0x0609CC, i_0609cc),
    (0x0609D2, i_0609d2),
    (0x0609D4, i_0609d4),
    (0x0609D8, i_0609d8),
    (0x0609DE, i_0609de),
    (0x0609E4, i_0609e4),
    (0x0609EA, i_0609ea),
    (0x0609F0, i_0609f0),
    (0x0609F4, i_0609f4),
    (0x0609F6, i_0609f6),
    (0x0609FC, i_0609fc),
    (0x060A02, i_060a02),
    (0x060A06, i_060a06),
    (0x060A0C, i_060a0c),
    (0x060A0E, i_060a0e),
    (0x060A12, i_060a12),
    (0x060A18, i_060a18),
    (0x060A1E, i_060a1e),
    (0x060A24, i_060a24),
    (0x060A2A, i_060a2a),
    (0x060A2E, i_060a2e),
    (0x060A30, i_060a30),
    (0x060A36, i_060a36),
    (0x060A3C, i_060a3c),
    (0x060A42, i_060a42),
    (0x060A48, i_060a48),
    (0x060A4C, i_060a4c),
    (0x060A52, i_060a52),
    (0x060A64, i_060a64),
    (0x060A68, i_060a68),
    (0x060A6E, i_060a6e),
    (0x060A74, i_060a74),
    (0x060A7A, i_060a7a),
    (0x060A7E, i_060a7e),
    (0x060A84, i_060a84),
    (0x060A88, i_060a88),
    (0x060A8E, i_060a8e),
    (0x060A92, i_060a92),
    (0x060A98, i_060a98),
    (0x060A9E, i_060a9e),
    (0x060AA4, i_060aa4),
    (0x060AAA, i_060aaa),
    (0x060AAE, i_060aae),
    (0x060AB2, i_060ab2),
    (0x060AB8, i_060ab8),
    (0x060ABE, i_060abe),
    (0x060AC4, i_060ac4),
    (0x060ACA, i_060aca),
    (0x060AD0, i_060ad0),
    (0x060AD4, i_060ad4),
    (0x060AD6, i_060ad6),
    (0x060ADC, i_060adc),
    (0x060AE2, i_060ae2),
    (0x060AE6, i_060ae6),
    (0x060AEA, i_060aea),
    (0x060AEC, i_060aec),
    (0x060AF0, i_060af0),
    (0x060AF6, i_060af6),
    (0x060AFA, i_060afa),
    (0x060B00, i_060b00),
    (0x060B06, i_060b06),
    (0x060B0C, i_060b0c),
    (0x060B10, i_060b10),
    (0x060B12, i_060b12),
    (0x060B18, i_060b18),
    (0x060B1E, i_060b1e),
    (0x060B22, i_060b22),
    (0x060B28, i_060b28),
    (0x060B2E, i_060b2e),
    (0x060B32, i_060b32),
    (0x060B38, i_060b38),
    (0x060B3E, i_060b3e),
    (0x060B40, i_060b40),
    (0x060B46, i_060b46),
    (0x060B4C, i_060b4c),
    (0x060B50, i_060b50),
    (0x060B52, i_060b52),
    (0x060B58, i_060b58),
    (0x060B5E, i_060b5e),
    (0x060B62, i_060b62),
    (0x060B68, i_060b68),
    (0x060B6E, i_060b6e),
    (0x060B72, i_060b72),
    (0x060B7A, i_060b7a),
    (0x060B80, i_060b80),
    (0x060B88, i_060b88),
    (0x060B8A, i_060b8a),
    (0x060B8E, i_060b8e),
    (0x060B92, i_060b92),
    (0x060B96, i_060b96),
    (0x060B98, i_060b98),
    (0x060B9A, i_060b9a),
    (0x060B9E, i_060b9e),
    (0x060BA2, i_060ba2),
    (0x060BA8, i_060ba8),
    (0x060BAE, i_060bae),
    (0x060BB4, i_060bb4),
    (0x060BB8, i_060bb8),
    (0x060BBC, i_060bbc),
    (0x060BC2, i_060bc2),
    (0x060BC6, i_060bc6),
    (0x060BCC, i_060bcc),
    (0x060BD2, i_060bd2),
    (0x060BD8, i_060bd8),
    (0x060BDC, i_060bdc),
    (0x060BE2, i_060be2),
    (0x060BE8, i_060be8),
    (0x060BEE, i_060bee),
    (0x060BF2, i_060bf2),
    (0x060BF4, i_060bf4),
    (0x060BF6, i_060bf6),
    (0x060BFC, i_060bfc),
    (0x060C02, i_060c02),
    (0x060C06, i_060c06),
    (0x060C0A, i_060c0a),
    (0x060C0E, i_060c0e),
    (0x060C14, i_060c14),
    (0x060C18, i_060c18),
    (0x060C1E, i_060c1e),
    (0x060C22, i_060c22),
    (0x060C26, i_060c26),
    (0x060C2A, i_060c2a),
    (0x060C30, i_060c30),
    (0x060C36, i_060c36),
    (0x060C3C, i_060c3c),
    (0x060C40, i_060c40),
    (0x060C46, i_060c46),
    (0x060C4C, i_060c4c),
    (0x060C50, i_060c50),
    (0x060C56, i_060c56),
    (0x060C5A, i_060c5a),
    (0x060C60, i_060c60),
    (0x060C66, i_060c66),
    (0x060C6A, i_060c6a),
    (0x060C6E, i_060c6e),
    (0x060C72, i_060c72),
    (0x060C76, i_060c76),
    (0x060C7C, i_060c7c),
    (0x060C82, i_060c82),
    (0x060C86, i_060c86),
    (0x060C8C, i_060c8c),
    (0x060C92, i_060c92),
    (0x060C96, i_060c96),
    (0x060C98, i_060c98),
    (0x060C9A, i_060c9a),
    (0x060C9E, i_060c9e),
    (0x060CA2, i_060ca2),
    (0x060CA8, i_060ca8),
    (0x060CAC, i_060cac),
    (0x060CB2, i_060cb2),
    (0x060CB8, i_060cb8),
    (0x060CBC, i_060cbc),
    (0x060CC0, i_060cc0),
    (0x060CC4, i_060cc4),
    (0x060CC8, i_060cc8),
    (0x060CCC, i_060ccc),
    (0x060CCE, i_060cce),
    (0x060CD2, i_060cd2),
    (0x060CD4, i_060cd4),
    (0x060CDA, i_060cda),
    (0x060CEE, i_060cee),
    (0x060CF2, i_060cf2),
    (0x060CF8, i_060cf8),
    (0x060CFA, i_060cfa),
    (0x060CFE, i_060cfe),
    (0x060D02, i_060d02),
    (0x060D08, i_060d08),
    (0x060D0C, i_060d0c),
    (0x060D0E, i_060d0e),
    (0x060D10, i_060d10),
    (0x060D14, i_060d14),
    (0x060D16, i_060d16),
    (0x060D1C, i_060d1c),
    (0x060D22, i_060d22),
    (0x060D26, i_060d26),
    (0x060D2C, i_060d2c),
    (0x060D32, i_060d32),
    (0x060D36, i_060d36),
    (0x060D38, i_060d38),
    (0x060D3A, i_060d3a),
    (0x060D3E, i_060d3e),
    (0x060D40, i_060d40),
    (0x060D46, i_060d46),
    (0x060D4C, i_060d4c),
    (0x060D52, i_060d52),
    (0x060D56, i_060d56),
    (0x060D58, i_060d58),
    (0x060D5E, i_060d5e),
    (0x060D64, i_060d64),
    (0x060D6A, i_060d6a),
    (0x060D6E, i_060d6e),
    (0x060D74, i_060d74),
    (0x060D7A, i_060d7a),
    (0x060D80, i_060d80),
    (0x060D86, i_060d86),
    (0x060D8A, i_060d8a),
    (0x060D8E, i_060d8e),
    (0x060D90, i_060d90),
    (0x060D94, i_060d94),
    (0x060D98, i_060d98),
    (0x060D9C, i_060d9c),
    (0x060D9E, i_060d9e),
    (0x060DA0, i_060da0),
    (0x060DA6, i_060da6),
    (0x060DAC, i_060dac),
    (0x060DB0, i_060db0),
    (0x060DB2, i_060db2),
    (0x060DB4, i_060db4),
    (0x060DB8, i_060db8),
    (0x060DBA, i_060dba),
    (0x060DC0, i_060dc0),
    (0x060DC6, i_060dc6),
    (0x060DCA, i_060dca),
    (0x060DCC, i_060dcc),
    (0x060DD2, i_060dd2),
    (0x060DD8, i_060dd8),
    (0x060DDC, i_060ddc),
    (0x060DE0, i_060de0),
    (0x060DE4, i_060de4),
    (0x060DE6, i_060de6),
    (0x060DE8, i_060de8),
    (0x060DF0, i_060df0),
    (0x060DF6, i_060df6),
    (0x060DFC, i_060dfc),
    (0x060E00, i_060e00),
    (0x060E04, i_060e04),
    (0x060E06, i_060e06),
    (0x060E0A, i_060e0a),
    (0x060E0C, i_060e0c),
    (0x060E12, i_060e12),
    (0x060E18, i_060e18),
    (0x060E1A, i_060e1a),
    (0x060E20, i_060e20),
    (0x060E26, i_060e26),
    (0x060E28, i_060e28),
    (0x060E2E, i_060e2e),
    (0x060E34, i_060e34),
    (0x060E36, i_060e36),
    (0x060E3C, i_060e3c),
    (0x060E42, i_060e42),
    (0x060E44, i_060e44),
    (0x060E46, i_060e46),
    (0x060E4A, i_060e4a),
    (0x060E4E, i_060e4e),
    (0x060E52, i_060e52),
    (0x060E56, i_060e56),
    (0x060E5A, i_060e5a),
    (0x060E5C, i_060e5c),
    (0x060E60, i_060e60),
    (0x060E62, i_060e62),
    (0x060E66, i_060e66),
    (0x060E6C, i_060e6c),
    (0x060E70, i_060e70),
    (0x060E76, i_060e76),
    (0x060E7A, i_060e7a),
    (0x060E80, i_060e80),
    (0x060E84, i_060e84),
    (0x060E8A, i_060e8a),
    (0x060E90, i_060e90),
    (0x060E96, i_060e96),
    (0x060E9C, i_060e9c),
    (0x060EA2, i_060ea2),
    (0x060EA6, i_060ea6),
    (0x060EAC, i_060eac),
    (0x060EB0, i_060eb0),
    (0x060EB6, i_060eb6),
    (0x060EBC, i_060ebc),
    (0x060EC0, i_060ec0),
    (0x060EC6, i_060ec6),
    (0x060ECC, i_060ecc),
    (0x060ED0, i_060ed0),
    (0x060ED6, i_060ed6),
    (0x060EDC, i_060edc),
    (0x060EE0, i_060ee0),
    (0x060EE6, i_060ee6),
    (0x060EEC, i_060eec),
    (0x060EF0, i_060ef0),
    (0x060EF6, i_060ef6),
    (0x060EFA, i_060efa),
    (0x060EFE, i_060efe),
    (0x060F04, i_060f04),
    (0x060F08, i_060f08),
    (0x060F0A, i_060f0a),
    (0x060F0E, i_060f0e),
    (0x060F12, i_060f12),
    (0x060F16, i_060f16),
    (0x060F1A, i_060f1a),
    (0x060F1E, i_060f1e),
    (0x060F24, i_060f24),
    (0x060F2A, i_060f2a),
    (0x060F2E, i_060f2e),
    (0x060F32, i_060f32),
    (0x060F34, i_060f34),
    (0x060F3A, i_060f3a),
    (0x060F3E, i_060f3e),
    (0x060F42, i_060f42),
    (0x060F44, i_060f44),
    (0x060F48, i_060f48),
    (0x060F4C, i_060f4c),
    (0x060F50, i_060f50),
    (0x060F54, i_060f54),
    (0x060F56, i_060f56),
    (0x060F5A, i_060f5a),
    (0x060F5E, i_060f5e),
    (0x060F62, i_060f62),
    (0x060F64, i_060f64),
    (0x060F6A, i_060f6a),
    (0x060F70, i_060f70),
    (0x060F76, i_060f76),
    (0x060F7A, i_060f7a),
    (0x060F80, i_060f80),
    (0x060F86, i_060f86),
    (0x060F88, i_060f88),
    (0x060F8C, i_060f8c),
    (0x060F8E, i_060f8e),
    (0x060F92, i_060f92),
    (0x060F96, i_060f96),
    (0x060F9A, i_060f9a),
    (0x060F9E, i_060f9e),
    (0x060FA2, i_060fa2),
    (0x060FA8, i_060fa8),
    (0x060FAE, i_060fae),
    (0x060FB2, i_060fb2),
    (0x060FB6, i_060fb6),
    (0x060FB8, i_060fb8),
    (0x060FBE, i_060fbe),
    (0x060FC2, i_060fc2),
    (0x060FC6, i_060fc6),
    (0x060FC8, i_060fc8),
    (0x060FCC, i_060fcc),
    (0x060FD0, i_060fd0),
    (0x060FD4, i_060fd4),
    (0x060FD8, i_060fd8),
    (0x060FDA, i_060fda),
    (0x060FDE, i_060fde),
    (0x060FE2, i_060fe2),
    (0x060FE6, i_060fe6),
    (0x060FE8, i_060fe8),
    (0x060FEE, i_060fee),
    (0x060FF4, i_060ff4),
    (0x060FFA, i_060ffa),
    (0x060FFE, i_060ffe),
    (0x061004, i_061004),
    (0x06100A, i_06100a),
    (0x06100C, i_06100c),
    (0x061012, i_061012),
    (0x061018, i_061018),
    (0x06101E, i_06101e),
    (0x061022, i_061022),
    (0x061024, i_061024),
    (0x061028, i_061028),
    (0x06102C, i_06102c),
    (0x061030, i_061030),
    (0x061036, i_061036),
    (0x06103C, i_06103c),
    (0x061040, i_061040),
    (0x061046, i_061046),
    (0x06104C, i_06104c),
    (0x061052, i_061052),
    (0x061058, i_061058),
    (0x06105E, i_06105e),
    (0x061064, i_061064),
    (0x061068, i_061068),
    (0x06106E, i_06106e),
    (0x061072, i_061072),
    (0x061078, i_061078),
    (0x06107E, i_06107e),
    (0x061082, i_061082),
    (0x061088, i_061088),
    (0x06108E, i_06108e),
    (0x061092, i_061092),
    (0x061096, i_061096),
    (0x06109C, i_06109c),
    (0x0610A2, i_0610a2),
    (0x0610A6, i_0610a6),
    (0x0610AC, i_0610ac),
    (0x0610B0, i_0610b0),
    (0x0610B2, i_0610b2),
    (0x0610B6, i_0610b6),
    (0x0610BC, i_0610bc),
    (0x0610C2, i_0610c2),
    (0x0610C8, i_0610c8),
    (0x0610CE, i_0610ce),
    (0x0610D2, i_0610d2),
    (0x0610D6, i_0610d6),
    (0x0610DC, i_0610dc),
    (0x0610E0, i_0610e0),
    (0x0610E6, i_0610e6),
    (0x0610EC, i_0610ec),
    (0x0610F2, i_0610f2),
    (0x0610F4, i_0610f4),
    (0x0610FA, i_0610fa),
    (0x061100, i_061100),
    (0x061104, i_061104),
    (0x061106, i_061106),
    (0x06110A, i_06110a),
    (0x061110, i_061110),
    (0x061116, i_061116),
    (0x06111A, i_06111a),
    (0x06111C, i_06111c),
    (0x061120, i_061120),
    (0x061124, i_061124),
    (0x061128, i_061128),
    (0x06112E, i_06112e),
    (0x061132, i_061132),
    (0x061136, i_061136),
    (0x06113C, i_06113c),
    (0x061140, i_061140),
    (0x061144, i_061144),
    (0x06114A, i_06114a),
    (0x061150, i_061150),
    (0x061156, i_061156),
    (0x06115C, i_06115c),
    (0x061162, i_061162),
    (0x061168, i_061168),
    (0x06116C, i_06116c),
    (0x061172, i_061172),
    (0x061176, i_061176),
    (0x06117C, i_06117c),
    (0x061182, i_061182),
    (0x061186, i_061186),
    (0x06118C, i_06118c),
    (0x061192, i_061192),
    (0x061198, i_061198),
    (0x06119E, i_06119e),
    (0x0611A2, i_0611a2),
    (0x0611A6, i_0611a6),
    (0x0611AC, i_0611ac),
    (0x0611B2, i_0611b2),
    (0x0611B6, i_0611b6),
    (0x0611BC, i_0611bc),
    (0x0611C0, i_0611c0),
    (0x0611C2, i_0611c2),
    (0x0611C6, i_0611c6),
    (0x0611CC, i_0611cc),
    (0x0611D2, i_0611d2),
    (0x0611D8, i_0611d8),
    (0x0611DE, i_0611de),
    (0x0611E2, i_0611e2),
    (0x0611E6, i_0611e6),
    (0x0611EC, i_0611ec),
    (0x0611F0, i_0611f0),
    (0x0611F6, i_0611f6),
    (0x0611FC, i_0611fc),
    (0x061202, i_061202),
    (0x061204, i_061204),
    (0x06120A, i_06120a),
    (0x06120E, i_06120e),
    (0x061210, i_061210),
    (0x061214, i_061214),
    (0x061218, i_061218),
    (0x06121C, i_06121c),
    (0x061220, i_061220),
    (0x061226, i_061226),
    (0x06122C, i_06122c),
    (0x061232, i_061232),
    (0x061238, i_061238),
    (0x06123C, i_06123c),
    (0x061242, i_061242),
    (0x061246, i_061246),
    (0x06124C, i_06124c),
    (0x061252, i_061252),
    (0x061254, i_061254),
    (0x061258, i_061258),
    (0x06125E, i_06125e),
    (0x061262, i_061262),
    (0x061268, i_061268),
    (0x06126E, i_06126e),
    (0x061274, i_061274),
    (0x061278, i_061278),
    (0x06127A, i_06127a),
    (0x061280, i_061280),
    (0x061284, i_061284),
    (0x06128A, i_06128a),
    (0x06128E, i_06128e),
    (0x061292, i_061292),
    (0x061296, i_061296),
    (0x06129C, i_06129c),
    (0x0612A0, i_0612a0),
    (0x0612A6, i_0612a6),
    (0x0612A8, i_0612a8),
    (0x0612AE, i_0612ae),
    (0x0612B2, i_0612b2),
    (0x0612B4, i_0612b4),
    (0x0612BA, i_0612ba),
    (0x0612C0, i_0612c0),
    (0x0612C4, i_0612c4),
    (0x0612CA, i_0612ca),
    (0x0612D0, i_0612d0),
    (0x0612D6, i_0612d6),
    (0x0612DA, i_0612da),
    (0x0612DE, i_0612de),
    (0x0612E4, i_0612e4),
    (0x0612E6, i_0612e6),
    (0x0612EA, i_0612ea),
    (0x0612F0, i_0612f0),
    (0x0612F6, i_0612f6),
    (0x0612FA, i_0612fa),
    (0x0612FE, i_0612fe),
    (0x061304, i_061304),
    (0x06130A, i_06130a),
    (0x061310, i_061310),
    (0x061314, i_061314),
    (0x061316, i_061316),
    (0x06131C, i_06131c),
    (0x061322, i_061322),
    (0x061326, i_061326),
    (0x06132C, i_06132c),
    (0x061332, i_061332),
    (0x061338, i_061338),
    (0x06133C, i_06133c),
    (0x061342, i_061342),
    (0x061344, i_061344),
    (0x061348, i_061348),
    (0x06134E, i_06134e),
    (0x061352, i_061352),
    (0x061358, i_061358),
    (0x06135E, i_06135e),
    (0x061364, i_061364),
    (0x06136A, i_06136a),
    (0x061370, i_061370),
    (0x061376, i_061376),
    (0x061378, i_061378),
    (0x06137A, i_06137a),
    (0x06137E, i_06137e),
    (0x061382, i_061382),
    (0x061388, i_061388),
    (0x06138E, i_06138e),
    (0x061394, i_061394),
    (0x06139A, i_06139a),
    (0x06139E, i_06139e),
    (0x0613A4, i_0613a4),
    (0x0613AA, i_0613aa),
    (0x0613AE, i_0613ae),
    (0x0613B4, i_0613b4),
    (0x0613B8, i_0613b8),
    (0x0613BE, i_0613be),
    (0x0613C2, i_0613c2),
    (0x0613C4, i_0613c4),
    (0x0613CA, i_0613ca),
    (0x0613CE, i_0613ce),
    (0x0613D2, i_0613d2),
    (0x0613D4, i_0613d4),
    (0x0613DA, i_0613da),
    (0x0613DE, i_0613de),
    (0x0613E4, i_0613e4),
    (0x0613E8, i_0613e8),
    (0x0613EC, i_0613ec),
    (0x0613EE, i_0613ee),
    (0x0613F4, i_0613f4),
    (0x0613F8, i_0613f8),
    (0x0613FE, i_0613fe),
    (0x061402, i_061402),
    (0x061408, i_061408),
    (0x06140E, i_06140e),
    (0x061412, i_061412),
    (0x061418, i_061418),
    (0x06141E, i_06141e),
    (0x061422, i_061422),
    (0x061426, i_061426),
    (0x061428, i_061428),
    (0x06142A, i_06142a),
    (0x06142E, i_06142e),
    (0x061430, i_061430),
    (0x061434, i_061434),
    (0x06143A, i_06143a),
    (0x061440, i_061440),
    (0x061446, i_061446),
    (0x06144A, i_06144a),
    (0x061450, i_061450),
    (0x061452, i_061452),
    (0x061456, i_061456),
    (0x06145C, i_06145c),
    (0x061462, i_061462),
    (0x061468, i_061468),
    (0x06146E, i_06146e),
    (0x061472, i_061472),
    (0x061474, i_061474),
    (0x06147A, i_06147a),
    (0x061480, i_061480),
    (0x061482, i_061482),
    (0x061486, i_061486),
    (0x06148A, i_06148a),
    (0x06148E, i_06148e),
    (0x061494, i_061494),
    (0x061498, i_061498),
    (0x06149A, i_06149a),
    (0x06149E, i_06149e),
    (0x0614A2, i_0614a2),
    (0x0614A6, i_0614a6),
    (0x0614AC, i_0614ac),
    (0x0614B0, i_0614b0),
    (0x0614B4, i_0614b4),
    (0x0614B6, i_0614b6),
    (0x0614B8, i_0614b8),
    (0x0614BC, i_0614bc),
    (0x0614C0, i_0614c0),
    (0x0614C4, i_0614c4),
    (0x0614C8, i_0614c8),
    (0x0614CE, i_0614ce),
    (0x0614D0, i_0614d0),
    (0x0614D4, i_0614d4),
    (0x0614DA, i_0614da),
    (0x0614DE, i_0614de),
    (0x0614E4, i_0614e4),
    (0x0614E6, i_0614e6),
    (0x0614EA, i_0614ea),
    (0x0614EC, i_0614ec),
    (0x0614F0, i_0614f0),
    (0x0614F4, i_0614f4),
    (0x0614F8, i_0614f8),
    (0x0614FC, i_0614fc),
    (0x0614FE, i_0614fe),
    (0x061504, i_061504),
    (0x061508, i_061508),
    (0x06150C, i_06150c),
    (0x061510, i_061510),
    (0x061516, i_061516),
    (0x06151A, i_06151a),
    (0x06151E, i_06151e),
    (0x061522, i_061522),
    (0x061524, i_061524),
    (0x061528, i_061528),
    (0x06152A, i_06152a),
    (0x06152E, i_06152e),
    (0x061530, i_061530),
    (0x061536, i_061536),
    (0x06153A, i_06153a),
    (0x061540, i_061540),
    (0x061544, i_061544),
    (0x061546, i_061546),
    (0x06154C, i_06154c),
    (0x061550, i_061550),
    (0x061552, i_061552),
    (0x061556, i_061556),
    (0x061558, i_061558),
    (0x06155C, i_06155c),
    (0x061560, i_061560),
    (0x061564, i_061564),
    (0x061568, i_061568),
    (0x06156C, i_06156c),
    (0x061570, i_061570),
    (0x061574, i_061574),
    (0x061578, i_061578),
    (0x06157E, i_06157e),
    (0x061580, i_061580),
    (0x061584, i_061584),
    (0x061588, i_061588),
    (0x06158C, i_06158c),
    (0x061590, i_061590),
    (0x061594, i_061594),
    (0x06159A, i_06159a),
    (0x06159E, i_06159e),
    (0x0615A2, i_0615a2),
    (0x0615A8, i_0615a8),
    (0x0615AC, i_0615ac),
    (0x0615AE, i_0615ae),
    (0x0615B2, i_0615b2),
    (0x0615B8, i_0615b8),
    (0x0615BC, i_0615bc),
    (0x0615C2, i_0615c2),
    (0x0615C8, i_0615c8),
    (0x0615CC, i_0615cc),
    (0x0615D2, i_0615d2),
    (0x0615D8, i_0615d8),
    (0x0615DC, i_0615dc),
    (0x0615E2, i_0615e2),
    (0x0615E6, i_0615e6),
    (0x0615EC, i_0615ec),
    (0x0615EE, i_0615ee),
    (0x0615F4, i_0615f4),
    (0x0615F6, i_0615f6),
    (0x0615FA, i_0615fa),
    (0x061600, i_061600),
    (0x061604, i_061604),
    (0x06160A, i_06160a),
    (0x06160E, i_06160e),
    (0x061614, i_061614),
    (0x061616, i_061616),
    (0x06161C, i_06161c),
    (0x06161E, i_06161e),
    (0x061624, i_061624),
    (0x06162A, i_06162a),
    (0x06162E, i_06162e),
    (0x061632, i_061632),
    (0x061638, i_061638),
    (0x06163A, i_06163a),
    (0x061640, i_061640),
    (0x061644, i_061644),
    (0x06164A, i_06164a),
    (0x06164C, i_06164c),
    (0x061650, i_061650),
    (0x061654, i_061654),
    (0x061658, i_061658),
    (0x06165C, i_06165c),
    (0x06165E, i_06165e),
    (0x061660, i_061660),
    (0x061666, i_061666),
    (0x061668, i_061668),
    (0x06166C, i_06166c),
    (0x061670, i_061670),
    (0x061676, i_061676),
    (0x06167A, i_06167a),
    (0x06167C, i_06167c),
    (0x06167E, i_06167e),
    (0x061682, i_061682),
    (0x061686, i_061686),
    (0x06168A, i_06168a),
    (0x06168E, i_06168e),
    (0x061692, i_061692),
    (0x061696, i_061696),
    (0x06169C, i_06169c),
    (0x06169E, i_06169e),
    (0x0616A0, i_0616a0),
    (0x0616A4, i_0616a4),
    (0x0616A8, i_0616a8),
    (0x0616AE, i_0616ae),
    (0x0616B2, i_0616b2),
    (0x0616B6, i_0616b6),
    (0x0616B8, i_0616b8),
    (0x0616BA, i_0616ba),
    (0x0616BE, i_0616be),
    (0x0616C2, i_0616c2),
    (0x0616C8, i_0616c8),
    (0x0616CA, i_0616ca),
    (0x0616CE, i_0616ce),
    (0x0616D2, i_0616d2),
    (0x0616D6, i_0616d6),
    (0x0616DA, i_0616da),
    (0x0616DC, i_0616dc),
    (0x0616DE, i_0616de),
    (0x0616E4, i_0616e4),
    (0x0616E6, i_0616e6),
    (0x0616EA, i_0616ea),
    (0x0616EE, i_0616ee),
    (0x0616F4, i_0616f4),
    (0x0616F8, i_0616f8),
    (0x0616FA, i_0616fa),
    (0x0616FC, i_0616fc),
    (0x061700, i_061700),
    (0x061704, i_061704),
    (0x061708, i_061708),
    (0x06170C, i_06170c),
    (0x061710, i_061710),
    (0x061714, i_061714),
    (0x06171A, i_06171a),
    (0x06171C, i_06171c),
    (0x06171E, i_06171e),
    (0x061722, i_061722),
    (0x061726, i_061726),
    (0x06172C, i_06172c),
    (0x061730, i_061730),
    (0x061734, i_061734),
    (0x061736, i_061736),
    (0x061738, i_061738),
    (0x06173C, i_06173c),
    (0x061742, i_061742),
    (0x061748, i_061748),
    (0x06174A, i_06174a),
    (0x06174E, i_06174e),
    (0x061752, i_061752),
    (0x061756, i_061756),
    (0x06175A, i_06175a),
    (0x061760, i_061760),
    (0x061764, i_061764),
    (0x061768, i_061768),
    (0x06176C, i_06176c),
    (0x06176E, i_06176e),
    (0x061772, i_061772),
    (0x061774, i_061774),
    (0x061778, i_061778),
    (0x06177C, i_06177c),
    (0x061780, i_061780),
    (0x061784, i_061784),
    (0x06178A, i_06178a),
    (0x06178E, i_06178e),
    (0x061792, i_061792),
    (0x061796, i_061796),
    (0x061798, i_061798),
    (0x06179C, i_06179c),
    (0x06179E, i_06179e),
    (0x0617A2, i_0617a2),
    (0x0617A6, i_0617a6),
    (0x0617AA, i_0617aa),
    (0x0617AE, i_0617ae),
    (0x0617B2, i_0617b2),
    (0x0617B6, i_0617b6),
    (0x0617B8, i_0617b8),
    (0x0617BA, i_0617ba),
    (0x0617BE, i_0617be),
    (0x0617C4, i_0617c4),
    (0x0617C8, i_0617c8),
    (0x0617CC, i_0617cc),
    (0x0617D0, i_0617d0),
    (0x0617D2, i_0617d2),
    (0x0617D6, i_0617d6),
    (0x0617DC, i_0617dc),
    (0x0617E0, i_0617e0),
    (0x0617E4, i_0617e4),
    (0x0617E8, i_0617e8),
    (0x0617EE, i_0617ee),
    (0x0617F0, i_0617f0),
    (0x0617F4, i_0617f4),
    (0x0617F8, i_0617f8),
    (0x0617FE, i_0617fe),
    (0x061800, i_061800),
    (0x061804, i_061804),
    (0x061808, i_061808),
    (0x06180E, i_06180e),
    (0x061810, i_061810),
    (0x061814, i_061814),
    (0x061818, i_061818),
    (0x06181E, i_06181e),
    (0x061820, i_061820),
    (0x061824, i_061824),
    (0x061828, i_061828),
    (0x06182E, i_06182e),
    (0x061830, i_061830),
    (0x061834, i_061834),
    (0x061838, i_061838),
    (0x06183E, i_06183e),
    (0x061840, i_061840),
    (0x061846, i_061846),
    (0x061848, i_061848),
    (0x06184C, i_06184c),
    (0x061850, i_061850),
    (0x061856, i_061856),
    (0x061858, i_061858),
    (0x06185C, i_06185c),
    (0x061860, i_061860),
    (0x061866, i_061866),
    (0x061868, i_061868),
    (0x06186C, i_06186c),
    (0x061870, i_061870),
    (0x061876, i_061876),
    (0x061878, i_061878),
    (0x06187C, i_06187c),
    (0x061880, i_061880),
    (0x061886, i_061886),
    (0x061888, i_061888),
    (0x06188C, i_06188c),
    (0x061890, i_061890),
    (0x061896, i_061896),
    (0x061898, i_061898),
    (0x06189C, i_06189c),
    (0x0618A0, i_0618a0),
    (0x0618A6, i_0618a6),
    (0x0618A8, i_0618a8),
    (0x0618AE, i_0618ae),
    (0x0618B0, i_0618b0),
    (0x0618B6, i_0618b6),
    (0x0618BA, i_0618ba),
    (0x0618C0, i_0618c0),
    (0x0618C4, i_0618c4),
    (0x0618CA, i_0618ca),
    (0x0618D0, i_0618d0),
    (0x0618D4, i_0618d4),
    (0x0618DA, i_0618da),
    (0x0618E0, i_0618e0),
    (0x0618E4, i_0618e4),
    (0x0618EA, i_0618ea),
    (0x0618EE, i_0618ee),
    (0x0618F4, i_0618f4),
    (0x0618F8, i_0618f8),
    (0x0618FE, i_0618fe),
    (0x061904, i_061904),
    (0x06190A, i_06190a),
    (0x06190E, i_06190e),
    (0x061910, i_061910),
    (0x061914, i_061914),
    (0x06191A, i_06191a),
    (0x061920, i_061920),
    (0x061924, i_061924),
    (0x06192A, i_06192a),
    (0x061930, i_061930),
    (0x061936, i_061936),
    (0x06193C, i_06193c),
    (0x061940, i_061940),
    (0x061944, i_061944),
    (0x061946, i_061946),
    (0x06194A, i_06194a),
    (0x06194C, i_06194c),
    (0x061950, i_061950),
    (0x061952, i_061952),
    (0x061954, i_061954),
    (0x061958, i_061958),
    (0x06195A, i_06195a),
    (0x06195E, i_06195e),
    (0x061962, i_061962),
    (0x061964, i_061964),
    (0x061968, i_061968),
    (0x06196A, i_06196a),
    (0x06196E, i_06196e),
    (0x061974, i_061974),
    (0x061976, i_061976),
    (0x06197A, i_06197a),
    (0x06197C, i_06197c),
    (0x061982, i_061982),
    (0x061988, i_061988),
    (0x06198C, i_06198c),
    (0x061992, i_061992),
    (0x061994, i_061994),
    (0x061998, i_061998),
    (0x06199A, i_06199a),
    (0x06199E, i_06199e),
    (0x0619A2, i_0619a2),
    (0x0619A6, i_0619a6),
    (0x0619A8, i_0619a8),
    (0x0619AC, i_0619ac),
    (0x0619AE, i_0619ae),
    (0x0619B2, i_0619b2),
    (0x0619B6, i_0619b6),
    (0x0619BA, i_0619ba),
    (0x0619BE, i_0619be),
    (0x0619C2, i_0619c2),
    (0x0619C4, i_0619c4),
    (0x0619C8, i_0619c8),
    (0x0619CA, i_0619ca),
    (0x0619CE, i_0619ce),
    (0x0619D2, i_0619d2),
    (0x0619EE, i_0619ee),
    (0x0619F4, i_0619f4),
    (0x0619FA, i_0619fa),
    (0x0619FE, i_0619fe),
    (0x061A02, i_061a02),
    (0x061A06, i_061a06),
    (0x061A0A, i_061a0a),
    (0x061A10, i_061a10),
    (0x061A16, i_061a16),
    (0x061A1C, i_061a1c),
    (0x061A20, i_061a20),
    (0x061A26, i_061a26),
    (0x061A2C, i_061a2c),
    (0x061A32, i_061a32),
    (0x061A38, i_061a38),
    (0x061A3E, i_061a3e),
    (0x061A42, i_061a42),
    (0x061A48, i_061a48),
    (0x061A4E, i_061a4e),
    (0x061A52, i_061a52),
    (0x061A5A, i_061a5a),
    (0x061A62, i_061a62),
    (0x061A68, i_061a68),
    (0x061A6C, i_061a6c),
    (0x061A72, i_061a72),
    (0x061A78, i_061a78),
    (0x061A7E, i_061a7e),
    (0x061A82, i_061a82),
    (0x061A88, i_061a88),
    (0x061A8E, i_061a8e),
    (0x061A94, i_061a94),
    (0x061A9A, i_061a9a),
    (0x061AA2, i_061aa2),
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
];
