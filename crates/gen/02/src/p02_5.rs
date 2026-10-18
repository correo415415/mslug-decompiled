// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $02D792: move.b #$28, $45(a6)
pub fn i_02d792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D798;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D798: move.b #$28, d0
pub fn i_02d798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D79C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D79C: jsr $5e722.l
pub fn i_02d79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D7A2);
    cpu.pc = 0x5E722;
    16
}
/// $02D7A2: bclr.b #$3, $13(a6)
pub fn i_02d7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D7A8: jsr $2aa0e(pc)
pub fn i_02d7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D7AC);
    cpu.pc = 0x2AA0E;
    16
}
/// $02D7AC: move.w $2a(a6), d0
pub fn i_02d7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7B0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D7B0: move.w #$400, d1
pub fn i_02d7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D7B4: jsr $267f4.l
pub fn i_02d7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D7BA);
    cpu.pc = 0x267F4;
    16
}
/// $02D7BA: move.w d0, $2a(a6)
pub fn i_02d7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7BE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D7BE: jsr $2a878(pc)
pub fn i_02d7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D7C2);
    cpu.pc = 0x2A878;
    16
}
/// $02D7C2: bcc.w $2d7d2
pub fn i_02d7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D7D2; return 10; }
    8
}
/// $02D7C6: lea.l $2d8c8(pc), a1
pub fn i_02d7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D8C8;
    16
}
/// $02D7CA: move.l a1, (a6)
pub fn i_02d7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D7CC: jsr $267e2.l
pub fn i_02d7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D7D2);
    cpu.pc = 0x267E2;
    16
}
/// $02D7D2: jsr $28d70.l
pub fn i_02d7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D7D8);
    cpu.pc = 0x28D70;
    16
}
/// $02D7D8: jsr $2a664(pc)
pub fn i_02d7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D7DC);
    cpu.pc = 0x2A664;
    16
}
/// $02D7DC: bcc.w $2d7e6
pub fn i_02d7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D7E6; return 10; }
    8
}
/// $02D7E0: lea.l $2db52(pc), a1
pub fn i_02d7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DB52;
    16
}
/// $02D7E4: move.l a1, (a6)
pub fn i_02d7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D7E6: movea.l #$ffffffff, a0
pub fn i_02d7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7EC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02D7EC: lea.l $27964e.l, a0
pub fn i_02d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02D7F2: jsr $5dd56.l
pub fn i_02d7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D7F8);
    cpu.pc = 0x5DD56;
    16
}
/// $02D7F8: bcc.w $2d800
pub fn i_02d7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D800; return 10; }
    8
}
/// $02D7FC: jmp $2dcbc(pc)
pub fn i_02d7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D800;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02D800: rts
pub fn i_02d800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D802;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_HitLaunchB_02d802  ($02D802, 198 B, slug/states_02ae3e.s)
/// $02D802: bclr.b #$5, $8c(a6)
pub fn i_02d802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D808;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D808: jsr $2a9a0(pc)
pub fn i_02d808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D80C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D80C);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D80C: movea.l #$279588, a0
pub fn i_02d80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D812;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279588;
    cpu.a[0] = v1;
    4
}
/// $02D812: lsl.w #$2, d0
pub fn i_02d812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D814;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D814: movea.l (a0, d0.w), a0
pub fn i_02d814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D818;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D818: cmpa.l #$ffffffff, a0
pub fn i_02d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D81E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D81E: beq.w $2d828
pub fn i_02d81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D822;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D828; return 10; }
    8
}
/// $02D822: jsr $28cd4.l
pub fn i_02d822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D828;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D828);
    cpu.pc = 0x28CD4;
    16
}
/// $02D828: move.w #$0, d0
pub fn i_02d828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D82C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D82C: move.b #$96, $20(a6)
pub fn i_02d82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D832;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D832: andi.w #$ff, d0
pub fn i_02d832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D836;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D836: lsr.w #$2, d0
pub fn i_02d836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D838;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D838: add.b d0, $20(a6)
pub fn i_02d838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D83C;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D83C: move.w #$3fc, $2a(a6)
pub fn i_02d83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D842;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D842: move.w #$ffab, $2e(a6)
pub fn i_02d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D848: clr.w $2c(a6)
pub fn i_02d848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D84C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D84C: lea.l $2d852(pc), a1
pub fn i_02d84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D850;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D852;
    16
}
/// $02D850: move.l a1, (a6)
pub fn i_02d850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D852;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D852: bset.b #$1, $8d(a6)
pub fn i_02d852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D858;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D858: move.b #$28, $45(a6)
pub fn i_02d858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D85E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D85E: move.b #$28, d0
pub fn i_02d85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D862;
    cpu.ir = 0x103C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D862: jsr $5e722.l
pub fn i_02d862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D868);
    cpu.pc = 0x5E722;
    16
}
/// $02D868: bclr.b #$3, $13(a6)
pub fn i_02d868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D86E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D86E: jsr $2aa0e(pc)
pub fn i_02d86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D872;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D872);
    cpu.pc = 0x2AA0E;
    16
}
/// $02D872: move.w $2a(a6), d0
pub fn i_02d872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D876;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D876: move.w #$400, d1
pub fn i_02d876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D87A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D87A: jsr $267f4.l
pub fn i_02d87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D880;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D880);
    cpu.pc = 0x267F4;
    16
}
/// $02D880: move.w d0, $2a(a6)
pub fn i_02d880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D884;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D884: jsr $2a878(pc)
pub fn i_02d884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D888;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D888);
    cpu.pc = 0x2A878;
    16
}
/// $02D888: bcc.w $2d898
pub fn i_02d888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D88C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D898; return 10; }
    8
}
/// $02D88C: lea.l $2d980(pc), a1
pub fn i_02d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D890;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D980;
    16
}
/// $02D890: move.l a1, (a6)
pub fn i_02d890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D892;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D892: jsr $267e2.l
pub fn i_02d892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D898;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D898);
    cpu.pc = 0x267E2;
    16
}
/// $02D898: jsr $28d70.l
pub fn i_02d898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D89E);
    cpu.pc = 0x28D70;
    16
}
/// $02D89E: jsr $2a664(pc)
pub fn i_02d89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D8A2);
    cpu.pc = 0x2A664;
    16
}
/// $02D8A2: bcc.w $2d8ac
pub fn i_02d8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D8AC; return 10; }
    8
}
/// $02D8A6: lea.l $2db52(pc), a1
pub fn i_02d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DB52;
    16
}
/// $02D8AA: move.l a1, (a6)
pub fn i_02d8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D8AC: movea.l #$ffffffff, a0
pub fn i_02d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8B2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02D8B2: lea.l $27964e.l, a0
pub fn i_02d8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02D8B8: jsr $5dd56.l
pub fn i_02d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D8BE);
    cpu.pc = 0x5DD56;
    16
}
/// $02D8BE: bcc.w $2d8c6
pub fn i_02d8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D8C6; return 10; }
    8
}
/// $02D8C2: jmp $2dcbc(pc)
pub fn i_02d8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8C6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02D8C6: rts
pub fn i_02d8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_HitLandA_02d8c8  ($02D8C8, 184 B, slug/states_02ae3e.s)
/// $02D8C8: bclr.b #$5, $8c(a6)
pub fn i_02d8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8CE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D8CE: jsr $2a9a0(pc)
pub fn i_02d8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D8D2);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D8D2: movea.l #$27959c, a0
pub fn i_02d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8D8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27959C;
    cpu.a[0] = v1;
    4
}
/// $02D8D8: lsl.w #$2, d0
pub fn i_02d8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8DA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D8DA: movea.l (a0, d0.w), a0
pub fn i_02d8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8DE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D8DE: cmpa.l #$ffffffff, a0
pub fn i_02d8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8E4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D8E4: beq.w $2d8ee
pub fn i_02d8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D8EE; return 10; }
    8
}
/// $02D8E8: jsr $28cd4.l
pub fn i_02d8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D8EE);
    cpu.pc = 0x28CD4;
    16
}
/// $02D8EE: move.w #$0, d0
pub fn i_02d8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8F2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D8F2: move.b #$96, $20(a6)
pub fn i_02d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D8F8: andi.w #$ff, d0
pub fn i_02d8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8FC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D8FC: lsr.w #$2, d0
pub fn i_02d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8FE;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D8FE: add.b d0, $20(a6)
pub fn i_02d8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D902;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D902: lea.l $2d908(pc), a1
pub fn i_02d902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D906;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D908;
    16
}
/// $02D906: move.l a1, (a6)
pub fn i_02d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D908;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D908: move.b #$28, $45(a6)
pub fn i_02d908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D90E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D90E: move.b #$28, d0
pub fn i_02d90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D912;
    cpu.ir = 0x103C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D912: jsr $5e722.l
pub fn i_02d912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D918);
    cpu.pc = 0x5E722;
    16
}
/// $02D918: bclr.b #$3, $13(a6)
pub fn i_02d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D91E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D91E: jsr $2aa0e(pc)
pub fn i_02d91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D922;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D922);
    cpu.pc = 0x2AA0E;
    16
}
/// $02D922: move.w $2a(a6), d0
pub fn i_02d922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D926;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D926: move.w #$400, d1
pub fn i_02d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D92A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D92A: jsr $267f4.l
pub fn i_02d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D930);
    cpu.pc = 0x267F4;
    16
}
/// $02D930: move.w d0, $2a(a6)
pub fn i_02d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D934;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D934: jsr $2a7d8(pc)
pub fn i_02d934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D938;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D938);
    cpu.pc = 0x2A7D8;
    16
}
/// $02D938: jsr $28d70.l
pub fn i_02d938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D93E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D93E);
    cpu.pc = 0x28D70;
    16
}
/// $02D93E: bcc.w $2d948
pub fn i_02d93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D942;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D948; return 10; }
    8
}
/// $02D942: lea.l $2d6ce(pc), a1
pub fn i_02d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D6CE;
    16
}
/// $02D946: move.l a1, (a6)
pub fn i_02d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D948;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D948: jsr $2ac80(pc)
pub fn i_02d948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D94C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D94C);
    cpu.pc = 0x2AC80;
    16
}
/// $02D94C: bcc.w $2d956
pub fn i_02d94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D950;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D956; return 10; }
    8
}
/// $02D950: lea.l $2dd74(pc), a1
pub fn i_02d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D954;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02D954: move.l a1, (a6)
pub fn i_02d954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D956;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D956: jsr $2a664(pc)
pub fn i_02d956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D95A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D95A);
    cpu.pc = 0x2A664;
    16
}
/// $02D95A: bcc.w $2d964
pub fn i_02d95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D95E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D964; return 10; }
    8
}
/// $02D95E: lea.l $2da38(pc), a1
pub fn i_02d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D962;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02D962: move.l a1, (a6)
pub fn i_02d962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D964;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D964: movea.l #$ffffffff, a0
pub fn i_02d964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D96A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02D96A: lea.l $27964e.l, a0
pub fn i_02d96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D970;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02D970: jsr $5dd56.l
pub fn i_02d970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D976);
    cpu.pc = 0x5DD56;
    16
}
/// $02D976: bcc.w $2d97e
pub fn i_02d976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D97A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D97E; return 10; }
    8
}
/// $02D97A: jmp $2dcbc(pc)
pub fn i_02d97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D97E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02D97E: rts
pub fn i_02d97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D980;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_HitLandB_02d980  ($02D980, 184 B, slug/states_02ae3e.s)
/// $02D980: bclr.b #$5, $8c(a6)
pub fn i_02d980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D986;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D986: jsr $2a9a0(pc)
pub fn i_02d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D98A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D98A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D98A: movea.l #$2795b0, a0
pub fn i_02d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D990;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2795B0;
    cpu.a[0] = v1;
    4
}
/// $02D990: lsl.w #$2, d0
pub fn i_02d990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D992;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D992: movea.l (a0, d0.w), a0
pub fn i_02d992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D996;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D996: cmpa.l #$ffffffff, a0
pub fn i_02d996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D99C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D99C: beq.w $2d9a6
pub fn i_02d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D9A6; return 10; }
    8
}
/// $02D9A0: jsr $28cd4.l
pub fn i_02d9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D9A6);
    cpu.pc = 0x28CD4;
    16
}
/// $02D9A6: move.w #$0, d0
pub fn i_02d9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D9AA: move.b #$96, $20(a6)
pub fn i_02d9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9B0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D9B0: andi.w #$ff, d0
pub fn i_02d9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9B4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D9B4: lsr.w #$2, d0
pub fn i_02d9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9B6;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D9B6: add.b d0, $20(a6)
pub fn i_02d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9BA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D9BA: lea.l $2d9c0(pc), a1
pub fn i_02d9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D9C0;
    16
}
/// $02D9BE: move.l a1, (a6)
pub fn i_02d9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D9C0: move.b #$28, $45(a6)
pub fn i_02d9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D9C6: move.b #$28, d0
pub fn i_02d9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9CA;
    cpu.ir = 0x103C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D9CA: jsr $5e722.l
pub fn i_02d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D9D0);
    cpu.pc = 0x5E722;
    16
}
/// $02D9D0: bclr.b #$3, $13(a6)
pub fn i_02d9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9D6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D9D6: jsr $2aa0e(pc)
pub fn i_02d9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D9DA);
    cpu.pc = 0x2AA0E;
    16
}
/// $02D9DA: move.w $2a(a6), d0
pub fn i_02d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D9DE: move.w #$400, d1
pub fn i_02d9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D9E2: jsr $267f4.l
pub fn i_02d9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D9E8);
    cpu.pc = 0x267F4;
    16
}
/// $02D9E8: move.w d0, $2a(a6)
pub fn i_02d9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9EC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D9EC: jsr $2a7d8(pc)
pub fn i_02d9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D9F0);
    cpu.pc = 0x2A7D8;
    16
}
/// $02D9F0: jsr $28d70.l
pub fn i_02d9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D9F6);
    cpu.pc = 0x28D70;
    16
}
/// $02D9F6: bcc.w $2da00
pub fn i_02d9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DA00; return 10; }
    8
}
/// $02D9FA: lea.l $2d6ce(pc), a1
pub fn i_02d9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D6CE;
    16
}
/// $02D9FE: move.l a1, (a6)
pub fn i_02d9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DA00: jsr $2ac80(pc)
pub fn i_02da00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DA04);
    cpu.pc = 0x2AC80;
    16
}
/// $02DA04: bcc.w $2da0e
pub fn i_02da04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DA0E; return 10; }
    8
}
/// $02DA08: lea.l $2dd74(pc), a1
pub fn i_02da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02DA0C: move.l a1, (a6)
pub fn i_02da0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DA0E: jsr $2a664(pc)
pub fn i_02da0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DA12);
    cpu.pc = 0x2A664;
    16
}
/// $02DA12: bcc.w $2da1c
pub fn i_02da12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DA1C; return 10; }
    8
}
/// $02DA16: lea.l $2da38(pc), a1
pub fn i_02da16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02DA1A: move.l a1, (a6)
pub fn i_02da1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DA1C: movea.l #$ffffffff, a0
pub fn i_02da1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA22;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DA22: lea.l $27964e.l, a0
pub fn i_02da22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02DA28: jsr $5dd56.l
pub fn i_02da28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DA2E);
    cpu.pc = 0x5DD56;
    16
}
/// $02DA2E: bcc.w $2da36
pub fn i_02da2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DA36; return 10; }
    8
}
/// $02DA32: jmp $2dcbc(pc)
pub fn i_02da32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA36;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02DA36: rts
pub fn i_02da36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DeathExplode_02da38  ($02DA38, 276 B, slug/states_02ae3e.s)
/// $02DA38: jsr $2feda(pc)
pub fn i_02da38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DA3C);
    cpu.pc = 0x2FEDA;
    16
}
/// $02DA3C: bclr.b #$5, $8c(a6)
pub fn i_02da3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA42;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DA42: bset.b #$0, $13(a6)
pub fn i_02da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA48;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DA48: movea.l $48(a6), a2
pub fn i_02da48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA4C;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02DA4C: move.w #$0, d0
pub fn i_02da4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA50;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DA50: cmpa.l #$ffffffff, a2
pub fn i_02da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA56;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DA56: beq.w $2da7a
pub fn i_02da56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DA7A; return 10; }
    8
}
/// $02DA5A: adda.l #$a, a2
pub fn i_02da5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA60;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0xA;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $02DA60: move.w $54(a6), d0
pub fn i_02da60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02DA64: move.w (a2), d2
pub fn i_02da64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA66;
    cpu.ir = 0x3412;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $02DA66: move.w $2(a2), d3
pub fn i_02da66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA6A;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02DA6A: add.w $22(a6), d2
pub fn i_02da6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA6E;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02DA6E: add.w $22(a6), d3
pub fn i_02da6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA72;
    cpu.ir = 0xD66E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02DA72: add.w d2, d3
pub fn i_02da72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA74;
    cpu.ir = 0xD642;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02DA74: asr.w #$1, d3
pub fn i_02da74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA76;
    cpu.ir = 0xE243;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02DA76: sub.w d3, d0
pub fn i_02da76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA78;
    cpu.ir = 0x9043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02DA78: subx.w d0, d0
pub fn i_02da78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA7A;
    cpu.ir = 0x9140;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = subx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02DA7A: tst.w d0
pub fn i_02da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA7C;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02DA7C: bne.w $2daa4
pub fn i_02da7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA80;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2DAA4; return 10; }
    8
}
/// $02DA80: jsr $2a9a0(pc)
pub fn i_02da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DA84);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DA84: movea.l #$27954c, a0
pub fn i_02da84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA8A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27954C;
    cpu.a[0] = v1;
    4
}
/// $02DA8A: lsl.w #$2, d0
pub fn i_02da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA8C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DA8C: movea.l (a0, d0.w), a0
pub fn i_02da8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA90;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DA90: cmpa.l #$ffffffff, a0
pub fn i_02da90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA96;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DA96: beq.w $2daa0
pub fn i_02da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DAA0; return 10; }
    8
}
/// $02DA9A: jsr $28cd4.l
pub fn i_02da9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DAA0);
    cpu.pc = 0x28CD4;
    16
}
/// $02DAA0: bra.w $2dac4
pub fn i_02daa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2DAC4;
    10
}
/// $02DAA4: jsr $2a9a0(pc)
pub fn i_02daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DAA8);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DAA8: movea.l #$279560, a0
pub fn i_02daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAAE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279560;
    cpu.a[0] = v1;
    4
}
/// $02DAAE: lsl.w #$2, d0
pub fn i_02daae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAB0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DAB0: movea.l (a0, d0.w), a0
pub fn i_02dab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAB4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DAB4: cmpa.l #$ffffffff, a0
pub fn i_02dab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DABA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DABA: beq.w $2dac4
pub fn i_02daba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DABE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DAC4; return 10; }
    8
}
/// $02DABE: jsr $28cd4.l
pub fn i_02dabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DAC4);
    cpu.pc = 0x28CD4;
    16
}
/// $02DAC4: move.w #$0, d0
pub fn i_02dac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAC8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DAC8: move.b #$96, $20(a6)
pub fn i_02dac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DACE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DACE: andi.w #$ff, d0
pub fn i_02dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAD2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DAD2: lsr.w #$2, d0
pub fn i_02dad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAD4;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DAD4: add.b d0, $20(a6)
pub fn i_02dad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAD8;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02DAD8: jsr $267e2.l
pub fn i_02dad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DADE);
    cpu.pc = 0x267E2;
    16
}
/// $02DADE: lea.l $2dae4(pc), a1
pub fn i_02dade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DAE4;
    16
}
/// $02DAE2: move.l a1, (a6)
pub fn i_02dae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DAE4: jsr $2aa24(pc)
pub fn i_02dae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DAE8);
    cpu.pc = 0x2AA24;
    16
}
/// $02DAE8: bset.b #$4, $8d(a6)
pub fn i_02dae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAEE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DAEE: jsr $2fafc(pc)
pub fn i_02daee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DAF2);
    cpu.pc = 0x2FAFC;
    16
}
/// $02DAF2: bcc.w $2db28
pub fn i_02daf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DB28; return 10; }
    8
}
/// $02DAF6: lea.l $2dc5c(pc), a1
pub fn i_02daf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DC5C;
    16
}
/// $02DAFA: move.l a1, (a6)
pub fn i_02dafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DAFC: lea.l $2973c(pc), a0
pub fn i_02dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB00;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2973C;
    16
}
/// $02DB00: move.l a0, $4c(a6)
pub fn i_02db00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB04;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DB04: jsr $283ca.l
pub fn i_02db04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB0A);
    cpu.pc = 0x283CA;
    16
}
/// $02DB0A: jsr $283ca.l
pub fn i_02db0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB10);
    cpu.pc = 0x283CA;
    16
}
/// $02DB10: jsr $283d8.l
pub fn i_02db10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB16);
    cpu.pc = 0x283D8;
    16
}
/// $02DB16: lea.l $ffff.w, a0
pub fn i_02db16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB1A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02DB1A: move.l a0, $4c(a6)
pub fn i_02db1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB1E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DB1E: jsr $283ca.l
pub fn i_02db1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB24);
    cpu.pc = 0x283CA;
    16
}
/// $02DB24: jsr $2ff22(pc)
pub fn i_02db24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DB28);
    cpu.pc = 0x2FF22;
    16
}
/// $02DB28: jsr $2a752(pc)
pub fn i_02db28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DB2C);
    cpu.pc = 0x2A752;
    16
}
/// $02DB2C: jsr $28d70.l
pub fn i_02db2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB32);
    cpu.pc = 0x28D70;
    16
}
/// $02DB32: movea.l #$ffffffff, a0
pub fn i_02db32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB38;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DB38: lea.l $27964e.l, a0
pub fn i_02db38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02DB3E: jsr $5dd56.l
pub fn i_02db3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB44);
    cpu.pc = 0x5DD56;
    16
}
/// $02DB44: bcc.w $2db4c
pub fn i_02db44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DB4C; return 10; }
    8
}
/// $02DB48: jmp $2dcbc(pc)
pub fn i_02db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB4C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
// ===== JsrPcThunk_02db4c  ($02DB4C, 6 B, thunks/jsr_pc_thunks.c)
/// $02DB4C: jsr $2ac80(pc)
pub fn i_02db4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DB50);
    cpu.pc = 0x2AC80;
    16
}
/// $02DB50: rts
pub fn i_02db50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DeathLaunch_02db52  ($02DB52, 260 B, slug/states_02ae3e.s)
/// $02DB52: bclr.b #$5, $8c(a6)
pub fn i_02db52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB58;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DB58: bset.b #$0, $13(a6)
pub fn i_02db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB5E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DB5E: movea.l $48(a6), a2
pub fn i_02db5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB62;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02DB62: move.w #$0, d0
pub fn i_02db62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB66;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DB66: cmpa.l #$ffffffff, a2
pub fn i_02db66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB6C;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DB6C: beq.w $2db90
pub fn i_02db6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DB90; return 10; }
    8
}
/// $02DB70: adda.l #$a, a2
pub fn i_02db70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB76;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0xA;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $02DB76: move.w $54(a6), d0
pub fn i_02db76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB7A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02DB7A: move.w (a2), d2
pub fn i_02db7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB7C;
    cpu.ir = 0x3412;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $02DB7C: move.w $2(a2), d3
pub fn i_02db7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB80;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02DB80: add.w $22(a6), d2
pub fn i_02db80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB84;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02DB84: add.w $22(a6), d3
pub fn i_02db84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB88;
    cpu.ir = 0xD66E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02DB88: add.w d2, d3
pub fn i_02db88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB8A;
    cpu.ir = 0xD642;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02DB8A: asr.w #$1, d3
pub fn i_02db8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB8C;
    cpu.ir = 0xE243;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02DB8C: sub.w d3, d0
pub fn i_02db8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB8E;
    cpu.ir = 0x9043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02DB8E: subx.w d0, d0
pub fn i_02db8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB90;
    cpu.ir = 0x9140;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = subx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02DB90: tst.w d0
