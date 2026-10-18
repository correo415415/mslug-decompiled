// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $089D9E: jsr $28cd4.l
pub fn i_089d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89DA4);
    cpu.pc = 0x28CD4;
    16
}
/// $089DA4: bra.w $89dc2
pub fn i_089da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x89DC2;
    10
}
/// $089DC2: move.w #$d7, d1
pub fn i_089dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DC6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089DC6: jsr $236e.l
pub fn i_089dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89DCC);
    cpu.pc = 0x236E;
    16
}
/// $089DCC: move.w #$50, $70(a6)
pub fn i_089dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089DD2: move.b #$ff, $32(a6)
pub fn i_089dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DD8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089DD8: move.b #$ff, $33(a6)
pub fn i_089dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DDE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089DDE: move.b #$0, $3a(a6)
pub fn i_089dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DE4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089DE4: move.w #$8000, $38(a6)
pub fn i_089de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089DEA: jsr $267e2.l
pub fn i_089dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89DF0);
    cpu.pc = 0x267E2;
    16
}
/// $089DF0: jsr $27cee.l
pub fn i_089df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89DF6);
    cpu.pc = 0x27CEE;
    16
}
/// $089DF6: lea.l $89dfc(pc), a1
pub fn i_089df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89DFC;
    16
}
/// $089DFA: move.l a1, (a6)
pub fn i_089dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89DFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089DFC: jsr $2783a.l
pub fn i_089dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E02);
    cpu.pc = 0x2783A;
    16
}
/// $089E02: jsr $28d70.l
pub fn i_089e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E08);
    cpu.pc = 0x28D70;
    16
}
/// $089E08: bclr.b #$3, $13(a6)
pub fn i_089e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E0E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $089E0E: bclr.b #$0, $13(a6)
pub fn i_089e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E14;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $089E14: move.w #$7fff, $66(a6)
pub fn i_089e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E1A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089E1A: jsr $4fa70.l
pub fn i_089e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E20);
    cpu.pc = 0x4FA70;
    16
}
/// $089E20: bcc.w $89e2a
pub fn i_089e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89E2A; return 10; }
    8
}
/// $089E24: jmp $518.l
pub fn i_089e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E2A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089E2A: rts
pub fn i_089e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_PropStatic_C_089e2c  ($089E2C, 98 B, enemies/bosses/scene5_airship_088a64.s)
/// $089E2C: movea.l $3c(a6), a1
pub fn i_089e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E30;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $089E30: jsr $2942a.l
pub fn i_089e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E36);
    cpu.pc = 0x2942A;
    16
}
/// $089E36: lea.l $2ee188.l, a0
pub fn i_089e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E3C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE188;
    20
}
/// $089E3C: jsr $28cd4.l
pub fn i_089e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E42);
    cpu.pc = 0x28CD4;
    16
}
/// $089E42: move.w #$d7, d1
pub fn i_089e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E46;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089E46: jsr $236e.l
pub fn i_089e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E4C);
    cpu.pc = 0x236E;
    16
}
/// $089E4C: move.w #$90, $70(a6)
pub fn i_089e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089E52: move.b #$ff, $32(a6)
pub fn i_089e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E58;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089E58: move.b #$ff, $33(a6)
pub fn i_089e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E5E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089E5E: move.b #$0, $3a(a6)
pub fn i_089e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E64;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089E64: move.w #$0, $38(a6)
pub fn i_089e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089E6A: lea.l $89e70(pc), a1
pub fn i_089e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89E70;
    16
}
/// $089E6E: move.l a1, (a6)
pub fn i_089e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089E70: jsr $2783a.l
pub fn i_089e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E76);
    cpu.pc = 0x2783A;
    16
}
/// $089E76: jsr $28d70.l
pub fn i_089e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E7C);
    cpu.pc = 0x28D70;
    16
}
/// $089E7C: jsr $4fa70.l
pub fn i_089e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E82);
    cpu.pc = 0x4FA70;
    16
}
/// $089E82: bcc.w $89e8c
pub fn i_089e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89E8C; return 10; }
    8
}
/// $089E86: jmp $518.l
pub fn i_089e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E8C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089E8C: rts
pub fn i_089e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_BarrelRow_Spawn_089e8e  ($089E8E, 184 B, enemies/bosses/scene5_airship_088a64.s)
/// $089E8E: movea.l $3c(a6), a1
pub fn i_089e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E92;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $089E92: jsr $2942a.l
pub fn i_089e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89E98);
    cpu.pc = 0x2942A;
    16
}
/// $089E98: move.b #$0, $21(a6)
pub fn i_089e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089E9E: lea.l $89f46(pc), a1
pub fn i_089e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89F46;
    16
}
/// $089EA2: jsr $4ae.l
pub fn i_089ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89EA8);
    cpu.pc = 0x4AE;
    16
}
/// $089EA8: jsr $5dd22.l
pub fn i_089ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89EAE);
    cpu.pc = 0x5DD22;
    16
}
/// $089EAE: addi.w #$40, $22(a0)
pub fn i_089eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EB4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089EB4: addi.w #$80, $24(a0)
pub fn i_089eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EBA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089EBA: lea.l $89f46(pc), a1
pub fn i_089eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89F46;
    16
}
/// $089EBE: jsr $4ae.l
pub fn i_089ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89EC4);
    cpu.pc = 0x4AE;
    16
}
/// $089EC4: jsr $5dd22.l
pub fn i_089ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89ECA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89ECA);
    cpu.pc = 0x5DD22;
    16
}
/// $089ECA: addi.w #$90, $22(a0)
pub fn i_089eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89ED0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089ED0: addi.w #$80, $24(a0)
pub fn i_089ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89ED6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089ED6: lea.l $89f46(pc), a1
pub fn i_089ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89F46;
    16
}
/// $089EDA: jsr $4ae.l
pub fn i_089eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89EE0);
    cpu.pc = 0x4AE;
    16
}
/// $089EE0: jsr $5dd22.l
pub fn i_089ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89EE6);
    cpu.pc = 0x5DD22;
    16
}
/// $089EE6: addi.w #$20, $22(a0)
pub fn i_089ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EEC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089EEC: lea.l $89f46(pc), a1
pub fn i_089eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89F46;
    16
}
/// $089EF0: jsr $4ae.l
pub fn i_089ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89EF6);
    cpu.pc = 0x4AE;
    16
}
/// $089EF6: jsr $5dd22.l
pub fn i_089ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89EFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89EFC);
    cpu.pc = 0x5DD22;
    16
}
/// $089EFC: addi.w #$80, $22(a0)
pub fn i_089efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F02;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089F02: lea.l $89f46(pc), a1
pub fn i_089f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89F46;
    16
}
/// $089F06: jsr $4ae.l
pub fn i_089f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89F0C);
    cpu.pc = 0x4AE;
    16
}
/// $089F0C: jsr $5dd22.l
pub fn i_089f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89F12);
    cpu.pc = 0x5DD22;
    16
}
/// $089F12: addi.w #$e0, $22(a0)
pub fn i_089f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F18;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089F18: lea.l $89f1e(pc), a1
pub fn i_089f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89F1E;
    16
}
/// $089F1C: move.l a1, (a6)
pub fn i_089f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089F1E: clr.b $10e39a.l
pub fn i_089f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F24;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $089F24: jsr $2783a.l
pub fn i_089f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89F2A);
    cpu.pc = 0x2783A;
    16
}
/// $089F2A: cmpi.b #$ff, $21(a6)
pub fn i_089f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F30;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $089F30: bne.w $89f44
pub fn i_089f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x89F44; return 10; }
    8
}
/// $089F34: move.w #$1028, d0
pub fn i_089f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F38;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089F38: jsr $2352.l
pub fn i_089f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89F3E);
    cpu.pc = 0x2352;
    16
}
/// $089F3E: jmp $518.l
pub fn i_089f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F44;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089F44: rts
pub fn i_089f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Barrel_089f46  ($089F46, 192 B, enemies/bosses/scene5_airship_088a64.s)
/// $089F46: jsr $2783a.l
pub fn i_089f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89F4C);
    cpu.pc = 0x2783A;
    16
}
/// $089F4C: cmpi.w #$140, $22(a6)
pub fn i_089f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F52;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $089F52: ble.w $89f58
pub fn i_089f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F56;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x89F58; return 10; }
    8
}
/// $089F56: rts
pub fn i_089f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $089F58: move.w #$cd, d1
pub fn i_089f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F5C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089F5C: jsr $236e.l
pub fn i_089f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89F62);
    cpu.pc = 0x236E;
    16
}
/// $089F62: move.w #$20, $70(a6)
pub fn i_089f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089F68: move.b #$ff, $32(a6)
pub fn i_089f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089F6E: move.b #$ff, $33(a6)
pub fn i_089f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089F74: move.b #$0, $3a(a6)
pub fn i_089f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089F7A: move.w #$1, $66(a6)
pub fn i_089f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089F80: move.w #$0, $38(a6)
pub fn i_089f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F86;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089F86: lea.l $2ee808.l, a0
pub fn i_089f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE808;
    20
}
/// $089F8C: move.l a0, $48(a6)
pub fn i_089f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F90;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $089F90: lea.l $89f96(pc), a1
pub fn i_089f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89F96;
    16
}
/// $089F94: move.l a1, (a6)
pub fn i_089f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089F96: jsr $2783a.l
pub fn i_089f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89F9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89F9C);
    cpu.pc = 0x2783A;
    16
}
/// $089F9C: movea.l $c(a6), a0
pub fn i_089f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FA0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089FA0: cmpi.b #$ff, $21(a0)
pub fn i_089fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FA6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $089FA6: beq.w $89fe6
pub fn i_089fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89FE6; return 10; }
    8
}
/// $089FAA: jsr $2870a.l
pub fn i_089faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89FB0);
    cpu.pc = 0x2870A;
    16
}
/// $089FB0: bcc.w $89fd2
pub fn i_089fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89FD2; return 10; }
    8
}
/// $089FB4: lea.l $5e766.l, a0
pub fn i_089fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $089FBA: jsr $5e770.l
pub fn i_089fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89FC0);
    cpu.pc = 0x5E770;
    16
}
/// $089FC0: lea.l $2edec2.l, a0
pub fn i_089fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDEC2;
    20
}
/// $089FC6: jsr $28cd4.l
pub fn i_089fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89FCC);
    cpu.pc = 0x28CD4;
    16
}
/// $089FCC: bclr.b #$3, $13(a6)
pub fn i_089fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FD2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $089FD2: jsr $28758.l
pub fn i_089fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89FD8);
    cpu.pc = 0x28758;
    16
}
/// $089FD8: bcc.w $8a004
pub fn i_089fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A004; return 10; }
    8
}
/// $089FDC: movea.l $c(a6), a0
pub fn i_089fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FE0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089FE0: move.b #$ff, $21(a0)
pub fn i_089fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FE6;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089FE6: lea.l $2ef1be.l, a1
pub fn i_089fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FEC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF1BE;
    20
}
/// $089FEC: jsr $77c7e.l
pub fn i_089fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89FF2);
    cpu.pc = 0x77C7E;
    16
}
/// $089FF2: lea.l $2ef1e2.l, a1
pub fn i_089ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FF8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF1E2;
    20
}
/// $089FF8: jsr $77c7e.l
pub fn i_089ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89FFE);
    cpu.pc = 0x77C7E;
    16
}
/// $089FFE: jmp $518.l
pub fn i_089ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A004;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08A004: rts
pub fn i_08a004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A006;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_RockRain_Spawn_08a006  ($08A006, 88 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A006: movea.l $3c(a6), a1
pub fn i_08a006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A00A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08A00A: jsr $2942a.l
pub fn i_08a00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A010);
    cpu.pc = 0x2942A;
    16
}
/// $08A010: lea.l $8a016(pc), a1
pub fn i_08a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A014;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A016;
    16
}
/// $08A014: move.l a1, (a6)
pub fn i_08a014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A016;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A016: cmpi.w #$40, $106f54.l
pub fn i_08a016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A01E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x40, Size::Word, false);
    20
}
/// $08A01E: bgt.w $8a024
pub fn i_08a01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A022;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8A024; return 10; }
    8
}
/// $08A022: rts
pub fn i_08a022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A024;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08A024: move.w #$f, $72(a6)
pub fn i_08a024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A02A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A02A: lea.l $8a030(pc), a1
pub fn i_08a02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A02E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A030;
    16
}
/// $08A02E: move.l a1, (a6)
pub fn i_08a02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A030;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A030: subi.w #$1, $72(a6)
pub fn i_08a030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A036;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A036: bne.w $8a04a
pub fn i_08a036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A03A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8A04A; return 10; }
    8
}
/// $08A03A: lea.l $8a05e(pc), a1
pub fn i_08a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A03E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A05E;
    16
}
/// $08A03E: jsr $4ae.l
pub fn i_08a03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A044);
    cpu.pc = 0x4AE;
    16
}
/// $08A044: move.w #$f, $72(a6)
pub fn i_08a044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A04A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A04A: cmpi.w #$180, $106f54.l
pub fn i_08a04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A052;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x180, Size::Word, false);
    20
}
/// $08A052: ble.w $8a05c
pub fn i_08a052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A056;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8A05C; return 10; }
    8
}
/// $08A056: jmp $518.l
pub fn i_08a056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A05C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08A05C: rts
pub fn i_08a05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A05E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_FallingRock_08a05e  ($08A05E, 174 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A05E: move.w #$ce, d1
pub fn i_08a05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A062;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A062: jsr $236e.l
pub fn i_08a062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A068;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A068);
    cpu.pc = 0x236E;
    16
}
/// $08A068: move.b #$ff, $32(a6)
pub fn i_08a068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A06E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A06E: move.b #$ff, $33(a6)
pub fn i_08a06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A074;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A074: move.b #$0, $3a(a6)
pub fn i_08a074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A07A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A07A: move.w #$c000, $38(a6)
pub fn i_08a07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A080;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A080: move.w #$0, $22(a6)
pub fn i_08a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A086;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A086: move.w #$a0, $24(a6)
pub fn i_08a086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A08C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A08C: jsr $267e2.l
pub fn i_08a08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A092);
    cpu.pc = 0x267E2;
    16
}
/// $08A092: move.w #$f000, $38(a6)
pub fn i_08a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A098;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A098: bset.b #$6, $12(a6)
pub fn i_08a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A09E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A09E: jsr $5e9b6.l
pub fn i_08a09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A0A4);
    cpu.pc = 0x5E9B6;
    16
}
/// $08A0A4: andi.w #$ff, d0
pub fn i_08a0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0A8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08A0A8: add.w d0, $22(a6)
pub fn i_08a0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0AC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08A0AC: andi.w #$3, d0
pub fn i_08a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0B0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08A0B0: move.w d0, $2a(a6)
pub fn i_08a0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0B4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A0B4: addi.w #$10, $2a(a6)
pub fn i_08a0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0BA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A0BA: movem.w d0, -(a7)
pub fn i_08a0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0BE;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $08A0BE: movea.l #$2ee1c4, a0
pub fn i_08a0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0C4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE1C4;
    cpu.a[0] = v1;
    4
}
/// $08A0C4: lsl.w #$2, d0
pub fn i_08a0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0C6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08A0C6: movea.l (a0, d0.w), a0
pub fn i_08a0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0CA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08A0CA: cmpa.l #$ffffffff, a0
pub fn i_08a0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0D0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08A0D0: beq.w $8a0da
pub fn i_08a0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A0DA; return 10; }
    8
}
/// $08A0D4: jsr $28cd4.l
pub fn i_08a0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A0DA);
    cpu.pc = 0x28CD4;
    16
}
/// $08A0DA: movem.w (a7)+, d0
pub fn i_08a0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0DE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08A0DE: andi.b #$1, d0
pub fn i_08a0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0E2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08A0E2: or.b d0, $3a(a6)
pub fn i_08a0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0E6;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08A0E6: lea.l $8a0ec(pc), a1
pub fn i_08a0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A0EC;
    16
}
/// $08A0EA: move.l a1, (a6)
pub fn i_08a0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A0EC: move.w $2a(a6), d0
pub fn i_08a0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0F0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08A0F0: add.w d0, $24(a6)
pub fn i_08a0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0F4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08A0F4: jsr $28d70.l
pub fn i_08a0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A0FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A0FA);
    cpu.pc = 0x28D70;
    16
}
/// $08A0FA: cmpi.w #$200, $24(a6)
pub fn i_08a0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A100;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x200, Size::Word, false);
    16
}
/// $08A100: ble.w $8a10a
pub fn i_08a100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A104;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8A10A; return 10; }
    8
}
/// $08A104: jmp $518.l
pub fn i_08a104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A10A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08A10A: rts
pub fn i_08a10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A10C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerA_Init_08a10c  ($08A10C, 472 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A10C: movea.l $3c(a6), a1
pub fn i_08a10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A110;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08A110: jsr $2942a.l
pub fn i_08a110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A116);
    cpu.pc = 0x2942A;
    16
}
/// $08A116: move.w #$1dc, d1
pub fn i_08a116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A11A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A11A: jsr $236e.l
pub fn i_08a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A120);
    cpu.pc = 0x236E;
    16
}
/// $08A120: move.w #$d3, d1
pub fn i_08a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A124;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A124: jsr $236e.l
pub fn i_08a124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A12A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A12A);
    cpu.pc = 0x236E;
    16
}
/// $08A12A: move.b #$ff, $32(a6)
pub fn i_08a12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A130;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A130: move.b #$ff, $33(a6)
pub fn i_08a130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A136;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A136: move.b #$0, $3a(a6)
pub fn i_08a136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A13C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A13C: move.b #$0, $20(a6)
pub fn i_08a13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A142;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A142: move.w #$7fff, $66(a6)
pub fn i_08a142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A148;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A148: move.w #$c000, $38(a6)
pub fn i_08a148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A14E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A14E: cmpi.b #$c, $106ece.l
pub fn i_08a14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A156;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08A156: beq.w $8a170
pub fn i_08a156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A15A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A170; return 10; }
    8
}
/// $08A15A: lea.l $f0612.l, a1
pub fn i_08a15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A160;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xF0612;
    20
}
/// $08A160: move.w #$82, d0
pub fn i_08a160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A164;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A164: move.b #$0, $82(a6)
pub fn i_08a164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A16A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A16A: jsr $4429e.l
pub fn i_08a16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A170);
    cpu.pc = 0x4429E;
    16
}
/// $08A170: lea.l $8ae56(pc), a1
pub fn i_08a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A174;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A174: jsr $4ae.l
pub fn i_08a174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A17A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A17A);
    cpu.pc = 0x4AE;
    16
}
/// $08A17A: jsr $5dd22.l
pub fn i_08a17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A180;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A180);
    cpu.pc = 0x5DD22;
    16
}
/// $08A180: move.b #$0, $21(a0)
pub fn i_08a180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A186;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A186: addi.w #$20, $22(a0)
pub fn i_08a186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A18C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A18C: lea.l $2ef3ac.l, a1
pub fn i_08a18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A192;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF3AC;
    20
}
/// $08A192: move.l a1, $70(a0)
pub fn i_08a192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A196;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A196: move.l #$ffffffff, $74(a0)
pub fn i_08a196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A19E;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A19E: lea.l $8ae56(pc), a1
pub fn i_08a19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A1A2: jsr $4ae.l
pub fn i_08a1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A1A8);
    cpu.pc = 0x4AE;
    16
}
/// $08A1A8: jsr $5dd22.l
pub fn i_08a1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A1AE);
    cpu.pc = 0x5DD22;
    16
}
/// $08A1AE: move.b #$0, $21(a0)
pub fn i_08a1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1B4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A1B4: addi.w #$60, $22(a0)
pub fn i_08a1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1BA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A1BA: lea.l $2ef3c0.l, a1
pub fn i_08a1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF3C0;
    20
}
/// $08A1C0: move.l a1, $70(a0)
pub fn i_08a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1C4;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A1C4: move.l #$ffffffff, $74(a0)
pub fn i_08a1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1CC;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A1CC: lea.l $8ae56(pc), a1
pub fn i_08a1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A1D0: jsr $4ae.l
pub fn i_08a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A1D6);
    cpu.pc = 0x4AE;
    16
}
/// $08A1D6: jsr $5dd22.l
pub fn i_08a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A1DC);
    cpu.pc = 0x5DD22;
    16
}
/// $08A1DC: move.b #$0, $21(a0)
pub fn i_08a1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1E2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A1E2: addi.w #$a0, $22(a0)
pub fn i_08a1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1E8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A1E8: lea.l $2ef3d4.l, a1
pub fn i_08a1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1EE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF3D4;
    20
}
/// $08A1EE: move.l a1, $70(a0)
pub fn i_08a1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1F2;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A1F2: move.l #$ffffffff, $74(a0)
pub fn i_08a1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A1FA;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A1FA: move.b #$0, $86(a6)
pub fn i_08a1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A200;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A200: lea.l $77228.l, a1
pub fn i_08a200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A206;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77228;
    20
}
/// $08A206: jsr $4ae.l
pub fn i_08a206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A20C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A20C);
    cpu.pc = 0x4AE;
    16
}
/// $08A20C: jsr $5dd22.l
pub fn i_08a20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A212);
    cpu.pc = 0x5DD22;
    16
}
/// $08A212: addi.w #$ec, $22(a0)
pub fn i_08a212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A218;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xEC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A218: subi.w #$80, $24(a0)
pub fn i_08a218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A21E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A21E: move.b #$80, $98(a0)
pub fn i_08a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A224;
    cpu.ir = 0x117C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A224: move.b #$86, $99(a0)
