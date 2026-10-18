// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $036A54: move.l a1, (a6)
pub fn i_036a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036A56: cmpi.b #$0, $90(a6)
pub fn i_036a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A5C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $036A5C: bne.w $36a70
pub fn i_036a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A60;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36A70; return 10; }
    8
}
/// $036A60: move.w $2a(a6), d0
pub fn i_036a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036A64: asr.w #$2, d0
pub fn i_036a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A66;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $036A66: move.w d0, $2a(a6)
pub fn i_036a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A6A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036A6A: lea.l $36a70(pc), a1
pub fn i_036a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36A70;
    16
}
/// $036A6E: move.l a1, (a6)
pub fn i_036a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Player_JumpAir_036a70  ($036A70, 540 B, player/air_death_crouch_036632.s)
/// $036A70: jsr $5cde4.l
pub fn i_036a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36A76);
    cpu.pc = 0x5CDE4;
    16
}
/// $036A76: bcc.w $36aa8
pub fn i_036a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36AA8; return 10; }
    8
}
/// $036A7A: jsr $5d5b6.l
pub fn i_036a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36A80);
    cpu.pc = 0x5D5B6;
    16
}
/// $036A80: asl.w #$1, d0
pub fn i_036a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A82;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036A82: lea.l $324d0(pc), a0
pub fn i_036a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A86;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036A86: move.w (a0, d0.w), $2c(a6)
pub fn i_036a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A8C;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036A8C: beq.w $36aa4
pub fn i_036a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36AA4; return 10; }
    8
}
/// $036A90: ble.w $36a9e
pub fn i_036a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A94;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x36A9E; return 10; }
    8
}
/// $036A94: bclr.b #$0, $3a(a6)
pub fn i_036a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A9A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036A9A: bra.w $36aa4
pub fn i_036a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x36AA4;
    10
}
/// $036A9E: bset.b #$0, $3a(a6)
pub fn i_036a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AA4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036AA4: bra.w $36aba
pub fn i_036aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x36ABA;
    10
}
/// $036AA8: jsr $5d5b6.l
pub fn i_036aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36AAE);
    cpu.pc = 0x5D5B6;
    16
}
/// $036AAE: asl.w #$1, d0
pub fn i_036aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AB0;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036AB0: lea.l $324d0(pc), a0
pub fn i_036ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AB4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036AB4: move.w (a0, d0.w), $2c(a6)
pub fn i_036ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ABA;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036ABA: cmpi.w #$0, $2c(a6)
pub fn i_036aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AC0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036AC0: bne.w $36ae8
pub fn i_036ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36AE8; return 10; }
    8
}
/// $036AC4: cmpi.w #$0, $28(a6)
pub fn i_036ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ACA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036ACA: beq.w $36ae8
pub fn i_036aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ACE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36AE8; return 10; }
    8
}
/// $036ACE: cmpi.w #$0, $28(a6)
pub fn i_036ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AD4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036AD4: bmi.w $36ae2
pub fn i_036ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AD8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36AE2; return 10; }
    8
}
/// $036AD8: move.w #$ffe8, $2c(a6)
pub fn i_036ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ADE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036ADE: bra.w $36ae8
pub fn i_036ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x36AE8;
    10
}
/// $036AE2: move.w #$18, $2c(a6)
pub fn i_036ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036AE8: move.w $28(a6), d0
pub fn i_036ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AEC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036AEC: move.w #$400, d1
pub fn i_036aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036AF0: move.w #$300, d1
pub fn i_036af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AF4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036AF4: jsr $267f4.l
pub fn i_036af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36AFA);
    cpu.pc = 0x267F4;
    16
}
/// $036AFA: move.w d0, $28(a6)
pub fn i_036afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036AFE: move.w $2a(a6), d0
pub fn i_036afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B02;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B02: bgt.w $36b14
pub fn i_036b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B06;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36B14; return 10; }
    8
}
/// $036B06: move.w #$780, d1
pub fn i_036b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B0A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x780;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036B0A: jsr $267f4.l
pub fn i_036b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36B10);
    cpu.pc = 0x267F4;
    16
}
/// $036B10: move.w d0, $2a(a6)
pub fn i_036b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B14;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036B14: jsr $33016(pc)
pub fn i_036b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36B18);
    cpu.pc = 0x33016;
    16
}
/// $036B18: bset.b #$1, $8d(a6)
pub fn i_036b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B1E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B1E: bclr.b #$5, $8d(a6)
pub fn i_036b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B24;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B24: btst.b #$5, $100001.l
pub fn i_036b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B2C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((5) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $036B2C: beq.w $36b54
pub fn i_036b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36B54; return 10; }
    8
}
/// $036B30: move.w $2e(a6), d0
pub fn i_036b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B34;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B34: movem.w d0, -(a7)
pub fn i_036b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B38;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $036B38: move.w $28(a6), d0
pub fn i_036b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B3C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B3C: movem.w d0, -(a7)
pub fn i_036b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B40;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $036B40: move.w $2a(a6), d0
pub fn i_036b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B44;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B44: movem.w d0, -(a7)
pub fn i_036b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B48;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $036B48: clr.w $28(a6)
pub fn i_036b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B4C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036B4C: clr.w $2a(a6)
pub fn i_036b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B50;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036B50: clr.w $2e(a6)
pub fn i_036b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B54;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036B54: jsr $27b66.l
pub fn i_036b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36B5A);
    cpu.pc = 0x27B66;
    16
}
/// $036B5A: bcc.w $36b70
pub fn i_036b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36B70; return 10; }
    8
}
/// $036B5E: lea.l $33ede(pc), a1
pub fn i_036b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33EDE;
    16
}
/// $036B62: move.l a1, (a6)
pub fn i_036b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036B64: bclr.b #$1, $8d(a6)
pub fn i_036b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B6A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B6A: bclr.b #$5, $8d(a6)
pub fn i_036b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B70: jsr $32aa8(pc)
pub fn i_036b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36B74);
    cpu.pc = 0x32AA8;
    16
}
/// $036B74: bcc.w $36b7e
pub fn i_036b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36B7E; return 10; }
    8
}
/// $036B78: bclr.b #$1, $8c(a6)
pub fn i_036b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B7E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B7E: jsr $3705a(pc)
pub fn i_036b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36B82);
    cpu.pc = 0x3705A;
    16
}
/// $036B82: btst.b #$5, $100001.l
pub fn i_036b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B8A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((5) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $036B8A: beq.w $36ba6
pub fn i_036b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36BA6; return 10; }
    8
}
/// $036B8E: movem.w (a7)+, d0
pub fn i_036b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B92;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $036B92: move.w d0, $2a(a6)
pub fn i_036b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B96;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036B96: movem.w (a7)+, d0
pub fn i_036b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B9A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $036B9A: move.w d0, $28(a6)
pub fn i_036b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036B9E: movem.w (a7)+, d0
pub fn i_036b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BA2;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $036BA2: move.w d0, $2e(a6)
pub fn i_036ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BA6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036BA6: move.w $2a(a6), d0
pub fn i_036ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BAA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036BAA: bgt.w $36c36
pub fn i_036baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BAE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36C36; return 10; }
    8
}
/// $036BAE: bset.b #$5, $8d(a6)
pub fn i_036bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BB4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036BB4: btst.b #$0, $8c(a6)
pub fn i_036bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BBA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036BBA: beq.w $36bc8
pub fn i_036bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BBE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36BC8; return 10; }
    8
}
/// $036BBE: lea.l $364a2(pc), a1
pub fn i_036bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x364A2;
    16
}
/// $036BC2: move.l a1, (a6)
pub fn i_036bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036BC4: jmp $36c3a(pc)
pub fn i_036bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BC8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036BC8: btst.b #$1, $8d(a6)
pub fn i_036bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BCE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036BCE: beq.w $36bfe
pub fn i_036bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36BFE; return 10; }
    8
}
/// $036BD2: jsr $8f470.l
pub fn i_036bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36BD8);
    cpu.pc = 0x8F470;
    16
}
/// $036BD8: bcc.w $36bfe
pub fn i_036bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36BFE; return 10; }
    8
}
/// $036BDC: cmpi.w #$0, d2
pub fn i_036bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BE0;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $036BE0: bmi.w $36bfe
pub fn i_036be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BE4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36BFE; return 10; }
    8
}
/// $036BE4: move.b d2, $86(a6)
pub fn i_036be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BE8;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036BE8: lea.l $36638(pc), a1
pub fn i_036be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36638;
    16
}
/// $036BEC: move.l a1, (a6)
pub fn i_036bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036BEE: jsr $8f520.l
pub fn i_036bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36BF4);
    cpu.pc = 0x8F520;
    16
}
/// $036BF4: bclr.b #$5, $8d(a6)
pub fn i_036bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BFA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036BFA: jmp $36c3a(pc)
pub fn i_036bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BFE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036BFE: move.b #$3, d0
pub fn i_036bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C02;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $036C02: and.b $69(a6), d0
pub fn i_036c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C06;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $036C06: beq.w $36c36
pub fn i_036c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C0A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C36; return 10; }
    8
}
/// $036C0A: btst.b #$2, $69(a6)
pub fn i_036c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C10;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036C10: beq.w $36c36
pub fn i_036c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C36; return 10; }
    8
}
/// $036C14: lea.l $368e0(pc), a1
pub fn i_036c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x368E0;
    16
}
/// $036C18: move.l a1, (a6)
pub fn i_036c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036C1A: move.w #$400, $28(a6)
pub fn i_036c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036C20: clr.w $2c(a6)
pub fn i_036c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C24;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036C24: btst.b #$1, $69(a6)
pub fn i_036c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C2A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036C2A: bne.w $36c32
pub fn i_036c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36C32; return 10; }
    8
}
/// $036C2E: neg.w $28(a6)
pub fn i_036c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C32;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $036C32: jmp $36c3a(pc)
pub fn i_036c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C36;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036C36: jsr $37168(pc)
pub fn i_036c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36C3A);
    cpu.pc = 0x37168;
    16
}
/// $036C3A: btst.b #$1, $8d(a6)
pub fn i_036c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C40;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036C40: beq.w $36c86
pub fn i_036c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C44;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C86; return 10; }
    8
}
/// $036C44: cmpi.b #$3, $106ece.l
pub fn i_036c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C4C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $036C4C: beq.w $36c64
pub fn i_036c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C64; return 10; }
    8
}
/// $036C50: movea.l #$ffffffff, a0
pub fn i_036c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C56;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036C56: lea.l $324c6(pc), a0
pub fn i_036c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C5A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $036C5A: jsr $5dd56.l
pub fn i_036c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36C60);
    cpu.pc = 0x5DD56;
    16
}
/// $036C60: bra.w $36c74
pub fn i_036c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C64;
    cpu.ir = 0x6000;
    cpu.pc = 0x36C74;
    10
}
/// $036C64: movea.l #$ffffffff, a0
pub fn i_036c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C6A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036C6A: lea.l $324bc(pc), a0
pub fn i_036c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C6E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $036C6E: jsr $5dd56.l
pub fn i_036c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36C74);
    cpu.pc = 0x5DD56;
    16
}
/// $036C74: bcc.w $36c7e
pub fn i_036c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36C7E; return 10; }
    8
}
/// $036C78: lea.l $37b8e(pc), a1
pub fn i_036c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $036C7C: move.l a1, (a6)
pub fn i_036c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036C7E: jsr $33522(pc)
pub fn i_036c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36C82);
    cpu.pc = 0x33522;
    16
}
/// $036C82: bra.w $36c8a
pub fn i_036c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C86;
    cpu.ir = 0x6000;
    cpu.pc = 0x36C8A;
    10
}
/// $036C86: jsr $33522(pc)
pub fn i_036c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36C8A);
    cpu.pc = 0x33522;
    16
}
/// $036C8A: rts
pub fn i_036c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SpawnFreeFall_036c8c  ($036C8C, 216 B, player/air_death_crouch_036632.s)
/// $036C8C: cmpa.l #$100440, a6
pub fn i_036c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C92;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $036C92: bne.w $36ca6
pub fn i_036c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36CA6; return 10; }
    8
}
/// $036C96: lea.l $32112.l, a1
pub fn i_036c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32112;
    20
}
/// $036C9C: jsr $4ae.l
pub fn i_036c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36CA2);
    cpu.pc = 0x4AE;
    16
}
/// $036CA2: bra.w $36cb2
pub fn i_036ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x36CB2;
    10
}
/// $036CA6: lea.l $32142.l, a1
pub fn i_036ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CAC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32142;
    20
}
/// $036CAC: jsr $4ae.l
pub fn i_036cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36CB2);
    cpu.pc = 0x4AE;
    16
}
/// $036CB2: clr.w $28(a6)
pub fn i_036cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CB6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036CB6: clr.w $2c(a6)
pub fn i_036cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CBA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036CBA: move.w #$0, $7c(a6)
pub fn i_036cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036CC0: move.w #$0, $7e(a6)
pub fn i_036cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036CC6: bclr.b #$2, $8c(a6)
pub fn i_036cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CCC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036CCC: bclr.b #$1, $8c(a6)
pub fn i_036ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CD2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036CD2: bclr.b #$3, $8c(a6)
pub fn i_036cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CD8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036CD8: move.l #$32500, $60(a6)
pub fn i_036cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CE0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036CE0: move.b #$26, $70(a6)
pub fn i_036ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036CE6: lea.l $279b2c.l, a0
pub fn i_036ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036CEC: move.l -$4(a0), $74(a6)
pub fn i_036cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CF2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036CF2: move.b #$ff, $21(a6)
pub fn i_036cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CF8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036CF8: lea.l $279b2c.l, a0
pub fn i_036cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036CFE: jsr $28cd4.l
pub fn i_036cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36D04);
    cpu.pc = 0x28CD4;
    16
}
/// $036D04: move.w #$ff80, $2e(a6)
pub fn i_036d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D0A: lea.l $ffff.w, a0
pub fn i_036d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D0E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $036D0E: move.l a0, $48(a6)
pub fn i_036d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D12;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036D12: clr.b $59(a6)
pub fn i_036d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D16;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x59);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036D16: clr.b $45(a6)
pub fn i_036d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D1A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036D1A: lea.l $36d20(pc), a1
pub fn i_036d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D20;
    16
}
/// $036D1E: move.l a1, (a6)
pub fn i_036d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036D20: ori.b #$3c, $59(a6)
pub fn i_036d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D26;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x59);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x3C) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D26: ori.b #$3c, $45(a6)
pub fn i_036d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D2C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x3C) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D2C: bset.b #$6, $13(a6)
pub fn i_036d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D32;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D32: jsr $36e42(pc)
pub fn i_036d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36D36);
    cpu.pc = 0x36E42;
    16
}
/// $036D36: lea.l $33ede(pc), a0
pub fn i_036d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D3A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x33EDE;
    16
}
/// $036D3A: cmpa.l (a6), a0
pub fn i_036d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D3C;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $036D3C: bne.w $36d4c
pub fn i_036d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36D4C; return 10; }
    8
}
/// $036D40: move.b #$50, $45(a6)
pub fn i_036d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D46: move.b #$50, $59(a6)
pub fn i_036d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D4C: lea.l $368e0(pc), a0
pub fn i_036d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D50;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x368E0;
    16
}
/// $036D50: cmpa.l (a6), a0
pub fn i_036d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D52;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $036D52: bne.w $36d62
pub fn i_036d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36D62; return 10; }
    8
}
/// $036D56: move.b #$50, $45(a6)
pub fn i_036d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D5C: move.b #$50, $59(a6)
pub fn i_036d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D62: rts
pub fn i_036d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Knockback_036d64  ($036D64, 102 B, player/air_death_crouch_036632.s)
/// $036D64: clr.w $28(a6)
pub fn i_036d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D68;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036D68: clr.w $2c(a6)
pub fn i_036d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D6C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036D6C: move.w #$ff80, $2e(a6)
pub fn i_036d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D72;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D72: move.w #$0, $7c(a6)
pub fn i_036d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D78;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D78: move.w #$0, $7e(a6)
pub fn i_036d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D7E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D7E: bclr.b #$2, $8c(a6)
pub fn i_036d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D84;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D84: bclr.b #$1, $8c(a6)
pub fn i_036d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D8A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D8A: bclr.b #$3, $8c(a6)
pub fn i_036d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D90: move.l #$32500, $60(a6)
pub fn i_036d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D98;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036D98: move.b #$26, $70(a6)
pub fn i_036d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D9E: lea.l $279b2c.l, a0
pub fn i_036d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036DA4: move.l -$4(a0), $74(a6)
pub fn i_036da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DAA;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036DAA: move.b #$ff, $21(a6)
pub fn i_036daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036DB0: lea.l $279b2c.l, a0
pub fn i_036db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DB6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036DB6: jsr $28cd4.l
pub fn i_036db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36DBC);
    cpu.pc = 0x28CD4;
    16
}
/// $036DBC: lea.l $32830(pc), a0
pub fn i_036dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DC0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32830;
    16
}
/// $036DC0: move.l a0, $48(a6)
pub fn i_036dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DC4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036DC4: lea.l $36dca(pc), a1
pub fn i_036dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36DCA;
    16
}
/// $036DC8: move.l a1, (a6)
pub fn i_036dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Player_Knockback_AirCtrl_036dca  ($036DCA, 120 B, player/air_death_crouch_036632.s)
/// $036DCA: jsr $5cde4.l
pub fn i_036dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36DD0);
    cpu.pc = 0x5CDE4;
    16
}
/// $036DD0: bcc.w $36e02
pub fn i_036dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36E02; return 10; }
    8
}
/// $036DD4: jsr $5d5b6.l
pub fn i_036dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36DDA);
    cpu.pc = 0x5D5B6;
    16
}
/// $036DDA: asl.w #$1, d0
pub fn i_036dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DDC;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036DDC: lea.l $324d0(pc), a0
pub fn i_036ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DE0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036DE0: move.w (a0, d0.w), $2c(a6)
pub fn i_036de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DE6;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036DE6: beq.w $36dfe
pub fn i_036de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DEA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36DFE; return 10; }
    8
}
/// $036DEA: ble.w $36df8
pub fn i_036dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DEE;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x36DF8; return 10; }
    8
}
/// $036DEE: bclr.b #$0, $3a(a6)
pub fn i_036dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DF4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036DF4: bra.w $36dfe
pub fn i_036df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x36DFE;
    10
}
/// $036DF8: bset.b #$0, $3a(a6)
pub fn i_036df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DFE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036DFE: bra.w $36e14
pub fn i_036dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E02;
    cpu.ir = 0x6000;
    cpu.pc = 0x36E14;
    10
}
/// $036E02: jsr $5d5b6.l
pub fn i_036e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E08);
    cpu.pc = 0x5D5B6;
    16
}
/// $036E08: asl.w #$1, d0
pub fn i_036e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E0A;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036E0A: lea.l $324d0(pc), a0
pub fn i_036e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E0E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036E0E: move.w (a0, d0.w), $2c(a6)
pub fn i_036e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E14;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036E14: cmpi.w #$0, $2c(a6)
pub fn i_036e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E1A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036E1A: bne.w $36e42
pub fn i_036e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36E42; return 10; }
    8
}
/// $036E1E: cmpi.w #$0, $28(a6)
pub fn i_036e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036E24: beq.w $36e42
pub fn i_036e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36E42; return 10; }
    8
}
/// $036E28: cmpi.w #$0, $28(a6)
pub fn i_036e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E2E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036E2E: bmi.w $36e3c
pub fn i_036e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E32;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36E3C; return 10; }
    8
}
/// $036E32: move.w #$ffe8, $2c(a6)
pub fn i_036e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036E38: bra.w $36e42
pub fn i_036e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x36E42;
    10
}
/// $036E3C: move.w #$18, $2c(a6)
pub fn i_036e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Player_Fall_Physics_036e42  ($036E42, 384 B, player/air_death_crouch_036632.s)
/// $036E42: move.w $28(a6), d0
pub fn i_036e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036E46: move.w #$400, d1
pub fn i_036e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E4A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036E4A: move.w #$300, d1
pub fn i_036e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E4E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036E4E: jsr $267f4.l
pub fn i_036e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E54);
    cpu.pc = 0x267F4;
    16
}
/// $036E54: move.w d0, $28(a6)
pub fn i_036e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E58;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036E58: move.w $2a(a6), d0
pub fn i_036e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E5C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036E5C: bgt.w $36e72
pub fn i_036e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E60;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36E72; return 10; }
    8
}
/// $036E60: move.w $2a(a6), d0
pub fn i_036e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036E64: move.w #$400, d1
pub fn i_036e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E68;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036E68: jsr $267f4.l
pub fn i_036e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E6E);
    cpu.pc = 0x267F4;
    16
}
/// $036E6E: move.w d0, $2a(a6)
pub fn i_036e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E72;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036E72: bset.b #$1, $8d(a6)
pub fn i_036e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E78;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E78: bclr.b #$5, $8d(a6)
pub fn i_036e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E7E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E7E: jsr $33016(pc)
pub fn i_036e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36E82);
    cpu.pc = 0x33016;
    16
}
/// $036E82: jsr $27bc8.l
pub fn i_036e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E88);
    cpu.pc = 0x27BC8;
    16
}
/// $036E88: bcc.w $36e9e
pub fn i_036e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36E9E; return 10; }
    8
}
/// $036E8C: lea.l $33ede(pc), a1
pub fn i_036e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33EDE;
    16
}
/// $036E90: move.l a1, (a6)
pub fn i_036e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036E92: bclr.b #$1, $8d(a6)
pub fn i_036e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E98;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E98: bclr.b #$5, $8d(a6)
pub fn i_036e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E9E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E9E: jsr $32aa8(pc)
pub fn i_036e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36EA2);
    cpu.pc = 0x32AA8;
    16
}
/// $036EA2: bcc.w $36eac
pub fn i_036ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36EAC; return 10; }
    8
}
/// $036EA6: bclr.b #$1, $8c(a6)
pub fn i_036ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036EAC: cmpi.l #$36ff6, (a6)
pub fn i_036eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EB2;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x36FF6, Size::Long, false);
    16
}
/// $036EB2: beq.b $36ebe
pub fn i_036eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EB4;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36EBE; return 10; }
    8
}
/// $036EB4: cmpi.l #$37046, (a6)
pub fn i_036eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EBA;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x37046, Size::Long, false);
    16
}
/// $036EBA: bne.w $36ec2
pub fn i_036eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36EC2; return 10; }
    8
}
/// $036EBE: bra.w $36ec6
pub fn i_036ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x36EC6;
    10
}
/// $036EC2: jsr $3705a(pc)
pub fn i_036ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36EC6);
    cpu.pc = 0x3705A;
    16
}
/// $036EC6: move.w $2a(a6), d0
pub fn i_036ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ECA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036ECA: bgt.w $36f56
pub fn i_036eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ECE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36F56; return 10; }
    8
}
/// $036ECE: bset.b #$5, $8d(a6)
pub fn i_036ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ED4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036ED4: btst.b #$0, $8c(a6)
pub fn i_036ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EDA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036EDA: beq.w $36ee8
pub fn i_036eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36EE8; return 10; }
    8
}
/// $036EDE: lea.l $364a2(pc), a1
pub fn i_036ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x364A2;
    16
}
/// $036EE2: move.l a1, (a6)
pub fn i_036ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036EE4: jmp $36c3a(pc)
pub fn i_036ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EE8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036EE8: btst.b #$1, $8d(a6)
pub fn i_036ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EEE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036EEE: beq.w $36f1e
pub fn i_036eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EF2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F1E; return 10; }
    8
}
/// $036EF2: jsr $8f470.l
pub fn i_036ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36EF8);
    cpu.pc = 0x8F470;
    16
}
/// $036EF8: bcc.w $36f1e
pub fn i_036ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36F1E; return 10; }
    8
}
/// $036EFC: cmpi.w #$0, d2
pub fn i_036efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F00;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $036F00: bmi.w $36f1e
pub fn i_036f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F04;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36F1E; return 10; }
    8
}
/// $036F04: move.b d2, $86(a6)
pub fn i_036f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F08;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036F08: lea.l $36638(pc), a1
pub fn i_036f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36638;
    16
}
/// $036F0C: move.l a1, (a6)
pub fn i_036f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036F0E: jsr $8f520.l
pub fn i_036f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36F14);
    cpu.pc = 0x8F520;
    16
}
/// $036F14: bclr.b #$5, $8d(a6)
pub fn i_036f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F1A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036F1A: jmp $36f70(pc)
pub fn i_036f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F1E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36F70;
    8
}
/// $036F1E: move.b #$3, d0
pub fn i_036f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F22;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $036F22: and.b $69(a6), d0
pub fn i_036f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F26;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $036F26: beq.w $36f56
pub fn i_036f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F56; return 10; }
    8
}
/// $036F2A: btst.b #$2, $69(a6)
pub fn i_036f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F30;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036F30: beq.w $36f56
pub fn i_036f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F56; return 10; }
    8
}
/// $036F34: lea.l $368e0(pc), a1
pub fn i_036f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x368E0;
    16
}
/// $036F38: move.l a1, (a6)
pub fn i_036f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036F3A: move.w #$400, $28(a6)
pub fn i_036f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F40;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036F40: clr.w $2c(a6)
pub fn i_036f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F44;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036F44: btst.b #$1, $69(a6)
pub fn i_036f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F4A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036F4A: bne.w $36f52
pub fn i_036f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36F52; return 10; }
    8
}
/// $036F4E: neg.w $28(a6)
pub fn i_036f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F52;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $036F52: jmp $36f70(pc)
pub fn i_036f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F56;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36F70;
    8
}
/// $036F56: cmpi.l #$36ff6, (a6)
pub fn i_036f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F5C;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x36FF6, Size::Long, false);
    16
}
/// $036F5C: beq.b $36f68
pub fn i_036f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F5E;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F68; return 10; }
    8
}
/// $036F5E: cmpi.l #$37046, (a6)
pub fn i_036f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F64;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x37046, Size::Long, false);
    16
}
/// $036F64: bne.w $36f6c
pub fn i_036f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F68;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36F6C; return 10; }
    8
}
/// $036F68: bra.w $36f70
pub fn i_036f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x36F70;
    10
}
/// $036F6C: jsr $37168(pc)
pub fn i_036f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36F70);
    cpu.pc = 0x37168;
    16
}
/// $036F70: btst.b #$1, $8d(a6)
pub fn i_036f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F76;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036F76: beq.w $36fbc
pub fn i_036f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F7A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36FBC; return 10; }
    8
}
/// $036F7A: cmpi.b #$3, $106ece.l
pub fn i_036f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F82;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $036F82: beq.w $36f9a
pub fn i_036f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F9A; return 10; }
    8
}
/// $036F86: movea.l #$ffffffff, a0
pub fn i_036f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F8C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036F8C: lea.l $324c6(pc), a0
pub fn i_036f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $036F90: jsr $5dd56.l
pub fn i_036f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36F96);
    cpu.pc = 0x5DD56;
    16
}
/// $036F96: bra.w $36faa
pub fn i_036f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x36FAA;
    10
}
/// $036F9A: movea.l #$ffffffff, a0
pub fn i_036f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FA0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036FA0: lea.l $324bc(pc), a0
pub fn i_036fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $036FA4: jsr $5dd56.l
pub fn i_036fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36FAA);
    cpu.pc = 0x5DD56;
    16
}
/// $036FAA: bcc.w $36fb4
pub fn i_036faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36FB4; return 10; }
    8
}
/// $036FAE: lea.l $37b8e(pc), a1
pub fn i_036fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $036FB2: move.l a1, (a6)
pub fn i_036fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036FB4: jsr $33522(pc)
pub fn i_036fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36FB8);
    cpu.pc = 0x33522;
    16
}
/// $036FB8: bra.w $36fc0
pub fn i_036fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x36FC0;
    10
}
/// $036FBC: jsr $33522(pc)
pub fn i_036fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36FC0);
    cpu.pc = 0x33522;
    16
}
/// $036FC0: rts
pub fn i_036fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerAnimState_03705A  ($03705A, 270 B, player/helpers_033638.s)
/// $03705A: btst.b #$2, $8c(a6)
pub fn i_03705a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37060;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037060: bne.w $37166
pub fn i_037060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37064;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37166; return 10; }
    8
}
/// $037064: btst.b #$1, $8c(a6)
pub fn i_037064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3706A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03706A: bne.w $37166
pub fn i_03706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3706E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37166; return 10; }
    8
}
/// $03706E: cmpi.b #$8, $78(a6)
pub fn i_03706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37074;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Byte, false);
    16
}
/// $037074: bne.w $370ec
pub fn i_037074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37078;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x370EC; return 10; }
    8
}
/// $037078: cmpi.w #$3, $72(a6)
pub fn i_037078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3707E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03707E: beq.w $370e8
pub fn i_03707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37082;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x370E8; return 10; }
    8
}
/// $037082: andi.w #$fffe, $38(a6)
pub fn i_037082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37088;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $037088: move.w #$3, $72(a6)
pub fn i_037088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3708E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03708E: cmpi.b #$26, $70(a6)
pub fn i_03708e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37094;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $037094: beq.w $370ba
pub fn i_037094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37098;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x370BA; return 10; }
    8
}
/// $037098: move.w #$2, $7c(a6)
pub fn i_037098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3709E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03709E: move.w #$2, $7e(a6)
pub fn i_03709e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370A4: move.b #$25, $70(a6)
pub fn i_0370a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0370AA: lea.l $279b0e.l, a0
pub fn i_0370aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B0E;
    20
}
/// $0370B0: move.l -$4(a0), $74(a6)
pub fn i_0370b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370B6;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0370B6: bra.w $370d8
pub fn i_0370b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x370D8;
    10
}
/// $0370BA: move.w #$0, $7c(a6)
pub fn i_0370ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370C0: move.w #$7, $7e(a6)
pub fn i_0370c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370C6: move.b #$26, $70(a6)
pub fn i_0370c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0370CC: lea.l $279b04.l, a0
pub fn i_0370cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B04;
    20
}
/// $0370D2: move.l -$4(a0), $74(a6)
pub fn i_0370d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370D8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0370D8: cmpi.b #$18, $79(a6)
pub fn i_0370d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Byte, false);
    16
}
/// $0370DE: bne.w $370e8
pub fn i_0370de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x370E8; return 10; }
    8
}
/// $0370E2: bset.b #$5, $69(a6)
pub fn i_0370e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370E8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0370E8: bra.w $37166
pub fn i_0370e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x37166;
    10
}
/// $0370EC: cmpi.w #$0, $72(a6)
pub fn i_0370ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370F2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0370F2: beq.w $37166
pub fn i_0370f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37166; return 10; }
    8
}
/// $0370F6: move.w #$0, $72(a6)
pub fn i_0370f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370FC: cmpi.b #$26, $70(a6)
pub fn i_0370fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37102;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $037102: beq.w $37138
pub fn i_037102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37106;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37138; return 10; }
    8
}
/// $037106: move.w #$0, $7c(a6)
pub fn i_037106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3710C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03710C: move.w #$2, $7e(a6)
pub fn i_03710c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37112;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037112: move.b #$25, $70(a6)
pub fn i_037112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37118;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037118: lea.l $279b22.l, a0
pub fn i_037118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3711E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B22;
    20
}
/// $03711E: move.l -$4(a0), $74(a6)
pub fn i_03711e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37124;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037124: cmpi.b #$0, $71(a6)
pub fn i_037124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3712A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03712A: bne.w $37134
pub fn i_03712a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3712E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37134; return 10; }
    8
}
/// $03712E: ori.w #$1, $38(a6)
pub fn i_03712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37134;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $037134: bra.w $37156
pub fn i_037134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37138;
    cpu.ir = 0x6000;
    cpu.pc = 0x37156;
    10
}
/// $037138: move.w #$0, $7c(a6)
pub fn i_037138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3713E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03713E: move.w #$7, $7e(a6)
pub fn i_03713e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37144;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037144: move.b #$26, $70(a6)
pub fn i_037144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3714A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03714A: lea.l $279b18.l, a0
pub fn i_03714a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37150;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B18;
    20
}
/// $037150: move.l -$4(a0), $74(a6)
pub fn i_037150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37156;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037156: cmpi.b #$81, $79(a6)
pub fn i_037156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3715C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x81, Size::Byte, false);
    16
}
/// $03715C: bne.w $37166
pub fn i_03715c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37160;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37166; return 10; }
    8
}
/// $037160: bset.b #$5, $69(a6)
pub fn i_037160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37166;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037166: rts
pub fn i_037166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37168;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_AirActionSelect_037168  ($037168, 1130 B, player/air_death_crouch_036632.s)
/// $037168: jsr $330d0(pc)
pub fn i_037168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3716C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3716C);
    cpu.pc = 0x330D0;
    16
}
/// $03716C: bcc.w $375d0
pub fn i_03716c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37170;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x375D0; return 10; }
    8
}
/// $037170: bset.b #$2, $8c(a6)
pub fn i_037170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37176;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037176: cmpi.b #$ff, d1
pub fn i_037176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3717A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03717A: bne.w $371d8
pub fn i_03717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3717E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x371D8; return 10; }
    8
}
/// $03717E: bset.b #$5, $69(a6)
pub fn i_03717e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37184;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037184: cmpi.b #$26, $70(a6)
pub fn i_037184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3718A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03718A: beq.w $371b0
pub fn i_03718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3718E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x371B0; return 10; }
    8
}
/// $03718E: move.w #$0, $7c(a6)
pub fn i_03718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37194;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037194: move.w #$2, $7e(a6)
pub fn i_037194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3719A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03719A: move.b #$25, $70(a6)
pub fn i_03719a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0371A0: lea.l $279d56.l, a0
pub fn i_0371a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $0371A6: move.l -$4(a0), $74(a6)
pub fn i_0371a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371AC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0371AC: bra.w $371ce
pub fn i_0371ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x371CE;
    10
}
/// $0371B0: move.w #$0, $7c(a6)
pub fn i_0371b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371B6: move.w #$7, $7e(a6)
pub fn i_0371b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371BC: move.b #$26, $70(a6)
pub fn i_0371bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0371C2: lea.l $279d56.l, a0
pub fn i_0371c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $0371C8: move.l -$4(a0), $74(a6)
pub fn i_0371c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371CE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0371CE: move.w #$0, $72(a6)
pub fn i_0371ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371D4: bra.w $375d0
pub fn i_0371d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $0371D8: bset.b #$5, $69(a6)
pub fn i_0371d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0371DE: cmpi.b #$3, d1
pub fn i_0371de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371E2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0371E2: bne.w $37328
pub fn i_0371e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37328; return 10; }
    8
}
/// $0371E6: cmpi.b #$26, $70(a6)
pub fn i_0371e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371EC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $0371EC: beq.w $3729e
pub fn i_0371ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3729E; return 10; }
    8
}
/// $0371F0: move.w #$0, $7c(a6)
pub fn i_0371f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371F6: move.w #$0, $7e(a6)
pub fn i_0371f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371FC: jsr $5d5b6.l
pub fn i_0371fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37202);
    cpu.pc = 0x5D5B6;
    16
}
/// $037202: btst.b #$0, $3a(a6)
pub fn i_037202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37208;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037208: bne.w $37214
pub fn i_037208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3720C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37214; return 10; }
    8
}
/// $03720C: move.b #$1, d2
pub fn i_03720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37210;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037210: bra.w $37218
pub fn i_037210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37214;
    cpu.ir = 0x6000;
    cpu.pc = 0x37218;
    10
}
/// $037214: move.b #$2, d2
pub fn i_037214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37218;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037218: cmpi.b #$0, d0
pub fn i_037218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3721C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $03721C: bne.w $3723c
pub fn i_03721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37220;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3723C; return 10; }
    8
}
/// $037220: cmpi.b #$1, $78(a6)
pub fn i_037220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37226;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037226: bne.w $3722e
pub fn i_037226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3722A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3722E; return 10; }
    8
}
/// $03722A: move.b #$ff, d2
pub fn i_03722a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3722E;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03722E: cmpi.b #$2, $78(a6)
pub fn i_03722e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37234;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $037234: bne.w $3723c
pub fn i_037234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37238;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3723C; return 10; }
    8
}
/// $037238: move.b #$0, d2
pub fn i_037238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3723C;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03723C: btst.b #$2, $100001.l
pub fn i_03723c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37244;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $037244: bne.w $37248
pub fn i_037244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37248;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37248; return 10; }
    8
}
/// $037248: cmp.b d0, d2
pub fn i_037248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3724A;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $03724A: bne.w $37276
pub fn i_03724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3724E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37276; return 10; }
    8
}
/// $03724E: move.w #$3, $7c(a6)
pub fn i_03724e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37254;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037254: move.w #$fffa, $7e(a6)
pub fn i_037254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3725A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03725A: move.b #$25, $70(a6)
pub fn i_03725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37260;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037260: lea.l $279d4c.l, a0
pub fn i_037260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D4C;
    20
}
/// $037266: move.l -$4(a0), $74(a6)
pub fn i_037266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3726C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03726C: move.w #$2, $72(a6)
pub fn i_03726c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37272;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037272: bra.w $3729a
pub fn i_037272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37276;
    cpu.ir = 0x6000;
    cpu.pc = 0x3729A;
    10
}
/// $037276: move.w #$0, $7c(a6)
pub fn i_037276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3727C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03727C: move.w #$2, $7e(a6)
pub fn i_03727c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37282;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037282: move.b #$25, $70(a6)
pub fn i_037282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37288;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037288: lea.l $279d42.l, a0
pub fn i_037288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3728E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D42;
    20
}
/// $03728E: move.l -$4(a0), $74(a6)
pub fn i_03728e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37294;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037294: move.w #$0, $72(a6)
pub fn i_037294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3729A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03729A: bra.w $37324
pub fn i_03729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3729E;
    cpu.ir = 0x6000;
    cpu.pc = 0x37324;
    10
}
/// $03729E: move.w #$6, $7c(a6)
pub fn i_03729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372A4: move.w #$3, $7e(a6)
pub fn i_0372a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372AA: jsr $5d5b6.l
pub fn i_0372aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x372B0);
    cpu.pc = 0x5D5B6;
    16
}
/// $0372B0: btst.b #$0, $3a(a6)
pub fn i_0372b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372B6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0372B6: bne.w $372c2
pub fn i_0372b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x372C2; return 10; }
    8
}
/// $0372BA: move.b #$1, d2
pub fn i_0372ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372BE;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0372BE: bra.w $372c6
pub fn i_0372be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x372C6;
    10
}
/// $0372C2: move.b #$2, d2
pub fn i_0372c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372C6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0372C6: btst.b #$2, $100001.l
pub fn i_0372c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372CE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0372CE: bne.w $372d2
pub fn i_0372ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x372D2; return 10; }
    8
}
/// $0372D2: cmp.b d0, d2
pub fn i_0372d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372D4;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0372D4: bne.w $37300
pub fn i_0372d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37300; return 10; }
    8
}
/// $0372D8: move.w #$0, $7c(a6)
pub fn i_0372d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372DE: move.w #$0, $7e(a6)
pub fn i_0372de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372E4: move.b #$26, $70(a6)
pub fn i_0372e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0372EA: lea.l $279d4c.l, a0
pub fn i_0372ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D4C;
    20
}
/// $0372F0: move.l -$4(a0), $74(a6)
pub fn i_0372f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372F6;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0372F6: move.w #$2, $72(a6)
pub fn i_0372f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372FC: bra.w $37324
pub fn i_0372fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37300;
    cpu.ir = 0x6000;
    cpu.pc = 0x37324;
    10
}
/// $037300: move.w #$0, $7c(a6)
pub fn i_037300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37306;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037306: move.w #$7, $7e(a6)
pub fn i_037306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3730C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03730C: move.b #$26, $70(a6)
pub fn i_03730c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37312;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037312: lea.l $279d42.l, a0
pub fn i_037312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37318;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D42;
    20
}
/// $037318: move.l -$4(a0), $74(a6)
pub fn i_037318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3731E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03731E: move.w #$0, $72(a6)
pub fn i_03731e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37324;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037324: bra.w $375d0
pub fn i_037324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37328;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $037328: cmpi.b #$4, d1
pub fn i_037328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3732C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $03732C: bne.w $37390
pub fn i_03732c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37330;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37390; return 10; }
    8
}
/// $037330: andi.w #$fffe, $38(a6)
pub fn i_037330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37336;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $037336: cmpi.b #$26, $70(a6)
pub fn i_037336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3733C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03733C: beq.w $37362
pub fn i_03733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37340;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37362; return 10; }
    8
}
/// $037340: move.w #$2, $7c(a6)
pub fn i_037340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37346;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037346: move.w #$2, $7e(a6)
pub fn i_037346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3734C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03734C: move.b #$25, $70(a6)
pub fn i_03734c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37352;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037352: lea.l $279c6e.l, a0
pub fn i_037352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37358;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C6E;
    20
}
/// $037358: move.l -$4(a0), $74(a6)
pub fn i_037358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3735E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03735E: bra.w $37380
pub fn i_03735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37362;
    cpu.ir = 0x6000;
    cpu.pc = 0x37380;
    10
}
/// $037362: move.w #$0, $7c(a6)
pub fn i_037362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37368;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037368: move.w #$7, $7e(a6)
pub fn i_037368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3736E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03736E: move.b #$26, $70(a6)
pub fn i_03736e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37374;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037374: lea.l $279afa.l, a0
pub fn i_037374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3737A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AFA;
    20
}
/// $03737A: move.l -$4(a0), $74(a6)
pub fn i_03737a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37380;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037380: move.w #$3, $72(a6)
pub fn i_037380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37386;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037386: bset.b #$1, $8c(a6)
pub fn i_037386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3738C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03738C: bra.w $375d0
pub fn i_03738c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37390;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $037390: cmpi.b #$1, d1
pub fn i_037390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37394;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $037394: bne.w $373f2
pub fn i_037394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37398;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x373F2; return 10; }
    8
}
/// $037398: cmpi.b #$26, $70(a6)
pub fn i_037398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3739E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03739E: beq.w $373c4
pub fn i_03739e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x373C4; return 10; }
    8
}
/// $0373A2: move.w #$0, $7c(a6)
pub fn i_0373a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373A8: move.w #$2, $7e(a6)
pub fn i_0373a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373AE: move.b #$25, $70(a6)
pub fn i_0373ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0373B4: lea.l $279c3c.l, a0
pub fn i_0373b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C3C;
    20
}
/// $0373BA: move.l -$4(a0), $74(a6)
pub fn i_0373ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373C0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0373C0: bra.w $373e2
pub fn i_0373c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x373E2;
    10
}
/// $0373C4: move.w #$0, $7c(a6)
pub fn i_0373c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373CA: move.w #$6, $7e(a6)
pub fn i_0373ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373D0: move.b #$26, $70(a6)
pub fn i_0373d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0373D6: lea.l $279ad2.l, a0
pub fn i_0373d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AD2;
    20
}
/// $0373DC: move.l -$4(a0), $74(a6)
pub fn i_0373dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373E2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0373E2: move.w #$1, $72(a6)
pub fn i_0373e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373E8: bset.b #$1, $8c(a6)
pub fn i_0373e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373EE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0373EE: bra.w $375d0
pub fn i_0373ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $0373F2: btst.b #$2, $100001.l
pub fn i_0373f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373FA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0373FA: bne.w $373fe
pub fn i_0373fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x373FE; return 10; }
    8
}
/// $0373FE: cmpi.b #$2, d1
pub fn i_0373fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37402;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $037402: bne.w $3746c
pub fn i_037402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37406;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3746C; return 10; }
    8
}
/// $037406: cmpi.b #$26, $70(a6)
pub fn i_037406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3740C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03740C: beq.w $37438
pub fn i_03740c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37410;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37438; return 10; }
    8
}
/// $037410: move.w #$6, $7c(a6)
pub fn i_037410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37416;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037416: move.w #$fffd, $7e(a6)
pub fn i_037416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3741C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03741C: move.b #$25, $70(a6)
pub fn i_03741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37422;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037422: lea.l $279c32.l, a0
pub fn i_037422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37428;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C32;
    20
}
/// $037428: move.l -$4(a0), $74(a6)
pub fn i_037428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3742E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03742E: move.w #$2, $72(a6)
pub fn i_03742e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37434;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037434: bra.w $3745c
pub fn i_037434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37438;
    cpu.ir = 0x6000;
    cpu.pc = 0x3745C;
    10
}
/// $037438: move.w #$0, $7c(a6)
pub fn i_037438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3743E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03743E: move.w #$0, $7e(a6)
pub fn i_03743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37444;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037444: move.b #$26, $70(a6)
pub fn i_037444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3744A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03744A: lea.l $279ac8.l, a0
pub fn i_03744a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37450;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AC8;
    20
}
/// $037450: move.l -$4(a0), $74(a6)
pub fn i_037450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37456;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037456: move.w #$2, $72(a6)
pub fn i_037456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3745C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03745C: move.w #$2, $72(a6)
pub fn i_03745c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37462;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037462: bset.b #$1, $8c(a6)
pub fn i_037462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37468;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037468: bra.w $375d0
pub fn i_037468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3746C;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $03746C: cmpi.b #$26, $70(a6)
pub fn i_03746c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37472;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $037472: beq.w $37528
pub fn i_037472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37476;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37528; return 10; }
    8
}
/// $037476: jsr $5d5b6.l
pub fn i_037476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3747C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3747C);
    cpu.pc = 0x5D5B6;
    16
}
/// $03747C: btst.b #$0, $3a(a6)
pub fn i_03747c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37482;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037482: bne.w $3748e
pub fn i_037482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37486;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3748E; return 10; }
    8
}
/// $037486: move.b #$1, d2
pub fn i_037486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3748A;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03748A: bra.w $37492
pub fn i_03748a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3748E;
    cpu.ir = 0x6000;
    cpu.pc = 0x37492;
    10
}
/// $03748E: move.b #$2, d2
pub fn i_03748e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37492;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037492: cmpi.b #$0, d0
pub fn i_037492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37496;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $037496: bne.w $374b6
pub fn i_037496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3749A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374B6; return 10; }
    8
}
/// $03749A: cmpi.b #$1, $78(a6)
pub fn i_03749a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374A0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0374A0: bne.w $374a8
pub fn i_0374a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374A8; return 10; }
    8
}
/// $0374A4: move.b #$ff, d2
pub fn i_0374a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374A8;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0374A8: cmpi.b #$2, $78(a6)
pub fn i_0374a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374AE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0374AE: bne.w $374b6
pub fn i_0374ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374B6; return 10; }
    8
}
/// $0374B2: move.b #$0, d2
pub fn i_0374b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374B6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0374B6: btst.b #$2, $100001.l
pub fn i_0374b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374BE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0374BE: bne.w $374c2
pub fn i_0374be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374C2; return 10; }
    8
}
/// $0374C2: cmp.b d0, d2
pub fn i_0374c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374C4;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0374C4: bne.w $374f0
pub fn i_0374c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374F0; return 10; }
    8
}
/// $0374C8: move.w #$6, $7c(a6)
pub fn i_0374c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374CE: move.w #$fffd, $7e(a6)
pub fn i_0374ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374D4: move.b #$25, $70(a6)
pub fn i_0374d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0374DA: lea.l $279c32.l, a0
pub fn i_0374da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C32;
    20
}
/// $0374E0: move.l -$4(a0), $74(a6)
pub fn i_0374e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374E6;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0374E6: move.w #$2, $72(a6)
pub fn i_0374e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374EC: bra.w $37524
pub fn i_0374ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x37524;
    10
}
/// $0374F0: move.w #$0, $7c(a6)
pub fn i_0374f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374F6: move.w #$2, $7e(a6)
pub fn i_0374f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374FC: move.b #$25, $70(a6)
pub fn i_0374fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37502;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037502: lea.l $279c28.l, a0
pub fn i_037502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37508;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C28;
    20
}
/// $037508: move.l -$4(a0), $74(a6)
pub fn i_037508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3750E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03750E: cmpi.b #$0, $71(a6)
pub fn i_03750e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37514;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037514: bne.w $3751e
pub fn i_037514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37518;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3751E; return 10; }
    8
}
/// $037518: ori.w #$1, $38(a6)
pub fn i_037518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3751E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03751E: move.w #$0, $72(a6)
pub fn i_03751e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37524;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037524: bra.w $375c6
pub fn i_037524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37528;
    cpu.ir = 0x6000;
    cpu.pc = 0x375C6;
    10
}
/// $037528: jsr $5d5b6.l
pub fn i_037528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3752E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3752E);
    cpu.pc = 0x5D5B6;
    16
}
/// $03752E: btst.b #$0, $3a(a6)
pub fn i_03752e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37534;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037534: bne.w $37540
pub fn i_037534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37538;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37540; return 10; }
    8
}
/// $037538: move.b #$1, d2
pub fn i_037538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3753C;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03753C: bra.w $37544
pub fn i_03753c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37540;
    cpu.ir = 0x6000;
    cpu.pc = 0x37544;
    10
}
/// $037540: move.b #$2, d2
pub fn i_037540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37544;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037544: cmpi.b #$0, d0
pub fn i_037544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37548;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $037548: bne.w $37568
pub fn i_037548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3754C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37568; return 10; }
    8
}
/// $03754C: cmpi.b #$1, $78(a6)
pub fn i_03754c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37552;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037552: bne.w $3755a
pub fn i_037552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37556;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3755A; return 10; }
    8
}
/// $037556: move.b #$ff, d2
pub fn i_037556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3755A;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03755A: cmpi.b #$2, $78(a6)
pub fn i_03755a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37560;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $037560: bne.w $37568
pub fn i_037560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37564;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37568; return 10; }
    8
}
/// $037564: move.b #$0, d2
pub fn i_037564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37568;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037568: btst.b #$2, $100001.l
pub fn i_037568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37570;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $037570: bne.w $37574
pub fn i_037570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37574;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37574; return 10; }
    8
}
/// $037574: cmp.b d0, d2
pub fn i_037574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37576;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $037576: bne.w $375a2
pub fn i_037576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3757A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x375A2; return 10; }
    8
}
/// $03757A: move.w #$0, $7c(a6)
pub fn i_03757a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37580;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037580: move.w #$0, $7e(a6)
pub fn i_037580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37586;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037586: move.b #$26, $70(a6)
pub fn i_037586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3758C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03758C: lea.l $279ac8.l, a0
pub fn i_03758c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37592;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AC8;
    20
}
/// $037592: move.l -$4(a0), $74(a6)
pub fn i_037592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37598;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037598: move.w #$2, $72(a6)
pub fn i_037598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3759E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03759E: bra.w $375c6
pub fn i_03759e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x375C6;
    10
}
/// $0375A2: move.w #$0, $7c(a6)
pub fn i_0375a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0375A8: move.w #$7, $7e(a6)
pub fn i_0375a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0375AE: move.b #$26, $70(a6)
pub fn i_0375ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0375B4: lea.l $279abe.l, a0
pub fn i_0375b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279ABE;
    20
}
/// $0375BA: move.l -$4(a0), $74(a6)
pub fn i_0375ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375C0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0375C0: move.w #$0, $72(a6)
pub fn i_0375c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0375C6: bset.b #$1, $8c(a6)
pub fn i_0375c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375CC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0375CC: bra.w $375d0
pub fn i_0375cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $0375D0: rts
pub fn i_0375d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_FallSpawnFx_0378c6  ($0378C6, 284 B, player/air_death_crouch_036632.s)
/// $0378C6: lea.l $391ee(pc), a1
pub fn i_0378c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391EE;
    16
}
/// $0378CA: jsr $4ae.l
pub fn i_0378ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378D0);
    cpu.pc = 0x4AE;
    16
}
/// $0378D0: jsr $5dd02.l
pub fn i_0378d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378D6);
    cpu.pc = 0x5DD02;
    16
}
/// $0378D6: lea.l $39148(pc), a1
pub fn i_0378d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39148;
    16
}
/// $0378DA: jsr $4ae.l
pub fn i_0378da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378E0);
    cpu.pc = 0x4AE;
    16
}
/// $0378E0: jsr $5dd02.l
pub fn i_0378e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378E6);
    cpu.pc = 0x5DD02;
    16
}
/// $0378E6: move.w #$0, $7c(a6)
pub fn i_0378e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0378EC: move.w #$0, $7e(a6)
pub fn i_0378ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0378F2: bclr.b #$2, $8c(a6)
pub fn i_0378f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378F8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0378F8: bclr.b #$1, $8c(a6)
pub fn i_0378f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0378FE: bclr.b #$3, $8c(a6)
pub fn i_0378fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37904;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037904: bset.b #$0, $13(a6)
pub fn i_037904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3790A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03790A: move.b #$0, $70(a6)
pub fn i_03790a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37910;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037910: lea.l $279f58.l, a0
pub fn i_037910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37916;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F58;
    20
}
/// $037916: move.l -$4(a0), $74(a6)
pub fn i_037916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3791C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03791C: move.b #$ff, $21(a6)
pub fn i_03791c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37922;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037922: lea.l $279f58.l, a0
pub fn i_037922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37928;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F58;
    20
}
/// $037928: jsr $28cd4.l
pub fn i_037928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3792E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3792E);
    cpu.pc = 0x28CD4;
    16
}
/// $03792E: jsr $267e2.l
pub fn i_03792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37934);
    cpu.pc = 0x267E2;
    16
}
/// $037934: move.w #$fee9, d0
pub fn i_037934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37938;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEE9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $037938: jsr $5dca4.l
pub fn i_037938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3793E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3793E);
    cpu.pc = 0x5DCA4;
    16
}
/// $03793E: move.w d0, $28(a6)
pub fn i_03793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37942;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037942: move.w #$457, $2a(a6)
pub fn i_037942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37948;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x457;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037948: move.w #$ff9b, $2e(a6)
pub fn i_037948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3794E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03794E: move.w #$0, $2c(a6)
pub fn i_03794e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37954;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037954: lea.l $3795a(pc), a1
pub fn i_037954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37958;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3795A;
    16
}
/// $037958: move.l a1, (a6)
pub fn i_037958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3795A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03795A: bset.b #$1, $8d(a6)
pub fn i_03795a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37960;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037960: jsr $32ff2(pc)
pub fn i_037960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37964;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37964);
    cpu.pc = 0x32FF2;
    16
}
/// $037964: jsr $27bc8.l
pub fn i_037964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3796A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3796A);
    cpu.pc = 0x27BC8;
    16
}
/// $03796A: bcc.w $37974
pub fn i_03796a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3796E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37974; return 10; }
    8
}
/// $03796E: lea.l $379e2(pc), a1
pub fn i_03796e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37972;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x379E2;
    16
}
/// $037972: move.l a1, (a6)
pub fn i_037972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37974;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037974: cmpi.b #$1, $106ece.l
pub fn i_037974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3797C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $03797C: bne.w $379a2
pub fn i_03797c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37980;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x379A2; return 10; }
    8
}
/// $037980: move.w $22(a6), d1
pub fn i_037980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37984;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $037984: move.w $24(a6), d2
pub fn i_037984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37988;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $037988: cmpi.w #$10c, d2
pub fn i_037988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3798C;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10C, Size::Word, false);
    8
}
/// $03798C: bge.w $379a2
pub fn i_03798c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37990;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x379A2; return 10; }
    8
}
/// $037990: jsr $27db2.l
pub fn i_037990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37996);
    cpu.pc = 0x27DB2;
    16
}
/// $037996: cmpi.b #$40, d7
pub fn i_037996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3799A;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $03799A: bne.w $379a2
pub fn i_03799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3799E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x379A2; return 10; }
    8
}
/// $03799E: jsr $33522(pc)
pub fn i_03799e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x379A2);
    cpu.pc = 0x33522;
    16
}
/// $0379A2: jsr $32aa8(pc)
pub fn i_0379a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x379A6);
    cpu.pc = 0x32AA8;
    16
}
/// $0379A6: cmpi.b #$3, $106ece.l
pub fn i_0379a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379AE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0379AE: beq.w $379c6
pub fn i_0379ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x379C6; return 10; }
    8
}
/// $0379B2: movea.l #$ffffffff, a0
pub fn i_0379b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379B8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0379B8: lea.l $324c6(pc), a0
pub fn i_0379b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379BC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $0379BC: jsr $5dd56.l
pub fn i_0379bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x379C2);
    cpu.pc = 0x5DD56;
    16
}
/// $0379C2: bra.w $379d6
pub fn i_0379c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x379D6;
    10
}
/// $0379C6: movea.l #$ffffffff, a0
pub fn i_0379c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0379CC: lea.l $324bc(pc), a0
pub fn i_0379cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379D0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $0379D0: jsr $5dd56.l
pub fn i_0379d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x379D6);
    cpu.pc = 0x5DD56;
    16
}
/// $0379D6: bcc.w $379e0
pub fn i_0379d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x379E0; return 10; }
    8
}
/// $0379DA: lea.l $37b8e(pc), a1
pub fn i_0379da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $0379DE: move.l a1, (a6)
pub fn i_0379de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0379E0: rts
pub fn i_0379e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_FallLandedB_0379e2  ($0379E2, 82 B, player/air_death_crouch_036632.s)
/// $0379E2: clr.w $28(a6)
pub fn i_0379e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379E6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0379E6: clr.w $2a(a6)
pub fn i_0379e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0379EA: move.b #$0, $70(a6)
pub fn i_0379ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0379F0: lea.l $279f62.l, a0
pub fn i_0379f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F62;
    20
}
/// $0379F6: move.l -$4(a0), $74(a6)
pub fn i_0379f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379FC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0379FC: move.b #$ff, $21(a6)
pub fn i_0379fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037A02: lea.l $279f62.l, a0
pub fn i_037a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F62;
    20
}
/// $037A08: jsr $28cd4.l
pub fn i_037a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A0E);
    cpu.pc = 0x28CD4;
    16
}
/// $037A0E: jsr $267e2.l
pub fn i_037a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A14);
    cpu.pc = 0x267E2;
    16
}
/// $037A14: lea.l $37a1a(pc), a1
pub fn i_037a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37A1A;
    16
}
/// $037A18: move.l a1, (a6)
pub fn i_037a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037A1A: jsr $32ff2(pc)
pub fn i_037a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37A1E);
    cpu.pc = 0x32FF2;
    16
}
/// $037A1E: jsr $27a92.l
pub fn i_037a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A24);
    cpu.pc = 0x27A92;
    16
}
/// $037A24: jsr $32aa8(pc)
pub fn i_037a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37A28);
    cpu.pc = 0x32AA8;
    16
}
/// $037A28: bcc.w $37a32
pub fn i_037a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37A32; return 10; }
    8
}
/// $037A2C: lea.l $37c1a(pc), a1
pub fn i_037a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C1A;
    16
}
/// $037A30: move.l a1, (a6)
pub fn i_037a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037A32: rts
pub fn i_037a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_Despawn_037c1a  ($037C1A, 90 B, player/air_death_crouch_036632.s)
/// $037C1A: jsr $267e2.l
pub fn i_037c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C20);
    cpu.pc = 0x267E2;
    16
}
/// $037C20: move.w #$0, $7c(a6)
pub fn i_037c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C26;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C26: move.w #$0, $7e(a6)
pub fn i_037c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C2C: move.b #$28, $90(a6)
pub fn i_037c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037C32: lea.l $ffff.w, a0
pub fn i_037c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C36;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $037C36: move.l a0, $48(a6)
pub fn i_037c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C3A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037C3A: lea.l $37c40(pc), a1
pub fn i_037c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C40;
    16
}
/// $037C3E: move.l a1, (a6)
pub fn i_037c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037C40: jsr $2783a.l
pub fn i_037c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C46);
    cpu.pc = 0x2783A;
    16
}
/// $037C46: subi.b #$1, $90(a6)
pub fn i_037c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C4C;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C4C: bne.w $37c72
pub fn i_037c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37C72; return 10; }
    8
}
/// $037C50: jsr $5b6.l
pub fn i_037c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C56);
    cpu.pc = 0x5B6;
    16
}
/// $037C56: bclr.b #$7, $5b(a6)
pub fn i_037c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C5C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C5C: jsr $13600.l
pub fn i_037c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C62);
    cpu.pc = 0x13600;
    16
}
/// $037C62: lea.l $400.l, a1
pub fn i_037c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $037C68: move.l a1, (a6)
pub fn i_037c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037C6A: movea.l a6, a0
pub fn i_037c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C6C;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $037C6C: jsr $5fe.l
pub fn i_037c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C72);
    cpu.pc = 0x5FE;
    16
}
/// $037C72: rts
pub fn i_037c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchEnter_037c74  ($037C74, 326 B, player/air_death_crouch_036632.s)
/// $037C74: move.w #$0, $7c(a6)
pub fn i_037c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C7A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C7A: move.w #$0, $7e(a6)
pub fn i_037c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C80: bclr.b #$2, $8c(a6)
pub fn i_037c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C86;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C86: bclr.b #$1, $8c(a6)
pub fn i_037c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C8C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C8C: bclr.b #$3, $8c(a6)
pub fn i_037c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C92;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C92: move.b #$30, $70(a6)
pub fn i_037c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C98;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037C98: lea.l $279864.l, a0
pub fn i_037c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279864;
    20
}
/// $037C9E: move.l -$4(a0), $74(a6)
pub fn i_037c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CA4;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037CA4: move.b #$ff, $21(a6)
pub fn i_037ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037CAA: lea.l $279864.l, a0
pub fn i_037caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279864;
    20
}
/// $037CB0: jsr $28cd4.l
pub fn i_037cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37CB6);
    cpu.pc = 0x28CD4;
    16
}
/// $037CB6: move.l #$32598, $60(a6)
pub fn i_037cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CBE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037CBE: clr.w $2c(a6)
pub fn i_037cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CC2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037CC2: lea.l $32734(pc), a0
pub fn i_037cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CC6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $037CC6: move.l a0, $48(a6)
pub fn i_037cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CCA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037CCA: lea.l $37cd0(pc), a1
pub fn i_037cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37CD0;
    16
}
/// $037CCE: move.l a1, (a6)
pub fn i_037cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037CD0: jsr $32ff2(pc)
pub fn i_037cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CD4);
    cpu.pc = 0x32FF2;
    16
}
/// $037CD4: jsr $32d32(pc)
pub fn i_037cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CD8);
    cpu.pc = 0x32D32;
    16
}
/// $037CD8: clr.w $28(a6)
pub fn i_037cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CDC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037CDC: jsr $32e3c(pc)
pub fn i_037cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CE0);
    cpu.pc = 0x32E3C;
    16
}
/// $037CE0: bcc.w $37cea
pub fn i_037ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37CEA; return 10; }
    8
}
/// $037CE4: move.w #$300, $28(a6)
pub fn i_037ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037CEA: jsr $32e42(pc)
pub fn i_037cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CEE);
    cpu.pc = 0x32E42;
    16
}
/// $037CEE: bcc.w $37cf8
pub fn i_037cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37CF8; return 10; }
    8
}
/// $037CF2: move.w #$fd00, $28(a6)
pub fn i_037cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037CF8: jsr $27a92.l
pub fn i_037cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37CFE);
    cpu.pc = 0x27A92;
    16
}
/// $037CFE: clr.w $28(a6)
pub fn i_037cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037D02: jsr $32aa8(pc)
pub fn i_037d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D06);
    cpu.pc = 0x32AA8;
    16
}
/// $037D06: bcc.w $37d2c
pub fn i_037d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D2C; return 10; }
    8
}
/// $037D0A: lea.l $3827a(pc), a1
pub fn i_037d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3827A;
    16
}
/// $037D0E: move.l a1, (a6)
pub fn i_037d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D10: jsr $32e3c(pc)
pub fn i_037d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D14);
    cpu.pc = 0x32E3C;
    16
}
/// $037D14: bcc.w $37d1e
pub fn i_037d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D1E; return 10; }
    8
}
/// $037D18: lea.l $3842c(pc), a1
pub fn i_037d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3842C;
    16
}
/// $037D1C: move.l a1, (a6)
pub fn i_037d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D1E: jsr $32e42(pc)
pub fn i_037d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D22);
    cpu.pc = 0x32E42;
    16
}
/// $037D22: bcc.w $37d2c
pub fn i_037d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D2C; return 10; }
    8
}
/// $037D26: lea.l $3855a(pc), a1
pub fn i_037d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3855A;
    16
}
/// $037D2A: move.l a1, (a6)
pub fn i_037d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D2C: jsr $330d0(pc)
pub fn i_037d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D30);
    cpu.pc = 0x330D0;
    16
}
/// $037D30: bcc.w $37d62
pub fn i_037d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D62; return 10; }
    8
}
/// $037D34: cmpi.b #$ff, d1
pub fn i_037d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D38;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $037D38: bne.w $37d46
pub fn i_037d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D46; return 10; }
    8
}
/// $037D3C: lea.l $38a28(pc), a1
pub fn i_037d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $037D40: move.l a1, (a6)
pub fn i_037d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D42: bra.w $37d62
pub fn i_037d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D46;
    cpu.ir = 0x6000;
    cpu.pc = 0x37D62;
    10
}
/// $037D46: cmpi.b #$3, d1
pub fn i_037d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D4A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $037D4A: bne.w $37d58
pub fn i_037d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D58; return 10; }
    8
}
/// $037D4E: lea.l $388f0(pc), a1
pub fn i_037d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $037D52: move.l a1, (a6)
pub fn i_037d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D54: bra.w $37d62
pub fn i_037d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D58;
    cpu.ir = 0x6000;
    cpu.pc = 0x37D62;
    10
}
/// $037D58: lea.l $3873c(pc), a1
pub fn i_037d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $037D5C: move.l a1, (a6)
pub fn i_037d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D5E: bra.w $37d62
pub fn i_037d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D62;
    cpu.ir = 0x6000;
    cpu.pc = 0x37D62;
    10
}
/// $037D62: cmpi.b #$0, $85(a6)
pub fn i_037d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D68;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037D68: bne.w $37d7c
pub fn i_037d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D7C; return 10; }
    8
}
/// $037D6C: cmpi.b #$1, $71(a6)
pub fn i_037d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D72;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037D72: bne.w $37d7c
pub fn i_037d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D7C; return 10; }
    8
}
/// $037D76: lea.l $33b9a(pc), a1
pub fn i_037d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $037D7A: move.l a1, (a6)
pub fn i_037d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D7C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D7C: cmpi.w #$0, $82(a6)
pub fn i_037d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D82;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $037D82: bne.w $37d96
pub fn i_037d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D96; return 10; }
    8
}
/// $037D86: cmpi.b #$0, $71(a6)
pub fn i_037d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D8C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037D8C: beq.w $37d96
pub fn i_037d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37D96; return 10; }
    8
}
/// $037D90: lea.l $38086(pc), a1
pub fn i_037d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38086;
    16
}
/// $037D94: move.l a1, (a6)
pub fn i_037d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D96: jsr $27eba.l
pub fn i_037d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37D9C);
    cpu.pc = 0x27EBA;
    16
}
/// $037D9C: bcc.w $37da6
pub fn i_037d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37DA6; return 10; }
    8
}
/// $037DA0: lea.l $36d64(pc), a1
pub fn i_037da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $037DA4: move.l a1, (a6)
pub fn i_037da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037DA6: jsr $33034(pc)
pub fn i_037da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37DAA);
    cpu.pc = 0x33034;
    16
}
/// $037DAA: bcc.w $37db4
pub fn i_037daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37DB4; return 10; }
    8
}
/// $037DAE: lea.l $36914(pc), a1
pub fn i_037dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $037DB2: move.l a1, (a6)
pub fn i_037db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037DB4: jsr $33522(pc)
pub fn i_037db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37DB8);
    cpu.pc = 0x33522;
    16
}
/// $037DB8: rts
pub fn i_037db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchExit_037ec2  ($037EC2, 386 B, player/air_death_crouch_036632.s)
/// $037EC2: move.w #$0, $7c(a6)
pub fn i_037ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037EC8: move.w #$0, $7e(a6)
pub fn i_037ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37ECE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037ECE: bclr.b #$2, $8c(a6)
pub fn i_037ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37ED4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037ED4: bclr.b #$1, $8c(a6)
pub fn i_037ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EDA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037EDA: bclr.b #$3, $8c(a6)
pub fn i_037eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EE0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037EE0: move.b #$30, $70(a6)
pub fn i_037ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037EE6: lea.l $27986e.l, a0
pub fn i_037ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27986E;
    20
}
/// $037EEC: move.l -$4(a0), $74(a6)
pub fn i_037eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EF2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037EF2: move.b #$ff, $21(a6)
pub fn i_037ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EF8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037EF8: lea.l $27986e.l, a0
pub fn i_037ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27986E;
    20
}
/// $037EFE: jsr $28cd4.l
pub fn i_037efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37F04);
    cpu.pc = 0x28CD4;
    16
}
/// $037F04: clr.w $2c(a6)
pub fn i_037f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F08;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037F08: lea.l $32734(pc), a0
pub fn i_037f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F0C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $037F0C: move.l a0, $48(a6)
pub fn i_037f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F10;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037F10: lea.l $37f16(pc), a1
pub fn i_037f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37F16;
    16
}
/// $037F14: move.l a1, (a6)
pub fn i_037f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F16: jsr $32ff2(pc)
pub fn i_037f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F1A);
    cpu.pc = 0x32FF2;
    16
}
/// $037F1A: jsr $32d32(pc)
pub fn i_037f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F1E);
    cpu.pc = 0x32D32;
    16
}
/// $037F1E: clr.w $28(a6)
pub fn i_037f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F22;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037F22: jsr $32e3c(pc)
pub fn i_037f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F26);
    cpu.pc = 0x32E3C;
    16
}
/// $037F26: bcc.w $37f30
pub fn i_037f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37F30; return 10; }
    8
}
/// $037F2A: move.w #$300, $28(a6)
pub fn i_037f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037F30: jsr $32e42(pc)
pub fn i_037f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F34);
    cpu.pc = 0x32E42;
    16
}
/// $037F34: bcc.w $37f3e
pub fn i_037f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37F3E; return 10; }
    8
}
/// $037F38: move.w #$fd00, $28(a6)
pub fn i_037f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037F3E: jsr $27a92.l
pub fn i_037f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37F44);
    cpu.pc = 0x27A92;
    16
}
/// $037F44: clr.w $28(a6)
pub fn i_037f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F48;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037F48: jsr $32aa8(pc)
pub fn i_037f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F4C);
    cpu.pc = 0x32AA8;
    16
}
/// $037F4C: bcc.w $37fc6
pub fn i_037f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37FC6; return 10; }
    8
}
/// $037F50: lea.l $34704(pc), a1
pub fn i_037f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $037F54: move.l a1, (a6)
pub fn i_037f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F56: btst.b #$0, $88(a6)
pub fn i_037f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F5C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037F5C: beq.w $37f66
pub fn i_037f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37F66; return 10; }
    8
}
/// $037F60: lea.l $33a5e(pc), a1
pub fn i_037f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $037F64: move.l a1, (a6)
pub fn i_037f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F66: cmpi.b #$0, $85(a6)
pub fn i_037f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F6C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037F6C: bne.w $37f80
pub fn i_037f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37F80; return 10; }
    8
}
/// $037F70: cmpi.b #$1, $71(a6)
pub fn i_037f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F76;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037F76: bne.w $37f80
pub fn i_037f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37F80; return 10; }
    8
}
/// $037F7A: lea.l $33b9a(pc), a1
pub fn i_037f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $037F7E: move.l a1, (a6)
pub fn i_037f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F80: jsr $5e9b6.l
pub fn i_037f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37F86);
    cpu.pc = 0x5E9B6;
    16
}
/// $037F86: andi.w #$7, d0
pub fn i_037f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F8A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $037F8A: moveq #$4, d0
pub fn i_037f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F8C;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $037F8C: cmp.w $82(a6), d0
pub fn i_037f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F90;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $037F90: bne.w $37f9a
pub fn i_037f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37F9A; return 10; }
    8
}
/// $037F94: lea.l $33afc(pc), a1
pub fn i_037f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AFC;
    16
}
/// $037F98: move.l a1, (a6)
pub fn i_037f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F9A: cmpi.w #$0, $82(a6)
pub fn i_037f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FA0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $037FA0: bne.w $37faa
pub fn i_037fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37FAA; return 10; }
    8
}
/// $037FA4: lea.l $33d64(pc), a1
pub fn i_037fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $037FA8: move.l a1, (a6)
pub fn i_037fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FAA: jsr $32e3c(pc)
pub fn i_037faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37FAE);
    cpu.pc = 0x32E3C;
    16
}
/// $037FAE: bcc.w $37fb8
pub fn i_037fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37FB8; return 10; }
    8
}
/// $037FB2: lea.l $351d8(pc), a1
pub fn i_037fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x351D8;
    16
}
/// $037FB6: move.l a1, (a6)
pub fn i_037fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FB8: jsr $32e42(pc)
pub fn i_037fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37FBC);
    cpu.pc = 0x32E42;
    16
}
/// $037FBC: bcc.w $37fc6
pub fn i_037fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37FC6; return 10; }
    8
}
/// $037FC0: lea.l $354f2(pc), a1
pub fn i_037fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x354F2;
    16
}
/// $037FC4: move.l a1, (a6)
pub fn i_037fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FC6: jsr $330d0(pc)
pub fn i_037fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37FCA);
    cpu.pc = 0x330D0;
    16
}
/// $037FCA: bcc.w $38020
pub fn i_037fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38020; return 10; }
    8
}
/// $037FCE: cmpi.b #$ff, d1
pub fn i_037fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FD2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $037FD2: bne.w $37fe0
pub fn i_037fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37FE0; return 10; }
    8
}
/// $037FD6: lea.l $35d34(pc), a1
pub fn i_037fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $037FDA: move.l a1, (a6)
pub fn i_037fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FDC: bra.w $38020
pub fn i_037fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $037FE0: cmpi.b #$3, d1
pub fn i_037fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FE4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $037FE4: bne.w $37ff2
pub fn i_037fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37FF2; return 10; }
    8
}
/// $037FE8: lea.l $360bc(pc), a1
pub fn i_037fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $037FEC: move.l a1, (a6)
pub fn i_037fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FEE: bra.w $38020
pub fn i_037fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $037FF2: cmpi.b #$4, d1
pub fn i_037ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FF6;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $037FF6: bne.w $38004
pub fn i_037ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FFA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38004; return 10; }
    8
}
/// $037FFA: lea.l $3873c(pc), a1
pub fn i_037ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $037FFE: move.l a1, (a6)
pub fn i_037ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38000;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038000: bra.w $38020
pub fn i_038000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38004;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $038004: cmpi.b #$1, d1
pub fn i_038004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38008;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $038008: bne.w $38016
pub fn i_038008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3800C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38016; return 10; }
    8
}
/// $03800C: lea.l $3437e(pc), a1
pub fn i_03800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3437E;
    16
}
/// $038010: move.l a1, (a6)
pub fn i_038010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038012: bra.w $38020
pub fn i_038012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38016;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $038016: lea.l $342c4(pc), a1
pub fn i_038016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3801A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x342C4;
    16
}
/// $03801A: move.l a1, (a6)
pub fn i_03801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3801C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03801C: bra.w $38020
pub fn i_03801c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38020;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $038020: jsr $27eba.l
pub fn i_038020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38026);
    cpu.pc = 0x27EBA;
    16
}
/// $038026: bcc.w $38030
pub fn i_038026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3802A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38030; return 10; }
    8
}
/// $03802A: lea.l $36d64(pc), a1
pub fn i_03802a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3802E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $03802E: move.l a1, (a6)
pub fn i_03802e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38030;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038030: jsr $33034(pc)
pub fn i_038030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38034;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38034);
    cpu.pc = 0x33034;
    16
}
/// $038034: bcc.w $3803e
pub fn i_038034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38038;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3803E; return 10; }
    8
}
/// $038038: lea.l $36914(pc), a1
pub fn i_038038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3803C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $03803C: move.l a1, (a6)
pub fn i_03803c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3803E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03803E: jsr $33522(pc)
pub fn i_03803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38042;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38042);
    cpu.pc = 0x33522;
    16
}
/// $038042: rts
pub fn i_038042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38044;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchIdleB_0381a2  ($0381A2, 216 B, player/air_death_crouch_036632.s)
/// $0381A2: bclr.b #$2, $8c(a6)
pub fn i_0381a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0381A8: bclr.b #$1, $8c(a6)
pub fn i_0381a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0381AE: bclr.b #$3, $8c(a6)
pub fn i_0381ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0381B4: jsr $2abcc.l
pub fn i_0381b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x381BA);
    cpu.pc = 0x2ABCC;
    16
}
/// $0381BA: bcs.w $381e6
pub fn i_0381ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x381E6; return 10; }
    8
}
/// $0381BE: move.b #$30, $70(a6)
pub fn i_0381be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381C4: lea.l $279850.l, a0
pub fn i_0381c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279850;
    20
}
/// $0381CA: move.l -$4(a0), $74(a6)
pub fn i_0381ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381D0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0381D0: move.b #$ff, $21(a6)
pub fn i_0381d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381D6: lea.l $279850.l, a0
pub fn i_0381d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279850;
    20
}
/// $0381DC: jsr $28cd4.l
pub fn i_0381dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x381E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0381E2: bra.w $3820a
pub fn i_0381e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3820A;
    10
}
/// $0381E6: move.b #$30, $70(a6)
pub fn i_0381e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381EC: lea.l $279a82.l, a0
pub fn i_0381ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $0381F2: move.l -$4(a0), $74(a6)
pub fn i_0381f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381F8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0381F8: move.b #$ff, $21(a6)
pub fn i_0381f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381FE: lea.l $279a82.l, a0
pub fn i_0381fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38204;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038204: jsr $28cd4.l
pub fn i_038204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3820A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3820A);
    cpu.pc = 0x28CD4;
    16
}
/// $03820A: bra.w $382e2
pub fn i_03820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3820E;
    cpu.ir = 0x6000;
    cpu.pc = 0x382E2;
    10
}
// ===== Player_CrouchIdle_03827a  ($03827A, 434 B, player/air_death_crouch_036632.s)
/// $03827A: bclr.b #$2, $8c(a6)
pub fn i_03827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38280;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038280: bclr.b #$1, $8c(a6)
pub fn i_038280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38286;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038286: bclr.b #$3, $8c(a6)
pub fn i_038286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3828C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03828C: jsr $2abcc.l
pub fn i_03828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38292);
    cpu.pc = 0x2ABCC;
    16
}
/// $038292: bcs.w $382be
pub fn i_038292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38296;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x382BE; return 10; }
    8
}
/// $038296: move.b #$30, $70(a6)
pub fn i_038296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3829C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03829C: lea.l $27983c.l, a0
pub fn i_03829c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27983C;
    20
}
/// $0382A2: move.l -$4(a0), $74(a6)
pub fn i_0382a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382A8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0382A8: move.b #$ff, $21(a6)
pub fn i_0382a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0382AE: lea.l $27983c.l, a0
pub fn i_0382ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27983C;
    20
}
/// $0382B4: jsr $28cd4.l
pub fn i_0382b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x382BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0382BA: bra.w $382e2
pub fn i_0382ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x382E2;
    10
}
/// $0382BE: move.b #$30, $70(a6)
pub fn i_0382be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0382C4: lea.l $279a82.l, a0
pub fn i_0382c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $0382CA: move.l -$4(a0), $74(a6)
pub fn i_0382ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382D0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0382D0: move.b #$ff, $21(a6)
pub fn i_0382d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0382D6: lea.l $279a82.l, a0
pub fn i_0382d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $0382DC: jsr $28cd4.l
pub fn i_0382dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x382E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0382E2: move.l #$32598, $60(a6)
pub fn i_0382e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382EA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0382EA: jsr $267e6.l
pub fn i_0382ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x382F0);
    cpu.pc = 0x267E6;
    16
}
/// $0382F0: clr.w $28(a6)
pub fn i_0382f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382F4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0382F4: clr.w $2c(a6)
pub fn i_0382f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382F8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0382F8: lea.l $32734(pc), a0
pub fn i_0382f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382FC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $0382FC: move.l a0, $48(a6)
pub fn i_0382fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38300;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038300: lea.l $38306(pc), a1
pub fn i_038300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38304;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38306;
    16
}
/// $038304: move.l a1, (a6)
pub fn i_038304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38306;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038306: bset.b #$6, $8d(a6)
pub fn i_038306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3830C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03830C: jsr $32ff2(pc)
pub fn i_03830c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38310;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38310);
    cpu.pc = 0x32FF2;
    16
}
/// $038310: jsr $32d32(pc)
pub fn i_038310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38314;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38314);
    cpu.pc = 0x32D32;
    16
}
/// $038314: bcc.w $3831e
pub fn i_038314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38318;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3831E; return 10; }
    8
}
/// $038318: lea.l $38044(pc), a1
pub fn i_038318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3831C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38044;
    16
}
/// $03831C: move.l a1, (a6)
pub fn i_03831c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3831E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03831E: jsr $27a92.l
pub fn i_03831e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38324);
    cpu.pc = 0x27A92;
    16
}
/// $038324: jsr $32aa8(pc)
pub fn i_038324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38328;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38328);
    cpu.pc = 0x32AA8;
    16
}
/// $038328: btst.b #$0, $3a(a6)
pub fn i_038328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3832E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03832E: bne.w $38352
pub fn i_03832e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38332;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38352; return 10; }
    8
}
/// $038332: jsr $32e3c(pc)
pub fn i_038332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38336;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38336);
    cpu.pc = 0x32E3C;
    16
}
/// $038336: bcc.w $38340
pub fn i_038336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3833A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38340; return 10; }
    8
}
/// $03833A: lea.l $3842c(pc), a1
pub fn i_03833a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3833E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3842C;
    16
}
/// $03833E: move.l a1, (a6)
pub fn i_03833e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38340;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038340: jsr $32e42(pc)
pub fn i_038340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38344;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38344);
    cpu.pc = 0x32E42;
    16
}
/// $038344: bcc.w $3834e
pub fn i_038344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38348;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3834E; return 10; }
    8
}
/// $038348: lea.l $386e2(pc), a1
pub fn i_038348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3834C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x386E2;
    16
}
/// $03834C: move.l a1, (a6)
pub fn i_03834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3834E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03834E: bra.w $3836e
pub fn i_03834e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38352;
    cpu.ir = 0x6000;
    cpu.pc = 0x3836E;
    10
}
/// $038352: jsr $32e3c(pc)
pub fn i_038352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38356;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38356);
    cpu.pc = 0x32E3C;
    16
}
/// $038356: bcc.w $38360
pub fn i_038356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3835A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38360; return 10; }
    8
}
/// $03835A: lea.l $38688(pc), a1
pub fn i_03835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3835E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38688;
    16
}
/// $03835E: move.l a1, (a6)
pub fn i_03835e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38360;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038360: jsr $32e42(pc)
pub fn i_038360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38364;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38364);
    cpu.pc = 0x32E42;
    16
}
/// $038364: bcc.w $3836e
pub fn i_038364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38368;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3836E; return 10; }
    8
}
/// $038368: lea.l $3855a(pc), a1
pub fn i_038368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3836C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3855A;
    16
}
/// $03836C: move.l a1, (a6)
pub fn i_03836c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3836E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03836E: jsr $5cef8.l
pub fn i_03836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38374);
    cpu.pc = 0x5CEF8;
    16
}
/// $038374: bcs.w $3837e
pub fn i_038374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38378;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3837E; return 10; }
    8
}
/// $038378: lea.l $37ec2(pc), a1
pub fn i_038378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3837C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $03837C: move.l a1, (a6)
pub fn i_03837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3837E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03837E: jsr $330d0(pc)
pub fn i_03837e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38382;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38382);
    cpu.pc = 0x330D0;
    16
}
/// $038382: bcc.w $383b4
pub fn i_038382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38386;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x383B4; return 10; }
    8
}
/// $038386: cmpi.b #$ff, d1
pub fn i_038386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3838A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03838A: bne.w $38398
pub fn i_03838a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3838E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38398; return 10; }
    8
}
/// $03838E: lea.l $38a28(pc), a1
pub fn i_03838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38392;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $038392: move.l a1, (a6)
pub fn i_038392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38394;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038394: bra.w $383b4
pub fn i_038394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38398;
    cpu.ir = 0x6000;
    cpu.pc = 0x383B4;
    10
}
/// $038398: cmpi.b #$3, d1
pub fn i_038398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3839C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03839C: bne.w $383aa
pub fn i_03839c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x383AA; return 10; }
    8
}
/// $0383A0: lea.l $388f0(pc), a1
pub fn i_0383a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $0383A4: move.l a1, (a6)
pub fn i_0383a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383A6: bra.w $383b4
pub fn i_0383a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x383B4;
    10
}
/// $0383AA: lea.l $3873c(pc), a1
pub fn i_0383aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $0383AE: move.l a1, (a6)
pub fn i_0383ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383B0: bra.w $383b4
pub fn i_0383b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x383B4;
    10
}
/// $0383B4: cmpi.w #$0, $82(a6)
pub fn i_0383b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383BA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0383BA: bne.w $383ce
pub fn i_0383ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x383CE; return 10; }
    8
}
/// $0383BE: cmpi.b #$0, $71(a6)
pub fn i_0383be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0383C4: beq.w $383ce
pub fn i_0383c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x383CE; return 10; }
    8
}
/// $0383C8: lea.l $38086(pc), a1
pub fn i_0383c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38086;
    16
}
/// $0383CC: move.l a1, (a6)
pub fn i_0383cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383CE: jsr $27eba.l
pub fn i_0383ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x383D4);
    cpu.pc = 0x27EBA;
    16
}
/// $0383D4: bcc.w $383de
pub fn i_0383d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x383DE; return 10; }
    8
}
/// $0383D8: lea.l $36fc2(pc), a1
pub fn i_0383d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $0383DC: move.l a1, (a6)
pub fn i_0383dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383DE: jsr $33034(pc)
pub fn i_0383de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x383E2);
    cpu.pc = 0x33034;
    16
}
/// $0383E2: bcc.w $383ec
pub fn i_0383e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x383EC; return 10; }
    8
}
/// $0383E6: lea.l $36914(pc), a1
pub fn i_0383e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $0383EA: move.l a1, (a6)
pub fn i_0383ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383EC: jsr $33522(pc)
pub fn i_0383ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x383F0);
    cpu.pc = 0x33522;
    16
}
/// $0383F0: cmpi.b #$3, $106ece.l
pub fn i_0383f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383F8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0383F8: beq.w $38410
pub fn i_0383f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38410; return 10; }
    8
}
/// $0383FC: movea.l #$ffffffff, a0
pub fn i_0383fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38402;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $038402: lea.l $324c6(pc), a0
pub fn i_038402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38406;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $038406: jsr $5dd56.l
pub fn i_038406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3840C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3840C);
    cpu.pc = 0x5DD56;
    16
}
/// $03840C: bra.w $38420
pub fn i_03840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38410;
    cpu.ir = 0x6000;
    cpu.pc = 0x38420;
    10
}
/// $038410: movea.l #$ffffffff, a0
pub fn i_038410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38416;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $038416: lea.l $324bc(pc), a0
pub fn i_038416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3841A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $03841A: jsr $5dd56.l
pub fn i_03841a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38420);
    cpu.pc = 0x5DD56;
    16
}
/// $038420: bcc.w $3842a
pub fn i_038420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3842A; return 10; }
    8
}
/// $038424: lea.l $37b8e(pc), a1
pub fn i_038424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38428;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $038428: move.l a1, (a6)
pub fn i_038428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3842A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03842A: rts
pub fn i_03842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3842C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchShoot_03873c  ($03873C, 436 B, player/air_death_crouch_036632.s)
/// $03873C: bset.b #$1, $8c(a6)
pub fn i_03873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38742;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038742: bclr.b #$1, $8c(a6)
pub fn i_038742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38748;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038748: move.l #$32598, $60(a6)
pub fn i_038748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38750;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038750: jsr $267e6.l
pub fn i_038750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38756;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38756);
    cpu.pc = 0x267E6;
    16
}
/// $038756: clr.w $28(a6)
pub fn i_038756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3875A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03875A: clr.w $2c(a6)
pub fn i_03875a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3875E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03875E: jsr $2abcc.l
pub fn i_03875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38764);
    cpu.pc = 0x2ABCC;
    16
}
/// $038764: bcs.w $38790
pub fn i_038764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38768;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38790; return 10; }
    8
}
/// $038768: move.b #$30, $70(a6)
pub fn i_038768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3876E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03876E: lea.l $279d06.l, a0
pub fn i_03876e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38774;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $038774: move.l -$4(a0), $74(a6)
pub fn i_038774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3877A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03877A: move.b #$ff, $21(a6)
pub fn i_03877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38780;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038780: lea.l $279d06.l, a0
pub fn i_038780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38786;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $038786: jsr $28cd4.l
pub fn i_038786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3878C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3878C);
    cpu.pc = 0x28CD4;
    16
}
/// $03878C: bra.w $387b4
pub fn i_03878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38790;
    cpu.ir = 0x6000;
    cpu.pc = 0x387B4;
    10
}
/// $038790: move.b #$30, $70(a6)
pub fn i_038790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38796;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038796: lea.l $279d06.l, a0
pub fn i_038796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3879C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $03879C: move.l -$4(a0), $74(a6)
pub fn i_03879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387A2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0387A2: move.b #$ff, $21(a6)
pub fn i_0387a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387A8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0387A8: lea.l $279d06.l, a0
pub fn i_0387a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $0387AE: jsr $28cd4.l
pub fn i_0387ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x387B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0387B4: lea.l $32734(pc), a0
pub fn i_0387b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $0387B8: move.l a0, $48(a6)
pub fn i_0387b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387BC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0387BC: lea.l $387c2(pc), a1
pub fn i_0387bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x387C2;
    16
}
/// $0387C0: move.l a1, (a6)
pub fn i_0387c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0387C2: bset.b #$6, $8d(a6)
pub fn i_0387c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387C8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0387C8: jsr $32ff2(pc)
pub fn i_0387c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387CC);
    cpu.pc = 0x32FF2;
    16
}
/// $0387CC: jsr $32d32(pc)
pub fn i_0387cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387D0);
    cpu.pc = 0x32D32;
    16
}
/// $0387D0: jsr $27a92.l
pub fn i_0387d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x387D6);
    cpu.pc = 0x27A92;
    16
}
/// $0387D6: jsr $32aa8(pc)
pub fn i_0387d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387DA);
    cpu.pc = 0x32AA8;
    16
}
/// $0387DA: bcc.w $387e4
pub fn i_0387da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x387E4; return 10; }
    8
}
/// $0387DE: lea.l $381a2(pc), a1
pub fn i_0387de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x381A2;
    16
}
/// $0387E2: move.l a1, (a6)
pub fn i_0387e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0387E4: btst.b #$2, $8c(a6)
pub fn i_0387e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0387EA: bne.w $38882
pub fn i_0387ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38882; return 10; }
    8
}
/// $0387EE: btst.b #$0, $3a(a6)
pub fn i_0387ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387F4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0387F4: bne.w $3880a
pub fn i_0387f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3880A; return 10; }
    8
}
/// $0387F8: jsr $32e42(pc)
pub fn i_0387f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387FC);
    cpu.pc = 0x32E42;
    16
}
/// $0387FC: bcc.w $38806
pub fn i_0387fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38800;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38806; return 10; }
    8
}
/// $038800: lea.l $386e2(pc), a1
pub fn i_038800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38804;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x386E2;
    16
}
/// $038804: move.l a1, (a6)
pub fn i_038804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38806;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038806: bra.w $38818
pub fn i_038806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3880A;
    cpu.ir = 0x6000;
    cpu.pc = 0x38818;
    10
}
/// $03880A: jsr $32e3c(pc)
pub fn i_03880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3880E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3880E);
    cpu.pc = 0x32E3C;
    16
}
/// $03880E: bcc.w $38818
pub fn i_03880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38812;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38818; return 10; }
    8
}
/// $038812: lea.l $38688(pc), a1
pub fn i_038812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38688;
    16
}
/// $038816: move.l a1, (a6)
pub fn i_038816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38818;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038818: jsr $330d0(pc)
pub fn i_038818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3881C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3881C);
    cpu.pc = 0x330D0;
    16
}
/// $03881C: bcc.w $3884e
pub fn i_03881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38820;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3884E; return 10; }
    8
}
/// $038820: cmpi.b #$ff, d1
pub fn i_038820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38824;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $038824: bne.w $38832
pub fn i_038824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38828;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38832; return 10; }
    8
}
/// $038828: lea.l $38a28(pc), a1
pub fn i_038828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3882C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $03882C: move.l a1, (a6)
pub fn i_03882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3882E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03882E: bra.w $3884e
pub fn i_03882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38832;
    cpu.ir = 0x6000;
    cpu.pc = 0x3884E;
    10
}
/// $038832: cmpi.b #$3, d1
pub fn i_038832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38836;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $038836: bne.w $38844
pub fn i_038836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3883A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38844; return 10; }
    8
}
/// $03883A: lea.l $388f0(pc), a1
pub fn i_03883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3883E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $03883E: move.l a1, (a6)
pub fn i_03883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38840;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038840: bra.w $3884e
pub fn i_038840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38844;
    cpu.ir = 0x6000;
    cpu.pc = 0x3884E;
    10
}
/// $038844: lea.l $3873c(pc), a1
pub fn i_038844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38848;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $038848: move.l a1, (a6)
pub fn i_038848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3884A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03884A: bra.w $3884e
pub fn i_03884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3884E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3884E;
    10
}
/// $03884E: cmpi.b #$0, $85(a6)
pub fn i_03884e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38854;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $038854: bne.w $38868
pub fn i_038854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38858;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38868; return 10; }
    8
}
/// $038858: cmpi.b #$1, $71(a6)
pub fn i_038858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3885E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03885E: bne.w $38868
pub fn i_03885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38862;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38868; return 10; }
    8
}
/// $038862: lea.l $38ae6(pc), a1
pub fn i_038862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38AE6;
    16
}
/// $038866: move.l a1, (a6)
pub fn i_038866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038868: cmpi.w #$0, $82(a6)
pub fn i_038868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3886E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03886E: bne.w $38882
pub fn i_03886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38872;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38882; return 10; }
    8
}
/// $038872: cmpi.b #$0, $71(a6)
pub fn i_038872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38878;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $038878: beq.w $38882
pub fn i_038878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3887C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38882; return 10; }
    8
}
/// $03887C: lea.l $38086(pc), a1
pub fn i_03887c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38880;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38086;
    16
}
/// $038880: move.l a1, (a6)
pub fn i_038880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38882;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038882: jsr $5cef8.l
pub fn i_038882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38888);
    cpu.pc = 0x5CEF8;
    16
}
/// $038888: bcs.w $38892
pub fn i_038888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3888C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38892; return 10; }
    8
}
/// $03888C: lea.l $37ec2(pc), a1
pub fn i_03888c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38890;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $038890: move.l a1, (a6)
pub fn i_038890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38892;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038892: jsr $33034(pc)
pub fn i_038892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38896;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38896);
    cpu.pc = 0x33034;
    16
}
/// $038896: bcc.w $388a0
pub fn i_038896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3889A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x388A0; return 10; }
    8
}
/// $03889A: lea.l $36914(pc), a1
pub fn i_03889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3889E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $03889E: move.l a1, (a6)
pub fn i_03889e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0388A0: jsr $27eba.l
pub fn i_0388a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x388A6);
    cpu.pc = 0x27EBA;
    16
}
/// $0388A6: bcc.w $388b0
pub fn i_0388a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x388B0; return 10; }
    8
}
/// $0388AA: lea.l $36fc2(pc), a1
pub fn i_0388aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $0388AE: move.l a1, (a6)
pub fn i_0388ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0388B0: jsr $33522(pc)
pub fn i_0388b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x388B4);
    cpu.pc = 0x33522;
    16
}
/// $0388B4: cmpi.b #$3, $106ece.l
pub fn i_0388b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388BC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0388BC: beq.w $388d4
pub fn i_0388bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x388D4; return 10; }
    8
}
/// $0388C0: movea.l #$ffffffff, a0
pub fn i_0388c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0388C6: lea.l $324c6(pc), a0
pub fn i_0388c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388CA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $0388CA: jsr $5dd56.l
pub fn i_0388ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x388D0);
    cpu.pc = 0x5DD56;
    16
}
/// $0388D0: bra.w $388e4
pub fn i_0388d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x388E4;
    10
}
/// $0388D4: movea.l #$ffffffff, a0
pub fn i_0388d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388DA;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0388DA: lea.l $324bc(pc), a0
pub fn i_0388da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388DE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $0388DE: jsr $5dd56.l
pub fn i_0388de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x388E4);
    cpu.pc = 0x5DD56;
    16
}
/// $0388E4: bcc.w $388ee
pub fn i_0388e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x388EE; return 10; }
    8
}
/// $0388E8: lea.l $37b8e(pc), a1
pub fn i_0388e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $0388EC: move.l a1, (a6)
pub fn i_0388ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0388EE: rts
pub fn i_0388ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_Spawn_038cf6  ($038CF6, 18 B, player/arm_weapon_fx_0388f0.s)
/// $038CF6: move.w #$18d, d1
pub fn i_038cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038CFA: jsr $236e.l
pub fn i_038cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D00);
    cpu.pc = 0x236E;
    16
}
/// $038D00: lea.l $ffff.w, a0
pub fn i_038d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D04;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $038D04: move.l a0, $48(a6)
pub fn i_038d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D08;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Parachute_Open_038d08  ($038D08, 128 B, player/arm_weapon_fx_0388f0.s)
/// $038D08: lea.l $38d0e(pc), a1
pub fn i_038d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38D0E;
    16
}
/// $038D0C: move.l a1, (a6)
pub fn i_038d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D0E: jsr $5e4ee.l
pub fn i_038d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D14);
    cpu.pc = 0x5E4EE;
    16
}
/// $038D14: subi.w #$20, $38(a6)
pub fn i_038d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D1A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $038D1A: clr.l d0
pub fn i_038d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D1C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $038D1C: jsr $38e9a(pc)
pub fn i_038d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38D20);
    cpu.pc = 0x38E9A;
    16
}
/// $038D20: movea.l #$28f8d4, a0
pub fn i_038d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D26;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28F8D4;
    cpu.a[0] = v1;
    4
}
/// $038D26: lsl.w #$2, d0
pub fn i_038d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D28;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $038D28: movea.l (a0, d0.w), a0
pub fn i_038d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D2C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $038D2C: cmpa.l #$ffffffff, a0
pub fn i_038d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D32;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $038D32: beq.w $38d3c
pub fn i_038d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D36;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D3C; return 10; }
    8
}
/// $038D36: jsr $28cd4.l
pub fn i_038d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D3C);
    cpu.pc = 0x28CD4;
    16
}
/// $038D3C: jsr $28d70.l
pub fn i_038d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D42);
    cpu.pc = 0x28D70;
    16
}
/// $038D42: jsr $28758.l
pub fn i_038d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D48);
    cpu.pc = 0x28758;
    16
}
/// $038D48: bcc.w $38d52
pub fn i_038d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38D52; return 10; }
    8
}
/// $038D4C: lea.l $38e4a(pc), a1
pub fn i_038d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E4A;
    16
}
/// $038D50: move.l a1, (a6)
pub fn i_038d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D52: movea.l $c(a6), a0
pub fn i_038d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D56;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038D56: btst.b #$2, $8c(a0)
pub fn i_038d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D5C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $038D5C: beq.w $38d66
pub fn i_038d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D66; return 10; }
    8
}
/// $038D60: lea.l $38d88(pc), a1
pub fn i_038d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38D88;
    16
}
/// $038D64: move.l a1, (a6)
pub fn i_038d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D66: cmpi.b #$4, $70(a0)
pub fn i_038d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D6C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $038D6C: beq.w $38d76
pub fn i_038d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D76; return 10; }
    8
}
/// $038D70: lea.l $38e10(pc), a1
pub fn i_038d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E10;
    16
}
/// $038D74: move.l a1, (a6)
pub fn i_038d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D76: btst.b #$0, $13(a0)
pub fn i_038d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D7C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $038D7C: beq.w $38d86
pub fn i_038d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D86; return 10; }
    8
}
/// $038D80: lea.l $38e4a(pc), a1
pub fn i_038d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E4A;
    16
}
/// $038D84: move.l a1, (a6)
pub fn i_038d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D86: rts
pub fn i_038d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_Release_038e10  ($038E10, 58 B, player/arm_weapon_fx_0388f0.s)
/// $038E10: movea.l $c(a6), a0
pub fn i_038e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E14;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038E14: move.b $3a(a0), d0
pub fn i_038e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E18;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $038E18: eori.b #$1, d0
pub fn i_038e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E1C;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $038E1C: move.b d0, $3a(a6)
pub fn i_038e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E20;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038E20: lea.l $28fe2c.l, a0
pub fn i_038e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FE2C;
    20
}
/// $038E26: jsr $28cd4.l
pub fn i_038e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E2C);
    cpu.pc = 0x28CD4;
    16
}
/// $038E2C: lea.l $38e32(pc), a1
pub fn i_038e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E32;
    16
}
/// $038E30: move.l a1, (a6)
pub fn i_038e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038E32: jsr $5e4ee.l
pub fn i_038e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E38);
    cpu.pc = 0x5E4EE;
    16
}
/// $038E38: jsr $28d70.l
pub fn i_038e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E3E);
    cpu.pc = 0x28D70;
    16
}
/// $038E3E: bcc.w $38e48
pub fn i_038e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38E48; return 10; }
    8
}
/// $038E42: jmp $518.l
pub fn i_038e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $038E48: rts
pub fn i_038e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_SwingIndexFromVelX_038e9a  ($038E9A, 46 B, player/arm_weapon_fx_0388f0.s)
/// $038E9A: movea.l $c(a6), a0
pub fn i_038e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038E9E: move.w $28(a0), d1
pub fn i_038e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EA2;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $038EA2: addi.w #$400, d1
pub fn i_038ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EA6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x400, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $038EA6: move.w #$400, d0
pub fn i_038ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EAA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038EAA: addi.w #$400, d0
pub fn i_038eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EAE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x400, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $038EAE: move.w d0, d2
pub fn i_038eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB0;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038EB0: asr.w #$5, d2
pub fn i_038eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB2;
    cpu.ir = 0xEA42;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $038EB2: add.w d2, d1