pub fn i_02db90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB92;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02DB92: bne.w $2dbba
pub fn i_02db92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2DBBA; return 10; }
    8
}
/// $02DB96: jsr $2a9a0(pc)
pub fn i_02db96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DB9A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DB9A: movea.l #$27954c, a0
pub fn i_02db9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBA0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27954C;
    cpu.a[0] = v1;
    4
}
/// $02DBA0: lsl.w #$2, d0
pub fn i_02dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBA2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DBA2: movea.l (a0, d0.w), a0
pub fn i_02dba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBA6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DBA6: cmpa.l #$ffffffff, a0
pub fn i_02dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBAC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DBAC: beq.w $2dbb6
pub fn i_02dbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DBB6; return 10; }
    8
}
/// $02DBB0: jsr $28cd4.l
pub fn i_02dbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DBB6);
    cpu.pc = 0x28CD4;
    16
}
/// $02DBB6: bra.w $2dbda
pub fn i_02dbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBBA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2DBDA;
    10
}
/// $02DBBA: jsr $2a9a0(pc)
pub fn i_02dbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DBBE);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DBBE: movea.l #$279560, a0
pub fn i_02dbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBC4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279560;
    cpu.a[0] = v1;
    4
}
/// $02DBC4: lsl.w #$2, d0
pub fn i_02dbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBC6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DBC6: movea.l (a0, d0.w), a0
pub fn i_02dbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBCA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DBCA: cmpa.l #$ffffffff, a0
pub fn i_02dbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBD0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DBD0: beq.w $2dbda
pub fn i_02dbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DBDA; return 10; }
    8
}
/// $02DBD4: jsr $28cd4.l
pub fn i_02dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DBDA);
    cpu.pc = 0x28CD4;
    16
}
/// $02DBDA: move.w #$0, d0
pub fn i_02dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DBDE: move.b #$96, $20(a6)
pub fn i_02dbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBE4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DBE4: andi.w #$ff, d0
pub fn i_02dbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBE8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DBE8: lsr.w #$2, d0
pub fn i_02dbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBEA;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DBEA: add.b d0, $20(a6)
pub fn i_02dbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBEE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02DBEE: jsr $267e2.l
pub fn i_02dbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DBF4);
    cpu.pc = 0x267E2;
    16
}
/// $02DBF4: move.w #$3fc, $2a(a6)
pub fn i_02dbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02DBFA: move.w #$ffab, $2e(a6)
pub fn i_02dbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC00;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02DC00: move.b #$3c, $91(a6)
pub fn i_02dc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DC06: lea.l $2dc0c(pc), a1
pub fn i_02dc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DC0C;
    16
}
/// $02DC0A: move.l a1, (a6)
pub fn i_02dc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DC0C: jsr $2aa24(pc)
pub fn i_02dc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DC10);
    cpu.pc = 0x2AA24;
    16
}
/// $02DC10: bset.b #$4, $8d(a6)
pub fn i_02dc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC16;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DC16: move.w $2a(a6), d0
pub fn i_02dc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02DC1A: move.w #$400, d1
pub fn i_02dc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC1E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DC1E: jsr $267f4.l
pub fn i_02dc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DC24);
    cpu.pc = 0x267F4;
    16
}
/// $02DC24: move.w d0, $2a(a6)
pub fn i_02dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC28;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02DC28: jsr $2a878(pc)
pub fn i_02dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DC2C);
    cpu.pc = 0x2A878;
    16
}
/// $02DC2C: bcc.w $2dc36
pub fn i_02dc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC30;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DC36; return 10; }
    8
}
/// $02DC30: lea.l $2da38(pc), a1
pub fn i_02dc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02DC34: move.l a1, (a6)
pub fn i_02dc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DC36: jsr $28d70.l
pub fn i_02dc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DC3C);
    cpu.pc = 0x28D70;
    16
}
/// $02DC3C: movea.l #$ffffffff, a0
pub fn i_02dc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC42;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DC42: lea.l $27964e.l, a0
pub fn i_02dc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02DC48: jsr $5dd56.l
pub fn i_02dc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DC4E);
    cpu.pc = 0x5DD56;
    16
}
/// $02DC4E: bcc.w $2dc56
pub fn i_02dc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC52;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DC56; return 10; }
    8
}
/// $02DC52: jmp $2dcbc(pc)
pub fn i_02dc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC56;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
// ===== JsrPcThunk_02dc56  ($02DC56, 6 B, thunks/jsr_pc_thunks.c)
/// $02DC56: jsr $2ac80(pc)
pub fn i_02dc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DC5A);
    cpu.pc = 0x2AC80;
    16
}
/// $02DC5A: rts
pub fn i_02dc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DeathFade_02dc5c  ($02DC5C, 72 B, slug/states_02ae3e.s)
/// $02DC5C: bclr.b #$5, $8c(a6)
pub fn i_02dc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC62;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DC62: move.b #$28, $91(a6)
pub fn i_02dc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DC68: lea.l $ffff.w, a0
pub fn i_02dc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC6C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02DC6C: move.l a0, $48(a6)
pub fn i_02dc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC70;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DC70: move.l #$ffffffff, $60(a6)
pub fn i_02dc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC78;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DC78: lea.l $2dc7e(pc), a1
pub fn i_02dc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DC7E;
    16
}
/// $02DC7C: move.l a1, (a6)
pub fn i_02dc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DC7E: jsr $2a752(pc)
pub fn i_02dc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DC82);
    cpu.pc = 0x2A752;
    16
}
/// $02DC82: subi.b #$1, $91(a6)
pub fn i_02dc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC88;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DC88: bne.w $2dca4
pub fn i_02dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2DCA4; return 10; }
    8
}
/// $02DC8C: jsr $5b6.l
pub fn i_02dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DC92);
    cpu.pc = 0x5B6;
    16
}
/// $02DC92: bclr.b #$7, $5b(a6)
pub fn i_02dc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC98;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DC98: jsr $13600.l
pub fn i_02dc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DC9E);
    cpu.pc = 0x13600;
    16
}
/// $02DC9E: lea.l $2dcaa(pc), a1
pub fn i_02dc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DCAA;
    16
}
/// $02DCA2: move.l a1, (a6)
pub fn i_02dca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_02dca4  ($02DCA4, 6 B, thunks/jsr_pc_thunks.c)
/// $02DCA4: jsr $2ac80(pc)
pub fn i_02dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DCA8);
    cpu.pc = 0x2AC80;
    16
}
/// $02DCA8: rts
pub fn i_02dca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DeathSetHandler400_02dcaa  ($02DCAA, 10 B, slug/states_02ae3e.s)
/// $02DCAA: lea.l $400.l, a1
pub fn i_02dcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCB0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $02DCB0: move.l a1, (a6)
pub fn i_02dcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DCB2: movea.l a6, a0
pub fn i_02dcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCB4;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
// ===== JsrAbsThunk_02dcb4  ($02DCB4, 8 B, thunks/jsr_abs_thunks.c)
/// $02DCB4: jsr $5fe.l
pub fn i_02dcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DCBA);
    cpu.pc = 0x5FE;
    16
}
/// $02DCBA: rts
pub fn i_02dcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DeathFinishJmp_02dcbc  ($02DCBC, 4 B, slug/states_02ae3e.s)
/// $02DCBC: jmp $2dc8c(pc)
pub fn i_02dcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCC0;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DC8C;
    8
}
// ===== Slug_DeathStart_02dcc0  ($02DCC0, 90 B, slug/states_02ae3e.s)
/// $02DCC0: lea.l $2973c(pc), a0
pub fn i_02dcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCC4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2973C;
    16
}
/// $02DCC4: move.l a0, $4c(a6)
pub fn i_02dcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCC8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DCC8: jsr $283ca.l
pub fn i_02dcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DCCE);
    cpu.pc = 0x283CA;
    16
}
/// $02DCCE: jsr $283ca.l
pub fn i_02dcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DCD4);
    cpu.pc = 0x283CA;
    16
}
/// $02DCD4: jsr $283d8.l
pub fn i_02dcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DCDA);
    cpu.pc = 0x283D8;
    16
}
/// $02DCDA: lea.l $ffff.w, a0
pub fn i_02dcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCDE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02DCDE: move.l a0, $4c(a6)
pub fn i_02dcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCE2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DCE2: jsr $283ca.l
pub fn i_02dce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DCE8);
    cpu.pc = 0x283CA;
    16
}
/// $02DCE8: jsr $2ff22(pc)
pub fn i_02dce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DCEC);
    cpu.pc = 0x2FF22;
    16
}
/// $02DCEC: lea.l $2dc5c(pc), a1
pub fn i_02dcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DC5C;
    16
}
/// $02DCF0: move.l a1, (a6)
pub fn i_02dcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DCF2: jsr $2aa24(pc)
pub fn i_02dcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DCF6);
    cpu.pc = 0x2AA24;
    16
}
/// $02DCF6: jsr $2a752(pc)
pub fn i_02dcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DCFA);
    cpu.pc = 0x2A752;
    16
}
/// $02DCFA: jsr $28d70.l
pub fn i_02dcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DD00);
    cpu.pc = 0x28D70;
    16
}
/// $02DD00: movea.l #$ffffffff, a0
pub fn i_02dd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD06;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DD06: lea.l $27964e.l, a0
pub fn i_02dd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02DD0C: jsr $5dd56.l
pub fn i_02dd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DD12);
    cpu.pc = 0x5DD56;
    16
}
/// $02DD12: bcc.w $2dd1a
pub fn i_02dd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DD1A; return 10; }
    8
}
/// $02DD16: jmp $2dcbc(pc)
pub fn i_02dd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD1A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
// ===== JsrPcThunk_02dd1a  ($02DD1A, 6 B, thunks/jsr_pc_thunks.c)
/// $02DD1A: jsr $2ac80(pc)
pub fn i_02dd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DD1E);
    cpu.pc = 0x2AC80;
    16
}
/// $02DD1E: rts
pub fn i_02dd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IdleEnterA_02dd20  ($02DD20, 84 B, slug/vehicle_02dd20.s)
/// $02DD20: bclr.b #$5, $8c(a6)
pub fn i_02dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD26;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DD26: jsr $2a9a0(pc)
pub fn i_02dd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DD2A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DD2A: movea.l #$279614, a0
pub fn i_02dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD30;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279614;
    cpu.a[0] = v1;
    4
}
/// $02DD30: lsl.w #$2, d0
pub fn i_02dd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD32;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DD32: movea.l (a0, d0.w), a0
pub fn i_02dd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD36;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DD36: cmpa.l #$ffffffff, a0
pub fn i_02dd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD3C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DD3C: beq.w $2dd46
pub fn i_02dd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DD46; return 10; }
    8
}
/// $02DD40: jsr $28cd4.l
pub fn i_02dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DD46);
    cpu.pc = 0x28CD4;
    16
}
/// $02DD46: move.b #$41, $20(a6)
pub fn i_02dd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DD4C: andi.w #$ff, d0
pub fn i_02dd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD50;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DD50: lsr.w #$2, d0
pub fn i_02dd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD52;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DD52: add.b d0, $20(a6)
pub fn i_02dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD56;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02DD56: clr.w $2c(a6)
pub fn i_02dd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD5A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DD5A: move.l #$ffffffff, $60(a6)
pub fn i_02dd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD62;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DD62: lea.l $2999e(pc), a0
pub fn i_02dd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD66;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02DD66: move.l a0, $48(a6)
pub fn i_02dd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD6A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DD6A: lea.l $2de18(pc), a1
pub fn i_02dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DE18;
    16
}
/// $02DD6E: move.l a1, (a6)
pub fn i_02dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DD70: jmp $2de18(pc)
pub fn i_02dd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD74;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DE18;
    8
}
// ===== Slug_IdleEnterB_02dd74  ($02DD74, 84 B, slug/vehicle_02dd20.s)
/// $02DD74: bclr.b #$5, $8c(a6)
pub fn i_02dd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD7A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DD7A: jsr $2a9a0(pc)
pub fn i_02dd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DD7E);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DD7E: movea.l #$279628, a0
pub fn i_02dd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD84;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279628;
    cpu.a[0] = v1;
    4
}
/// $02DD84: lsl.w #$2, d0
pub fn i_02dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD86;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DD86: movea.l (a0, d0.w), a0
pub fn i_02dd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD8A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DD8A: cmpa.l #$ffffffff, a0
pub fn i_02dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD90;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DD90: beq.w $2dd9a
pub fn i_02dd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DD9A; return 10; }
    8
}
/// $02DD94: jsr $28cd4.l
pub fn i_02dd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DD9A);
    cpu.pc = 0x28CD4;
    16
}
/// $02DD9A: move.b #$41, $20(a6)
pub fn i_02dd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDA0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DDA0: andi.w #$ff, d0
pub fn i_02dda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDA4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DDA4: lsr.w #$2, d0
pub fn i_02dda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDA6;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DDA6: add.b d0, $20(a6)
pub fn i_02dda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDAA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02DDAA: clr.w $2c(a6)
pub fn i_02ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDAE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DDAE: move.l #$ffffffff, $60(a6)
pub fn i_02ddae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDB6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DDB6: lea.l $2999e(pc), a0
pub fn i_02ddb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDBA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02DDBA: move.l a0, $48(a6)
pub fn i_02ddba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDBE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DDBE: lea.l $2de18(pc), a1
pub fn i_02ddbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DE18;
    16
}
/// $02DDC2: move.l a1, (a6)
pub fn i_02ddc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DDC4: bra.w $2de18
pub fn i_02ddc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2DE18;
    10
}
// ===== Slug_IdleEnterC_02ddc8  ($02DDC8, 80 B, slug/vehicle_02dd20.s)
/// $02DDC8: bclr.b #$5, $8c(a6)
pub fn i_02ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDCE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DDCE: jsr $2a9a0(pc)
pub fn i_02ddce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DDD2);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DDD2: movea.l #$279614, a0
pub fn i_02ddd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDD8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279614;
    cpu.a[0] = v1;
    4
}
/// $02DDD8: lsl.w #$2, d0
pub fn i_02ddd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDDA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DDDA: movea.l (a0, d0.w), a0
pub fn i_02ddda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDDE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DDDE: cmpa.l #$ffffffff, a0
pub fn i_02ddde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDE4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DDE4: beq.w $2ddee
pub fn i_02dde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DDEE; return 10; }
    8
}
/// $02DDE8: jsr $28cd4.l
pub fn i_02dde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DDEE);
    cpu.pc = 0x28CD4;
    16
}
/// $02DDEE: move.b #$41, $20(a6)
pub fn i_02ddee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DDF4: andi.w #$ff, d0
pub fn i_02ddf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DDF8: lsr.w #$2, d0
pub fn i_02ddf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDFA;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DDFA: add.b d0, $20(a6)
pub fn i_02ddfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDFE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02DDFE: clr.w $2c(a6)
pub fn i_02ddfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DE02: move.l #$ffffffff, $60(a6)
pub fn i_02de02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE0A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DE0A: lea.l $2999e(pc), a0
pub fn i_02de0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE0E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02DE0E: move.l a0, $48(a6)
pub fn i_02de0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE12;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DE12: lea.l $2de18(pc), a1
pub fn i_02de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DE18;
    16
}
/// $02DE16: move.l a1, (a6)
pub fn i_02de16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Slug_Idle_02de18  ($02DE18, 156 B, slug/vehicle_02dd20.s)
/// $02DE18: jsr $2aa24(pc)
pub fn i_02de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE1C);
    cpu.pc = 0x2AA24;
    16
}
/// $02DE1C: bset.b #$4, $8d(a6)
pub fn i_02de1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE22;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DE22: bset.b #$5, $8d(a6)
pub fn i_02de22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE28;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DE28: clr.w $28(a6)
pub fn i_02de28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE2C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DE2C: jsr $2a824(pc)
pub fn i_02de2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE30);
    cpu.pc = 0x2A824;
    16
}
/// $02DE30: bcc.w $2de3a
pub fn i_02de30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE3A; return 10; }
    8
}
/// $02DE34: lea.l $2deb4(pc), a1
pub fn i_02de34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DEB4;
    16
}
/// $02DE38: move.l a1, (a6)
pub fn i_02de38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE3A: jsr $2a478(pc)
pub fn i_02de3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE3E);
    cpu.pc = 0x2A478;
    16
}
/// $02DE3E: jsr $28d70.l
pub fn i_02de3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DE44);
    cpu.pc = 0x28D70;
    16
}
/// $02DE44: jsr $2a8c0(pc)
pub fn i_02de44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE48);
    cpu.pc = 0x2A8C0;
    16
}
/// $02DE48: bcc.w $2de52
pub fn i_02de48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE52; return 10; }
    8
}
/// $02DE4C: lea.l $2e092(pc), a1
pub fn i_02de4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E092;
    16
}
/// $02DE50: move.l a1, (a6)
pub fn i_02de50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE52: jsr $2acb8(pc)
pub fn i_02de52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE56);
    cpu.pc = 0x2ACB8;
    16
}
/// $02DE56: bcc.w $2de60
pub fn i_02de56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE5A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE60; return 10; }
    8
}
/// $02DE5A: lea.l $2e3b0(pc), a1
pub fn i_02de5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E3B0;
    16
}
/// $02DE5E: move.l a1, (a6)
pub fn i_02de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE60: jsr $2a664(pc)
pub fn i_02de60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE64);
    cpu.pc = 0x2A664;
    16
}
/// $02DE64: bcc.w $2de6e
pub fn i_02de64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE6E; return 10; }
    8
}
/// $02DE68: lea.l $2da38(pc), a1
pub fn i_02de68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02DE6C: move.l a1, (a6)
pub fn i_02de6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE6E: cmpi.b #$0, $106f4a.l
pub fn i_02de6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE76;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F4A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02DE76: beq.w $2de98
pub fn i_02de76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE7A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DE98; return 10; }
    8
}
/// $02DE7A: movea.l #$ffffffff, a0
pub fn i_02de7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE80;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DE80: lea.l $27964e.l, a0
pub fn i_02de80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02DE86: jsr $5dd56.l
pub fn i_02de86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DE8C);
    cpu.pc = 0x5DD56;
    16
}
/// $02DE8C: bcc.w $2de94
pub fn i_02de8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE94; return 10; }
    8
}
/// $02DE90: jmp $2dcbc(pc)
pub fn i_02de90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE94;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02DE94: bra.w $2deb2
pub fn i_02de94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE98;
    cpu.ir = 0x6000;
    cpu.pc = 0x2DEB2;
    10
}
/// $02DE98: movea.l #$ffffffff, a0
pub fn i_02de98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE9E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DE9E: lea.l $27965a.l, a0
pub fn i_02de9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27965A;
    20
}
/// $02DEA4: jsr $5dd56.l
pub fn i_02dea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DEAA);
    cpu.pc = 0x5DD56;
    16
}
/// $02DEAA: bcc.w $2deb2
pub fn i_02deaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DEB2; return 10; }
    8
}
/// $02DEAE: jmp $2dcbc(pc)
pub fn i_02deae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEB2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02DEB2: rts
pub fn i_02deb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IdleAngled_02deb4  ($02DEB4, 406 B, slug/vehicle_02dd20.s)
/// $02DEB4: clr.w $2c(a6)
pub fn i_02deb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEB8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DEB8: jsr $2a958(pc)
pub fn i_02deb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DEBC);
    cpu.pc = 0x2A958;
    16
}
/// $02DEBC: move.w d2, $80(a6)
pub fn i_02debc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEC0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02DEC0: move.w d2, d0
pub fn i_02dec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEC2;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DEC2: asr.w #$1, d0
pub fn i_02dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEC4;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02DEC4: neg.w d0
pub fn i_02dec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEC6;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02DEC6: addi.w #$20, d0
pub fn i_02dec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DECA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DECA: cmpi.w #$40, d0
pub fn i_02deca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DECE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02DECE: bls.w $2ded6
pub fn i_02dece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DED2;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2DED6; return 10; }
    8
}
/// $02DED2: move.w #$40, d0
pub fn i_02ded2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DED6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DED6: movem.w d0, -(a7)
pub fn i_02ded6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEDA;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02DEDA: jsr $295a6(pc)
pub fn i_02deda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DEDE);
    cpu.pc = 0x295A6;
    16
}
/// $02DEDE: movea.l #$2b0dc8, a0
pub fn i_02dede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEE4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0DC8;
    cpu.a[0] = v1;
    4
}
/// $02DEE4: lsl.w #$2, d0
pub fn i_02dee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEE6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DEE6: movea.l (a0, d0.w), a0
pub fn i_02dee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEEA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DEEA: cmpa.l #$ffffffff, a0
pub fn i_02deea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEF0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DEF0: beq.w $2defa
pub fn i_02def0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DEFA; return 10; }
    8
}
/// $02DEF4: jsr $28cd4.l
pub fn i_02def4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DEFA);
    cpu.pc = 0x28CD4;
    16
}
/// $02DEFA: movem.w (a7)+, d0
pub fn i_02defa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEFE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02DEFE: move.w #$0, d0
pub fn i_02defe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF02;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DF02: move.b #$c3, $20(a6)
pub fn i_02df02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DF08: move.l #$ffffffff, $60(a6)
pub fn i_02df08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF10;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DF10: lea.l $2999e(pc), a0
pub fn i_02df10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF14;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02DF14: move.l a0, $48(a6)
pub fn i_02df14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF18;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DF18: lea.l $2df1e(pc), a1
pub fn i_02df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DF1E;
    16
}
/// $02DF1C: move.l a1, (a6)
pub fn i_02df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DF1E: bset.b #$2, $8d(a6)
pub fn i_02df1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF24;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DF24: bset.b #$4, $8d(a6)
pub fn i_02df24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF2A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DF2A: bset.b #$5, $8d(a6)
pub fn i_02df2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF30;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DF30: jsr $2aa24(pc)
pub fn i_02df30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DF34);
    cpu.pc = 0x2AA24;
    16
}
/// $02DF34: clr.w $28(a6)
pub fn i_02df34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF38;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DF38: cmpi.b #$3, $106f2a.l
pub fn i_02df38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF40;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02DF40: bne.w $2df62
pub fn i_02df40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF44;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2DF62; return 10; }
    8
}
/// $02DF44: move.w $28(a6), d7
pub fn i_02df44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF48;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02DF48: jsr $2a958(pc)
pub fn i_02df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DF4C);
    cpu.pc = 0x2A958;
    16
}
/// $02DF4C: sub.w $80(a6), d2
pub fn i_02df4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF50;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02DF50: eor.w d7, d2
pub fn i_02df50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF52;
    cpu.ir = 0xBF42;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[7]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02DF52: asl.w #$1, d2
pub fn i_02df52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF54;
    cpu.ir = 0xE342;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02DF54: bcs.w $2df5e
pub fn i_02df54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF58;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2DF5E; return 10; }
    8
}
/// $02DF58: move.w d7, d2
pub fn i_02df58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF5A;
    cpu.ir = 0x3407;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DF5A: asr.w #$1, d2