pub fn i_08a224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A22A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A22A: lea.l $2ee1e8.l, a0
pub fn i_08a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A230;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1E8;
    20
}
/// $08A230: jsr $28cd4.l
pub fn i_08a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A236);
    cpu.pc = 0x28CD4;
    16
}
/// $08A236: lea.l $2eeb50.l, a0
pub fn i_08a236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A23C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEB50;
    20
}
/// $08A23C: move.l a0, $48(a6)
pub fn i_08a23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A240;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A240: lea.l $8a246(pc), a1
pub fn i_08a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A244;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A246;
    16
}
/// $08A244: move.l a1, (a6)
pub fn i_08a244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A246;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A246: clr.b $10e39a.l
pub fn i_08a246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A24C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08A24C: jsr $2783a.l
pub fn i_08a24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A252);
    cpu.pc = 0x2783A;
    16
}
/// $08A252: cmpi.w #$b0, $22(a6)
pub fn i_08a252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A258;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB0, Size::Word, false);
    16
}
/// $08A258: bgt.w $8a282
pub fn i_08a258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A25C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8A282; return 10; }
    8
}
/// $08A25C: jsr $28d70.l
pub fn i_08a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A262;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A262);
    cpu.pc = 0x28D70;
    16
}
/// $08A262: bcc.w $8a282
pub fn i_08a262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A266;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A282; return 10; }
    8
}
/// $08A266: jsr $2870a.l
pub fn i_08a266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A26C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A26C);
    cpu.pc = 0x2870A;
    16
}
/// $08A26C: bcc.w $8a282
pub fn i_08a26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A270;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A282; return 10; }
    8
}
/// $08A270: lea.l $2ee1d4.l, a0
pub fn i_08a270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A276;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1D4;
    20
}
/// $08A276: jsr $28cd4.l
pub fn i_08a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A27C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A27C);
    cpu.pc = 0x28CD4;
    16
}
/// $08A27C: jsr $28d70.l
pub fn i_08a27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A282;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A282);
    cpu.pc = 0x28D70;
    16
}
/// $08A282: jsr $2870a.l
pub fn i_08a282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A288;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A288);
    cpu.pc = 0x2870A;
    16
}
/// $08A288: bcc.w $8a31a
pub fn i_08a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A28C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A31A; return 10; }
    8
}
/// $08A28C: bclr.b #$3, $13(a6)
pub fn i_08a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A292;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A292: cmpi.b #$1, $58(a6)
pub fn i_08a292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A298;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08A298: beq.w $8a31a
pub fn i_08a298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A29C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A31A; return 10; }
    8
}
/// $08A29C: cmpi.b #$c, $106ece.l
pub fn i_08a29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2A4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08A2A4: bne.w $8a2ec
pub fn i_08a2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8A2EC; return 10; }
    8
}
/// $08A2A8: lea.l $2ef6a4.l, a1
pub fn i_08a2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2AE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6A4;
    20
}
/// $08A2AE: jsr $43fac.l
pub fn i_08a2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A2B4);
    cpu.pc = 0x43FAC;
    16
}
/// $08A2B4: lea.l $2ef6c4.l, a1
pub fn i_08a2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6C4;
    20
}
/// $08A2BA: jsr $43fac.l
pub fn i_08a2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A2C0);
    cpu.pc = 0x43FAC;
    16
}
/// $08A2C0: lea.l $2ef6e4.l, a1
pub fn i_08a2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6E4;
    20
}
/// $08A2C6: jsr $43fac.l
pub fn i_08a2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A2CC);
    cpu.pc = 0x43FAC;
    16
}
/// $08A2CC: lea.l $2ef410.l, a2
pub fn i_08a2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2D2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF410;
    20
}
/// $08A2D2: jsr $5022a.l
pub fn i_08a2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A2D8);
    cpu.pc = 0x5022A;
    16
}
/// $08A2D8: lea.l $2ef474.l, a2
pub fn i_08a2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF474;
    20
}
/// $08A2DE: jsr $5022a.l
pub fn i_08a2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A2E4);
    cpu.pc = 0x5022A;
    16
}
// ===== SetTaskHandler_08a2e4  ($08A2E4, 8 B, kernel/task_handlers.c)
/// $08A2E4: lea.l $8a5c8(pc), a1
pub fn i_08a2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A5C8;
    16
}
/// $08A2E8: move.l a1, (a6)
pub fn i_08a2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A2EA: rts
pub fn i_08a2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerA_Hit_08a2ec  ($08A2EC, 40 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A2EC: move.w #$1035, d0
pub fn i_08a2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2F0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A2F0: jsr $2352.l
pub fn i_08a2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A2F6);
    cpu.pc = 0x2352;
    16
}
/// $08A2F6: lea.l $7808a.l, a1
pub fn i_08a2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2FC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A2FC: jsr $4ae.l
pub fn i_08a2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A302);
    cpu.pc = 0x4AE;
    16
}
/// $08A302: jsr $5dd22.l
pub fn i_08a302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A308);
    cpu.pc = 0x5DD22;
    16
}
/// $08A308: addi.w #$c0, $22(a0)
pub fn i_08a308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A30E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A30E: subi.w #$80, $24(a0)
pub fn i_08a30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A314;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08a314  ($08A314, 8 B, kernel/task_handlers.c)
/// $08A314: lea.l $8a31c(pc), a1
pub fn i_08a314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A318;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A31C;
    16
}
/// $08A318: move.l a1, (a6)
pub fn i_08a318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A31A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A31A: rts
pub fn i_08a31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A31C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerA_Stage2_08a31c  ($08A31C, 296 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A31C: jsr $434dc.l
pub fn i_08a31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A322);
    cpu.pc = 0x434DC;
    16
}
/// $08A322: jsr $2783a.l
pub fn i_08a322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A328);
    cpu.pc = 0x2783A;
    16
}
/// $08A328: lea.l $2ef3e8.l, a2
pub fn i_08a328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A32E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF3E8;
    20
}
/// $08A32E: jsr $5022a.l
pub fn i_08a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A334;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A334);
    cpu.pc = 0x5022A;
    16
}
/// $08A334: lea.l $2ef44c.l, a2
pub fn i_08a334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A33A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF44C;
    20
}
/// $08A33A: jsr $5022a.l
pub fn i_08a33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A340);
    cpu.pc = 0x5022A;
    16
}
/// $08A340: lea.l $2ef6a4.l, a1
pub fn i_08a340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A346;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6A4;
    20
}
/// $08A346: jsr $43fac.l
pub fn i_08a346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A34C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A34C);
    cpu.pc = 0x43FAC;
    16
}
/// $08A34C: lea.l $2ef286.l, a1
pub fn i_08a34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A352;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08A352: jsr $77c7e.l
pub fn i_08a352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A358);
    cpu.pc = 0x77C7E;
    16
}
/// $08A358: lea.l $2eeba4.l, a0
pub fn i_08a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A35E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEBA4;
    20
}
/// $08A35E: move.l a0, $48(a6)
pub fn i_08a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A362;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A362: lea.l $8a368(pc), a1
pub fn i_08a362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A366;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A368;
    16
}
/// $08A366: move.l a1, (a6)
pub fn i_08a366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A368;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A368: move.l $106f50.l, d0
pub fn i_08a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A36E;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08A36E: swap d0
pub fn i_08a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A370;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08A370: cmpi.w #$7d0, d0
pub fn i_08a370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A374;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7D0, Size::Word, false);
    8
}
/// $08A374: bge.w $8a3ae
pub fn i_08a374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A378;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8A3AE; return 10; }
    8
}
/// $08A378: clr.b $10e39a.l
pub fn i_08a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A37E;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08A37E: jsr $2783a.l
pub fn i_08a37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A384);
    cpu.pc = 0x2783A;
    16
}
/// $08A384: jsr $28d70.l
pub fn i_08a384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A38A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A38A);
    cpu.pc = 0x28D70;
    16
}
/// $08A38A: jsr $2870a.l
pub fn i_08a38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A390);
    cpu.pc = 0x2870A;
    16
}
/// $08A390: bcc.w $8a3a0
pub fn i_08a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A394;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A3A0; return 10; }
    8
}
/// $08A394: lea.l $5e766.l, a0
pub fn i_08a394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A39A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08A39A: jsr $5e770.l
pub fn i_08a39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3A0);
    cpu.pc = 0x5E770;
    16
}
/// $08A3A0: bclr.b #$3, $13(a6)
pub fn i_08a3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A3A6: bclr.b #$0, $13(a6)
pub fn i_08a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3AC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A3AC: rts
pub fn i_08a3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08A3AE: lea.l $2c06aa.l, a0
pub fn i_08a3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C06AA;
    20
}
/// $08A3B4: jsr $799de.l
pub fn i_08a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3BA);
    cpu.pc = 0x799DE;
    16
}
/// $08A3BA: move.w d0, $66(a6)
pub fn i_08a3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3BE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A3BE: lea.l $8a3c4(pc), a1
pub fn i_08a3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A3C4;
    16
}
/// $08A3C2: move.l a1, (a6)
pub fn i_08a3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A3C4: clr.b $10e39a.l
pub fn i_08a3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3CA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08A3CA: jsr $2783a.l
pub fn i_08a3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3D0);
    cpu.pc = 0x2783A;
    16
}
/// $08A3D0: jsr $28d70.l
pub fn i_08a3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3D6);
    cpu.pc = 0x28D70;
    16
}
/// $08A3D6: bcc.w $8a3f6
pub fn i_08a3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A3F6; return 10; }
    8
}
/// $08A3DA: jsr $2870a.l
pub fn i_08a3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3E0);
    cpu.pc = 0x2870A;
    16
}
/// $08A3E0: bcc.w $8a3f6
pub fn i_08a3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A3F6; return 10; }
    8
}
/// $08A3E4: lea.l $2ee1d4.l, a0
pub fn i_08a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1D4;
    20
}
/// $08A3EA: jsr $28cd4.l
pub fn i_08a3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3F0);
    cpu.pc = 0x28CD4;
    16
}
/// $08A3F0: jsr $28d70.l
pub fn i_08a3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3F6);
    cpu.pc = 0x28D70;
    16
}
/// $08A3F6: jsr $2870a.l
pub fn i_08a3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A3FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A3FC);
    cpu.pc = 0x2870A;
    16
}
/// $08A3FC: bcc.w $8a412
pub fn i_08a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A400;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A412; return 10; }
    8
}
/// $08A400: lea.l $5e766.l, a0
pub fn i_08a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A406;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08A406: jsr $5e770.l
pub fn i_08a406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A40C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A40C);
    cpu.pc = 0x5E770;
    16
}
/// $08A40C: bclr.b #$3, $13(a6)
pub fn i_08a40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A412;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A412: cmpi.w #$29a, $66(a6)
pub fn i_08a412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A418;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29A, Size::Word, false);
    16
}
/// $08A418: bgt.w $8a44a
pub fn i_08a418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A41C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8A44A; return 10; }
    8
}
/// $08A41C: move.w #$1035, d0
pub fn i_08a41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A420;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A420: jsr $2352.l
pub fn i_08a420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A426;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A426);
    cpu.pc = 0x2352;
    16
}
/// $08A426: lea.l $7808a.l, a1
pub fn i_08a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A42C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A42C: jsr $4ae.l
pub fn i_08a42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A432);
    cpu.pc = 0x4AE;
    16
}
/// $08A432: jsr $5dd22.l
pub fn i_08a432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A438;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A438);
    cpu.pc = 0x5DD22;
    16
}
/// $08A438: addi.w #$c0, $22(a0)
pub fn i_08a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A43E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A43E: subi.w #$80, $24(a0)
pub fn i_08a43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A444;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08a444  ($08A444, 8 B, kernel/task_handlers.c)
/// $08A444: lea.l $8a44c(pc), a1
pub fn i_08a444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A448;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A44C;
    16
}
/// $08A448: move.l a1, (a6)
pub fn i_08a448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A44A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A44A: rts
pub fn i_08a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A44C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerA_Stage3_08a44c  ($08A44C, 194 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A44C: jsr $434dc.l
pub fn i_08a44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A452);
    cpu.pc = 0x434DC;
    16
}
/// $08A452: jsr $2783a.l
pub fn i_08a452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A458);
    cpu.pc = 0x2783A;
    16
}
/// $08A458: lea.l $2ef3fc.l, a2
pub fn i_08a458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A45E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF3FC;
    20
}
/// $08A45E: jsr $5022a.l
pub fn i_08a45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A464);
    cpu.pc = 0x5022A;
    16
}
/// $08A464: lea.l $2ef460.l, a2
pub fn i_08a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A46A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF460;
    20
}
/// $08A46A: jsr $5022a.l
pub fn i_08a46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A470;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A470);
    cpu.pc = 0x5022A;
    16
}
/// $08A470: lea.l $2ef6c4.l, a1
pub fn i_08a470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A476;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6C4;
    20
}
/// $08A476: jsr $43fac.l
pub fn i_08a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A47C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A47C);
    cpu.pc = 0x43FAC;
    16
}
/// $08A47C: lea.l $2ef286.l, a1
pub fn i_08a47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A482;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08A482: jsr $77c7e.l
pub fn i_08a482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A488);
    cpu.pc = 0x77C7E;
    16
}
/// $08A488: lea.l $8a48e(pc), a1
pub fn i_08a488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A48C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A48E;
    16
}
/// $08A48C: move.l a1, (a6)
pub fn i_08a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A48E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A48E: clr.b $10e39a.l
pub fn i_08a48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A494;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08A494: jsr $2783a.l
pub fn i_08a494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A49A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A49A);
    cpu.pc = 0x2783A;
    16
}
/// $08A49A: jsr $28d70.l
pub fn i_08a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4A0);
    cpu.pc = 0x28D70;
    16
}
/// $08A4A0: bcc.w $8a4c0
pub fn i_08a4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A4C0; return 10; }
    8
}
/// $08A4A4: jsr $2870a.l
pub fn i_08a4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4AA);
    cpu.pc = 0x2870A;
    16
}
/// $08A4AA: bcc.w $8a4c0
pub fn i_08a4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A4C0; return 10; }
    8
}
/// $08A4AE: lea.l $2ee1d4.l, a0
pub fn i_08a4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1D4;
    20
}
/// $08A4B4: jsr $28cd4.l
pub fn i_08a4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4BA);
    cpu.pc = 0x28CD4;
    16
}
/// $08A4BA: jsr $28d70.l
pub fn i_08a4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4C0);
    cpu.pc = 0x28D70;
    16
}
/// $08A4C0: jsr $2870a.l
pub fn i_08a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4C6);
    cpu.pc = 0x2870A;
    16
}
/// $08A4C6: bcc.w $8a4dc
pub fn i_08a4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A4DC; return 10; }
    8
}
/// $08A4CA: lea.l $5e766.l, a0
pub fn i_08a4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08A4D0: jsr $5e770.l
pub fn i_08a4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4D6);
    cpu.pc = 0x5E770;
    16
}
/// $08A4D6: bclr.b #$3, $13(a6)
pub fn i_08a4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4DC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A4DC: cmpi.w #$14d, $66(a6)
pub fn i_08a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14D, Size::Word, false);
    16
}
/// $08A4E2: bgt.w $8a44a
pub fn i_08a4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4E6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8A44A; return 10; }
    8
}
/// $08A4E6: move.w #$1035, d0
pub fn i_08a4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4EA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A4EA: jsr $2352.l
pub fn i_08a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4F0);
    cpu.pc = 0x2352;
    16
}
/// $08A4F0: lea.l $7808a.l, a1
pub fn i_08a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A4F6: jsr $4ae.l
pub fn i_08a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A4FC);
    cpu.pc = 0x4AE;
    16
}
/// $08A4FC: jsr $5dd22.l
pub fn i_08a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A502);
    cpu.pc = 0x5DD22;
    16
}
/// $08A502: addi.w #$c0, $22(a0)
pub fn i_08a502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A508;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A508: subi.w #$80, $24(a0)
pub fn i_08a508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A50E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08a50e  ($08A50E, 8 B, kernel/task_handlers.c)
/// $08A50E: lea.l $8a516(pc), a1
pub fn i_08a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A512;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A516;
    16
}
/// $08A512: move.l a1, (a6)
pub fn i_08a512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A514;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A514: rts
pub fn i_08a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A516;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerA_Stage4_08a516  ($08A516, 178 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A516: jsr $434dc.l
pub fn i_08a516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A51C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A51C);
    cpu.pc = 0x434DC;
    16
}
/// $08A51C: jsr $2783a.l
pub fn i_08a51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A522;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A522);
    cpu.pc = 0x2783A;
    16
}
/// $08A522: lea.l $2ef410.l, a2
pub fn i_08a522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A528;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF410;
    20
}
/// $08A528: jsr $5022a.l
pub fn i_08a528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A52E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A52E);
    cpu.pc = 0x5022A;
    16
}
/// $08A52E: lea.l $2ef474.l, a2
pub fn i_08a52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A534;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF474;
    20
}
/// $08A534: jsr $5022a.l
pub fn i_08a534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A53A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A53A);
    cpu.pc = 0x5022A;
    16
}
/// $08A53A: lea.l $2ef6e4.l, a1
pub fn i_08a53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A540;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6E4;
    20
}
/// $08A540: jsr $43fac.l
pub fn i_08a540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A546;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A546);
    cpu.pc = 0x43FAC;
    16
}
/// $08A546: lea.l $2ee212.l, a0
pub fn i_08a546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A54C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE212;
    20
}
/// $08A54C: jsr $28cd4.l
pub fn i_08a54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A552);
    cpu.pc = 0x28CD4;
    16
}
/// $08A552: lea.l $2ef286.l, a1
pub fn i_08a552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A558;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08A558: jsr $77c7e.l
pub fn i_08a558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A55E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A55E);
    cpu.pc = 0x77C7E;
    16
}
/// $08A55E: lea.l $8a564(pc), a1
pub fn i_08a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A562;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A564;
    16
}
/// $08A562: move.l a1, (a6)
pub fn i_08a562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A564;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A564: clr.b $10e39a.l
pub fn i_08a564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A56A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08A56A: jsr $2783a.l
pub fn i_08a56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A570;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A570);
    cpu.pc = 0x2783A;
    16
}
/// $08A570: jsr $28d70.l
pub fn i_08a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A576;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A576);
    cpu.pc = 0x28D70;
    16
}
/// $08A576: bcc.w $8a596
pub fn i_08a576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A57A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A596; return 10; }
    8
}
/// $08A57A: jsr $2870a.l
pub fn i_08a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A580;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A580);
    cpu.pc = 0x2870A;
    16
}
/// $08A580: bcc.w $8a596
pub fn i_08a580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A584;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A596; return 10; }
    8
}
/// $08A584: lea.l $2ee1fe.l, a0
pub fn i_08a584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A58A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1FE;
    20
}
/// $08A58A: jsr $28cd4.l
pub fn i_08a58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A590);
    cpu.pc = 0x28CD4;
    16
}
/// $08A590: jsr $28d70.l
pub fn i_08a590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A596);
    cpu.pc = 0x28D70;
    16
}
/// $08A596: jsr $2870a.l
pub fn i_08a596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A59C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A59C);
    cpu.pc = 0x2870A;
    16
}
/// $08A59C: bcc.w $8a5b2
pub fn i_08a59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A5B2; return 10; }
    8
}
/// $08A5A0: lea.l $5e766.l, a0
pub fn i_08a5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08A5A6: jsr $5e770.l
pub fn i_08a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A5AC);
    cpu.pc = 0x5E770;
    16
}
/// $08A5AC: bclr.b #$3, $13(a6)
pub fn i_08a5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5B2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A5B2: jsr $28758.l
pub fn i_08a5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A5B8);
    cpu.pc = 0x28758;
    16
}
/// $08A5B8: bcc.w $8a74c
pub fn i_08a5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A74C; return 10; }
    8
}
/// $08A5BC: move.l #$5000, d0
pub fn i_08a5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5C2;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08A5C2: jsr $51a28.l
pub fn i_08a5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A5C8);
    cpu.pc = 0x51A28;
    16
}
// ===== S5_TowerA_Destroy_08a5c8  ($08A5C8, 390 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A5C8: jsr $434ce.l
pub fn i_08a5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A5CE);
    cpu.pc = 0x434CE;
    16
}
/// $08A5CE: move.w #$102f, d0
pub fn i_08a5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A5D2: jsr $2352.l
pub fn i_08a5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A5D8);
    cpu.pc = 0x2352;
    16
}
/// $08A5D8: lea.l $7808a.l, a1
pub fn i_08a5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A5DE: jsr $4ae.l
pub fn i_08a5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A5E4);
    cpu.pc = 0x4AE;
    16
}
/// $08A5E4: jsr $5dd22.l
pub fn i_08a5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A5EA);
    cpu.pc = 0x5DD22;
    16
}
/// $08A5EA: addi.w #$c0, $22(a0)
pub fn i_08a5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5F0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A5F0: subi.w #$80, $24(a0)
pub fn i_08a5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5F6;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A5F6: lea.l $7808a.l, a1
pub fn i_08a5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A5FC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A5FC: jsr $4ae.l
pub fn i_08a5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A602);
    cpu.pc = 0x4AE;
    16
}
/// $08A602: jsr $5dd22.l
pub fn i_08a602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A608);
    cpu.pc = 0x5DD22;
    16
}
/// $08A608: addi.w #$30, $22(a0)
pub fn i_08a608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A60E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A60E: subi.w #$30, $24(a0)
pub fn i_08a60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A614;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A614: lea.l $7808a.l, a1
pub fn i_08a614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A61A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A61A: jsr $4ae.l
pub fn i_08a61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A620);
    cpu.pc = 0x4AE;
    16
}
/// $08A620: jsr $5dd22.l
pub fn i_08a620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A626;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A626);
    cpu.pc = 0x5DD22;
    16
}
/// $08A626: addi.w #$70, $22(a0)
pub fn i_08a626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A62C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A62C: subi.w #$10, $24(a0)
pub fn i_08a62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A632;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A632: lea.l $7808a.l, a1
pub fn i_08a632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A638;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A638: jsr $4ae.l
pub fn i_08a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A63E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A63E);
    cpu.pc = 0x4AE;
    16
}
/// $08A63E: jsr $5dd22.l
pub fn i_08a63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A644);
    cpu.pc = 0x5DD22;
    16
}
/// $08A644: addi.w #$90, $22(a0)
pub fn i_08a644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A64A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A64A: subi.w #$40, $24(a0)
pub fn i_08a64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A650;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A650: lea.l $7808a.l, a1
pub fn i_08a650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A656;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A656: jsr $4ae.l
pub fn i_08a656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A65C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A65C);
    cpu.pc = 0x4AE;
    16
}
/// $08A65C: jsr $5dd22.l
pub fn i_08a65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A662);
    cpu.pc = 0x5DD22;
    16
}
/// $08A662: addi.w #$c0, $22(a0)
pub fn i_08a662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A668;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A668: subi.w #$30, $24(a0)
pub fn i_08a668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A66E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A66E: lea.l $7808a.l, a1
pub fn i_08a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A674;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A674: jsr $4ae.l
pub fn i_08a674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A67A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A67A);
    cpu.pc = 0x4AE;
    16
}
/// $08A67A: jsr $5dd22.l
pub fn i_08a67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A680);
    cpu.pc = 0x5DD22;
    16
}
/// $08A680: addi.w #$100, $22(a0)
pub fn i_08a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A686;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A686: subi.w #$40, $24(a0)
pub fn i_08a686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A68C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A68C: lea.l $2ef286.l, a1
pub fn i_08a68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A692;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08A692: jsr $77c7e.l
pub fn i_08a692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A698);
    cpu.pc = 0x77C7E;
    16
}
/// $08A698: lea.l $2ef438.l, a2
pub fn i_08a698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A69E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF438;
    20
}
/// $08A69E: jsr $5022a.l
pub fn i_08a69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6A4);
    cpu.pc = 0x5022A;
    16
}
/// $08A6A4: lea.l $2ef424.l, a2
pub fn i_08a6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6AA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF424;
    20
}
/// $08A6AA: jsr $5022a.l
pub fn i_08a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6B0);
    cpu.pc = 0x5022A;
    16
}
/// $08A6B0: lea.l $2ef488.l, a2
pub fn i_08a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6B6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF488;
    20
}
/// $08A6B6: jsr $5022a.l
pub fn i_08a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6BC);
    cpu.pc = 0x5022A;
    16
}
/// $08A6BC: lea.l $2ef63e.l, a1
pub fn i_08a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF63E;
    20
}
/// $08A6C2: jsr $43fac.l
pub fn i_08a6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6C8);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6C8: lea.l $2ef66c.l, a1
pub fn i_08a6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF66C;
    20
}
/// $08A6CE: jsr $43fac.l
pub fn i_08a6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6D4);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6D4: lea.l $2ef688.l, a1
pub fn i_08a6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF688;
    20
}
/// $08A6DA: jsr $43fac.l
pub fn i_08a6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6E0);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6E0: lea.l $2ef696.l, a1
pub fn i_08a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF696;
    20
}
/// $08A6E6: jsr $43fac.l
pub fn i_08a6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6EC);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6EC: lea.l $2ef704.l, a1
pub fn i_08a6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6F2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF704;
    20
}
/// $08A6F2: jsr $43fac.l
pub fn i_08a6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6F8);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6F8: move.b #$ff, $82(a6)
pub fn i_08a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A6FE: move.b #$ff, $86(a6)
pub fn i_08a6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A704;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A704: lea.l $8ae0c(pc), a1
pub fn i_08a704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A708;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE0C;
    16
}
/// $08A708: jsr $4ae.l
pub fn i_08a708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A70E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A70E);
    cpu.pc = 0x4AE;
    16
}
/// $08A70E: jsr $5dd22.l
pub fn i_08a70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A714);
    cpu.pc = 0x5DD22;
    16
}
/// $08A714: addi.w #$20, $22(a0)
pub fn i_08a714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A71A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A71A: lea.l $8b07c(pc), a1
pub fn i_08a71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A71E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B07C;
    16
}
/// $08A71E: jsr $4ae.l
pub fn i_08a71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A724);
    cpu.pc = 0x4AE;
    16
}
/// $08A724: jsr $5dd22.l
pub fn i_08a724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A72A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A72A);
    cpu.pc = 0x5DD22;
    16
}
/// $08A72A: addi.w #$90, $22(a0)
pub fn i_08a72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A730;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A730: subi.w #$10, $24(a0)
pub fn i_08a730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A736;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A736: move.b #$0, $21(a0)
pub fn i_08a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A73C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A73C: move.b #$ff, $20(a6)
pub fn i_08a73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A742;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A742: jsr $8b490(pc)
pub fn i_08a742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A746;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8A746);
    cpu.pc = 0x8B490;
    16
}
/// $08A746: jmp $518.l
pub fn i_08a746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A74C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08A74C: rts
pub fn i_08a74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A74E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Init_08a74e  ($08A74E, 554 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A74E: movea.l $3c(a6), a1
pub fn i_08a74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A752;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08A752: jsr $2942a.l
pub fn i_08a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A758);
    cpu.pc = 0x2942A;
    16
}
/// $08A758: move.w #$1dc, d1
pub fn i_08a758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A75C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A75C: jsr $236e.l
pub fn i_08a75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A762;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A762);
    cpu.pc = 0x236E;
    16
}
/// $08A762: move.w #$d3, d1
pub fn i_08a762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A766;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A766: jsr $236e.l
pub fn i_08a766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A76C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A76C);
    cpu.pc = 0x236E;
    16
}
/// $08A76C: move.b #$ff, $32(a6)
pub fn i_08a76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A772;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A772: move.b #$ff, $33(a6)
pub fn i_08a772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A778;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A778: move.b #$0, $3a(a6)
pub fn i_08a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A77E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A77E: move.b #$0, $20(a6)
pub fn i_08a77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A784;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A784: move.w #$7fff, $66(a6)
pub fn i_08a784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A78A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A78A: move.w #$c000, $38(a6)
pub fn i_08a78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A790;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A790: cmpi.b #$c, $106ece.l
pub fn i_08a790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A798;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08A798: beq.w $8a7b2
pub fn i_08a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A79C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A7B2; return 10; }
    8
}
/// $08A79C: lea.l $f0638.l, a1
pub fn i_08a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xF0638;
    20
}
/// $08A7A2: move.w #$82, d0
pub fn i_08a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A7A6: move.b #$0, $82(a6)
pub fn i_08a7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A7AC: jsr $4429e.l
pub fn i_08a7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7B2);
    cpu.pc = 0x4429E;
    16
}
/// $08A7B2: lea.l $8ae56(pc), a1
pub fn i_08a7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A7B6: jsr $4ae.l
pub fn i_08a7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7BC);
    cpu.pc = 0x4AE;
    16
}
/// $08A7BC: jsr $5dd22.l
pub fn i_08a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7C2);
    cpu.pc = 0x5DD22;
    16
}
/// $08A7C2: move.b #$0, $21(a0)
pub fn i_08a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7C8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A7C8: subi.w #$20, $22(a0)
pub fn i_08a7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7CE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A7CE: lea.l $2ef5c8.l, a1
pub fn i_08a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7D4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF5C8;
    20
}
/// $08A7D4: move.l a1, $70(a0)
pub fn i_08a7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7D8;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A7D8: lea.l $2ef5dc.l, a1
pub fn i_08a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF5DC;
    20
}
/// $08A7DE: move.l a1, $74(a0)
pub fn i_08a7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7E2;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A7E2: lea.l $8ae56(pc), a1
pub fn i_08a7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A7E6: jsr $4ae.l
pub fn i_08a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7EC);
    cpu.pc = 0x4AE;
    16
}
/// $08A7EC: jsr $5dd22.l
pub fn i_08a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7F2);
    cpu.pc = 0x5DD22;
    16
}
/// $08A7F2: move.b #$0, $21(a0)
pub fn i_08a7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7F8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A7F8: addi.w #$20, $22(a0)
pub fn i_08a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7FE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A7FE: lea.l $2ef49c.l, a1
pub fn i_08a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A804;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF49C;
    20
}
/// $08A804: move.l a1, $70(a0)
pub fn i_08a804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A808;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A808: lea.l $2ef4b0.l, a1
pub fn i_08a808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A80E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4B0;
    20
}
/// $08A80E: move.l a1, $74(a0)
pub fn i_08a80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A812;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A812: lea.l $8ae56(pc), a1
pub fn i_08a812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A816: jsr $4ae.l
pub fn i_08a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A81C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A81C);
    cpu.pc = 0x4AE;
    16
}
/// $08A81C: jsr $5dd22.l
pub fn i_08a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A822);
    cpu.pc = 0x5DD22;
    16
}
/// $08A822: move.b #$0, $21(a0)
pub fn i_08a822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A828;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A828: addi.w #$60, $22(a0)
pub fn i_08a828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A82E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A82E: lea.l $2ef4c4.l, a1
pub fn i_08a82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A834;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4C4;
    20
}
/// $08A834: move.l a1, $70(a0)
pub fn i_08a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A838;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A838: lea.l $2ef4d8.l, a1
pub fn i_08a838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A83E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4D8;
    20
}
/// $08A83E: move.l a1, $74(a0)
pub fn i_08a83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A842;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A842: lea.l $8ae56(pc), a1
pub fn i_08a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A846;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A846: jsr $4ae.l
pub fn i_08a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A84C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A84C);
    cpu.pc = 0x4AE;
    16
}
/// $08A84C: jsr $5dd22.l
pub fn i_08a84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A852);
    cpu.pc = 0x5DD22;
    16
}
/// $08A852: move.b #$1, $21(a0)
pub fn i_08a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A858;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A858: addi.w #$a0, $22(a0)
pub fn i_08a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A85E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A85E: lea.l $2ef4ec.l, a1
pub fn i_08a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A864;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4EC;
    20
}
/// $08A864: move.l a1, $70(a0)
pub fn i_08a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A868;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A868: lea.l $2ef500.l, a1
pub fn i_08a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A86E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF500;
    20
}
/// $08A86E: move.l a1, $74(a0)
pub fn i_08a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A872;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A872: move.b #$0, $86(a6)
pub fn i_08a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A878;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A878: lea.l $77228.l, a1
pub fn i_08a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A87E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77228;
    20
}
/// $08A87E: jsr $4ae.l
pub fn i_08a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A884);
    cpu.pc = 0x4AE;
    16
}
/// $08A884: jsr $5dd22.l
pub fn i_08a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A88A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A88A);
    cpu.pc = 0x5DD22;
    16
}
/// $08A88A: addi.w #$110, $22(a0)
pub fn i_08a88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A890;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x110, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A890: subi.w #$48, $24(a0)
pub fn i_08a890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A896;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A896: move.b #$49, $98(a0)
pub fn i_08a896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A89C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x49;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A89C: move.b #$86, $99(a0)
pub fn i_08a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8A2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A8A2: lea.l $2ee23c.l, a0
pub fn i_08a8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE23C;
    20
}
/// $08A8A8: jsr $28cd4.l
pub fn i_08a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8AE);
    cpu.pc = 0x28CD4;
    16
}
/// $08A8AE: lea.l $2eeb50.l, a0
pub fn i_08a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEB50;
    20
}
/// $08A8B4: move.l a0, $48(a6)
pub fn i_08a8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8B8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A8B8: move.b #$0, $87(a6)
pub fn i_08a8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A8BE: lea.l $8a8c4(pc), a1
pub fn i_08a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A8C4;
    16
}
/// $08A8C2: move.l a1, (a6)
pub fn i_08a8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A8C4: clr.b $10e39a.l
pub fn i_08a8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8CA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08A8CA: jsr $2783a.l
pub fn i_08a8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8D0);
    cpu.pc = 0x2783A;
    16
}
/// $08A8D0: cmpi.w #$b0, $22(a6)
pub fn i_08a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB0, Size::Word, false);
    16
}
/// $08A8D6: bgt.w $8a90c
pub fn i_08a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8DA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8A90C; return 10; }
    8
}
/// $08A8DA: move.b #$0, $87(a6)
pub fn i_08a8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A8E0: jsr $28d70.l
pub fn i_08a8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8E6);
    cpu.pc = 0x28D70;
    16
}
/// $08A8E6: bcc.w $8a90c
pub fn i_08a8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A90C; return 10; }
    8
}
/// $08A8EA: jsr $2870a.l
pub fn i_08a8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8F0);
    cpu.pc = 0x2870A;
    16
}
/// $08A8F0: bcc.w $8a90c
pub fn i_08a8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A90C; return 10; }
    8
}
/// $08A8F4: lea.l $2ee228.l, a0
pub fn i_08a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE228;
    20
}
/// $08A8FA: jsr $28cd4.l
pub fn i_08a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A900);
    cpu.pc = 0x28CD4;
    16
}
/// $08A900: move.b #$ff, $87(a6)
pub fn i_08a900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A906;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A906: jsr $28d70.l
pub fn i_08a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A90C);
    cpu.pc = 0x28D70;
    16
}
/// $08A90C: jsr $2870a.l
pub fn i_08a90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A912);
    cpu.pc = 0x2870A;
    16
}
/// $08A912: bcc.w $8a9ae
pub fn i_08a912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A916;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A9AE; return 10; }
    8
}
/// $08A916: bclr.b #$3, $13(a6)
pub fn i_08a916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A91C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A91C: cmpi.b #$1, $58(a6)
pub fn i_08a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A922;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08A922: beq.w $8a9ae
pub fn i_08a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A926;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A9AE; return 10; }
    8
}
/// $08A926: cmpi.b #$2, $58(a6)
pub fn i_08a926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A92C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08A92C: beq.w $8a9ae
pub fn i_08a92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A930;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A9AE; return 10; }
    8
}
/// $08A930: cmpi.b #$c, $106ece.l
pub fn i_08a930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A938;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08A938: bne.w $8a980
pub fn i_08a938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A93C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8A980; return 10; }
    8
}
/// $08A93C: lea.l $2ef6a4.l, a1
pub fn i_08a93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A942;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6A4;
    20
}
/// $08A942: jsr $43fac.l
pub fn i_08a942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A948;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A948);
    cpu.pc = 0x43FAC;
    16
}
/// $08A948: lea.l $2ef6c4.l, a1
pub fn i_08a948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A94E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6C4;
    20
}
/// $08A94E: jsr $43fac.l
pub fn i_08a94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A954);
    cpu.pc = 0x43FAC;
    16
}
/// $08A954: lea.l $2ef6e4.l, a1
pub fn i_08a954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A95A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6E4;
    20
}
/// $08A95A: jsr $43fac.l
pub fn i_08a95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A960);
    cpu.pc = 0x43FAC;
    16
}
/// $08A960: lea.l $2ef53c.l, a2
pub fn i_08a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A966;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF53C;
    20
}
/// $08A966: jsr $5022a.l
pub fn i_08a966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A96C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A96C);
    cpu.pc = 0x5022A;
    16
}
/// $08A96C: lea.l $2ef5a0.l, a2
pub fn i_08a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A972;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF5A0;
    20
}
/// $08A972: jsr $5022a.l
pub fn i_08a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A978);
    cpu.pc = 0x5022A;
    16
}
// ===== SetTaskHandler_08a978  ($08A978, 8 B, kernel/task_handlers.c)
/// $08A978: lea.l $8ac92(pc), a1
pub fn i_08a978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A97C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AC92;
    16
}
/// $08A97C: move.l a1, (a6)
pub fn i_08a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A97E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A97E: rts
pub fn i_08a97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A980;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Hit_08a980  ($08A980, 40 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A980: move.w #$1035, d0
pub fn i_08a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A984;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A984: jsr $2352.l
pub fn i_08a984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A98A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A98A);
    cpu.pc = 0x2352;
    16
}
/// $08A98A: lea.l $7808a.l, a1
pub fn i_08a98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A990;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A990: jsr $4ae.l
pub fn i_08a990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A996);
    cpu.pc = 0x4AE;
    16
}
/// $08A996: jsr $5dd22.l
pub fn i_08a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A99C);
    cpu.pc = 0x5DD22;
    16
}
/// $08A99C: addi.w #$c0, $22(a0)
pub fn i_08a99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9A2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A9A2: subi.w #$80, $24(a0)
pub fn i_08a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9A8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08a9a8  ($08A9A8, 8 B, kernel/task_handlers.c)
/// $08A9A8: lea.l $8a9b0(pc), a1
pub fn i_08a9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A9B0;
    16
}
/// $08A9AC: move.l a1, (a6)
pub fn i_08a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A9AE: rts
pub fn i_08a9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Stage2_08a9b0  ($08A9B0, 314 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A9B0: jsr $434dc.l
pub fn i_08a9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9B6);
    cpu.pc = 0x434DC;
    16
}
/// $08A9B6: jsr $2783a.l
pub fn i_08a9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9BC);
    cpu.pc = 0x2783A;
    16
}
/// $08A9BC: lea.l $2ef514.l, a2
pub fn i_08a9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9C2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF514;
    20
}
/// $08A9C2: jsr $5022a.l
pub fn i_08a9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9C8);
    cpu.pc = 0x5022A;
    16
}
/// $08A9C8: lea.l $2ef578.l, a2
pub fn i_08a9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9CE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF578;
    20
}
/// $08A9CE: jsr $5022a.l
pub fn i_08a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9D4);
    cpu.pc = 0x5022A;
    16
}
/// $08A9D4: lea.l $2ef6a4.l, a1
pub fn i_08a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6A4;
    20
}
/// $08A9DA: jsr $43fac.l
pub fn i_08a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9E0);
    cpu.pc = 0x43FAC;
    16
}
/// $08A9E0: lea.l $2ef286.l, a1
pub fn i_08a9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08A9E6: jsr $77c7e.l
pub fn i_08a9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9EC);
    cpu.pc = 0x77C7E;
    16
}
/// $08A9EC: lea.l $2eeba4.l, a0
pub fn i_08a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEBA4;
    20
}
/// $08A9F2: move.l a0, $48(a6)
pub fn i_08a9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9F6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A9F6: move.b #$0, $87(a6)
pub fn i_08a9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A9FC: lea.l $8aa02(pc), a1
pub fn i_08a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AA02;
    16
}
/// $08AA00: move.l a1, (a6)
pub fn i_08aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AA02: move.l $106f50.l, d0
pub fn i_08aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA08;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08AA08: swap d0
pub fn i_08aa08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA0A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08AA0A: cmpi.w #$950, d0
pub fn i_08aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA0E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x950, Size::Word, false);
    8
}
/// $08AA0E: bge.w $8aa48
pub fn i_08aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA12;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8AA48; return 10; }
    8
}
/// $08AA12: clr.b $10e39a.l
pub fn i_08aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA18;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AA18: jsr $2783a.l
pub fn i_08aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA1E);
    cpu.pc = 0x2783A;
    16
}
/// $08AA1E: jsr $28d70.l
pub fn i_08aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA24);
    cpu.pc = 0x28D70;
    16
}
/// $08AA24: jsr $2870a.l
pub fn i_08aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA2A);
    cpu.pc = 0x2870A;
    16
}
/// $08AA2A: bcc.w $8aa3a
pub fn i_08aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AA3A; return 10; }
    8
}
/// $08AA2E: lea.l $5e766.l, a0
pub fn i_08aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AA34: jsr $5e770.l
pub fn i_08aa34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA3A);
    cpu.pc = 0x5E770;
    16
}
/// $08AA3A: bclr.b #$3, $13(a6)
pub fn i_08aa3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA40;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AA40: bclr.b #$0, $13(a6)
pub fn i_08aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA46;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AA46: rts
pub fn i_08aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08AA48: lea.l $2c06aa.l, a0
pub fn i_08aa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C06AA;
    20
}
/// $08AA4E: jsr $799de.l
pub fn i_08aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA54);
    cpu.pc = 0x799DE;
    16
}
/// $08AA54: move.w d0, $66(a6)
pub fn i_08aa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA58;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AA58: lea.l $8aa5e(pc), a1
pub fn i_08aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AA5E;
    16
}
/// $08AA5C: move.l a1, (a6)
pub fn i_08aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AA5E: clr.b $10e39a.l
pub fn i_08aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA64;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AA64: jsr $2783a.l
pub fn i_08aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA6A);
    cpu.pc = 0x2783A;
    16
}
/// $08AA6A: move.b #$0, $87(a6)
pub fn i_08aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA70;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AA70: jsr $28d70.l
pub fn i_08aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA76);
    cpu.pc = 0x28D70;
    16
}
/// $08AA76: bcc.w $8aa9c
pub fn i_08aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AA9C; return 10; }
    8
}
/// $08AA7A: jsr $2870a.l
pub fn i_08aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA80);
    cpu.pc = 0x2870A;
    16
}
/// $08AA80: bcc.w $8aa9c
pub fn i_08aa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AA9C; return 10; }
    8
}
/// $08AA84: lea.l $2ee228.l, a0
pub fn i_08aa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE228;
    20
}
/// $08AA8A: jsr $28cd4.l
pub fn i_08aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA90);
    cpu.pc = 0x28CD4;
    16
}
/// $08AA90: move.b #$ff, $87(a6)
pub fn i_08aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AA96: jsr $28d70.l
pub fn i_08aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA9C);
    cpu.pc = 0x28D70;
    16
}
/// $08AA9C: jsr $2870a.l
pub fn i_08aa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAA2);
    cpu.pc = 0x2870A;
    16
}
/// $08AAA2: bcc.w $8aab8
pub fn i_08aaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AAB8; return 10; }
    8
}
/// $08AAA6: lea.l $5e766.l, a0
pub fn i_08aaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AAAC: jsr $5e770.l
pub fn i_08aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAB2);
    cpu.pc = 0x5E770;
    16
}
/// $08AAB2: bclr.b #$3, $13(a6)
pub fn i_08aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAB8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AAB8: cmpi.w #$29a, $66(a6)
pub fn i_08aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AABE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29A, Size::Word, false);
    16
}
/// $08AABE: bgt.w $8aaf0
pub fn i_08aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAC2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8AAF0; return 10; }
    8
}
/// $08AAC2: move.w #$1035, d0
pub fn i_08aac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AAC6: jsr $2352.l
pub fn i_08aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AACC);
    cpu.pc = 0x2352;
    16
}
/// $08AACC: lea.l $7808a.l, a1
pub fn i_08aacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAD2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AAD2: jsr $4ae.l
pub fn i_08aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAD8);
    cpu.pc = 0x4AE;
    16
}
/// $08AAD8: jsr $5dd22.l
pub fn i_08aad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AADE);
    cpu.pc = 0x5DD22;
    16
}
/// $08AADE: addi.w #$c0, $22(a0)
pub fn i_08aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAE4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AAE4: subi.w #$80, $24(a0)
pub fn i_08aae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAEA;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08aaea  ($08AAEA, 8 B, kernel/task_handlers.c)
/// $08AAEA: lea.l $8aaf2(pc), a1
pub fn i_08aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AAF2;
    16
}
/// $08AAEE: move.l a1, (a6)
pub fn i_08aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AAF0: rts
pub fn i_08aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Stage3_08aaf2  ($08AAF2, 218 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AAF2: jsr $434dc.l
pub fn i_08aaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAF8);
    cpu.pc = 0x434DC;
    16
}
/// $08AAF8: jsr $2783a.l
pub fn i_08aaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAFE);
    cpu.pc = 0x2783A;
    16
}
/// $08AAFE: lea.l $2ef528.l, a2
pub fn i_08aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB04;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF528;
    20
}
/// $08AB04: jsr $5022a.l
pub fn i_08ab04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB0A);
    cpu.pc = 0x5022A;
    16
}
/// $08AB0A: lea.l $2ef58c.l, a2
pub fn i_08ab0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF58C;
    20
}
/// $08AB10: jsr $5022a.l
pub fn i_08ab10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB16);
    cpu.pc = 0x5022A;
    16
}
/// $08AB16: lea.l $2ef6c4.l, a1
pub fn i_08ab16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB1C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6C4;
    20
}
/// $08AB1C: jsr $43fac.l
pub fn i_08ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB22);
    cpu.pc = 0x43FAC;
    16
}
/// $08AB22: lea.l $2ef286.l, a1
pub fn i_08ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB28;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08AB28: jsr $77c7e.l
pub fn i_08ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB2E);
    cpu.pc = 0x77C7E;
    16
}
/// $08AB2E: lea.l $2ee266.l, a0
pub fn i_08ab2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE266;
    20
}
/// $08AB34: jsr $28cd4.l
pub fn i_08ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB3A);
    cpu.pc = 0x28CD4;
    16
}
/// $08AB3A: lea.l $8ab40(pc), a1
pub fn i_08ab3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AB40;
    16
}
/// $08AB3E: move.l a1, (a6)
pub fn i_08ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AB40: clr.b $10e39a.l
pub fn i_08ab40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB46;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AB46: jsr $2783a.l
pub fn i_08ab46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB4C);
    cpu.pc = 0x2783A;
    16
}
/// $08AB4C: move.b #$0, $87(a6)
pub fn i_08ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AB52: jsr $28d70.l
pub fn i_08ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB58);
    cpu.pc = 0x28D70;
    16
}
/// $08AB58: bcc.w $8ab7e
pub fn i_08ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AB7E; return 10; }
    8
}
/// $08AB5C: jsr $2870a.l
pub fn i_08ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB62);
    cpu.pc = 0x2870A;
    16
}
/// $08AB62: bcc.w $8ab7e
pub fn i_08ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AB7E; return 10; }
    8
}
/// $08AB66: lea.l $2ee252.l, a0
pub fn i_08ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE252;
    20
}
/// $08AB6C: jsr $28cd4.l
pub fn i_08ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB72);
    cpu.pc = 0x28CD4;
    16
}
/// $08AB72: move.b #$ff, $87(a6)
pub fn i_08ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AB78: jsr $28d70.l
pub fn i_08ab78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB7E);
    cpu.pc = 0x28D70;
    16
}
/// $08AB7E: jsr $2870a.l
pub fn i_08ab7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB84);
    cpu.pc = 0x2870A;
    16
}
/// $08AB84: bcc.w $8ab9a
pub fn i_08ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AB9A; return 10; }
    8
}
/// $08AB88: lea.l $5e766.l, a0
pub fn i_08ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AB8E: jsr $5e770.l
pub fn i_08ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB94);
    cpu.pc = 0x5E770;
    16
}
/// $08AB94: bclr.b #$3, $13(a6)
pub fn i_08ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB9A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AB9A: cmpi.w #$14d, $66(a6)
pub fn i_08ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABA0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14D, Size::Word, false);
    16
}
/// $08ABA0: bgt.w $8aaf0
pub fn i_08aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABA4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8AAF0; return 10; }
    8
}
/// $08ABA4: move.w #$1035, d0
pub fn i_08aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ABA8: jsr $2352.l
pub fn i_08aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABAE);
    cpu.pc = 0x2352;
    16
}
/// $08ABAE: lea.l $7808a.l, a1
pub fn i_08abae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABB4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ABB4: jsr $4ae.l
pub fn i_08abb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABBA);
    cpu.pc = 0x4AE;
    16
}
/// $08ABBA: jsr $5dd22.l
pub fn i_08abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABC0);
    cpu.pc = 0x5DD22;
    16
}
/// $08ABC0: addi.w #$c0, $22(a0)
pub fn i_08abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABC6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ABC6: subi.w #$80, $24(a0)
pub fn i_08abc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABCC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08abcc  ($08ABCC, 8 B, kernel/task_handlers.c)
/// $08ABCC: lea.l $8abd4(pc), a1
pub fn i_08abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ABD4;
    16
}
/// $08ABD0: move.l a1, (a6)
pub fn i_08abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ABD2: rts
pub fn i_08abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Stage4_08abd4  ($08ABD4, 190 B, enemies/bosses/scene5_airship_088a64.s)
/// $08ABD4: jsr $434dc.l
pub fn i_08abd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABDA);
    cpu.pc = 0x434DC;
    16
}
/// $08ABDA: jsr $2783a.l
pub fn i_08abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABE0);
    cpu.pc = 0x2783A;
    16
}
/// $08ABE0: lea.l $2ef53c.l, a2
pub fn i_08abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABE6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF53C;
    20
}
/// $08ABE6: jsr $5022a.l
pub fn i_08abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABEC);
    cpu.pc = 0x5022A;
    16
}
/// $08ABEC: lea.l $2ef5a0.l, a2
pub fn i_08abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABF2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF5A0;
    20
}
/// $08ABF2: jsr $5022a.l
pub fn i_08abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABF8);
    cpu.pc = 0x5022A;
    16
}
/// $08ABF8: lea.l $2ef6e4.l, a1
pub fn i_08abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABFE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6E4;
    20
}
/// $08ABFE: jsr $43fac.l
pub fn i_08abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC04);
    cpu.pc = 0x43FAC;
    16
}
/// $08AC04: lea.l $2ef286.l, a1
pub fn i_08ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC0A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08AC0A: jsr $77c7e.l
pub fn i_08ac0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC10);
    cpu.pc = 0x77C7E;
    16
}
/// $08AC10: lea.l $2ee290.l, a0
pub fn i_08ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE290;
    20
}
/// $08AC16: jsr $28cd4.l
pub fn i_08ac16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC1C);
    cpu.pc = 0x28CD4;
    16
}
/// $08AC1C: lea.l $8ac22(pc), a1
pub fn i_08ac1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AC22;
    16
}
/// $08AC20: move.l a1, (a6)
pub fn i_08ac20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AC22: clr.b $10e39a.l
pub fn i_08ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC28;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AC28: jsr $2783a.l
pub fn i_08ac28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC2E);
    cpu.pc = 0x2783A;
    16
}
/// $08AC2E: move.b #$0, $87(a6)
pub fn i_08ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AC34: jsr $28d70.l
pub fn i_08ac34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC3A);
    cpu.pc = 0x28D70;
    16
}
/// $08AC3A: bcc.w $8ac60
pub fn i_08ac3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AC60; return 10; }
    8
}
/// $08AC3E: jsr $2870a.l
pub fn i_08ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC44);
    cpu.pc = 0x2870A;
    16
}
/// $08AC44: bcc.w $8ac60
pub fn i_08ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AC60; return 10; }
    8
}
/// $08AC48: lea.l $2ee27c.l, a0
pub fn i_08ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE27C;
    20
}
/// $08AC4E: jsr $28cd4.l
pub fn i_08ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC54);
    cpu.pc = 0x28CD4;
    16
}
/// $08AC54: move.b #$ff, $87(a6)
pub fn i_08ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC5A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AC5A: jsr $28d70.l
pub fn i_08ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC60);
    cpu.pc = 0x28D70;
    16
}
/// $08AC60: jsr $2870a.l
pub fn i_08ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC66);
    cpu.pc = 0x2870A;
    16
}
/// $08AC66: bcc.w $8ac7c
pub fn i_08ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AC7C; return 10; }
    8
}
/// $08AC6A: lea.l $5e766.l, a0
pub fn i_08ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AC70: jsr $5e770.l
pub fn i_08ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC76);
    cpu.pc = 0x5E770;
    16
}
/// $08AC76: bclr.b #$3, $13(a6)
pub fn i_08ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AC7C: jsr $28758.l
pub fn i_08ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC82);
    cpu.pc = 0x28758;
    16
}
/// $08AC82: bcc.w $8ae0a
pub fn i_08ac82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AE0A; return 10; }
    8
}
/// $08AC86: move.l #$5000, d0
pub fn i_08ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC8C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08AC8C: jsr $51a28.l
pub fn i_08ac8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC92);
    cpu.pc = 0x51A28;
    16
}
// ===== S5_TowerB_Destroy_08ac92  ($08AC92, 378 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AC92: jsr $434ce.l
pub fn i_08ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC98);
    cpu.pc = 0x434CE;
    16
}
/// $08AC98: move.w #$102f, d0
pub fn i_08ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AC9C: jsr $2352.l
pub fn i_08ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACA2);
    cpu.pc = 0x2352;
    16
}
/// $08ACA2: lea.l $7808a.l, a1
pub fn i_08aca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACA8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ACA8: jsr $4ae.l
pub fn i_08aca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACAE);
    cpu.pc = 0x4AE;
    16
}
/// $08ACAE: jsr $5dd22.l
pub fn i_08acae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACB4);
    cpu.pc = 0x5DD22;
    16
}
/// $08ACB4: addi.w #$c0, $22(a0)
pub fn i_08acb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACBA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACBA: subi.w #$80, $24(a0)
pub fn i_08acba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACC0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACC0: lea.l $7808a.l, a1
pub fn i_08acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACC6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ACC6: jsr $4ae.l
pub fn i_08acc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACCC);
    cpu.pc = 0x4AE;
    16
}
/// $08ACCC: jsr $5dd22.l
pub fn i_08accc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACD2);
    cpu.pc = 0x5DD22;
    16
}
/// $08ACD2: addi.w #$30, $22(a0)
pub fn i_08acd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACD8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACD8: subi.w #$30, $24(a0)
pub fn i_08acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACDE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACDE: lea.l $7808a.l, a1
pub fn i_08acde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACE4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ACE4: jsr $4ae.l
pub fn i_08ace4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACEA);
    cpu.pc = 0x4AE;
    16
}
/// $08ACEA: jsr $5dd22.l
pub fn i_08acea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACF0);
    cpu.pc = 0x5DD22;
    16
}
/// $08ACF0: addi.w #$70, $22(a0)
pub fn i_08acf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACF6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACF6: subi.w #$10, $24(a0)
pub fn i_08acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACFC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACFC: lea.l $7808a.l, a1
pub fn i_08acfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD02;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AD02: jsr $4ae.l
pub fn i_08ad02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD08);
    cpu.pc = 0x4AE;
    16
}
/// $08AD08: jsr $5dd22.l
pub fn i_08ad08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD0E);
    cpu.pc = 0x5DD22;
    16
}
/// $08AD0E: addi.w #$90, $22(a0)
pub fn i_08ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD14;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD14: subi.w #$40, $24(a0)
pub fn i_08ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD1A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD1A: lea.l $7808a.l, a1
pub fn i_08ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AD20: jsr $4ae.l
pub fn i_08ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD26);
    cpu.pc = 0x4AE;
    16
}
/// $08AD26: jsr $5dd22.l
pub fn i_08ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD2C);
    cpu.pc = 0x5DD22;
    16
}
/// $08AD2C: addi.w #$c0, $22(a0)
pub fn i_08ad2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD32;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD32: subi.w #$30, $24(a0)
pub fn i_08ad32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD38;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD38: lea.l $7808a.l, a1
pub fn i_08ad38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD3E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AD3E: jsr $4ae.l
pub fn i_08ad3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD44);
    cpu.pc = 0x4AE;
    16
}
/// $08AD44: jsr $5dd22.l
pub fn i_08ad44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD4A);
    cpu.pc = 0x5DD22;
    16
}
/// $08AD4A: addi.w #$100, $22(a0)
pub fn i_08ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD50;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD50: subi.w #$40, $24(a0)
pub fn i_08ad50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD56;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD56: lea.l $2ef286.l, a1
pub fn i_08ad56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD5C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08AD5C: jsr $77c7e.l
pub fn i_08ad5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD62);
    cpu.pc = 0x77C7E;
    16
}
/// $08AD62: lea.l $2ef564.l, a2
pub fn i_08ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD68;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF564;
    20
}
/// $08AD68: jsr $5022a.l
pub fn i_08ad68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD6E);
    cpu.pc = 0x5022A;
    16
}
/// $08AD6E: lea.l $2ef550.l, a2
pub fn i_08ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD74;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF550;
    20
}
/// $08AD74: jsr $5022a.l
pub fn i_08ad74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD7A);
    cpu.pc = 0x5022A;
    16
}
/// $08AD7A: lea.l $2ef5b4.l, a2
pub fn i_08ad7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD80;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF5B4;
    20
}
/// $08AD80: jsr $5022a.l
pub fn i_08ad80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD86);
    cpu.pc = 0x5022A;
    16
}
/// $08AD86: lea.l $2ef740.l, a1
pub fn i_08ad86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF740;
    20
}
/// $08AD8C: jsr $43fac.l
pub fn i_08ad8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD92);
    cpu.pc = 0x43FAC;
    16
}
/// $08AD92: lea.l $2ef77a.l, a1
pub fn i_08ad92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD98;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF77A;
    20
}
/// $08AD98: jsr $43fac.l
pub fn i_08ad98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD9E);
    cpu.pc = 0x43FAC;
    16
}
/// $08AD9E: lea.l $2ef79a.l, a1
pub fn i_08ad9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADA4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF79A;
    20
}
/// $08ADA4: jsr $43fac.l
pub fn i_08ada4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADAA);
    cpu.pc = 0x43FAC;
    16
}
/// $08ADAA: lea.l $2ef704.l, a1
pub fn i_08adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADB0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF704;
    20
}
/// $08ADB0: jsr $43fac.l
pub fn i_08adb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADB6);
    cpu.pc = 0x43FAC;
    16
}
/// $08ADB6: move.b #$ff, $82(a6)
pub fn i_08adb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ADBC: move.b #$ff, $86(a6)
pub fn i_08adbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ADC2: lea.l $8ae0c(pc), a1
pub fn i_08adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE0C;
    16
}
/// $08ADC6: jsr $4ae.l
pub fn i_08adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADCC);
    cpu.pc = 0x4AE;
    16
}
/// $08ADCC: jsr $5dd22.l
pub fn i_08adcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADD2);
    cpu.pc = 0x5DD22;
    16
}
/// $08ADD2: addi.w #$20, $22(a0)
pub fn i_08add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADD8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ADD8: lea.l $8b07c(pc), a1
pub fn i_08add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B07C;
    16
}
/// $08ADDC: jsr $4ae.l
pub fn i_08addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADE2);
    cpu.pc = 0x4AE;
    16
}
/// $08ADE2: jsr $5dd22.l
pub fn i_08ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADE8);
    cpu.pc = 0x5DD22;
    16
}
/// $08ADE8: addi.w #$90, $22(a0)
pub fn i_08ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADEE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ADEE: subi.w #$10, $24(a0)
pub fn i_08adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADF4;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ADF4: move.b #$1, $21(a0)
pub fn i_08adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADFA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ADFA: move.b #$ff, $20(a6)
pub fn i_08adfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AE00: jsr $8b50e(pc)
pub fn i_08ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8AE04);
    cpu.pc = 0x8B50E;
    16
}
/// $08AE04: jmp $518.l
pub fn i_08ae04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE0A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08AE0A: rts
pub fn i_08ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_FlagSet_08ae0c  ($08AE0C, 36 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE0C: lea.l $2eeec8.l, a0
pub fn i_08ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEEC8;
    20
}
/// $08AE12: move.l a0, $4c(a6)
pub fn i_08ae12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE16;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08AE16: jsr $283ca.l
pub fn i_08ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE1C);
    cpu.pc = 0x283CA;
    16
}
/// $08AE1C: jsr $283ca.l
pub fn i_08ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE22);
    cpu.pc = 0x283CA;
    16
}
/// $08AE22: jsr $283d8.l
pub fn i_08ae22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE28);
    cpu.pc = 0x283D8;
    16
}
/// $08AE28: move.b #$1, $10e39e.l
pub fn i_08ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE30;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
// ===== SetTaskHandler_08ae30  ($08AE30, 8 B, kernel/task_handlers.c)
/// $08AE30: lea.l $8ae38(pc), a1
pub fn i_08ae30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE38;
    16
}
/// $08AE34: move.l a1, (a6)
pub fn i_08ae34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AE36: rts
pub fn i_08ae36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_FlagClear_08ae38  ($08AE38, 28 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE38: move.b #$0, $10e39e.l
pub fn i_08ae38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE40;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $08AE40: lea.l $ffff.w, a0
pub fn i_08ae40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE44;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08AE44: move.l a0, $4c(a6)
pub fn i_08ae44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE48;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08AE48: jsr $283ca.l
pub fn i_08ae48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE4E);
    cpu.pc = 0x283CA;
    16
}
/// $08AE4E: jmp $518.l
pub fn i_08ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE54;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08ae54  ($08AE54, 2 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE54: rts
pub fn i_08ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Init_08ae56  ($08AE56, 10 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE56: cmpi.w #$140, $22(a6)
pub fn i_08ae56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE5C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08AE5C: blt.w $8ae68
pub fn i_08ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE60;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8AE68; return 10; }
    8
}
// ===== JsrAbsThunk_08ae60  ($08AE60, 8 B, thunks/jsr_abs_thunks.c)
/// $08AE60: jsr $2783a.l
pub fn i_08ae60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE66);
    cpu.pc = 0x2783A;
    16
}
/// $08AE66: rts
pub fn i_08ae66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Active_08ae68  ($08AE68, 246 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE68: move.w #$14, $66(a6)
pub fn i_08ae68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AE6E: lea.l $2eebf8.l, a0
pub fn i_08ae6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEBF8;
    20
}
/// $08AE74: move.l a0, $48(a6)
pub fn i_08ae74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE78;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08AE78: lea.l $2ee5be.l, a0
pub fn i_08ae78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE5BE;
    20
}
/// $08AE7E: jsr $28cd4.l
pub fn i_08ae7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE84);
    cpu.pc = 0x28CD4;
    16
}
/// $08AE84: cmpi.b #$0, $21(a6)
pub fn i_08ae84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE8A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AE8A: beq.w $8aec0
pub fn i_08ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AEC0; return 10; }
    8
}
/// $08AE8E: move.w #$1dd, d1
pub fn i_08ae8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AE92: jsr $236e.l
pub fn i_08ae92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE98);
    cpu.pc = 0x236E;
    16
}
/// $08AE98: move.w #$d4, d1
pub fn i_08ae98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE9C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AE9C: jsr $236e.l
pub fn i_08ae9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEA2);
    cpu.pc = 0x236E;
    16
}
/// $08AEA2: lea.l $2ee58e.l, a0
pub fn i_08aea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE58E;
    20
}
/// $08AEA8: jsr $28cd4.l
pub fn i_08aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEAE);
    cpu.pc = 0x28CD4;
    16
}
/// $08AEAE: move.w #$8000, $38(a6)
pub fn i_08aeae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AEB4: jsr $267e2.l
pub fn i_08aeb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEBA);
    cpu.pc = 0x267E2;
    16
}
/// $08AEBA: jsr $27cee.l
pub fn i_08aeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEC0);
    cpu.pc = 0x27CEE;
    16
}
/// $08AEC0: lea.l $8aec6(pc), a1
pub fn i_08aec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AEC6;
    16
}
/// $08AEC4: move.l a1, (a6)
pub fn i_08aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AEC6: movea.l $c(a6), a0
pub fn i_08aec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AECA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08AECA: cmpi.b #$ff, $20(a0)
pub fn i_08aeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AED0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AED0: beq.w $8af58
pub fn i_08aed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AED4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AF58; return 10; }
    8
}
/// $08AED4: move.w $18(a6), $14(a6)
pub fn i_08aed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEDA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AEDA: cmpi.b #$0, $21(a6)
pub fn i_08aeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEE0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AEE0: beq.w $8aef4
pub fn i_08aee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEE4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AEF4; return 10; }
    8
}
/// $08AEE4: cmpi.b #$ff, $87(a0)
pub fn i_08aee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEEA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AEEA: bne.w $8aef4
pub fn i_08aeea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8AEF4; return 10; }
    8
}
/// $08AEEE: move.w $16(a6), $14(a6)
pub fn i_08aeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEF4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AEF4: jsr $2783a.l
pub fn i_08aef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEFA);
    cpu.pc = 0x2783A;
    16
}
/// $08AEFA: jsr $28d70.l
pub fn i_08aefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF00);
    cpu.pc = 0x28D70;
    16
}
/// $08AF00: jsr $2870a.l
pub fn i_08af00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF06);
    cpu.pc = 0x2870A;
    16
}
/// $08AF06: bcc.w $8af1c
pub fn i_08af06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AF1C; return 10; }
    8
}
/// $08AF0A: lea.l $5e766.l, a0
pub fn i_08af0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AF10: jsr $5e770.l
pub fn i_08af10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF16);
    cpu.pc = 0x5E770;
    16
}
/// $08AF16: bclr.b #$3, $13(a6)
pub fn i_08af16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF1C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AF1C: jsr $28758.l
pub fn i_08af1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF22);
    cpu.pc = 0x28758;
    16
}
/// $08AF22: bcc.w $8af66
pub fn i_08af22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AF66; return 10; }
    8
}
/// $08AF26: bclr.b #$0, $13(a6)
pub fn i_08af26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF2C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AF2C: move.w #$1027, d0
pub fn i_08af2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF30;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AF30: jsr $2352.l
pub fn i_08af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF36);
    cpu.pc = 0x2352;
    16
}
/// $08AF36: movea.l $70(a6), a2
pub fn i_08af36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF3A;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08AF3A: jsr $5022a.l
pub fn i_08af3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF40);
    cpu.pc = 0x5022A;
    16
}
/// $08AF40: lea.l $2ef274.l, a1
pub fn i_08af40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF46;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF274;
    20
}
/// $08AF46: jsr $77c7e.l
pub fn i_08af46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF4C);
    cpu.pc = 0x77C7E;
    16
}
/// $08AF4C: cmpi.l #$ffffffff, $74(a6)
pub fn i_08af4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF54;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $08AF54: bne.w $8af60
pub fn i_08af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF58;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8AF60; return 10; }
    8
}
/// $08AF58: jmp $518.l
pub fn i_08af58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF5E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08af5e  ($08AF5E, 2 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AF5E: rts
pub fn i_08af5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_08af60  ($08AF60, 8 B, kernel/task_handlers.c)
/// $08AF60: lea.l $8af68(pc), a1
pub fn i_08af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AF68;
    16
}
/// $08AF64: move.l a1, (a6)
pub fn i_08af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AF66: rts
pub fn i_08af66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Stage2_08af68  ($08AF68, 196 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AF68: move.w #$14, $66(a6)
pub fn i_08af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AF6E: cmpi.b #$0, $21(a6)
pub fn i_08af6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF74;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AF74: beq.w $8af84
pub fn i_08af74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF78;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AF84; return 10; }
    8
}
/// $08AF78: lea.l $2ee59e.l, a0
pub fn i_08af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE59E;
    20
}
/// $08AF7E: jsr $28cd4.l
pub fn i_08af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF84);
    cpu.pc = 0x28CD4;
    16
}
/// $08AF84: lea.l $8af8a(pc), a1
pub fn i_08af84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AF8A;
    16
}
/// $08AF88: move.l a1, (a6)
pub fn i_08af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AF8A: movea.l $c(a6), a0
pub fn i_08af8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF8E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08AF8E: cmpi.b #$ff, $20(a0)
pub fn i_08af8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF94;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AF94: beq.w $8b034
pub fn i_08af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B034; return 10; }
    8
}
/// $08AF98: move.w $18(a6), $14(a6)
pub fn i_08af98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF9E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AF9E: cmpi.b #$0, $21(a6)
pub fn i_08af9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFA4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AFA4: beq.w $8afb8
pub fn i_08afa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AFB8; return 10; }
    8
}
/// $08AFA8: cmpi.b #$ff, $87(a0)
pub fn i_08afa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFAE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AFAE: bne.w $8afb8
pub fn i_08afae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8AFB8; return 10; }
    8
}
/// $08AFB2: move.w $16(a6), $14(a6)
pub fn i_08afb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFB8;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AFB8: jsr $2783a.l
pub fn i_08afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFBE);
    cpu.pc = 0x2783A;
    16
}
/// $08AFBE: jsr $28d70.l
pub fn i_08afbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFC4);
    cpu.pc = 0x28D70;
    16
}
/// $08AFC4: jsr $2870a.l
pub fn i_08afc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFCA);
    cpu.pc = 0x2870A;
    16
}
/// $08AFCA: bcc.w $8afe0
pub fn i_08afca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AFE0; return 10; }
    8
}
/// $08AFCE: lea.l $5e766.l, a0
pub fn i_08afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AFD4: jsr $5e770.l
pub fn i_08afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFDA);
    cpu.pc = 0x5E770;
    16
}
/// $08AFDA: bclr.b #$3, $13(a6)
pub fn i_08afda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFE0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AFE0: jsr $28758.l
pub fn i_08afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFE6);
    cpu.pc = 0x28758;
    16
}
/// $08AFE6: bcc.w $8b03a
pub fn i_08afe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B03A; return 10; }
    8
}
/// $08AFEA: move.l #$1000, d0
pub fn i_08afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFF0;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08AFF0: jsr $51a28.l
pub fn i_08aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFF6);
    cpu.pc = 0x51A28;
    16
}
/// $08AFF6: move.w #$1027, d0
pub fn i_08aff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AFFA: jsr $2352.l
pub fn i_08affa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B000);
    cpu.pc = 0x2352;
    16
}
/// $08B000: movea.l $74(a6), a2
pub fn i_08b000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B004;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08B004: jsr $5022a.l
pub fn i_08b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B00A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B00A);
    cpu.pc = 0x5022A;
    16
}
/// $08B00A: lea.l $2ef274.l, a1
pub fn i_08b00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B010;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF274;
    20
}
/// $08B010: jsr $77c7e.l
pub fn i_08b010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B016;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B016);
    cpu.pc = 0x77C7E;
    16
}
/// $08B016: cmpi.b #$0, $21(a6)
pub fn i_08b016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B01C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B01C: beq.w $8b034
pub fn i_08b01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B020;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B034; return 10; }
    8
}
/// $08B020: lea.l $2ee5ae.l, a0
pub fn i_08b020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B026;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE5AE;
    20
}
/// $08B026: jsr $28cd4.l
pub fn i_08b026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B02C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B02C);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_08b02c  ($08B02C, 8 B, kernel/task_handlers.c)
/// $08B02C: lea.l $8b03c(pc), a1
pub fn i_08b02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B030;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B03C;
    16
}
/// $08B030: move.l a1, (a6)
pub fn i_08b030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B032;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B032: rts
pub fn i_08b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B034;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_08b034  ($08B034, 8 B, kernel/jmp_to_scheduler.c)
/// $08B034: jmp $518.l
pub fn i_08b034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B03A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B03A: rts
pub fn i_08b03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B03C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Idle_08b03c  ($08B03C, 48 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B03C: lea.l $8b042(pc), a1
pub fn i_08b03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B040;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B042;
    16
}
/// $08B040: move.l a1, (a6)
pub fn i_08b040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B042;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B042: movea.l $c(a6), a0
pub fn i_08b042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B046;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B046: cmpi.b #$ff, $20(a0)
pub fn i_08b046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B04C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B04C: beq.w $8b074
pub fn i_08b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B050;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B074; return 10; }
    8
}
/// $08B050: move.w $18(a6), $14(a6)
pub fn i_08b050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B056;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B056: cmpi.b #$ff, $87(a0)
pub fn i_08b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B05C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B05C: bne.w $8b066
pub fn i_08b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B060;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B066; return 10; }
    8
}
/// $08B060: move.w $16(a6), $14(a6)
pub fn i_08b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B066;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B066: jsr $2783a.l
pub fn i_08b066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B06C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B06C);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08b06c  ($08B06C, 8 B, thunks/jsr_abs_thunks.c)
/// $08B06C: jsr $28d70.l
pub fn i_08b06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B072);
    cpu.pc = 0x28D70;
    16
}
/// $08B072: rts
pub fn i_08b072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B074;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_08b074  ($08B074, 8 B, kernel/jmp_to_scheduler.c)
/// $08B074: jmp $518.l
pub fn i_08b074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B07A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Wreck_SmokeRise_08b07c  ($08B07C, 134 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B07C: move.w #$d5, d1
pub fn i_08b07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B080;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B080: jsr $236e.l
pub fn i_08b080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B086;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B086);
    cpu.pc = 0x236E;
    16
}
/// $08B086: move.b #$ff, $32(a6)
pub fn i_08b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B08C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B08C: move.b #$50, $33(a6)
pub fn i_08b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B092;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B092: move.b #$0, $3a(a6)
pub fn i_08b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B098;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B098: move.w #$0, $38(a6)
pub fn i_08b098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B09E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B09E: jsr $267e2.l
pub fn i_08b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0A4);
    cpu.pc = 0x267E2;
    16
}
/// $08B0A4: move.w #$fffa, $2e(a6)
pub fn i_08b0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B0AA: clr.l d0
pub fn i_08b0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0AC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B0AC: move.b $21(a6), d0
pub fn i_08b0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0B0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B0B0: movea.l #$2ee2be, a0
pub fn i_08b0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0B6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE2BE;
    cpu.a[0] = v1;
    4
}
/// $08B0B6: lsl.w #$2, d0
pub fn i_08b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0B8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08B0B8: movea.l (a0, d0.w), a0
pub fn i_08b0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0BC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08B0BC: cmpa.l #$ffffffff, a0
pub fn i_08b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0C2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08B0C2: beq.w $8b0cc
pub fn i_08b0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B0CC; return 10; }
    8
}
/// $08B0C6: jsr $28cd4.l
pub fn i_08b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0CC);
    cpu.pc = 0x28CD4;
    16
}
/// $08B0CC: move.w #$1026, d0
pub fn i_08b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1026;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B0D0: jsr $2352.l
pub fn i_08b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0D6);
    cpu.pc = 0x2352;
    16
}
/// $08B0D6: lea.l $8b0dc(pc), a1
pub fn i_08b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B0DC;
    16
}
/// $08B0DA: move.l a1, (a6)
pub fn i_08b0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B0DC: jsr $27cee.l
pub fn i_08b0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0E2);
    cpu.pc = 0x27CEE;
    16
}
/// $08B0E2: cmpi.b #$ff, $33(a6)
pub fn i_08b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0E8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B0E8: beq.w $8b0f2
pub fn i_08b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B0F2; return 10; }
    8
}
/// $08B0EC: addi.b #$1, $33(a6)
pub fn i_08b0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0F2;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B0F2: jsr $28d70.l
pub fn i_08b0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0F8);
    cpu.pc = 0x28D70;
    16
}
/// $08B0F8: cmpi.w #$170, $24(a6)
pub fn i_08b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $08B0FE: bgt.w $8b108
pub fn i_08b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B102;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8B108; return 10; }
    8
}
// ===== SetTaskHandler_08b102  ($08B102, 8 B, kernel/task_handlers.c)
/// $08B102: lea.l $8b10a(pc), a1
pub fn i_08b102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B106;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B10A;
    16
}
/// $08B106: move.l a1, (a6)
pub fn i_08b106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B108;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B108: rts
pub fn i_08b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B10A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_SparkBurst_08b10a  ($08B10A, 232 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B10A: move.w #$1051, d0
pub fn i_08b10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B10E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1051;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B10E: jsr $2352.l
pub fn i_08b10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B114;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B114);
    cpu.pc = 0x2352;
    16
}
/// $08B114: lea.l $8b1f2(pc), a1
pub fn i_08b114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B118;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B118: jsr $4ae.l
pub fn i_08b118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B11E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B11E);
    cpu.pc = 0x4AE;
    16
}
/// $08B11E: jsr $5dd22.l
pub fn i_08b11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B124);
    cpu.pc = 0x5DD22;
    16
}
/// $08B124: addi.w #$70, $22(a0)
pub fn i_08b124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B12A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B12A: move.w #$125, $24(a0)
pub fn i_08b12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B130;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B130: move.w #$0, $72(a0)
pub fn i_08b130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B136;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B136: lea.l $8b1f2(pc), a1
pub fn i_08b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B13A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B13A: jsr $4ae.l
pub fn i_08b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B140);
    cpu.pc = 0x4AE;
    16
}
/// $08B140: jsr $5dd22.l
pub fn i_08b140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B146);
    cpu.pc = 0x5DD22;
    16
}
/// $08B146: addi.w #$40, $22(a0)
pub fn i_08b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B14C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B14C: move.w #$125, $24(a0)
pub fn i_08b14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B152;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B152: move.w #$6, $72(a0)
pub fn i_08b152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B158;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B158: lea.l $8b1f2(pc), a1
pub fn i_08b158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B15C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B15C: jsr $4ae.l
pub fn i_08b15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B162);
    cpu.pc = 0x4AE;
    16
}
/// $08B162: jsr $5dd22.l
pub fn i_08b162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B168);
    cpu.pc = 0x5DD22;
    16
}
/// $08B168: addi.w #$10, $22(a0)
pub fn i_08b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B16E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B16E: move.w #$125, $24(a0)
pub fn i_08b16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B174;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B174: move.w #$c, $72(a0)
pub fn i_08b174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B17A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B17A: lea.l $8b1f2(pc), a1
pub fn i_08b17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B17E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B17E: jsr $4ae.l
pub fn i_08b17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B184);
    cpu.pc = 0x4AE;
    16
}
/// $08B184: jsr $5dd22.l
pub fn i_08b184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B18A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B18A);
    cpu.pc = 0x5DD22;
    16
}
/// $08B18A: subi.w #$20, $22(a0)
pub fn i_08b18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B190;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B190: move.w #$125, $24(a0)
pub fn i_08b190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B196;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B196: move.w #$12, $72(a0)
pub fn i_08b196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B19C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B19C: lea.l $8b1f2(pc), a1
pub fn i_08b19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B1A0: jsr $4ae.l
pub fn i_08b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1A6);
    cpu.pc = 0x4AE;
    16
}
/// $08B1A6: jsr $5dd22.l
pub fn i_08b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1AC);
    cpu.pc = 0x5DD22;
    16
}
/// $08B1AC: subi.w #$50, $22(a0)
pub fn i_08b1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1B2;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B1B2: move.w #$125, $24(a0)
pub fn i_08b1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1B8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B1B8: move.w #$18, $72(a0)
pub fn i_08b1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1BE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B1BE: lea.l $8b1c4(pc), a1
pub fn i_08b1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1C4;
    16
}
/// $08B1C2: move.l a1, (a6)
pub fn i_08b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B1C4: jsr $27cee.l
pub fn i_08b1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1CA);
    cpu.pc = 0x27CEE;
    16
}
/// $08B1CA: cmpi.b #$ff, $33(a6)
pub fn i_08b1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1D0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B1D0: beq.w $8b1da
pub fn i_08b1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B1DA; return 10; }
    8
}
/// $08B1D4: addi.b #$1, $33(a6)
pub fn i_08b1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1DA;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B1DA: jsr $28d70.l
pub fn i_08b1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1E0);
    cpu.pc = 0x28D70;
    16
}
/// $08B1E0: cmpi.w #$80, $24(a6)
pub fn i_08b1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Word, false);
    16
}
/// $08B1E6: bgt.w $8b1f0
pub fn i_08b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8B1F0; return 10; }
    8
}
/// $08B1EA: jmp $518.l
pub fn i_08b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B1F0: rts
pub fn i_08b1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_Spark_08b1f2  ($08B1F2, 102 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B1F2: jsr $2783a.l
pub fn i_08b1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1F8);
    cpu.pc = 0x2783A;
    16
}
/// $08B1F8: subq.w #$1, $72(a6)
pub fn i_08b1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1FC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B1FC: cmpi.w #$0, $72(a6)
pub fn i_08b1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B202;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08B202: ble.w $8b208
pub fn i_08b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B206;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B208; return 10; }
    8
}
/// $08B206: rts
pub fn i_08b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B208;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08B208: move.w #$139, d1
pub fn i_08b208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B20C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x139;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B20C: jsr $236e.l
pub fn i_08b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B212);
    cpu.pc = 0x236E;
    16
}
/// $08B212: move.b #$ff, $32(a6)
pub fn i_08b212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B218;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B218: move.b #$ff, $33(a6)
pub fn i_08b218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B21E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B21E: lea.l $2ded26.l, a0
pub fn i_08b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B224;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DED26;
    20
}
/// $08B224: jsr $28cd4.l
pub fn i_08b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B22A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B22A);
    cpu.pc = 0x28CD4;
    16
}
/// $08B22A: lea.l $8b230(pc), a1
pub fn i_08b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B22E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B230;
    16
}
/// $08B22E: move.l a1, (a6)
pub fn i_08b22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B230;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B230: jsr $44048.l
pub fn i_08b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B236);
    cpu.pc = 0x44048;
    16
}
/// $08B236: jsr $28d70.l
pub fn i_08b236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B23C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B23C);
    cpu.pc = 0x28D70;
    16
}
/// $08B23C: bcs.w $8b250
pub fn i_08b23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B240;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8B250; return 10; }
    8
}
/// $08B240: lea.l $2ef7a8.l, a0
pub fn i_08b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B246;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7A8;
    20
}
/// $08B246: jsr $5dd56.l
pub fn i_08b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B24C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B24C);
    cpu.pc = 0x5DD56;
    16
}
/// $08B24C: bcc.w $8b256
pub fn i_08b24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B250;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B256; return 10; }
    8
}
/// $08B250: jmp $518.l
pub fn i_08b250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B256;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B256: rts
pub fn i_08b256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B258;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Thrown_08b258  ($08B258, 244 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B258: cmpi.b #$0, $9c(a6)
pub fn i_08b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B25E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B25E: bne.w $8b270
pub fn i_08b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B262;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B270; return 10; }
    8
}
/// $08B262: move.w #$162, d1
pub fn i_08b262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B266;
    cpu.ir = 0x323C;
    let v1: u32 = 0x162;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B266: jsr $236e.l
