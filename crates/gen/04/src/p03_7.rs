// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $03EBA0: cmpi.w #$0, $70(a6)
pub fn i_03eba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBA6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EBA6: bgt.w $3ebf6
pub fn i_03eba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBAA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EBF6; return 10; }
    8
}
/// $03EBAA: cmpi.w #$0, $30(a6)
pub fn i_03ebaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EBB0: bgt.w $3ebbc
pub fn i_03ebb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBB4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EBBC; return 10; }
    8
}
// ===== SetTaskHandler_03ebb4  ($03EBB4, 8 B, kernel/task_handlers.c)
/// $03EBB4: lea.l $3ebf8(pc), a1
pub fn i_03ebb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EBF8;
    16
}
/// $03EBB8: move.l a1, (a6)
pub fn i_03ebb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EBBA: rts
pub fn i_03ebba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerBBlink_03ebbc  ($03EBBC, 60 B, pow/results_squadleader_03daa8.s)
/// $03EBBC: cmpi.b #$0, $5c(a6)
pub fn i_03ebbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBC2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03EBC2: bne.w $3ebd0
pub fn i_03ebc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EBD0; return 10; }
    8
}
/// $03EBC6: lea.l $282a7c.l, a2
pub fn i_03ebc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBCC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A7C;
    20
}
/// $03EBCC: bra.w $3ebd6
pub fn i_03ebcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EBD6;
    10
}
/// $03EBD0: lea.l $282a88.l, a2
pub fn i_03ebd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBD6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A88;
    20
}
/// $03EBD6: movea.w #$70d1, a1
pub fn i_03ebd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBDA;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70D1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EBDA: adda.w $72(a6), a1
pub fn i_03ebda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBDE;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EBDE: move.b #$4, d1
pub fn i_03ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBE2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EBE2: jsr $477fc.l
pub fn i_03ebe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EBE8);
    cpu.pc = 0x477FC;
    16
}
/// $03EBE8: move.w #$8, $70(a6)
pub fn i_03ebe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EBEE: not.b $5c(a6)
pub fn i_03ebee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBF2;
    cpu.ir = 0x462E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03EBF2: subq.w #$1, $30(a6)
pub fn i_03ebf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBF6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EBF6: rts
pub fn i_03ebf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerBDraw_03ebf8  ($03EBF8, 24 B, pow/results_squadleader_03daa8.s)
/// $03EBF8: movea.w #$70d1, a1
pub fn i_03ebf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBFC;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70D1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EBFC: adda.w $72(a6), a1
pub fn i_03ebfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC00;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EC00: move.b #$4, d1
pub fn i_03ec00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC04;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EC04: lea.l $282a7c.l, a2
pub fn i_03ec04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC0A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A7C;
    20
}
/// $03EC0A: jsr $477fc.l
pub fn i_03ec0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EC10);
    cpu.pc = 0x477FC;
    16
}
// ===== SetTaskHandler_03ec10  ($03EC10, 8 B, kernel/task_handlers.c)
/// $03EC10: lea.l $3ec16(pc), a1
pub fn i_03ec10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EC16;
    16
}
/// $03EC14: move.l a1, (a6)
pub fn i_03ec14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EC16: rts
pub fn i_03ec16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerC_03ec18  ($03EC18, 48 B, pow/results_squadleader_03daa8.s)
/// $03EC18: move.w #$6, $30(a6)
pub fn i_03ec18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EC1E: move.w #$0, $70(a6)
pub fn i_03ec1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EC24: move.b #$0, $5c(a6)
pub fn i_03ec24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03EC2A: lea.l $3ec30(pc), a1
pub fn i_03ec2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EC30;
    16
}
/// $03EC2E: move.l a1, (a6)
pub fn i_03ec2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EC30: subq.w #$1, $70(a6)
pub fn i_03ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC34;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EC34: cmpi.w #$0, $70(a6)
pub fn i_03ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC3A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EC3A: bgt.w $3ec8a
pub fn i_03ec3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC3E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EC8A; return 10; }
    8
}
/// $03EC3E: cmpi.w #$0, $30(a6)
pub fn i_03ec3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC44;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EC44: bgt.w $3ec50
pub fn i_03ec44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC48;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EC50; return 10; }
    8
}
// ===== SetTaskHandler_03ec48  ($03EC48, 8 B, kernel/task_handlers.c)
/// $03EC48: lea.l $3ec8c(pc), a1
pub fn i_03ec48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EC8C;
    16
}
/// $03EC4C: move.l a1, (a6)
pub fn i_03ec4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EC4E: rts
pub fn i_03ec4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerCBlink_03ec50  ($03EC50, 60 B, pow/results_squadleader_03daa8.s)
/// $03EC50: cmpi.b #$0, $5c(a6)
pub fn i_03ec50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC56;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03EC56: bne.w $3ec64
pub fn i_03ec56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC5A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EC64; return 10; }
    8
}
/// $03EC5A: lea.l $282a5c.l, a2
pub fn i_03ec5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC60;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A5C;
    20
}
/// $03EC60: bra.w $3ec6a
pub fn i_03ec60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC64;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EC6A;
    10
}
/// $03EC64: lea.l $282a62.l, a2
pub fn i_03ec64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC6A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A62;
    20
}
/// $03EC6A: movea.w #$70f1, a1
pub fn i_03ec6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC6E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70F1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EC6E: adda.w $72(a6), a1
pub fn i_03ec6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC72;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EC72: move.b #$4, d1
pub fn i_03ec72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC76;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EC76: jsr $4784c.l
pub fn i_03ec76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EC7C);
    cpu.pc = 0x4784C;
    16
}
/// $03EC7C: move.w #$8, $70(a6)
pub fn i_03ec7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EC82: not.b $5c(a6)
pub fn i_03ec82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC86;
    cpu.ir = 0x462E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03EC86: subq.w #$1, $30(a6)
pub fn i_03ec86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC8A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EC8A: rts
pub fn i_03ec8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerCDraw_03ec8c  ($03EC8C, 24 B, pow/results_squadleader_03daa8.s)
/// $03EC8C: movea.w #$70f1, a1
pub fn i_03ec8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC90;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70F1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EC90: adda.w $72(a6), a1
pub fn i_03ec90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC94;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EC94: move.b #$4, d1
pub fn i_03ec94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC98;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EC98: lea.l $282a5c.l, a2
pub fn i_03ec98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC9E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A5C;
    20
}
/// $03EC9E: jsr $4784c.l
pub fn i_03ec9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ECA4);
    cpu.pc = 0x4784C;
    16
}
// ===== SetTaskHandler_03eca4  ($03ECA4, 8 B, kernel/task_handlers.c)
/// $03ECA4: lea.l $3ecaa(pc), a1
pub fn i_03eca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3ECAA;
    16
}
/// $03ECA8: move.l a1, (a6)
pub fn i_03eca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03ECAA: rts
pub fn i_03ecaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_DrawLabels_03ecac  ($03ECAC, 292 B, pow/results_squadleader_03daa8.s)
/// $03ECAC: movea.w #$7068, a1
pub fn i_03ecac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECB0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7068;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ECB0: adda.w $72(a6), a1
pub fn i_03ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECB4;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ECB4: move.w #$23ba, d0
pub fn i_03ecb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECB8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECB8: move.w #$1, d1
pub fn i_03ecb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECBC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECBC: move.w #$1, d2
pub fn i_03ecbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECC0;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECC0: jsr $5da9c.l
pub fn i_03ecc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ECC6);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ECC6: movea.w #$7088, a1
pub fn i_03ecc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECCA;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7088;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ECCA: adda.w $72(a6), a1
pub fn i_03ecca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECCE;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ECCE: move.w #$23bf, d0
pub fn i_03ecce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECD2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23BF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECD2: move.w #$f, d1
pub fn i_03ecd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECD6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECD6: move.w #$1, d2
pub fn i_03ecd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECDA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECDA: jsr $5da9c.l
pub fn i_03ecda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ECE0);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ECE0: movea.w #$7268, a1
pub fn i_03ece0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECE4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7268;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ECE4: adda.w $72(a6), a1
pub fn i_03ece4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECE8;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ECE8: move.w #$23bb, d0
pub fn i_03ece8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23BB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECEC: move.w #$1, d1
pub fn i_03ecec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECF0: move.w #$1, d2
pub fn i_03ecf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECF4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ECF4: jsr $5da9c.l
pub fn i_03ecf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ECFA);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ECFA: movea.w #$7093, a1
pub fn i_03ecfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ECFE;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7093;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ECFE: adda.w $72(a6), a1
pub fn i_03ecfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED02;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ED02: move.w #$23cc, d0
pub fn i_03ed02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED06;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23CC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED06: move.w #$1, d1
pub fn i_03ed06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED0A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED0A: move.w #$1, d2
pub fn i_03ed0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED0E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED0E: jsr $5da9c.l
pub fn i_03ed0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ED14);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ED14: movea.w #$70b3, a1
pub fn i_03ed14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED18;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70B3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ED18: adda.w $72(a6), a1
pub fn i_03ed18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED1C;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ED1C: move.w #$23cd, d0
pub fn i_03ed1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23CD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED20: move.w #$d, d1
pub fn i_03ed20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED24;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED24: move.w #$1, d2
pub fn i_03ed24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED28;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED28: jsr $5da9c.l
pub fn i_03ed28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ED2E);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ED2E: movea.w #$7253, a1
pub fn i_03ed2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED32;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7253;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ED32: adda.w $72(a6), a1
pub fn i_03ed32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED36;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ED36: move.w #$23ce, d0
pub fn i_03ed36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED3A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23CE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED3A: move.w #$1, d1
pub fn i_03ed3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED3E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED3E: move.w #$1, d2
pub fn i_03ed3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED42;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED42: jsr $5da9c.l
pub fn i_03ed42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ED48);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ED48: movea.w #$707b, a1
pub fn i_03ed48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED4C;
    cpu.ir = 0x327C;
    let v1: u32 = 0x707B;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ED4C: adda.w $72(a6), a1
