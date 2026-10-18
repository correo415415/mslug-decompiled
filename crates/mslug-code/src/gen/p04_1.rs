// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Squad_SteerTowardTarget_041CE0  ($041CE0, 120 B, enemies/squad/wave_motion_041c1a.s)
/// $041CE0: move.w $8a(a6), d0
pub fn i_041ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CE4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041CE4: move.w $8c(a6), d1
pub fn i_041ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CE8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041CE8: sub.w $22(a6), d0
pub fn i_041ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CEC;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $041CEC: sub.w $24(a6), d1
pub fn i_041cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CF0;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $041CF0: move.w d0, d2
pub fn i_041cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CF2;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041CF2: cmpi.w #$0, d0
pub fn i_041cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CF6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $041CF6: bge.w $41cfc
pub fn i_041cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CFA;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x41CFC; return 10; }
    8
}
/// $041CFA: neg.w d2
pub fn i_041cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CFC;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $041CFC: add.w d1, d2
pub fn i_041cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CFE;
    cpu.ir = 0xD441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041CFE: cmpi.w #$0, d1
pub fn i_041cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D02;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $041D02: bge.w $41d0a
pub fn i_041d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D06;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x41D0A; return 10; }
    8
}
/// $041D06: sub.w d1, d2
pub fn i_041d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D08;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041D08: sub.w d1, d2
pub fn i_041d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D0A;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041D0A: cmp.w d5, d2
pub fn i_041d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D0C;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $041D0C: bcs.w $41d5e
pub fn i_041d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D10;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x41D5E; return 10; }
    8
}
/// $041D10: movem.w d1, -(a7)
pub fn i_041d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D14;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $041D14: jsr $5e018.l
pub fn i_041d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41D1A);
    cpu.pc = 0x5E018;
    16
}
/// $041D1A: movem.w (a7)+, d1
pub fn i_041d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D1E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $041D1E: cmpi.w #$20, d1
pub fn i_041d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D22;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $041D22: bgt.w $41d2a
pub fn i_041d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D26;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x41D2A; return 10; }
    8
}
/// $041D26: move.b d0, $76(a6)
pub fn i_041d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D2A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041D2A: cmp.b $76(a6), d0
pub fn i_041d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D2E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $041D2E: beq.w $41d54
pub fn i_041d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41D54; return 10; }
    8
}
/// $041D32: move.w $36(a6), d1
pub fn i_041d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D36;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041D36: lsl.w #$1, d1
pub fn i_041d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D38;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $041D38: neg.w d1
pub fn i_041d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D3A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $041D3A: sub.b $76(a6), d0
pub fn i_041d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D3E;
    cpu.ir = 0x902E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $041D3E: cmpi.b #$0, d0
pub fn i_041d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D42;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $041D42: blt.w $41d48
pub fn i_041d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D46;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x41D48; return 10; }
    8
}
/// $041D46: neg.w d1
pub fn i_041d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D48;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $041D48: add.w d1, $76(a6)
pub fn i_041d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D4C;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041D4C: move.b $76(a6), d0
pub fn i_041d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D50;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041D50: andi.w #$ff, d0
pub fn i_041d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D54;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041D54: jsr $420a4(pc)
pub fn i_041d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41D58);
    cpu.pc = 0x420A4;
    16
}
// ===== ClearXN_041d58  ($041D58, 6 B, thunks/ccr_helpers.c)
/// $041D58: andi.b #$ee, ccr
pub fn i_041d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D5C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $041D5C: rts
pub fn i_041d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_HaltVelocity_041D5E  ($041D5E, 8 B, enemies/squad/wave_motion_041c1a.s)
/// $041D5E: clr.w $28(a6)
pub fn i_041d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D62;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $041D62: clr.w $2a(a6)
pub fn i_041d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D66;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetXN_041d66  ($041D66, 6 B, thunks/ccr_helpers.c)
/// $041D66: ori.b #$11, ccr
pub fn i_041d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D6A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $041D6A: rts
pub fn i_041d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_TurnRateStepClamp_041D6C  ($041D6C, 44 B, enemies/squad/wave_motion_041c1a.s)
/// $041D6C: cmpi.w #$0, d1
pub fn i_041d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D70;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $041D70: bne.w $41d80
pub fn i_041d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41D80; return 10; }
    8
}
/// $041D74: nop
pub fn i_041d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D76;
    cpu.ir = 0x4E71;
    4
}
/// $041D76: nop
pub fn i_041d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D78;
    cpu.ir = 0x4E71;
    4
}
/// $041D78: cmpi.w #$0, d1
pub fn i_041d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D7C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $041D7C: nop
pub fn i_041d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D7E;
    cpu.ir = 0x4E71;
    4
}
/// $041D7E: trap #$f
pub fn i_041d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D80;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $041D80: add.w d1, $36(a6)
pub fn i_041d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D84;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041D84: cmpi.w #$0, d1
pub fn i_041d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D88;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $041D88: bgt.w $41d9e
pub fn i_041d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D8C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x41D9E; return 10; }
    8
}
/// $041D8C: cmp.w $36(a6), d0
pub fn i_041d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D90;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041D90: blt.w $41db0
pub fn i_041d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D94;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x41DB0; return 10; }
    8
}
/// $041D94: move.w d0, $36(a6)
pub fn i_041d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D98;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_041d98  ($041D98, 6 B, thunks/ccr_helpers.c)
/// $041D98: ori.b #$11, ccr
pub fn i_041d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D9C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $041D9C: rts
pub fn i_041d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41D9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_TurnRateClampHi_041D9E  ($041D9E, 12 B, enemies/squad/wave_motion_041c1a.s)
/// $041D9E: cmp.w $36(a6), d0
pub fn i_041d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DA2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041DA2: bgt.w $41db0
pub fn i_041da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DA6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x41DB0; return 10; }
    8
}
/// $041DA6: move.w d0, $36(a6)
pub fn i_041da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DAA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_041daa  ($041DAA, 6 B, thunks/ccr_helpers.c)
/// $041DAA: ori.b #$11, ccr
pub fn i_041daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DAE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $041DAE: rts
pub fn i_041dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_041db0  ($041DB0, 6 B, thunks/ccr_helpers.c)
/// $041DB0: andi.b #$ee, ccr
pub fn i_041db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DB4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $041DB4: rts
pub fn i_041db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_BobYFast_041DB6  ($041DB6, 38 B, enemies/squad/wave_motion_041c1a.s)
/// $041DB6: move.w $24(a6), d4
pub fn i_041db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DBA;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041DBA: move.w d4, -(a7)
pub fn i_041dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DBC;
    cpu.ir = 0x3F04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $041DBC: move.b $7a(a6), d0
pub fn i_041dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DC0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041DC0: addi.b #$80, $7a(a6)
pub fn i_041dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DC6;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $041DC6: andi.w #$ff, d0
pub fn i_041dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DCA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041DCA: add.w d0, d0
pub fn i_041dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DCC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041DCC: lea.l $2c072c.l, a0
pub fn i_041dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $041DD2: move.w (a0, d0.w), d0
pub fn i_041dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DD6;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $041DD6: asr.w #$8, d0
pub fn i_041dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DD8;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $041DD8: bra.w $41e24
pub fn i_041dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x41E24;
    10
}
// ===== Squad_BobYWide_041DDC  ($041DDC, 38 B, enemies/squad/wave_motion_041c1a.s)
/// $041DDC: move.w $24(a6), d4
pub fn i_041ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DE0;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041DE0: move.w d4, -(a7)
pub fn i_041de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DE2;
    cpu.ir = 0x3F04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $041DE2: move.b $7a(a6), d0
pub fn i_041de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DE6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041DE6: addi.b #$10, $7a(a6)
pub fn i_041de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DEC;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $041DEC: andi.w #$ff, d0
pub fn i_041dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DF0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041DF0: add.w d0, d0
pub fn i_041df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DF2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041DF2: lea.l $2c072c.l, a0
pub fn i_041df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $041DF8: move.w (a0, d0.w), d0
pub fn i_041df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DFC;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $041DFC: asr.w #$6, d0
pub fn i_041dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41DFE;
    cpu.ir = 0xEC40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $041DFE: bra.w $41e24
pub fn i_041dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E02;
    cpu.ir = 0x6000;
    cpu.pc = 0x41E24;
    10
}
// ===== Squad_BobYNarrow_041E02  ($041E02, 34 B, enemies/squad/wave_motion_041c1a.s)
/// $041E02: move.w $24(a6), d4
pub fn i_041e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E06;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041E06: move.w d4, -(a7)
pub fn i_041e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E08;
    cpu.ir = 0x3F04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $041E08: move.b $7a(a6), d0
pub fn i_041e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E0C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041E0C: addi.b #$10, $7a(a6)
pub fn i_041e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E12;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $041E12: andi.w #$ff, d0
pub fn i_041e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E16;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041E16: add.w d0, d0
pub fn i_041e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E18;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041E18: lea.l $2c072c.l, a0
pub fn i_041e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $041E1E: move.w (a0, d0.w), d0
pub fn i_041e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E22;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $041E22: asr.w #$7, d0
pub fn i_041e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E24;
    cpu.ir = 0xEE40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
// ===== Squad_BobYApply_041E24  ($041E24, 24 B, enemies/squad/wave_motion_041c1a.s)
/// $041E24: move.w d0, $8e(a6)
pub fn i_041e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E28;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041E28: add.w d0, $24(a6)
pub fn i_041e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E2C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041E2C: jsr $28d70.l
pub fn i_041e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41E32);
    cpu.pc = 0x28D70;
    16
}
/// $041E32: bcs.w $41e42
pub fn i_041e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E36;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x41E42; return 10; }
    8
}
/// $041E36: move.w (a7)+, d4
pub fn i_041e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E38;
    cpu.ir = 0x381F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $041E38: move.w d4, $24(a6)
pub fn i_041e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E3C;
    cpu.ir = 0x3D44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ClearXN_041e3c  ($041E3C, 6 B, thunks/ccr_helpers.c)
/// $041E3C: andi.b #$ee, ccr
pub fn i_041e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E40;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $041E40: rts
pub fn i_041e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_BobYRestore_041E42  ($041E42, 6 B, enemies/squad/wave_motion_041c1a.s)
/// $041E42: move.w (a7)+, d4
pub fn i_041e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E44;
    cpu.ir = 0x381F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $041E44: move.w d4, $24(a6)
pub fn i_041e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E48;
    cpu.ir = 0x3D44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_041e48  ($041E48, 6 B, thunks/ccr_helpers.c)
/// $041E48: ori.b #$11, ccr
pub fn i_041e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E4C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $041E4C: rts
pub fn i_041e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_StateDispatch_041E4E  ($041E4E, 34 B, enemies/squad/wave_motion_041c1a.s)
/// $041E4E: cmpi.w #$6, d0
pub fn i_041e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E52;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $041E52: bcs.w $41e62
pub fn i_041e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E56;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x41E62; return 10; }
    8
}
/// $041E56: nop
pub fn i_041e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E58;
    cpu.ir = 0x4E71;
    4
}
/// $041E58: nop
pub fn i_041e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E5A;
    cpu.ir = 0x4E71;
    4
}
/// $041E5A: cmpi.w #$6, d0
pub fn i_041e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E5E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $041E5E: nop
pub fn i_041e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E60;
    cpu.ir = 0x4E71;
    4
}
/// $041E60: trap #$f
pub fn i_041e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E62;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $041E62: lsl.w #$2, d0
pub fn i_041e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E64;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $041E64: lea.l $28633c.l, a0
pub fn i_041e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28633C;
    20
}
/// $041E6A: move.l (a0, d0.w), (a6)
pub fn i_041e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E6E;
    cpu.ir = 0x2CB0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $041E6E: rts
pub fn i_041e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadAnim_State4Select_041E70  ($041E70, 58 B, enemies/squad/wave_motion_041c1a.s)
/// $041E70: jsr $5e1ea.l
pub fn i_041e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41E76);
    cpu.pc = 0x5E1EA;
    16
}
/// $041E76: movem.l a0, -(a7)
pub fn i_041e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E7A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $041E7A: move.b #$4, $89(a6)
pub fn i_041e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041E80: jsr $41c1a(pc)
pub fn i_041e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41E84);
    cpu.pc = 0x41C1A;
    16
}
/// $041E84: movem.l (a7)+, a0
pub fn i_041e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E88;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $041E88: move.w $8c(a6), d0
pub fn i_041e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E8C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041E8C: cmp.w $24(a0), d0
pub fn i_041e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E90;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041E90: bgt.w $41eb2
pub fn i_041e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E94;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x41EB2; return 10; }
    8
}
/// $041E94: lea.l $2bc62e.l, a0
pub fn i_041e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41E9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC62E;
    20
}
/// $041E9A: cmpi.b #$ff, $7c(a6)
pub fn i_041e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EA0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $041EA0: bne.w $41eaa
pub fn i_041ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41EAA; return 10; }
    8
}
/// $041EA4: lea.l $2bc6b0.l, a0
pub fn i_041ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC6B0;
    20
}
// ===== JsrAbsThunk_041eaa  ($041EAA, 8 B, thunks/jsr_abs_thunks.c)
/// $041EAA: jsr $799de.l
pub fn i_041eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41EB0);
    cpu.pc = 0x799DE;
    16
}
/// $041EB0: rts
pub fn i_041eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadAnim_State5Select_041EB2  ($041EB2, 52 B, enemies/squad/wave_motion_041c1a.s)
/// $041EB2: movem.l a0, -(a7)
pub fn i_041eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EB6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $041EB6: move.b #$5, $89(a6)
pub fn i_041eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041EBC: jsr $41c1a(pc)
pub fn i_041ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41EC0);
    cpu.pc = 0x41C1A;
    16
}
/// $041EC0: movem.l (a7)+, a0
pub fn i_041ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EC4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $041EC4: move.w $8c(a6), d0
pub fn i_041ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EC8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041EC8: cmp.w $24(a0), d0
pub fn i_041ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41ECC;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041ECC: bgt.w $41eee
pub fn i_041ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41ED0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x41EEE; return 10; }
    8
}
/// $041ED0: lea.l $2bc732.l, a0
pub fn i_041ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41ED6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC732;
    20
}
/// $041ED6: cmpi.b #$ff, $7c(a6)
pub fn i_041ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EDC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $041EDC: bne.w $41ee6
pub fn i_041edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41EE6; return 10; }
    8
}
/// $041EE0: lea.l $2bc7b4.l, a0
pub fn i_041ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EE6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC7B4;
    20
}
// ===== JsrAbsThunk_041ee6  ($041EE6, 8 B, thunks/jsr_abs_thunks.c)
/// $041EE6: jsr $799de.l
pub fn i_041ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41EEC);
    cpu.pc = 0x799DE;
    16
}
/// $041EEC: rts
pub fn i_041eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadAnim_ApproachSelect_041EEE  ($041EEE, 38 B, enemies/squad/wave_motion_041c1a.s)
/// $041EEE: move.w $22(a6), d0
pub fn i_041eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041EF2: subi.w #$20, d0
pub fn i_041ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EF6;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041EF6: cmp.w $22(a0), d0
pub fn i_041ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EFA;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041EFA: blt.w $41f1c
pub fn i_041efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41EFE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x41F1C; return 10; }
    8
}
/// $041EFE: lea.l $2bc836.l, a0
pub fn i_041efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC836;
    20
}
/// $041F04: cmpi.b #$ff, $7c(a6)
pub fn i_041f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F0A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $041F0A: bne.w $41f14
pub fn i_041f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41F14; return 10; }
    8
}
/// $041F0E: lea.l $2bc8b8.l, a0
pub fn i_041f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F14;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC8B8;
    20
}
// ===== JsrAbsThunk_041f14  ($041F14, 8 B, thunks/jsr_abs_thunks.c)
/// $041F14: jsr $799de.l
pub fn i_041f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41F1A);
    cpu.pc = 0x799DE;
    16
}
/// $041F1A: rts
pub fn i_041f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadAnim_ArriveSelect_041F1C  ($041F1C, 22 B, enemies/squad/wave_motion_041c1a.s)
/// $041F1C: lea.l $2bc93a.l, a0
pub fn i_041f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC93A;
    20
}
/// $041F22: cmpi.b #$ff, $7c(a6)
pub fn i_041f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F28;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $041F28: bne.w $41f32
pub fn i_041f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F2C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41F32; return 10; }
    8
}
/// $041F2C: lea.l $2bc9bc.l, a0
pub fn i_041f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC9BC;
    20
}
// ===== JsrAbsThunk_041f32  ($041F32, 8 B, thunks/jsr_abs_thunks.c)
/// $041F32: jsr $799de.l
pub fn i_041f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41F38);
    cpu.pc = 0x799DE;
    16
}
/// $041F38: rts
pub fn i_041f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_TagSharedBit_041F3A  ($041F3A, 14 B, enemies/squad/wave_motion_041c1a.s)
/// $041F3A: movea.l $c(a6), a0
pub fn i_041f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F3E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $041F3E: move.b $85(a6), d0
pub fn i_041f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F42;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041F42: bset.b d0, $21(a0)
pub fn i_041f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F46;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041F46: rts
pub fn i_041f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_PhaseStepToTarget_041F48  ($041F48, 48 B, enemies/squad/wave_motion_041c1a.s)
/// $041F48: move.w #$4, d1
pub fn i_041f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041F4C: bra.w $41f54
pub fn i_041f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F50;
    cpu.ir = 0x6000;
    cpu.pc = 0x41F54;
    10
}
/// $041F50: move.w #$1, d1
pub fn i_041f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F54;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041F54: move.w $76(a6), d0
pub fn i_041f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F58;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041F58: cmp.w $78(a6), d0
pub fn i_041f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F5C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041F5C: beq.w $41f7e
pub fn i_041f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41F7E; return 10; }
    8
}
/// $041F60: blt.w $41f66
pub fn i_041f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F64;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x41F66; return 10; }
    8
}
/// $041F64: neg.w d1
pub fn i_041f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F66;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $041F66: add.w d1, d0
pub fn i_041f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F68;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041F68: andi.w #$ff, d0
pub fn i_041f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F6C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041F6C: move.w d0, $76(a6)
pub fn i_041f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F70;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041F70: cmp.w $78(a6), d0
pub fn i_041f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F74;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041F74: beq.w $41f7e
pub fn i_041f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F78;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41F7E; return 10; }
    8
}
// ===== ClearXN_041f78  ($041F78, 6 B, thunks/ccr_helpers.c)
/// $041F78: andi.b #$ee, ccr
pub fn i_041f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F7C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $041F7C: rts
pub fn i_041f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_041f7e  ($041F7E, 6 B, thunks/ccr_helpers.c)
/// $041F7E: ori.b #$11, ccr
pub fn i_041f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F82;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $041F82: rts
pub fn i_041f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ApplyLeaderDelta_041F84  ($041F84, 48 B, enemies/squad/wave_motion_041c1a.s)
/// $041F84: jsr $5e506.l
pub fn i_041f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41F8A);
    cpu.pc = 0x5E506;
    16
}
/// $041F8A: move.b $84(a6), d0
pub fn i_041f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F8E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041F8E: ext.w d0
pub fn i_041f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F90;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $041F90: add.w d0, $38(a6)
pub fn i_041f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F94;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041F94: move.b $82(a6), d0
pub fn i_041f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F98;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041F98: ext.w d0
pub fn i_041f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F9A;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $041F9A: add.w d0, $22(a6)
pub fn i_041f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41F9E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041F9E: move.b $83(a6), d0
pub fn i_041f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FA2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041FA2: ext.w d0
pub fn i_041fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FA4;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $041FA4: add.w $8e(a0), d0
pub fn i_041fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FA8;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $041FA8: add.w d0, $24(a6)
pub fn i_041fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FAC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041FAC: move.b $7c(a0), $7c(a6)
pub fn i_041fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FB2;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $041FB2: rts
pub fn i_041fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_SpawnEight_041FB4  ($041FB4, 66 B, enemies/squad/spawn_states_041fb4.s)
/// $041FB4: clr.w d0
pub fn i_041fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FB6;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $041FB6: lea.l $28615c.l, a1
pub fn i_041fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FBC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28615C;
    20
}
/// $041FBC: movem.l d0/a1, -(a7)
pub fn i_041fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FC0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $041FC0: lea.l $40f00(pc), a1
pub fn i_041fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40F00;
    16
}
/// $041FC4: jsr $4ae.l
pub fn i_041fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41FCA);
    cpu.pc = 0x4AE;
    16
}
/// $041FCA: jsr $5dd02.l
pub fn i_041fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41FD0);
    cpu.pc = 0x5DD02;
    16
}
/// $041FD0: movem.l (a7)+, d0/a1
pub fn i_041fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FD4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $041FD4: move.b (a1), $82(a0)
pub fn i_041fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FD8;
    cpu.ir = 0x1151;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x82);
    bus.write8(ad3, v2 as u8);
    16
}
/// $041FD8: move.b $1(a1), $83(a0)
pub fn i_041fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FDE;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x83);
    bus.write8(ad4, v3 as u8);
    20
}
/// $041FDE: move.b $2(a1), $84(a0)
pub fn i_041fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FE4;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x84);
    bus.write8(ad4, v3 as u8);
    20
}
/// $041FE4: move.b $3(a1), $85(a0)
pub fn i_041fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FEA;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x85);
    bus.write8(ad4, v3 as u8);
    20
}
/// $041FEA: addq.l #$4, a1
pub fn i_041fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FEC;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $041FEC: addq.w #$1, d0
pub fn i_041fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FEE;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $041FEE: cmpi.w #$8, d0
pub fn i_041fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FF2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $041FF2: bcs.b $41fbc
pub fn i_041ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FF4;
    cpu.ir = 0x65C8;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x41FBC; return 10; }
    8
}
/// $041FF4: rts
pub fn i_041ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_PollSharedState_041FF6  ($041FF6, 74 B, enemies/squad/spawn_states_041fb4.s)
/// $041FF6: movea.l $c(a6), a0
pub fn i_041ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FFA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $041FFA: move.b $85(a6), d0
pub fn i_041ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41FFE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041FFE: andi.w #$7, d0
pub fn i_041ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42002;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042002: addi.w #$80, d0
pub fn i_042002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42006;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042006: move.b (a0, d0.w), d0
pub fn i_042006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4200A;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $04200A: cmp.b $80(a6), d0
pub fn i_04200a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4200E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04200E: beq.w $4203e
pub fn i_04200e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42012;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4203E; return 10; }
    8
}
/// $042012: move.b d0, $80(a6)
pub fn i_042012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42016;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042016: move.b d0, $81(a6)
pub fn i_042016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4201A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04201A: lea.l $40f82(pc), a1
pub fn i_04201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4201E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40F82;
    16
}
/// $04201E: move.l a1, (a6)
pub fn i_04201e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42020;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042020: btst.b #$7, $80(a6)
pub fn i_042020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42026;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $042026: beq.w $4203e
pub fn i_042026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4202A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4203E; return 10; }
    8
}
/// $04202A: move.b $80(a6), d0
pub fn i_04202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4202E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04202E: andi.w #$7f, d0
pub fn i_04202e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42032;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042032: lsl.w #$2, d0
pub fn i_042032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42034;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $042034: lea.l $2863bc.l, a0
pub fn i_042034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4203A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2863BC;
    20
}
/// $04203A: move.l (a0, d0.w), (a6)
pub fn i_04203a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4203E;
    cpu.ir = 0x2CB0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $04203E: rts
pub fn i_04203e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42040;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_WriteBackState_042040  ($042040, 42 B, enemies/squad/spawn_states_041fb4.s)
/// $042040: move.b $80(a6), d1
pub fn i_042040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42044;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042044: movea.l $c(a6), a0
pub fn i_042044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42048;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $042048: move.b $85(a6), d0
pub fn i_042048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4204C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04204C: andi.w #$7, d0
pub fn i_04204c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42050;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042050: addi.w #$80, d0
pub fn i_042050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42054;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042054: move.b (a0, d0.w), d2
pub fn i_042054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42058;
    cpu.ir = 0x1430;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $042058: cmp.b $81(a6), d2
pub fn i_042058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4205C;
    cpu.ir = 0xB42E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04205C: bne.w $42068
pub fn i_04205c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42060;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42068; return 10; }
    8
}
/// $042060: move.b d1, (a0, d0.w)
pub fn i_042060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42064;
    cpu.ir = 0x1181;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $042064: move.b d1, $81(a6)
pub fn i_042064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42068;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042068: rts
pub fn i_042068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4206A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_DepthToScaleIdx_04206A  ($04206A, 58 B, enemies/squad/spawn_states_041fb4.s)
/// $04206A: cmpi.w #$70, d0
pub fn i_04206a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4206E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x70, Size::Word, false);
    8
}
/// $04206E: bgt.w $42076
pub fn i_04206e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42072;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x42076; return 10; }
    8
}
/// $042072: move.w #$70, d0
pub fn i_042072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42076;
    cpu.ir = 0x303C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042076: cmpi.w #$c0, d0