pub fn i_02df5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF5C;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02DF5C: add.w d2, d7
pub fn i_02df5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF5E;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02DF5E: move.w d7, $28(a6)
pub fn i_02df5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF62;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02DF62: jsr $2a478(pc)
pub fn i_02df62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DF66);
    cpu.pc = 0x2A478;
    16
}
/// $02DF66: jsr $2a766(pc)
pub fn i_02df66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DF6A);
    cpu.pc = 0x2A766;
    16
}
/// $02DF6A: jsr $2a958(pc)
pub fn i_02df6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DF6E);
    cpu.pc = 0x2A958;
    16
}
/// $02DF6E: cmp.w $80(a6), d2
pub fn i_02df6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF72;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02DF72: beq.w $2dfc2
pub fn i_02df72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DFC2; return 10; }
    8
}
/// $02DF76: move.w d2, $80(a6)
pub fn i_02df76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF7A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02DF7A: move.w d2, d0
pub fn i_02df7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF7C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DF7C: asr.w #$1, d0
pub fn i_02df7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF7E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02DF7E: neg.w d0
pub fn i_02df7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF80;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02DF80: addi.w #$20, d0
pub fn i_02df80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF84;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DF84: cmpi.w #$40, d0
pub fn i_02df84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF88;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02DF88: bls.w $2df90
pub fn i_02df88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF8C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2DF90; return 10; }
    8
}
/// $02DF8C: move.w #$40, d0
pub fn i_02df8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF90;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DF90: movem.w d0, -(a7)
pub fn i_02df90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF94;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02DF94: jsr $295a6(pc)
pub fn i_02df94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DF98);
    cpu.pc = 0x295A6;
    16
}
/// $02DF98: movea.l #$2b0dc8, a0
pub fn i_02df98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF9E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0DC8;
    cpu.a[0] = v1;
    4
}
/// $02DF9E: lsl.w #$2, d0
pub fn i_02df9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFA0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DFA0: movea.l (a0, d0.w), a0
pub fn i_02dfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFA4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DFA4: cmpa.l #$ffffffff, a0
pub fn i_02dfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFAA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DFAA: beq.w $2dfb4
pub fn i_02dfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DFB4; return 10; }
    8
}
/// $02DFAE: jsr $28cd4.l
pub fn i_02dfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DFB4);
    cpu.pc = 0x28CD4;
    16
}
/// $02DFB4: movem.w (a7)+, d0
pub fn i_02dfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFB8;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02DFB8: move.w #$0, d0
pub fn i_02dfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02DFBC: move.b #$c3, $20(a6)
pub fn i_02dfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DFC2: jsr $28d70.l
pub fn i_02dfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DFC8);
    cpu.pc = 0x28D70;
    16
}
/// $02DFC8: jsr $2a4ec(pc)
pub fn i_02dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DFCC);
    cpu.pc = 0x2A4EC;
    16
}
/// $02DFCC: bcs.w $2dfda
pub fn i_02dfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFD0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2DFDA; return 10; }
    8
}
/// $02DFD0: bra.w $2dfd4
pub fn i_02dfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFD4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2DFD4;
    10
}
/// $02DFD4: lea.l $2ddc8(pc), a1
pub fn i_02dfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DDC8;
    16
}
/// $02DFD8: move.l a1, (a6)
pub fn i_02dfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DFDA: jsr $2a8c0(pc)
pub fn i_02dfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DFDE);
    cpu.pc = 0x2A8C0;
    16
}
/// $02DFDE: bcc.w $2dfe8
pub fn i_02dfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DFE8; return 10; }
    8
}
/// $02DFE2: lea.l $2e04a(pc), a1
pub fn i_02dfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E04A;
    16
}
/// $02DFE6: move.l a1, (a6)
pub fn i_02dfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFE8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DFE8: jsr $2acb8(pc)
pub fn i_02dfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DFEC);
    cpu.pc = 0x2ACB8;
    16
}
/// $02DFEC: bcc.w $2dff6
pub fn i_02dfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DFF6; return 10; }
    8
}
/// $02DFF0: lea.l $2e496(pc), a1
pub fn i_02dff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E496;
    16
}
/// $02DFF4: move.l a1, (a6)
pub fn i_02dff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DFF6: jsr $2a664(pc)
pub fn i_02dff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DFFA);
    cpu.pc = 0x2A664;
    16
}
/// $02DFFA: bcc.w $2e004
pub fn i_02dffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E004; return 10; }
    8
}
/// $02DFFE: lea.l $2da38(pc), a1
pub fn i_02dffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E002;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02E002: move.l a1, (a6)
pub fn i_02e002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E004;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E004: cmpi.b #$0, $106f4a.l
pub fn i_02e004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E00C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F4A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02E00C: beq.w $2e02e
pub fn i_02e00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E010;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E02E; return 10; }
    8
}
/// $02E010: movea.l #$ffffffff, a0
pub fn i_02e010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E016;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02E016: lea.l $27964e.l, a0
pub fn i_02e016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E01C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02E01C: jsr $5dd56.l
pub fn i_02e01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E022);
    cpu.pc = 0x5DD56;
    16
}
/// $02E022: bcc.w $2e02a
pub fn i_02e022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E026;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E02A; return 10; }
    8
}
/// $02E026: jmp $2dcbc(pc)
pub fn i_02e026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E02A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02E02A: bra.w $2e048
pub fn i_02e02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E02E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E048;
    10
}
/// $02E02E: movea.l #$ffffffff, a0
pub fn i_02e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E034;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02E034: lea.l $27965a.l, a0
pub fn i_02e034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E03A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27965A;
    20
}
/// $02E03A: jsr $5dd56.l
pub fn i_02e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E040);
    cpu.pc = 0x5DD56;
    16
}
/// $02E040: bcc.w $2e048
pub fn i_02e040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E044;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E048; return 10; }
    8
}
/// $02E044: jmp $2dcbc(pc)
pub fn i_02e044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E048;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02E048: rts
pub fn i_02e048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E04A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JumpB_02e04a  ($02E04A, 72 B, slug/vehicle_02dd20.s)
/// $02E04A: bclr.b #$5, $8c(a6)
pub fn i_02e04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E050;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E050: move.w #$ffe0, $2e(a6)
pub fn i_02e050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E056;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E056: clr.w $28(a6)
pub fn i_02e056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E05A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E05A: clr.w $2c(a6)
pub fn i_02e05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E05E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E05E: addq.w #$4, $82(a6)
pub fn i_02e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E062;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E062: move.w $24(a6), d0
pub fn i_02e062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E066;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02E066: cmp.w $82(a6), d0
pub fn i_02e066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E06A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02E06A: ble.w $2e072
pub fn i_02e06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E06E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2E072; return 10; }
    8
}
/// $02E06E: move.w d0, $82(a6)
pub fn i_02e06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E072;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E072: move.l #$ffffffff, $60(a6)
pub fn i_02e072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E07A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E07A: lea.l $2999e(pc), a0
pub fn i_02e07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E07E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02E07E: move.l a0, $48(a6)
pub fn i_02e07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E082;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E082: lea.l $2e088(pc), a1
pub fn i_02e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E086;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E088;
    16
}
/// $02E086: move.l a1, (a6)
pub fn i_02e086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E088;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E088: bset.b #$2, $8d(a6)
pub fn i_02e088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E08E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E08E: jmp $2e12c(pc)
pub fn i_02e08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E092;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2E12C;
    8
}
// ===== Slug_Jump_02e092  ($02E092, 434 B, slug/vehicle_02dd20.s)
/// $02E092: bclr.b #$5, $8c(a6)
pub fn i_02e092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E098;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E098: move.w #$ffe0, $2e(a6)
pub fn i_02e098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E09E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E09E: clr.w $28(a6)
pub fn i_02e09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0A2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E0A2: clr.w $2c(a6)
pub fn i_02e0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0A6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E0A6: addq.w #$4, $82(a6)
pub fn i_02e0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0AA;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E0AA: move.w $24(a6), d0
pub fn i_02e0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0AE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02E0AE: cmp.w $82(a6), d0
pub fn i_02e0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0B2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02E0B2: ble.w $2e0ba
pub fn i_02e0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0B6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2E0BA; return 10; }
    8
}
/// $02E0B6: move.w d0, $82(a6)
pub fn i_02e0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0BA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E0BA: jsr $2a958(pc)
pub fn i_02e0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E0BE);
    cpu.pc = 0x2A958;
    16
}
/// $02E0BE: move.w d2, $80(a6)
pub fn i_02e0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0C2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E0C2: move.w d2, d0
pub fn i_02e0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0C4;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E0C4: asr.w #$1, d0
pub fn i_02e0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0C6;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02E0C6: neg.w d0
pub fn i_02e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0C8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02E0C8: addi.w #$20, d0
pub fn i_02e0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0CC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E0CC: cmpi.w #$40, d0
pub fn i_02e0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0D0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02E0D0: bls.w $2e0d8
pub fn i_02e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0D4;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2E0D8; return 10; }
    8
}
/// $02E0D4: move.w #$40, d0
pub fn i_02e0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E0D8: movem.w d0, -(a7)
pub fn i_02e0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0DC;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02E0DC: jsr $295a6(pc)
pub fn i_02e0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E0E0);
    cpu.pc = 0x295A6;
    16
}
/// $02E0E0: movea.l #$2b0dc8, a0
pub fn i_02e0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0E6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0DC8;
    cpu.a[0] = v1;
    4
}
/// $02E0E6: lsl.w #$2, d0
pub fn i_02e0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E0E8: movea.l (a0, d0.w), a0
pub fn i_02e0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E0EC: cmpa.l #$ffffffff, a0
pub fn i_02e0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0F2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E0F2: beq.w $2e0fc
pub fn i_02e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E0FC; return 10; }
    8
}
/// $02E0F6: jsr $28cd4.l
pub fn i_02e0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E0FC);
    cpu.pc = 0x28CD4;
    16
}
/// $02E0FC: movem.w (a7)+, d0
pub fn i_02e0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E100;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02E100: move.b #$0, $20(a6)
pub fn i_02e100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E106;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E106: lsr.w #$2, d0
pub fn i_02e106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E108;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E108: andi.w #$ff, d0
pub fn i_02e108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E10C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E10C: add.b d0, $20(a6)
pub fn i_02e10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E110;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E110: bset.b #$2, $8d(a6)
pub fn i_02e110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E116;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E116: move.l #$ffffffff, $60(a6)
pub fn i_02e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E11E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E11E: lea.l $2999e(pc), a0
pub fn i_02e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E122;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02E122: move.l a0, $48(a6)
pub fn i_02e122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E126;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E126: lea.l $2e12c(pc), a1
pub fn i_02e126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E12A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E12C;
    16
}
/// $02E12A: move.l a1, (a6)
pub fn i_02e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E12C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E12C: move.w $2a(a6), d0
pub fn i_02e12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E130;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02E130: move.w #$400, d1
pub fn i_02e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E134;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E134: jsr $267f4.l
pub fn i_02e134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E13A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E13A);
    cpu.pc = 0x267F4;
    16
}
/// $02E13A: move.w d0, $2a(a6)
pub fn i_02e13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E13E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E13E: bset.b #$1, $8d(a6)
pub fn i_02e13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E144;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E144: bset.b #$5, $8d(a6)
pub fn i_02e144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E14A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E14A: jsr $2aa0e(pc)
pub fn i_02e14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E14E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E14E);
    cpu.pc = 0x2AA0E;
    16
}
/// $02E14E: jsr $2a878(pc)
pub fn i_02e14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E152;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E152);
    cpu.pc = 0x2A878;
    16
}
/// $02E152: bcc.w $2e15c
pub fn i_02e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E156;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E15C; return 10; }
    8
}
/// $02E156: lea.l $2ddc8(pc), a1
pub fn i_02e156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E15A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DDC8;
    16
}
/// $02E15A: move.l a1, (a6)
pub fn i_02e15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E15C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E15C: jsr $2a478(pc)
pub fn i_02e15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E160;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E160);
    cpu.pc = 0x2A478;
    16
}
/// $02E160: jsr $28d70.l
pub fn i_02e160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E166);
    cpu.pc = 0x28D70;
    16
}
/// $02E166: tst.w $2a(a6)
pub fn i_02e166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E16A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02E16A: bgt.w $2e1e6
pub fn i_02e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E16E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2E1E6; return 10; }
    8
}
/// $02E16E: jsr $2a8c0(pc)
pub fn i_02e16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E172;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E172);
    cpu.pc = 0x2A8C0;
    16
}
/// $02E172: cmpi.b #$0, d1
pub fn i_02e172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E176;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02E176: ble.w $2e182
pub fn i_02e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E17A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2E182; return 10; }
    8
}
/// $02E17A: clr.w $28(a6)
pub fn i_02e17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E17E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E17E: clr.w $2c(a6)
pub fn i_02e17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E182;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E182: jsr $2a958(pc)
pub fn i_02e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E186;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E186);
    cpu.pc = 0x2A958;
    16
}
/// $02E186: cmp.w $80(a6), d2
pub fn i_02e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E18A;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02E18A: beq.w $2e1e6
pub fn i_02e18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E18E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E1E6; return 10; }
    8
}
/// $02E18E: move.w d2, $80(a6)
pub fn i_02e18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E192;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E192: move.w d2, d0
pub fn i_02e192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E194;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E194: asr.w #$1, d0
pub fn i_02e194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E196;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02E196: neg.w d0
pub fn i_02e196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E198;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02E198: addi.w #$20, d0
pub fn i_02e198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E19C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E19C: cmpi.w #$40, d0
pub fn i_02e19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1A0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02E1A0: bls.w $2e1a8
pub fn i_02e1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1A4;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2E1A8; return 10; }
    8
}
/// $02E1A4: move.w #$40, d0
pub fn i_02e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E1A8: movem.w d0, -(a7)
pub fn i_02e1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1AC;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02E1AC: jsr $295a6(pc)
pub fn i_02e1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E1B0);
    cpu.pc = 0x295A6;
    16
}
/// $02E1B0: movea.l #$2b0dc8, a0
pub fn i_02e1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1B6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0DC8;
    cpu.a[0] = v1;
    4
}
/// $02E1B6: lsl.w #$2, d0
pub fn i_02e1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1B8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E1B8: movea.l (a0, d0.w), a0
pub fn i_02e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1BC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E1BC: cmpa.l #$ffffffff, a0
pub fn i_02e1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1C2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E1C2: beq.w $2e1cc
pub fn i_02e1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E1CC; return 10; }
    8
}
/// $02E1C6: jsr $28cd4.l
pub fn i_02e1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E1CC);
    cpu.pc = 0x28CD4;
    16
}
/// $02E1CC: movem.w (a7)+, d0
pub fn i_02e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1D0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02E1D0: move.b #$0, $20(a6)
pub fn i_02e1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E1D6: lsr.w #$2, d0
pub fn i_02e1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1D8;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E1D8: andi.w #$ff, d0
pub fn i_02e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E1DC: add.b d0, $20(a6)
pub fn i_02e1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1E0;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E1E0: bset.b #$2, $8d(a6)
pub fn i_02e1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1E6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E1E6: btst.b #$1, $8d(a6)
pub fn i_02e1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1EC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02E1EC: bne.w $2e1fe
pub fn i_02e1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2E1FE; return 10; }
    8
}
/// $02E1F0: jsr $2a664(pc)
pub fn i_02e1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E1F4);
    cpu.pc = 0x2A664;
    16
}
/// $02E1F4: bcc.w $2e1fe
pub fn i_02e1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E1FE; return 10; }
    8
}
/// $02E1F8: lea.l $2da38(pc), a1
pub fn i_02e1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02E1FC: move.l a1, (a6)
pub fn i_02e1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E1FE: cmpi.b #$0, $106f4a.l
pub fn i_02e1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E206;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F4A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02E206: beq.w $2e228
pub fn i_02e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E20A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E228; return 10; }
    8
}
/// $02E20A: movea.l #$ffffffff, a0
pub fn i_02e20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E210;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02E210: lea.l $27964e.l, a0
pub fn i_02e210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E216;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02E216: jsr $5dd56.l
pub fn i_02e216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E21C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E21C);
    cpu.pc = 0x5DD56;
    16
}
/// $02E21C: bcc.w $2e224
pub fn i_02e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E220;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E224; return 10; }
    8
}
/// $02E220: jmp $2dcbc(pc)
pub fn i_02e220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E224;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02E224: bra.w $2e242
pub fn i_02e224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E228;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E242;
    10
}
/// $02E228: movea.l #$ffffffff, a0
pub fn i_02e228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E22E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02E22E: lea.l $27965a.l, a0
pub fn i_02e22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E234;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27965A;
    20
}
/// $02E234: jsr $5dd56.l
pub fn i_02e234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E23A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E23A);
    cpu.pc = 0x5DD56;
    16
}
/// $02E23A: bcc.w $2e242
pub fn i_02e23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E23E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E242; return 10; }
    8
}
/// $02E23E: jmp $2dcbc(pc)
pub fn i_02e23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E242;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02E242: rts
pub fn i_02e242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E244;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IdleAngledB_02e244  ($02E244, 356 B, slug/vehicle_02dd20.s)
/// $02E244: clr.w $2c(a6)
pub fn i_02e244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E248;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E248: jsr $2a958(pc)
pub fn i_02e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E24C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E24C);
    cpu.pc = 0x2A958;
    16
}
/// $02E24C: move.w d2, $80(a6)
pub fn i_02e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E250;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E250: move.w d2, d0
pub fn i_02e250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E252;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E252: asr.w #$1, d0
pub fn i_02e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E254;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02E254: neg.w d0
pub fn i_02e254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E256;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02E256: addi.w #$20, d0
pub fn i_02e256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E25A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E25A: cmpi.w #$40, d0
pub fn i_02e25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E25E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02E25E: bls.w $2e266
pub fn i_02e25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E262;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2E266; return 10; }
    8
}
/// $02E262: move.w #$40, d0
pub fn i_02e262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E266;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E266: movem.w d0, -(a7)
pub fn i_02e266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E26A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02E26A: jsr $295a6(pc)
pub fn i_02e26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E26E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E26E);
    cpu.pc = 0x295A6;
    16
}
/// $02E26E: movea.l #$2b0dc8, a0
pub fn i_02e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E274;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0DC8;
    cpu.a[0] = v1;
    4
}
/// $02E274: lsl.w #$2, d0
pub fn i_02e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E276;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E276: movea.l (a0, d0.w), a0
pub fn i_02e276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E27A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E27A: cmpa.l #$ffffffff, a0
pub fn i_02e27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E280;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E280: beq.w $2e28a
pub fn i_02e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E284;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E28A; return 10; }
    8
}
/// $02E284: jsr $28cd4.l
pub fn i_02e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E28A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E28A);
    cpu.pc = 0x28CD4;
    16
}
/// $02E28A: movem.w (a7)+, d0
pub fn i_02e28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E28E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02E28E: move.w #$0, d0
pub fn i_02e28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E292;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E292: move.b #$c3, $20(a6)
pub fn i_02e292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E298;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E298: move.l #$ffffffff, $60(a6)
pub fn i_02e298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2A0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E2A0: lea.l $2999e(pc), a0
pub fn i_02e2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2A4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02E2A4: move.l a0, $48(a6)
pub fn i_02e2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2A8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E2A8: lea.l $2e2ae(pc), a1
pub fn i_02e2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E2AE;
    16
}
/// $02E2AC: move.l a1, (a6)
pub fn i_02e2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E2AE: bset.b #$2, $8d(a6)
pub fn i_02e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2B4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E2B4: bset.b #$4, $8d(a6)
pub fn i_02e2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2BA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E2BA: bset.b #$5, $8d(a6)
pub fn i_02e2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2C0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E2C0: jsr $2aa24(pc)
pub fn i_02e2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E2C4);
    cpu.pc = 0x2AA24;
    16
}
/// $02E2C4: clr.w $28(a6)
pub fn i_02e2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2C8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E2C8: cmpi.b #$3, $106f2a.l
pub fn i_02e2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2D0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02E2D0: bne.w $2e2f2
pub fn i_02e2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2E2F2; return 10; }
    8
}
/// $02E2D4: move.w $28(a6), d7
pub fn i_02e2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2D8;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02E2D8: jsr $2a958(pc)
pub fn i_02e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E2DC);
    cpu.pc = 0x2A958;
    16
}
/// $02E2DC: sub.w $80(a6), d2
pub fn i_02e2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2E0;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02E2E0: eor.w d7, d2
pub fn i_02e2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2E2;
    cpu.ir = 0xBF42;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[7]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02E2E2: asl.w #$1, d2
pub fn i_02e2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2E4;
    cpu.ir = 0xE342;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02E2E4: bcs.w $2e2ee
pub fn i_02e2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E2EE; return 10; }
    8
}
/// $02E2E8: move.w d7, d2
pub fn i_02e2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2EA;
    cpu.ir = 0x3407;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E2EA: asr.w #$1, d2