pub fn i_03ed4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED50;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ED50: move.w #$23bc, d0
pub fn i_03ed50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED54;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23BC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED54: move.w #$1, d1
pub fn i_03ed54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED58;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED58: move.w #$1, d2
pub fn i_03ed58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED5C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED5C: jsr $5da9c.l
pub fn i_03ed5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ED62);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ED62: movea.w #$709b, a1
pub fn i_03ed62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED66;
    cpu.ir = 0x327C;
    let v1: u32 = 0x709B;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ED66: adda.w $72(a6), a1
pub fn i_03ed66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED6A;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ED6A: move.w #$23cb, d0
pub fn i_03ed6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23CB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED6E: move.w #$f, d1
pub fn i_03ed6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED72;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED72: move.w #$1, d2
pub fn i_03ed72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED76;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED76: jsr $5da9c.l
pub fn i_03ed76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ED7C);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ED7C: movea.w #$727b, a1
pub fn i_03ed7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED80;
    cpu.ir = 0x327C;
    let v1: u32 = 0x727B;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ED80: adda.w $72(a6), a1
pub fn i_03ed80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED84;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ED84: move.w #$23bd, d0
pub fn i_03ed84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED88;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23BD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED88: move.w #$1, d1
pub fn i_03ed88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED8C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED8C: move.w #$1, d2
pub fn i_03ed8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED90;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03ED90: jsr $5da9c.l
pub fn i_03ed90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ED96);
    cpu.pc = 0x5DA9C;
    16
}
/// $03ED96: movea.w #$7069, a1
pub fn i_03ed96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED9A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7069;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03ED9A: adda.w $72(a6), a1
pub fn i_03ed9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ED9E;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03ED9E: move.w #$23be, d0
pub fn i_03ed9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDA2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23BE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EDA2: move.w #$1, d1
pub fn i_03eda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDA6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EDA6: move.w #$12, d2
pub fn i_03eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDAA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EDAA: jsr $5da9c.l
pub fn i_03edaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EDB0);
    cpu.pc = 0x5DA9C;
    16
}
/// $03EDB0: movea.w #$7269, a1
pub fn i_03edb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDB4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7269;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EDB4: adda.w $72(a6), a1
pub fn i_03edb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDB8;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EDB8: move.w #$23ca, d0
pub fn i_03edb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23CA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EDBC: move.w #$1, d1
pub fn i_03edbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDC0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EDC0: move.w #$12, d2
pub fn i_03edc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDC4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EDC4: jsr $5da9c.l
pub fn i_03edc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EDCA);
    cpu.pc = 0x5DA9C;
    16
}
/// $03EDCA: jmp $518.l
pub fn i_03edca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDD0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_03edd0  ($03EDD0, 2 B, pow/results_squadleader_03daa8.s)
/// $03EDD0: rts
pub fn i_03edd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PollStartP1_03edd2  ($03EDD2, 28 B, pow/results_squadleader_03daa8.s)
/// $03EDD2: move.b $10e214.l, d0
pub fn i_03edd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDD8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E214) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $03EDD8: btst.b #$5, d0
pub fn i_03edd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDDC;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $03EDDC: bne.w $3ede8
pub fn i_03eddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EDE8; return 10; }
    8
}
/// $03EDE0: clr.b $8a(a6)
pub fn i_03ede0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03EDE4: bra.w $3edf4
pub fn i_03ede4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EDF4;
    10
}
/// $03EDE8: move.b #$1, $8a(a6)
pub fn i_03ede8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDEE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_03edee  ($03EDEE, 6 B, thunks/ccr_helpers.c)
/// $03EDEE: ori.b #$11, ccr
pub fn i_03edee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDF2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03EDF2: rts
pub fn i_03edf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PollStartP2_03edf4  ($03EDF4, 18 B, pow/results_squadleader_03daa8.s)
/// $03EDF4: move.b $10e21a.l, d0
pub fn i_03edf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDFA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E21A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $03EDFA: btst.b #$5, d0
pub fn i_03edfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EDFE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $03EDFE: bne.w $3ee10
pub fn i_03edfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE02;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EE10; return 10; }
    8
}
/// $03EE02: clr.b $8a(a6)
pub fn i_03ee02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE06;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== ClearXN_03ee06  ($03EE06, 6 B, thunks/ccr_helpers.c)
/// $03EE06: andi.b #$ee, ccr
pub fn i_03ee06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE0A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03EE0A: rts
pub fn i_03ee0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PollRts_03ee0c  ($03EE0C, 4 B, pow/results_squadleader_03daa8.s)
/// $03EE0C: bra.w $3ee1c
pub fn i_03ee0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE10;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EE1C;
    10
}
// ===== Results_PollSetDone_03ee10  ($03EE10, 6 B, pow/results_squadleader_03daa8.s)
/// $03EE10: move.b #$1, $8a(a6)
pub fn i_03ee10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_03ee16  ($03EE16, 6 B, thunks/ccr_helpers.c)
/// $03EE16: ori.b #$11, ccr
pub fn i_03ee16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE1A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03EE1A: rts
pub fn i_03ee1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0003EE1C  ($03EE1C, 2 B, thunks/trivial_rts.c)
/// $03EE1C: rts
pub fn i_03ee1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PollTail_03ee1e  ($03EE1E, 16 B, pow/results_squadleader_03daa8.s)
/// $03EE1E: movea.l $8(a6), a1
pub fn i_03ee1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE22;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03EE22: move.b $10(a6), d0
pub fn i_03ee22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE26;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03EE26: cmp.b $10(a1), d0
pub fn i_03ee26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE2A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $03EE2A: bcs.w $3ee34
pub fn i_03ee2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE2E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3EE34; return 10; }
    8
}
// ===== ClearXN_03ee2e  ($03EE2E, 6 B, thunks/ccr_helpers.c)
/// $03EE2E: andi.b #$ee, ccr
pub fn i_03ee2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE32;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03EE32: rts
pub fn i_03ee32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_03ee34  ($03EE34, 6 B, thunks/ccr_helpers.c)
/// $03EE34: ori.b #$11, ccr
pub fn i_03ee34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE38;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03EE38: rts
pub fn i_03ee38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Subsystem_ScoresInit_03EE3A  ($03EE3A, 14 B, pow/results_squadleader_03daa8.s)
/// $03EE3A: clr.w $106f4c.l
pub fn i_03ee3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE40;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F4C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F4C, r2 as u16);
    16
}
/// $03EE40: clr.w $106f4e.l
pub fn i_03ee40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE46;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F4E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F4E, r2 as u16);
    16
}
/// $03EE46: rts
pub fn i_03ee46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_CountIfPending_03ee48  ($03EE48, 10 B, pow/results_squadleader_03daa8.s)
/// $03EE48: tst.b $106ed2.l
pub fn i_03ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE4E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $03EE4E: bne.w $3ee58
pub fn i_03ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EE58; return 10; }
    8
}
// ===== ClearC_03ee52  ($03EE52, 6 B, thunks/ccr_helpers.c)
/// $03EE52: andi.b #$fe, ccr
pub fn i_03ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE56;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EE56: rts
pub fn i_03ee56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueCredit_03ee58  ($03EE58, 84 B, pow/results_squadleader_03daa8.s)
/// $03EE58: move.l #$ffffffff, -(a7)
pub fn i_03ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE5E;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $03EE5E: moveq #$0, d0
pub fn i_03ee5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE60;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EE60: jsr $5e3a2.l
pub fn i_03ee60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EE66);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EE66: bcc.w $3ee74
pub fn i_03ee66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EE74; return 10; }
    8
}
/// $03EE6A: bsr.w $3ef5c
pub fn i_03ee6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE6E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EE6E);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EE6E: bcc.w $3ee74
pub fn i_03ee6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EE74; return 10; }
    8
}
/// $03EE72: move.l a0, (a7)
pub fn i_03ee72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE74;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EE74: moveq #$1, d0
pub fn i_03ee74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE76;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EE76: jsr $5e3a2.l
pub fn i_03ee76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EE7C);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EE7C: bcc.w $3eea0
pub fn i_03ee7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEA0; return 10; }
    8
}
/// $03EE80: bsr.w $3ef5c
pub fn i_03ee80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE84;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EE84);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EE84: bcc.w $3eea0
pub fn i_03ee84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEA0; return 10; }
    8
}
/// $03EE88: move.l (a7), d0
pub fn i_03ee88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE8A;
    cpu.ir = 0x2017;
    let v2 = bus.read32(cpu.a[7]);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EE8A: bmi.w $3ee9e
pub fn i_03ee8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE8E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x3EE9E; return 10; }
    8
}
/// $03EE8E: movea.l d0, a2
pub fn i_03ee8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE90;
    cpu.ir = 0x2440;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $03EE90: move.w $22(a0), d0
pub fn i_03ee90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE94;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EE94: cmp.w $22(a2), d0
pub fn i_03ee94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE98;
    cpu.ir = 0xB06A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03EE98: bcc.w $3ee9e
pub fn i_03ee98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EE9E; return 10; }
    8
}
/// $03EE9C: movea.l a2, a0
pub fn i_03ee9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE9E;
    cpu.ir = 0x204A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03EE9E: move.l a0, (a7)