pub fn i_042076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4207A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $04207A: blt.w $42082
pub fn i_04207a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4207E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x42082; return 10; }
    8
}
/// $04207E: move.w #$c0, d0
pub fn i_04207e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42082;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042082: subi.w #$68, d0
pub fn i_042082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42086;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x68, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042086: lsr.w #$3, d0
pub fn i_042086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42088;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042088: cmpi.w #$0, d0
pub fn i_042088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4208C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $04208C: bge.w $42092
pub fn i_04208c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42090;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x42092; return 10; }
    8
}
/// $042090: clr.w d0
pub fn i_042090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42092;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042092: cmpi.w #$a, d0
pub fn i_042092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42096;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $042096: blt.w $4209e
pub fn i_042096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4209A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4209E; return 10; }
    8
}
/// $04209A: move.w #$a, d0
pub fn i_04209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4209E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04209E: andi.w #$fe, d0
pub fn i_04209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420A2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0420A2: rts
pub fn i_0420a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_SinCosVelocity_0420A4  ($0420A4, 64 B, enemies/squad/spawn_states_041fb4.s)
/// $0420A4: andi.w #$ff, d0
pub fn i_0420a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420A8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0420A8: add.w d0, d0
pub fn i_0420a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420AA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0420AA: lea.l $2c072c.l, a1
pub fn i_0420aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $0420B0: lea.l $2c07ac.l, a2
pub fn i_0420b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420B6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C07AC;
    20
}
/// $0420B6: move.w (a1, d0.w), d1
pub fn i_0420b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420BA;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0420BA: move.w (a2, d0.w), d2
pub fn i_0420ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420BE;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0420BE: move.w $36(a6), d0
pub fn i_0420be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420C2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0420C2: andi.w #$7fff, d0
pub fn i_0420c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7FFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0420C6: cmpi.w #$100, d0
pub fn i_0420c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420CA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $0420CA: bgt.w $420d2
pub fn i_0420ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420CE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x420D2; return 10; }
    8
}
/// $0420CE: move.w #$100, d0
pub fn i_0420ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0420D2: muls.w d0, d1
pub fn i_0420d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420D4;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0420D4: muls.w d0, d2
pub fn i_0420d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420D6;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0420D6: asr.l #$8, d1
pub fn i_0420d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420D8;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $0420D8: asr.l #$8, d2
pub fn i_0420d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420DA;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $0420DA: move.w d1, $2a(a6)
pub fn i_0420da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420DE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0420DE: move.w d2, $28(a6)
pub fn i_0420de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420E2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0420E2: rts
pub fn i_0420e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadPair_SpawnJoinTail_0420E4  ($0420E4, 4 B, enemies/squad/spawn_states_041fb4.s)
/// $0420E4: bra.w $42188
pub fn i_0420e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x42188;
    10
}
// ===== SquadPair_SpawnFlagged_0420E8  ($0420E8, 22 B, enemies/squad/spawn_states_041fb4.s)
/// $0420E8: clr.b $88(a6)
pub fn i_0420e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420EC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0420EC: move.b #$1, $7f(a6)
pub fn i_0420ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0420F2: bra.w $420fe
pub fn i_0420f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x420FE;
    10
}
// ===== SquadPair_SpawnCore_0420FE  ($0420FE, 132 B, enemies/squad/spawn_states_041fb4.s)
/// $0420FE: move.w #$1064, d0
pub fn i_0420fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42102;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042102: jsr $2352.l
pub fn i_042102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42108);
    cpu.pc = 0x2352;
    16
}
/// $042108: move.w $76(a6), d0
pub fn i_042108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4210C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04210C: lsr.w #$1, d0
pub fn i_04210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4210E;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04210E: cmpi.w #$6, d0
pub fn i_04210e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42112;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $042112: bcs.w $4211a
pub fn i_042112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42116;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4211A; return 10; }
    8
}
/// $042116: move.w #$5, d0
pub fn i_042116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4211A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04211A: move.w d0, d3
pub fn i_04211a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4211C;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04211C: lsl.w #$2, d3
pub fn i_04211c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4211E;
    cpu.ir = 0xE54B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04211E: lea.l $2861d4.l, a0
pub fn i_04211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42124;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2861D4;
    20
}
/// $042124: move.w (a0, d3.w), d1
pub fn i_042124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42128;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $042128: move.w $2(a0, d3.w), d2
pub fn i_042128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4212C;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04212C: movem.w d0-d2, -(a7)
pub fn i_04212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42130;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $042130: lea.l $415c6(pc), a1
pub fn i_042130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42134;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415C6;
    16
}
/// $042134: jsr $4ae.l
pub fn i_042134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4213A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4213A);
    cpu.pc = 0x4AE;
    16
}
/// $04213A: jsr $5dd02.l
pub fn i_04213a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42140);
    cpu.pc = 0x5DD02;
    16
}
/// $042140: movem.w (a7)+, d0-d2
pub fn i_042140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42144;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $042144: add.w d1, $22(a0)
pub fn i_042144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42148;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $042148: add.w d2, $24(a0)
pub fn i_042148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4214C;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04214C: movem.w d0-d2, -(a7)
pub fn i_04214c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42150;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $042150: lea.l $41626(pc), a1
pub fn i_042150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42154;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41626;
    16
}
/// $042154: jsr $4ae.l
pub fn i_042154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4215A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4215A);
    cpu.pc = 0x4AE;
    16
}
/// $04215A: jsr $5dd02.l
pub fn i_04215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42160;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42160);
    cpu.pc = 0x5DD02;
    16
}
/// $042160: movem.w (a7)+, d0-d2
pub fn i_042160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42164;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $042164: move.w d0, $76(a0)
pub fn i_042164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42168;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042168: add.w d1, $22(a0)
pub fn i_042168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4216C;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04216C: add.w d2, $24(a0)
pub fn i_04216c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42170;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $042170: move.b $7f(a6), $7f(a0)
pub fn i_042170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42176;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7F);
    bus.write8(ad4, v3 as u8);
    20
}
/// $042176: move.b $88(a6), $88(a0)
pub fn i_042176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4217C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x88);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04217C: move.b #$87, $80(a6)
pub fn i_04217c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42182;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_042182  ($042182, 6 B, thunks/jsr_pc_thunks.c)
/// $042182: jsr $42040(pc)
pub fn i_042182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42186;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x42186);
    cpu.pc = 0x42040;
    16
}
/// $042186: rts
pub fn i_042186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42188;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadPair_SpawnPlain_042188  ($042188, 126 B, enemies/squad/spawn_states_041fb4.s)
/// $042188: move.w #$1064, d0
pub fn i_042188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4218C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04218C: jsr $2352.l
pub fn i_04218c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42192);
    cpu.pc = 0x2352;
    16
}
/// $042192: move.w $76(a6), d0
pub fn i_042192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42196;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042196: lsr.w #$1, d0
pub fn i_042196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42198;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $042198: cmpi.w #$6, d0
pub fn i_042198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4219C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $04219C: bcs.w $421a4
pub fn i_04219c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421A0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x421A4; return 10; }
    8
}
/// $0421A0: move.w #$5, d0
pub fn i_0421a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0421A4: move.w d0, d3
pub fn i_0421a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421A6;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0421A6: lsl.w #$2, d3
pub fn i_0421a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421A8;
    cpu.ir = 0xE54B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0421A8: lea.l $2861d4.l, a0
pub fn i_0421a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2861D4;
    20
}
/// $0421AE: move.w (a0, d3.w), d1
pub fn i_0421ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421B2;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0421B2: move.w $2(a0, d3.w), d2
pub fn i_0421b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421B6;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0421B6: movem.w d0-d2, -(a7)
pub fn i_0421b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421BA;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $0421BA: lea.l $415c6(pc), a1
pub fn i_0421ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415C6;
    16
}
/// $0421BE: jsr $4ae.l
pub fn i_0421be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x421C4);
    cpu.pc = 0x4AE;
    16
}
/// $0421C4: jsr $5dd02.l
pub fn i_0421c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x421CA);
    cpu.pc = 0x5DD02;
    16
}
/// $0421CA: movem.w (a7)+, d0-d2
pub fn i_0421ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421CE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0421CE: add.w d1, $22(a0)
pub fn i_0421ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421D2;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0421D2: add.w d2, $24(a0)
pub fn i_0421d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421D6;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0421D6: movem.w d0-d2, -(a7)
pub fn i_0421d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421DA;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $0421DA: lea.l $41626(pc), a1
pub fn i_0421da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41626;
    16
}
/// $0421DE: jsr $4ae.l
pub fn i_0421de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x421E4);
    cpu.pc = 0x4AE;
    16
}
/// $0421E4: jsr $5dd02.l
pub fn i_0421e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x421EA);
    cpu.pc = 0x5DD02;
    16
}
/// $0421EA: movem.w (a7)+, d0-d2
pub fn i_0421ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421EE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0421EE: move.w d0, $76(a0)
pub fn i_0421ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421F2;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0421F2: add.w d1, $22(a0)
pub fn i_0421f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421F6;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0421F6: add.w d2, $24(a0)
pub fn i_0421f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421FA;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0421FA: clr.b $7f(a0)
pub fn i_0421fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x421FE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0421FE: move.b #$1, $88(a0)
pub fn i_0421fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42204;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042204: rts
pub fn i_042204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42206;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TrioSpawner_PatternedPair_042206  ($042206, 124 B, enemies/squad/spawn_states_041fb4.s)
/// $042206: move.w #$10a3, d0
pub fn i_042206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4220A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04220A: jsr $2352.l
pub fn i_04220a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42210;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42210);
    cpu.pc = 0x2352;
    16
}
/// $042210: move.w $9a(a6), d0
pub fn i_042210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42214;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042214: addq.w #$1, $9a(a6)
pub fn i_042214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42218;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x9A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $042218: andi.w #$f, d0
pub fn i_042218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4221C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04221C: lea.l $28631c.l, a0
pub fn i_04221c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42222;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28631C;
    20
}
/// $042222: move.b (a0, d0.w), d0
pub fn i_042222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42226;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $042226: andi.w #$3, d0
pub fn i_042226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4222A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04222A: move.w d0, d3
pub fn i_04222a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4222C;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04222C: lea.l $286310.l, a0
pub fn i_04222c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42232;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286310;
    20
}
/// $042232: lsl.w #$2, d0
pub fn i_042232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42234;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $042234: move.w (a0, d0.w), d1
pub fn i_042234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42238;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $042238: move.w $2(a0, d0.w), d2
pub fn i_042238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4223C;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04223C: movem.w d1-d3, -(a7)
pub fn i_04223c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42240;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $042240: lea.l $419cc(pc), a1
pub fn i_042240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42244;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x419CC;
    16
}
/// $042244: jsr $4ae.l
pub fn i_042244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4224A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4224A);
    cpu.pc = 0x4AE;
    16
}
/// $04224A: jsr $5dd02.l
pub fn i_04224a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42250);
    cpu.pc = 0x5DD02;
    16
}
/// $042250: movem.w (a7)+, d1-d3
pub fn i_042250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42254;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $042254: add.w d1, $22(a0)
pub fn i_042254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42258;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $042258: add.w d2, $24(a0)
pub fn i_042258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4225C;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04225C: movem.w d1-d3, -(a7)
pub fn i_04225c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42260;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $042260: lea.l $419fc(pc), a1
pub fn i_042260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42264;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x419FC;
    16
}
/// $042264: jsr $4ae.l
pub fn i_042264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4226A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4226A);
    cpu.pc = 0x4AE;
    16
}
/// $04226A: jsr $5dd02.l
pub fn i_04226a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42270);
    cpu.pc = 0x5DD02;
    16
}
/// $042270: movem.w (a7)+, d1-d3
pub fn i_042270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42274;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $042274: add.w d1, $22(a0)
pub fn i_042274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42278;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $042278: add.w d2, $24(a0)
pub fn i_042278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4227C;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04227C: move.w d3, $9a(a0)
pub fn i_04227c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42280;
    cpu.ir = 0x3143;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042280: rts
pub fn i_042280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ShadeBySine_042282  ($042282, 72 B, enemies/squad/spawn_states_041fb4.s)
/// $042282: move.b $34(a6), d0
pub fn i_042282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42286;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042286: andi.w #$ff, d0
pub fn i_042286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4228A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04228A: cmpi.w #$80, d0
pub fn i_04228a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4228E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $04228E: bcs.w $4229a
pub fn i_04228e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42292;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4229A; return 10; }
    8
}
/// $042292: move.w d0, d1
pub fn i_042292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42294;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042294: move.w #$ff, d0
pub fn i_042294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42298;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042298: sub.w d1, d0
pub fn i_042298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4229A;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04229A: cmpi.w #$40, d0
pub fn i_04229a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4229E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $04229E: bcs.w $422aa
pub fn i_04229e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422A2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x422AA; return 10; }
    8
}
/// $0422A2: move.w d0, d1
pub fn i_0422a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422A4;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0422A4: move.w #$7f, d0
pub fn i_0422a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0422A8: sub.w d1, d0
pub fn i_0422a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422AA;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0422AA: cmpi.w #$18, d0
pub fn i_0422aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422AE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x18, Size::Word, false);
    8
}
/// $0422AE: bcc.w $422b4
pub fn i_0422ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x422B4; return 10; }
    8
}
/// $0422B2: clr.w d0
pub fn i_0422b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422B4;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0422B4: move.w d0, d1
pub fn i_0422b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422B6;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0422B6: lsl.w #$1, d0
pub fn i_0422b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422B8;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0422B8: addi.w #$80, d0
pub fn i_0422b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422BC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0422BC: move.b d0, $32(a6)
pub fn i_0422bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422C0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0422C0: move.b d0, $33(a6)
pub fn i_0422c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422C4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0422C4: add.w d1, $38(a6)
pub fn i_0422c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422C8;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0422C8: rts
pub fn i_0422c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SineToStep_0422CA  ($0422CA, 26 B, enemies/squad/spawn_states_041fb4.s)
/// $0422CA: move.b $34(a6), d0
pub fn i_0422ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422CE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0422CE: andi.w #$ff, d0
pub fn i_0422ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422D2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0422D2: cmpi.w #$80, d0
pub fn i_0422d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422D6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $0422D6: bcs.w $422e2
pub fn i_0422d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x422E2; return 10; }
    8
}
/// $0422DA: move.w d0, d1
pub fn i_0422da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422DC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0422DC: move.w #$ff, d0
pub fn i_0422dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0422E0: sub.w d1, d0
pub fn i_0422e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422E2;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0422E2: lsr.w #$3, d0
pub fn i_0422e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422E4;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
// ===== SetTaskW_0422e4  ($0422E4, 6 B, kernel/task_setters_w.c)
/// $0422E4: move.w d0, $76(a6)
pub fn i_0422e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422E8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0422E8: rts
pub fn i_0422e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadSpawn_DropPair_0422EA  ($0422EA, 58 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0422EA: lea.l $417ac(pc), a1
pub fn i_0422ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x417AC;
    16
}
/// $0422EE: jsr $4ae.l
pub fn i_0422ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x422F4);
    cpu.pc = 0x4AE;
    16
}
/// $0422F4: jsr $5dd02.l
pub fn i_0422f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x422FA);
    cpu.pc = 0x5DD02;
    16
}
/// $0422FA: move.w $76(a6), d0
pub fn i_0422fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422FE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0422FE: lsl.w #$5, d0
pub fn i_0422fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42300;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $042300: subi.w #$40, d0
pub fn i_042300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42304;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042304: sub.w d0, $22(a0)
pub fn i_042304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42308;
    cpu.ir = 0x9168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $042308: lea.l $41850(pc), a1
pub fn i_042308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4230C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41850;
    16
}
/// $04230C: jsr $4ae.l
pub fn i_04230c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42312;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42312);
    cpu.pc = 0x4AE;
    16
}
/// $042312: jsr $5dd02.l
pub fn i_042312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42318);
    cpu.pc = 0x5DD02;
    16
}
/// $042318: addq.w #$8, $22(a0)
pub fn i_042318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4231C;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04231C: move.w $76(a6), $34(a0)
pub fn i_04231c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42322;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $042322: rts
pub fn i_042322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42324;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadSpawn_FxChild_042324  ($042324, 26 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042324: lea.l $78042.l, a1
pub fn i_042324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4232A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78042;
    20
}
/// $04232A: jsr $4ae.l
pub fn i_04232a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42330;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42330);
    cpu.pc = 0x4AE;
    16
}
/// $042330: jsr $5dd02.l
pub fn i_042330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42336);
    cpu.pc = 0x5DD02;
    16
}
/// $042336: move.w #$e000, $38(a0)
pub fn i_042336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4233C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04233C: rts
pub fn i_04233c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4233E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadSpawn_GlideAttacker_04233E  ($04233E, 40 B, enemies/soldiers/melee_guard_0422ea.s)
/// $04233E: move.w #$109b, d0
pub fn i_04233e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42342;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042342: jsr $2352.l
pub fn i_042342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42348);
    cpu.pc = 0x2352;
    16
}
/// $042348: lea.l $4191a(pc), a1
pub fn i_042348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4234C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4191A;
    16
}
/// $04234C: jsr $4ae.l
pub fn i_04234c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42352);
    cpu.pc = 0x4AE;
    16
}
/// $042352: jsr $5dd02.l
pub fn i_042352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42358);
    cpu.pc = 0x5DD02;
    16
}
/// $042358: addi.w #$ffd8, $24(a0)
pub fn i_042358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4235E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFD8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04235E: addi.w #$a, $22(a0)
pub fn i_04235e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42364;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $042364: rts
pub fn i_042364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42366;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadSpawn_FinalPoseChild_042366  ($042366, 42 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042366: lea.l $41bc6(pc), a1
pub fn i_042366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4236A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41BC6;
    16
}
/// $04236A: jsr $4ae.l
pub fn i_04236a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42370);
    cpu.pc = 0x4AE;
    16
}
/// $042370: jsr $5dd02.l
pub fn i_042370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42376);
    cpu.pc = 0x5DD02;
    16
}
/// $042376: addi.w #$ffc8, $22(a0)
pub fn i_042376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4237C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFC8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04237C: addi.w #$14, $24(a0)
pub fn i_04237c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42382;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $042382: move.w $28(a6), $28(a0)
pub fn i_042382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42388;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $042388: move.w $2e(a6), $2e(a0)
pub fn i_042388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4238E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04238E: rts
pub fn i_04238e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42390;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_042390  ($042390, 16 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042390: movea.l $8(a6), a1
pub fn i_042390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42394;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $042394: move.b $10(a6), d0
pub fn i_042394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42398;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042398: cmp.b $10(a1), d0
pub fn i_042398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4239C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04239C: bcs.w $423a6
pub fn i_04239c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423A0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x423A6; return 10; }
    8
}
// ===== ClearXN_0423a0  ($0423A0, 6 B, thunks/ccr_helpers.c)
/// $0423A0: andi.b #$ee, ccr
pub fn i_0423a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423A4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0423A4: rts
pub fn i_0423a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0423a6  ($0423A6, 6 B, thunks/ccr_helpers.c)
/// $0423A6: ori.b #$11, ccr
pub fn i_0423a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423AA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0423AA: rts
pub fn i_0423aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetFacingTest_0423AC  ($0423AC, 12 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0423AC: jsr $56b38.l
pub fn i_0423ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x423B2);
    cpu.pc = 0x56B38;
    16
}
/// $0423B2: move.l a0, d0
pub fn i_0423b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423B4;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0423B4: bne.w $423c2
pub fn i_0423b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x423C2; return 10; }
    8
}
// ===== ClearC_0423b8  ($0423B8, 6 B, thunks/ccr_helpers.c)
/// $0423B8: andi.b #$fe, ccr
pub fn i_0423b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423BC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0423BC: rts
pub fn i_0423bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetFacingTest_Cont_0423BE  ($0423BE, 28 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0423BE: bra.w $423ea
pub fn i_0423be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x423EA;
    10
}
/// $0423C2: move.w $22(a0), d0
pub fn i_0423c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423C6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0423C6: sub.w $22(a6), d0
pub fn i_0423c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423CA;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0423CA: smi.b d0
pub fn i_0423ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423CC;
    cpu.ir = 0x5BC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0423CC: move.b $3a(a6), d1
