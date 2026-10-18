// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


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

pub const TABLE: &[(u32, crate::InsFn)] = &[
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
];