pub fn i_08b266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B26C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B26C);
    cpu.pc = 0x236E;
    16
}
/// $08B26C: bra.w $8b27a
pub fn i_08b26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B270;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B27A;
    10
}
/// $08B270: move.w #$d8, d1
pub fn i_08b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B274;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B274: jsr $236e.l
pub fn i_08b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B27A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B27A);
    cpu.pc = 0x236E;
    16
}
/// $08B27A: move.b #$ff, $32(a6)
pub fn i_08b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B280;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B280: move.b #$ff, $33(a6)
pub fn i_08b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B286;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B286: jsr $267e2.l
pub fn i_08b286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B28C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B28C);
    cpu.pc = 0x267E2;
    16
}
/// $08B28C: jsr $8f002.l
pub fn i_08b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B292);
    cpu.pc = 0x8F002;
    16
}
/// $08B292: jsr $8f010.l
pub fn i_08b292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B298);
    cpu.pc = 0x8F010;
    16
}
/// $08B298: cmpi.b #$1, $9a(a6)
pub fn i_08b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B29E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08B29E: bne.w $8b2b2
pub fn i_08b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B2B2; return 10; }
    8
}
/// $08B2A2: bset.b #$6, $12(a6)
pub fn i_08b2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2A8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B2A8: move.w #$ffff, $38(a6)
pub fn i_08b2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B2AE: bra.w $8b2b8
pub fn i_08b2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B2B8;
    10
}
/// $08B2B2: move.w #$8000, $38(a6)
pub fn i_08b2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B2B8: clr.l d0
pub fn i_08b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2BA;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B2BA: move.b $9d(a6), d0
pub fn i_08b2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B2BE: movea.l #$2ee7a8, a0
pub fn i_08b2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2C4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE7A8;
    cpu.a[0] = v1;
    4
}
/// $08B2C4: lsl.w #$2, d0
pub fn i_08b2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2C6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08B2C6: movea.l (a0, d0.w), a0
pub fn i_08b2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2CA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08B2CA: cmpa.l #$ffffffff, a0
pub fn i_08b2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2D0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08B2D0: beq.w $8b2da
pub fn i_08b2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B2DA; return 10; }
    8
}
/// $08B2D4: jsr $28cd4.l
pub fn i_08b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B2DA);
    cpu.pc = 0x28CD4;
    16
}
/// $08B2DA: lea.l $8b2e0(pc), a1
pub fn i_08b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B2E0;
    16
}
/// $08B2DE: move.l a1, (a6)
pub fn i_08b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B2E0: jsr $27cee.l
pub fn i_08b2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B2E6);
    cpu.pc = 0x27CEE;
    16
}
/// $08B2E6: jsr $28d70.l
pub fn i_08b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B2EC);
    cpu.pc = 0x28D70;
    16
}
/// $08B2EC: cmpi.b #$0, $9b(a6)
pub fn i_08b2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2F2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B2F2: bne.w $8b2fe
pub fn i_08b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B2FE; return 10; }
    8
}
/// $08B2F6: lea.l $ffff.w, a0
pub fn i_08b2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2FA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08B2FA: move.l a0, $48(a6)
pub fn i_08b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2FE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B2FE: jsr $2870a.l
pub fn i_08b2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B304);
    cpu.pc = 0x2870A;
    16
}
/// $08B304: bcc.w $8b31e
pub fn i_08b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B308;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B31E; return 10; }
    8
}
/// $08B308: lea.l $5e766.l, a0
pub fn i_08b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B30E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08B30E: jsr $5e770.l
pub fn i_08b30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B314);
    cpu.pc = 0x5E770;
    16
}
/// $08B314: bclr.b #$3, $13(a6)
pub fn i_08b314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B31A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B31A: jsr $8b558(pc)
pub fn i_08b31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B31E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B31E);
    cpu.pc = 0x8B558;
    16
}
/// $08B31E: cmpi.b #$0, $9b(a6)
pub fn i_08b31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B324;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B324: beq.w $8b334
pub fn i_08b324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B328;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B334; return 10; }
    8
}
/// $08B328: jsr $283ca.l
pub fn i_08b328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B32E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B32E);
    cpu.pc = 0x283CA;
    16
}
/// $08B32E: jsr $283d8.l
pub fn i_08b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B334;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B334);
    cpu.pc = 0x283D8;
    16
}
/// $08B334: lea.l $2ef7bc.l, a0
pub fn i_08b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B33A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7BC;
    20
}
/// $08B33A: jsr $5dd5c.l
pub fn i_08b33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B340);
    cpu.pc = 0x5DD5C;
    16
}
/// $08B340: bcc.w $8b34a
pub fn i_08b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B344;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B34A; return 10; }
    8
}
/// $08B344: jmp $518.l
pub fn i_08b344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B34A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B34A: rts
pub fn i_08b34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B34C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Drop_V0_08b34c  ($08B34C, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B34C: lea.l $2ee194.l, a0
pub fn i_08b34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B352;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE194;
    20
}
/// $08B352: jsr $28cd4.l
pub fn i_08b352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B358);
    cpu.pc = 0x28CD4;
    16
}
/// $08B358: lea.l $2ef7c6.l, a0
pub fn i_08b358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B35E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7C6;
    20
}
/// $08B35E: move.l a0, $70(a6)
pub fn i_08b35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B362;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B362: bra.w $8b3b4
pub fn i_08b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B366;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
// ===== Proj_Drop_V1_08b366  ($08B366, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B366: lea.l $2ee1a0.l, a0
pub fn i_08b366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B36C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1A0;
    20
}
/// $08B36C: jsr $28cd4.l
pub fn i_08b36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B372);
    cpu.pc = 0x28CD4;
    16
}
/// $08B372: lea.l $2ef7d0.l, a0
pub fn i_08b372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B378;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7D0;
    20
}
/// $08B378: move.l a0, $70(a6)
pub fn i_08b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B37C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B37C: bra.w $8b3b4
pub fn i_08b37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B380;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
// ===== Proj_Drop_V2_08b380  ($08B380, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B380: lea.l $2ee1ac.l, a0
pub fn i_08b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B386;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1AC;
    20
}
/// $08B386: jsr $28cd4.l
pub fn i_08b386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B38C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B38C);
    cpu.pc = 0x28CD4;
    16
}
/// $08B38C: lea.l $2ef7da.l, a0
pub fn i_08b38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B392;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7DA;
    20
}
/// $08B392: move.l a0, $70(a6)
pub fn i_08b392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B396;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B396: bra.w $8b3b4
pub fn i_08b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B39A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
// ===== Proj_Drop_V3_08b39a  ($08B39A, 194 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B39A: lea.l $2ee1b8.l, a0
pub fn i_08b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1B8;
    20
}
/// $08B3A0: jsr $28cd4.l
pub fn i_08b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3A6);
    cpu.pc = 0x28CD4;
    16
}
/// $08B3A6: lea.l $2ef7e4.l, a0
pub fn i_08b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7E4;
    20
}
/// $08B3AC: move.l a0, $70(a6)
pub fn i_08b3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B3B0: bra.w $8b3b4
pub fn i_08b3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
/// $08B3B4: move.b $9a(a6), $32(a6)
pub fn i_08b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3BA;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08B3BA: move.b $9a(a6), $33(a6)
pub fn i_08b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3C0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08B3C0: jsr $267e2.l
pub fn i_08b3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3C6);
    cpu.pc = 0x267E2;
    16
}
/// $08B3C6: jsr $8f002.l
pub fn i_08b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3CC);
    cpu.pc = 0x8F002;
    16
}
/// $08B3CC: jsr $8f010.l
pub fn i_08b3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3D2);
    cpu.pc = 0x8F010;
    16
}
/// $08B3D2: move.l $106f50.l, d0
pub fn i_08b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3D8;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08B3D8: swap d0
pub fn i_08b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3DA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08B3DA: cmpi.b #$0, $9b(a6)
pub fn i_08b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3E0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B3E0: bne.w $8b40e
pub fn i_08b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B40E; return 10; }
    8
}
/// $08B3E4: move.w #$0, $38(a6)
pub fn i_08b3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B3EA: cmpi.w #$670, d0
pub fn i_08b3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3EE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x670, Size::Word, false);
    8
}
/// $08B3EE: ble.w $8b400
pub fn i_08b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3F2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B400; return 10; }
    8
}
/// $08B3F2: move.w #$16d, d1
pub fn i_08b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3F6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B3F6: jsr $236e.l
pub fn i_08b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3FC);
    cpu.pc = 0x236E;
    16
}
/// $08B3FC: bra.w $8b40a
pub fn i_08b3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B400;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B40A;
    10
}
/// $08B400: move.w #$16f, d1
pub fn i_08b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B404;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B404: jsr $236e.l
pub fn i_08b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B40A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B40A);
    cpu.pc = 0x236E;
    16
}
/// $08B40A: bra.w $8b434
pub fn i_08b40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B40E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B434;
    10
}
/// $08B40E: move.w #$1fff, $38(a6)
pub fn i_08b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B414;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B414: cmpi.w #$670, d0
pub fn i_08b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B418;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x670, Size::Word, false);
    8
}
/// $08B418: ble.w $8b42a
pub fn i_08b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B41C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B42A; return 10; }
    8
}
/// $08B41C: move.w #$ce, d1
pub fn i_08b41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B420;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B420: jsr $236e.l
pub fn i_08b420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B426;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B426);
    cpu.pc = 0x236E;
    16
}
/// $08B426: bra.w $8b434
pub fn i_08b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B42A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B434;
    10
}
/// $08B42A: move.w #$16e, d1
pub fn i_08b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B42E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B42E: jsr $236e.l
pub fn i_08b42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B434);
    cpu.pc = 0x236E;
    16
}
/// $08B434: lea.l $8b43a(pc), a1
pub fn i_08b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B438;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B43A;
    16
}
/// $08B438: move.l a1, (a6)
pub fn i_08b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B43A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B43A: jsr $27cee.l
pub fn i_08b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B440);
    cpu.pc = 0x27CEE;
    16
}
/// $08B440: jsr $28d70.l
pub fn i_08b440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B446);
    cpu.pc = 0x28D70;
    16
}
/// $08B446: movea.l $70(a6), a0
pub fn i_08b446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B44A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B44A: jsr $5dd5c.l
pub fn i_08b44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B450);
    cpu.pc = 0x5DD5C;
    16
}
/// $08B450: bcc.w $8b45a
pub fn i_08b450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B454;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B45A; return 10; }
    8
}
/// $08B454: jmp $518.l
pub fn i_08b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B45A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B45A: rts
pub fn i_08b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B45C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp2On_08b45c  ($08B45C, 52 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B45C: move.w #$11, d1
pub fn i_08b45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B460;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B460: move.w #$1ac, d2
pub fn i_08b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B464;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B464: move.w #$ffff, d3
pub fn i_08b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B468;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B468: move.w #$1, d4
pub fn i_08b468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B46C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B46C: jsr $2c26.l
pub fn i_08b46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B472);
    cpu.pc = 0x2C26;
    16
}
/// $08B472: move.w #$12, d1
pub fn i_08b472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B476;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B476: move.w #$1ad, d2
pub fn i_08b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B47A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B47A: move.w #$ffff, d3
pub fn i_08b47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B47E;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B47E: move.w #$1, d4
pub fn i_08b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B482;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B482: jsr $2c26.l
pub fn i_08b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B488);
    cpu.pc = 0x2C26;
    16
}
/// $08B488: move.w $16(a6), $14(a6)
pub fn i_08b488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B48E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B48E: rts
pub fn i_08b48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B490;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp2Off_08b490  ($08B490, 52 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B490: move.w #$11, d1
pub fn i_08b490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B494;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B494: move.w #$81, d2
pub fn i_08b494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B498;
    cpu.ir = 0x343C;
    let v1: u32 = 0x81;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B498: move.w #$ffff, d3
pub fn i_08b498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B49C;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B49C: move.w #$1, d4
pub fn i_08b49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4A0;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4A0: jsr $2c26.l
pub fn i_08b4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4A6);
    cpu.pc = 0x2C26;
    16
}
/// $08B4A6: move.w #$12, d1
pub fn i_08b4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4AA: move.w #$82, d2
pub fn i_08b4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4AE;
    cpu.ir = 0x343C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4AE: move.w #$ffff, d3
