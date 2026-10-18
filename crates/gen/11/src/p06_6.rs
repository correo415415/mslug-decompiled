// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $06A5BE: bcc.w $6a5c8
pub fn i_06a5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A5C8; return 10; }
    8
}
/// $06A5C2: lea.l $6a5cc(pc), a1
pub fn i_06a5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A5CC;
    16
}
/// $06A5C6: move.l a1, (a6)
pub fn i_06a5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A5C8: bra.w $6a772
pub fn i_06a5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A772;
    10
}
// ===== BazookaB_Alert_06a5cc  ($06A5CC, 106 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A5CC: tst.b $9e(a6)
pub fn i_06a5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5D0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A5D0: beq.w $6a6c2
pub fn i_06a5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A6C2; return 10; }
    8
}
/// $06A5D4: jsr $267e2.l
pub fn i_06a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A5DA);
    cpu.pc = 0x267E2;
    16
}
/// $06A5DA: lea.l $2bd8fa.l, a0
pub fn i_06a5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD8FA;
    20
}
/// $06A5E0: jsr $799de.l
pub fn i_06a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A5E6);
    cpu.pc = 0x799DE;
    16
}
/// $06A5E6: move.w d0, $70(a6)
pub fn i_06a5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5EA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06A5EA: lea.l $2ca84a.l, a0
pub fn i_06a5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CA84A;
    20
}
/// $06A5F0: jsr $28cd4.l
pub fn i_06a5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A5F6);
    cpu.pc = 0x28CD4;
    16
}
/// $06A5F6: lea.l $6a5fc(pc), a1
pub fn i_06a5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A5FC;
    16
}
/// $06A5FA: move.l a1, (a6)
pub fn i_06a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A5FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A5FC: jsr $6b232(pc)
pub fn i_06a5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A600;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A600);
    cpu.pc = 0x6B232;
    16
}
/// $06A600: jsr $28d70.l
pub fn i_06a600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A606;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A606);
    cpu.pc = 0x28D70;
    16
}
/// $06A606: subq.w #$1, $70(a6)
pub fn i_06a606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A60A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06A60A: cmpi.w #$0, $70(a6)
pub fn i_06a60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A610;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06A610: bgt.w $6a61a
pub fn i_06a610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A614;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6A61A; return 10; }
    8
}
/// $06A614: lea.l $6a6c2(pc), a1
pub fn i_06a614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A618;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A6C2;
    16
}
/// $06A618: move.l a1, (a6)
pub fn i_06a618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A61A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A61A: bra.w $6a772
pub fn i_06a61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A61E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A772;
    10
}
// ===== BazookaB_AttackLoop_06a636  ($06A636, 140 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A636: subq.b #$1, $74(a6)
pub fn i_06a636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A63A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A63A: cmpi.b #$0, $74(a6)
pub fn i_06a63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A640;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06A640: ble.w $6a4d6
pub fn i_06a640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A644;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6A4D6; return 10; }
    8
}
/// $06A644: clr.b $73(a6)
pub fn i_06a644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A648;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A648: move.b #$2, $72(a6)
pub fn i_06a648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A64E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A64E: lea.l $2bd878.l, a0
pub fn i_06a64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A654;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD878;
    20
}
/// $06A654: jsr $799de.l
pub fn i_06a654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A65A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A65A);
    cpu.pc = 0x799DE;
    16
}
/// $06A65A: move.w d0, $70(a6)
pub fn i_06a65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A65E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06A65E: lea.l $2cad10.l, a0
pub fn i_06a65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A664;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CAD10;
    20
}
/// $06A664: jsr $28cd4.l
pub fn i_06a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A66A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A66A);
    cpu.pc = 0x28CD4;
    16
}
/// $06A66A: lea.l $6a670(pc), a1
pub fn i_06a66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A66E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A670;
    16
}
/// $06A66E: move.l a1, (a6)
pub fn i_06a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A670;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A670: subq.w #$1, $70(a6)
pub fn i_06a670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A674;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06A674: cmpi.w #$0, $70(a6)
pub fn i_06a674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A67A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06A67A: bgt.w $6a684
pub fn i_06a67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A67E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6A684; return 10; }
    8
}
/// $06A67E: lea.l $6a636(pc), a1
pub fn i_06a67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A682;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A636;
    16
}
/// $06A682: move.l a1, (a6)
pub fn i_06a682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A684;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A684: jsr $6b232(pc)
pub fn i_06a684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A688;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A688);
    cpu.pc = 0x6B232;
    16
}
/// $06A688: bcc.w $6a698
pub fn i_06a688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A68C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A698; return 10; }
    8
}
/// $06A68C: eori.b #$1, $75(a6)
pub fn i_06a68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A692;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06A692: lea.l $6a568(pc), a1
pub fn i_06a692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A696;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A568;
    16
}
/// $06A696: move.l a1, (a6)
pub fn i_06a696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A698;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A698: jsr $28d70.l
pub fn i_06a698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A69E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A69E);
    cpu.pc = 0x28D70;
    16
}
/// $06A69E: tst.b $73(a6)
pub fn i_06a69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A6A2: beq.w $6a6b0
pub fn i_06a6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A6B0; return 10; }
    8
}
/// $06A6A6: clr.b $73(a6)
pub fn i_06a6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6AA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A6AA: move.b #$0, $72(a6)
pub fn i_06a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6B0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A6B0: jsr $6b27a(pc)
pub fn i_06a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A6B4);
    cpu.pc = 0x6B27A;
    16
}
/// $06A6B4: bcc.w $6a6be
pub fn i_06a6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A6BE; return 10; }
    8
}
/// $06A6B8: lea.l $6a568(pc), a1
pub fn i_06a6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A568;
    16
}
/// $06A6BC: move.l a1, (a6)
pub fn i_06a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A6BE: bra.w $6a772
pub fn i_06a6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A772;
    10
}
// ===== BazookaB_Reacquire_06a6c2  ($06A6C2, 30 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A6C2: jsr $267e2.l
pub fn i_06a6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A6C8);
    cpu.pc = 0x267E2;
    16
}
/// $06A6C8: clr.b $73(a6)
pub fn i_06a6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6CC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A6CC: lea.l $2bd97c.l, a0
pub fn i_06a6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD97C;
    20
}
/// $06A6D2: jsr $799de.l
pub fn i_06a6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A6D8);
    cpu.pc = 0x799DE;
    16
}
/// $06A6D8: move.b d0, $74(a6)
pub fn i_06a6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6DC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A6DC: bra.w $6a6ee
pub fn i_06a6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A6EE;
    10
}
// ===== BazookaB_Attack_06a6e0  ($06A6E0, 212 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A6E0: subq.b #$1, $74(a6)
pub fn i_06a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6E4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A6E4: cmpi.b #$0, $74(a6)
pub fn i_06a6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6EA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06A6EA: ble.w $6a752
pub fn i_06a6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6EE;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6A752; return 10; }
    8
}
/// $06A6EE: clr.b $73(a6)
pub fn i_06a6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6F2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A6F2: move.b #$2, $72(a6)
pub fn i_06a6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A6F8: lea.l $2bd878.l, a0
pub fn i_06a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD878;
    20
}
/// $06A6FE: jsr $799de.l
pub fn i_06a6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A704;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A704);
    cpu.pc = 0x799DE;
    16
}
/// $06A704: move.w d0, $70(a6)
pub fn i_06a704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A708;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06A708: lea.l $2cacc0.l, a0
pub fn i_06a708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A70E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CACC0;
    20
}
/// $06A70E: jsr $28cd4.l
pub fn i_06a70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A714);
    cpu.pc = 0x28CD4;
    16
}
/// $06A714: lea.l $6a71a(pc), a1
pub fn i_06a714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A718;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A71A;
    16
}
/// $06A718: move.l a1, (a6)
pub fn i_06a718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A71A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A71A: jsr $6b232(pc)
pub fn i_06a71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A71E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A71E);
    cpu.pc = 0x6B232;
    16
}
/// $06A71E: jsr $28d70.l
pub fn i_06a71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A724);
    cpu.pc = 0x28D70;
    16
}
/// $06A724: bcc.w $6a73a
pub fn i_06a724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A728;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A73A; return 10; }
    8
}
/// $06A728: tst.b $73(a6)
pub fn i_06a728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A72C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A72C: beq.w $6a73a
pub fn i_06a72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A730;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A73A; return 10; }
    8
}
/// $06A730: clr.b $73(a6)
pub fn i_06a730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A734;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A734: move.b #$0, $72(a6)
pub fn i_06a734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A73A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A73A: subq.w #$1, $70(a6)
pub fn i_06a73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A73E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06A73E: cmpi.w #$0, $70(a6)
pub fn i_06a73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A744;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06A744: bgt.w $6a74e
pub fn i_06a744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A748;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6A74E; return 10; }
    8
}
/// $06A748: lea.l $6a6e0(pc), a1
pub fn i_06a748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A74C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A6E0;
    16
}
/// $06A74C: move.l a1, (a6)
pub fn i_06a74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A74E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A74E: bra.w $6a772
pub fn i_06a74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A752;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A772;
    10
}
/// $06A752: lea.l $2bd7f6.l, a0
pub fn i_06a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A758;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD7F6;
    20
}
/// $06A758: jsr $799de.l
pub fn i_06a758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A75E);
    cpu.pc = 0x799DE;
    16
}
/// $06A75E: move.w d0, $70(a6)
pub fn i_06a75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A762;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06A762: move.l #$ffffffff, $90(a6)
pub fn i_06a762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A76A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06A76A: jsr $6b3a4(pc)
pub fn i_06a76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A76E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A76E);
    cpu.pc = 0x6B3A4;
    16
}
/// $06A76E: bra.w $6a568
pub fn i_06a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A772;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A568;
    10
}
/// $06A772: jsr $6b3a4(pc)
pub fn i_06a772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A776;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A776);
    cpu.pc = 0x6B3A4;
    16
}
/// $06A776: clr.b $78(a6)
pub fn i_06a776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A77A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A77A: jsr $2870a.l
pub fn i_06a77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A780);
    cpu.pc = 0x2870A;
    16
}
/// $06A780: bcc.w $6a79c
pub fn i_06a780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A784;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A79C; return 10; }
    8
}
/// $06A784: lea.l $5e766.l, a0
pub fn i_06a784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A78A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06A78A: jsr $5e770.l
pub fn i_06a78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A790);
    cpu.pc = 0x5E770;
    16
}
/// $06A790: bclr.b #$3, $13(a6)
pub fn i_06a790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A796;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06A796: move.b #$1, $78(a6)
pub fn i_06a796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A79C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A79C: jsr $28758.l
pub fn i_06a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A7A2);
    cpu.pc = 0x28758;
    16
}
/// $06A7A2: bcc.w $6a7ac
pub fn i_06a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A7AC; return 10; }
    8
}
/// $06A7A6: lea.l $6a3d6(pc), a1
pub fn i_06a7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A3D6;
    16
}
/// $06A7AA: move.l a1, (a6)
pub fn i_06a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A7AC: jsr $6b1e2(pc)
pub fn i_06a7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A7B0);
    cpu.pc = 0x6B1E2;
    16
}
/// $06A7B0: bcc.w $6a7ba
pub fn i_06a7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A7BA; return 10; }
    8
}
// ===== SetTaskHandler_06a7b4  ($06A7B4, 8 B, kernel/task_handlers.c)
/// $06A7B4: lea.l $6a41c(pc), a1
pub fn i_06a7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A41C;
    16
}
/// $06A7B8: move.l a1, (a6)
pub fn i_06a7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A7BA: rts
pub fn i_06a7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaCrew_Tmpl5C_06a7bc  ($06A7BC, 14 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A7BC: move.b #$1, $9c(a6)
pub fn i_06a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A7C2: clr.b $9e(a6)
pub fn i_06a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7C6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A7C6: bra.w $6a490
pub fn i_06a7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A490;
    10
}
// ===== BazookaCrew_Tmpl5D_06a7ca  ($06A7CA, 12 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A7CA: clr.b $9c(a6)
pub fn i_06a7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7CE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A7CE: clr.b $9e(a6)
pub fn i_06a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7D2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A7D2: bra.w $6a490
pub fn i_06a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A490;
    10
}
// ===== BazookaCrew_Init_06a7d6  ($06A7D6, 38 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A7D6: clr.b $7a(a6)
pub fn i_06a7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7DA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A7DA: bra.w $6a7e4
pub fn i_06a7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A7E4;
    10
}
/// $06A7E4: jsr $6b810(pc)
pub fn i_06a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A7E8);
    cpu.pc = 0x6B810;
    16
}
/// $06A7E8: clr.b $74(a6)
pub fn i_06a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7EC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A7EC: lea.l $2bd4ea.l, a0
pub fn i_06a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD4EA;
    20
}
/// $06A7F2: jsr $799de.l
pub fn i_06a7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A7F8);
    cpu.pc = 0x799DE;
    16
}
/// $06A7F8: move.w d0, $70(a6)
pub fn i_06a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A7FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== BazookaCrew_Idle_06a7fc  ($06A7FC, 108 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A7FC: jsr $5e0d4.l
pub fn i_06a7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A802);
    cpu.pc = 0x5E0D4;
    16
}
/// $06A802: move.l a0, $90(a6)
pub fn i_06a802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A806;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06A806: move.b #$4, $72(a6)
pub fn i_06a806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A80C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A80C: clr.b $73(a6)
pub fn i_06a80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A810;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A810: lea.l $2ca804.l, a0
pub fn i_06a810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A816;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CA804;
    20
}
/// $06A816: jsr $28cd4.l
pub fn i_06a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A81C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A81C);
    cpu.pc = 0x28CD4;
    16
}
/// $06A81C: lea.l $6a822(pc), a1
pub fn i_06a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A820;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A822;
    16
}
/// $06A820: move.l a1, (a6)
pub fn i_06a820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A822;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A822: jsr $6b87a(pc)
pub fn i_06a822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A826;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A826);
    cpu.pc = 0x6B87A;
    16
}
/// $06A826: jsr $28d70.l
pub fn i_06a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A82C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A82C);
    cpu.pc = 0x28D70;
    16
}
/// $06A82C: jsr $6b8bc(pc)
pub fn i_06a82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A830;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A830);
    cpu.pc = 0x6B8BC;
    16
}
/// $06A830: bcc.w $6a864
pub fn i_06a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A834;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A864; return 10; }
    8
}
/// $06A834: tst.b $74(a6)
pub fn i_06a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A838;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A838: beq.w $6a842
pub fn i_06a838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A83C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A842; return 10; }
    8
}
/// $06A83C: lea.l $6a878(pc), a1
pub fn i_06a83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A840;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A878;
    16
}
/// $06A840: move.l a1, (a6)
pub fn i_06a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A842;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A842: move.b #$1, $72(a6)
pub fn i_06a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A848;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A848: subq.w #$1, $70(a6)
pub fn i_06a848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A84C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06A84C: tst.b $73(a6)
pub fn i_06a84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A850;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A850: beq.w $6a864
pub fn i_06a850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A854;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A864; return 10; }
    8
}
/// $06A854: cmpi.w #$0, $70(a6)
pub fn i_06a854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A85A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06A85A: bgt.w $6a864
pub fn i_06a85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A85E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6A864; return 10; }
    8
}
/// $06A85E: lea.l $6a868(pc), a1
pub fn i_06a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A862;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A868;
    16
}
/// $06A862: move.l a1, (a6)
pub fn i_06a862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A864;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A864: bra.w $6a96c
pub fn i_06a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A868;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A96C;
    10
}
// ===== BazookaCrew_RngTimer_06a868  ($06A868, 16 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A868: lea.l $2bd5ee.l, a0
pub fn i_06a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A86E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD5EE;
    20
}
/// $06A86E: jsr $799de.l
pub fn i_06a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A874);
    cpu.pc = 0x799DE;
    16
}
/// $06A874: move.b d0, $74(a6)
pub fn i_06a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A878;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== BazookaCrew_Attack_06a878  ($06A878, 144 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A878: lea.l $2bd56c.l, a0
pub fn i_06a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A87E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD56C;
    20
}
/// $06A87E: jsr $799de.l
pub fn i_06a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A884);
    cpu.pc = 0x799DE;
    16
}
/// $06A884: move.w d0, $70(a6)
pub fn i_06a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A888;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06A888: clr.b $73(a6)
pub fn i_06a888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A88C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A88C: move.b #$2, $72(a6)
pub fn i_06a88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A892;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A892: lea.l $2cacc0.l, a0
pub fn i_06a892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A898;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CACC0;
    20
}
/// $06A898: jsr $28cd4.l
pub fn i_06a898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A89E);
    cpu.pc = 0x28CD4;
    16
}
/// $06A89E: lea.l $6a8a4(pc), a1
pub fn i_06a89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A8A4;
    16
}
/// $06A8A2: move.l a1, (a6)
pub fn i_06a8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A8A4: jsr $6b87a(pc)
pub fn i_06a8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A8A8);
    cpu.pc = 0x6B87A;
    16
}
/// $06A8A8: jsr $28d70.l
pub fn i_06a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A8AE);
    cpu.pc = 0x28D70;
    16
}
/// $06A8AE: bcc.w $6a8be
pub fn i_06a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A8BE; return 10; }
    8
}
/// $06A8B2: lea.l $2ca804.l, a0
pub fn i_06a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CA804;
    20
}
/// $06A8B8: jsr $28cd4.l
pub fn i_06a8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A8BE);
    cpu.pc = 0x28CD4;
    16
}
/// $06A8BE: subq.w #$1, $70(a6)
pub fn i_06a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8C2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06A8C2: cmpi.w #$0, $70(a6)
pub fn i_06a8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8C8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06A8C8: bgt.w $6a904
pub fn i_06a8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8CC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6A904; return 10; }
    8
}
/// $06A8CC: tst.b $73(a6)
pub fn i_06a8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8D0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06A8D0: beq.w $6a904
pub fn i_06a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A904; return 10; }
    8
}
/// $06A8D4: move.b #$4, $72(a6)
pub fn i_06a8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A8DA: lea.l $6a878(pc), a1
pub fn i_06a8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A878;
    16
}
/// $06A8DE: move.l a1, (a6)
pub fn i_06a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A8E0: subq.b #$1, $74(a6)
pub fn i_06a8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8E4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A8E4: cmpi.b #$0, $74(a6)
pub fn i_06a8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8EA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06A8EA: bgt.w $6a904
pub fn i_06a8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8EE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6A904; return 10; }
    8
}
/// $06A8EE: lea.l $2bd4ea.l, a0
pub fn i_06a8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD4EA;
    20
}
/// $06A8F4: jsr $799de.l
pub fn i_06a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A8FA);
    cpu.pc = 0x799DE;
    16
}
/// $06A8FA: move.w d0, $70(a6)
pub fn i_06a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A8FE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06A8FE: lea.l $6a7fc(pc), a1
pub fn i_06a8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A902;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A7FC;
    16
}
/// $06A902: move.l a1, (a6)
pub fn i_06a902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A904;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A904: bra.w $6a96c
pub fn i_06a904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A908;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A96C;
    10
}
// ===== BazookaCrew_Hurt_06a908  ($06A908, 54 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A908: move.b #$5, $72(a6)
pub fn i_06a908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A90E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A90E: lea.l $2cae00.l, a0
pub fn i_06a90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A914;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CAE00;
    20
}
/// $06A914: jsr $28cd4.l
pub fn i_06a914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A91A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A91A);
    cpu.pc = 0x28CD4;
    16
}
/// $06A91A: lea.l $6a920(pc), a1
pub fn i_06a91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A91E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A920;
    16
}
/// $06A91E: move.l a1, (a6)
pub fn i_06a91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A920;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A920: jsr $6b87a(pc)
pub fn i_06a920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A924;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A924);
    cpu.pc = 0x6B87A;
    16
}
/// $06A924: jsr $28d70.l
pub fn i_06a924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A92A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A92A);
    cpu.pc = 0x28D70;
    16
}
/// $06A92A: bcc.w $6a93a
pub fn i_06a92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A92E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A93A; return 10; }
    8
}
/// $06A92E: bclr.b #$3, $13(a6)
pub fn i_06a92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A934;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06A934: lea.l $6a7fc(pc), a1
pub fn i_06a934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A938;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A7FC;
    16
}
/// $06A938: move.l a1, (a6)
pub fn i_06a938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A93A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A93A: bra.w $6a9e2
pub fn i_06a93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A93E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A9E2;
    10
}
// ===== BazookaCrew_Die_06a93e  ($06A93E, 38 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A93E: move.b #$6, $72(a6)
pub fn i_06a93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A944;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A944: lea.l $2caf58.l, a0
pub fn i_06a944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A94A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CAF58;
    20
}
/// $06A94A: jsr $28cd4.l
pub fn i_06a94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A950);
    cpu.pc = 0x28CD4;
    16
}
/// $06A950: lea.l $6a956(pc), a1
pub fn i_06a950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A954;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A956;
    16
}
/// $06A954: move.l a1, (a6)
pub fn i_06a954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A956;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A956: jsr $6b87a(pc)
pub fn i_06a956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A95A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6A95A);
    cpu.pc = 0x6B87A;
    16
}
/// $06A95A: jsr $28d70.l
pub fn i_06a95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A960);
    cpu.pc = 0x28D70;
    16
}
/// $06A960: bcc.w $6a96a
pub fn i_06a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A964;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A96A; return 10; }
    8
}
// ===== SetTaskHandler_06a964  ($06A964, 8 B, kernel/task_handlers.c)
/// $06A964: lea.l $6a41c(pc), a1
pub fn i_06a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A41C;
    16
}
/// $06A968: move.l a1, (a6)
pub fn i_06a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A96A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A96A: rts
pub fn i_06a96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A96C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaCrew_HitTest_06a96c  ($06A96C, 144 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06A96C: clr.b $78(a6)
pub fn i_06a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A970;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06A970: movea.l $c(a6), a0
pub fn i_06a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A974;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06A974: cmpi.b #$ff, $20(a0)
pub fn i_06a974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A97A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $06A97A: bne.w $6a9a0
pub fn i_06a97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A97E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6A9A0; return 10; }
    8
}
/// $06A97E: btst.b #$0, $13(a6)
pub fn i_06a97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A984;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06A984: bne.w $6a9a0
pub fn i_06a984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A988;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6A9A0; return 10; }
    8
}
/// $06A988: addi.w #$20, $24(a6)
pub fn i_06a988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A98E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06A98E: lea.l $6ba34.l, a1
pub fn i_06a98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A994;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x6BA34;
    20
}
/// $06A994: move.l a1, (a6)
pub fn i_06a994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A996;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A996: move.b #$7, $72(a6)
pub fn i_06a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A99C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A99C: bra.w $6a9f2
pub fn i_06a99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6A9F2;
    10
}
/// $06A9A0: lea.l $5e766.l, a0
pub fn i_06a9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06A9A6: jsr $5e770.l
pub fn i_06a9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A9AC);
    cpu.pc = 0x5E770;
    16
}
/// $06A9AC: jsr $2870a.l
pub fn i_06a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A9B2);
    cpu.pc = 0x2870A;
    16
}
/// $06A9B2: bcc.w $6a9e2
pub fn i_06a9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A9E2; return 10; }
    8
}
/// $06A9B6: bclr.b #$3, $13(a6)
pub fn i_06a9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06A9BC: cmpi.b #$2, $58(a6)
pub fn i_06a9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9C2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06A9C2: beq.w $6a9d0
pub fn i_06a9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6A9D0; return 10; }
    8
}
/// $06A9C6: cmpi.b #$3, $58(a6)
pub fn i_06a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9CC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $06A9CC: bne.w $6a9e2
pub fn i_06a9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6A9E2; return 10; }
    8
}
/// $06A9D0: bset.b #$3, $13(a6)
pub fn i_06a9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9D6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06A9D6: lea.l $6a908(pc), a1
pub fn i_06a9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A908;
    16
}
/// $06A9DA: move.l a1, (a6)
pub fn i_06a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A9DC: move.b #$1, $78(a6)
pub fn i_06a9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9E2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06A9E2: jsr $28758.l
pub fn i_06a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A9E8);
    cpu.pc = 0x28758;
    16
}
/// $06A9E8: bcc.w $6a9f2
pub fn i_06a9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6A9F2; return 10; }
    8
}
/// $06A9EC: lea.l $6a93e(pc), a1
pub fn i_06a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A93E;
    16
}
/// $06A9F0: move.l a1, (a6)
pub fn i_06a9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06A9F2: jsr $5e45a.l
pub fn i_06a9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6A9F8);
    cpu.pc = 0x5E45A;
    16
}
/// $06A9F8: bcc.w $6aa02
pub fn i_06a9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A9FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AA02; return 10; }
    8
}
// ===== SetTaskHandler_06a9fc  ($06A9FC, 8 B, kernel/task_handlers.c)
/// $06A9FC: lea.l $6a93e(pc), a1
pub fn i_06a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A93E;
    16
}
/// $06AA00: move.l a1, (a6)
pub fn i_06aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AA02: rts
pub fn i_06aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AllyBazooka_Tmpl40_06aa04  ($06AA04, 88 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AA04: clr.b $9e(a6)
pub fn i_06aa04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA08;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AA08: clr.b $7a(a6)
pub fn i_06aa08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA0C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AA0C: jsr $6b810(pc)
pub fn i_06aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AA10);
    cpu.pc = 0x6B810;
    16
}
/// $06AA10: bra.w $6aa28
pub fn i_06aa10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA14;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AA28;
    10
}
/// $06AA28: move.b #$1, $98(a0)
pub fn i_06aa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA2E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AA2E: clr.b $9f(a0)
pub fn i_06aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA32;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AA32: move.b $98(a6), d0
pub fn i_06aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA36;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06AA36: andi.w #$ff, d0
pub fn i_06aa36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA3A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06AA3A: lsl.w #$4, d0
pub fn i_06aa3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA3C;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06AA3C: move.w #$110, d0
pub fn i_06aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA40;
    cpu.ir = 0x303C;
    let v1: u32 = 0x110;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06AA40: move.w d0, $98(a6)