pub fn i_038eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB4;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $038EB4: add.w d2, d0
pub fn i_038eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB6;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $038EB6: sub.w d1, d0
pub fn i_038eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB8;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $038EB8: lsr.w #$8, d0
pub fn i_038eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EBA;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $038EBA: cmpi.w #$a, d0
pub fn i_038eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EBE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $038EBE: bcs.w $38ec6
pub fn i_038ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EC2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38EC6; return 10; }
    8
}
/// $038EC2: andi.w #$7, d0
pub fn i_038ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EC6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $038EC6: rts
pub fn i_038ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerDeathFx_Splash_039148  ($039148, 98 B, player/arm_weapon_fx_0388f0.s)
/// $039148: move.w #$185, d1
pub fn i_039148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3914C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x185;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03914C: jsr $236e.l
pub fn i_03914c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39152);
    cpu.pc = 0x236E;
    16
}
/// $039152: lea.l $27a182.l, a0
pub fn i_039152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39158;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27A182;
    20
}
/// $039158: jsr $28cd4.l
pub fn i_039158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3915E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3915E);
    cpu.pc = 0x28CD4;
    16
}
/// $03915E: move.w #$0, $5c(a6)
pub fn i_03915e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39164;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039164: lea.l $3916a(pc), a1
pub fn i_039164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39168;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3916A;
    16
}
/// $039168: move.l a1, (a6)
pub fn i_039168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3916A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03916A: jsr $5e4ca.l
pub fn i_03916a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39170);
    cpu.pc = 0x5E4CA;
    16
}
/// $039170: move.w d1, $22(a6)
pub fn i_039170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39174;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039174: move.w d2, $24(a6)
pub fn i_039174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39178;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039178: andi.w #$ffe3, d0
pub fn i_039178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3917C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03917C: or.w $5c(a6), d0
pub fn i_03917c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39180;
    cpu.ir = 0x806E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $039180: move.w d0, $38(a6)