pub fn i_08b4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4B2;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4B2: move.w #$1, d4
pub fn i_08b4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4B6;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4B6: jsr $2c26.l
pub fn i_08b4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4BC);
    cpu.pc = 0x2C26;
    16
}
/// $08B4BC: move.w $18(a6), $14(a6)
pub fn i_08b4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4C2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B4C2: rts
pub fn i_08b4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp3On_08b4c4  ($08B4C4, 74 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B4C4: move.w #$10, d1
pub fn i_08b4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4C8: move.w #$1ab, d2
pub fn i_08b4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4CC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4CC: move.w #$ffff, d3
pub fn i_08b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4D0;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4D0: move.w #$1, d4
pub fn i_08b4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4D4;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4D4: jsr $2c26.l
pub fn i_08b4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4DA);
    cpu.pc = 0x2C26;
    16
}
/// $08B4DA: move.w #$11, d1
pub fn i_08b4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4DE: move.w #$1ac, d2
pub fn i_08b4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4E2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4E2: move.w #$ffff, d3
pub fn i_08b4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4E6;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4E6: move.w #$1, d4
pub fn i_08b4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4EA;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4EA: jsr $2c26.l
pub fn i_08b4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4F0);
    cpu.pc = 0x2C26;
    16
}
/// $08B4F0: move.w #$12, d1
pub fn i_08b4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4F4: move.w #$1ad, d2
pub fn i_08b4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4F8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4F8: move.w #$ffff, d3
pub fn i_08b4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4FC;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4FC: move.w #$1, d4
pub fn i_08b4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B500;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B500: jsr $2c26.l
pub fn i_08b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B506;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B506);
    cpu.pc = 0x2C26;
    16
}
/// $08B506: move.w $16(a6), $14(a6)
pub fn i_08b506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B50C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B50C: rts
pub fn i_08b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B50E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp3Off_08b50e  ($08B50E, 74 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B50E: move.w #$10, d1
pub fn i_08b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B512;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B512: move.w #$80, d2
pub fn i_08b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B516;
    cpu.ir = 0x343C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B516: move.w #$ffff, d3
pub fn i_08b516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B51A;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B51A: move.w #$1, d4
pub fn i_08b51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B51E;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B51E: jsr $2c26.l
pub fn i_08b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B524);
    cpu.pc = 0x2C26;
    16
}
/// $08B524: move.w #$11, d1
pub fn i_08b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B528;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B528: move.w #$81, d2
pub fn i_08b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B52C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x81;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B52C: move.w #$ffff, d3
pub fn i_08b52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B530;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B530: move.w #$1, d4
pub fn i_08b530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B534;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B534: jsr $2c26.l
pub fn i_08b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B53A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B53A);
    cpu.pc = 0x2C26;
    16
}
/// $08B53A: move.w #$12, d1
pub fn i_08b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B53E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B53E: move.w #$82, d2
pub fn i_08b53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B542;
    cpu.ir = 0x343C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B542: move.w #$ffff, d3