pub fn i_02e2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2EC;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02E2EC: add.w d2, d7
pub fn i_02e2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2EE;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02E2EE: move.w d7, $28(a6)
pub fn i_02e2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2F2;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E2F2: jsr $2a478(pc)
pub fn i_02e2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E2F6);
    cpu.pc = 0x2A478;
    16
}
/// $02E2F6: jsr $28992.l
pub fn i_02e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E2FC);
    cpu.pc = 0x28992;
    16
}
/// $02E2FC: jsr $2a766(pc)
pub fn i_02e2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E300;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E300);
    cpu.pc = 0x2A766;
    16
}
/// $02E300: jsr $2a958(pc)
pub fn i_02e300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E304;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E304);
    cpu.pc = 0x2A958;
    16
}
/// $02E304: cmp.w $80(a6), d2
pub fn i_02e304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E308;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02E308: beq.w $2e358
pub fn i_02e308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E30C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E358; return 10; }
    8
}
/// $02E30C: move.w d2, $80(a6)
pub fn i_02e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E310;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E310: move.w d2, d0
pub fn i_02e310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E312;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E312: asr.w #$1, d0
pub fn i_02e312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E314;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02E314: neg.w d0
pub fn i_02e314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E316;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02E316: addi.w #$20, d0
pub fn i_02e316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E31A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E31A: cmpi.w #$40, d0
pub fn i_02e31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E31E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02E31E: bls.w $2e326
pub fn i_02e31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E322;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2E326; return 10; }
    8
}
/// $02E322: move.w #$40, d0
pub fn i_02e322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E326;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E326: movem.w d0, -(a7)
pub fn i_02e326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E32A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02E32A: jsr $295a6(pc)
pub fn i_02e32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E32E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E32E);
    cpu.pc = 0x295A6;
    16
}
/// $02E32E: movea.l #$2b0dc8, a0
pub fn i_02e32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E334;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0DC8;
    cpu.a[0] = v1;
    4
}
/// $02E334: lsl.w #$2, d0
pub fn i_02e334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E336;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E336: movea.l (a0, d0.w), a0
pub fn i_02e336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E33A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E33A: cmpa.l #$ffffffff, a0
pub fn i_02e33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E340;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E340: beq.w $2e34a
pub fn i_02e340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E344;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E34A; return 10; }
    8
}
/// $02E344: jsr $28cd4.l
pub fn i_02e344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E34A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E34A);
    cpu.pc = 0x28CD4;
    16
}
/// $02E34A: movem.w (a7)+, d0
pub fn i_02e34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E34E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02E34E: move.w #$0, d0
pub fn i_02e34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E352;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E352: move.b #$c3, $20(a6)
pub fn i_02e352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E358;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E358: jsr $28d70.l
pub fn i_02e358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E35E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E35E);
    cpu.pc = 0x28D70;
    16
}
/// $02E35E: jsr $2a4ec(pc)
pub fn i_02e35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E362;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E362);
    cpu.pc = 0x2A4EC;
    16
}
/// $02E362: bcs.w $2e370
pub fn i_02e362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E366;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E370; return 10; }
    8
}
/// $02E366: bra.w $2e36a
pub fn i_02e366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E36A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E36A;
    10
}
/// $02E36A: lea.l $2ddc8(pc), a1
pub fn i_02e36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E36E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DDC8;
    16
}
/// $02E36E: move.l a1, (a6)
pub fn i_02e36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E370;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E370: tst.b $3b(a6)
pub fn i_02e370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E374;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02E374: bne.w $2e37e
pub fn i_02e374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E378;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2E37E; return 10; }
    8
}
/// $02E378: jsr $283d8.l
pub fn i_02e378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E37E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E37E);
    cpu.pc = 0x283D8;
    16
}
/// $02E37E: jsr $2a59a(pc)
pub fn i_02e37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E382;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E382);
    cpu.pc = 0x2A59A;
    16
}
/// $02E382: bcs.w $2e38c
pub fn i_02e382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E386;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E38C; return 10; }
    8
}
/// $02E386: lea.l $2e3b0(pc), a1
pub fn i_02e386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E38A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E3B0;
    16
}
/// $02E38A: move.l a1, (a6)
pub fn i_02e38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E38C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E38C: jsr $2a8c0(pc)
pub fn i_02e38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E390;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E390);
    cpu.pc = 0x2A8C0;
    16
}
/// $02E390: bcc.w $2e39a
pub fn i_02e390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E394;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E39A; return 10; }
    8
}
/// $02E394: lea.l $2cffa(pc), a1
pub fn i_02e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E398;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CFFA;
    16
}
/// $02E398: move.l a1, (a6)
pub fn i_02e398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E39A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E39A: jsr $2a690(pc)
pub fn i_02e39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E39E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E39E);
    cpu.pc = 0x2A690;
    16
}
/// $02E39E: lea.l $2a060(pc), a0
pub fn i_02e39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3A2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02E3A2: movea.l #$ffffffff, a1
pub fn i_02e3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3A8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02e3a8  ($02E3A8, 8 B, thunks/jsr_abs_thunks.c)
/// $02E3A8: jsr $772.l
pub fn i_02e3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E3AE);
    cpu.pc = 0x772;
    16
}
/// $02E3AE: rts
pub fn i_02e3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Hunker_02e3b0  ($02E3B0, 222 B, slug/vehicle_02dd20.s)
/// $02E3B0: bset.b #$0, $6b(a6)
pub fn i_02e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3B6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E3B6: bclr.b #$5, $8c(a6)
pub fn i_02e3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E3BC: bclr.b #$1, $8c(a6)
pub fn i_02e3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3C2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E3C2: jsr $2a9a0(pc)
pub fn i_02e3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E3C6);
    cpu.pc = 0x2A9A0;
    16
}
/// $02E3C6: movea.l #$279600, a0
pub fn i_02e3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279600;
    cpu.a[0] = v1;
    4
}
/// $02E3CC: lsl.w #$2, d0
pub fn i_02e3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3CE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E3CE: movea.l (a0, d0.w), a0
pub fn i_02e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3D2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E3D2: cmpa.l #$ffffffff, a0
pub fn i_02e3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3D8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E3D8: beq.w $2e3e2
pub fn i_02e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E3E2; return 10; }
    8
}
/// $02E3DC: jsr $28cd4.l
pub fn i_02e3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E3E2);
    cpu.pc = 0x28CD4;
    16
}
/// $02E3E2: lea.l $2a024(pc), a1
pub fn i_02e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A024;
    16
}
/// $02E3E6: movea.l (a1, d0.w), a0
pub fn i_02e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3EA;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E3EA: move.l a0, $48(a6)
pub fn i_02e3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3EE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E3EE: move.b #$28, $20(a6)
pub fn i_02e3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E3F4: andi.w #$ff, d0
pub fn i_02e3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E3F8: lsr.w #$2, d0
pub fn i_02e3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3FA;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E3FA: add.b d0, $20(a6)
pub fn i_02e3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3FE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E3FE: lea.l $3d944.l, a1
pub fn i_02e3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E404;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3D944;
    20
}
/// $02E404: jsr $4ae.l
pub fn i_02e404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E40A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E40A);
    cpu.pc = 0x4AE;
    16
}
/// $02E40A: jsr $5dd02.l
pub fn i_02e40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E410);
    cpu.pc = 0x5DD02;
    16
}
/// $02E410: move.l #$ffffffff, $60(a6)
pub fn i_02e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E418;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E418: lea.l $2999e(pc), a0
pub fn i_02e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E41C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02E41C: move.l a0, $48(a6)
pub fn i_02e41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E420;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E420: clr.w $2c(a6)
pub fn i_02e420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E424;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E424: lea.l $2e42a(pc), a1
pub fn i_02e424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E428;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E42A;
    16
}
/// $02E428: move.l a1, (a6)
pub fn i_02e428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E42A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E42A: jsr $2aa24(pc)
pub fn i_02e42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E42E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E42E);
    cpu.pc = 0x2AA24;
    16
}
/// $02E42E: clr.w $28(a6)
pub fn i_02e42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E432;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E432: jsr $28992.l
pub fn i_02e432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E438;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E438);
    cpu.pc = 0x28992;
    16
}
/// $02E438: jsr $2a7d8(pc)
pub fn i_02e438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E43C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E43C);
    cpu.pc = 0x2A7D8;
    16
}
/// $02E43C: clr.w $28(a6)
pub fn i_02e43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E440;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E440: jsr $2a478(pc)
pub fn i_02e440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E444;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E444);
    cpu.pc = 0x2A478;
    16
}
/// $02E444: jsr $28d70.l
pub fn i_02e444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E44A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E44A);
    cpu.pc = 0x28D70;
    16
}
/// $02E44A: bcc.w $2e464
pub fn i_02e44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E44E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E464; return 10; }
    8
}
/// $02E44E: lea.l $29a14(pc), a0
pub fn i_02e44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E452;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29A14;
    16
}
/// $02E452: move.l a0, $48(a6)
pub fn i_02e452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E456;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E456: move.l #$29600, $60(a6)
pub fn i_02e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E45E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29600;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E45E: lea.l $2e9c8(pc), a1
pub fn i_02e45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E462;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02E462: move.l a1, (a6)
pub fn i_02e462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E464;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E464: jsr $2a8c0(pc)
pub fn i_02e464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E468;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E468);
    cpu.pc = 0x2A8C0;
    16
}
/// $02E468: bcc.w $2e472
pub fn i_02e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E46C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E472; return 10; }
    8
}
/// $02E46C: lea.l $2d02e(pc), a1
pub fn i_02e46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E470;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02E470: move.l a1, (a6)
pub fn i_02e470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E472;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E472: jsr $2a5cc(pc)
pub fn i_02e472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E476;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E476);
    cpu.pc = 0x2A5CC;
    16
}
/// $02E476: bcc.w $2e480
pub fn i_02e476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E47A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E480; return 10; }
    8
}
/// $02E47A: lea.l $2bbf2(pc), a1
pub fn i_02e47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E47E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02E47E: move.l a1, (a6)
pub fn i_02e47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E480: jsr $2a690(pc)
pub fn i_02e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E484;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E484);
    cpu.pc = 0x2A690;
    16
}
/// $02E484: lea.l $2a060(pc), a0
pub fn i_02e484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E488;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02E488: movea.l #$ffffffff, a1
pub fn i_02e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E48E;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02e48e  ($02E48E, 8 B, thunks/jsr_abs_thunks.c)
/// $02E48E: jsr $772.l
pub fn i_02e48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E494);
    cpu.pc = 0x772;
    16
}
/// $02E494: rts
pub fn i_02e494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E496;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PlayerMount_02e496  ($02E496, 228 B, slug/vehicle_02dd20.s)
/// $02E496: bset.b #$0, $6b(a6)
pub fn i_02e496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E49C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E49C: lea.l $3d944.l, a1
pub fn i_02e49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3D944;
    20
}
/// $02E4A2: jsr $4ae.l
pub fn i_02e4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E4A8);
    cpu.pc = 0x4AE;
    16
}
/// $02E4A8: jsr $5dd02.l
pub fn i_02e4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E4AE);
    cpu.pc = 0x5DD02;
    16
}
/// $02E4AE: move.l #$ffffffff, $60(a6)
pub fn i_02e4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4B6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E4B6: bclr.b #$5, $8c(a6)
pub fn i_02e4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E4BC: clr.w $28(a6)
pub fn i_02e4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4C0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E4C0: clr.w $2c(a6)
pub fn i_02e4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4C4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E4C4: jsr $2a958(pc)
pub fn i_02e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E4C8);
    cpu.pc = 0x2A958;
    16
}
/// $02E4C8: move.w $80(a6), d2
pub fn i_02e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4CC;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02E4CC: move.w d2, d0
pub fn i_02e4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4CE;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E4CE: asr.w #$1, d0
pub fn i_02e4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4D0;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02E4D0: neg.w d0
pub fn i_02e4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4D2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02E4D2: addi.w #$20, d0
pub fn i_02e4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4D6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E4D6: cmpi.w #$40, d0
pub fn i_02e4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4DA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02E4DA: bls.w $2e4e2
pub fn i_02e4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4DE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2E4E2; return 10; }
    8
}
/// $02E4DE: move.w #$40, d0
pub fn i_02e4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E4E2: movem.w d0, -(a7)
pub fn i_02e4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4E6;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02E4E6: jsr $295a6(pc)
pub fn i_02e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E4EA);
    cpu.pc = 0x295A6;
    16
}
/// $02E4EA: movea.l #$2b0d40, a0
pub fn i_02e4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4F0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0D40;
    cpu.a[0] = v1;
    4
}
/// $02E4F0: lsl.w #$2, d0
pub fn i_02e4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4F2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E4F2: movea.l (a0, d0.w), a0
pub fn i_02e4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4F6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E4F6: cmpa.l #$ffffffff, a0
pub fn i_02e4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4FC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E4FC: beq.w $2e506
pub fn i_02e4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E500;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E506; return 10; }
    8
}
/// $02E500: jsr $28cd4.l
pub fn i_02e500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E506;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E506);
    cpu.pc = 0x28CD4;
    16
}
/// $02E506: movem.w (a7)+, d0
pub fn i_02e506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E50A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02E50A: move.b #$82, $20(a6)
pub fn i_02e50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E510;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E510: lsr.w #$2, d0
pub fn i_02e510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E512;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E512: andi.w #$ff, d0
pub fn i_02e512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E516;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E516: add.b d0, $20(a6)
pub fn i_02e516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E51A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E51A: move.b #$ff, $8b(a6)
pub fn i_02e51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E520;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E520: lea.l $2999e(pc), a0
pub fn i_02e520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E524;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02E524: move.l a0, $48(a6)
pub fn i_02e524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E528;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E528: lea.l $2e52e(pc), a1
pub fn i_02e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E52C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E52E;
    16
}
/// $02E52C: move.l a1, (a6)
pub fn i_02e52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E52E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E52E: bset.b #$2, $8d(a6)
pub fn i_02e52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E534;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E534: jsr $2aa24(pc)
pub fn i_02e534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E538;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E538);
    cpu.pc = 0x2AA24;
    16
}
/// $02E538: jsr $2a478(pc)
pub fn i_02e538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E53C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E53C);
    cpu.pc = 0x2A478;
    16
}
/// $02E53C: jsr $2a760(pc)
pub fn i_02e53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E540;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E540);
    cpu.pc = 0x2A760;
    16
}
/// $02E540: jsr $28d70.l
pub fn i_02e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E546;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E546);
    cpu.pc = 0x28D70;
    16
}
/// $02E546: bcc.w $2e550
pub fn i_02e546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E54A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E550; return 10; }
    8
}
/// $02E54A: lea.l $2b7da(pc), a1
pub fn i_02e54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E54E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B7DA;
    16
}
/// $02E54E: move.l a1, (a6)
pub fn i_02e54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E550;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E550: jsr $2a8c0(pc)
pub fn i_02e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E554;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E554);
    cpu.pc = 0x2A8C0;
    16
}
/// $02E554: bcc.w $2e55e
pub fn i_02e554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E558;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E55E; return 10; }
    8
}
/// $02E558: lea.l $2cffa(pc), a1
pub fn i_02e558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E55C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CFFA;
    16
}
/// $02E55C: move.l a1, (a6)
pub fn i_02e55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E55E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E55E: jsr $2a5cc(pc)
pub fn i_02e55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E562;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E562);
    cpu.pc = 0x2A5CC;
    16
}
/// $02E562: bcc.w $2e56c
pub fn i_02e562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E566;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E56C; return 10; }
    8
}
/// $02E566: lea.l $2fc70(pc), a1
pub fn i_02e566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E56A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC70;
    16
}
/// $02E56A: move.l a1, (a6)
pub fn i_02e56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E56C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E56C: jsr $2a690(pc)
pub fn i_02e56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E570;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E570);
    cpu.pc = 0x2A690;
    16
}
/// $02E570: lea.l $2a060(pc), a0
pub fn i_02e570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E574;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02E574: movea.l #$ffffffff, a1
pub fn i_02e574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E57A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02e57a  ($02E57A, 8 B, thunks/jsr_abs_thunks.c)
/// $02E57A: jsr $772.l
pub fn i_02e57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E580;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E580);
    cpu.pc = 0x772;
    16
}
/// $02E580: rts
pub fn i_02e580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E582;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Drive_02e6c2  ($02E6C2, 304 B, slug/vehicle_02dd20.s)
/// $02E6C2: bclr.b #$5, $8c(a6)
pub fn i_02e6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6C8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E6C8: bset.b #$1, $8c(a6)
pub fn i_02e6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6CE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E6CE: jsr $2a9a0(pc)
pub fn i_02e6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E6D2);
    cpu.pc = 0x2A9A0;
    16
}
/// $02E6D2: movea.l #$27940c, a0
pub fn i_02e6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6D8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27940C;
    cpu.a[0] = v1;
    4
}
/// $02E6D8: lsl.w #$2, d0
pub fn i_02e6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6DA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E6DA: movea.l (a0, d0.w), a0
pub fn i_02e6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6DE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E6DE: cmpa.l #$ffffffff, a0
pub fn i_02e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6E4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E6E4: beq.w $2e6ee
pub fn i_02e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E6EE; return 10; }
    8
}
/// $02E6E8: jsr $28cd4.l
pub fn i_02e6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E6EE);
    cpu.pc = 0x28CD4;
    16
}
/// $02E6EE: lea.l $2a024(pc), a1
pub fn i_02e6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A024;
    16
}
/// $02E6F2: movea.l (a1, d0.w), a0
pub fn i_02e6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6F6;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E6F6: move.l a0, $48(a6)
pub fn i_02e6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6FA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E6FA: move.b #$3, $8b(a6)
pub fn i_02e6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E700;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E700: move.b #$23, $20(a6)
pub fn i_02e700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E706;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E706: andi.w #$ff, d0
pub fn i_02e706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E70A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E70A: lsr.w #$2, d0
pub fn i_02e70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E70C;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E70C: add.b d0, $20(a6)
pub fn i_02e70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E710;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E710: move.l #$29600, $60(a6)
pub fn i_02e710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E718;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29600;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E718: clr.w $2c(a6)
pub fn i_02e718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E71C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E71C: lea.l $2e722(pc), a1
pub fn i_02e71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E720;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E722;
    16
}
/// $02E720: move.l a1, (a6)
pub fn i_02e720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E722;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E722: jsr $2aa24(pc)
pub fn i_02e722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E726;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E726);
    cpu.pc = 0x2AA24;
    16
}
/// $02E726: bset.b #$4, $8d(a6)
pub fn i_02e726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E72C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E72C: clr.w $28(a6)
pub fn i_02e72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E730;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E730: jsr $2abc0(pc)
pub fn i_02e730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E734;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E734);
    cpu.pc = 0x2ABC0;
    16
}
/// $02E734: bcs.w $2e754
pub fn i_02e734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E738;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E754; return 10; }
    8
}
/// $02E738: jsr $2a328(pc)
pub fn i_02e738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E73C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E73C);
    cpu.pc = 0x2A328;
    16
}
/// $02E73C: bcc.w $2e746
pub fn i_02e73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E740;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E746; return 10; }
    8
}
/// $02E740: move.w #$2a0, $28(a6)
pub fn i_02e740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E746;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E746: jsr $2a34e(pc)
pub fn i_02e746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E74A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E74A);
    cpu.pc = 0x2A34E;
    16
}
/// $02E74A: bcc.w $2e754
pub fn i_02e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E74E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E754; return 10; }
    8
}
/// $02E74E: move.w #$fd60, $28(a6)
pub fn i_02e74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E754;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E754: jsr $28992.l
pub fn i_02e754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E75A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E75A);
    cpu.pc = 0x28992;
    16
}
/// $02E75A: jsr $2a824(pc)
pub fn i_02e75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E75E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E75E);
    cpu.pc = 0x2A824;
    16
}
/// $02E75E: bcc.w $2e768
pub fn i_02e75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E762;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E768; return 10; }
    8
}
/// $02E762: lea.l $2ecc2(pc), a1
pub fn i_02e762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECC2;
    16
}
/// $02E766: move.l a1, (a6)
pub fn i_02e766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E768: clr.w $28(a6)
pub fn i_02e768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E76C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E76C: jsr $2a478(pc)
pub fn i_02e76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E770;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E770);
    cpu.pc = 0x2A478;
    16
}
/// $02E770: jsr $28d70.l
pub fn i_02e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E776);
    cpu.pc = 0x28D70;
    16
}
/// $02E776: bcc.w $2e79c
pub fn i_02e776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E77A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E79C; return 10; }
    8
}
/// $02E77A: lea.l $2ebb0(pc), a1
pub fn i_02e77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E77E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EBB0;
    16
}
/// $02E77E: move.l a1, (a6)
pub fn i_02e77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E780;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E780: jsr $2a328(pc)
pub fn i_02e780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E784;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E784);
    cpu.pc = 0x2A328;
    16
}
/// $02E784: bcc.w $2e78e
pub fn i_02e784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E788;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E78E; return 10; }
    8
}
/// $02E788: lea.l $2f422(pc), a1
pub fn i_02e788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E78C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F422;
    16
}
/// $02E78C: move.l a1, (a6)
pub fn i_02e78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E78E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E78E: jsr $2a34e(pc)
pub fn i_02e78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E792;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E792);
    cpu.pc = 0x2A34E;
    16
}
/// $02E792: bcc.w $2e79c
pub fn i_02e792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E796;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E79C; return 10; }
    8
}
/// $02E796: lea.l $2f4fc(pc), a1
pub fn i_02e796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E79A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F4FC;
    16
}
/// $02E79A: move.l a1, (a6)
pub fn i_02e79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E79C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E79C: jsr $5cef8.l
pub fn i_02e79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E7A2);
    cpu.pc = 0x5CEF8;
    16
}
/// $02E7A2: bcs.w $2e7ac
pub fn i_02e7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7A6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E7AC; return 10; }
    8
}
/// $02E7A6: lea.l $2eb14(pc), a1
pub fn i_02e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EB14;
    16
}
/// $02E7AA: move.l a1, (a6)
pub fn i_02e7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E7AC: jsr $2a8c0(pc)
pub fn i_02e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E7B0);
    cpu.pc = 0x2A8C0;
    16
}
/// $02E7B0: bcc.w $2e7ba
pub fn i_02e7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E7BA; return 10; }
    8
}
/// $02E7B4: lea.l $2d02e(pc), a1
pub fn i_02e7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02E7B8: move.l a1, (a6)
pub fn i_02e7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E7BA: jsr $2aaf0(pc)
pub fn i_02e7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E7BE);
    cpu.pc = 0x2AAF0;
    16
}
/// $02E7BE: bcc.w $2e7c8
pub fn i_02e7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E7C8; return 10; }
    8
}
/// $02E7C2: lea.l $2f614(pc), a1
pub fn i_02e7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F614;
    16
}
/// $02E7C6: move.l a1, (a6)
pub fn i_02e7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E7C8: jsr $2ac80(pc)
pub fn i_02e7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E7CC);
    cpu.pc = 0x2AC80;
    16
}
/// $02E7CC: bcc.w $2e7d6
pub fn i_02e7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E7D6; return 10; }
    8
}
/// $02E7D0: lea.l $2dd74(pc), a1
pub fn i_02e7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02E7D4: move.l a1, (a6)
pub fn i_02e7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E7D6: jsr $2a5cc(pc)
pub fn i_02e7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E7DA);
    cpu.pc = 0x2A5CC;
    16
}
/// $02E7DA: bcc.w $2e7e4
pub fn i_02e7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E7E4; return 10; }
    8
}
/// $02E7DE: lea.l $2f0e6(pc), a1
pub fn i_02e7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02E7E2: move.l a1, (a6)
pub fn i_02e7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E7E4: jsr $2a690(pc)
pub fn i_02e7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E7E8);
    cpu.pc = 0x2A690;
    16
}
/// $02E7E8: lea.l $2a060(pc), a0
pub fn i_02e7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02E7EC: movea.l #$ffffffff, a1
pub fn i_02e7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7F2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02e7f2  ($02E7F2, 8 B, thunks/jsr_abs_thunks.c)
/// $02E7F2: jsr $772.l
pub fn i_02e7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E7F8);
    cpu.pc = 0x772;
    16
}
/// $02E7F8: rts
pub fn i_02e7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DriveB_02e7fa  ($02E7FA, 290 B, slug/vehicle_02dd20.s)
/// $02E7FA: bclr.b #$5, $8c(a6)
pub fn i_02e7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E800;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E800: bset.b #$7, $5b(a6)
pub fn i_02e800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E806;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E806: lea.l $29a68(pc), a0
pub fn i_02e806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E80A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29A68;
    16
}
/// $02E80A: move.l a0, $48(a6)
pub fn i_02e80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E80E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E80E: move.b #$3, $8b(a6)
pub fn i_02e80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E814;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E814: bset.b #$1, $8c(a6)
pub fn i_02e814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E81A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E81A: jsr $2a9a0(pc)
pub fn i_02e81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E81E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E81E);
    cpu.pc = 0x2A9A0;
    16
}
/// $02E81E: movea.l #$279420, a0
pub fn i_02e81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E824;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279420;
    cpu.a[0] = v1;
    4
}
/// $02E824: lsl.w #$2, d0
pub fn i_02e824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E826;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E826: movea.l (a0, d0.w), a0
pub fn i_02e826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E82A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E82A: cmpa.l #$ffffffff, a0
pub fn i_02e82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E830;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E830: beq.w $2e83a
pub fn i_02e830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E834;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E83A; return 10; }
    8
}
/// $02E834: jsr $28cd4.l
pub fn i_02e834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E83A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E83A);
    cpu.pc = 0x28CD4;
    16
}
/// $02E83A: move.b #$41, $20(a6)
pub fn i_02e83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E840;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E840: andi.w #$ff, d0
pub fn i_02e840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E844;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E844: lsr.w #$2, d0
pub fn i_02e844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E846;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E846: add.b d0, $20(a6)
pub fn i_02e846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E84A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E84A: move.l #$29600, $60(a6)
pub fn i_02e84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E852;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29600;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E852: clr.w $2c(a6)
pub fn i_02e852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E856;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E856: lea.l $2e85c(pc), a1
pub fn i_02e856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E85A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E85C;
    16
}
/// $02E85A: move.l a1, (a6)
pub fn i_02e85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E85C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E85C: jsr $2aa24(pc)
pub fn i_02e85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E860;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E860);
    cpu.pc = 0x2AA24;
    16
}
/// $02E860: clr.w $28(a6)
pub fn i_02e860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E864;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E864: bset.b #$4, $8d(a6)
pub fn i_02e864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E86A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E86A: jsr $2abc0(pc)
pub fn i_02e86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E86E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E86E);
    cpu.pc = 0x2ABC0;
    16
}
/// $02E86E: bcs.w $2e88e
pub fn i_02e86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E872;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E88E; return 10; }
    8
}
/// $02E872: jsr $2a328(pc)
pub fn i_02e872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E876;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E876);
    cpu.pc = 0x2A328;
    16
}
/// $02E876: bcc.w $2e880
pub fn i_02e876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E87A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E880; return 10; }
    8
}
/// $02E87A: move.w #$2a0, $28(a6)
pub fn i_02e87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E880;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E880: jsr $2a34e(pc)
pub fn i_02e880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E884;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E884);
    cpu.pc = 0x2A34E;
    16
}
/// $02E884: bcc.w $2e88e
pub fn i_02e884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E888;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E88E; return 10; }
    8
}
/// $02E888: move.w #$fd60, $28(a6)
pub fn i_02e888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E88E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02E88E: jsr $28992.l
pub fn i_02e88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E894);
    cpu.pc = 0x28992;
    16
}
/// $02E894: jsr $2a824(pc)
pub fn i_02e894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E898;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E898);
    cpu.pc = 0x2A824;
    16
}
/// $02E898: bcc.w $2e8a2
pub fn i_02e898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E89C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E8A2; return 10; }
    8
}
/// $02E89C: lea.l $2ecc2(pc), a1
pub fn i_02e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECC2;
    16
}
/// $02E8A0: move.l a1, (a6)
pub fn i_02e8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E8A2: clr.w $28(a6)
pub fn i_02e8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8A6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E8A6: jsr $2a478(pc)
pub fn i_02e8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E8AA);
    cpu.pc = 0x2A478;
    16
}
/// $02E8AA: jsr $28d70.l
pub fn i_02e8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E8B0);
    cpu.pc = 0x28D70;
    16
}
/// $02E8B0: bcc.w $2e8d6
pub fn i_02e8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E8D6; return 10; }
    8
}
/// $02E8B4: lea.l $2ebb0(pc), a1
pub fn i_02e8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EBB0;
    16
}
/// $02E8B8: move.l a1, (a6)
pub fn i_02e8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E8BA: jsr $2a328(pc)
pub fn i_02e8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E8BE);
    cpu.pc = 0x2A328;
    16
}
/// $02E8BE: bcc.w $2e8c8
pub fn i_02e8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E8C8; return 10; }
    8
}
/// $02E8C2: lea.l $2f422(pc), a1
pub fn i_02e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F422;
    16
}
/// $02E8C6: move.l a1, (a6)
pub fn i_02e8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E8C8: jsr $2a34e(pc)
pub fn i_02e8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E8CC);
    cpu.pc = 0x2A34E;
    16
}
/// $02E8CC: bcc.w $2e8d6
pub fn i_02e8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E8D6; return 10; }
    8
}
/// $02E8D0: lea.l $2f4fc(pc), a1
pub fn i_02e8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F4FC;
    16
}
/// $02E8D4: move.l a1, (a6)
pub fn i_02e8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E8D6: jsr $2a8c0(pc)
pub fn i_02e8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E8DA);
    cpu.pc = 0x2A8C0;
    16
}
/// $02E8DA: bcc.w $2e8e4
pub fn i_02e8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E8E4; return 10; }
    8
}
/// $02E8DE: lea.l $2d02e(pc), a1
pub fn i_02e8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02E8E2: move.l a1, (a6)
pub fn i_02e8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E8E4: jsr $2ab3c(pc)
pub fn i_02e8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E8E8);
    cpu.pc = 0x2AB3C;
    16
}
/// $02E8E8: bcc.w $2e8f2
pub fn i_02e8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E8F2; return 10; }
    8
}
/// $02E8EC: lea.l $2f614(pc), a1
pub fn i_02e8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F614;
    16
}
/// $02E8F0: move.l a1, (a6)
pub fn i_02e8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E8F2: jsr $2ac80(pc)
pub fn i_02e8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E8F6);
    cpu.pc = 0x2AC80;
    16
}
/// $02E8F6: bcc.w $2e900
pub fn i_02e8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E900; return 10; }
    8
}
/// $02E8FA: lea.l $2dd74(pc), a1
pub fn i_02e8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02E8FE: move.l a1, (a6)
pub fn i_02e8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E900;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E900: jsr $2a5cc(pc)
pub fn i_02e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E904;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E904);
    cpu.pc = 0x2A5CC;
    16
}
/// $02E904: bcc.w $2e90e
pub fn i_02e904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E908;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E90E; return 10; }
    8
}
/// $02E908: lea.l $2f0e6(pc), a1
pub fn i_02e908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E90C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02E90C: move.l a1, (a6)
pub fn i_02e90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E90E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E90E: jsr $2a690(pc)
pub fn i_02e90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E912;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E912);
    cpu.pc = 0x2A690;
    16
}
/// $02E912: lea.l $2a060(pc), a0
pub fn i_02e912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E916;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02E916: movea.l #$ffffffff, a1
pub fn i_02e916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E91C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02e91c  ($02E91C, 8 B, thunks/jsr_abs_thunks.c)
/// $02E91C: jsr $772.l
pub fn i_02e91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E922);
    cpu.pc = 0x772;
    16
}
/// $02E922: rts
pub fn i_02e922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E924;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TurnToDrive_02e924  ($02E924, 164 B, slug/vehicle_02dd20.s)
/// $02E924: bclr.b #$5, $8c(a6)
pub fn i_02e924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E92A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E92A: clr.b $8b(a6)
pub fn i_02e92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E92E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E92E: bclr.b #$1, $8c(a6)
pub fn i_02e92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E934;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E934: moveq #$0, d0
pub fn i_02e934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E936;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02E936: moveq #$0, d1
pub fn i_02e936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E938;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02E938: move.b $3b(a6), d0
pub fn i_02e938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E93C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02E93C: cmpi.l #$8, d0
pub fn i_02e93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E942;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Long, false);
    8
}
/// $02E942: bcc.w $2e6c2
pub fn i_02e942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E946;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E6C2; return 10; }
    8
}
/// $02E946: movem.l d0, -(a7)
pub fn i_02e946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E94A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $02E94A: jsr $2a9a0(pc)
pub fn i_02e94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E94E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E94E);
    cpu.pc = 0x2A9A0;
    16
}
/// $02E94E: lea.l $2794d4.l, a0
pub fn i_02e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E954;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2794D4;
    20
}
/// $02E954: move.w d0, d1
pub fn i_02e954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E956;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02E956: asl.w #$2, d0
pub fn i_02e956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E958;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E958: movea.l (a0, d0.w), a0
pub fn i_02e958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E95C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E95C: movem.l (a7)+, d0
pub fn i_02e95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E960;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $02E960: lsl.w #$2, d0
pub fn i_02e960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E962;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E962: movea.l (a0, d0.w), a0
pub fn i_02e962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E966;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E966: cmpa.l #$ffffffff, a0
pub fn i_02e966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E96C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E96C: beq.w $2e976
pub fn i_02e96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E970;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E976; return 10; }
    8
}
/// $02E970: jsr $28cd4.l
pub fn i_02e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E976);
    cpu.pc = 0x28CD4;
    16
}
/// $02E976: move.b #$46, $20(a6)
pub fn i_02e976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E97C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E97C: lsr.w #$2, d0
pub fn i_02e97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E97E;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E97E: andi.w #$ff, d0
pub fn i_02e97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E982;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E982: cmpi.b #$8, d0
pub fn i_02e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E986;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $02E986: bcs.w $2e996
pub fn i_02e986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E98A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E996; return 10; }
    8
}
/// $02E98A: nop
pub fn i_02e98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E98C;
    cpu.ir = 0x4E71;
    4
}
/// $02E98C: nop
pub fn i_02e98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E98E;
    cpu.ir = 0x4E71;
    4
}
/// $02E98E: cmpi.b #$8, d0
pub fn i_02e98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E992;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $02E992: nop
pub fn i_02e992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E994;
    cpu.ir = 0x4E71;
    4
}
/// $02E994: trap #$f
pub fn i_02e994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E996;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02E996: mulu.w #$5, d0
pub fn i_02e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E99A;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x5;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $02E99A: add.b d0, $20(a6)
pub fn i_02e99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E99E;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E99E: andi.l #$ff, d1
pub fn i_02e99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9A4;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $02E9A4: cmpi.b #$5, d1
pub fn i_02e9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9A8;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02E9A8: bcs.w $2e9b8
pub fn i_02e9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9AC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2E9B8; return 10; }
    8
}
/// $02E9AC: nop
pub fn i_02e9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9AE;
    cpu.ir = 0x4E71;
    4
}
/// $02E9AE: nop
pub fn i_02e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9B0;
    cpu.ir = 0x4E71;
    4
}
/// $02E9B0: cmpi.b #$5, d1
pub fn i_02e9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9B4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02E9B4: nop
pub fn i_02e9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9B6;
    cpu.ir = 0x4E71;
    4
}
/// $02E9B6: trap #$f
pub fn i_02e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9B8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02E9B8: add.b d1, $20(a6)
pub fn i_02e9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9BC;
    cpu.ir = 0xD32E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E9BC: lea.l $29a68(pc), a0