pub fn i_03ee9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA0;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EEA0: moveq #$0, d0
pub fn i_03eea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEA2: move.b #$1, d2
pub fn i_03eea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EEA6: move.l (a7)+, d1
pub fn i_03eea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA8;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EEA8: bpl.w $3eeb6
pub fn i_03eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEAC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3EEB6; return 10; }
    8
}
// ===== ClearC_03eeac  ($03EEAC, 6 B, thunks/ccr_helpers.c)
/// $03EEAC: andi.b #$fe, ccr
pub fn i_03eeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEB0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EEB0: rts
pub fn i_03eeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueRts_03eeb2  ($03EEB2, 4 B, pow/results_squadleader_03daa8.s)
/// $03EEB2: bra.w $3eec6
pub fn i_03eeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EEC6;
    10
}
// ===== Pow_RescueCreditP2_03eeb6  ($03EEB6, 16 B, pow/results_squadleader_03daa8.s)
/// $03EEB6: cmpi.l #$100440, d1
pub fn i_03eeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEBC;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $03EEBC: beq.w $3eec6
pub fn i_03eebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3EEC6; return 10; }
    8
}
/// $03EEC0: moveq #$1, d0
pub fn i_03eec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEC2;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEC2: move.b #$2, d2
pub fn i_03eec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEC6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== SetC_03eec6  ($03EEC6, 6 B, thunks/ccr_helpers.c)
/// $03EEC6: ori.b #$1, ccr
pub fn i_03eec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EECA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $03EECA: rts
pub fn i_03eeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EECC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_CountIfPendingB_03eecc  ($03EECC, 10 B, pow/results_squadleader_03daa8.s)
/// $03EECC: tst.b $106ed2.l
pub fn i_03eecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EED2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $03EED2: bne.w $3eedc
pub fn i_03eed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EED6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EEDC; return 10; }
    8
}
// ===== ClearC_03eed6  ($03EED6, 6 B, thunks/ccr_helpers.c)
/// $03EED6: andi.b #$fe, ccr
pub fn i_03eed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEDA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EEDA: rts
pub fn i_03eeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueCreditB_03eedc  ($03EEDC, 44 B, pow/results_squadleader_03daa8.s)
/// $03EEDC: move.l #$ffffffff, -(a7)
pub fn i_03eedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEE2;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $03EEE2: moveq #$0, d0
pub fn i_03eee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEE4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEE4: jsr $5e3a2.l
pub fn i_03eee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EEEA);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EEEA: bcc.w $3eefc
pub fn i_03eeea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEFC; return 10; }
    8
}
/// $03EEEE: lea.l $3ef9a(pc), a1
pub fn i_03eeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EF9A;
    16
}
/// $03EEF2: bsr.w $3ef5c
pub fn i_03eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEF6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EEF6);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EEF6: bcc.w $3eefc
pub fn i_03eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEFC; return 10; }
    8
}
/// $03EEFA: move.l a0, (a7)
pub fn i_03eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEFC;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EEFC: moveq #$0, d0
pub fn i_03eefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEFE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEFE: move.b #$1, d2
pub fn i_03eefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF02;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EF02: move.l (a7)+, d1
pub fn i_03ef02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF04;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EF04: bpl.w $3ef0e
pub fn i_03ef04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF08;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3EF0E; return 10; }
    8
}
// ===== ClearC_03ef08  ($03EF08, 6 B, thunks/ccr_helpers.c)
/// $03EF08: andi.b #$fe, ccr
pub fn i_03ef08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF0C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF0C: rts
pub fn i_03ef0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_03ef0e  ($03EF0E, 6 B, thunks/ccr_helpers.c)
/// $03EF0E: ori.b #$1, ccr
pub fn i_03ef0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF12;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $03EF12: rts
pub fn i_03ef12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_CountIfPendingC_03ef14  ($03EF14, 10 B, pow/results_squadleader_03daa8.s)
/// $03EF14: tst.b $106ed2.l
pub fn i_03ef14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF1A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $03EF1A: bne.w $3ef24
pub fn i_03ef1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EF24; return 10; }
    8
}
// ===== ClearC_03ef1e  ($03EF1E, 6 B, thunks/ccr_helpers.c)
/// $03EF1E: andi.b #$fe, ccr
pub fn i_03ef1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF22;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF22: rts
pub fn i_03ef22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueCreditC_03ef24  ($03EF24, 44 B, pow/results_squadleader_03daa8.s)
/// $03EF24: move.l #$ffffffff, -(a7)
pub fn i_03ef24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF2A;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $03EF2A: moveq #$1, d0
pub fn i_03ef2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF2C;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EF2C: jsr $5e3a2.l
pub fn i_03ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EF32);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EF32: bcc.w $3ef44
pub fn i_03ef32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EF44; return 10; }
    8
}
/// $03EF36: lea.l $3ef9a(pc), a1
pub fn i_03ef36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EF9A;
    16
}
/// $03EF3A: bsr.w $3ef5c
pub fn i_03ef3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF3E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EF3E);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EF3E: bcc.w $3ef44
pub fn i_03ef3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EF44; return 10; }
    8
}
/// $03EF42: move.l a0, (a7)
pub fn i_03ef42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF44;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EF44: moveq #$1, d0
pub fn i_03ef44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF46;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EF46: move.b #$2, d2
pub fn i_03ef46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF4A;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EF4A: move.l (a7)+, d1
pub fn i_03ef4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF4C;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EF4C: bpl.w $3ef56
pub fn i_03ef4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF50;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3EF56; return 10; }
    8
}
// ===== ClearC_03ef50  ($03EF50, 6 B, thunks/ccr_helpers.c)
/// $03EF50: andi.b #$fe, ccr
pub fn i_03ef50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF54;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF54: rts
pub fn i_03ef54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_03ef56  ($03EF56, 6 B, thunks/ccr_helpers.c)
/// $03EF56: ori.b #$1, ccr
pub fn i_03ef56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF5A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $03EF5A: rts
pub fn i_03ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RectOverlap_03ef5c  ($03EF5C, 56 B, pow/results_squadleader_03daa8.s)
/// $03EF5C: btst.b #$3, $13(a0)
pub fn i_03ef5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF62;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03EF62: bne.w $3ef94
pub fn i_03ef62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EF94; return 10; }
    8
}
/// $03EF66: btst.b #$0, $13(a0)
pub fn i_03ef66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF6C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03EF6C: bne.w $3ef94
pub fn i_03ef6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EF94; return 10; }
    8
}
/// $03EF70: move.w $22(a0), d0
pub fn i_03ef70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF74;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EF74: sub.w $22(a6), d0
pub fn i_03ef74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF78;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EF78: add.w (a1), d0
pub fn i_03ef78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF7A;
    cpu.ir = 0xD051;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03EF7A: cmp.w $2(a1), d0
pub fn i_03ef7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF7E;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03EF7E: bcc.w $3ef92
pub fn i_03ef7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EF92; return 10; }
    8
}
/// $03EF82: move.w $24(a0), d0
pub fn i_03ef82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF86;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EF86: sub.w $24(a6), d0
pub fn i_03ef86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF8A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EF8A: add.w $4(a1), d0
pub fn i_03ef8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF8E;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EF8E: cmp.w $6(a1), d0
pub fn i_03ef8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF92;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03EF92: rts
pub fn i_03ef92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_03ef94  ($03EF94, 6 B, thunks/ccr_helpers.c)
/// $03EF94: andi.b #$fe, ccr
pub fn i_03ef94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF98;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF98: rts
pub fn i_03ef98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_AddRescued_03ef9a  ($03EF9A, 52 B, pow/results_squadleader_03daa8.s)
/// $03EF9A: ori.b #$20, (a0)
pub fn i_03ef9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF9E;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x20) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03EF9E: ori.b #$38, (a0)
pub fn i_03ef9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFA2;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x38) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03EFA2: movem.w d0, -(a7)
pub fn i_03efa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFA6;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $03EFA6: jsr $5e3a2.l
pub fn i_03efa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EFAC);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EFAC: movem.w (a7)+, d0
pub fn i_03efac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFB0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $03EFB0: bcc.w $3efcc
pub fn i_03efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EFCC; return 10; }
    8
}
/// $03EFB4: tst.b d0
pub fn i_03efb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFB6;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $03EFB6: bne.w $3efc4
pub fn i_03efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EFC4; return 10; }
    8
}
/// $03EFBA: lea.l $106f4c.l, a1
pub fn i_03efba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x106F4C;
    20
}
/// $03EFC0: bra.w $3efca
pub fn i_03efc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EFCA;
    10
}
/// $03EFC4: lea.l $106f4e.l, a1
pub fn i_03efc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFCA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x106F4E;
    20
}
/// $03EFCA: addq.w #$1, (a1)
pub fn i_03efca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFCC;
    cpu.ir = 0x5251;
    let ad1 = cpu.a[1];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $03EFCC: rts
pub fn i_03efcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_OffworldFree_03efce  ($03EFCE, 22 B, pow/results_squadleader_03daa8.s)
/// $03EFCE: lea.l $3efe4(pc), a0
pub fn i_03efce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFD2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3EFE4;
    16
}
/// $03EFD2: jsr $5dd56.l
pub fn i_03efd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EFD8);
    cpu.pc = 0x5DD56;
    16
}
/// $03EFD8: bcc.w $3efe2
pub fn i_03efd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EFE2; return 10; }
    8
}
/// $03EFDC: jmp $518.l
pub fn i_03efdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFE2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03EFE2: rts
pub fn i_03efe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_HitboxHdr_03efe4  ($03EFE4, 22 B, pow/results_squadleader_03daa8.s)
/// $03EFE4: dc.w $ffd8
pub fn i_03efe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFE6;
    cpu.ir = 0xFFD8;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $03EFEC: move.b $9b(a6), d0