pub fn i_08b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B546;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B546: move.w #$1, d4
pub fn i_08b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B54A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B54A: jsr $2c26.l
pub fn i_08b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B550);
    cpu.pc = 0x2C26;
    16
}
/// $08B550: move.w $18(a6), $14(a6)
pub fn i_08b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B556;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B556: rts
pub fn i_08b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B558;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Handler_ConditionalHitCounter_08B558  ($08B558, 54 B, entity/handler_conditional_hit_08b558.s)
/// $08B558: cmpi.b #$1, $58(a6)
pub fn i_08b558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B55E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08B55E: bne.w $8b58c
pub fn i_08b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B562;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B58C; return 10; }
    8
}
/// $08B562: addi.w #$a, $66(a6)
pub fn i_08b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B568;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B568: jsr $28758.l
pub fn i_08b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B56E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B56E);
    cpu.pc = 0x28758;
    16
}
/// $08B56E: bcc.w $8b578
pub fn i_08b56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B572;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B578; return 10; }
    8
}
/// $08B572: bclr.b #$0, $13(a6)
pub fn i_08b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B578;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B578: move.b $106f28.l, d0
pub fn i_08b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B57E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B57E: andi.b #$1, d0
pub fn i_08b57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B582;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08B582: beq.w $8b58c
pub fn i_08b582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B586;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B58C; return 10; }
    8
}
/// $08B586: jsr $6e224.l
pub fn i_08b586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B58C);
    cpu.pc = 0x6E224;
    16
}
/// $08B58C: rts
pub fn i_08b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B58E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Math_AbsW_08b58e  ($08B58E, 16 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B58E: move.w d0, d1
pub fn i_08b58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B590;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B590: asr.w #$8, d1
pub fn i_08b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B592;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08B592: btst.b #$7, d1
pub fn i_08b592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B596;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B596: beq.w $8b59c
pub fn i_08b596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B59A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B59C; return 10; }
    8
}
/// $08B59A: neg.w d0
pub fn i_08b59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B59C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B59C: rts
pub fn i_08b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B59E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FollowParentPlus40_08b59e  ($08B59E, 24 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B59E: movea.l $c(a6), a0
pub fn i_08b59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5A2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B5A2: move.w $22(a0), $22(a6)
pub fn i_08b5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5A8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5A8: move.w $24(a0), $24(a6)
pub fn i_08b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5AE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5AE: addi.w #$40, $24(a6)
pub fn i_08b5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5B4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B5B4: rts
pub fn i_08b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FollowParent_08b5b6  ($08B5B6, 18 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B5B6: movea.l $c(a6), a0
pub fn i_08b5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5BA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B5BA: move.w $22(a0), $22(a6)
pub fn i_08b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5C0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5C0: move.w $24(a0), $24(a6)
pub fn i_08b5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5C6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5C6: rts
pub fn i_08b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_RotateStep_08b5c8  ($08B5C8, 82 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B5C8: subq.w #$1, $92(a6)
pub fn i_08b5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5CC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B5CC: bne.w $8b620
pub fn i_08b5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B620; return 10; }
    8
}
/// $08B5D0: move.w $90(a6), d0
pub fn i_08b5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5D4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B5D4: add.w d0, $34(a6)
pub fn i_08b5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5D8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B5D8: move.w $90(a6), d0
pub fn i_08b5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5DC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B5DC: asr.w #$8, d0
pub fn i_08b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5DE;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08B5DE: btst.b #$7, d0
pub fn i_08b5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5E2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B5E2: bne.w $8b5fa
pub fn i_08b5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B5FA; return 10; }
    8
}
/// $08B5E6: cmpi.w #$3, $34(a6)
pub fn i_08b5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08B5EC: ble.w $8b60e
pub fn i_08b5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5F0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B60E; return 10; }
    8
}
/// $08B5F0: move.w #$ffff, $90(a6)
pub fn i_08b5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B5F6: bra.w $8b60e
pub fn i_08b5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B60E;
    10
}
/// $08B5FA: cmpi.w #$2, $34(a6)
pub fn i_08b5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B600;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $08B600: bge.w $8b60e
pub fn i_08b600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B604;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8B60E; return 10; }
    8
}
/// $08B604: move.w #$1, $90(a6)
pub fn i_08b604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B60A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B60A: bra.w $8b60e
pub fn i_08b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B60E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B60E;
    10
}
/// $08B60E: andi.w #$7, $34(a6)
pub fn i_08b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B614;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B614: move.w #$f, $92(a6)
pub fn i_08b614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B61A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_08b61a  ($08B61A, 6 B, thunks/ccr_helpers.c)
/// $08B61A: ori.b #$11, ccr
pub fn i_08b61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B61E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B61E: rts
pub fn i_08b61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B620;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_08b620  ($08B620, 6 B, thunks/ccr_helpers.c)
/// $08B620: andi.b #$ee, ccr
pub fn i_08b620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B624;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B624: rts
pub fn i_08b624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B626;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_FireBullet_08b626  ($08B626, 130 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B626: move.b $106f28.l, d0
pub fn i_08b626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B62C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B62C: andi.b #$7, d0
pub fn i_08b62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B630;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08B630: bne.w $8b6a6
pub fn i_08b630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B634;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B6A6; return 10; }
    8
}
/// $08B634: move.b $106f28.l, d0
pub fn i_08b634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B63A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B63A: btst.b #$3, d0
pub fn i_08b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B63E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B63E: bne.w $8b65e
pub fn i_08b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B642;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B65E; return 10; }
    8
}
/// $08B642: lea.l $89ad0(pc), a1
pub fn i_08b642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B646;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89AD0;
    16
}
/// $08B646: jsr $6fe.l
pub fn i_08b646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B64C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B64C);
    cpu.pc = 0x6FE;
    16
}
/// $08B64C: jsr $5dd02.l
pub fn i_08b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B652);
    cpu.pc = 0x5DD02;
    16
}
/// $08B652: addi.w #$8, $22(a0)
pub fn i_08b652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B658;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B658: addi.w #$20, $24(a0)
pub fn i_08b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B65E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B65E: move.w #$108c, d0
pub fn i_08b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B662;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B662: jsr $2352.l
pub fn i_08b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B668);
    cpu.pc = 0x2352;
    16
}
/// $08B668: lea.l $3093a.l, a1
pub fn i_08b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B66E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3093A;
    20
}
/// $08B66E: jsr $4ae.l
pub fn i_08b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B674;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B674);
    cpu.pc = 0x4AE;
    16
}
/// $08B674: jsr $5dd22.l
pub fn i_08b674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B67A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B67A);
    cpu.pc = 0x5DD22;
    16
}
/// $08B67A: move.w $34(a6), d0
pub fn i_08b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B67E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B67E: asl.w #$4, d0
pub fn i_08b67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B680;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08B680: andi.w #$ff, d0
pub fn i_08b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B684;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B684: move.b d0, $98(a0)
pub fn i_08b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B688;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B688: clr.l d0
pub fn i_08b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B68A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B68A: move.w $34(a6), d0
pub fn i_08b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B68E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B68E: asl.l #$2, d0
pub fn i_08b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B690;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $08B690: lea.l $2ef7ee.l, a1
pub fn i_08b690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B696;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF7EE;
    20
}
/// $08B696: move.w (a1, d0.w), d1
pub fn i_08b696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B69A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08B69A: add.w d1, $22(a0)
pub fn i_08b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B69E;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B69E: move.w $2(a1, d0.w), d1
pub fn i_08b69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6A2;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08B6A2: add.w d1, $24(a0)
pub fn i_08b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6A6;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B6A6: rts
pub fn i_08b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Steer_08b6a8  ($08B6A8, 106 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B6A8: subi.w #$2, $34(a6)
pub fn i_08b6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6AE;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B6AE: andi.w #$ff, $34(a6)
pub fn i_08b6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6B4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B6B4: move.w $34(a6), d0
pub fn i_08b6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6B8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B6B8: move.w #$3e, d1
pub fn i_08b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6BC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B6BC: jsr $13c0e.l
pub fn i_08b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B6C2);
    cpu.pc = 0x13C0E;
    16
}
/// $08B6C2: asl.w #$1, d1
pub fn i_08b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6C4;
    cpu.ir = 0xE341;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08B6C4: jsr $8b778(pc)
pub fn i_08b6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B6C8);
    cpu.pc = 0x8B778;
    16
}
/// $08B6C8: move.w d1, $28(a6)
pub fn i_08b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6CC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B6CC: jsr $8b796(pc)
pub fn i_08b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B6D0);
    cpu.pc = 0x8B796;
    16
}
/// $08B6D0: move.w d2, $2a(a6)
pub fn i_08b6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6D4;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B6D4: jsr $8b7f0(pc)
pub fn i_08b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B6D8);
    cpu.pc = 0x8B7F0;
    16
}
/// $08B6D8: bcc.w $8b6f6
pub fn i_08b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B6F6; return 10; }
    8
}
/// $08B6DC: bclr.b #$6, $13(a6)
pub fn i_08b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6E2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B6E2: jsr $2783a.l
pub fn i_08b6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B6E8);
    cpu.pc = 0x2783A;
    16
}
/// $08B6E8: cmpi.w #$0, $28(a6)
pub fn i_08b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6EE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08B6EE: ble.w $8b6f6
pub fn i_08b6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6F2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B6F6; return 10; }
    8
}
/// $08B6F2: clr.w $28(a6)
pub fn i_08b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B6F6: jsr $27cee.l
pub fn i_08b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B6FC);
    cpu.pc = 0x27CEE;
    16
}
/// $08B6FC: jsr $8b862(pc)
pub fn i_08b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B700;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B700);
    cpu.pc = 0x8B862;
    16
}
/// $08B700: jsr $8b7b0(pc)
pub fn i_08b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B704;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B704);
    cpu.pc = 0x8B7B0;
    16
}
/// $08B704: bcc.w $8b716
pub fn i_08b704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B708;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B716; return 10; }
    8
}
/// $08B708: move.w #$1081, d0
pub fn i_08b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B70C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1081;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B70C: jsr $2352.l
pub fn i_08b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B712;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B712);
    cpu.pc = 0x2352;
    16
}
// ===== JsrPcThunk_08b712  ($08B712, 6 B, thunks/jsr_pc_thunks.c)
/// $08B712: jsr $8b82c(pc)
pub fn i_08b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B716;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B716);
    cpu.pc = 0x8B82C;
    16
}
/// $08B716: rts
pub fn i_08b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B718;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_SpiralDescent_08b718  ($08B718, 88 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B718: move.w $36(a6), d1
pub fn i_08b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B71C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B71C: btst.b #$f, d1
pub fn i_08b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B720;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B720: beq.w $8b72c
pub fn i_08b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B724;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B72C; return 10; }
    8
}
/// $08B724: clr.w d1
pub fn i_08b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B726;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B726: clr.w d2
pub fn i_08b726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B728;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B728: bra.w $8b768
pub fn i_08b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B72C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B768;
    10
}
/// $08B72C: move.w $36(a6), d1
pub fn i_08b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B730;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B730: asr.w #$5, d1
pub fn i_08b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B732;
    cpu.ir = 0xEA41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08B732: move.b $106f28.l, d0
pub fn i_08b732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B738;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B738: btst.b #$0, d0
pub fn i_08b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B73C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B73C: beq.w $8b742
pub fn i_08b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B740;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B742; return 10; }
    8
}
/// $08B740: neg.w d1
pub fn i_08b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B742;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B742: asl.w #$8, d1
pub fn i_08b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B744;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08B744: move.w d1, $2a(a6)
pub fn i_08b744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B748;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B748: move.w #$0, $28(a6)
pub fn i_08b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B74E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B74E: jsr $27cee.l
pub fn i_08b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B754);
    cpu.pc = 0x27CEE;
    16
}
/// $08B754: subi.w #$3, $36(a6)
pub fn i_08b754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B75A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B75A: move.w $36(a6), d1
pub fn i_08b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B75E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B75E: move.w #$1f, d0
pub fn i_08b75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B762;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B762: jsr $13c0e.l
pub fn i_08b762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B768;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B768);
    cpu.pc = 0x13C0E;
    16
}
/// $08B768: move.w d1, $28(a6)
pub fn i_08b768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B76C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B76C: move.w d2, $2a(a6)
pub fn i_08b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B770;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_08b770  ($08B770, 8 B, thunks/jsr_abs_thunks.c)
/// $08B770: jsr $27cee.l
pub fn i_08b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B776);
    cpu.pc = 0x27CEE;
    16
}
/// $08B776: rts
pub fn i_08b776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B778;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_SteerX_08b778  ($08B778, 30 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B778: move.w $7e(a6), d0
pub fn i_08b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B77C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B77C: sub.w $22(a6), d0
pub fn i_08b77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B780;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08B780: beq.w $8b794
pub fn i_08b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B784;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B794; return 10; }
    8
}
/// $08B784: bpl.w $8b790
pub fn i_08b784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B788;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8B790; return 10; }
    8
}
/// $08B788: subi.w #$12, d1
pub fn i_08b788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B78C;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x12, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B78C: bra.w $8b794
pub fn i_08b78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B790;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B794;
    10
}
/// $08B790: addi.w #$12, d1
pub fn i_08b790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B794;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x12, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B794: rts
pub fn i_08b794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B796;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_SteerY_08b796  ($08B796, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B796: move.w $80(a6), d0
pub fn i_08b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B79A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B79A: sub.w $24(a6), d0
pub fn i_08b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B79E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08B79E: beq.w $8b7ae
pub fn i_08b79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B7AE; return 10; }
    8
}
/// $08B7A2: bpl.w $8b7ac
pub fn i_08b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7A6;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8B7AC; return 10; }
    8
}
/// $08B7A6: subq.w #$4, d2
pub fn i_08b7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7A8;
    cpu.ir = 0x5942;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B7A8: bra.w $8b7ae
pub fn i_08b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B7AE;
    10
}
/// $08B7AC: addq.w #$4, d2
pub fn i_08b7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7AE;
    cpu.ir = 0x5842;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B7AE: rts
pub fn i_08b7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_ProbeGround_08b7b0  ($08B7B0, 48 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B7B0: move.w $22(a6), d1
pub fn i_08b7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7B4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7B4: move.w $24(a6), d2
pub fn i_08b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7B8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7B8: addi.w #$a8, d1
pub fn i_08b7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7BC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B7BC: addi.w #$10, d2
pub fn i_08b7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7C0;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B7C0: jsr $280c6.l
pub fn i_08b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B7C6);
    cpu.pc = 0x280C6;
    16
}
/// $08B7C6: bcc.w $8b7e6
pub fn i_08b7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B7E6; return 10; }
    8
}
/// $08B7CA: cmpi.b #$25, d7
pub fn i_08b7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7CE;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x25, Size::Byte, false);
    8
}
/// $08B7CE: bne.w $8b7e6
pub fn i_08b7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B7E6; return 10; }
    8
}
/// $08B7D2: cmpi.b #$0, $83(a6)
pub fn i_08b7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7D8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B7D8: bne.w $8b7e6
pub fn i_08b7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B7E6; return 10; }
    8
}
/// $08B7DC: move.b d0, $83(a6)
pub fn i_08b7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7E0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_08b7e0  ($08B7E0, 6 B, thunks/ccr_helpers.c)
/// $08B7E0: ori.b #$11, ccr
pub fn i_08b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7E4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B7E4: rts
pub fn i_08b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_ProbeGround_Tail_08b7e6  ($08B7E6, 4 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B7E6: move.b d0, $83(a6)
pub fn i_08b7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7EA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_08b7ea  ($08B7EA, 6 B, thunks/ccr_helpers.c)
/// $08B7EA: andi.b #$ee, ccr
pub fn i_08b7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B7EE: rts
pub fn i_08b7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_CheckLanding_08b7f0  ($08B7F0, 48 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B7F0: move.w $22(a6), d1
pub fn i_08b7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7F4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7F4: move.w $24(a6), d2
pub fn i_08b7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7F8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7F8: addi.w #$a8, d1
pub fn i_08b7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7FC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B7FC: addi.w #$10, d2
pub fn i_08b7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B800;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B800: jsr $280c6.l
pub fn i_08b800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B806);
    cpu.pc = 0x280C6;
    16
}
/// $08B806: bcc.w $8b826
pub fn i_08b806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B80A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B826; return 10; }
    8
}
/// $08B80A: cmpi.b #$25, d7
pub fn i_08b80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B80E;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x25, Size::Byte, false);
    8
}
/// $08B80E: beq.w $8b820
pub fn i_08b80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B812;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B820; return 10; }
    8
}
/// $08B812: cmpi.b #$1e, d7
pub fn i_08b812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B816;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x1E, Size::Byte, false);
    8
}
/// $08B816: bne.w $8b826
pub fn i_08b816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B81A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B826; return 10; }
    8
}
/// $08B81A: lea.l $895fe(pc), a1
pub fn i_08b81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B81E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x895FE;
    16
}
/// $08B81E: move.l a1, (a6)
pub fn i_08b81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== SetXN_08b820  ($08B820, 6 B, thunks/ccr_helpers.c)
/// $08B820: ori.b #$11, ccr
pub fn i_08b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B824;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B824: rts
pub fn i_08b824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B826;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_08b826  ($08B826, 6 B, thunks/ccr_helpers.c)
/// $08B826: andi.b #$ee, ccr
pub fn i_08b826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B82A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B82A: rts
pub fn i_08b82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B82C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_DropSoldier_08b82c  ($08B82C, 54 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B82C: move.w #$a, $84(a6)
pub fn i_08b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B832;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B832: move.l $106f50.l, d0
pub fn i_08b832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B838;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08B838: swap d0
pub fn i_08b838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B83A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08B83A: cmpi.w #$9a0, d0
pub fn i_08b83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B83E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9A0, Size::Word, false);
    8
}
/// $08B83E: bge.w $8b860
pub fn i_08b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B842;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8B860; return 10; }
    8
}
/// $08B842: lea.l $631d0.l, a1
pub fn i_08b842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B848;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x631D0;
    20
}
/// $08B848: jsr $4ae.l
pub fn i_08b848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B84E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B84E);
    cpu.pc = 0x4AE;
    16
}
/// $08B84E: jsr $5dd22.l
pub fn i_08b84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B854);
    cpu.pc = 0x5DD22;
    16
}
/// $08B854: addi.w #$b8, $22(a0)
pub fn i_08b854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B85A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xB8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B85A: addi.w #$50, $24(a0)
pub fn i_08b85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B860;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B860: rts
pub fn i_08b860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B862;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Recoil_08b862  ($08B862, 78 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B862: cmpi.w #$0, $84(a6)
pub fn i_08b862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B868;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08B868: beq.w $8b8ae
pub fn i_08b868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B86C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B8AE; return 10; }
    8
}
/// $08B86C: subq.w #$1, $84(a6)
pub fn i_08b86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B870;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B870: move.w #$200, d1
pub fn i_08b870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B874;
    cpu.ir = 0x323C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B874: move.b $106f28.l, d0