pub fn i_0423cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423D0;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0423D0: eor.b d0, d1
pub fn i_0423d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423D2;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0423D2: btst.b #$0, d1
pub fn i_0423d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423D6;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0423D6: bne.w $423e4
pub fn i_0423d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x423E4; return 10; }
    8
}
// ===== SetC_0423da  ($0423DA, 6 B, thunks/ccr_helpers.c)
/// $0423DA: ori.b #$1, ccr
pub fn i_0423da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423DE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0423DE: rts
pub fn i_0423de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetFacingTest_Pad_0423E0  ($0423E0, 4 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0423E0: bra.w $423ea
pub fn i_0423e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x423EA;
    10
}
// ===== ClearC_0423e4  ($0423E4, 6 B, thunks/ccr_helpers.c)
/// $0423E4: andi.b #$fe, ccr
pub fn i_0423e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423E8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0423E8: rts
pub fn i_0423e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_000423EA  ($0423EA, 2 B, thunks/trivial_rts.c)
/// $0423EA: rts
pub fn i_0423ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_TrackTarget_0423EC  ($0423EC, 74 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0423EC: jsr $2783a.l
pub fn i_0423ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x423F2);
    cpu.pc = 0x2783A;
    16
}
/// $0423F2: tst.b $79(a6)
pub fn i_0423f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423F6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0423F6: beq.w $42434
pub fn i_0423f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x423FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x42434; return 10; }
    8
}
/// $0423FA: jsr $27f08.l
pub fn i_0423fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42400);
    cpu.pc = 0x27F08;
    16
}
/// $042400: bcc.w $4240c
pub fn i_042400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42404;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4240C; return 10; }
    8
}
/// $042404: move.b d3, $78(a6)
pub fn i_042404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42408;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042408: bra.w $42422
pub fn i_042408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4240C;
    cpu.ir = 0x6000;
    cpu.pc = 0x42422;
    10
}
/// $04240C: cmp.b $78(a6), d0
pub fn i_04240c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42410;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $042410: bne.w $4241c
pub fn i_042410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42414;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4241C; return 10; }
    8
}
/// $042414: move.b d3, $78(a6)
pub fn i_042414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42418;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042418: bra.w $42422
pub fn i_042418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4241C;
    cpu.ir = 0x6000;
    cpu.pc = 0x42422;
    10
}
/// $04241C: move.b #$ff, $78(a6)
pub fn i_04241c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42422;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042422: tst.b $78(a6)
pub fn i_042422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42426;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042426: bne.w $42434
pub fn i_042426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4242A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42434; return 10; }
    8
}
/// $04242A: jsr $28364.l
pub fn i_04242a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42430);
    cpu.pc = 0x28364;
    16
}
/// $042430: scs.b $78(a6)
pub fn i_042430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42434;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042434: rts
pub fn i_042434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42436;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_EngageAndTimers_042436  ($042436, 64 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042436: tst.b $7a(a6)
pub fn i_042436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4243A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04243A: bne.w $4245e
pub fn i_04243a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4243E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4245E; return 10; }
    8
}
/// $04243E: moveq #$0, d0
pub fn i_04243e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42440;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $042440: move.b $98(a6), d0
pub fn i_042440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42444;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042444: lsl.w #$3, d0
pub fn i_042444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42446;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042446: neg.w d0
pub fn i_042446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42448;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042448: addi.w #$140, d0
pub fn i_042448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4244C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x140, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04244C: cmp.w $22(a6), d0
pub fn i_04244c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42450;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $042450: bmi.w $4245a
pub fn i_042450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42454;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4245A; return 10; }
    8
}
/// $042454: move.b #$1, $7a(a6)
pub fn i_042454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4245A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04245A: bra.w $4247a
pub fn i_04245a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4245E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4247A;
    10
}
/// $04245E: move.w $7c(a6), d0
pub fn i_04245e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42462;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042462: beq.w $4246c
pub fn i_042462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42466;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4246C; return 10; }
    8
}
/// $042466: subq.w #$1, d0
pub fn i_042466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42468;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042468: move.w d0, $7c(a6)
pub fn i_042468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4246C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04246C: move.w $7e(a6), d0
pub fn i_04246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42470;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042470: beq.w $4247a
pub fn i_042470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42474;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4247A; return 10; }
    8
}
/// $042474: subq.w #$1, d0
pub fn i_042474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42476;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_042476  ($042476, 6 B, kernel/task_setters_w.c)
/// $042476: move.w d0, $7e(a6)
pub fn i_042476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4247A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04247A: rts
pub fn i_04247a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4247C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_MeleeGate_04247C  ($04247C, 38 B, enemies/soldiers/melee_guard_0422ea.s)
/// $04247C: tst.w $7c(a6)
pub fn i_04247c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42480;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $042480: bne.w $424a8
pub fn i_042480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42484;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x424A8; return 10; }
    8
}
/// $042484: move.w $22(a6), d0
pub fn i_042484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42488;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042488: subi.w #$30, d0
pub fn i_042488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4248C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04248C: cmpi.w #$f0, d0
pub fn i_04248c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42490;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF0, Size::Word, false);
    8
}
/// $042490: bcc.w $424a8
pub fn i_042490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42494;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x424A8; return 10; }
    8
}
/// $042494: move.l $86(a6), d0
pub fn i_042494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42498;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042498: beq.w $424a8
pub fn i_042498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4249C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x424A8; return 10; }
    8
}
/// $04249C: move.b $9a(a6), $7d(a6)
pub fn i_04249c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424A2;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetTaskHandler_0424a2  ($0424A2, 8 B, kernel/task_handlers.c)
/// $0424A2: lea.l $42740(pc), a1
pub fn i_0424a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42740;
    16
}
/// $0424A6: move.l a1, (a6)
pub fn i_0424a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0424A8: rts
pub fn i_0424a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_Think_0424AA  ($0424AA, 104 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0424AA: move.b $106f28.l, d0
pub fn i_0424aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424B0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0424B0: andi.b #$1f, d0
pub fn i_0424b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424B4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0424B4: bne.w $424f0
pub fn i_0424b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x424F0; return 10; }
    8
}
/// $0424B8: jsr $423ac(pc)
pub fn i_0424b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x424BC);
    cpu.pc = 0x423AC;
    16
}
/// $0424BC: bcc.w $424c6
pub fn i_0424bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x424C6; return 10; }
    8
}
/// $0424C0: bchg.b #$0, $3a(a6)
pub fn i_0424c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424C6;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0424C6: move.l a0, $86(a6)
pub fn i_0424c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424CA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0424CA: move.l a0, d0
pub fn i_0424ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424CC;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0424CC: beq.w $424f0
pub fn i_0424cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x424F0; return 10; }
    8
}
/// $0424D0: move.w $24(a6), d0
pub fn i_0424d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424D4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0424D4: sub.w $24(a0), d0
pub fn i_0424d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424D8;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0424D8: cmpi.w #$40, d0
pub fn i_0424d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424DC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $0424DC: bge.w $424ea
pub fn i_0424dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424E0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x424EA; return 10; }
    8
}
/// $0424E0: move.b #$6, $84(a6)
pub fn i_0424e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0424E6: bra.w $424f0
pub fn i_0424e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x424F0;
    10
}
/// $0424EA: move.b #$10, $84(a6)
pub fn i_0424ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0424F0: bsr.w $42512
pub fn i_0424f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x424F4);
    cpu.pc = 0x42512;
    18
}
/// $0424F4: bcc.w $424fe
pub fn i_0424f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x424FE; return 10; }
    8
}
/// $0424F8: lea.l $42794(pc), a1
pub fn i_0424f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42794;
    16
}
/// $0424FC: move.l a1, (a6)
pub fn i_0424fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x424FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0424FE: jsr $8f344.l
pub fn i_0424fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42504);
    cpu.pc = 0x8F344;
    16
}
/// $042504: bcc.w $42510
pub fn i_042504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42508;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42510; return 10; }
    8
}
/// $042508: lea.l $58fc2.l, a1
pub fn i_042508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4250E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FC2;
    20
}
/// $04250E: move.l a1, (a6)
pub fn i_04250e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42510;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042510: rts
pub fn i_042510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42512;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_MeleeRange_042512  ($042512, 40 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042512: jsr $5e0d4.l
pub fn i_042512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42518;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42518);
    cpu.pc = 0x5E0D4;
    16
}
/// $042518: move.w $22(a0), d0
pub fn i_042518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4251C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04251C: move.w $24(a0), d1
pub fn i_04251c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42520;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042520: sub.w $22(a6), d0
pub fn i_042520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42524;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042524: sub.w $24(a6), d1
pub fn i_042524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42528;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042528: cmpi.w #$40, d1
pub fn i_042528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4252C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $04252C: bcc.w $42538
pub fn i_04252c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42530;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42538; return 10; }
    8
}
/// $042530: addi.w #$20, d0
pub fn i_042530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42534;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $042534: cmpi.w #$40, d0
pub fn i_042534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42538;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $042538: rts
pub fn i_042538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4253A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_FaceSelect_04253A  ($04253A, 46 B, enemies/soldiers/melee_guard_0422ea.s)
/// $04253A: jsr $423ac(pc)
pub fn i_04253a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4253E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4253E);
    cpu.pc = 0x423AC;
    16
}
/// $04253E: bcs.w $42554
pub fn i_04253e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42542;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x42554; return 10; }
    8
}
/// $042542: move.l #$2b6596, $74(a6)
pub fn i_042542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4254A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6596;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04254A: move.b #$7, $84(a6)
pub fn i_04254a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42550;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042550: bra.w $42562
pub fn i_042550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42554;
    cpu.ir = 0x6000;
    cpu.pc = 0x42562;
    10
}
/// $042554: move.l #$2b6644, $74(a6)
pub fn i_042554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4255C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6644;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04255C: move.b #$8, $84(a6)
pub fn i_04255c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42562;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042562: move.l a0, $86(a6)
pub fn i_042562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42566;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042566: rts
pub fn i_042566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42568;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_TimerHandoff_042568  ($042568, 14 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042568: tst.w $7e(a6)
pub fn i_042568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4256C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $04256C: bne.w $42574
pub fn i_04256c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42570;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42574; return 10; }
    8
}
/// $042570: move.l $80(a6), (a6)
pub fn i_042570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42574;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $042574: rts
pub fn i_042574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42576;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_TailDespawn_042576  ($042576, 24 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042576: lea.l $28838c.l, a0
pub fn i_042576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4257C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28838C;
    20
}
/// $04257C: jsr $5dd56.l
pub fn i_04257c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42582);
    cpu.pc = 0x5DD56;
    16
}
/// $042582: bcc.w $4258c
pub fn i_042582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42586;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4258C; return 10; }
    8
}
/// $042586: jmp $518.l
pub fn i_042586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4258C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04258C: rts
pub fn i_04258c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4258E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_Handler_04258E  ($04258E, 330 B, enemies/soldiers/melee_guard_0422ea.s)
/// $04258E: move.b #$0, $85(a6)
pub fn i_04258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42594;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042594: move.l #$29b8ee, $70(a6)
pub fn i_042594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4259C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29B8EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04259C: move.l #$29bb4c, $74(a6)
pub fn i_04259c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425A4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29BB4C;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0425A4: move.b #$1, $79(a6)
pub fn i_0425a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0425AA: move.l #$58fc2, $80(a6)
pub fn i_0425aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425B2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x58FC2;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0425B2: bra.w $425e0
pub fn i_0425b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x425E0;
    10
}
/// $0425B6: move.b #$1, $85(a6)
pub fn i_0425b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0425BC: move.l #$288394, $70(a6)
pub fn i_0425bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425C4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x288394;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0425C4: move.l #$2b6596, $74(a6)
pub fn i_0425c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425CC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6596;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0425CC: move.b #$0, $79(a6)
pub fn i_0425cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0425D2: move.l #$58f82, $80(a6)
pub fn i_0425d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425DA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x58F82;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0425DA: bra.w $425e0
pub fn i_0425da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x425E0;
    10
}
/// $0425E0: lea.l $776e2.l, a1
pub fn i_0425e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $0425E6: jsr $4ae.l
pub fn i_0425e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x425EC);
    cpu.pc = 0x4AE;
    16
}
/// $0425EC: move.w #$e, d1
pub fn i_0425ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0425F0: jsr $236e.l
pub fn i_0425f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x425F6);
    cpu.pc = 0x236E;
    16
}
/// $0425F6: jsr $27f60.l
pub fn i_0425f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x425FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x425FC);
    cpu.pc = 0x27F60;
    16
}
/// $0425FC: scc.b $78(a6)
pub fn i_0425fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42600;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042600: clr.b $7a(a6)
pub fn i_042600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42604;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $042604: move.b #$6, $84(a6)
pub fn i_042604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4260A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04260A: moveq #$0, d0
pub fn i_04260a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4260C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04260C: move.b $99(a6), d0
pub fn i_04260c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42610;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042610: bne.w $4261a
pub fn i_042610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42614;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4261A; return 10; }
    8
}
/// $042614: moveq #$ff, d0
pub fn i_042614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42616;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $042616: bra.w $4262c
pub fn i_042616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4261A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4262C;
    10
}
/// $04261A: move.w d0, -(a7)
pub fn i_04261a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4261C;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $04261C: lea.l $2b74ec.l, a0
pub fn i_04261c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42622;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B74EC;
    20
}
/// $042622: jsr $799de.l
pub fn i_042622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42628;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42628);
    cpu.pc = 0x799DE;
    16
}
/// $042628: mulu.w (a7)+, d0
pub fn i_042628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4262A;
    cpu.ir = 0xC0DF;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    74
}
/// $04262A: lsr.l #$8, d0
pub fn i_04262a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4262C;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $04262C: move.w d0, $7c(a6)
pub fn i_04262c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42630;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042630: lea.l $2b746a.l, a0
pub fn i_042630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42636;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B746A;
    20
}
/// $042636: jsr $799de.l
pub fn i_042636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4263C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4263C);
    cpu.pc = 0x799DE;
    16
}
/// $04263C: moveq #$0, d1
pub fn i_04263c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4263E;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04263E: move.b $9a(a6), d1
pub fn i_04263e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42642;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042642: mulu.w d1, d0
pub fn i_042642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42644;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $042644: lsr.l #$8, d0
pub fn i_042644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42646;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $042646: cmpi.w #$100, d0
pub fn i_042646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4264A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $04264A: bcs.w $42652
pub fn i_04264a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4264E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x42652; return 10; }
    8
}
/// $04264E: move.b #$ff, d0
pub fn i_04264e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42652;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $042652: move.b d0, $9a(a6)
pub fn i_042652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42656;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042656: moveq #$0, d0
pub fn i_042656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42658;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $042658: move.b $9b(a6), d0
pub fn i_042658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4265C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04265C: bne.w $42666
pub fn i_04265c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42660;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42666; return 10; }
    8
}
/// $042660: moveq #$ff, d0
pub fn i_042660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42662;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $042662: bra.w $42668
pub fn i_042662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42666;
    cpu.ir = 0x6000;
    cpu.pc = 0x42668;
    10
}
/// $042666: lsl.w #$6, d0
pub fn i_042666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42668;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $042668: move.w d0, $7e(a6)
pub fn i_042668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4266C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04266C: move.w #$8000, d0
pub fn i_04266c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42670;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042670: jsr $28134.l
pub fn i_042670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42676;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42676);
    cpu.pc = 0x28134;
    16
}
/// $042676: andi.w #$ffe3, $38(a6)
pub fn i_042676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4267C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04267C: ori.w #$18, $38(a6)
pub fn i_04267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42682;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $042682: movea.l $70(a6), a0
pub fn i_042682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42686;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $042686: jsr $28cd4.l
pub fn i_042686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4268C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4268C);
    cpu.pc = 0x28CD4;
    16
}
/// $04268C: lea.l $42692(pc), a1
pub fn i_04268c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42690;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42692;
    16
}
/// $042690: move.l a1, (a6)
pub fn i_042690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42692;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042692: bsr.w $42436
pub fn i_042692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42696;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x42696);
    cpu.pc = 0x42436;
    18
}
/// $042696: bsr.w $423ec
pub fn i_042696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4269A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4269A);
    cpu.pc = 0x423EC;
    18
}
/// $04269A: jsr $28d70.l
pub fn i_04269a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x426A0);
    cpu.pc = 0x28D70;
    16
}
/// $0426A0: move.b $85(a6), d0
pub fn i_0426a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426A4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0426A4: cmpi.b #$0, d0
pub fn i_0426a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426A8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0426A8: bne.w $426c2
pub fn i_0426a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x426C2; return 10; }
    8
}
/// $0426AC: bsr.w $424aa
pub fn i_0426ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426B0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x426B0);
    cpu.pc = 0x424AA;
    18
}
/// $0426B0: tst.b $78(a6)
pub fn i_0426b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426B4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0426B4: bne.w $426be
pub fn i_0426b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x426BE; return 10; }
    8
}
/// $0426B8: lea.l $426d8(pc), a1
pub fn i_0426b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x426D8;
    16
}
/// $0426BC: move.l a1, (a6)
pub fn i_0426bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0426BE: bra.w $426c6
pub fn i_0426be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x426C6;
    10
}
/// $0426C2: bsr.w $4253a
pub fn i_0426c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426C6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x426C6);
    cpu.pc = 0x4253A;
    18
}
/// $0426C6: bsr.w $4247c
pub fn i_0426c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426CA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x426CA);
    cpu.pc = 0x4247C;
    18
}
/// $0426CA: bsr.w $42568
pub fn i_0426ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426CE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x426CE);
    cpu.pc = 0x42568;
    18
}
/// $0426CE: jsr $49fd0.l
pub fn i_0426ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x426D4);
    cpu.pc = 0x49FD0;
    16
}
/// $0426D4: bra.w $42576
pub fn i_0426d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== MeleeGuard_IdleState_0426D8  ($0426D8, 56 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0426D8: lea.l $29b956.l, a0
pub fn i_0426d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B956;
    20
}
/// $0426DE: jsr $28cd4.l
pub fn i_0426de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x426E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0426E4: lea.l $426ea(pc), a1
pub fn i_0426e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x426EA;
    16
}
/// $0426E8: move.l a1, (a6)
pub fn i_0426e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0426EA: bsr.w $423ec
pub fn i_0426ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426EE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x426EE);
    cpu.pc = 0x423EC;
    18
}
/// $0426EE: jsr $28d70.l
pub fn i_0426ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x426F4);
    cpu.pc = 0x28D70;
    16
}
/// $0426F4: bcc.w $42706
pub fn i_0426f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42706; return 10; }
    8
}
/// $0426F8: tst.b $78(a6)
pub fn i_0426f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426FC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0426FC: beq.w $42706
pub fn i_0426fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42700;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x42706; return 10; }
    8
}
/// $042700: lea.l $42710(pc), a1
pub fn i_042700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42704;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42710;
    16
}
/// $042704: move.l a1, (a6)
pub fn i_042704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42706;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042706: jsr $49fd0.l
pub fn i_042706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4270C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4270C);
    cpu.pc = 0x49FD0;
    16
}
/// $04270C: bra.w $42576
pub fn i_04270c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42710;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== MeleeGuard_RecoverState_042710  ($042710, 48 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042710: lea.l $288414.l, a0
pub fn i_042710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42716;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x288414;
    20
}
/// $042716: jsr $28cd4.l
pub fn i_042716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4271C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4271C);
    cpu.pc = 0x28CD4;
    16
}
/// $04271C: lea.l $42722(pc), a1
pub fn i_04271c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42720;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42722;
    16
}
/// $042720: move.l a1, (a6)
pub fn i_042720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42722;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042722: bsr.w $423ec
pub fn i_042722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42726;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x42726);
    cpu.pc = 0x423EC;
    18
}
/// $042726: jsr $28d70.l
pub fn i_042726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4272C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4272C);
    cpu.pc = 0x28D70;
    16
}
/// $04272C: bcc.w $42736
pub fn i_04272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42730;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42736; return 10; }
    8
}
/// $042730: lea.l $42682(pc), a1
pub fn i_042730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42734;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42682;
    16
}
/// $042734: move.l a1, (a6)
pub fn i_042734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42736;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042736: jsr $49fd0.l
pub fn i_042736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4273C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4273C);
    cpu.pc = 0x49FD0;
    16
}
/// $04273C: bra.w $42576
pub fn i_04273c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42740;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== MeleeGuard_ApproachState_042740  ($042740, 84 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042740: move.b $84(a6), $5c(a6)
pub fn i_042740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42746;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $042746: movea.l $74(a6), a0
pub fn i_042746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4274A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04274A: jsr $28cd4.l
pub fn i_04274a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42750);
    cpu.pc = 0x28CD4;
    16
}
/// $042750: lea.l $42756(pc), a1
pub fn i_042750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42754;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42756;
    16
}
/// $042754: move.l a1, (a6)
pub fn i_042754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42756;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042756: bsr.w $42436
pub fn i_042756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4275A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4275A);
    cpu.pc = 0x42436;
    18
}
/// $04275A: bsr.w $423ec
pub fn i_04275a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4275E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4275E);
    cpu.pc = 0x423EC;
    18
}
/// $04275E: jsr $28d70.l
pub fn i_04275e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42764);
    cpu.pc = 0x28D70;
    16
}
/// $042764: bcc.w $4276e
pub fn i_042764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42768;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4276E; return 10; }
    8
}
/// $042768: lea.l $42682(pc), a1
pub fn i_042768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4276C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42682;
    16
}
/// $04276C: move.l a1, (a6)
pub fn i_04276c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4276E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04276E: move.b $85(a6), d0
pub fn i_04276e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42772;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042772: cmpi.w #$0, d0
pub fn i_042772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42776;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $042776: bne.w $42788
pub fn i_042776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4277A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42788; return 10; }
    8
}
/// $04277A: bsr.w $42512
pub fn i_04277a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4277E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4277E);
    cpu.pc = 0x42512;
    18
}
/// $04277E: bcc.w $42788
pub fn i_04277e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42782;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42788; return 10; }
    8
}
/// $042782: lea.l $42794(pc), a1
pub fn i_042782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42786;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42794;
    16
}
/// $042786: move.l a1, (a6)
pub fn i_042786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42788;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042788: jsr $49fd0.l
pub fn i_042788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4278E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4278E);
    cpu.pc = 0x49FD0;
    16
}
/// $04278E: bsr.w $42576
pub fn i_04278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42792;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x42792);
    cpu.pc = 0x42576;
    18
}
/// $042792: rts
pub fn i_042792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42794;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_RetreatState_042794  ($042794, 54 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042794: lea.l $2883f4.l, a0
pub fn i_042794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4279A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2883F4;
    20
}
/// $04279A: jsr $28cd4.l
pub fn i_04279a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x427A0);
    cpu.pc = 0x28CD4;
    16
}
/// $0427A0: lea.l $427a6(pc), a1
pub fn i_0427a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x427A6;
    16
}
/// $0427A4: move.l a1, (a6)
pub fn i_0427a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0427A6: bsr.w $423ec
pub fn i_0427a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427AA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x427AA);
    cpu.pc = 0x423EC;
    18
}
/// $0427AA: jsr $28d70.l
pub fn i_0427aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x427B0);
    cpu.pc = 0x28D70;
    16
}
/// $0427B0: bsr.w $42512
pub fn i_0427b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427B4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x427B4);
    cpu.pc = 0x42512;
    18
}
/// $0427B4: bcs.w $427be
pub fn i_0427b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427B8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x427BE; return 10; }
    8
}
/// $0427B8: lea.l $42682(pc), a1
pub fn i_0427b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42682;
    16
}
/// $0427BC: move.l a1, (a6)
pub fn i_0427bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0427BE: jsr $49fd0.l
pub fn i_0427be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x427C4);
    cpu.pc = 0x49FD0;
    16
}
/// $0427C4: bsr.w $42576
pub fn i_0427c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427C8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x427C8);
    cpu.pc = 0x42576;
    18
}
/// $0427C8: rts
pub fn i_0427c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MeleeGuard_DeathToExtern_0427CA  ($0427CA, 48 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0427CA: move.w #$e, d1
pub fn i_0427ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0427CE: jsr $236e.l
pub fn i_0427ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x427D4);
    cpu.pc = 0x236E;
    16
}
/// $0427D4: move.w #$8000, d0
pub fn i_0427d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0427D8: jsr $28134.l
pub fn i_0427d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x427DE);
    cpu.pc = 0x28134;
    16
}
/// $0427DE: andi.w #$ffe3, $38(a6)
pub fn i_0427de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427E4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0427E4: ori.w #$18, $38(a6)
pub fn i_0427e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427EA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0427EA: jsr $2783a.l
pub fn i_0427ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x427F0);
    cpu.pc = 0x2783A;
    16
}
/// $0427F0: lea.l $58f1e.l, a1
pub fn i_0427f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F1E;
    20
}
/// $0427F6: move.l a1, (a6)
pub fn i_0427f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0427F8: rts
pub fn i_0427f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x427FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_TrackTarget_0428C6  ($0428C6, 62 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0428C6: jsr $27f08.l
pub fn i_0428c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x428CC);
    cpu.pc = 0x27F08;
    16
}
/// $0428CC: bcc.w $428d8
pub fn i_0428cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x428D8; return 10; }
    8
}
/// $0428D0: move.b d3, $75(a6)
pub fn i_0428d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428D4;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0428D4: bra.w $428ee
pub fn i_0428d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x428EE;
    10
}
/// $0428D8: cmp.b $75(a6), d0
pub fn i_0428d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428DC;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0428DC: bne.w $428e8
pub fn i_0428dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x428E8; return 10; }
    8
}
/// $0428E0: move.b d3, $75(a6)
pub fn i_0428e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428E4;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0428E4: bra.w $428ee
pub fn i_0428e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x428EE;
    10
}
/// $0428E8: move.b #$ff, $75(a6)
pub fn i_0428e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0428EE: tst.b $75(a6)
pub fn i_0428ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428F2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0428F2: bne.w $42904
pub fn i_0428f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42904; return 10; }
    8
}
/// $0428F6: jsr $28364.l
pub fn i_0428f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x428FC);
    cpu.pc = 0x28364;
    16
}
/// $0428FC: scs.b $75(a6)
pub fn i_0428fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42900;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042900: bra.w $4290a
pub fn i_042900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42904;
    cpu.ir = 0x6000;
    cpu.pc = 0x4290A;
    10
}
// ===== JsrAbsThunk_042904  ($042904, 8 B, thunks/jsr_abs_thunks.c)
/// $042904: jsr $28292.l
pub fn i_042904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4290A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4290A);
    cpu.pc = 0x28292;
    16
}
/// $04290A: rts
pub fn i_04290a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4290C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_Init_04290C  ($04290C, 240 B, enemies/soldiers/charger_states_04290c.s)
/// $04290C: jsr $27f60.l
pub fn i_04290c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42912);
    cpu.pc = 0x27F60;
    16
}
/// $042912: scc.b $75(a6)
pub fn i_042912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42916;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042916: clr.l $7c(a6)
pub fn i_042916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4291A;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $04291A: clr.b $76(a6)
pub fn i_04291a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4291E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04291E: move.w #$e, d1
pub fn i_04291e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42922;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042922: jsr $236e.l
pub fn i_042922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42928);
    cpu.pc = 0x236E;
    16
}
/// $042928: move.w #$8000, d0
pub fn i_042928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4292C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04292C: jsr $28134.l
pub fn i_04292c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42932);
    cpu.pc = 0x28134;
    16
}
/// $042932: andi.w #$ffe3, $38(a6)
pub fn i_042932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42938;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $042938: ori.w #$18, $38(a6)
pub fn i_042938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4293E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04293E: lea.l $776e2.l, a1
pub fn i_04293e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42944;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $042944: jsr $4ae.l
pub fn i_042944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4294A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4294A);
    cpu.pc = 0x4AE;
    16
}
/// $04294A: move.w $22(a6), d0
pub fn i_04294a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4294E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04294E: jsr $440bc.l
pub fn i_04294e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42954);
    cpu.pc = 0x440BC;
    16
}
/// $042954: move.w d0, $70(a6)
pub fn i_042954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42958;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042958: moveq #$0, d0
pub fn i_042958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4295A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04295A: move.b $98(a6), d0
pub fn i_04295a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4295E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04295E: lsl.w #$3, d0
pub fn i_04295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42960;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042960: move.w d0, $72(a6)
pub fn i_042960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42964;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042964: lea.l $2b74ec.l, a0
pub fn i_042964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4296A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B74EC;
    20
}
/// $04296A: jsr $799de.l
pub fn i_04296a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42970);
    cpu.pc = 0x799DE;
    16
}
/// $042970: clr.w d1
pub fn i_042970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42972;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042972: move.b $9a(a6), d1
pub fn i_042972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42976;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042976: mulu.w d0, d1
pub fn i_042976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42978;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $042978: lsr.l #$8, d1
pub fn i_042978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4297A;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $04297A: cmpi.w #$100, d1
pub fn i_04297a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4297E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $04297E: bcs.w $42986
pub fn i_04297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42982;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x42986; return 10; }
    8
}
/// $042982: move.b #$ff, d1
pub fn i_042982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42986;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $042986: move.b d1, $74(a6)
pub fn i_042986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4298A;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04298A: lea.l $2b746a.l, a0
pub fn i_04298a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42990;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B746A;
    20
}
/// $042990: jsr $799de.l
pub fn i_042990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42996);
    cpu.pc = 0x799DE;
    16
}
/// $042996: clr.w d1
pub fn i_042996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42998;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042998: move.b $9b(a6), d1
pub fn i_042998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4299C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04299C: mulu.w d1, d0
pub fn i_04299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4299E;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $04299E: lsr.l #$8, d0
pub fn i_04299e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429A0;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $0429A0: cmpi.w #$100, d0
pub fn i_0429a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429A4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $0429A4: bcs.w $429ac
pub fn i_0429a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x429AC; return 10; }
    8
}
/// $0429A8: move.b #$ff, d0
pub fn i_0429a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429AC;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0429AC: move.b d0, $9b(a6)
pub fn i_0429ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429B0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429B0: jsr $27f08.l
pub fn i_0429b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x429B6);
    cpu.pc = 0x27F08;
    16
}
/// $0429B6: bcc.w $429c2
pub fn i_0429b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x429C2; return 10; }
    8
}
/// $0429BA: move.b d3, $75(a6)
pub fn i_0429ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429BE;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429BE: bra.w $429d8
pub fn i_0429be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x429D8;
    10
}
/// $0429C2: cmp.b $75(a6), d0
pub fn i_0429c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429C6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0429C6: bne.w $429d2
pub fn i_0429c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x429D2; return 10; }
    8
}
/// $0429CA: move.b d3, $75(a6)
pub fn i_0429ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429CE;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429CE: bra.w $429d8
pub fn i_0429ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x429D8;
    10
}
/// $0429D2: move.b #$ff, $75(a6)
pub fn i_0429d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429D8: move.w $22(a6), d1
pub fn i_0429d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429DC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0429DC: cmpi.w #$a0, d1
pub fn i_0429dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429E0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0429E0: bge.w $429ee
pub fn i_0429e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429E4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x429EE; return 10; }
    8
}
/// $0429E4: bset.b #$0, $3a(a6)
pub fn i_0429e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429EA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0429EA: bra.w $429f4
pub fn i_0429ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x429F4;
    10
}
/// $0429EE: bclr.b #$0, $3a(a6)
pub fn i_0429ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0429F4: tst.w $72(a6)
pub fn i_0429f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429F8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0429F8: bne.w $42a04
pub fn i_0429f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A04; return 10; }
    8
}
// ===== SetTaskHandler_0429fc  ($0429FC, 8 B, kernel/task_handlers.c)
/// $0429FC: lea.l $42acc(pc), a1
pub fn i_0429fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42ACC;
    16
}
/// $042A00: move.l a1, (a6)
pub fn i_042a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A02: rts
pub fn i_042a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_InitDispatch_042A04  ($042A04, 8 B, enemies/soldiers/charger_states_04290c.s)
/// $042A04: move.b $9d(a6), d0
pub fn i_042a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042A08: bne.w $42a14
pub fn i_042a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A14; return 10; }
    8
}
// ===== SetTaskHandler_042a0c  ($042A0C, 8 B, kernel/task_handlers.c)
/// $042A0C: lea.l $42a6e(pc), a1
pub fn i_042a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A6E;
    16
}
/// $042A10: move.l a1, (a6)
pub fn i_042a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A12: rts
pub fn i_042a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_InitDispatch2_042A14  ($042A14, 6 B, enemies/soldiers/charger_states_04290c.s)
/// $042A14: subq.b #$1, d0
pub fn i_042a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A16;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $042A16: bne.w $42a22
pub fn i_042a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A22; return 10; }
    8
}
// ===== SetTaskHandler_042a1a  ($042A1A, 8 B, kernel/task_handlers.c)
/// $042A1A: lea.l $42a56(pc), a1
pub fn i_042a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A56;
    16
}
/// $042A1E: move.l a1, (a6)
pub fn i_042a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A20: rts
pub fn i_042a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_042a22  ($042A22, 8 B, kernel/task_handlers.c)
/// $042A22: lea.l $42a44(pc), a1
pub fn i_042a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A44;
    16
}
/// $042A26: move.l a1, (a6)
pub fn i_042a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A28: rts
pub fn i_042a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_ColumnDeltaTest_042A2A  ($042A2A, 162 B, enemies/soldiers/charger_states_04290c.s)
/// $042A2A: move.w $22(a6), d0
pub fn i_042a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042A2E: jsr $440bc.l
pub fn i_042a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A34);
    cpu.pc = 0x440BC;
    16
}
/// $042A34: sub.w $70(a6), d0
pub fn i_042a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A38;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042A38: bpl.w $42a3e
pub fn i_042a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A3C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x42A3E; return 10; }
    8
}
/// $042A3C: neg.w d0
pub fn i_042a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A3E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042A3E: cmp.w $72(a6), d0
pub fn i_042a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A42;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $042A42: rts
pub fn i_042a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $042A44: lea.l $427fa(pc), a0
pub fn i_042a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A48;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x427FA;
    16
}
/// $042A48: jsr $28cd4.l
pub fn i_042a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A4E);
    cpu.pc = 0x28CD4;
    16
}
/// $042A4E: move.w #$100, d0
pub fn i_042a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042A52: bra.w $42a86
pub fn i_042a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A56;
    cpu.ir = 0x6000;
    cpu.pc = 0x42A86;
    10
}
/// $042A56: move.b #$ff, $76(a6)
pub fn i_042a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042A5C: lea.l $4287e(pc), a0
pub fn i_042a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A60;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4287E;
    16
}
/// $042A60: jsr $28cd4.l
pub fn i_042a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A66);
    cpu.pc = 0x28CD4;
    16
}
/// $042A66: move.w #$100, d0
pub fn i_042a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042A6A: bra.w $42a86
pub fn i_042a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x42A86;
    10
}
/// $042A6E: lea.l $29b744.l, a0
pub fn i_042a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $042A74: jsr $28cd4.l
pub fn i_042a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A7A);
    cpu.pc = 0x28CD4;
    16
}
/// $042A7A: lea.l $2b75d0.l, a0
pub fn i_042a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B75D0;
    20
}
/// $042A80: jsr $799de.l
pub fn i_042a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A86);
    cpu.pc = 0x799DE;
    16
}
/// $042A86: btst.b #$0, $3a(a6)
pub fn i_042a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A8C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $042A8C: bne.w $42a92
pub fn i_042a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A90;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A92; return 10; }
    8
}
/// $042A90: neg.w d0
pub fn i_042a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A92;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042A92: move.w d0, $28(a6)
pub fn i_042a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A96;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042A96: lea.l $42a9c(pc), a1
pub fn i_042a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A9C;
    16
}
/// $042A9A: move.l a1, (a6)
pub fn i_042a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A9C: jsr $428c6(pc)
pub fn i_042a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x42AA0);
    cpu.pc = 0x428C6;
    16
}
/// $042AA0: jsr $28d70.l
pub fn i_042aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AA6);
    cpu.pc = 0x28D70;
    16
}
/// $042AA6: bsr.b $42a2a
pub fn i_042aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AA8;
    cpu.ir = 0x6182;
    cpu.push32(bus, 0x42AA8);
    cpu.pc = 0x42A2A;
    18
}
/// $042AA8: bcs.w $42ab2
pub fn i_042aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x42AB2; return 10; }
    8
}
/// $042AAC: lea.l $42acc(pc), a1
pub fn i_042aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42ACC;
    16
}
/// $042AB0: move.l a1, (a6)
pub fn i_042ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042AB2: jsr $574e8.l
pub fn i_042ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AB8);
    cpu.pc = 0x574E8;
    16
}
/// $042AB8: bcc.w $42ac2
pub fn i_042ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42ABC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42AC2; return 10; }
    8
}
/// $042ABC: lea.l $42b5c(pc), a1
pub fn i_042abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42B5C;
    16
}
/// $042AC0: move.l a1, (a6)
pub fn i_042ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042AC2: jsr $49fd0.l
pub fn i_042ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AC8);
    cpu.pc = 0x49FD0;
    16
}
/// $042AC8: bra.w $42576
pub fn i_042ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42ACC;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== Charger_IdleState_042ACC  ($042ACC, 144 B, enemies/soldiers/charger_states_04290c.s)
/// $042ACC: tst.b $76(a6)
pub fn i_042acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AD0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042AD0: bne.w $42ae4
pub fn i_042ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42AE4; return 10; }
    8
}
/// $042AD4: lea.l $29b816.l, a0
pub fn i_042ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42ADA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $042ADA: jsr $28cd4.l
pub fn i_042ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AE0);
    cpu.pc = 0x28CD4;
    16
}
/// $042AE0: bra.w $42af0
pub fn i_042ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x42AF0;
    10
}
/// $042AE4: lea.l $2b5c10.l, a0
pub fn i_042ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C10;
    20
}
/// $042AEA: jsr $28cd4.l
pub fn i_042aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AF0);
    cpu.pc = 0x28CD4;
    16
}
/// $042AF0: jsr $283ca.l
pub fn i_042af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AF6);
    cpu.pc = 0x283CA;
    16
}
/// $042AF6: clr.w $28(a6)
pub fn i_042af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AFA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $042AFA: lea.l $42b00(pc), a1
pub fn i_042afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42B00;
    16
}
/// $042AFE: move.l a1, (a6)
pub fn i_042afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042B00: jsr $423ac(pc)
pub fn i_042b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x42B04);
    cpu.pc = 0x423AC;
    16
}
/// $042B04: bcc.w $42b0e
pub fn i_042b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42B0E; return 10; }
    8
}
/// $042B08: bchg.b #$0, $3a(a6)
pub fn i_042b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B0E;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $042B0E: move.l a0, $7c(a6)
pub fn i_042b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B12;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042B12: bsr.w $428c6
pub fn i_042b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B16;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x42B16);
    cpu.pc = 0x428C6;
    18
}
/// $042B16: jsr $28d70.l
pub fn i_042b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42B1C);
    cpu.pc = 0x28D70;
    16
}
/// $042B1C: tst.b $74(a6)
pub fn i_042b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B20;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042B20: bne.w $42b36
pub fn i_042b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B24;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42B36; return 10; }
    8
}
/// $042B24: move.l $7c(a6), d0
pub fn i_042b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B28;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042B28: beq.w $42b32
pub fn i_042b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B2C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x42B32; return 10; }
    8
}
/// $042B2C: lea.l $42ba6(pc), a1
pub fn i_042b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42BA6;
    16
}
/// $042B30: move.l a1, (a6)
pub fn i_042b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042B32: bra.w $42b3a
pub fn i_042b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B36;
    cpu.ir = 0x6000;
    cpu.pc = 0x42B3A;
    10
}
/// $042B36: subq.b #$1, $74(a6)
pub fn i_042b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B3A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $042B3A: tst.b $76(a6)
pub fn i_042b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B3E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042B3E: bne.w $42b52
pub fn i_042b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42B52; return 10; }
    8
}
/// $042B42: jsr $574e8.l
pub fn i_042b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42B48);
    cpu.pc = 0x574E8;
    16
}
/// $042B48: bcc.w $42b52
pub fn i_042b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42B52; return 10; }
    8
}
/// $042B4C: lea.l $42b5c(pc), a1
pub fn i_042b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42B5C;
    16
}
/// $042B50: move.l a1, (a6)
pub fn i_042b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042B52: jsr $49fd0.l
pub fn i_042b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42B58);
    cpu.pc = 0x49FD0;
    16
}
/// $042B58: bra.w $42576
pub fn i_042b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== Charger_HurtState_042B5C  ($042B5C, 74 B, enemies/soldiers/charger_states_04290c.s)
/// $042B5C: clr.w $28(a6)
pub fn i_042b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B60;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $042B60: lea.l $2b70d2.l, a0
pub fn i_042b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B70D2;
    20
}
/// $042B66: jsr $28cd4.l
pub fn i_042b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42B6C);
    cpu.pc = 0x28CD4;
    16
}
/// $042B6C: lea.l $57494.l, a0
pub fn i_042b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x57494;
    20
}
/// $042B72: move.l a0, $4c(a6)
pub fn i_042b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B76;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042B76: jsr $283ca.l
pub fn i_042b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42B7C);
    cpu.pc = 0x283CA;
    16
}
/// $042B7C: jsr $283ca.l
pub fn i_042b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42B82);
    cpu.pc = 0x283CA;
    16
}
/// $042B82: lea.l $42b88(pc), a1
pub fn i_042b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42B88;
    16
}
/// $042B86: move.l a1, (a6)
pub fn i_042b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042B88: bsr.w $428c6
pub fn i_042b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B8C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x42B8C);
    cpu.pc = 0x428C6;
    18
}
/// $042B8C: jsr $28d70.l
pub fn i_042b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42B92);
    cpu.pc = 0x28D70;
    16
}
/// $042B92: bcc.w $42b9c
pub fn i_042b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42B9C; return 10; }
    8
}
/// $042B96: lea.l $42acc(pc), a1
pub fn i_042b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42ACC;
    16
}
/// $042B9A: move.l a1, (a6)
pub fn i_042b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42B9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042B9C: jsr $49fd0.l
pub fn i_042b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42BA2);
    cpu.pc = 0x49FD0;
    16
}
/// $042BA2: bra.w $42576
pub fn i_042ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== Charger_AttackState_042BA6  ($042BA6, 292 B, enemies/soldiers/charger_states_04290c.s)
/// $042BA6: tst.b $99(a6)
pub fn i_042ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BAA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042BAA: beq.w $42cb0
pub fn i_042baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x42CB0; return 10; }
    8
}
/// $042BAE: tst.b $76(a6)
pub fn i_042bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BB2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042BB2: bne.w $42bd0
pub fn i_042bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42BD0; return 10; }
    8
}
/// $042BB6: cmpi.b #$4, $9c(a6)
pub fn i_042bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BBC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $042BBC: bcs.w $42be6
pub fn i_042bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BC0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x42BE6; return 10; }
    8
}
/// $042BC0: move.l #$42be6, $78(a6)
pub fn i_042bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BC8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x42BE6;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042BC8: bra.w $42d02
pub fn i_042bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x42D02;
    10
}
/// $042BD0: cmpi.b #$4, $9c(a6)
pub fn i_042bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BD6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $042BD6: bcc.w $42be6
pub fn i_042bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42BE6; return 10; }
    8
}
/// $042BDA: move.l #$42be6, $78(a6)
pub fn i_042bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BE2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x42BE6;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042BE2: bra.w $42cca
pub fn i_042be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x42CCA;
    10
}
/// $042BE6: move.b $9c(a6), d0
pub fn i_042be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BEA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042BEA: bne.w $42c04
pub fn i_042bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42C04; return 10; }
    8
}
/// $042BEE: lea.l $29ba70.l, a0
pub fn i_042bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BA70;
    20
}
/// $042BF4: jsr $28cd4.l
pub fn i_042bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42BFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42BFA);
    cpu.pc = 0x28CD4;
    16
}
/// $042BFA: move.b #$5, $5c(a6)
pub fn i_042bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042C00: bra.w $42c58
pub fn i_042c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C04;
    cpu.ir = 0x6000;
    cpu.pc = 0x42C58;
    10
}
/// $042C04: subq.b #$1, d0
pub fn i_042c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C06;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $042C06: bne.w $42c20
pub fn i_042c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42C20; return 10; }
    8
}
/// $042C0A: move.b #$ff, $76(a6)
pub fn i_042c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C10;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042C10: lea.l $29b9ce.l, a0
pub fn i_042c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B9CE;
    20
}
/// $042C16: jsr $28cd4.l
pub fn i_042c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42C1C);
    cpu.pc = 0x28CD4;
    16
}
/// $042C1C: bra.w $42c58
pub fn i_042c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C20;
    cpu.ir = 0x6000;
    cpu.pc = 0x42C58;
    10
}
/// $042C20: subq.b #$1, d0
pub fn i_042c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C22;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $042C22: bne.w $42c36
pub fn i_042c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42C36; return 10; }
    8
}
/// $042C26: lea.l $29bbec.l, a0
pub fn i_042c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BBEC;
    20
}
/// $042C2C: jsr $28cd4.l
pub fn i_042c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42C32);
    cpu.pc = 0x28CD4;
    16
}
/// $042C32: bra.w $42c58
pub fn i_042c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C36;
    cpu.ir = 0x6000;
    cpu.pc = 0x42C58;
    10
}
/// $042C36: subq.b #$1, d0
pub fn i_042c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C38;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $042C38: bne.w $42c4c
pub fn i_042c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42C4C; return 10; }
    8
}
/// $042C3C: lea.l $29bc64.l, a0
pub fn i_042c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BC64;
    20
}
/// $042C42: jsr $28cd4.l
pub fn i_042c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42C48);
    cpu.pc = 0x28CD4;
    16
}
/// $042C48: bra.w $42c58
pub fn i_042c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x42C58;
    10
}
/// $042C4C: lea.l $29bb4c.l, a0
pub fn i_042c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C52;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BB4C;
    20
}
/// $042C52: jsr $28cd4.l
pub fn i_042c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42C58);
    cpu.pc = 0x28CD4;
    16
}
/// $042C58: move.l $7c(a6), d0
pub fn i_042c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C5C;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042C5C: jsr $57044.l
pub fn i_042c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42C62);
    cpu.pc = 0x57044;
    16
}
/// $042C62: clr.w $28(a6)
pub fn i_042c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C66;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $042C66: lea.l $42c6c(pc), a1
pub fn i_042c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42C6C;
    16
}
/// $042C6A: move.l a1, (a6)
pub fn i_042c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042C6C: tst.b $74(a6)
pub fn i_042c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C70;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042C70: beq.w $42c78
pub fn i_042c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x42C78; return 10; }
    8
}
/// $042C74: subq.b #$1, $74(a6)
pub fn i_042c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C78;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $042C78: bsr.w $428c6
pub fn i_042c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C7C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x42C7C);
    cpu.pc = 0x428C6;
    18
}
/// $042C7C: jsr $28d70.l
pub fn i_042c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42C82);
    cpu.pc = 0x28D70;
    16
}
/// $042C82: bcc.w $42ca6
pub fn i_042c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42CA6; return 10; }
    8
}
/// $042C86: moveq #$0, d0
pub fn i_042c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C88;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $042C88: move.b $9b(a6), $74(a6)
pub fn i_042c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C8E;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad4, v3 as u8);
    20
}
/// $042C8E: subq.b #$1, $99(a6)
pub fn i_042c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C92;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $042C92: bne.w $42ca0
pub fn i_042c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42CA0; return 10; }
    8
}
/// $042C96: lea.l $42cb0(pc), a1
pub fn i_042c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42CB0;
    16
}
/// $042C9A: move.l a1, (a6)
pub fn i_042c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042C9C: bra.w $42ca6
pub fn i_042c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x42CA6;
    10
}
/// $042CA0: lea.l $42acc(pc), a1
pub fn i_042ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42ACC;
    16
}
/// $042CA4: move.l a1, (a6)
pub fn i_042ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042CA6: jsr $49fd0.l
pub fn i_042ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42CAC);
    cpu.pc = 0x49FD0;
    16
}
/// $042CAC: bra.w $42576
pub fn i_042cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
/// $042CB0: tst.b $76(a6)
pub fn i_042cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CB4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042CB4: beq.w $42cc4
pub fn i_042cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CB8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x42CC4; return 10; }
    8
}
/// $042CB8: move.l #$59042, $78(a6)
pub fn i_042cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CC0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x59042;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042CC0: bra.w $42cca
pub fn i_042cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x42CCA;
    10
}
/// $042CC4: jmp $59042.l
pub fn i_042cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CCA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x59042;
    8
}
// ===== Charger_WindupA_042CCA  ($042CCA, 56 B, enemies/soldiers/charger_states_04290c.s)
/// $042CCA: lea.l $29bd84.l, a0
pub fn i_042cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BD84;
    20
}
/// $042CD0: jsr $28cd4.l
pub fn i_042cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42CD6);
    cpu.pc = 0x28CD4;
    16
}
/// $042CD6: clr.w $28(a6)
pub fn i_042cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CDA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $042CDA: lea.l $42ce0(pc), a1
pub fn i_042cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42CE0;
    16
}
/// $042CDE: move.l a1, (a6)
pub fn i_042cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042CE0: bsr.w $428c6
pub fn i_042ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CE4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x42CE4);
    cpu.pc = 0x428C6;
    18
}
/// $042CE4: jsr $28d70.l
pub fn i_042ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42CEA);
    cpu.pc = 0x28D70;
    16
}
/// $042CEA: bcc.w $42cf8
pub fn i_042cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42CF8; return 10; }
    8
}
/// $042CEE: move.l $78(a6), d0
pub fn i_042cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CF2;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042CF2: move.l d0, (a6)
pub fn i_042cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CF4;
    cpu.ir = 0x2C80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042CF4: clr.b $76(a6)