pub fn i_03efec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFF0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03EFF0: move.w #$9c, d1
pub fn i_03eff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFF4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EFF4: jmp $9a7cc.l
pub fn i_03eff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFFA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x9A7CC;
    8
}
/// $03F626: addi.w #$24, $24(a6)
pub fn i_03f626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F62C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x24, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03F62C: rts
pub fn i_03f62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F62E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_SubX36_03f62e  ($03F62E, 8 B, pow/results_squadleader_03daa8.s)
/// $03F62E: subi.w #$24, $24(a6)
pub fn i_03f62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F634;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x24, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03F634: rts
pub fn i_03f634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F636;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03F9B8: tst.b $9a(a6)
pub fn i_03f9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9BC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03F9BC: beq.w $3f9c6
pub fn i_03f9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3F9C6; return 10; }
    8
}
/// $03F9C0: bset.b #$0, $3a(a6)
pub fn i_03f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9C6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03F9C6: move.w #$182, d1
pub fn i_03f9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x182;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03F9CA: jsr $236e.l
pub fn i_03f9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3F9D0);
    cpu.pc = 0x236E;
    16
}
/// $03F9D0: move.w #$8000, d0
pub fn i_03f9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03F9D4: jsr $28134.l
pub fn i_03f9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3F9DA);
    cpu.pc = 0x28134;
    16
}
/// $03F9DA: andi.w #$ffe3, $38(a6)
pub fn i_03f9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9E0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03F9E0: ori.w #$18, $38(a6)
pub fn i_03f9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9E6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03F9E6: clr.w $28(a6)
pub fn i_03f9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9EA: clr.w $2a(a6)
pub fn i_03f9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9EE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9EE: clr.w $2c(a6)
pub fn i_03f9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9F2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9F2: clr.w $2e(a6)
pub fn i_03f9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9F6: jsr $27f60.l
pub fn i_03f9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3F9FC);
    cpu.pc = 0x27F60;
    16
}
/// $03F9FC: scc.b $70(a6)
pub fn i_03f9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA00;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $03FA00: lea.l $776e2.l, a1
pub fn i_03fa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA06;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
// ===== JsrAbsThunk_03fa06  ($03FA06, 8 B, thunks/jsr_abs_thunks.c)
/// $03FA06: jsr $4ae.l
pub fn i_03fa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FA0C);
    cpu.pc = 0x4AE;
    16
}
/// $03FA0C: rts
pub fn i_03fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03FC38: bsr.w $3f9b8
pub fn i_03fc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC3C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FC3C);
    cpu.pc = 0x3F9B8;
    18
}
/// $03FC3C: lea.l $3f292(pc), a0
pub fn i_03fc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC40;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F292;
    16
}
/// $03FC40: jsr $28cd4.l
pub fn i_03fc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC46);
    cpu.pc = 0x28CD4;
    16
}
/// $03FC46: lea.l $3fc4c(pc), a1
pub fn i_03fc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FC4C;
    16
}
/// $03FC4A: move.l a1, (a6)
pub fn i_03fc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC4C: jsr $3fece(pc)
pub fn i_03fc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3FC50);
    cpu.pc = 0x3FECE;
    16
}
/// $03FC50: jsr $28d70.l
pub fn i_03fc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC56);
    cpu.pc = 0x28D70;
    16
}
/// $03FC56: jsr $2870a.l
pub fn i_03fc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC5C);
    cpu.pc = 0x2870A;
    16
}
/// $03FC5C: bcc.w $3fc66
pub fn i_03fc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FC66; return 10; }
    8
}
/// $03FC60: lea.l $3fc6a(pc), a1
pub fn i_03fc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FC6A;
    16
}
/// $03FC64: move.l a1, (a6)
pub fn i_03fc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC66: jmp $3efce(pc)
pub fn i_03fc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC6A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_Freed_03fc6a  ($03FC6A, 58 B, pow/results_squadleader_03daa8.s)
/// $03FC6A: lea.l $3f39e(pc), a0
pub fn i_03fc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC6E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F39E;
    16
}
/// $03FC6E: jsr $28cd4.l
pub fn i_03fc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC74);
    cpu.pc = 0x28CD4;
    16
}
/// $03FC74: lea.l $3fc7a(pc), a1
pub fn i_03fc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FC7A;
    16
}
/// $03FC78: move.l a1, (a6)
pub fn i_03fc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC7A: jsr $2783a.l
pub fn i_03fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC80);
    cpu.pc = 0x2783A;
    16
}
/// $03FC80: jsr $28d70.l
pub fn i_03fc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC86);
    cpu.pc = 0x28D70;
    16
}
/// $03FC86: bcc.w $3fca0
pub fn i_03fc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FCA0; return 10; }
    8
}
/// $03FC8A: lea.l $3ff14(pc), a1
pub fn i_03fc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF14;
    16
}
/// $03FC8E: move.l a1, (a6)
pub fn i_03fc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC90: lea.l $3fca4(pc), a1
pub fn i_03fc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FCA4;
    16
}
/// $03FC94: jsr $6fe.l
pub fn i_03fc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC9A);
    cpu.pc = 0x6FE;
    16
}
/// $03FC9A: jsr $5dd02.l
pub fn i_03fc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCA0);
    cpu.pc = 0x5DD02;
    16
}
/// $03FCA0: jmp $3efce(pc)
pub fn i_03fca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCA4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_FreedCheer_03fca4  ($03FCA4, 20 B, pow/results_squadleader_03daa8.s)
/// $03FCA4: move.w #$182, d1
pub fn i_03fca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCA8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x182;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FCA8: jsr $236e.l
pub fn i_03fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCAE);
    cpu.pc = 0x236E;
    16
}
/// $03FCAE: lea.l $3f3c2(pc), a0
pub fn i_03fcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCB2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F3C2;
    16
}
/// $03FCB2: jsr $28cd4.l
pub fn i_03fcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCB8);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_03fcb8  ($03FCB8, 8 B, kernel/task_handlers.c)
/// $03FCB8: lea.l $3fcc0(pc), a1
pub fn i_03fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FCC0;
    16
}
/// $03FCBC: move.l a1, (a6)
pub fn i_03fcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FCBE: rts
pub fn i_03fcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_FreedWait_03fcc0  ($03FCC0, 24 B, pow/results_squadleader_03daa8.s)
/// $03FCC0: jsr $2783a.l
pub fn i_03fcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCC6);
    cpu.pc = 0x2783A;
    16
}
/// $03FCC6: jsr $28d70.l
pub fn i_03fcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCCC);
    cpu.pc = 0x28D70;
    16
}
/// $03FCCC: bcc.w $3fcd6
pub fn i_03fccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FCD6; return 10; }
    8
}
/// $03FCD0: jmp $518.l
pub fn i_03fcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCD6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03FCD6: rts
pub fn i_03fcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_Tied_03fcd8  ($03FCD8, 106 B, pow/results_squadleader_03daa8.s)
/// $03FCD8: bsr.w $3f9b8
pub fn i_03fcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCDC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FCDC);
    cpu.pc = 0x3F9B8;
    18
}
/// $03FCDC: lea.l $3f0fa(pc), a0
pub fn i_03fcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCE0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F0FA;
    16
}
/// $03FCE0: jsr $28cd4.l
pub fn i_03fce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCE6);
    cpu.pc = 0x28CD4;
    16
}
/// $03FCE6: moveq #$0, d7
pub fn i_03fce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCE8;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03FCE8: move.b $99(a6), d7
pub fn i_03fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCEC;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03FCEC: cmpi.b #$4, d7
pub fn i_03fcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCF0;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $03FCF0: bcs.w $3fcf8
pub fn i_03fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCF4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3FCF8; return 10; }
    8
}
/// $03FCF4: move.w #$3, d7
pub fn i_03fcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCF8;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FCF8: add.w d7, d7
pub fn i_03fcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCFA;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $03FCFA: add.w d7, d7
pub fn i_03fcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCFC;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $03FCFC: lea.l $3fc28(pc), a0
pub fn i_03fcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD00;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3FC28;
    16
}
/// $03FD00: adda.w d7, a0
pub fn i_03fd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD02;
    cpu.ir = 0xD0C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $03FD02: move.l (a0), $48(a6)
pub fn i_03fd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD06;
    cpu.ir = 0x2D50;
    let v2 = bus.read32(cpu.a[0]);
    let ad3 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $03FD06: lea.l $3fd98(pc), a1