pub fn i_06aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA44;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AA44: clr.b $75(a6)
pub fn i_06aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA48;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AA48: clr.b $74(a6)
pub fn i_06aa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA4C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AA4C: lea.l $2bd4ea.l, a0
pub fn i_06aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA52;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD4EA;
    20
}
/// $06AA52: jsr $799de.l
pub fn i_06aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AA58);
    cpu.pc = 0x799DE;
    16
}
/// $06AA58: move.w d0, $70(a6)
pub fn i_06aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA5C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== AllyBazooka_Advance_06aa5c  ($06AA5C, 110 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AA5C: jsr $5e0d4.l
pub fn i_06aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AA62);
    cpu.pc = 0x5E0D4;
    16
}
/// $06AA62: move.l a0, $90(a6)
pub fn i_06aa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA66;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06AA66: move.b #$4, $72(a6)
pub fn i_06aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA6C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AA6C: clr.b $73(a6)
pub fn i_06aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA70;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AA70: lea.l $2ca832.l, a0
pub fn i_06aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CA832;
    20
}
/// $06AA76: jsr $28cd4.l
pub fn i_06aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AA7C);
    cpu.pc = 0x28CD4;
    16
}
/// $06AA7C: lea.l $6aa82(pc), a1
pub fn i_06aa7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AA82;
    16
}
/// $06AA80: move.l a1, (a6)
pub fn i_06aa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AA82: jsr $2783a.l
pub fn i_06aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AA88);
    cpu.pc = 0x2783A;
    16
}
/// $06AA88: jsr $28d70.l
pub fn i_06aa88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AA8E);
    cpu.pc = 0x28D70;
    16
}
/// $06AA8E: tst.b $74(a6)
pub fn i_06aa8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA92;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06AA92: bne.w $6aa9c
pub fn i_06aa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6AA9C; return 10; }
    8
}
/// $06AA96: lea.l $6aada(pc), a1
pub fn i_06aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AADA;
    16
}
/// $06AA9A: move.l a1, (a6)
pub fn i_06aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AA9C: jsr $6b8bc(pc)
pub fn i_06aa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AAA0);
    cpu.pc = 0x6B8BC;
    16
}
/// $06AAA0: bcc.w $6aac6
pub fn i_06aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAA4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AAC6; return 10; }
    8
}
/// $06AAA4: move.b #$1, $72(a6)
pub fn i_06aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AAAA: subq.w #$1, $70(a6)
pub fn i_06aaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAAE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06AAAE: tst.b $73(a6)
pub fn i_06aaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAB2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06AAB2: beq.w $6aac6
pub fn i_06aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AAC6; return 10; }
    8
}
/// $06AAB6: cmpi.w #$0, $70(a6)
pub fn i_06aab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AABC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06AABC: bgt.w $6aac6
pub fn i_06aabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAC0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6AAC6; return 10; }
    8
}
/// $06AAC0: lea.l $6aaca(pc), a1
pub fn i_06aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AACA;
    16
}
/// $06AAC4: move.l a1, (a6)
pub fn i_06aac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AAC6: bra.w $6abe2
pub fn i_06aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AACA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6ABE2;
    10
}
// ===== AllyBazooka_PickBurst_06aaca  ($06AACA, 16 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AACA: lea.l $2bd5ee.l, a0
pub fn i_06aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD5EE;
    20
}
/// $06AAD0: jsr $799de.l
pub fn i_06aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AAD6);
    cpu.pc = 0x799DE;
    16
}
/// $06AAD6: move.b d0, $74(a6)
pub fn i_06aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AADA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== AllyBazooka_Attack_06aada  ($06AADA, 146 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AADA: lea.l $2bd56c.l, a0
pub fn i_06aada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD56C;
    20
}
/// $06AAE0: jsr $799de.l
pub fn i_06aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AAE6);
    cpu.pc = 0x799DE;
    16
}
/// $06AAE6: move.w d0, $70(a6)
pub fn i_06aae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAEA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AAEA: clr.b $73(a6)
pub fn i_06aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAEE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AAEE: move.b #$2, $72(a6)
pub fn i_06aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AAF4: lea.l $2cacc0.l, a0
pub fn i_06aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AAFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CACC0;
    20
}
/// $06AAFA: jsr $28cd4.l
pub fn i_06aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB00);
    cpu.pc = 0x28CD4;
    16
}
/// $06AB00: lea.l $6ab06(pc), a1
pub fn i_06ab00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AB06;
    16
}
/// $06AB04: move.l a1, (a6)
pub fn i_06ab04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AB06: jsr $2783a.l
pub fn i_06ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB0C);
    cpu.pc = 0x2783A;
    16
}
/// $06AB0C: jsr $28d70.l
pub fn i_06ab0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB12);
    cpu.pc = 0x28D70;
    16
}
/// $06AB12: bcc.w $6ab22
pub fn i_06ab12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AB22; return 10; }
    8
}
/// $06AB16: lea.l $2ca832.l, a0
pub fn i_06ab16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CA832;
    20
}
/// $06AB1C: jsr $28cd4.l
pub fn i_06ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB22);
    cpu.pc = 0x28CD4;
    16
}
/// $06AB22: subq.w #$1, $70(a6)
pub fn i_06ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB26;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06AB26: cmpi.w #$0, $70(a6)
pub fn i_06ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB2C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06AB2C: bgt.w $6ab68
pub fn i_06ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB30;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6AB68; return 10; }
    8
}
/// $06AB30: tst.b $73(a6)
pub fn i_06ab30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB34;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06AB34: beq.w $6ab68
pub fn i_06ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AB68; return 10; }
    8
}
/// $06AB38: move.b #$4, $72(a6)
pub fn i_06ab38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AB3E: lea.l $6aada(pc), a1
pub fn i_06ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AADA;
    16
}
/// $06AB42: move.l a1, (a6)
pub fn i_06ab42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AB44: subq.b #$1, $74(a6)
pub fn i_06ab44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB48;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AB48: cmpi.b #$0, $74(a6)
pub fn i_06ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB4E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06AB4E: bgt.w $6ab68
pub fn i_06ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB52;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6AB68; return 10; }
    8
}
/// $06AB52: lea.l $2bd4ea.l, a0
pub fn i_06ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD4EA;
    20
}
/// $06AB58: jsr $799de.l
pub fn i_06ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB5E);
    cpu.pc = 0x799DE;
    16
}
/// $06AB5E: move.w d0, $70(a6)
pub fn i_06ab5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB62;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AB62: lea.l $6aa5c(pc), a1
pub fn i_06ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AA5C;
    16
}
/// $06AB66: move.l a1, (a6)
pub fn i_06ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AB68: bra.w $6abe2
pub fn i_06ab68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6ABE2;
    10
}
// ===== AllyBazooka_Hurt_06ab6c  ($06AB6C, 56 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AB6C: move.b #$5, $72(a6)
pub fn i_06ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AB72: lea.l $2cae00.l, a0
pub fn i_06ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CAE00;
    20
}
/// $06AB78: jsr $28cd4.l
pub fn i_06ab78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB7E);
    cpu.pc = 0x28CD4;
    16
}
/// $06AB7E: lea.l $6ab84(pc), a1
pub fn i_06ab7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AB84;
    16
}
/// $06AB82: move.l a1, (a6)
pub fn i_06ab82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AB84: jsr $2783a.l
pub fn i_06ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB8A);
    cpu.pc = 0x2783A;
    16
}
/// $06AB8A: jsr $28d70.l
pub fn i_06ab8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AB90);
    cpu.pc = 0x28D70;
    16
}
/// $06AB90: bcc.w $6aba0
pub fn i_06ab90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ABA0; return 10; }
    8
}
/// $06AB94: bclr.b #$3, $13(a6)
pub fn i_06ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB9A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06AB9A: lea.l $6aa5c(pc), a1
pub fn i_06ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AB9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AA5C;
    16
}
/// $06AB9E: move.l a1, (a6)
pub fn i_06ab9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ABA0: bra.w $6ac28
pub fn i_06aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AC28;
    10
}
// ===== AllyBazooka_Die_06aba4  ($06ABA4, 54 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06ABA4: move.b #$6, $72(a6)
pub fn i_06aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06ABAA: lea.l $2caf58.l, a0
pub fn i_06abaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CAF58;
    20
}
/// $06ABB0: jsr $28cd4.l
pub fn i_06abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ABB6);
    cpu.pc = 0x28CD4;
    16
}
/// $06ABB6: lea.l $6abbc(pc), a1
pub fn i_06abb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ABBC;
    16
}
/// $06ABBA: move.l a1, (a6)
pub fn i_06abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ABBC: jsr $2783a.l
pub fn i_06abbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ABC2);
    cpu.pc = 0x2783A;
    16
}
/// $06ABC2: jsr $28d70.l
pub fn i_06abc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ABC8);
    cpu.pc = 0x28D70;
    16
}
/// $06ABC8: bcc.w $6abd2
pub fn i_06abc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABCC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ABD2; return 10; }
    8
}
/// $06ABCC: lea.l $6a41c(pc), a1
pub fn i_06abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A41C;
    16
}
/// $06ABD0: move.l a1, (a6)
pub fn i_06abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ABD2: jsr $6b1e2(pc)
pub fn i_06abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6ABD6);
    cpu.pc = 0x6B1E2;
    16
}
/// $06ABD6: bcc.w $6abe0
pub fn i_06abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ABE0; return 10; }
    8
}
// ===== SetTaskHandler_06abda  ($06ABDA, 8 B, kernel/task_handlers.c)
/// $06ABDA: lea.l $6a41c(pc), a1
pub fn i_06abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A41C;
    16
}
/// $06ABDE: move.l a1, (a6)
pub fn i_06abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ABE0: rts
pub fn i_06abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AllyBazooka_HitTest_06abe2  ($06ABE2, 114 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06ABE2: clr.b $78(a6)
pub fn i_06abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABE6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06ABE6: lea.l $5e766.l, a0
pub fn i_06abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06ABEC: jsr $5e770.l
pub fn i_06abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ABF2);
    cpu.pc = 0x5E770;
    16
}
/// $06ABF2: jsr $2870a.l
pub fn i_06abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ABF8);
    cpu.pc = 0x2870A;
    16
}
/// $06ABF8: bcc.w $6ac28
pub fn i_06abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ABFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AC28; return 10; }
    8
}
/// $06ABFC: bclr.b #$3, $13(a6)
pub fn i_06abfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC02;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06AC02: cmpi.b #$2, $58(a6)
pub fn i_06ac02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC08;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06AC08: beq.w $6ac16
pub fn i_06ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC0C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AC16; return 10; }
    8
}
/// $06AC0C: cmpi.b #$3, $58(a6)
pub fn i_06ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC12;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $06AC12: bne.w $6ac28
pub fn i_06ac12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6AC28; return 10; }
    8
}
/// $06AC16: bset.b #$3, $13(a6)
pub fn i_06ac16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC1C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06AC1C: lea.l $6ab6c(pc), a1
pub fn i_06ac1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AB6C;
    16
}
/// $06AC20: move.l a1, (a6)
pub fn i_06ac20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AC22: move.b #$1, $78(a6)
pub fn i_06ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AC28: jsr $28758.l
pub fn i_06ac28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AC2E);
    cpu.pc = 0x28758;
    16
}
/// $06AC2E: bcc.w $6ac38
pub fn i_06ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AC38; return 10; }
    8
}
/// $06AC32: lea.l $6aba4(pc), a1
pub fn i_06ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ABA4;
    16
}
/// $06AC36: move.l a1, (a6)
pub fn i_06ac36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AC38: movea.l $c(a6), a0
pub fn i_06ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC3C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06AC3C: cmpi.b #$ff, $20(a0)
pub fn i_06ac3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC42;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $06AC42: bne.w $6ac4c
pub fn i_06ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6AC4C; return 10; }
    8
}
/// $06AC46: lea.l $6aba4(pc), a1
pub fn i_06ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ABA4;
    16
}
/// $06AC4A: move.l a1, (a6)
pub fn i_06ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AC4C: jsr $6b1e2(pc)
pub fn i_06ac4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AC50);
    cpu.pc = 0x6B1E2;
    16
}
/// $06AC50: bcc.w $6ac5a
pub fn i_06ac50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AC5A; return 10; }
    8
}
// ===== SetTaskHandler_06ac54  ($06AC54, 8 B, kernel/task_handlers.c)
/// $06AC54: lea.l $6a41c(pc), a1
pub fn i_06ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A41C;
    16
}
/// $06AC58: move.l a1, (a6)
pub fn i_06ac58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AC5A: rts
pub fn i_06ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AllyBazooka_Spawner_Tmpl41_06ac5c  ($06AC5C, 70 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AC5C: clr.b $20(a6)
pub fn i_06ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC60;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AC60: lea.l $6aa04(pc), a1
pub fn i_06ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AA04;
    16
}
/// $06AC64: jsr $4ae.l
pub fn i_06ac64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AC6A);
    cpu.pc = 0x4AE;
    16
}
/// $06AC6A: jsr $5dd02.l
pub fn i_06ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AC70);
    cpu.pc = 0x5DD02;
    16
}
/// $06AC70: move.b #$12, $98(a0)
pub fn i_06ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC76;
    cpu.ir = 0x117C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AC76: move.l a0, $5c(a6)