pub fn i_02e9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9C0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29A68;
    16
}
/// $02E9C0: move.l a0, $48(a6)
pub fn i_02e9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E9C4: bra.w $2e710
pub fn i_02e9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E710;
    10
}
// ===== Slug_DriveAlt_02e9c8  ($02E9C8, 324 B, slug/vehicle_02dd20.s)
/// $02E9C8: bset.b #$0, $6b(a6)
pub fn i_02e9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9CE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E9CE: bclr.b #$5, $8c(a6)
pub fn i_02e9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9D4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E9D4: bclr.b #$1, $8c(a6)
pub fn i_02e9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9DA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E9DA: jsr $2a9a0(pc)
pub fn i_02e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E9DE);
    cpu.pc = 0x2A9A0;
    16
}
/// $02E9DE: movea.l #$279344, a0
pub fn i_02e9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9E4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279344;
    cpu.a[0] = v1;
    4
}
/// $02E9E4: lsl.w #$2, d0
pub fn i_02e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9E6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E9E6: movea.l (a0, d0.w), a0
pub fn i_02e9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9EA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E9EA: cmpa.l #$ffffffff, a0
pub fn i_02e9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9F0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E9F0: beq.w $2e9fa
pub fn i_02e9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E9FA; return 10; }
    8
}
/// $02E9F4: jsr $28cd4.l
pub fn i_02e9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E9FA);
    cpu.pc = 0x28CD4;
    16
}
/// $02E9FA: lea.l $2a024(pc), a1
pub fn i_02e9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A024;
    16
}
/// $02E9FE: movea.l (a1, d0.w), a0
pub fn i_02e9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA02;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EA02: move.l a0, $48(a6)
pub fn i_02ea02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA06;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02EA06: move.b #$9, $8b(a6)
pub fn i_02ea06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EA0C: move.b #$28, $20(a6)
pub fn i_02ea0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EA12: andi.w #$ff, d0
pub fn i_02ea12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA16;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02EA16: lsr.w #$2, d0
pub fn i_02ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA18;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EA18: add.b d0, $20(a6)
pub fn i_02ea18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA1C;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EA1C: clr.w $2c(a6)
pub fn i_02ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA20;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EA20: lea.l $2ea26(pc), a1
pub fn i_02ea20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EA26;
    16
}
/// $02EA24: move.l a1, (a6)
pub fn i_02ea24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA26: jsr $2aa24(pc)
pub fn i_02ea26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA2A);
    cpu.pc = 0x2AA24;
    16
}
/// $02EA2A: clr.w $28(a6)
pub fn i_02ea2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA2E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EA2E: jsr $2abc0(pc)
pub fn i_02ea2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA32);
    cpu.pc = 0x2ABC0;
    16
}
/// $02EA32: bcs.w $2ea52
pub fn i_02ea32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA36;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EA52; return 10; }
    8
}
/// $02EA36: jsr $2a328(pc)
pub fn i_02ea36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA3A);
    cpu.pc = 0x2A328;
    16
}
/// $02EA3A: bcc.w $2ea44
pub fn i_02ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA44; return 10; }
    8
}
/// $02EA3E: move.w #$2a0, $28(a6)
pub fn i_02ea3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA44;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EA44: jsr $2a34e(pc)
pub fn i_02ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA48);
    cpu.pc = 0x2A34E;
    16
}
/// $02EA48: bcc.w $2ea52
pub fn i_02ea48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA52; return 10; }
    8
}
/// $02EA4C: move.w #$fd60, $28(a6)
pub fn i_02ea4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EA52: jsr $28992.l
pub fn i_02ea52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EA58);
    cpu.pc = 0x28992;
    16
}
/// $02EA58: jsr $2a7d8(pc)
pub fn i_02ea58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA5C);
    cpu.pc = 0x2A7D8;
    16
}
/// $02EA5C: bcc.w $2ea66
pub fn i_02ea5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA66; return 10; }
    8
}
/// $02EA60: lea.l $2b4d2(pc), a1
pub fn i_02ea60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02EA64: move.l a1, (a6)
pub fn i_02ea64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA66: clr.w $28(a6)
pub fn i_02ea66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA6A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EA6A: jsr $2a478(pc)
pub fn i_02ea6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA6E);
    cpu.pc = 0x2A478;
    16
}
/// $02EA6E: jsr $28d70.l
pub fn i_02ea6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EA74);
    cpu.pc = 0x28D70;
    16
}
/// $02EA74: bcc.w $2eaaa
pub fn i_02ea74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAAA; return 10; }
    8
}
/// $02EA78: lea.l $29a14(pc), a0
pub fn i_02ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA7C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29A14;
    16
}
/// $02EA7C: move.l a0, $48(a6)
pub fn i_02ea7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA80;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02EA80: move.l #$29600, $60(a6)
pub fn i_02ea80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA88;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29600;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02EA88: lea.l $2b38c(pc), a1
pub fn i_02ea88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B38C;
    16
}
/// $02EA8C: move.l a1, (a6)
pub fn i_02ea8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA8E: jsr $2a328(pc)
pub fn i_02ea8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA92);
    cpu.pc = 0x2A328;
    16
}
/// $02EA92: bcc.w $2ea9c
pub fn i_02ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA9C; return 10; }
    8
}
/// $02EA96: lea.l $2bf64(pc), a1
pub fn i_02ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02EA9A: move.l a1, (a6)
pub fn i_02ea9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA9C: jsr $2a34e(pc)
pub fn i_02ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAA0);
    cpu.pc = 0x2A34E;
    16
}
/// $02EAA0: bcc.w $2eaaa
pub fn i_02eaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAA4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAAA; return 10; }
    8
}
/// $02EAA4: lea.l $2c07a(pc), a1
pub fn i_02eaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02EAA8: move.l a1, (a6)
pub fn i_02eaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAAA: jsr $2a59a(pc)
pub fn i_02eaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAAE);
    cpu.pc = 0x2A59A;
    16
}
/// $02EAAE: bcc.w $2eab8
pub fn i_02eaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAB8; return 10; }
    8
}
/// $02EAB2: lea.l $2e924(pc), a1
pub fn i_02eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E924;
    16
}
/// $02EAB6: move.l a1, (a6)
pub fn i_02eab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAB8: jsr $2a8c0(pc)
pub fn i_02eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EABC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EABC);
    cpu.pc = 0x2A8C0;
    16
}
/// $02EABC: bcc.w $2eac6
pub fn i_02eabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAC6; return 10; }
    8
}
/// $02EAC0: lea.l $2d02e(pc), a1
pub fn i_02eac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02EAC4: move.l a1, (a6)
pub fn i_02eac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAC6: jsr $2aaf0(pc)
pub fn i_02eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EACA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EACA);
    cpu.pc = 0x2AAF0;
    16
}
/// $02EACA: bcc.w $2ead4
pub fn i_02eaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EACE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAD4; return 10; }
    8
}
/// $02EACE: lea.l $2f614(pc), a1
pub fn i_02eace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F614;
    16
}
/// $02EAD2: move.l a1, (a6)
pub fn i_02ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAD4: jsr $2aac0(pc)
pub fn i_02ead4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAD8);
    cpu.pc = 0x2AAC0;
    16
}
/// $02EAD8: bcc.w $2eae2
pub fn i_02ead8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EADC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAE2; return 10; }
    8
}
/// $02EADC: lea.l $2c24a(pc), a1
pub fn i_02eadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02EAE0: move.l a1, (a6)
pub fn i_02eae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAE2: jsr $2ac80(pc)
pub fn i_02eae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAE6);
    cpu.pc = 0x2AC80;
    16
}
/// $02EAE6: bcc.w $2eaf0
pub fn i_02eae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAF0; return 10; }
    8
}
/// $02EAEA: lea.l $2dd74(pc), a1
pub fn i_02eaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02EAEE: move.l a1, (a6)
pub fn i_02eaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAF0: jsr $2a5cc(pc)
pub fn i_02eaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAF4);
    cpu.pc = 0x2A5CC;
    16
}
/// $02EAF4: bcc.w $2eafe
pub fn i_02eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAFE; return 10; }
    8
}
/// $02EAF8: lea.l $2bbf2(pc), a1
pub fn i_02eaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02EAFC: move.l a1, (a6)
pub fn i_02eafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAFE: jsr $2a690(pc)
pub fn i_02eafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EB02);
    cpu.pc = 0x2A690;
    16
}
/// $02EB02: lea.l $2a060(pc), a0
pub fn i_02eb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB06;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02EB06: movea.l #$ffffffff, a1
pub fn i_02eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB0C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02eb0c  ($02EB0C, 8 B, thunks/jsr_abs_thunks.c)
/// $02EB0C: jsr $772.l
pub fn i_02eb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EB12);
    cpu.pc = 0x772;
    16
}
/// $02EB12: rts
pub fn i_02eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TurnToDriveAlt_02eb14  ($02EB14, 156 B, slug/vehicle_02dd20.s)
/// $02EB14: bclr.b #$5, $8c(a6)
pub fn i_02eb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB1A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02EB1A: bclr.b #$1, $8c(a6)
pub fn i_02eb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB20;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02EB20: clr.b $8b(a6)
pub fn i_02eb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB24;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EB24: moveq #$0, d0
pub fn i_02eb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB26;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02EB26: moveq #$0, d1
pub fn i_02eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB28;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02EB28: move.b $3b(a6), d0
pub fn i_02eb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB2C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02EB2C: cmpi.l #$8, d0
pub fn i_02eb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB32;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Long, false);
    8
}
/// $02EB32: bcc.w $2e9c8
pub fn i_02eb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E9C8; return 10; }
    8
}
/// $02EB36: movem.l d0, -(a7)
pub fn i_02eb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB3A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $02EB3A: jsr $2a9a0(pc)
pub fn i_02eb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EB3E);
    cpu.pc = 0x2A9A0;
    16
}
/// $02EB3E: lea.l $2794c0.l, a0
pub fn i_02eb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2794C0;
    20
}
/// $02EB44: move.w d0, d1
pub fn i_02eb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB46;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EB46: asl.w #$2, d0
pub fn i_02eb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB48;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EB48: movea.l (a0, d0.w), a0
pub fn i_02eb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB4C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EB4C: movem.l (a7)+, d0
pub fn i_02eb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB50;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $02EB50: lsl.w #$2, d0
pub fn i_02eb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB52;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EB52: movea.l (a0, d0.w), a0
pub fn i_02eb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB56;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EB56: cmpa.l #$ffffffff, a0
pub fn i_02eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB5C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02EB5C: beq.w $2eb66
pub fn i_02eb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EB66; return 10; }
    8
}
/// $02EB60: jsr $28cd4.l
pub fn i_02eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EB66);
    cpu.pc = 0x28CD4;
    16
}
/// $02EB66: move.b #$6e, $20(a6)
pub fn i_02eb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB6C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EB6C: lsr.w #$2, d0
pub fn i_02eb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB6E;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EB6E: andi.w #$ff, d0
pub fn i_02eb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB72;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02EB72: cmpi.b #$8, d0
pub fn i_02eb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB76;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $02EB76: bcs.w $2eb86
pub fn i_02eb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB7A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EB86; return 10; }
    8
}
/// $02EB7A: nop
pub fn i_02eb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB7C;
    cpu.ir = 0x4E71;
    4
}
/// $02EB7C: nop
pub fn i_02eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB7E;
    cpu.ir = 0x4E71;
    4
}
/// $02EB7E: cmpi.b #$8, d0
pub fn i_02eb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB82;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $02EB82: nop
pub fn i_02eb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB84;
    cpu.ir = 0x4E71;
    4
}
/// $02EB84: trap #$f
pub fn i_02eb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB86;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02EB86: mulu.w #$5, d0
pub fn i_02eb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB8A;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x5;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $02EB8A: add.b d0, $20(a6)
pub fn i_02eb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB8E;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EB8E: andi.l #$ff, d1
pub fn i_02eb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB94;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $02EB94: cmpi.b #$5, d1
pub fn i_02eb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB98;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02EB98: bcs.w $2eba8
pub fn i_02eb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB9C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EBA8; return 10; }
    8
}
/// $02EB9C: nop
pub fn i_02eb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB9E;
    cpu.ir = 0x4E71;
    4
}
/// $02EB9E: nop
pub fn i_02eb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBA0;
    cpu.ir = 0x4E71;
    4
}
/// $02EBA0: cmpi.b #$5, d1
pub fn i_02eba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBA4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02EBA4: nop
pub fn i_02eba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBA6;
    cpu.ir = 0x4E71;
    4
}
/// $02EBA6: trap #$f
pub fn i_02eba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBA8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02EBA8: add.b d1, $20(a6)
pub fn i_02eba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBAC;
    cpu.ir = 0xD32E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EBAC: bra.w $2ea1c
pub fn i_02ebac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2EA1C;
    10
}
// ===== Slug_Brake_02ebb0  ($02EBB0, 238 B, slug/vehicle_02dd20.s)
/// $02EBB0: jsr $267e6.l
pub fn i_02ebb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EBB6);
    cpu.pc = 0x267E6;
    16
}
/// $02EBB6: clr.w $28(a6)
pub fn i_02ebb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBBA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EBBA: clr.w $2c(a6)
pub fn i_02ebba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBBE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EBBE: jsr $2a9a0(pc)
pub fn i_02ebbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EBC2);
    cpu.pc = 0x2A9A0;
    16
}
/// $02EBC2: movea.l #$2793e4, a0
pub fn i_02ebc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBC8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2793E4;
    cpu.a[0] = v1;
    4
}
/// $02EBC8: lsl.w #$2, d0
pub fn i_02ebc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBCA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EBCA: movea.l (a0, d0.w), a0
pub fn i_02ebca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBCE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EBCE: cmpa.l #$ffffffff, a0
pub fn i_02ebce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBD4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02EBD4: beq.w $2ebde
pub fn i_02ebd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBD8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EBDE; return 10; }
    8
}
/// $02EBD8: jsr $28cd4.l
pub fn i_02ebd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EBDE);
    cpu.pc = 0x28CD4;
    16
}
/// $02EBDE: move.w #$0, d0
pub fn i_02ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EBE2: move.b #$96, $20(a6)
pub fn i_02ebe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBE8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EBE8: andi.w #$ff, d0
pub fn i_02ebe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBEC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02EBEC: lsr.w #$2, d0
pub fn i_02ebec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBEE;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EBEE: add.b d0, $20(a6)
pub fn i_02ebee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBF2;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EBF2: move.l #$29600, $60(a6)
pub fn i_02ebf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBFA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29600;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02EBFA: lea.l $2ec00(pc), a1
pub fn i_02ebfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EC00;
    16
}
/// $02EBFE: move.l a1, (a6)
pub fn i_02ebfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC00: jsr $2aa24(pc)
pub fn i_02ec00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC04);
    cpu.pc = 0x2AA24;
    16
}
/// $02EC04: bset.b #$4, $8d(a6)
pub fn i_02ec04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC0A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02EC0A: jsr $28992.l
pub fn i_02ec0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EC10);
    cpu.pc = 0x28992;
    16
}
/// $02EC10: jsr $2a824(pc)
pub fn i_02ec10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC14);
    cpu.pc = 0x2A824;
    16
}
/// $02EC14: bcc.w $2ec1e
pub fn i_02ec14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EC1E; return 10; }
    8
}
/// $02EC18: lea.l $2ecc2(pc), a1
pub fn i_02ec18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECC2;
    16
}
/// $02EC1C: move.l a1, (a6)
pub fn i_02ec1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC1E: jsr $2a4f0(pc)
pub fn i_02ec1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC22);
    cpu.pc = 0x2A4F0;
    16
}
/// $02EC22: bcc.w $2ec2c
pub fn i_02ec22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EC2C; return 10; }
    8
}
/// $02EC26: lea.l $2ecc2(pc), a1
pub fn i_02ec26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECC2;
    16
}
/// $02EC2A: move.l a1, (a6)
pub fn i_02ec2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC2C: jsr $2abc0(pc)
pub fn i_02ec2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC30);
    cpu.pc = 0x2ABC0;
    16
}
/// $02EC30: bcs.w $2ec50
pub fn i_02ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC34;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EC50; return 10; }
    8
}
/// $02EC34: jsr $2a328(pc)
pub fn i_02ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC38);
    cpu.pc = 0x2A328;
    16
}
/// $02EC38: bcc.w $2ec42
pub fn i_02ec38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EC42; return 10; }
    8
}
/// $02EC3C: lea.l $2ee7e(pc), a1
pub fn i_02ec3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EE7E;
    16
}
/// $02EC40: move.l a1, (a6)
pub fn i_02ec40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC42: jsr $2a34e(pc)
pub fn i_02ec42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC46);
    cpu.pc = 0x2A34E;
    16
}
/// $02EC46: bcc.w $2ec50
pub fn i_02ec46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EC50; return 10; }
    8
}
/// $02EC4A: lea.l $2efb0(pc), a1
pub fn i_02ec4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EFB0;
    16
}
/// $02EC4E: move.l a1, (a6)
pub fn i_02ec4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC50: jsr $28d70.l
pub fn i_02ec50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EC56);
    cpu.pc = 0x28D70;
    16
}
/// $02EC56: jsr $5cef8.l
pub fn i_02ec56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EC5C);
    cpu.pc = 0x5CEF8;
    16
}
/// $02EC5C: bcs.w $2ec66
pub fn i_02ec5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC60;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EC66; return 10; }
    8
}
/// $02EC60: lea.l $2e9c8(pc), a1
pub fn i_02ec60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02EC64: move.l a1, (a6)
pub fn i_02ec64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC66: jsr $2a8c0(pc)
pub fn i_02ec66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC6A);
    cpu.pc = 0x2A8C0;
    16
}
/// $02EC6A: bcc.w $2ec74
pub fn i_02ec6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EC74; return 10; }
    8
}
/// $02EC6E: lea.l $2d02e(pc), a1
pub fn i_02ec6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02EC72: move.l a1, (a6)
pub fn i_02ec72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC74: jsr $2ac80(pc)
pub fn i_02ec74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC78);
    cpu.pc = 0x2AC80;
    16
}
/// $02EC78: bcc.w $2ec82
pub fn i_02ec78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC7C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EC82; return 10; }
    8
}
/// $02EC7C: lea.l $2dd74(pc), a1
pub fn i_02ec7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02EC80: move.l a1, (a6)
pub fn i_02ec80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC82: jsr $2a5cc(pc)
pub fn i_02ec82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC86);
    cpu.pc = 0x2A5CC;
    16
}
/// $02EC86: bcc.w $2ec90
pub fn i_02ec86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EC90; return 10; }
    8
}
/// $02EC8A: lea.l $2f0e6(pc), a1
pub fn i_02ec8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02EC8E: move.l a1, (a6)
pub fn i_02ec8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EC90: jsr $2a690(pc)
pub fn i_02ec90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EC94);
    cpu.pc = 0x2A690;
    16
}
/// $02EC94: lea.l $2a060(pc), a0
pub fn i_02ec94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC98;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02EC98: movea.l #$ffffffff, a1
pub fn i_02ec98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC9E;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02ec9e  ($02EC9E, 8 B, thunks/jsr_abs_thunks.c)
/// $02EC9E: jsr $772.l
pub fn i_02ec9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ECA4);
    cpu.pc = 0x772;
    16
}
/// $02ECA4: rts
pub fn i_02eca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Stall_02eca6  ($02ECA6, 458 B, slug/vehicle_02dd20.s)
/// $02ECA6: move.w #$10af, d0
pub fn i_02eca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECAA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10AF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02ECAA: jsr $2352.l
pub fn i_02ecaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ECB0);
    cpu.pc = 0x2352;
    16
}
/// $02ECB0: bra.w $2ecc2
pub fn i_02ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ECC2;
    10
}
/// $02ECC2: clr.w $2c(a6)
pub fn i_02ecc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECC6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02ECC6: jsr $2a958(pc)
pub fn i_02ecc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ECCA);
    cpu.pc = 0x2A958;
    16
}
/// $02ECCA: move.w d2, $80(a6)
pub fn i_02ecca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECCE;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ECCE: move.w d2, d0
pub fn i_02ecce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECD0;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02ECD0: asr.w #$1, d0
pub fn i_02ecd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECD2;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02ECD2: neg.w d0
pub fn i_02ecd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECD4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02ECD4: addi.w #$20, d0
pub fn i_02ecd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECD8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02ECD8: cmpi.w #$40, d0
pub fn i_02ecd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECDC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02ECDC: bls.w $2ece4
pub fn i_02ecdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECE0;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2ECE4; return 10; }
    8
}
/// $02ECE0: move.w #$40, d0
pub fn i_02ece0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02ECE4: movem.w d0, -(a7)
pub fn i_02ece4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECE8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02ECE8: jsr $295a6(pc)
pub fn i_02ece8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ECEC);
    cpu.pc = 0x295A6;
    16
}
/// $02ECEC: movea.l #$2b0c74, a0
pub fn i_02ecec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECF2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C74;
    cpu.a[0] = v1;
    4
}
/// $02ECF2: lsl.w #$2, d0
pub fn i_02ecf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECF4;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02ECF4: movea.l (a0, d0.w), a0
pub fn i_02ecf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECF8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02ECF8: cmpa.l #$ffffffff, a0
pub fn i_02ecf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECFE;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02ECFE: beq.w $2ed08
pub fn i_02ecfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2ED08; return 10; }
    8
}
/// $02ED02: jsr $28cd4.l
pub fn i_02ed02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ED08);
    cpu.pc = 0x28CD4;
    16
}
/// $02ED08: movem.w (a7)+, d0
pub fn i_02ed08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED0C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02ED0C: move.w #$0, d0
pub fn i_02ed0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED10;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02ED10: move.b #$c3, $20(a6)
pub fn i_02ed10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02ED16: lea.l $2ed1c(pc), a1
pub fn i_02ed16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ED1C;
    16
}
/// $02ED1A: move.l a1, (a6)
pub fn i_02ed1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02ED1C: bset.b #$2, $8d(a6)
pub fn i_02ed1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED22;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02ED22: jsr $2aa24(pc)
pub fn i_02ed22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED26);
    cpu.pc = 0x2AA24;
    16
}
/// $02ED26: clr.w $28(a6)
pub fn i_02ed26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED2A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02ED2A: bset.b #$4, $8d(a6)
pub fn i_02ed2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED30;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02ED30: jsr $2abc0(pc)
pub fn i_02ed30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED34);
    cpu.pc = 0x2ABC0;
    16
}
/// $02ED34: bcs.w $2ed58
pub fn i_02ed34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED38;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2ED58; return 10; }
    8
}
/// $02ED38: jsr $2a328(pc)
pub fn i_02ed38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED3C);
    cpu.pc = 0x2A328;
    16
}
/// $02ED3C: bcc.w $2ed4a
pub fn i_02ed3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2ED4A; return 10; }
    8
}
/// $02ED40: move.w #$2a0, $28(a6)
pub fn i_02ed40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED46;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ED46: bra.w $2ed58
pub fn i_02ed46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ED58;
    10
}
/// $02ED4A: jsr $2a34e(pc)
pub fn i_02ed4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED4E);
    cpu.pc = 0x2A34E;
    16
}
/// $02ED4E: bcc.w $2ed58
pub fn i_02ed4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED52;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2ED58; return 10; }
    8
}
/// $02ED52: move.w #$fd60, $28(a6)
pub fn i_02ed52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ED58: cmpi.b #$3, $106f2a.l
pub fn i_02ed58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED60;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02ED60: bne.w $2ed82
pub fn i_02ed60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ED82; return 10; }
    8
}
/// $02ED64: move.w $28(a6), d7
pub fn i_02ed64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED68;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02ED68: jsr $2a958(pc)
pub fn i_02ed68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED6C);
    cpu.pc = 0x2A958;
    16
}
/// $02ED6C: sub.w $80(a6), d2
pub fn i_02ed6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED70;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02ED70: eor.w d7, d2
pub fn i_02ed70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED72;
    cpu.ir = 0xBF42;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[7]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02ED72: asl.w #$1, d2
pub fn i_02ed72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED74;
    cpu.ir = 0xE342;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02ED74: bcs.w $2ed7e
pub fn i_02ed74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED78;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2ED7E; return 10; }
    8
}
/// $02ED78: move.w d7, d2
pub fn i_02ed78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED7A;
    cpu.ir = 0x3407;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02ED7A: asr.w #$1, d2