pub fn i_042cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CF8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $042CF8: jsr $49fd0.l
pub fn i_042cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42CFE);
    cpu.pc = 0x49FD0;
    16
}
/// $042CFE: bra.w $42576
pub fn i_042cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D02;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== Charger_WindupB_042D02  ($042D02, 58 B, enemies/soldiers/charger_states_04290c.s)
/// $042D02: lea.l $29b956.l, a0
pub fn i_042d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B956;
    20
}
/// $042D08: jsr $28cd4.l
pub fn i_042d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D0E);
    cpu.pc = 0x28CD4;
    16
}
/// $042D0E: clr.w $28(a6)
pub fn i_042d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D12;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $042D12: lea.l $42d18(pc), a1
pub fn i_042d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42D18;
    16
}
/// $042D16: move.l a1, (a6)
pub fn i_042d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042D18: jsr $428c6(pc)
pub fn i_042d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x42D1C);
    cpu.pc = 0x428C6;
    16
}
/// $042D1C: jsr $28d70.l
pub fn i_042d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D22);
    cpu.pc = 0x28D70;
    16
}
/// $042D22: bcc.w $42d32
pub fn i_042d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42D32; return 10; }
    8
}
/// $042D26: move.l $78(a6), d0
pub fn i_042d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D2A;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042D2A: move.l d0, (a6)
pub fn i_042d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D2C;
    cpu.ir = 0x2C80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042D2C: move.b #$ff, $76(a6)
pub fn i_042d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042D32: jsr $49fd0.l
pub fn i_042d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D38);
    cpu.pc = 0x49FD0;
    16
}
/// $042D38: bra.w $42576
pub fn i_042d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== Skirmisher_Init_042D3C  ($042D3C, 84 B, enemies/soldiers/charger_states_04290c.s)
/// $042D3C: bset.b #$0, $3a(a6)
pub fn i_042d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D42;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $042D42: move.w #$e, d1
pub fn i_042d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D46;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042D46: jsr $236e.l
pub fn i_042d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D4C);
    cpu.pc = 0x236E;
    16
}
/// $042D4C: jsr $27f60.l
pub fn i_042d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D52);
    cpu.pc = 0x27F60;
    16
}
/// $042D52: scc.b $70(a6)
pub fn i_042d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D56;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042D56: lea.l $42d5c(pc), a1
pub fn i_042d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42D5C;
    16
}
/// $042D5A: move.l a1, (a6)
pub fn i_042d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042D5C: lea.l $2b76b4.l, a0
pub fn i_042d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B76B4;
    20
}
/// $042D62: jsr $799de.l
pub fn i_042d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D68);
    cpu.pc = 0x799DE;
    16
}
/// $042D68: btst.b #$0, $3a(a6)
pub fn i_042d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D6E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $042D6E: bne.w $42d74
pub fn i_042d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42D74; return 10; }
    8
}
/// $042D72: neg.w d0
pub fn i_042d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D74;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042D74: move.w d0, $28(a6)
pub fn i_042d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D78;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042D78: lea.l $29b744.l, a0
pub fn i_042d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $042D7E: jsr $28cd4.l
pub fn i_042d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D84);
    cpu.pc = 0x28CD4;
    16
}
/// $042D84: jsr $283ca.l
pub fn i_042d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D8A);
    cpu.pc = 0x283CA;
    16
}
/// $042D8A: lea.l $42d90(pc), a1
pub fn i_042d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42D90;
    16
}
/// $042D8E: move.l a1, (a6)
pub fn i_042d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Skirmisher_Main_042D90  ($042D90, 106 B, enemies/soldiers/charger_states_04290c.s)
/// $042D90: jsr $2783a.l
pub fn i_042d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D96);
    cpu.pc = 0x2783A;
    16
}
/// $042D96: jsr $27f08.l
pub fn i_042d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42D9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42D9C);
    cpu.pc = 0x27F08;
    16
}
/// $042D9C: bcc.w $42da8
pub fn i_042d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42DA8; return 10; }
    8
}
/// $042DA0: move.b d3, $70(a6)
pub fn i_042da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DA4;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042DA4: bra.w $42dbe
pub fn i_042da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x42DBE;
    10
}
/// $042DA8: cmp.b $70(a6), d0
pub fn i_042da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DAC;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $042DAC: bne.w $42db8
pub fn i_042dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DB0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42DB8; return 10; }
    8
}
/// $042DB0: move.b d3, $70(a6)
pub fn i_042db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DB4;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042DB4: bra.w $42dbe
pub fn i_042db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x42DBE;
    10
}
/// $042DB8: move.b #$ff, $70(a6)
pub fn i_042db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042DBE: tst.b $70(a6)
pub fn i_042dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DC2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042DC2: bne.w $42dd4
pub fn i_042dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42DD4; return 10; }
    8
}
/// $042DC6: jsr $28364.l
pub fn i_042dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42DCC);
    cpu.pc = 0x28364;
    16
}
/// $042DCC: scs.b $70(a6)
pub fn i_042dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DD0;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042DD0: bra.w $42dda
pub fn i_042dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DD4;
    cpu.ir = 0x6000;
    cpu.pc = 0x42DDA;
    10
}
/// $042DD4: jsr $28292.l
pub fn i_042dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42DDA);
    cpu.pc = 0x28292;
    16
}
/// $042DDA: jsr $574e8.l
pub fn i_042dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42DE0);
    cpu.pc = 0x574E8;
    16
}
/// $042DE0: bcc.w $42dea
pub fn i_042de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42DEA; return 10; }
    8
}
/// $042DE4: lea.l $42dfa(pc), a1
pub fn i_042de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42DFA;
    16
}
/// $042DE8: move.l a1, (a6)
pub fn i_042de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042DEA: jsr $28d70.l
pub fn i_042dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42DF0);
    cpu.pc = 0x28D70;
    16
}
/// $042DF0: jsr $49fd0.l
pub fn i_042df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42DF6);
    cpu.pc = 0x49FD0;
    16
}
/// $042DF6: bra.w $42576
pub fn i_042df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== Skirmisher_Hurt_042DFA  ($042DFA, 144 B, enemies/soldiers/charger_states_04290c.s)
/// $042DFA: clr.w $28(a6)
pub fn i_042dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42DFE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $042DFE: lea.l $2b70d2.l, a0
pub fn i_042dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B70D2;
    20
}
/// $042E04: jsr $28cd4.l
pub fn i_042e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E0A);
    cpu.pc = 0x28CD4;
    16
}
/// $042E0A: lea.l $57494.l, a0
pub fn i_042e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x57494;
    20
}
/// $042E10: move.l a0, $4c(a6)
pub fn i_042e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E14;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $042E14: jsr $283ca.l
pub fn i_042e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E1A);
    cpu.pc = 0x283CA;
    16
}
/// $042E1A: jsr $283ca.l
pub fn i_042e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E20);
    cpu.pc = 0x283CA;
    16
}
/// $042E20: lea.l $42e26(pc), a1
pub fn i_042e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42E26;
    16
}
/// $042E24: move.l a1, (a6)
pub fn i_042e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042E26: jsr $2783a.l
pub fn i_042e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E2C);
    cpu.pc = 0x2783A;
    16
}
/// $042E2C: jsr $27f08.l
pub fn i_042e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E32);
    cpu.pc = 0x27F08;
    16
}
/// $042E32: bcc.w $42e3e
pub fn i_042e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42E3E; return 10; }
    8
}
/// $042E36: move.b d3, $70(a6)
pub fn i_042e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E3A;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042E3A: bra.w $42e54
pub fn i_042e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x42E54;
    10
}
/// $042E3E: cmp.b $70(a6), d0
pub fn i_042e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E42;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $042E42: bne.w $42e4e
pub fn i_042e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42E4E; return 10; }
    8
}
/// $042E46: move.b d3, $70(a6)
pub fn i_042e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E4A;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042E4A: bra.w $42e54
pub fn i_042e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x42E54;
    10
}
/// $042E4E: move.b #$ff, $70(a6)
pub fn i_042e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E54;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042E54: tst.b $70(a6)
pub fn i_042e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E58;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $042E58: bne.w $42e6a
pub fn i_042e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42E6A; return 10; }
    8
}
/// $042E5C: jsr $28364.l
pub fn i_042e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E62);
    cpu.pc = 0x28364;
    16
}
/// $042E62: scs.b $70(a6)
pub fn i_042e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E66;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042E66: bra.w $42e70
pub fn i_042e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x42E70;
    10
}
/// $042E6A: jsr $28292.l
pub fn i_042e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E70);
    cpu.pc = 0x28292;
    16
}
/// $042E70: jsr $28d70.l
pub fn i_042e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E76);
    cpu.pc = 0x28D70;
    16
}
/// $042E76: bcc.w $42e80
pub fn i_042e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42E80; return 10; }
    8
}
/// $042E7A: lea.l $42d5c(pc), a1
pub fn i_042e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42D5C;
    16
}
/// $042E7E: move.l a1, (a6)
pub fn i_042e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042E80: jsr $49fd0.l
pub fn i_042e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42E86);
    cpu.pc = 0x49FD0;
    16
}
/// $042E86: bra.w $42576
pub fn i_042e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42E8A;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== TrackTargetLatch75_04345E  ($04345E, 42 B, enemies/soldiers/heading16_042e8a.s)
/// $04345E: jsr $27f08.l
pub fn i_04345e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43464);
    cpu.pc = 0x27F08;
    16
}
/// $043464: bcs.w $43470
pub fn i_043464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43468;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x43470; return 10; }
    8
}
/// $043468: move.b d3, $75(a6)
pub fn i_043468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4346C;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04346C: bra.w $43486
pub fn i_04346c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43470;
    cpu.ir = 0x6000;
    cpu.pc = 0x43486;
    10
}
/// $043470: cmp.b $75(a6), d0
pub fn i_043470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43474;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $043474: bne.w $43480
pub fn i_043474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43478;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43480; return 10; }
    8
}
/// $043478: move.b d3, $75(a6)
pub fn i_043478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4347C;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04347C: bra.w $43486
pub fn i_04347c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43480;
    cpu.ir = 0x6000;
    cpu.pc = 0x43486;
    10
}
/// $043480: move.b #$ff, $75(a6)
pub fn i_043480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43486;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $043486: rts
pub fn i_043486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43488;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TrackTargetLatch78_043488  ($043488, 42 B, enemies/soldiers/heading16_042e8a.s)
/// $043488: jsr $27f08.l
pub fn i_043488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4348E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4348E);
    cpu.pc = 0x27F08;
    16
}
/// $04348E: bcs.w $4349a
pub fn i_04348e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43492;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4349A; return 10; }
    8
}
/// $043492: move.b d3, $78(a6)
pub fn i_043492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43496;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $043496: bra.w $434b0
pub fn i_043496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4349A;
    cpu.ir = 0x6000;
    cpu.pc = 0x434B0;
    10
}
/// $04349A: cmp.b $78(a6), d0
pub fn i_04349a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4349E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04349E: bne.w $434aa
pub fn i_04349e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x434AA; return 10; }
    8
}
/// $0434A2: move.b d3, $78(a6)
pub fn i_0434a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434A6;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0434A6: bra.w $434b0
pub fn i_0434a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x434B0;
    10
}
/// $0434AA: move.b #$ff, $78(a6)
pub fn i_0434aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434B0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0434B0: rts
pub fn i_0434b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_0434B2  ($0434B2, 16 B, enemies/soldiers/heading16_042e8a.s)
/// $0434B2: movea.l $8(a6), a1
pub fn i_0434b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434B6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0434B6: move.b $10(a6), d0
pub fn i_0434b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434BA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0434BA: cmp.b $10(a1), d0
pub fn i_0434ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434BE;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0434BE: bcs.w $434c8
pub fn i_0434be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434C2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x434C8; return 10; }
    8
}
// ===== ClearXN_0434c2  ($0434C2, 6 B, thunks/ccr_helpers.c)
/// $0434C2: andi.b #$ee, ccr
pub fn i_0434c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434C6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0434C6: rts
pub fn i_0434c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0434c8  ($0434C8, 6 B, thunks/ccr_helpers.c)
/// $0434C8: ori.b #$11, ccr
pub fn i_0434c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434CC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0434CC: rts
pub fn i_0434cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_000434CE  ($0434CE, 2 B, thunks/trivial_rts.c)
/// $0434CE: rts
pub fn i_0434ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camera_PresetVelFast_0434D0  ($0434D0, 12 B, video/camera_setters.c)
/// $0434D0: move.l #$100000, $10817e.l
pub fn i_0434d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434DA;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x100000;
    bus.write32(0x10817E, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0434DA: rts
pub fn i_0434da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_000434DC  ($0434DC, 2 B, thunks/trivial_rts.c)
/// $0434DC: rts
pub fn i_0434dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camera_PresetVelSlow_0434DE  ($0434DE, 12 B, video/camera_setters.c)
/// $0434DE: move.l #$40000, $10817e.l
pub fn i_0434de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434E8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x40000;
    bus.write32(0x10817E, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0434E8: rts
pub fn i_0434e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camera_ResetSmoothing_0434EA  ($0434EA, 14 B, mission/scene_loader_0434ea.s)
/// $0434EA: clr.l $10817a.l
pub fn i_0434ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434F0;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x10817A);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x10817A, r2);
    20
}
/// $0434F0: clr.l $10817e.l
pub fn i_0434f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434F6;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x10817E);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x10817E, r2);
    20
}
/// $0434F6: rts
pub fn i_0434f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camera_SmoothingIntegrate_0434F8  ($0434F8, 106 B, mission/scene_loader_0434ea.s)
/// $0434F8: move.l $10817a.l, d0
pub fn i_0434f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434FE;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x10817A);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0434FE: move.l d0, d1
pub fn i_0434fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43500;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043500: move.l $10817e.l, d2
pub fn i_043500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43506;
    cpu.ir = 0x2439;
    let v2 = bus.read32(0x10817E);
    cpu.d[2] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043506: move.l d2, d3
