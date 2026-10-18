// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p28_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $28D6BC: move.w #$1027, d0
pub fn i_28d6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D6C0: jsr $2352.l
pub fn i_28d6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6C6);
    cpu.pc = 0x2352;
    16
}
/// $28D6C6: jsr $13600.l
pub fn i_28d6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6CC);
    cpu.pc = 0x13600;
    16
}
/// $28D6CC: move.w #$178, d1
pub fn i_28d6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6D0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D6D0: jsr $236e.l
pub fn i_28d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6D6);
    cpu.pc = 0x236E;
    16
}
/// $28D6D6: lea.l $29e76c.l, a0
pub fn i_28d6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E76C;
    20
}
/// $28D6DC: jsr $28cd4.l
pub fn i_28d6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6E2);
    cpu.pc = 0x28CD4;
    16
}
/// $28D6E2: lea.l $28d29a(pc), a0
pub fn i_28d6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6E6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D29A;
    16
}
/// $28D6E6: move.l a0, $4c(a6)
pub fn i_28d6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6EA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D6EA: jsr $283ca.l
pub fn i_28d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6F0);
    cpu.pc = 0x283CA;
    16
}
/// $28D6F0: lea.l $28d74e(pc), a1
pub fn i_28d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D74E;
    16
}
/// $28D6F4: move.l a1, (a6)
pub fn i_28d6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D6F6: rts
pub fn i_28d6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D706: move.w #$1027, d0
pub fn i_28d706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D70A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D70A: jsr $2352.l
pub fn i_28d70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D710);
    cpu.pc = 0x2352;
    16
}
/// $28D710: bra.w $28d71e
pub fn i_28d710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D714;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D71E;
    10
}
/// $28D71E: jsr $13600.l
pub fn i_28d71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D724);
    cpu.pc = 0x13600;
    16
}
/// $28D724: move.w #$d, d1
pub fn i_28d724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D728;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D728: jsr $236e.l
pub fn i_28d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D72E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D72E);
    cpu.pc = 0x236E;
    16
}
/// $28D72E: lea.l $28d52e(pc), a0
pub fn i_28d72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D732;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D52E;
    16
}
/// $28D732: jsr $28cd4.l
pub fn i_28d732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D738);
    cpu.pc = 0x28CD4;
    16
}
/// $28D738: lea.l $28d74e(pc), a1
pub fn i_28d738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D73C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D74E;
    16
}
/// $28D73C: move.l a1, (a6)
pub fn i_28d73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D73E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D73E: lea.l $28d1f6(pc), a0
pub fn i_28d73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D742;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D1F6;
    16
}
/// $28D742: move.l a0, $4c(a6)
pub fn i_28d742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D746;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D746: jsr $283ca.l
pub fn i_28d746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D74C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D74C);
    cpu.pc = 0x283CA;
    16
}
/// $28D74C: rts
pub fn i_28d74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D74E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D74E: jsr $283ca.l
pub fn i_28d74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D754);
    cpu.pc = 0x283CA;
    16
}
/// $28D754: jsr $283d8.l
pub fn i_28d754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D75A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D75A);
    cpu.pc = 0x283D8;
    16
}
/// $28D75A: lea.l $28d76e(pc), a1
pub fn i_28d75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D75E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D76E;
    16
}
/// $28D75E: move.l a1, (a6)
pub fn i_28d75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D760;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D760: jsr $2783a.l
pub fn i_28d760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D766;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D766);
    cpu.pc = 0x2783A;
    16
}
/// $28D766: jsr $28d70.l
pub fn i_28d766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D76C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D76C);
    cpu.pc = 0x28D70;
    16
}
/// $28D76C: rts
pub fn i_28d76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D76E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D76E: lea.l $ffff.w, a0
pub fn i_28d76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D772;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $28D772: move.l a0, $4c(a6)
pub fn i_28d772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D776;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D776: jsr $283ca.l
pub fn i_28d776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D77C);
    cpu.pc = 0x283CA;
    16
}
/// $28D77C: jsr $2783a.l
pub fn i_28d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D782);
    cpu.pc = 0x2783A;
    16
}
/// $28D782: jsr $28d70.l
pub fn i_28d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D788);
    cpu.pc = 0x28D70;
    16
}
/// $28D788: bcc.w $28d792
pub fn i_28d788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D78C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D792; return 10; }
    8
}
/// $28D78C: jmp $518.l
pub fn i_28d78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D792;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28D792: rts
pub fn i_28d792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D794;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D794: jmp $518.l
pub fn i_28d794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D79A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28D876: move.w #$fae2, d0
pub fn i_28d876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D87A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFAE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D87A: jsr $5dca4.l
pub fn i_28d87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D880;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D880);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D880: move.w d0, $28(a6)
pub fn i_28d880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D884;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D884: move.w #$ccb, $2a(a6)
pub fn i_28d884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D88A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xCCB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D88A: move.w #$fefa, $2e(a6)
pub fn i_28d88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D890;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEFA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D890: move.w #$0, $2c(a6)
pub fn i_28d890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D896;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D896: bset.b #$4, $6b(a6)
pub fn i_28d896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D89C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $28D89C: move.w #$d000, $38(a6)
pub fn i_28d89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D8A2: lea.l $28d3e2(pc), a0
pub fn i_28d8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8A6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D3E2;
    16
}
/// $28D8A6: jsr $28cd4.l
pub fn i_28d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8AC);
    cpu.pc = 0x28CD4;
    16
}
/// $28D8AC: lea.l $ffff.w, a0
pub fn i_28d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8B0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $28D8B0: move.l a0, $48(a6)
pub fn i_28d8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8B4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D8B4: lea.l $28d152(pc), a0
pub fn i_28d8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D152;
    16
}
/// $28D8B8: move.l a0, $4c(a6)
pub fn i_28d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8BC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D8BC: jsr $283ca.l
pub fn i_28d8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8C2);
    cpu.pc = 0x283CA;
    16
}
/// $28D8C2: jsr $283ca.l
pub fn i_28d8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8C8);
    cpu.pc = 0x283CA;
    16
}
/// $28D8C8: move.w #$14c, d1
pub fn i_28d8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D8CC: jsr $236e.l
pub fn i_28d8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8D2);
    cpu.pc = 0x236E;
    16
}
/// $28D8D2: move.w #$1, d1
pub fn i_28d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D8D6: jsr $236e.l
pub fn i_28d8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8DC);
    cpu.pc = 0x236E;
    16
}
/// $28D8DC: move.w #$14c, d1
pub fn i_28d8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8E0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D8E0: jsr $236e.l
pub fn i_28d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8E6);
    cpu.pc = 0x236E;
    16
}
/// $28D8E6: movea.l $c(a6), a0
pub fn i_28d8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $28D8EA: cmpa.l #$1008a0, a0
pub fn i_28d8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8F0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1008A0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $28D8F0: bne.w $28d8fe
pub fn i_28d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D8FE; return 10; }
    8
}
/// $28D8F4: addi.w #$10, $24(a6)
pub fn i_28d8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8FA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $28D8FA: bra.w $28d914
pub fn i_28d8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D914;
    10
}
/// $28D8FE: move.w $22(a0), $22(a6)
pub fn i_28d8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D904;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $28D904: move.w $24(a0), d0
pub fn i_28d904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D908;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D908: addi.w #$10, d0
pub fn i_28d908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D90C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $28D90C: move.w d0, $82(a6)
pub fn i_28d90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D910;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D910: move.w d0, $24(a6)
pub fn i_28d910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D914;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D914: clr.w $5c(a6)
pub fn i_28d914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D918;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D918: lea.l $28d91e(pc), a1
pub fn i_28d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D91C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D91E;
    16
}
/// $28D91C: move.l a1, (a6)
pub fn i_28d91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D91E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D91E: move.w $2a(a6), d0
pub fn i_28d91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D922;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D922: cmpi.w #$ff00, d0
pub fn i_28d922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D926;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF00, Size::Word, false);
    8
}
/// $28D926: bpl.w $28d934
pub fn i_28d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D92A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x28D934; return 10; }
    8
}
/// $28D92A: move.w $28(a6), d0
pub fn i_28d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D92E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D92E: asr.w #$4, d0
pub fn i_28d92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D930;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $28D930: sub.w d0, $28(a6)
pub fn i_28d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D934;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D934: cmpi.b #$0, $1081ae.l
pub fn i_28d934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D93C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x1081AE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $28D93C: bne.w $28d94a
pub fn i_28d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D940;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D94A; return 10; }
    8
}
/// $28D940: jsr $27d50.l
pub fn i_28d940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D946);
    cpu.pc = 0x27D50;
    16
}
/// $28D946: bra.w $28d950
pub fn i_28d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D94A;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D950;
    10
}
/// $28D94A: jsr $27bc8.l
pub fn i_28d94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D950);
    cpu.pc = 0x27BC8;
    16
}
/// $28D950: bcc.w $28d996
pub fn i_28d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D954;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D996; return 10; }
    8
}
/// $28D954: cmpi.w #$1, $5c(a6)
pub fn i_28d954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D95A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $28D95A: bcs.w $28d966
pub fn i_28d95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D95E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28D966; return 10; }
    8
}
/// $28D95E: bra.w $28d6bc
pub fn i_28d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D962;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D6BC;
    10
}
/// $28D966: move.w #$fce0, d0
pub fn i_28d966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D96A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFCE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D96A: jsr $5dca4.l
pub fn i_28d96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D970);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D970: move.w d0, $28(a6)
pub fn i_28d970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D974;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D974: move.w #$3c0, $2a(a6)
pub fn i_28d974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D97A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D97A: move.w #$ff88, $2e(a6)
pub fn i_28d97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D980;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D980: move.w #$0, $2c(a6)
pub fn i_28d980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D986;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D986: addq.w #$8, $24(a6)
pub fn i_28d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D98A;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D98A: addi.w #$1, $5c(a6)
pub fn i_28d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D990;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $28D990: lea.l $28d91e(pc), a1
pub fn i_28d990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D994;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D91E;
    16
}
/// $28D994: move.l a1, (a6)
pub fn i_28d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D996;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D996: jsr $28d70.l
pub fn i_28d996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D99C);
    cpu.pc = 0x28D70;
    16
}
/// $28D99C: cmpi.b #$1, $3b(a6)
pub fn i_28d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9A2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $28D9A2: bcs.w $28d9b6
pub fn i_28d9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9A6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28D9B6; return 10; }
    8
}
/// $28D9A6: jsr $283d8.l
pub fn i_28d9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D9AC);
    cpu.pc = 0x283D8;
    16
}
/// $28D9AC: btst.b #$1, $13(a6)
pub fn i_28d9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D9B2: bne.w $28d6bc
pub fn i_28d9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D6BC; return 10; }
    8
}
/// $28D9B6: btst.b #$3, $13(a6)
pub fn i_28d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9BC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D9BC: bne.w $28d706
pub fn i_28d9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D706; return 10; }
    8
}
/// $28D9C0: cmpi.w #$100, $24(a6)
pub fn i_28d9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9C6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $28D9C6: bmi.w $28d794
pub fn i_28d9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9CA;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D9CA: cmpi.w #$150, $22(a6)
pub fn i_28d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9D0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $28D9D0: bcc.w $28d794
pub fn i_28d9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D9D4: jsr $32afa.l
pub fn i_28d9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D9DA);
    cpu.pc = 0x32AFA;
    16
}
/// $28D9DA: rts
pub fn i_28d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28DB6A: clr.b $1081ae.l
pub fn i_28db6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB70;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x1081AE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x1081AE, r2 as u8);
    16
}
/// $28DB70: jmp $518.l
pub fn i_28db70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB76;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x28D6BC, i_28d6bc),
    (0x28D6C0, i_28d6c0),
    (0x28D6C6, i_28d6c6),
    (0x28D6CC, i_28d6cc),
    (0x28D6D0, i_28d6d0),
    (0x28D6D6, i_28d6d6),
    (0x28D6DC, i_28d6dc),
    (0x28D6E2, i_28d6e2),
    (0x28D6E6, i_28d6e6),
    (0x28D6EA, i_28d6ea),
    (0x28D6F0, i_28d6f0),
    (0x28D6F4, i_28d6f4),
    (0x28D6F6, i_28d6f6),
    (0x28D706, i_28d706),
    (0x28D70A, i_28d70a),
    (0x28D710, i_28d710),
    (0x28D71E, i_28d71e),
    (0x28D724, i_28d724),
    (0x28D728, i_28d728),
    (0x28D72E, i_28d72e),
    (0x28D732, i_28d732),
    (0x28D738, i_28d738),
    (0x28D73C, i_28d73c),
    (0x28D73E, i_28d73e),
    (0x28D742, i_28d742),
    (0x28D746, i_28d746),
    (0x28D74C, i_28d74c),
    (0x28D74E, i_28d74e),
    (0x28D754, i_28d754),
    (0x28D75A, i_28d75a),
    (0x28D75E, i_28d75e),
    (0x28D760, i_28d760),
    (0x28D766, i_28d766),
    (0x28D76C, i_28d76c),
    (0x28D76E, i_28d76e),
    (0x28D772, i_28d772),
    (0x28D776, i_28d776),
    (0x28D77C, i_28d77c),
    (0x28D782, i_28d782),
    (0x28D788, i_28d788),
    (0x28D78C, i_28d78c),
    (0x28D792, i_28d792),
    (0x28D794, i_28d794),
    (0x28D876, i_28d876),
    (0x28D87A, i_28d87a),
    (0x28D880, i_28d880),
    (0x28D884, i_28d884),
    (0x28D88A, i_28d88a),
    (0x28D890, i_28d890),
    (0x28D896, i_28d896),
    (0x28D89C, i_28d89c),
    (0x28D8A2, i_28d8a2),
    (0x28D8A6, i_28d8a6),
    (0x28D8AC, i_28d8ac),
    (0x28D8B0, i_28d8b0),
    (0x28D8B4, i_28d8b4),
    (0x28D8B8, i_28d8b8),
    (0x28D8BC, i_28d8bc),
    (0x28D8C2, i_28d8c2),
    (0x28D8C8, i_28d8c8),
    (0x28D8CC, i_28d8cc),
    (0x28D8D2, i_28d8d2),
    (0x28D8D6, i_28d8d6),
    (0x28D8DC, i_28d8dc),
    (0x28D8E0, i_28d8e0),
    (0x28D8E6, i_28d8e6),
    (0x28D8EA, i_28d8ea),
    (0x28D8F0, i_28d8f0),
    (0x28D8F4, i_28d8f4),
    (0x28D8FA, i_28d8fa),
    (0x28D8FE, i_28d8fe),
    (0x28D904, i_28d904),
    (0x28D908, i_28d908),
    (0x28D90C, i_28d90c),
    (0x28D910, i_28d910),
    (0x28D914, i_28d914),
    (0x28D918, i_28d918),
    (0x28D91C, i_28d91c),
    (0x28D91E, i_28d91e),
    (0x28D922, i_28d922),
    (0x28D926, i_28d926),
    (0x28D92A, i_28d92a),
    (0x28D92E, i_28d92e),
    (0x28D930, i_28d930),
    (0x28D934, i_28d934),
    (0x28D93C, i_28d93c),
    (0x28D940, i_28d940),
    (0x28D946, i_28d946),
    (0x28D94A, i_28d94a),
    (0x28D950, i_28d950),
    (0x28D954, i_28d954),
    (0x28D95A, i_28d95a),
    (0x28D95E, i_28d95e),
    (0x28D966, i_28d966),
    (0x28D96A, i_28d96a),
    (0x28D970, i_28d970),
    (0x28D974, i_28d974),
    (0x28D97A, i_28d97a),
    (0x28D980, i_28d980),
    (0x28D986, i_28d986),
    (0x28D98A, i_28d98a),
    (0x28D990, i_28d990),
    (0x28D994, i_28d994),
    (0x28D996, i_28d996),
    (0x28D99C, i_28d99c),
    (0x28D9A2, i_28d9a2),
    (0x28D9A6, i_28d9a6),
    (0x28D9AC, i_28d9ac),
    (0x28D9B2, i_28d9b2),
    (0x28D9B6, i_28d9b6),
    (0x28D9BC, i_28d9bc),
    (0x28D9C0, i_28d9c0),
    (0x28D9C6, i_28d9c6),
    (0x28D9CA, i_28d9ca),
    (0x28D9D0, i_28d9d0),
    (0x28D9D4, i_28d9d4),
    (0x28D9DA, i_28d9da),
    (0x28DB6A, i_28db6a),
    (0x28DB70, i_28db70),
];