pub fn i_02ed7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED7C;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02ED7C: add.w d2, d7
pub fn i_02ed7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED7E;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02ED7E: move.w d7, $28(a6)
pub fn i_02ed7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED82;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ED82: jsr $2a478(pc)
pub fn i_02ed82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED86);
    cpu.pc = 0x2A478;
    16
}
/// $02ED86: jsr $28992.l
pub fn i_02ed86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ED8C);
    cpu.pc = 0x28992;
    16
}
/// $02ED8C: jsr $2a766(pc)
pub fn i_02ed8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED90);
    cpu.pc = 0x2A766;
    16
}
/// $02ED90: jsr $2a958(pc)
pub fn i_02ed90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ED94);
    cpu.pc = 0x2A958;
    16
}
/// $02ED94: cmp.w $80(a6), d2
pub fn i_02ed94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED98;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02ED98: beq.w $2ede8
pub fn i_02ed98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EDE8; return 10; }
    8
}
/// $02ED9C: move.w d2, $80(a6)
pub fn i_02ed9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDA0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EDA0: move.w d2, d0
pub fn i_02eda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDA2;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EDA2: asr.w #$1, d0
pub fn i_02eda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDA4;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02EDA4: neg.w d0
pub fn i_02eda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDA6;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02EDA6: addi.w #$20, d0
pub fn i_02eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDAA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02EDAA: cmpi.w #$40, d0
pub fn i_02edaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDAE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02EDAE: bls.w $2edb6
pub fn i_02edae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDB2;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2EDB6; return 10; }
    8
}
/// $02EDB2: move.w #$40, d0
pub fn i_02edb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDB6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EDB6: movem.w d0, -(a7)
pub fn i_02edb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDBA;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02EDBA: jsr $295a6(pc)
pub fn i_02edba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EDBE);
    cpu.pc = 0x295A6;
    16
}
/// $02EDBE: movea.l #$2b0c74, a0
pub fn i_02edbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDC4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C74;
    cpu.a[0] = v1;
    4
}
/// $02EDC4: lsl.w #$2, d0
pub fn i_02edc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDC6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EDC6: movea.l (a0, d0.w), a0
pub fn i_02edc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDCA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EDCA: cmpa.l #$ffffffff, a0
pub fn i_02edca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDD0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02EDD0: beq.w $2edda
pub fn i_02edd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EDDA; return 10; }
    8
}
/// $02EDD4: jsr $28cd4.l
pub fn i_02edd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EDDA);
    cpu.pc = 0x28CD4;
    16
}
/// $02EDDA: movem.w (a7)+, d0
pub fn i_02edda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDDE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02EDDE: move.w #$0, d0
pub fn i_02edde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EDE2: move.b #$c3, $20(a6)
pub fn i_02ede2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDE8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EDE8: jsr $28d70.l
pub fn i_02ede8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EDEE);
    cpu.pc = 0x28D70;
    16
}
/// $02EDEE: jsr $2a4ec(pc)
pub fn i_02edee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EDF2);
    cpu.pc = 0x2A4EC;
    16
}
/// $02EDF2: bcs.w $2ee1c
pub fn i_02edf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EE1C; return 10; }
    8
}
/// $02EDF6: bra.w $2edfa
pub fn i_02edf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2EDFA;
    10
}
/// $02EDFA: lea.l $2ebb0(pc), a1
pub fn i_02edfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EBB0;
    16
}
/// $02EDFE: move.l a1, (a6)
pub fn i_02edfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EE00: jsr $2a328(pc)
pub fn i_02ee00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE04);
    cpu.pc = 0x2A328;
    16
}
/// $02EE04: bcc.w $2ee0e
pub fn i_02ee04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EE0E; return 10; }
    8
}
/// $02EE08: lea.l $2f422(pc), a1
pub fn i_02ee08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F422;
    16
}
/// $02EE0C: move.l a1, (a6)
pub fn i_02ee0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EE0E: jsr $2a34e(pc)
pub fn i_02ee0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE12);
    cpu.pc = 0x2A34E;
    16
}
/// $02EE12: bcc.w $2ee1c
pub fn i_02ee12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EE1C; return 10; }
    8
}
/// $02EE16: lea.l $2f4fc(pc), a1
pub fn i_02ee16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F4FC;
    16
}
/// $02EE1A: move.l a1, (a6)
pub fn i_02ee1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EE1C: tst.b $3b(a6)
pub fn i_02ee1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE20;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02EE20: bne.w $2ee2a
pub fn i_02ee20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE24;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2EE2A; return 10; }
    8
}
/// $02EE24: jsr $283d8.l
pub fn i_02ee24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EE2A);
    cpu.pc = 0x283D8;
    16
}
/// $02EE2A: jsr $2a59a(pc)
pub fn i_02ee2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE2E);
    cpu.pc = 0x2A59A;
    16
}
/// $02EE2E: bcs.w $2ee38
pub fn i_02ee2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE32;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EE38; return 10; }
    8
}
/// $02EE32: lea.l $2b7da(pc), a1
pub fn i_02ee32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B7DA;
    16
}
/// $02EE36: move.l a1, (a6)
pub fn i_02ee36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EE38: jsr $2a8c0(pc)
pub fn i_02ee38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE3C);
    cpu.pc = 0x2A8C0;
    16
}
/// $02EE3C: bcc.w $2ee46
pub fn i_02ee3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EE46; return 10; }
    8
}
/// $02EE40: lea.l $2cffa(pc), a1
pub fn i_02ee40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CFFA;
    16
}
/// $02EE44: move.l a1, (a6)
pub fn i_02ee44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EE46: jsr $2ac80(pc)
pub fn i_02ee46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE4A);
    cpu.pc = 0x2AC80;
    16
}
/// $02EE4A: bcc.w $2ee54
pub fn i_02ee4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EE54; return 10; }
    8
}
/// $02EE4E: lea.l $2dd74(pc), a1
pub fn i_02ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02EE52: move.l a1, (a6)
pub fn i_02ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EE54: jsr $2a5cc(pc)
pub fn i_02ee54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE58);
    cpu.pc = 0x2A5CC;
    16
}
/// $02EE58: bcc.w $2ee62
pub fn i_02ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EE62; return 10; }
    8
}
/// $02EE5C: lea.l $2fc70(pc), a1
pub fn i_02ee5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC70;
    16
}
/// $02EE60: move.l a1, (a6)
pub fn i_02ee60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EE62: jsr $2a690(pc)
pub fn i_02ee62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE66);
    cpu.pc = 0x2A690;
    16
}
/// $02EE66: lea.l $2a060(pc), a0
pub fn i_02ee66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE6A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02EE6A: movea.l #$ffffffff, a1
pub fn i_02ee6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE70;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02ee70  ($02EE70, 8 B, thunks/jsr_abs_thunks.c)
/// $02EE70: jsr $772.l
pub fn i_02ee70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EE76);
    cpu.pc = 0x772;
    16
}
/// $02EE76: rts
pub fn i_02ee76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_AccelRight_02ee78  ($02EE78, 298 B, slug/vehicle_02dd20.s)
/// $02EE78: jsr $31e5e.l
pub fn i_02ee78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EE7E);
    cpu.pc = 0x31E5E;
    16
}
/// $02EE7E: move.w #$2a0, d0
pub fn i_02ee7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE82;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EE82: sub.w $28(a6), d0
pub fn i_02ee82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE86;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02EE86: asr.w #$3, d0
pub fn i_02ee86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE88;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02EE88: move.w d0, $2c(a6)
pub fn i_02ee88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE8C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EE8C: move.b #$8, $91(a6)
pub fn i_02ee8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EE92: jsr $2a9a0(pc)
pub fn i_02ee92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EE96);
    cpu.pc = 0x2A9A0;
    16
}
/// $02EE96: movea.l #$279470, a0
pub fn i_02ee96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE9C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279470;
    cpu.a[0] = v1;
    4
}
/// $02EE9C: lsl.w #$2, d0
pub fn i_02ee9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE9E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EE9E: movea.l (a0, d0.w), a0
pub fn i_02ee9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEA2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EEA2: cmpa.l #$ffffffff, a0
pub fn i_02eea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEA8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02EEA8: beq.w $2eeb2
pub fn i_02eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EEB2; return 10; }
    8
}
/// $02EEAC: jsr $28cd4.l
pub fn i_02eeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EEB2);
    cpu.pc = 0x28CD4;
    16
}
/// $02EEB2: lsr.w #$1, d0
pub fn i_02eeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEB4;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02EEB4: lea.l $2b8ce(pc), a0
pub fn i_02eeb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEB8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2B8CE;
    16
}
/// $02EEB8: move.w (a0, d0.w), d0
pub fn i_02eeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEBC;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02EEBC: cmpi.w #$ffff, d0
pub fn i_02eebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEC0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02EEC0: beq.w $2eeca
pub fn i_02eec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EECA; return 10; }
    8
}
/// $02EEC4: jsr $2352.l
pub fn i_02eec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EECA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EECA);
    cpu.pc = 0x2352;
    16
}
/// $02EECA: move.w #$0, d0
pub fn i_02eeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EECE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EECE: move.b #$96, $20(a6)
pub fn i_02eece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EED4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EED4: andi.w #$ff, d0
pub fn i_02eed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EED8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02EED8: lsr.w #$2, d0
pub fn i_02eed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEDA;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EEDA: add.b d0, $20(a6)
pub fn i_02eeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEDE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EEDE: lea.l $2eee4(pc), a1
pub fn i_02eede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EEE4;
    16
}
/// $02EEE2: move.l a1, (a6)
pub fn i_02eee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EEE4: subq.b #$1, $91(a6)
pub fn i_02eee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEE8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EEE8: bne.w $2eefc
pub fn i_02eee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2EEFC; return 10; }
    8
}
/// $02EEEC: move.w #$2a0, $28(a6)
pub fn i_02eeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EEF2: clr.w $2c(a6)
pub fn i_02eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEF6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EEF6: lea.l $2eefc(pc), a1
pub fn i_02eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EEFC;
    16
}
/// $02EEFA: move.l a1, (a6)
pub fn i_02eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EEFC: jsr $2aa24(pc)
pub fn i_02eefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF00);
    cpu.pc = 0x2AA24;
    16
}
/// $02EF00: bset.b #$4, $8d(a6)
pub fn i_02ef00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF06;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02EF06: jsr $28992.l
pub fn i_02ef06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EF0C);
    cpu.pc = 0x28992;
    16
}
/// $02EF0C: jsr $2a824(pc)
pub fn i_02ef0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF10);
    cpu.pc = 0x2A824;
    16
}
/// $02EF10: bcc.w $2ef1a
pub fn i_02ef10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EF1A; return 10; }
    8
}
/// $02EF14: lea.l $2eca6(pc), a1
pub fn i_02ef14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECA6;
    16
}
/// $02EF18: move.l a1, (a6)
pub fn i_02ef18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF1A: jsr $2a4f0(pc)
pub fn i_02ef1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF1E);
    cpu.pc = 0x2A4F0;
    16
}
/// $02EF1E: bcc.w $2ef28
pub fn i_02ef1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EF28; return 10; }
    8
}
/// $02EF22: lea.l $2eca6(pc), a1
pub fn i_02ef22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECA6;
    16
}
/// $02EF26: move.l a1, (a6)
pub fn i_02ef26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF28: jsr $5cef8.l
pub fn i_02ef28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EF2E);
    cpu.pc = 0x5CEF8;
    16
}
/// $02EF2E: bcs.w $2ef38
pub fn i_02ef2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF32;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EF38; return 10; }
    8
}
/// $02EF32: lea.l $2e9c8(pc), a1
pub fn i_02ef32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02EF36: move.l a1, (a6)
pub fn i_02ef36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF38: jsr $28d70.l
pub fn i_02ef38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EF3E);
    cpu.pc = 0x28D70;
    16
}
/// $02EF3E: bcc.w $2ef48
pub fn i_02ef3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EF48; return 10; }
    8
}
/// $02EF42: lea.l $2f422(pc), a1
pub fn i_02ef42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F422;
    16
}
/// $02EF46: move.l a1, (a6)
pub fn i_02ef46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF48: jsr $283d8.l
pub fn i_02ef48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EF4E);
    cpu.pc = 0x283D8;
    16
}
/// $02EF4E: jsr $2a328(pc)
pub fn i_02ef4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF52);
    cpu.pc = 0x2A328;
    16
}
/// $02EF52: bcs.w $2ef5c
pub fn i_02ef52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF56;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EF5C; return 10; }
    8
}
/// $02EF56: lea.l $2f202(pc), a1
pub fn i_02ef56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F202;
    16
}
/// $02EF5A: move.l a1, (a6)
pub fn i_02ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF5C: jsr $2a34e(pc)
pub fn i_02ef5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF60);
    cpu.pc = 0x2A34E;
    16
}
/// $02EF60: bcc.w $2ef6a
pub fn i_02ef60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF64;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EF6A; return 10; }
    8
}
/// $02EF64: lea.l $2f4fc(pc), a1
pub fn i_02ef64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F4FC;
    16
}
/// $02EF68: move.l a1, (a6)
pub fn i_02ef68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF6A: jsr $2a8c0(pc)
pub fn i_02ef6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF6E);
    cpu.pc = 0x2A8C0;
    16
}
/// $02EF6E: bcc.w $2ef78
pub fn i_02ef6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EF78; return 10; }
    8
}
/// $02EF72: lea.l $2d02e(pc), a1
pub fn i_02ef72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02EF76: move.l a1, (a6)
pub fn i_02ef76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF78: jsr $2ac80(pc)
pub fn i_02ef78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF7C);
    cpu.pc = 0x2AC80;
    16
}
/// $02EF7C: bcc.w $2ef86
pub fn i_02ef7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EF86; return 10; }
    8
}
/// $02EF80: lea.l $2dd74(pc), a1
pub fn i_02ef80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02EF84: move.l a1, (a6)
pub fn i_02ef84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF86: jsr $2a5cc(pc)
pub fn i_02ef86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF8A);
    cpu.pc = 0x2A5CC;
    16
}
/// $02EF8A: bcc.w $2ef94
pub fn i_02ef8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EF94; return 10; }
    8
}
/// $02EF8E: lea.l $2f0e6(pc), a1
pub fn i_02ef8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02EF92: move.l a1, (a6)
pub fn i_02ef92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EF94: jsr $2a690(pc)
pub fn i_02ef94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EF98);
    cpu.pc = 0x2A690;
    16
}
/// $02EF98: lea.l $2a060(pc), a0
pub fn i_02ef98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF9C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02EF9C: movea.l #$ffffffff, a1
pub fn i_02ef9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFA2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02efa2  ($02EFA2, 8 B, thunks/jsr_abs_thunks.c)
/// $02EFA2: jsr $772.l
pub fn i_02efa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EFA8);
    cpu.pc = 0x772;
    16
}
/// $02EFA8: rts
pub fn i_02efa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_AccelLeft_02efaa  ($02EFAA, 298 B, slug/vehicle_02dd20.s)
/// $02EFAA: jsr $31e76.l
pub fn i_02efaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EFB0);
    cpu.pc = 0x31E76;
    16
}
/// $02EFB0: move.w #$fd60, d0
pub fn i_02efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02EFB4: sub.w $28(a6), d0
pub fn i_02efb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFB8;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02EFB8: asr.w #$3, d0
pub fn i_02efb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFBA;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02EFBA: move.w d0, $2c(a6)
pub fn i_02efba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFBE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EFBE: move.b #$8, $91(a6)
pub fn i_02efbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EFC4: jsr $2a9a0(pc)
pub fn i_02efc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EFC8);
    cpu.pc = 0x2A9A0;
    16
}
/// $02EFC8: movea.l #$279484, a0
pub fn i_02efc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFCE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279484;
    cpu.a[0] = v1;
    4
}
/// $02EFCE: lsl.w #$2, d0
pub fn i_02efce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFD0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EFD0: movea.l (a0, d0.w), a0
pub fn i_02efd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFD4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EFD4: cmpa.l #$ffffffff, a0
pub fn i_02efd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFDA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02EFDA: beq.w $2efe4
pub fn i_02efda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EFE4; return 10; }
    8
}
/// $02EFDE: jsr $28cd4.l
pub fn i_02efde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EFE4);
    cpu.pc = 0x28CD4;
    16
}
/// $02EFE4: lsr.w #$1, d0
pub fn i_02efe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFE6;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02EFE6: lea.l $2ba34(pc), a0
pub fn i_02efe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFEA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2BA34;
    16
}
/// $02EFEA: move.w (a0, d0.w), d0
pub fn i_02efea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFEE;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02EFEE: cmpi.w #$ffff, d0
pub fn i_02efee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFF2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02EFF2: beq.w $2effc
pub fn i_02eff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EFFC; return 10; }
    8
}
/// $02EFF6: jsr $2352.l
pub fn i_02eff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EFFC);
    cpu.pc = 0x2352;
    16
}
/// $02EFFC: move.w #$0, d0
pub fn i_02effc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F000;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F000: move.b #$96, $20(a6)
pub fn i_02f000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F006;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F006: andi.w #$ff, d0
pub fn i_02f006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F00A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02F00A: lsr.w #$2, d0
pub fn i_02f00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F00C;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F00C: add.b d0, $20(a6)
pub fn i_02f00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F010;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F010: lea.l $2f016(pc), a1
pub fn i_02f010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F014;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F016;
    16
}
/// $02F014: move.l a1, (a6)
pub fn i_02f014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F016;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F016: subq.b #$1, $91(a6)
pub fn i_02f016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F01A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F01A: bne.w $2f02e
pub fn i_02f01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F01E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F02E; return 10; }
    8
}
/// $02F01E: move.w #$fd60, $28(a6)
pub fn i_02f01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F024;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F024: clr.w $2c(a6)
pub fn i_02f024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F028;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F028: lea.l $2f02e(pc), a1
pub fn i_02f028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F02C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F02E;
    16
}
/// $02F02C: move.l a1, (a6)
pub fn i_02f02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F02E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F02E: jsr $2aa24(pc)
pub fn i_02f02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F032;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F032);
    cpu.pc = 0x2AA24;
    16
}
/// $02F032: bset.b #$4, $8d(a6)
pub fn i_02f032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F038;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02F038: jsr $28992.l
pub fn i_02f038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F03E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F03E);
    cpu.pc = 0x28992;
    16
}
/// $02F03E: jsr $2a824(pc)
pub fn i_02f03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F042;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F042);
    cpu.pc = 0x2A824;
    16
}
/// $02F042: bcc.w $2f04c
pub fn i_02f042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F046;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F04C; return 10; }
    8
}
/// $02F046: lea.l $2ecb4(pc), a1
pub fn i_02f046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F04A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECB4;
    16
}
/// $02F04A: move.l a1, (a6)
pub fn i_02f04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F04C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F04C: jsr $2a4f0(pc)
pub fn i_02f04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F050;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F050);
    cpu.pc = 0x2A4F0;
    16
}
/// $02F050: bcc.w $2f05a
pub fn i_02f050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F054;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F05A; return 10; }
    8
}
/// $02F054: lea.l $2ecb4(pc), a1
pub fn i_02f054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F058;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECB4;
    16
}
/// $02F058: move.l a1, (a6)
pub fn i_02f058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F05A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F05A: jsr $5cef8.l
pub fn i_02f05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F060;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F060);
    cpu.pc = 0x5CEF8;
    16
}
/// $02F060: bcs.w $2f06a
pub fn i_02f060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F064;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F06A; return 10; }
    8
}
/// $02F064: lea.l $2e9c8(pc), a1
pub fn i_02f064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F068;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02F068: move.l a1, (a6)
pub fn i_02f068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F06A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F06A: jsr $28d70.l
pub fn i_02f06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F070;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F070);
    cpu.pc = 0x28D70;
    16
}
/// $02F070: bcc.w $2f07a
pub fn i_02f070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F074;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F07A; return 10; }
    8
}
/// $02F074: lea.l $2f4fc(pc), a1
pub fn i_02f074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F078;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F4FC;
    16
}
/// $02F078: move.l a1, (a6)
pub fn i_02f078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F07A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F07A: jsr $283d8.l
pub fn i_02f07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F080;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F080);
    cpu.pc = 0x283D8;
    16
}
/// $02F080: jsr $2a34e(pc)
pub fn i_02f080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F084;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F084);
    cpu.pc = 0x2A34E;
    16
}
/// $02F084: bcs.w $2f08e
pub fn i_02f084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F088;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F08E; return 10; }
    8
}
/// $02F088: lea.l $2f312(pc), a1
pub fn i_02f088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F08C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F312;
    16
}
/// $02F08C: move.l a1, (a6)
pub fn i_02f08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F08E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F08E: jsr $2a328(pc)
pub fn i_02f08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F092;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F092);
    cpu.pc = 0x2A328;
    16
}
/// $02F092: bcc.w $2f09c
pub fn i_02f092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F096;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F09C; return 10; }
    8
}
/// $02F096: lea.l $2f422(pc), a1
pub fn i_02f096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F09A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F422;
    16
}
/// $02F09A: move.l a1, (a6)
pub fn i_02f09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F09C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F09C: jsr $2a8c0(pc)
pub fn i_02f09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F0A0);
    cpu.pc = 0x2A8C0;
    16
}
/// $02F0A0: bcc.w $2f0aa
pub fn i_02f0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F0AA; return 10; }
    8
}
/// $02F0A4: lea.l $2d02e(pc), a1
pub fn i_02f0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02F0A8: move.l a1, (a6)
pub fn i_02f0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F0AA: jsr $2ac80(pc)
pub fn i_02f0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F0AE);
    cpu.pc = 0x2AC80;
    16
}
/// $02F0AE: bcc.w $2f0b8
pub fn i_02f0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F0B8; return 10; }
    8
}
/// $02F0B2: lea.l $2dd74(pc), a1
pub fn i_02f0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02F0B6: move.l a1, (a6)
pub fn i_02f0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F0B8: jsr $2a5cc(pc)
pub fn i_02f0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F0BC);
    cpu.pc = 0x2A5CC;
    16
}
/// $02F0BC: bcc.w $2f0c6
pub fn i_02f0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F0C6; return 10; }
    8
}
/// $02F0C0: lea.l $2f0e6(pc), a1
pub fn i_02f0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02F0C4: move.l a1, (a6)
pub fn i_02f0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F0C6: jsr $2a690(pc)
pub fn i_02f0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F0CA);
    cpu.pc = 0x2A690;
    16
}
/// $02F0CA: lea.l $2a060(pc), a0
pub fn i_02f0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0CE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02F0CE: movea.l #$ffffffff, a1
pub fn i_02f0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0D4;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02f0d4  ($02F0D4, 8 B, thunks/jsr_abs_thunks.c)
/// $02F0D4: jsr $772.l
pub fn i_02f0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F0DA);
    cpu.pc = 0x772;
    16
}
/// $02F0DA: rts
pub fn i_02f0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Knocked_02f0dc  ($02F0DC, 288 B, slug/vehicle_02dd20.s)
/// $02F0DC: move.w #$600, $28(a6)
pub fn i_02f0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F0E2: bra.w $2f102
pub fn i_02f0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2F102;
    10
}
/// $02F102: jsr $2a9a0(pc)
pub fn i_02f102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F106;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F106);
    cpu.pc = 0x2A9A0;
    16
}
/// $02F106: movea.l #$279394, a0
pub fn i_02f106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F10C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279394;
    cpu.a[0] = v1;
    4
}
/// $02F10C: lsl.w #$2, d0
pub fn i_02f10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F10E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F10E: movea.l (a0, d0.w), a0
pub fn i_02f10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F112;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02F112: cmpa.l #$ffffffff, a0
pub fn i_02f112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F118;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02F118: beq.w $2f122
pub fn i_02f118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F11C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F122; return 10; }
    8
}
/// $02F11C: jsr $28cd4.l
pub fn i_02f11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F122;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F122);
    cpu.pc = 0x28CD4;
    16
}
/// $02F122: lsr.w #$1, d0
pub fn i_02f122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F124;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02F124: lea.l $2bb9a(pc), a0
pub fn i_02f124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F128;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2BB9A;
    16
}
/// $02F128: move.w (a0, d0.w), d0
pub fn i_02f128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F12C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02F12C: cmpi.w #$ffff, d0
pub fn i_02f12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F130;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02F130: beq.w $2f13a
pub fn i_02f130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F134;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F13A; return 10; }
    8
}
/// $02F134: jsr $2352.l
pub fn i_02f134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F13A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F13A);
    cpu.pc = 0x2352;
    16
}
/// $02F13A: move.w #$0, d0
pub fn i_02f13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F13E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F13E: move.b #$96, $20(a6)
pub fn i_02f13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F144;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F144: andi.w #$ff, d0
pub fn i_02f144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F148;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02F148: lsr.w #$2, d0
pub fn i_02f148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F14A;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F14A: add.b d0, $20(a6)
pub fn i_02f14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F14E;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F14E: lea.l $2f154(pc), a1
pub fn i_02f14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F152;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F154;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x02D792, i_02d792),
    (0x02D798, i_02d798),
    (0x02D79C, i_02d79c),
    (0x02D7A2, i_02d7a2),
    (0x02D7A8, i_02d7a8),
    (0x02D7AC, i_02d7ac),
    (0x02D7B0, i_02d7b0),
    (0x02D7B4, i_02d7b4),
    (0x02D7BA, i_02d7ba),
    (0x02D7BE, i_02d7be),
    (0x02D7C2, i_02d7c2),
    (0x02D7C6, i_02d7c6),
    (0x02D7CA, i_02d7ca),
    (0x02D7CC, i_02d7cc),
    (0x02D7D2, i_02d7d2),
    (0x02D7D8, i_02d7d8),
    (0x02D7DC, i_02d7dc),
    (0x02D7E0, i_02d7e0),
    (0x02D7E4, i_02d7e4),
    (0x02D7E6, i_02d7e6),
    (0x02D7EC, i_02d7ec),
    (0x02D7F2, i_02d7f2),
    (0x02D7F8, i_02d7f8),
    (0x02D7FC, i_02d7fc),
    (0x02D800, i_02d800),
    (0x02D802, i_02d802),
    (0x02D808, i_02d808),
    (0x02D80C, i_02d80c),
    (0x02D812, i_02d812),
    (0x02D814, i_02d814),
    (0x02D818, i_02d818),
    (0x02D81E, i_02d81e),
    (0x02D822, i_02d822),
    (0x02D828, i_02d828),
    (0x02D82C, i_02d82c),
    (0x02D832, i_02d832),
    (0x02D836, i_02d836),
    (0x02D838, i_02d838),
    (0x02D83C, i_02d83c),
    (0x02D842, i_02d842),
    (0x02D848, i_02d848),
    (0x02D84C, i_02d84c),
    (0x02D850, i_02d850),
    (0x02D852, i_02d852),
    (0x02D858, i_02d858),
    (0x02D85E, i_02d85e),
    (0x02D862, i_02d862),
    (0x02D868, i_02d868),
    (0x02D86E, i_02d86e),
    (0x02D872, i_02d872),
    (0x02D876, i_02d876),
    (0x02D87A, i_02d87a),
    (0x02D880, i_02d880),
    (0x02D884, i_02d884),
    (0x02D888, i_02d888),
    (0x02D88C, i_02d88c),
    (0x02D890, i_02d890),
    (0x02D892, i_02d892),
    (0x02D898, i_02d898),
    (0x02D89E, i_02d89e),
    (0x02D8A2, i_02d8a2),
    (0x02D8A6, i_02d8a6),
    (0x02D8AA, i_02d8aa),
    (0x02D8AC, i_02d8ac),
    (0x02D8B2, i_02d8b2),
    (0x02D8B8, i_02d8b8),
    (0x02D8BE, i_02d8be),
    (0x02D8C2, i_02d8c2),
    (0x02D8C6, i_02d8c6),
    (0x02D8C8, i_02d8c8),
    (0x02D8CE, i_02d8ce),
    (0x02D8D2, i_02d8d2),
    (0x02D8D8, i_02d8d8),
    (0x02D8DA, i_02d8da),
    (0x02D8DE, i_02d8de),
    (0x02D8E4, i_02d8e4),
    (0x02D8E8, i_02d8e8),
    (0x02D8EE, i_02d8ee),
    (0x02D8F2, i_02d8f2),
    (0x02D8F8, i_02d8f8),
    (0x02D8FC, i_02d8fc),
    (0x02D8FE, i_02d8fe),
    (0x02D902, i_02d902),
    (0x02D906, i_02d906),
    (0x02D908, i_02d908),
    (0x02D90E, i_02d90e),
    (0x02D912, i_02d912),
    (0x02D918, i_02d918),
    (0x02D91E, i_02d91e),
    (0x02D922, i_02d922),
    (0x02D926, i_02d926),
    (0x02D92A, i_02d92a),
    (0x02D930, i_02d930),
    (0x02D934, i_02d934),
    (0x02D938, i_02d938),
    (0x02D93E, i_02d93e),
    (0x02D942, i_02d942),
    (0x02D946, i_02d946),
    (0x02D948, i_02d948),
    (0x02D94C, i_02d94c),
    (0x02D950, i_02d950),
    (0x02D954, i_02d954),
    (0x02D956, i_02d956),
    (0x02D95A, i_02d95a),
    (0x02D95E, i_02d95e),
    (0x02D962, i_02d962),
    (0x02D964, i_02d964),
    (0x02D96A, i_02d96a),
    (0x02D970, i_02d970),
    (0x02D976, i_02d976),
    (0x02D97A, i_02d97a),
    (0x02D97E, i_02d97e),
    (0x02D980, i_02d980),
    (0x02D986, i_02d986),
    (0x02D98A, i_02d98a),
    (0x02D990, i_02d990),
    (0x02D992, i_02d992),
    (0x02D996, i_02d996),
    (0x02D99C, i_02d99c),
    (0x02D9A0, i_02d9a0),
    (0x02D9A6, i_02d9a6),
    (0x02D9AA, i_02d9aa),
    (0x02D9B0, i_02d9b0),
    (0x02D9B4, i_02d9b4),
    (0x02D9B6, i_02d9b6),
    (0x02D9BA, i_02d9ba),
    (0x02D9BE, i_02d9be),
    (0x02D9C0, i_02d9c0),
    (0x02D9C6, i_02d9c6),
    (0x02D9CA, i_02d9ca),
    (0x02D9D0, i_02d9d0),
    (0x02D9D6, i_02d9d6),
    (0x02D9DA, i_02d9da),
    (0x02D9DE, i_02d9de),
    (0x02D9E2, i_02d9e2),
    (0x02D9E8, i_02d9e8),
    (0x02D9EC, i_02d9ec),
    (0x02D9F0, i_02d9f0),
    (0x02D9F6, i_02d9f6),
    (0x02D9FA, i_02d9fa),
    (0x02D9FE, i_02d9fe),
    (0x02DA00, i_02da00),
    (0x02DA04, i_02da04),
    (0x02DA08, i_02da08),
    (0x02DA0C, i_02da0c),
    (0x02DA0E, i_02da0e),
    (0x02DA12, i_02da12),
    (0x02DA16, i_02da16),
    (0x02DA1A, i_02da1a),
    (0x02DA1C, i_02da1c),
    (0x02DA22, i_02da22),
    (0x02DA28, i_02da28),
    (0x02DA2E, i_02da2e),
    (0x02DA32, i_02da32),
    (0x02DA36, i_02da36),
    (0x02DA38, i_02da38),
    (0x02DA3C, i_02da3c),
    (0x02DA42, i_02da42),
    (0x02DA48, i_02da48),
    (0x02DA4C, i_02da4c),
    (0x02DA50, i_02da50),
    (0x02DA56, i_02da56),
    (0x02DA5A, i_02da5a),
    (0x02DA60, i_02da60),
    (0x02DA64, i_02da64),
    (0x02DA66, i_02da66),
    (0x02DA6A, i_02da6a),
    (0x02DA6E, i_02da6e),
    (0x02DA72, i_02da72),
    (0x02DA74, i_02da74),
    (0x02DA76, i_02da76),
    (0x02DA78, i_02da78),
    (0x02DA7A, i_02da7a),
    (0x02DA7C, i_02da7c),
    (0x02DA80, i_02da80),
    (0x02DA84, i_02da84),
    (0x02DA8A, i_02da8a),
    (0x02DA8C, i_02da8c),
    (0x02DA90, i_02da90),
    (0x02DA96, i_02da96),
    (0x02DA9A, i_02da9a),
    (0x02DAA0, i_02daa0),
    (0x02DAA4, i_02daa4),
    (0x02DAA8, i_02daa8),
    (0x02DAAE, i_02daae),
    (0x02DAB0, i_02dab0),
    (0x02DAB4, i_02dab4),
    (0x02DABA, i_02daba),
    (0x02DABE, i_02dabe),
    (0x02DAC4, i_02dac4),
    (0x02DAC8, i_02dac8),
    (0x02DACE, i_02dace),
    (0x02DAD2, i_02dad2),
    (0x02DAD4, i_02dad4),
    (0x02DAD8, i_02dad8),
    (0x02DADE, i_02dade),
    (0x02DAE2, i_02dae2),
    (0x02DAE4, i_02dae4),
    (0x02DAE8, i_02dae8),
    (0x02DAEE, i_02daee),
    (0x02DAF2, i_02daf2),
    (0x02DAF6, i_02daf6),
    (0x02DAFA, i_02dafa),
    (0x02DAFC, i_02dafc),
    (0x02DB00, i_02db00),
    (0x02DB04, i_02db04),
    (0x02DB0A, i_02db0a),
    (0x02DB10, i_02db10),
    (0x02DB16, i_02db16),
    (0x02DB1A, i_02db1a),
    (0x02DB1E, i_02db1e),
    (0x02DB24, i_02db24),
    (0x02DB28, i_02db28),
    (0x02DB2C, i_02db2c),
    (0x02DB32, i_02db32),
    (0x02DB38, i_02db38),
    (0x02DB3E, i_02db3e),
    (0x02DB44, i_02db44),
    (0x02DB48, i_02db48),
    (0x02DB4C, i_02db4c),
    (0x02DB50, i_02db50),
    (0x02DB52, i_02db52),
    (0x02DB58, i_02db58),
    (0x02DB5E, i_02db5e),
    (0x02DB62, i_02db62),
    (0x02DB66, i_02db66),
    (0x02DB6C, i_02db6c),
    (0x02DB70, i_02db70),
    (0x02DB76, i_02db76),
    (0x02DB7A, i_02db7a),
    (0x02DB7C, i_02db7c),
    (0x02DB80, i_02db80),
    (0x02DB84, i_02db84),
    (0x02DB88, i_02db88),
    (0x02DB8A, i_02db8a),
    (0x02DB8C, i_02db8c),
    (0x02DB8E, i_02db8e),
    (0x02DB90, i_02db90),
    (0x02DB92, i_02db92),
    (0x02DB96, i_02db96),
    (0x02DB9A, i_02db9a),
    (0x02DBA0, i_02dba0),
    (0x02DBA2, i_02dba2),
    (0x02DBA6, i_02dba6),
    (0x02DBAC, i_02dbac),
    (0x02DBB0, i_02dbb0),
    (0x02DBB6, i_02dbb6),
    (0x02DBBA, i_02dbba),
    (0x02DBBE, i_02dbbe),
    (0x02DBC4, i_02dbc4),
    (0x02DBC6, i_02dbc6),
    (0x02DBCA, i_02dbca),
    (0x02DBD0, i_02dbd0),
    (0x02DBD4, i_02dbd4),
    (0x02DBDA, i_02dbda),
    (0x02DBDE, i_02dbde),
    (0x02DBE4, i_02dbe4),
    (0x02DBE8, i_02dbe8),
    (0x02DBEA, i_02dbea),
    (0x02DBEE, i_02dbee),
    (0x02DBF4, i_02dbf4),
    (0x02DBFA, i_02dbfa),
    (0x02DC00, i_02dc00),
    (0x02DC06, i_02dc06),
    (0x02DC0A, i_02dc0a),
    (0x02DC0C, i_02dc0c),
    (0x02DC10, i_02dc10),
    (0x02DC16, i_02dc16),
    (0x02DC1A, i_02dc1a),
    (0x02DC1E, i_02dc1e),
    (0x02DC24, i_02dc24),
    (0x02DC28, i_02dc28),
    (0x02DC2C, i_02dc2c),
    (0x02DC30, i_02dc30),
    (0x02DC34, i_02dc34),
    (0x02DC36, i_02dc36),
    (0x02DC3C, i_02dc3c),
    (0x02DC42, i_02dc42),
    (0x02DC48, i_02dc48),
    (0x02DC4E, i_02dc4e),
    (0x02DC52, i_02dc52),
    (0x02DC56, i_02dc56),
    (0x02DC5A, i_02dc5a),
    (0x02DC5C, i_02dc5c),
    (0x02DC62, i_02dc62),
    (0x02DC68, i_02dc68),
    (0x02DC6C, i_02dc6c),
    (0x02DC70, i_02dc70),
    (0x02DC78, i_02dc78),
    (0x02DC7C, i_02dc7c),
    (0x02DC7E, i_02dc7e),
    (0x02DC82, i_02dc82),
    (0x02DC88, i_02dc88),
    (0x02DC8C, i_02dc8c),
    (0x02DC92, i_02dc92),
    (0x02DC98, i_02dc98),
    (0x02DC9E, i_02dc9e),
    (0x02DCA2, i_02dca2),
    (0x02DCA4, i_02dca4),
    (0x02DCA8, i_02dca8),
    (0x02DCAA, i_02dcaa),
    (0x02DCB0, i_02dcb0),
    (0x02DCB2, i_02dcb2),
    (0x02DCB4, i_02dcb4),
    (0x02DCBA, i_02dcba),
    (0x02DCBC, i_02dcbc),
    (0x02DCC0, i_02dcc0),
    (0x02DCC4, i_02dcc4),
    (0x02DCC8, i_02dcc8),
    (0x02DCCE, i_02dcce),
    (0x02DCD4, i_02dcd4),
    (0x02DCDA, i_02dcda),
    (0x02DCDE, i_02dcde),
    (0x02DCE2, i_02dce2),
    (0x02DCE8, i_02dce8),
    (0x02DCEC, i_02dcec),
    (0x02DCF0, i_02dcf0),
    (0x02DCF2, i_02dcf2),
    (0x02DCF6, i_02dcf6),
    (0x02DCFA, i_02dcfa),
    (0x02DD00, i_02dd00),
    (0x02DD06, i_02dd06),
    (0x02DD0C, i_02dd0c),
    (0x02DD12, i_02dd12),
    (0x02DD16, i_02dd16),
    (0x02DD1A, i_02dd1a),
    (0x02DD1E, i_02dd1e),
    (0x02DD20, i_02dd20),
    (0x02DD26, i_02dd26),
    (0x02DD2A, i_02dd2a),
    (0x02DD30, i_02dd30),
    (0x02DD32, i_02dd32),
    (0x02DD36, i_02dd36),
    (0x02DD3C, i_02dd3c),
    (0x02DD40, i_02dd40),
    (0x02DD46, i_02dd46),
    (0x02DD4C, i_02dd4c),
    (0x02DD50, i_02dd50),
    (0x02DD52, i_02dd52),
    (0x02DD56, i_02dd56),
    (0x02DD5A, i_02dd5a),
    (0x02DD62, i_02dd62),
    (0x02DD66, i_02dd66),
    (0x02DD6A, i_02dd6a),
    (0x02DD6E, i_02dd6e),
    (0x02DD70, i_02dd70),
    (0x02DD74, i_02dd74),
    (0x02DD7A, i_02dd7a),
    (0x02DD7E, i_02dd7e),
    (0x02DD84, i_02dd84),
    (0x02DD86, i_02dd86),
    (0x02DD8A, i_02dd8a),
    (0x02DD90, i_02dd90),
    (0x02DD94, i_02dd94),
    (0x02DD9A, i_02dd9a),
    (0x02DDA0, i_02dda0),
    (0x02DDA4, i_02dda4),
    (0x02DDA6, i_02dda6),
    (0x02DDAA, i_02ddaa),
    (0x02DDAE, i_02ddae),
    (0x02DDB6, i_02ddb6),
    (0x02DDBA, i_02ddba),
    (0x02DDBE, i_02ddbe),
    (0x02DDC2, i_02ddc2),
    (0x02DDC4, i_02ddc4),
    (0x02DDC8, i_02ddc8),
    (0x02DDCE, i_02ddce),
    (0x02DDD2, i_02ddd2),
    (0x02DDD8, i_02ddd8),
    (0x02DDDA, i_02ddda),
    (0x02DDDE, i_02ddde),
    (0x02DDE4, i_02dde4),
    (0x02DDE8, i_02dde8),
    (0x02DDEE, i_02ddee),
    (0x02DDF4, i_02ddf4),
    (0x02DDF8, i_02ddf8),
    (0x02DDFA, i_02ddfa),
    (0x02DDFE, i_02ddfe),
    (0x02DE02, i_02de02),
    (0x02DE0A, i_02de0a),
    (0x02DE0E, i_02de0e),
    (0x02DE12, i_02de12),
    (0x02DE16, i_02de16),
    (0x02DE18, i_02de18),
    (0x02DE1C, i_02de1c),
    (0x02DE22, i_02de22),
    (0x02DE28, i_02de28),
    (0x02DE2C, i_02de2c),
    (0x02DE30, i_02de30),
    (0x02DE34, i_02de34),
    (0x02DE38, i_02de38),
    (0x02DE3A, i_02de3a),
    (0x02DE3E, i_02de3e),
    (0x02DE44, i_02de44),
    (0x02DE48, i_02de48),
    (0x02DE4C, i_02de4c),
    (0x02DE50, i_02de50),
    (0x02DE52, i_02de52),
    (0x02DE56, i_02de56),
    (0x02DE5A, i_02de5a),
    (0x02DE5E, i_02de5e),
    (0x02DE60, i_02de60),
    (0x02DE64, i_02de64),
    (0x02DE68, i_02de68),
    (0x02DE6C, i_02de6c),
    (0x02DE6E, i_02de6e),
    (0x02DE76, i_02de76),
    (0x02DE7A, i_02de7a),
    (0x02DE80, i_02de80),
    (0x02DE86, i_02de86),
    (0x02DE8C, i_02de8c),
    (0x02DE90, i_02de90),
    (0x02DE94, i_02de94),
    (0x02DE98, i_02de98),
    (0x02DE9E, i_02de9e),
    (0x02DEA4, i_02dea4),
    (0x02DEAA, i_02deaa),
    (0x02DEAE, i_02deae),
    (0x02DEB2, i_02deb2),
    (0x02DEB4, i_02deb4),
    (0x02DEB8, i_02deb8),
    (0x02DEBC, i_02debc),
    (0x02DEC0, i_02dec0),
    (0x02DEC2, i_02dec2),
    (0x02DEC4, i_02dec4),
    (0x02DEC6, i_02dec6),
    (0x02DECA, i_02deca),
    (0x02DECE, i_02dece),
    (0x02DED2, i_02ded2),
    (0x02DED6, i_02ded6),
    (0x02DEDA, i_02deda),
    (0x02DEDE, i_02dede),
    (0x02DEE4, i_02dee4),
    (0x02DEE6, i_02dee6),
    (0x02DEEA, i_02deea),
    (0x02DEF0, i_02def0),
    (0x02DEF4, i_02def4),
    (0x02DEFA, i_02defa),
    (0x02DEFE, i_02defe),
    (0x02DF02, i_02df02),
    (0x02DF08, i_02df08),
    (0x02DF10, i_02df10),
    (0x02DF14, i_02df14),
    (0x02DF18, i_02df18),
    (0x02DF1C, i_02df1c),
    (0x02DF1E, i_02df1e),
    (0x02DF24, i_02df24),
    (0x02DF2A, i_02df2a),
    (0x02DF30, i_02df30),
    (0x02DF34, i_02df34),
    (0x02DF38, i_02df38),
    (0x02DF40, i_02df40),
    (0x02DF44, i_02df44),
    (0x02DF48, i_02df48),
    (0x02DF4C, i_02df4c),
    (0x02DF50, i_02df50),
    (0x02DF52, i_02df52),
    (0x02DF54, i_02df54),
    (0x02DF58, i_02df58),
    (0x02DF5A, i_02df5a),
    (0x02DF5C, i_02df5c),
    (0x02DF5E, i_02df5e),
    (0x02DF62, i_02df62),
    (0x02DF66, i_02df66),
    (0x02DF6A, i_02df6a),
    (0x02DF6E, i_02df6e),
    (0x02DF72, i_02df72),
    (0x02DF76, i_02df76),
    (0x02DF7A, i_02df7a),
    (0x02DF7C, i_02df7c),
    (0x02DF7E, i_02df7e),
    (0x02DF80, i_02df80),
    (0x02DF84, i_02df84),
    (0x02DF88, i_02df88),
    (0x02DF8C, i_02df8c),
    (0x02DF90, i_02df90),
    (0x02DF94, i_02df94),
    (0x02DF98, i_02df98),
    (0x02DF9E, i_02df9e),
    (0x02DFA0, i_02dfa0),
    (0x02DFA4, i_02dfa4),
    (0x02DFAA, i_02dfaa),
    (0x02DFAE, i_02dfae),
    (0x02DFB4, i_02dfb4),
    (0x02DFB8, i_02dfb8),
    (0x02DFBC, i_02dfbc),
    (0x02DFC2, i_02dfc2),
    (0x02DFC8, i_02dfc8),
    (0x02DFCC, i_02dfcc),
    (0x02DFD0, i_02dfd0),
    (0x02DFD4, i_02dfd4),
    (0x02DFD8, i_02dfd8),
    (0x02DFDA, i_02dfda),
    (0x02DFDE, i_02dfde),
    (0x02DFE2, i_02dfe2),
    (0x02DFE6, i_02dfe6),
    (0x02DFE8, i_02dfe8),
    (0x02DFEC, i_02dfec),
    (0x02DFF0, i_02dff0),
    (0x02DFF4, i_02dff4),
    (0x02DFF6, i_02dff6),
    (0x02DFFA, i_02dffa),
    (0x02DFFE, i_02dffe),
    (0x02E002, i_02e002),
    (0x02E004, i_02e004),
    (0x02E00C, i_02e00c),
    (0x02E010, i_02e010),
    (0x02E016, i_02e016),
    (0x02E01C, i_02e01c),
    (0x02E022, i_02e022),
    (0x02E026, i_02e026),
    (0x02E02A, i_02e02a),
    (0x02E02E, i_02e02e),
    (0x02E034, i_02e034),
    (0x02E03A, i_02e03a),
    (0x02E040, i_02e040),
    (0x02E044, i_02e044),
    (0x02E048, i_02e048),
    (0x02E04A, i_02e04a),
    (0x02E050, i_02e050),
    (0x02E056, i_02e056),
    (0x02E05A, i_02e05a),
    (0x02E05E, i_02e05e),
    (0x02E062, i_02e062),
    (0x02E066, i_02e066),
    (0x02E06A, i_02e06a),
    (0x02E06E, i_02e06e),
    (0x02E072, i_02e072),
    (0x02E07A, i_02e07a),
    (0x02E07E, i_02e07e),
    (0x02E082, i_02e082),
    (0x02E086, i_02e086),
    (0x02E088, i_02e088),
    (0x02E08E, i_02e08e),
    (0x02E092, i_02e092),
    (0x02E098, i_02e098),
    (0x02E09E, i_02e09e),
    (0x02E0A2, i_02e0a2),
    (0x02E0A6, i_02e0a6),
    (0x02E0AA, i_02e0aa),
    (0x02E0AE, i_02e0ae),
    (0x02E0B2, i_02e0b2),
    (0x02E0B6, i_02e0b6),
    (0x02E0BA, i_02e0ba),
    (0x02E0BE, i_02e0be),
    (0x02E0C2, i_02e0c2),
    (0x02E0C4, i_02e0c4),
    (0x02E0C6, i_02e0c6),
    (0x02E0C8, i_02e0c8),
    (0x02E0CC, i_02e0cc),
    (0x02E0D0, i_02e0d0),
    (0x02E0D4, i_02e0d4),
    (0x02E0D8, i_02e0d8),
    (0x02E0DC, i_02e0dc),
    (0x02E0E0, i_02e0e0),
    (0x02E0E6, i_02e0e6),
    (0x02E0E8, i_02e0e8),
    (0x02E0EC, i_02e0ec),
    (0x02E0F2, i_02e0f2),
    (0x02E0F6, i_02e0f6),
    (0x02E0FC, i_02e0fc),
    (0x02E100, i_02e100),
    (0x02E106, i_02e106),
    (0x02E108, i_02e108),
    (0x02E10C, i_02e10c),
    (0x02E110, i_02e110),
    (0x02E116, i_02e116),
    (0x02E11E, i_02e11e),
    (0x02E122, i_02e122),
    (0x02E126, i_02e126),
    (0x02E12A, i_02e12a),
    (0x02E12C, i_02e12c),
    (0x02E130, i_02e130),
    (0x02E134, i_02e134),
    (0x02E13A, i_02e13a),
    (0x02E13E, i_02e13e),
    (0x02E144, i_02e144),
    (0x02E14A, i_02e14a),
    (0x02E14E, i_02e14e),
    (0x02E152, i_02e152),
    (0x02E156, i_02e156),
    (0x02E15A, i_02e15a),
    (0x02E15C, i_02e15c),
    (0x02E160, i_02e160),
    (0x02E166, i_02e166),
    (0x02E16A, i_02e16a),
    (0x02E16E, i_02e16e),
    (0x02E172, i_02e172),
    (0x02E176, i_02e176),
    (0x02E17A, i_02e17a),
    (0x02E17E, i_02e17e),
    (0x02E182, i_02e182),
    (0x02E186, i_02e186),
    (0x02E18A, i_02e18a),
    (0x02E18E, i_02e18e),
    (0x02E192, i_02e192),
    (0x02E194, i_02e194),
    (0x02E196, i_02e196),
    (0x02E198, i_02e198),
    (0x02E19C, i_02e19c),
    (0x02E1A0, i_02e1a0),
    (0x02E1A4, i_02e1a4),
    (0x02E1A8, i_02e1a8),
    (0x02E1AC, i_02e1ac),
    (0x02E1B0, i_02e1b0),
    (0x02E1B6, i_02e1b6),
    (0x02E1B8, i_02e1b8),
    (0x02E1BC, i_02e1bc),
    (0x02E1C2, i_02e1c2),
    (0x02E1C6, i_02e1c6),
    (0x02E1CC, i_02e1cc),
    (0x02E1D0, i_02e1d0),
    (0x02E1D6, i_02e1d6),
    (0x02E1D8, i_02e1d8),
    (0x02E1DC, i_02e1dc),
    (0x02E1E0, i_02e1e0),
    (0x02E1E6, i_02e1e6),
    (0x02E1EC, i_02e1ec),
    (0x02E1F0, i_02e1f0),
    (0x02E1F4, i_02e1f4),
    (0x02E1F8, i_02e1f8),
    (0x02E1FC, i_02e1fc),
    (0x02E1FE, i_02e1fe),
    (0x02E206, i_02e206),
    (0x02E20A, i_02e20a),
    (0x02E210, i_02e210),
    (0x02E216, i_02e216),
    (0x02E21C, i_02e21c),
    (0x02E220, i_02e220),
    (0x02E224, i_02e224),
    (0x02E228, i_02e228),
    (0x02E22E, i_02e22e),
    (0x02E234, i_02e234),
    (0x02E23A, i_02e23a),
    (0x02E23E, i_02e23e),
    (0x02E242, i_02e242),
    (0x02E244, i_02e244),
    (0x02E248, i_02e248),
    (0x02E24C, i_02e24c),
    (0x02E250, i_02e250),
    (0x02E252, i_02e252),
    (0x02E254, i_02e254),
    (0x02E256, i_02e256),
    (0x02E25A, i_02e25a),
    (0x02E25E, i_02e25e),
    (0x02E262, i_02e262),
    (0x02E266, i_02e266),
    (0x02E26A, i_02e26a),
    (0x02E26E, i_02e26e),
    (0x02E274, i_02e274),
    (0x02E276, i_02e276),
    (0x02E27A, i_02e27a),
    (0x02E280, i_02e280),
    (0x02E284, i_02e284),
    (0x02E28A, i_02e28a),
    (0x02E28E, i_02e28e),
    (0x02E292, i_02e292),
    (0x02E298, i_02e298),
    (0x02E2A0, i_02e2a0),
    (0x02E2A4, i_02e2a4),
    (0x02E2A8, i_02e2a8),
    (0x02E2AC, i_02e2ac),
    (0x02E2AE, i_02e2ae),
    (0x02E2B4, i_02e2b4),
    (0x02E2BA, i_02e2ba),
    (0x02E2C0, i_02e2c0),
    (0x02E2C4, i_02e2c4),
    (0x02E2C8, i_02e2c8),
    (0x02E2D0, i_02e2d0),
    (0x02E2D4, i_02e2d4),
    (0x02E2D8, i_02e2d8),
    (0x02E2DC, i_02e2dc),
    (0x02E2E0, i_02e2e0),
    (0x02E2E2, i_02e2e2),
    (0x02E2E4, i_02e2e4),
    (0x02E2E8, i_02e2e8),
    (0x02E2EA, i_02e2ea),
    (0x02E2EC, i_02e2ec),
    (0x02E2EE, i_02e2ee),
    (0x02E2F2, i_02e2f2),
    (0x02E2F6, i_02e2f6),
    (0x02E2FC, i_02e2fc),
    (0x02E300, i_02e300),
    (0x02E304, i_02e304),
    (0x02E308, i_02e308),
    (0x02E30C, i_02e30c),
    (0x02E310, i_02e310),
    (0x02E312, i_02e312),
    (0x02E314, i_02e314),
    (0x02E316, i_02e316),
    (0x02E31A, i_02e31a),
    (0x02E31E, i_02e31e),
    (0x02E322, i_02e322),
    (0x02E326, i_02e326),
    (0x02E32A, i_02e32a),
    (0x02E32E, i_02e32e),
    (0x02E334, i_02e334),
    (0x02E336, i_02e336),
    (0x02E33A, i_02e33a),
    (0x02E340, i_02e340),
    (0x02E344, i_02e344),
    (0x02E34A, i_02e34a),
    (0x02E34E, i_02e34e),
    (0x02E352, i_02e352),
    (0x02E358, i_02e358),
    (0x02E35E, i_02e35e),
    (0x02E362, i_02e362),
    (0x02E366, i_02e366),
    (0x02E36A, i_02e36a),
    (0x02E36E, i_02e36e),
    (0x02E370, i_02e370),
    (0x02E374, i_02e374),
    (0x02E378, i_02e378),
    (0x02E37E, i_02e37e),
    (0x02E382, i_02e382),
    (0x02E386, i_02e386),
    (0x02E38A, i_02e38a),
    (0x02E38C, i_02e38c),
    (0x02E390, i_02e390),
    (0x02E394, i_02e394),
    (0x02E398, i_02e398),
    (0x02E39A, i_02e39a),
    (0x02E39E, i_02e39e),
    (0x02E3A2, i_02e3a2),
    (0x02E3A8, i_02e3a8),
    (0x02E3AE, i_02e3ae),
    (0x02E3B0, i_02e3b0),
    (0x02E3B6, i_02e3b6),
    (0x02E3BC, i_02e3bc),
    (0x02E3C2, i_02e3c2),
    (0x02E3C6, i_02e3c6),
    (0x02E3CC, i_02e3cc),
    (0x02E3CE, i_02e3ce),
    (0x02E3D2, i_02e3d2),
    (0x02E3D8, i_02e3d8),
    (0x02E3DC, i_02e3dc),
    (0x02E3E2, i_02e3e2),
    (0x02E3E6, i_02e3e6),
    (0x02E3EA, i_02e3ea),
    (0x02E3EE, i_02e3ee),
    (0x02E3F4, i_02e3f4),
    (0x02E3F8, i_02e3f8),
    (0x02E3FA, i_02e3fa),
    (0x02E3FE, i_02e3fe),
    (0x02E404, i_02e404),
    (0x02E40A, i_02e40a),
    (0x02E410, i_02e410),
    (0x02E418, i_02e418),
    (0x02E41C, i_02e41c),
    (0x02E420, i_02e420),
    (0x02E424, i_02e424),
    (0x02E428, i_02e428),
    (0x02E42A, i_02e42a),
    (0x02E42E, i_02e42e),
    (0x02E432, i_02e432),
    (0x02E438, i_02e438),
    (0x02E43C, i_02e43c),
    (0x02E440, i_02e440),
    (0x02E444, i_02e444),
    (0x02E44A, i_02e44a),
    (0x02E44E, i_02e44e),
    (0x02E452, i_02e452),
    (0x02E456, i_02e456),
    (0x02E45E, i_02e45e),
    (0x02E462, i_02e462),
    (0x02E464, i_02e464),
    (0x02E468, i_02e468),
    (0x02E46C, i_02e46c),
    (0x02E470, i_02e470),
    (0x02E472, i_02e472),
    (0x02E476, i_02e476),
    (0x02E47A, i_02e47a),
    (0x02E47E, i_02e47e),
    (0x02E480, i_02e480),
    (0x02E484, i_02e484),
    (0x02E488, i_02e488),
    (0x02E48E, i_02e48e),
    (0x02E494, i_02e494),
    (0x02E496, i_02e496),
    (0x02E49C, i_02e49c),
    (0x02E4A2, i_02e4a2),
    (0x02E4A8, i_02e4a8),
    (0x02E4AE, i_02e4ae),
    (0x02E4B6, i_02e4b6),
    (0x02E4BC, i_02e4bc),
    (0x02E4C0, i_02e4c0),
    (0x02E4C4, i_02e4c4),
    (0x02E4C8, i_02e4c8),
    (0x02E4CC, i_02e4cc),
    (0x02E4CE, i_02e4ce),
    (0x02E4D0, i_02e4d0),
    (0x02E4D2, i_02e4d2),
    (0x02E4D6, i_02e4d6),
    (0x02E4DA, i_02e4da),
    (0x02E4DE, i_02e4de),
    (0x02E4E2, i_02e4e2),
    (0x02E4E6, i_02e4e6),
    (0x02E4EA, i_02e4ea),
    (0x02E4F0, i_02e4f0),
    (0x02E4F2, i_02e4f2),
    (0x02E4F6, i_02e4f6),
    (0x02E4FC, i_02e4fc),
    (0x02E500, i_02e500),
    (0x02E506, i_02e506),
    (0x02E50A, i_02e50a),
    (0x02E510, i_02e510),
    (0x02E512, i_02e512),
    (0x02E516, i_02e516),
    (0x02E51A, i_02e51a),
    (0x02E520, i_02e520),
    (0x02E524, i_02e524),
    (0x02E528, i_02e528),
    (0x02E52C, i_02e52c),
    (0x02E52E, i_02e52e),
    (0x02E534, i_02e534),
    (0x02E538, i_02e538),
    (0x02E53C, i_02e53c),
    (0x02E540, i_02e540),
    (0x02E546, i_02e546),
    (0x02E54A, i_02e54a),
    (0x02E54E, i_02e54e),
    (0x02E550, i_02e550),
    (0x02E554, i_02e554),
    (0x02E558, i_02e558),
    (0x02E55C, i_02e55c),
    (0x02E55E, i_02e55e),
    (0x02E562, i_02e562),
    (0x02E566, i_02e566),
    (0x02E56A, i_02e56a),
    (0x02E56C, i_02e56c),
    (0x02E570, i_02e570),
    (0x02E574, i_02e574),
    (0x02E57A, i_02e57a),
    (0x02E580, i_02e580),
    (0x02E6C2, i_02e6c2),
    (0x02E6C8, i_02e6c8),
    (0x02E6CE, i_02e6ce),
    (0x02E6D2, i_02e6d2),
    (0x02E6D8, i_02e6d8),
    (0x02E6DA, i_02e6da),
    (0x02E6DE, i_02e6de),
    (0x02E6E4, i_02e6e4),
    (0x02E6E8, i_02e6e8),
    (0x02E6EE, i_02e6ee),
    (0x02E6F2, i_02e6f2),
    (0x02E6F6, i_02e6f6),
    (0x02E6FA, i_02e6fa),
    (0x02E700, i_02e700),
    (0x02E706, i_02e706),
    (0x02E70A, i_02e70a),
    (0x02E70C, i_02e70c),
    (0x02E710, i_02e710),
    (0x02E718, i_02e718),
    (0x02E71C, i_02e71c),
    (0x02E720, i_02e720),
    (0x02E722, i_02e722),
    (0x02E726, i_02e726),
    (0x02E72C, i_02e72c),
    (0x02E730, i_02e730),
    (0x02E734, i_02e734),
    (0x02E738, i_02e738),
    (0x02E73C, i_02e73c),
    (0x02E740, i_02e740),
    (0x02E746, i_02e746),
    (0x02E74A, i_02e74a),
    (0x02E74E, i_02e74e),
    (0x02E754, i_02e754),
    (0x02E75A, i_02e75a),
    (0x02E75E, i_02e75e),
    (0x02E762, i_02e762),
    (0x02E766, i_02e766),
    (0x02E768, i_02e768),
    (0x02E76C, i_02e76c),
    (0x02E770, i_02e770),
    (0x02E776, i_02e776),
    (0x02E77A, i_02e77a),
    (0x02E77E, i_02e77e),
    (0x02E780, i_02e780),
    (0x02E784, i_02e784),
    (0x02E788, i_02e788),
    (0x02E78C, i_02e78c),
    (0x02E78E, i_02e78e),
    (0x02E792, i_02e792),
    (0x02E796, i_02e796),
    (0x02E79A, i_02e79a),
    (0x02E79C, i_02e79c),
    (0x02E7A2, i_02e7a2),
    (0x02E7A6, i_02e7a6),
    (0x02E7AA, i_02e7aa),
    (0x02E7AC, i_02e7ac),
    (0x02E7B0, i_02e7b0),
    (0x02E7B4, i_02e7b4),
    (0x02E7B8, i_02e7b8),
    (0x02E7BA, i_02e7ba),
    (0x02E7BE, i_02e7be),
    (0x02E7C2, i_02e7c2),
    (0x02E7C6, i_02e7c6),
    (0x02E7C8, i_02e7c8),
    (0x02E7CC, i_02e7cc),
    (0x02E7D0, i_02e7d0),
    (0x02E7D4, i_02e7d4),
    (0x02E7D6, i_02e7d6),
    (0x02E7DA, i_02e7da),
    (0x02E7DE, i_02e7de),
    (0x02E7E2, i_02e7e2),
    (0x02E7E4, i_02e7e4),
    (0x02E7E8, i_02e7e8),
    (0x02E7EC, i_02e7ec),
    (0x02E7F2, i_02e7f2),
    (0x02E7F8, i_02e7f8),
    (0x02E7FA, i_02e7fa),
    (0x02E800, i_02e800),
    (0x02E806, i_02e806),
    (0x02E80A, i_02e80a),
    (0x02E80E, i_02e80e),
    (0x02E814, i_02e814),
    (0x02E81A, i_02e81a),
    (0x02E81E, i_02e81e),
    (0x02E824, i_02e824),
    (0x02E826, i_02e826),
    (0x02E82A, i_02e82a),
    (0x02E830, i_02e830),
    (0x02E834, i_02e834),
    (0x02E83A, i_02e83a),
    (0x02E840, i_02e840),
    (0x02E844, i_02e844),
    (0x02E846, i_02e846),
    (0x02E84A, i_02e84a),
    (0x02E852, i_02e852),
    (0x02E856, i_02e856),
    (0x02E85A, i_02e85a),
    (0x02E85C, i_02e85c),
    (0x02E860, i_02e860),
    (0x02E864, i_02e864),
    (0x02E86A, i_02e86a),
    (0x02E86E, i_02e86e),
    (0x02E872, i_02e872),
    (0x02E876, i_02e876),
    (0x02E87A, i_02e87a),
    (0x02E880, i_02e880),
    (0x02E884, i_02e884),
    (0x02E888, i_02e888),
    (0x02E88E, i_02e88e),
    (0x02E894, i_02e894),
    (0x02E898, i_02e898),
    (0x02E89C, i_02e89c),
    (0x02E8A0, i_02e8a0),
    (0x02E8A2, i_02e8a2),
    (0x02E8A6, i_02e8a6),
    (0x02E8AA, i_02e8aa),
    (0x02E8B0, i_02e8b0),
    (0x02E8B4, i_02e8b4),
    (0x02E8B8, i_02e8b8),
    (0x02E8BA, i_02e8ba),
    (0x02E8BE, i_02e8be),
    (0x02E8C2, i_02e8c2),
    (0x02E8C6, i_02e8c6),
    (0x02E8C8, i_02e8c8),
    (0x02E8CC, i_02e8cc),
    (0x02E8D0, i_02e8d0),
    (0x02E8D4, i_02e8d4),
    (0x02E8D6, i_02e8d6),
    (0x02E8DA, i_02e8da),
    (0x02E8DE, i_02e8de),
    (0x02E8E2, i_02e8e2),
    (0x02E8E4, i_02e8e4),
    (0x02E8E8, i_02e8e8),
    (0x02E8EC, i_02e8ec),
    (0x02E8F0, i_02e8f0),
    (0x02E8F2, i_02e8f2),
    (0x02E8F6, i_02e8f6),
    (0x02E8FA, i_02e8fa),
    (0x02E8FE, i_02e8fe),
    (0x02E900, i_02e900),
    (0x02E904, i_02e904),
    (0x02E908, i_02e908),
    (0x02E90C, i_02e90c),
    (0x02E90E, i_02e90e),
    (0x02E912, i_02e912),
    (0x02E916, i_02e916),
    (0x02E91C, i_02e91c),
    (0x02E922, i_02e922),
    (0x02E924, i_02e924),
    (0x02E92A, i_02e92a),
    (0x02E92E, i_02e92e),
    (0x02E934, i_02e934),
    (0x02E936, i_02e936),
    (0x02E938, i_02e938),
    (0x02E93C, i_02e93c),
    (0x02E942, i_02e942),
    (0x02E946, i_02e946),
    (0x02E94A, i_02e94a),
    (0x02E94E, i_02e94e),
    (0x02E954, i_02e954),
    (0x02E956, i_02e956),
    (0x02E958, i_02e958),
    (0x02E95C, i_02e95c),
    (0x02E960, i_02e960),
    (0x02E962, i_02e962),
    (0x02E966, i_02e966),
    (0x02E96C, i_02e96c),
    (0x02E970, i_02e970),
    (0x02E976, i_02e976),
    (0x02E97C, i_02e97c),
    (0x02E97E, i_02e97e),
    (0x02E982, i_02e982),
    (0x02E986, i_02e986),
    (0x02E98A, i_02e98a),
    (0x02E98C, i_02e98c),
    (0x02E98E, i_02e98e),
    (0x02E992, i_02e992),
    (0x02E994, i_02e994),
    (0x02E996, i_02e996),
    (0x02E99A, i_02e99a),
    (0x02E99E, i_02e99e),
    (0x02E9A4, i_02e9a4),
    (0x02E9A8, i_02e9a8),
    (0x02E9AC, i_02e9ac),
    (0x02E9AE, i_02e9ae),
    (0x02E9B0, i_02e9b0),
    (0x02E9B4, i_02e9b4),
    (0x02E9B6, i_02e9b6),
    (0x02E9B8, i_02e9b8),
    (0x02E9BC, i_02e9bc),
    (0x02E9C0, i_02e9c0),
    (0x02E9C4, i_02e9c4),
    (0x02E9C8, i_02e9c8),
    (0x02E9CE, i_02e9ce),
    (0x02E9D4, i_02e9d4),
    (0x02E9DA, i_02e9da),
    (0x02E9DE, i_02e9de),
    (0x02E9E4, i_02e9e4),
    (0x02E9E6, i_02e9e6),
    (0x02E9EA, i_02e9ea),
    (0x02E9F0, i_02e9f0),
    (0x02E9F4, i_02e9f4),
    (0x02E9FA, i_02e9fa),
    (0x02E9FE, i_02e9fe),
    (0x02EA02, i_02ea02),
    (0x02EA06, i_02ea06),
    (0x02EA0C, i_02ea0c),
    (0x02EA12, i_02ea12),
    (0x02EA16, i_02ea16),
    (0x02EA18, i_02ea18),
    (0x02EA1C, i_02ea1c),
    (0x02EA20, i_02ea20),
    (0x02EA24, i_02ea24),
    (0x02EA26, i_02ea26),
    (0x02EA2A, i_02ea2a),
    (0x02EA2E, i_02ea2e),
    (0x02EA32, i_02ea32),
    (0x02EA36, i_02ea36),
    (0x02EA3A, i_02ea3a),
    (0x02EA3E, i_02ea3e),
    (0x02EA44, i_02ea44),
    (0x02EA48, i_02ea48),
    (0x02EA4C, i_02ea4c),
    (0x02EA52, i_02ea52),
    (0x02EA58, i_02ea58),
    (0x02EA5C, i_02ea5c),
    (0x02EA60, i_02ea60),
    (0x02EA64, i_02ea64),
    (0x02EA66, i_02ea66),
    (0x02EA6A, i_02ea6a),
    (0x02EA6E, i_02ea6e),
    (0x02EA74, i_02ea74),
    (0x02EA78, i_02ea78),
    (0x02EA7C, i_02ea7c),
    (0x02EA80, i_02ea80),
    (0x02EA88, i_02ea88),
    (0x02EA8C, i_02ea8c),
    (0x02EA8E, i_02ea8e),
    (0x02EA92, i_02ea92),
    (0x02EA96, i_02ea96),
    (0x02EA9A, i_02ea9a),
    (0x02EA9C, i_02ea9c),
    (0x02EAA0, i_02eaa0),
    (0x02EAA4, i_02eaa4),
    (0x02EAA8, i_02eaa8),
    (0x02EAAA, i_02eaaa),
    (0x02EAAE, i_02eaae),
    (0x02EAB2, i_02eab2),
    (0x02EAB6, i_02eab6),
    (0x02EAB8, i_02eab8),
    (0x02EABC, i_02eabc),
    (0x02EAC0, i_02eac0),
    (0x02EAC4, i_02eac4),
    (0x02EAC6, i_02eac6),
    (0x02EACA, i_02eaca),
    (0x02EACE, i_02eace),
    (0x02EAD2, i_02ead2),
    (0x02EAD4, i_02ead4),
    (0x02EAD8, i_02ead8),
    (0x02EADC, i_02eadc),
    (0x02EAE0, i_02eae0),
    (0x02EAE2, i_02eae2),
    (0x02EAE6, i_02eae6),
    (0x02EAEA, i_02eaea),
    (0x02EAEE, i_02eaee),
    (0x02EAF0, i_02eaf0),
    (0x02EAF4, i_02eaf4),
    (0x02EAF8, i_02eaf8),
    (0x02EAFC, i_02eafc),
    (0x02EAFE, i_02eafe),
    (0x02EB02, i_02eb02),
    (0x02EB06, i_02eb06),
    (0x02EB0C, i_02eb0c),
    (0x02EB12, i_02eb12),
    (0x02EB14, i_02eb14),
    (0x02EB1A, i_02eb1a),
    (0x02EB20, i_02eb20),
    (0x02EB24, i_02eb24),
    (0x02EB26, i_02eb26),
    (0x02EB28, i_02eb28),
    (0x02EB2C, i_02eb2c),
    (0x02EB32, i_02eb32),
    (0x02EB36, i_02eb36),
    (0x02EB3A, i_02eb3a),
    (0x02EB3E, i_02eb3e),
    (0x02EB44, i_02eb44),
    (0x02EB46, i_02eb46),
    (0x02EB48, i_02eb48),
    (0x02EB4C, i_02eb4c),
    (0x02EB50, i_02eb50),
    (0x02EB52, i_02eb52),
    (0x02EB56, i_02eb56),
    (0x02EB5C, i_02eb5c),
    (0x02EB60, i_02eb60),
    (0x02EB66, i_02eb66),
    (0x02EB6C, i_02eb6c),
    (0x02EB6E, i_02eb6e),
    (0x02EB72, i_02eb72),
    (0x02EB76, i_02eb76),
    (0x02EB7A, i_02eb7a),
    (0x02EB7C, i_02eb7c),
    (0x02EB7E, i_02eb7e),
    (0x02EB82, i_02eb82),
    (0x02EB84, i_02eb84),
    (0x02EB86, i_02eb86),
    (0x02EB8A, i_02eb8a),
    (0x02EB8E, i_02eb8e),
    (0x02EB94, i_02eb94),
    (0x02EB98, i_02eb98),
    (0x02EB9C, i_02eb9c),
    (0x02EB9E, i_02eb9e),
    (0x02EBA0, i_02eba0),
    (0x02EBA4, i_02eba4),
    (0x02EBA6, i_02eba6),
    (0x02EBA8, i_02eba8),
    (0x02EBAC, i_02ebac),
    (0x02EBB0, i_02ebb0),
    (0x02EBB6, i_02ebb6),
    (0x02EBBA, i_02ebba),
    (0x02EBBE, i_02ebbe),
    (0x02EBC2, i_02ebc2),
    (0x02EBC8, i_02ebc8),
    (0x02EBCA, i_02ebca),
    (0x02EBCE, i_02ebce),
    (0x02EBD4, i_02ebd4),
    (0x02EBD8, i_02ebd8),
    (0x02EBDE, i_02ebde),
    (0x02EBE2, i_02ebe2),
    (0x02EBE8, i_02ebe8),
    (0x02EBEC, i_02ebec),
    (0x02EBEE, i_02ebee),
    (0x02EBF2, i_02ebf2),
    (0x02EBFA, i_02ebfa),
    (0x02EBFE, i_02ebfe),
    (0x02EC00, i_02ec00),
    (0x02EC04, i_02ec04),
    (0x02EC0A, i_02ec0a),
    (0x02EC10, i_02ec10),
    (0x02EC14, i_02ec14),
    (0x02EC18, i_02ec18),
    (0x02EC1C, i_02ec1c),
    (0x02EC1E, i_02ec1e),
    (0x02EC22, i_02ec22),
    (0x02EC26, i_02ec26),
    (0x02EC2A, i_02ec2a),
    (0x02EC2C, i_02ec2c),
    (0x02EC30, i_02ec30),
    (0x02EC34, i_02ec34),
    (0x02EC38, i_02ec38),
    (0x02EC3C, i_02ec3c),
    (0x02EC40, i_02ec40),
    (0x02EC42, i_02ec42),
    (0x02EC46, i_02ec46),
    (0x02EC4A, i_02ec4a),
    (0x02EC4E, i_02ec4e),
    (0x02EC50, i_02ec50),
    (0x02EC56, i_02ec56),
    (0x02EC5C, i_02ec5c),
    (0x02EC60, i_02ec60),
    (0x02EC64, i_02ec64),
    (0x02EC66, i_02ec66),
    (0x02EC6A, i_02ec6a),
    (0x02EC6E, i_02ec6e),
    (0x02EC72, i_02ec72),
    (0x02EC74, i_02ec74),
    (0x02EC78, i_02ec78),
    (0x02EC7C, i_02ec7c),
    (0x02EC80, i_02ec80),
    (0x02EC82, i_02ec82),
    (0x02EC86, i_02ec86),
    (0x02EC8A, i_02ec8a),
    (0x02EC8E, i_02ec8e),
    (0x02EC90, i_02ec90),
    (0x02EC94, i_02ec94),
    (0x02EC98, i_02ec98),
    (0x02EC9E, i_02ec9e),
    (0x02ECA4, i_02eca4),
    (0x02ECA6, i_02eca6),
    (0x02ECAA, i_02ecaa),
    (0x02ECB0, i_02ecb0),
    (0x02ECC2, i_02ecc2),
    (0x02ECC6, i_02ecc6),
    (0x02ECCA, i_02ecca),
    (0x02ECCE, i_02ecce),
    (0x02ECD0, i_02ecd0),
    (0x02ECD2, i_02ecd2),
    (0x02ECD4, i_02ecd4),
    (0x02ECD8, i_02ecd8),
    (0x02ECDC, i_02ecdc),
    (0x02ECE0, i_02ece0),
    (0x02ECE4, i_02ece4),
    (0x02ECE8, i_02ece8),
    (0x02ECEC, i_02ecec),
    (0x02ECF2, i_02ecf2),
    (0x02ECF4, i_02ecf4),
    (0x02ECF8, i_02ecf8),
    (0x02ECFE, i_02ecfe),
    (0x02ED02, i_02ed02),
    (0x02ED08, i_02ed08),
    (0x02ED0C, i_02ed0c),
    (0x02ED10, i_02ed10),
    (0x02ED16, i_02ed16),
    (0x02ED1A, i_02ed1a),
    (0x02ED1C, i_02ed1c),
    (0x02ED22, i_02ed22),
    (0x02ED26, i_02ed26),
    (0x02ED2A, i_02ed2a),
    (0x02ED30, i_02ed30),
    (0x02ED34, i_02ed34),
    (0x02ED38, i_02ed38),
    (0x02ED3C, i_02ed3c),
    (0x02ED40, i_02ed40),
    (0x02ED46, i_02ed46),
    (0x02ED4A, i_02ed4a),
    (0x02ED4E, i_02ed4e),
    (0x02ED52, i_02ed52),
    (0x02ED58, i_02ed58),
    (0x02ED60, i_02ed60),
    (0x02ED64, i_02ed64),
    (0x02ED68, i_02ed68),
    (0x02ED6C, i_02ed6c),
    (0x02ED70, i_02ed70),
    (0x02ED72, i_02ed72),
    (0x02ED74, i_02ed74),
    (0x02ED78, i_02ed78),
    (0x02ED7A, i_02ed7a),
    (0x02ED7C, i_02ed7c),
    (0x02ED7E, i_02ed7e),
    (0x02ED82, i_02ed82),
    (0x02ED86, i_02ed86),
    (0x02ED8C, i_02ed8c),
    (0x02ED90, i_02ed90),
    (0x02ED94, i_02ed94),
    (0x02ED98, i_02ed98),
    (0x02ED9C, i_02ed9c),
    (0x02EDA0, i_02eda0),
    (0x02EDA2, i_02eda2),
    (0x02EDA4, i_02eda4),
    (0x02EDA6, i_02eda6),
    (0x02EDAA, i_02edaa),
    (0x02EDAE, i_02edae),
    (0x02EDB2, i_02edb2),
    (0x02EDB6, i_02edb6),
    (0x02EDBA, i_02edba),
    (0x02EDBE, i_02edbe),
    (0x02EDC4, i_02edc4),
    (0x02EDC6, i_02edc6),
    (0x02EDCA, i_02edca),
    (0x02EDD0, i_02edd0),
    (0x02EDD4, i_02edd4),
    (0x02EDDA, i_02edda),
    (0x02EDDE, i_02edde),
    (0x02EDE2, i_02ede2),
    (0x02EDE8, i_02ede8),
    (0x02EDEE, i_02edee),
    (0x02EDF2, i_02edf2),
    (0x02EDF6, i_02edf6),
    (0x02EDFA, i_02edfa),
    (0x02EDFE, i_02edfe),
    (0x02EE00, i_02ee00),
    (0x02EE04, i_02ee04),
    (0x02EE08, i_02ee08),
    (0x02EE0C, i_02ee0c),
    (0x02EE0E, i_02ee0e),
    (0x02EE12, i_02ee12),
    (0x02EE16, i_02ee16),
    (0x02EE1A, i_02ee1a),
    (0x02EE1C, i_02ee1c),
    (0x02EE20, i_02ee20),
    (0x02EE24, i_02ee24),
    (0x02EE2A, i_02ee2a),
    (0x02EE2E, i_02ee2e),
    (0x02EE32, i_02ee32),
    (0x02EE36, i_02ee36),
    (0x02EE38, i_02ee38),
    (0x02EE3C, i_02ee3c),
    (0x02EE40, i_02ee40),
    (0x02EE44, i_02ee44),
    (0x02EE46, i_02ee46),
    (0x02EE4A, i_02ee4a),
    (0x02EE4E, i_02ee4e),
    (0x02EE52, i_02ee52),
    (0x02EE54, i_02ee54),
    (0x02EE58, i_02ee58),
    (0x02EE5C, i_02ee5c),
    (0x02EE60, i_02ee60),
    (0x02EE62, i_02ee62),
    (0x02EE66, i_02ee66),
    (0x02EE6A, i_02ee6a),
    (0x02EE70, i_02ee70),
    (0x02EE76, i_02ee76),
    (0x02EE78, i_02ee78),
    (0x02EE7E, i_02ee7e),
    (0x02EE82, i_02ee82),
    (0x02EE86, i_02ee86),
    (0x02EE88, i_02ee88),
    (0x02EE8C, i_02ee8c),
    (0x02EE92, i_02ee92),
    (0x02EE96, i_02ee96),
    (0x02EE9C, i_02ee9c),
    (0x02EE9E, i_02ee9e),
    (0x02EEA2, i_02eea2),
    (0x02EEA8, i_02eea8),
    (0x02EEAC, i_02eeac),
    (0x02EEB2, i_02eeb2),
    (0x02EEB4, i_02eeb4),
    (0x02EEB8, i_02eeb8),
    (0x02EEBC, i_02eebc),
    (0x02EEC0, i_02eec0),
    (0x02EEC4, i_02eec4),
    (0x02EECA, i_02eeca),
    (0x02EECE, i_02eece),
    (0x02EED4, i_02eed4),
    (0x02EED8, i_02eed8),
    (0x02EEDA, i_02eeda),
    (0x02EEDE, i_02eede),
    (0x02EEE2, i_02eee2),
    (0x02EEE4, i_02eee4),
    (0x02EEE8, i_02eee8),
    (0x02EEEC, i_02eeec),
    (0x02EEF2, i_02eef2),
    (0x02EEF6, i_02eef6),
    (0x02EEFA, i_02eefa),
    (0x02EEFC, i_02eefc),
    (0x02EF00, i_02ef00),
    (0x02EF06, i_02ef06),
    (0x02EF0C, i_02ef0c),
    (0x02EF10, i_02ef10),
    (0x02EF14, i_02ef14),
    (0x02EF18, i_02ef18),
    (0x02EF1A, i_02ef1a),
    (0x02EF1E, i_02ef1e),
    (0x02EF22, i_02ef22),
    (0x02EF26, i_02ef26),
    (0x02EF28, i_02ef28),
    (0x02EF2E, i_02ef2e),
    (0x02EF32, i_02ef32),
    (0x02EF36, i_02ef36),
    (0x02EF38, i_02ef38),
    (0x02EF3E, i_02ef3e),
    (0x02EF42, i_02ef42),
    (0x02EF46, i_02ef46),
    (0x02EF48, i_02ef48),
    (0x02EF4E, i_02ef4e),
    (0x02EF52, i_02ef52),
    (0x02EF56, i_02ef56),
    (0x02EF5A, i_02ef5a),
    (0x02EF5C, i_02ef5c),
    (0x02EF60, i_02ef60),
    (0x02EF64, i_02ef64),
    (0x02EF68, i_02ef68),
    (0x02EF6A, i_02ef6a),
    (0x02EF6E, i_02ef6e),
    (0x02EF72, i_02ef72),
    (0x02EF76, i_02ef76),
    (0x02EF78, i_02ef78),
    (0x02EF7C, i_02ef7c),
    (0x02EF80, i_02ef80),
    (0x02EF84, i_02ef84),
    (0x02EF86, i_02ef86),
    (0x02EF8A, i_02ef8a),
    (0x02EF8E, i_02ef8e),
    (0x02EF92, i_02ef92),
    (0x02EF94, i_02ef94),
    (0x02EF98, i_02ef98),
    (0x02EF9C, i_02ef9c),
    (0x02EFA2, i_02efa2),
    (0x02EFA8, i_02efa8),
    (0x02EFAA, i_02efaa),
    (0x02EFB0, i_02efb0),
    (0x02EFB4, i_02efb4),
    (0x02EFB8, i_02efb8),
    (0x02EFBA, i_02efba),
    (0x02EFBE, i_02efbe),
    (0x02EFC4, i_02efc4),
    (0x02EFC8, i_02efc8),
    (0x02EFCE, i_02efce),
    (0x02EFD0, i_02efd0),
    (0x02EFD4, i_02efd4),
    (0x02EFDA, i_02efda),
    (0x02EFDE, i_02efde),
    (0x02EFE4, i_02efe4),
    (0x02EFE6, i_02efe6),
    (0x02EFEA, i_02efea),
    (0x02EFEE, i_02efee),
    (0x02EFF2, i_02eff2),
    (0x02EFF6, i_02eff6),
    (0x02EFFC, i_02effc),
    (0x02F000, i_02f000),
    (0x02F006, i_02f006),
    (0x02F00A, i_02f00a),
    (0x02F00C, i_02f00c),
    (0x02F010, i_02f010),
    (0x02F014, i_02f014),
    (0x02F016, i_02f016),
    (0x02F01A, i_02f01a),
    (0x02F01E, i_02f01e),
    (0x02F024, i_02f024),
    (0x02F028, i_02f028),
    (0x02F02C, i_02f02c),
    (0x02F02E, i_02f02e),
    (0x02F032, i_02f032),
    (0x02F038, i_02f038),
    (0x02F03E, i_02f03e),
    (0x02F042, i_02f042),
    (0x02F046, i_02f046),
    (0x02F04A, i_02f04a),
    (0x02F04C, i_02f04c),
    (0x02F050, i_02f050),
    (0x02F054, i_02f054),
    (0x02F058, i_02f058),
    (0x02F05A, i_02f05a),
    (0x02F060, i_02f060),
    (0x02F064, i_02f064),
    (0x02F068, i_02f068),
    (0x02F06A, i_02f06a),
    (0x02F070, i_02f070),
    (0x02F074, i_02f074),
    (0x02F078, i_02f078),
    (0x02F07A, i_02f07a),
    (0x02F080, i_02f080),
    (0x02F084, i_02f084),
    (0x02F088, i_02f088),
    (0x02F08C, i_02f08c),
    (0x02F08E, i_02f08e),
    (0x02F092, i_02f092),
    (0x02F096, i_02f096),
    (0x02F09A, i_02f09a),
    (0x02F09C, i_02f09c),
    (0x02F0A0, i_02f0a0),
    (0x02F0A4, i_02f0a4),
    (0x02F0A8, i_02f0a8),
    (0x02F0AA, i_02f0aa),
    (0x02F0AE, i_02f0ae),
    (0x02F0B2, i_02f0b2),
    (0x02F0B6, i_02f0b6),
    (0x02F0B8, i_02f0b8),
    (0x02F0BC, i_02f0bc),
    (0x02F0C0, i_02f0c0),
    (0x02F0C4, i_02f0c4),
    (0x02F0C6, i_02f0c6),
    (0x02F0CA, i_02f0ca),
    (0x02F0CE, i_02f0ce),
    (0x02F0D4, i_02f0d4),
    (0x02F0DA, i_02f0da),
    (0x02F0DC, i_02f0dc),
    (0x02F0E2, i_02f0e2),
    (0x02F102, i_02f102),
    (0x02F106, i_02f106),
    (0x02F10C, i_02f10c),
    (0x02F10E, i_02f10e),
    (0x02F112, i_02f112),
    (0x02F118, i_02f118),
    (0x02F11C, i_02f11c),
    (0x02F122, i_02f122),
    (0x02F124, i_02f124),
    (0x02F128, i_02f128),
    (0x02F12C, i_02f12c),
    (0x02F130, i_02f130),
    (0x02F134, i_02f134),
    (0x02F13A, i_02f13a),
    (0x02F13E, i_02f13e),
    (0x02F144, i_02f144),
    (0x02F148, i_02f148),
    (0x02F14A, i_02f14a),
    (0x02F14E, i_02f14e),
];