pub fn i_043506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43508;
    cpu.ir = 0x2602;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043508: cmpi.l #$4000, d3
pub fn i_043508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4350E;
    cpu.ir = 0xC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x4000, Size::Long, false);
    8
}
/// $04350E: bge.b $4351a
pub fn i_04350e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43510;
    cpu.ir = 0x6C0A;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x4351A; return 10; }
    8
}
/// $043510: cmpi.l #$ffffc000, d3
pub fn i_043510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43516;
    cpu.ir = 0xC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFC000, Size::Long, false);
    8
}
/// $043516: bgt.w $4351c
pub fn i_043516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4351A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4351C; return 10; }
    8
}
/// $04351A: asr.l #$3, d3
pub fn i_04351a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4351C;
    cpu.ir = 0xE683;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, false);
    cpu.d[3] = r;
    12
}
/// $04351C: add.l d1, d3
pub fn i_04351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4351E;
    cpu.ir = 0xD681;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[3] = r;
    4
}
/// $04351E: sub.l d3, d2
pub fn i_04351e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43520;
    cpu.ir = 0x9483;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $043520: move.l d2, $10817e.l
pub fn i_043520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43526;
    cpu.ir = 0x23C2;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    bus.write32(0x10817E, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043526: add.l d0, d2
pub fn i_043526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43528;
    cpu.ir = 0xD480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $043528: cmpi.l #$80000, d2
pub fn i_043528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4352E;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x80000, Size::Long, false);
    8
}
/// $04352E: ble.w $4353c
pub fn i_04352e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43532;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4353C; return 10; }
    8
}
/// $043532: move.l #$80000, d2
pub fn i_043532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43538;
    cpu.ir = 0x243C;
    let v1: u32 = 0x80000;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043538: bra.w $4354c
pub fn i_043538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4353C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4354C;
    10
}
/// $04353C: cmpi.l #$fff80000, d2
pub fn i_04353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43542;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFF80000, Size::Long, false);
    8
}
/// $043542: bgt.w $4354c
pub fn i_043542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43546;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4354C; return 10; }
    8
}
/// $043546: move.l #$fff80000, d2
pub fn i_043546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4354C;
    cpu.ir = 0x243C;
    let v1: u32 = 0xFFF80000;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04354C: move.l d2, $10817a.l
pub fn i_04354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43552;
    cpu.ir = 0x23C2;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    bus.write32(0x10817A, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043552: swap d2
pub fn i_043552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43554;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043554: swap d0
pub fn i_043554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43556;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043556: sub.w d0, d2
pub fn i_043556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43558;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043558: move.w d2, d0
pub fn i_043558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4355A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04355A: move.w d0, $108182.l
pub fn i_04355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43560;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108182, v1 as u16);
    16
}
/// $043560: rts
pub fn i_043560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43562;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneLoader_ByIndex_043562  ($043562, 6 B, mission/scene_loader_0434ea.s)
/// $043562: clr.w d0
pub fn i_043562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43564;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043564: bra.w $43574
pub fn i_043564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43568;
    cpu.ir = 0x6000;
    cpu.pc = 0x43574;
    10
}
// ===== SceneLoader_Main_043568  ($043568, 374 B, mission/scene_loader_0434ea.s)
/// $043568: andi.w #$ff, d0
pub fn i_043568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4356C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04356C: lsl.w #$3, d0
pub fn i_04356c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4356E;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04356E: lea.l $916c8.l, a0
pub fn i_04356e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43574;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x916C8;
    20
}
/// $043574: move.l (a0, d0.w), $10815c.l
pub fn i_043574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4357C;
    cpu.ir = 0x23F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(0x10815C, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    34
}
/// $04357C: movea.l $4(a0, d0.w), a0
pub fn i_04357c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43580;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $043580: move.l a0, -(a7)
pub fn i_043580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43582;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $043582: bsr.w $43d6c
pub fn i_043582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43586;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43586);
    cpu.pc = 0x43D6C;
    18
}
/// $043586: movea.l (a7), a0
pub fn i_043586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43588;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $043588: moveq #$0, d1
pub fn i_043588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4358A;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04358A: move.l d1, d2
pub fn i_04358a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4358C;
    cpu.ir = 0x2401;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04358C: cmpi.b #$2, (a0)
pub fn i_04358c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43590;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $043590: beq.w $435b0
pub fn i_043590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43594;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x435B0; return 10; }
    8
}
/// $043594: move.b (a0), d0
pub fn i_043594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43596;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $043596: cmpi.b #$0, d0
pub fn i_043596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4359A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $04359A: bne.w $435a6
pub fn i_04359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4359E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x435A6; return 10; }
    8
}
/// $04359E: addi.w #$20, d1
pub fn i_04359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435A2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0435A2: bra.w $435aa
pub fn i_0435a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x435AA;
    10
}
/// $0435A6: addi.w #$20, d2
pub fn i_0435a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435AA;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0435AA: adda.w #$e, a0
pub fn i_0435aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435AE;
    cpu.ir = 0xD0FC;
    let v1: u32 = 0xE;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $0435AE: bra.b $4358c
pub fn i_0435ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B0;
    cpu.ir = 0x60DC;
    cpu.pc = 0x4358C;
    10
}
/// $0435B0: move.w d2, d0
pub fn i_0435b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B2;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0435B2: moveq #$0, d2
pub fn i_0435b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B4;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0435B4: move.b $1(a0), d2
pub fn i_0435b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B8;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0435B8: jsr $1390e.l
pub fn i_0435b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435BE);
    cpu.pc = 0x1390E;
    16
}
/// $0435BE: movea.l (a7)+, a0
pub fn i_0435be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435C0;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $0435C0: cmpi.b #$2, (a0)
pub fn i_0435c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435C4;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $0435C4: beq.w $43610
pub fn i_0435c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43610; return 10; }
    8
}
/// $0435C8: move.l a0, -(a7)
pub fn i_0435c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435CA;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0435CA: moveq #$20, d0
pub fn i_0435ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435CC;
    cpu.ir = 0x7020;
    cpu.d[0] = 0x20;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0435CC: cmpi.b #$0, (a0)
pub fn i_0435cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435D0;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $0435D0: bne.w $435de
pub fn i_0435d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x435DE; return 10; }
    8
}
/// $0435D4: jsr $13982.l
pub fn i_0435d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435DA);
    cpu.pc = 0x13982;
    16
}
/// $0435DA: bra.w $435e4
pub fn i_0435da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x435E4;
    10
}
/// $0435DE: jsr $13952.l
pub fn i_0435de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435E4);
    cpu.pc = 0x13952;
    16
}
/// $0435E4: movea.l (a7), a2
pub fn i_0435e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435E6;
    cpu.ir = 0x2457;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[2] = v2;
    12
}
/// $0435E6: movea.l $2(a2), a0
pub fn i_0435e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435EA;
    cpu.ir = 0x206A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0435EA: lea.l $6(a2), a1
pub fn i_0435ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435EE;
    cpu.ir = 0x43EA;
    let ad1 = cpu.a[2].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $0435EE: jsr $51abe.l
pub fn i_0435ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435F4);
    cpu.pc = 0x51ABE;
    16
}
/// $0435F4: move.w #$100, $74(a0)
pub fn i_0435f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435FA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0435FA: move.w #$100, $76(a0)
pub fn i_0435fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43600;
    cpu.ir = 0x317C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $043600: clr.b $72(a0)
pub fn i_043600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43604;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $043604: clr.l $78(a0)
pub fn i_043604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43608;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x78);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $043608: movea.l (a7)+, a0
pub fn i_043608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4360A;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $04360A: adda.w #$e, a0
pub fn i_04360a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4360E;
    cpu.ir = 0xD0FC;
    let v1: u32 = 0xE;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $04360E: bra.b $435c0
pub fn i_04360e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43610;
    cpu.ir = 0x60B0;
    cpu.pc = 0x435C0;
    10
}
/// $043610: clr.w $106f5e.l
pub fn i_043610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43616;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5E, r2 as u16);
    16
}
/// $043616: clr.l $106f50.l
pub fn i_043616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4361C;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F50);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F50, r2);
    20
}
/// $04361C: clr.l $106f54.l
pub fn i_04361c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43622;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F54);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F54, r2);
    20
}
/// $043622: clr.l $106f58.l
pub fn i_043622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43628;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F58);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F58, r2);
    20
}
/// $043628: clr.w $108160.l
pub fn i_043628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4362E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108160) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108160, r2 as u16);
    16
}
/// $04362E: clr.w $108162.l
pub fn i_04362e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43634;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108162) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043634: move.w #$a0, $108164.l
pub fn i_043634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4363C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108164, v1 as u16);
    16
}
/// $04363C: move.w #$180, $108166.l
pub fn i_04363c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43644;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108166, v1 as u16);
    16
}
/// $043644: move.w #$0, $108168.l
pub fn i_043644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4364C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108168, v1 as u16);
    16
}
/// $04364C: move.w #$0, $10816a.l
pub fn i_04364c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43654;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v1 as u16);
    16
}
/// $043654: move.w #$ffff, $10816c.l
pub fn i_043654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4365C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816C, v1 as u16);
    16
}
/// $04365C: move.w #$ffff, $10816e.l
pub fn i_04365c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43664;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v1 as u16);
    16
}
/// $043664: move.w #$0, $108170.l
pub fn i_043664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4366C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108170, v1 as u16);
    16
}
/// $04366C: clr.w $106f5c.l
pub fn i_04366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43672;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5C, r2 as u16);
    16
}
/// $043672: clr.b $108179.l
pub fn i_043672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43678;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x108179) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x108179, r2 as u8);
    16
}
/// $043678: move.w $106f5c.l, $108172.l
pub fn i_043678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43682;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108172, v2 as u16);
    28
}
/// $043682: move.w $106f50.l, $108174.l
pub fn i_043682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4368C;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108174, v2 as u16);
    28
}
/// $04368C: move.w $106f58.l, $108176.l
pub fn i_04368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43696;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108176, v2 as u16);
    28
}
/// $043696: lea.l $106f6c.l, a0
pub fn i_043696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4369C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $04369C: bsr.w $43eda
pub fn i_04369c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x436A0);
    cpu.pc = 0x43EDA;
    18
}
/// $0436A0: bsr.w $434ea
pub fn i_0436a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x436A4);
    cpu.pc = 0x434EA;
    18
}
/// $0436A4: clr.b $10e39c.l
pub fn i_0436a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436AA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $0436AA: clr.b $10e39d.l
pub fn i_0436aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436B0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
/// $0436B0: clr.w $106f42.l
pub fn i_0436b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436B6;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F42) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F42, r2 as u16);
    16
}
/// $0436B6: move.b #$1, $106ed3.l
pub fn i_0436b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436BE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED3, v1 as u8);
    16
}
/// $0436BE: jsr $7707c.l
pub fn i_0436be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436C4);
    cpu.pc = 0x7707C;
    16
}
/// $0436C4: jsr $8f158.l
pub fn i_0436c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436CA);
    cpu.pc = 0x8F158;
    16
}
/// $0436CA: jsr $3ee3a.l
pub fn i_0436ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436D0);
    cpu.pc = 0x3EE3A;
    16
}
/// $0436D0: jsr $997b8.l
pub fn i_0436d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436D6);
    cpu.pc = 0x997B8;
    16
}
/// $0436D6: jsr $4cb5c.l
pub fn i_0436d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436DC);
    cpu.pc = 0x4CB5C;
    16
}
/// $0436DC: rts
pub fn i_0436dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Geom_Proj_Clamp_0436DE  ($0436DE, 252 B, video/geom_proj_clamp_0436de.s)
/// $0436DE: tst.b $108179.l
pub fn i_0436de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436E4;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x108179) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0436E4: bne.w $436f4
pub fn i_0436e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x436F4; return 10; }
    8
}
/// $0436E8: tst.l d0
pub fn i_0436e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436EA;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0436EA: bmi.w $436f4
pub fn i_0436ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436EE;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x436F4; return 10; }
    8
}
/// $0436EE: tst.l d1
pub fn i_0436ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F0;
    cpu.ir = 0x4A81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0436F0: bpl.w $436fc
pub fn i_0436f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x436FC; return 10; }
    8
}
/// $0436F4: clr.l d0
pub fn i_0436f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0436F6: clr.l d1
pub fn i_0436f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F8;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $0436F8: bra.w $437da
pub fn i_0436f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x437DA;
    10
}
/// $0436FC: move.w #$200, d4
pub fn i_0436fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43700;
    cpu.ir = 0x383C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043700: sub.w d4, d1
pub fn i_043700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43702;
    cpu.ir = 0x9244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043702: neg.w d1
pub fn i_043702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43704;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043704: sub.w d4, d3
pub fn i_043704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43706;
    cpu.ir = 0x9644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043706: neg.w d3
pub fn i_043706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43708;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043708: cmp.w d0, d2
pub fn i_043708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4370A;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $04370A: bcc.w $43710
pub fn i_04370a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4370E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43710; return 10; }
    8
}
/// $04370E: exg.l d0, d2
pub fn i_04370e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43710;
    cpu.ir = 0xC142;
    cpu.d.swap(0, 2);
    6
}
/// $043710: cmp.w d1, d3
pub fn i_043710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43712;
    cpu.ir = 0xB641;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $043712: bcc.w $43718
pub fn i_043712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43716;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43718; return 10; }
    8
}
/// $043716: exg.l d1, d3
pub fn i_043716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43718;
    cpu.ir = 0xC343;
    cpu.d.swap(1, 3);
    6
}
/// $043718: move.w $108164.l, d4
pub fn i_043718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4371E;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x108164) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04371E: move.w #$1e, d5
pub fn i_04371e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43722;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043722: move.w #$117, d6
pub fn i_043722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43726;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x117;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043726: bsr.w $4374c
pub fn i_043726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4372A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4372A);
    cpu.pc = 0x4374C;
    18
}
/// $04372A: exg.l d1, d0
pub fn i_04372a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4372C;
    cpu.ir = 0xC340;
    cpu.d.swap(1, 0);
    6
}
/// $04372C: move.w d3, d2
pub fn i_04372c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4372E;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04372E: move.w $108166.l, d4
pub fn i_04372e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43734;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x108166) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043734: sub.w $10817a.l, d4
pub fn i_043734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4373A;
    cpu.ir = 0x9879;
    let v2 = bus.read16(0x10817A) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04373A: move.w #$0, d5
pub fn i_04373a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4373E;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04373E: move.w #$d0, d6
pub fn i_04373e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43742;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043742: bsr.w $4374c
pub fn i_043742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43746;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43746);
    cpu.pc = 0x4374C;
    18
}
/// $043746: exg.l d1, d0
pub fn i_043746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43748;
    cpu.ir = 0xC340;
    cpu.d.swap(1, 0);
    6
}
/// $043748: bra.w $437da
pub fn i_043748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4374C;
    cpu.ir = 0x6000;
    cpu.pc = 0x437DA;
    10
}
/// $04374C: swap d0
pub fn i_04374c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4374E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04374E: clr.w d0
pub fn i_04374e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43750;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043750: swap d2
pub fn i_043750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43752;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043752: clr.w d2
pub fn i_043752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43754;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043754: swap d4
pub fn i_043754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43756;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043756: clr.w d4
pub fn i_043756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43758;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043758: swap d5
pub fn i_043758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4375A;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04375A: clr.w d5
pub fn i_04375a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4375C;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04375C: swap d6
pub fn i_04375c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4375E;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04375E: clr.w d6
pub fn i_04375e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43760;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043760: move.l d0, d7
pub fn i_043760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43762;
    cpu.ir = 0x2E00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043762: add.l d2, d7
pub fn i_043762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43764;
    cpu.ir = 0xDE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $043764: asr.l #$1, d7
pub fn i_043764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43766;
    cpu.ir = 0xE287;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, false);
    cpu.d[7] = r;
    8
}
/// $043766: sub.l d4, d7
pub fn i_043766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43768;
    cpu.ir = 0x9E84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $043768: addi.l #$10000, d7
pub fn i_043768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4376E;
    cpu.ir = 0x687;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10000, Size::Long, true);
    cpu.d[7] = r2;
    8
}
/// $04376E: cmpi.l #$20000, d7
pub fn i_04376e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43774;
    cpu.ir = 0xC87;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x20000, Size::Long, false);
    8
}
/// $043774: bcs.w $437d6
pub fn i_043774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43778;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x437D6; return 10; }
    8
}
/// $043778: subi.l #$10000, d7
pub fn i_043778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4377E;
    cpu.ir = 0x487;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10000, Size::Long, true);
    cpu.d[7] = r2;
    8
}
/// $04377E: sub.l d5, d0
pub fn i_04377e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43780;
    cpu.ir = 0x9085;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $043780: sub.l d6, d2
pub fn i_043780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43782;
    cpu.ir = 0x9486;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $043782: move.l d7, -(a7)
pub fn i_043782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43784;
    cpu.ir = 0x2F07;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $043784: asr.l #$1, d7
pub fn i_043784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43786;
    cpu.ir = 0xE287;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, false);
    cpu.d[7] = r;
    8
}
/// $043786: add.l (a7)+, d7
pub fn i_043786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43788;
    cpu.ir = 0xDE9F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Long, true);
    cpu.d[7] = r;
    12
}
/// $043788: bmi.w $437a4
pub fn i_043788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4378C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x437A4; return 10; }
    8
}
/// $04378C: tst.l d0
pub fn i_04378c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4378E;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04378E: bge.w $43798
pub fn i_04378e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43792;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x43798; return 10; }
    8
}
/// $043792: clr.l d7
pub fn i_043792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43794;
    cpu.ir = 0x4287;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = r2;
    4
}
/// $043794: bra.w $437a0
pub fn i_043794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43798;
    cpu.ir = 0x6000;
    cpu.pc = 0x437A0;
    10
}
/// $043798: cmp.l d0, d7
pub fn i_043798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4379A;
    cpu.ir = 0xBE80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $04379A: ble.w $437a0
pub fn i_04379a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4379E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x437A0; return 10; }
    8
}
/// $04379E: move.l d0, d7
pub fn i_04379e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437A0;
    cpu.ir = 0x2E00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437A0: bra.w $437b8
pub fn i_0437a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x437B8;
    10
}
/// $0437A4: tst.l d2
pub fn i_0437a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437A6;
    cpu.ir = 0x4A82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437A6: ble.w $437b0
pub fn i_0437a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437AA;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x437B0; return 10; }
    8
}
/// $0437AA: clr.l d7
pub fn i_0437aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437AC;
    cpu.ir = 0x4287;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = r2;
    4
}
/// $0437AC: bra.w $437b8
pub fn i_0437ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x437B8;
    10
}
/// $0437B0: cmp.l d2, d7
pub fn i_0437b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B2;
    cpu.ir = 0xBE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $0437B2: bge.w $437b8
pub fn i_0437b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x437B8; return 10; }
    8
}
/// $0437B6: move.l d2, d7
pub fn i_0437b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B8;
    cpu.ir = 0x2E02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437B8: asr.l #$4, d7
pub fn i_0437b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437BA;
    cpu.ir = 0xE887;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[7] = r;
    14
}
/// $0437BA: move.l #$80000, d2
pub fn i_0437ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C0;
    cpu.ir = 0x243C;
    let v1: u32 = 0x80000;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437C0: cmp.l d2, d7
pub fn i_0437c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C2;
    cpu.ir = 0xBE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $0437C2: ble.w $437c8
pub fn i_0437c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x437C8; return 10; }
    8
}
/// $0437C6: move.l d2, d7
pub fn i_0437c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C8;
    cpu.ir = 0x2E02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437C8: neg.l d2
pub fn i_0437c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437CA;
    cpu.ir = 0x4482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = r2;
    4
}
/// $0437CA: cmp.l d2, d7
pub fn i_0437ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437CC;
    cpu.ir = 0xBE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $0437CC: bge.w $437d2
pub fn i_0437cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x437D2; return 10; }
    8
}
/// $0437D0: move.l d2, d7
pub fn i_0437d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D2;
    cpu.ir = 0x2E02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437D2: move.l d7, d0
pub fn i_0437d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D4;
    cpu.ir = 0x2007;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437D4: rts
pub fn i_0437d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0437D6: moveq #$0, d0
pub fn i_0437d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437D8: rts
pub fn i_0437d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneScriptVM_Frame_0437DA  ($0437DA, 1288 B, mission/scene_script_vm_0437da.s)
/// $0437DA: tst.w $106f5e.l
pub fn i_0437da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437E0;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $0437E0: beq.w $437f0
pub fn i_0437e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x437F0; return 10; }
    8
}
/// $0437E4: move.l $106f60.l, d0
pub fn i_0437e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437EA;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F60);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0437EA: move.l $106f64.l, d1
pub fn i_0437ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437F0;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F64);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0437F0: movem.l d0-d1, -(a7)
pub fn i_0437f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437F4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0437F4: jsr $77092.l
pub fn i_0437f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x437FA);
    cpu.pc = 0x77092;
    16
}
/// $0437FA: jsr $8f172.l
pub fn i_0437fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43800;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43800);
    cpu.pc = 0x8F172;
    16
}
/// $043800: jsr $997cc.l
pub fn i_043800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43806);
    cpu.pc = 0x997CC;
    16
}
/// $043806: jsr $8f8e2.l
pub fn i_043806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4380C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4380C);
    cpu.pc = 0x8F8E2;
    16
}
/// $04380C: movea.l $10815c.l, a1
pub fn i_04380c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43812;
    cpu.ir = 0x2279;
    let v2 = bus.read32(0x10815C);
    cpu.a[1] = v2;
    20
}
/// $043812: move.w (a1), d0
pub fn i_043812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43814;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $043814: add.w d0, d0
pub fn i_043814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43816;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043816: add.w d0, d0
pub fn i_043816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43818;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043818: jmp $4381c(pc, d0.w)
pub fn i_043818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4381C;
    cpu.ir = 0x4EFB;
    let ad1 = 0x4381C_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $04381C: bra.w $43b66