pub fn i_06ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC7A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06AC7A: lea.l $6ac80(pc), a1
pub fn i_06ac7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AC80;
    16
}
/// $06AC7E: move.l a1, (a6)
pub fn i_06ac7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AC80: jsr $2783a.l
pub fn i_06ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AC86);
    cpu.pc = 0x2783A;
    16
}
/// $06AC86: movea.l $5c(a6), a0
pub fn i_06ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC8A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06AC8A: btst.b #$0, $13(a0)
pub fn i_06ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC90;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06AC90: beq.w $6ac9a
pub fn i_06ac90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AC9A; return 10; }
    8
}
/// $06AC94: lea.l $6acaa(pc), a1
pub fn i_06ac94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ACAA;
    16
}
/// $06AC98: move.l a1, (a6)
pub fn i_06ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AC9A: jsr $6b1e2(pc)
pub fn i_06ac9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AC9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AC9E);
    cpu.pc = 0x6B1E2;
    16
}
/// $06AC9E: bcc.w $6aca8
pub fn i_06ac9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ACA8; return 10; }
    8
}
// ===== SetTaskHandler_06aca2  ($06ACA2, 8 B, kernel/task_handlers.c)
/// $06ACA2: lea.l $6acaa(pc), a1
pub fn i_06aca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ACAA;
    16
}
/// $06ACA6: move.l a1, (a6)
pub fn i_06aca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ACA8: rts
pub fn i_06aca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06acaa  ($06ACAA, 8 B, kernel/jmp_to_scheduler.c)
/// $06ACAA: jmp $518.l
pub fn i_06acaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACB0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== BazookaWeapon_Child_06acb2  ($06ACB2, 614 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06ACB2: jsr $6b94c(pc)
pub fn i_06acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6ACB6);
    cpu.pc = 0x6B94C;
    16
}
/// $06ACB6: andi.b #$3, $98(a6)
pub fn i_06acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACBC;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06ACBC: move.b #$0, $72(a6)
pub fn i_06acbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06ACC2: tst.b $9d(a6)
pub fn i_06acc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACC6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06ACC6: bne.w $6aebc
pub fn i_06acc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6AEBC; return 10; }
    8
}
/// $06ACCA: move.w #$8, $34(a6)
pub fn i_06acca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACD0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06ACD0: lea.l $2cb082.l, a0
pub fn i_06acd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06ACD6: jsr $28cd4.l
pub fn i_06acd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ACDC);
    cpu.pc = 0x28CD4;
    16
}
/// $06ACDC: lea.l $6ace2(pc), a1
pub fn i_06acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ACE2;
    16
}
/// $06ACE0: move.l a1, (a6)
pub fn i_06ace0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ACE2: cmpi.b #$6, $72(a6)
pub fn i_06ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACE8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $06ACE8: beq.w $6af68
pub fn i_06ace8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF68; return 10; }
    8
}
/// $06ACEC: jsr $6b8ea(pc)
pub fn i_06acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6ACF0);
    cpu.pc = 0x6B8EA;
    16
}
/// $06ACF0: bcc.w $6acf6
pub fn i_06acf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ACF6; return 10; }
    8
}
/// $06ACF4: jmp (a1)
pub fn i_06acf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACF6;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06ACF6: jsr $6b550(pc)
pub fn i_06acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ACFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6ACFA);
    cpu.pc = 0x6B550;
    16
}
/// $06ACFA: jsr $28d70.l
pub fn i_06acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AD00);
    cpu.pc = 0x28D70;
    16
}
/// $06AD00: bcc.w $6ad10
pub fn i_06ad00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD04;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AD10; return 10; }
    8
}
/// $06AD04: lea.l $2cb082.l, a0
pub fn i_06ad04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AD0A: jsr $28cd4.l
pub fn i_06ad0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AD10);
    cpu.pc = 0x28CD4;
    16
}
/// $06AD10: bra.w $6af4e
pub fn i_06ad10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD14;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AF4E;
    10
}
/// $06AD14: clr.b $74(a6)
pub fn i_06ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD18;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AD18: movea.l $c(a6), a0
pub fn i_06ad18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD1C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06AD1C: move.l $90(a0), $90(a6)
pub fn i_06ad1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD22;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $06AD22: lea.l $2cb082.l, a0
pub fn i_06ad22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AD28: jsr $28cd4.l
pub fn i_06ad28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AD2E);
    cpu.pc = 0x28CD4;
    16
}
/// $06AD2E: lea.l $6ad34(pc), a1
pub fn i_06ad2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AD34;
    16
}
/// $06AD32: move.l a1, (a6)
pub fn i_06ad32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AD34: addq.b #$1, $74(a6)
pub fn i_06ad34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD38;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AD38: andi.b #$3, $74(a6)
pub fn i_06ad38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD3E;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06AD3E: bne.w $6ad68
pub fn i_06ad3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6AD68; return 10; }
    8
}
/// $06AD42: jsr $6b5a4(pc)
pub fn i_06ad42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AD46);
    cpu.pc = 0x6B5A4;
    16
}
/// $06AD46: movea.l $c(a6), a0
pub fn i_06ad46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD4A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06AD4A: clr.b $73(a0)
pub fn i_06ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD4E;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AD4E: jsr $6b5ea(pc)
pub fn i_06ad4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AD52);
    cpu.pc = 0x6B5EA;
    16
}
/// $06AD52: bcc.w $6ad68
pub fn i_06ad52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AD68; return 10; }
    8
}
/// $06AD56: jsr $6b6cc(pc)
pub fn i_06ad56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AD5A);
    cpu.pc = 0x6B6CC;
    16
}
/// $06AD5A: bcs.w $6ad68
pub fn i_06ad5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD5E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6AD68; return 10; }
    8
}
/// $06AD5E: movea.l $c(a6), a0
pub fn i_06ad5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD62;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06AD62: move.b #$1, $73(a0)
pub fn i_06ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD68;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AD68: cmpi.b #$6, $72(a6)
pub fn i_06ad68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD6E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $06AD6E: beq.w $6af68
pub fn i_06ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF68; return 10; }
    8
}
/// $06AD72: jsr $6b8ea(pc)
pub fn i_06ad72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AD76);
    cpu.pc = 0x6B8EA;
    16
}
/// $06AD76: bcc.w $6ad7c
pub fn i_06ad76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AD7C; return 10; }
    8
}
/// $06AD7A: jmp (a1)
pub fn i_06ad7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD7C;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06AD7C: jsr $6b550(pc)
pub fn i_06ad7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AD80);
    cpu.pc = 0x6B550;
    16
}
/// $06AD80: jsr $28d70.l
pub fn i_06ad80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AD86);
    cpu.pc = 0x28D70;
    16
}
/// $06AD86: bcc.w $6ad96
pub fn i_06ad86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AD96; return 10; }
    8
}
/// $06AD8A: lea.l $2cb082.l, a0
pub fn i_06ad8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD90;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AD90: jsr $28cd4.l
pub fn i_06ad90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AD96);
    cpu.pc = 0x28CD4;
    16
}
/// $06AD96: bra.w $6af4e
pub fn i_06ad96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AD9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AF4E;
    10
}
/// $06AD9A: lea.l $2cb1ac.l, a0
pub fn i_06ad9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB1AC;
    20
}
/// $06ADA0: move.w $34(a6), d0
pub fn i_06ada0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06ADA4: andi.w #$f, d0
pub fn i_06ada4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADA8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06ADA8: lsl.w #$2, d0
pub fn i_06ada8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADAA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06ADAA: movea.l (a0, d0.w), a0
pub fn i_06adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADAE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06ADAE: cmpa.l #$ffffffff, a0
pub fn i_06adae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADB4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06ADB4: beq.w $6adbe
pub fn i_06adb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADB8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6ADBE; return 10; }
    8
}
/// $06ADB8: jsr $28cd4.l
pub fn i_06adb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ADBE);
    cpu.pc = 0x28CD4;
    16
}
/// $06ADBE: jsr $6b6f2(pc)
pub fn i_06adbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6ADC2);
    cpu.pc = 0x6B6F2;
    16
}
/// $06ADC2: lea.l $6adc8(pc), a1
pub fn i_06adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ADC8;
    16
}
/// $06ADC6: move.l a1, (a6)
pub fn i_06adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ADC8: cmpi.b #$6, $72(a6)
pub fn i_06adc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADCE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $06ADCE: beq.w $6af68
pub fn i_06adce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF68; return 10; }
    8
}
/// $06ADD2: jsr $6b8ea(pc)
pub fn i_06add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6ADD6);
    cpu.pc = 0x6B8EA;
    16
}
/// $06ADD6: bcc.w $6addc
pub fn i_06add6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ADDC; return 10; }
    8
}
/// $06ADDA: jmp (a1)
pub fn i_06adda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADDC;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06ADDC: jsr $6b550(pc)
pub fn i_06addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6ADE0);
    cpu.pc = 0x6B550;
    16
}
/// $06ADE0: jsr $28d70.l
pub fn i_06ade0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ADE6);
    cpu.pc = 0x28D70;
    16
}
/// $06ADE6: bcc.w $6adf4
pub fn i_06ade6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ADF4; return 10; }
    8
}
/// $06ADEA: movea.l $c(a6), a0
pub fn i_06adea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADEE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06ADEE: move.b #$1, $73(a0)
pub fn i_06adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADF4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06ADF4: rts
pub fn i_06adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $06ADF6: clr.b $74(a6)
pub fn i_06adf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ADFA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06ADFA: lea.l $2cb082.l, a0
pub fn i_06adfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AE00: jsr $28cd4.l
pub fn i_06ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AE06);
    cpu.pc = 0x28CD4;
    16
}
/// $06AE06: lea.l $6ae0c(pc), a1
pub fn i_06ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AE0C;
    16
}
/// $06AE0A: move.l a1, (a6)
pub fn i_06ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AE0C: addq.b #$1, $74(a6)
pub fn i_06ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE10;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AE10: andi.b #$3, $74(a6)
pub fn i_06ae10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE16;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06AE16: bne.w $6ae42
pub fn i_06ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6AE42; return 10; }
    8
}
/// $06AE1A: move.w #$1, d1
pub fn i_06ae1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE1E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06AE1E: cmpi.w #$8, $34(a6)
pub fn i_06ae1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $06AE24: beq.w $6ae38
pub fn i_06ae24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AE38; return 10; }
    8
}
/// $06AE28: blt.w $6ae30
pub fn i_06ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE2C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6AE30; return 10; }
    8
}
/// $06AE2C: move.w #$ffff, d1
pub fn i_06ae2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE30;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06AE30: add.w d1, $34(a6)
pub fn i_06ae30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE34;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06AE34: bra.w $6ae42
pub fn i_06ae34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE38;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AE42;
    10
}
/// $06AE38: movea.l $c(a6), a0
pub fn i_06ae38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE3C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06AE3C: move.b #$1, $73(a0)
pub fn i_06ae3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE42;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06AE42: cmpi.b #$6, $72(a6)
pub fn i_06ae42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE48;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $06AE48: beq.w $6af68
pub fn i_06ae48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF68; return 10; }
    8
}
/// $06AE4C: jsr $6b8ea(pc)
pub fn i_06ae4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AE50);
    cpu.pc = 0x6B8EA;
    16
}
/// $06AE50: bcc.w $6ae56
pub fn i_06ae50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AE56; return 10; }
    8
}
/// $06AE54: jmp (a1)
pub fn i_06ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE56;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06AE56: jsr $6b550(pc)
pub fn i_06ae56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AE5A);
    cpu.pc = 0x6B550;
    16
}
/// $06AE5A: jsr $28d70.l
pub fn i_06ae5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AE60);
    cpu.pc = 0x28D70;
    16
}
/// $06AE60: bcc.w $6ae70
pub fn i_06ae60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE64;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AE70; return 10; }
    8
}
/// $06AE64: lea.l $2cb082.l, a0
pub fn i_06ae64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AE6A: jsr $28cd4.l
pub fn i_06ae6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AE70);
    cpu.pc = 0x28CD4;
    16
}
/// $06AE70: bra.w $6af4e
pub fn i_06ae70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE74;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AF4E;
    10
}
/// $06AE74: clr.b $74(a6)
pub fn i_06ae74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE78;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AE78: lea.l $2cb082.l, a0
pub fn i_06ae78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AE7E: jsr $28cd4.l
pub fn i_06ae7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AE84);
    cpu.pc = 0x28CD4;
    16
}
/// $06AE84: lea.l $6ae8a(pc), a1
pub fn i_06ae84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AE8A;
    16
}
/// $06AE88: move.l a1, (a6)
pub fn i_06ae88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AE8A: cmpi.b #$6, $72(a6)
pub fn i_06ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE90;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $06AE90: beq.w $6af68
pub fn i_06ae90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF68; return 10; }
    8
}
/// $06AE94: jsr $6b8ea(pc)
pub fn i_06ae94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AE98);
    cpu.pc = 0x6B8EA;
    16
}
/// $06AE98: bcc.w $6ae9e
pub fn i_06ae98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AE9E; return 10; }
    8
}
/// $06AE9C: jmp (a1)
pub fn i_06ae9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE9E;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06AE9E: jsr $6b550(pc)
pub fn i_06ae9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AEA2);
    cpu.pc = 0x6B550;
    16
}
/// $06AEA2: jsr $28d70.l
pub fn i_06aea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AEA8);
    cpu.pc = 0x28D70;
    16
}
/// $06AEA8: bcc.w $6aeb8
pub fn i_06aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AEB8; return 10; }
    8
}
/// $06AEAC: lea.l $2cb082.l, a0
pub fn i_06aeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AEB2: jsr $28cd4.l
pub fn i_06aeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AEB8);
    cpu.pc = 0x28CD4;
    16
}
/// $06AEB8: bra.w $6af4e
pub fn i_06aeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AF4E;
    10
}
/// $06AEBC: clr.b $74(a6)
pub fn i_06aebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEC0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06AEC0: move.w #$c, d0
pub fn i_06aec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEC4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06AEC4: tst.b $9c(a6)
pub fn i_06aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEC8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06AEC8: beq.w $6aed0
pub fn i_06aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AECC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AED0; return 10; }
    8
}
/// $06AECC: move.w #$4, d0
pub fn i_06aecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AED0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06AED0: move.w d0, $34(a6)
pub fn i_06aed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AED4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AED4: lea.l $2cb082.l, a0
pub fn i_06aed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AEDA: jsr $28cd4.l
pub fn i_06aeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AEE0);
    cpu.pc = 0x28CD4;
    16
}
/// $06AEE0: lea.l $6aee6(pc), a1
pub fn i_06aee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AEE6;
    16
}
/// $06AEE4: move.l a1, (a6)
pub fn i_06aee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AEE6: cmpi.b #$6, $72(a6)
pub fn i_06aee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEEC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $06AEEC: beq.w $6af68
pub fn i_06aeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF68; return 10; }
    8
}
/// $06AEF0: jsr $6b8ea(pc)
pub fn i_06aef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AEF4);
    cpu.pc = 0x6B8EA;
    16
}
/// $06AEF4: bcc.w $6aefa
pub fn i_06aef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AEFA; return 10; }
    8
}
/// $06AEF8: jmp (a1)
pub fn i_06aef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEFA;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06AEFA: jsr $6b550(pc)
pub fn i_06aefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AEFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AEFE);
    cpu.pc = 0x6B550;
    16
}
/// $06AEFE: jsr $28d70.l
pub fn i_06aefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AF04);
    cpu.pc = 0x28D70;
    16
}
/// $06AF04: bcc.w $6af14
pub fn i_06af04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AF14; return 10; }
    8
}
/// $06AF08: lea.l $2cb082.l, a0
pub fn i_06af08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF0E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB082;
    20
}
/// $06AF0E: jsr $28cd4.l
pub fn i_06af0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AF14);
    cpu.pc = 0x28CD4;
    16
}
/// $06AF14: bra.w $6af4e
pub fn i_06af14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF18;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AF4E;
    10
}
// ===== BazookaWeapon_State5_06af18  ($06AF18, 46 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AF18: lea.l $2cc1f0.l, a0
pub fn i_06af18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CC1F0;
    20
}
/// $06AF1E: jsr $28cd4.l
pub fn i_06af1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AF24);
    cpu.pc = 0x28CD4;
    16
}
/// $06AF24: lea.l $6af2a(pc), a1
pub fn i_06af24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AF2A;
    16
}
/// $06AF28: move.l a1, (a6)
pub fn i_06af28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AF2A: cmpi.b #$6, $72(a6)
pub fn i_06af2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF30;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $06AF30: beq.w $6af68
pub fn i_06af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF68; return 10; }
    8
}
/// $06AF34: jsr $6b8ea(pc)
pub fn i_06af34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AF38);
    cpu.pc = 0x6B8EA;
    16
}
/// $06AF38: bcc.w $6af3e
pub fn i_06af38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AF3E; return 10; }
    8
}
/// $06AF3C: jmp (a1)
pub fn i_06af3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF3E;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06AF3E: jsr $6b550(pc)
pub fn i_06af3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AF42);
    cpu.pc = 0x6B550;
    16
}
/// $06AF42: subq.w #$1, $38(a6)
pub fn i_06af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF46;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrAbsThunk_06af46  ($06AF46, 8 B, thunks/jsr_abs_thunks.c)
/// $06AF46: jsr $28d70.l
pub fn i_06af46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AF4C);
    cpu.pc = 0x28D70;
    16
}
/// $06AF4C: rts
pub fn i_06af4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaWeapon_SpriteByParent78_06af4e  ($06AF4E, 18 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AF4E: movea.l $c(a6), a0
pub fn i_06af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF52;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06AF52: tst.b $78(a0)
pub fn i_06af52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF56;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06AF56: beq.w $6af66
pub fn i_06af56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AF66; return 10; }
    8
}
/// $06AF5A: lea.l $2cb8ce.l, a0
pub fn i_06af5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CB8CE;
    20
}
// ===== JsrAbsThunk_06af60  ($06AF60, 8 B, thunks/jsr_abs_thunks.c)
/// $06AF60: jsr $28cd4.l
pub fn i_06af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AF66);
    cpu.pc = 0x28CD4;
    16
}
/// $06AF66: rts
pub fn i_06af66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaWeapon_FlyOff_06af68  ($06AF68, 134 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AF68: move.w #$ffde, d0
pub fn i_06af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF6C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06AF6C: jsr $5dca4.l
pub fn i_06af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AF72);
    cpu.pc = 0x5DCA4;
    16
}
/// $06AF72: move.w d0, $28(a6)
pub fn i_06af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF76;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AF76: move.w #$663, $2a(a6)
pub fn i_06af76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x663;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AF7C: move.w #$ff93, $2e(a6)
pub fn i_06af7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF93;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AF82: move.w #$0, $2c(a6)
pub fn i_06af82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06AF88: move.w #$4000, d0
pub fn i_06af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06AF8C: jsr $28134.l
pub fn i_06af8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AF92);
    cpu.pc = 0x28134;
    16
}
/// $06AF92: andi.w #$ffe3, $38(a6)
pub fn i_06af92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF98;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06AF98: ori.w #$0, $38(a6)
pub fn i_06af98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AF9E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06AF9E: lea.l $6afa4(pc), a1
pub fn i_06af9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AFA4;
    16
}
/// $06AFA2: move.l a1, (a6)
pub fn i_06afa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AFA4: move.w $34(a6), $5c(a6)
pub fn i_06afa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFAA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06AFAA: jsr $27bc8.l
pub fn i_06afaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AFB0);
    cpu.pc = 0x27BC8;
    16
}
/// $06AFB0: bcc.w $6afba
pub fn i_06afb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AFBA; return 10; }
    8
}
/// $06AFB4: lea.l $6b014(pc), a1
pub fn i_06afb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B014;
    16
}
/// $06AFB8: move.l a1, (a6)
pub fn i_06afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AFBA: move.w $5c(a6), $34(a6)
pub fn i_06afba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFC0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06AFC0: jsr $28d70.l
pub fn i_06afc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AFC6);
    cpu.pc = 0x28D70;
    16
}
/// $06AFC6: cmpi.w #$c0, $24(a6)
pub fn i_06afc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFCC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Word, false);
    16
}
/// $06AFCC: bgt.w $6afd6
pub fn i_06afcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFD0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6AFD6; return 10; }
    8
}
/// $06AFD0: lea.l $6a460(pc), a1
pub fn i_06afd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A460;
    16
}
/// $06AFD4: move.l a1, (a6)
pub fn i_06afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AFD6: jsr $5e45a.l
pub fn i_06afd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6AFDC);
    cpu.pc = 0x5E45A;
    16
}
/// $06AFDC: bcc.w $6afe6
pub fn i_06afdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AFE6; return 10; }
    8
}
/// $06AFE0: lea.l $6b014(pc), a1
pub fn i_06afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B014;
    16
}
/// $06AFE4: move.l a1, (a6)
pub fn i_06afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AFE6: jsr $6b1e2(pc)
pub fn i_06afe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6AFEA);
    cpu.pc = 0x6B1E2;
    16
}
/// $06AFEA: bcc.w $6aff4
pub fn i_06afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6AFF4; return 10; }
    8
}
// ===== SetTaskHandler_06afee  ($06AFEE, 8 B, kernel/task_handlers.c)
/// $06AFEE: lea.l $6a468(pc), a1
pub fn i_06afee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A468;
    16
}
/// $06AFF2: move.l a1, (a6)
pub fn i_06aff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AFF4: rts
pub fn i_06aff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaWeapon_Fall_06aff6  ($06AFF6, 22 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06AFF6: lea.l $6affc(pc), a1
pub fn i_06aff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6AFFC;
    16
}
/// $06AFFA: move.l a1, (a6)
pub fn i_06affa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AFFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06AFFC: jsr $2783a.l
pub fn i_06affc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B002;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B002);
    cpu.pc = 0x2783A;
    16
}
/// $06B002: jsr $28d70.l
pub fn i_06b002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B008;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B008);
    cpu.pc = 0x28D70;
    16
}
/// $06B008: bcc.w $6b012
pub fn i_06b008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B00C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B012; return 10; }
    8
}
// ===== SetTaskHandler_06b00c  ($06B00C, 8 B, kernel/task_handlers.c)
/// $06B00C: lea.l $6a468(pc), a1
pub fn i_06b00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A468;
    16
}
/// $06B010: move.l a1, (a6)
pub fn i_06b010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06B012: rts
pub fn i_06b012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B014;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaWeapon_Explode_06b014  ($06B014, 36 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B014: move.w #$4000, d0
pub fn i_06b014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B018;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B018: jsr $28134.l
pub fn i_06b018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B01E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B01E);
    cpu.pc = 0x28134;
    16
}
/// $06B01E: andi.w #$ffe3, $38(a6)
pub fn i_06b01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B024;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B024: ori.w #$0, $38(a6)
pub fn i_06b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B02A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B02A: move.l #$ffffffff, $48(a6)
pub fn i_06b02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B032;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B032: jmp $77f6a.l
pub fn i_06b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B038;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Bazooka_MuzzleFlash_06b038  ($06B038, 114 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B038: move.w #$4000, d0
pub fn i_06b038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B03C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B03C: jsr $28134.l
pub fn i_06b03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B042);
    cpu.pc = 0x28134;
    16
}
/// $06B042: andi.w #$ffe3, $38(a6)
pub fn i_06b042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B048;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B048: ori.w #$1c, $38(a6)
pub fn i_06b048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B04E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B04E: move.w #$8, d1
pub fn i_06b04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B052;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B052: jsr $236e.l
pub fn i_06b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B058);
    cpu.pc = 0x236E;
    16
}
/// $06B058: move.w $34(a6), d0
pub fn i_06b058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B05C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B05C: andi.w #$7, d0
pub fn i_06b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B060;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B060: bra.w $6b082
pub fn i_06b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B064;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B082;
    10
}
/// $06B082: move.b $76(a6), d0
pub fn i_06b082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B086;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B086: move.b $77(a6), d1
pub fn i_06b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B08A;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B08A: ext.w d0
pub fn i_06b08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B08C;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06B08C: ext.w d1
pub fn i_06b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B08E;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06B08E: add.w d0, $22(a6)
pub fn i_06b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B092;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B092: add.w d1, $24(a6)
pub fn i_06b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B096;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B096: subq.w #$8, $24(a6)
pub fn i_06b096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B09A;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B09A: lea.l $2dd6d2.l, a0
pub fn i_06b09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD6D2;
    20
}
/// $06B0A0: jsr $28cd4.l
pub fn i_06b0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B0A6);
    cpu.pc = 0x28CD4;
    16
}
/// $06B0A6: bra.w $6aff6
pub fn i_06b0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6AFF6;
    10
}
// ===== Bazooka_Rocket_06b0aa  ($06B0AA, 290 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B0AA: lea.l $2ca4b0.l, a1
pub fn i_06b0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA4B0;
    20
}
/// $06B0B0: move.b $98(a6), d0
pub fn i_06b0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0B4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B0B4: andi.w #$7, d0
pub fn i_06b0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0B8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B0B8: lsl.w #$2, d0
pub fn i_06b0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0BA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06B0BA: movea.l (a1, d0.w), a1
pub fn i_06b0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0BE;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06B0BE: move.w $34(a6), d0
pub fn i_06b0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0C2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B0C2: andi.w #$f, d0
pub fn i_06b0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B0C6: move.w d0, d1
pub fn i_06b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0C8;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B0C8: add.w d0, d0
pub fn i_06b0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0CA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06B0CA: add.w d1, d0
pub fn i_06b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0CC;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06B0CC: add.w d0, d0
pub fn i_06b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0CE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06B0CE: ext.l d0
pub fn i_06b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0D0;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06B0D0: adda.l d0, a1
pub fn i_06b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0D2;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $06B0D2: move.w (a1), $28(a6)
pub fn i_06b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0D6;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad3, v2 as u16);
    16
}
/// $06B0D6: move.w $2(a1), $2e(a6)
pub fn i_06b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0DC;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06B0DC: move.w $4(a1), $2a(a6)
pub fn i_06b0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0E2;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06B0E2: andi.b #$fe, $3a(a6)
pub fn i_06b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0E8;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B0E8: move.w #$173, d1
pub fn i_06b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0EC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x173;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B0EC: jsr $236e.l
pub fn i_06b0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B0F2);
    cpu.pc = 0x236E;
    16
}
/// $06B0F2: bset.b #$4, $6b(a6)
pub fn i_06b0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0F8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B0F8: lea.l $2cc686.l, a0
pub fn i_06b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B0FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CC686;
    20
}
/// $06B0FE: jsr $28cd4.l
pub fn i_06b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B104;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B104);
    cpu.pc = 0x28CD4;
    16
}
/// $06B104: move.w #$d000, d0
pub fn i_06b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B108;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B108: jsr $28134.l
pub fn i_06b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B10E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B10E);
    cpu.pc = 0x28134;
    16
}
/// $06B10E: andi.w #$ffe3, $38(a6)
pub fn i_06b10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B114;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B114: ori.w #$1c, $38(a6)
pub fn i_06b114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B11A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B11A: clr.b $79(a6)
pub fn i_06b11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B11E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B11E: move.w $34(a6), d0
pub fn i_06b11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B122;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B122: andi.w #$f, d0
pub fn i_06b122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B126;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B126: cmpi.w #$0, d0
pub fn i_06b126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B12A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06B12A: beq.w $6b16c
pub fn i_06b12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B12E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B16C; return 10; }
    8
}
/// $06B12E: cmpi.w #$8, d0
pub fn i_06b12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B132;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $06B132: bge.w $6b16c
pub fn i_06b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B136;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x6B16C; return 10; }
    8
}
/// $06B136: move.w #$d000, d0
pub fn i_06b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B13A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B13A: jsr $28134.l
pub fn i_06b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B140);
    cpu.pc = 0x28134;
    16
}
/// $06B140: andi.w #$ffe3, $38(a6)
pub fn i_06b140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B146;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B146: ori.w #$8, $38(a6)
pub fn i_06b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B14C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B14C: move.b #$1, $79(a6)
pub fn i_06b14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B152;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B152: lea.l $6b158(pc), a1
pub fn i_06b152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B156;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B158;
    16
}
/// $06B156: move.l a1, (a6)
pub fn i_06b156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B158;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06B158: jsr $27bc8.l
pub fn i_06b158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B15E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B15E);
    cpu.pc = 0x27BC8;
    16
}
/// $06B15E: bcc.w $6b168
pub fn i_06b15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B162;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B168; return 10; }
    8
}
/// $06B162: lea.l $6b1d4(pc), a1
pub fn i_06b162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B166;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B1D4;
    16
}
/// $06B166: move.l a1, (a6)
pub fn i_06b166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B168;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06B168: bra.w $6b182
pub fn i_06b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B16C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B182;
    10
}
/// $06B16C: lea.l $6b172(pc), a1
pub fn i_06b16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B170;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B172;
    16
}
/// $06B170: move.l a1, (a6)
pub fn i_06b170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B172;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06B172: jsr $27d50.l
pub fn i_06b172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B178;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B178);
    cpu.pc = 0x27D50;
    16
}
/// $06B178: bcc.w $6b182
pub fn i_06b178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B17C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B182; return 10; }
    8
}
/// $06B17C: lea.l $6b1d4(pc), a1
pub fn i_06b17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B180;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B1D4;
    16
}
/// $06B180: move.l a1, (a6)
pub fn i_06b180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B182;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06B182: move.w $28(a6), d0
pub fn i_06b182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B186;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B186: move.w $2a(a6), d1
pub fn i_06b186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B18A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B18A: asr.w #$4, d0
pub fn i_06b18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B18C;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06B18C: asr.w #$4, d1
pub fn i_06b18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B18E;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06B18E: jsr $5e018.l
pub fn i_06b18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B194);
    cpu.pc = 0x5E018;
    16
}
/// $06B194: lsr.w #$3, d0
pub fn i_06b194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B196;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B196: move.w d0, $34(a6)
pub fn i_06b196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B19A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B19A: jsr $28d70.l
pub fn i_06b19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B1A0);
    cpu.pc = 0x28D70;
    16
}
/// $06B1A0: jsr $283d8.l
pub fn i_06b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B1A6);
    cpu.pc = 0x283D8;
    16
}
/// $06B1A6: btst.b #$1, $13(a6)
pub fn i_06b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1AC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B1AC: beq.w $6b1b6
pub fn i_06b1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B1B6; return 10; }
    8
}
/// $06B1B0: lea.l $6b014(pc), a1
pub fn i_06b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B014;
    16
}
/// $06B1B4: move.l a1, (a6)
pub fn i_06b1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06B1B6: movea.l #$ffffffff, a0
pub fn i_06b1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1BC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06B1BC: lea.l $2ca3d8.l, a0
pub fn i_06b1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CA3D8;
    20
}
/// $06B1C2: jsr $5dd56.l
pub fn i_06b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B1C8);
    cpu.pc = 0x5DD56;
    16
}
/// $06B1C8: bcc.w $6b1d2
pub fn i_06b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B1D2; return 10; }
    8
}
// ===== SetTaskHandler_06b1cc  ($06B1CC, 8 B, kernel/task_handlers.c)
/// $06B1CC: lea.l $6a468(pc), a1
pub fn i_06b1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6A468;
    16
}
/// $06B1D0: move.l a1, (a6)
pub fn i_06b1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06B1D2: rts
pub fn i_06b1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_RocketHit_06b1d4  ($06B1D4, 14 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B1D4: move.w #$1025, d0
pub fn i_06b1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1025;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B1D8: jsr $2352.l
pub fn i_06b1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B1DE);
    cpu.pc = 0x2352;
    16
}
/// $06B1DE: bra.w $6b014
pub fn i_06b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B014;
    10
}
// ===== Bazooka_OffWorldOrLeft_06b1e2  ($06B1E2, 32 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B1E2: movea.l #$ffffffff, a0
pub fn i_06b1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1E8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06B1E8: lea.l $2ca3d0.l, a0
pub fn i_06b1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CA3D0;
    20
}
/// $06B1EE: jsr $5dd5c.l
pub fn i_06b1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B1F4);
    cpu.pc = 0x5DD5C;
    16
}
/// $06B1F4: bcs.w $6b208
pub fn i_06b1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1F8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6B208; return 10; }
    8
}
/// $06B1F8: cmpi.w #$ff80, $22(a6)
pub fn i_06b1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B1FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $06B1FE: blt.w $6b208
pub fn i_06b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B202;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B208; return 10; }
    8
}
// ===== ClearXN_06b202  ($06B202, 6 B, thunks/ccr_helpers.c)
/// $06B202: andi.b #$ee, ccr
pub fn i_06b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B206;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B206: rts
pub fn i_06b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B208;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06b208  ($06B208, 6 B, thunks/ccr_helpers.c)
/// $06B208: ori.b #$11, ccr
pub fn i_06b208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B20C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B20C: rts
pub fn i_06b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B20E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_ScrollGround_06b20e  ($06B20E, 36 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B20E: jsr $2783a.l
pub fn i_06b20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B214);
    cpu.pc = 0x2783A;
    16
}
/// $06B214: jsr $27eba.l
pub fn i_06b214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B21A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B21A);
    cpu.pc = 0x27EBA;
    16
}
/// $06B21A: bcc.w $6b230
pub fn i_06b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B21E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B230; return 10; }
    8
}
/// $06B21E: jsr $27c8c.l
pub fn i_06b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B224);
    cpu.pc = 0x27C8C;
    16
}
/// $06B224: bcc.w $6b230
pub fn i_06b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B228;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B230; return 10; }
    8
}
/// $06B228: clr.w $2a(a6)
pub fn i_06b228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B22C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B22C: clr.w $2e(a6)
pub fn i_06b22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B230;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B230: rts
pub fn i_06b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B232;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_ScrollGroundIfBit6_06b232  ($06B232, 10 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B232: btst.b #$6, $13(a6)
pub fn i_06b232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B238;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B238: beq.w $6b242
pub fn i_06b238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B23C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B242; return 10; }
    8
}
// ===== ClearXN_06b23c  ($06B23C, 6 B, thunks/ccr_helpers.c)
/// $06B23C: andi.b #$ee, ccr
pub fn i_06b23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B240;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B240: rts
pub fn i_06b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B242;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_DispatchScrollGround_06b242  ($06B242, 48 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B242: jsr $28998.l
pub fn i_06b242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B248);
    cpu.pc = 0x28998;
    16
}
/// $06B248: jsr $2783a.l
pub fn i_06b248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B24E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B24E);
    cpu.pc = 0x2783A;
    16
}
/// $06B24E: jsr $27eba.l
pub fn i_06b24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B254;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B254);
    cpu.pc = 0x27EBA;
    16
}
/// $06B254: bcc.w $6b272
pub fn i_06b254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B258;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B272; return 10; }
    8
}
/// $06B258: jsr $27c8c.l
pub fn i_06b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B25E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B25E);
    cpu.pc = 0x27C8C;
    16
}
/// $06B25E: bcc.w $6b278
pub fn i_06b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B262;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B278; return 10; }
    8
}
/// $06B262: clr.w $2a(a6)
pub fn i_06b262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B266;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B266: clr.w $2e(a6)
pub fn i_06b266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B26A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B26A: andi.b #$ee, ccr
pub fn i_06b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B26E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B26E: bra.w $6b278
pub fn i_06b26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B272;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B278;
    10
}
// ===== JsrAbsThunk_06b272  ($06B272, 8 B, thunks/jsr_abs_thunks.c)
/// $06B272: jsr $27afc.l
pub fn i_06b272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B278;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B278);
    cpu.pc = 0x27AFC;
    16
}
/// $06B278: rts
pub fn i_06b278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B27A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_FaceTarget_06b27a  ($06B27A, 86 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B27A: jsr $6b336(pc)
pub fn i_06b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B27E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B27E);
    cpu.pc = 0x6B336;
    16
}
/// $06B27E: bcc.w $6b28c
pub fn i_06b27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B282;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B28C; return 10; }
    8
}
/// $06B282: eori.b #$1, $75(a6)
pub fn i_06b282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B288;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B288: bra.w $6b2d6
pub fn i_06b288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B28C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B2D6;
    10
}
/// $06B28C: cmpi.w #$30, $22(a6)
pub fn i_06b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B292;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $06B292: bgt.w $6b2aa
pub fn i_06b292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B296;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B2AA; return 10; }
    8
}
/// $06B296: btst.b #$0, $75(a6)
pub fn i_06b296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B29C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B29C: bne.w $6b2aa
pub fn i_06b29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6B2AA; return 10; }
    8
}
/// $06B2A0: ori.b #$1, $75(a6)
pub fn i_06b2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2A6;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B2A6: bra.w $6b2d6
pub fn i_06b2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B2D6;
    10
}
/// $06B2AA: cmpi.w #$110, $22(a6)
pub fn i_06b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $06B2B0: blt.w $6b2c8
pub fn i_06b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2B4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B2C8; return 10; }
    8
}
/// $06B2B4: btst.b #$0, $75(a6)
pub fn i_06b2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2BA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B2BA: beq.w $6b2c8
pub fn i_06b2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B2C8; return 10; }
    8
}
/// $06B2BE: andi.b #$fe, $75(a6)
pub fn i_06b2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2C4;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B2C4: bra.w $6b2d6
pub fn i_06b2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B2D6;
    10
}
/// $06B2C8: jsr $6b2dc(pc)
pub fn i_06b2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B2CC);
    cpu.pc = 0x6B2DC;
    16
}
/// $06B2CC: bcs.w $6b2d6
pub fn i_06b2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2D0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6B2D6; return 10; }
    8
}
// ===== ClearXN_06b2d0  ($06B2D0, 6 B, thunks/ccr_helpers.c)
/// $06B2D0: andi.b #$ee, ccr
pub fn i_06b2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2D4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B2D4: rts
pub fn i_06b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06b2d6  ($06B2D6, 6 B, thunks/ccr_helpers.c)
/// $06B2D6: ori.b #$11, ccr
pub fn i_06b2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2DA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B2DA: rts
pub fn i_06b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_TargetBehind_06b2dc  ($06B2DC, 78 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B2DC: tst.b $9e(a6)
pub fn i_06b2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2E0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06B2E0: beq.w $6b32a
pub fn i_06b2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B32A; return 10; }
    8
}
/// $06B2E4: move.l $90(a6), d0
pub fn i_06b2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2E8;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B2E8: cmpi.l #$ffffffff, d0
pub fn i_06b2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2EE;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $06B2EE: beq.w $6b32a
pub fn i_06b2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B32A; return 10; }
    8
}
/// $06B2F2: movea.l d0, a0
pub fn i_06b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2F4;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06B2F4: move.w $22(a0), d0
pub fn i_06b2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2F8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B2F8: btst.b #$0, $75(a6)
pub fn i_06b2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2FE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B2FE: bne.w $6b316
pub fn i_06b2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B302;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6B316; return 10; }
    8
}
/// $06B302: addq.w #$8, d0
pub fn i_06b302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B304;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B304: cmp.w $22(a6), d0
pub fn i_06b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B308;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06B308: blt.w $6b32a
pub fn i_06b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B30C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B32A; return 10; }
    8
}
/// $06B30C: eori.b #$1, $75(a6)
pub fn i_06b30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B312;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B312: bra.w $6b330
pub fn i_06b312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B316;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B330;
    10
}
/// $06B316: subq.w #$8, d0
pub fn i_06b316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B318;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B318: cmp.w $22(a6), d0
pub fn i_06b318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B31C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06B31C: bgt.w $6b32a
pub fn i_06b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B320;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B32A; return 10; }
    8
}
/// $06B320: eori.b #$1, $75(a6)
pub fn i_06b320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B326;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B326: bra.w $6b330
pub fn i_06b326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B32A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B330;
    10
}
// ===== ClearXN_06b32a  ($06B32A, 6 B, thunks/ccr_helpers.c)
/// $06B32A: andi.b #$ee, ccr
pub fn i_06b32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B32E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B32E: rts
pub fn i_06b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B330;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06b330  ($06B330, 6 B, thunks/ccr_helpers.c)
/// $06B330: ori.b #$11, ccr
pub fn i_06b330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B334;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B334: rts
pub fn i_06b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B336;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_WallAhead_06b336  ($06B336, 38 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B336: cmpi.w #$0, $28(a6)
pub fn i_06b336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B33C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06B33C: beq.w $6b35c
pub fn i_06b33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B340;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B35C; return 10; }
    8
}
/// $06B340: bgt.w $6b352
pub fn i_06b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B344;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B352; return 10; }
    8
}
/// $06B344: btst.b #$1, $69(a6)
pub fn i_06b344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B34A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B34A: bne.w $6b362
pub fn i_06b34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B34E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6B362; return 10; }
    8
}
/// $06B34E: bra.w $6b35c
pub fn i_06b34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B352;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B35C;
    10
}
/// $06B352: btst.b #$0, $69(a6)
pub fn i_06b352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B358;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B358: bne.w $6b362
pub fn i_06b358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B35C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6B362; return 10; }
    8
}
// ===== ClearXN_06b35c  ($06B35C, 6 B, thunks/ccr_helpers.c)
/// $06B35C: andi.b #$ee, ccr
pub fn i_06b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B360;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B360: rts
pub fn i_06b360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B362;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06b362  ($06B362, 6 B, thunks/ccr_helpers.c)
/// $06B362: ori.b #$11, ccr
pub fn i_06b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B366;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B366: rts
pub fn i_06b366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B368;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_FacingMismatch_06b368  ($06B368, 22 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B368: move.b $75(a6), d0
pub fn i_06b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B36C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B36C: move.b $3a(a6), d1
pub fn i_06b36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B370;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B370: andi.b #$1, d0
pub fn i_06b370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B374;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06B374: andi.b #$1, d1
pub fn i_06b374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B378;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06B378: cmp.b d0, d1
pub fn i_06b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B37A;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $06B37A: bne.w $6b384
pub fn i_06b37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B37E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6B384; return 10; }
    8
}
// ===== ClearXN_06b37e  ($06B37E, 6 B, thunks/ccr_helpers.c)
/// $06B37E: andi.b #$ee, ccr
pub fn i_06b37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B382;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B382: rts
pub fn i_06b382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B384;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06b384  ($06B384, 6 B, thunks/ccr_helpers.c)
/// $06B384: ori.b #$11, ccr
pub fn i_06b384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B388;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B388: rts
pub fn i_06b388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B38A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_PickTargetEvery4_06b38a  ($06B38A, 26 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B38A: addq.b #$1, $7b(a6)
pub fn i_06b38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B38E;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B38E: andi.b #$3, $7b(a6)
pub fn i_06b38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B394;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B394: bne.w $6b3a2
pub fn i_06b394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B398;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6B3A2; return 10; }
    8
}
/// $06B398: jsr $5e0d4.l
pub fn i_06b398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B39E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B39E);
    cpu.pc = 0x5E0D4;
    16
}
/// $06B39E: move.l a0, $90(a6)
pub fn i_06b39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3A2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B3A2: rts
pub fn i_06b3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_AcquireTarget_06b3a4  ($06B3A4, 170 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B3A4: move.l $90(a6), d0
pub fn i_06b3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3A8;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B3A8: cmpi.l #$ffffffff, d0
pub fn i_06b3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3AE;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $06B3AE: beq.w $6b3e0
pub fn i_06b3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B3E0; return 10; }
    8
}
/// $06B3B2: clr.w d0
pub fn i_06b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3B4;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B3B4: jsr $5e3a2.l
pub fn i_06b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B3BA);
    cpu.pc = 0x5E3A2;
    16
}
/// $06B3BA: bcc.w $6b3c8
pub fn i_06b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B3C8; return 10; }
    8
}
/// $06B3BE: move.l a0, d0
pub fn i_06b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3C0;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06B3C0: cmp.l $90(a6), d0
pub fn i_06b3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3C4;
    cpu.ir = 0xB0AE;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $06B3C4: beq.w $6b454