pub fn i_039180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39184;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039184: jsr $28d70.l
pub fn i_039184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3918A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3918A);
    cpu.pc = 0x28D70;
    16
}
/// $03918A: bcc.w $39194
pub fn i_03918a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3918E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x39194; return 10; }
    8
}
/// $03918E: lea.l $391aa(pc), a1
pub fn i_03918e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39192;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391AA;
    16
}
/// $039192: move.l a1, (a6)
pub fn i_039192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39194;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $039194: movea.l $c(a6), a0
pub fn i_039194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39198;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039198: btst.b #$1, $8d(a0)
pub fn i_039198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3919E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03919E: bne.w $391a8
pub fn i_03919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x391A8; return 10; }
    8
}
/// $0391A2: lea.l $391aa(pc), a1
pub fn i_0391a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391AA;
    16
}
/// $0391A6: move.l a1, (a6)
pub fn i_0391a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0391A8: rts
pub fn i_0391a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerDeathFx_Loop_0391aa  ($0391AA, 68 B, player/arm_weapon_fx_0388f0.s)
/// $0391AA: lea.l $27a228.l, a0
pub fn i_0391aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27A228;
    20
}
/// $0391B0: jsr $28cd4.l
pub fn i_0391b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0391B6: move.w #$0, $5c(a6)
pub fn i_0391b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391BC: lea.l $391c2(pc), a1
pub fn i_0391bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391C2;
    16
}
/// $0391C0: move.l a1, (a6)
pub fn i_0391c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0391C2: jsr $5e4ca.l
pub fn i_0391c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391C8);
    cpu.pc = 0x5E4CA;
    16
}
/// $0391C8: move.w d1, $22(a6)
pub fn i_0391c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391CC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391CC: move.w d2, $24(a6)
pub fn i_0391cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391D0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391D0: andi.w #$ffe3, d0
pub fn i_0391d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391D4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0391D4: or.w $5c(a6), d0
pub fn i_0391d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391D8;
    cpu.ir = 0x806E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0391D8: move.w d0, $38(a6)