pub fn i_04381c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43820;
    cpu.ir = 0x6000;
    cpu.pc = 0x43B66;
    10
}
/// $043820: bra.w $43ba2
pub fn i_043820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43824;
    cpu.ir = 0x6000;
    cpu.pc = 0x43BA2;
    10
}
/// $043824: bra.w $43bc6
pub fn i_043824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43828;
    cpu.ir = 0x6000;
    cpu.pc = 0x43BC6;
    10
}
/// $043828: bra.w $43afa
pub fn i_043828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4382C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43AFA;
    10
}
/// $04382C: bra.w $43ada
pub fn i_04382c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43830;
    cpu.ir = 0x6000;
    cpu.pc = 0x43ADA;
    10
}
/// $043830: bra.w $43ab8
pub fn i_043830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43834;
    cpu.ir = 0x6000;
    cpu.pc = 0x43AB8;
    10
}
/// $043834: bra.w $43aa4
pub fn i_043834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43838;
    cpu.ir = 0x6000;
    cpu.pc = 0x43AA4;
    10
}
/// $043838: bra.w $43a74
pub fn i_043838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4383C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A74;
    10
}
/// $043840: bra.w $43a58
pub fn i_043840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43844;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A58;
    10
}
/// $043844: bra.w $43a42
pub fn i_043844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43848;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A42;
    10
}
/// $04384C: bra.w $43a08
pub fn i_04384c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43850;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A08;
    10
}
/// $043850: bra.w $439f0
pub fn i_043850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43854;
    cpu.ir = 0x6000;
    cpu.pc = 0x439F0;
    10
}
/// $043858: bra.w $43984
pub fn i_043858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4385C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43984;
    10
}
/// $04385C: bra.w $43966
pub fn i_04385c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43860;
    cpu.ir = 0x6000;
    cpu.pc = 0x43966;
    10
}
/// $043860: bra.w $4394a
pub fn i_043860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43864;
    cpu.ir = 0x6000;
    cpu.pc = 0x4394A;
    10
}
/// $04394A: lea.l $a(a1), a0
pub fn i_04394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4394E;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xA);
    cpu.a[0] = ad1;
    16
}
/// $04394E: move.l a0, $10815c.l
pub fn i_04394e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43954;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043954: movea.l $2(a1), a0
pub fn i_043954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43958;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043958: movea.l $6(a1), a1
pub fn i_043958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4395C;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04395C: jsr $51b38.l
pub fn i_04395c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43962);
    cpu.pc = 0x51B38;
    16
}
/// $043962: bra.w $4380c
pub fn i_043962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43966;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043966: lea.l $6(a1), a0
pub fn i_043966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4396A;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $04396A: move.l a0, $10815c.l
pub fn i_04396a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43970;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043970: move.w $2(a1), $108164.l
pub fn i_043970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43978;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108164, v3 as u16);
    24
}
/// $043978: move.w $4(a1), $108166.l
pub fn i_043978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43980;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108166, v3 as u16);
    24
}
/// $043980: bra.w $4380c
pub fn i_043980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43984;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043984: lea.l $c(a1), a0
pub fn i_043984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43988;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $043988: move.l a0, $10815c.l
pub fn i_043988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4398E;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04398E: move.w $2(a1), $108168.l
pub fn i_04398e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43996;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108168, v3 as u16);
    24
}
/// $043996: move.w $4(a1), $10816a.l
pub fn i_043996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4399E;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v3 as u16);
    24
}
/// $04399E: move.w $6(a1), $10816c.l
pub fn i_04399e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439A6;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816C, v3 as u16);
    24
}
/// $0439A6: move.w $8(a1), $10816e.l
pub fn i_0439a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439AE;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v3 as u16);
    24
}
/// $0439AE: move.w $a(a1), $108170.l
pub fn i_0439ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439B6;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108170, v3 as u16);
    24
}
/// $0439B6: bra.w $4380c
pub fn i_0439b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $0439F0: lea.l $e(a1), a0
pub fn i_0439f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439F4;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xE);
    cpu.a[0] = ad1;
    16
}
/// $0439F4: move.l a0, $10815c.l
pub fn i_0439f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439FA;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0439FA: lea.l $6(a1), a0
pub fn i_0439fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439FE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $0439FE: movea.l $2(a1), a1
pub fn i_0439fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A02;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043A02: jsr (a1)
pub fn i_043a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A04;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x43A04);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $043A04: bra.w $4380c
pub fn i_043a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A08;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A08: lea.l $4(a1), a0
pub fn i_043a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A0C;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[0] = ad1;
    16
}
/// $043A0C: move.l a0, $10815c.l
pub fn i_043a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A12;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A12: move.b $2(a1), $106eac.l
pub fn i_043a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A1A;
    cpu.ir = 0x13E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EAC, v3 as u8);
    24
}
/// $043A1A: bra.w $4380c
pub fn i_043a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A42: lea.l $2(a1), a0
pub fn i_043a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A46;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[0] = ad1;
    16
}
/// $043A46: jsr $2b58.l
pub fn i_043a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A4C);
    cpu.pc = 0x2B58;
    16
}
/// $043A4C: addq.w #$2, a0
pub fn i_043a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A4E;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $043A4E: move.l a0, $10815c.l
pub fn i_043a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A54;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A54: bra.w $4380c
pub fn i_043a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A58;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A58: lea.l $6(a1), a0
pub fn i_043a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A5C;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A5C: move.l a0, $10815c.l
pub fn i_043a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A62;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A62: movea.l $2(a1), a0
pub fn i_043a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A66;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A66: bsr.w $43df4
pub fn i_043a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A6A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43A6A);
    cpu.pc = 0x43DF4;
    18
}
/// $043A6A: jsr $51ece.l
pub fn i_043a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A70);
    cpu.pc = 0x51ECE;
    16
}
/// $043A70: bra.w $4380c
pub fn i_043a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A74;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A74: lea.l $6(a1), a0
pub fn i_043a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A78;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A78: move.l a0, $10815c.l
pub fn i_043a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A7E;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A7E: movea.l $2(a1), a0
pub fn i_043a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A82;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A82: jsr $51ece.l
pub fn i_043a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A88);
    cpu.pc = 0x51ECE;
    16
}
/// $043A88: bra.w $4380c
pub fn i_043a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AA4: jsr $2(a1)
pub fn i_043aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AA8;
    cpu.ir = 0x4EA9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.push32(bus, 0x43AA8);
    cpu.jump_to(ad1);
    16
}
/// $043AA8: tst.b d0
pub fn i_043aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AAA;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $043AAA: bne.w $43bc6
pub fn i_043aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43BC6; return 10; }
    8
}
/// $043AAE: move.l a1, $10815c.l
pub fn i_043aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AB4;
    cpu.ir = 0x23C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AB4: bra.w $4380c
pub fn i_043ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AB8: lea.l $6(a1), a0
pub fn i_043ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ABC;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043ABC: move.l a0, $10815c.l
pub fn i_043abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AC2;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AC2: movea.l $2(a1), a0
pub fn i_043ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AC6;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043AC6: clr.l $e(a0)
pub fn i_043ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ACA;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0xE);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $043ACA: jsr $51ed6.l
pub fn i_043aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AD0);
    cpu.pc = 0x51ED6;
    16
}
/// $043AD0: jsr $51f30.l
pub fn i_043ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AD6);
    cpu.pc = 0x51F30;
    16
}
/// $043AD6: bra.w $4380c
pub fn i_043ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ADA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043ADA: lea.l $8(a1), a0
pub fn i_043ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ADE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x8);
    cpu.a[0] = ad1;
    16
}
/// $043ADE: move.l a0, $10815c.l
pub fn i_043ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AE4;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AE4: move.b $6(a1), d0
pub fn i_043ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AE8;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $043AE8: move.b $7(a1), d1
pub fn i_043ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AEC;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $043AEC: movea.l $2(a1), a0
pub fn i_043aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AF0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043AF0: jsr $51b1c.l
pub fn i_043af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AF6);
    cpu.pc = 0x51B1C;
    16
}
/// $043AF6: bra.w $4380c
pub fn i_043af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AFA: lea.l $6(a1), a0
pub fn i_043afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AFE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043AFE: move.l a0, $10815c.l
pub fn i_043afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B04;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B04: move.w $2(a1), d0
pub fn i_043b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B08;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043B08: move.w $4(a1), d1
pub fn i_043b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B0C;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043B0C: move.b #$1, $108178.l
pub fn i_043b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B14;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x108178, v1 as u8);
    16
}
/// $043B14: clr.w $106f5c.l
pub fn i_043b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5C, r2 as u16);
    16
}
/// $043B1A: swap d0
pub fn i_043b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043B1C: swap d1
pub fn i_043b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043B1E: clr.w d0
pub fn i_043b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B20;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043B20: clr.w d1
pub fn i_043b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B22;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043B22: move.l d0, $106f50.l
pub fn i_043b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B28;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x106F50, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B28: move.l d1, $106f54.l
pub fn i_043b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B2E;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F54, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B2E: move.l d1, $106f58.l
pub fn i_043b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B34;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F58, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B34: clr.w $108160.l
pub fn i_043b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B3A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108160) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108160, r2 as u16);
    16
}
/// $043B3A: clr.w $108162.l
pub fn i_043b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B40;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108162) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043B40: move.w $106f5c.l, $108172.l
pub fn i_043b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B4A;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108172, v2 as u16);
    28
}
/// $043B4A: move.w $106f50.l, $108174.l
pub fn i_043b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B54;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108174, v2 as u16);
    28
}
/// $043B54: move.w $106f58.l, $108176.l
pub fn i_043b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B5E;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108176, v2 as u16);
    28
}
/// $043B5E: bsr.w $434ea
pub fn i_043b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B62;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43B62);
    cpu.pc = 0x434EA;
    18
}
/// $043B62: bra.w $4380c
pub fn i_043b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B66;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043B66: lea.l $16(a1), a0
pub fn i_043b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B6A;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x16);
    cpu.a[0] = ad1;
    16
}
/// $043B6A: move.l a0, $10815c.l
pub fn i_043b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B70;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B70: movea.l $2(a1), a0
pub fn i_043b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B74;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043B74: move.l $e(a1), $78(a0)
pub fn i_043b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B7A;
    cpu.ir = 0x2169;
    let ad2 = cpu.a[1].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $043B7A: move.w $106f50.l, d0
pub fn i_043b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B80;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043B80: move.w $106f58.l, d1
pub fn i_043b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B86;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043B86: add.w $10817a.l, d1
pub fn i_043b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B8C;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x10817A) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043B8C: sub.w $12(a1), d0
pub fn i_043b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B90;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x12);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043B90: sub.w $14(a1), d1
pub fn i_043b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B94;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043B94: lea.l $6(a1), a1
pub fn i_043b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B98;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $043B98: jsr $51b3e.l
pub fn i_043b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43B9E);
    cpu.pc = 0x51B3E;
    16
}
/// $043B9E: bra.w $4380c
pub fn i_043b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043BA2: lea.l $c(a1), a0
pub fn i_043ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BA6;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $043BA6: move.l a0, $10815c.l
pub fn i_043ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BAC;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BAC: movea.l $2(a1), a0
pub fn i_043bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BB0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043BB0: move.b $6(a1), $72(a0)
pub fn i_043bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BB6;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write8(ad4, v3 as u8);
    20
}
/// $043BB6: move.w $8(a1), $74(a0)
pub fn i_043bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BBC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    20
}
/// $043BBC: move.w $a(a1), $76(a0)
pub fn i_043bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC2;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $043BC2: bra.w $4380c
pub fn i_043bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043BC6: move.l (a7)+, d0
pub fn i_043bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC8;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $043BC8: move.l $106f50.l, d1
pub fn i_043bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BCE;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F50);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BCE: move.w $108168.l, d2
pub fn i_043bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BD4;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x108168) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BD4: move.w $10816c.l, d3
pub fn i_043bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BDA;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10816C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BDA: bsr.w $43e3a
pub fn i_043bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BDE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43BDE);
    cpu.pc = 0x43E3A;
    18
}
/// $043BDE: move.w d0, $108160.l
pub fn i_043bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BE4;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108160, v1 as u16);
    16
}
/// $043BE4: move.l d1, $106f50.l
pub fn i_043be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BEA;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F50, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BEA: move.l (a7)+, d0
pub fn i_043bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BEC;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $043BEC: move.l $106f58.l, d1
pub fn i_043bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BF2;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F58);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BF2: move.w $10816a.l, d2
pub fn i_043bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BF8;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x10816A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BF8: move.w $10816e.l, d3
pub fn i_043bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BFE;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10816E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BFE: move.w $108170.l, d4
pub fn i_043bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C04;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x108170) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C04: beq.w $43c36
pub fn i_043c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43C36; return 10; }
    8
}
/// $043C08: bmi.w $43c18
pub fn i_043c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C0C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x43C18; return 10; }
    8
}
/// $043C0C: move.w $108160.l, d0
pub fn i_043c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C12;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C12: neg.w d0
pub fn i_043c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C14;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043C14: bra.w $43c1e
pub fn i_043c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C18;
    cpu.ir = 0x6000;
    cpu.pc = 0x43C1E;
    10
}
/// $043C18: move.w $108160.l, d0
pub fn i_043c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C1E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C1E: swap d0
pub fn i_043c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C20;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043C20: clr.w d0
pub fn i_043c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C22;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043C22: move.w $10816c.l, d5
pub fn i_043c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C28;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x10816C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C28: sub.w $106f50.l, d5
pub fn i_043c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C2E;
    cpu.ir = 0x9A79;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C2E: muls.w d5, d4
pub fn i_043c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C30;
    cpu.ir = 0xC9C5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[4] as u16 as i16 as i32)) as u32;
    cpu.d[4] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $043C30: asr.l #$8, d4
pub fn i_043c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C32;
    cpu.ir = 0xE084;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[4] = r;
    22
}
/// $043C32: add.w d4, d2
pub fn i_043c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C34;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043C34: add.w d4, d3
pub fn i_043c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C36;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043C36: bsr.w $43e3a
pub fn i_043c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C3A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43C3A);
    cpu.pc = 0x43E3A;
    18
}
/// $043C3A: move.w d0, $108162.l
pub fn i_043c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C40;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108162, v1 as u16);
    16
}
/// $043C40: move.l d1, $106f58.l
pub fn i_043c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C46;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F58, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043C46: move.b $108178.l, d0
pub fn i_043c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C4C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x108178) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $043C4C: cmpi.b #$3, d0
pub fn i_043c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C50;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $043C50: bne.w $43c64
pub fn i_043c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C64; return 10; }
    8
}
/// $043C54: move.w $106f58.l, d1
pub fn i_043c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C5A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C5A: sub.w $108176.l, d1
pub fn i_043c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C60;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x108176) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C60: bra.w $43ca0
pub fn i_043c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C64;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C64: cmpi.b #$2, d0
pub fn i_043c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C68;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $043C68: bne.w $43c7c
pub fn i_043c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C7C; return 10; }
    8
}
/// $043C6C: move.w $108176.l, d1
pub fn i_043c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C72;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108176) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C72: sub.w $106f58.l, d1
pub fn i_043c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C78;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F58) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C78: bra.w $43ca0
pub fn i_043c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C7C: cmpi.b #$0, d0
pub fn i_043c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C80;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $043C80: bne.w $43c94
pub fn i_043c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C94; return 10; }
    8
}
/// $043C84: move.w $108174.l, d1
pub fn i_043c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C8A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108174) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C8A: sub.w $106f50.l, d1
pub fn i_043c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C90;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C90: bra.w $43ca0
pub fn i_043c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C94;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C94: move.w $106f50.l, d1
pub fn i_043c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C9A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C9A: sub.w $108174.l, d1
pub fn i_043c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CA0;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x108174) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043CA0: add.w $108172.l, d1
pub fn i_043ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CA6;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x108172) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043CA6: cmp.w $106f5c.l, d1
pub fn i_043ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CAC;
    cpu.ir = 0xB279;
    let v2 = bus.read16(0x106F5C) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043CAC: bls.w $43cb6
pub fn i_043cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CB0;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x43CB6; return 10; }
    8
}
/// $043CB0: move.w d1, $106f5c.l
pub fn i_043cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CB6;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5C, v1 as u16);
    16
}
/// $043CB6: bsr.w $434f8
pub fn i_043cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CBA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43CBA);
    cpu.pc = 0x434F8;
    18
}
/// $043CBA: move.l $10817a.l, d0
pub fn i_043cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CC0;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x10817A);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043CC0: add.l $106f58.l, d0
pub fn i_043cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CC6;
    cpu.ir = 0xD0B9;
    let v2 = bus.read32(0x106F58);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Long, true);
    cpu.d[0] = r;
    20
}
/// $043CC6: move.l d0, $106f54.l
pub fn i_043cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CCC;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x106F54, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043CCC: move.w $108182.l, d0
pub fn i_043ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CD2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108182) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043CD2: add.w d0, $108162.l
pub fn i_043cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CD8;
    cpu.ir = 0xD179;
    let v1 = bus.read16(0x108162) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Word, true);
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043CD8: jsr $4cb88.l
pub fn i_043cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43CDE);
    cpu.pc = 0x4CB88;
    16
}
/// $043CDE: bra.w $43d86
pub fn i_043cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D86;
    10
}
// ===== SceneScript_EdgeArrivalTest_043CE2  ($043CE2, 126 B, mission/scene_script_vm_0437da.s)
/// $043CE2: tst.b $108179.l
pub fn i_043ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CE8;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x108179) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $043CE8: bne.w $43d66
pub fn i_043ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043CEC: tst.b $10e39a.l
pub fn i_043cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CF2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $043CF2: beq.w $43d66
pub fn i_043cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043CF6: move.b $108178.l, d0
pub fn i_043cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CFC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x108178) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $043CFC: cmpi.b #$3, d0
pub fn i_043cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D00;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $043D00: bne.w $43d18
pub fn i_043d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D18; return 10; }
    8
}
/// $043D04: move.w $106f58.l, d0
pub fn i_043d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D0A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D0A: cmp.w $10816e.l, d0
pub fn i_043d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D10;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816E) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D10: beq.w $43d66
pub fn i_043d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D14: bra.w $43d60
pub fn i_043d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D18;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D18: cmpi.b #$2, d0
pub fn i_043d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D1C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $043D1C: bne.w $43d34
pub fn i_043d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D34; return 10; }
    8
}
/// $043D20: move.w $106f58.l, d0
pub fn i_043d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D26;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D26: cmp.w $10816a.l, d0
pub fn i_043d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D2C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816A) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D2C: beq.w $43d66
pub fn i_043d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D30: bra.w $43d60
pub fn i_043d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D34;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D34: cmpi.b #$0, d0
pub fn i_043d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D38;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $043D38: bne.w $43d50
pub fn i_043d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D50; return 10; }
    8
}
/// $043D3C: move.w $106f50.l, d0
pub fn i_043d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D42;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D42: cmp.w $108168.l, d0
pub fn i_043d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D48;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x108168) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D48: beq.w $43d66
pub fn i_043d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D4C: bra.w $43d60
pub fn i_043d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D50;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D50: move.w $106f50.l, d0
pub fn i_043d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D56;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D56: cmp.w $10816c.l, d0
pub fn i_043d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D5C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D5C: beq.w $43d66
pub fn i_043d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
// ===== ClearC_043d60  ($043D60, 6 B, thunks/ccr_helpers.c)
/// $043D60: andi.b #$fe, ccr
pub fn i_043d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D64;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $043D64: rts
pub fn i_043d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_043d66  ($043D66, 6 B, thunks/ccr_helpers.c)
/// $043D66: ori.b #$1, ccr
pub fn i_043d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D6A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $043D6A: rts
pub fn i_043d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Reset4CameraLongs_043D6C  ($043D6C, 26 B, video/camera_list_ctx_helpers_025012.s)
/// $043D6C: clr.l $106f7a.l
pub fn i_043d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D72;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F7A);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F7A, r2);
    20
}
/// $043D72: clr.l $107ff6.l
pub fn i_043d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D78;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x107FF6);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x107FF6, r2);
    20
}
/// $043D78: clr.l $108072.l
pub fn i_043d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D7E;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x108072);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x108072, r2);
    20
}
/// $043D7E: clr.l $1080ee.l
pub fn i_043d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D84;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x1080EE);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x1080EE, r2);
    20
}
/// $043D84: rts
pub fn i_043d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraApplyAll4_043D86  ($043D86, 36 B, video/camera_list_ctx_helpers_025012.s)
/// $043D86: lea.l $106f6c.l, a0
pub fn i_043d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043D8C: bsr.w $43daa
pub fn i_043d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D90;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43D90);
    cpu.pc = 0x43DAA;
    18
}
/// $043D90: lea.l $107fe8.l, a0
pub fn i_043d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D96;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x107FE8;
    20
}
/// $043D96: bsr.w $43daa
pub fn i_043d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D9A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43D9A);
    cpu.pc = 0x43DAA;
    18
}
/// $043D9A: lea.l $108064.l, a0
pub fn i_043d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x108064;
    20
}
/// $043DA0: bsr.w $43daa
pub fn i_043da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DA4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43DA4);
    cpu.pc = 0x43DAA;
    18
}
/// $043DA4: lea.l $1080e0.l, a0
pub fn i_043da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1080E0;
    20
}
// ===== CameraApplyOne_043DAA  ($043DAA, 74 B, video/camera_apply_043daa.s)
/// $043DAA: tst.l $e(a0)
pub fn i_043daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DAE;
    cpu.ir = 0x4AA8;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $043DAE: beq.w $43df2