pub fn i_06b3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B454; return 10; }
    8
}
/// $06B3C8: move.w #$1, d0
pub fn i_06b3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3CC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B3CC: jsr $5e3a2.l
pub fn i_06b3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B3D2);
    cpu.pc = 0x5E3A2;
    16
}
/// $06B3D2: bcc.w $6b3e0
pub fn i_06b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B3E0; return 10; }
    8
}
/// $06B3D6: move.l a0, d0
pub fn i_06b3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3D8;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06B3D8: cmp.l $90(a6), d0
pub fn i_06b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3DC;
    cpu.ir = 0xB0AE;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $06B3DC: beq.w $6b454
pub fn i_06b3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B454; return 10; }
    8
}
/// $06B3E0: jsr $5e0d4.l
pub fn i_06b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B3E6);
    cpu.pc = 0x5E0D4;
    16
}
/// $06B3E6: move.l a0, $90(a6)
pub fn i_06b3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3EA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B3EA: clr.w d0
pub fn i_06b3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3EC;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B3EC: jsr $5e3a2.l
pub fn i_06b3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B3F2);
    cpu.pc = 0x5E3A2;
    16
}
/// $06B3F2: bcc.w $6b426
pub fn i_06b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B426; return 10; }
    8
}
/// $06B3F6: jsr $6b45a(pc)
pub fn i_06b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B3FA);
    cpu.pc = 0x6B45A;
    16
}
/// $06B3FA: bcc.w $6b426
pub fn i_06b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B3FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B426; return 10; }
    8
}
/// $06B3FE: move.l a0, $90(a6)
pub fn i_06b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B402;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B402: move.w #$1, d0
pub fn i_06b402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B406;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B406: jsr $5e3a2.l
pub fn i_06b406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B40C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B40C);
    cpu.pc = 0x5E3A2;
    16
}
/// $06B40C: bcc.w $6b44e
pub fn i_06b40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B410;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B44E; return 10; }
    8
}
/// $06B410: jsr $6b45a(pc)
pub fn i_06b410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B414;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B414);
    cpu.pc = 0x6B45A;
    16
}
/// $06B414: bcc.w $6b44e
pub fn i_06b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B418;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B44E; return 10; }
    8
}
/// $06B418: jsr $5e0d4.l
pub fn i_06b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B41E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B41E);
    cpu.pc = 0x5E0D4;
    16
}
/// $06B41E: move.l a0, $90(a6)
pub fn i_06b41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B422;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B422: bra.w $6b44e
pub fn i_06b422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B426;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B44E;
    10
}
/// $06B426: move.w #$1, d0
pub fn i_06b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B42A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B42A: jsr $5e3a2.l
pub fn i_06b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B430);
    cpu.pc = 0x5E3A2;
    16
}
/// $06B430: bcc.w $6b444
pub fn i_06b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B434;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B444; return 10; }
    8
}
/// $06B434: jsr $6b45a(pc)
pub fn i_06b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B438;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B438);
    cpu.pc = 0x6B45A;
    16
}
/// $06B438: bcc.w $6b444
pub fn i_06b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B43C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B444; return 10; }
    8
}
/// $06B43C: move.l a0, $90(a6)
pub fn i_06b43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B440;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B440: bra.w $6b44e
pub fn i_06b440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B444;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B44E;
    10
}
/// $06B444: jsr $5e0d4.l
pub fn i_06b444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B44A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B44A);
    cpu.pc = 0x5E0D4;
    16
}
/// $06B44A: move.l a0, $90(a6)
pub fn i_06b44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B44E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== SetXN_06b44e  ($06B44E, 6 B, thunks/ccr_helpers.c)
/// $06B44E: ori.b #$11, ccr
pub fn i_06b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B452;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B452: rts
pub fn i_06b452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06b454  ($06B454, 6 B, thunks/ccr_helpers.c)
/// $06B454: andi.b #$ee, ccr
pub fn i_06b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B458;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B458: rts
pub fn i_06b458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B45A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_TargetWithin32X_06b45a  ($06B45A, 24 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B45A: move.w $22(a6), d0
pub fn i_06b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B45E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B45E: sub.w $22(a0), d0
pub fn i_06b45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B462;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B462: cmpi.w #$ffe0, d0
pub fn i_06b462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B466;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFE0, Size::Word, false);
    8
}
/// $06B466: blt.w $6b478
pub fn i_06b466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B46A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B478; return 10; }
    8
}
/// $06B46A: cmpi.w #$20, d0
pub fn i_06b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B46E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $06B46E: bgt.w $6b478
pub fn i_06b46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B472;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B478; return 10; }
    8
}
// ===== SetXN_06b472  ($06B472, 6 B, thunks/ccr_helpers.c)
/// $06B472: ori.b #$11, ccr
pub fn i_06b472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B476;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B476: rts
pub fn i_06b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B478;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06b478  ($06B478, 6 B, thunks/ccr_helpers.c)
/// $06B478: andi.b #$ee, ccr
pub fn i_06b478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B47C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B47C: rts
pub fn i_06b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B47E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_AttackTimerInRange_06b47e  ($06B47E, 104 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B47E: subq.w #$1, $70(a6)
pub fn i_06b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B482;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B482: cmpi.w #$0, $70(a6)
pub fn i_06b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B488;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06B488: bgt.w $6b4ec
pub fn i_06b488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B48C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B4EC; return 10; }
    8
}
/// $06B48C: jsr $5e0d4.l
pub fn i_06b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B492);
    cpu.pc = 0x5E0D4;
    16
}
/// $06B492: move.w $24(a0), d0
pub fn i_06b492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B496;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B496: sub.w $24(a6), d0
pub fn i_06b496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B49A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B49A: cmpi.w #$0, d0
pub fn i_06b49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B49E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06B49E: bgt.w $6b4a4
pub fn i_06b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4A2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B4A4; return 10; }
    8
}
/// $06B4A2: neg.w d0
pub fn i_06b4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4A4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B4A4: cmpi.w #$80, d0
pub fn i_06b4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4A8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $06B4A8: ble.w $6b4b0
pub fn i_06b4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4AC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6B4B0; return 10; }
    8
}
/// $06B4AC: move.w #$80, d0
pub fn i_06b4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B4B0: lsr.w #$5, d0
pub fn i_06b4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4B2;
    cpu.ir = 0xEA48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $06B4B2: lea.l $2ca44c.l, a1
pub fn i_06b4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4B8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA44C;
    20
}
/// $06B4B8: move.b $98(a6), d1
pub fn i_06b4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4BC;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B4BC: andi.w #$3, d1
pub fn i_06b4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4C0;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B4C0: lsl.w #$2, d1
pub fn i_06b4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4C2;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06B4C2: movea.l (a1, d1.w), a2
pub fn i_06b4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4C6;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $06B4C6: move.b (a2, d0.w), d1
pub fn i_06b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4CA;
    cpu.ir = 0x1232;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06B4CA: andi.w #$ff, d1
pub fn i_06b4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4CE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B4CE: move.w d1, d2
pub fn i_06b4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4D0;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B4D0: neg.w d1
pub fn i_06b4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4D2;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B4D2: move.w $22(a0), d0
pub fn i_06b4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4D6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B4D6: sub.w $22(a6), d0
pub fn i_06b4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4DA;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B4DA: cmp.w d1, d0
pub fn i_06b4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4DC;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06B4DC: blt.w $6b4ec
pub fn i_06b4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4E0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B4EC; return 10; }
    8
}
/// $06B4E0: cmp.w d2, d0
pub fn i_06b4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4E2;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06B4E2: bgt.w $6b4ec
pub fn i_06b4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4E6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B4EC; return 10; }
    8
}
// ===== SetXN_06b4e6  ($06B4E6, 6 B, thunks/ccr_helpers.c)
/// $06B4E6: ori.b #$11, ccr
pub fn i_06b4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4EA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B4EA: rts
pub fn i_06b4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06b4ec  ($06B4EC, 6 B, thunks/ccr_helpers.c)
/// $06B4EC: andi.b #$ee, ccr
pub fn i_06b4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4F0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B4F0: rts
pub fn i_06b4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_TargetOverhead_06b4f2  ($06B4F2, 60 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B4F2: cmpi.w #$0, $70(a6)
pub fn i_06b4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06B4F8: bgt.w $6b534
pub fn i_06b4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B4FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B534; return 10; }
    8
}
/// $06B4FC: tst.b $9e(a6)
pub fn i_06b4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B500;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06B500: beq.w $6b52e
pub fn i_06b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B504;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B52E; return 10; }
    8
}
/// $06B504: move.l $90(a6), d0
pub fn i_06b504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B508;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B508: cmpi.l #$ffffffff, d0
pub fn i_06b508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B50E;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $06B50E: beq.w $6b534
pub fn i_06b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B512;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B534; return 10; }
    8
}
/// $06B512: movea.l d0, a0
pub fn i_06b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B514;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06B514: move.w $22(a0), d0
pub fn i_06b514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B518;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B518: move.w d0, d1
pub fn i_06b518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B51A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B51A: subq.w #$8, d0
pub fn i_06b51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B51C;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B51C: addq.w #$8, d1
pub fn i_06b51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B51E;
    cpu.ir = 0x5041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B51E: cmp.w $22(a6), d0
pub fn i_06b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B522;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06B522: bgt.w $6b534
pub fn i_06b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B526;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B534; return 10; }
    8
}
/// $06B526: cmp.w $22(a6), d1
pub fn i_06b526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B52A;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06B52A: blt.w $6b534
pub fn i_06b52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B52E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B534; return 10; }
    8
}
// ===== SetXN_06b52e  ($06B52E, 6 B, thunks/ccr_helpers.c)
/// $06B52E: ori.b #$11, ccr
pub fn i_06b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B532;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B532: rts
pub fn i_06b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B534;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06b534  ($06B534, 6 B, thunks/ccr_helpers.c)
/// $06B534: andi.b #$ee, ccr
pub fn i_06b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B538;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B538: rts
pub fn i_06b538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B53A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_TimerExpired_06b53a  ($06B53A, 10 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B53A: cmpi.w #$0, $70(a6)
pub fn i_06b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B540;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06B540: bgt.w $6b54a
pub fn i_06b540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B544;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B54A; return 10; }
    8
}
// ===== SetXN_06b544  ($06B544, 6 B, thunks/ccr_helpers.c)
/// $06B544: ori.b #$11, ccr
pub fn i_06b544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B548;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B548: rts
pub fn i_06b548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B54A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06b54a  ($06B54A, 6 B, thunks/ccr_helpers.c)
/// $06B54A: andi.b #$ee, ccr
pub fn i_06b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B54E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B54E: rts
pub fn i_06b54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B550;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaWeapon_FollowParent_06b550  ($06B550, 84 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B550: jsr $5e506.l
pub fn i_06b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B556;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B556);
    cpu.pc = 0x5E506;
    16
}
/// $06B556: move.b $76(a0), d0
pub fn i_06b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B55A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B55A: ext.w d0
pub fn i_06b55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B55C;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06B55C: btst.b #$0, $3a(a6)
pub fn i_06b55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B562;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B562: beq.w $6b568
pub fn i_06b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B566;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B568; return 10; }
    8
}
/// $06B566: neg.w d0
pub fn i_06b566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B568;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B568: add.w d0, $22(a6)
pub fn i_06b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B56C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B56C: move.b $77(a0), d0
pub fn i_06b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B570;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B570: ext.w d0
pub fn i_06b570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B572;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06B572: add.w d0, $24(a6)
pub fn i_06b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B576;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B576: move.w $34(a6), d0
pub fn i_06b576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B57A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B57A: andi.w #$f, d0
pub fn i_06b57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B57E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B57E: beq.w $6b58a
pub fn i_06b57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B582;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B58A; return 10; }
    8
}
/// $06B582: cmpi.w #$8, d0
pub fn i_06b582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B586;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $06B586: bcs.w $6b58e
pub fn i_06b586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B58A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6B58E; return 10; }
    8
}
/// $06B58A: subq.w #$1, $38(a6)
pub fn i_06b58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B58E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B58E: movea.l $c(a6), a0
pub fn i_06b58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B592;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06B592: btst.b #$7, $5a(a0)
pub fn i_06b592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B598;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B598: beq.w $6b5a2
pub fn i_06b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B59C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B5A2; return 10; }
    8
}
/// $06B59C: bset.b #$0, $5a(a6)
pub fn i_06b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5A2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B5A2: rts
pub fn i_06b5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_RangeClass_06b5a4  ($06B5A4, 70 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B5A4: tst.b $9f(a6)
pub fn i_06b5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5A8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06B5A8: beq.w $6b5e8
pub fn i_06b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B5E8; return 10; }
    8
}
/// $06B5AC: movea.l $90(a6), a0
pub fn i_06b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5B0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06B5B0: move.w $22(a6), d0
pub fn i_06b5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B5B4: move.w $24(a6), d1
pub fn i_06b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5B8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B5B8: move.w $22(a0), d2
pub fn i_06b5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5BC;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B5BC: move.w $24(a0), d3
pub fn i_06b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5C0;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B5C0: jsr $5e23a.l
pub fn i_06b5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B5C6);
    cpu.pc = 0x5E23A;
    16
}
/// $06B5C6: clr.b d1
pub fn i_06b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5C8;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06B5C8: cmpi.w #$70, d0
pub fn i_06b5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5CC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x70, Size::Word, false);
    8
}
/// $06B5CC: blt.w $6b5e0
pub fn i_06b5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5D0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B5E0; return 10; }
    8
}
/// $06B5D0: move.b #$1, d1
pub fn i_06b5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5D4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06B5D4: cmpi.w #$b0, d0
pub fn i_06b5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5D8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB0, Size::Word, false);
    8
}
/// $06B5D8: blt.w $6b5e0
pub fn i_06b5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5DC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B5E0; return 10; }
    8
}
/// $06B5DC: move.b #$2, d1
pub fn i_06b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5E0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06B5E0: andi.b #$3, d1
pub fn i_06b5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5E4;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06B5E4: move.b d1, $98(a6)
pub fn i_06b5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5E8;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B5E8: rts
pub fn i_06b5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_AimStep_06b5ea  ($06B5EA, 130 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B5EA: movea.l $90(a6), a0
pub fn i_06b5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5EE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06B5EE: lea.l $2ca5f0.l, a1
pub fn i_06b5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5F4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA5F0;
    20
}
/// $06B5F4: move.w $24(a0), d0
pub fn i_06b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5F8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B5F8: cmp.w $24(a6), d0
pub fn i_06b5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B5FC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06B5FC: bgt.w $6b606
pub fn i_06b5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B600;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B606; return 10; }
    8
}
/// $06B600: lea.l $2ca612.l, a1
pub fn i_06b600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B606;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA612;
    20
}
/// $06B606: move.w $22(a0), d0
pub fn i_06b606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B60A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B60A: sub.w $22(a6), d0
pub fn i_06b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B60E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B60E: cmpi.w #$ff00, d0
pub fn i_06b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B612;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF00, Size::Word, false);
    8
}
/// $06B612: bgt.w $6b61a
pub fn i_06b612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B616;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B61A; return 10; }
    8
}
/// $06B616: move.w #$ff00, d0
pub fn i_06b616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B61A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B61A: cmpi.w #$100, d0
pub fn i_06b61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B61E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $06B61E: blt.w $6b626
pub fn i_06b61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B622;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B626; return 10; }
    8
}
/// $06B622: move.w #$100, d0
pub fn i_06b622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B626;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B626: addi.w #$100, d0
pub fn i_06b626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B62A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B62A: lsr.w #$4, d0
pub fn i_06b62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B62C;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06B62C: move.b (a1, d0.w), d0
pub fn i_06b62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B630;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06B630: btst.b #$0, $3a(a6)
pub fn i_06b630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B636;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B636: beq.w $6b644
pub fn i_06b636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B63A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B644; return 10; }
    8
}
/// $06B63A: lea.l $2ca634.l, a1
pub fn i_06b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B640;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA634;
    20
}
/// $06B640: move.b (a1, d0.w), d0
pub fn i_06b640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B644;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06B644: jsr $6b678(pc)
pub fn i_06b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B648;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B648);
    cpu.pc = 0x6B678;
    16
}
/// $06B648: andi.w #$f, d0
pub fn i_06b648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B64C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B64C: cmp.w $34(a6), d0
pub fn i_06b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B650;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06B650: beq.w $6b672
pub fn i_06b650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B654;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B672; return 10; }
    8
}
/// $06B654: sub.w $34(a6), d0
pub fn i_06b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B658;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B658: move.w #$1, d1
pub fn i_06b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B65C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B65C: cmpi.w #$0, d0
pub fn i_06b65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B660;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06B660: bgt.w $6b668
pub fn i_06b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B664;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B668; return 10; }
    8
}
/// $06B664: move.w #$ffff, d1
pub fn i_06b664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B668;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B668: add.w d1, $34(a6)
pub fn i_06b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B66C;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_06b66c  ($06B66C, 6 B, thunks/ccr_helpers.c)
/// $06B66C: andi.b #$ee, ccr
pub fn i_06b66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B670;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B670: rts
pub fn i_06b670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B672;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06b672  ($06B672, 6 B, thunks/ccr_helpers.c)
/// $06B672: ori.b #$11, ccr
pub fn i_06b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B676;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B676: rts
pub fn i_06b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B678;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_AimClampNear_06b678  ($06B678, 68 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B678: move.b d0, $5c(a6)
pub fn i_06b678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B67C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B67C: movea.l $90(a6), a0
pub fn i_06b67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B680;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06B680: move.w $24(a0), d0
pub fn i_06b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B684;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B684: sub.w $24(a6), d0
pub fn i_06b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B688;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B688: cmpi.w #$fff8, d0
pub fn i_06b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B68C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF8, Size::Word, false);
    8
}
/// $06B68C: blt.w $6b6c2
pub fn i_06b68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B690;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B6C2; return 10; }
    8
}
/// $06B690: cmpi.w #$28, d0
pub fn i_06b690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B694;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x28, Size::Word, false);
    8
}
/// $06B694: bgt.w $6b6c2
pub fn i_06b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B698;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B6C2; return 10; }
    8
}
/// $06B698: move.w $22(a0), d0
pub fn i_06b698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B69C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B69C: sub.w $22(a6), d0
pub fn i_06b69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6A0;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06B6A0: btst.b #$0, $3a(a6)
pub fn i_06b6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6A6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B6A6: beq.w $6b6ac
pub fn i_06b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B6AC; return 10; }
    8
}
/// $06B6AA: neg.w d0
pub fn i_06b6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6AC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B6AC: clr.b d1
pub fn i_06b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6AE;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06B6AE: cmpi.w #$0, d0
pub fn i_06b6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6B2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06B6B2: bgt.w $6b6ba
pub fn i_06b6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6B6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B6BA; return 10; }
    8
}
/// $06B6B6: move.b #$8, d1
pub fn i_06b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6BA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06B6BA: move.b d1, d0
pub fn i_06b6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6BC;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== SetXN_06b6bc  ($06B6BC, 6 B, thunks/ccr_helpers.c)
/// $06B6BC: ori.b #$11, ccr
pub fn i_06b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B6C0: rts
pub fn i_06b6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_AimRestore_06b6c2  ($06B6C2, 4 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B6C2: move.b $5c(a6), d0
pub fn i_06b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
// ===== ClearXN_06b6c6  ($06B6C6, 6 B, thunks/ccr_helpers.c)
/// $06B6C6: andi.b #$ee, ccr
pub fn i_06b6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B6CA: rts
pub fn i_06b6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_AimDone_06b6cc  ($06B6CC, 26 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B6CC: tst.b $9e(a6)
pub fn i_06b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6D0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06B6D0: beq.w $6b6ec
pub fn i_06b6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B6EC; return 10; }
    8
}
/// $06B6D4: tst.w $34(a6)
pub fn i_06b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6D8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $06B6D8: beq.w $6b6ec
pub fn i_06b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B6EC; return 10; }
    8
}
/// $06B6DC: cmpi.w #$d, $34(a6)
pub fn i_06b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $06B6E2: bcc.w $6b6ec
pub fn i_06b6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6B6EC; return 10; }
    8
}
// ===== SetXN_06b6e6  ($06B6E6, 6 B, thunks/ccr_helpers.c)
/// $06B6E6: ori.b #$11, ccr
pub fn i_06b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6EA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B6EA: rts
pub fn i_06b6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06b6ec  ($06B6EC, 6 B, thunks/ccr_helpers.c)
/// $06B6EC: andi.b #$ee, ccr
pub fn i_06b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6F0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B6F0: rts
pub fn i_06b6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_Fire_06b6f2  ($06B6F2, 152 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B6F2: move.w #$1064, d0
pub fn i_06b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B6F6: jsr $2352.l
pub fn i_06b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B6FC);
    cpu.pc = 0x2352;
    16
}
/// $06B6FC: move.w $34(a6), d0
pub fn i_06b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B700;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B700: btst.b #$0, $3a(a6)
pub fn i_06b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B706;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B706: beq.w $6b714
pub fn i_06b706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B70A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B714; return 10; }
    8
}
/// $06B70A: lea.l $2ca634.l, a1
pub fn i_06b70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B710;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA634;
    20
}
/// $06B710: move.b (a1, d0.w), d0
pub fn i_06b710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B714;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06B714: andi.w #$f, d0
pub fn i_06b714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B718;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B718: move.w d0, $5c(a6)
pub fn i_06b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B71C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B71C: lea.l $6b038(pc), a1
pub fn i_06b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B720;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B038;
    16
}
/// $06B720: jsr $4ae.l
pub fn i_06b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B726;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B726);
    cpu.pc = 0x4AE;
    16
}
/// $06B726: jsr $5dd02.l
pub fn i_06b726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B72C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B72C);
    cpu.pc = 0x5DD02;
    16
}
/// $06B72C: lea.l $2ca470.l, a1
pub fn i_06b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B732;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA470;
    20
}
/// $06B732: move.w $34(a6), d0
pub fn i_06b732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B736;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B736: move.w d0, $34(a0)
pub fn i_06b736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B73A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B73A: lsl.w #$2, d0
pub fn i_06b73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B73C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06B73C: move.w (a1, d0.w), d1
pub fn i_06b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B740;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06B740: move.w $2(a1, d0.w), d2
pub fn i_06b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B744;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06B744: btst.b #$0, $3a(a6)
pub fn i_06b744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B74A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B74A: beq.w $6b750
pub fn i_06b74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B74E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B750; return 10; }
    8
}
/// $06B74E: neg.w d1
pub fn i_06b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B750;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B750: move.w d1, d3
pub fn i_06b750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B752;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B752: move.w d2, d4
pub fn i_06b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B754;
    cpu.ir = 0x3802;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B754: move.b d3, $76(a0)
pub fn i_06b754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B758;
    cpu.ir = 0x1143;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B758: move.b d4, $77(a0)
pub fn i_06b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B75C;
    cpu.ir = 0x1144;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B75C: movem.w d1-d2, -(a7)