pub fn i_0391d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391DC: jsr $28d70.l
pub fn i_0391dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391E2);
    cpu.pc = 0x28D70;
    16
}
/// $0391E2: bcc.w $391ec
pub fn i_0391e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x391EC; return 10; }
    8
}
/// $0391E6: jmp $518.l
pub fn i_0391e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0391EC: rts
pub fn i_0391ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerDeathFx_Ripple_0391ee  ($0391EE, 38 B, player/arm_weapon_fx_0388f0.s)
/// $0391EE: move.w #$185, d1
pub fn i_0391ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x185;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0391F2: jsr $236e.l
pub fn i_0391f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391F8);
    cpu.pc = 0x236E;
    16
}
/// $0391F8: lea.l $27a0fe.l, a0
pub fn i_0391f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27A0FE;
    20
}
/// $0391FE: jsr $28cd4.l
pub fn i_0391fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39204);
    cpu.pc = 0x28CD4;
    16
}
/// $039204: move.w #$1c, $5c(a6)
pub fn i_039204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3920A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03920A: lea.l $39210(pc), a1
pub fn i_03920a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3920E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39210;
    16
}
/// $03920E: move.l a1, (a6)
pub fn i_03920e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39210;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $039210: jmp $391c2(pc)
pub fn i_039210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39214;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x391C2;
    8
}
// ===== PlayerArm_ResetState_0392f8  ($0392F8, 66 B, player/arm_weapon_fx_0388f0.s)
/// $0392F8: move.w $16(a6), $14(a6)
pub fn i_0392f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x392FE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0392FE: bset.b #$6, $6b(a6)
pub fn i_0392fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39304;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039304: move.b #$ff, $32(a6)
pub fn i_039304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3930A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03930A: move.b #$ff, $33(a6)
pub fn i_03930a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39310;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $039310: jsr $5e98a.l
pub fn i_039310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39316);
    cpu.pc = 0x5E98A;
    16
}
/// $039316: clr.b $87(a1)
pub fn i_039316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3931A;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03931A: clr.b $8c(a6)
pub fn i_03931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3931E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03931E: clr.w $72(a6)
pub fn i_03931e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39322;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $039322: clr.w $36(a6)
pub fn i_039322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39326;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $039326: clr.b $91(a6)
pub fn i_039326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3932A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03932A: bclr.b #$3, $13(a6)
pub fn i_03932a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39330;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039330: lea.l $ffff.w, a0
pub fn i_039330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39334;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $039334: move.l a0, $48(a6)
pub fn i_039334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39338;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039338: rts
pub fn i_039338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3933A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WeaponTableIndex_03933a  ($03933A, 26 B, player/arm_weapon_fx_0388f0.s)
/// $03933A: movea.l $c(a6), a0
pub fn i_03933a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3933E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03933E: moveq #$0, d0
pub fn i_03933e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39340;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $039340: move.b $71(a0), d0
pub fn i_039340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39344;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $039344: cmpa.l #$1004e0, a0
pub fn i_039344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3934A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03934A: bne.w $39350
pub fn i_03934a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3934E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39350; return 10; }
    8
}
/// $03934E: addq.w #$5, d0
pub fn i_03934e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39350;
    cpu.ir = 0x5A40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 5, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $039350: lsl.w #$2, d0