pub fn i_043dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43DF2; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x041CE0, i_041ce0),
    (0x041CE4, i_041ce4),
    (0x041CE8, i_041ce8),
    (0x041CEC, i_041cec),
    (0x041CF0, i_041cf0),
    (0x041CF2, i_041cf2),
    (0x041CF6, i_041cf6),
    (0x041CFA, i_041cfa),
    (0x041CFC, i_041cfc),
    (0x041CFE, i_041cfe),
    (0x041D02, i_041d02),
    (0x041D06, i_041d06),
    (0x041D08, i_041d08),
    (0x041D0A, i_041d0a),
    (0x041D0C, i_041d0c),
    (0x041D10, i_041d10),
    (0x041D14, i_041d14),
    (0x041D1A, i_041d1a),
    (0x041D1E, i_041d1e),
    (0x041D22, i_041d22),
    (0x041D26, i_041d26),
    (0x041D2A, i_041d2a),
    (0x041D2E, i_041d2e),
    (0x041D32, i_041d32),
    (0x041D36, i_041d36),
    (0x041D38, i_041d38),
    (0x041D3A, i_041d3a),
    (0x041D3E, i_041d3e),
    (0x041D42, i_041d42),
    (0x041D46, i_041d46),
    (0x041D48, i_041d48),
    (0x041D4C, i_041d4c),
    (0x041D50, i_041d50),
    (0x041D54, i_041d54),
    (0x041D58, i_041d58),
    (0x041D5C, i_041d5c),
    (0x041D5E, i_041d5e),
    (0x041D62, i_041d62),
    (0x041D66, i_041d66),
    (0x041D6A, i_041d6a),
    (0x041D6C, i_041d6c),
    (0x041D70, i_041d70),
    (0x041D74, i_041d74),
    (0x041D76, i_041d76),
    (0x041D78, i_041d78),
    (0x041D7C, i_041d7c),
    (0x041D7E, i_041d7e),
    (0x041D80, i_041d80),
    (0x041D84, i_041d84),
    (0x041D88, i_041d88),
    (0x041D8C, i_041d8c),
    (0x041D90, i_041d90),
    (0x041D94, i_041d94),
    (0x041D98, i_041d98),
    (0x041D9C, i_041d9c),
    (0x041D9E, i_041d9e),
    (0x041DA2, i_041da2),
    (0x041DA6, i_041da6),
    (0x041DAA, i_041daa),
    (0x041DAE, i_041dae),
    (0x041DB0, i_041db0),
    (0x041DB4, i_041db4),
    (0x041DB6, i_041db6),
    (0x041DBA, i_041dba),
    (0x041DBC, i_041dbc),
    (0x041DC0, i_041dc0),
    (0x041DC6, i_041dc6),
    (0x041DCA, i_041dca),
    (0x041DCC, i_041dcc),
    (0x041DD2, i_041dd2),
    (0x041DD6, i_041dd6),
    (0x041DD8, i_041dd8),
    (0x041DDC, i_041ddc),
    (0x041DE0, i_041de0),
    (0x041DE2, i_041de2),
    (0x041DE6, i_041de6),
    (0x041DEC, i_041dec),
    (0x041DF0, i_041df0),
    (0x041DF2, i_041df2),
    (0x041DF8, i_041df8),
    (0x041DFC, i_041dfc),
    (0x041DFE, i_041dfe),
    (0x041E02, i_041e02),
    (0x041E06, i_041e06),
    (0x041E08, i_041e08),
    (0x041E0C, i_041e0c),
    (0x041E12, i_041e12),
    (0x041E16, i_041e16),
    (0x041E18, i_041e18),
    (0x041E1E, i_041e1e),
    (0x041E22, i_041e22),
    (0x041E24, i_041e24),
    (0x041E28, i_041e28),
    (0x041E2C, i_041e2c),
    (0x041E32, i_041e32),
    (0x041E36, i_041e36),
    (0x041E38, i_041e38),
    (0x041E3C, i_041e3c),
    (0x041E40, i_041e40),
    (0x041E42, i_041e42),
    (0x041E44, i_041e44),
    (0x041E48, i_041e48),
    (0x041E4C, i_041e4c),
    (0x041E4E, i_041e4e),
    (0x041E52, i_041e52),
    (0x041E56, i_041e56),
    (0x041E58, i_041e58),
    (0x041E5A, i_041e5a),
    (0x041E5E, i_041e5e),
    (0x041E60, i_041e60),
    (0x041E62, i_041e62),
    (0x041E64, i_041e64),
    (0x041E6A, i_041e6a),
    (0x041E6E, i_041e6e),
    (0x041E70, i_041e70),
    (0x041E76, i_041e76),
    (0x041E7A, i_041e7a),
    (0x041E80, i_041e80),
    (0x041E84, i_041e84),
    (0x041E88, i_041e88),
    (0x041E8C, i_041e8c),
    (0x041E90, i_041e90),
    (0x041E94, i_041e94),
    (0x041E9A, i_041e9a),
    (0x041EA0, i_041ea0),
    (0x041EA4, i_041ea4),
    (0x041EAA, i_041eaa),
    (0x041EB0, i_041eb0),
    (0x041EB2, i_041eb2),
    (0x041EB6, i_041eb6),
    (0x041EBC, i_041ebc),
    (0x041EC0, i_041ec0),
    (0x041EC4, i_041ec4),
    (0x041EC8, i_041ec8),
    (0x041ECC, i_041ecc),
    (0x041ED0, i_041ed0),
    (0x041ED6, i_041ed6),
    (0x041EDC, i_041edc),
    (0x041EE0, i_041ee0),
    (0x041EE6, i_041ee6),
    (0x041EEC, i_041eec),
    (0x041EEE, i_041eee),
    (0x041EF2, i_041ef2),
    (0x041EF6, i_041ef6),
    (0x041EFA, i_041efa),
    (0x041EFE, i_041efe),
    (0x041F04, i_041f04),
    (0x041F0A, i_041f0a),
    (0x041F0E, i_041f0e),
    (0x041F14, i_041f14),
    (0x041F1A, i_041f1a),
    (0x041F1C, i_041f1c),
    (0x041F22, i_041f22),
    (0x041F28, i_041f28),
    (0x041F2C, i_041f2c),
    (0x041F32, i_041f32),
    (0x041F38, i_041f38),
    (0x041F3A, i_041f3a),
    (0x041F3E, i_041f3e),
    (0x041F42, i_041f42),
    (0x041F46, i_041f46),
    (0x041F48, i_041f48),
    (0x041F4C, i_041f4c),
    (0x041F50, i_041f50),
    (0x041F54, i_041f54),
    (0x041F58, i_041f58),
    (0x041F5C, i_041f5c),
    (0x041F60, i_041f60),
    (0x041F64, i_041f64),
    (0x041F66, i_041f66),
    (0x041F68, i_041f68),
    (0x041F6C, i_041f6c),
    (0x041F70, i_041f70),
    (0x041F74, i_041f74),
    (0x041F78, i_041f78),
    (0x041F7C, i_041f7c),
    (0x041F7E, i_041f7e),
    (0x041F82, i_041f82),
    (0x041F84, i_041f84),
    (0x041F8A, i_041f8a),
    (0x041F8E, i_041f8e),
    (0x041F90, i_041f90),
    (0x041F94, i_041f94),
    (0x041F98, i_041f98),
    (0x041F9A, i_041f9a),
    (0x041F9E, i_041f9e),
    (0x041FA2, i_041fa2),
    (0x041FA4, i_041fa4),
    (0x041FA8, i_041fa8),
    (0x041FAC, i_041fac),
    (0x041FB2, i_041fb2),
    (0x041FB4, i_041fb4),
    (0x041FB6, i_041fb6),
    (0x041FBC, i_041fbc),
    (0x041FC0, i_041fc0),
    (0x041FC4, i_041fc4),
    (0x041FCA, i_041fca),
    (0x041FD0, i_041fd0),
    (0x041FD4, i_041fd4),
    (0x041FD8, i_041fd8),
    (0x041FDE, i_041fde),
    (0x041FE4, i_041fe4),
    (0x041FEA, i_041fea),
    (0x041FEC, i_041fec),
    (0x041FEE, i_041fee),
    (0x041FF2, i_041ff2),
    (0x041FF4, i_041ff4),
    (0x041FF6, i_041ff6),
    (0x041FFA, i_041ffa),
    (0x041FFE, i_041ffe),
    (0x042002, i_042002),
    (0x042006, i_042006),
    (0x04200A, i_04200a),
    (0x04200E, i_04200e),
    (0x042012, i_042012),
    (0x042016, i_042016),
    (0x04201A, i_04201a),
    (0x04201E, i_04201e),
    (0x042020, i_042020),
    (0x042026, i_042026),
    (0x04202A, i_04202a),
    (0x04202E, i_04202e),
    (0x042032, i_042032),
    (0x042034, i_042034),
    (0x04203A, i_04203a),
    (0x04203E, i_04203e),
    (0x042040, i_042040),
    (0x042044, i_042044),
    (0x042048, i_042048),
    (0x04204C, i_04204c),
    (0x042050, i_042050),
    (0x042054, i_042054),
    (0x042058, i_042058),
    (0x04205C, i_04205c),
    (0x042060, i_042060),
    (0x042064, i_042064),
    (0x042068, i_042068),
    (0x04206A, i_04206a),
    (0x04206E, i_04206e),
    (0x042072, i_042072),
    (0x042076, i_042076),
    (0x04207A, i_04207a),
    (0x04207E, i_04207e),
    (0x042082, i_042082),
    (0x042086, i_042086),
    (0x042088, i_042088),
    (0x04208C, i_04208c),
    (0x042090, i_042090),
    (0x042092, i_042092),
    (0x042096, i_042096),
    (0x04209A, i_04209a),
    (0x04209E, i_04209e),
    (0x0420A2, i_0420a2),
    (0x0420A4, i_0420a4),
    (0x0420A8, i_0420a8),
    (0x0420AA, i_0420aa),
    (0x0420B0, i_0420b0),
    (0x0420B6, i_0420b6),
    (0x0420BA, i_0420ba),
    (0x0420BE, i_0420be),
    (0x0420C2, i_0420c2),
    (0x0420C6, i_0420c6),
    (0x0420CA, i_0420ca),
    (0x0420CE, i_0420ce),
    (0x0420D2, i_0420d2),
    (0x0420D4, i_0420d4),
    (0x0420D6, i_0420d6),
    (0x0420D8, i_0420d8),
    (0x0420DA, i_0420da),
    (0x0420DE, i_0420de),
    (0x0420E2, i_0420e2),
    (0x0420E4, i_0420e4),
    (0x0420E8, i_0420e8),
    (0x0420EC, i_0420ec),
    (0x0420F2, i_0420f2),
    (0x0420FE, i_0420fe),
    (0x042102, i_042102),
    (0x042108, i_042108),
    (0x04210C, i_04210c),
    (0x04210E, i_04210e),
    (0x042112, i_042112),
    (0x042116, i_042116),
    (0x04211A, i_04211a),
    (0x04211C, i_04211c),
    (0x04211E, i_04211e),
    (0x042124, i_042124),
    (0x042128, i_042128),
    (0x04212C, i_04212c),
    (0x042130, i_042130),
    (0x042134, i_042134),
    (0x04213A, i_04213a),
    (0x042140, i_042140),
    (0x042144, i_042144),
    (0x042148, i_042148),
    (0x04214C, i_04214c),
    (0x042150, i_042150),
    (0x042154, i_042154),
    (0x04215A, i_04215a),
    (0x042160, i_042160),
    (0x042164, i_042164),
    (0x042168, i_042168),
    (0x04216C, i_04216c),
    (0x042170, i_042170),
    (0x042176, i_042176),
    (0x04217C, i_04217c),
    (0x042182, i_042182),
    (0x042186, i_042186),
    (0x042188, i_042188),
    (0x04218C, i_04218c),
    (0x042192, i_042192),
    (0x042196, i_042196),
    (0x042198, i_042198),
    (0x04219C, i_04219c),
    (0x0421A0, i_0421a0),
    (0x0421A4, i_0421a4),
    (0x0421A6, i_0421a6),
    (0x0421A8, i_0421a8),
    (0x0421AE, i_0421ae),
    (0x0421B2, i_0421b2),
    (0x0421B6, i_0421b6),
    (0x0421BA, i_0421ba),
    (0x0421BE, i_0421be),
    (0x0421C4, i_0421c4),
    (0x0421CA, i_0421ca),
    (0x0421CE, i_0421ce),
    (0x0421D2, i_0421d2),
    (0x0421D6, i_0421d6),
    (0x0421DA, i_0421da),
    (0x0421DE, i_0421de),
    (0x0421E4, i_0421e4),
    (0x0421EA, i_0421ea),
    (0x0421EE, i_0421ee),
    (0x0421F2, i_0421f2),
    (0x0421F6, i_0421f6),
    (0x0421FA, i_0421fa),
    (0x0421FE, i_0421fe),
    (0x042204, i_042204),
    (0x042206, i_042206),
    (0x04220A, i_04220a),
    (0x042210, i_042210),
    (0x042214, i_042214),
    (0x042218, i_042218),
    (0x04221C, i_04221c),
    (0x042222, i_042222),
    (0x042226, i_042226),
    (0x04222A, i_04222a),
    (0x04222C, i_04222c),
    (0x042232, i_042232),
    (0x042234, i_042234),
    (0x042238, i_042238),
    (0x04223C, i_04223c),
    (0x042240, i_042240),
    (0x042244, i_042244),
    (0x04224A, i_04224a),
    (0x042250, i_042250),
    (0x042254, i_042254),
    (0x042258, i_042258),
    (0x04225C, i_04225c),
    (0x042260, i_042260),
    (0x042264, i_042264),
    (0x04226A, i_04226a),
    (0x042270, i_042270),
    (0x042274, i_042274),
    (0x042278, i_042278),
    (0x04227C, i_04227c),
    (0x042280, i_042280),
    (0x042282, i_042282),
    (0x042286, i_042286),
    (0x04228A, i_04228a),
    (0x04228E, i_04228e),
    (0x042292, i_042292),
    (0x042294, i_042294),
    (0x042298, i_042298),
    (0x04229A, i_04229a),
    (0x04229E, i_04229e),
    (0x0422A2, i_0422a2),
    (0x0422A4, i_0422a4),
    (0x0422A8, i_0422a8),
    (0x0422AA, i_0422aa),
    (0x0422AE, i_0422ae),
    (0x0422B2, i_0422b2),
    (0x0422B4, i_0422b4),
    (0x0422B6, i_0422b6),
    (0x0422B8, i_0422b8),
    (0x0422BC, i_0422bc),
    (0x0422C0, i_0422c0),
    (0x0422C4, i_0422c4),
    (0x0422C8, i_0422c8),
    (0x0422CA, i_0422ca),
    (0x0422CE, i_0422ce),
    (0x0422D2, i_0422d2),
    (0x0422D6, i_0422d6),
    (0x0422DA, i_0422da),
    (0x0422DC, i_0422dc),
    (0x0422E0, i_0422e0),
    (0x0422E2, i_0422e2),
    (0x0422E4, i_0422e4),
    (0x0422E8, i_0422e8),
    (0x0422EA, i_0422ea),
    (0x0422EE, i_0422ee),
    (0x0422F4, i_0422f4),
    (0x0422FA, i_0422fa),
    (0x0422FE, i_0422fe),
    (0x042300, i_042300),
    (0x042304, i_042304),
    (0x042308, i_042308),
    (0x04230C, i_04230c),
    (0x042312, i_042312),
    (0x042318, i_042318),
    (0x04231C, i_04231c),
    (0x042322, i_042322),
    (0x042324, i_042324),
    (0x04232A, i_04232a),
    (0x042330, i_042330),
    (0x042336, i_042336),
    (0x04233C, i_04233c),
    (0x04233E, i_04233e),
    (0x042342, i_042342),
    (0x042348, i_042348),
    (0x04234C, i_04234c),
    (0x042352, i_042352),
    (0x042358, i_042358),
    (0x04235E, i_04235e),
    (0x042364, i_042364),
    (0x042366, i_042366),
    (0x04236A, i_04236a),
    (0x042370, i_042370),
    (0x042376, i_042376),
    (0x04237C, i_04237c),
    (0x042382, i_042382),
    (0x042388, i_042388),
    (0x04238E, i_04238e),
    (0x042390, i_042390),
    (0x042394, i_042394),
    (0x042398, i_042398),
    (0x04239C, i_04239c),
    (0x0423A0, i_0423a0),
    (0x0423A4, i_0423a4),
    (0x0423A6, i_0423a6),
    (0x0423AA, i_0423aa),
    (0x0423AC, i_0423ac),
    (0x0423B2, i_0423b2),
    (0x0423B4, i_0423b4),
    (0x0423B8, i_0423b8),
    (0x0423BC, i_0423bc),
    (0x0423BE, i_0423be),
    (0x0423C2, i_0423c2),
    (0x0423C6, i_0423c6),
    (0x0423CA, i_0423ca),
    (0x0423CC, i_0423cc),
    (0x0423D0, i_0423d0),
    (0x0423D2, i_0423d2),
    (0x0423D6, i_0423d6),
    (0x0423DA, i_0423da),
    (0x0423DE, i_0423de),
    (0x0423E0, i_0423e0),
    (0x0423E4, i_0423e4),
    (0x0423E8, i_0423e8),
    (0x0423EA, i_0423ea),
    (0x0423EC, i_0423ec),
    (0x0423F2, i_0423f2),
    (0x0423F6, i_0423f6),
    (0x0423FA, i_0423fa),
    (0x042400, i_042400),
    (0x042404, i_042404),
    (0x042408, i_042408),
    (0x04240C, i_04240c),
    (0x042410, i_042410),
    (0x042414, i_042414),
    (0x042418, i_042418),
    (0x04241C, i_04241c),
    (0x042422, i_042422),
    (0x042426, i_042426),
    (0x04242A, i_04242a),
    (0x042430, i_042430),
    (0x042434, i_042434),
    (0x042436, i_042436),
    (0x04243A, i_04243a),
    (0x04243E, i_04243e),
    (0x042440, i_042440),
    (0x042444, i_042444),
    (0x042446, i_042446),
    (0x042448, i_042448),
    (0x04244C, i_04244c),
    (0x042450, i_042450),
    (0x042454, i_042454),
    (0x04245A, i_04245a),
    (0x04245E, i_04245e),
    (0x042462, i_042462),
    (0x042466, i_042466),
    (0x042468, i_042468),
    (0x04246C, i_04246c),
    (0x042470, i_042470),
    (0x042474, i_042474),
    (0x042476, i_042476),
    (0x04247A, i_04247a),
    (0x04247C, i_04247c),
    (0x042480, i_042480),
    (0x042484, i_042484),
    (0x042488, i_042488),
    (0x04248C, i_04248c),
    (0x042490, i_042490),
    (0x042494, i_042494),
    (0x042498, i_042498),
    (0x04249C, i_04249c),
    (0x0424A2, i_0424a2),
    (0x0424A6, i_0424a6),
    (0x0424A8, i_0424a8),
    (0x0424AA, i_0424aa),
    (0x0424B0, i_0424b0),
    (0x0424B4, i_0424b4),
    (0x0424B8, i_0424b8),
    (0x0424BC, i_0424bc),
    (0x0424C0, i_0424c0),
    (0x0424C6, i_0424c6),
    (0x0424CA, i_0424ca),
    (0x0424CC, i_0424cc),
    (0x0424D0, i_0424d0),
    (0x0424D4, i_0424d4),
    (0x0424D8, i_0424d8),
    (0x0424DC, i_0424dc),
    (0x0424E0, i_0424e0),
    (0x0424E6, i_0424e6),
    (0x0424EA, i_0424ea),
    (0x0424F0, i_0424f0),
    (0x0424F4, i_0424f4),
    (0x0424F8, i_0424f8),
    (0x0424FC, i_0424fc),
    (0x0424FE, i_0424fe),
    (0x042504, i_042504),
    (0x042508, i_042508),
    (0x04250E, i_04250e),
    (0x042510, i_042510),
    (0x042512, i_042512),
    (0x042518, i_042518),
    (0x04251C, i_04251c),
    (0x042520, i_042520),
    (0x042524, i_042524),
    (0x042528, i_042528),
    (0x04252C, i_04252c),
    (0x042530, i_042530),
    (0x042534, i_042534),
    (0x042538, i_042538),
    (0x04253A, i_04253a),
    (0x04253E, i_04253e),
    (0x042542, i_042542),
    (0x04254A, i_04254a),
    (0x042550, i_042550),
    (0x042554, i_042554),
    (0x04255C, i_04255c),
    (0x042562, i_042562),
    (0x042566, i_042566),
    (0x042568, i_042568),
    (0x04256C, i_04256c),
    (0x042570, i_042570),
    (0x042574, i_042574),
    (0x042576, i_042576),
    (0x04257C, i_04257c),
    (0x042582, i_042582),
    (0x042586, i_042586),
    (0x04258C, i_04258c),
    (0x04258E, i_04258e),
    (0x042594, i_042594),
    (0x04259C, i_04259c),
    (0x0425A4, i_0425a4),
    (0x0425AA, i_0425aa),
    (0x0425B2, i_0425b2),
    (0x0425B6, i_0425b6),
    (0x0425BC, i_0425bc),
    (0x0425C4, i_0425c4),
    (0x0425CC, i_0425cc),
    (0x0425D2, i_0425d2),
    (0x0425DA, i_0425da),
    (0x0425E0, i_0425e0),
    (0x0425E6, i_0425e6),
    (0x0425EC, i_0425ec),
    (0x0425F0, i_0425f0),
    (0x0425F6, i_0425f6),
    (0x0425FC, i_0425fc),
    (0x042600, i_042600),
    (0x042604, i_042604),
    (0x04260A, i_04260a),
    (0x04260C, i_04260c),
    (0x042610, i_042610),
    (0x042614, i_042614),
    (0x042616, i_042616),
    (0x04261A, i_04261a),
    (0x04261C, i_04261c),
    (0x042622, i_042622),
    (0x042628, i_042628),
    (0x04262A, i_04262a),
    (0x04262C, i_04262c),
    (0x042630, i_042630),
    (0x042636, i_042636),
    (0x04263C, i_04263c),
    (0x04263E, i_04263e),
    (0x042642, i_042642),
    (0x042644, i_042644),
    (0x042646, i_042646),
    (0x04264A, i_04264a),
    (0x04264E, i_04264e),
    (0x042652, i_042652),
    (0x042656, i_042656),
    (0x042658, i_042658),
    (0x04265C, i_04265c),
    (0x042660, i_042660),
    (0x042662, i_042662),
    (0x042666, i_042666),
    (0x042668, i_042668),
    (0x04266C, i_04266c),
    (0x042670, i_042670),
    (0x042676, i_042676),
    (0x04267C, i_04267c),
    (0x042682, i_042682),
    (0x042686, i_042686),
    (0x04268C, i_04268c),
    (0x042690, i_042690),
    (0x042692, i_042692),
    (0x042696, i_042696),
    (0x04269A, i_04269a),
    (0x0426A0, i_0426a0),
    (0x0426A4, i_0426a4),
    (0x0426A8, i_0426a8),
    (0x0426AC, i_0426ac),
    (0x0426B0, i_0426b0),
    (0x0426B4, i_0426b4),
    (0x0426B8, i_0426b8),
    (0x0426BC, i_0426bc),
    (0x0426BE, i_0426be),
    (0x0426C2, i_0426c2),
    (0x0426C6, i_0426c6),
    (0x0426CA, i_0426ca),
    (0x0426CE, i_0426ce),
    (0x0426D4, i_0426d4),
    (0x0426D8, i_0426d8),
    (0x0426DE, i_0426de),
    (0x0426E4, i_0426e4),
    (0x0426E8, i_0426e8),
    (0x0426EA, i_0426ea),
    (0x0426EE, i_0426ee),
    (0x0426F4, i_0426f4),
    (0x0426F8, i_0426f8),
    (0x0426FC, i_0426fc),
    (0x042700, i_042700),
    (0x042704, i_042704),
    (0x042706, i_042706),
    (0x04270C, i_04270c),
    (0x042710, i_042710),
    (0x042716, i_042716),
    (0x04271C, i_04271c),
    (0x042720, i_042720),
    (0x042722, i_042722),
    (0x042726, i_042726),
    (0x04272C, i_04272c),
    (0x042730, i_042730),
    (0x042734, i_042734),
    (0x042736, i_042736),
    (0x04273C, i_04273c),
    (0x042740, i_042740),
    (0x042746, i_042746),
    (0x04274A, i_04274a),
    (0x042750, i_042750),
    (0x042754, i_042754),
    (0x042756, i_042756),
    (0x04275A, i_04275a),
    (0x04275E, i_04275e),
    (0x042764, i_042764),
    (0x042768, i_042768),
    (0x04276C, i_04276c),
    (0x04276E, i_04276e),
    (0x042772, i_042772),
    (0x042776, i_042776),
    (0x04277A, i_04277a),
    (0x04277E, i_04277e),
    (0x042782, i_042782),
    (0x042786, i_042786),
    (0x042788, i_042788),
    (0x04278E, i_04278e),
    (0x042792, i_042792),
    (0x042794, i_042794),
    (0x04279A, i_04279a),
    (0x0427A0, i_0427a0),
    (0x0427A4, i_0427a4),
    (0x0427A6, i_0427a6),
    (0x0427AA, i_0427aa),
    (0x0427B0, i_0427b0),
    (0x0427B4, i_0427b4),
    (0x0427B8, i_0427b8),
    (0x0427BC, i_0427bc),
    (0x0427BE, i_0427be),
    (0x0427C4, i_0427c4),
    (0x0427C8, i_0427c8),
    (0x0427CA, i_0427ca),
    (0x0427CE, i_0427ce),
    (0x0427D4, i_0427d4),
    (0x0427D8, i_0427d8),
    (0x0427DE, i_0427de),
    (0x0427E4, i_0427e4),
    (0x0427EA, i_0427ea),
    (0x0427F0, i_0427f0),
    (0x0427F6, i_0427f6),
    (0x0427F8, i_0427f8),
    (0x0428C6, i_0428c6),
    (0x0428CC, i_0428cc),
    (0x0428D0, i_0428d0),
    (0x0428D4, i_0428d4),
    (0x0428D8, i_0428d8),
    (0x0428DC, i_0428dc),
    (0x0428E0, i_0428e0),
    (0x0428E4, i_0428e4),
    (0x0428E8, i_0428e8),
    (0x0428EE, i_0428ee),
    (0x0428F2, i_0428f2),
    (0x0428F6, i_0428f6),
    (0x0428FC, i_0428fc),
    (0x042900, i_042900),
    (0x042904, i_042904),
    (0x04290A, i_04290a),
    (0x04290C, i_04290c),
    (0x042912, i_042912),
    (0x042916, i_042916),
    (0x04291A, i_04291a),
    (0x04291E, i_04291e),
    (0x042922, i_042922),
    (0x042928, i_042928),
    (0x04292C, i_04292c),
    (0x042932, i_042932),
    (0x042938, i_042938),
    (0x04293E, i_04293e),
    (0x042944, i_042944),
    (0x04294A, i_04294a),
    (0x04294E, i_04294e),
    (0x042954, i_042954),
    (0x042958, i_042958),
    (0x04295A, i_04295a),
    (0x04295E, i_04295e),
    (0x042960, i_042960),
    (0x042964, i_042964),
    (0x04296A, i_04296a),
    (0x042970, i_042970),
    (0x042972, i_042972),
    (0x042976, i_042976),
    (0x042978, i_042978),
    (0x04297A, i_04297a),
    (0x04297E, i_04297e),
    (0x042982, i_042982),
    (0x042986, i_042986),
    (0x04298A, i_04298a),
    (0x042990, i_042990),
    (0x042996, i_042996),
    (0x042998, i_042998),
    (0x04299C, i_04299c),
    (0x04299E, i_04299e),
    (0x0429A0, i_0429a0),
    (0x0429A4, i_0429a4),
    (0x0429A8, i_0429a8),
    (0x0429AC, i_0429ac),
    (0x0429B0, i_0429b0),
    (0x0429B6, i_0429b6),
    (0x0429BA, i_0429ba),
    (0x0429BE, i_0429be),
    (0x0429C2, i_0429c2),
    (0x0429C6, i_0429c6),
    (0x0429CA, i_0429ca),
    (0x0429CE, i_0429ce),
    (0x0429D2, i_0429d2),
    (0x0429D8, i_0429d8),
    (0x0429DC, i_0429dc),
    (0x0429E0, i_0429e0),
    (0x0429E4, i_0429e4),
    (0x0429EA, i_0429ea),
    (0x0429EE, i_0429ee),
    (0x0429F4, i_0429f4),
    (0x0429F8, i_0429f8),
    (0x0429FC, i_0429fc),
    (0x042A00, i_042a00),
    (0x042A02, i_042a02),
    (0x042A04, i_042a04),
    (0x042A08, i_042a08),
    (0x042A0C, i_042a0c),
    (0x042A10, i_042a10),
    (0x042A12, i_042a12),
    (0x042A14, i_042a14),
    (0x042A16, i_042a16),
    (0x042A1A, i_042a1a),
    (0x042A1E, i_042a1e),
    (0x042A20, i_042a20),
    (0x042A22, i_042a22),
    (0x042A26, i_042a26),
    (0x042A28, i_042a28),
    (0x042A2A, i_042a2a),
    (0x042A2E, i_042a2e),
    (0x042A34, i_042a34),
    (0x042A38, i_042a38),
    (0x042A3C, i_042a3c),
    (0x042A3E, i_042a3e),
    (0x042A42, i_042a42),
    (0x042A44, i_042a44),
    (0x042A48, i_042a48),
    (0x042A4E, i_042a4e),
    (0x042A52, i_042a52),
    (0x042A56, i_042a56),
    (0x042A5C, i_042a5c),
    (0x042A60, i_042a60),
    (0x042A66, i_042a66),
    (0x042A6A, i_042a6a),
    (0x042A6E, i_042a6e),
    (0x042A74, i_042a74),
    (0x042A7A, i_042a7a),
    (0x042A80, i_042a80),
    (0x042A86, i_042a86),
    (0x042A8C, i_042a8c),
    (0x042A90, i_042a90),
    (0x042A92, i_042a92),
    (0x042A96, i_042a96),
    (0x042A9A, i_042a9a),
    (0x042A9C, i_042a9c),
    (0x042AA0, i_042aa0),
    (0x042AA6, i_042aa6),
    (0x042AA8, i_042aa8),
    (0x042AAC, i_042aac),
    (0x042AB0, i_042ab0),
    (0x042AB2, i_042ab2),
    (0x042AB8, i_042ab8),
    (0x042ABC, i_042abc),
    (0x042AC0, i_042ac0),
    (0x042AC2, i_042ac2),
    (0x042AC8, i_042ac8),
    (0x042ACC, i_042acc),
    (0x042AD0, i_042ad0),
    (0x042AD4, i_042ad4),
    (0x042ADA, i_042ada),
    (0x042AE0, i_042ae0),
    (0x042AE4, i_042ae4),
    (0x042AEA, i_042aea),
    (0x042AF0, i_042af0),
    (0x042AF6, i_042af6),
    (0x042AFA, i_042afa),
    (0x042AFE, i_042afe),
    (0x042B00, i_042b00),
    (0x042B04, i_042b04),
    (0x042B08, i_042b08),
    (0x042B0E, i_042b0e),
    (0x042B12, i_042b12),
    (0x042B16, i_042b16),
    (0x042B1C, i_042b1c),
    (0x042B20, i_042b20),
    (0x042B24, i_042b24),
    (0x042B28, i_042b28),
    (0x042B2C, i_042b2c),
    (0x042B30, i_042b30),
    (0x042B32, i_042b32),
    (0x042B36, i_042b36),
    (0x042B3A, i_042b3a),
    (0x042B3E, i_042b3e),
    (0x042B42, i_042b42),
    (0x042B48, i_042b48),
    (0x042B4C, i_042b4c),
    (0x042B50, i_042b50),
    (0x042B52, i_042b52),
    (0x042B58, i_042b58),
    (0x042B5C, i_042b5c),
    (0x042B60, i_042b60),
    (0x042B66, i_042b66),
    (0x042B6C, i_042b6c),
    (0x042B72, i_042b72),
    (0x042B76, i_042b76),
    (0x042B7C, i_042b7c),
    (0x042B82, i_042b82),
    (0x042B86, i_042b86),
    (0x042B88, i_042b88),
    (0x042B8C, i_042b8c),
    (0x042B92, i_042b92),
    (0x042B96, i_042b96),
    (0x042B9A, i_042b9a),
    (0x042B9C, i_042b9c),
    (0x042BA2, i_042ba2),
    (0x042BA6, i_042ba6),
    (0x042BAA, i_042baa),
    (0x042BAE, i_042bae),
    (0x042BB2, i_042bb2),
    (0x042BB6, i_042bb6),
    (0x042BBC, i_042bbc),
    (0x042BC0, i_042bc0),
    (0x042BC8, i_042bc8),
    (0x042BD0, i_042bd0),
    (0x042BD6, i_042bd6),
    (0x042BDA, i_042bda),
    (0x042BE2, i_042be2),
    (0x042BE6, i_042be6),
    (0x042BEA, i_042bea),
    (0x042BEE, i_042bee),
    (0x042BF4, i_042bf4),
    (0x042BFA, i_042bfa),
    (0x042C00, i_042c00),
    (0x042C04, i_042c04),
    (0x042C06, i_042c06),
    (0x042C0A, i_042c0a),
    (0x042C10, i_042c10),
    (0x042C16, i_042c16),
    (0x042C1C, i_042c1c),
    (0x042C20, i_042c20),
    (0x042C22, i_042c22),
    (0x042C26, i_042c26),
    (0x042C2C, i_042c2c),
    (0x042C32, i_042c32),
    (0x042C36, i_042c36),
    (0x042C38, i_042c38),
    (0x042C3C, i_042c3c),
    (0x042C42, i_042c42),
    (0x042C48, i_042c48),
    (0x042C4C, i_042c4c),
    (0x042C52, i_042c52),
    (0x042C58, i_042c58),
    (0x042C5C, i_042c5c),
    (0x042C62, i_042c62),
    (0x042C66, i_042c66),
    (0x042C6A, i_042c6a),
    (0x042C6C, i_042c6c),
    (0x042C70, i_042c70),
    (0x042C74, i_042c74),
    (0x042C78, i_042c78),
    (0x042C7C, i_042c7c),
    (0x042C82, i_042c82),
    (0x042C86, i_042c86),
    (0x042C88, i_042c88),
    (0x042C8E, i_042c8e),
    (0x042C92, i_042c92),
    (0x042C96, i_042c96),
    (0x042C9A, i_042c9a),
    (0x042C9C, i_042c9c),
    (0x042CA0, i_042ca0),
    (0x042CA4, i_042ca4),
    (0x042CA6, i_042ca6),
    (0x042CAC, i_042cac),
    (0x042CB0, i_042cb0),
    (0x042CB4, i_042cb4),
    (0x042CB8, i_042cb8),
    (0x042CC0, i_042cc0),
    (0x042CC4, i_042cc4),
    (0x042CCA, i_042cca),
    (0x042CD0, i_042cd0),
    (0x042CD6, i_042cd6),
    (0x042CDA, i_042cda),
    (0x042CDE, i_042cde),
    (0x042CE0, i_042ce0),
    (0x042CE4, i_042ce4),
    (0x042CEA, i_042cea),
    (0x042CEE, i_042cee),
    (0x042CF2, i_042cf2),
    (0x042CF4, i_042cf4),
    (0x042CF8, i_042cf8),
    (0x042CFE, i_042cfe),
    (0x042D02, i_042d02),
    (0x042D08, i_042d08),
    (0x042D0E, i_042d0e),
    (0x042D12, i_042d12),
    (0x042D16, i_042d16),
    (0x042D18, i_042d18),
    (0x042D1C, i_042d1c),
    (0x042D22, i_042d22),
    (0x042D26, i_042d26),
    (0x042D2A, i_042d2a),
    (0x042D2C, i_042d2c),
    (0x042D32, i_042d32),
    (0x042D38, i_042d38),
    (0x042D3C, i_042d3c),
    (0x042D42, i_042d42),
    (0x042D46, i_042d46),
    (0x042D4C, i_042d4c),
    (0x042D52, i_042d52),
    (0x042D56, i_042d56),
    (0x042D5A, i_042d5a),
    (0x042D5C, i_042d5c),
    (0x042D62, i_042d62),
    (0x042D68, i_042d68),
    (0x042D6E, i_042d6e),
    (0x042D72, i_042d72),
    (0x042D74, i_042d74),
    (0x042D78, i_042d78),
    (0x042D7E, i_042d7e),
    (0x042D84, i_042d84),
    (0x042D8A, i_042d8a),
    (0x042D8E, i_042d8e),
    (0x042D90, i_042d90),
    (0x042D96, i_042d96),
    (0x042D9C, i_042d9c),
    (0x042DA0, i_042da0),
    (0x042DA4, i_042da4),
    (0x042DA8, i_042da8),
    (0x042DAC, i_042dac),
    (0x042DB0, i_042db0),
    (0x042DB4, i_042db4),
    (0x042DB8, i_042db8),
    (0x042DBE, i_042dbe),
    (0x042DC2, i_042dc2),
    (0x042DC6, i_042dc6),
    (0x042DCC, i_042dcc),
    (0x042DD0, i_042dd0),
    (0x042DD4, i_042dd4),
    (0x042DDA, i_042dda),
    (0x042DE0, i_042de0),
    (0x042DE4, i_042de4),
    (0x042DE8, i_042de8),
    (0x042DEA, i_042dea),
    (0x042DF0, i_042df0),
    (0x042DF6, i_042df6),
    (0x042DFA, i_042dfa),
    (0x042DFE, i_042dfe),
    (0x042E04, i_042e04),
    (0x042E0A, i_042e0a),
    (0x042E10, i_042e10),
    (0x042E14, i_042e14),
    (0x042E1A, i_042e1a),
    (0x042E20, i_042e20),
    (0x042E24, i_042e24),
    (0x042E26, i_042e26),
    (0x042E2C, i_042e2c),
    (0x042E32, i_042e32),
    (0x042E36, i_042e36),
    (0x042E3A, i_042e3a),
    (0x042E3E, i_042e3e),
    (0x042E42, i_042e42),
    (0x042E46, i_042e46),
    (0x042E4A, i_042e4a),
    (0x042E4E, i_042e4e),
    (0x042E54, i_042e54),
    (0x042E58, i_042e58),
    (0x042E5C, i_042e5c),
    (0x042E62, i_042e62),
    (0x042E66, i_042e66),
    (0x042E6A, i_042e6a),
    (0x042E70, i_042e70),
    (0x042E76, i_042e76),
    (0x042E7A, i_042e7a),
    (0x042E7E, i_042e7e),
    (0x042E80, i_042e80),
    (0x042E86, i_042e86),
    (0x04345E, i_04345e),
    (0x043464, i_043464),
    (0x043468, i_043468),
    (0x04346C, i_04346c),
    (0x043470, i_043470),
    (0x043474, i_043474),
    (0x043478, i_043478),
    (0x04347C, i_04347c),
    (0x043480, i_043480),
    (0x043486, i_043486),
    (0x043488, i_043488),
    (0x04348E, i_04348e),
    (0x043492, i_043492),
    (0x043496, i_043496),
    (0x04349A, i_04349a),
    (0x04349E, i_04349e),
    (0x0434A2, i_0434a2),
    (0x0434A6, i_0434a6),
    (0x0434AA, i_0434aa),
    (0x0434B0, i_0434b0),
    (0x0434B2, i_0434b2),
    (0x0434B6, i_0434b6),
    (0x0434BA, i_0434ba),
    (0x0434BE, i_0434be),
    (0x0434C2, i_0434c2),
    (0x0434C6, i_0434c6),
    (0x0434C8, i_0434c8),
    (0x0434CC, i_0434cc),
    (0x0434CE, i_0434ce),
    (0x0434D0, i_0434d0),
    (0x0434DA, i_0434da),
    (0x0434DC, i_0434dc),
    (0x0434DE, i_0434de),
    (0x0434E8, i_0434e8),
    (0x0434EA, i_0434ea),
    (0x0434F0, i_0434f0),
    (0x0434F6, i_0434f6),
    (0x0434F8, i_0434f8),
    (0x0434FE, i_0434fe),
    (0x043500, i_043500),
    (0x043506, i_043506),
    (0x043508, i_043508),
    (0x04350E, i_04350e),
    (0x043510, i_043510),
    (0x043516, i_043516),
    (0x04351A, i_04351a),
    (0x04351C, i_04351c),
    (0x04351E, i_04351e),
    (0x043520, i_043520),
    (0x043526, i_043526),
    (0x043528, i_043528),
    (0x04352E, i_04352e),
    (0x043532, i_043532),
    (0x043538, i_043538),
    (0x04353C, i_04353c),
    (0x043542, i_043542),
    (0x043546, i_043546),
    (0x04354C, i_04354c),
    (0x043552, i_043552),
    (0x043554, i_043554),
    (0x043556, i_043556),
    (0x043558, i_043558),
    (0x04355A, i_04355a),
    (0x043560, i_043560),
    (0x043562, i_043562),
    (0x043564, i_043564),
    (0x043568, i_043568),
    (0x04356C, i_04356c),
    (0x04356E, i_04356e),
    (0x043574, i_043574),
    (0x04357C, i_04357c),
    (0x043580, i_043580),
    (0x043582, i_043582),
    (0x043586, i_043586),
    (0x043588, i_043588),
    (0x04358A, i_04358a),
    (0x04358C, i_04358c),
    (0x043590, i_043590),
    (0x043594, i_043594),
    (0x043596, i_043596),
    (0x04359A, i_04359a),
    (0x04359E, i_04359e),
    (0x0435A2, i_0435a2),
    (0x0435A6, i_0435a6),
    (0x0435AA, i_0435aa),
    (0x0435AE, i_0435ae),
    (0x0435B0, i_0435b0),
    (0x0435B2, i_0435b2),
    (0x0435B4, i_0435b4),
    (0x0435B8, i_0435b8),
    (0x0435BE, i_0435be),
    (0x0435C0, i_0435c0),
    (0x0435C4, i_0435c4),
    (0x0435C8, i_0435c8),
    (0x0435CA, i_0435ca),
    (0x0435CC, i_0435cc),
    (0x0435D0, i_0435d0),
    (0x0435D4, i_0435d4),
    (0x0435DA, i_0435da),
    (0x0435DE, i_0435de),
    (0x0435E4, i_0435e4),
    (0x0435E6, i_0435e6),
    (0x0435EA, i_0435ea),
    (0x0435EE, i_0435ee),
    (0x0435F4, i_0435f4),
    (0x0435FA, i_0435fa),
    (0x043600, i_043600),
    (0x043604, i_043604),
    (0x043608, i_043608),
    (0x04360A, i_04360a),
    (0x04360E, i_04360e),
    (0x043610, i_043610),
    (0x043616, i_043616),
    (0x04361C, i_04361c),
    (0x043622, i_043622),
    (0x043628, i_043628),
    (0x04362E, i_04362e),
    (0x043634, i_043634),
    (0x04363C, i_04363c),
    (0x043644, i_043644),
    (0x04364C, i_04364c),
    (0x043654, i_043654),
    (0x04365C, i_04365c),
    (0x043664, i_043664),
    (0x04366C, i_04366c),
    (0x043672, i_043672),
    (0x043678, i_043678),
    (0x043682, i_043682),
    (0x04368C, i_04368c),
    (0x043696, i_043696),
    (0x04369C, i_04369c),
    (0x0436A0, i_0436a0),
    (0x0436A4, i_0436a4),
    (0x0436AA, i_0436aa),
    (0x0436B0, i_0436b0),
    (0x0436B6, i_0436b6),
    (0x0436BE, i_0436be),
    (0x0436C4, i_0436c4),
    (0x0436CA, i_0436ca),
    (0x0436D0, i_0436d0),
    (0x0436D6, i_0436d6),
    (0x0436DC, i_0436dc),
    (0x0436DE, i_0436de),
    (0x0436E4, i_0436e4),
    (0x0436E8, i_0436e8),
    (0x0436EA, i_0436ea),
    (0x0436EE, i_0436ee),
    (0x0436F0, i_0436f0),
    (0x0436F4, i_0436f4),
    (0x0436F6, i_0436f6),
    (0x0436F8, i_0436f8),
    (0x0436FC, i_0436fc),
    (0x043700, i_043700),
    (0x043702, i_043702),
    (0x043704, i_043704),
    (0x043706, i_043706),
    (0x043708, i_043708),
    (0x04370A, i_04370a),
    (0x04370E, i_04370e),
    (0x043710, i_043710),
    (0x043712, i_043712),
    (0x043716, i_043716),
    (0x043718, i_043718),
    (0x04371E, i_04371e),
    (0x043722, i_043722),
    (0x043726, i_043726),
    (0x04372A, i_04372a),
    (0x04372C, i_04372c),
    (0x04372E, i_04372e),
    (0x043734, i_043734),
    (0x04373A, i_04373a),
    (0x04373E, i_04373e),
    (0x043742, i_043742),
    (0x043746, i_043746),
    (0x043748, i_043748),
    (0x04374C, i_04374c),
    (0x04374E, i_04374e),
    (0x043750, i_043750),
    (0x043752, i_043752),
    (0x043754, i_043754),
    (0x043756, i_043756),
    (0x043758, i_043758),
    (0x04375A, i_04375a),
    (0x04375C, i_04375c),
    (0x04375E, i_04375e),
    (0x043760, i_043760),
    (0x043762, i_043762),
    (0x043764, i_043764),
    (0x043766, i_043766),
    (0x043768, i_043768),
    (0x04376E, i_04376e),
    (0x043774, i_043774),
    (0x043778, i_043778),
    (0x04377E, i_04377e),
    (0x043780, i_043780),
    (0x043782, i_043782),
    (0x043784, i_043784),
    (0x043786, i_043786),
    (0x043788, i_043788),
    (0x04378C, i_04378c),
    (0x04378E, i_04378e),
    (0x043792, i_043792),
    (0x043794, i_043794),
    (0x043798, i_043798),
    (0x04379A, i_04379a),
    (0x04379E, i_04379e),
    (0x0437A0, i_0437a0),
    (0x0437A4, i_0437a4),
    (0x0437A6, i_0437a6),
    (0x0437AA, i_0437aa),
    (0x0437AC, i_0437ac),
    (0x0437B0, i_0437b0),
    (0x0437B2, i_0437b2),
    (0x0437B6, i_0437b6),
    (0x0437B8, i_0437b8),
    (0x0437BA, i_0437ba),
    (0x0437C0, i_0437c0),
    (0x0437C2, i_0437c2),
    (0x0437C6, i_0437c6),
    (0x0437C8, i_0437c8),
    (0x0437CA, i_0437ca),
    (0x0437CC, i_0437cc),
    (0x0437D0, i_0437d0),
    (0x0437D2, i_0437d2),
    (0x0437D4, i_0437d4),
    (0x0437D6, i_0437d6),
    (0x0437D8, i_0437d8),
    (0x0437DA, i_0437da),
    (0x0437E0, i_0437e0),
    (0x0437E4, i_0437e4),
    (0x0437EA, i_0437ea),
    (0x0437F0, i_0437f0),
    (0x0437F4, i_0437f4),
    (0x0437FA, i_0437fa),
    (0x043800, i_043800),
    (0x043806, i_043806),
    (0x04380C, i_04380c),
    (0x043812, i_043812),
    (0x043814, i_043814),
    (0x043816, i_043816),
    (0x043818, i_043818),
    (0x04381C, i_04381c),
    (0x043820, i_043820),
    (0x043824, i_043824),
    (0x043828, i_043828),
    (0x04382C, i_04382c),
    (0x043830, i_043830),
    (0x043834, i_043834),
    (0x043838, i_043838),
    (0x043840, i_043840),
    (0x043844, i_043844),
    (0x04384C, i_04384c),
    (0x043850, i_043850),
    (0x043858, i_043858),
    (0x04385C, i_04385c),
    (0x043860, i_043860),
    (0x04394A, i_04394a),
    (0x04394E, i_04394e),
    (0x043954, i_043954),
    (0x043958, i_043958),
    (0x04395C, i_04395c),
    (0x043962, i_043962),
    (0x043966, i_043966),
    (0x04396A, i_04396a),
    (0x043970, i_043970),
    (0x043978, i_043978),
    (0x043980, i_043980),
    (0x043984, i_043984),
    (0x043988, i_043988),
    (0x04398E, i_04398e),
    (0x043996, i_043996),
    (0x04399E, i_04399e),
    (0x0439A6, i_0439a6),
    (0x0439AE, i_0439ae),
    (0x0439B6, i_0439b6),
    (0x0439F0, i_0439f0),
    (0x0439F4, i_0439f4),
    (0x0439FA, i_0439fa),
    (0x0439FE, i_0439fe),
    (0x043A02, i_043a02),
    (0x043A04, i_043a04),
    (0x043A08, i_043a08),
    (0x043A0C, i_043a0c),
    (0x043A12, i_043a12),
    (0x043A1A, i_043a1a),
    (0x043A42, i_043a42),
    (0x043A46, i_043a46),
    (0x043A4C, i_043a4c),
    (0x043A4E, i_043a4e),
    (0x043A54, i_043a54),
    (0x043A58, i_043a58),
    (0x043A5C, i_043a5c),
    (0x043A62, i_043a62),
    (0x043A66, i_043a66),
    (0x043A6A, i_043a6a),
    (0x043A70, i_043a70),
    (0x043A74, i_043a74),
    (0x043A78, i_043a78),
    (0x043A7E, i_043a7e),
    (0x043A82, i_043a82),
    (0x043A88, i_043a88),
    (0x043AA4, i_043aa4),
    (0x043AA8, i_043aa8),
    (0x043AAA, i_043aaa),
    (0x043AAE, i_043aae),
    (0x043AB4, i_043ab4),
    (0x043AB8, i_043ab8),
    (0x043ABC, i_043abc),
    (0x043AC2, i_043ac2),
    (0x043AC6, i_043ac6),
    (0x043ACA, i_043aca),
    (0x043AD0, i_043ad0),
    (0x043AD6, i_043ad6),
    (0x043ADA, i_043ada),
    (0x043ADE, i_043ade),
    (0x043AE4, i_043ae4),
    (0x043AE8, i_043ae8),
    (0x043AEC, i_043aec),
    (0x043AF0, i_043af0),
    (0x043AF6, i_043af6),
    (0x043AFA, i_043afa),
    (0x043AFE, i_043afe),
    (0x043B04, i_043b04),
    (0x043B08, i_043b08),
    (0x043B0C, i_043b0c),
    (0x043B14, i_043b14),
    (0x043B1A, i_043b1a),
    (0x043B1C, i_043b1c),
    (0x043B1E, i_043b1e),
    (0x043B20, i_043b20),
    (0x043B22, i_043b22),
    (0x043B28, i_043b28),
    (0x043B2E, i_043b2e),
    (0x043B34, i_043b34),
    (0x043B3A, i_043b3a),
    (0x043B40, i_043b40),
    (0x043B4A, i_043b4a),
    (0x043B54, i_043b54),
    (0x043B5E, i_043b5e),
    (0x043B62, i_043b62),
    (0x043B66, i_043b66),
    (0x043B6A, i_043b6a),
    (0x043B70, i_043b70),
    (0x043B74, i_043b74),
    (0x043B7A, i_043b7a),
    (0x043B80, i_043b80),
    (0x043B86, i_043b86),
    (0x043B8C, i_043b8c),
    (0x043B90, i_043b90),
    (0x043B94, i_043b94),
    (0x043B98, i_043b98),
    (0x043B9E, i_043b9e),
    (0x043BA2, i_043ba2),
    (0x043BA6, i_043ba6),
    (0x043BAC, i_043bac),
    (0x043BB0, i_043bb0),
    (0x043BB6, i_043bb6),
    (0x043BBC, i_043bbc),
    (0x043BC2, i_043bc2),
    (0x043BC6, i_043bc6),
    (0x043BC8, i_043bc8),
    (0x043BCE, i_043bce),
    (0x043BD4, i_043bd4),
    (0x043BDA, i_043bda),
    (0x043BDE, i_043bde),
    (0x043BE4, i_043be4),
    (0x043BEA, i_043bea),
    (0x043BEC, i_043bec),
    (0x043BF2, i_043bf2),
    (0x043BF8, i_043bf8),
    (0x043BFE, i_043bfe),
    (0x043C04, i_043c04),
    (0x043C08, i_043c08),
    (0x043C0C, i_043c0c),
    (0x043C12, i_043c12),
    (0x043C14, i_043c14),
    (0x043C18, i_043c18),
    (0x043C1E, i_043c1e),
    (0x043C20, i_043c20),
    (0x043C22, i_043c22),
    (0x043C28, i_043c28),
    (0x043C2E, i_043c2e),
    (0x043C30, i_043c30),
    (0x043C32, i_043c32),
    (0x043C34, i_043c34),
    (0x043C36, i_043c36),
    (0x043C3A, i_043c3a),
    (0x043C40, i_043c40),
    (0x043C46, i_043c46),
    (0x043C4C, i_043c4c),
    (0x043C50, i_043c50),
    (0x043C54, i_043c54),
    (0x043C5A, i_043c5a),
    (0x043C60, i_043c60),
    (0x043C64, i_043c64),
    (0x043C68, i_043c68),
    (0x043C6C, i_043c6c),
    (0x043C72, i_043c72),
    (0x043C78, i_043c78),
    (0x043C7C, i_043c7c),
    (0x043C80, i_043c80),
    (0x043C84, i_043c84),
    (0x043C8A, i_043c8a),
    (0x043C90, i_043c90),
    (0x043C94, i_043c94),
    (0x043C9A, i_043c9a),
    (0x043CA0, i_043ca0),
    (0x043CA6, i_043ca6),
    (0x043CAC, i_043cac),
    (0x043CB0, i_043cb0),
    (0x043CB6, i_043cb6),
    (0x043CBA, i_043cba),
    (0x043CC0, i_043cc0),
    (0x043CC6, i_043cc6),
    (0x043CCC, i_043ccc),
    (0x043CD2, i_043cd2),
    (0x043CD8, i_043cd8),
    (0x043CDE, i_043cde),
    (0x043CE2, i_043ce2),
    (0x043CE8, i_043ce8),
    (0x043CEC, i_043cec),
    (0x043CF2, i_043cf2),
    (0x043CF6, i_043cf6),
    (0x043CFC, i_043cfc),
    (0x043D00, i_043d00),
    (0x043D04, i_043d04),
    (0x043D0A, i_043d0a),
    (0x043D10, i_043d10),
    (0x043D14, i_043d14),
    (0x043D18, i_043d18),
    (0x043D1C, i_043d1c),
    (0x043D20, i_043d20),
    (0x043D26, i_043d26),
    (0x043D2C, i_043d2c),
    (0x043D30, i_043d30),
    (0x043D34, i_043d34),
    (0x043D38, i_043d38),
    (0x043D3C, i_043d3c),
    (0x043D42, i_043d42),
    (0x043D48, i_043d48),
    (0x043D4C, i_043d4c),
    (0x043D50, i_043d50),
    (0x043D56, i_043d56),
    (0x043D5C, i_043d5c),
    (0x043D60, i_043d60),
    (0x043D64, i_043d64),
    (0x043D66, i_043d66),
    (0x043D6A, i_043d6a),
    (0x043D6C, i_043d6c),
    (0x043D72, i_043d72),
    (0x043D78, i_043d78),
    (0x043D7E, i_043d7e),
    (0x043D84, i_043d84),
    (0x043D86, i_043d86),
    (0x043D8C, i_043d8c),
    (0x043D90, i_043d90),
    (0x043D96, i_043d96),
    (0x043D9A, i_043d9a),
    (0x043DA0, i_043da0),
    (0x043DA4, i_043da4),
    (0x043DAA, i_043daa),
    (0x043DAE, i_043dae),
];