pub fn i_06b75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B760;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $06B760: lea.l $6b0aa(pc), a1
pub fn i_06b760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B764;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6B0AA;
    16
}
/// $06B764: jsr $4ae.l
pub fn i_06b764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B76A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B76A);
    cpu.pc = 0x4AE;
    16
}
/// $06B76A: jsr $5dd02.l
pub fn i_06b76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B770);
    cpu.pc = 0x5DD02;
    16
}
/// $06B770: movem.w (a7)+, d1-d2
pub fn i_06b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B774;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06B774: move.w $5c(a6), $34(a0)
pub fn i_06b774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B77A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06B77A: add.w d1, $22(a0)
pub fn i_06b77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B77E;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B77E: add.w d2, $24(a0)
pub fn i_06b77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B782;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06B782: move.b $98(a6), $98(a0)
pub fn i_06b782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B788;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B788: rts
pub fn i_06b788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B78A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_SpawnWeaponGround_06b78a  ($06B78A, 134 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B78A: jsr $5e7c0.l
pub fn i_06b78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B790);
    cpu.pc = 0x5E7C0;
    16
}
/// $06B790: move.b $99(a6), d0
pub fn i_06b790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B794;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B794: andi.b #$1, d0
pub fn i_06b794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B798;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06B798: move.b d0, $3a(a6)
pub fn i_06b798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B79C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B79C: move.b d0, $75(a6)
pub fn i_06b79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7A0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B7A0: move.b $9a(a6), $7a(a6)
pub fn i_06b7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7A6;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B7A6: jsr $6b94c(pc)
pub fn i_06b7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B7AA);
    cpu.pc = 0x6B94C;
    16
}
/// $06B7AA: move.w #$8000, d0
pub fn i_06b7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7AE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B7AE: jsr $28134.l
pub fn i_06b7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B7B4);
    cpu.pc = 0x28134;
    16
}
/// $06B7B4: andi.w #$ffe3, $38(a6)
pub fn i_06b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7BA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B7BA: ori.w #$14, $38(a6)
pub fn i_06b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7C0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B7C0: move.l #$2ca198, $60(a6)
pub fn i_06b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7C8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CA198;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B7C8: move.l #$2ca1e4, $48(a6)
pub fn i_06b7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7D0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CA1E4;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B7D0: lea.l $6acb2(pc), a1
pub fn i_06b7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ACB2;
    16
}
/// $06B7D4: jsr $4ae.l
pub fn i_06b7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B7DA);
    cpu.pc = 0x4AE;
    16
}
/// $06B7DA: jsr $5dd02.l
pub fn i_06b7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B7E0);
    cpu.pc = 0x5DD02;
    16
}
/// $06B7E0: clr.b $9e(a0)
pub fn i_06b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7E4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B7E4: move.b $9f(a6), $9f(a0)
pub fn i_06b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7EA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9F);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B7EA: move.b $98(a6), $98(a0)
pub fn i_06b7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7F0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B7F0: move.b $9a(a6), $7a(a0)
pub fn i_06b7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7F6;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B7F6: move.b $9c(a6), $9c(a0)
pub fn i_06b7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B7FC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B7FC: move.b $9d(a6), $9d(a0)
pub fn i_06b7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B802;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B802: move.b $9e(a6), $9e(a0)
pub fn i_06b802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B808;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B808: bset.b #$2, $6b(a6)
pub fn i_06b808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B80E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06B80E: rts
pub fn i_06b80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B810;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_SpawnWeaponTable_06b810  ($06B810, 106 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B810: jsr $6b94c(pc)
pub fn i_06b810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6B814);
    cpu.pc = 0x6B94C;
    16
}
/// $06B814: move.w #$8000, d0
pub fn i_06b814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B818;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B818: jsr $28134.l
pub fn i_06b818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B81E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B81E);
    cpu.pc = 0x28134;
    16
}
/// $06B81E: andi.w #$ffe3, $38(a6)
pub fn i_06b81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B824;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B824: ori.w #$8, $38(a6)
pub fn i_06b824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B82A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06B82A: lea.l $2bd3e6.l, a0
pub fn i_06b82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B830;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD3E6;
    20
}
/// $06B830: jsr $799de.l
pub fn i_06b830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B836;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B836);
    cpu.pc = 0x799DE;
    16
}
/// $06B836: move.w d0, $66(a6)
pub fn i_06b836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B83A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B83A: move.l #$2ca1e4, $48(a6)
pub fn i_06b83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B842;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CA1E4;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06B842: move.b $9d(a6), $75(a6)
pub fn i_06b842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B848;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B848: lea.l $6acb2(pc), a1
pub fn i_06b848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B84C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ACB2;
    16
}
/// $06B84C: jsr $4ae.l
pub fn i_06b84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B852);
    cpu.pc = 0x4AE;
    16
}
/// $06B852: jsr $5dd02.l
pub fn i_06b852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B858;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B858);
    cpu.pc = 0x5DD02;
    16
}
/// $06B858: clr.b $98(a0)
pub fn i_06b858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B85C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B85C: clr.b $9f(a0)
pub fn i_06b85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B860;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B860: clr.b $9c(a0)
pub fn i_06b860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B864;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B864: clr.b $9d(a0)
pub fn i_06b864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B868;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B868: clr.b $9e(a0)
pub fn i_06b868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B86C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06B86C: move.b $9e(a6), $9e(a0)
pub fn i_06b86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B872;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B872: move.b $7a(a6), $7a(a0)
pub fn i_06b872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B878;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06B878: rts
pub fn i_06b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B87A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_FollowParentOffs_06b87a  ($06B87A, 66 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B87A: movea.l $c(a6), a0
pub fn i_06b87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B87E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06B87E: move.w $22(a0), d0
pub fn i_06b87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B882;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B882: move.w $24(a0), d1
pub fn i_06b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B886;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B886: move.w $38(a0), d2
pub fn i_06b886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B88A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B88A: move.b $9a(a6), d3
pub fn i_06b88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B88E;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B88E: move.b $9b(a6), d4
pub fn i_06b88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B892;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B892: move.b $9c(a6), d5
pub fn i_06b892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B896;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B896: ext.w d3
pub fn i_06b896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B898;
    cpu.ir = 0x4883;
    let v = cpu.d[3] as i8 as i16 as u16 as u32; cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06B898: ext.w d4
pub fn i_06b898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B89A;
    cpu.ir = 0x4884;
    let v = cpu.d[4] as i8 as i16 as u16 as u32; cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06B89A: ext.w d5
pub fn i_06b89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B89C;
    cpu.ir = 0x4885;
    let v = cpu.d[5] as i8 as i16 as u16 as u32; cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06B89C: btst.b #$0, $3a(a0)
pub fn i_06b89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8A2;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B8A2: bne.w $6b8a8
pub fn i_06b8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6B8A8; return 10; }
    8
}
/// $06B8A6: neg.w d3
pub fn i_06b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8A8;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06B8A8: add.w d3, d0
pub fn i_06b8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8AA;
    cpu.ir = 0xD043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06B8AA: add.w d4, d1
pub fn i_06b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8AC;
    cpu.ir = 0xD244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06B8AC: add.w d5, d2
pub fn i_06b8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8AE;
    cpu.ir = 0xD445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06B8AE: move.w d0, $22(a6)