pub fn i_03fd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FD98;
    16
}
/// $03FD0A: jsr $4ae.l
pub fn i_03fd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD10);
    cpu.pc = 0x4AE;
    16
}
/// $03FD10: move.b $99(a6), $5c(a0)
pub fn i_03fd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD16;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03FD16: subq.b #$1, $99(a6)
pub fn i_03fd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD1A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03FD1A: bcc.b $3fd06
pub fn i_03fd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD1C;
    cpu.ir = 0x64EA;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FD06; return 10; }
    8
}
/// $03FD1C: lea.l $3fd22(pc), a1
pub fn i_03fd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FD22;
    16
}
/// $03FD20: move.l a1, (a6)
pub fn i_03fd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FD22: jsr $27cee.l
pub fn i_03fd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD28);
    cpu.pc = 0x27CEE;
    16
}
/// $03FD28: jsr $28d70.l
pub fn i_03fd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD2E);
    cpu.pc = 0x28D70;
    16
}
/// $03FD2E: jsr $2870a.l
pub fn i_03fd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD34);
    cpu.pc = 0x2870A;
    16
}
/// $03FD34: bcc.w $3fd3e
pub fn i_03fd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FD3E; return 10; }
    8
}
/// $03FD38: lea.l $3fd42(pc), a1
pub fn i_03fd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FD42;
    16
}
/// $03FD3C: move.l a1, (a6)
pub fn i_03fd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FD3E: jmp $3efce(pc)
pub fn i_03fd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD42;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_TiedStruggle_03fd42  ($03FD42, 44 B, pow/results_squadleader_03daa8.s)
/// $03FD42: lea.l $3f190(pc), a0
pub fn i_03fd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD46;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F190;
    16
}
/// $03FD46: jsr $28cd4.l
pub fn i_03fd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD4C);
    cpu.pc = 0x28CD4;
    16
}
/// $03FD4C: lea.l $3fd52(pc), a1
pub fn i_03fd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FD52;
    16
}
/// $03FD50: move.l a1, (a6)
pub fn i_03fd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FD52: jsr $3fece(pc)
pub fn i_03fd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3FD56);
    cpu.pc = 0x3FECE;
    16
}
/// $03FD56: tst.b $70(a6)
pub fn i_03fd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD5A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FD5A: beq.w $3fd64
pub fn i_03fd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3FD64; return 10; }
    8
}
/// $03FD5E: lea.l $3fd6e(pc), a1
pub fn i_03fd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FD6E;
    16
}
/// $03FD62: move.l a1, (a6)
pub fn i_03fd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FD64: jsr $28d70.l
pub fn i_03fd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD6A);
    cpu.pc = 0x28D70;
    16
}
/// $03FD6A: jmp $3efce(pc)
pub fn i_03fd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD6E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_TiedBreak_03fd6e  ($03FD6E, 42 B, pow/results_squadleader_03daa8.s)
/// $03FD6E: lea.l $3f20a(pc), a0
pub fn i_03fd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD72;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F20A;
    16
}
/// $03FD72: jsr $28cd4.l
pub fn i_03fd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD78);
    cpu.pc = 0x28CD4;
    16
}
/// $03FD78: lea.l $3fd7e(pc), a1
pub fn i_03fd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FD7E;
    16
}
/// $03FD7C: move.l a1, (a6)
pub fn i_03fd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FD7E: jsr $2783a.l
pub fn i_03fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD84);
    cpu.pc = 0x2783A;
    16
}
/// $03FD84: jsr $28d70.l
pub fn i_03fd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FD8A);
    cpu.pc = 0x28D70;
    16
}
/// $03FD8A: bcc.w $3fd94
pub fn i_03fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FD94; return 10; }
    8
}
/// $03FD8E: lea.l $3ff14(pc), a1
pub fn i_03fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF14;
    16
}
/// $03FD92: move.l a1, (a6)
pub fn i_03fd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FD94: jmp $3efce(pc)
pub fn i_03fd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD98;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_TiedCheer_03fd98  ($03FD98, 48 B, pow/results_squadleader_03daa8.s)
/// $03FD98: move.w #$182, d1
pub fn i_03fd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FD9C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x182;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FD9C: jsr $236e.l
pub fn i_03fd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FDA2);
    cpu.pc = 0x236E;
    16
}
/// $03FDA2: moveq #$0, d0
pub fn i_03fda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDA4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03FDA4: move.b $5c(a6), d0
pub fn i_03fda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDA8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03FDA8: lsl.w #$4, d0
pub fn i_03fda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDAA;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $03FDAA: move.w d0, d1
pub fn i_03fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDAC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FDAC: add.w d1, d0
pub fn i_03fdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDAE;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $03FDAE: add.w d1, d0
pub fn i_03fdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDB0;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $03FDB0: movea.l $c(a6), a0
pub fn i_03fdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDB4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03FDB4: move.w $32(a0), $32(a6)
pub fn i_03fdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDBA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03FDBA: move.w $22(a0), $22(a6)
pub fn i_03fdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDC0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03FDC0: add.w $24(a0), d0
pub fn i_03fdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDC4;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03FDC4: move.w d0, $24(a6)
pub fn i_03fdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDC8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_03fdc8  ($03FDC8, 8 B, kernel/task_handlers.c)
/// $03FDC8: lea.l $3fdd0(pc), a1
pub fn i_03fdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FDD0;
    16
}
/// $03FDCC: move.l a1, (a6)
pub fn i_03fdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FDCE: rts
pub fn i_03fdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_WaitRider_03fdd0  ($03FDD0, 74 B, pow/results_squadleader_03daa8.s)
/// $03FDD0: jsr $2783a.l
pub fn i_03fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FDD6);
    cpu.pc = 0x2783A;
    16
}
/// $03FDD6: movea.l $c(a6), a0
pub fn i_03fdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDDA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03FDDA: cmpi.l #$52a, (a0)
pub fn i_03fdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDE0;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $03FDE0: bne.w $3fdea
pub fn i_03fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FDEA; return 10; }
    8
}
/// $03FDE4: jmp $518.l
pub fn i_03fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDEA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03FDEA: btst.b #$3, $13(a0)
pub fn i_03fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDF0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03FDF0: beq.w $3fdfa
pub fn i_03fdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3FDFA; return 10; }
    8
}
/// $03FDF4: lea.l $3fe1a(pc), a1
pub fn i_03fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE1A;
    16
}
/// $03FDF8: move.l a1, (a6)
pub fn i_03fdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FDFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FDFA: move.l $5c(a0), $3c(a6)
pub fn i_03fdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE00;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03FE00: jsr $5ca2a.l
pub fn i_03fe00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE06);
    cpu.pc = 0x5CA2A;
    16
}
/// $03FE06: move.w $22(a6), d0
pub fn i_03fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE0A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03FE0A: addi.w #$10, d0
pub fn i_03fe0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE0E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03FE0E: bpl.w $3fe18
pub fn i_03fe0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE12;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3FE18; return 10; }
    8
}
/// $03FE12: jmp $518.l
pub fn i_03fe12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE18;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03FE18: rts
pub fn i_03fe18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RiderGreet_03fe1a  ($03FE1A, 64 B, pow/results_squadleader_03daa8.s)
/// $03FE1A: tst.b $5c(a6)
pub fn i_03fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE1E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FE1E: bne.w $3fe30
pub fn i_03fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE22;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FE30; return 10; }
    8
}
/// $03FE22: lea.l $3f216(pc), a0
pub fn i_03fe22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE26;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F216;
    16
}
/// $03FE26: jsr $28cd4.l
pub fn i_03fe26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE2C);
    cpu.pc = 0x28CD4;
    16
}
/// $03FE2C: bra.w $3fe3a
pub fn i_03fe2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE30;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FE3A;
    10
}
/// $03FE30: lea.l $3f25e(pc), a0
pub fn i_03fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE34;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F25E;
    16
}
/// $03FE34: jsr $28cd4.l
pub fn i_03fe34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE3A);
    cpu.pc = 0x28CD4;
    16
}
/// $03FE3A: lea.l $3fe40(pc), a1
pub fn i_03fe3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE40;
    16
}
/// $03FE3E: move.l a1, (a6)
pub fn i_03fe3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FE40: jsr $2783a.l
pub fn i_03fe40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE46);
    cpu.pc = 0x2783A;
    16
}
/// $03FE46: jsr $28d70.l
pub fn i_03fe46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE4C);
    cpu.pc = 0x28D70;
    16
}
/// $03FE4C: bcc.w $3fe56
pub fn i_03fe4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FE56; return 10; }
    8
}
/// $03FE50: jmp $518.l
pub fn i_03fe50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE56;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03FE56: jmp $3efce(pc)
pub fn i_03fe56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE5A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_Entry_03fe5a  ($03FE5A, 4 B, pow/results_squadleader_03daa8.s)
/// $03FE5A: bsr.w $3f9b8
pub fn i_03fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE5E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FE5E);
    cpu.pc = 0x3F9B8;
    18
}
// ===== SetTaskHandler_03fe5e  ($03FE5E, 8 B, kernel/task_handlers.c)
/// $03FE5E: lea.l $3fe66(pc), a1
pub fn i_03fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE66;
    16
}
/// $03FE62: move.l a1, (a6)
pub fn i_03fe62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FE64: rts
pub fn i_03fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_Idle_03fe66  ($03FE66, 54 B, pow/results_squadleader_03daa8.s)
/// $03FE66: lea.l $3f070(pc), a0
pub fn i_03fe66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE6A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F070;
    16
}
/// $03FE6A: jsr $28cd4.l
pub fn i_03fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE70);
    cpu.pc = 0x28CD4;
    16
}
/// $03FE70: clr.w $28(a6)
pub fn i_03fe70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE74;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03FE74: clr.w $2a(a6)
pub fn i_03fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE78;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03FE78: lea.l $3fe7e(pc), a1
pub fn i_03fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE7E;
    16
}
/// $03FE7C: move.l a1, (a6)
pub fn i_03fe7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FE7E: jsr $3fece(pc)
pub fn i_03fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3FE82);
    cpu.pc = 0x3FECE;
    16
}
/// $03FE82: jsr $28d70.l
pub fn i_03fe82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE88);
    cpu.pc = 0x28D70;
    16
}
/// $03FE88: jsr $2870a.l
pub fn i_03fe88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE8E);
    cpu.pc = 0x2870A;
    16
}
/// $03FE8E: bcc.w $3fe98
pub fn i_03fe8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FE98; return 10; }
    8
}
/// $03FE92: lea.l $3fe9c(pc), a1
pub fn i_03fe92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE9C;
    16
}
/// $03FE96: move.l a1, (a6)
pub fn i_03fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FE98: jmp $3efce(pc)
pub fn i_03fe98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE9C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_IdleWave_03fe9c  ($03FE9C, 42 B, pow/results_squadleader_03daa8.s)
/// $03FE9C: lea.l $3f0cc(pc), a0
pub fn i_03fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEA0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F0CC;
    16
}
/// $03FEA0: jsr $28cd4.l
pub fn i_03fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FEA6);
    cpu.pc = 0x28CD4;
    16
}
/// $03FEA6: lea.l $3feac(pc), a1
pub fn i_03fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FEAC;
    16
}
/// $03FEAA: move.l a1, (a6)
pub fn i_03feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FEAC: jsr $28364.l
pub fn i_03feac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FEB2);
    cpu.pc = 0x28364;
    16
}
/// $03FEB2: jsr $28d70.l
pub fn i_03feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FEB8);
    cpu.pc = 0x28D70;
    16
}
/// $03FEB8: bcc.w $3fec2
pub fn i_03feb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FEC2; return 10; }
    8
}
/// $03FEBC: lea.l $3ff14(pc), a1
pub fn i_03febc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF14;
    16
}
/// $03FEC0: move.l a1, (a6)
pub fn i_03fec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FEC2: jmp $3efce(pc)
pub fn i_03fec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEC6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_EntryB_03fec6  ($03FEC6, 8 B, pow/results_squadleader_03daa8.s)
/// $03FEC6: bsr.w $3f9b8
pub fn i_03fec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FECA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FECA);
    cpu.pc = 0x3F9B8;
    18
}
/// $03FECA: bra.w $3ff14
pub fn i_03feca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FECE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FF14;
    10
}
// ===== Pow_Physics_03fece  ($03FECE, 62 B, pow/results_squadleader_03daa8.s)
/// $03FECE: jsr $27f08.l
pub fn i_03fece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FED4);
    cpu.pc = 0x27F08;
    16
}
/// $03FED4: bcc.w $3fee0
pub fn i_03fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FED8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FEE0; return 10; }
    8
}
/// $03FED8: move.b d3, $70(a6)
pub fn i_03fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEDC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FEDC: bra.w $3fef6
pub fn i_03fedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FEF6;
    10
}
/// $03FEE0: cmp.b $70(a6), d0
pub fn i_03fee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEE4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $03FEE4: bne.w $3fef0
pub fn i_03fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FEF0; return 10; }
    8
}
/// $03FEE8: move.b d3, $70(a6)
pub fn i_03fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEEC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FEEC: bra.w $3fef6
pub fn i_03feec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FEF6;
    10
}
/// $03FEF0: move.b #$ff, $70(a6)
pub fn i_03fef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FEF6: tst.b $70(a6)
pub fn i_03fef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEFA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FEFA: bne.w $3ff0c
pub fn i_03fefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF0C; return 10; }
    8
}
/// $03FEFE: jsr $28364.l
pub fn i_03fefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF04);
    cpu.pc = 0x28364;
    16
}
/// $03FF04: scs.b $70(a6)
pub fn i_03ff04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF08;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $03FF08: bra.w $3ff12
pub fn i_03ff08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FF12;
    10
}
// ===== JsrAbsThunk_03ff0c  ($03FF0C, 8 B, thunks/jsr_abs_thunks.c)
/// $03FF0C: jsr $28292.l
pub fn i_03ff0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF12);
    cpu.pc = 0x28292;
    16
}
/// $03FF12: rts
pub fn i_03ff12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_Despawn_03ff14  ($03FF14, 16 B, pow/results_squadleader_03daa8.s)
/// $03FF14: jsr $13600.l
pub fn i_03ff14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF1A);
    cpu.pc = 0x13600;
    16
}
/// $03FF1A: move.w #$181, d1
pub fn i_03ff1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF1E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x181;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FF1E: jsr $236e.l
pub fn i_03ff1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF24);
    cpu.pc = 0x236E;
    16
}
// ===== Pow_PickWalk_03ff24  ($03FF24, 62 B, pow/results_squadleader_03daa8.s)
/// $03FF24: clr.b $70(a6)
pub fn i_03ff24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF28;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03FF28: lea.l $3f01c(pc), a0
pub fn i_03ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF2C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F01C;
    16
}
/// $03FF2C: move.l a0, $48(a6)
pub fn i_03ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF30;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03FF30: bclr.b #$3, $13(a6)
pub fn i_03ff30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF36;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF36: bclr.b #$0, $13(a6)
pub fn i_03ff36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF3C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF3C: move.b $98(a6), d0
pub fn i_03ff3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03FF40: beq.w $3ff5c
pub fn i_03ff40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF44;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3FF5C; return 10; }
    8
}
/// $03FF44: cmpi.b #$1, d0
pub fn i_03ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF48;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $03FF48: bne.w $3ff56
pub fn i_03ff48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF56; return 10; }
    8
}
/// $03FF4C: bclr.b #$0, $3a(a6)
pub fn i_03ff4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF52;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF52: bra.w $3ff5c
pub fn i_03ff52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF56;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FF5C;
    10
}
/// $03FF56: bset.b #$0, $3a(a6)
pub fn i_03ff56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF5C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF5C: move.w #$f, $5c(a6)
pub fn i_03ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Pow_Walk_03ff62  ($03FF62, 178 B, pow/results_squadleader_03daa8.s)
/// $03FF62: lea.l $3f40a(pc), a0
pub fn i_03ff62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF66;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F40A;
    16
}
/// $03FF66: jsr $28cd4.l
pub fn i_03ff66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF6C);
    cpu.pc = 0x28CD4;
    16
}
/// $03FF6C: move.w #$180, d0
pub fn i_03ff6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FF70: btst.b #$0, $3a(a6)
pub fn i_03ff70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF76;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03FF76: bne.w $3ff7c
pub fn i_03ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF7C; return 10; }
    8
}
/// $03FF7A: neg.w d0
pub fn i_03ff7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF7C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03FF7C: move.w d0, $28(a6)
pub fn i_03ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF80;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03FF80: lea.l $3ff86(pc), a1
pub fn i_03ff80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF86;
    16
}
/// $03FF84: move.l a1, (a6)
pub fn i_03ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FF86: bsr.w $3fece
pub fn i_03ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF8A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FF8A);
    cpu.pc = 0x3FECE;
    18
}
/// $03FF8A: tst.b $70(a6)
pub fn i_03ff8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF8E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FF8E: bne.w $3ff9c
pub fn i_03ff8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF9C; return 10; }
    8
}
/// $03FF92: lea.l $40014(pc), a1
pub fn i_03ff92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40014;
    16
}
/// $03FF96: move.l a1, (a6)
pub fn i_03ff96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FF98: bra.w $3ffbc
pub fn i_03ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFBC;
    10
}
/// $03FF9C: btst.b #$5, $5a(a6)
pub fn i_03ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFA2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03FFA2: bne.w $3ffb6
pub fn i_03ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FFB6; return 10; }
    8
}
/// $03FFA6: tst.b $98(a6)
pub fn i_03ffa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFAA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FFAA: bne.w $3ffbc
pub fn i_03ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FFBC; return 10; }
    8
}
/// $03FFAE: subq.w #$1, $5c(a6)
pub fn i_03ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFB2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03FFB2: bcc.w $3ffbc
pub fn i_03ffb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FFBC; return 10; }
    8
}
/// $03FFB6: lea.l $40054(pc), a1
pub fn i_03ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40054;
    16
}
/// $03FFBA: move.l a1, (a6)
pub fn i_03ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FFBC: jsr $770cc.l
pub fn i_03ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FFC2);
    cpu.pc = 0x770CC;
    16
}
/// $03FFC2: tst.w d0
pub fn i_03ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFC4;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $03FFC4: bmi.w $3ffe4
pub fn i_03ffc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFC8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x3FFE4; return 10; }
    8
}
/// $03FFC8: move.b d0, $72(a6)
pub fn i_03ffc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFCC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FFCC: andi.b #$1, d1
pub fn i_03ffcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFD0;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03FFD0: beq.w $3ffde
pub fn i_03ffd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3FFDE; return 10; }
    8
}
/// $03FFD4: lea.l $4008c(pc), a1
pub fn i_03ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4008C;
    16
}
/// $03FFD8: move.l a1, (a6)
pub fn i_03ffd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FFDA: bra.w $3ffe4
pub fn i_03ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFE4;
    10
}
/// $03FFDE: lea.l $401a8(pc), a1
pub fn i_03ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x401A8;
    16
}
/// $03FFE2: move.l a1, (a6)
pub fn i_03ffe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FFE4: jsr $28d70.l
pub fn i_03ffe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FFEA);
    cpu.pc = 0x28D70;
    16
}
/// $03FFEA: lea.l $3ef9a(pc), a1
pub fn i_03ffea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EF9A;
    16
}
/// $03FFEE: bsr.w $3ee48
pub fn i_03ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFF2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FFF2);
    cpu.pc = 0x3EE48;
    18
}
/// $03FFF2: bcc.w $40000
pub fn i_03fff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40000; return 10; }
    8
}
/// $03FFF6: move.b d0, $71(a6)
pub fn i_03fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFFA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FFFA: lea.l $40268(pc), a1
pub fn i_03fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40268;
    16
}
/// $03FFFE: move.l a1, (a6)
pub fn i_03fffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40000;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x03EBA0, i_03eba0),
    (0x03EBA6, i_03eba6),
    (0x03EBAA, i_03ebaa),
    (0x03EBB0, i_03ebb0),
    (0x03EBB4, i_03ebb4),
    (0x03EBB8, i_03ebb8),
    (0x03EBBA, i_03ebba),
    (0x03EBBC, i_03ebbc),
    (0x03EBC2, i_03ebc2),
    (0x03EBC6, i_03ebc6),
    (0x03EBCC, i_03ebcc),
    (0x03EBD0, i_03ebd0),
    (0x03EBD6, i_03ebd6),
    (0x03EBDA, i_03ebda),
    (0x03EBDE, i_03ebde),
    (0x03EBE2, i_03ebe2),
    (0x03EBE8, i_03ebe8),
    (0x03EBEE, i_03ebee),
    (0x03EBF2, i_03ebf2),
    (0x03EBF6, i_03ebf6),
    (0x03EBF8, i_03ebf8),
    (0x03EBFC, i_03ebfc),
    (0x03EC00, i_03ec00),
    (0x03EC04, i_03ec04),
    (0x03EC0A, i_03ec0a),
    (0x03EC10, i_03ec10),
    (0x03EC14, i_03ec14),
    (0x03EC16, i_03ec16),
    (0x03EC18, i_03ec18),
    (0x03EC1E, i_03ec1e),
    (0x03EC24, i_03ec24),
    (0x03EC2A, i_03ec2a),
    (0x03EC2E, i_03ec2e),
    (0x03EC30, i_03ec30),
    (0x03EC34, i_03ec34),
    (0x03EC3A, i_03ec3a),
    (0x03EC3E, i_03ec3e),
    (0x03EC44, i_03ec44),
    (0x03EC48, i_03ec48),
    (0x03EC4C, i_03ec4c),
    (0x03EC4E, i_03ec4e),
    (0x03EC50, i_03ec50),
    (0x03EC56, i_03ec56),
    (0x03EC5A, i_03ec5a),
    (0x03EC60, i_03ec60),
    (0x03EC64, i_03ec64),
    (0x03EC6A, i_03ec6a),
    (0x03EC6E, i_03ec6e),
    (0x03EC72, i_03ec72),
    (0x03EC76, i_03ec76),
    (0x03EC7C, i_03ec7c),
    (0x03EC82, i_03ec82),
    (0x03EC86, i_03ec86),
    (0x03EC8A, i_03ec8a),
    (0x03EC8C, i_03ec8c),
    (0x03EC90, i_03ec90),
    (0x03EC94, i_03ec94),
    (0x03EC98, i_03ec98),
    (0x03EC9E, i_03ec9e),
    (0x03ECA4, i_03eca4),
    (0x03ECA8, i_03eca8),
    (0x03ECAA, i_03ecaa),
    (0x03ECAC, i_03ecac),
    (0x03ECB0, i_03ecb0),
    (0x03ECB4, i_03ecb4),
    (0x03ECB8, i_03ecb8),
    (0x03ECBC, i_03ecbc),
    (0x03ECC0, i_03ecc0),
    (0x03ECC6, i_03ecc6),
    (0x03ECCA, i_03ecca),
    (0x03ECCE, i_03ecce),
    (0x03ECD2, i_03ecd2),
    (0x03ECD6, i_03ecd6),
    (0x03ECDA, i_03ecda),
    (0x03ECE0, i_03ece0),
    (0x03ECE4, i_03ece4),
    (0x03ECE8, i_03ece8),
    (0x03ECEC, i_03ecec),
    (0x03ECF0, i_03ecf0),
    (0x03ECF4, i_03ecf4),
    (0x03ECFA, i_03ecfa),
    (0x03ECFE, i_03ecfe),
    (0x03ED02, i_03ed02),
    (0x03ED06, i_03ed06),
    (0x03ED0A, i_03ed0a),
    (0x03ED0E, i_03ed0e),
    (0x03ED14, i_03ed14),
    (0x03ED18, i_03ed18),
    (0x03ED1C, i_03ed1c),
    (0x03ED20, i_03ed20),
    (0x03ED24, i_03ed24),
    (0x03ED28, i_03ed28),
    (0x03ED2E, i_03ed2e),
    (0x03ED32, i_03ed32),
    (0x03ED36, i_03ed36),
    (0x03ED3A, i_03ed3a),
    (0x03ED3E, i_03ed3e),
    (0x03ED42, i_03ed42),
    (0x03ED48, i_03ed48),
    (0x03ED4C, i_03ed4c),
    (0x03ED50, i_03ed50),
    (0x03ED54, i_03ed54),
    (0x03ED58, i_03ed58),
    (0x03ED5C, i_03ed5c),
    (0x03ED62, i_03ed62),
    (0x03ED66, i_03ed66),
    (0x03ED6A, i_03ed6a),
    (0x03ED6E, i_03ed6e),
    (0x03ED72, i_03ed72),
    (0x03ED76, i_03ed76),
    (0x03ED7C, i_03ed7c),
    (0x03ED80, i_03ed80),
    (0x03ED84, i_03ed84),
    (0x03ED88, i_03ed88),
    (0x03ED8C, i_03ed8c),
    (0x03ED90, i_03ed90),
    (0x03ED96, i_03ed96),
    (0x03ED9A, i_03ed9a),
    (0x03ED9E, i_03ed9e),
    (0x03EDA2, i_03eda2),
    (0x03EDA6, i_03eda6),
    (0x03EDAA, i_03edaa),
    (0x03EDB0, i_03edb0),
    (0x03EDB4, i_03edb4),
    (0x03EDB8, i_03edb8),
    (0x03EDBC, i_03edbc),
    (0x03EDC0, i_03edc0),
    (0x03EDC4, i_03edc4),
    (0x03EDCA, i_03edca),
    (0x03EDD0, i_03edd0),
    (0x03EDD2, i_03edd2),
    (0x03EDD8, i_03edd8),
    (0x03EDDC, i_03eddc),
    (0x03EDE0, i_03ede0),
    (0x03EDE4, i_03ede4),
    (0x03EDE8, i_03ede8),
    (0x03EDEE, i_03edee),
    (0x03EDF2, i_03edf2),
    (0x03EDF4, i_03edf4),
    (0x03EDFA, i_03edfa),
    (0x03EDFE, i_03edfe),
    (0x03EE02, i_03ee02),
    (0x03EE06, i_03ee06),
    (0x03EE0A, i_03ee0a),
    (0x03EE0C, i_03ee0c),
    (0x03EE10, i_03ee10),
    (0x03EE16, i_03ee16),
    (0x03EE1A, i_03ee1a),
    (0x03EE1C, i_03ee1c),
    (0x03EE1E, i_03ee1e),
    (0x03EE22, i_03ee22),
    (0x03EE26, i_03ee26),
    (0x03EE2A, i_03ee2a),
    (0x03EE2E, i_03ee2e),
    (0x03EE32, i_03ee32),
    (0x03EE34, i_03ee34),
    (0x03EE38, i_03ee38),
    (0x03EE3A, i_03ee3a),
    (0x03EE40, i_03ee40),
    (0x03EE46, i_03ee46),
    (0x03EE48, i_03ee48),
    (0x03EE4E, i_03ee4e),
    (0x03EE52, i_03ee52),
    (0x03EE56, i_03ee56),
    (0x03EE58, i_03ee58),
    (0x03EE5E, i_03ee5e),
    (0x03EE60, i_03ee60),
    (0x03EE66, i_03ee66),
    (0x03EE6A, i_03ee6a),
    (0x03EE6E, i_03ee6e),
    (0x03EE72, i_03ee72),
    (0x03EE74, i_03ee74),
    (0x03EE76, i_03ee76),
    (0x03EE7C, i_03ee7c),
    (0x03EE80, i_03ee80),
    (0x03EE84, i_03ee84),
    (0x03EE88, i_03ee88),
    (0x03EE8A, i_03ee8a),
    (0x03EE8E, i_03ee8e),
    (0x03EE90, i_03ee90),
    (0x03EE94, i_03ee94),
    (0x03EE98, i_03ee98),
    (0x03EE9C, i_03ee9c),
    (0x03EE9E, i_03ee9e),
    (0x03EEA0, i_03eea0),
    (0x03EEA2, i_03eea2),
    (0x03EEA6, i_03eea6),
    (0x03EEA8, i_03eea8),
    (0x03EEAC, i_03eeac),
    (0x03EEB0, i_03eeb0),
    (0x03EEB2, i_03eeb2),
    (0x03EEB6, i_03eeb6),
    (0x03EEBC, i_03eebc),
    (0x03EEC0, i_03eec0),
    (0x03EEC2, i_03eec2),
    (0x03EEC6, i_03eec6),
    (0x03EECA, i_03eeca),
    (0x03EECC, i_03eecc),
    (0x03EED2, i_03eed2),
    (0x03EED6, i_03eed6),
    (0x03EEDA, i_03eeda),
    (0x03EEDC, i_03eedc),
    (0x03EEE2, i_03eee2),
    (0x03EEE4, i_03eee4),
    (0x03EEEA, i_03eeea),
    (0x03EEEE, i_03eeee),
    (0x03EEF2, i_03eef2),
    (0x03EEF6, i_03eef6),
    (0x03EEFA, i_03eefa),
    (0x03EEFC, i_03eefc),
    (0x03EEFE, i_03eefe),
    (0x03EF02, i_03ef02),
    (0x03EF04, i_03ef04),
    (0x03EF08, i_03ef08),
    (0x03EF0C, i_03ef0c),
    (0x03EF0E, i_03ef0e),
    (0x03EF12, i_03ef12),
    (0x03EF14, i_03ef14),
    (0x03EF1A, i_03ef1a),
    (0x03EF1E, i_03ef1e),
    (0x03EF22, i_03ef22),
    (0x03EF24, i_03ef24),
    (0x03EF2A, i_03ef2a),
    (0x03EF2C, i_03ef2c),
    (0x03EF32, i_03ef32),
    (0x03EF36, i_03ef36),
    (0x03EF3A, i_03ef3a),
    (0x03EF3E, i_03ef3e),
    (0x03EF42, i_03ef42),
    (0x03EF44, i_03ef44),
    (0x03EF46, i_03ef46),
    (0x03EF4A, i_03ef4a),
    (0x03EF4C, i_03ef4c),
    (0x03EF50, i_03ef50),
    (0x03EF54, i_03ef54),
    (0x03EF56, i_03ef56),
    (0x03EF5A, i_03ef5a),
    (0x03EF5C, i_03ef5c),
    (0x03EF62, i_03ef62),
    (0x03EF66, i_03ef66),
    (0x03EF6C, i_03ef6c),
    (0x03EF70, i_03ef70),
    (0x03EF74, i_03ef74),
    (0x03EF78, i_03ef78),
    (0x03EF7A, i_03ef7a),
    (0x03EF7E, i_03ef7e),
    (0x03EF82, i_03ef82),
    (0x03EF86, i_03ef86),
    (0x03EF8A, i_03ef8a),
    (0x03EF8E, i_03ef8e),
    (0x03EF92, i_03ef92),
    (0x03EF94, i_03ef94),
    (0x03EF98, i_03ef98),
    (0x03EF9A, i_03ef9a),
    (0x03EF9E, i_03ef9e),
    (0x03EFA2, i_03efa2),
    (0x03EFA6, i_03efa6),
    (0x03EFAC, i_03efac),
    (0x03EFB0, i_03efb0),
    (0x03EFB4, i_03efb4),
    (0x03EFB6, i_03efb6),
    (0x03EFBA, i_03efba),
    (0x03EFC0, i_03efc0),
    (0x03EFC4, i_03efc4),
    (0x03EFCA, i_03efca),
    (0x03EFCC, i_03efcc),
    (0x03EFCE, i_03efce),
    (0x03EFD2, i_03efd2),
    (0x03EFD8, i_03efd8),
    (0x03EFDC, i_03efdc),
    (0x03EFE2, i_03efe2),
    (0x03EFE4, i_03efe4),
    (0x03EFEC, i_03efec),
    (0x03EFF0, i_03eff0),
    (0x03EFF4, i_03eff4),
    (0x03F626, i_03f626),
    (0x03F62C, i_03f62c),
    (0x03F62E, i_03f62e),
    (0x03F634, i_03f634),
    (0x03F9B8, i_03f9b8),
    (0x03F9BC, i_03f9bc),
    (0x03F9C0, i_03f9c0),
    (0x03F9C6, i_03f9c6),
    (0x03F9CA, i_03f9ca),
    (0x03F9D0, i_03f9d0),
    (0x03F9D4, i_03f9d4),
    (0x03F9DA, i_03f9da),
    (0x03F9E0, i_03f9e0),
    (0x03F9E6, i_03f9e6),
    (0x03F9EA, i_03f9ea),
    (0x03F9EE, i_03f9ee),
    (0x03F9F2, i_03f9f2),
    (0x03F9F6, i_03f9f6),
    (0x03F9FC, i_03f9fc),
    (0x03FA00, i_03fa00),
    (0x03FA06, i_03fa06),
    (0x03FA0C, i_03fa0c),
    (0x03FC38, i_03fc38),
    (0x03FC3C, i_03fc3c),
    (0x03FC40, i_03fc40),
    (0x03FC46, i_03fc46),
    (0x03FC4A, i_03fc4a),
    (0x03FC4C, i_03fc4c),
    (0x03FC50, i_03fc50),
    (0x03FC56, i_03fc56),
    (0x03FC5C, i_03fc5c),
    (0x03FC60, i_03fc60),
    (0x03FC64, i_03fc64),
    (0x03FC66, i_03fc66),
    (0x03FC6A, i_03fc6a),
    (0x03FC6E, i_03fc6e),
    (0x03FC74, i_03fc74),
    (0x03FC78, i_03fc78),
    (0x03FC7A, i_03fc7a),
    (0x03FC80, i_03fc80),
    (0x03FC86, i_03fc86),
    (0x03FC8A, i_03fc8a),
    (0x03FC8E, i_03fc8e),
    (0x03FC90, i_03fc90),
    (0x03FC94, i_03fc94),
    (0x03FC9A, i_03fc9a),
    (0x03FCA0, i_03fca0),
    (0x03FCA4, i_03fca4),
    (0x03FCA8, i_03fca8),
    (0x03FCAE, i_03fcae),
    (0x03FCB2, i_03fcb2),
    (0x03FCB8, i_03fcb8),
    (0x03FCBC, i_03fcbc),
    (0x03FCBE, i_03fcbe),
    (0x03FCC0, i_03fcc0),
    (0x03FCC6, i_03fcc6),
    (0x03FCCC, i_03fccc),
    (0x03FCD0, i_03fcd0),
    (0x03FCD6, i_03fcd6),
    (0x03FCD8, i_03fcd8),
    (0x03FCDC, i_03fcdc),
    (0x03FCE0, i_03fce0),
    (0x03FCE6, i_03fce6),
    (0x03FCE8, i_03fce8),
    (0x03FCEC, i_03fcec),
    (0x03FCF0, i_03fcf0),
    (0x03FCF4, i_03fcf4),
    (0x03FCF8, i_03fcf8),
    (0x03FCFA, i_03fcfa),
    (0x03FCFC, i_03fcfc),
    (0x03FD00, i_03fd00),
    (0x03FD02, i_03fd02),
    (0x03FD06, i_03fd06),
    (0x03FD0A, i_03fd0a),
    (0x03FD10, i_03fd10),
    (0x03FD16, i_03fd16),
    (0x03FD1A, i_03fd1a),
    (0x03FD1C, i_03fd1c),
    (0x03FD20, i_03fd20),
    (0x03FD22, i_03fd22),
    (0x03FD28, i_03fd28),
    (0x03FD2E, i_03fd2e),
    (0x03FD34, i_03fd34),
    (0x03FD38, i_03fd38),
    (0x03FD3C, i_03fd3c),
    (0x03FD3E, i_03fd3e),
    (0x03FD42, i_03fd42),
    (0x03FD46, i_03fd46),
    (0x03FD4C, i_03fd4c),
    (0x03FD50, i_03fd50),
    (0x03FD52, i_03fd52),
    (0x03FD56, i_03fd56),
    (0x03FD5A, i_03fd5a),
    (0x03FD5E, i_03fd5e),
    (0x03FD62, i_03fd62),
    (0x03FD64, i_03fd64),
    (0x03FD6A, i_03fd6a),
    (0x03FD6E, i_03fd6e),
    (0x03FD72, i_03fd72),
    (0x03FD78, i_03fd78),
    (0x03FD7C, i_03fd7c),
    (0x03FD7E, i_03fd7e),
    (0x03FD84, i_03fd84),
    (0x03FD8A, i_03fd8a),
    (0x03FD8E, i_03fd8e),
    (0x03FD92, i_03fd92),
    (0x03FD94, i_03fd94),
    (0x03FD98, i_03fd98),
    (0x03FD9C, i_03fd9c),
    (0x03FDA2, i_03fda2),
    (0x03FDA4, i_03fda4),
    (0x03FDA8, i_03fda8),
    (0x03FDAA, i_03fdaa),
    (0x03FDAC, i_03fdac),
    (0x03FDAE, i_03fdae),
    (0x03FDB0, i_03fdb0),
    (0x03FDB4, i_03fdb4),
    (0x03FDBA, i_03fdba),
    (0x03FDC0, i_03fdc0),
    (0x03FDC4, i_03fdc4),
    (0x03FDC8, i_03fdc8),
    (0x03FDCC, i_03fdcc),
    (0x03FDCE, i_03fdce),
    (0x03FDD0, i_03fdd0),
    (0x03FDD6, i_03fdd6),
    (0x03FDDA, i_03fdda),
    (0x03FDE0, i_03fde0),
    (0x03FDE4, i_03fde4),
    (0x03FDEA, i_03fdea),
    (0x03FDF0, i_03fdf0),
    (0x03FDF4, i_03fdf4),
    (0x03FDF8, i_03fdf8),
    (0x03FDFA, i_03fdfa),
    (0x03FE00, i_03fe00),
    (0x03FE06, i_03fe06),
    (0x03FE0A, i_03fe0a),
    (0x03FE0E, i_03fe0e),
    (0x03FE12, i_03fe12),
    (0x03FE18, i_03fe18),
    (0x03FE1A, i_03fe1a),
    (0x03FE1E, i_03fe1e),
    (0x03FE22, i_03fe22),
    (0x03FE26, i_03fe26),
    (0x03FE2C, i_03fe2c),
    (0x03FE30, i_03fe30),
    (0x03FE34, i_03fe34),
    (0x03FE3A, i_03fe3a),
    (0x03FE3E, i_03fe3e),
    (0x03FE40, i_03fe40),
    (0x03FE46, i_03fe46),
    (0x03FE4C, i_03fe4c),
    (0x03FE50, i_03fe50),
    (0x03FE56, i_03fe56),
    (0x03FE5A, i_03fe5a),
    (0x03FE5E, i_03fe5e),
    (0x03FE62, i_03fe62),
    (0x03FE64, i_03fe64),
    (0x03FE66, i_03fe66),
    (0x03FE6A, i_03fe6a),
    (0x03FE70, i_03fe70),
    (0x03FE74, i_03fe74),
    (0x03FE78, i_03fe78),
    (0x03FE7C, i_03fe7c),
    (0x03FE7E, i_03fe7e),
    (0x03FE82, i_03fe82),
    (0x03FE88, i_03fe88),
    (0x03FE8E, i_03fe8e),
    (0x03FE92, i_03fe92),
    (0x03FE96, i_03fe96),
    (0x03FE98, i_03fe98),
    (0x03FE9C, i_03fe9c),
    (0x03FEA0, i_03fea0),
    (0x03FEA6, i_03fea6),
    (0x03FEAA, i_03feaa),
    (0x03FEAC, i_03feac),
    (0x03FEB2, i_03feb2),
    (0x03FEB8, i_03feb8),
    (0x03FEBC, i_03febc),
    (0x03FEC0, i_03fec0),
    (0x03FEC2, i_03fec2),
    (0x03FEC6, i_03fec6),
    (0x03FECA, i_03feca),
    (0x03FECE, i_03fece),
    (0x03FED4, i_03fed4),
    (0x03FED8, i_03fed8),
    (0x03FEDC, i_03fedc),
    (0x03FEE0, i_03fee0),
    (0x03FEE4, i_03fee4),
    (0x03FEE8, i_03fee8),
    (0x03FEEC, i_03feec),
    (0x03FEF0, i_03fef0),
    (0x03FEF6, i_03fef6),
    (0x03FEFA, i_03fefa),
    (0x03FEFE, i_03fefe),
    (0x03FF04, i_03ff04),
    (0x03FF08, i_03ff08),
    (0x03FF0C, i_03ff0c),
    (0x03FF12, i_03ff12),
    (0x03FF14, i_03ff14),
    (0x03FF1A, i_03ff1a),
    (0x03FF1E, i_03ff1e),
    (0x03FF24, i_03ff24),
    (0x03FF28, i_03ff28),
    (0x03FF2C, i_03ff2c),
    (0x03FF30, i_03ff30),
    (0x03FF36, i_03ff36),
    (0x03FF3C, i_03ff3c),
    (0x03FF40, i_03ff40),
    (0x03FF44, i_03ff44),
    (0x03FF48, i_03ff48),
    (0x03FF4C, i_03ff4c),
    (0x03FF52, i_03ff52),
    (0x03FF56, i_03ff56),
    (0x03FF5C, i_03ff5c),
    (0x03FF62, i_03ff62),
    (0x03FF66, i_03ff66),
    (0x03FF6C, i_03ff6c),
    (0x03FF70, i_03ff70),
    (0x03FF76, i_03ff76),
    (0x03FF7A, i_03ff7a),
    (0x03FF7C, i_03ff7c),
    (0x03FF80, i_03ff80),
    (0x03FF84, i_03ff84),
    (0x03FF86, i_03ff86),
    (0x03FF8A, i_03ff8a),
    (0x03FF8E, i_03ff8e),
    (0x03FF92, i_03ff92),
    (0x03FF96, i_03ff96),
    (0x03FF98, i_03ff98),
    (0x03FF9C, i_03ff9c),
    (0x03FFA2, i_03ffa2),
    (0x03FFA6, i_03ffa6),
    (0x03FFAA, i_03ffaa),
    (0x03FFAE, i_03ffae),
    (0x03FFB2, i_03ffb2),
    (0x03FFB6, i_03ffb6),
    (0x03FFBA, i_03ffba),
    (0x03FFBC, i_03ffbc),
    (0x03FFC2, i_03ffc2),
    (0x03FFC4, i_03ffc4),
    (0x03FFC8, i_03ffc8),
    (0x03FFCC, i_03ffcc),
    (0x03FFD0, i_03ffd0),
    (0x03FFD4, i_03ffd4),
    (0x03FFD8, i_03ffd8),
    (0x03FFDA, i_03ffda),
    (0x03FFDE, i_03ffde),
    (0x03FFE2, i_03ffe2),
    (0x03FFE4, i_03ffe4),
    (0x03FFEA, i_03ffea),
    (0x03FFEE, i_03ffee),
    (0x03FFF2, i_03fff2),
    (0x03FFF6, i_03fff6),
    (0x03FFFA, i_03fffa),
    (0x03FFFE, i_03fffe),
];