pub fn i_039350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39352;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $039352: rts
pub fn i_039352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39354;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_AnimStepToParentDeath_039354  ($039354, 40 B, player/arm_weapon_fx_0388f0.s)
/// $039354: jsr $28d70.l
pub fn i_039354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3935A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3935A);
    cpu.pc = 0x28D70;
    16
}
/// $03935A: bcc.w $3936c
pub fn i_03935a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3935E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3936C; return 10; }
    8
}
/// $03935E: movea.l $c(a6), a0
pub fn i_03935e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39362;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039362: clr.b $21(a0)
pub fn i_039362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39366;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $039366: ori.b #$11, ccr
pub fn i_039366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3936A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03936A: rts
pub fn i_03936a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3936C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03936C: movea.l $c(a6), a0
pub fn i_03936c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39370;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039370: move.b #$ff, $21(a0)
pub fn i_039370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39376;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $039376: andi.b #$ee, ccr
pub fn i_039376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3937A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03937A: rts
pub fn i_03937a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3937C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FollowParent_03937c  ($03937C, 82 B, player/arm_weapon_fx_0388f0.s)
/// $03937C: jsr $5e4b2.l
pub fn i_03937c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39382);
    cpu.pc = 0x5E4B2;
    16
}
/// $039382: move.w $7c(a0), d1
pub fn i_039382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39386;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039386: move.w $7e(a0), d2
pub fn i_039386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3938A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03938A: move.b $3a(a0), d3
pub fn i_03938a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3938E;
    cpu.ir = 0x1628;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03938E: btst.b #$0, d3
pub fn i_03938e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39392;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $039392: bne.w $3939e
pub fn i_039392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39396;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3939E; return 10; }
    8
}
/// $039396: add.w d1, $22(a6)
pub fn i_039396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3939A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03939A: bra.w $393a2
pub fn i_03939a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3939E;
    cpu.ir = 0x6000;
    cpu.pc = 0x393A2;
    10
}
/// $03939E: sub.w d1, $22(a6)
pub fn i_03939e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393A2;
    cpu.ir = 0x936E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0393A2: add.w d2, $24(a6)