pub fn i_08b874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B87A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B87A: btst.b #$0, d0
pub fn i_08b87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B87E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B87E: bne.w $8b884
pub fn i_08b87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B882;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B884; return 10; }
    8
}
/// $08B882: neg.w d1
pub fn i_08b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B884;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B884: move.w $28(a6), d2
pub fn i_08b884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B888;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B888: move.w $2a(a6), d3
pub fn i_08b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B88C;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B88C: movem.w d2, -(a7)
pub fn i_08b88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B890;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    cpu.a[7] = a;
    20
}
/// $08B890: movem.w d3, -(a7)
pub fn i_08b890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B894;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    cpu.a[7] = a;
    20
}
/// $08B894: move.w d1, $2a(a6)
pub fn i_08b894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B898;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B898: jsr $27cee.l
pub fn i_08b898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B89E);
    cpu.pc = 0x27CEE;
    16
}
/// $08B89E: movem.w (a7)+, d3
pub fn i_08b89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8A2;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08B8A2: movem.w (a7)+, d2
pub fn i_08b8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8A6;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08B8A6: move.w d2, $28(a6)
pub fn i_08b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8AA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B8AA: move.w d3, $2a(a6)
pub fn i_08b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8AE;
    cpu.ir = 0x3D43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B8AE: rts
pub fn i_08b8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_TrailRecord_08b8b0  ($08B8B0, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8B0: move.w $22(a6), d0
pub fn i_08b8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8B4: move.w $24(a6), d1
pub fn i_08b8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8B8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8B8: subi.w #$a0, d0
pub fn i_08b8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8BC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xA0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8BC: addi.w #$2e, d1
pub fn i_08b8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8C0;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2E, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8C0: move.w #$140, d2
pub fn i_08b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8C4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B8C4: jmp $99812.l
pub fn i_08b8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x99812;
    8
}
// ===== Airship_TrailRecordAlt_08b8ca  ($08B8CA, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8CA: move.w $22(a6), d0
pub fn i_08b8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8CE: move.w $24(a6), d1
pub fn i_08b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8D2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8D2: subi.w #$a0, d0
pub fn i_08b8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8D6;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xA0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8D6: addi.w #$2e, d1
pub fn i_08b8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8DA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2E, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8DA: move.w #$140, d2
pub fn i_08b8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8DE;
    cpu.ir = 0x343C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B8DE: jmp $997e2.l
pub fn i_08b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8E4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x997E2;
    8
}
// ===== Camera_PublishLockX_08b8e4  ($08B8E4, 16 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8E4: move.w $22(a6), d0
pub fn i_08b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8E8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8E8: addi.w #$70, d0
pub fn i_08b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8EC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x70, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8EC: move.w d0, $106f42.l
pub fn i_08b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8F2;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F42, v1 as u16);
    16
}
/// $08B8F2: rts
pub fn i_08b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Tent_BlitRowByIdx_08b8f4  ($08B8F4, 18 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8F4: clr.l d0
pub fn i_08b8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8F6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B8F6: move.b $21(a6), d0
pub fn i_08b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8FA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B8FA: asl.l #$2, d0
pub fn i_08b8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8FC;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $08B8FC: lea.l $2ef61a.l, a0
pub fn i_08b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B902;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF61A;
    20
}
/// $08B902: movea.l (a0, d0.w), a1
pub fn i_08b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B906;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
// ===== JsrAbsThunk_08b906  ($08B906, 8 B, thunks/jsr_abs_thunks.c)
/// $08B906: jsr $43fac.l
pub fn i_08b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B90C);
    cpu.pc = 0x43FAC;
    16
}
/// $08B90C: rts
pub fn i_08b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B90E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Bunker_BlitByIdx_08b90e  ($08B90E, 18 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B90E: clr.l d0
pub fn i_08b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B910;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B910: move.b $21(a6), d0
pub fn i_08b910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B914;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B914: asl.l #$2, d0
pub fn i_08b914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B916;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $08B916: lea.l $2ef39c.l, a2
pub fn i_08b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B91C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF39C;
    20
}
/// $08B91C: movea.l (a2, d0.w), a2
pub fn i_08b91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B920;
    cpu.ir = 0x2472;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_08b920  ($08B920, 8 B, thunks/jsr_abs_thunks.c)