pub fn i_06b8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8B2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B8B2: move.w d1, $24(a6)
pub fn i_06b8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8B6;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B8B6: move.w d2, $38(a6)
pub fn i_06b8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8BA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B8BA: rts
pub fn i_06b8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_PastTargetX_06b8bc  ($06B8BC, 34 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B8BC: move.w $22(a6), d0
pub fn i_06b8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8C0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B8C0: move.w $98(a6), d1
pub fn i_06b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8C4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06B8C4: btst.b #$0, $75(a6)
pub fn i_06b8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06B8CA: beq.w $6b8d8
pub fn i_06b8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B8D8; return 10; }
    8
}
/// $06B8CE: cmp.w d0, d1
pub fn i_06b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8D0;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06B8D0: bgt.w $6b8e4
pub fn i_06b8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8D4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6B8E4; return 10; }
    8
}
/// $06B8D4: bra.w $6b8de
pub fn i_06b8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6B8DE;
    10
}
/// $06B8D8: cmp.w d0, d1
pub fn i_06b8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8DA;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06B8DA: blt.w $6b8e4
pub fn i_06b8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8DE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6B8E4; return 10; }
    8
}
// ===== SetXN_06b8de  ($06B8DE, 6 B, thunks/ccr_helpers.c)
/// $06B8DE: ori.b #$11, ccr
pub fn i_06b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8E2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B8E2: rts
pub fn i_06b8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06b8e4  ($06B8E4, 6 B, thunks/ccr_helpers.c)
/// $06B8E4: andi.b #$ee, ccr
pub fn i_06b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8E8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B8E8: rts
pub fn i_06b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaWeapon_StateLookup_06b8ea  ($06B8EA, 82 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B8EA: lea.l $2ca644.l, a1
pub fn i_06b8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA644;
    20
}
/// $06B8F0: tst.b $9d(a6)
pub fn i_06b8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8F4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06B8F4: beq.w $6b8fe
pub fn i_06b8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B8FE; return 10; }
    8
}
/// $06B8F8: lea.l $2ca664.l, a1
pub fn i_06b8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8FE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA664;
    20
}
/// $06B8FE: movea.l $c(a6), a0
pub fn i_06b8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B902;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06B902: move.b $72(a0), d0
pub fn i_06b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B906;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B906: cmp.b $72(a6), d0
pub fn i_06b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B90A;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06B90A: beq.w $6b942
pub fn i_06b90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B90E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B942; return 10; }
    8
}
/// $06B90E: move.b d0, $72(a6)
pub fn i_06b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B912;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06B912: cmpi.b #$8, d0
pub fn i_06b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B916;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $06B916: bcs.w $6b926
pub fn i_06b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B91A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6B926; return 10; }
    8
}
/// $06B91A: nop
pub fn i_06b91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B91C;
    cpu.ir = 0x4E71;
    4
}
/// $06B91C: nop
pub fn i_06b91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B91E;
    cpu.ir = 0x4E71;
    4
}
/// $06B91E: cmpi.b #$8, d0
pub fn i_06b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B922;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $06B922: nop
pub fn i_06b922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B924;
    cpu.ir = 0x4E71;
    4
}
/// $06B924: trap #$f
pub fn i_06b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B926;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06B926: andi.w #$7, d0
pub fn i_06b926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B92A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B92A: lsl.w #$2, d0
pub fn i_06b92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B92C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06B92C: move.l (a1, d0.w), d0
pub fn i_06b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B930;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $06B930: cmpi.l #$ffffffff, d0
pub fn i_06b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B936;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $06B936: beq.w $6b946
pub fn i_06b936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B93A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6B946; return 10; }
    8
}
/// $06B93A: movea.l d0, a1
pub fn i_06b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B93C;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== SetXN_06b93c  ($06B93C, 6 B, thunks/ccr_helpers.c)
/// $06B93C: ori.b #$11, ccr
pub fn i_06b93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B940;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06B940: rts
pub fn i_06b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B942;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BazookaWeapon_SetState_06b942  ($06B942, 4 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B942: move.b d0, $72(a6)
pub fn i_06b942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B946;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_06b946  ($06B946, 6 B, thunks/ccr_helpers.c)
/// $06B946: andi.b #$ee, ccr
pub fn i_06b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B94A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06B94A: rts
pub fn i_06b94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B94C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_PlayCry_06b94c  ($06B94C, 32 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B94C: lea.l $2ca404.l, a1
pub fn i_06b94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B952;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA404;
    20
}
/// $06B952: move.b $7a(a6), d0
pub fn i_06b952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B956;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06B956: andi.w #$3, d0
pub fn i_06b956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B95A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06B95A: lsl.w #$1, d0
pub fn i_06b95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B95C;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06B95C: move.w (a1, d0.w), d1
pub fn i_06b95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B960;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06B960: jsr $236e.l
pub fn i_06b960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B966;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B966);
    cpu.pc = 0x236E;
    16
}
/// $06B966: move.w #$7, $1c(a6)
pub fn i_06b966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B96C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_06b96c  ($06B96C, 8 B, thunks/jsr_abs_thunks.c)
/// $06B96C: jsr $138fe.l
pub fn i_06b96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B972);
    cpu.pc = 0x138FE;
    16
}
/// $06B972: rts
pub fn i_06b972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B974;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_DeathDebris_06b974  ($06B974, 48 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B974: move.w #$1033, d0
pub fn i_06b974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B978;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B978: jsr $2352.l
pub fn i_06b978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B97E);
    cpu.pc = 0x2352;
    16
}
/// $06B97E: lea.l $2ca3e0.l, a1
pub fn i_06b97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B984;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA3E0;
    20
}
/// $06B984: jsr $77c7e.l
pub fn i_06b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B98A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B98A);
    cpu.pc = 0x77C7E;
    16
}
/// $06B98A: move.w #$4000, $38(a0)
pub fn i_06b98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B990;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B990: lea.l $2ca3f2.l, a1
pub fn i_06b990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B996;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA3F2;
    20
}
/// $06B996: jsr $77c7e.l
pub fn i_06b996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B99C);
    cpu.pc = 0x77C7E;
    16
}
/// $06B99C: move.w #$4000, $38(a0)
pub fn i_06b99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9A2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B9A2: rts
pub fn i_06b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_DeathExplodeA_06b9a4  ($06B9A4, 54 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B9A4: move.w #$1021, d0
pub fn i_06b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B9A8: jsr $2352.l
pub fn i_06b9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9AE);
    cpu.pc = 0x2352;
    16
}
/// $06B9AE: lea.l $2ca3f2.l, a1
pub fn i_06b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA3F2;
    20
}
/// $06B9B4: jsr $77c7e.l
pub fn i_06b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9BA);
    cpu.pc = 0x77C7E;
    16
}
/// $06B9BA: move.w #$4000, $38(a0)
pub fn i_06b9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B9C0: lea.l $77fd6.l, a1
pub fn i_06b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06B9C6: jsr $4ae.l
pub fn i_06b9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9CC);
    cpu.pc = 0x4AE;
    16
}
/// $06B9CC: jsr $5dd02.l
pub fn i_06b9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9D2);
    cpu.pc = 0x5DD02;
    16
}
/// $06B9D2: move.w #$4000, $38(a0)
pub fn i_06b9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9D8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B9D8: rts
pub fn i_06b9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_DeathExplodeB_06b9da  ($06B9DA, 54 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B9DA: move.w #$1021, d0
pub fn i_06b9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B9DE: jsr $2352.l
pub fn i_06b9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9E4);
    cpu.pc = 0x2352;
    16
}
/// $06B9E4: lea.l $2ca3f2.l, a1
pub fn i_06b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA3F2;
    20
}
/// $06B9EA: jsr $77c7e.l
pub fn i_06b9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9F0);
    cpu.pc = 0x77C7E;
    16
}
/// $06B9F0: move.w #$4000, $38(a0)
pub fn i_06b9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9F6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B9F6: lea.l $77fd6.l, a1
pub fn i_06b9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9FC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06B9FC: jsr $4ae.l
pub fn i_06b9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA02);
    cpu.pc = 0x4AE;
    16
}
/// $06BA02: jsr $5dd02.l
pub fn i_06ba02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA08);
    cpu.pc = 0x5DD02;
    16
}
/// $06BA08: move.w #$4000, $38(a0)
pub fn i_06ba08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA0E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA0E: rts
pub fn i_06ba0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_CmpSlotWithParent_06ba10  ($06BA10, 16 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BA10: movea.l $8(a6), a1
pub fn i_06ba10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA14;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06BA14: move.b $10(a6), d0
pub fn i_06ba14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA18;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06BA18: cmp.b $10(a1), d0
pub fn i_06ba18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA1C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BA1C: bcs.w $6ba26
pub fn i_06ba1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA20;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BA26; return 10; }
    8
}
// ===== ClearXN_06ba20  ($06BA20, 6 B, thunks/ccr_helpers.c)
/// $06BA20: andi.b #$ee, ccr
pub fn i_06ba20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA24;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06BA24: rts
pub fn i_06ba24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06ba26  ($06BA26, 6 B, thunks/ccr_helpers.c)
/// $06BA26: ori.b #$11, ccr
pub fn i_06ba26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA2A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06BA2A: rts
pub fn i_06ba2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_Tmpl59_06ba2c  ($06BA2C, 214 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BA2C: clr.b $9e(a6)
pub fn i_06ba2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA30;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BA30: bra.w $6ba3a
pub fn i_06ba30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA34;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BA3A;
    10
}
/// $06BA3A: jsr $283ca.l
pub fn i_06ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA40);
    cpu.pc = 0x283CA;
    16
}
/// $06BA40: jsr $5e7c0.l
pub fn i_06ba40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA46);
    cpu.pc = 0x5E7C0;
    16
}
/// $06BA46: move.b $99(a6), d0
pub fn i_06ba46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA4A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06BA4A: andi.b #$1, d0
pub fn i_06ba4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA4E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06BA4E: move.b d0, $3a(a6)
pub fn i_06ba4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA52;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BA52: move.b d0, $8d(a6)
pub fn i_06ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA56;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BA56: jsr $6d446(pc)
pub fn i_06ba56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BA5A);
    cpu.pc = 0x6D446;
    16
}
/// $06BA5A: move.b $9a(a6), $96(a6)
pub fn i_06ba5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA60;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BA60: jsr $6d574(pc)
pub fn i_06ba60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BA64);
    cpu.pc = 0x6D574;
    16
}
/// $06BA64: lea.l $2bd468.l, a0
pub fn i_06ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD468;
    20
}
/// $06BA6A: jsr $799de.l
pub fn i_06ba6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA70);
    cpu.pc = 0x799DE;
    16
}
/// $06BA70: move.w d0, $36(a6)
pub fn i_06ba70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA74;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA74: lea.l $2bd4ea.l, a0
pub fn i_06ba74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD4EA;
    20
}
/// $06BA7A: jsr $799de.l
pub fn i_06ba7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA80);
    cpu.pc = 0x799DE;
    16
}
/// $06BA80: move.w d0, $70(a6)
pub fn i_06ba80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA84;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA84: lea.l $2bd3e6.l, a0
pub fn i_06ba84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD3E6;
    20
}
/// $06BA8A: jsr $799de.l
pub fn i_06ba8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA90);
    cpu.pc = 0x799DE;
    16
}
/// $06BA90: move.w d0, $66(a6)
pub fn i_06ba90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA94;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA94: move.w #$8000, d0
pub fn i_06ba94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06BA98: jsr $28134.l
pub fn i_06ba98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA9E);
    cpu.pc = 0x28134;
    16
}
/// $06BA9E: andi.w #$ffe3, $38(a6)
pub fn i_06ba9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAA4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06BAA4: ori.w #$14, $38(a6)
pub fn i_06baa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAAA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06BAAA: move.l #$2cc7a6, $60(a6)
pub fn i_06baaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAB2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC7A6;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06BAB2: move.l #$2cc8d6, $48(a6)
pub fn i_06bab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BABA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC8D6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06BABA: clr.b $89(a6)
pub fn i_06baba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BABE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BABE: move.w #$8, $8a(a6)
pub fn i_06babe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BAC4: move.b #$ff, $97(a6)
pub fn i_06bac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BACA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BACA: move.w $24(a6), $82(a6)
pub fn i_06baca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAD0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06BAD0: jsr $30696.l
pub fn i_06bad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BAD6);
    cpu.pc = 0x30696;
    16
}
/// $06BAD6: lea.l $6c562(pc), a1
pub fn i_06bad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BADA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C562;
    16
}
/// $06BADA: jsr $4ae.l
pub fn i_06bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BAE0);
    cpu.pc = 0x4AE;
    16
}
/// $06BAE0: jsr $5dd02.l
pub fn i_06bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BAE6);
    cpu.pc = 0x5DD02;
    16
}
/// $06BAE6: move.b $9f(a6), $9f(a0)
pub fn i_06bae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAEC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9F);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BAEC: move.b $98(a6), $98(a0)
pub fn i_06baec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAF2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BAF2: move.b $9a(a6), $96(a0)
pub fn i_06baf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAF8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x96);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BAF8: bset.b #$2, $6b(a6)
pub fn i_06baf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAFE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BAFE: bra.w $6bee0
pub fn i_06bafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB02;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BEE0;
    10
}
// ===== RocketVehicle_Drive_06bb02  ($06BB02, 86 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BB02: move.b #$0, $72(a6)
pub fn i_06bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BB08: move.w $36(a6), d0
pub fn i_06bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB0C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06BB0C: btst.b #$0, $8d(a6)
pub fn i_06bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB12;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06BB12: bne.w $6bb18
pub fn i_06bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6BB18; return 10; }
    8
}
/// $06BB16: neg.w d0
pub fn i_06bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB18;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06BB18: move.w d0, $28(a6)
pub fn i_06bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BB1C: jsr $6d206(pc)
pub fn i_06bb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB20);
    cpu.pc = 0x6D206;
    16
}
/// $06BB20: bcs.w $6bb6c
pub fn i_06bb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB24;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BB6C; return 10; }
    8
}
/// $06BB24: jsr $6d1dc(pc)
pub fn i_06bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB28);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BB28: bcs.w $6c272
pub fn i_06bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB2C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BB2C: move.b d0, $97(a6)
pub fn i_06bb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB30;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BB30: andi.w #$3, d0
pub fn i_06bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB34;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BB34: movea.l #$6bb58, a0
pub fn i_06bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB3A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BB58;
    cpu.a[0] = v1;
    4
}
/// $06BB3A: lsl.w #$2, d0
pub fn i_06bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB3C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BB3C: movea.l (a0, d0.w), a0
pub fn i_06bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB40;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BB40: cmpa.l #$ffffffff, a0
pub fn i_06bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB46;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BB46: beq.w $6bb50
pub fn i_06bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BB50; return 10; }
    8
}
/// $06BB4A: jsr $28cd4.l
pub fn i_06bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BB50);
    cpu.pc = 0x28CD4;
    16
}
/// $06BB50: jsr $6d484(pc)
pub fn i_06bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB54);
    cpu.pc = 0x6D484;
    16
}
/// $06BB54: bra.w $6bb68
pub fn i_06bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB58;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BB68;
    10
}
/// $06BB68: bra.w $6bbb0
pub fn i_06bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BBB0;
    10
}
/// $06BB6C: jsr $6d1dc(pc)
pub fn i_06bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB70);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BB70: bcs.w $6c272
pub fn i_06bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB74;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BB74: move.b d0, $97(a6)
pub fn i_06bb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB78;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BB78: andi.w #$3, d0
pub fn i_06bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB7C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BB7C: movea.l #$6bba0, a0
pub fn i_06bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB82;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BBA0;
    cpu.a[0] = v1;
    4
}
/// $06BB82: lsl.w #$2, d0
pub fn i_06bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB84;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BB84: movea.l (a0, d0.w), a0
pub fn i_06bb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB88;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BB88: cmpa.l #$ffffffff, a0
pub fn i_06bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB8E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BB8E: beq.w $6bb98
pub fn i_06bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BB98; return 10; }
    8
}
/// $06BB92: jsr $28cd4.l
pub fn i_06bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BB98);
    cpu.pc = 0x28CD4;
    16
}
/// $06BB98: jsr $6d484(pc)
pub fn i_06bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB9C);
    cpu.pc = 0x6D484;
    16
}
/// $06BB9C: bra.w $6bbb0
pub fn i_06bb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BBB0;
    10
}
/// $06BBB0: lea.l $6bbb6(pc), a1
pub fn i_06bbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BBB6;
    16
}
/// $06BBB4: move.l a1, (a6)
pub fn i_06bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BBB6: jsr $28998.l
pub fn i_06bbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BBBC);
    cpu.pc = 0x28998;
    16
}
/// $06BBBC: jsr $6d0f6(pc)
pub fn i_06bbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBC0);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BBC0: bcc.w $6bbd0
pub fn i_06bbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BBD0; return 10; }
    8
}
/// $06BBC4: eori.b #$1, $8d(a6)
pub fn i_06bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBCA;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BBCA: lea.l $6bcfc(pc), a1
pub fn i_06bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BBCE: move.l a1, (a6)
pub fn i_06bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BBD0: jsr $6d1dc(pc)
pub fn i_06bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBD4);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BBD4: bcs.w $6c272
pub fn i_06bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBD8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BBD8: cmp.b $97(a6), d0
pub fn i_06bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBDC;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BBDC: beq.w $6bc74
pub fn i_06bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BC74; return 10; }
    8
}
/// $06BBE0: jsr $6d206(pc)
pub fn i_06bbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBE4);
    cpu.pc = 0x6D206;
    16
}
/// $06BBE4: bcs.w $6bc30
pub fn i_06bbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBE8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BC30; return 10; }
    8
}
/// $06BBE8: jsr $6d1dc(pc)
pub fn i_06bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBEC);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BBEC: bcs.w $6c272
pub fn i_06bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBF0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BBF0: move.b d0, $97(a6)
pub fn i_06bbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBF4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BBF4: andi.w #$3, d0
pub fn i_06bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BBF8: movea.l #$6bc1c, a0
pub fn i_06bbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBFE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BC1C;
    cpu.a[0] = v1;
    4
}
/// $06BBFE: lsl.w #$2, d0
pub fn i_06bbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC00;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BC00: movea.l (a0, d0.w), a0
pub fn i_06bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC04;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BC04: cmpa.l #$ffffffff, a0
pub fn i_06bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC0A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BC0A: beq.w $6bc14
pub fn i_06bc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BC14; return 10; }
    8
}
/// $06BC0E: jsr $28cd4.l
pub fn i_06bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BC14);
    cpu.pc = 0x28CD4;
    16
}
/// $06BC14: jsr $6d484(pc)
pub fn i_06bc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC18);
    cpu.pc = 0x6D484;
    16
}
/// $06BC18: bra.w $6bc2c
pub fn i_06bc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BC2C;
    10
}
/// $06BC2C: bra.w $6bc74
pub fn i_06bc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC30;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BC74;
    10
}
/// $06BC30: jsr $6d1dc(pc)
pub fn i_06bc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC34);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BC34: bcs.w $6c272
pub fn i_06bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC38;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BC38: move.b d0, $97(a6)
pub fn i_06bc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC3C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BC3C: andi.w #$3, d0
pub fn i_06bc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC40;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BC40: movea.l #$6bc64, a0
pub fn i_06bc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC46;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BC64;
    cpu.a[0] = v1;
    4
}
/// $06BC46: lsl.w #$2, d0
pub fn i_06bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC48;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BC48: movea.l (a0, d0.w), a0
pub fn i_06bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC4C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BC4C: cmpa.l #$ffffffff, a0
pub fn i_06bc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC52;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BC52: beq.w $6bc5c
pub fn i_06bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BC5C; return 10; }
    8
}
/// $06BC56: jsr $28cd4.l
pub fn i_06bc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BC5C);
    cpu.pc = 0x28CD4;
    16
}
/// $06BC5C: jsr $6d484(pc)
pub fn i_06bc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC60);
    cpu.pc = 0x6D484;
    16
}
/// $06BC60: bra.w $6bc74
pub fn i_06bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC64;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BC74;
    10
}
/// $06BC74: jsr $28d70.l
pub fn i_06bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BC7A);
    cpu.pc = 0x28D70;
    16
}
/// $06BC7A: jsr $6d172(pc)
pub fn i_06bc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC7E);
    cpu.pc = 0x6D172;
    16
}
/// $06BC7E: bcc.w $6bc92
pub fn i_06bc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BC92; return 10; }
    8
}
/// $06BC82: eori.b #$1, $8d(a6)
pub fn i_06bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC88;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BC88: lea.l $6bcfc(pc), a1
pub fn i_06bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BC8C: move.l a1, (a6)
pub fn i_06bc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BC8E: bra.w $6bcda
pub fn i_06bc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC92;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BCDA;
    10
}
/// $06BC92: cmpi.w #$30, $22(a6)
pub fn i_06bc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC98;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $06BC98: bgt.w $6bcb6
pub fn i_06bc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC9C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6BCB6; return 10; }
    8
}
/// $06BC9C: btst.b #$0, $8d(a6)
pub fn i_06bc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCA2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06BCA2: bne.w $6bcb6
pub fn i_06bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6BCB6; return 10; }
    8
}
/// $06BCA6: ori.b #$1, $8d(a6)
pub fn i_06bca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCAC;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BCAC: clr.w $28(a6)
pub fn i_06bcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCB0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BCB0: lea.l $6bcfc(pc), a1
pub fn i_06bcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BCB4: move.l a1, (a6)
pub fn i_06bcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BCB6: cmpi.w #$110, $22(a6)
pub fn i_06bcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCBC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $06BCBC: blt.w $6bcda
pub fn i_06bcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCC0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6BCDA; return 10; }
    8
}
/// $06BCC0: btst.b #$0, $8d(a6)
pub fn i_06bcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCC6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06BCC6: beq.w $6bcda
pub fn i_06bcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BCDA; return 10; }
    8
}
/// $06BCCA: andi.b #$fe, $8d(a6)
pub fn i_06bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCD0;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BCD0: clr.w $28(a6)
pub fn i_06bcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCD4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BCD4: lea.l $6bcfc(pc), a1
pub fn i_06bcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BCD8: move.l a1, (a6)
pub fn i_06bcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BCDA: jsr $6d228(pc)
pub fn i_06bcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BCDE);
    cpu.pc = 0x6D228;
    16
}
/// $06BCDE: bcc.w $6bcec
pub fn i_06bcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BCEC; return 10; }
    8
}
/// $06BCE2: clr.w $28(a6)
pub fn i_06bce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCE6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BCE6: lea.l $6bcfc(pc), a1
pub fn i_06bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BCEA: move.l a1, (a6)
pub fn i_06bcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BCEC: jsr $283ca.l
pub fn i_06bcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BCF2);
    cpu.pc = 0x283CA;
    16
}
/// $06BCF2: jsr $283d8.l
pub fn i_06bcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BCF8);
    cpu.pc = 0x283D8;
    16
}
/// $06BCF8: bra.w $6c418
pub fn i_06bcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Stop_06bcfc  ($06BCFC, 74 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BCFC: move.b #$0, $72(a6)
pub fn i_06bcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BD02: jsr $6d1dc(pc)
pub fn i_06bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD06);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BD06: bcs.w $6c272
pub fn i_06bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD0A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BD0A: jsr $6d206(pc)
pub fn i_06bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD0E);
    cpu.pc = 0x6D206;
    16
}
/// $06BD0E: bcc.w $6bd5a
pub fn i_06bd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BD5A; return 10; }
    8
}
/// $06BD12: jsr $6d1dc(pc)
pub fn i_06bd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD16);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BD16: bcs.w $6c272
pub fn i_06bd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD1A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BD1A: move.b d0, $97(a6)
pub fn i_06bd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD1E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BD1E: andi.w #$3, d0
pub fn i_06bd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD22;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BD22: movea.l #$6bd46, a0
pub fn i_06bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD28;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BD46;
    cpu.a[0] = v1;
    4
}
/// $06BD28: lsl.w #$2, d0
pub fn i_06bd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD2A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BD2A: movea.l (a0, d0.w), a0
pub fn i_06bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD2E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BD2E: cmpa.l #$ffffffff, a0
pub fn i_06bd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD34;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BD34: beq.w $6bd3e
pub fn i_06bd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BD3E; return 10; }
    8
}
/// $06BD38: jsr $28cd4.l
pub fn i_06bd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BD3E);
    cpu.pc = 0x28CD4;
    16
}
/// $06BD3E: jsr $6d484(pc)
pub fn i_06bd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD42);
    cpu.pc = 0x6D484;
    16
}
/// $06BD42: bra.w $6bd56
pub fn i_06bd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD46;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BD56;
    10
}
/// $06BD56: bra.w $6bd9e
pub fn i_06bd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BD9E;
    10
}
/// $06BD5A: jsr $6d1dc(pc)
pub fn i_06bd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD5E);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BD5E: bcs.w $6c272
pub fn i_06bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD62;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BD62: move.b d0, $97(a6)
pub fn i_06bd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD66;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BD66: andi.w #$3, d0
pub fn i_06bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD6A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BD6A: movea.l #$6bd8e, a0
pub fn i_06bd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD70;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BD8E;
    cpu.a[0] = v1;
    4
}
/// $06BD70: lsl.w #$2, d0
pub fn i_06bd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD72;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BD72: movea.l (a0, d0.w), a0
pub fn i_06bd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD76;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BD76: cmpa.l #$ffffffff, a0
pub fn i_06bd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD7C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BD7C: beq.w $6bd86
pub fn i_06bd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BD86; return 10; }
    8
}
/// $06BD80: jsr $28cd4.l
pub fn i_06bd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BD86);
    cpu.pc = 0x28CD4;
    16
}
/// $06BD86: jsr $6d484(pc)
pub fn i_06bd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD8A);
    cpu.pc = 0x6D484;
    16
}
/// $06BD8A: bra.w $6bd9e
pub fn i_06bd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BD9E;
    10
}
/// $06BD9E: clr.w $28(a6)
pub fn i_06bd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDA2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BDA2: lea.l $6bda8(pc), a1
pub fn i_06bda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BDA8;
    16
}
/// $06BDA6: move.l a1, (a6)
pub fn i_06bda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BDA8: jsr $28998.l
pub fn i_06bda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDAE);
    cpu.pc = 0x28998;
    16
}
/// $06BDAE: jsr $6d0f6(pc)
pub fn i_06bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDB2);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BDB2: jsr $6d1dc(pc)
pub fn i_06bdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDB6);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BDB6: bcs.w $6bdc2
pub fn i_06bdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDBA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BDC2; return 10; }
    8
}
/// $06BDBA: cmp.b $97(a6), d0
pub fn i_06bdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDBE;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BDBE: beq.w $6bdc8
pub fn i_06bdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BDC8; return 10; }
    8
}
/// $06BDC2: lea.l $6bee0(pc), a1
pub fn i_06bdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06BDC6: move.l a1, (a6)
pub fn i_06bdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BDC8: jsr $28d70.l
pub fn i_06bdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDCE);
    cpu.pc = 0x28D70;
    16
}
/// $06BDCE: bcc.w $6bdd8
pub fn i_06bdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BDD8; return 10; }
    8
}
/// $06BDD2: lea.l $6bee0(pc), a1
pub fn i_06bdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06BDD6: move.l a1, (a6)
pub fn i_06bdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BDD8: jsr $283ca.l
pub fn i_06bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDDE);
    cpu.pc = 0x283CA;
    16
}
/// $06BDDE: jsr $283d8.l
pub fn i_06bdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDE4);
    cpu.pc = 0x283D8;
    16
}
/// $06BDE4: bra.w $6c418
pub fn i_06bde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Turn_06bde8  ($06BDE8, 74 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BDE8: move.b #$0, $72(a6)
pub fn i_06bde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDEE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BDEE: jsr $6d1dc(pc)
pub fn i_06bdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDF2);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BDF2: bcs.w $6c272
pub fn i_06bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BDF6: jsr $6d206(pc)
pub fn i_06bdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDFA);
    cpu.pc = 0x6D206;
    16
}
/// $06BDFA: bcs.w $6be46
pub fn i_06bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDFE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BE46; return 10; }
    8
}
/// $06BDFE: jsr $6d1dc(pc)
pub fn i_06bdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE02);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BE02: bcs.w $6c272
pub fn i_06be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE06;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BE06: move.b d0, $97(a6)
pub fn i_06be06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE0A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BE0A: andi.w #$3, d0
pub fn i_06be0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE0E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BE0E: movea.l #$6be32, a0
pub fn i_06be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE14;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BE32;
    cpu.a[0] = v1;
    4
}
/// $06BE14: lsl.w #$2, d0
pub fn i_06be14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE16;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BE16: movea.l (a0, d0.w), a0
pub fn i_06be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE1A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BE1A: cmpa.l #$ffffffff, a0
pub fn i_06be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE20;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BE20: beq.w $6be2a
pub fn i_06be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BE2A; return 10; }
    8
}
/// $06BE24: jsr $28cd4.l
pub fn i_06be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BE2A);
    cpu.pc = 0x28CD4;
    16
}
/// $06BE2A: jsr $6d484(pc)
pub fn i_06be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE2E);
    cpu.pc = 0x6D484;
    16
}
/// $06BE2E: bra.w $6be42
pub fn i_06be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE32;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BE42;
    10
}
/// $06BE42: bra.w $6be8a
pub fn i_06be42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE46;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BE8A;
    10
}
/// $06BE46: jsr $6d1dc(pc)
pub fn i_06be46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE4A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BE4A: bcs.w $6c272
pub fn i_06be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE4E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BE4E: move.b d0, $97(a6)
pub fn i_06be4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE52;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BE52: andi.w #$3, d0
pub fn i_06be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE56;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BE56: movea.l #$6be7a, a0
pub fn i_06be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE5C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BE7A;
    cpu.a[0] = v1;
    4
}
/// $06BE5C: lsl.w #$2, d0
pub fn i_06be5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE5E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BE5E: movea.l (a0, d0.w), a0
pub fn i_06be5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE62;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BE62: cmpa.l #$ffffffff, a0
pub fn i_06be62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE68;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BE68: beq.w $6be72
pub fn i_06be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BE72; return 10; }
    8
}
/// $06BE6C: jsr $28cd4.l
pub fn i_06be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BE72);
    cpu.pc = 0x28CD4;
    16
}
/// $06BE72: jsr $6d484(pc)
pub fn i_06be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE76);
    cpu.pc = 0x6D484;
    16
}
/// $06BE76: bra.w $6be8a
pub fn i_06be76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BE8A;
    10
}
/// $06BE8A: lea.l $6be90(pc), a1
pub fn i_06be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BE90;
    16
}
/// $06BE8E: move.l a1, (a6)
pub fn i_06be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BE90: jsr $28998.l
pub fn i_06be90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BE96);
    cpu.pc = 0x28998;
    16
}
/// $06BE96: jsr $6d0f6(pc)
pub fn i_06be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE9A);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BE9A: bcc.w $6beaa
pub fn i_06be9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE9E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BEAA; return 10; }
    8
}
/// $06BE9E: eori.b #$1, $8d(a6)
pub fn i_06be9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEA4;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BEA4: lea.l $6bcfc(pc), a1
pub fn i_06bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BEA8: move.l a1, (a6)
pub fn i_06bea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BEAA: jsr $6d1dc(pc)
pub fn i_06beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BEAE);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BEAE: bcs.w $6beba
pub fn i_06beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEB2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BEBA; return 10; }
    8
}
/// $06BEB2: cmp.b $97(a6), d0
pub fn i_06beb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEB6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BEB6: beq.w $6bec0
pub fn i_06beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BEC0; return 10; }
    8
}
/// $06BEBA: lea.l $6bee0(pc), a1
pub fn i_06beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06BEBE: move.l a1, (a6)
pub fn i_06bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BEC0: jsr $28d70.l
pub fn i_06bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BEC6);
    cpu.pc = 0x28D70;
    16
}
/// $06BEC6: bcc.w $6bed0
pub fn i_06bec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BECA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BED0; return 10; }
    8
}
/// $06BECA: lea.l $6bb02(pc), a1
pub fn i_06beca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BECE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BB02;
    16
}
/// $06BECE: move.l a1, (a6)
pub fn i_06bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BED0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BED0: jsr $283ca.l
pub fn i_06bed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BED6);
    cpu.pc = 0x283CA;
    16
}
/// $06BED6: jsr $283d8.l
pub fn i_06bed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BEDC);
    cpu.pc = 0x283D8;
    16
}
/// $06BEDC: bra.w $6c418
pub fn i_06bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Idle_06bee0  ($06BEE0, 86 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BEE0: move.b #$0, $72(a6)
pub fn i_06bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BEE6: lea.l $2bd670.l, a0
pub fn i_06bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD670;
    20
}
/// $06BEEC: jsr $799de.l
pub fn i_06beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BEF2);
    cpu.pc = 0x799DE;
    16
}
/// $06BEF2: move.b d0, $88(a6)
pub fn i_06bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEF6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BEF6: jsr $6d1dc(pc)
pub fn i_06bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BEFA);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BEFA: bcs.w $6c272
pub fn i_06befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEFE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BEFE: clr.w $28(a6)
pub fn i_06befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BF02: jsr $6d1dc(pc)
pub fn i_06bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF06);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BF06: bcs.w $6c272
pub fn i_06bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF0A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BF0A: move.b d0, $97(a6)
pub fn i_06bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF0E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BF0E: andi.w #$3, d0
pub fn i_06bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF12;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BF12: movea.l #$6bf36, a0
pub fn i_06bf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF18;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BF36;
    cpu.a[0] = v1;
    4
}
/// $06BF18: lsl.w #$2, d0
pub fn i_06bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF1A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BF1A: movea.l (a0, d0.w), a0
pub fn i_06bf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF1E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BF1E: cmpa.l #$ffffffff, a0
pub fn i_06bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF24;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BF24: beq.w $6bf2e
pub fn i_06bf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BF2E; return 10; }
    8
}
/// $06BF28: jsr $28cd4.l
pub fn i_06bf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BF2E);
    cpu.pc = 0x28CD4;
    16
}
/// $06BF2E: jsr $6d484(pc)
pub fn i_06bf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF32);
    cpu.pc = 0x6D484;
    16
}
/// $06BF32: bra.w $6bf46
pub fn i_06bf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF36;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BF46;
    10
}
/// $06BF46: lea.l $6bf4c(pc), a1
pub fn i_06bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BF4C;
    16
}
/// $06BF4A: move.l a1, (a6)
pub fn i_06bf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BF4C: jsr $28998.l
pub fn i_06bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BF52);
    cpu.pc = 0x28998;
    16
}
/// $06BF52: jsr $6d0f6(pc)
pub fn i_06bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF56);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BF56: jsr $6d1dc(pc)
pub fn i_06bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF5A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BF5A: bcs.w $6bf66
pub fn i_06bf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF5E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BF66; return 10; }
    8
}
/// $06BF5E: cmp.b $97(a6), d0
pub fn i_06bf5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF62;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BF62: beq.w $6bfaa
pub fn i_06bf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BFAA; return 10; }
    8
}
/// $06BF66: jsr $6d1dc(pc)
pub fn i_06bf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF6A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BF6A: bcs.w $6c272
pub fn i_06bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF6E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BF6E: move.b d0, $97(a6)
pub fn i_06bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF72;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BF72: andi.w #$3, d0
pub fn i_06bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF76;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BF76: movea.l #$6bf9a, a0
pub fn i_06bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF7C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BF9A;
    cpu.a[0] = v1;
    4
}
/// $06BF7C: lsl.w #$2, d0
pub fn i_06bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF7E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BF7E: movea.l (a0, d0.w), a0
pub fn i_06bf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF82;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BF82: cmpa.l #$ffffffff, a0
pub fn i_06bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF88;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BF88: beq.w $6bf92
pub fn i_06bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BF92; return 10; }
    8
}
/// $06BF8C: jsr $28cd4.l
pub fn i_06bf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BF92);
    cpu.pc = 0x28CD4;
    16
}
/// $06BF92: jsr $6d484(pc)
pub fn i_06bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF96);
    cpu.pc = 0x6D484;
    16
}
/// $06BF96: bra.w $6bfaa
pub fn i_06bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BFAA;
    10
}
/// $06BFAA: jsr $28d70.l
pub fn i_06bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BFB0);
    cpu.pc = 0x28D70;
    16
}
/// $06BFB0: subq.b #$1, $88(a6)
pub fn i_06bfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFB4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BFB4: bgt.w $6bfbe
pub fn i_06bfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFB8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6BFBE; return 10; }
    8
}
/// $06BFB8: lea.l $6bde8(pc), a1
pub fn i_06bfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BDE8;
    16
}
/// $06BFBC: move.l a1, (a6)
pub fn i_06bfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BFBE: jsr $6d228(pc)
pub fn i_06bfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BFC2);
    cpu.pc = 0x6D228;
    16
}
/// $06BFC2: bcc.w $6bfcc
pub fn i_06bfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BFCC; return 10; }
    8
}
/// $06BFC6: lea.l $6bfd0(pc), a1
pub fn i_06bfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BFD0;
    16
}
/// $06BFCA: move.l a1, (a6)
pub fn i_06bfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BFCC: bra.w $6c418
pub fn i_06bfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Alert_06bfd0  ($06BFD0, 90 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BFD0: jsr $267e2.l
pub fn i_06bfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BFD6);
    cpu.pc = 0x267E2;
    16
}
/// $06BFD6: jsr $5e0d4.l
pub fn i_06bfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BFDC);
    cpu.pc = 0x5E0D4;
    16
}
/// $06BFDC: move.l a0, $90(a6)
pub fn i_06bfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFE0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06BFE0: move.b #$2, $72(a6)
pub fn i_06bfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BFE6: clr.b $88(a6)
pub fn i_06bfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFEA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BFEA: clr.b $73(a6)
pub fn i_06bfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFEE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BFEE: jsr $6d1dc(pc)
pub fn i_06bfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BFF2);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BFF2: bcs.w $6c272
pub fn i_06bff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BFF6: jsr $6d1dc(pc)
pub fn i_06bff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BFFA);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BFFA: bcs.w $6c272
pub fn i_06bffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFFE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BFFE: move.b d0, $97(a6)
pub fn i_06bffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C002;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C002: andi.w #$3, d0
pub fn i_06c002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C006;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C006: movea.l #$6c02a, a0
pub fn i_06c006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C00C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C02A;
    cpu.a[0] = v1;
    4
}
/// $06C00C: lsl.w #$2, d0
pub fn i_06c00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C00E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C00E: movea.l (a0, d0.w), a0
pub fn i_06c00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C012;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x06A5BE, i_06a5be),
    (0x06A5C2, i_06a5c2),
    (0x06A5C6, i_06a5c6),
    (0x06A5C8, i_06a5c8),
    (0x06A5CC, i_06a5cc),
    (0x06A5D0, i_06a5d0),
    (0x06A5D4, i_06a5d4),
    (0x06A5DA, i_06a5da),
    (0x06A5E0, i_06a5e0),
    (0x06A5E6, i_06a5e6),
    (0x06A5EA, i_06a5ea),
    (0x06A5F0, i_06a5f0),
    (0x06A5F6, i_06a5f6),
    (0x06A5FA, i_06a5fa),
    (0x06A5FC, i_06a5fc),
    (0x06A600, i_06a600),
    (0x06A606, i_06a606),
    (0x06A60A, i_06a60a),
    (0x06A610, i_06a610),
    (0x06A614, i_06a614),
    (0x06A618, i_06a618),
    (0x06A61A, i_06a61a),
    (0x06A636, i_06a636),
    (0x06A63A, i_06a63a),
    (0x06A640, i_06a640),
    (0x06A644, i_06a644),
    (0x06A648, i_06a648),
    (0x06A64E, i_06a64e),
    (0x06A654, i_06a654),
    (0x06A65A, i_06a65a),
    (0x06A65E, i_06a65e),
    (0x06A664, i_06a664),
    (0x06A66A, i_06a66a),
    (0x06A66E, i_06a66e),
    (0x06A670, i_06a670),
    (0x06A674, i_06a674),
    (0x06A67A, i_06a67a),
    (0x06A67E, i_06a67e),
    (0x06A682, i_06a682),
    (0x06A684, i_06a684),
    (0x06A688, i_06a688),
    (0x06A68C, i_06a68c),
    (0x06A692, i_06a692),
    (0x06A696, i_06a696),
    (0x06A698, i_06a698),
    (0x06A69E, i_06a69e),
    (0x06A6A2, i_06a6a2),
    (0x06A6A6, i_06a6a6),
    (0x06A6AA, i_06a6aa),
    (0x06A6B0, i_06a6b0),
    (0x06A6B4, i_06a6b4),
    (0x06A6B8, i_06a6b8),
    (0x06A6BC, i_06a6bc),
    (0x06A6BE, i_06a6be),
    (0x06A6C2, i_06a6c2),
    (0x06A6C8, i_06a6c8),
    (0x06A6CC, i_06a6cc),
    (0x06A6D2, i_06a6d2),
    (0x06A6D8, i_06a6d8),
    (0x06A6DC, i_06a6dc),
    (0x06A6E0, i_06a6e0),
    (0x06A6E4, i_06a6e4),
    (0x06A6EA, i_06a6ea),
    (0x06A6EE, i_06a6ee),
    (0x06A6F2, i_06a6f2),
    (0x06A6F8, i_06a6f8),
    (0x06A6FE, i_06a6fe),
    (0x06A704, i_06a704),
    (0x06A708, i_06a708),
    (0x06A70E, i_06a70e),
    (0x06A714, i_06a714),
    (0x06A718, i_06a718),
    (0x06A71A, i_06a71a),
    (0x06A71E, i_06a71e),
    (0x06A724, i_06a724),
    (0x06A728, i_06a728),
    (0x06A72C, i_06a72c),
    (0x06A730, i_06a730),
    (0x06A734, i_06a734),
    (0x06A73A, i_06a73a),
    (0x06A73E, i_06a73e),
    (0x06A744, i_06a744),
    (0x06A748, i_06a748),
    (0x06A74C, i_06a74c),
    (0x06A74E, i_06a74e),
    (0x06A752, i_06a752),
    (0x06A758, i_06a758),
    (0x06A75E, i_06a75e),
    (0x06A762, i_06a762),
    (0x06A76A, i_06a76a),
    (0x06A76E, i_06a76e),
    (0x06A772, i_06a772),
    (0x06A776, i_06a776),
    (0x06A77A, i_06a77a),
    (0x06A780, i_06a780),
    (0x06A784, i_06a784),
    (0x06A78A, i_06a78a),
    (0x06A790, i_06a790),
    (0x06A796, i_06a796),
    (0x06A79C, i_06a79c),
    (0x06A7A2, i_06a7a2),
    (0x06A7A6, i_06a7a6),
    (0x06A7AA, i_06a7aa),
    (0x06A7AC, i_06a7ac),
    (0x06A7B0, i_06a7b0),
    (0x06A7B4, i_06a7b4),
    (0x06A7B8, i_06a7b8),
    (0x06A7BA, i_06a7ba),
    (0x06A7BC, i_06a7bc),
    (0x06A7C2, i_06a7c2),
    (0x06A7C6, i_06a7c6),
    (0x06A7CA, i_06a7ca),
    (0x06A7CE, i_06a7ce),
    (0x06A7D2, i_06a7d2),
    (0x06A7D6, i_06a7d6),
    (0x06A7DA, i_06a7da),
    (0x06A7E4, i_06a7e4),
    (0x06A7E8, i_06a7e8),
    (0x06A7EC, i_06a7ec),
    (0x06A7F2, i_06a7f2),
    (0x06A7F8, i_06a7f8),
    (0x06A7FC, i_06a7fc),
    (0x06A802, i_06a802),
    (0x06A806, i_06a806),
    (0x06A80C, i_06a80c),
    (0x06A810, i_06a810),
    (0x06A816, i_06a816),
    (0x06A81C, i_06a81c),
    (0x06A820, i_06a820),
    (0x06A822, i_06a822),
    (0x06A826, i_06a826),
    (0x06A82C, i_06a82c),
    (0x06A830, i_06a830),
    (0x06A834, i_06a834),
    (0x06A838, i_06a838),
    (0x06A83C, i_06a83c),
    (0x06A840, i_06a840),
    (0x06A842, i_06a842),
    (0x06A848, i_06a848),
    (0x06A84C, i_06a84c),
    (0x06A850, i_06a850),
    (0x06A854, i_06a854),
    (0x06A85A, i_06a85a),
    (0x06A85E, i_06a85e),
    (0x06A862, i_06a862),
    (0x06A864, i_06a864),
    (0x06A868, i_06a868),
    (0x06A86E, i_06a86e),
    (0x06A874, i_06a874),
    (0x06A878, i_06a878),
    (0x06A87E, i_06a87e),
    (0x06A884, i_06a884),
    (0x06A888, i_06a888),
    (0x06A88C, i_06a88c),
    (0x06A892, i_06a892),
    (0x06A898, i_06a898),
    (0x06A89E, i_06a89e),
    (0x06A8A2, i_06a8a2),
    (0x06A8A4, i_06a8a4),
    (0x06A8A8, i_06a8a8),
    (0x06A8AE, i_06a8ae),
    (0x06A8B2, i_06a8b2),
    (0x06A8B8, i_06a8b8),
    (0x06A8BE, i_06a8be),
    (0x06A8C2, i_06a8c2),
    (0x06A8C8, i_06a8c8),
    (0x06A8CC, i_06a8cc),
    (0x06A8D0, i_06a8d0),
    (0x06A8D4, i_06a8d4),
    (0x06A8DA, i_06a8da),
    (0x06A8DE, i_06a8de),
    (0x06A8E0, i_06a8e0),
    (0x06A8E4, i_06a8e4),
    (0x06A8EA, i_06a8ea),
    (0x06A8EE, i_06a8ee),
    (0x06A8F4, i_06a8f4),
    (0x06A8FA, i_06a8fa),
    (0x06A8FE, i_06a8fe),
    (0x06A902, i_06a902),
    (0x06A904, i_06a904),
    (0x06A908, i_06a908),
    (0x06A90E, i_06a90e),
    (0x06A914, i_06a914),
    (0x06A91A, i_06a91a),
    (0x06A91E, i_06a91e),
    (0x06A920, i_06a920),
    (0x06A924, i_06a924),
    (0x06A92A, i_06a92a),
    (0x06A92E, i_06a92e),
    (0x06A934, i_06a934),
    (0x06A938, i_06a938),
    (0x06A93A, i_06a93a),
    (0x06A93E, i_06a93e),
    (0x06A944, i_06a944),
    (0x06A94A, i_06a94a),
    (0x06A950, i_06a950),
    (0x06A954, i_06a954),
    (0x06A956, i_06a956),
    (0x06A95A, i_06a95a),
    (0x06A960, i_06a960),
    (0x06A964, i_06a964),
    (0x06A968, i_06a968),
    (0x06A96A, i_06a96a),
    (0x06A96C, i_06a96c),
    (0x06A970, i_06a970),
    (0x06A974, i_06a974),
    (0x06A97A, i_06a97a),
    (0x06A97E, i_06a97e),
    (0x06A984, i_06a984),
    (0x06A988, i_06a988),
    (0x06A98E, i_06a98e),
    (0x06A994, i_06a994),
    (0x06A996, i_06a996),
    (0x06A99C, i_06a99c),
    (0x06A9A0, i_06a9a0),
    (0x06A9A6, i_06a9a6),
    (0x06A9AC, i_06a9ac),
    (0x06A9B2, i_06a9b2),
    (0x06A9B6, i_06a9b6),
    (0x06A9BC, i_06a9bc),
    (0x06A9C2, i_06a9c2),
    (0x06A9C6, i_06a9c6),
    (0x06A9CC, i_06a9cc),
    (0x06A9D0, i_06a9d0),
    (0x06A9D6, i_06a9d6),
    (0x06A9DA, i_06a9da),
    (0x06A9DC, i_06a9dc),
    (0x06A9E2, i_06a9e2),
    (0x06A9E8, i_06a9e8),
    (0x06A9EC, i_06a9ec),
    (0x06A9F0, i_06a9f0),
    (0x06A9F2, i_06a9f2),
    (0x06A9F8, i_06a9f8),
    (0x06A9FC, i_06a9fc),
    (0x06AA00, i_06aa00),
    (0x06AA02, i_06aa02),
    (0x06AA04, i_06aa04),
    (0x06AA08, i_06aa08),
    (0x06AA0C, i_06aa0c),
    (0x06AA10, i_06aa10),
    (0x06AA28, i_06aa28),
    (0x06AA2E, i_06aa2e),
    (0x06AA32, i_06aa32),
    (0x06AA36, i_06aa36),
    (0x06AA3A, i_06aa3a),
    (0x06AA3C, i_06aa3c),
    (0x06AA40, i_06aa40),
    (0x06AA44, i_06aa44),
    (0x06AA48, i_06aa48),
    (0x06AA4C, i_06aa4c),
    (0x06AA52, i_06aa52),
    (0x06AA58, i_06aa58),
    (0x06AA5C, i_06aa5c),
    (0x06AA62, i_06aa62),
    (0x06AA66, i_06aa66),
    (0x06AA6C, i_06aa6c),
    (0x06AA70, i_06aa70),
    (0x06AA76, i_06aa76),
    (0x06AA7C, i_06aa7c),
    (0x06AA80, i_06aa80),
    (0x06AA82, i_06aa82),
    (0x06AA88, i_06aa88),
    (0x06AA8E, i_06aa8e),
    (0x06AA92, i_06aa92),
    (0x06AA96, i_06aa96),
    (0x06AA9A, i_06aa9a),
    (0x06AA9C, i_06aa9c),
    (0x06AAA0, i_06aaa0),
    (0x06AAA4, i_06aaa4),
    (0x06AAAA, i_06aaaa),
    (0x06AAAE, i_06aaae),
    (0x06AAB2, i_06aab2),
    (0x06AAB6, i_06aab6),
    (0x06AABC, i_06aabc),
    (0x06AAC0, i_06aac0),
    (0x06AAC4, i_06aac4),
    (0x06AAC6, i_06aac6),
    (0x06AACA, i_06aaca),
    (0x06AAD0, i_06aad0),
    (0x06AAD6, i_06aad6),
    (0x06AADA, i_06aada),
    (0x06AAE0, i_06aae0),
    (0x06AAE6, i_06aae6),
    (0x06AAEA, i_06aaea),
    (0x06AAEE, i_06aaee),
    (0x06AAF4, i_06aaf4),
    (0x06AAFA, i_06aafa),
    (0x06AB00, i_06ab00),
    (0x06AB04, i_06ab04),
    (0x06AB06, i_06ab06),
    (0x06AB0C, i_06ab0c),
    (0x06AB12, i_06ab12),
    (0x06AB16, i_06ab16),
    (0x06AB1C, i_06ab1c),
    (0x06AB22, i_06ab22),
    (0x06AB26, i_06ab26),
    (0x06AB2C, i_06ab2c),
    (0x06AB30, i_06ab30),
    (0x06AB34, i_06ab34),
    (0x06AB38, i_06ab38),
    (0x06AB3E, i_06ab3e),
    (0x06AB42, i_06ab42),
    (0x06AB44, i_06ab44),
    (0x06AB48, i_06ab48),
    (0x06AB4E, i_06ab4e),
    (0x06AB52, i_06ab52),
    (0x06AB58, i_06ab58),
    (0x06AB5E, i_06ab5e),
    (0x06AB62, i_06ab62),
    (0x06AB66, i_06ab66),
    (0x06AB68, i_06ab68),
    (0x06AB6C, i_06ab6c),
    (0x06AB72, i_06ab72),
    (0x06AB78, i_06ab78),
    (0x06AB7E, i_06ab7e),
    (0x06AB82, i_06ab82),
    (0x06AB84, i_06ab84),
    (0x06AB8A, i_06ab8a),
    (0x06AB90, i_06ab90),
    (0x06AB94, i_06ab94),
    (0x06AB9A, i_06ab9a),
    (0x06AB9E, i_06ab9e),
    (0x06ABA0, i_06aba0),
    (0x06ABA4, i_06aba4),
    (0x06ABAA, i_06abaa),
    (0x06ABB0, i_06abb0),
    (0x06ABB6, i_06abb6),
    (0x06ABBA, i_06abba),
    (0x06ABBC, i_06abbc),
    (0x06ABC2, i_06abc2),
    (0x06ABC8, i_06abc8),
    (0x06ABCC, i_06abcc),
    (0x06ABD0, i_06abd0),
    (0x06ABD2, i_06abd2),
    (0x06ABD6, i_06abd6),
    (0x06ABDA, i_06abda),
    (0x06ABDE, i_06abde),
    (0x06ABE0, i_06abe0),
    (0x06ABE2, i_06abe2),
    (0x06ABE6, i_06abe6),
    (0x06ABEC, i_06abec),
    (0x06ABF2, i_06abf2),
    (0x06ABF8, i_06abf8),
    (0x06ABFC, i_06abfc),
    (0x06AC02, i_06ac02),
    (0x06AC08, i_06ac08),
    (0x06AC0C, i_06ac0c),
    (0x06AC12, i_06ac12),
    (0x06AC16, i_06ac16),
    (0x06AC1C, i_06ac1c),
    (0x06AC20, i_06ac20),
    (0x06AC22, i_06ac22),
    (0x06AC28, i_06ac28),
    (0x06AC2E, i_06ac2e),
    (0x06AC32, i_06ac32),
    (0x06AC36, i_06ac36),
    (0x06AC38, i_06ac38),
    (0x06AC3C, i_06ac3c),
    (0x06AC42, i_06ac42),
    (0x06AC46, i_06ac46),
    (0x06AC4A, i_06ac4a),
    (0x06AC4C, i_06ac4c),
    (0x06AC50, i_06ac50),
    (0x06AC54, i_06ac54),
    (0x06AC58, i_06ac58),
    (0x06AC5A, i_06ac5a),
    (0x06AC5C, i_06ac5c),
    (0x06AC60, i_06ac60),
    (0x06AC64, i_06ac64),
    (0x06AC6A, i_06ac6a),
    (0x06AC70, i_06ac70),
    (0x06AC76, i_06ac76),
    (0x06AC7A, i_06ac7a),
    (0x06AC7E, i_06ac7e),
    (0x06AC80, i_06ac80),
    (0x06AC86, i_06ac86),
    (0x06AC8A, i_06ac8a),
    (0x06AC90, i_06ac90),
    (0x06AC94, i_06ac94),
    (0x06AC98, i_06ac98),
    (0x06AC9A, i_06ac9a),
    (0x06AC9E, i_06ac9e),
    (0x06ACA2, i_06aca2),
    (0x06ACA6, i_06aca6),
    (0x06ACA8, i_06aca8),
    (0x06ACAA, i_06acaa),
    (0x06ACB2, i_06acb2),
    (0x06ACB6, i_06acb6),
    (0x06ACBC, i_06acbc),
    (0x06ACC2, i_06acc2),
    (0x06ACC6, i_06acc6),
    (0x06ACCA, i_06acca),
    (0x06ACD0, i_06acd0),
    (0x06ACD6, i_06acd6),
    (0x06ACDC, i_06acdc),
    (0x06ACE0, i_06ace0),
    (0x06ACE2, i_06ace2),
    (0x06ACE8, i_06ace8),
    (0x06ACEC, i_06acec),
    (0x06ACF0, i_06acf0),
    (0x06ACF4, i_06acf4),
    (0x06ACF6, i_06acf6),
    (0x06ACFA, i_06acfa),
    (0x06AD00, i_06ad00),
    (0x06AD04, i_06ad04),
    (0x06AD0A, i_06ad0a),
    (0x06AD10, i_06ad10),
    (0x06AD14, i_06ad14),
    (0x06AD18, i_06ad18),
    (0x06AD1C, i_06ad1c),
    (0x06AD22, i_06ad22),
    (0x06AD28, i_06ad28),
    (0x06AD2E, i_06ad2e),
    (0x06AD32, i_06ad32),
    (0x06AD34, i_06ad34),
    (0x06AD38, i_06ad38),
    (0x06AD3E, i_06ad3e),
    (0x06AD42, i_06ad42),
    (0x06AD46, i_06ad46),
    (0x06AD4A, i_06ad4a),
    (0x06AD4E, i_06ad4e),
    (0x06AD52, i_06ad52),
    (0x06AD56, i_06ad56),
    (0x06AD5A, i_06ad5a),
    (0x06AD5E, i_06ad5e),
    (0x06AD62, i_06ad62),
    (0x06AD68, i_06ad68),
    (0x06AD6E, i_06ad6e),
    (0x06AD72, i_06ad72),
    (0x06AD76, i_06ad76),
    (0x06AD7A, i_06ad7a),
    (0x06AD7C, i_06ad7c),
    (0x06AD80, i_06ad80),
    (0x06AD86, i_06ad86),
    (0x06AD8A, i_06ad8a),
    (0x06AD90, i_06ad90),
    (0x06AD96, i_06ad96),
    (0x06AD9A, i_06ad9a),
    (0x06ADA0, i_06ada0),
    (0x06ADA4, i_06ada4),
    (0x06ADA8, i_06ada8),
    (0x06ADAA, i_06adaa),
    (0x06ADAE, i_06adae),
    (0x06ADB4, i_06adb4),
    (0x06ADB8, i_06adb8),
    (0x06ADBE, i_06adbe),
    (0x06ADC2, i_06adc2),
    (0x06ADC6, i_06adc6),
    (0x06ADC8, i_06adc8),
    (0x06ADCE, i_06adce),
    (0x06ADD2, i_06add2),
    (0x06ADD6, i_06add6),
    (0x06ADDA, i_06adda),
    (0x06ADDC, i_06addc),
    (0x06ADE0, i_06ade0),
    (0x06ADE6, i_06ade6),
    (0x06ADEA, i_06adea),
    (0x06ADEE, i_06adee),
    (0x06ADF4, i_06adf4),
    (0x06ADF6, i_06adf6),
    (0x06ADFA, i_06adfa),
    (0x06AE00, i_06ae00),
    (0x06AE06, i_06ae06),
    (0x06AE0A, i_06ae0a),
    (0x06AE0C, i_06ae0c),
    (0x06AE10, i_06ae10),
    (0x06AE16, i_06ae16),
    (0x06AE1A, i_06ae1a),
    (0x06AE1E, i_06ae1e),
    (0x06AE24, i_06ae24),
    (0x06AE28, i_06ae28),
    (0x06AE2C, i_06ae2c),
    (0x06AE30, i_06ae30),
    (0x06AE34, i_06ae34),
    (0x06AE38, i_06ae38),
    (0x06AE3C, i_06ae3c),
    (0x06AE42, i_06ae42),
    (0x06AE48, i_06ae48),
    (0x06AE4C, i_06ae4c),
    (0x06AE50, i_06ae50),
    (0x06AE54, i_06ae54),
    (0x06AE56, i_06ae56),
    (0x06AE5A, i_06ae5a),
    (0x06AE60, i_06ae60),
    (0x06AE64, i_06ae64),
    (0x06AE6A, i_06ae6a),
    (0x06AE70, i_06ae70),
    (0x06AE74, i_06ae74),
    (0x06AE78, i_06ae78),
    (0x06AE7E, i_06ae7e),
    (0x06AE84, i_06ae84),
    (0x06AE88, i_06ae88),
    (0x06AE8A, i_06ae8a),
    (0x06AE90, i_06ae90),
    (0x06AE94, i_06ae94),
    (0x06AE98, i_06ae98),
    (0x06AE9C, i_06ae9c),
    (0x06AE9E, i_06ae9e),
    (0x06AEA2, i_06aea2),
    (0x06AEA8, i_06aea8),
    (0x06AEAC, i_06aeac),
    (0x06AEB2, i_06aeb2),
    (0x06AEB8, i_06aeb8),
    (0x06AEBC, i_06aebc),
    (0x06AEC0, i_06aec0),
    (0x06AEC4, i_06aec4),
    (0x06AEC8, i_06aec8),
    (0x06AECC, i_06aecc),
    (0x06AED0, i_06aed0),
    (0x06AED4, i_06aed4),
    (0x06AEDA, i_06aeda),
    (0x06AEE0, i_06aee0),
    (0x06AEE4, i_06aee4),
    (0x06AEE6, i_06aee6),
    (0x06AEEC, i_06aeec),
    (0x06AEF0, i_06aef0),
    (0x06AEF4, i_06aef4),
    (0x06AEF8, i_06aef8),
    (0x06AEFA, i_06aefa),
    (0x06AEFE, i_06aefe),
    (0x06AF04, i_06af04),
    (0x06AF08, i_06af08),
    (0x06AF0E, i_06af0e),
    (0x06AF14, i_06af14),
    (0x06AF18, i_06af18),
    (0x06AF1E, i_06af1e),
    (0x06AF24, i_06af24),
    (0x06AF28, i_06af28),
    (0x06AF2A, i_06af2a),
    (0x06AF30, i_06af30),
    (0x06AF34, i_06af34),
    (0x06AF38, i_06af38),
    (0x06AF3C, i_06af3c),
    (0x06AF3E, i_06af3e),
    (0x06AF42, i_06af42),
    (0x06AF46, i_06af46),
    (0x06AF4C, i_06af4c),
    (0x06AF4E, i_06af4e),
    (0x06AF52, i_06af52),
    (0x06AF56, i_06af56),
    (0x06AF5A, i_06af5a),
    (0x06AF60, i_06af60),
    (0x06AF66, i_06af66),
    (0x06AF68, i_06af68),
    (0x06AF6C, i_06af6c),
    (0x06AF72, i_06af72),
    (0x06AF76, i_06af76),
    (0x06AF7C, i_06af7c),
    (0x06AF82, i_06af82),
    (0x06AF88, i_06af88),
    (0x06AF8C, i_06af8c),
    (0x06AF92, i_06af92),
    (0x06AF98, i_06af98),
    (0x06AF9E, i_06af9e),
    (0x06AFA2, i_06afa2),
    (0x06AFA4, i_06afa4),
    (0x06AFAA, i_06afaa),
    (0x06AFB0, i_06afb0),
    (0x06AFB4, i_06afb4),
    (0x06AFB8, i_06afb8),
    (0x06AFBA, i_06afba),
    (0x06AFC0, i_06afc0),
    (0x06AFC6, i_06afc6),
    (0x06AFCC, i_06afcc),
    (0x06AFD0, i_06afd0),
    (0x06AFD4, i_06afd4),
    (0x06AFD6, i_06afd6),
    (0x06AFDC, i_06afdc),
    (0x06AFE0, i_06afe0),
    (0x06AFE4, i_06afe4),
    (0x06AFE6, i_06afe6),
    (0x06AFEA, i_06afea),
    (0x06AFEE, i_06afee),
    (0x06AFF2, i_06aff2),
    (0x06AFF4, i_06aff4),
    (0x06AFF6, i_06aff6),
    (0x06AFFA, i_06affa),
    (0x06AFFC, i_06affc),
    (0x06B002, i_06b002),
    (0x06B008, i_06b008),
    (0x06B00C, i_06b00c),
    (0x06B010, i_06b010),
    (0x06B012, i_06b012),
    (0x06B014, i_06b014),
    (0x06B018, i_06b018),
    (0x06B01E, i_06b01e),
    (0x06B024, i_06b024),
    (0x06B02A, i_06b02a),
    (0x06B032, i_06b032),
    (0x06B038, i_06b038),
    (0x06B03C, i_06b03c),
    (0x06B042, i_06b042),
    (0x06B048, i_06b048),
    (0x06B04E, i_06b04e),
    (0x06B052, i_06b052),
    (0x06B058, i_06b058),
    (0x06B05C, i_06b05c),
    (0x06B060, i_06b060),
    (0x06B082, i_06b082),
    (0x06B086, i_06b086),
    (0x06B08A, i_06b08a),
    (0x06B08C, i_06b08c),
    (0x06B08E, i_06b08e),
    (0x06B092, i_06b092),
    (0x06B096, i_06b096),
    (0x06B09A, i_06b09a),
    (0x06B0A0, i_06b0a0),
    (0x06B0A6, i_06b0a6),
    (0x06B0AA, i_06b0aa),
    (0x06B0B0, i_06b0b0),
    (0x06B0B4, i_06b0b4),
    (0x06B0B8, i_06b0b8),
    (0x06B0BA, i_06b0ba),
    (0x06B0BE, i_06b0be),
    (0x06B0C2, i_06b0c2),
    (0x06B0C6, i_06b0c6),
    (0x06B0C8, i_06b0c8),
    (0x06B0CA, i_06b0ca),
    (0x06B0CC, i_06b0cc),
    (0x06B0CE, i_06b0ce),
    (0x06B0D0, i_06b0d0),
    (0x06B0D2, i_06b0d2),
    (0x06B0D6, i_06b0d6),
    (0x06B0DC, i_06b0dc),
    (0x06B0E2, i_06b0e2),
    (0x06B0E8, i_06b0e8),
    (0x06B0EC, i_06b0ec),
    (0x06B0F2, i_06b0f2),
    (0x06B0F8, i_06b0f8),
    (0x06B0FE, i_06b0fe),
    (0x06B104, i_06b104),
    (0x06B108, i_06b108),
    (0x06B10E, i_06b10e),
    (0x06B114, i_06b114),
    (0x06B11A, i_06b11a),
    (0x06B11E, i_06b11e),
    (0x06B122, i_06b122),
    (0x06B126, i_06b126),
    (0x06B12A, i_06b12a),
    (0x06B12E, i_06b12e),
    (0x06B132, i_06b132),
    (0x06B136, i_06b136),
    (0x06B13A, i_06b13a),
    (0x06B140, i_06b140),
    (0x06B146, i_06b146),
    (0x06B14C, i_06b14c),
    (0x06B152, i_06b152),
    (0x06B156, i_06b156),
    (0x06B158, i_06b158),
    (0x06B15E, i_06b15e),
    (0x06B162, i_06b162),
    (0x06B166, i_06b166),
    (0x06B168, i_06b168),
    (0x06B16C, i_06b16c),
    (0x06B170, i_06b170),
    (0x06B172, i_06b172),
    (0x06B178, i_06b178),
    (0x06B17C, i_06b17c),
    (0x06B180, i_06b180),
    (0x06B182, i_06b182),
    (0x06B186, i_06b186),
    (0x06B18A, i_06b18a),
    (0x06B18C, i_06b18c),
    (0x06B18E, i_06b18e),
    (0x06B194, i_06b194),
    (0x06B196, i_06b196),
    (0x06B19A, i_06b19a),
    (0x06B1A0, i_06b1a0),
    (0x06B1A6, i_06b1a6),
    (0x06B1AC, i_06b1ac),
    (0x06B1B0, i_06b1b0),
    (0x06B1B4, i_06b1b4),
    (0x06B1B6, i_06b1b6),
    (0x06B1BC, i_06b1bc),
    (0x06B1C2, i_06b1c2),
    (0x06B1C8, i_06b1c8),
    (0x06B1CC, i_06b1cc),
    (0x06B1D0, i_06b1d0),
    (0x06B1D2, i_06b1d2),
    (0x06B1D4, i_06b1d4),
    (0x06B1D8, i_06b1d8),
    (0x06B1DE, i_06b1de),
    (0x06B1E2, i_06b1e2),
    (0x06B1E8, i_06b1e8),
    (0x06B1EE, i_06b1ee),
    (0x06B1F4, i_06b1f4),
    (0x06B1F8, i_06b1f8),
    (0x06B1FE, i_06b1fe),
    (0x06B202, i_06b202),
    (0x06B206, i_06b206),
    (0x06B208, i_06b208),
    (0x06B20C, i_06b20c),
    (0x06B20E, i_06b20e),
    (0x06B214, i_06b214),
    (0x06B21A, i_06b21a),
    (0x06B21E, i_06b21e),
    (0x06B224, i_06b224),
    (0x06B228, i_06b228),
    (0x06B22C, i_06b22c),
    (0x06B230, i_06b230),
    (0x06B232, i_06b232),
    (0x06B238, i_06b238),
    (0x06B23C, i_06b23c),
    (0x06B240, i_06b240),
    (0x06B242, i_06b242),
    (0x06B248, i_06b248),
    (0x06B24E, i_06b24e),
    (0x06B254, i_06b254),
    (0x06B258, i_06b258),
    (0x06B25E, i_06b25e),
    (0x06B262, i_06b262),
    (0x06B266, i_06b266),
    (0x06B26A, i_06b26a),
    (0x06B26E, i_06b26e),
    (0x06B272, i_06b272),
    (0x06B278, i_06b278),
    (0x06B27A, i_06b27a),
    (0x06B27E, i_06b27e),
    (0x06B282, i_06b282),
    (0x06B288, i_06b288),
    (0x06B28C, i_06b28c),
    (0x06B292, i_06b292),
    (0x06B296, i_06b296),
    (0x06B29C, i_06b29c),
    (0x06B2A0, i_06b2a0),
    (0x06B2A6, i_06b2a6),
    (0x06B2AA, i_06b2aa),
    (0x06B2B0, i_06b2b0),
    (0x06B2B4, i_06b2b4),
    (0x06B2BA, i_06b2ba),
    (0x06B2BE, i_06b2be),
    (0x06B2C4, i_06b2c4),
    (0x06B2C8, i_06b2c8),
    (0x06B2CC, i_06b2cc),
    (0x06B2D0, i_06b2d0),
    (0x06B2D4, i_06b2d4),
    (0x06B2D6, i_06b2d6),
    (0x06B2DA, i_06b2da),
    (0x06B2DC, i_06b2dc),
    (0x06B2E0, i_06b2e0),
    (0x06B2E4, i_06b2e4),
    (0x06B2E8, i_06b2e8),
    (0x06B2EE, i_06b2ee),
    (0x06B2F2, i_06b2f2),
    (0x06B2F4, i_06b2f4),
    (0x06B2F8, i_06b2f8),
    (0x06B2FE, i_06b2fe),
    (0x06B302, i_06b302),
    (0x06B304, i_06b304),
    (0x06B308, i_06b308),
    (0x06B30C, i_06b30c),
    (0x06B312, i_06b312),
    (0x06B316, i_06b316),
    (0x06B318, i_06b318),
    (0x06B31C, i_06b31c),
    (0x06B320, i_06b320),
    (0x06B326, i_06b326),
    (0x06B32A, i_06b32a),
    (0x06B32E, i_06b32e),
    (0x06B330, i_06b330),
    (0x06B334, i_06b334),
    (0x06B336, i_06b336),
    (0x06B33C, i_06b33c),
    (0x06B340, i_06b340),
    (0x06B344, i_06b344),
    (0x06B34A, i_06b34a),
    (0x06B34E, i_06b34e),
    (0x06B352, i_06b352),
    (0x06B358, i_06b358),
    (0x06B35C, i_06b35c),
    (0x06B360, i_06b360),
    (0x06B362, i_06b362),
    (0x06B366, i_06b366),
    (0x06B368, i_06b368),
    (0x06B36C, i_06b36c),
    (0x06B370, i_06b370),
    (0x06B374, i_06b374),
    (0x06B378, i_06b378),
    (0x06B37A, i_06b37a),
    (0x06B37E, i_06b37e),
    (0x06B382, i_06b382),
    (0x06B384, i_06b384),
    (0x06B388, i_06b388),
    (0x06B38A, i_06b38a),
    (0x06B38E, i_06b38e),
    (0x06B394, i_06b394),
    (0x06B398, i_06b398),
    (0x06B39E, i_06b39e),
    (0x06B3A2, i_06b3a2),
    (0x06B3A4, i_06b3a4),
    (0x06B3A8, i_06b3a8),
    (0x06B3AE, i_06b3ae),
    (0x06B3B2, i_06b3b2),
    (0x06B3B4, i_06b3b4),
    (0x06B3BA, i_06b3ba),
    (0x06B3BE, i_06b3be),
    (0x06B3C0, i_06b3c0),
    (0x06B3C4, i_06b3c4),
    (0x06B3C8, i_06b3c8),
    (0x06B3CC, i_06b3cc),
    (0x06B3D2, i_06b3d2),
    (0x06B3D6, i_06b3d6),
    (0x06B3D8, i_06b3d8),
    (0x06B3DC, i_06b3dc),
    (0x06B3E0, i_06b3e0),
    (0x06B3E6, i_06b3e6),
    (0x06B3EA, i_06b3ea),
    (0x06B3EC, i_06b3ec),
    (0x06B3F2, i_06b3f2),
    (0x06B3F6, i_06b3f6),
    (0x06B3FA, i_06b3fa),
    (0x06B3FE, i_06b3fe),
    (0x06B402, i_06b402),
    (0x06B406, i_06b406),
    (0x06B40C, i_06b40c),
    (0x06B410, i_06b410),
    (0x06B414, i_06b414),
    (0x06B418, i_06b418),
    (0x06B41E, i_06b41e),
    (0x06B422, i_06b422),
    (0x06B426, i_06b426),
    (0x06B42A, i_06b42a),
    (0x06B430, i_06b430),
    (0x06B434, i_06b434),
    (0x06B438, i_06b438),
    (0x06B43C, i_06b43c),
    (0x06B440, i_06b440),
    (0x06B444, i_06b444),
    (0x06B44A, i_06b44a),
    (0x06B44E, i_06b44e),
    (0x06B452, i_06b452),
    (0x06B454, i_06b454),
    (0x06B458, i_06b458),
    (0x06B45A, i_06b45a),
    (0x06B45E, i_06b45e),
    (0x06B462, i_06b462),
    (0x06B466, i_06b466),
    (0x06B46A, i_06b46a),
    (0x06B46E, i_06b46e),
    (0x06B472, i_06b472),
    (0x06B476, i_06b476),
    (0x06B478, i_06b478),
    (0x06B47C, i_06b47c),
    (0x06B47E, i_06b47e),
    (0x06B482, i_06b482),
    (0x06B488, i_06b488),
    (0x06B48C, i_06b48c),
    (0x06B492, i_06b492),
    (0x06B496, i_06b496),
    (0x06B49A, i_06b49a),
    (0x06B49E, i_06b49e),
    (0x06B4A2, i_06b4a2),
    (0x06B4A4, i_06b4a4),
    (0x06B4A8, i_06b4a8),
    (0x06B4AC, i_06b4ac),
    (0x06B4B0, i_06b4b0),
    (0x06B4B2, i_06b4b2),
    (0x06B4B8, i_06b4b8),
    (0x06B4BC, i_06b4bc),
    (0x06B4C0, i_06b4c0),
    (0x06B4C2, i_06b4c2),
    (0x06B4C6, i_06b4c6),
    (0x06B4CA, i_06b4ca),
    (0x06B4CE, i_06b4ce),
    (0x06B4D0, i_06b4d0),
    (0x06B4D2, i_06b4d2),
    (0x06B4D6, i_06b4d6),
    (0x06B4DA, i_06b4da),
    (0x06B4DC, i_06b4dc),
    (0x06B4E0, i_06b4e0),
    (0x06B4E2, i_06b4e2),
    (0x06B4E6, i_06b4e6),
    (0x06B4EA, i_06b4ea),
    (0x06B4EC, i_06b4ec),
    (0x06B4F0, i_06b4f0),
    (0x06B4F2, i_06b4f2),
    (0x06B4F8, i_06b4f8),
    (0x06B4FC, i_06b4fc),
    (0x06B500, i_06b500),
    (0x06B504, i_06b504),
    (0x06B508, i_06b508),
    (0x06B50E, i_06b50e),
    (0x06B512, i_06b512),
    (0x06B514, i_06b514),
    (0x06B518, i_06b518),
    (0x06B51A, i_06b51a),
    (0x06B51C, i_06b51c),
    (0x06B51E, i_06b51e),
    (0x06B522, i_06b522),
    (0x06B526, i_06b526),
    (0x06B52A, i_06b52a),
    (0x06B52E, i_06b52e),
    (0x06B532, i_06b532),
    (0x06B534, i_06b534),
    (0x06B538, i_06b538),
    (0x06B53A, i_06b53a),
    (0x06B540, i_06b540),
    (0x06B544, i_06b544),
    (0x06B548, i_06b548),
    (0x06B54A, i_06b54a),
    (0x06B54E, i_06b54e),
    (0x06B550, i_06b550),
    (0x06B556, i_06b556),
    (0x06B55A, i_06b55a),
    (0x06B55C, i_06b55c),
    (0x06B562, i_06b562),
    (0x06B566, i_06b566),
    (0x06B568, i_06b568),
    (0x06B56C, i_06b56c),
    (0x06B570, i_06b570),
    (0x06B572, i_06b572),
    (0x06B576, i_06b576),
    (0x06B57A, i_06b57a),
    (0x06B57E, i_06b57e),
    (0x06B582, i_06b582),
    (0x06B586, i_06b586),
    (0x06B58A, i_06b58a),
    (0x06B58E, i_06b58e),
    (0x06B592, i_06b592),
    (0x06B598, i_06b598),
    (0x06B59C, i_06b59c),
    (0x06B5A2, i_06b5a2),
    (0x06B5A4, i_06b5a4),
    (0x06B5A8, i_06b5a8),
    (0x06B5AC, i_06b5ac),
    (0x06B5B0, i_06b5b0),
    (0x06B5B4, i_06b5b4),
    (0x06B5B8, i_06b5b8),
    (0x06B5BC, i_06b5bc),
    (0x06B5C0, i_06b5c0),
    (0x06B5C6, i_06b5c6),
    (0x06B5C8, i_06b5c8),
    (0x06B5CC, i_06b5cc),
    (0x06B5D0, i_06b5d0),
    (0x06B5D4, i_06b5d4),
    (0x06B5D8, i_06b5d8),
    (0x06B5DC, i_06b5dc),
    (0x06B5E0, i_06b5e0),
    (0x06B5E4, i_06b5e4),
    (0x06B5E8, i_06b5e8),
    (0x06B5EA, i_06b5ea),
    (0x06B5EE, i_06b5ee),
    (0x06B5F4, i_06b5f4),
    (0x06B5F8, i_06b5f8),
    (0x06B5FC, i_06b5fc),
    (0x06B600, i_06b600),
    (0x06B606, i_06b606),
    (0x06B60A, i_06b60a),
    (0x06B60E, i_06b60e),
    (0x06B612, i_06b612),
    (0x06B616, i_06b616),
    (0x06B61A, i_06b61a),
    (0x06B61E, i_06b61e),
    (0x06B622, i_06b622),
    (0x06B626, i_06b626),
    (0x06B62A, i_06b62a),
    (0x06B62C, i_06b62c),
    (0x06B630, i_06b630),
    (0x06B636, i_06b636),
    (0x06B63A, i_06b63a),
    (0x06B640, i_06b640),
    (0x06B644, i_06b644),
    (0x06B648, i_06b648),
    (0x06B64C, i_06b64c),
    (0x06B650, i_06b650),
    (0x06B654, i_06b654),
    (0x06B658, i_06b658),
    (0x06B65C, i_06b65c),
    (0x06B660, i_06b660),
    (0x06B664, i_06b664),
    (0x06B668, i_06b668),
    (0x06B66C, i_06b66c),
    (0x06B670, i_06b670),
    (0x06B672, i_06b672),
    (0x06B676, i_06b676),
    (0x06B678, i_06b678),
    (0x06B67C, i_06b67c),
    (0x06B680, i_06b680),
    (0x06B684, i_06b684),
    (0x06B688, i_06b688),
    (0x06B68C, i_06b68c),
    (0x06B690, i_06b690),
    (0x06B694, i_06b694),
    (0x06B698, i_06b698),
    (0x06B69C, i_06b69c),
    (0x06B6A0, i_06b6a0),
    (0x06B6A6, i_06b6a6),
    (0x06B6AA, i_06b6aa),
    (0x06B6AC, i_06b6ac),
    (0x06B6AE, i_06b6ae),
    (0x06B6B2, i_06b6b2),
    (0x06B6B6, i_06b6b6),
    (0x06B6BA, i_06b6ba),
    (0x06B6BC, i_06b6bc),
    (0x06B6C0, i_06b6c0),
    (0x06B6C2, i_06b6c2),
    (0x06B6C6, i_06b6c6),
    (0x06B6CA, i_06b6ca),
    (0x06B6CC, i_06b6cc),
    (0x06B6D0, i_06b6d0),
    (0x06B6D4, i_06b6d4),
    (0x06B6D8, i_06b6d8),
    (0x06B6DC, i_06b6dc),
    (0x06B6E2, i_06b6e2),
    (0x06B6E6, i_06b6e6),
    (0x06B6EA, i_06b6ea),
    (0x06B6EC, i_06b6ec),
    (0x06B6F0, i_06b6f0),
    (0x06B6F2, i_06b6f2),
    (0x06B6F6, i_06b6f6),
    (0x06B6FC, i_06b6fc),
    (0x06B700, i_06b700),
    (0x06B706, i_06b706),
    (0x06B70A, i_06b70a),
    (0x06B710, i_06b710),
    (0x06B714, i_06b714),
    (0x06B718, i_06b718),
    (0x06B71C, i_06b71c),
    (0x06B720, i_06b720),
    (0x06B726, i_06b726),
    (0x06B72C, i_06b72c),
    (0x06B732, i_06b732),
    (0x06B736, i_06b736),
    (0x06B73A, i_06b73a),
    (0x06B73C, i_06b73c),
    (0x06B740, i_06b740),
    (0x06B744, i_06b744),
    (0x06B74A, i_06b74a),
    (0x06B74E, i_06b74e),
    (0x06B750, i_06b750),
    (0x06B752, i_06b752),
    (0x06B754, i_06b754),
    (0x06B758, i_06b758),
    (0x06B75C, i_06b75c),
    (0x06B760, i_06b760),
    (0x06B764, i_06b764),
    (0x06B76A, i_06b76a),
    (0x06B770, i_06b770),
    (0x06B774, i_06b774),
    (0x06B77A, i_06b77a),
    (0x06B77E, i_06b77e),
    (0x06B782, i_06b782),
    (0x06B788, i_06b788),
    (0x06B78A, i_06b78a),
    (0x06B790, i_06b790),
    (0x06B794, i_06b794),
    (0x06B798, i_06b798),
    (0x06B79C, i_06b79c),
    (0x06B7A0, i_06b7a0),
    (0x06B7A6, i_06b7a6),
    (0x06B7AA, i_06b7aa),
    (0x06B7AE, i_06b7ae),
    (0x06B7B4, i_06b7b4),
    (0x06B7BA, i_06b7ba),
    (0x06B7C0, i_06b7c0),
    (0x06B7C8, i_06b7c8),
    (0x06B7D0, i_06b7d0),
    (0x06B7D4, i_06b7d4),
    (0x06B7DA, i_06b7da),
    (0x06B7E0, i_06b7e0),
    (0x06B7E4, i_06b7e4),
    (0x06B7EA, i_06b7ea),
    (0x06B7F0, i_06b7f0),
    (0x06B7F6, i_06b7f6),
    (0x06B7FC, i_06b7fc),
    (0x06B802, i_06b802),
    (0x06B808, i_06b808),
    (0x06B80E, i_06b80e),
    (0x06B810, i_06b810),
    (0x06B814, i_06b814),
    (0x06B818, i_06b818),
    (0x06B81E, i_06b81e),
    (0x06B824, i_06b824),
    (0x06B82A, i_06b82a),
    (0x06B830, i_06b830),
    (0x06B836, i_06b836),
    (0x06B83A, i_06b83a),
    (0x06B842, i_06b842),
    (0x06B848, i_06b848),
    (0x06B84C, i_06b84c),
    (0x06B852, i_06b852),
    (0x06B858, i_06b858),
    (0x06B85C, i_06b85c),
    (0x06B860, i_06b860),
    (0x06B864, i_06b864),
    (0x06B868, i_06b868),
    (0x06B86C, i_06b86c),
    (0x06B872, i_06b872),
    (0x06B878, i_06b878),
    (0x06B87A, i_06b87a),
    (0x06B87E, i_06b87e),
    (0x06B882, i_06b882),
    (0x06B886, i_06b886),
    (0x06B88A, i_06b88a),
    (0x06B88E, i_06b88e),
    (0x06B892, i_06b892),
    (0x06B896, i_06b896),
    (0x06B898, i_06b898),
    (0x06B89A, i_06b89a),
    (0x06B89C, i_06b89c),
    (0x06B8A2, i_06b8a2),
    (0x06B8A6, i_06b8a6),
    (0x06B8A8, i_06b8a8),
    (0x06B8AA, i_06b8aa),
    (0x06B8AC, i_06b8ac),
    (0x06B8AE, i_06b8ae),
    (0x06B8B2, i_06b8b2),
    (0x06B8B6, i_06b8b6),
    (0x06B8BA, i_06b8ba),
    (0x06B8BC, i_06b8bc),
    (0x06B8C0, i_06b8c0),
    (0x06B8C4, i_06b8c4),
    (0x06B8CA, i_06b8ca),
    (0x06B8CE, i_06b8ce),
    (0x06B8D0, i_06b8d0),
    (0x06B8D4, i_06b8d4),
    (0x06B8D8, i_06b8d8),
    (0x06B8DA, i_06b8da),
    (0x06B8DE, i_06b8de),
    (0x06B8E2, i_06b8e2),
    (0x06B8E4, i_06b8e4),
    (0x06B8E8, i_06b8e8),
    (0x06B8EA, i_06b8ea),
    (0x06B8F0, i_06b8f0),
    (0x06B8F4, i_06b8f4),
    (0x06B8F8, i_06b8f8),
    (0x06B8FE, i_06b8fe),
    (0x06B902, i_06b902),
    (0x06B906, i_06b906),
    (0x06B90A, i_06b90a),
    (0x06B90E, i_06b90e),
    (0x06B912, i_06b912),
    (0x06B916, i_06b916),
    (0x06B91A, i_06b91a),
    (0x06B91C, i_06b91c),
    (0x06B91E, i_06b91e),
    (0x06B922, i_06b922),
    (0x06B924, i_06b924),
    (0x06B926, i_06b926),
    (0x06B92A, i_06b92a),
    (0x06B92C, i_06b92c),
    (0x06B930, i_06b930),
    (0x06B936, i_06b936),
    (0x06B93A, i_06b93a),
    (0x06B93C, i_06b93c),
    (0x06B940, i_06b940),
    (0x06B942, i_06b942),
    (0x06B946, i_06b946),
    (0x06B94A, i_06b94a),
    (0x06B94C, i_06b94c),
    (0x06B952, i_06b952),
    (0x06B956, i_06b956),
    (0x06B95A, i_06b95a),
    (0x06B95C, i_06b95c),
    (0x06B960, i_06b960),
    (0x06B966, i_06b966),
    (0x06B96C, i_06b96c),
    (0x06B972, i_06b972),
    (0x06B974, i_06b974),
    (0x06B978, i_06b978),
    (0x06B97E, i_06b97e),
    (0x06B984, i_06b984),
    (0x06B98A, i_06b98a),
    (0x06B990, i_06b990),
    (0x06B996, i_06b996),
    (0x06B99C, i_06b99c),
    (0x06B9A2, i_06b9a2),
    (0x06B9A4, i_06b9a4),
    (0x06B9A8, i_06b9a8),
    (0x06B9AE, i_06b9ae),
    (0x06B9B4, i_06b9b4),
    (0x06B9BA, i_06b9ba),
    (0x06B9C0, i_06b9c0),
    (0x06B9C6, i_06b9c6),
    (0x06B9CC, i_06b9cc),
    (0x06B9D2, i_06b9d2),
    (0x06B9D8, i_06b9d8),
    (0x06B9DA, i_06b9da),
    (0x06B9DE, i_06b9de),
    (0x06B9E4, i_06b9e4),
    (0x06B9EA, i_06b9ea),
    (0x06B9F0, i_06b9f0),
    (0x06B9F6, i_06b9f6),
    (0x06B9FC, i_06b9fc),
    (0x06BA02, i_06ba02),
    (0x06BA08, i_06ba08),
    (0x06BA0E, i_06ba0e),
    (0x06BA10, i_06ba10),
    (0x06BA14, i_06ba14),
    (0x06BA18, i_06ba18),
    (0x06BA1C, i_06ba1c),
    (0x06BA20, i_06ba20),
    (0x06BA24, i_06ba24),
    (0x06BA26, i_06ba26),
    (0x06BA2A, i_06ba2a),
    (0x06BA2C, i_06ba2c),
    (0x06BA30, i_06ba30),
    (0x06BA3A, i_06ba3a),
    (0x06BA40, i_06ba40),
    (0x06BA46, i_06ba46),
    (0x06BA4A, i_06ba4a),
    (0x06BA4E, i_06ba4e),
    (0x06BA52, i_06ba52),
    (0x06BA56, i_06ba56),
    (0x06BA5A, i_06ba5a),
    (0x06BA60, i_06ba60),
    (0x06BA64, i_06ba64),
    (0x06BA6A, i_06ba6a),
    (0x06BA70, i_06ba70),
    (0x06BA74, i_06ba74),
    (0x06BA7A, i_06ba7a),
    (0x06BA80, i_06ba80),
    (0x06BA84, i_06ba84),
    (0x06BA8A, i_06ba8a),
    (0x06BA90, i_06ba90),
    (0x06BA94, i_06ba94),
    (0x06BA98, i_06ba98),
    (0x06BA9E, i_06ba9e),
    (0x06BAA4, i_06baa4),
    (0x06BAAA, i_06baaa),
    (0x06BAB2, i_06bab2),
    (0x06BABA, i_06baba),
    (0x06BABE, i_06babe),
    (0x06BAC4, i_06bac4),
    (0x06BACA, i_06baca),
    (0x06BAD0, i_06bad0),
    (0x06BAD6, i_06bad6),
    (0x06BADA, i_06bada),
    (0x06BAE0, i_06bae0),
    (0x06BAE6, i_06bae6),
    (0x06BAEC, i_06baec),
    (0x06BAF2, i_06baf2),
    (0x06BAF8, i_06baf8),
    (0x06BAFE, i_06bafe),
    (0x06BB02, i_06bb02),
    (0x06BB08, i_06bb08),
    (0x06BB0C, i_06bb0c),
    (0x06BB12, i_06bb12),
    (0x06BB16, i_06bb16),
    (0x06BB18, i_06bb18),
    (0x06BB1C, i_06bb1c),
    (0x06BB20, i_06bb20),
    (0x06BB24, i_06bb24),
    (0x06BB28, i_06bb28),
    (0x06BB2C, i_06bb2c),
    (0x06BB30, i_06bb30),
    (0x06BB34, i_06bb34),
    (0x06BB3A, i_06bb3a),
    (0x06BB3C, i_06bb3c),
    (0x06BB40, i_06bb40),
    (0x06BB46, i_06bb46),
    (0x06BB4A, i_06bb4a),
    (0x06BB50, i_06bb50),
    (0x06BB54, i_06bb54),
    (0x06BB68, i_06bb68),
    (0x06BB6C, i_06bb6c),
    (0x06BB70, i_06bb70),
    (0x06BB74, i_06bb74),
    (0x06BB78, i_06bb78),
    (0x06BB7C, i_06bb7c),
    (0x06BB82, i_06bb82),
    (0x06BB84, i_06bb84),
    (0x06BB88, i_06bb88),
    (0x06BB8E, i_06bb8e),
    (0x06BB92, i_06bb92),
    (0x06BB98, i_06bb98),
    (0x06BB9C, i_06bb9c),
    (0x06BBB0, i_06bbb0),
    (0x06BBB4, i_06bbb4),
    (0x06BBB6, i_06bbb6),
    (0x06BBBC, i_06bbbc),
    (0x06BBC0, i_06bbc0),
    (0x06BBC4, i_06bbc4),
    (0x06BBCA, i_06bbca),
    (0x06BBCE, i_06bbce),
    (0x06BBD0, i_06bbd0),
    (0x06BBD4, i_06bbd4),
    (0x06BBD8, i_06bbd8),
    (0x06BBDC, i_06bbdc),
    (0x06BBE0, i_06bbe0),
    (0x06BBE4, i_06bbe4),
    (0x06BBE8, i_06bbe8),
    (0x06BBEC, i_06bbec),
    (0x06BBF0, i_06bbf0),
    (0x06BBF4, i_06bbf4),
    (0x06BBF8, i_06bbf8),
    (0x06BBFE, i_06bbfe),
    (0x06BC00, i_06bc00),
    (0x06BC04, i_06bc04),
    (0x06BC0A, i_06bc0a),
    (0x06BC0E, i_06bc0e),
    (0x06BC14, i_06bc14),
    (0x06BC18, i_06bc18),
    (0x06BC2C, i_06bc2c),
    (0x06BC30, i_06bc30),
    (0x06BC34, i_06bc34),
    (0x06BC38, i_06bc38),
    (0x06BC3C, i_06bc3c),
    (0x06BC40, i_06bc40),
    (0x06BC46, i_06bc46),
    (0x06BC48, i_06bc48),
    (0x06BC4C, i_06bc4c),
    (0x06BC52, i_06bc52),
    (0x06BC56, i_06bc56),
    (0x06BC5C, i_06bc5c),
    (0x06BC60, i_06bc60),
    (0x06BC74, i_06bc74),
    (0x06BC7A, i_06bc7a),
    (0x06BC7E, i_06bc7e),
    (0x06BC82, i_06bc82),
    (0x06BC88, i_06bc88),
    (0x06BC8C, i_06bc8c),
    (0x06BC8E, i_06bc8e),
    (0x06BC92, i_06bc92),
    (0x06BC98, i_06bc98),
    (0x06BC9C, i_06bc9c),
    (0x06BCA2, i_06bca2),
    (0x06BCA6, i_06bca6),
    (0x06BCAC, i_06bcac),
    (0x06BCB0, i_06bcb0),
    (0x06BCB4, i_06bcb4),
    (0x06BCB6, i_06bcb6),
    (0x06BCBC, i_06bcbc),
    (0x06BCC0, i_06bcc0),
    (0x06BCC6, i_06bcc6),
    (0x06BCCA, i_06bcca),
    (0x06BCD0, i_06bcd0),
    (0x06BCD4, i_06bcd4),
    (0x06BCD8, i_06bcd8),
    (0x06BCDA, i_06bcda),
    (0x06BCDE, i_06bcde),
    (0x06BCE2, i_06bce2),
    (0x06BCE6, i_06bce6),
    (0x06BCEA, i_06bcea),
    (0x06BCEC, i_06bcec),
    (0x06BCF2, i_06bcf2),
    (0x06BCF8, i_06bcf8),
    (0x06BCFC, i_06bcfc),
    (0x06BD02, i_06bd02),
    (0x06BD06, i_06bd06),
    (0x06BD0A, i_06bd0a),
    (0x06BD0E, i_06bd0e),
    (0x06BD12, i_06bd12),
    (0x06BD16, i_06bd16),
    (0x06BD1A, i_06bd1a),
    (0x06BD1E, i_06bd1e),
    (0x06BD22, i_06bd22),
    (0x06BD28, i_06bd28),
    (0x06BD2A, i_06bd2a),
    (0x06BD2E, i_06bd2e),
    (0x06BD34, i_06bd34),
    (0x06BD38, i_06bd38),
    (0x06BD3E, i_06bd3e),
    (0x06BD42, i_06bd42),
    (0x06BD56, i_06bd56),
    (0x06BD5A, i_06bd5a),
    (0x06BD5E, i_06bd5e),
    (0x06BD62, i_06bd62),
    (0x06BD66, i_06bd66),
    (0x06BD6A, i_06bd6a),
    (0x06BD70, i_06bd70),
    (0x06BD72, i_06bd72),
    (0x06BD76, i_06bd76),
    (0x06BD7C, i_06bd7c),
    (0x06BD80, i_06bd80),
    (0x06BD86, i_06bd86),
    (0x06BD8A, i_06bd8a),
    (0x06BD9E, i_06bd9e),
    (0x06BDA2, i_06bda2),
    (0x06BDA6, i_06bda6),
    (0x06BDA8, i_06bda8),
    (0x06BDAE, i_06bdae),
    (0x06BDB2, i_06bdb2),
    (0x06BDB6, i_06bdb6),
    (0x06BDBA, i_06bdba),
    (0x06BDBE, i_06bdbe),
    (0x06BDC2, i_06bdc2),
    (0x06BDC6, i_06bdc6),
    (0x06BDC8, i_06bdc8),
    (0x06BDCE, i_06bdce),
    (0x06BDD2, i_06bdd2),
    (0x06BDD6, i_06bdd6),
    (0x06BDD8, i_06bdd8),
    (0x06BDDE, i_06bdde),
    (0x06BDE4, i_06bde4),
    (0x06BDE8, i_06bde8),
    (0x06BDEE, i_06bdee),
    (0x06BDF2, i_06bdf2),
    (0x06BDF6, i_06bdf6),
    (0x06BDFA, i_06bdfa),
    (0x06BDFE, i_06bdfe),
    (0x06BE02, i_06be02),
    (0x06BE06, i_06be06),
    (0x06BE0A, i_06be0a),
    (0x06BE0E, i_06be0e),
    (0x06BE14, i_06be14),
    (0x06BE16, i_06be16),
    (0x06BE1A, i_06be1a),
    (0x06BE20, i_06be20),
    (0x06BE24, i_06be24),
    (0x06BE2A, i_06be2a),
    (0x06BE2E, i_06be2e),
    (0x06BE42, i_06be42),
    (0x06BE46, i_06be46),
    (0x06BE4A, i_06be4a),
    (0x06BE4E, i_06be4e),
    (0x06BE52, i_06be52),
    (0x06BE56, i_06be56),
    (0x06BE5C, i_06be5c),
    (0x06BE5E, i_06be5e),
    (0x06BE62, i_06be62),
    (0x06BE68, i_06be68),
    (0x06BE6C, i_06be6c),
    (0x06BE72, i_06be72),
    (0x06BE76, i_06be76),
    (0x06BE8A, i_06be8a),
    (0x06BE8E, i_06be8e),
    (0x06BE90, i_06be90),
    (0x06BE96, i_06be96),
    (0x06BE9A, i_06be9a),
    (0x06BE9E, i_06be9e),
    (0x06BEA4, i_06bea4),
    (0x06BEA8, i_06bea8),
    (0x06BEAA, i_06beaa),
    (0x06BEAE, i_06beae),
    (0x06BEB2, i_06beb2),
    (0x06BEB6, i_06beb6),
    (0x06BEBA, i_06beba),
    (0x06BEBE, i_06bebe),
    (0x06BEC0, i_06bec0),
    (0x06BEC6, i_06bec6),
    (0x06BECA, i_06beca),
    (0x06BECE, i_06bece),
    (0x06BED0, i_06bed0),
    (0x06BED6, i_06bed6),
    (0x06BEDC, i_06bedc),
    (0x06BEE0, i_06bee0),
    (0x06BEE6, i_06bee6),
    (0x06BEEC, i_06beec),
    (0x06BEF2, i_06bef2),
    (0x06BEF6, i_06bef6),
    (0x06BEFA, i_06befa),
    (0x06BEFE, i_06befe),
    (0x06BF02, i_06bf02),
    (0x06BF06, i_06bf06),
    (0x06BF0A, i_06bf0a),
    (0x06BF0E, i_06bf0e),
    (0x06BF12, i_06bf12),
    (0x06BF18, i_06bf18),
    (0x06BF1A, i_06bf1a),
    (0x06BF1E, i_06bf1e),
    (0x06BF24, i_06bf24),
    (0x06BF28, i_06bf28),
    (0x06BF2E, i_06bf2e),
    (0x06BF32, i_06bf32),
    (0x06BF46, i_06bf46),
    (0x06BF4A, i_06bf4a),
    (0x06BF4C, i_06bf4c),
    (0x06BF52, i_06bf52),
    (0x06BF56, i_06bf56),
    (0x06BF5A, i_06bf5a),
    (0x06BF5E, i_06bf5e),
    (0x06BF62, i_06bf62),
    (0x06BF66, i_06bf66),
    (0x06BF6A, i_06bf6a),
    (0x06BF6E, i_06bf6e),
    (0x06BF72, i_06bf72),
    (0x06BF76, i_06bf76),
    (0x06BF7C, i_06bf7c),
    (0x06BF7E, i_06bf7e),
    (0x06BF82, i_06bf82),
    (0x06BF88, i_06bf88),
    (0x06BF8C, i_06bf8c),
    (0x06BF92, i_06bf92),
    (0x06BF96, i_06bf96),
    (0x06BFAA, i_06bfaa),
    (0x06BFB0, i_06bfb0),
    (0x06BFB4, i_06bfb4),
    (0x06BFB8, i_06bfb8),
    (0x06BFBC, i_06bfbc),
    (0x06BFBE, i_06bfbe),
    (0x06BFC2, i_06bfc2),
    (0x06BFC6, i_06bfc6),
    (0x06BFCA, i_06bfca),
    (0x06BFCC, i_06bfcc),
    (0x06BFD0, i_06bfd0),
    (0x06BFD6, i_06bfd6),
    (0x06BFDC, i_06bfdc),
    (0x06BFE0, i_06bfe0),
    (0x06BFE6, i_06bfe6),
    (0x06BFEA, i_06bfea),
    (0x06BFEE, i_06bfee),
    (0x06BFF2, i_06bff2),
    (0x06BFF6, i_06bff6),
    (0x06BFFA, i_06bffa),
    (0x06BFFE, i_06bffe),
    (0x06C002, i_06c002),
    (0x06C006, i_06c006),
    (0x06C00C, i_06c00c),
    (0x06C00E, i_06c00e),
];