pub fn i_0393a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393A6;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0393A6: move.b d3, $3a(a6)
pub fn i_0393a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393AA;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0393AA: btst.b #$7, $5a(a0)
pub fn i_0393aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393B0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0393B0: beq.w $393ba
pub fn i_0393b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x393BA; return 10; }
    8
}
/// $0393B4: bset.b #$7, $5a(a6)
pub fn i_0393b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393BA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0393BA: andi.w #$fffc, $38(a6)
pub fn i_0393ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393C0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0393C0: move.b $59(a0), $59(a6)
pub fn i_0393c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393C6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0393C6: jsr $32d00.l
pub fn i_0393c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x393CC);
    cpu.pc = 0x32D00;
    16
}
/// $0393CC: rts
pub fn i_0393cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CopyBit2ToParent_039416  ($039416, 24 B, player/arm_weapon_fx_0388f0.s)
/// $039416: movea.l $c(a6), a0
pub fn i_039416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3941A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03941A: move.b $8c(a6), d0
pub fn i_03941a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3941E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03941E: andi.b #$4, d0
pub fn i_03941e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39422;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $039422: andi.b #$fb, $8c(a0)
pub fn i_039422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39428;
    cpu.ir = 0x228;
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFB) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039428: or.b d0, $8c(a0)
pub fn i_039428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3942C;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03942C: rts
pub fn i_03942c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3942E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AnimCb_AttackProbeMusic_03942e  ($03942E, 34 B, player/arm_weapon_fx_0388f0.s)
/// $03942E: jsr $283ca.l
pub fn i_03942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39434);
    cpu.pc = 0x283CA;
    16
}
/// $039434: jsr $283d8.l
pub fn i_039434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3943A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3943A);
    cpu.pc = 0x283D8;
    16
}
/// $03943A: btst.b #$2, $5a(a6)
pub fn i_03943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39440;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $039440: beq.w $3944e
pub fn i_039440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39444;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3944E; return 10; }
    8
}
/// $039444: move.w #$10fb, d0
pub fn i_039444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39448;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $039448: jsr $2352.l
pub fn i_039448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3944E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3944E);
    cpu.pc = 0x2352;
    16
}
/// $03944E: rts
pub fn i_03944e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AnimCb_RandField47_039450  ($039450, 16 B, player/arm_weapon_fx_0388f0.s)
/// $039450: jsr $5e9b6.l
pub fn i_039450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39456;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39456);
    cpu.pc = 0x5E9B6;
    16
}
/// $039456: andi.b #$a, d0
pub fn i_039456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3945A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03945A: move.b d0, $47(a6)
pub fn i_03945a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3945E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03945E: rts
pub fn i_03945e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39460;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Spawn_0394a8  ($0394A8, 1024 B, player/arm_weapon_fx_0388f0.s)
/// $0394A8: move.w #$176, d1
pub fn i_0394a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394AC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x176;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394AC: jsr $236e.l
pub fn i_0394ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394B2);
    cpu.pc = 0x236E;
    16
}
/// $0394B2: move.w #$190, d1
pub fn i_0394b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394B6: jsr $236e.l
pub fn i_0394b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394BC);
    cpu.pc = 0x236E;
    16
}
/// $0394BC: move.w #$191, d1
pub fn i_0394bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x191;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394C0: jsr $236e.l
pub fn i_0394c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394C6);
    cpu.pc = 0x236E;
    16
}
/// $0394C6: move.w #$1b, $1c(a6)
pub fn i_0394c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0394CC: jsr $138fe.l
pub fn i_0394cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394D2);
    cpu.pc = 0x138FE;
    16
}
/// $0394D2: jsr $392f8(pc)
pub fn i_0394d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x394D6);
    cpu.pc = 0x392F8;
    16
}
/// $0394D6: bclr.b #$4, $12(a6)
pub fn i_0394d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394DC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0394DC: ori.w #$2, $38(a6)
pub fn i_0394dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394E2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0394E2: bra.w $39520
pub fn i_0394e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x39520;
    10
}
/// $039520: lea.l $27aaea.l, a0
pub fn i_039520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39526;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27AAEA;
    20
}
/// $039526: jsr $28cd4.l
pub fn i_039526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3952C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3952C);
    cpu.pc = 0x28CD4;
    16
}
/// $03952C: lea.l $39532(pc), a1
pub fn i_03952c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39530;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39532;
    16
}
/// $039530: move.l a1, (a6)
pub fn i_039530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39532;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $039532: jsr $3937c(pc)
pub fn i_039532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39536;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39536);
    cpu.pc = 0x3937C;
    16
}
/// $039536: movea.l $c(a6), a0
pub fn i_039536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3953A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03953A: move.b $70(a0), d0
pub fn i_03953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3953E;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03953E: cmpi.b #$10, d0
pub fn i_03953e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39542;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $039542: beq.w $39644
pub fn i_039542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39546;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39644; return 10; }
    8
}
/// $039546: cmpi.b #$21, d0
pub fn i_039546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3954A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $03954A: beq.w $3961c
pub fn i_03954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3954E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3961C; return 10; }
    8
}
/// $03954E: cmpi.b #$11, d0
pub fn i_03954e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39552;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $039552: beq.w $396d8
pub fn i_039552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39556;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x396D8; return 10; }
    8
}
/// $039556: cmpi.b #$22, d0
pub fn i_039556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3955A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x22, Size::Byte, false);
    8
}
/// $03955A: beq.w $396b0
pub fn i_03955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3955E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x396B0; return 10; }
    8
}
/// $03955E: cmpi.b #$12, d0
pub fn i_03955e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39562;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x12, Size::Byte, false);
    8
}
/// $039562: beq.w $3976c
pub fn i_039562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39566;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3976C; return 10; }
    8
}
/// $039566: cmpi.b #$23, d0
pub fn i_039566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3956A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x23, Size::Byte, false);
    8
}
/// $03956A: beq.w $39744
pub fn i_03956a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3956E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39744; return 10; }
    8
}
/// $03956E: cmpi.b #$25, d0
pub fn i_03956e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39572;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x25, Size::Byte, false);
    8
}
/// $039572: beq.w $3984a
pub fn i_039572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39576;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3984A; return 10; }
    8
}
/// $039576: cmpi.b #$27, d0
pub fn i_039576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3957A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x27, Size::Byte, false);
    8
}
/// $03957A: beq.w $39842
pub fn i_03957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3957E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39842; return 10; }
    8
}
/// $03957E: cmpi.b #$26, d0
pub fn i_03957e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39582;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x26, Size::Byte, false);
    8
}
/// $039582: beq.w $397e4
pub fn i_039582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39586;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x397E4; return 10; }
    8
}
/// $039586: cmpi.b #$28, d0
pub fn i_039586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3958A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x28, Size::Byte, false);
    8
}
/// $03958A: beq.w $397dc
pub fn i_03958a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3958E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x397DC; return 10; }
    8
}
/// $03958E: cmpi.b #$30, d0
pub fn i_03958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39592;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $039592: beq.w $395ee
pub fn i_039592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39596;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $039596: cmpi.b #$31, d0
pub fn i_039596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3959A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $03959A: beq.w $395ee
pub fn i_03959a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3959E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $03959E: cmpi.b #$1, d0
pub fn i_03959e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395A2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0395A2: beq.w $395ee
pub fn i_0395a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395A6: cmpi.b #$32, d0
pub fn i_0395a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395AA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x32, Size::Byte, false);
    8
}
/// $0395AA: beq.w $395ee
pub fn i_0395aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395AE: cmpi.b #$2, d0
pub fn i_0395ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395B2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $0395B2: beq.w $395ee
pub fn i_0395b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395B6: cmpi.b #$3, d0
pub fn i_0395b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395BA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0395BA: beq.w $395ee
pub fn i_0395ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395BE: cmpi.b #$20, d0
pub fn i_0395be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395C2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $0395C2: beq.w $395ee
pub fn i_0395c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395C6: cmpi.b #$4, d0
pub fn i_0395c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395CA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $0395CA: beq.w $395ee
pub fn i_0395ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395CE: cmpi.b #$0, d0
pub fn i_0395ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395D2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0395D2: beq.w $395ee
pub fn i_0395d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395D6: cmpi.b #$34, d0
pub fn i_0395d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395DA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x34, Size::Byte, false);
    8
}
/// $0395DA: bcs.w $395ea
pub fn i_0395da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395DE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x395EA; return 10; }
    8
}
/// $0395DE: nop
pub fn i_0395de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E0;
    cpu.ir = 0x4E71;
    4
}
/// $0395E0: nop
pub fn i_0395e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E2;
    cpu.ir = 0x4E71;
    4
}
/// $0395E2: cmpi.b #$34, d0
pub fn i_0395e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x34, Size::Byte, false);
    8
}
/// $0395E6: nop
pub fn i_0395e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E8;
    cpu.ir = 0x4E71;
    4
}
/// $0395E8: trap #$f
pub fn i_0395e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395EA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0395EA: bra.w $395ee
pub fn i_0395ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x395EE;
    10
}
/// $0395EE: movea.l $c(a6), a0
pub fn i_0395ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0395F2: btst.b #$5, $69(a0)
pub fn i_0395f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395F8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0395F8: beq.w $3960e
pub fn i_0395f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3960E; return 10; }
    8
}
/// $0395FC: movea.l $74(a0), a1
pub fn i_0395fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39600;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039600: jsr (a1)
pub fn i_039600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39602;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39602);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039602: movea.l $c(a6), a0
pub fn i_039602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39606;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039606: move.w $72(a0), d0
pub fn i_039606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3960A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03960A: move.w d0, $72(a6)
pub fn i_03960a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3960E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03960E: clr.w $72(a6)
pub fn i_03960e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39612;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $039612: jsr $39354(pc)
pub fn i_039612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39616;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39616);
    cpu.pc = 0x39354;
    16
}
/// $039616: jsr $39416(pc)
pub fn i_039616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3961A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3961A);
    cpu.pc = 0x39416;
    16
}
/// $03961A: rts
pub fn i_03961a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3961C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03961C: movea.l $c(a6), a0
pub fn i_03961c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39620;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039620: btst.b #$5, $69(a0)
pub fn i_039620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39626;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $039626: beq.w $39640
pub fn i_039626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3962A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39640; return 10; }
    8
}
/// $03962A: movea.l $74(a0), a1
pub fn i_03962a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3962E;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03962E: jsr (a1)
pub fn i_03962e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39630;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39630);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039630: movea.l $c(a6), a0
pub fn i_039630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39634;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039634: move.w $72(a0), d0
pub fn i_039634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39638;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039638: move.w d0, $72(a6)
pub fn i_039638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3963C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03963C: bra.w $396a6
pub fn i_03963c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39640;
    cpu.ir = 0x6000;
    cpu.pc = 0x396A6;
    10
}
/// $039640: bra.w $396a6
pub fn i_039640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39644;
    cpu.ir = 0x6000;
    cpu.pc = 0x396A6;
    10
}
/// $039644: movea.l $c(a6), a0
pub fn i_039644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39648;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039648: btst.b #$2, $8c(a6)
pub fn i_039648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3964E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03964E: beq.w $39656
pub fn i_03964e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39652;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39656; return 10; }
    8
}
/// $039652: bra.w $39676
pub fn i_039652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39656;
    cpu.ir = 0x6000;
    cpu.pc = 0x39676;
    10
}
/// $039656: btst.b #$5, $69(a0)
pub fn i_039656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3965C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03965C: beq.w $39676
pub fn i_03965c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39660;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39676; return 10; }
    8
}
/// $039660: movea.l $74(a0), a1
pub fn i_039660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39664;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039664: jsr (a1)
pub fn i_039664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39666;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39666);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039666: movea.l $c(a6), a0
pub fn i_039666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3966A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03966A: move.w $72(a0), d0
pub fn i_03966a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3966E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03966E: move.w d0, $72(a6)
pub fn i_03966e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39672;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039672: bra.w $396a6
pub fn i_039672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39676;
    cpu.ir = 0x6000;
    cpu.pc = 0x396A6;
    10
}
/// $039676: movea.l $74(a0), a1
pub fn i_039676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3967A;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03967A: cmpa.l #$39e54, a1
pub fn i_03967a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39680;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x39E54;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $039680: beq.b $3968c
pub fn i_039680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39682;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3968C; return 10; }
    8
}
/// $039682: cmpa.l #$39f56, a1
pub fn i_039682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39688;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x39F56;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $039688: bne.w $396a6
pub fn i_039688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3968C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x396A6; return 10; }
    8
}
/// $03968C: move.w $72(a0), d0
pub fn i_03968c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39690;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039690: cmp.w $72(a6), d0
pub fn i_039690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39694;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $039694: beq.w $396a6
pub fn i_039694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39698;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x396A6; return 10; }
    8
}
/// $039698: move.w d0, $72(a6)
pub fn i_039698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3969C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03969C: movea.l $74(a0), a1
pub fn i_03969c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396A0;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0396A0: jsr (a1)
pub fn i_0396a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396A2;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x396A2);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $0396A2: bra.w $396a6
pub fn i_0396a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x396A6;
    10
}
/// $0396A6: jsr $39354(pc)
pub fn i_0396a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x396AA);
    cpu.pc = 0x39354;
    16
}
/// $0396AA: jsr $39416(pc)
pub fn i_0396aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x396AE);
    cpu.pc = 0x39416;
    16
}
/// $0396AE: rts
pub fn i_0396ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0396B0: movea.l $c(a6), a0
pub fn i_0396b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396B4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0396B4: btst.b #$5, $69(a0)
pub fn i_0396b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396BA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0396BA: beq.w $396d4
pub fn i_0396ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x396D4; return 10; }
    8
}
/// $0396BE: movea.l $74(a0), a1
pub fn i_0396be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396C2;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0396C2: jsr (a1)
pub fn i_0396c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396C4;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x396C4);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $0396C4: movea.l $c(a6), a0
pub fn i_0396c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396C8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0396C8: move.w $72(a0), d0
pub fn i_0396c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396CC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0396CC: move.w d0, $72(a6)
pub fn i_0396cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396D0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0396D0: bra.w $3973a
pub fn i_0396d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3973A;
    10
}
/// $0396D4: bra.w $3973a
pub fn i_0396d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3973A;
    10
}
/// $0396D8: movea.l $c(a6), a0
pub fn i_0396d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396DC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0396DC: btst.b #$2, $8c(a6)
pub fn i_0396dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396E2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0396E2: beq.w $396ea
pub fn i_0396e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x396EA; return 10; }
    8
}
/// $0396E6: bra.w $3970a
pub fn i_0396e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3970A;
    10
}
/// $0396EA: btst.b #$5, $69(a0)
pub fn i_0396ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396F0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0396F0: beq.w $3970a
pub fn i_0396f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3970A; return 10; }
    8
}
/// $0396F4: movea.l $74(a0), a1
pub fn i_0396f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396F8;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0396F8: jsr (a1)
pub fn i_0396f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396FA;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x396FA);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $0396FA: movea.l $c(a6), a0
pub fn i_0396fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x396FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0396FE: move.w $72(a0), d0
pub fn i_0396fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39702;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039702: move.w d0, $72(a6)
pub fn i_039702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39706;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039706: bra.w $3973a
pub fn i_039706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3970A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3973A;
    10
}
/// $03970A: movea.l $74(a0), a1
pub fn i_03970a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3970E;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03970E: cmpa.l #$3a058, a1
pub fn i_03970e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39714;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x3A058;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $039714: beq.b $39720
pub fn i_039714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39716;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39720; return 10; }
    8
}
/// $039716: cmpa.l #$3a108, a1
pub fn i_039716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3971C;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x3A108;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03971C: bne.w $3973a
pub fn i_03971c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39720;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3973A; return 10; }
    8
}
/// $039720: move.w $72(a0), d0
pub fn i_039720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39724;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039724: cmp.w $72(a6), d0
pub fn i_039724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39728;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $039728: beq.w $3973a
pub fn i_039728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3972C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3973A; return 10; }
    8
}
/// $03972C: move.w d0, $72(a6)
pub fn i_03972c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39730;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039730: movea.l $74(a0), a1
pub fn i_039730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39734;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039734: jsr (a1)
pub fn i_039734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39736;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39736);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039736: bra.w $3973a
pub fn i_039736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3973A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3973A;
    10
}
/// $03973A: jsr $39354(pc)
pub fn i_03973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3973E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3973E);
    cpu.pc = 0x39354;
    16
}
/// $03973E: jsr $39416(pc)
pub fn i_03973e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39742);
    cpu.pc = 0x39416;
    16
}
/// $039742: rts
pub fn i_039742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39744;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $039744: movea.l $c(a6), a0
pub fn i_039744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39748;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039748: btst.b #$5, $69(a0)
pub fn i_039748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3974E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03974E: beq.w $39768
pub fn i_03974e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39752;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39768; return 10; }
    8
}
/// $039752: movea.l $74(a0), a1
pub fn i_039752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39756;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039756: jsr (a1)
pub fn i_039756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39758;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39758);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039758: movea.l $c(a6), a0
pub fn i_039758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3975C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03975C: move.w $72(a0), d0
pub fn i_03975c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39760;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039760: move.w d0, $72(a6)
pub fn i_039760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39764;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039764: bra.w $397d2
pub fn i_039764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39768;
    cpu.ir = 0x6000;
    cpu.pc = 0x397D2;
    10
}
/// $039768: bra.w $397d2
pub fn i_039768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3976C;
    cpu.ir = 0x6000;
    cpu.pc = 0x397D2;
    10
}
/// $03976C: movea.l $c(a6), a0
pub fn i_03976c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39770;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039770: btst.b #$2, $8c(a6)
pub fn i_039770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39776;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $039776: beq.w $3977e
pub fn i_039776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3977A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3977E; return 10; }
    8
}
/// $03977A: bra.w $397a2
pub fn i_03977a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3977E;
    cpu.ir = 0x6000;
    cpu.pc = 0x397A2;
    10
}
/// $03977E: movea.l $c(a6), a0
pub fn i_03977e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39782;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039782: btst.b #$5, $69(a0)
pub fn i_039782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39788;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $039788: beq.w $397a2
pub fn i_039788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3978C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x397A2; return 10; }
    8
}
/// $03978C: movea.l $74(a0), a1
pub fn i_03978c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39790;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039790: jsr (a1)
pub fn i_039790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39792;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39792);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039792: movea.l $c(a6), a0
pub fn i_039792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39796;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039796: move.w $72(a0), d0
pub fn i_039796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3979A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03979A: move.w d0, $72(a6)
pub fn i_03979a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3979E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03979E: bra.w $397d2
pub fn i_03979e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x397D2;
    10
}
/// $0397A2: movea.l $74(a0), a1
pub fn i_0397a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397A6;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0397A6: cmpa.l #$3a1b8, a1
pub fn i_0397a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397AC;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x3A1B8;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0397AC: beq.b $397b8
pub fn i_0397ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397AE;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x397B8; return 10; }
    8
}
/// $0397AE: cmpa.l #$3a268, a1
pub fn i_0397ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397B4;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x3A268;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0397B4: bne.w $397d2
pub fn i_0397b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x397D2; return 10; }
    8
}
/// $0397B8: move.w $72(a0), d0
pub fn i_0397b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397BC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0397BC: cmp.w $72(a6), d0
pub fn i_0397bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397C0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0397C0: beq.w $397d2
pub fn i_0397c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x397D2; return 10; }
    8
}
/// $0397C4: move.w d0, $72(a6)
pub fn i_0397c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397C8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0397C8: movea.l $74(a0), a1
pub fn i_0397c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397CC;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0397CC: jsr (a1)
pub fn i_0397cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397CE;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x397CE);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $0397CE: bra.w $397d2
pub fn i_0397ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x397D2;
    10
}
/// $0397D2: jsr $39354(pc)
pub fn i_0397d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x397D6);
    cpu.pc = 0x39354;
    16
}
/// $0397D6: jsr $39416(pc)
pub fn i_0397d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x397DA);
    cpu.pc = 0x39416;
    16
}
/// $0397DA: rts
pub fn i_0397da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0397DC: movea.l $c(a6), a0
pub fn i_0397dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0397E0: bra.w $397e4
pub fn i_0397e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x397E4;
    10
}
/// $0397E4: movea.l $c(a6), a0
pub fn i_0397e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0397E8: btst.b #$5, $69(a0)
pub fn i_0397e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397EE;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0397EE: beq.w $39808
pub fn i_0397ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39808; return 10; }
    8
}
/// $0397F2: movea.l $74(a0), a1
pub fn i_0397f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397F6;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0397F6: jsr (a1)
pub fn i_0397f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397F8;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x397F8);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $0397F8: movea.l $c(a6), a0
pub fn i_0397f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x397FC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0397FC: move.w $72(a0), d0
pub fn i_0397fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39800;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039800: move.w d0, $72(a6)
pub fn i_039800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39804;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039804: bra.w $39838
pub fn i_039804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39808;
    cpu.ir = 0x6000;
    cpu.pc = 0x39838;
    10
}
/// $039808: movea.l $74(a0), a1
pub fn i_039808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3980C;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03980C: cmpa.l #$3a656, a1
pub fn i_03980c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39812;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x3A656;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $039812: bne.w $39838
pub fn i_039812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39816;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39838; return 10; }
    8
}
/// $039816: move.w $72(a0), d0
pub fn i_039816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3981A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03981A: cmp.w $72(a6), d0
pub fn i_03981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3981E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03981E: beq.w $39838
pub fn i_03981e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39822;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39838; return 10; }
    8
}
/// $039822: movem.w d0, -(a7)
pub fn i_039822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39826;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $039826: movea.l $74(a0), a1
pub fn i_039826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3982A;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03982A: jsr (a1)
pub fn i_03982a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3982C;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x3982C);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $03982C: movem.w (a7)+, d0
pub fn i_03982c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39830;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $039830: move.w d0, $72(a6)
pub fn i_039830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39834;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039834: bra.w $39838
pub fn i_039834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39838;
    cpu.ir = 0x6000;
    cpu.pc = 0x39838;
    10
}
/// $039838: jsr $39354(pc)
pub fn i_039838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3983C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3983C);
    cpu.pc = 0x39354;
    16
}
/// $03983C: jsr $39416(pc)
pub fn i_03983c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39840;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39840);
    cpu.pc = 0x39416;
    16
}
/// $039840: rts
pub fn i_039840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39842;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $039842: movea.l $c(a6), a0
pub fn i_039842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39846;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039846: bra.w $3984a
pub fn i_039846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3984A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3984A;
    10
}
/// $03984A: movea.l $c(a6), a0
pub fn i_03984a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3984E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03984E: btst.b #$5, $69(a0)
pub fn i_03984e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39854;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $039854: beq.w $3986e
pub fn i_039854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39858;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3986E; return 10; }
    8
}
/// $039858: movea.l $74(a0), a1
pub fn i_039858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3985C;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03985C: jsr (a1)
pub fn i_03985c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3985E;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x3985E);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $03985E: movea.l $c(a6), a0
pub fn i_03985e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39862;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039862: move.w $72(a0), d0
pub fn i_039862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39866;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039866: move.w d0, $72(a6)
pub fn i_039866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3986A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03986A: bra.w $3989e
pub fn i_03986a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3986E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3989E;
    10
}
/// $03986E: movea.l $74(a0), a1
pub fn i_03986e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39872;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039872: cmpa.l #$3a60a, a1
pub fn i_039872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39878;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x3A60A;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $039878: bne.w $3989e
pub fn i_039878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3987C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3989E; return 10; }
    8
}
/// $03987C: move.w $72(a0), d0
pub fn i_03987c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39880;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039880: cmp.w $72(a6), d0
pub fn i_039880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39884;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $039884: beq.w $3989e
pub fn i_039884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39888;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3989E; return 10; }
    8
}
/// $039888: movem.w d0, -(a7)
pub fn i_039888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3988C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $03988C: movea.l $74(a0), a1
pub fn i_03988c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39890;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039890: jsr (a1)
pub fn i_039890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39892;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39892);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039892: movem.w (a7)+, d0
pub fn i_039892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39896;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $039896: move.w d0, $72(a6)
pub fn i_039896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3989A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03989A: bra.w $3989e
pub fn i_03989a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3989E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3989E;
    10
}
/// $03989E: jsr $39354(pc)
pub fn i_03989e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x398A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x398A2);
    cpu.pc = 0x39354;
    16
}
/// $0398A2: jsr $39416(pc)
pub fn i_0398a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x398A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x398A6);
    cpu.pc = 0x39416;
    16
}
/// $0398A6: rts
pub fn i_0398a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x398A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchB_0398f4  ($0398F4, 34 B, player/arm_weapon_fx_0388f0.s)
/// $0398F4: bclr.b #$2, $8c(a6)
pub fn i_0398f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x398FA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0398FA: bclr.b #$2, $8c(a6)
pub fn i_0398fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39900;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039900: jsr $3933a(pc)
pub fn i_039900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39904;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39904);
    cpu.pc = 0x3933A;
    16
}
/// $039904: lea.l $39916(pc), a0
pub fn i_039904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39908;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39916;
    16
}
/// $039908: movea.l (a0, d0.w), a0
pub fn i_039908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3990C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03990C: jsr $28cd4.l
pub fn i_03990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39912);
    cpu.pc = 0x28CD4;
    16
}
/// $039912: bra.w $3993e
pub fn i_039912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39916;
    cpu.ir = 0x6000;
    cpu.pc = 0x3993E;
    10
}
/// $03993E: rts
pub fn i_03993e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39940;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Idle_039940  ($039940, 34 B, player/arm_weapon_fx_0388f0.s)
/// $039940: bclr.b #$2, $8c(a6)
pub fn i_039940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39946;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039946: bclr.b #$2, $8c(a6)
pub fn i_039946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3994C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03994C: jsr $3933a(pc)
pub fn i_03994c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39950;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39950);
    cpu.pc = 0x3933A;
    16
}
/// $039950: lea.l $39962(pc), a0
pub fn i_039950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39954;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39962;
    16
}
/// $039954: movea.l (a0, d0.w), a0
pub fn i_039954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39958;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039958: jsr $28cd4.l
pub fn i_039958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3995E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3995E);
    cpu.pc = 0x28CD4;
    16
}
/// $03995E: bra.w $3998a
pub fn i_03995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39962;
    cpu.ir = 0x6000;
    cpu.pc = 0x3998A;
    10
}
/// $03998A: rts
pub fn i_03998a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3998C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_SpawnLandB_039a24  ($039A24, 34 B, player/arm_weapon_fx_0388f0.s)
/// $039A24: bclr.b #$2, $8c(a6)
pub fn i_039a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A2A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039A2A: bclr.b #$2, $8c(a6)
pub fn i_039a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A30;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039A30: jsr $3933a(pc)
pub fn i_039a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39A34);
    cpu.pc = 0x3933A;
    16
}
/// $039A34: lea.l $39a46(pc), a0
pub fn i_039a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A38;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39A46;
    16
}
/// $039A38: movea.l (a0, d0.w), a0
pub fn i_039a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A3C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039A3C: jsr $28cd4.l
pub fn i_039a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39A42);
    cpu.pc = 0x28CD4;
    16
}
/// $039A42: bra.w $39a6e
pub fn i_039a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A46;
    cpu.ir = 0x6000;
    cpu.pc = 0x39A6E;
    10
}
/// $039A6E: rts
pub fn i_039a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_ShootStandA_039a70  ($039A70, 28 B, player/arm_weapon_fx_0388f0.s)
/// $039A70: bclr.b #$2, $8c(a6)
pub fn i_039a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A76;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039A76: jsr $3933a(pc)
pub fn i_039a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39A7A);
    cpu.pc = 0x3933A;
    16
}
/// $039A7A: lea.l $39a8c(pc), a0
pub fn i_039a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A7E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39A8C;
    16
}
/// $039A7E: movea.l (a0, d0.w), a0
pub fn i_039a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A82;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039A82: jsr $28cd4.l
pub fn i_039a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39A88);
    cpu.pc = 0x28CD4;
    16
}
/// $039A88: bra.w $39ab4
pub fn i_039a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39A8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x39AB4;
    10
}
/// $039AB4: rts
pub fn i_039ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39AB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_ShootStandD_039b4e  ($039B4E, 28 B, player/arm_weapon_fx_0388f0.s)
/// $039B4E: bclr.b #$2, $8c(a6)
pub fn i_039b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39B54;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039B54: jsr $3933a(pc)
pub fn i_039b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39B58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39B58);
    cpu.pc = 0x3933A;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x036A54, i_036a54),
    (0x036A56, i_036a56),
    (0x036A5C, i_036a5c),
    (0x036A60, i_036a60),
    (0x036A64, i_036a64),
    (0x036A66, i_036a66),
    (0x036A6A, i_036a6a),
    (0x036A6E, i_036a6e),
    (0x036A70, i_036a70),
    (0x036A76, i_036a76),
    (0x036A7A, i_036a7a),
    (0x036A80, i_036a80),
    (0x036A82, i_036a82),
    (0x036A86, i_036a86),
    (0x036A8C, i_036a8c),
    (0x036A90, i_036a90),
    (0x036A94, i_036a94),
    (0x036A9A, i_036a9a),
    (0x036A9E, i_036a9e),
    (0x036AA4, i_036aa4),
    (0x036AA8, i_036aa8),
    (0x036AAE, i_036aae),
    (0x036AB0, i_036ab0),
    (0x036AB4, i_036ab4),
    (0x036ABA, i_036aba),
    (0x036AC0, i_036ac0),
    (0x036AC4, i_036ac4),
    (0x036ACA, i_036aca),
    (0x036ACE, i_036ace),
    (0x036AD4, i_036ad4),
    (0x036AD8, i_036ad8),
    (0x036ADE, i_036ade),
    (0x036AE2, i_036ae2),
    (0x036AE8, i_036ae8),
    (0x036AEC, i_036aec),
    (0x036AF0, i_036af0),
    (0x036AF4, i_036af4),
    (0x036AFA, i_036afa),
    (0x036AFE, i_036afe),
    (0x036B02, i_036b02),
    (0x036B06, i_036b06),
    (0x036B0A, i_036b0a),
    (0x036B10, i_036b10),
    (0x036B14, i_036b14),
    (0x036B18, i_036b18),
    (0x036B1E, i_036b1e),
    (0x036B24, i_036b24),
    (0x036B2C, i_036b2c),
    (0x036B30, i_036b30),
    (0x036B34, i_036b34),
    (0x036B38, i_036b38),
    (0x036B3C, i_036b3c),
    (0x036B40, i_036b40),
    (0x036B44, i_036b44),
    (0x036B48, i_036b48),
    (0x036B4C, i_036b4c),
    (0x036B50, i_036b50),
    (0x036B54, i_036b54),
    (0x036B5A, i_036b5a),
    (0x036B5E, i_036b5e),
    (0x036B62, i_036b62),
    (0x036B64, i_036b64),
    (0x036B6A, i_036b6a),
    (0x036B70, i_036b70),
    (0x036B74, i_036b74),
    (0x036B78, i_036b78),
    (0x036B7E, i_036b7e),
    (0x036B82, i_036b82),
    (0x036B8A, i_036b8a),
    (0x036B8E, i_036b8e),
    (0x036B92, i_036b92),
    (0x036B96, i_036b96),
    (0x036B9A, i_036b9a),
    (0x036B9E, i_036b9e),
    (0x036BA2, i_036ba2),
    (0x036BA6, i_036ba6),
    (0x036BAA, i_036baa),
    (0x036BAE, i_036bae),
    (0x036BB4, i_036bb4),
    (0x036BBA, i_036bba),
    (0x036BBE, i_036bbe),
    (0x036BC2, i_036bc2),
    (0x036BC4, i_036bc4),
    (0x036BC8, i_036bc8),
    (0x036BCE, i_036bce),
    (0x036BD2, i_036bd2),
    (0x036BD8, i_036bd8),
    (0x036BDC, i_036bdc),
    (0x036BE0, i_036be0),
    (0x036BE4, i_036be4),
    (0x036BE8, i_036be8),
    (0x036BEC, i_036bec),
    (0x036BEE, i_036bee),
    (0x036BF4, i_036bf4),
    (0x036BFA, i_036bfa),
    (0x036BFE, i_036bfe),
    (0x036C02, i_036c02),
    (0x036C06, i_036c06),
    (0x036C0A, i_036c0a),
    (0x036C10, i_036c10),
    (0x036C14, i_036c14),
    (0x036C18, i_036c18),
    (0x036C1A, i_036c1a),
    (0x036C20, i_036c20),
    (0x036C24, i_036c24),
    (0x036C2A, i_036c2a),
    (0x036C2E, i_036c2e),
    (0x036C32, i_036c32),
    (0x036C36, i_036c36),
    (0x036C3A, i_036c3a),
    (0x036C40, i_036c40),
    (0x036C44, i_036c44),
    (0x036C4C, i_036c4c),
    (0x036C50, i_036c50),
    (0x036C56, i_036c56),
    (0x036C5A, i_036c5a),
    (0x036C60, i_036c60),
    (0x036C64, i_036c64),
    (0x036C6A, i_036c6a),
    (0x036C6E, i_036c6e),
    (0x036C74, i_036c74),
    (0x036C78, i_036c78),
    (0x036C7C, i_036c7c),
    (0x036C7E, i_036c7e),
    (0x036C82, i_036c82),
    (0x036C86, i_036c86),
    (0x036C8A, i_036c8a),
    (0x036C8C, i_036c8c),
    (0x036C92, i_036c92),
    (0x036C96, i_036c96),
    (0x036C9C, i_036c9c),
    (0x036CA2, i_036ca2),
    (0x036CA6, i_036ca6),
    (0x036CAC, i_036cac),
    (0x036CB2, i_036cb2),
    (0x036CB6, i_036cb6),
    (0x036CBA, i_036cba),
    (0x036CC0, i_036cc0),
    (0x036CC6, i_036cc6),
    (0x036CCC, i_036ccc),
    (0x036CD2, i_036cd2),
    (0x036CD8, i_036cd8),
    (0x036CE0, i_036ce0),
    (0x036CE6, i_036ce6),
    (0x036CEC, i_036cec),
    (0x036CF2, i_036cf2),
    (0x036CF8, i_036cf8),
    (0x036CFE, i_036cfe),
    (0x036D04, i_036d04),
    (0x036D0A, i_036d0a),
    (0x036D0E, i_036d0e),
    (0x036D12, i_036d12),
    (0x036D16, i_036d16),
    (0x036D1A, i_036d1a),
    (0x036D1E, i_036d1e),
    (0x036D20, i_036d20),
    (0x036D26, i_036d26),
    (0x036D2C, i_036d2c),
    (0x036D32, i_036d32),
    (0x036D36, i_036d36),
    (0x036D3A, i_036d3a),
    (0x036D3C, i_036d3c),
    (0x036D40, i_036d40),
    (0x036D46, i_036d46),
    (0x036D4C, i_036d4c),
    (0x036D50, i_036d50),
    (0x036D52, i_036d52),
    (0x036D56, i_036d56),
    (0x036D5C, i_036d5c),
    (0x036D62, i_036d62),
    (0x036D64, i_036d64),
    (0x036D68, i_036d68),
    (0x036D6C, i_036d6c),
    (0x036D72, i_036d72),
    (0x036D78, i_036d78),
    (0x036D7E, i_036d7e),
    (0x036D84, i_036d84),
    (0x036D8A, i_036d8a),
    (0x036D90, i_036d90),
    (0x036D98, i_036d98),
    (0x036D9E, i_036d9e),
    (0x036DA4, i_036da4),
    (0x036DAA, i_036daa),
    (0x036DB0, i_036db0),
    (0x036DB6, i_036db6),
    (0x036DBC, i_036dbc),
    (0x036DC0, i_036dc0),
    (0x036DC4, i_036dc4),
    (0x036DC8, i_036dc8),
    (0x036DCA, i_036dca),
    (0x036DD0, i_036dd0),
    (0x036DD4, i_036dd4),
    (0x036DDA, i_036dda),
    (0x036DDC, i_036ddc),
    (0x036DE0, i_036de0),
    (0x036DE6, i_036de6),
    (0x036DEA, i_036dea),
    (0x036DEE, i_036dee),
    (0x036DF4, i_036df4),
    (0x036DF8, i_036df8),
    (0x036DFE, i_036dfe),
    (0x036E02, i_036e02),
    (0x036E08, i_036e08),
    (0x036E0A, i_036e0a),
    (0x036E0E, i_036e0e),
    (0x036E14, i_036e14),
    (0x036E1A, i_036e1a),
    (0x036E1E, i_036e1e),
    (0x036E24, i_036e24),
    (0x036E28, i_036e28),
    (0x036E2E, i_036e2e),
    (0x036E32, i_036e32),
    (0x036E38, i_036e38),
    (0x036E3C, i_036e3c),
    (0x036E42, i_036e42),
    (0x036E46, i_036e46),
    (0x036E4A, i_036e4a),
    (0x036E4E, i_036e4e),
    (0x036E54, i_036e54),
    (0x036E58, i_036e58),
    (0x036E5C, i_036e5c),
    (0x036E60, i_036e60),
    (0x036E64, i_036e64),
    (0x036E68, i_036e68),
    (0x036E6E, i_036e6e),
    (0x036E72, i_036e72),
    (0x036E78, i_036e78),
    (0x036E7E, i_036e7e),
    (0x036E82, i_036e82),
    (0x036E88, i_036e88),
    (0x036E8C, i_036e8c),
    (0x036E90, i_036e90),
    (0x036E92, i_036e92),
    (0x036E98, i_036e98),
    (0x036E9E, i_036e9e),
    (0x036EA2, i_036ea2),
    (0x036EA6, i_036ea6),
    (0x036EAC, i_036eac),
    (0x036EB2, i_036eb2),
    (0x036EB4, i_036eb4),
    (0x036EBA, i_036eba),
    (0x036EBE, i_036ebe),
    (0x036EC2, i_036ec2),
    (0x036EC6, i_036ec6),
    (0x036ECA, i_036eca),
    (0x036ECE, i_036ece),
    (0x036ED4, i_036ed4),
    (0x036EDA, i_036eda),
    (0x036EDE, i_036ede),
    (0x036EE2, i_036ee2),
    (0x036EE4, i_036ee4),
    (0x036EE8, i_036ee8),
    (0x036EEE, i_036eee),
    (0x036EF2, i_036ef2),
    (0x036EF8, i_036ef8),
    (0x036EFC, i_036efc),
    (0x036F00, i_036f00),
    (0x036F04, i_036f04),
    (0x036F08, i_036f08),
    (0x036F0C, i_036f0c),
    (0x036F0E, i_036f0e),
    (0x036F14, i_036f14),
    (0x036F1A, i_036f1a),
    (0x036F1E, i_036f1e),
    (0x036F22, i_036f22),
    (0x036F26, i_036f26),
    (0x036F2A, i_036f2a),
    (0x036F30, i_036f30),
    (0x036F34, i_036f34),
    (0x036F38, i_036f38),
    (0x036F3A, i_036f3a),
    (0x036F40, i_036f40),
    (0x036F44, i_036f44),
    (0x036F4A, i_036f4a),
    (0x036F4E, i_036f4e),
    (0x036F52, i_036f52),
    (0x036F56, i_036f56),
    (0x036F5C, i_036f5c),
    (0x036F5E, i_036f5e),
    (0x036F64, i_036f64),
    (0x036F68, i_036f68),
    (0x036F6C, i_036f6c),
    (0x036F70, i_036f70),
    (0x036F76, i_036f76),
    (0x036F7A, i_036f7a),
    (0x036F82, i_036f82),
    (0x036F86, i_036f86),
    (0x036F8C, i_036f8c),
    (0x036F90, i_036f90),
    (0x036F96, i_036f96),
    (0x036F9A, i_036f9a),
    (0x036FA0, i_036fa0),
    (0x036FA4, i_036fa4),
    (0x036FAA, i_036faa),
    (0x036FAE, i_036fae),
    (0x036FB2, i_036fb2),
    (0x036FB4, i_036fb4),
    (0x036FB8, i_036fb8),
    (0x036FBC, i_036fbc),
    (0x036FC0, i_036fc0),
    (0x03705A, i_03705a),
    (0x037060, i_037060),
    (0x037064, i_037064),
    (0x03706A, i_03706a),
    (0x03706E, i_03706e),
    (0x037074, i_037074),
    (0x037078, i_037078),
    (0x03707E, i_03707e),
    (0x037082, i_037082),
    (0x037088, i_037088),
    (0x03708E, i_03708e),
    (0x037094, i_037094),
    (0x037098, i_037098),
    (0x03709E, i_03709e),
    (0x0370A4, i_0370a4),
    (0x0370AA, i_0370aa),
    (0x0370B0, i_0370b0),
    (0x0370B6, i_0370b6),
    (0x0370BA, i_0370ba),
    (0x0370C0, i_0370c0),
    (0x0370C6, i_0370c6),
    (0x0370CC, i_0370cc),
    (0x0370D2, i_0370d2),
    (0x0370D8, i_0370d8),
    (0x0370DE, i_0370de),
    (0x0370E2, i_0370e2),
    (0x0370E8, i_0370e8),
    (0x0370EC, i_0370ec),
    (0x0370F2, i_0370f2),
    (0x0370F6, i_0370f6),
    (0x0370FC, i_0370fc),
    (0x037102, i_037102),
    (0x037106, i_037106),
    (0x03710C, i_03710c),
    (0x037112, i_037112),
    (0x037118, i_037118),
    (0x03711E, i_03711e),
    (0x037124, i_037124),
    (0x03712A, i_03712a),
    (0x03712E, i_03712e),
    (0x037134, i_037134),
    (0x037138, i_037138),
    (0x03713E, i_03713e),
    (0x037144, i_037144),
    (0x03714A, i_03714a),
    (0x037150, i_037150),
    (0x037156, i_037156),
    (0x03715C, i_03715c),
    (0x037160, i_037160),
    (0x037166, i_037166),
    (0x037168, i_037168),
    (0x03716C, i_03716c),
    (0x037170, i_037170),
    (0x037176, i_037176),
    (0x03717A, i_03717a),
    (0x03717E, i_03717e),
    (0x037184, i_037184),
    (0x03718A, i_03718a),
    (0x03718E, i_03718e),
    (0x037194, i_037194),
    (0x03719A, i_03719a),
    (0x0371A0, i_0371a0),
    (0x0371A6, i_0371a6),
    (0x0371AC, i_0371ac),
    (0x0371B0, i_0371b0),
    (0x0371B6, i_0371b6),
    (0x0371BC, i_0371bc),
    (0x0371C2, i_0371c2),
    (0x0371C8, i_0371c8),
    (0x0371CE, i_0371ce),
    (0x0371D4, i_0371d4),
    (0x0371D8, i_0371d8),
    (0x0371DE, i_0371de),
    (0x0371E2, i_0371e2),
    (0x0371E6, i_0371e6),
    (0x0371EC, i_0371ec),
    (0x0371F0, i_0371f0),
    (0x0371F6, i_0371f6),
    (0x0371FC, i_0371fc),
    (0x037202, i_037202),
    (0x037208, i_037208),
    (0x03720C, i_03720c),
    (0x037210, i_037210),
    (0x037214, i_037214),
    (0x037218, i_037218),
    (0x03721C, i_03721c),
    (0x037220, i_037220),
    (0x037226, i_037226),
    (0x03722A, i_03722a),
    (0x03722E, i_03722e),
    (0x037234, i_037234),
    (0x037238, i_037238),
    (0x03723C, i_03723c),
    (0x037244, i_037244),
    (0x037248, i_037248),
    (0x03724A, i_03724a),
    (0x03724E, i_03724e),
    (0x037254, i_037254),
    (0x03725A, i_03725a),
    (0x037260, i_037260),
    (0x037266, i_037266),
    (0x03726C, i_03726c),
    (0x037272, i_037272),
    (0x037276, i_037276),
    (0x03727C, i_03727c),
    (0x037282, i_037282),
    (0x037288, i_037288),
    (0x03728E, i_03728e),
    (0x037294, i_037294),
    (0x03729A, i_03729a),
    (0x03729E, i_03729e),
    (0x0372A4, i_0372a4),
    (0x0372AA, i_0372aa),
    (0x0372B0, i_0372b0),
    (0x0372B6, i_0372b6),
    (0x0372BA, i_0372ba),
    (0x0372BE, i_0372be),
    (0x0372C2, i_0372c2),
    (0x0372C6, i_0372c6),
    (0x0372CE, i_0372ce),
    (0x0372D2, i_0372d2),
    (0x0372D4, i_0372d4),
    (0x0372D8, i_0372d8),
    (0x0372DE, i_0372de),
    (0x0372E4, i_0372e4),
    (0x0372EA, i_0372ea),
    (0x0372F0, i_0372f0),
    (0x0372F6, i_0372f6),
    (0x0372FC, i_0372fc),
    (0x037300, i_037300),
    (0x037306, i_037306),
    (0x03730C, i_03730c),
    (0x037312, i_037312),
    (0x037318, i_037318),
    (0x03731E, i_03731e),
    (0x037324, i_037324),
    (0x037328, i_037328),
    (0x03732C, i_03732c),
    (0x037330, i_037330),
    (0x037336, i_037336),
    (0x03733C, i_03733c),
    (0x037340, i_037340),
    (0x037346, i_037346),
    (0x03734C, i_03734c),
    (0x037352, i_037352),
    (0x037358, i_037358),
    (0x03735E, i_03735e),
    (0x037362, i_037362),
    (0x037368, i_037368),
    (0x03736E, i_03736e),
    (0x037374, i_037374),
    (0x03737A, i_03737a),
    (0x037380, i_037380),
    (0x037386, i_037386),
    (0x03738C, i_03738c),
    (0x037390, i_037390),
    (0x037394, i_037394),
    (0x037398, i_037398),
    (0x03739E, i_03739e),
    (0x0373A2, i_0373a2),
    (0x0373A8, i_0373a8),
    (0x0373AE, i_0373ae),
    (0x0373B4, i_0373b4),
    (0x0373BA, i_0373ba),
    (0x0373C0, i_0373c0),
    (0x0373C4, i_0373c4),
    (0x0373CA, i_0373ca),
    (0x0373D0, i_0373d0),
    (0x0373D6, i_0373d6),
    (0x0373DC, i_0373dc),
    (0x0373E2, i_0373e2),
    (0x0373E8, i_0373e8),
    (0x0373EE, i_0373ee),
    (0x0373F2, i_0373f2),
    (0x0373FA, i_0373fa),
    (0x0373FE, i_0373fe),
    (0x037402, i_037402),
    (0x037406, i_037406),
    (0x03740C, i_03740c),
    (0x037410, i_037410),
    (0x037416, i_037416),
    (0x03741C, i_03741c),
    (0x037422, i_037422),
    (0x037428, i_037428),
    (0x03742E, i_03742e),
    (0x037434, i_037434),
    (0x037438, i_037438),
    (0x03743E, i_03743e),
    (0x037444, i_037444),
    (0x03744A, i_03744a),
    (0x037450, i_037450),
    (0x037456, i_037456),
    (0x03745C, i_03745c),
    (0x037462, i_037462),
    (0x037468, i_037468),
    (0x03746C, i_03746c),
    (0x037472, i_037472),
    (0x037476, i_037476),
    (0x03747C, i_03747c),
    (0x037482, i_037482),
    (0x037486, i_037486),
    (0x03748A, i_03748a),
    (0x03748E, i_03748e),
    (0x037492, i_037492),
    (0x037496, i_037496),
    (0x03749A, i_03749a),
    (0x0374A0, i_0374a0),
    (0x0374A4, i_0374a4),
    (0x0374A8, i_0374a8),
    (0x0374AE, i_0374ae),
    (0x0374B2, i_0374b2),
    (0x0374B6, i_0374b6),
    (0x0374BE, i_0374be),
    (0x0374C2, i_0374c2),
    (0x0374C4, i_0374c4),
    (0x0374C8, i_0374c8),
    (0x0374CE, i_0374ce),
    (0x0374D4, i_0374d4),
    (0x0374DA, i_0374da),
    (0x0374E0, i_0374e0),
    (0x0374E6, i_0374e6),
    (0x0374EC, i_0374ec),
    (0x0374F0, i_0374f0),
    (0x0374F6, i_0374f6),
    (0x0374FC, i_0374fc),
    (0x037502, i_037502),
    (0x037508, i_037508),
    (0x03750E, i_03750e),
    (0x037514, i_037514),
    (0x037518, i_037518),
    (0x03751E, i_03751e),
    (0x037524, i_037524),
    (0x037528, i_037528),
    (0x03752E, i_03752e),
    (0x037534, i_037534),
    (0x037538, i_037538),
    (0x03753C, i_03753c),
    (0x037540, i_037540),
    (0x037544, i_037544),
    (0x037548, i_037548),
    (0x03754C, i_03754c),
    (0x037552, i_037552),
    (0x037556, i_037556),
    (0x03755A, i_03755a),
    (0x037560, i_037560),
    (0x037564, i_037564),
    (0x037568, i_037568),
    (0x037570, i_037570),
    (0x037574, i_037574),
    (0x037576, i_037576),
    (0x03757A, i_03757a),
    (0x037580, i_037580),
    (0x037586, i_037586),
    (0x03758C, i_03758c),
    (0x037592, i_037592),
    (0x037598, i_037598),
    (0x03759E, i_03759e),
    (0x0375A2, i_0375a2),
    (0x0375A8, i_0375a8),
    (0x0375AE, i_0375ae),
    (0x0375B4, i_0375b4),
    (0x0375BA, i_0375ba),
    (0x0375C0, i_0375c0),
    (0x0375C6, i_0375c6),
    (0x0375CC, i_0375cc),
    (0x0375D0, i_0375d0),
    (0x0378C6, i_0378c6),
    (0x0378CA, i_0378ca),
    (0x0378D0, i_0378d0),
    (0x0378D6, i_0378d6),
    (0x0378DA, i_0378da),
    (0x0378E0, i_0378e0),
    (0x0378E6, i_0378e6),
    (0x0378EC, i_0378ec),
    (0x0378F2, i_0378f2),
    (0x0378F8, i_0378f8),
    (0x0378FE, i_0378fe),
    (0x037904, i_037904),
    (0x03790A, i_03790a),
    (0x037910, i_037910),
    (0x037916, i_037916),
    (0x03791C, i_03791c),
    (0x037922, i_037922),
    (0x037928, i_037928),
    (0x03792E, i_03792e),
    (0x037934, i_037934),
    (0x037938, i_037938),
    (0x03793E, i_03793e),
    (0x037942, i_037942),
    (0x037948, i_037948),
    (0x03794E, i_03794e),
    (0x037954, i_037954),
    (0x037958, i_037958),
    (0x03795A, i_03795a),
    (0x037960, i_037960),
    (0x037964, i_037964),
    (0x03796A, i_03796a),
    (0x03796E, i_03796e),
    (0x037972, i_037972),
    (0x037974, i_037974),
    (0x03797C, i_03797c),
    (0x037980, i_037980),
    (0x037984, i_037984),
    (0x037988, i_037988),
    (0x03798C, i_03798c),
    (0x037990, i_037990),
    (0x037996, i_037996),
    (0x03799A, i_03799a),
    (0x03799E, i_03799e),
    (0x0379A2, i_0379a2),
    (0x0379A6, i_0379a6),
    (0x0379AE, i_0379ae),
    (0x0379B2, i_0379b2),
    (0x0379B8, i_0379b8),
    (0x0379BC, i_0379bc),
    (0x0379C2, i_0379c2),
    (0x0379C6, i_0379c6),
    (0x0379CC, i_0379cc),
    (0x0379D0, i_0379d0),
    (0x0379D6, i_0379d6),
    (0x0379DA, i_0379da),
    (0x0379DE, i_0379de),
    (0x0379E0, i_0379e0),
    (0x0379E2, i_0379e2),
    (0x0379E6, i_0379e6),
    (0x0379EA, i_0379ea),
    (0x0379F0, i_0379f0),
    (0x0379F6, i_0379f6),
    (0x0379FC, i_0379fc),
    (0x037A02, i_037a02),
    (0x037A08, i_037a08),
    (0x037A0E, i_037a0e),
    (0x037A14, i_037a14),
    (0x037A18, i_037a18),
    (0x037A1A, i_037a1a),
    (0x037A1E, i_037a1e),
    (0x037A24, i_037a24),
    (0x037A28, i_037a28),
    (0x037A2C, i_037a2c),
    (0x037A30, i_037a30),
    (0x037A32, i_037a32),
    (0x037C1A, i_037c1a),
    (0x037C20, i_037c20),
    (0x037C26, i_037c26),
    (0x037C2C, i_037c2c),
    (0x037C32, i_037c32),
    (0x037C36, i_037c36),
    (0x037C3A, i_037c3a),
    (0x037C3E, i_037c3e),
    (0x037C40, i_037c40),
    (0x037C46, i_037c46),
    (0x037C4C, i_037c4c),
    (0x037C50, i_037c50),
    (0x037C56, i_037c56),
    (0x037C5C, i_037c5c),
    (0x037C62, i_037c62),
    (0x037C68, i_037c68),
    (0x037C6A, i_037c6a),
    (0x037C6C, i_037c6c),
    (0x037C72, i_037c72),
    (0x037C74, i_037c74),
    (0x037C7A, i_037c7a),
    (0x037C80, i_037c80),
    (0x037C86, i_037c86),
    (0x037C8C, i_037c8c),
    (0x037C92, i_037c92),
    (0x037C98, i_037c98),
    (0x037C9E, i_037c9e),
    (0x037CA4, i_037ca4),
    (0x037CAA, i_037caa),
    (0x037CB0, i_037cb0),
    (0x037CB6, i_037cb6),
    (0x037CBE, i_037cbe),
    (0x037CC2, i_037cc2),
    (0x037CC6, i_037cc6),
    (0x037CCA, i_037cca),
    (0x037CCE, i_037cce),
    (0x037CD0, i_037cd0),
    (0x037CD4, i_037cd4),
    (0x037CD8, i_037cd8),
    (0x037CDC, i_037cdc),
    (0x037CE0, i_037ce0),
    (0x037CE4, i_037ce4),
    (0x037CEA, i_037cea),
    (0x037CEE, i_037cee),
    (0x037CF2, i_037cf2),
    (0x037CF8, i_037cf8),
    (0x037CFE, i_037cfe),
    (0x037D02, i_037d02),
    (0x037D06, i_037d06),
    (0x037D0A, i_037d0a),
    (0x037D0E, i_037d0e),
    (0x037D10, i_037d10),
    (0x037D14, i_037d14),
    (0x037D18, i_037d18),
    (0x037D1C, i_037d1c),
    (0x037D1E, i_037d1e),
    (0x037D22, i_037d22),
    (0x037D26, i_037d26),
    (0x037D2A, i_037d2a),
    (0x037D2C, i_037d2c),
    (0x037D30, i_037d30),
    (0x037D34, i_037d34),
    (0x037D38, i_037d38),
    (0x037D3C, i_037d3c),
    (0x037D40, i_037d40),
    (0x037D42, i_037d42),
    (0x037D46, i_037d46),
    (0x037D4A, i_037d4a),
    (0x037D4E, i_037d4e),
    (0x037D52, i_037d52),
    (0x037D54, i_037d54),
    (0x037D58, i_037d58),
    (0x037D5C, i_037d5c),
    (0x037D5E, i_037d5e),
    (0x037D62, i_037d62),
    (0x037D68, i_037d68),
    (0x037D6C, i_037d6c),
    (0x037D72, i_037d72),
    (0x037D76, i_037d76),
    (0x037D7A, i_037d7a),
    (0x037D7C, i_037d7c),
    (0x037D82, i_037d82),
    (0x037D86, i_037d86),
    (0x037D8C, i_037d8c),
    (0x037D90, i_037d90),
    (0x037D94, i_037d94),
    (0x037D96, i_037d96),
    (0x037D9C, i_037d9c),
    (0x037DA0, i_037da0),
    (0x037DA4, i_037da4),
    (0x037DA6, i_037da6),
    (0x037DAA, i_037daa),
    (0x037DAE, i_037dae),
    (0x037DB2, i_037db2),
    (0x037DB4, i_037db4),
    (0x037DB8, i_037db8),
    (0x037EC2, i_037ec2),
    (0x037EC8, i_037ec8),
    (0x037ECE, i_037ece),
    (0x037ED4, i_037ed4),
    (0x037EDA, i_037eda),
    (0x037EE0, i_037ee0),
    (0x037EE6, i_037ee6),
    (0x037EEC, i_037eec),
    (0x037EF2, i_037ef2),
    (0x037EF8, i_037ef8),
    (0x037EFE, i_037efe),
    (0x037F04, i_037f04),
    (0x037F08, i_037f08),
    (0x037F0C, i_037f0c),
    (0x037F10, i_037f10),
    (0x037F14, i_037f14),
    (0x037F16, i_037f16),
    (0x037F1A, i_037f1a),
    (0x037F1E, i_037f1e),
    (0x037F22, i_037f22),
    (0x037F26, i_037f26),
    (0x037F2A, i_037f2a),
    (0x037F30, i_037f30),
    (0x037F34, i_037f34),
    (0x037F38, i_037f38),
    (0x037F3E, i_037f3e),
    (0x037F44, i_037f44),
    (0x037F48, i_037f48),
    (0x037F4C, i_037f4c),
    (0x037F50, i_037f50),
    (0x037F54, i_037f54),
    (0x037F56, i_037f56),
    (0x037F5C, i_037f5c),
    (0x037F60, i_037f60),
    (0x037F64, i_037f64),
    (0x037F66, i_037f66),
    (0x037F6C, i_037f6c),
    (0x037F70, i_037f70),
    (0x037F76, i_037f76),
    (0x037F7A, i_037f7a),
    (0x037F7E, i_037f7e),
    (0x037F80, i_037f80),
    (0x037F86, i_037f86),
    (0x037F8A, i_037f8a),
    (0x037F8C, i_037f8c),
    (0x037F90, i_037f90),
    (0x037F94, i_037f94),
    (0x037F98, i_037f98),
    (0x037F9A, i_037f9a),
    (0x037FA0, i_037fa0),
    (0x037FA4, i_037fa4),
    (0x037FA8, i_037fa8),
    (0x037FAA, i_037faa),
    (0x037FAE, i_037fae),
    (0x037FB2, i_037fb2),
    (0x037FB6, i_037fb6),
    (0x037FB8, i_037fb8),
    (0x037FBC, i_037fbc),
    (0x037FC0, i_037fc0),
    (0x037FC4, i_037fc4),
    (0x037FC6, i_037fc6),
    (0x037FCA, i_037fca),
    (0x037FCE, i_037fce),
    (0x037FD2, i_037fd2),
    (0x037FD6, i_037fd6),
    (0x037FDA, i_037fda),
    (0x037FDC, i_037fdc),
    (0x037FE0, i_037fe0),
    (0x037FE4, i_037fe4),
    (0x037FE8, i_037fe8),
    (0x037FEC, i_037fec),
    (0x037FEE, i_037fee),
    (0x037FF2, i_037ff2),
    (0x037FF6, i_037ff6),
    (0x037FFA, i_037ffa),
    (0x037FFE, i_037ffe),
    (0x038000, i_038000),
    (0x038004, i_038004),
    (0x038008, i_038008),
    (0x03800C, i_03800c),
    (0x038010, i_038010),
    (0x038012, i_038012),
    (0x038016, i_038016),
    (0x03801A, i_03801a),
    (0x03801C, i_03801c),
    (0x038020, i_038020),
    (0x038026, i_038026),
    (0x03802A, i_03802a),
    (0x03802E, i_03802e),
    (0x038030, i_038030),
    (0x038034, i_038034),
    (0x038038, i_038038),
    (0x03803C, i_03803c),
    (0x03803E, i_03803e),
    (0x038042, i_038042),
    (0x0381A2, i_0381a2),
    (0x0381A8, i_0381a8),
    (0x0381AE, i_0381ae),
    (0x0381B4, i_0381b4),
    (0x0381BA, i_0381ba),
    (0x0381BE, i_0381be),
    (0x0381C4, i_0381c4),
    (0x0381CA, i_0381ca),
    (0x0381D0, i_0381d0),
    (0x0381D6, i_0381d6),
    (0x0381DC, i_0381dc),
    (0x0381E2, i_0381e2),
    (0x0381E6, i_0381e6),
    (0x0381EC, i_0381ec),
    (0x0381F2, i_0381f2),
    (0x0381F8, i_0381f8),
    (0x0381FE, i_0381fe),
    (0x038204, i_038204),
    (0x03820A, i_03820a),
    (0x03827A, i_03827a),
    (0x038280, i_038280),
    (0x038286, i_038286),
    (0x03828C, i_03828c),
    (0x038292, i_038292),
    (0x038296, i_038296),
    (0x03829C, i_03829c),
    (0x0382A2, i_0382a2),
    (0x0382A8, i_0382a8),
    (0x0382AE, i_0382ae),
    (0x0382B4, i_0382b4),
    (0x0382BA, i_0382ba),
    (0x0382BE, i_0382be),
    (0x0382C4, i_0382c4),
    (0x0382CA, i_0382ca),
    (0x0382D0, i_0382d0),
    (0x0382D6, i_0382d6),
    (0x0382DC, i_0382dc),
    (0x0382E2, i_0382e2),
    (0x0382EA, i_0382ea),
    (0x0382F0, i_0382f0),
    (0x0382F4, i_0382f4),
    (0x0382F8, i_0382f8),
    (0x0382FC, i_0382fc),
    (0x038300, i_038300),
    (0x038304, i_038304),
    (0x038306, i_038306),
    (0x03830C, i_03830c),
    (0x038310, i_038310),
    (0x038314, i_038314),
    (0x038318, i_038318),
    (0x03831C, i_03831c),
    (0x03831E, i_03831e),
    (0x038324, i_038324),
    (0x038328, i_038328),
    (0x03832E, i_03832e),
    (0x038332, i_038332),
    (0x038336, i_038336),
    (0x03833A, i_03833a),
    (0x03833E, i_03833e),
    (0x038340, i_038340),
    (0x038344, i_038344),
    (0x038348, i_038348),
    (0x03834C, i_03834c),
    (0x03834E, i_03834e),
    (0x038352, i_038352),
    (0x038356, i_038356),
    (0x03835A, i_03835a),
    (0x03835E, i_03835e),
    (0x038360, i_038360),
    (0x038364, i_038364),
    (0x038368, i_038368),
    (0x03836C, i_03836c),
    (0x03836E, i_03836e),
    (0x038374, i_038374),
    (0x038378, i_038378),
    (0x03837C, i_03837c),
    (0x03837E, i_03837e),
    (0x038382, i_038382),
    (0x038386, i_038386),
    (0x03838A, i_03838a),
    (0x03838E, i_03838e),
    (0x038392, i_038392),
    (0x038394, i_038394),
    (0x038398, i_038398),
    (0x03839C, i_03839c),
    (0x0383A0, i_0383a0),
    (0x0383A4, i_0383a4),
    (0x0383A6, i_0383a6),
    (0x0383AA, i_0383aa),
    (0x0383AE, i_0383ae),
    (0x0383B0, i_0383b0),
    (0x0383B4, i_0383b4),
    (0x0383BA, i_0383ba),
    (0x0383BE, i_0383be),
    (0x0383C4, i_0383c4),
    (0x0383C8, i_0383c8),
    (0x0383CC, i_0383cc),
    (0x0383CE, i_0383ce),
    (0x0383D4, i_0383d4),
    (0x0383D8, i_0383d8),
    (0x0383DC, i_0383dc),
    (0x0383DE, i_0383de),
    (0x0383E2, i_0383e2),
    (0x0383E6, i_0383e6),
    (0x0383EA, i_0383ea),
    (0x0383EC, i_0383ec),
    (0x0383F0, i_0383f0),
    (0x0383F8, i_0383f8),
    (0x0383FC, i_0383fc),
    (0x038402, i_038402),
    (0x038406, i_038406),
    (0x03840C, i_03840c),
    (0x038410, i_038410),
    (0x038416, i_038416),
    (0x03841A, i_03841a),
    (0x038420, i_038420),
    (0x038424, i_038424),
    (0x038428, i_038428),
    (0x03842A, i_03842a),
    (0x03873C, i_03873c),
    (0x038742, i_038742),
    (0x038748, i_038748),
    (0x038750, i_038750),
    (0x038756, i_038756),
    (0x03875A, i_03875a),
    (0x03875E, i_03875e),
    (0x038764, i_038764),
    (0x038768, i_038768),
    (0x03876E, i_03876e),
    (0x038774, i_038774),
    (0x03877A, i_03877a),
    (0x038780, i_038780),
    (0x038786, i_038786),
    (0x03878C, i_03878c),
    (0x038790, i_038790),
    (0x038796, i_038796),
    (0x03879C, i_03879c),
    (0x0387A2, i_0387a2),
    (0x0387A8, i_0387a8),
    (0x0387AE, i_0387ae),
    (0x0387B4, i_0387b4),
    (0x0387B8, i_0387b8),
    (0x0387BC, i_0387bc),
    (0x0387C0, i_0387c0),
    (0x0387C2, i_0387c2),
    (0x0387C8, i_0387c8),
    (0x0387CC, i_0387cc),
    (0x0387D0, i_0387d0),
    (0x0387D6, i_0387d6),
    (0x0387DA, i_0387da),
    (0x0387DE, i_0387de),
    (0x0387E2, i_0387e2),
    (0x0387E4, i_0387e4),
    (0x0387EA, i_0387ea),
    (0x0387EE, i_0387ee),
    (0x0387F4, i_0387f4),
    (0x0387F8, i_0387f8),
    (0x0387FC, i_0387fc),
    (0x038800, i_038800),
    (0x038804, i_038804),
    (0x038806, i_038806),
    (0x03880A, i_03880a),
    (0x03880E, i_03880e),
    (0x038812, i_038812),
    (0x038816, i_038816),
    (0x038818, i_038818),
    (0x03881C, i_03881c),
    (0x038820, i_038820),
    (0x038824, i_038824),
    (0x038828, i_038828),
    (0x03882C, i_03882c),
    (0x03882E, i_03882e),
    (0x038832, i_038832),
    (0x038836, i_038836),
    (0x03883A, i_03883a),
    (0x03883E, i_03883e),
    (0x038840, i_038840),
    (0x038844, i_038844),
    (0x038848, i_038848),
    (0x03884A, i_03884a),
    (0x03884E, i_03884e),
    (0x038854, i_038854),
    (0x038858, i_038858),
    (0x03885E, i_03885e),
    (0x038862, i_038862),
    (0x038866, i_038866),
    (0x038868, i_038868),
    (0x03886E, i_03886e),
    (0x038872, i_038872),
    (0x038878, i_038878),
    (0x03887C, i_03887c),
    (0x038880, i_038880),
    (0x038882, i_038882),
    (0x038888, i_038888),
    (0x03888C, i_03888c),
    (0x038890, i_038890),
    (0x038892, i_038892),
    (0x038896, i_038896),
    (0x03889A, i_03889a),
    (0x03889E, i_03889e),
    (0x0388A0, i_0388a0),
    (0x0388A6, i_0388a6),
    (0x0388AA, i_0388aa),
    (0x0388AE, i_0388ae),
    (0x0388B0, i_0388b0),
    (0x0388B4, i_0388b4),
    (0x0388BC, i_0388bc),
    (0x0388C0, i_0388c0),
    (0x0388C6, i_0388c6),
    (0x0388CA, i_0388ca),
    (0x0388D0, i_0388d0),
    (0x0388D4, i_0388d4),
    (0x0388DA, i_0388da),
    (0x0388DE, i_0388de),
    (0x0388E4, i_0388e4),
    (0x0388E8, i_0388e8),
    (0x0388EC, i_0388ec),
    (0x0388EE, i_0388ee),
    (0x038CF6, i_038cf6),
    (0x038CFA, i_038cfa),
    (0x038D00, i_038d00),
    (0x038D04, i_038d04),
    (0x038D08, i_038d08),
    (0x038D0C, i_038d0c),
    (0x038D0E, i_038d0e),
    (0x038D14, i_038d14),
    (0x038D1A, i_038d1a),
    (0x038D1C, i_038d1c),
    (0x038D20, i_038d20),
    (0x038D26, i_038d26),
    (0x038D28, i_038d28),
    (0x038D2C, i_038d2c),
    (0x038D32, i_038d32),
    (0x038D36, i_038d36),
    (0x038D3C, i_038d3c),
    (0x038D42, i_038d42),
    (0x038D48, i_038d48),
    (0x038D4C, i_038d4c),
    (0x038D50, i_038d50),
    (0x038D52, i_038d52),
    (0x038D56, i_038d56),
    (0x038D5C, i_038d5c),
    (0x038D60, i_038d60),
    (0x038D64, i_038d64),
    (0x038D66, i_038d66),
    (0x038D6C, i_038d6c),
    (0x038D70, i_038d70),
    (0x038D74, i_038d74),
    (0x038D76, i_038d76),
    (0x038D7C, i_038d7c),
    (0x038D80, i_038d80),
    (0x038D84, i_038d84),
    (0x038D86, i_038d86),
    (0x038E10, i_038e10),
    (0x038E14, i_038e14),
    (0x038E18, i_038e18),
    (0x038E1C, i_038e1c),
    (0x038E20, i_038e20),
    (0x038E26, i_038e26),
    (0x038E2C, i_038e2c),
    (0x038E30, i_038e30),
    (0x038E32, i_038e32),
    (0x038E38, i_038e38),
    (0x038E3E, i_038e3e),
    (0x038E42, i_038e42),
    (0x038E48, i_038e48),
    (0x038E9A, i_038e9a),
    (0x038E9E, i_038e9e),
    (0x038EA2, i_038ea2),
    (0x038EA6, i_038ea6),
    (0x038EAA, i_038eaa),
    (0x038EAE, i_038eae),
    (0x038EB0, i_038eb0),
    (0x038EB2, i_038eb2),
    (0x038EB4, i_038eb4),
    (0x038EB6, i_038eb6),
    (0x038EB8, i_038eb8),
    (0x038EBA, i_038eba),
    (0x038EBE, i_038ebe),
    (0x038EC2, i_038ec2),
    (0x038EC6, i_038ec6),
    (0x039148, i_039148),
    (0x03914C, i_03914c),
    (0x039152, i_039152),
    (0x039158, i_039158),
    (0x03915E, i_03915e),
    (0x039164, i_039164),
    (0x039168, i_039168),
    (0x03916A, i_03916a),
    (0x039170, i_039170),
    (0x039174, i_039174),
    (0x039178, i_039178),
    (0x03917C, i_03917c),
    (0x039180, i_039180),
    (0x039184, i_039184),
    (0x03918A, i_03918a),
    (0x03918E, i_03918e),
    (0x039192, i_039192),
    (0x039194, i_039194),
    (0x039198, i_039198),
    (0x03919E, i_03919e),
    (0x0391A2, i_0391a2),
    (0x0391A6, i_0391a6),
    (0x0391A8, i_0391a8),
    (0x0391AA, i_0391aa),
    (0x0391B0, i_0391b0),
    (0x0391B6, i_0391b6),
    (0x0391BC, i_0391bc),
    (0x0391C0, i_0391c0),
    (0x0391C2, i_0391c2),
    (0x0391C8, i_0391c8),
    (0x0391CC, i_0391cc),
    (0x0391D0, i_0391d0),
    (0x0391D4, i_0391d4),
    (0x0391D8, i_0391d8),
    (0x0391DC, i_0391dc),
    (0x0391E2, i_0391e2),
    (0x0391E6, i_0391e6),
    (0x0391EC, i_0391ec),
    (0x0391EE, i_0391ee),
    (0x0391F2, i_0391f2),
    (0x0391F8, i_0391f8),
    (0x0391FE, i_0391fe),
    (0x039204, i_039204),
    (0x03920A, i_03920a),
    (0x03920E, i_03920e),
    (0x039210, i_039210),
    (0x0392F8, i_0392f8),
    (0x0392FE, i_0392fe),
    (0x039304, i_039304),
    (0x03930A, i_03930a),
    (0x039310, i_039310),
    (0x039316, i_039316),
    (0x03931A, i_03931a),
    (0x03931E, i_03931e),
    (0x039322, i_039322),
    (0x039326, i_039326),
    (0x03932A, i_03932a),
    (0x039330, i_039330),
    (0x039334, i_039334),
    (0x039338, i_039338),
    (0x03933A, i_03933a),
    (0x03933E, i_03933e),
    (0x039340, i_039340),
    (0x039344, i_039344),
    (0x03934A, i_03934a),
    (0x03934E, i_03934e),
    (0x039350, i_039350),
    (0x039352, i_039352),
    (0x039354, i_039354),
    (0x03935A, i_03935a),
    (0x03935E, i_03935e),
    (0x039362, i_039362),
    (0x039366, i_039366),
    (0x03936A, i_03936a),
    (0x03936C, i_03936c),
    (0x039370, i_039370),
    (0x039376, i_039376),
    (0x03937A, i_03937a),
    (0x03937C, i_03937c),
    (0x039382, i_039382),
    (0x039386, i_039386),
    (0x03938A, i_03938a),
    (0x03938E, i_03938e),
    (0x039392, i_039392),
    (0x039396, i_039396),
    (0x03939A, i_03939a),
    (0x03939E, i_03939e),
    (0x0393A2, i_0393a2),
    (0x0393A6, i_0393a6),
    (0x0393AA, i_0393aa),
    (0x0393B0, i_0393b0),
    (0x0393B4, i_0393b4),
    (0x0393BA, i_0393ba),
    (0x0393C0, i_0393c0),
    (0x0393C6, i_0393c6),
    (0x0393CC, i_0393cc),
    (0x039416, i_039416),
    (0x03941A, i_03941a),
    (0x03941E, i_03941e),
    (0x039422, i_039422),
    (0x039428, i_039428),
    (0x03942C, i_03942c),
    (0x03942E, i_03942e),
    (0x039434, i_039434),
    (0x03943A, i_03943a),
    (0x039440, i_039440),
    (0x039444, i_039444),
    (0x039448, i_039448),
    (0x03944E, i_03944e),
    (0x039450, i_039450),
    (0x039456, i_039456),
    (0x03945A, i_03945a),
    (0x03945E, i_03945e),
    (0x0394A8, i_0394a8),
    (0x0394AC, i_0394ac),
    (0x0394B2, i_0394b2),
    (0x0394B6, i_0394b6),
    (0x0394BC, i_0394bc),
    (0x0394C0, i_0394c0),
    (0x0394C6, i_0394c6),
    (0x0394CC, i_0394cc),
    (0x0394D2, i_0394d2),
    (0x0394D6, i_0394d6),
    (0x0394DC, i_0394dc),
    (0x0394E2, i_0394e2),
    (0x039520, i_039520),
    (0x039526, i_039526),
    (0x03952C, i_03952c),
    (0x039530, i_039530),
    (0x039532, i_039532),
    (0x039536, i_039536),
    (0x03953A, i_03953a),
    (0x03953E, i_03953e),
    (0x039542, i_039542),
    (0x039546, i_039546),
    (0x03954A, i_03954a),
    (0x03954E, i_03954e),
    (0x039552, i_039552),
    (0x039556, i_039556),
    (0x03955A, i_03955a),
    (0x03955E, i_03955e),
    (0x039562, i_039562),
    (0x039566, i_039566),
    (0x03956A, i_03956a),
    (0x03956E, i_03956e),
    (0x039572, i_039572),
    (0x039576, i_039576),
    (0x03957A, i_03957a),
    (0x03957E, i_03957e),
    (0x039582, i_039582),
    (0x039586, i_039586),
    (0x03958A, i_03958a),
    (0x03958E, i_03958e),
    (0x039592, i_039592),
    (0x039596, i_039596),
    (0x03959A, i_03959a),
    (0x03959E, i_03959e),
    (0x0395A2, i_0395a2),
    (0x0395A6, i_0395a6),
    (0x0395AA, i_0395aa),
    (0x0395AE, i_0395ae),
    (0x0395B2, i_0395b2),
    (0x0395B6, i_0395b6),
    (0x0395BA, i_0395ba),
    (0x0395BE, i_0395be),
    (0x0395C2, i_0395c2),
    (0x0395C6, i_0395c6),
    (0x0395CA, i_0395ca),
    (0x0395CE, i_0395ce),
    (0x0395D2, i_0395d2),
    (0x0395D6, i_0395d6),
    (0x0395DA, i_0395da),
    (0x0395DE, i_0395de),
    (0x0395E0, i_0395e0),
    (0x0395E2, i_0395e2),
    (0x0395E6, i_0395e6),
    (0x0395E8, i_0395e8),
    (0x0395EA, i_0395ea),
    (0x0395EE, i_0395ee),
    (0x0395F2, i_0395f2),
    (0x0395F8, i_0395f8),
    (0x0395FC, i_0395fc),
    (0x039600, i_039600),
    (0x039602, i_039602),
    (0x039606, i_039606),
    (0x03960A, i_03960a),
    (0x03960E, i_03960e),
    (0x039612, i_039612),
    (0x039616, i_039616),
    (0x03961A, i_03961a),
    (0x03961C, i_03961c),
    (0x039620, i_039620),
    (0x039626, i_039626),
    (0x03962A, i_03962a),
    (0x03962E, i_03962e),
    (0x039630, i_039630),
    (0x039634, i_039634),
    (0x039638, i_039638),
    (0x03963C, i_03963c),
    (0x039640, i_039640),
    (0x039644, i_039644),
    (0x039648, i_039648),
    (0x03964E, i_03964e),
    (0x039652, i_039652),
    (0x039656, i_039656),
    (0x03965C, i_03965c),
    (0x039660, i_039660),
    (0x039664, i_039664),
    (0x039666, i_039666),
    (0x03966A, i_03966a),
    (0x03966E, i_03966e),
    (0x039672, i_039672),
    (0x039676, i_039676),
    (0x03967A, i_03967a),
    (0x039680, i_039680),
    (0x039682, i_039682),
    (0x039688, i_039688),
    (0x03968C, i_03968c),
    (0x039690, i_039690),
    (0x039694, i_039694),
    (0x039698, i_039698),
    (0x03969C, i_03969c),
    (0x0396A0, i_0396a0),
    (0x0396A2, i_0396a2),
    (0x0396A6, i_0396a6),
    (0x0396AA, i_0396aa),
    (0x0396AE, i_0396ae),
    (0x0396B0, i_0396b0),
    (0x0396B4, i_0396b4),
    (0x0396BA, i_0396ba),
    (0x0396BE, i_0396be),
    (0x0396C2, i_0396c2),
    (0x0396C4, i_0396c4),
    (0x0396C8, i_0396c8),
    (0x0396CC, i_0396cc),
    (0x0396D0, i_0396d0),
    (0x0396D4, i_0396d4),
    (0x0396D8, i_0396d8),
    (0x0396DC, i_0396dc),
    (0x0396E2, i_0396e2),
    (0x0396E6, i_0396e6),
    (0x0396EA, i_0396ea),
    (0x0396F0, i_0396f0),
    (0x0396F4, i_0396f4),
    (0x0396F8, i_0396f8),
    (0x0396FA, i_0396fa),
    (0x0396FE, i_0396fe),
    (0x039702, i_039702),
    (0x039706, i_039706),
    (0x03970A, i_03970a),
    (0x03970E, i_03970e),
    (0x039714, i_039714),
    (0x039716, i_039716),
    (0x03971C, i_03971c),
    (0x039720, i_039720),
    (0x039724, i_039724),
    (0x039728, i_039728),
    (0x03972C, i_03972c),
    (0x039730, i_039730),
    (0x039734, i_039734),
    (0x039736, i_039736),
    (0x03973A, i_03973a),
    (0x03973E, i_03973e),
    (0x039742, i_039742),
    (0x039744, i_039744),
    (0x039748, i_039748),
    (0x03974E, i_03974e),
    (0x039752, i_039752),
    (0x039756, i_039756),
    (0x039758, i_039758),
    (0x03975C, i_03975c),
    (0x039760, i_039760),
    (0x039764, i_039764),
    (0x039768, i_039768),
    (0x03976C, i_03976c),
    (0x039770, i_039770),
    (0x039776, i_039776),
    (0x03977A, i_03977a),
    (0x03977E, i_03977e),
    (0x039782, i_039782),
    (0x039788, i_039788),
    (0x03978C, i_03978c),
    (0x039790, i_039790),
    (0x039792, i_039792),
    (0x039796, i_039796),
    (0x03979A, i_03979a),
    (0x03979E, i_03979e),
    (0x0397A2, i_0397a2),
    (0x0397A6, i_0397a6),
    (0x0397AC, i_0397ac),
    (0x0397AE, i_0397ae),
    (0x0397B4, i_0397b4),
    (0x0397B8, i_0397b8),
    (0x0397BC, i_0397bc),
    (0x0397C0, i_0397c0),
    (0x0397C4, i_0397c4),
    (0x0397C8, i_0397c8),
    (0x0397CC, i_0397cc),
    (0x0397CE, i_0397ce),
    (0x0397D2, i_0397d2),
    (0x0397D6, i_0397d6),
    (0x0397DA, i_0397da),
    (0x0397DC, i_0397dc),
    (0x0397E0, i_0397e0),
    (0x0397E4, i_0397e4),
    (0x0397E8, i_0397e8),
    (0x0397EE, i_0397ee),
    (0x0397F2, i_0397f2),
    (0x0397F6, i_0397f6),
    (0x0397F8, i_0397f8),
    (0x0397FC, i_0397fc),
    (0x039800, i_039800),
    (0x039804, i_039804),
    (0x039808, i_039808),
    (0x03980C, i_03980c),
    (0x039812, i_039812),
    (0x039816, i_039816),
    (0x03981A, i_03981a),
    (0x03981E, i_03981e),
    (0x039822, i_039822),
    (0x039826, i_039826),
    (0x03982A, i_03982a),
    (0x03982C, i_03982c),
    (0x039830, i_039830),
    (0x039834, i_039834),
    (0x039838, i_039838),
    (0x03983C, i_03983c),
    (0x039840, i_039840),
    (0x039842, i_039842),
    (0x039846, i_039846),
    (0x03984A, i_03984a),
    (0x03984E, i_03984e),
    (0x039854, i_039854),
    (0x039858, i_039858),
    (0x03985C, i_03985c),
    (0x03985E, i_03985e),
    (0x039862, i_039862),
    (0x039866, i_039866),
    (0x03986A, i_03986a),
    (0x03986E, i_03986e),
    (0x039872, i_039872),
    (0x039878, i_039878),
    (0x03987C, i_03987c),
    (0x039880, i_039880),
    (0x039884, i_039884),
    (0x039888, i_039888),
    (0x03988C, i_03988c),
    (0x039890, i_039890),
    (0x039892, i_039892),
    (0x039896, i_039896),
    (0x03989A, i_03989a),
    (0x03989E, i_03989e),
    (0x0398A2, i_0398a2),
    (0x0398A6, i_0398a6),
    (0x0398F4, i_0398f4),
    (0x0398FA, i_0398fa),
    (0x039900, i_039900),
    (0x039904, i_039904),
    (0x039908, i_039908),
    (0x03990C, i_03990c),
    (0x039912, i_039912),
    (0x03993E, i_03993e),
    (0x039940, i_039940),
    (0x039946, i_039946),
    (0x03994C, i_03994c),
    (0x039950, i_039950),
    (0x039954, i_039954),
    (0x039958, i_039958),
    (0x03995E, i_03995e),
    (0x03998A, i_03998a),
    (0x039A24, i_039a24),
    (0x039A2A, i_039a2a),
    (0x039A30, i_039a30),
    (0x039A34, i_039a34),
    (0x039A38, i_039a38),
    (0x039A3C, i_039a3c),
    (0x039A42, i_039a42),
    (0x039A6E, i_039a6e),
    (0x039A70, i_039a70),
    (0x039A76, i_039a76),
    (0x039A7A, i_039a7a),
    (0x039A7E, i_039a7e),
    (0x039A82, i_039a82),
    (0x039A88, i_039a88),
    (0x039AB4, i_039ab4),
    (0x039B4E, i_039b4e),
    (0x039B54, i_039b54),
];