/// $08B920: jsr $5022a.l
pub fn i_08b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B926;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B926);
    cpu.pc = 0x5022A;
    16
}
/// $08B926: rts
pub fn i_08b926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B928;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08b928  ($08B928, 16 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B928: movea.l $8(a6), a1
pub fn i_08b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B92C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08B92C: move.b $10(a6), d0
pub fn i_08b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B930;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B930: cmp.b $10(a1), d0
pub fn i_08b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B934;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08B934: bcs.w $8b93e
pub fn i_08b934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B938;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8B93E; return 10; }
    8
}
// ===== ClearXN_08b938  ($08B938, 6 B, thunks/ccr_helpers.c)
/// $08B938: andi.b #$ee, ccr
pub fn i_08b938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B93C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B93C: rts
pub fn i_08b93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B93E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08b93e  ($08B93E, 6 B, thunks/ccr_helpers.c)
/// $08B93E: ori.b #$11, ccr
pub fn i_08b93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B942;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B942: rts
pub fn i_08b942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B944;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_08b950  ($08B950, 82 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B950: ori.b #$ff, d2
pub fn i_08b950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B954;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08B954: dc.w $ffff
pub fn i_08b954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B956;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Proj_Tmpl168_08b9a2  ($08B9A2, 8 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9A2: jsr $8bb5e(pc)
pub fn i_08b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B9A6);
    cpu.pc = 0x8BB5E;
    16
}
/// $08B9A6: bra.w $8ba0c
pub fn i_08b9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BA0C;
    10
}
// ===== Proj_Tmpl169_08b9aa  ($08B9AA, 8 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9AA: jsr $8bb34(pc)
pub fn i_08b9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B9AE);
    cpu.pc = 0x8BB34;
    16
}
/// $08B9AE: bra.w $8b9ba
pub fn i_08b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B9BA;
    10
}
// ===== Proj_Tmpl170_08b9b2  ($08B9B2, 8 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9B2: jsr $8bb34(pc)
pub fn i_08b9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B9B6);
    cpu.pc = 0x8BB34;
    16
}
/// $08B9B6: bra.w $8ba52
pub fn i_08b9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BA52;
    10
}
// ===== Proj_Bounce_08b9ba  ($08B9BA, 74 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9BA: move.w #$0, $28(a6)
pub fn i_08b9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B9C0: lea.l $29b744.l, a0
pub fn i_08b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $08B9C6: jsr $28cd4.l
pub fn i_08b9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9CC);
    cpu.pc = 0x28CD4;
    16
}
/// $08B9CC: lea.l $29ca20.l, a0
pub fn i_08b9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CA20;
    20
}
/// $08B9D2: jsr $28cd4.l
pub fn i_08b9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9D8);
    cpu.pc = 0x28CD4;
    16
}
/// $08B9D8: lea.l $8b9de(pc), a1
pub fn i_08b9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B9DE;
    16
}
/// $08B9DC: move.l a1, (a6)
pub fn i_08b9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B9DE: jsr $2783a.l
pub fn i_08b9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9E4);
    cpu.pc = 0x2783A;
    16
}
/// $08B9E4: jsr $28d70.l
pub fn i_08b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9EA);
    cpu.pc = 0x28D70;
    16
}
/// $08B9EA: bcc.w $8b9f4
pub fn i_08b9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B9F4; return 10; }
    8
}
/// $08B9EE: lea.l $8b9ba(pc), a1
pub fn i_08b9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B9BA;
    16
}
/// $08B9F2: move.l a1, (a6)
pub fn i_08b9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B9F4: jsr $2870a.l
pub fn i_08b9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9FA);
    cpu.pc = 0x2870A;
    16
}
/// $08B9FA: lea.l $8b944(pc), a0
pub fn i_08b9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B944;
    16
}
/// $08B9FE: movea.l #$ffffffff, a1
pub fn i_08b9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA04;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ba04  ($08BA04, 8 B, thunks/jsr_abs_thunks.c)
/// $08BA04: jsr $772.l
pub fn i_08ba04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA0A);
    cpu.pc = 0x772;
    16
}
/// $08BA0A: rts
pub fn i_08ba0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Bounce_V1_08ba0c  ($08BA0C, 62 B, mission/cutscene_anim_08ba0c.s)
/// $08BA0C: move.w #$0, $28(a6)
pub fn i_08ba0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA12;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BA12: lea.l $29ca0a.l, a0
pub fn i_08ba12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CA0A;
    20
}
/// $08BA18: jsr $28cd4.l
pub fn i_08ba18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA1E);
    cpu.pc = 0x28CD4;
    16
}
/// $08BA1E: lea.l $8ba24(pc), a1
pub fn i_08ba1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA24;
    16
}
/// $08BA22: move.l a1, (a6)
pub fn i_08ba22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA24: jsr $2783a.l
pub fn i_08ba24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA2A);
    cpu.pc = 0x2783A;
    16
}
/// $08BA2A: jsr $28d70.l
pub fn i_08ba2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA30);
    cpu.pc = 0x28D70;
    16
}
/// $08BA30: bcc.w $8ba3a
pub fn i_08ba30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BA3A; return 10; }
    8
}
/// $08BA34: lea.l $8ba0c(pc), a1
pub fn i_08ba34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA0C;
    16
}
/// $08BA38: move.l a1, (a6)
pub fn i_08ba38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA3A: jsr $2870a.l
pub fn i_08ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA40);
    cpu.pc = 0x2870A;
    16
}
/// $08BA40: lea.l $8b944(pc), a0
pub fn i_08ba40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA44;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B944;
    16
}
/// $08BA44: movea.l #$ffffffff, a1
pub fn i_08ba44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA4A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ba4a  ($08BA4A, 8 B, thunks/jsr_abs_thunks.c)
/// $08BA4A: jsr $772.l
pub fn i_08ba4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA50);
    cpu.pc = 0x772;
    16
}
/// $08BA50: rts
pub fn i_08ba50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Bounce_V2_08ba52  ($08BA52, 68 B, mission/cutscene_anim_08ba0c.s)
/// $08BA52: bset.b #$0, $3a(a6)
pub fn i_08ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA58;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08BA58: move.w #$0, $28(a6)
pub fn i_08ba58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA5E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BA5E: lea.l $29bfc4.l, a0
pub fn i_08ba5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFC4;
    20
}
/// $08BA64: jsr $28cd4.l
pub fn i_08ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA6A);
    cpu.pc = 0x28CD4;
    16
}
/// $08BA6A: lea.l $8ba70(pc), a1
pub fn i_08ba6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA70;
    16
}
/// $08BA6E: move.l a1, (a6)
pub fn i_08ba6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA70: jsr $2783a.l
pub fn i_08ba70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA76);
    cpu.pc = 0x2783A;
    16
}
/// $08BA76: jsr $28d70.l
pub fn i_08ba76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA7C);
    cpu.pc = 0x28D70;
    16
}
/// $08BA7C: bcc.w $8ba86
pub fn i_08ba7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BA86; return 10; }
    8
}
/// $08BA80: lea.l $8ba0c(pc), a1
pub fn i_08ba80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA0C;
    16
}
/// $08BA84: move.l a1, (a6)
pub fn i_08ba84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA86: jsr $2870a.l
pub fn i_08ba86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA8C);
    cpu.pc = 0x2870A;
    16
}
/// $08BA8C: lea.l $8b944(pc), a0
pub fn i_08ba8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B944;
    16
}
/// $08BA90: movea.l #$ffffffff, a1
pub fn i_08ba90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA96;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ba96  ($08BA96, 8 B, thunks/jsr_abs_thunks.c)
/// $08BA96: jsr $772.l
pub fn i_08ba96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA9C);
    cpu.pc = 0x772;
    16
}
/// $08BA9C: rts
pub fn i_08ba9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Burst_RandDispatch_08babe  ($08BABE, 28 B, mission/cutscene_anim_08ba0c.s)
/// $08BABE: jsr $5e9b6.l
pub fn i_08babe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAC4);
    cpu.pc = 0x5E9B6;
    16
}
/// $08BAC4: andi.w #$3, d0
pub fn i_08bac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAC8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BAC8: lea.l $8ba9e(pc), a0
pub fn i_08bac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BACC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8BA9E;
    16
}
/// $08BACC: movea.l #$ffffffff, a1
pub fn i_08bacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAD2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08BAD2: jsr $772.l
pub fn i_08bad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAD8);
    cpu.pc = 0x772;
    16
}
/// $08BAD8: jmp (a1)
pub fn i_08bad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BADA;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
// ===== Proj_Burst_RandDispatch2_08bada  ($08BADA, 28 B, mission/cutscene_anim_08ba0c.s)
/// $08BADA: jsr $5e9b6.l
pub fn i_08bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAE0);
    cpu.pc = 0x5E9B6;
    16
}
/// $08BAE0: andi.w #$3, d0
pub fn i_08bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAE4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BAE4: lea.l $8baae(pc), a0
pub fn i_08bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAE8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8BAAE;
    16
}
/// $08BAE8: movea.l #$ffffffff, a1
pub fn i_08bae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAEE;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08BAEE: jsr $772.l
pub fn i_08baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAF4);
    cpu.pc = 0x772;
    16
}
/// $08BAF4: jmp (a1)
pub fn i_08baf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAF6;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
// ===== Proj_Burst_08baf6  ($08BAF6, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08BAF6: move.w #$2f, d1
pub fn i_08baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BAFA: jsr $236e.l
pub fn i_08bafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB00);
    cpu.pc = 0x236E;
    16
}
/// $08BB00: lea.l $4b136.l, a0
pub fn i_08bb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x4B136;
    20
}
/// $08BB06: jsr $28cd4.l
pub fn i_08bb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB0C);
    cpu.pc = 0x28CD4;
    16
}
/// $08BB0C: lea.l $8bb12(pc), a1
pub fn i_08bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BB12;
    16
}
/// $08BB10: move.l a1, (a6)
pub fn i_08bb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BB12: jsr $2783a.l
pub fn i_08bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB18);
    cpu.pc = 0x2783A;
    16
}
/// $08BB18: jsr $28d70.l
pub fn i_08bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB1E);
    cpu.pc = 0x28D70;
    16
}
/// $08BB1E: bcc.w $8bb28
pub fn i_08bb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BB28; return 10; }
    8
}
// ===== SetTaskHandler_08bb22  ($08BB22, 8 B, kernel/task_handlers.c)
/// $08BB22: lea.l $8bb84(pc), a1
pub fn i_08bb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BB84;
    16
}
/// $08BB26: move.l a1, (a6)
pub fn i_08bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BB28: rts
pub fn i_08bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeTransformLoop_08bb2a  ($08BB2A, 10 B, mission/cutscene_anim_08ba0c.s)
/// $08BB2A: jsr $27c8c.l
pub fn i_08bb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB30);
    cpu.pc = 0x27C8C;
    16
}
/// $08BB30: bcc.b $8bb2a
pub fn i_08bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB32;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BB2A; return 10; }
    8
}
/// $08BB32: rts
pub fn i_08bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Tmpl_InitHitboxProbe_08bb34  ($08BB34, 34 B, mission/cutscene_anim_08ba0c.s)
/// $08BB34: move.w #$e, d1
pub fn i_08bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB38;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BB38: jsr $236e.l
pub fn i_08bb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB3E);
    cpu.pc = 0x236E;
    16
}
/// $08BB3E: lea.l $8b950(pc), a0
pub fn i_08bb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB42;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B950;
    16
}
/// $08BB42: move.l a0, $48(a6)
pub fn i_08bb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB46;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BB46: jsr $8bb2a(pc)
pub fn i_08bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8BB4A);
    cpu.pc = 0x8BB2A;
    16
}
/// $08BB4A: lea.l $776e2.l, a1
pub fn i_08bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $08BB50: jsr $4ae.l
pub fn i_08bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB56);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_08bb56  ($08BB56, 8 B, thunks/jsr_abs_thunks.c)
/// $08BB56: jsr $5dd02.l
pub fn i_08bb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB5C);
    cpu.pc = 0x5DD02;
    16
}
/// $08BB5C: rts
pub fn i_08bb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Tmpl_InitHitbox_08bb5e  ($08BB5E, 30 B, mission/cutscene_anim_08ba0c.s)
/// $08BB5E: move.w #$e, d1
pub fn i_08bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB62;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BB62: jsr $236e.l
pub fn i_08bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB68);
    cpu.pc = 0x236E;
    16
}
/// $08BB68: lea.l $8b950(pc), a0
pub fn i_08bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB6C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B950;
    16
}
/// $08BB6C: move.l a0, $48(a6)
pub fn i_08bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB70;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BB70: lea.l $776e2.l, a1
pub fn i_08bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $08BB76: jsr $4ae.l
pub fn i_08bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB7C);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_08bb7c  ($08BB7C, 8 B, thunks/jsr_abs_thunks.c)
/// $08BB7C: jsr $5dd02.l
pub fn i_08bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB82);
    cpu.pc = 0x5DD02;
    16
}
/// $08BB82: rts
pub fn i_08bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x089D9E, i_089d9e),
    (0x089DA4, i_089da4),
    (0x089DC2, i_089dc2),
    (0x089DC6, i_089dc6),
    (0x089DCC, i_089dcc),
    (0x089DD2, i_089dd2),
    (0x089DD8, i_089dd8),
    (0x089DDE, i_089dde),
    (0x089DE4, i_089de4),
    (0x089DEA, i_089dea),
    (0x089DF0, i_089df0),
    (0x089DF6, i_089df6),
    (0x089DFA, i_089dfa),
    (0x089DFC, i_089dfc),
    (0x089E02, i_089e02),
    (0x089E08, i_089e08),
    (0x089E0E, i_089e0e),
    (0x089E14, i_089e14),
    (0x089E1A, i_089e1a),
    (0x089E20, i_089e20),
    (0x089E24, i_089e24),
    (0x089E2A, i_089e2a),
    (0x089E2C, i_089e2c),
    (0x089E30, i_089e30),
    (0x089E36, i_089e36),
    (0x089E3C, i_089e3c),
    (0x089E42, i_089e42),
    (0x089E46, i_089e46),
    (0x089E4C, i_089e4c),
    (0x089E52, i_089e52),
    (0x089E58, i_089e58),
    (0x089E5E, i_089e5e),
    (0x089E64, i_089e64),
    (0x089E6A, i_089e6a),
    (0x089E6E, i_089e6e),
    (0x089E70, i_089e70),
    (0x089E76, i_089e76),
    (0x089E7C, i_089e7c),
    (0x089E82, i_089e82),
    (0x089E86, i_089e86),
    (0x089E8C, i_089e8c),
    (0x089E8E, i_089e8e),
    (0x089E92, i_089e92),
    (0x089E98, i_089e98),
    (0x089E9E, i_089e9e),
    (0x089EA2, i_089ea2),
    (0x089EA8, i_089ea8),
    (0x089EAE, i_089eae),
    (0x089EB4, i_089eb4),
    (0x089EBA, i_089eba),
    (0x089EBE, i_089ebe),
    (0x089EC4, i_089ec4),
    (0x089ECA, i_089eca),
    (0x089ED0, i_089ed0),
    (0x089ED6, i_089ed6),
    (0x089EDA, i_089eda),
    (0x089EE0, i_089ee0),
    (0x089EE6, i_089ee6),
    (0x089EEC, i_089eec),
    (0x089EF0, i_089ef0),
    (0x089EF6, i_089ef6),
    (0x089EFC, i_089efc),
    (0x089F02, i_089f02),
    (0x089F06, i_089f06),
    (0x089F0C, i_089f0c),
    (0x089F12, i_089f12),
    (0x089F18, i_089f18),
    (0x089F1C, i_089f1c),
    (0x089F1E, i_089f1e),
    (0x089F24, i_089f24),
    (0x089F2A, i_089f2a),
    (0x089F30, i_089f30),
    (0x089F34, i_089f34),
    (0x089F38, i_089f38),
    (0x089F3E, i_089f3e),
    (0x089F44, i_089f44),
    (0x089F46, i_089f46),
    (0x089F4C, i_089f4c),
    (0x089F52, i_089f52),
    (0x089F56, i_089f56),
    (0x089F58, i_089f58),
    (0x089F5C, i_089f5c),
    (0x089F62, i_089f62),
    (0x089F68, i_089f68),
    (0x089F6E, i_089f6e),
    (0x089F74, i_089f74),
    (0x089F7A, i_089f7a),
    (0x089F80, i_089f80),
    (0x089F86, i_089f86),
    (0x089F8C, i_089f8c),
    (0x089F90, i_089f90),
    (0x089F94, i_089f94),
    (0x089F96, i_089f96),
    (0x089F9C, i_089f9c),
    (0x089FA0, i_089fa0),
    (0x089FA6, i_089fa6),
    (0x089FAA, i_089faa),
    (0x089FB0, i_089fb0),
    (0x089FB4, i_089fb4),
    (0x089FBA, i_089fba),
    (0x089FC0, i_089fc0),
    (0x089FC6, i_089fc6),
    (0x089FCC, i_089fcc),
    (0x089FD2, i_089fd2),
    (0x089FD8, i_089fd8),
    (0x089FDC, i_089fdc),
    (0x089FE0, i_089fe0),
    (0x089FE6, i_089fe6),
    (0x089FEC, i_089fec),
    (0x089FF2, i_089ff2),
    (0x089FF8, i_089ff8),
    (0x089FFE, i_089ffe),
    (0x08A004, i_08a004),
    (0x08A006, i_08a006),
    (0x08A00A, i_08a00a),
    (0x08A010, i_08a010),
    (0x08A014, i_08a014),
    (0x08A016, i_08a016),
    (0x08A01E, i_08a01e),
    (0x08A022, i_08a022),
    (0x08A024, i_08a024),
    (0x08A02A, i_08a02a),
    (0x08A02E, i_08a02e),
    (0x08A030, i_08a030),
    (0x08A036, i_08a036),
    (0x08A03A, i_08a03a),
    (0x08A03E, i_08a03e),
    (0x08A044, i_08a044),
    (0x08A04A, i_08a04a),
    (0x08A052, i_08a052),
    (0x08A056, i_08a056),
    (0x08A05C, i_08a05c),
    (0x08A05E, i_08a05e),
    (0x08A062, i_08a062),
    (0x08A068, i_08a068),
    (0x08A06E, i_08a06e),
    (0x08A074, i_08a074),
    (0x08A07A, i_08a07a),
    (0x08A080, i_08a080),
    (0x08A086, i_08a086),
    (0x08A08C, i_08a08c),
    (0x08A092, i_08a092),
    (0x08A098, i_08a098),
    (0x08A09E, i_08a09e),
    (0x08A0A4, i_08a0a4),
    (0x08A0A8, i_08a0a8),
    (0x08A0AC, i_08a0ac),
    (0x08A0B0, i_08a0b0),
    (0x08A0B4, i_08a0b4),
    (0x08A0BA, i_08a0ba),
    (0x08A0BE, i_08a0be),
    (0x08A0C4, i_08a0c4),
    (0x08A0C6, i_08a0c6),
    (0x08A0CA, i_08a0ca),
    (0x08A0D0, i_08a0d0),
    (0x08A0D4, i_08a0d4),
    (0x08A0DA, i_08a0da),
    (0x08A0DE, i_08a0de),
    (0x08A0E2, i_08a0e2),
    (0x08A0E6, i_08a0e6),
    (0x08A0EA, i_08a0ea),
    (0x08A0EC, i_08a0ec),
    (0x08A0F0, i_08a0f0),
    (0x08A0F4, i_08a0f4),
    (0x08A0FA, i_08a0fa),
    (0x08A100, i_08a100),
    (0x08A104, i_08a104),
    (0x08A10A, i_08a10a),
    (0x08A10C, i_08a10c),
    (0x08A110, i_08a110),
    (0x08A116, i_08a116),
    (0x08A11A, i_08a11a),
    (0x08A120, i_08a120),
    (0x08A124, i_08a124),
    (0x08A12A, i_08a12a),
    (0x08A130, i_08a130),
    (0x08A136, i_08a136),
    (0x08A13C, i_08a13c),
    (0x08A142, i_08a142),
    (0x08A148, i_08a148),
    (0x08A14E, i_08a14e),
    (0x08A156, i_08a156),
    (0x08A15A, i_08a15a),
    (0x08A160, i_08a160),
    (0x08A164, i_08a164),
    (0x08A16A, i_08a16a),
    (0x08A170, i_08a170),
    (0x08A174, i_08a174),
    (0x08A17A, i_08a17a),
    (0x08A180, i_08a180),
    (0x08A186, i_08a186),
    (0x08A18C, i_08a18c),
    (0x08A192, i_08a192),
    (0x08A196, i_08a196),
    (0x08A19E, i_08a19e),
    (0x08A1A2, i_08a1a2),
    (0x08A1A8, i_08a1a8),
    (0x08A1AE, i_08a1ae),
    (0x08A1B4, i_08a1b4),
    (0x08A1BA, i_08a1ba),
    (0x08A1C0, i_08a1c0),
    (0x08A1C4, i_08a1c4),
    (0x08A1CC, i_08a1cc),
    (0x08A1D0, i_08a1d0),
    (0x08A1D6, i_08a1d6),
    (0x08A1DC, i_08a1dc),
    (0x08A1E2, i_08a1e2),
    (0x08A1E8, i_08a1e8),
    (0x08A1EE, i_08a1ee),
    (0x08A1F2, i_08a1f2),
    (0x08A1FA, i_08a1fa),
    (0x08A200, i_08a200),
    (0x08A206, i_08a206),
    (0x08A20C, i_08a20c),
    (0x08A212, i_08a212),
    (0x08A218, i_08a218),
    (0x08A21E, i_08a21e),
    (0x08A224, i_08a224),
    (0x08A22A, i_08a22a),
    (0x08A230, i_08a230),
    (0x08A236, i_08a236),
    (0x08A23C, i_08a23c),
    (0x08A240, i_08a240),
    (0x08A244, i_08a244),
    (0x08A246, i_08a246),
    (0x08A24C, i_08a24c),
    (0x08A252, i_08a252),
    (0x08A258, i_08a258),
    (0x08A25C, i_08a25c),
    (0x08A262, i_08a262),
    (0x08A266, i_08a266),
    (0x08A26C, i_08a26c),
    (0x08A270, i_08a270),
    (0x08A276, i_08a276),
    (0x08A27C, i_08a27c),
    (0x08A282, i_08a282),
    (0x08A288, i_08a288),
    (0x08A28C, i_08a28c),
    (0x08A292, i_08a292),
    (0x08A298, i_08a298),
    (0x08A29C, i_08a29c),
    (0x08A2A4, i_08a2a4),
    (0x08A2A8, i_08a2a8),
    (0x08A2AE, i_08a2ae),
    (0x08A2B4, i_08a2b4),
    (0x08A2BA, i_08a2ba),
    (0x08A2C0, i_08a2c0),
    (0x08A2C6, i_08a2c6),
    (0x08A2CC, i_08a2cc),
    (0x08A2D2, i_08a2d2),
    (0x08A2D8, i_08a2d8),
    (0x08A2DE, i_08a2de),
    (0x08A2E4, i_08a2e4),
    (0x08A2E8, i_08a2e8),
    (0x08A2EA, i_08a2ea),
    (0x08A2EC, i_08a2ec),
    (0x08A2F0, i_08a2f0),
    (0x08A2F6, i_08a2f6),
    (0x08A2FC, i_08a2fc),
    (0x08A302, i_08a302),
    (0x08A308, i_08a308),
    (0x08A30E, i_08a30e),
    (0x08A314, i_08a314),
    (0x08A318, i_08a318),
    (0x08A31A, i_08a31a),
    (0x08A31C, i_08a31c),
    (0x08A322, i_08a322),
    (0x08A328, i_08a328),
    (0x08A32E, i_08a32e),
    (0x08A334, i_08a334),
    (0x08A33A, i_08a33a),
    (0x08A340, i_08a340),
    (0x08A346, i_08a346),
    (0x08A34C, i_08a34c),
    (0x08A352, i_08a352),
    (0x08A358, i_08a358),
    (0x08A35E, i_08a35e),
    (0x08A362, i_08a362),
    (0x08A366, i_08a366),
    (0x08A368, i_08a368),
    (0x08A36E, i_08a36e),
    (0x08A370, i_08a370),
    (0x08A374, i_08a374),
    (0x08A378, i_08a378),
    (0x08A37E, i_08a37e),
    (0x08A384, i_08a384),
    (0x08A38A, i_08a38a),
    (0x08A390, i_08a390),
    (0x08A394, i_08a394),
    (0x08A39A, i_08a39a),
    (0x08A3A0, i_08a3a0),
    (0x08A3A6, i_08a3a6),
    (0x08A3AC, i_08a3ac),
    (0x08A3AE, i_08a3ae),
    (0x08A3B4, i_08a3b4),
    (0x08A3BA, i_08a3ba),
    (0x08A3BE, i_08a3be),
    (0x08A3C2, i_08a3c2),
    (0x08A3C4, i_08a3c4),
    (0x08A3CA, i_08a3ca),
    (0x08A3D0, i_08a3d0),
    (0x08A3D6, i_08a3d6),
    (0x08A3DA, i_08a3da),
    (0x08A3E0, i_08a3e0),
    (0x08A3E4, i_08a3e4),
    (0x08A3EA, i_08a3ea),
    (0x08A3F0, i_08a3f0),
    (0x08A3F6, i_08a3f6),
    (0x08A3FC, i_08a3fc),
    (0x08A400, i_08a400),
    (0x08A406, i_08a406),
    (0x08A40C, i_08a40c),
    (0x08A412, i_08a412),
    (0x08A418, i_08a418),
    (0x08A41C, i_08a41c),
    (0x08A420, i_08a420),
    (0x08A426, i_08a426),
    (0x08A42C, i_08a42c),
    (0x08A432, i_08a432),
    (0x08A438, i_08a438),
    (0x08A43E, i_08a43e),
    (0x08A444, i_08a444),
    (0x08A448, i_08a448),
    (0x08A44A, i_08a44a),
    (0x08A44C, i_08a44c),
    (0x08A452, i_08a452),
    (0x08A458, i_08a458),
    (0x08A45E, i_08a45e),
    (0x08A464, i_08a464),
    (0x08A46A, i_08a46a),
    (0x08A470, i_08a470),
    (0x08A476, i_08a476),
    (0x08A47C, i_08a47c),
    (0x08A482, i_08a482),
    (0x08A488, i_08a488),
    (0x08A48C, i_08a48c),
    (0x08A48E, i_08a48e),
    (0x08A494, i_08a494),
    (0x08A49A, i_08a49a),
    (0x08A4A0, i_08a4a0),
    (0x08A4A4, i_08a4a4),
    (0x08A4AA, i_08a4aa),
    (0x08A4AE, i_08a4ae),
    (0x08A4B4, i_08a4b4),
    (0x08A4BA, i_08a4ba),
    (0x08A4C0, i_08a4c0),
    (0x08A4C6, i_08a4c6),
    (0x08A4CA, i_08a4ca),
    (0x08A4D0, i_08a4d0),
    (0x08A4D6, i_08a4d6),
    (0x08A4DC, i_08a4dc),
    (0x08A4E2, i_08a4e2),
    (0x08A4E6, i_08a4e6),
    (0x08A4EA, i_08a4ea),
    (0x08A4F0, i_08a4f0),
    (0x08A4F6, i_08a4f6),
    (0x08A4FC, i_08a4fc),
    (0x08A502, i_08a502),
    (0x08A508, i_08a508),
    (0x08A50E, i_08a50e),
    (0x08A512, i_08a512),
    (0x08A514, i_08a514),
    (0x08A516, i_08a516),
    (0x08A51C, i_08a51c),
    (0x08A522, i_08a522),
    (0x08A528, i_08a528),
    (0x08A52E, i_08a52e),
    (0x08A534, i_08a534),
    (0x08A53A, i_08a53a),
    (0x08A540, i_08a540),
    (0x08A546, i_08a546),
    (0x08A54C, i_08a54c),
    (0x08A552, i_08a552),
    (0x08A558, i_08a558),
    (0x08A55E, i_08a55e),
    (0x08A562, i_08a562),
    (0x08A564, i_08a564),
    (0x08A56A, i_08a56a),
    (0x08A570, i_08a570),
    (0x08A576, i_08a576),
    (0x08A57A, i_08a57a),
    (0x08A580, i_08a580),
    (0x08A584, i_08a584),
    (0x08A58A, i_08a58a),
    (0x08A590, i_08a590),
    (0x08A596, i_08a596),
    (0x08A59C, i_08a59c),
    (0x08A5A0, i_08a5a0),
    (0x08A5A6, i_08a5a6),
    (0x08A5AC, i_08a5ac),
    (0x08A5B2, i_08a5b2),
    (0x08A5B8, i_08a5b8),
    (0x08A5BC, i_08a5bc),
    (0x08A5C2, i_08a5c2),
    (0x08A5C8, i_08a5c8),
    (0x08A5CE, i_08a5ce),
    (0x08A5D2, i_08a5d2),
    (0x08A5D8, i_08a5d8),
    (0x08A5DE, i_08a5de),
    (0x08A5E4, i_08a5e4),
    (0x08A5EA, i_08a5ea),
    (0x08A5F0, i_08a5f0),
    (0x08A5F6, i_08a5f6),
    (0x08A5FC, i_08a5fc),
    (0x08A602, i_08a602),
    (0x08A608, i_08a608),
    (0x08A60E, i_08a60e),
    (0x08A614, i_08a614),
    (0x08A61A, i_08a61a),
    (0x08A620, i_08a620),
    (0x08A626, i_08a626),
    (0x08A62C, i_08a62c),
    (0x08A632, i_08a632),
    (0x08A638, i_08a638),
    (0x08A63E, i_08a63e),
    (0x08A644, i_08a644),
    (0x08A64A, i_08a64a),
    (0x08A650, i_08a650),
    (0x08A656, i_08a656),
    (0x08A65C, i_08a65c),
    (0x08A662, i_08a662),
    (0x08A668, i_08a668),
    (0x08A66E, i_08a66e),
    (0x08A674, i_08a674),
    (0x08A67A, i_08a67a),
    (0x08A680, i_08a680),
    (0x08A686, i_08a686),
    (0x08A68C, i_08a68c),
    (0x08A692, i_08a692),
    (0x08A698, i_08a698),
    (0x08A69E, i_08a69e),
    (0x08A6A4, i_08a6a4),
    (0x08A6AA, i_08a6aa),
    (0x08A6B0, i_08a6b0),
    (0x08A6B6, i_08a6b6),
    (0x08A6BC, i_08a6bc),
    (0x08A6C2, i_08a6c2),
    (0x08A6C8, i_08a6c8),
    (0x08A6CE, i_08a6ce),
    (0x08A6D4, i_08a6d4),
    (0x08A6DA, i_08a6da),
    (0x08A6E0, i_08a6e0),
    (0x08A6E6, i_08a6e6),
    (0x08A6EC, i_08a6ec),
    (0x08A6F2, i_08a6f2),
    (0x08A6F8, i_08a6f8),
    (0x08A6FE, i_08a6fe),
    (0x08A704, i_08a704),
    (0x08A708, i_08a708),
    (0x08A70E, i_08a70e),
    (0x08A714, i_08a714),
    (0x08A71A, i_08a71a),
    (0x08A71E, i_08a71e),
    (0x08A724, i_08a724),
    (0x08A72A, i_08a72a),
    (0x08A730, i_08a730),
    (0x08A736, i_08a736),
    (0x08A73C, i_08a73c),
    (0x08A742, i_08a742),
    (0x08A746, i_08a746),
    (0x08A74C, i_08a74c),
    (0x08A74E, i_08a74e),
    (0x08A752, i_08a752),
    (0x08A758, i_08a758),
    (0x08A75C, i_08a75c),
    (0x08A762, i_08a762),
    (0x08A766, i_08a766),
    (0x08A76C, i_08a76c),
    (0x08A772, i_08a772),
    (0x08A778, i_08a778),
    (0x08A77E, i_08a77e),
    (0x08A784, i_08a784),
    (0x08A78A, i_08a78a),
    (0x08A790, i_08a790),
    (0x08A798, i_08a798),
    (0x08A79C, i_08a79c),
    (0x08A7A2, i_08a7a2),
    (0x08A7A6, i_08a7a6),
    (0x08A7AC, i_08a7ac),
    (0x08A7B2, i_08a7b2),
    (0x08A7B6, i_08a7b6),
    (0x08A7BC, i_08a7bc),
    (0x08A7C2, i_08a7c2),
    (0x08A7C8, i_08a7c8),
    (0x08A7CE, i_08a7ce),
    (0x08A7D4, i_08a7d4),
    (0x08A7D8, i_08a7d8),
    (0x08A7DE, i_08a7de),
    (0x08A7E2, i_08a7e2),
    (0x08A7E6, i_08a7e6),
    (0x08A7EC, i_08a7ec),
    (0x08A7F2, i_08a7f2),
    (0x08A7F8, i_08a7f8),
    (0x08A7FE, i_08a7fe),
    (0x08A804, i_08a804),
    (0x08A808, i_08a808),
    (0x08A80E, i_08a80e),
    (0x08A812, i_08a812),
    (0x08A816, i_08a816),
    (0x08A81C, i_08a81c),
    (0x08A822, i_08a822),
    (0x08A828, i_08a828),
    (0x08A82E, i_08a82e),
    (0x08A834, i_08a834),
    (0x08A838, i_08a838),
    (0x08A83E, i_08a83e),
    (0x08A842, i_08a842),
    (0x08A846, i_08a846),
    (0x08A84C, i_08a84c),
    (0x08A852, i_08a852),
    (0x08A858, i_08a858),
    (0x08A85E, i_08a85e),
    (0x08A864, i_08a864),
    (0x08A868, i_08a868),
    (0x08A86E, i_08a86e),
    (0x08A872, i_08a872),
    (0x08A878, i_08a878),
    (0x08A87E, i_08a87e),
    (0x08A884, i_08a884),
    (0x08A88A, i_08a88a),
    (0x08A890, i_08a890),
    (0x08A896, i_08a896),
    (0x08A89C, i_08a89c),
    (0x08A8A2, i_08a8a2),
    (0x08A8A8, i_08a8a8),
    (0x08A8AE, i_08a8ae),
    (0x08A8B4, i_08a8b4),
    (0x08A8B8, i_08a8b8),
    (0x08A8BE, i_08a8be),
    (0x08A8C2, i_08a8c2),
    (0x08A8C4, i_08a8c4),
    (0x08A8CA, i_08a8ca),
    (0x08A8D0, i_08a8d0),
    (0x08A8D6, i_08a8d6),
    (0x08A8DA, i_08a8da),
    (0x08A8E0, i_08a8e0),
    (0x08A8E6, i_08a8e6),
    (0x08A8EA, i_08a8ea),
    (0x08A8F0, i_08a8f0),
    (0x08A8F4, i_08a8f4),
    (0x08A8FA, i_08a8fa),
    (0x08A900, i_08a900),
    (0x08A906, i_08a906),
    (0x08A90C, i_08a90c),
    (0x08A912, i_08a912),
    (0x08A916, i_08a916),
    (0x08A91C, i_08a91c),
    (0x08A922, i_08a922),
    (0x08A926, i_08a926),
    (0x08A92C, i_08a92c),
    (0x08A930, i_08a930),
    (0x08A938, i_08a938),
    (0x08A93C, i_08a93c),
    (0x08A942, i_08a942),
    (0x08A948, i_08a948),
    (0x08A94E, i_08a94e),
    (0x08A954, i_08a954),
    (0x08A95A, i_08a95a),
    (0x08A960, i_08a960),
    (0x08A966, i_08a966),
    (0x08A96C, i_08a96c),
    (0x08A972, i_08a972),
    (0x08A978, i_08a978),
    (0x08A97C, i_08a97c),
    (0x08A97E, i_08a97e),
    (0x08A980, i_08a980),
    (0x08A984, i_08a984),
    (0x08A98A, i_08a98a),
    (0x08A990, i_08a990),
    (0x08A996, i_08a996),
    (0x08A99C, i_08a99c),
    (0x08A9A2, i_08a9a2),
    (0x08A9A8, i_08a9a8),
    (0x08A9AC, i_08a9ac),
    (0x08A9AE, i_08a9ae),
    (0x08A9B0, i_08a9b0),
    (0x08A9B6, i_08a9b6),
    (0x08A9BC, i_08a9bc),
    (0x08A9C2, i_08a9c2),
    (0x08A9C8, i_08a9c8),
    (0x08A9CE, i_08a9ce),
    (0x08A9D4, i_08a9d4),
    (0x08A9DA, i_08a9da),
    (0x08A9E0, i_08a9e0),
    (0x08A9E6, i_08a9e6),
    (0x08A9EC, i_08a9ec),
    (0x08A9F2, i_08a9f2),
    (0x08A9F6, i_08a9f6),
    (0x08A9FC, i_08a9fc),
    (0x08AA00, i_08aa00),
    (0x08AA02, i_08aa02),
    (0x08AA08, i_08aa08),
    (0x08AA0A, i_08aa0a),
    (0x08AA0E, i_08aa0e),
    (0x08AA12, i_08aa12),
    (0x08AA18, i_08aa18),
    (0x08AA1E, i_08aa1e),
    (0x08AA24, i_08aa24),
    (0x08AA2A, i_08aa2a),
    (0x08AA2E, i_08aa2e),
    (0x08AA34, i_08aa34),
    (0x08AA3A, i_08aa3a),
    (0x08AA40, i_08aa40),
    (0x08AA46, i_08aa46),
    (0x08AA48, i_08aa48),
    (0x08AA4E, i_08aa4e),
    (0x08AA54, i_08aa54),
    (0x08AA58, i_08aa58),
    (0x08AA5C, i_08aa5c),
    (0x08AA5E, i_08aa5e),
    (0x08AA64, i_08aa64),
    (0x08AA6A, i_08aa6a),
    (0x08AA70, i_08aa70),
    (0x08AA76, i_08aa76),
    (0x08AA7A, i_08aa7a),
    (0x08AA80, i_08aa80),
    (0x08AA84, i_08aa84),
    (0x08AA8A, i_08aa8a),
    (0x08AA90, i_08aa90),
    (0x08AA96, i_08aa96),
    (0x08AA9C, i_08aa9c),
    (0x08AAA2, i_08aaa2),
    (0x08AAA6, i_08aaa6),
    (0x08AAAC, i_08aaac),
    (0x08AAB2, i_08aab2),
    (0x08AAB8, i_08aab8),
    (0x08AABE, i_08aabe),
    (0x08AAC2, i_08aac2),
    (0x08AAC6, i_08aac6),
    (0x08AACC, i_08aacc),
    (0x08AAD2, i_08aad2),
    (0x08AAD8, i_08aad8),
    (0x08AADE, i_08aade),
    (0x08AAE4, i_08aae4),
    (0x08AAEA, i_08aaea),
    (0x08AAEE, i_08aaee),
    (0x08AAF0, i_08aaf0),
    (0x08AAF2, i_08aaf2),
    (0x08AAF8, i_08aaf8),
    (0x08AAFE, i_08aafe),
    (0x08AB04, i_08ab04),
    (0x08AB0A, i_08ab0a),
    (0x08AB10, i_08ab10),
    (0x08AB16, i_08ab16),
    (0x08AB1C, i_08ab1c),
    (0x08AB22, i_08ab22),
    (0x08AB28, i_08ab28),
    (0x08AB2E, i_08ab2e),
    (0x08AB34, i_08ab34),
    (0x08AB3A, i_08ab3a),
    (0x08AB3E, i_08ab3e),
    (0x08AB40, i_08ab40),
    (0x08AB46, i_08ab46),
    (0x08AB4C, i_08ab4c),
    (0x08AB52, i_08ab52),
    (0x08AB58, i_08ab58),
    (0x08AB5C, i_08ab5c),
    (0x08AB62, i_08ab62),
    (0x08AB66, i_08ab66),
    (0x08AB6C, i_08ab6c),
    (0x08AB72, i_08ab72),
    (0x08AB78, i_08ab78),
    (0x08AB7E, i_08ab7e),
    (0x08AB84, i_08ab84),
    (0x08AB88, i_08ab88),
    (0x08AB8E, i_08ab8e),
    (0x08AB94, i_08ab94),
    (0x08AB9A, i_08ab9a),
    (0x08ABA0, i_08aba0),
    (0x08ABA4, i_08aba4),
    (0x08ABA8, i_08aba8),
    (0x08ABAE, i_08abae),
    (0x08ABB4, i_08abb4),
    (0x08ABBA, i_08abba),
    (0x08ABC0, i_08abc0),
    (0x08ABC6, i_08abc6),
    (0x08ABCC, i_08abcc),
    (0x08ABD0, i_08abd0),
    (0x08ABD2, i_08abd2),
    (0x08ABD4, i_08abd4),
    (0x08ABDA, i_08abda),
    (0x08ABE0, i_08abe0),
    (0x08ABE6, i_08abe6),
    (0x08ABEC, i_08abec),
    (0x08ABF2, i_08abf2),
    (0x08ABF8, i_08abf8),
    (0x08ABFE, i_08abfe),
    (0x08AC04, i_08ac04),
    (0x08AC0A, i_08ac0a),
    (0x08AC10, i_08ac10),
    (0x08AC16, i_08ac16),
    (0x08AC1C, i_08ac1c),
    (0x08AC20, i_08ac20),
    (0x08AC22, i_08ac22),
    (0x08AC28, i_08ac28),
    (0x08AC2E, i_08ac2e),
    (0x08AC34, i_08ac34),
    (0x08AC3A, i_08ac3a),
    (0x08AC3E, i_08ac3e),
    (0x08AC44, i_08ac44),
    (0x08AC48, i_08ac48),
    (0x08AC4E, i_08ac4e),
    (0x08AC54, i_08ac54),
    (0x08AC5A, i_08ac5a),
    (0x08AC60, i_08ac60),
    (0x08AC66, i_08ac66),
    (0x08AC6A, i_08ac6a),
    (0x08AC70, i_08ac70),
    (0x08AC76, i_08ac76),
    (0x08AC7C, i_08ac7c),
    (0x08AC82, i_08ac82),
    (0x08AC86, i_08ac86),
    (0x08AC8C, i_08ac8c),
    (0x08AC92, i_08ac92),
    (0x08AC98, i_08ac98),
    (0x08AC9C, i_08ac9c),
    (0x08ACA2, i_08aca2),
    (0x08ACA8, i_08aca8),
    (0x08ACAE, i_08acae),
    (0x08ACB4, i_08acb4),
    (0x08ACBA, i_08acba),
    (0x08ACC0, i_08acc0),
    (0x08ACC6, i_08acc6),
    (0x08ACCC, i_08accc),
    (0x08ACD2, i_08acd2),
    (0x08ACD8, i_08acd8),
    (0x08ACDE, i_08acde),
    (0x08ACE4, i_08ace4),
    (0x08ACEA, i_08acea),
    (0x08ACF0, i_08acf0),
    (0x08ACF6, i_08acf6),
    (0x08ACFC, i_08acfc),
    (0x08AD02, i_08ad02),
    (0x08AD08, i_08ad08),
    (0x08AD0E, i_08ad0e),
    (0x08AD14, i_08ad14),
    (0x08AD1A, i_08ad1a),
    (0x08AD20, i_08ad20),
    (0x08AD26, i_08ad26),
    (0x08AD2C, i_08ad2c),
    (0x08AD32, i_08ad32),
    (0x08AD38, i_08ad38),
    (0x08AD3E, i_08ad3e),
    (0x08AD44, i_08ad44),
    (0x08AD4A, i_08ad4a),
    (0x08AD50, i_08ad50),
    (0x08AD56, i_08ad56),
    (0x08AD5C, i_08ad5c),
    (0x08AD62, i_08ad62),
    (0x08AD68, i_08ad68),
    (0x08AD6E, i_08ad6e),
    (0x08AD74, i_08ad74),
    (0x08AD7A, i_08ad7a),
    (0x08AD80, i_08ad80),
    (0x08AD86, i_08ad86),
    (0x08AD8C, i_08ad8c),
    (0x08AD92, i_08ad92),
    (0x08AD98, i_08ad98),
    (0x08AD9E, i_08ad9e),
    (0x08ADA4, i_08ada4),
    (0x08ADAA, i_08adaa),
    (0x08ADB0, i_08adb0),
    (0x08ADB6, i_08adb6),
    (0x08ADBC, i_08adbc),
    (0x08ADC2, i_08adc2),
    (0x08ADC6, i_08adc6),
    (0x08ADCC, i_08adcc),
    (0x08ADD2, i_08add2),
    (0x08ADD8, i_08add8),
    (0x08ADDC, i_08addc),
    (0x08ADE2, i_08ade2),
    (0x08ADE8, i_08ade8),
    (0x08ADEE, i_08adee),
    (0x08ADF4, i_08adf4),
    (0x08ADFA, i_08adfa),
    (0x08AE00, i_08ae00),
    (0x08AE04, i_08ae04),
    (0x08AE0A, i_08ae0a),
    (0x08AE0C, i_08ae0c),
    (0x08AE12, i_08ae12),
    (0x08AE16, i_08ae16),
    (0x08AE1C, i_08ae1c),
    (0x08AE22, i_08ae22),
    (0x08AE28, i_08ae28),
    (0x08AE30, i_08ae30),
    (0x08AE34, i_08ae34),
    (0x08AE36, i_08ae36),
    (0x08AE38, i_08ae38),
    (0x08AE40, i_08ae40),
    (0x08AE44, i_08ae44),
    (0x08AE48, i_08ae48),
    (0x08AE4E, i_08ae4e),
    (0x08AE54, i_08ae54),
    (0x08AE56, i_08ae56),
    (0x08AE5C, i_08ae5c),
    (0x08AE60, i_08ae60),
    (0x08AE66, i_08ae66),
    (0x08AE68, i_08ae68),
    (0x08AE6E, i_08ae6e),
    (0x08AE74, i_08ae74),
    (0x08AE78, i_08ae78),
    (0x08AE7E, i_08ae7e),
    (0x08AE84, i_08ae84),
    (0x08AE8A, i_08ae8a),
    (0x08AE8E, i_08ae8e),
    (0x08AE92, i_08ae92),
    (0x08AE98, i_08ae98),
    (0x08AE9C, i_08ae9c),
    (0x08AEA2, i_08aea2),
    (0x08AEA8, i_08aea8),
    (0x08AEAE, i_08aeae),
    (0x08AEB4, i_08aeb4),
    (0x08AEBA, i_08aeba),
    (0x08AEC0, i_08aec0),
    (0x08AEC4, i_08aec4),
    (0x08AEC6, i_08aec6),
    (0x08AECA, i_08aeca),
    (0x08AED0, i_08aed0),
    (0x08AED4, i_08aed4),
    (0x08AEDA, i_08aeda),
    (0x08AEE0, i_08aee0),
    (0x08AEE4, i_08aee4),
    (0x08AEEA, i_08aeea),
    (0x08AEEE, i_08aeee),
    (0x08AEF4, i_08aef4),
    (0x08AEFA, i_08aefa),
    (0x08AF00, i_08af00),
    (0x08AF06, i_08af06),
    (0x08AF0A, i_08af0a),
    (0x08AF10, i_08af10),
    (0x08AF16, i_08af16),
    (0x08AF1C, i_08af1c),
    (0x08AF22, i_08af22),
    (0x08AF26, i_08af26),
    (0x08AF2C, i_08af2c),
    (0x08AF30, i_08af30),
    (0x08AF36, i_08af36),
    (0x08AF3A, i_08af3a),
    (0x08AF40, i_08af40),
    (0x08AF46, i_08af46),
    (0x08AF4C, i_08af4c),
    (0x08AF54, i_08af54),
    (0x08AF58, i_08af58),
    (0x08AF5E, i_08af5e),
    (0x08AF60, i_08af60),
    (0x08AF64, i_08af64),
    (0x08AF66, i_08af66),
    (0x08AF68, i_08af68),
    (0x08AF6E, i_08af6e),
    (0x08AF74, i_08af74),
    (0x08AF78, i_08af78),
    (0x08AF7E, i_08af7e),
    (0x08AF84, i_08af84),
    (0x08AF88, i_08af88),
    (0x08AF8A, i_08af8a),
    (0x08AF8E, i_08af8e),
    (0x08AF94, i_08af94),
    (0x08AF98, i_08af98),
    (0x08AF9E, i_08af9e),
    (0x08AFA4, i_08afa4),
    (0x08AFA8, i_08afa8),
    (0x08AFAE, i_08afae),
    (0x08AFB2, i_08afb2),
    (0x08AFB8, i_08afb8),
    (0x08AFBE, i_08afbe),
    (0x08AFC4, i_08afc4),
    (0x08AFCA, i_08afca),
    (0x08AFCE, i_08afce),
    (0x08AFD4, i_08afd4),
    (0x08AFDA, i_08afda),
    (0x08AFE0, i_08afe0),
    (0x08AFE6, i_08afe6),
    (0x08AFEA, i_08afea),
    (0x08AFF0, i_08aff0),
    (0x08AFF6, i_08aff6),
    (0x08AFFA, i_08affa),
    (0x08B000, i_08b000),
    (0x08B004, i_08b004),
    (0x08B00A, i_08b00a),
    (0x08B010, i_08b010),
    (0x08B016, i_08b016),
    (0x08B01C, i_08b01c),
    (0x08B020, i_08b020),
    (0x08B026, i_08b026),
    (0x08B02C, i_08b02c),
    (0x08B030, i_08b030),
    (0x08B032, i_08b032),
    (0x08B034, i_08b034),
    (0x08B03A, i_08b03a),
    (0x08B03C, i_08b03c),
    (0x08B040, i_08b040),
    (0x08B042, i_08b042),
    (0x08B046, i_08b046),
    (0x08B04C, i_08b04c),
    (0x08B050, i_08b050),
    (0x08B056, i_08b056),
    (0x08B05C, i_08b05c),
    (0x08B060, i_08b060),
    (0x08B066, i_08b066),
    (0x08B06C, i_08b06c),
    (0x08B072, i_08b072),
    (0x08B074, i_08b074),
    (0x08B07C, i_08b07c),
    (0x08B080, i_08b080),
    (0x08B086, i_08b086),
    (0x08B08C, i_08b08c),
    (0x08B092, i_08b092),
    (0x08B098, i_08b098),
    (0x08B09E, i_08b09e),
    (0x08B0A4, i_08b0a4),
    (0x08B0AA, i_08b0aa),
    (0x08B0AC, i_08b0ac),
    (0x08B0B0, i_08b0b0),
    (0x08B0B6, i_08b0b6),
    (0x08B0B8, i_08b0b8),
    (0x08B0BC, i_08b0bc),
    (0x08B0C2, i_08b0c2),
    (0x08B0C6, i_08b0c6),
    (0x08B0CC, i_08b0cc),
    (0x08B0D0, i_08b0d0),
    (0x08B0D6, i_08b0d6),
    (0x08B0DA, i_08b0da),
    (0x08B0DC, i_08b0dc),
    (0x08B0E2, i_08b0e2),
    (0x08B0E8, i_08b0e8),
    (0x08B0EC, i_08b0ec),
    (0x08B0F2, i_08b0f2),
    (0x08B0F8, i_08b0f8),
    (0x08B0FE, i_08b0fe),
    (0x08B102, i_08b102),
    (0x08B106, i_08b106),
    (0x08B108, i_08b108),
    (0x08B10A, i_08b10a),
    (0x08B10E, i_08b10e),
    (0x08B114, i_08b114),
    (0x08B118, i_08b118),
    (0x08B11E, i_08b11e),
    (0x08B124, i_08b124),
    (0x08B12A, i_08b12a),
    (0x08B130, i_08b130),
    (0x08B136, i_08b136),
    (0x08B13A, i_08b13a),
    (0x08B140, i_08b140),
    (0x08B146, i_08b146),
    (0x08B14C, i_08b14c),
    (0x08B152, i_08b152),
    (0x08B158, i_08b158),
    (0x08B15C, i_08b15c),
    (0x08B162, i_08b162),
    (0x08B168, i_08b168),
    (0x08B16E, i_08b16e),
    (0x08B174, i_08b174),
    (0x08B17A, i_08b17a),
    (0x08B17E, i_08b17e),
    (0x08B184, i_08b184),
    (0x08B18A, i_08b18a),
    (0x08B190, i_08b190),
    (0x08B196, i_08b196),
    (0x08B19C, i_08b19c),
    (0x08B1A0, i_08b1a0),
    (0x08B1A6, i_08b1a6),
    (0x08B1AC, i_08b1ac),
    (0x08B1B2, i_08b1b2),
    (0x08B1B8, i_08b1b8),
    (0x08B1BE, i_08b1be),
    (0x08B1C2, i_08b1c2),
    (0x08B1C4, i_08b1c4),
    (0x08B1CA, i_08b1ca),
    (0x08B1D0, i_08b1d0),
    (0x08B1D4, i_08b1d4),
    (0x08B1DA, i_08b1da),
    (0x08B1E0, i_08b1e0),
    (0x08B1E6, i_08b1e6),
    (0x08B1EA, i_08b1ea),
    (0x08B1F0, i_08b1f0),
    (0x08B1F2, i_08b1f2),
    (0x08B1F8, i_08b1f8),
    (0x08B1FC, i_08b1fc),
    (0x08B202, i_08b202),
    (0x08B206, i_08b206),
    (0x08B208, i_08b208),
    (0x08B20C, i_08b20c),
    (0x08B212, i_08b212),
    (0x08B218, i_08b218),
    (0x08B21E, i_08b21e),
    (0x08B224, i_08b224),
    (0x08B22A, i_08b22a),
    (0x08B22E, i_08b22e),
    (0x08B230, i_08b230),
    (0x08B236, i_08b236),
    (0x08B23C, i_08b23c),
    (0x08B240, i_08b240),
    (0x08B246, i_08b246),
    (0x08B24C, i_08b24c),
    (0x08B250, i_08b250),
    (0x08B256, i_08b256),
    (0x08B258, i_08b258),
    (0x08B25E, i_08b25e),
    (0x08B262, i_08b262),
    (0x08B266, i_08b266),
    (0x08B26C, i_08b26c),
    (0x08B270, i_08b270),
    (0x08B274, i_08b274),
    (0x08B27A, i_08b27a),
    (0x08B280, i_08b280),
    (0x08B286, i_08b286),
    (0x08B28C, i_08b28c),
    (0x08B292, i_08b292),
    (0x08B298, i_08b298),
    (0x08B29E, i_08b29e),
    (0x08B2A2, i_08b2a2),
    (0x08B2A8, i_08b2a8),
    (0x08B2AE, i_08b2ae),
    (0x08B2B2, i_08b2b2),
    (0x08B2B8, i_08b2b8),
    (0x08B2BA, i_08b2ba),
    (0x08B2BE, i_08b2be),
    (0x08B2C4, i_08b2c4),
    (0x08B2C6, i_08b2c6),
    (0x08B2CA, i_08b2ca),
    (0x08B2D0, i_08b2d0),
    (0x08B2D4, i_08b2d4),
    (0x08B2DA, i_08b2da),
    (0x08B2DE, i_08b2de),
    (0x08B2E0, i_08b2e0),
    (0x08B2E6, i_08b2e6),
    (0x08B2EC, i_08b2ec),
    (0x08B2F2, i_08b2f2),
    (0x08B2F6, i_08b2f6),
    (0x08B2FA, i_08b2fa),
    (0x08B2FE, i_08b2fe),
    (0x08B304, i_08b304),
    (0x08B308, i_08b308),
    (0x08B30E, i_08b30e),
    (0x08B314, i_08b314),
    (0x08B31A, i_08b31a),
    (0x08B31E, i_08b31e),
    (0x08B324, i_08b324),
    (0x08B328, i_08b328),
    (0x08B32E, i_08b32e),
    (0x08B334, i_08b334),
    (0x08B33A, i_08b33a),
    (0x08B340, i_08b340),
    (0x08B344, i_08b344),
    (0x08B34A, i_08b34a),
    (0x08B34C, i_08b34c),
    (0x08B352, i_08b352),
    (0x08B358, i_08b358),
    (0x08B35E, i_08b35e),
    (0x08B362, i_08b362),
    (0x08B366, i_08b366),
    (0x08B36C, i_08b36c),
    (0x08B372, i_08b372),
    (0x08B378, i_08b378),
    (0x08B37C, i_08b37c),
    (0x08B380, i_08b380),
    (0x08B386, i_08b386),
    (0x08B38C, i_08b38c),
    (0x08B392, i_08b392),
    (0x08B396, i_08b396),
    (0x08B39A, i_08b39a),
    (0x08B3A0, i_08b3a0),
    (0x08B3A6, i_08b3a6),
    (0x08B3AC, i_08b3ac),
    (0x08B3B0, i_08b3b0),
    (0x08B3B4, i_08b3b4),
    (0x08B3BA, i_08b3ba),
    (0x08B3C0, i_08b3c0),
    (0x08B3C6, i_08b3c6),
    (0x08B3CC, i_08b3cc),
    (0x08B3D2, i_08b3d2),
    (0x08B3D8, i_08b3d8),
    (0x08B3DA, i_08b3da),
    (0x08B3E0, i_08b3e0),
    (0x08B3E4, i_08b3e4),
    (0x08B3EA, i_08b3ea),
    (0x08B3EE, i_08b3ee),
    (0x08B3F2, i_08b3f2),
    (0x08B3F6, i_08b3f6),
    (0x08B3FC, i_08b3fc),
    (0x08B400, i_08b400),
    (0x08B404, i_08b404),
    (0x08B40A, i_08b40a),
    (0x08B40E, i_08b40e),
    (0x08B414, i_08b414),
    (0x08B418, i_08b418),
    (0x08B41C, i_08b41c),
    (0x08B420, i_08b420),
    (0x08B426, i_08b426),
    (0x08B42A, i_08b42a),
    (0x08B42E, i_08b42e),
    (0x08B434, i_08b434),
    (0x08B438, i_08b438),
    (0x08B43A, i_08b43a),
    (0x08B440, i_08b440),
    (0x08B446, i_08b446),
    (0x08B44A, i_08b44a),
    (0x08B450, i_08b450),
    (0x08B454, i_08b454),
    (0x08B45A, i_08b45a),
    (0x08B45C, i_08b45c),
    (0x08B460, i_08b460),
    (0x08B464, i_08b464),
    (0x08B468, i_08b468),
    (0x08B46C, i_08b46c),
    (0x08B472, i_08b472),
    (0x08B476, i_08b476),
    (0x08B47A, i_08b47a),
    (0x08B47E, i_08b47e),
    (0x08B482, i_08b482),
    (0x08B488, i_08b488),
    (0x08B48E, i_08b48e),
    (0x08B490, i_08b490),
    (0x08B494, i_08b494),
    (0x08B498, i_08b498),
    (0x08B49C, i_08b49c),
    (0x08B4A0, i_08b4a0),
    (0x08B4A6, i_08b4a6),
    (0x08B4AA, i_08b4aa),
    (0x08B4AE, i_08b4ae),
    (0x08B4B2, i_08b4b2),
    (0x08B4B6, i_08b4b6),
    (0x08B4BC, i_08b4bc),
    (0x08B4C2, i_08b4c2),
    (0x08B4C4, i_08b4c4),
    (0x08B4C8, i_08b4c8),
    (0x08B4CC, i_08b4cc),
    (0x08B4D0, i_08b4d0),
    (0x08B4D4, i_08b4d4),
    (0x08B4DA, i_08b4da),
    (0x08B4DE, i_08b4de),
    (0x08B4E2, i_08b4e2),
    (0x08B4E6, i_08b4e6),
    (0x08B4EA, i_08b4ea),
    (0x08B4F0, i_08b4f0),
    (0x08B4F4, i_08b4f4),
    (0x08B4F8, i_08b4f8),
    (0x08B4FC, i_08b4fc),
    (0x08B500, i_08b500),
    (0x08B506, i_08b506),
    (0x08B50C, i_08b50c),
    (0x08B50E, i_08b50e),
    (0x08B512, i_08b512),
    (0x08B516, i_08b516),
    (0x08B51A, i_08b51a),
    (0x08B51E, i_08b51e),
    (0x08B524, i_08b524),
    (0x08B528, i_08b528),
    (0x08B52C, i_08b52c),
    (0x08B530, i_08b530),
    (0x08B534, i_08b534),
    (0x08B53A, i_08b53a),
    (0x08B53E, i_08b53e),
    (0x08B542, i_08b542),
    (0x08B546, i_08b546),
    (0x08B54A, i_08b54a),
    (0x08B550, i_08b550),
    (0x08B556, i_08b556),
    (0x08B558, i_08b558),
    (0x08B55E, i_08b55e),
    (0x08B562, i_08b562),
    (0x08B568, i_08b568),
    (0x08B56E, i_08b56e),
    (0x08B572, i_08b572),
    (0x08B578, i_08b578),
    (0x08B57E, i_08b57e),
    (0x08B582, i_08b582),
    (0x08B586, i_08b586),
    (0x08B58C, i_08b58c),
    (0x08B58E, i_08b58e),
    (0x08B590, i_08b590),
    (0x08B592, i_08b592),
    (0x08B596, i_08b596),
    (0x08B59A, i_08b59a),
    (0x08B59C, i_08b59c),
    (0x08B59E, i_08b59e),
    (0x08B5A2, i_08b5a2),
    (0x08B5A8, i_08b5a8),
    (0x08B5AE, i_08b5ae),
    (0x08B5B4, i_08b5b4),
    (0x08B5B6, i_08b5b6),
    (0x08B5BA, i_08b5ba),
    (0x08B5C0, i_08b5c0),
    (0x08B5C6, i_08b5c6),
    (0x08B5C8, i_08b5c8),
    (0x08B5CC, i_08b5cc),
    (0x08B5D0, i_08b5d0),
    (0x08B5D4, i_08b5d4),
    (0x08B5D8, i_08b5d8),
    (0x08B5DC, i_08b5dc),
    (0x08B5DE, i_08b5de),
    (0x08B5E2, i_08b5e2),
    (0x08B5E6, i_08b5e6),
    (0x08B5EC, i_08b5ec),
    (0x08B5F0, i_08b5f0),
    (0x08B5F6, i_08b5f6),
    (0x08B5FA, i_08b5fa),
    (0x08B600, i_08b600),
    (0x08B604, i_08b604),
    (0x08B60A, i_08b60a),
    (0x08B60E, i_08b60e),
    (0x08B614, i_08b614),
    (0x08B61A, i_08b61a),
    (0x08B61E, i_08b61e),
    (0x08B620, i_08b620),
    (0x08B624, i_08b624),
    (0x08B626, i_08b626),
    (0x08B62C, i_08b62c),
    (0x08B630, i_08b630),
    (0x08B634, i_08b634),
    (0x08B63A, i_08b63a),
    (0x08B63E, i_08b63e),
    (0x08B642, i_08b642),
    (0x08B646, i_08b646),
    (0x08B64C, i_08b64c),
    (0x08B652, i_08b652),
    (0x08B658, i_08b658),
    (0x08B65E, i_08b65e),
    (0x08B662, i_08b662),
    (0x08B668, i_08b668),
    (0x08B66E, i_08b66e),
    (0x08B674, i_08b674),
    (0x08B67A, i_08b67a),
    (0x08B67E, i_08b67e),
    (0x08B680, i_08b680),
    (0x08B684, i_08b684),
    (0x08B688, i_08b688),
    (0x08B68A, i_08b68a),
    (0x08B68E, i_08b68e),
    (0x08B690, i_08b690),
    (0x08B696, i_08b696),
    (0x08B69A, i_08b69a),
    (0x08B69E, i_08b69e),
    (0x08B6A2, i_08b6a2),
    (0x08B6A6, i_08b6a6),
    (0x08B6A8, i_08b6a8),
    (0x08B6AE, i_08b6ae),
    (0x08B6B4, i_08b6b4),
    (0x08B6B8, i_08b6b8),
    (0x08B6BC, i_08b6bc),
    (0x08B6C2, i_08b6c2),
    (0x08B6C4, i_08b6c4),
    (0x08B6C8, i_08b6c8),
    (0x08B6CC, i_08b6cc),
    (0x08B6D0, i_08b6d0),
    (0x08B6D4, i_08b6d4),
    (0x08B6D8, i_08b6d8),
    (0x08B6DC, i_08b6dc),
    (0x08B6E2, i_08b6e2),
    (0x08B6E8, i_08b6e8),
    (0x08B6EE, i_08b6ee),
    (0x08B6F2, i_08b6f2),
    (0x08B6F6, i_08b6f6),
    (0x08B6FC, i_08b6fc),
    (0x08B700, i_08b700),
    (0x08B704, i_08b704),
    (0x08B708, i_08b708),
    (0x08B70C, i_08b70c),
    (0x08B712, i_08b712),
    (0x08B716, i_08b716),
    (0x08B718, i_08b718),
    (0x08B71C, i_08b71c),
    (0x08B720, i_08b720),
    (0x08B724, i_08b724),
    (0x08B726, i_08b726),
    (0x08B728, i_08b728),
    (0x08B72C, i_08b72c),
    (0x08B730, i_08b730),
    (0x08B732, i_08b732),
    (0x08B738, i_08b738),
    (0x08B73C, i_08b73c),
    (0x08B740, i_08b740),
    (0x08B742, i_08b742),
    (0x08B744, i_08b744),
    (0x08B748, i_08b748),
    (0x08B74E, i_08b74e),
    (0x08B754, i_08b754),
    (0x08B75A, i_08b75a),
    (0x08B75E, i_08b75e),
    (0x08B762, i_08b762),
    (0x08B768, i_08b768),
    (0x08B76C, i_08b76c),
    (0x08B770, i_08b770),
    (0x08B776, i_08b776),
    (0x08B778, i_08b778),
    (0x08B77C, i_08b77c),
    (0x08B780, i_08b780),
    (0x08B784, i_08b784),
    (0x08B788, i_08b788),
    (0x08B78C, i_08b78c),
    (0x08B790, i_08b790),
    (0x08B794, i_08b794),
    (0x08B796, i_08b796),
    (0x08B79A, i_08b79a),
    (0x08B79E, i_08b79e),
    (0x08B7A2, i_08b7a2),
    (0x08B7A6, i_08b7a6),
    (0x08B7A8, i_08b7a8),
    (0x08B7AC, i_08b7ac),
    (0x08B7AE, i_08b7ae),
    (0x08B7B0, i_08b7b0),
    (0x08B7B4, i_08b7b4),
    (0x08B7B8, i_08b7b8),
    (0x08B7BC, i_08b7bc),
    (0x08B7C0, i_08b7c0),
    (0x08B7C6, i_08b7c6),
    (0x08B7CA, i_08b7ca),
    (0x08B7CE, i_08b7ce),
    (0x08B7D2, i_08b7d2),
    (0x08B7D8, i_08b7d8),
    (0x08B7DC, i_08b7dc),
    (0x08B7E0, i_08b7e0),
    (0x08B7E4, i_08b7e4),
    (0x08B7E6, i_08b7e6),
    (0x08B7EA, i_08b7ea),
    (0x08B7EE, i_08b7ee),
    (0x08B7F0, i_08b7f0),
    (0x08B7F4, i_08b7f4),
    (0x08B7F8, i_08b7f8),
    (0x08B7FC, i_08b7fc),
    (0x08B800, i_08b800),
    (0x08B806, i_08b806),
    (0x08B80A, i_08b80a),
    (0x08B80E, i_08b80e),
    (0x08B812, i_08b812),
    (0x08B816, i_08b816),
    (0x08B81A, i_08b81a),
    (0x08B81E, i_08b81e),
    (0x08B820, i_08b820),
    (0x08B824, i_08b824),
    (0x08B826, i_08b826),
    (0x08B82A, i_08b82a),
    (0x08B82C, i_08b82c),
    (0x08B832, i_08b832),
    (0x08B838, i_08b838),
    (0x08B83A, i_08b83a),
    (0x08B83E, i_08b83e),
    (0x08B842, i_08b842),
    (0x08B848, i_08b848),
    (0x08B84E, i_08b84e),
    (0x08B854, i_08b854),
    (0x08B85A, i_08b85a),
    (0x08B860, i_08b860),
    (0x08B862, i_08b862),
    (0x08B868, i_08b868),
    (0x08B86C, i_08b86c),
    (0x08B870, i_08b870),
    (0x08B874, i_08b874),
    (0x08B87A, i_08b87a),
    (0x08B87E, i_08b87e),
    (0x08B882, i_08b882),
    (0x08B884, i_08b884),
    (0x08B888, i_08b888),
    (0x08B88C, i_08b88c),
    (0x08B890, i_08b890),
    (0x08B894, i_08b894),
    (0x08B898, i_08b898),
    (0x08B89E, i_08b89e),
    (0x08B8A2, i_08b8a2),
    (0x08B8A6, i_08b8a6),
    (0x08B8AA, i_08b8aa),
    (0x08B8AE, i_08b8ae),
    (0x08B8B0, i_08b8b0),
    (0x08B8B4, i_08b8b4),
    (0x08B8B8, i_08b8b8),
    (0x08B8BC, i_08b8bc),
    (0x08B8C0, i_08b8c0),
    (0x08B8C4, i_08b8c4),
    (0x08B8CA, i_08b8ca),
    (0x08B8CE, i_08b8ce),
    (0x08B8D2, i_08b8d2),
    (0x08B8D6, i_08b8d6),
    (0x08B8DA, i_08b8da),
    (0x08B8DE, i_08b8de),
    (0x08B8E4, i_08b8e4),
    (0x08B8E8, i_08b8e8),
    (0x08B8EC, i_08b8ec),
    (0x08B8F2, i_08b8f2),
    (0x08B8F4, i_08b8f4),
    (0x08B8F6, i_08b8f6),
    (0x08B8FA, i_08b8fa),
    (0x08B8FC, i_08b8fc),
    (0x08B902, i_08b902),
    (0x08B906, i_08b906),
    (0x08B90C, i_08b90c),
    (0x08B90E, i_08b90e),
    (0x08B910, i_08b910),
    (0x08B914, i_08b914),
    (0x08B916, i_08b916),
    (0x08B91C, i_08b91c),
    (0x08B920, i_08b920),
    (0x08B926, i_08b926),
    (0x08B928, i_08b928),
    (0x08B92C, i_08b92c),
    (0x08B930, i_08b930),
    (0x08B934, i_08b934),
    (0x08B938, i_08b938),
    (0x08B93C, i_08b93c),
    (0x08B93E, i_08b93e),
    (0x08B942, i_08b942),
    (0x08B950, i_08b950),
    (0x08B954, i_08b954),
    (0x08B9A2, i_08b9a2),
    (0x08B9A6, i_08b9a6),
    (0x08B9AA, i_08b9aa),
    (0x08B9AE, i_08b9ae),
    (0x08B9B2, i_08b9b2),
    (0x08B9B6, i_08b9b6),
    (0x08B9BA, i_08b9ba),
    (0x08B9C0, i_08b9c0),
    (0x08B9C6, i_08b9c6),
    (0x08B9CC, i_08b9cc),
    (0x08B9D2, i_08b9d2),
    (0x08B9D8, i_08b9d8),
    (0x08B9DC, i_08b9dc),
    (0x08B9DE, i_08b9de),
    (0x08B9E4, i_08b9e4),
    (0x08B9EA, i_08b9ea),
    (0x08B9EE, i_08b9ee),
    (0x08B9F2, i_08b9f2),
    (0x08B9F4, i_08b9f4),
    (0x08B9FA, i_08b9fa),
    (0x08B9FE, i_08b9fe),
    (0x08BA04, i_08ba04),
    (0x08BA0A, i_08ba0a),
    (0x08BA0C, i_08ba0c),
    (0x08BA12, i_08ba12),
    (0x08BA18, i_08ba18),
    (0x08BA1E, i_08ba1e),
    (0x08BA22, i_08ba22),
    (0x08BA24, i_08ba24),
    (0x08BA2A, i_08ba2a),
    (0x08BA30, i_08ba30),
    (0x08BA34, i_08ba34),
    (0x08BA38, i_08ba38),
    (0x08BA3A, i_08ba3a),
    (0x08BA40, i_08ba40),
    (0x08BA44, i_08ba44),
    (0x08BA4A, i_08ba4a),
    (0x08BA50, i_08ba50),
    (0x08BA52, i_08ba52),
    (0x08BA58, i_08ba58),
    (0x08BA5E, i_08ba5e),
    (0x08BA64, i_08ba64),
    (0x08BA6A, i_08ba6a),
    (0x08BA6E, i_08ba6e),
    (0x08BA70, i_08ba70),
    (0x08BA76, i_08ba76),
    (0x08BA7C, i_08ba7c),
    (0x08BA80, i_08ba80),
    (0x08BA84, i_08ba84),
    (0x08BA86, i_08ba86),
    (0x08BA8C, i_08ba8c),
    (0x08BA90, i_08ba90),
    (0x08BA96, i_08ba96),
    (0x08BA9C, i_08ba9c),
    (0x08BABE, i_08babe),
    (0x08BAC4, i_08bac4),
    (0x08BAC8, i_08bac8),
    (0x08BACC, i_08bacc),
    (0x08BAD2, i_08bad2),
    (0x08BAD8, i_08bad8),
    (0x08BADA, i_08bada),
    (0x08BAE0, i_08bae0),
    (0x08BAE4, i_08bae4),
    (0x08BAE8, i_08bae8),
    (0x08BAEE, i_08baee),
    (0x08BAF4, i_08baf4),
    (0x08BAF6, i_08baf6),
    (0x08BAFA, i_08bafa),
    (0x08BB00, i_08bb00),
    (0x08BB06, i_08bb06),
    (0x08BB0C, i_08bb0c),
    (0x08BB10, i_08bb10),
    (0x08BB12, i_08bb12),
    (0x08BB18, i_08bb18),
    (0x08BB1E, i_08bb1e),
    (0x08BB22, i_08bb22),
    (0x08BB26, i_08bb26),
    (0x08BB28, i_08bb28),
    (0x08BB2A, i_08bb2a),
    (0x08BB30, i_08bb30),
    (0x08BB32, i_08bb32),
    (0x08BB34, i_08bb34),
    (0x08BB38, i_08bb38),
    (0x08BB3E, i_08bb3e),
    (0x08BB42, i_08bb42),
    (0x08BB46, i_08bb46),
    (0x08BB4A, i_08bb4a),
    (0x08BB50, i_08bb50),
    (0x08BB56, i_08bb56),
    (0x08BB5C, i_08bb5c),
    (0x08BB5E, i_08bb5e),
    (0x08BB62, i_08bb62),
    (0x08BB68, i_08bb68),
    (0x08BB6C, i_08bb6c),
    (0x08BB70, i_08bb70),
    (0x08BB76, i_08bb76),
    (0x08BB7C, i_08bb7c),
    (0x08BB82, i_08bb82),
];
