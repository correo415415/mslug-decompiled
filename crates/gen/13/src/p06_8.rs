// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $06D464: beq.w $6d46e
pub fn i_06d464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D468;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D46E; return 10; }
    8
}
/// $06D468: bclr.b #$4, $13(a6)
pub fn i_06d468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D46E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D46E: bset.b #$5, $13(a6)
pub fn i_06d46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D474;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D474: btst.b #$1, d0
pub fn i_06d474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D478;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $06D478: beq.w $6d482
pub fn i_06d478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D47C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D482; return 10; }
    8
}
/// $06D47C: bclr.b #$5, $13(a6)
pub fn i_06d47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D482;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D482: rts
pub fn i_06d482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D484;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_Fire_06d484  ($06D484, 62 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D484: lea.l $2ccfea.l, a1
pub fn i_06d484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D48A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CCFEA;
    20
}
/// $06D48A: lea.l $2ccffa.l, a2
pub fn i_06d48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D490;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2CCFFA;
    20
}
/// $06D490: lea.l $2cd00a.l, a3
pub fn i_06d490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D496;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x2CD00A;
    20
}
/// $06D496: tst.b $9e(a6)
pub fn i_06d496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D49A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06D49A: beq.w $6d4a4
pub fn i_06d49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D49E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D4A4; return 10; }
    8
}
/// $06D49E: lea.l $2cd01a.l, a3
pub fn i_06d49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4A4;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x2CD01A;
    20
}
/// $06D4A4: move.b $97(a6), d0
pub fn i_06d4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4A8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D4A8: andi.w #$3, d0
pub fn i_06d4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4AC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D4AC: lsl.w #$2, d0
pub fn i_06d4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4AE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06D4AE: move.l (a1, d0.w), $48(a6)
pub fn i_06d4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4B4;
    cpu.ir = 0x2D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06D4B4: move.l (a2, d0.w), $60(a6)
pub fn i_06d4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4BA;
    cpu.ir = 0x2D72;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06D4BA: move.l (a3, d0.w), $4c(a6)
pub fn i_06d4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4C0;
    cpu.ir = 0x2D73;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06D4C0: rts
pub fn i_06d4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_FireRocket_06d4c2  ($06D4C2, 178 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D4C2: move.w #$1064, d0
pub fn i_06d4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D4C6: jsr $2352.l
pub fn i_06d4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D4CC);
    cpu.pc = 0x2352;
    16
}
/// $06D4CC: move.w $34(a6), d0
pub fn i_06d4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D4D0: btst.b #$0, $3a(a6)
pub fn i_06d4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4D6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D4D6: beq.w $6d4e4
pub fn i_06d4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D4E4; return 10; }
    8
}
/// $06D4DA: lea.l $2cd5f6.l, a1
pub fn i_06d4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4E0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CD5F6;
    20
}
/// $06D4E0: move.b (a1, d0.w), d0
pub fn i_06d4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4E4;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06D4E4: andi.w #$f, d0
pub fn i_06d4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4E8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D4E8: move.w d0, $5c(a6)
pub fn i_06d4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4EC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D4EC: lea.l $6ce8a(pc), a1
pub fn i_06d4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CE8A;
    16
}
/// $06D4F0: jsr $4ae.l
pub fn i_06d4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D4F6);
    cpu.pc = 0x4AE;
    16
}
/// $06D4F6: jsr $5dd02.l
pub fn i_06d4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D4FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D4FC);
    cpu.pc = 0x5DD02;
    16
}
/// $06D4FC: lea.l $2cd03a.l, a1
pub fn i_06d4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D502;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CD03A;
    20
}
/// $06D502: move.b $97(a6), d0
pub fn i_06d502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D506;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D506: andi.w #$3, d0
pub fn i_06d506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D50A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D50A: lsl.w #$2, d0
pub fn i_06d50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D50C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06D50C: movea.l (a1, d0.w), a1
pub fn i_06d50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D510;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06D510: move.w $34(a6), d0
pub fn i_06d510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D514;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D514: move.w d0, $34(a0)
pub fn i_06d514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D518;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D518: lsl.w #$2, d0
pub fn i_06d518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D51A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06D51A: move.w (a1, d0.w), d1
pub fn i_06d51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D51E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06D51E: move.w $2(a1, d0.w), d2
pub fn i_06d51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D522;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06D522: btst.b #$0, $3a(a6)
pub fn i_06d522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D528;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D528: beq.w $6d52e
pub fn i_06d528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D52C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D52E; return 10; }
    8
}
/// $06D52C: neg.w d1
pub fn i_06d52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D52E;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D52E: move.w d1, d3
pub fn i_06d52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D530;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D530: move.w d2, d4
pub fn i_06d530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D532;
    cpu.ir = 0x3802;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D532: move.b d3, $8e(a0)
pub fn i_06d532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D536;
    cpu.ir = 0x1143;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D536: move.b d4, $8f(a0)
pub fn i_06d536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D53A;
    cpu.ir = 0x1144;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D53A: move.b $97(a6), $97(a0)
pub fn i_06d53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D540;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x97);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06D540: movem.w d1-d2, -(a7)
pub fn i_06d540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D544;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $06D544: lea.l $6cf06(pc), a1
pub fn i_06d544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CF06;
    16
}
/// $06D548: jsr $4ae.l
pub fn i_06d548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D54E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D54E);
    cpu.pc = 0x4AE;
    16
}
/// $06D54E: jsr $5dd02.l
pub fn i_06d54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D554;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D554);
    cpu.pc = 0x5DD02;
    16
}
/// $06D554: movem.w (a7)+, d1-d2
pub fn i_06d554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D558;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06D558: move.w $5c(a6), $34(a0)
pub fn i_06d558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D55E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06D55E: add.w d1, $22(a0)
pub fn i_06d55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D562;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D562: add.w d2, $24(a0)
pub fn i_06d562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D566;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D566: move.b $98(a6), $98(a0)
pub fn i_06d566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D56C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06D56C: move.b $97(a6), $97(a0)
pub fn i_06d56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D572;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x97);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06D572: rts
pub fn i_06d572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D574;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_PlayCry_06d574  ($06D574, 32 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D574: lea.l $2ccf6e.l, a1
pub fn i_06d574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D57A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CCF6E;
    20
}
/// $06D57A: move.b $96(a6), d0
pub fn i_06d57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D57E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D57E: andi.w #$3, d0
pub fn i_06d57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D582;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D582: lsl.w #$1, d0
pub fn i_06d582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D584;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06D584: move.w (a1, d0.w), d1
pub fn i_06d584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D588;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06D588: jsr $236e.l
pub fn i_06d588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D58E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D58E);
    cpu.pc = 0x236E;
    16
}
/// $06D58E: move.w #$7, $1c(a6)
pub fn i_06d58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D594;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_06d594  ($06D594, 8 B, thunks/jsr_abs_thunks.c)
/// $06D594: jsr $138fe.l
pub fn i_06d594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D59A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D59A);
    cpu.pc = 0x138FE;
    16
}
/// $06D59A: rts
pub fn i_06d59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D59C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_DeathDebris_06d59c  ($06D59C, 48 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D59C: move.w #$1033, d0
pub fn i_06d59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D5A0: jsr $2352.l
pub fn i_06d5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D5A6);
    cpu.pc = 0x2352;
    16
}
/// $06D5A6: lea.l $2ccf4a.l, a1
pub fn i_06d5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CCF4A;
    20
}
/// $06D5AC: jsr $77c7e.l
pub fn i_06d5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D5B2);
    cpu.pc = 0x77C7E;
    16
}
/// $06D5B2: move.w #$4000, $38(a0)
pub fn i_06d5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5B8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D5B8: lea.l $2ccf5c.l, a1
pub fn i_06d5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CCF5C;
    20
}
/// $06D5BE: jsr $77c7e.l
pub fn i_06d5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D5C4);
    cpu.pc = 0x77C7E;
    16
}
/// $06D5C4: move.w #$4000, $38(a0)
pub fn i_06d5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5CA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D5CA: rts
pub fn i_06d5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_DeathExplodeA_06d5cc  ($06D5CC, 54 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D5CC: move.w #$1021, d0
pub fn i_06d5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D5D0: jsr $2352.l
pub fn i_06d5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D5D6);
    cpu.pc = 0x2352;
    16
}
/// $06D5D6: lea.l $2ccf5c.l, a1
pub fn i_06d5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5DC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CCF5C;
    20
}
/// $06D5DC: jsr $77c7e.l
pub fn i_06d5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D5E2);
    cpu.pc = 0x77C7E;
    16
}
/// $06D5E2: move.w #$4000, $38(a0)
pub fn i_06d5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5E8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D5E8: lea.l $77fd6.l, a1
pub fn i_06d5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5EE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06D5EE: jsr $4ae.l
pub fn i_06d5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D5F4);
    cpu.pc = 0x4AE;
    16
}
/// $06D5F4: jsr $5dd02.l
pub fn i_06d5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D5FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D5FA);
    cpu.pc = 0x5DD02;
    16
}
/// $06D5FA: move.w #$4000, $38(a0)
pub fn i_06d5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D600;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D600: rts
pub fn i_06d600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D602;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_DeathExplodeB_06d602  ($06D602, 54 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D602: move.w #$1021, d0
pub fn i_06d602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D606;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D606: jsr $2352.l
pub fn i_06d606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D60C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D60C);
    cpu.pc = 0x2352;
    16
}
/// $06D60C: lea.l $2ccf5c.l, a1
pub fn i_06d60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D612;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CCF5C;
    20
}
/// $06D612: jsr $77c7e.l
pub fn i_06d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D618;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D618);
    cpu.pc = 0x77C7E;
    16
}
/// $06D618: move.w #$4000, $38(a0)
pub fn i_06d618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D61E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D61E: lea.l $77fd6.l, a1
pub fn i_06d61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D624;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06D624: jsr $4ae.l
pub fn i_06d624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D62A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D62A);
    cpu.pc = 0x4AE;
    16
}
/// $06D62A: jsr $5dd02.l
pub fn i_06d62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D630);
    cpu.pc = 0x5DD02;
    16
}
/// $06D630: move.w #$4000, $38(a0)
pub fn i_06d630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D636;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D636: rts
pub fn i_06d636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_CmpSlotWithParent_06d638  ($06D638, 16 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D638: movea.l $8(a6), a1
pub fn i_06d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D63C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06D63C: move.b $10(a6), d0
pub fn i_06d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D640;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D640: cmp.b $10(a1), d0
pub fn i_06d640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D644;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06D644: bcs.w $6d64e
pub fn i_06d644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D648;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6D64E; return 10; }
    8
}
// ===== ClearXN_06d648  ($06D648, 6 B, thunks/ccr_helpers.c)
/// $06D648: andi.b #$ee, ccr
pub fn i_06d648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D64C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D64C: rts
pub fn i_06d64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D64E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06d64e  ($06D64E, 6 B, thunks/ccr_helpers.c)
/// $06D64E: ori.b #$11, ccr
pub fn i_06d64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D652;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D652: rts
pub fn i_06d652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D654;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_Tmpl42_06d654  ($06D654, 154 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D654: clr.b $7e(a6)
pub fn i_06d654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D658;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D658: clr.b $7f(a6)
pub fn i_06d658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D65C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D65C: bra.w $6d67a
pub fn i_06d65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D660;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D67A;
    10
}
/// $06D660: move.b #$1, $7e(a6)
pub fn i_06d660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D666;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D666: clr.b $7f(a6)
pub fn i_06d666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D66A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D66A: bra.w $6d67a
pub fn i_06d66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D66E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D67A;
    10
}
/// $06D66E: move.b #$1, $7e(a6)
pub fn i_06d66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D674;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D674: move.b #$1, $7f(a6)
pub fn i_06d674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D67A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D67A: jsr $6d6ee(pc)
pub fn i_06d67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D67E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D67E);
    cpu.pc = 0x6D6EE;
    16
}
/// $06D67E: clr.w $70(a6)
pub fn i_06d67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D682;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D682: bra.w $6d7a2
pub fn i_06d682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D686;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D7A2;
    10
}
/// $06D686: clr.b $7e(a6)
pub fn i_06d686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D68A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D68A: clr.b $7f(a6)
pub fn i_06d68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D68E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D68E: bra.w $6d6ac
pub fn i_06d68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D692;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D6AC;
    10
}
/// $06D692: move.b #$1, $7e(a6)
pub fn i_06d692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D698;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D698: clr.b $7f(a6)
pub fn i_06d698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D69C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D69C: bra.w $6d6ac
pub fn i_06d69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D6AC;
    10
}
/// $06D6A0: move.b #$1, $7e(a6)
pub fn i_06d6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D6A6: move.b #$1, $7f(a6)
pub fn i_06d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D6AC: jsr $6d6ee(pc)
pub fn i_06d6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D6B0);
    cpu.pc = 0x6D6EE;
    16
}
/// $06D6B0: move.w #$1, $70(a6)
pub fn i_06d6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D6B6: bra.w $6d83a
pub fn i_06d6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D83A;
    10
}
/// $06D6BA: clr.b $7e(a6)
pub fn i_06d6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6BE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D6BE: clr.b $7f(a6)
pub fn i_06d6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6C2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D6C2: bra.w $6d6e0
pub fn i_06d6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D6E0;
    10
}
/// $06D6C6: move.b #$1, $7e(a6)
pub fn i_06d6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D6CC: clr.b $7f(a6)
pub fn i_06d6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6D0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06D6D0: bra.w $6d6e0
pub fn i_06d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D6E0;
    10
}
/// $06D6D4: move.b #$1, $7e(a6)
pub fn i_06d6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D6DA: move.b #$1, $7f(a6)
pub fn i_06d6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D6E0: jsr $6d6ee(pc)
pub fn i_06d6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D6E4);
    cpu.pc = 0x6D6EE;
    16
}
/// $06D6E4: move.w #$2, $70(a6)
pub fn i_06d6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D6EA: bra.w $6d83a
pub fn i_06d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D83A;
    10
}
// ===== Walker_Init_06d6ee  ($06D6EE, 282 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D6EE: jsr $5e7c0.l
pub fn i_06d6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D6F4);
    cpu.pc = 0x5E7C0;
    16
}
/// $06D6F4: jsr $267e2.l
pub fn i_06d6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D6FA);
    cpu.pc = 0x267E2;
    16
}
/// $06D6FA: jsr $6e31e(pc)
pub fn i_06d6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D6FE);
    cpu.pc = 0x6E31E;
    16
}
/// $06D6FE: move.w #$8, $1c(a6)
pub fn i_06d6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D704;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D704: jsr $138fe.l
pub fn i_06d704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D70A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D70A);
    cpu.pc = 0x138FE;
    16
}
/// $06D70A: lea.l $2b8d6c.l, a0
pub fn i_06d70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D710;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8D6C;
    20
}
/// $06D710: jsr $799de.l
pub fn i_06d710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D716);
    cpu.pc = 0x799DE;
    16
}
/// $06D716: move.w d0, $66(a6)
pub fn i_06d716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D71A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D71A: move.w $66(a6), $7a(a6)
pub fn i_06d71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D720;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06D720: move.l #$2d3bae, $48(a6)
pub fn i_06d720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D728;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3BAE;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06D728: move.l #$2d4056, $60(a6)
pub fn i_06d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D730;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D4056;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06D730: move.b #$0, $20(a6)
pub fn i_06d730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D736;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D736: clr.w $76(a6)
pub fn i_06d736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D73A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D73A: move.b $9a(a6), d0
pub fn i_06d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D73E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D73E: tst.b d0
pub fn i_06d73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D740;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $06D740: bne.w $6d748
pub fn i_06d740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D744;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D748; return 10; }
    8
}
/// $06D744: move.b #$15, d0
pub fn i_06d744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D748;
    cpu.ir = 0x103C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06D748: andi.w #$ff, d0
pub fn i_06d748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D74C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D74C: lsl.w #$4, d0
pub fn i_06d74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D74E;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06D74E: move.w d0, $86(a6)
pub fn i_06d74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D752;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D752: move.w #$4000, d0
pub fn i_06d752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D756;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D756: jsr $28134.l
pub fn i_06d756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D75C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D75C);
    cpu.pc = 0x28134;
    16
}
/// $06D75C: andi.w #$ffe3, $38(a6)
pub fn i_06d75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D762;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06D762: ori.w #$14, $38(a6)
pub fn i_06d762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D768;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06D768: tst.b $9d(a6)
pub fn i_06d768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D76C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06D76C: beq.w $6d786
pub fn i_06d76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D770;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D786; return 10; }
    8
}
/// $06D770: move.w #$8000, d0
pub fn i_06d770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D774;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D774: jsr $28134.l
pub fn i_06d774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D77A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D77A);
    cpu.pc = 0x28134;
    16
}
/// $06D77A: andi.w #$ffe3, $38(a6)
pub fn i_06d77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D780;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06D780: ori.w #$14, $38(a6)
pub fn i_06d780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D786;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06D786: jsr $6e484(pc)
pub fn i_06d786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D78A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D78A);
    cpu.pc = 0x6E484;
    16
}
/// $06D78A: lea.l $723d2.l, a1
pub fn i_06d78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D790;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723D2;
    20
}
/// $06D790: jsr $4ae.l
pub fn i_06d790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D796;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D796);
    cpu.pc = 0x4AE;
    16
}
/// $06D796: jsr $5dd02.l
pub fn i_06d796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D79C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D79C);
    cpu.pc = 0x5DD02;
    16
}
/// $06D79C: clr.w $98(a0)
pub fn i_06d79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7A0;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D7A0: rts
pub fn i_06d7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $06D7A2: clr.w $76(a6)
pub fn i_06d7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7A6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D7A6: lea.l $2d428c.l, a0
pub fn i_06d7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D428C;
    20
}
/// $06D7AC: jsr $28cd4.l
pub fn i_06d7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D7B2);
    cpu.pc = 0x28CD4;
    16
}
/// $06D7B2: lea.l $6d7b8(pc), a1
pub fn i_06d7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D7B8;
    16
}
/// $06D7B6: move.l a1, (a6)
pub fn i_06d7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D7B8: jsr $28998.l
pub fn i_06d7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D7BE);
    cpu.pc = 0x28998;
    16
}
/// $06D7BE: jsr $2783a.l
pub fn i_06d7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D7C4);
    cpu.pc = 0x2783A;
    16
}
/// $06D7C4: jsr $28d70.l
pub fn i_06d7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D7CA);
    cpu.pc = 0x28D70;
    16
}
/// $06D7CA: move.w $22(a6), d0
pub fn i_06d7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D7CE: cmp.w $86(a6), d0
pub fn i_06d7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7D2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06D7D2: bge.w $6d7f6
pub fn i_06d7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7D6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x6D7F6; return 10; }
    8
}
/// $06D7D6: jsr $5e0d4.l
pub fn i_06d7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D7DC);
    cpu.pc = 0x5E0D4;
    16
}
/// $06D7DC: bcs.w $6d7f6
pub fn i_06d7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6D7F6; return 10; }
    8
}
/// $06D7E0: move.w $22(a6), d0
pub fn i_06d7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7E4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D7E4: sub.w $22(a0), d0
pub fn i_06d7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7E8;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D7E8: cmpi.w #$60, d0
pub fn i_06d7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7EC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $06D7EC: bgt.w $6d7f6
pub fn i_06d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7F0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D7F6; return 10; }
    8
}
/// $06D7F0: lea.l $6d808(pc), a1
pub fn i_06d7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D808;
    16
}
/// $06D7F4: move.l a1, (a6)
pub fn i_06d7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D7F6: tst.w $76(a6)
pub fn i_06d7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7FA;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $06D7FA: beq.w $6d804
pub fn i_06d7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D7FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D804; return 10; }
    8
}
/// $06D7FE: lea.l $6d808(pc), a1
pub fn i_06d7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D802;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D808;
    16
}
/// $06D802: move.l a1, (a6)
pub fn i_06d802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D804;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D804: bra.w $6d986
pub fn i_06d804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D808;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D986;
    10
}
// ===== Walker_Approach_06d808  ($06D808, 124 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D808: lea.l $2d42b0.l, a0
pub fn i_06d808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D80E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D42B0;
    20
}
/// $06D80E: jsr $28cd4.l
pub fn i_06d80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D814);
    cpu.pc = 0x28CD4;
    16
}
/// $06D814: lea.l $6d81a(pc), a1
pub fn i_06d814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D818;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D81A;
    16
}
/// $06D818: move.l a1, (a6)
pub fn i_06d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D81A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D81A: jsr $28998.l
pub fn i_06d81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D820);
    cpu.pc = 0x28998;
    16
}
/// $06D820: jsr $2783a.l
pub fn i_06d820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D826);
    cpu.pc = 0x2783A;
    16
}
/// $06D826: jsr $28d70.l
pub fn i_06d826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D82C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D82C);
    cpu.pc = 0x28D70;
    16
}
/// $06D82C: bcc.w $6d836
pub fn i_06d82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D830;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D836; return 10; }
    8
}
/// $06D830: lea.l $6d900(pc), a1
pub fn i_06d830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D834;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D900;
    16
}
/// $06D834: move.l a1, (a6)
pub fn i_06d834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D836;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D836: bra.w $6d986
pub fn i_06d836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D83A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D986;
    10
}
/// $06D83A: lea.l $2d4298.l, a0
pub fn i_06d83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D840;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4298;
    20
}
/// $06D840: jsr $28cd4.l
pub fn i_06d840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D846);
    cpu.pc = 0x28CD4;
    16
}
/// $06D846: lea.l $6d84c(pc), a1
pub fn i_06d846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D84A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D84C;
    16
}
/// $06D84A: move.l a1, (a6)
pub fn i_06d84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D84C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D84C: jsr $28998.l
pub fn i_06d84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D852);
    cpu.pc = 0x28998;
    16
}
/// $06D852: jsr $2783a.l
pub fn i_06d852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D858;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D858);
    cpu.pc = 0x2783A;
    16
}
/// $06D858: jsr $28d70.l
pub fn i_06d858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D85E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D85E);
    cpu.pc = 0x28D70;
    16
}
/// $06D85E: move.w $22(a6), d0
pub fn i_06d85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D862;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D862: cmp.w $86(a6), d0
pub fn i_06d862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D866;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06D866: bge.w $6d880
pub fn i_06d866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D86A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x6D880; return 10; }
    8
}
/// $06D86A: lea.l $2d4036.l, a0
pub fn i_06d86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D870;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4036;
    20
}
/// $06D870: jsr $5e086.l
pub fn i_06d870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D876);
    cpu.pc = 0x5E086;
    16
}
/// $06D876: bcs.w $6d880
pub fn i_06d876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D87A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6D880; return 10; }
    8
}
/// $06D87A: lea.l $6d900(pc), a1
pub fn i_06d87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D87E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D900;
    16
}
/// $06D87E: move.l a1, (a6)
pub fn i_06d87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D880;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D880: bra.w $6d986
pub fn i_06d880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D884;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D986;
    10
}
// ===== Walker_AttackPause_06d884  ($06D884, 124 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D884: lea.l $2b8dee.l, a0
pub fn i_06d884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D88A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8DEE;
    20
}
/// $06D88A: jsr $799de.l
pub fn i_06d88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D890;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D890);
    cpu.pc = 0x799DE;
    16
}
/// $06D890: move.w d0, $72(a6)
pub fn i_06d890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D894;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D894: lea.l $2d42a4.l, a0
pub fn i_06d894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D89A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D42A4;
    20
}
/// $06D89A: jsr $28cd4.l
pub fn i_06d89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D8A0);
    cpu.pc = 0x28CD4;
    16
}
/// $06D8A0: lea.l $6d8a6(pc), a1
pub fn i_06d8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D8A6;
    16
}
/// $06D8A4: move.l a1, (a6)
pub fn i_06d8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D8A6: jsr $28998.l
pub fn i_06d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D8AC);
    cpu.pc = 0x28998;
    16
}
/// $06D8AC: jsr $2783a.l
pub fn i_06d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D8B2);
    cpu.pc = 0x2783A;
    16
}
/// $06D8B2: jsr $28d70.l
pub fn i_06d8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D8B8);
    cpu.pc = 0x28D70;
    16
}
/// $06D8B8: subq.w #$1, $72(a6)
pub fn i_06d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8BC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D8BC: cmpi.w #$0, $72(a6)
pub fn i_06d8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8C2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06D8C2: bgt.w $6d8fc
pub fn i_06d8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8C6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D8FC; return 10; }
    8
}
/// $06D8C6: cmpi.w #$0, $70(a6)
pub fn i_06d8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06D8CC: bne.w $6d8d6
pub fn i_06d8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D8D6; return 10; }
    8
}
/// $06D8D0: lea.l $6d916(pc), a1
pub fn i_06d8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D916;
    16
}
/// $06D8D4: move.l a1, (a6)
pub fn i_06d8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D8D6: cmpi.w #$2, $70(a6)
pub fn i_06d8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8DC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $06D8DC: bne.w $6d8e6
pub fn i_06d8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D8E6; return 10; }
    8
}
/// $06D8E0: lea.l $6d916(pc), a1
pub fn i_06d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D916;
    16
}
/// $06D8E4: move.l a1, (a6)
pub fn i_06d8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D8E6: lea.l $2d4036.l, a0
pub fn i_06d8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4036;
    20
}
/// $06D8EC: jsr $5e086.l
pub fn i_06d8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D8F2);
    cpu.pc = 0x5E086;
    16
}
/// $06D8F2: bcs.w $6d8fc
pub fn i_06d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8F6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6D8FC; return 10; }
    8
}
/// $06D8F6: lea.l $6d916(pc), a1
pub fn i_06d8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D916;
    16
}
/// $06D8FA: move.l a1, (a6)
pub fn i_06d8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D8FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D8FC: bra.w $6d986
pub fn i_06d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D900;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D986;
    10
}
// ===== Walker_AttackOnce_06d900  ($06D900, 22 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D900: move.w #$1, $78(a6)
pub fn i_06d900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D906;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D906: lea.l $2d42e8.l, a0
pub fn i_06d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D90C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D42E8;
    20
}
/// $06D90C: jsr $28cd4.l
pub fn i_06d90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D912);
    cpu.pc = 0x28CD4;
    16
}
/// $06D912: bra.w $6d932
pub fn i_06d912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D916;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D932;
    10
}
// ===== Walker_AttackBurst_06d916  ($06D916, 16 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D916: lea.l $2b8ef2.l, a0
pub fn i_06d916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D91C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8EF2;
    20
}
/// $06D91C: jsr $799de.l
pub fn i_06d91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D922);
    cpu.pc = 0x799DE;
    16
}
/// $06D922: move.w d0, $78(a6)
pub fn i_06d922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D926;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Walker_Attack_06d926  ($06D926, 190 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D926: lea.l $2d431e.l, a0
pub fn i_06d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D92C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D431E;
    20
}
/// $06D92C: jsr $28cd4.l
pub fn i_06d92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D932);
    cpu.pc = 0x28CD4;
    16
}
/// $06D932: lea.l $2b8e70.l, a0
pub fn i_06d932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D938;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8E70;
    20
}
/// $06D938: jsr $799de.l
pub fn i_06d938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D93E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D93E);
    cpu.pc = 0x799DE;
    16
}
/// $06D93E: move.w d0, $72(a6)
pub fn i_06d93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D942;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D942: lea.l $6d948(pc), a1
pub fn i_06d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D948;
    16
}
/// $06D946: move.l a1, (a6)
pub fn i_06d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D948;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D948: jsr $28998.l
pub fn i_06d948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D94E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D94E);
    cpu.pc = 0x28998;
    16
}
/// $06D94E: jsr $2783a.l
pub fn i_06d94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D954);
    cpu.pc = 0x2783A;
    16
}
/// $06D954: jsr $28d70.l
pub fn i_06d954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D95A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D95A);
    cpu.pc = 0x28D70;
    16
}
/// $06D95A: bcc.w $6d986
pub fn i_06d95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D95E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D986; return 10; }
    8
}
/// $06D95E: subq.w #$1, $72(a6)
pub fn i_06d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D962;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D962: cmpi.w #$0, $72(a6)
pub fn i_06d962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D968;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06D968: bgt.w $6d986
pub fn i_06d968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D96C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D986; return 10; }
    8
}
/// $06D96C: lea.l $6d926(pc), a1
pub fn i_06d96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D926;
    16
}
/// $06D970: move.l a1, (a6)
pub fn i_06d970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D972: subq.w #$1, $78(a6)
pub fn i_06d972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D976;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D976: cmpi.w #$0, $78(a6)
pub fn i_06d976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D97C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06D97C: bgt.w $6d986
pub fn i_06d97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D980;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D986; return 10; }
    8
}
/// $06D980: lea.l $6d884(pc), a1
pub fn i_06d980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D984;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D884;
    16
}
/// $06D984: move.l a1, (a6)
pub fn i_06d984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D986;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D986: jsr $6e394(pc)
pub fn i_06d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D98A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D98A);
    cpu.pc = 0x6E394;
    16
}
/// $06D98A: jsr $2870a.l
pub fn i_06d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D990);
    cpu.pc = 0x2870A;
    16
}
/// $06D990: bcc.w $6d9a6
pub fn i_06d990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D994;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D9A6; return 10; }
    8
}
/// $06D994: bclr.b #$3, $13(a6)
pub fn i_06d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D99A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D99A: lea.l $5e766.l, a0
pub fn i_06d99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06D9A0: jsr $5e770.l
pub fn i_06d9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D9A6);
    cpu.pc = 0x5E770;
    16
}
/// $06D9A6: jsr $28758.l
pub fn i_06d9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D9AC);
    cpu.pc = 0x28758;
    16
}
/// $06D9AC: bcc.w $6d9b6
pub fn i_06d9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D9B6; return 10; }
    8
}
/// $06D9B0: lea.l $6d9ec(pc), a1
pub fn i_06d9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D9EC;
    16
}
/// $06D9B4: move.l a1, (a6)
pub fn i_06d9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D9B6: tst.b $99(a6)
pub fn i_06d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9BA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06D9BA: beq.w $6d9ce
pub fn i_06d9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D9CE; return 10; }
    8
}
/// $06D9BE: jsr $27eba.l
pub fn i_06d9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D9C4);
    cpu.pc = 0x27EBA;
    16
}
/// $06D9C4: bcc.w $6d9ce
pub fn i_06d9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D9CE; return 10; }
    8
}
/// $06D9C8: lea.l $6da4c(pc), a1
pub fn i_06d9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA4C;
    16
}
/// $06D9CC: move.l a1, (a6)
pub fn i_06d9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D9CE: movea.l #$ffffffff, a0
pub fn i_06d9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9D4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06D9D4: lea.l $2d402e.l, a0
pub fn i_06d9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D402E;
    20
}
/// $06D9DA: jsr $5dd5c.l
pub fn i_06d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D9E0);
    cpu.pc = 0x5DD5C;
    16
}
/// $06D9E0: bcc.w $6d9ea
pub fn i_06d9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D9EA; return 10; }
    8
}
// ===== SetTaskHandler_06d9e4  ($06D9E4, 8 B, kernel/task_handlers.c)
/// $06D9E4: lea.l $6da7a(pc), a1
pub fn i_06d9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA7A;
    16
}
/// $06D9E8: move.l a1, (a6)
pub fn i_06d9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D9EA: rts
pub fn i_06d9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_Die_06d9ec  ($06D9EC, 88 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D9EC: bclr.b #$1, $12(a6)
pub fn i_06d9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9F2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D9F2: jsr $267e2.l
pub fn i_06d9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D9F8);
    cpu.pc = 0x267E2;
    16
}
/// $06D9F8: lea.l $2d4394.l, a0
pub fn i_06d9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D9FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4394;
    20
}
/// $06D9FE: jsr $28cd4.l
pub fn i_06d9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA04);
    cpu.pc = 0x28CD4;
    16
}
/// $06DA04: lea.l $6da0a(pc), a1
pub fn i_06da04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA0A;
    16
}
/// $06DA08: move.l a1, (a6)
pub fn i_06da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DA0A: jsr $2783a.l
pub fn i_06da0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA10);
    cpu.pc = 0x2783A;
    16
}
/// $06DA10: jsr $28d70.l
pub fn i_06da10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA16);
    cpu.pc = 0x28D70;
    16
}
/// $06DA16: tst.b $99(a6)
pub fn i_06da16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA1A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06DA1A: beq.w $6da2e
pub fn i_06da1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DA2E; return 10; }
    8
}
/// $06DA1E: jsr $27eba.l
pub fn i_06da1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA24);
    cpu.pc = 0x27EBA;
    16
}
/// $06DA24: bcc.w $6da2e
pub fn i_06da24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA28;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DA2E; return 10; }
    8
}
/// $06DA28: lea.l $6da4c(pc), a1
pub fn i_06da28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA4C;
    16
}
/// $06DA2C: move.l a1, (a6)
pub fn i_06da2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DA2E: movea.l #$ffffffff, a0
pub fn i_06da2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA34;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06DA34: lea.l $2d402e.l, a0
pub fn i_06da34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D402E;
    20
}
/// $06DA3A: jsr $5dd5c.l
pub fn i_06da3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA40);
    cpu.pc = 0x5DD5C;
    16
}
/// $06DA40: bcc.w $6da4a
pub fn i_06da40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DA4A; return 10; }
    8
}
// ===== SetTaskHandler_06da44  ($06DA44, 8 B, kernel/task_handlers.c)
/// $06DA44: lea.l $6da7a(pc), a1
pub fn i_06da44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA7A;
    16
}
/// $06DA48: move.l a1, (a6)
pub fn i_06da48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DA4A: rts
pub fn i_06da4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_Explode_06da4c  ($06DA4C, 44 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DA4C: move.w #$1033, d0
pub fn i_06da4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA50;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DA50: jsr $2352.l
pub fn i_06da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA56);
    cpu.pc = 0x2352;
    16
}
/// $06DA56: lea.l $77fd6.l, a1
pub fn i_06da56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA5C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06DA5C: jsr $4ae.l
pub fn i_06da5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA62);
    cpu.pc = 0x4AE;
    16
}
/// $06DA62: jsr $5dd02.l
pub fn i_06da62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA68);
    cpu.pc = 0x5DD02;
    16
}
/// $06DA68: move.w #$8000, $38(a0)
pub fn i_06da68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA6E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DA6E: jsr $6e412(pc)
pub fn i_06da6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DA72);
    cpu.pc = 0x6E412;
    16
}
/// $06DA72: jmp $518.l
pub fn i_06da72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA78;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Walker_Nop_06da78  ($06DA78, 2 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DA78: rts
pub fn i_06da78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_06da7a  ($06DA7A, 14 B, kernel/jsr5b6_jmp518.c)
/// $06DA7A: jsr $5b6.l
pub fn i_06da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA80);
    cpu.pc = 0x5B6;
    16
}
/// $06DA80: jmp $518.l
pub fn i_06da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA86;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06DA86: rts
pub fn i_06da86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06da88  ($06DA88, 8 B, kernel/jmp_to_scheduler.c)
/// $06DA88: jmp $518.l
pub fn i_06da88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA8E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06DA8E: rts
pub fn i_06da8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_06da90  ($06DA90, 14 B, kernel/jsr5b6_jmp518.c)
/// $06DA90: jsr $5b6.l
pub fn i_06da90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DA96);
    cpu.pc = 0x5B6;
    16
}
/// $06DA96: jmp $518.l
pub fn i_06da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA9C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06DA9C: rts
pub fn i_06da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DA9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_ExplodeB_06da9e  ($06DA9E, 36 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DA9E: move.w #$4000, d0
pub fn i_06da9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAA2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DAA2: jsr $28134.l
pub fn i_06daa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DAA8);
    cpu.pc = 0x28134;
    16
}
/// $06DAA8: andi.w #$ffe3, $38(a6)
pub fn i_06daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAAE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DAAE: ori.w #$1c, $38(a6)
pub fn i_06daae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAB4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DAB4: move.l #$ffffffff, $48(a6)
pub fn i_06dab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DABC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DABC: jmp $77f6a.l
pub fn i_06dabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAC2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Frag_ExplodeC_06dac2  ($06DAC2, 42 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DAC2: jsr $13600.l
pub fn i_06dac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DAC8);
    cpu.pc = 0x13600;
    16
}
/// $06DAC8: move.w #$4000, d0
pub fn i_06dac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DACC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DACC: jsr $28134.l
pub fn i_06dacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DAD2);
    cpu.pc = 0x28134;
    16
}
/// $06DAD2: andi.w #$ffe3, $38(a6)
pub fn i_06dad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAD8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DAD8: ori.w #$1c, $38(a6)
pub fn i_06dad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DADE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DADE: move.l #$ffffffff, $48(a6)
pub fn i_06dade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAE6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DAE6: jmp $77fd6.l
pub fn i_06dae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAEC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
// ===== Frag_Shell_06daec  ($06DAEC, 224 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DAEC: jsr $13600.l
pub fn i_06daec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DAF2);
    cpu.pc = 0x13600;
    16
}
/// $06DAF2: move.w #$4000, d0
pub fn i_06daf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAF6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DAF6: jsr $28134.l
pub fn i_06daf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DAFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DAFC);
    cpu.pc = 0x28134;
    16
}
/// $06DAFC: andi.w #$ffe3, $38(a6)
pub fn i_06dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB02;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DB02: ori.w #$0, $38(a6)
pub fn i_06db02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB08;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DB08: move.l #$ffffffff, $48(a6)
pub fn i_06db08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB10;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DB10: jmp $77fd6.l
pub fn i_06db10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB16;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
/// $06DB16: tst.b $89(a6)
pub fn i_06db16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB1A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06DB1A: bne.b $6daec
pub fn i_06db1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB1C;
    cpu.ir = 0x66D0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6DAEC; return 10; }
    8
}
/// $06DB1C: jsr $13600.l
pub fn i_06db1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB22);
    cpu.pc = 0x13600;
    16
}
/// $06DB22: move.w #$4000, d0
pub fn i_06db22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB26;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DB26: jsr $28134.l
pub fn i_06db26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB2C);
    cpu.pc = 0x28134;
    16
}
/// $06DB2C: andi.w #$ffe3, $38(a6)
pub fn i_06db2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB32;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DB32: ori.w #$1c, $38(a6)
pub fn i_06db32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB38;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DB38: move.w #$1021, d0
pub fn i_06db38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB3C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DB3C: jsr $2352.l
pub fn i_06db3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB42);
    cpu.pc = 0x2352;
    16
}
/// $06DB42: lea.l $2d40b4.l, a1
pub fn i_06db42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB48;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D40B4;
    20
}
/// $06DB48: jsr $77c7e.l
pub fn i_06db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB4E);
    cpu.pc = 0x77C7E;
    16
}
/// $06DB4E: move.w #$c000, $38(a0)
pub fn i_06db4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB54;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DB54: move.w #$4, d1
pub fn i_06db54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB58;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DB58: jsr $236e.l
pub fn i_06db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB5E);
    cpu.pc = 0x236E;
    16
}
/// $06DB5E: move.w #$2, $72(a6)
pub fn i_06db5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB64;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DB64: move.l #$2d3f8a, $4c(a6)
pub fn i_06db64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB6C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3F8A;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DB6C: jsr $283ca.l
pub fn i_06db6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB72);
    cpu.pc = 0x283CA;
    16
}
/// $06DB72: lea.l $2dd37e.l, a0
pub fn i_06db72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $06DB78: jsr $28cd4.l
pub fn i_06db78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB7E);
    cpu.pc = 0x28CD4;
    16
}
/// $06DB7E: lea.l $6db84(pc), a1
pub fn i_06db7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DB84;
    16
}
/// $06DB82: move.l a1, (a6)
pub fn i_06db82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DB84: jsr $2783a.l
pub fn i_06db84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB8A);
    cpu.pc = 0x2783A;
    16
}
/// $06DB8A: jsr $28d70.l
pub fn i_06db8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DB90);
    cpu.pc = 0x28D70;
    16
}
/// $06DB90: bcc.w $6db9a
pub fn i_06db90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DB9A; return 10; }
    8
}
/// $06DB94: lea.l $6da90(pc), a1
pub fn i_06db94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DB98: move.l a1, (a6)
pub fn i_06db98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DB9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DB9A: jsr $283d8.l
pub fn i_06db9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DBA0);
    cpu.pc = 0x283D8;
    16
}
/// $06DBA0: btst.b #$1, $13(a6)
pub fn i_06dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBA6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06DBA6: subq.w #$1, $72(a6)
pub fn i_06dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBAA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06DBAA: cmpi.w #$0, $72(a6)
pub fn i_06dbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06DBB0: bgt.w $6dbbc
pub fn i_06dbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBB4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6DBBC; return 10; }
    8
}
/// $06DBB4: move.l #$ffffffff, $4c(a6)
pub fn i_06dbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBBC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DBBC: movea.l #$ffffffff, a0
pub fn i_06dbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBC2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06DBC2: jsr $5dd56.l
pub fn i_06dbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DBC8);
    cpu.pc = 0x5DD56;
    16
}
/// $06DBC8: bcc.w $6dbd2
pub fn i_06dbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBCC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DBD2; return 10; }
    8
}
// ===== SetTaskHandler_06dbcc  ($06DBCC, 8 B, kernel/task_handlers.c)
/// $06DBCC: lea.l $6da90(pc), a1
pub fn i_06dbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DBD0: move.l a1, (a6)
pub fn i_06dbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DBD2: rts
pub fn i_06dbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_Debris_06dbd4  ($06DBD4, 130 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DBD4: move.b #$1, $89(a6)
pub fn i_06dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06DBDA: bra.w $6dbe2
pub fn i_06dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DBE2;
    10
}
/// $06DBDE: jsr $6e176(pc)
pub fn i_06dbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DBE2);
    cpu.pc = 0x6E176;
    16
}
/// $06DBE2: bset.b #$4, $6b(a6)
pub fn i_06dbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBE8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DBE8: jsr $6e356(pc)
pub fn i_06dbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DBEC);
    cpu.pc = 0x6E356;
    16
}
/// $06DBEC: move.w #$d000, d0
pub fn i_06dbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBF0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DBF0: jsr $28134.l
pub fn i_06dbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DBF6);
    cpu.pc = 0x28134;
    16
}
/// $06DBF6: andi.w #$ffe3, $38(a6)
pub fn i_06dbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DBFC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DBFC: ori.w #$14, $38(a6)
pub fn i_06dbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC02;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DC02: lea.l $2d45fc.l, a0
pub fn i_06dc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D45FC;
    20
}
/// $06DC08: jsr $28cd4.l
pub fn i_06dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC0E);
    cpu.pc = 0x28CD4;
    16
}
/// $06DC0E: lea.l $6dc14(pc), a1
pub fn i_06dc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DC14;
    16
}
/// $06DC12: move.l a1, (a6)
pub fn i_06dc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DC14: jsr $27d50.l
pub fn i_06dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC1A);
    cpu.pc = 0x27D50;
    16
}
/// $06DC1A: bcc.w $6dc24
pub fn i_06dc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DC24; return 10; }
    8
}
/// $06DC1E: lea.l $6db16(pc), a1
pub fn i_06dc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DB16;
    16
}
/// $06DC22: move.l a1, (a6)
pub fn i_06dc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DC24: jsr $28d70.l
pub fn i_06dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC2A);
    cpu.pc = 0x28D70;
    16
}
/// $06DC2A: jsr $283d8.l
pub fn i_06dc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC30);
    cpu.pc = 0x283D8;
    16
}
/// $06DC30: btst.b #$1, $13(a6)
pub fn i_06dc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC36;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06DC36: beq.w $6dc40
pub fn i_06dc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DC40; return 10; }
    8
}
/// $06DC3A: lea.l $6dac2(pc), a1
pub fn i_06dc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DAC2;
    16
}
/// $06DC3E: move.l a1, (a6)
pub fn i_06dc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DC40: movea.l #$ffffffff, a0
pub fn i_06dc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC46;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06DC46: lea.l $2d404e.l, a0
pub fn i_06dc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D404E;
    20
}
/// $06DC4C: jsr $5dd56.l
pub fn i_06dc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC52);
    cpu.pc = 0x5DD56;
    16
}
/// $06DC52: bcc.w $6dc5c
pub fn i_06dc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DC5C; return 10; }
    8
}
// ===== SetTaskHandler_06dc56  ($06DC56, 8 B, kernel/task_handlers.c)
/// $06DC56: lea.l $6da90(pc), a1
pub fn i_06dc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DC5A: move.l a1, (a6)
pub fn i_06dc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DC5C: rts
pub fn i_06dc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_Smoke_06dc5e  ($06DC5E, 42 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DC5E: jsr $6e356(pc)
pub fn i_06dc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DC62);
    cpu.pc = 0x6E356;
    16
}
/// $06DC62: lea.l $2d4628.l, a0
pub fn i_06dc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC68;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4628;
    20
}
/// $06DC68: jsr $28cd4.l
pub fn i_06dc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC6E);
    cpu.pc = 0x28CD4;
    16
}
/// $06DC6E: lea.l $6dc74(pc), a1
pub fn i_06dc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DC74;
    16
}
/// $06DC72: move.l a1, (a6)
pub fn i_06dc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DC74: jsr $6e34a(pc)
pub fn i_06dc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DC78);
    cpu.pc = 0x6E34A;
    16
}
/// $06DC78: jsr $2783a.l
pub fn i_06dc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC7E);
    cpu.pc = 0x2783A;
    16
}
/// $06DC7E: jsr $28d70.l
pub fn i_06dc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DC84);
    cpu.pc = 0x28D70;
    16
}
/// $06DC84: bcc.w $6dc8e
pub fn i_06dc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DC8E; return 10; }
    8
}
// ===== SetTaskHandler_06dc88  ($06DC88, 8 B, kernel/task_handlers.c)
/// $06DC88: lea.l $6da90(pc), a1
pub fn i_06dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DC8C: move.l a1, (a6)
pub fn i_06dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DC8E: rts
pub fn i_06dc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_Spark_06dc90  ($06DC90, 72 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DC90: eori.b #$1, $3a(a6)
pub fn i_06dc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC96;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DC96: move.w #$8, d1
pub fn i_06dc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC9A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DC9A: jsr $236e.l
pub fn i_06dc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DCA0);
    cpu.pc = 0x236E;
    16
}
/// $06DCA0: move.w #$4000, d0
pub fn i_06dca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DCA4: jsr $28134.l
pub fn i_06dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DCAA);
    cpu.pc = 0x28134;
    16
}
/// $06DCAA: andi.w #$ffe3, $38(a6)
pub fn i_06dcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCB0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DCB0: ori.w #$0, $38(a6)
pub fn i_06dcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCB6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DCB6: lea.l $2d455a.l, a0
pub fn i_06dcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCBC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D455A;
    20
}
/// $06DCBC: jsr $28cd4.l
pub fn i_06dcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DCC2);
    cpu.pc = 0x28CD4;
    16
}
/// $06DCC2: lea.l $6dcc8(pc), a1
pub fn i_06dcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DCC8;
    16
}
/// $06DCC6: move.l a1, (a6)
pub fn i_06dcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DCC8: jsr $2783a.l
pub fn i_06dcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DCCE);
    cpu.pc = 0x2783A;
    16
}
/// $06DCCE: jsr $28d70.l
pub fn i_06dcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DCD4);
    cpu.pc = 0x28D70;
    16
}
/// $06DCD4: bcc.w $6dcde
pub fn i_06dcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DCDE; return 10; }
    8
}
// ===== SetTaskHandler_06dcd8  ($06DCD8, 8 B, kernel/task_handlers.c)
/// $06DCD8: lea.l $6da90(pc), a1
pub fn i_06dcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DCDC: move.l a1, (a6)
pub fn i_06dcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DCDE: rts
pub fn i_06dcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_ScatterSlow_06dce0  ($06DCE0, 124 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DCE0: cmpi.w #$0, $36(a6)
pub fn i_06dce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCE6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06DCE6: bgt.w $6dcf0
pub fn i_06dce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCEA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6DCF0; return 10; }
    8
}
/// $06DCEA: move.w #$200, $36(a6)
pub fn i_06dcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCF0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DCF0: move.w #$1, $80(a6)
pub fn i_06dcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCF6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DCF6: move.w #$1, $82(a6)
pub fn i_06dcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DCFC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DCFC: move.w #$8000, d0
pub fn i_06dcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DD00: jsr $28134.l
pub fn i_06dd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DD06);
    cpu.pc = 0x28134;
    16
}
/// $06DD06: andi.w #$ffe3, $38(a6)
pub fn i_06dd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD0C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DD0C: ori.w #$14, $38(a6)
pub fn i_06dd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD12;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DD12: bra.w $6dde4
pub fn i_06dd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD16;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DDE4;
    10
}
/// $06DD16: cmpi.w #$0, $36(a6)
pub fn i_06dd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD1C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06DD1C: bgt.w $6dd26
pub fn i_06dd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD20;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6DD26; return 10; }
    8
}
/// $06DD20: move.w #$200, $36(a6)
pub fn i_06dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD26;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DD26: move.w #$8000, d0
pub fn i_06dd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD2A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DD2A: jsr $28134.l
pub fn i_06dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DD30);
    cpu.pc = 0x28134;
    16
}
/// $06DD30: andi.w #$ffe3, $38(a6)
pub fn i_06dd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD36;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DD36: ori.w #$14, $38(a6)
pub fn i_06dd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD3C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DD3C: move.w #$1, $80(a6)
pub fn i_06dd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DD42: move.w #$1, $82(a6)
pub fn i_06dd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DD48: jsr $5e9b6.l
pub fn i_06dd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DD4E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DD4E: move.w d0, d3
pub fn i_06dd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD50;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DD50: andi.w #$3e, d0
pub fn i_06dd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD54;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3E) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DD54: addi.w #$50, d0
pub fn i_06dd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD58;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x50, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DD58: bra.w $6ddf4
pub fn i_06dd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DDF4;
    10
}
// ===== Frag_Scatter_06dd5c  ($06DD5C, 418 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DD5C: jsr $5e9b6.l
pub fn i_06dd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DD62);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DD62: andi.w #$f, d0
pub fn i_06dd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD66;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DD66: movea.l #$2d4106, a0
pub fn i_06dd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD6C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D4106;
    cpu.a[0] = v1;
    4
}
/// $06DD6C: lsl.w #$2, d0
pub fn i_06dd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD6E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06DD6E: movea.l (a0, d0.w), a0
pub fn i_06dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD72;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06DD72: cmpa.l #$ffffffff, a0
pub fn i_06dd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD78;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06DD78: beq.w $6dd82
pub fn i_06dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD7C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DD82; return 10; }
    8
}
/// $06DD7C: jsr $28cd4.l
pub fn i_06dd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DD82);
    cpu.pc = 0x28CD4;
    16
}
/// $06DD82: bra.w $6ddac
pub fn i_06dd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD86;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DDAC;
    10
}
/// $06DD86: jsr $5e9b6.l
pub fn i_06dd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DD8C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DD8C: andi.w #$f, d0
pub fn i_06dd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD90;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DD90: movea.l #$2d4146, a0
pub fn i_06dd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD96;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D4146;
    cpu.a[0] = v1;
    4
}
/// $06DD96: lsl.w #$2, d0
pub fn i_06dd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD98;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06DD98: movea.l (a0, d0.w), a0
pub fn i_06dd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DD9C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06DD9C: cmpa.l #$ffffffff, a0
pub fn i_06dd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDA2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06DDA2: beq.w $6ddac
pub fn i_06dda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DDAC; return 10; }
    8
}
/// $06DDA6: jsr $28cd4.l
pub fn i_06dda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DDAC);
    cpu.pc = 0x28CD4;
    16
}
/// $06DDAC: move.w #$4000, d0
pub fn i_06ddac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDB0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DDB0: jsr $28134.l
pub fn i_06ddb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DDB6);
    cpu.pc = 0x28134;
    16
}
/// $06DDB6: andi.w #$ffe3, $38(a6)
pub fn i_06ddb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDBC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DDBC: ori.w #$0, $38(a6)
pub fn i_06ddbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDC2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DDC2: jsr $5e9b6.l
pub fn i_06ddc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DDC8);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DDC8: andi.w #$700, d0
pub fn i_06ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDCC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DDCC: addi.w #$200, d0
pub fn i_06ddcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDD0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DDD0: move.w d0, $36(a6)
pub fn i_06ddd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDD4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DDD4: move.w #$3, $80(a6)
pub fn i_06ddd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDDA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DDDA: move.w #$3, $82(a6)
pub fn i_06ddda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDE0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DDE0: jsr $6e31e(pc)
pub fn i_06dde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DDE4);
    cpu.pc = 0x6E31E;
    16
}
/// $06DDE4: jsr $5e9b6.l
pub fn i_06dde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DDEA);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DDEA: move.w d0, d3
pub fn i_06ddea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDEC;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DDEC: andi.w #$3e, d0
pub fn i_06ddec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDF0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3E) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DDF0: addi.w #$20, d0
pub fn i_06ddf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDF4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DDF4: add.w d0, d0
pub fn i_06ddf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDF6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06DDF6: lea.l $2c072c.l, a1
pub fn i_06ddf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DDFC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $06DDFC: move.w (a1, d0.w), d1
pub fn i_06ddfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE00;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06DE00: lea.l $2c07ac.l, a1
pub fn i_06de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE06;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $06DE06: move.w (a1, d0.w), d2
pub fn i_06de06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE0A;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06DE0A: andi.w #$7, d3
pub fn i_06de0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE0E;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DE0E: addq.w #$1, d3
pub fn i_06de0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE10;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06DE10: muls.w d3, d1
pub fn i_06de10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE12;
    cpu.ir = 0xC3C3;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE12: asr.w #$2, d1
pub fn i_06de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE14;
    cpu.ir = 0xE441;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06DE14: move.w $36(a6), d0
pub fn i_06de14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE18;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE18: muls.w d0, d1
pub fn i_06de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE1A;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE1A: muls.w d0, d2
pub fn i_06de1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE1C;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE1C: asr.l #$8, d1
pub fn i_06de1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE1E;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $06DE1E: asr.l #$8, d2
pub fn i_06de1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE20;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $06DE20: move.w d1, $2a(a6)
pub fn i_06de20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE24;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE24: move.w d2, $28(a6)
pub fn i_06de24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE28;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE28: jsr $5e9b6.l
pub fn i_06de28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE2E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DE2E: cmpi.b #$0, d0
pub fn i_06de2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE32;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $06DE32: bgt.w $6de66
pub fn i_06de32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE36;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6DE66; return 10; }
    8
}
/// $06DE36: jsr $5e9b6.l
pub fn i_06de36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE3C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DE3C: andi.w #$3, d0
pub fn i_06de3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE40;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DE40: addq.w #$2, d0
pub fn i_06de40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE42;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06DE42: move.w $28(a6), d1
pub fn i_06de42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE46;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE46: muls.w d1, d0
pub fn i_06de46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE48;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE48: asr.w #$1, d0
pub fn i_06de48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE4A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06DE4A: move.w d0, $28(a6)
pub fn i_06de4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE4E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE4E: jsr $5e9b6.l
pub fn i_06de4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE54);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DE54: andi.w #$3, d0
pub fn i_06de54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE58;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DE58: addq.w #$2, d0
pub fn i_06de58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE5A;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06DE5A: move.w $2a(a6), d1
pub fn i_06de5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE5E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE5E: muls.w d1, d0
pub fn i_06de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE60;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE60: asr.w #$1, d0
pub fn i_06de60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE62;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06DE62: move.w d0, $2a(a6)
pub fn i_06de62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE66;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE66: move.w $2a(a6), d0
pub fn i_06de66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE6A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE6A: move.w d0, $84(a6)
pub fn i_06de6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE6E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE6E: move.w #$ffd0, $2e(a6)
pub fn i_06de6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE74: lea.l $6de7a(pc), a1
pub fn i_06de74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DE7A;
    16
}
/// $06DE78: move.l a1, (a6)
pub fn i_06de78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DE7A: jsr $27d50.l
pub fn i_06de7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE80);
    cpu.pc = 0x27D50;
    16
}
/// $06DE80: bcc.w $6deb6
pub fn i_06de80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DEB6; return 10; }
    8
}
/// $06DE84: move.w #$ffd0, $2e(a6)
pub fn i_06de84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE8A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE8A: move.w $28(a6), d0
pub fn i_06de8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE8E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE8E: asr.w #$2, d0
pub fn i_06de8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE90;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06DE90: move.w d0, $28(a6)
pub fn i_06de90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE94;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE94: move.w $84(a6), d0
pub fn i_06de94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE98;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE98: asr.w #$1, d0
pub fn i_06de98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE9A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06DE9A: move.w d0, $2a(a6)
pub fn i_06de9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE9E: move.w d0, $84(a6)
pub fn i_06de9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEA2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DEA2: subq.w #$1, $80(a6)
pub fn i_06dea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEA6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06DEA6: cmpi.w #$0, $80(a6)
pub fn i_06dea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEAC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06DEAC: bgt.w $6deb6
pub fn i_06deac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEB0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6DEB6; return 10; }
    8
}
/// $06DEB0: lea.l $6da90(pc), a1
pub fn i_06deb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DEB4: move.l a1, (a6)
pub fn i_06deb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DEB6: move.w $82(a6), d0
pub fn i_06deb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEBA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DEBA: cmp.w $80(a6), d0
pub fn i_06deba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEBE;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06DEBE: beq.w $6ded0
pub fn i_06debe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DED0; return 10; }
    8
}
/// $06DEC2: addq.b #$1, $72(a6)
pub fn i_06dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEC6;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06DEC6: btst.b #$0, $72(a6)
pub fn i_06dec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DECC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06DECC: beq.w $6dee8
pub fn i_06decc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DED0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DEE8; return 10; }
    8
}
/// $06DED0: jsr $5e804.l
pub fn i_06ded0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DED6);
    cpu.pc = 0x5E804;
    16
}
/// $06DED6: bcc.w $6dee2
pub fn i_06ded6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DEE2; return 10; }
    8
}
/// $06DEDA: andi.b #$ee, ccr
pub fn i_06deda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06DEDE: bra.w $6dee8
pub fn i_06dede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DEE8;
    10
}
/// $06DEE2: jsr $28d70.l
pub fn i_06dee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DEE8);
    cpu.pc = 0x28D70;
    16
}
/// $06DEE8: movea.l #$ffffffff, a0
pub fn i_06dee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEEE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06DEEE: lea.l $2d403e.l, a0
pub fn i_06deee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D403E;
    20
}
/// $06DEF4: jsr $5dd56.l
pub fn i_06def4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DEFA);
    cpu.pc = 0x5DD56;
    16
}
/// $06DEFA: bcc.w $6df04
pub fn i_06defa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DF04; return 10; }
    8
}
// ===== SetTaskHandler_06defe  ($06DEFE, 8 B, kernel/task_handlers.c)
/// $06DEFE: lea.l $6da90(pc), a1
pub fn i_06defe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DF02: move.l a1, (a6)
pub fn i_06df02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DF04: rts
pub fn i_06df04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_Wide_06df06  ($06DF06, 44 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DF06: jsr $5e9b6.l
pub fn i_06df06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF0C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF0C: andi.w #$f00, d0
pub fn i_06df0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF10;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF10: addi.w #$800, d0
pub fn i_06df10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF14;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x800, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF14: jsr $6e20c(pc)
pub fn i_06df14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DF18);
    cpu.pc = 0x6E20C;
    16
}
/// $06DF18: move.w d0, $28(a6)
pub fn i_06df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF1C: jsr $5e9b6.l
pub fn i_06df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF22);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF22: andi.w #$f00, d0
pub fn i_06df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF26;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF26: addi.w #$800, d0
pub fn i_06df26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF2A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x800, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF2A: move.w d0, $2a(a6)
pub fn i_06df2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF2E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF2E: bra.w $6df5a
pub fn i_06df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF32;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DF5A;
    10
}
// ===== FireBurst_Tmpl_06df32  ($06DF32, 182 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DF32: jsr $5e9b6.l
pub fn i_06df32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF38);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF38: andi.w #$700, d0
pub fn i_06df38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF3C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF3C: addi.w #$300, d0
pub fn i_06df3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF40;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x300, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF40: jsr $6e20c(pc)
pub fn i_06df40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DF44);
    cpu.pc = 0x6E20C;
    16
}
/// $06DF44: move.w d0, $28(a6)
pub fn i_06df44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF48;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF48: jsr $5e9b6.l
pub fn i_06df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF4E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF4E: andi.w #$700, d0
pub fn i_06df4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF52;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF52: addi.w #$500, d0
pub fn i_06df52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF56;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x500, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF56: move.w d0, $2a(a6)
pub fn i_06df56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF5A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF5A: bset.b #$4, $6b(a6)
pub fn i_06df5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF60;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DF60: move.w #$108e, d0
pub fn i_06df60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF64;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DF64: jsr $2352.l
pub fn i_06df64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF6A);
    cpu.pc = 0x2352;
    16
}
/// $06DF6A: lea.l $2d4666.l, a0
pub fn i_06df6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4666;
    20
}
/// $06DF70: jsr $28cd4.l
pub fn i_06df70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF76);
    cpu.pc = 0x28CD4;
    16
}
/// $06DF76: move.l #$ffffffff, $48(a6)
pub fn i_06df76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF7E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DF7E: move.l #$2d3e42, $4c(a6)
pub fn i_06df7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF86;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3E42;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DF86: move.w #$2, d1
pub fn i_06df86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF8A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DF8A: jsr $236e.l
pub fn i_06df8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF90);
    cpu.pc = 0x236E;
    16
}
/// $06DF90: clr.w $72(a6)
pub fn i_06df90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF94;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06DF94: jsr $283ca.l
pub fn i_06df94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF9A);
    cpu.pc = 0x283CA;
    16
}
/// $06DF9A: move.w #$d000, d0
pub fn i_06df9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF9E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DF9E: jsr $28134.l
pub fn i_06df9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFA4);
    cpu.pc = 0x28134;
    16
}
/// $06DFA4: andi.w #$ffe3, $38(a6)
pub fn i_06dfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFAA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DFAA: ori.w #$14, $38(a6)
pub fn i_06dfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFB0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DFB0: lea.l $6dfb6(pc), a1
pub fn i_06dfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DFB6;
    16
}
/// $06DFB4: move.l a1, (a6)
pub fn i_06dfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DFB6: jsr $27cee.l
pub fn i_06dfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFBC);
    cpu.pc = 0x27CEE;
    16
}
/// $06DFBC: bcc.w $6dfc8
pub fn i_06dfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DFC8; return 10; }
    8
}
/// $06DFC0: lea.l $31d26.l, a1
pub fn i_06dfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFC6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $06DFC6: move.l a1, (a6)
pub fn i_06dfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DFC8: jsr $28d70.l
pub fn i_06dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFCE);
    cpu.pc = 0x28D70;
    16
}
/// $06DFCE: jsr $6e2fe(pc)
pub fn i_06dfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DFD2);
    cpu.pc = 0x6E2FE;
    16
}
/// $06DFD2: btst.b #$1, $13(a6)
pub fn i_06dfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFD8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06DFD8: beq.w $6dfe4
pub fn i_06dfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DFE4; return 10; }
    8
}
/// $06DFDC: lea.l $31d26.l, a1
pub fn i_06dfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFE2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $06DFE2: move.l a1, (a6)
pub fn i_06dfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DFE4: bra.w $6e15e
pub fn i_06dfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6E15E;
    10
}
// ===== FireBurst_Low_06dfe8  ($06DFE8, 114 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06DFE8: bset.b #$4, $6b(a6)
pub fn i_06dfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFEE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DFEE: move.w #$1086, d0
pub fn i_06dfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1086;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DFF2: jsr $2352.l
pub fn i_06dff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFF8);
    cpu.pc = 0x2352;
    16
}
/// $06DFF8: eori.b #$1, $3a(a6)
pub fn i_06dff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFFE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DFFE: move.l #$2d3d9e, $4c(a6)
pub fn i_06dffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E006;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3D9E;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E006: move.w #$3, d1
pub fn i_06e006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E00A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E00A: jsr $236e.l
pub fn i_06e00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E010);
    cpu.pc = 0x236E;
    16
}
/// $06E010: addi.w #$10, $24(a6)
pub fn i_06e010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E016;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E016: clr.w $2e(a6)
pub fn i_06e016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E01A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E01A: move.w #$ff00, $2a(a6)
pub fn i_06e01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E020;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E020: jsr $27bc8.l
pub fn i_06e020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E026);
    cpu.pc = 0x27BC8;
    16
}
/// $06E026: bcc.b $6e020
pub fn i_06e026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E028;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E020; return 10; }
    8
}
/// $06E028: jsr $267e2.l
pub fn i_06e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E02E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E02E);
    cpu.pc = 0x267E2;
    16
}
/// $06E02E: move.w #$180, d0
pub fn i_06e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E032;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E032: jsr $6e20c(pc)
pub fn i_06e032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E036;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E036);
    cpu.pc = 0x6E20C;
    16
}
/// $06E036: move.w d0, $28(a6)
pub fn i_06e036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E03A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E03A: clr.w $72(a6)
pub fn i_06e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E03E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E03E: jsr $283ca.l
pub fn i_06e03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E044);
    cpu.pc = 0x283CA;
    16
}
/// $06E044: move.w #$d000, d0
pub fn i_06e044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E048;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E048: jsr $28134.l
pub fn i_06e048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E04E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E04E);
    cpu.pc = 0x28134;
    16
}
/// $06E04E: andi.w #$ffe3, $38(a6)
pub fn i_06e04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E054;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E054: ori.w #$14, $38(a6)
pub fn i_06e054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E05A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_06e05a  ($06E05A, 8 B, kernel/task_handlers.c)
/// $06E05A: lea.l $6e062(pc), a1
pub fn i_06e05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E05E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E062;
    16
}
/// $06E05E: move.l a1, (a6)
pub fn i_06e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E060;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E060: rts
pub fn i_06e060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E062;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_Fly_06e062  ($06E062, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E062: jsr $27a92.l
pub fn i_06e062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E068;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E068);
    cpu.pc = 0x27A92;
    16
}
/// $06E068: jsr $28d70.l
pub fn i_06e068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E06E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E06E);
    cpu.pc = 0x28D70;
    16
}
/// $06E06E: jsr $6e2fe(pc)
pub fn i_06e06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E072;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E072);
    cpu.pc = 0x6E2FE;
    16
}
/// $06E072: cmpi.w #$32, $72(a6)
pub fn i_06e072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E078;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x32, Size::Word, false);
    16
}
/// $06E078: blt.w $6e084
pub fn i_06e078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E07C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6E084; return 10; }
    8
}
/// $06E07C: lea.l $31c20.l, a1
pub fn i_06e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E082;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31C20;
    20
}
/// $06E082: move.l a1, (a6)
pub fn i_06e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E084;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E084: btst.b #$1, $13(a6)
pub fn i_06e084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E08A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E08A: beq.w $6e096
pub fn i_06e08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E08E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E096; return 10; }
    8
}
/// $06E08E: lea.l $31c20.l, a1
pub fn i_06e08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E094;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31C20;
    20
}
/// $06E094: move.l a1, (a6)
pub fn i_06e094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E096;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E096: bra.w $6e15e
pub fn i_06e096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E09A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6E15E;
    10
}
// ===== FireBurst_Arc_06e09a  ($06E09A, 188 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E09A: bset.b #$4, $6b(a6)
pub fn i_06e09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0A0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E0A0: move.w #$1086, d0
pub fn i_06e0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1086;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0A4: jsr $2352.l
pub fn i_06e0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0AA);
    cpu.pc = 0x2352;
    16
}
/// $06E0AA: move.l #$2d3cfa, $4c(a6)
pub fn i_06e0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0B2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3CFA;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E0B2: move.w #$58, d1
pub fn i_06e0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0B6: jsr $236e.l
pub fn i_06e0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0BC);
    cpu.pc = 0x236E;
    16
}
/// $06E0BC: jsr $5e9b6.l
pub fn i_06e0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0C2);
    cpu.pc = 0x5E9B6;
    16
}
/// $06E0C2: andi.w #$3, d0
pub fn i_06e0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E0C6: move.w d0, d1
pub fn i_06e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0C8;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0C8: add.w d0, d0
pub fn i_06e0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0CA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E0CA: add.w d1, d0
pub fn i_06e0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0CC;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E0CC: add.w d0, d0
pub fn i_06e0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0CE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E0CE: ext.l d0
pub fn i_06e0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0D0;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06E0D0: lea.l $2d4212.l, a0
pub fn i_06e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4212;
    20
}
/// $06E0D6: adda.l d0, a0
pub fn i_06e0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0D8;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $06E0D8: move.w $2(a0), $2e(a6)
pub fn i_06e0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0DE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E0DE: move.w $4(a0), $2a(a6)
pub fn i_06e0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0E4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E0E4: move.w (a0), d0
pub fn i_06e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0E6;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $06E0E6: jsr $6e20c(pc)
pub fn i_06e0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E0EA);
    cpu.pc = 0x6E20C;
    16
}
/// $06E0EA: move.w d0, $28(a6)
pub fn i_06e0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E0EE: clr.w $72(a6)
pub fn i_06e0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0F2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E0F2: jsr $283ca.l
pub fn i_06e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0F8);
    cpu.pc = 0x283CA;
    16
}
/// $06E0F8: move.w #$d000, d0
pub fn i_06e0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0FC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0FC: jsr $28134.l
pub fn i_06e0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E102);
    cpu.pc = 0x28134;
    16
}
/// $06E102: andi.w #$ffe3, $38(a6)
pub fn i_06e102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E108;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E108: ori.w #$14, $38(a6)
pub fn i_06e108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E10E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E10E: lea.l $6e114(pc), a1
pub fn i_06e10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E112;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E114;
    16
}
/// $06E112: move.l a1, (a6)
pub fn i_06e112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E114;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E114: jsr $27d50.l
pub fn i_06e114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E11A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E11A);
    cpu.pc = 0x27D50;
    16
}
/// $06E11A: bcc.w $6e126
pub fn i_06e11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E11E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E126; return 10; }
    8
}
/// $06E11E: lea.l $31c20.l, a1
pub fn i_06e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E124;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31C20;
    20
}
/// $06E124: move.l a1, (a6)
pub fn i_06e124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E126;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E126: jsr $28d70.l
pub fn i_06e126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E12C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E12C);
    cpu.pc = 0x28D70;
    16
}
/// $06E12C: jsr $6e2fe(pc)
pub fn i_06e12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E130;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E130);
    cpu.pc = 0x6E2FE;
    16
}
/// $06E130: btst.b #$1, $13(a6)
pub fn i_06e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E136;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E136: beq.w $6e140
pub fn i_06e136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E13A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E140; return 10; }
    8
}
/// $06E13A: lea.l $6da9e(pc), a1
pub fn i_06e13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E13E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA9E;
    16
}
/// $06E13E: move.l a1, (a6)
pub fn i_06e13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E140;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E140: movea.l #$ffffffff, a0
pub fn i_06e140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E146;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E146: lea.l $2d4046.l, a0
pub fn i_06e146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E14C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4046;
    20
}
/// $06E14C: jsr $5dd56.l
pub fn i_06e14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E152);
    cpu.pc = 0x5DD56;
    16
}
/// $06E152: bcc.w $6e15c
pub fn i_06e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E156;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E15C; return 10; }
    8
}
// ===== SetTaskHandler_06e156  ($06E156, 8 B, kernel/task_handlers.c)
/// $06E156: lea.l $6da90(pc), a1
pub fn i_06e156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E15A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06E15A: move.l a1, (a6)
pub fn i_06e15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E15C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E15C: rts
pub fn i_06e15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E15E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_FreeIfOffWorld_06e15e  ($06E15E, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E15E: movea.l #$ffffffff, a0
pub fn i_06e15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E164;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E164: jsr $5dd56.l
pub fn i_06e164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E16A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E16A);
    cpu.pc = 0x5DD56;
    16
}
/// $06E16A: bcc.w $6e174
pub fn i_06e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E16E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E174; return 10; }
    8
}
// ===== SetTaskHandler_06e16e  ($06E16E, 8 B, kernel/task_handlers.c)
/// $06E16E: lea.l $6da90(pc), a1
pub fn i_06e16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E172;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06E172: move.l a1, (a6)
pub fn i_06e172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E174;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E174: rts
pub fn i_06e174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E176;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_PickBurstVel_06e176  ($06E176, 150 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E176: lea.l $2b8f74.l, a0
pub fn i_06e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E17C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8F74;
    20
}
/// $06E17C: jsr $799de.l
pub fn i_06e17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E182);
    cpu.pc = 0x799DE;
    16
}
/// $06E182: andi.w #$3, d0
pub fn i_06e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E186;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E186: cmpi.w #$2, $70(a6)
pub fn i_06e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E18C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $06E18C: bne.w $6e194
pub fn i_06e18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E190;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E194; return 10; }
    8
}
/// $06E190: move.w #$4, d0
pub fn i_06e190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E194;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E194: lsl.w #$2, d0
pub fn i_06e194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E196;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06E196: lea.l $2d4186.l, a0
pub fn i_06e196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E19C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4186;
    20
}
/// $06E19C: move.l (a0, d0.w), $5c(a6)
pub fn i_06e19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1A2;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06E1A2: lea.l $2d4036.l, a0
pub fn i_06e1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4036;
    20
}
/// $06E1A8: jsr $5e086.l
pub fn i_06e1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E1AE);
    cpu.pc = 0x5E086;
    16
}
/// $06E1AE: bcc.w $6e1b8
pub fn i_06e1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E1B8; return 10; }
    8
}
/// $06E1B2: jsr $5e0d4.l
pub fn i_06e1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E1B8);
    cpu.pc = 0x5E0D4;
    16
}
/// $06E1B8: move.w $22(a6), d0
pub fn i_06e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1BC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06E1BC: sub.w $22(a0), d0
pub fn i_06e1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1C0;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06E1C0: cmpi.w #$60, d0
pub fn i_06e1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1C4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $06E1C4: bgt.w $6e1cc
pub fn i_06e1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1C8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E1CC; return 10; }
    8
}
/// $06E1C8: move.w #$60, d0
pub fn i_06e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1CC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E1CC: subi.w #$60, d0
pub fn i_06e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1D0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x60, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E1D0: lsr.w #$4, d0
pub fn i_06e1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1D2;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06E1D2: cmpi.w #$3, d0
pub fn i_06e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1D6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $06E1D6: bcs.w $6e1de
pub fn i_06e1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6E1DE; return 10; }
    8
}
/// $06E1DA: move.w #$3, d0
pub fn i_06e1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E1DE: move.w d0, d1
pub fn i_06e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E0;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E1E0: add.w d0, d0
pub fn i_06e1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E1E2: add.w d1, d0
pub fn i_06e1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E4;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E1E4: add.w d0, d0
pub fn i_06e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E1E6: movea.l $5c(a6), a0
pub fn i_06e1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E1EA: adda.l d0, a0
pub fn i_06e1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1EC;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $06E1EC: move.w (a0), $28(a6)
pub fn i_06e1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1F0;
    cpu.ir = 0x3D50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad3, v2 as u16);
    16
}
/// $06E1F0: move.w $2(a0), $2e(a6)
pub fn i_06e1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1F6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E1F6: move.w $4(a0), $2a(a6)
pub fn i_06e1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1FC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E1FC: btst.b #$0, $3a(a6)
pub fn i_06e1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E202;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E202: beq.w $6e20a
pub fn i_06e202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E206;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E20A; return 10; }
    8
}
/// $06E206: neg.w $28(a6)
pub fn i_06e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E20A;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E20A: rts
pub fn i_06e20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E20C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Facing_NegIfLeft_06e20c  ($06E20C, 12 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E20C: btst.b #$0, $3a(a6)
pub fn i_06e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E212;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E212: beq.w $6e21e
pub fn i_06e212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E216;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E21E; return 10; }
    8
}
/// $06E216: neg.w d0
pub fn i_06e216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E218;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_06e218  ($06E218, 6 B, thunks/ccr_helpers.c)
/// $06E218: ori.b #$11, ccr
pub fn i_06e218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E21C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06E21C: rts
pub fn i_06e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E21E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06e21e  ($06E21E, 6 B, thunks/ccr_helpers.c)
/// $06E21E: andi.b #$ee, ccr
pub fn i_06e21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E222;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06E222: rts
pub fn i_06e222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E224;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndTag_06E224  ($06E224, 38 B, entity/spawn_and_tag_06e224.s)
/// $06E224: movem.w d0, -(a7)
pub fn i_06e224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E228;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E228: lea.l $6df32(pc), a1
pub fn i_06e228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E22C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DF32;
    16
}
/// $06E22C: jsr $4ae.l
pub fn i_06e22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E232);
    cpu.pc = 0x4AE;
    16
}
/// $06E232: jsr $5dd02.l
pub fn i_06e232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E238);
    cpu.pc = 0x5DD02;
    16
}
/// $06E238: movem.w (a7)+, d0
pub fn i_06e238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E23C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E23C: andi.b #$1, d0
pub fn i_06e23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E240;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E240: move.b d0, $3a(a0)
pub fn i_06e240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E244;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E244: jsr $6e2bc(pc)
pub fn i_06e244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E248;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E248);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E248: rts
pub fn i_06e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E24A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FireBurstWide_06e24a  ($06E24A, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E24A: movem.w d0, -(a7)
pub fn i_06e24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E24E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E24E: lea.l $6df06(pc), a1
pub fn i_06e24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E252;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DF06;
    16
}
/// $06E252: jsr $4ae.l
pub fn i_06e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E258;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E258);
    cpu.pc = 0x4AE;
    16
}
/// $06E258: jsr $5dd02.l
pub fn i_06e258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E25E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E25E);
    cpu.pc = 0x5DD02;
    16
}
/// $06E25E: movem.w (a7)+, d0
pub fn i_06e25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E262;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E262: andi.b #$1, d0
pub fn i_06e262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E266;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E266: move.b d0, $3a(a0)
pub fn i_06e266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E26A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_06e26a  ($06E26A, 6 B, thunks/jsr_pc_thunks.c)
/// $06E26A: jsr $6e2bc(pc)
pub fn i_06e26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E26E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E26E);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E26E: rts
pub fn i_06e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FireBurstArc_06e270  ($06E270, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E270: movem.w d0, -(a7)
pub fn i_06e270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E274;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E274: lea.l $6e09a(pc), a1
pub fn i_06e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E09A;
    16
}
/// $06E278: jsr $4ae.l
pub fn i_06e278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E27E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E27E);
    cpu.pc = 0x4AE;
    16
}
/// $06E27E: jsr $5dd02.l
pub fn i_06e27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E284;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E284);
    cpu.pc = 0x5DD02;
    16
}
/// $06E284: movem.w (a7)+, d0
pub fn i_06e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E288;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E288: andi.b #$1, d0
pub fn i_06e288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E28C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E28C: move.b d0, $3a(a0)
pub fn i_06e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E290;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_06e290  ($06E290, 6 B, thunks/jsr_pc_thunks.c)
/// $06E290: jsr $6e2bc(pc)
pub fn i_06e290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E294;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E294);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E294: rts
pub fn i_06e294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E296;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FireBurstLow_06e296  ($06E296, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E296: movem.w d0, -(a7)
pub fn i_06e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E29A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E29A: lea.l $6dfe8(pc), a1
pub fn i_06e29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E29E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DFE8;
    16
}
/// $06E29E: jsr $4ae.l
pub fn i_06e29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E2A4);
    cpu.pc = 0x4AE;
    16
}
/// $06E2A4: jsr $5dd02.l
pub fn i_06e2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E2AA);
    cpu.pc = 0x5DD02;
    16
}
/// $06E2AA: movem.w (a7)+, d0
pub fn i_06e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2AE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E2AE: andi.b #$1, d0
pub fn i_06e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2B2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E2B2: move.b d0, $3a(a0)
pub fn i_06e2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2B6;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_06e2b6  ($06E2B6, 6 B, thunks/jsr_pc_thunks.c)
/// $06E2B6: jsr $6e2bc(pc)
pub fn i_06e2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E2BA);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E2BA: rts
pub fn i_06e2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyAnimFromLeader_06E2BC  ($06E2BC, 66 B, entity/copy_anim_from_leader_06e2bc.s)
/// $06E2BC: movea.l $50(a6), a1
pub fn i_06e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2C0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06E2C0: move.w $22(a1), $22(a0)
pub fn i_06e2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2C6;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2C6: move.w $24(a1), $24(a0)
pub fn i_06e2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2CC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2CC: move.w $38(a1), $38(a0)
pub fn i_06e2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2D2;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2D2: move.w $30(a1), $30(a0)
pub fn i_06e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2D8;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2D8: move.b $3b(a1), $3b(a0)
pub fn i_06e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2DE;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E2DE: move.b $46(a1), $46(a0)
pub fn i_06e2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2E4;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x46);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E2E4: move.b $47(a1), $47(a0)
pub fn i_06e2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2EA;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x47);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x47);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E2EA: move.l $3c(a1), $3c(a0)
pub fn i_06e2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2F0;
    cpu.ir = 0x2169;
    let ad2 = cpu.a[1].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $06E2F0: move.l $40(a1), $40(a0)
pub fn i_06e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2F6;
    cpu.ir = 0x2169;
    let ad2 = cpu.a[1].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x40);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $06E2F6: move.w $30(a1), $30(a0)
pub fn i_06e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2FC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2FC: rts
pub fn i_06e2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_TickHit_06e2fe  ($06E2FE, 20 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E2FE: addq.w #$1, $72(a6)
pub fn i_06e2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E302;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E302: cmpi.w #$4, $72(a6)
pub fn i_06e302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E308;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $06E308: ble.w $6e318
pub fn i_06e308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E30C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6E318; return 10; }
    8
}
/// $06E30C: jsr $283d8.l
pub fn i_06e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E312;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E312);
    cpu.pc = 0x283D8;
    16
}
// ===== ClearXN_06e312  ($06E312, 6 B, thunks/ccr_helpers.c)
/// $06E312: andi.b #$ee, ccr
pub fn i_06e312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E316;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06E316: rts
pub fn i_06e316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E318;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06e318  ($06E318, 6 B, thunks/ccr_helpers.c)
/// $06E318: ori.b #$11, ccr
pub fn i_06e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E31C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06E31C: rts
pub fn i_06e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E31E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_PlayCry_06e31e  ($06E31E, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E31E: lea.l $2d4284.l, a0
pub fn i_06e31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E324;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4284;
    20
}
/// $06E324: move.b $9b(a6), d0
pub fn i_06e324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E328;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E328: andi.w #$3, d0
pub fn i_06e328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E32C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E32C: add.w d0, d0
pub fn i_06e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E32E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E32E: move.w (a0, d0.w), d1
pub fn i_06e32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E332;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06E332: jsr $236e.l
pub fn i_06e332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E338);
    cpu.pc = 0x236E;
    16
}
/// $06E338: move.w #$1, d1
pub fn i_06e338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E33C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E33C: jsr $236e.l
pub fn i_06e33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E342);
    cpu.pc = 0x236E;
    16
}
/// $06E342: move.w $16(a6), $14(a6)
pub fn i_06e342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E348;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E348: rts
pub fn i_06e348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E34A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyParentAnimTimer_06e34a  ($06E34A, 12 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E34A: movea.l $c(a6), a0
pub fn i_06e34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E34E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E34E: move.w $14(a0), $14(a6)
pub fn i_06e34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E354;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E354: rts
pub fn i_06e354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E356;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_PlaySnd157To159_06e356  ($06E356, 38 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E356: move.w #$157, d1
pub fn i_06e356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E35A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x157;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E35A: jsr $236e.l
pub fn i_06e35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E360);
    cpu.pc = 0x236E;
    16
}
/// $06E360: move.w #$158, d1
pub fn i_06e360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E364;
    cpu.ir = 0x323C;
    let v1: u32 = 0x158;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E364: jsr $236e.l
pub fn i_06e364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E36A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E36A);
    cpu.pc = 0x236E;
    16
}
/// $06E36A: move.w #$159, d1
pub fn i_06e36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E36E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x159;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E36E: jsr $236e.l
pub fn i_06e36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E374);
    cpu.pc = 0x236E;
    16
}
/// $06E374: move.w $16(a6), $14(a6)
pub fn i_06e374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E37A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E37A: rts
pub fn i_06e37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E37C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_PlaySnd47Or144_06e37c  ($06E37C, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E37C: move.w #$47, d1
pub fn i_06e37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E380;
    cpu.ir = 0x323C;
    let v1: u32 = 0x47;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E380: tst.b $9e(a6)
pub fn i_06e380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E384;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E384: beq.w $6e38c
pub fn i_06e384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E388;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E38C; return 10; }
    8
}
/// $06E388: move.w #$144, d1
pub fn i_06e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E38C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x144;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_06e38c  ($06E38C, 8 B, thunks/jsr_abs_thunks.c)
/// $06E38C: jsr $236e.l
pub fn i_06e38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E392);
    cpu.pc = 0x236E;
    16
}
/// $06E392: rts
pub fn i_06e392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E394;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_ClearFlag10E39A_06e394  ($06E394, 8 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E394: tst.b $9c(a6)
pub fn i_06e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E398;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E398: beq.w $6e3a2
pub fn i_06e398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E39C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E3A2; return 10; }
    8
}
// ===== ClrRamWord_06e39c  ($06E39C, 8 B, thunks/clr_ram_word.c)
/// $06E39C: clr.b $10e39a.l
pub fn i_06e39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3A2;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $06E3A2: rts
pub fn i_06e3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_SpawnSparkAndDebris_06e3a4  ($06E3A4, 110 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E3A4: move.w #$1065, d0
pub fn i_06e3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1065;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E3A8: jsr $2352.l
pub fn i_06e3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3AE);
    cpu.pc = 0x2352;
    16
}
/// $06E3AE: lea.l $6dc90(pc), a1
pub fn i_06e3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DC90;
    16
}
/// $06E3B2: jsr $4ae.l
pub fn i_06e3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3B8);
    cpu.pc = 0x4AE;
    16
}
/// $06E3B8: jsr $5dd02.l
pub fn i_06e3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3BE);
    cpu.pc = 0x5DD02;
    16
}
/// $06E3BE: move.w #$d000, $38(a0)
pub fn i_06e3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3C4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E3C4: move.w #$ffec, d0
pub fn i_06e3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E3C8: jsr $6e20c(pc)
pub fn i_06e3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E3CC);
    cpu.pc = 0x6E20C;
    16
}
/// $06E3CC: add.w d0, $22(a0)
pub fn i_06e3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3D0;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E3D0: addi.w #$c, $24(a0)
pub fn i_06e3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3D6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E3D6: lea.l $6dbde(pc), a1
pub fn i_06e3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DBDE;
    16
}
/// $06E3DA: jsr $4ae.l
pub fn i_06e3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3E0);
    cpu.pc = 0x4AE;
    16
}
/// $06E3E0: jsr $5dd02.l
pub fn i_06e3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3E6);
    cpu.pc = 0x5DD02;
    16
}
/// $06E3E6: move.b $9f(a6), $89(a0)
pub fn i_06e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3EC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x89);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E3EC: move.w $70(a6), $70(a0)
pub fn i_06e3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3F2;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E3F2: move.b $9e(a6), $9e(a0)
pub fn i_06e3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3F8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E3F8: move.w #$d000, $38(a0)
pub fn i_06e3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3FE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E3FE: move.w #$ffd0, d0
pub fn i_06e3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E402;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E402: jsr $6e20c(pc)
pub fn i_06e402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E406;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E406);
    cpu.pc = 0x6E20C;
    16
}
/// $06E406: add.w d0, $22(a0)
pub fn i_06e406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E40A;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E40A: addi.w #$2c, $24(a0)
pub fn i_06e40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E410;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E410: rts
pub fn i_06e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnLoop16_06E412  ($06E412, 36 B, entity/spawn_loop16_06e412.s)
/// $06E412: move.w #$10, d0
pub fn i_06e412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E416;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E416: move.w d0, -(a7)
pub fn i_06e416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E418;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $06E418: lea.l $6dd5c(pc), a1
pub fn i_06e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E41C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DD5C;
    16
}
/// $06E41C: jsr $4ae.l
pub fn i_06e41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E422);
    cpu.pc = 0x4AE;
    16
}
/// $06E422: jsr $5dd02.l
pub fn i_06e422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E428);
    cpu.pc = 0x5DD02;
    16
}
/// $06E428: move.b $9b(a6), $9b(a0)
pub fn i_06e428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E42E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E42E: move.w (a7)+, d0
pub fn i_06e42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E430;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $06E430: subq.w #$1, d0
pub fn i_06e430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E432;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E432: bne.b $6e416
pub fn i_06e432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E434;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E416; return 10; }
    8
}
/// $06E434: rts
pub fn i_06e434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E436;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_ExplosionAbove_06e436  ($06E436, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E436: lea.l $77fd6.l, a1
pub fn i_06e436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E43C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06E43C: jsr $4ae.l
pub fn i_06e43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E442);
    cpu.pc = 0x4AE;
    16
}
/// $06E442: jsr $5dd02.l
pub fn i_06e442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E448);
    cpu.pc = 0x5DD02;
    16
}
/// $06E448: move.w #$8000, $38(a0)
pub fn i_06e448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E44E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E44E: addi.w #$10, $24(a0)
pub fn i_06e44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E454;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E454: rts
pub fn i_06e454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E456;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_Fx7801E_06e456  ($06E456, 26 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E456: lea.l $7801e.l, a1
pub fn i_06e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E45C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7801E;
    20
}
/// $06E45C: jsr $4ae.l
pub fn i_06e45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E462);
    cpu.pc = 0x4AE;
    16
}
/// $06E462: jsr $5dd02.l
pub fn i_06e462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E468);
    cpu.pc = 0x5DD02;
    16
}
/// $06E468: move.w #$2000, $38(a0)
pub fn i_06e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E46E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E46E: rts
pub fn i_06e46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E470;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_Fx2D40A2_06e470  ($06E470, 20 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E470: lea.l $2d40a2.l, a1
pub fn i_06e470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E476;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D40A2;
    20
}
/// $06E476: jsr $77c7e.l
pub fn i_06e476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E47C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E47C);
    cpu.pc = 0x77C7E;
    16
}
/// $06E47C: move.w #$8000, $38(a0)
pub fn i_06e47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E482;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E482: rts
pub fn i_06e482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E484;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_SetSpriteByFlags7E7F_06e484  ($06E484, 28 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E484: tst.b $7e(a6)
pub fn i_06e484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E488;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E488: beq.w $6e4a6
pub fn i_06e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E48C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E4A6; return 10; }
    8
}
/// $06E48C: lea.l $2d422a.l, a1
pub fn i_06e48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E492;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D422A;
    20
}
/// $06E492: tst.b $7f(a6)
pub fn i_06e492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E496;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E496: beq.w $6e4a0
pub fn i_06e496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E49A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E4A0; return 10; }
    8
}
/// $06E49A: lea.l $2d424a.l, a1
pub fn i_06e49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D424A;
    20
}
// ===== JsrAbsThunk_06e4a0  ($06E4A0, 8 B, thunks/jsr_abs_thunks.c)
/// $06E4A0: jsr $43fac.l
pub fn i_06e4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4A6);
    cpu.pc = 0x43FAC;
    16
}
/// $06E4A6: rts
pub fn i_06e4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_Spawn12Scatter_06e4a8  ($06E4A8, 36 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E4A8: move.w #$c, d0
pub fn i_06e4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4AC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E4AC: move.w d0, -(a7)
pub fn i_06e4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4AE;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $06E4AE: lea.l $6dd86(pc), a1
pub fn i_06e4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DD86;
    16
}
/// $06E4B2: jsr $4ae.l
pub fn i_06e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4B8);
    cpu.pc = 0x4AE;
    16
}
/// $06E4B8: jsr $5dd02.l
pub fn i_06e4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4BE);
    cpu.pc = 0x5DD02;
    16
}
/// $06E4BE: move.b $9b(a6), $9b(a0)
pub fn i_06e4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4C4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E4C4: move.w (a7)+, d0
pub fn i_06e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4C6;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $06E4C6: subq.w #$1, d0
pub fn i_06e4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4C8;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E4C8: bne.b $6e4ac
pub fn i_06e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4CA;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E4AC; return 10; }
    8
}
/// $06E4CA: rts
pub fn i_06e4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_SpawnSmokeCycle_06e4cc  ($06E4CC, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E4CC: lea.l $6dc5e(pc), a1
pub fn i_06e4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DC5E;
    16
}
/// $06E4D0: jsr $6fe.l
pub fn i_06e4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4D6);
    cpu.pc = 0x6FE;
    16
}
/// $06E4D6: jsr $5dd02.l
pub fn i_06e4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4DC);
    cpu.pc = 0x5DD02;
    16
}
/// $06E4DC: move.b $9e(a6), $9e(a0)
pub fn i_06e4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4E2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E4E2: subq.w #$1, $38(a0)
pub fn i_06e4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4E6;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E4E6: move.b $88(a6), d0
pub fn i_06e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E4EA: addq.b #$1, d0
pub fn i_06e4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4EC;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06E4EC: andi.w #$3, d0
pub fn i_06e4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4F0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E4F0: cmpi.w #$3, d0
pub fn i_06e4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4F4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $06E4F4: blt.w $6e4fa
pub fn i_06e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4F8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6E4FA; return 10; }
    8
}
/// $06E4F8: clr.w d0
pub fn i_06e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4FA;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E4FA: move.w d0, d1
pub fn i_06e4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4FC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E4FC: move.b d0, $88(a6)
pub fn i_06e4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E500;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E500: add.w d1, d1
pub fn i_06e500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E502;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E502: addi.w #$16, d1
pub fn i_06e502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E506;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x16, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E506: move.w (a6, d1.w), $14(a6)
pub fn i_06e506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E50C;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06E50C: rts
pub fn i_06e50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E50E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_CmpSlotWithParent_06e50e  ($06E50E, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E50E: movea.l $8(a6), a1
pub fn i_06e50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E512;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06E512: move.b $10(a6), d0
pub fn i_06e512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E516;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E516: cmp.b $10(a1), d0
pub fn i_06e516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E51A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06E51A: bcs.w $6e524
pub fn i_06e51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E51E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6E524; return 10; }
    8
}
// ===== ClearXN_06e51e  ($06E51E, 6 B, thunks/ccr_helpers.c)
/// $06E51E: andi.b #$ee, ccr
pub fn i_06e51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E522;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06E522: rts
pub fn i_06e522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06e524  ($06E524, 6 B, thunks/ccr_helpers.c)
/// $06E524: ori.b #$11, ccr
pub fn i_06e524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E528;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06E528: rts
pub fn i_06e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E52A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Tmpl4B_06e52a  ($06E52A, 314 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E52A: move.w #$3b, d1
pub fn i_06e52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E52E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E52E: jsr $236e.l
pub fn i_06e52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E534;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E534);
    cpu.pc = 0x236E;
    16
}
/// $06E534: move.w #$9, $1c(a6)
pub fn i_06e534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E53A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E53A: jsr $138fe.l
pub fn i_06e53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E540);
    cpu.pc = 0x138FE;
    16
}
/// $06E540: lea.l $2d4a4e.l, a0
pub fn i_06e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E546;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A4E;
    20
}
/// $06E546: move.b $9a(a6), d0
pub fn i_06e546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E54A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E54A: andi.w #$3, d0
pub fn i_06e54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E54E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E54E: lsl.w #$2, d0
pub fn i_06e54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E550;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06E550: movea.l (a0, d0.w), a0
pub fn i_06e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E554;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06E554: jsr $799de.l
pub fn i_06e554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E55A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E55A);
    cpu.pc = 0x799DE;
    16
}
/// $06E55A: move.w d0, $72(a6)
pub fn i_06e55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E55E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E55E: lea.l $2b91fe.l, a0
pub fn i_06e55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E564;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B91FE;
    20
}
/// $06E564: jsr $799de.l
pub fn i_06e564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E56A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E56A);
    cpu.pc = 0x799DE;
    16
}
/// $06E56A: move.w d0, $66(a6)
pub fn i_06e56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E56E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E56E: move.w #$c000, d0
pub fn i_06e56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E572;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E572: jsr $28134.l
pub fn i_06e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E578);
    cpu.pc = 0x28134;
    16
}
/// $06E578: andi.w #$ffe3, $38(a6)
pub fn i_06e578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E57E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E57E: ori.w #$14, $38(a6)
pub fn i_06e57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E584;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E584: move.l #$2d46c6, $48(a6)
pub fn i_06e584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E58C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D46C6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E58C: clr.b $20(a6)
pub fn i_06e58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E590;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06E590: jsr $6eeae(pc)
pub fn i_06e590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E594;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E594);
    cpu.pc = 0x6EEAE;
    16
}
/// $06E594: lea.l $723da.l, a1
pub fn i_06e594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E59A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723DA;
    20
}
/// $06E59A: jsr $4ae.l
pub fn i_06e59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5A0);
    cpu.pc = 0x4AE;
    16
}
/// $06E5A0: jsr $5dd02.l
pub fn i_06e5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5A6);
    cpu.pc = 0x5DD02;
    16
}
/// $06E5A6: move.w #$ffe0, $98(a0)
pub fn i_06e5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5AC;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E5AC: lea.l $6e966(pc), a1
pub fn i_06e5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E966;
    16
}
/// $06E5B0: jsr $4ae.l
pub fn i_06e5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5B6);
    cpu.pc = 0x4AE;
    16
}
/// $06E5B6: jsr $5dd02.l
pub fn i_06e5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5BC);
    cpu.pc = 0x5DD02;
    16
}
/// $06E5BC: lea.l $6e9c0(pc), a1
pub fn i_06e5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E9C0;
    16
}
/// $06E5C0: jsr $4ae.l
pub fn i_06e5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5C6);
    cpu.pc = 0x4AE;
    16
}
/// $06E5C6: jsr $5dd02.l
pub fn i_06e5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5CC);
    cpu.pc = 0x5DD02;
    16
}
/// $06E5CC: move.b $99(a6), $99(a0)
pub fn i_06e5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5D2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5D2: move.b $9b(a6), $9b(a0)
pub fn i_06e5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5D8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5D8: move.b $9d(a6), $9d(a0)
pub fn i_06e5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5DE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5DE: move.b $9e(a6), $9e(a0)
pub fn i_06e5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5E4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5E4: lea.l $6e5ea(pc), a1
pub fn i_06e5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E5EA;
    16
}
/// $06E5E8: move.l a1, (a6)
pub fn i_06e5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E5EA: tst.b $9d(a6)
pub fn i_06e5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5EE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E5EE: beq.w $6e5f8
pub fn i_06e5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E5F8; return 10; }
    8
}
/// $06E5F2: bset.b #$6, $13(a6)
pub fn i_06e5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5F8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E5F8: jsr $6f094(pc)
pub fn i_06e5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E5FC);
    cpu.pc = 0x6F094;
    16
}
/// $06E5FC: jsr $78f8a.l
pub fn i_06e5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E602);
    cpu.pc = 0x78F8A;
    16
}
/// $06E602: bcc.w $6e60c
pub fn i_06e602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E606;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E60C; return 10; }
    8
}
/// $06E606: lea.l $6e770(pc), a1
pub fn i_06e606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E60A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E770;
    16
}
/// $06E60A: move.l a1, (a6)
pub fn i_06e60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E60C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E60C: jsr $6ef0e(pc)
pub fn i_06e60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E610;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E610);
    cpu.pc = 0x6EF0E;
    16
}
/// $06E610: jsr $28d70.l
pub fn i_06e610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E616);
    cpu.pc = 0x28D70;
    16
}
/// $06E616: jsr $2870a.l
pub fn i_06e616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E61C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E61C);
    cpu.pc = 0x2870A;
    16
}
/// $06E61C: bcc.w $6e63e
pub fn i_06e61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E620;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E63E; return 10; }
    8
}
/// $06E620: bclr.b #$3, $13(a6)
pub fn i_06e620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E626;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E626: lea.l $2d4d20.l, a0
pub fn i_06e626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E62C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4D20;
    20
}
/// $06E62C: jsr $28cd4.l
pub fn i_06e62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E632;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E632);
    cpu.pc = 0x28CD4;
    16
}
/// $06E632: lea.l $5e766.l, a0
pub fn i_06e632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E638;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06E638: jsr $5e770.l
pub fn i_06e638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E63E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E63E);
    cpu.pc = 0x5E770;
    16
}
/// $06E63E: jsr $28758.l
pub fn i_06e63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E644);
    cpu.pc = 0x28758;
    16
}
/// $06E644: bcc.w $6e64e
pub fn i_06e644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E648;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E64E; return 10; }
    8
}
/// $06E648: lea.l $6e66c(pc), a1
pub fn i_06e648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E64C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E66C;
    16
}
/// $06E64C: move.l a1, (a6)
pub fn i_06e64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E64E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E64E: movea.l #$ffffffff, a0
pub fn i_06e64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E654;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E654: lea.l $2d49b6.l, a0
pub fn i_06e654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E65A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06E65A: jsr $5dd5c.l
pub fn i_06e65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E660;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E660);
    cpu.pc = 0x5DD5C;
    16
}
/// $06E660: bcc.w $6e66a
pub fn i_06e660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E664;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E66A; return 10; }
    8
}
// ===== SetTaskHandler_06e664  ($06E664, 8 B, kernel/task_handlers.c)
/// $06E664: lea.l $6e80a(pc), a1
pub fn i_06e664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E668;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E80A;
    16
}
/// $06E668: move.l a1, (a6)
pub fn i_06e668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E66A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E66A: rts
pub fn i_06e66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E66C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Attack_06e66c  ($06E66C, 222 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E66C: move.b $9c(a6), d0
pub fn i_06e66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E670;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E670: cmpi.b #$2, d0
pub fn i_06e670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E674;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $06E674: bne.w $6e682
pub fn i_06e674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E678;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E682; return 10; }
    8
}
/// $06E678: jsr $5e9b6.l
pub fn i_06e678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E67E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E67E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06E67E: andi.b #$1, d0
pub fn i_06e67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E682;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E682: move.b d0, $9c(a6)
pub fn i_06e682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E686;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E686: tst.b d0
pub fn i_06e686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E688;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $06E688: bne.w $6e770
pub fn i_06e688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E68C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E770; return 10; }
    8
}
/// $06E68C: move.w #$10b8, d0
pub fn i_06e68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E690;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E690: jsr $2352.l
pub fn i_06e690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E696);
    cpu.pc = 0x2352;
    16
}
/// $06E696: move.w #$c000, d0
pub fn i_06e696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E69A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E69A: jsr $28134.l
pub fn i_06e69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6A0);
    cpu.pc = 0x28134;
    16
}
/// $06E6A0: andi.w #$ffe3, $38(a6)
pub fn i_06e6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6A6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E6A6: ori.w #$1c, $38(a6)
pub fn i_06e6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6AC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E6AC: move.l #$2d4816, $4c(a6)
pub fn i_06e6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6B4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D4816;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E6B4: jsr $283ca.l
pub fn i_06e6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6BA);
    cpu.pc = 0x283CA;
    16
}
/// $06E6BA: clr.w $2a(a6)
pub fn i_06e6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6BE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E6BE: move.w #$ffe0, $2e(a6)
pub fn i_06e6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E6C4: move.w $36(a6), d0
pub fn i_06e6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6C8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06E6C8: cmpi.w #$600, d0
pub fn i_06e6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6CC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x600, Size::Word, false);
    8
}
/// $06E6CC: ble.w $6e6d4
pub fn i_06e6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6D0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6E6D4; return 10; }
    8
}
/// $06E6D0: move.w #$600, d0
pub fn i_06e6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E6D4: btst.b #$0, $3a(a6)
pub fn i_06e6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6DA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E6DA: bne.w $6e6e0
pub fn i_06e6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E6E0; return 10; }
    8
}
/// $06E6DE: neg.w d0
pub fn i_06e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6E0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E6E0: move.w d0, $28(a6)
pub fn i_06e6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6E4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E6E4: lea.l $2d4c9c.l, a0
pub fn i_06e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C9C;
    20
}
/// $06E6EA: jsr $28cd4.l
pub fn i_06e6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6F0);
    cpu.pc = 0x28CD4;
    16
}
/// $06E6F0: lea.l $6e6f6(pc), a1
pub fn i_06e6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E6F6;
    16
}
/// $06E6F4: move.l a1, (a6)
pub fn i_06e6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E6F6: jsr $27d50.l
pub fn i_06e6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6FC);
    cpu.pc = 0x27D50;
    16
}
/// $06E6FC: bcc.w $6e706
pub fn i_06e6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E700;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E706; return 10; }
    8
}
/// $06E700: lea.l $6e770(pc), a1
pub fn i_06e700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E704;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E770;
    16
}
/// $06E704: move.l a1, (a6)
pub fn i_06e704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E706;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E706: jsr $6ef0e(pc)
pub fn i_06e706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E70A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E70A);
    cpu.pc = 0x6EF0E;
    16
}
/// $06E70A: jsr $28d70.l
pub fn i_06e70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E710);
    cpu.pc = 0x28D70;
    16
}
/// $06E710: jsr $283ca.l
pub fn i_06e710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E716);
    cpu.pc = 0x283CA;
    16
}
/// $06E716: jsr $283d8.l
pub fn i_06e716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E71C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E71C);
    cpu.pc = 0x283D8;
    16
}
/// $06E71C: tst.b $9b(a6)
pub fn i_06e71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E720;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E720: beq.w $6e734
pub fn i_06e720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E724;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E734; return 10; }
    8
}
/// $06E724: cmpi.w #$120, $24(a6)
pub fn i_06e724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E72A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $06E72A: bgt.w $6e734
pub fn i_06e72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E72E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E734; return 10; }
    8
}
/// $06E72E: lea.l $6e752(pc), a1
pub fn i_06e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E732;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E752;
    16
}
/// $06E732: move.l a1, (a6)
pub fn i_06e732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E734;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E734: movea.l #$ffffffff, a0
pub fn i_06e734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E73A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E73A: lea.l $2d49b6.l, a0
pub fn i_06e73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E740;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06E740: jsr $5dd5c.l
pub fn i_06e740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E746);
    cpu.pc = 0x5DD5C;
    16
}
/// $06E746: bcc.w $6e750
pub fn i_06e746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E74A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E750; return 10; }
    8
}
// ===== SetTaskHandler_06e74a  ($06E74A, 8 B, kernel/task_handlers.c)
/// $06E74A: lea.l $6e80a(pc), a1
pub fn i_06e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E74E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E80A;
    16
}
/// $06E74E: move.l a1, (a6)
pub fn i_06e74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E750;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E750: rts
pub fn i_06e750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E752;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Die_06e752  ($06E752, 184 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E752: move.w #$120, $24(a6)
pub fn i_06e752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E758;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E758: move.w #$ffff, $38(a6)
pub fn i_06e758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E75E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E75E: lea.l $78890.l, a1
pub fn i_06e75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E764;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78890;
    20
}
/// $06E764: jsr $4ae.l
pub fn i_06e764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E76A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E76A);
    cpu.pc = 0x4AE;
    16
}
/// $06E76A: jsr $5dd02.l
pub fn i_06e76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E770);
    cpu.pc = 0x5DD02;
    16
}
/// $06E770: tst.b $9c(a6)
pub fn i_06e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E774;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E774: bne.w $6e786
pub fn i_06e774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E778;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E786; return 10; }
    8
}
/// $06E778: tst.b $9b(a6)
pub fn i_06e778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E77C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E77C: bne.w $6e786
pub fn i_06e77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E780;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E786; return 10; }
    8
}
/// $06E780: jsr $434dc.l
pub fn i_06e780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E786);
    cpu.pc = 0x434DC;
    16
}
/// $06E786: move.w #$4000, d0
pub fn i_06e786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E78A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E78A: jsr $28134.l
pub fn i_06e78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E790);
    cpu.pc = 0x28134;
    16
}
/// $06E790: andi.w #$ffe3, $38(a6)
pub fn i_06e790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E796;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E796: ori.w #$0, $38(a6)
pub fn i_06e796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E79C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E79C: move.w #$1023, d0
pub fn i_06e79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E7A0: jsr $2352.l
pub fn i_06e7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7A6);
    cpu.pc = 0x2352;
    16
}
/// $06E7A6: lea.l $77fd6.l, a1
pub fn i_06e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06E7AC: jsr $4ae.l
pub fn i_06e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7B2);
    cpu.pc = 0x4AE;
    16
}
/// $06E7B2: jsr $5dd02.l
pub fn i_06e7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7B8);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7B8: lea.l $2d49d0.l, a1
pub fn i_06e7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D49D0;
    20
}
/// $06E7BE: jsr $77c7e.l
pub fn i_06e7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7C4);
    cpu.pc = 0x77C7E;
    16
}
/// $06E7C4: lea.l $6edfc(pc), a1
pub fn i_06e7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDFC;
    16
}
/// $06E7C8: jsr $4ae.l
pub fn i_06e7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7CE);
    cpu.pc = 0x4AE;
    16
}
/// $06E7CE: jsr $5dd02.l
pub fn i_06e7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7D4);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7D4: lea.l $6edfc(pc), a1
pub fn i_06e7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDFC;
    16
}
/// $06E7D8: jsr $4ae.l
pub fn i_06e7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7DE);
    cpu.pc = 0x4AE;
    16
}
/// $06E7DE: jsr $5dd02.l
pub fn i_06e7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7E4);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7E4: eori.b #$1, $3a(a0)
pub fn i_06e7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7EA;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E7EA: lea.l $6ee0c(pc), a1
pub fn i_06e7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE0C;
    16
}
/// $06E7EE: jsr $4ae.l
pub fn i_06e7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7F4);
    cpu.pc = 0x4AE;
    16
}
/// $06E7F4: jsr $5dd02.l
pub fn i_06e7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7FA);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7FA: lea.l $6ee1c(pc), a1
pub fn i_06e7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE1C;
    16
}
/// $06E7FE: jsr $4ae.l
pub fn i_06e7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E804);
    cpu.pc = 0x4AE;
    16
}
/// $06E804: jsr $5dd02.l
pub fn i_06e804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E80A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E80A);
    cpu.pc = 0x5DD02;
    16
}
// ===== Gunship_Free_06e80a  ($06E80A, 12 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E80A: move.b #$1, $20(a6)
pub fn i_06e80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E810;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E810: jmp $518.l
pub fn i_06e810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E816;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Gunship_Nop_06e816  ($06E816, 2 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E816: rts
pub fn i_06e816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E818;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06e818  ($06E818, 8 B, kernel/jmp_to_scheduler.c)
/// $06E818: jmp $518.l
pub fn i_06e818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E81E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06E81E: rts
pub fn i_06e81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E820;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_FlashCount5_06e820  ($06E820, 100 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E820: move.w #$3b, d1
pub fn i_06e820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E824;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E824: jsr $236e.l
pub fn i_06e824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E82A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E82A);
    cpu.pc = 0x236E;
    16
}
/// $06E82A: clr.b $20(a6)
pub fn i_06e82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E82E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06E82E: move.w #$10b8, d0
pub fn i_06e82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E832;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E832: jsr $2352.l
pub fn i_06e832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E838);
    cpu.pc = 0x2352;
    16
}
/// $06E838: move.w #$d000, d0
pub fn i_06e838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E83C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E83C: jsr $28134.l
pub fn i_06e83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E842;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E842);
    cpu.pc = 0x28134;
    16
}
/// $06E842: andi.w #$ffe3, $38(a6)
pub fn i_06e842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E848;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E848: ori.w #$18, $38(a6)
pub fn i_06e848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E84E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E84E: bset.b #$6, $12(a6)
pub fn i_06e84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E854;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E854: move.w #$d000, $34(a6)
pub fn i_06e854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E85A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E85A: lea.l $2d4c42.l, a0
pub fn i_06e85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E860;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C42;
    20
}
/// $06E860: jsr $28cd4.l
pub fn i_06e860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E866);
    cpu.pc = 0x28CD4;
    16
}
/// $06E866: move.w #$5, $70(a6)
pub fn i_06e866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E86C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E86C: clr.b $7e(a6)
pub fn i_06e86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E870;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06E870: lea.l $6e876(pc), a1
pub fn i_06e870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E876;
    16
}
/// $06E874: move.l a1, (a6)
pub fn i_06e874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E876: subq.w #$1, $70(a6)
pub fn i_06e876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E87A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E87A: cmpi.w #$0, $70(a6)
pub fn i_06e87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E880;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06E880: bgt.w $6e88a
pub fn i_06e880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E884;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E88A; return 10; }
    8
}
// ===== SetTaskHandler_06e884  ($06E884, 8 B, kernel/task_handlers.c)
/// $06E884: lea.l $6e88c(pc), a1
pub fn i_06e884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E888;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E88C;
    16
}
/// $06E888: move.l a1, (a6)
pub fn i_06e888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E88A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E88A: rts
pub fn i_06e88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E88C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpawnCrewArc_06e88c  ($06E88C, 158 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E88C: lea.l $6e966(pc), a1
pub fn i_06e88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E890;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E966;
    16
}
/// $06E890: jsr $4ae.l
pub fn i_06e890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E896);
    cpu.pc = 0x4AE;
    16
}
/// $06E896: jsr $5dd02.l
pub fn i_06e896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E89C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E89C);
    cpu.pc = 0x5DD02;
    16
}
/// $06E89C: bset.b #$6, $12(a0)
pub fn i_06e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8A2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E8A2: lea.l $6e8a8(pc), a1
pub fn i_06e8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E8A8;
    16
}
/// $06E8A6: move.l a1, (a6)
pub fn i_06e8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E8A8: movea.l $c(a6), a0
pub fn i_06e8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8AC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E8AC: lea.l $2c07ac.l, a1
pub fn i_06e8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8B2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $06E8B2: lea.l $2c072c.l, a2
pub fn i_06e8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8B8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $06E8B8: move.b $34(a6), d0
pub fn i_06e8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8BC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E8BC: andi.w #$ff, d0
pub fn i_06e8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8C0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E8C0: lsl.w #$1, d0
pub fn i_06e8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8C2;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06E8C2: move.w (a1, d0.w), d1
pub fn i_06e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8C6;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06E8C6: move.w (a2, d0.w), d2
pub fn i_06e8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8CA;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06E8CA: move.w $22(a0), d0
pub fn i_06e8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8CE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06E8CE: add.w d0, d1
pub fn i_06e8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8D0;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E8D0: move.w d1, $22(a6)
pub fn i_06e8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8D4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E8D4: addi.w #$100, d2
pub fn i_06e8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8D8;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E8D8: add.w $24(a0), d2
pub fn i_06e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8DC;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06E8DC: move.w d2, $24(a6)
pub fn i_06e8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8E0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E8E0: cmpi.w #$140, $22(a6)
pub fn i_06e8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $06E8E6: bgt.w $6e8f0
pub fn i_06e8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E8F0; return 10; }
    8
}
/// $06E8EA: jsr $28d70.l
pub fn i_06e8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E8F0);
    cpu.pc = 0x28D70;
    16
}
/// $06E8F0: subi.w #$80, $34(a6)
pub fn i_06e8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8F6;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E8F6: cmpi.b #$c0, $34(a6)
pub fn i_06e8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8FC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $06E8FC: bne.w $6e90a
pub fn i_06e8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E900;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E90A; return 10; }
    8
}
/// $06E900: movea.l $c(a6), a0
pub fn i_06e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E904;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E904: move.b #$1, $21(a0)
pub fn i_06e904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E90A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E90A: move.b $34(a6), d0
pub fn i_06e90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E90E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E90E: andi.w #$ff, d0
pub fn i_06e90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E912;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E912: cmpi.w #$80, d0
pub fn i_06e912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E916;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $06E916: bgt.w $6e920
pub fn i_06e916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E91A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E920; return 10; }
    8
}
/// $06E91A: lea.l $6e932(pc), a1
pub fn i_06e91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E91E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E932;
    16
}
/// $06E91E: move.l a1, (a6)
pub fn i_06e91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E920;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E920: cmpi.w #$ffe0, $22(a6)
pub fn i_06e920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E926;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $06E926: bgt.w $6e930
pub fn i_06e926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E92A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E930; return 10; }
    8
}
// ===== SetTaskHandler_06e92a  ($06E92A, 8 B, kernel/task_handlers.c)
/// $06E92A: lea.l $6e932(pc), a1
pub fn i_06e92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E92E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E932;
    16
}
/// $06E92E: move.l a1, (a6)
pub fn i_06e92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E930;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E930: rts
pub fn i_06e930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E932;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_FallToGround_06e932  ($06E932, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E932: movea.l $c(a6), a0
pub fn i_06e932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E936;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E936: move.b #$2, $21(a0)
pub fn i_06e936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E93C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E93C: move.w #$fe00, $28(a6)
pub fn i_06e93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E942;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E942: lea.l $6e948(pc), a1
pub fn i_06e942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E948;
    16
}
/// $06E946: move.l a1, (a6)
pub fn i_06e946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E948;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E948: jsr $27cee.l
pub fn i_06e948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E94E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E94E);
    cpu.pc = 0x27CEE;
    16
}
/// $06E94E: jsr $28d70.l
pub fn i_06e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E954);
    cpu.pc = 0x28D70;
    16
}
/// $06E954: cmpi.w #$ffe0, $22(a6)
pub fn i_06e954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E95A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $06E95A: bgt.w $6e964
pub fn i_06e95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E95E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E964; return 10; }
    8
}
// ===== SetTaskHandler_06e95e  ($06E95E, 8 B, kernel/task_handlers.c)
/// $06E95E: lea.l $6e818(pc), a1
pub fn i_06e95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E962;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06E962: move.l a1, (a6)
pub fn i_06e962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E964;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E964: rts
pub fn i_06e964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E966;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Crew_06e966  ($06E966, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E966: move.w #$3b, d1
pub fn i_06e966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E96A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E96A: jsr $236e.l
pub fn i_06e96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E970);
    cpu.pc = 0x236E;
    16
}
/// $06E970: move.w #$9, $1c(a6)
pub fn i_06e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E976;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E976: jsr $138fe.l
pub fn i_06e976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E97C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E97C);
    cpu.pc = 0x138FE;
    16
}
/// $06E97C: lea.l $2d4d6e.l, a0
pub fn i_06e97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E982;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4D6E;
    20
}
/// $06E982: jsr $28cd4.l
pub fn i_06e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E988;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E988);
    cpu.pc = 0x28CD4;
    16
}
/// $06E988: lea.l $6e98e(pc), a1
pub fn i_06e988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E98C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E98E;
    16
}
/// $06E98C: move.l a1, (a6)
pub fn i_06e98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E98E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E98E: jsr $5e506.l
pub fn i_06e98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E994;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E994);
    cpu.pc = 0x5E506;
    16
}
/// $06E994: btst.b #$0, $5a(a0)
pub fn i_06e994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E99A;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E99A: beq.w $6e9a4
pub fn i_06e99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E99E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E9A4; return 10; }
    8
}
/// $06E99E: bset.b #$0, $5a(a6)
pub fn i_06e99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9A4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E9A4: movea.l $c(a6), a0
pub fn i_06e9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9A8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E9A8: cmpi.b #$1, $20(a0)
pub fn i_06e9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9AE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06E9AE: bne.w $6e9b8
pub fn i_06e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E9B8; return 10; }
    8
}
/// $06E9B2: lea.l $6e818(pc), a1
pub fn i_06e9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06E9B6: move.l a1, (a6)
pub fn i_06e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_06e9b8  ($06E9B8, 8 B, thunks/jsr_abs_thunks.c)
/// $06E9B8: jsr $28d70.l
pub fn i_06e9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9BE);
    cpu.pc = 0x28D70;
    16
}
/// $06E9BE: rts
pub fn i_06e9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Gunner_06e9c0  ($06E9C0, 206 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E9C0: bset.b #$4, $6b(a6)
pub fn i_06e9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9C6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E9C6: move.b $99(a6), d0
pub fn i_06e9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9CA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E9CA: cmpi.b #$2, d0
pub fn i_06e9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9CE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $06E9CE: bne.w $6e9d8
pub fn i_06e9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E9D8; return 10; }
    8
}
/// $06E9D2: jsr $5e9b6.l
pub fn i_06e9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9D8);
    cpu.pc = 0x5E9B6;
    16
}
/// $06E9D8: andi.b #$1, d0
pub fn i_06e9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9DC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E9DC: move.b d0, $99(a6)
pub fn i_06e9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9E0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E9E0: move.w #$3c, d1
pub fn i_06e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E9E4: jsr $236e.l
pub fn i_06e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9EA);
    cpu.pc = 0x236E;
    16
}
/// $06E9EA: move.w #$6, $1c(a6)
pub fn i_06e9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E9F0: jsr $138fe.l
pub fn i_06e9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9F6);
    cpu.pc = 0x138FE;
    16
}
/// $06E9F6: lea.l $2b950a.l, a0
pub fn i_06e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B950A;
    20
}
/// $06E9FC: jsr $799de.l
pub fn i_06e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA02);
    cpu.pc = 0x799DE;
    16
}
/// $06EA02: move.w d0, $36(a6)
pub fn i_06ea02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA06;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EA06: move.w #$d000, d0
pub fn i_06ea06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA0A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EA0A: jsr $28134.l
pub fn i_06ea0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA10);
    cpu.pc = 0x28134;
    16
}
/// $06EA10: andi.w #$ffe3, $38(a6)
pub fn i_06ea10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA16;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EA16: ori.w #$1c, $38(a6)
pub fn i_06ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA1C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EA1C: move.l #$2d471a, $48(a6)
pub fn i_06ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA24;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D471A;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EA24: lea.l $2b9488.l, a0
pub fn i_06ea24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9488;
    20
}
/// $06EA2A: jsr $799de.l
pub fn i_06ea2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA30);
    cpu.pc = 0x799DE;
    16
}
/// $06EA30: move.w d0, $66(a6)
pub fn i_06ea30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA34;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EA34: lea.l $2d4e64.l, a0
pub fn i_06ea34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4E64;
    20
}
/// $06EA3A: jsr $28cd4.l
pub fn i_06ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA40);
    cpu.pc = 0x28CD4;
    16
}
/// $06EA40: lea.l $6ea46(pc), a1
pub fn i_06ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EA46;
    16
}
/// $06EA44: move.l a1, (a6)
pub fn i_06ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA46: jsr $5e506.l
pub fn i_06ea46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA4C);
    cpu.pc = 0x5E506;
    16
}
/// $06EA4C: jsr $6ef2c(pc)
pub fn i_06ea4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EA50);
    cpu.pc = 0x6EF2C;
    16
}
/// $06EA50: jsr $28d70.l
pub fn i_06ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA56);
    cpu.pc = 0x28D70;
    16
}
/// $06EA56: jsr $6ef66(pc)
pub fn i_06ea56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EA5A);
    cpu.pc = 0x6EF66;
    16
}
/// $06EA5A: bcc.w $6ea64
pub fn i_06ea5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EA64; return 10; }
    8
}
/// $06EA5E: lea.l $6ea96(pc), a1
pub fn i_06ea5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EA96;
    16
}
/// $06EA62: move.l a1, (a6)
pub fn i_06ea62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA64: movea.l $c(a6), a0
pub fn i_06ea64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA68;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06EA68: cmpi.b #$1, $20(a0)
pub fn i_06ea68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA6E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06EA6E: bne.w $6ea78
pub fn i_06ea6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6EA78; return 10; }
    8
}
/// $06EA72: lea.l $6e818(pc), a1
pub fn i_06ea72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EA76: move.l a1, (a6)
pub fn i_06ea76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA78: movea.l #$ffffffff, a0
pub fn i_06ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA7E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06EA7E: lea.l $2d49b6.l, a0
pub fn i_06ea7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06EA84: jsr $5dd5c.l
pub fn i_06ea84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA8A);
    cpu.pc = 0x5DD5C;
    16
}
/// $06EA8A: bcc.w $6ea94
pub fn i_06ea8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EA94; return 10; }
    8
}
// ===== SetTaskHandler_06ea8e  ($06EA8E, 8 B, kernel/task_handlers.c)
/// $06EA8E: lea.l $6e818(pc), a1
pub fn i_06ea8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EA92: move.l a1, (a6)
pub fn i_06ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA94: rts
pub fn i_06ea94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_GunnerAim_06ea96  ($06EA96, 60 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EA96: lea.l $2d4e70.l, a0
pub fn i_06ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4E70;
    20
}
/// $06EA9C: jsr $28cd4.l
pub fn i_06ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAA2);
    cpu.pc = 0x28CD4;
    16
}
/// $06EAA2: lea.l $6eaa8(pc), a1
pub fn i_06eaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EAA8;
    16
}
/// $06EAA6: move.l a1, (a6)
pub fn i_06eaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EAA8: jsr $5e506.l
pub fn i_06eaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAAE);
    cpu.pc = 0x5E506;
    16
}
/// $06EAAE: jsr $6ef2c(pc)
pub fn i_06eaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EAB2);
    cpu.pc = 0x6EF2C;
    16
}
/// $06EAB2: jsr $28d70.l
pub fn i_06eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAB8);
    cpu.pc = 0x28D70;
    16
}
/// $06EAB8: bcc.w $6ead0
pub fn i_06eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EABC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EAD0; return 10; }
    8
}
/// $06EABC: lea.l $6ead2(pc), a1
pub fn i_06eabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EAD2;
    16
}
/// $06EAC0: move.l a1, (a6)
pub fn i_06eac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EAC2: tst.b $9e(a6)
pub fn i_06eac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAC6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06EAC6: beq.w $6ead0
pub fn i_06eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EACA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EAD0; return 10; }
    8
}
/// $06EACA: lea.l $6ec54(pc), a1
pub fn i_06eaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EC54;
    16
}
/// $06EACE: move.l a1, (a6)
pub fn i_06eace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EAD0: bra.b $6ea64
pub fn i_06ead0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAD2;
    cpu.ir = 0x6092;
    cpu.pc = 0x6EA64;
    10
}
// ===== Gunship_GunnerFire_06ead2  ($06EAD2, 170 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EAD2: move.w #$d000, d0
pub fn i_06ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAD6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EAD6: jsr $28134.l
pub fn i_06ead6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EADC);
    cpu.pc = 0x28134;
    16
}
/// $06EADC: andi.w #$ffe3, $38(a6)
pub fn i_06eadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAE2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EAE2: ori.w #$1c, $38(a6)
pub fn i_06eae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAE8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EAE8: move.w #$ff80, $2e(a6)
pub fn i_06eae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EAEE: lea.l $2d4e9a.l, a0
pub fn i_06eaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4E9A;
    20
}
/// $06EAF4: jsr $28cd4.l
pub fn i_06eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAFA);
    cpu.pc = 0x28CD4;
    16
}
/// $06EAFA: lea.l $6eb00(pc), a1
pub fn i_06eafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EB00;
    16
}
/// $06EAFE: move.l a1, (a6)
pub fn i_06eafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB00: jsr $27d50.l
pub fn i_06eb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB06);
    cpu.pc = 0x27D50;
    16
}
/// $06EB06: jsr $6eff0(pc)
pub fn i_06eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EB0A);
    cpu.pc = 0x6EFF0;
    16
}
/// $06EB0A: bcc.w $6eb14
pub fn i_06eb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB14; return 10; }
    8
}
/// $06EB0E: lea.l $6ebd0(pc), a1
pub fn i_06eb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EBD0;
    16
}
/// $06EB12: move.l a1, (a6)
pub fn i_06eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB14: jsr $28d70.l
pub fn i_06eb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB1A);
    cpu.pc = 0x28D70;
    16
}
/// $06EB1A: jsr $283d8.l
pub fn i_06eb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB20);
    cpu.pc = 0x283D8;
    16
}
/// $06EB20: btst.b #$1, $13(a6)
pub fn i_06eb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB26;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06EB26: beq.w $6eb30
pub fn i_06eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EB30; return 10; }
    8
}
/// $06EB2A: lea.l $6ee8a(pc), a1
pub fn i_06eb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE8A;
    16
}
/// $06EB2E: move.l a1, (a6)
pub fn i_06eb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB30: jsr $2870a.l
pub fn i_06eb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB36);
    cpu.pc = 0x2870A;
    16
}
/// $06EB36: bcc.w $6eb4c
pub fn i_06eb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB4C; return 10; }
    8
}
/// $06EB3A: bclr.b #$3, $13(a6)
pub fn i_06eb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB40;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06EB40: lea.l $5e766.l, a0
pub fn i_06eb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06EB46: jsr $5e770.l
pub fn i_06eb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB4C);
    cpu.pc = 0x5E770;
    16
}
/// $06EB4C: jsr $28758.l
pub fn i_06eb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB52);
    cpu.pc = 0x28758;
    16
}
/// $06EB52: bcc.w $6eb66
pub fn i_06eb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB66; return 10; }
    8
}
/// $06EB56: move.w #$1022, d0
pub fn i_06eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB5A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EB5A: jsr $2352.l
pub fn i_06eb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB60);
    cpu.pc = 0x2352;
    16
}
/// $06EB60: lea.l $6ee8a(pc), a1
pub fn i_06eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE8A;
    16
}
/// $06EB64: move.l a1, (a6)
pub fn i_06eb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB66: movea.l #$ffffffff, a0
pub fn i_06eb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB6C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06EB6C: lea.l $2d49b6.l, a0
pub fn i_06eb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06EB72: jsr $5dd56.l
pub fn i_06eb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB78);
    cpu.pc = 0x5DD56;
    16
}
/// $06EB78: bcc.w $6eb82
pub fn i_06eb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB7C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB82; return 10; }
    8
}
// ===== SetTaskHandler_06eb7c  ($06EB7C, 8 B, kernel/task_handlers.c)
/// $06EB7C: lea.l $6e818(pc), a1
pub fn i_06eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EB80: move.l a1, (a6)
pub fn i_06eb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB82: rts
pub fn i_06eb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $06EBD0: move.l #$6eb84, $60(a6)
pub fn i_06ebd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBD8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x6EB84;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EBD8: move.w $2a(a6), d0
pub fn i_06ebd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBDC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EBDC: asr.w #$4, d0
pub fn i_06ebdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBDE;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06EBDE: neg.w d0
pub fn i_06ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBE0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06EBE0: move.w d0, $2e(a6)
pub fn i_06ebe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBE4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EBE4: lea.l $2d4ee4.l, a0
pub fn i_06ebe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4EE4;
    20
}
/// $06EBEA: jsr $28cd4.l
pub fn i_06ebea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EBF0);
    cpu.pc = 0x28CD4;
    16
}
/// $06EBF0: lea.l $6ebf6(pc), a1
pub fn i_06ebf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EBF6;
    16
}
/// $06EBF4: move.l a1, (a6)
pub fn i_06ebf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EBF6: tst.b $9d(a6)
pub fn i_06ebf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBFA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06EBFA: beq.w $6ec04
pub fn i_06ebfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EBFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EC04; return 10; }
    8
}
/// $06EBFE: bset.b #$6, $13(a6)
pub fn i_06ebfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC04;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06EC04: jsr $27cee.l
pub fn i_06ec04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC0A);
    cpu.pc = 0x27CEE;
    16
}
/// $06EC0A: jsr $28998.l
pub fn i_06ec0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC10);
    cpu.pc = 0x28998;
    16
}
/// $06EC10: jsr $6f020(pc)
pub fn i_06ec10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EC14);
    cpu.pc = 0x6F020;
    16
}
/// $06EC14: bcc.w $6ec1e
pub fn i_06ec14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EC1E; return 10; }
    8
}
/// $06EC18: lea.l $6ec2c(pc), a1
pub fn i_06ec18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EC2C;
    16
}
/// $06EC1C: move.l a1, (a6)
pub fn i_06ec1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EC1E: jsr $6f050(pc)
pub fn i_06ec1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EC22);
    cpu.pc = 0x6F050;
    16
}
/// $06EC22: jsr $28d70.l
pub fn i_06ec22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC28);
    cpu.pc = 0x28D70;
    16
}
/// $06EC28: bra.w $6eb1a
pub fn i_06ec28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EB1A;
    10
}
// ===== Gunship_GunnerIdle_06ec2c  ($06EC2C, 40 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EC2C: clr.w $2a(a6)
pub fn i_06ec2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC30;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EC30: clr.w $2e(a6)
pub fn i_06ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC34;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EC34: lea.l $6ec3a(pc), a1
pub fn i_06ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EC3A;
    16
}
/// $06EC38: move.l a1, (a6)
pub fn i_06ec38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EC3A: jsr $27cee.l
pub fn i_06ec3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC40);
    cpu.pc = 0x27CEE;
    16
}
/// $06EC40: jsr $28998.l
pub fn i_06ec40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC46);
    cpu.pc = 0x28998;
    16
}
/// $06EC46: jsr $6f050(pc)
pub fn i_06ec46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EC4A);
    cpu.pc = 0x6F050;
    16
}
/// $06EC4A: jsr $28d70.l
pub fn i_06ec4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC50);
    cpu.pc = 0x28D70;
    16
}
/// $06EC50: bra.w $6eb1a
pub fn i_06ec50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC54;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EB1A;
    10
}
// ===== Gunship_GunnerFireB_06ec54  ($06EC54, 230 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EC54: move.w #$d000, d0
pub fn i_06ec54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC58;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EC58: jsr $28134.l
pub fn i_06ec58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC5E);
    cpu.pc = 0x28134;
    16
}
/// $06EC5E: andi.w #$ffe3, $38(a6)
pub fn i_06ec5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC64;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EC64: ori.w #$1c, $38(a6)
pub fn i_06ec64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC6A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EC6A: move.w #$ff80, $2e(a6)
pub fn i_06ec6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EC70: move.l #$2d4962, $4c(a6)
pub fn i_06ec70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC78;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D4962;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EC78: move.l #$2d47c2, $48(a6)
pub fn i_06ec78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC80;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D47C2;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EC80: move.w #$28, d0
pub fn i_06ec80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EC84: move.w d0, $8c(a6)
pub fn i_06ec84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC88;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EC88: sub.w d0, $24(a6)
pub fn i_06ec88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC8C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EC8C: lea.l $2d4ea0.l, a0
pub fn i_06ec8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC92;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4EA0;
    20
}
/// $06EC92: jsr $28cd4.l
pub fn i_06ec92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC98);
    cpu.pc = 0x28CD4;
    16
}
/// $06EC98: lea.l $6ec9e(pc), a1
pub fn i_06ec98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EC9E;
    16
}
/// $06EC9C: move.l a1, (a6)
pub fn i_06ec9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EC9E: move.w $2a(a6), $8a(a6)
pub fn i_06ec9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECA4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECA4: move.w $24(a6), $86(a6)
pub fn i_06eca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECAA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECAA: move.b $27(a6), $89(a6)
pub fn i_06ecaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECB0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06ECB0: jsr $27bc8.l
pub fn i_06ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECB6);
    cpu.pc = 0x27BC8;
    16
}
/// $06ECB6: bcc.w $6ecde
pub fn i_06ecb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ECDE; return 10; }
    8
}
/// $06ECBA: move.w #$ff80, $2e(a6)
pub fn i_06ecba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06ECC0: move.w $8a(a6), $2a(a6)
pub fn i_06ecc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECC6;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECC6: move.w $86(a6), $24(a6)
pub fn i_06ecc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECCC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECCC: move.b $89(a6), $27(a6)
pub fn i_06eccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECD2;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06ECD2: jsr $27cee.l
pub fn i_06ecd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECD8);
    cpu.pc = 0x27CEE;
    16
}
/// $06ECD8: lea.l $6ed42(pc), a1
pub fn i_06ecd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ED42;
    16
}
/// $06ECDC: move.l a1, (a6)
pub fn i_06ecdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ECDE: move.w $8c(a6), d0
pub fn i_06ecde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECE2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06ECE2: add.w d0, $24(a6)
pub fn i_06ece2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECE6;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ECE6: jsr $28d70.l
pub fn i_06ece6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECEC);
    cpu.pc = 0x28D70;
    16
}
/// $06ECEC: move.w $8c(a6), d0
pub fn i_06ecec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECF0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06ECF0: sub.w d0, $24(a6)
pub fn i_06ecf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECF4;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ECF4: jsr $283d8.l
pub fn i_06ecf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECFA);
    cpu.pc = 0x283D8;
    16
}
/// $06ECFA: btst.b #$1, $13(a6)
pub fn i_06ecfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED00;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06ED00: beq.w $6ed0a
pub fn i_06ed00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6ED0A; return 10; }
    8
}
/// $06ED04: lea.l $6ee82(pc), a1
pub fn i_06ed04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE82;
    16
}
/// $06ED08: move.l a1, (a6)
pub fn i_06ed08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED0A: jsr $2870a.l
pub fn i_06ed0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED10);
    cpu.pc = 0x2870A;
    16
}
/// $06ED10: bcc.w $6ed24
pub fn i_06ed10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ED24; return 10; }
    8
}
/// $06ED14: move.w #$1022, d0
pub fn i_06ed14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06ED18: jsr $2352.l
pub fn i_06ed18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED1E);
    cpu.pc = 0x2352;
    16
}
/// $06ED1E: lea.l $6ee82(pc), a1
pub fn i_06ed1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE82;
    16
}
/// $06ED22: move.l a1, (a6)
pub fn i_06ed22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED24: movea.l #$ffffffff, a0
pub fn i_06ed24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED2A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06ED2A: lea.l $2d49b6.l, a0
pub fn i_06ed2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06ED30: jsr $5dd56.l
pub fn i_06ed30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED36);
    cpu.pc = 0x5DD56;
    16
}
/// $06ED36: bcc.w $6ed40
pub fn i_06ed36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ED40; return 10; }
    8
}
// ===== SetTaskHandler_06ed3a  ($06ED3A, 8 B, kernel/task_handlers.c)
/// $06ED3A: lea.l $6e818(pc), a1
pub fn i_06ed3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06ED3E: move.l a1, (a6)
pub fn i_06ed3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED40: rts
pub fn i_06ed40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_GunnerReload_06ed42  ($06ED42, 122 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06ED42: move.w $8c(a6), d0
pub fn i_06ed42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06ED46: add.w d0, $24(a6)
pub fn i_06ed46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED4A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ED4A: move.l #$2d490e, $4c(a6)
pub fn i_06ed4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED52;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D490E;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06ED52: move.l #$2d476e, $48(a6)
pub fn i_06ed52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED5A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D476E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06ED5A: move.w #$8, d0
pub fn i_06ed5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED5E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06ED5E: move.w d0, $8c(a6)
pub fn i_06ed5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED62;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06ED62: sub.w d0, $24(a6)
pub fn i_06ed62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED66;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ED66: move.w #$ff80, d1
pub fn i_06ed66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED6A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06ED6A: move.w d1, $2e(a6)
pub fn i_06ed6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED6E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06ED6E: lea.l $2d4ee4.l, a0
pub fn i_06ed6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4EE4;
    20
}
/// $06ED74: jsr $28cd4.l
pub fn i_06ed74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED7A);
    cpu.pc = 0x28CD4;
    16
}
/// $06ED7A: lea.l $6ed80(pc), a1
pub fn i_06ed7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ED80;
    16
}
/// $06ED7E: move.l a1, (a6)
pub fn i_06ed7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED80: tst.b $9d(a6)
pub fn i_06ed80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED84;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06ED84: beq.w $6ed8e
pub fn i_06ed84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED88;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6ED8E; return 10; }
    8
}
/// $06ED88: bset.b #$6, $13(a6)
pub fn i_06ed88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED8E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06ED8E: jsr $27bc8.l
pub fn i_06ed8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED94);
    cpu.pc = 0x27BC8;
    16
}
/// $06ED94: bcc.w $6ed9e
pub fn i_06ed94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ED9E; return 10; }
    8
}
/// $06ED98: lea.l $6edbc(pc), a1
pub fn i_06ed98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDBC;
    16
}
/// $06ED9C: move.l a1, (a6)
pub fn i_06ed9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED9E: jsr $6f050(pc)
pub fn i_06ed9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EDA2);
    cpu.pc = 0x6F050;
    16
}
/// $06EDA2: move.w $8c(a6), d0
pub fn i_06eda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDA6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDA6: add.w d0, $24(a6)
pub fn i_06eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDAA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDAA: jsr $28d70.l
pub fn i_06edaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EDB0);
    cpu.pc = 0x28D70;
    16
}
/// $06EDB0: move.w $8c(a6), d0
pub fn i_06edb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDB4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDB4: sub.w d0, $24(a6)
pub fn i_06edb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDB8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDB8: bra.w $6ecf4
pub fn i_06edb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6ECF4;
    10
}
// ===== Gunship_GunnerWait_06edbc  ($06EDBC, 64 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EDBC: clr.w $2e(a6)
pub fn i_06edbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDC0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDC0: clr.w $2a(a6)
pub fn i_06edc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDC4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDC4: lea.l $6edca(pc), a1
pub fn i_06edc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDCA;
    16
}
/// $06EDC8: move.l a1, (a6)
pub fn i_06edc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EDCA: tst.b $9d(a6)
pub fn i_06edca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDCE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06EDCE: beq.w $6edd8
pub fn i_06edce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EDD8; return 10; }
    8
}
/// $06EDD2: bset.b #$6, $13(a6)
pub fn i_06edd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDD8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06EDD8: jsr $27a92.l
pub fn i_06edd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EDDE);
    cpu.pc = 0x27A92;
    16
}
/// $06EDDE: jsr $6f050(pc)
pub fn i_06edde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EDE2);
    cpu.pc = 0x6F050;
    16
}
/// $06EDE2: move.w $8c(a6), d0
pub fn i_06ede2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDE6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDE6: add.w d0, $24(a6)
pub fn i_06ede6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDEA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDEA: jsr $28d70.l
pub fn i_06edea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EDF0);
    cpu.pc = 0x28D70;
    16
}
/// $06EDF0: move.w $8c(a6), d0
pub fn i_06edf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDF4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDF4: sub.w d0, $24(a6)
pub fn i_06edf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDF8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDF8: bra.w $6ecf4
pub fn i_06edf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6ECF4;
    10
}
// ===== Gunship_WreckPiece_06edfc  ($06EDFC, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EDFC: lea.l $2d4fc4.l, a0
pub fn i_06edfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4FC4;
    20
}
/// $06EE02: jsr $28cd4.l
pub fn i_06ee02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE08);
    cpu.pc = 0x28CD4;
    16
}
/// $06EE08: bra.w $6ee28
pub fn i_06ee08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EE28;
    10
}
/// $06EE0C: lea.l $2d501a.l, a0
pub fn i_06ee0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D501A;
    20
}
/// $06EE12: jsr $28cd4.l
pub fn i_06ee12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE18);
    cpu.pc = 0x28CD4;
    16
}
/// $06EE18: bra.w $6ee28
pub fn i_06ee18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EE28;
    10
}
/// $06EE1C: lea.l $2d5070.l, a0
pub fn i_06ee1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5070;
    20
}
/// $06EE22: jsr $28cd4.l
pub fn i_06ee22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE28);
    cpu.pc = 0x28CD4;
    16
}
/// $06EE28: move.w #$8000, d0
pub fn i_06ee28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE2C: jsr $28134.l
pub fn i_06ee2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE32);
    cpu.pc = 0x28134;
    16
}
/// $06EE32: andi.w #$ffe3, $38(a6)
pub fn i_06ee32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE38;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EE38: ori.w #$0, $38(a6)
pub fn i_06ee38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE3E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EE3E: move.w #$3b, d1
pub fn i_06ee3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE42;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE42: jsr $236e.l
pub fn i_06ee42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE48);
    cpu.pc = 0x236E;
    16
}
/// $06EE48: jmp $6dce0.l
pub fn i_06ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE4E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
// ===== Gunship_Shell_06ee4e  ($06EE4E, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EE4E: move.w #$4, d1
pub fn i_06ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE52;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE52: jsr $236e.l
pub fn i_06ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE58);
    cpu.pc = 0x236E;
    16
}
/// $06EE58: lea.l $2dd8b6.l, a0
pub fn i_06ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD8B6;
    20
}
/// $06EE5E: jsr $28cd4.l
pub fn i_06ee5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE64);
    cpu.pc = 0x28CD4;
    16
}
/// $06EE64: lea.l $6ee6a(pc), a1
pub fn i_06ee64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE6A;
    16
}
/// $06EE68: move.l a1, (a6)
pub fn i_06ee68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EE6A: jsr $2783a.l
pub fn i_06ee6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE70);
    cpu.pc = 0x2783A;
    16
}
/// $06EE70: jsr $28d70.l
pub fn i_06ee70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE76);
    cpu.pc = 0x28D70;
    16
}
/// $06EE76: bcc.w $6ee80
pub fn i_06ee76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EE80; return 10; }
    8
}
// ===== SetTaskHandler_06ee7a  ($06EE7A, 8 B, kernel/task_handlers.c)
/// $06EE7A: lea.l $6e818(pc), a1
pub fn i_06ee7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EE7E: move.l a1, (a6)
pub fn i_06ee7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EE80: rts
pub fn i_06ee80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Explode_06ee82  ($06EE82, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EE82: move.w $8c(a6), d0
pub fn i_06ee82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE86;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EE86: add.w d0, $24(a6)
pub fn i_06ee86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE8A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EE8A: move.w #$4000, d0
pub fn i_06ee8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE8E: jsr $28134.l
pub fn i_06ee8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE94);
    cpu.pc = 0x28134;
    16
}
/// $06EE94: andi.w #$ffe3, $38(a6)
pub fn i_06ee94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE9A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EE9A: ori.w #$1c, $38(a6)
pub fn i_06ee9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEA0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EEA0: move.l #$ffffffff, $48(a6)
pub fn i_06eea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEA8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EEA8: jmp $77f6a.l
pub fn i_06eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Gunship_SetPathByParam98_06eeae  ($06EEAE, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EEAE: move.b $98(a6), d0
pub fn i_06eeae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEB2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06EEB2: andi.w #$7, d0
pub fn i_06eeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEB6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EEB6: lsl.w #$2, d0
pub fn i_06eeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEB8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06EEB8: lea.l $2d4a2e.l, a0
pub fn i_06eeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A2E;
    20
}
/// $06EEBE: move.l (a0, d0.w), $90(a6)
pub fn i_06eebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEC4;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06EEC4: clr.w $94(a6)
pub fn i_06eec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEC8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EEC8: clr.b $96(a6)
pub fn i_06eec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EECC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06EECC: rts
pub fn i_06eecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EECE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_HeightClass_06eece  ($06EECE, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EECE: move.w #$2, d0
pub fn i_06eece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EED2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EED2: move.w $24(a6), d1
pub fn i_06eed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EED6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EED6: cmpi.w #$1a0, d1
pub fn i_06eed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEDA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1A0, Size::Word, false);
    8
}
/// $06EEDA: bgt.w $6eeec
pub fn i_06eeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEDE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6EEEC; return 10; }
    8
}
/// $06EEDE: move.w #$1, d0
pub fn i_06eede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EEE2: cmpi.w #$160, d1
pub fn i_06eee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEE6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x160, Size::Word, false);
    8
}
/// $06EEE6: bgt.w $6eeec
pub fn i_06eee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEEA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6EEEC; return 10; }
    8
}
/// $06EEEA: clr.w d0
pub fn i_06eeea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEEC;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06EEEC: btst.b #$0, $3a(a6)
pub fn i_06eeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEF2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06EEF2: beq.w $6eefa
pub fn i_06eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EEFA; return 10; }
    8
}
/// $06EEF6: addi.w #$3, d0
pub fn i_06eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEFA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EEFA: add.w d0, d0
pub fn i_06eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEFC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06EEFC: lea.l $2d4a22.l, a0
pub fn i_06eefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A22;
    20
}
/// $06EF02: move.w (a0, d0.w), d0
pub fn i_06ef02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF06;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
// ===== JsrAbsThunk_06ef06  ($06EF06, 8 B, thunks/jsr_abs_thunks.c)
/// $06EF06: jsr $2352.l
pub fn i_06ef06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EF0C);
    cpu.pc = 0x2352;
    16
}
/// $06EF0C: rts
pub fn i_06ef0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0006EF0E  ($06EF0E, 2 B, thunks/trivial_rts.c)
/// $06EF0E: rts
pub fn i_06ef0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_AddDepthOffset_06ef10  ($06EF10, 28 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF10: move.b $106f28.l, d0
pub fn i_06ef10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF16;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $06EF16: andi.w #$3, d0
pub fn i_06ef16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF1A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EF1A: add.w d0, d0
pub fn i_06ef1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF1C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06EF1C: lea.l $2d4a9e.l, a0
pub fn i_06ef1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A9E;
    20
}
/// $06EF22: move.w (a0, d0.w), d1
pub fn i_06ef22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF26;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF26: add.w d1, $24(a6)
pub fn i_06ef26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF2A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF2A: rts
pub fn i_06ef2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_CrewOffsetByParent7D_06ef2c  ($06EF2C, 58 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF2C: movea.l $c(a6), a0
pub fn i_06ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF30;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06EF30: move.b $7d(a0), d1
pub fn i_06ef30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF34;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06EF34: andi.w #$7, d1
pub fn i_06ef34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF38;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EF38: lsl.w #$3, d1
pub fn i_06ef38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF3A;
    cpu.ir = 0xE749;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06EF3A: lea.l $2d4a5e.l, a1
pub fn i_06ef3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF40;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D4A5E;
    20
}
/// $06EF40: move.w $4(a1, d1.w), d0
pub fn i_06ef40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF44;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF44: add.w d0, $38(a6)
pub fn i_06ef44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF48;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF48: move.w $2(a1, d1.w), d0
pub fn i_06ef48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF4C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF4C: add.w d0, $24(a6)
pub fn i_06ef4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF50;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF50: move.w (a1, d1.w), d0
pub fn i_06ef50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF54;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF54: btst.b #$0, $3a(a6)
pub fn i_06ef54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF5A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06EF5A: beq.w $6ef60
pub fn i_06ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EF60; return 10; }
    8
}
/// $06EF5E: neg.w d0
pub fn i_06ef5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF60;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06EF60: add.w d0, $22(a6)
pub fn i_06ef60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF64;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF64: rts
pub fn i_06ef64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_ParentFlag7F_06ef66  ($06EF66, 14 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF66: movea.l $c(a6), a0
pub fn i_06ef66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF6A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06EF6A: cmpi.b #$0, $7f(a0)
pub fn i_06ef6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF70;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06EF70: beq.w $6ef7a
pub fn i_06ef70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EF7A; return 10; }
    8
}
// ===== ClearXN_06ef74  ($06EF74, 6 B, thunks/ccr_helpers.c)
/// $06EF74: andi.b #$ee, ccr
pub fn i_06ef74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF78;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06EF78: rts
pub fn i_06ef78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06ef7a  ($06EF7A, 6 B, thunks/ccr_helpers.c)
/// $06EF7A: ori.b #$11, ccr
pub fn i_06ef7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF7E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06EF7E: rts
pub fn i_06ef7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_InScreen_06ef80  ($06EF80, 68 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF80: move.w $24(a6), d2
pub fn i_06ef80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF84;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EF84: addq.w #$8, d2
pub fn i_06ef84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF86;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x06D464, i_06d464),
    (0x06D468, i_06d468),
    (0x06D46E, i_06d46e),
    (0x06D474, i_06d474),
    (0x06D478, i_06d478),
    (0x06D47C, i_06d47c),
    (0x06D482, i_06d482),
    (0x06D484, i_06d484),
    (0x06D48A, i_06d48a),
    (0x06D490, i_06d490),
    (0x06D496, i_06d496),
    (0x06D49A, i_06d49a),
    (0x06D49E, i_06d49e),
    (0x06D4A4, i_06d4a4),
    (0x06D4A8, i_06d4a8),
    (0x06D4AC, i_06d4ac),
    (0x06D4AE, i_06d4ae),
    (0x06D4B4, i_06d4b4),
    (0x06D4BA, i_06d4ba),
    (0x06D4C0, i_06d4c0),
    (0x06D4C2, i_06d4c2),
    (0x06D4C6, i_06d4c6),
    (0x06D4CC, i_06d4cc),
    (0x06D4D0, i_06d4d0),
    (0x06D4D6, i_06d4d6),
    (0x06D4DA, i_06d4da),
    (0x06D4E0, i_06d4e0),
    (0x06D4E4, i_06d4e4),
    (0x06D4E8, i_06d4e8),
    (0x06D4EC, i_06d4ec),
    (0x06D4F0, i_06d4f0),
    (0x06D4F6, i_06d4f6),
    (0x06D4FC, i_06d4fc),
    (0x06D502, i_06d502),
    (0x06D506, i_06d506),
    (0x06D50A, i_06d50a),
    (0x06D50C, i_06d50c),
    (0x06D510, i_06d510),
    (0x06D514, i_06d514),
    (0x06D518, i_06d518),
    (0x06D51A, i_06d51a),
    (0x06D51E, i_06d51e),
    (0x06D522, i_06d522),
    (0x06D528, i_06d528),
    (0x06D52C, i_06d52c),
    (0x06D52E, i_06d52e),
    (0x06D530, i_06d530),
    (0x06D532, i_06d532),
    (0x06D536, i_06d536),
    (0x06D53A, i_06d53a),
    (0x06D540, i_06d540),
    (0x06D544, i_06d544),
    (0x06D548, i_06d548),
    (0x06D54E, i_06d54e),
    (0x06D554, i_06d554),
    (0x06D558, i_06d558),
    (0x06D55E, i_06d55e),
    (0x06D562, i_06d562),
    (0x06D566, i_06d566),
    (0x06D56C, i_06d56c),
    (0x06D572, i_06d572),
    (0x06D574, i_06d574),
    (0x06D57A, i_06d57a),
    (0x06D57E, i_06d57e),
    (0x06D582, i_06d582),
    (0x06D584, i_06d584),
    (0x06D588, i_06d588),
    (0x06D58E, i_06d58e),
    (0x06D594, i_06d594),
    (0x06D59A, i_06d59a),
    (0x06D59C, i_06d59c),
    (0x06D5A0, i_06d5a0),
    (0x06D5A6, i_06d5a6),
    (0x06D5AC, i_06d5ac),
    (0x06D5B2, i_06d5b2),
    (0x06D5B8, i_06d5b8),
    (0x06D5BE, i_06d5be),
    (0x06D5C4, i_06d5c4),
    (0x06D5CA, i_06d5ca),
    (0x06D5CC, i_06d5cc),
    (0x06D5D0, i_06d5d0),
    (0x06D5D6, i_06d5d6),
    (0x06D5DC, i_06d5dc),
    (0x06D5E2, i_06d5e2),
    (0x06D5E8, i_06d5e8),
    (0x06D5EE, i_06d5ee),
    (0x06D5F4, i_06d5f4),
    (0x06D5FA, i_06d5fa),
    (0x06D600, i_06d600),
    (0x06D602, i_06d602),
    (0x06D606, i_06d606),
    (0x06D60C, i_06d60c),
    (0x06D612, i_06d612),
    (0x06D618, i_06d618),
    (0x06D61E, i_06d61e),
    (0x06D624, i_06d624),
    (0x06D62A, i_06d62a),
    (0x06D630, i_06d630),
    (0x06D636, i_06d636),
    (0x06D638, i_06d638),
    (0x06D63C, i_06d63c),
    (0x06D640, i_06d640),
    (0x06D644, i_06d644),
    (0x06D648, i_06d648),
    (0x06D64C, i_06d64c),
    (0x06D64E, i_06d64e),
    (0x06D652, i_06d652),
    (0x06D654, i_06d654),
    (0x06D658, i_06d658),
    (0x06D65C, i_06d65c),
    (0x06D660, i_06d660),
    (0x06D666, i_06d666),
    (0x06D66A, i_06d66a),
    (0x06D66E, i_06d66e),
    (0x06D674, i_06d674),
    (0x06D67A, i_06d67a),
    (0x06D67E, i_06d67e),
    (0x06D682, i_06d682),
    (0x06D686, i_06d686),
    (0x06D68A, i_06d68a),
    (0x06D68E, i_06d68e),
    (0x06D692, i_06d692),
    (0x06D698, i_06d698),
    (0x06D69C, i_06d69c),
    (0x06D6A0, i_06d6a0),
    (0x06D6A6, i_06d6a6),
    (0x06D6AC, i_06d6ac),
    (0x06D6B0, i_06d6b0),
    (0x06D6B6, i_06d6b6),
    (0x06D6BA, i_06d6ba),
    (0x06D6BE, i_06d6be),
    (0x06D6C2, i_06d6c2),
    (0x06D6C6, i_06d6c6),
    (0x06D6CC, i_06d6cc),
    (0x06D6D0, i_06d6d0),
    (0x06D6D4, i_06d6d4),
    (0x06D6DA, i_06d6da),
    (0x06D6E0, i_06d6e0),
    (0x06D6E4, i_06d6e4),
    (0x06D6EA, i_06d6ea),
    (0x06D6EE, i_06d6ee),
    (0x06D6F4, i_06d6f4),
    (0x06D6FA, i_06d6fa),
    (0x06D6FE, i_06d6fe),
    (0x06D704, i_06d704),
    (0x06D70A, i_06d70a),
    (0x06D710, i_06d710),
    (0x06D716, i_06d716),
    (0x06D71A, i_06d71a),
    (0x06D720, i_06d720),
    (0x06D728, i_06d728),
    (0x06D730, i_06d730),
    (0x06D736, i_06d736),
    (0x06D73A, i_06d73a),
    (0x06D73E, i_06d73e),
    (0x06D740, i_06d740),
    (0x06D744, i_06d744),
    (0x06D748, i_06d748),
    (0x06D74C, i_06d74c),
    (0x06D74E, i_06d74e),
    (0x06D752, i_06d752),
    (0x06D756, i_06d756),
    (0x06D75C, i_06d75c),
    (0x06D762, i_06d762),
    (0x06D768, i_06d768),
    (0x06D76C, i_06d76c),
    (0x06D770, i_06d770),
    (0x06D774, i_06d774),
    (0x06D77A, i_06d77a),
    (0x06D780, i_06d780),
    (0x06D786, i_06d786),
    (0x06D78A, i_06d78a),
    (0x06D790, i_06d790),
    (0x06D796, i_06d796),
    (0x06D79C, i_06d79c),
    (0x06D7A0, i_06d7a0),
    (0x06D7A2, i_06d7a2),
    (0x06D7A6, i_06d7a6),
    (0x06D7AC, i_06d7ac),
    (0x06D7B2, i_06d7b2),
    (0x06D7B6, i_06d7b6),
    (0x06D7B8, i_06d7b8),
    (0x06D7BE, i_06d7be),
    (0x06D7C4, i_06d7c4),
    (0x06D7CA, i_06d7ca),
    (0x06D7CE, i_06d7ce),
    (0x06D7D2, i_06d7d2),
    (0x06D7D6, i_06d7d6),
    (0x06D7DC, i_06d7dc),
    (0x06D7E0, i_06d7e0),
    (0x06D7E4, i_06d7e4),
    (0x06D7E8, i_06d7e8),
    (0x06D7EC, i_06d7ec),
    (0x06D7F0, i_06d7f0),
    (0x06D7F4, i_06d7f4),
    (0x06D7F6, i_06d7f6),
    (0x06D7FA, i_06d7fa),
    (0x06D7FE, i_06d7fe),
    (0x06D802, i_06d802),
    (0x06D804, i_06d804),
    (0x06D808, i_06d808),
    (0x06D80E, i_06d80e),
    (0x06D814, i_06d814),
    (0x06D818, i_06d818),
    (0x06D81A, i_06d81a),
    (0x06D820, i_06d820),
    (0x06D826, i_06d826),
    (0x06D82C, i_06d82c),
    (0x06D830, i_06d830),
    (0x06D834, i_06d834),
    (0x06D836, i_06d836),
    (0x06D83A, i_06d83a),
    (0x06D840, i_06d840),
    (0x06D846, i_06d846),
    (0x06D84A, i_06d84a),
    (0x06D84C, i_06d84c),
    (0x06D852, i_06d852),
    (0x06D858, i_06d858),
    (0x06D85E, i_06d85e),
    (0x06D862, i_06d862),
    (0x06D866, i_06d866),
    (0x06D86A, i_06d86a),
    (0x06D870, i_06d870),
    (0x06D876, i_06d876),
    (0x06D87A, i_06d87a),
    (0x06D87E, i_06d87e),
    (0x06D880, i_06d880),
    (0x06D884, i_06d884),
    (0x06D88A, i_06d88a),
    (0x06D890, i_06d890),
    (0x06D894, i_06d894),
    (0x06D89A, i_06d89a),
    (0x06D8A0, i_06d8a0),
    (0x06D8A4, i_06d8a4),
    (0x06D8A6, i_06d8a6),
    (0x06D8AC, i_06d8ac),
    (0x06D8B2, i_06d8b2),
    (0x06D8B8, i_06d8b8),
    (0x06D8BC, i_06d8bc),
    (0x06D8C2, i_06d8c2),
    (0x06D8C6, i_06d8c6),
    (0x06D8CC, i_06d8cc),
    (0x06D8D0, i_06d8d0),
    (0x06D8D4, i_06d8d4),
    (0x06D8D6, i_06d8d6),
    (0x06D8DC, i_06d8dc),
    (0x06D8E0, i_06d8e0),
    (0x06D8E4, i_06d8e4),
    (0x06D8E6, i_06d8e6),
    (0x06D8EC, i_06d8ec),
    (0x06D8F2, i_06d8f2),
    (0x06D8F6, i_06d8f6),
    (0x06D8FA, i_06d8fa),
    (0x06D8FC, i_06d8fc),
    (0x06D900, i_06d900),
    (0x06D906, i_06d906),
    (0x06D90C, i_06d90c),
    (0x06D912, i_06d912),
    (0x06D916, i_06d916),
    (0x06D91C, i_06d91c),
    (0x06D922, i_06d922),
    (0x06D926, i_06d926),
    (0x06D92C, i_06d92c),
    (0x06D932, i_06d932),
    (0x06D938, i_06d938),
    (0x06D93E, i_06d93e),
    (0x06D942, i_06d942),
    (0x06D946, i_06d946),
    (0x06D948, i_06d948),
    (0x06D94E, i_06d94e),
    (0x06D954, i_06d954),
    (0x06D95A, i_06d95a),
    (0x06D95E, i_06d95e),
    (0x06D962, i_06d962),
    (0x06D968, i_06d968),
    (0x06D96C, i_06d96c),
    (0x06D970, i_06d970),
    (0x06D972, i_06d972),
    (0x06D976, i_06d976),
    (0x06D97C, i_06d97c),
    (0x06D980, i_06d980),
    (0x06D984, i_06d984),
    (0x06D986, i_06d986),
    (0x06D98A, i_06d98a),
    (0x06D990, i_06d990),
    (0x06D994, i_06d994),
    (0x06D99A, i_06d99a),
    (0x06D9A0, i_06d9a0),
    (0x06D9A6, i_06d9a6),
    (0x06D9AC, i_06d9ac),
    (0x06D9B0, i_06d9b0),
    (0x06D9B4, i_06d9b4),
    (0x06D9B6, i_06d9b6),
    (0x06D9BA, i_06d9ba),
    (0x06D9BE, i_06d9be),
    (0x06D9C4, i_06d9c4),
    (0x06D9C8, i_06d9c8),
    (0x06D9CC, i_06d9cc),
    (0x06D9CE, i_06d9ce),
    (0x06D9D4, i_06d9d4),
    (0x06D9DA, i_06d9da),
    (0x06D9E0, i_06d9e0),
    (0x06D9E4, i_06d9e4),
    (0x06D9E8, i_06d9e8),
    (0x06D9EA, i_06d9ea),
    (0x06D9EC, i_06d9ec),
    (0x06D9F2, i_06d9f2),
    (0x06D9F8, i_06d9f8),
    (0x06D9FE, i_06d9fe),
    (0x06DA04, i_06da04),
    (0x06DA08, i_06da08),
    (0x06DA0A, i_06da0a),
    (0x06DA10, i_06da10),
    (0x06DA16, i_06da16),
    (0x06DA1A, i_06da1a),
    (0x06DA1E, i_06da1e),
    (0x06DA24, i_06da24),
    (0x06DA28, i_06da28),
    (0x06DA2C, i_06da2c),
    (0x06DA2E, i_06da2e),
    (0x06DA34, i_06da34),
    (0x06DA3A, i_06da3a),
    (0x06DA40, i_06da40),
    (0x06DA44, i_06da44),
    (0x06DA48, i_06da48),
    (0x06DA4A, i_06da4a),
    (0x06DA4C, i_06da4c),
    (0x06DA50, i_06da50),
    (0x06DA56, i_06da56),
    (0x06DA5C, i_06da5c),
    (0x06DA62, i_06da62),
    (0x06DA68, i_06da68),
    (0x06DA6E, i_06da6e),
    (0x06DA72, i_06da72),
    (0x06DA78, i_06da78),
    (0x06DA7A, i_06da7a),
    (0x06DA80, i_06da80),
    (0x06DA86, i_06da86),
    (0x06DA88, i_06da88),
    (0x06DA8E, i_06da8e),
    (0x06DA90, i_06da90),
    (0x06DA96, i_06da96),
    (0x06DA9C, i_06da9c),
    (0x06DA9E, i_06da9e),
    (0x06DAA2, i_06daa2),
    (0x06DAA8, i_06daa8),
    (0x06DAAE, i_06daae),
    (0x06DAB4, i_06dab4),
    (0x06DABC, i_06dabc),
    (0x06DAC2, i_06dac2),
    (0x06DAC8, i_06dac8),
    (0x06DACC, i_06dacc),
    (0x06DAD2, i_06dad2),
    (0x06DAD8, i_06dad8),
    (0x06DADE, i_06dade),
    (0x06DAE6, i_06dae6),
    (0x06DAEC, i_06daec),
    (0x06DAF2, i_06daf2),
    (0x06DAF6, i_06daf6),
    (0x06DAFC, i_06dafc),
    (0x06DB02, i_06db02),
    (0x06DB08, i_06db08),
    (0x06DB10, i_06db10),
    (0x06DB16, i_06db16),
    (0x06DB1A, i_06db1a),
    (0x06DB1C, i_06db1c),
    (0x06DB22, i_06db22),
    (0x06DB26, i_06db26),
    (0x06DB2C, i_06db2c),
    (0x06DB32, i_06db32),
    (0x06DB38, i_06db38),
    (0x06DB3C, i_06db3c),
    (0x06DB42, i_06db42),
    (0x06DB48, i_06db48),
    (0x06DB4E, i_06db4e),
    (0x06DB54, i_06db54),
    (0x06DB58, i_06db58),
    (0x06DB5E, i_06db5e),
    (0x06DB64, i_06db64),
    (0x06DB6C, i_06db6c),
    (0x06DB72, i_06db72),
    (0x06DB78, i_06db78),
    (0x06DB7E, i_06db7e),
    (0x06DB82, i_06db82),
    (0x06DB84, i_06db84),
    (0x06DB8A, i_06db8a),
    (0x06DB90, i_06db90),
    (0x06DB94, i_06db94),
    (0x06DB98, i_06db98),
    (0x06DB9A, i_06db9a),
    (0x06DBA0, i_06dba0),
    (0x06DBA6, i_06dba6),
    (0x06DBAA, i_06dbaa),
    (0x06DBB0, i_06dbb0),
    (0x06DBB4, i_06dbb4),
    (0x06DBBC, i_06dbbc),
    (0x06DBC2, i_06dbc2),
    (0x06DBC8, i_06dbc8),
    (0x06DBCC, i_06dbcc),
    (0x06DBD0, i_06dbd0),
    (0x06DBD2, i_06dbd2),
    (0x06DBD4, i_06dbd4),
    (0x06DBDA, i_06dbda),
    (0x06DBDE, i_06dbde),
    (0x06DBE2, i_06dbe2),
    (0x06DBE8, i_06dbe8),
    (0x06DBEC, i_06dbec),
    (0x06DBF0, i_06dbf0),
    (0x06DBF6, i_06dbf6),
    (0x06DBFC, i_06dbfc),
    (0x06DC02, i_06dc02),
    (0x06DC08, i_06dc08),
    (0x06DC0E, i_06dc0e),
    (0x06DC12, i_06dc12),
    (0x06DC14, i_06dc14),
    (0x06DC1A, i_06dc1a),
    (0x06DC1E, i_06dc1e),
    (0x06DC22, i_06dc22),
    (0x06DC24, i_06dc24),
    (0x06DC2A, i_06dc2a),
    (0x06DC30, i_06dc30),
    (0x06DC36, i_06dc36),
    (0x06DC3A, i_06dc3a),
    (0x06DC3E, i_06dc3e),
    (0x06DC40, i_06dc40),
    (0x06DC46, i_06dc46),
    (0x06DC4C, i_06dc4c),
    (0x06DC52, i_06dc52),
    (0x06DC56, i_06dc56),
    (0x06DC5A, i_06dc5a),
    (0x06DC5C, i_06dc5c),
    (0x06DC5E, i_06dc5e),
    (0x06DC62, i_06dc62),
    (0x06DC68, i_06dc68),
    (0x06DC6E, i_06dc6e),
    (0x06DC72, i_06dc72),
    (0x06DC74, i_06dc74),
    (0x06DC78, i_06dc78),
    (0x06DC7E, i_06dc7e),
    (0x06DC84, i_06dc84),
    (0x06DC88, i_06dc88),
    (0x06DC8C, i_06dc8c),
    (0x06DC8E, i_06dc8e),
    (0x06DC90, i_06dc90),
    (0x06DC96, i_06dc96),
    (0x06DC9A, i_06dc9a),
    (0x06DCA0, i_06dca0),
    (0x06DCA4, i_06dca4),
    (0x06DCAA, i_06dcaa),
    (0x06DCB0, i_06dcb0),
    (0x06DCB6, i_06dcb6),
    (0x06DCBC, i_06dcbc),
    (0x06DCC2, i_06dcc2),
    (0x06DCC6, i_06dcc6),
    (0x06DCC8, i_06dcc8),
    (0x06DCCE, i_06dcce),
    (0x06DCD4, i_06dcd4),
    (0x06DCD8, i_06dcd8),
    (0x06DCDC, i_06dcdc),
    (0x06DCDE, i_06dcde),
    (0x06DCE0, i_06dce0),
    (0x06DCE6, i_06dce6),
    (0x06DCEA, i_06dcea),
    (0x06DCF0, i_06dcf0),
    (0x06DCF6, i_06dcf6),
    (0x06DCFC, i_06dcfc),
    (0x06DD00, i_06dd00),
    (0x06DD06, i_06dd06),
    (0x06DD0C, i_06dd0c),
    (0x06DD12, i_06dd12),
    (0x06DD16, i_06dd16),
    (0x06DD1C, i_06dd1c),
    (0x06DD20, i_06dd20),
    (0x06DD26, i_06dd26),
    (0x06DD2A, i_06dd2a),
    (0x06DD30, i_06dd30),
    (0x06DD36, i_06dd36),
    (0x06DD3C, i_06dd3c),
    (0x06DD42, i_06dd42),
    (0x06DD48, i_06dd48),
    (0x06DD4E, i_06dd4e),
    (0x06DD50, i_06dd50),
    (0x06DD54, i_06dd54),
    (0x06DD58, i_06dd58),
    (0x06DD5C, i_06dd5c),
    (0x06DD62, i_06dd62),
    (0x06DD66, i_06dd66),
    (0x06DD6C, i_06dd6c),
    (0x06DD6E, i_06dd6e),
    (0x06DD72, i_06dd72),
    (0x06DD78, i_06dd78),
    (0x06DD7C, i_06dd7c),
    (0x06DD82, i_06dd82),
    (0x06DD86, i_06dd86),
    (0x06DD8C, i_06dd8c),
    (0x06DD90, i_06dd90),
    (0x06DD96, i_06dd96),
    (0x06DD98, i_06dd98),
    (0x06DD9C, i_06dd9c),
    (0x06DDA2, i_06dda2),
    (0x06DDA6, i_06dda6),
    (0x06DDAC, i_06ddac),
    (0x06DDB0, i_06ddb0),
    (0x06DDB6, i_06ddb6),
    (0x06DDBC, i_06ddbc),
    (0x06DDC2, i_06ddc2),
    (0x06DDC8, i_06ddc8),
    (0x06DDCC, i_06ddcc),
    (0x06DDD0, i_06ddd0),
    (0x06DDD4, i_06ddd4),
    (0x06DDDA, i_06ddda),
    (0x06DDE0, i_06dde0),
    (0x06DDE4, i_06dde4),
    (0x06DDEA, i_06ddea),
    (0x06DDEC, i_06ddec),
    (0x06DDF0, i_06ddf0),
    (0x06DDF4, i_06ddf4),
    (0x06DDF6, i_06ddf6),
    (0x06DDFC, i_06ddfc),
    (0x06DE00, i_06de00),
    (0x06DE06, i_06de06),
    (0x06DE0A, i_06de0a),
    (0x06DE0E, i_06de0e),
    (0x06DE10, i_06de10),
    (0x06DE12, i_06de12),
    (0x06DE14, i_06de14),
    (0x06DE18, i_06de18),
    (0x06DE1A, i_06de1a),
    (0x06DE1C, i_06de1c),
    (0x06DE1E, i_06de1e),
    (0x06DE20, i_06de20),
    (0x06DE24, i_06de24),
    (0x06DE28, i_06de28),
    (0x06DE2E, i_06de2e),
    (0x06DE32, i_06de32),
    (0x06DE36, i_06de36),
    (0x06DE3C, i_06de3c),
    (0x06DE40, i_06de40),
    (0x06DE42, i_06de42),
    (0x06DE46, i_06de46),
    (0x06DE48, i_06de48),
    (0x06DE4A, i_06de4a),
    (0x06DE4E, i_06de4e),
    (0x06DE54, i_06de54),
    (0x06DE58, i_06de58),
    (0x06DE5A, i_06de5a),
    (0x06DE5E, i_06de5e),
    (0x06DE60, i_06de60),
    (0x06DE62, i_06de62),
    (0x06DE66, i_06de66),
    (0x06DE6A, i_06de6a),
    (0x06DE6E, i_06de6e),
    (0x06DE74, i_06de74),
    (0x06DE78, i_06de78),
    (0x06DE7A, i_06de7a),
    (0x06DE80, i_06de80),
    (0x06DE84, i_06de84),
    (0x06DE8A, i_06de8a),
    (0x06DE8E, i_06de8e),
    (0x06DE90, i_06de90),
    (0x06DE94, i_06de94),
    (0x06DE98, i_06de98),
    (0x06DE9A, i_06de9a),
    (0x06DE9E, i_06de9e),
    (0x06DEA2, i_06dea2),
    (0x06DEA6, i_06dea6),
    (0x06DEAC, i_06deac),
    (0x06DEB0, i_06deb0),
    (0x06DEB4, i_06deb4),
    (0x06DEB6, i_06deb6),
    (0x06DEBA, i_06deba),
    (0x06DEBE, i_06debe),
    (0x06DEC2, i_06dec2),
    (0x06DEC6, i_06dec6),
    (0x06DECC, i_06decc),
    (0x06DED0, i_06ded0),
    (0x06DED6, i_06ded6),
    (0x06DEDA, i_06deda),
    (0x06DEDE, i_06dede),
    (0x06DEE2, i_06dee2),
    (0x06DEE8, i_06dee8),
    (0x06DEEE, i_06deee),
    (0x06DEF4, i_06def4),
    (0x06DEFA, i_06defa),
    (0x06DEFE, i_06defe),
    (0x06DF02, i_06df02),
    (0x06DF04, i_06df04),
    (0x06DF06, i_06df06),
    (0x06DF0C, i_06df0c),
    (0x06DF10, i_06df10),
    (0x06DF14, i_06df14),
    (0x06DF18, i_06df18),
    (0x06DF1C, i_06df1c),
    (0x06DF22, i_06df22),
    (0x06DF26, i_06df26),
    (0x06DF2A, i_06df2a),
    (0x06DF2E, i_06df2e),
    (0x06DF32, i_06df32),
    (0x06DF38, i_06df38),
    (0x06DF3C, i_06df3c),
    (0x06DF40, i_06df40),
    (0x06DF44, i_06df44),
    (0x06DF48, i_06df48),
    (0x06DF4E, i_06df4e),
    (0x06DF52, i_06df52),
    (0x06DF56, i_06df56),
    (0x06DF5A, i_06df5a),
    (0x06DF60, i_06df60),
    (0x06DF64, i_06df64),
    (0x06DF6A, i_06df6a),
    (0x06DF70, i_06df70),
    (0x06DF76, i_06df76),
    (0x06DF7E, i_06df7e),
    (0x06DF86, i_06df86),
    (0x06DF8A, i_06df8a),
    (0x06DF90, i_06df90),
    (0x06DF94, i_06df94),
    (0x06DF9A, i_06df9a),
    (0x06DF9E, i_06df9e),
    (0x06DFA4, i_06dfa4),
    (0x06DFAA, i_06dfaa),
    (0x06DFB0, i_06dfb0),
    (0x06DFB4, i_06dfb4),
    (0x06DFB6, i_06dfb6),
    (0x06DFBC, i_06dfbc),
    (0x06DFC0, i_06dfc0),
    (0x06DFC6, i_06dfc6),
    (0x06DFC8, i_06dfc8),
    (0x06DFCE, i_06dfce),
    (0x06DFD2, i_06dfd2),
    (0x06DFD8, i_06dfd8),
    (0x06DFDC, i_06dfdc),
    (0x06DFE2, i_06dfe2),
    (0x06DFE4, i_06dfe4),
    (0x06DFE8, i_06dfe8),
    (0x06DFEE, i_06dfee),
    (0x06DFF2, i_06dff2),
    (0x06DFF8, i_06dff8),
    (0x06DFFE, i_06dffe),
    (0x06E006, i_06e006),
    (0x06E00A, i_06e00a),
    (0x06E010, i_06e010),
    (0x06E016, i_06e016),
    (0x06E01A, i_06e01a),
    (0x06E020, i_06e020),
    (0x06E026, i_06e026),
    (0x06E028, i_06e028),
    (0x06E02E, i_06e02e),
    (0x06E032, i_06e032),
    (0x06E036, i_06e036),
    (0x06E03A, i_06e03a),
    (0x06E03E, i_06e03e),
    (0x06E044, i_06e044),
    (0x06E048, i_06e048),
    (0x06E04E, i_06e04e),
    (0x06E054, i_06e054),
    (0x06E05A, i_06e05a),
    (0x06E05E, i_06e05e),
    (0x06E060, i_06e060),
    (0x06E062, i_06e062),
    (0x06E068, i_06e068),
    (0x06E06E, i_06e06e),
    (0x06E072, i_06e072),
    (0x06E078, i_06e078),
    (0x06E07C, i_06e07c),
    (0x06E082, i_06e082),
    (0x06E084, i_06e084),
    (0x06E08A, i_06e08a),
    (0x06E08E, i_06e08e),
    (0x06E094, i_06e094),
    (0x06E096, i_06e096),
    (0x06E09A, i_06e09a),
    (0x06E0A0, i_06e0a0),
    (0x06E0A4, i_06e0a4),
    (0x06E0AA, i_06e0aa),
    (0x06E0B2, i_06e0b2),
    (0x06E0B6, i_06e0b6),
    (0x06E0BC, i_06e0bc),
    (0x06E0C2, i_06e0c2),
    (0x06E0C6, i_06e0c6),
    (0x06E0C8, i_06e0c8),
    (0x06E0CA, i_06e0ca),
    (0x06E0CC, i_06e0cc),
    (0x06E0CE, i_06e0ce),
    (0x06E0D0, i_06e0d0),
    (0x06E0D6, i_06e0d6),
    (0x06E0D8, i_06e0d8),
    (0x06E0DE, i_06e0de),
    (0x06E0E4, i_06e0e4),
    (0x06E0E6, i_06e0e6),
    (0x06E0EA, i_06e0ea),
    (0x06E0EE, i_06e0ee),
    (0x06E0F2, i_06e0f2),
    (0x06E0F8, i_06e0f8),
    (0x06E0FC, i_06e0fc),
    (0x06E102, i_06e102),
    (0x06E108, i_06e108),
    (0x06E10E, i_06e10e),
    (0x06E112, i_06e112),
    (0x06E114, i_06e114),
    (0x06E11A, i_06e11a),
    (0x06E11E, i_06e11e),
    (0x06E124, i_06e124),
    (0x06E126, i_06e126),
    (0x06E12C, i_06e12c),
    (0x06E130, i_06e130),
    (0x06E136, i_06e136),
    (0x06E13A, i_06e13a),
    (0x06E13E, i_06e13e),
    (0x06E140, i_06e140),
    (0x06E146, i_06e146),
    (0x06E14C, i_06e14c),
    (0x06E152, i_06e152),
    (0x06E156, i_06e156),
    (0x06E15A, i_06e15a),
    (0x06E15C, i_06e15c),
    (0x06E15E, i_06e15e),
    (0x06E164, i_06e164),
    (0x06E16A, i_06e16a),
    (0x06E16E, i_06e16e),
    (0x06E172, i_06e172),
    (0x06E174, i_06e174),
    (0x06E176, i_06e176),
    (0x06E17C, i_06e17c),
    (0x06E182, i_06e182),
    (0x06E186, i_06e186),
    (0x06E18C, i_06e18c),
    (0x06E190, i_06e190),
    (0x06E194, i_06e194),
    (0x06E196, i_06e196),
    (0x06E19C, i_06e19c),
    (0x06E1A2, i_06e1a2),
    (0x06E1A8, i_06e1a8),
    (0x06E1AE, i_06e1ae),
    (0x06E1B2, i_06e1b2),
    (0x06E1B8, i_06e1b8),
    (0x06E1BC, i_06e1bc),
    (0x06E1C0, i_06e1c0),
    (0x06E1C4, i_06e1c4),
    (0x06E1C8, i_06e1c8),
    (0x06E1CC, i_06e1cc),
    (0x06E1D0, i_06e1d0),
    (0x06E1D2, i_06e1d2),
    (0x06E1D6, i_06e1d6),
    (0x06E1DA, i_06e1da),
    (0x06E1DE, i_06e1de),
    (0x06E1E0, i_06e1e0),
    (0x06E1E2, i_06e1e2),
    (0x06E1E4, i_06e1e4),
    (0x06E1E6, i_06e1e6),
    (0x06E1EA, i_06e1ea),
    (0x06E1EC, i_06e1ec),
    (0x06E1F0, i_06e1f0),
    (0x06E1F6, i_06e1f6),
    (0x06E1FC, i_06e1fc),
    (0x06E202, i_06e202),
    (0x06E206, i_06e206),
    (0x06E20A, i_06e20a),
    (0x06E20C, i_06e20c),
    (0x06E212, i_06e212),
    (0x06E216, i_06e216),
    (0x06E218, i_06e218),
    (0x06E21C, i_06e21c),
    (0x06E21E, i_06e21e),
    (0x06E222, i_06e222),
    (0x06E224, i_06e224),
    (0x06E228, i_06e228),
    (0x06E22C, i_06e22c),
    (0x06E232, i_06e232),
    (0x06E238, i_06e238),
    (0x06E23C, i_06e23c),
    (0x06E240, i_06e240),
    (0x06E244, i_06e244),
    (0x06E248, i_06e248),
    (0x06E24A, i_06e24a),
    (0x06E24E, i_06e24e),
    (0x06E252, i_06e252),
    (0x06E258, i_06e258),
    (0x06E25E, i_06e25e),
    (0x06E262, i_06e262),
    (0x06E266, i_06e266),
    (0x06E26A, i_06e26a),
    (0x06E26E, i_06e26e),
    (0x06E270, i_06e270),
    (0x06E274, i_06e274),
    (0x06E278, i_06e278),
    (0x06E27E, i_06e27e),
    (0x06E284, i_06e284),
    (0x06E288, i_06e288),
    (0x06E28C, i_06e28c),
    (0x06E290, i_06e290),
    (0x06E294, i_06e294),
    (0x06E296, i_06e296),
    (0x06E29A, i_06e29a),
    (0x06E29E, i_06e29e),
    (0x06E2A4, i_06e2a4),
    (0x06E2AA, i_06e2aa),
    (0x06E2AE, i_06e2ae),
    (0x06E2B2, i_06e2b2),
    (0x06E2B6, i_06e2b6),
    (0x06E2BA, i_06e2ba),
    (0x06E2BC, i_06e2bc),
    (0x06E2C0, i_06e2c0),
    (0x06E2C6, i_06e2c6),
    (0x06E2CC, i_06e2cc),
    (0x06E2D2, i_06e2d2),
    (0x06E2D8, i_06e2d8),
    (0x06E2DE, i_06e2de),
    (0x06E2E4, i_06e2e4),
    (0x06E2EA, i_06e2ea),
    (0x06E2F0, i_06e2f0),
    (0x06E2F6, i_06e2f6),
    (0x06E2FC, i_06e2fc),
    (0x06E2FE, i_06e2fe),
    (0x06E302, i_06e302),
    (0x06E308, i_06e308),
    (0x06E30C, i_06e30c),
    (0x06E312, i_06e312),
    (0x06E316, i_06e316),
    (0x06E318, i_06e318),
    (0x06E31C, i_06e31c),
    (0x06E31E, i_06e31e),
    (0x06E324, i_06e324),
    (0x06E328, i_06e328),
    (0x06E32C, i_06e32c),
    (0x06E32E, i_06e32e),
    (0x06E332, i_06e332),
    (0x06E338, i_06e338),
    (0x06E33C, i_06e33c),
    (0x06E342, i_06e342),
    (0x06E348, i_06e348),
    (0x06E34A, i_06e34a),
    (0x06E34E, i_06e34e),
    (0x06E354, i_06e354),
    (0x06E356, i_06e356),
    (0x06E35A, i_06e35a),
    (0x06E360, i_06e360),
    (0x06E364, i_06e364),
    (0x06E36A, i_06e36a),
    (0x06E36E, i_06e36e),
    (0x06E374, i_06e374),
    (0x06E37A, i_06e37a),
    (0x06E37C, i_06e37c),
    (0x06E380, i_06e380),
    (0x06E384, i_06e384),
    (0x06E388, i_06e388),
    (0x06E38C, i_06e38c),
    (0x06E392, i_06e392),
    (0x06E394, i_06e394),
    (0x06E398, i_06e398),
    (0x06E39C, i_06e39c),
    (0x06E3A2, i_06e3a2),
    (0x06E3A4, i_06e3a4),
    (0x06E3A8, i_06e3a8),
    (0x06E3AE, i_06e3ae),
    (0x06E3B2, i_06e3b2),
    (0x06E3B8, i_06e3b8),
    (0x06E3BE, i_06e3be),
    (0x06E3C4, i_06e3c4),
    (0x06E3C8, i_06e3c8),
    (0x06E3CC, i_06e3cc),
    (0x06E3D0, i_06e3d0),
    (0x06E3D6, i_06e3d6),
    (0x06E3DA, i_06e3da),
    (0x06E3E0, i_06e3e0),
    (0x06E3E6, i_06e3e6),
    (0x06E3EC, i_06e3ec),
    (0x06E3F2, i_06e3f2),
    (0x06E3F8, i_06e3f8),
    (0x06E3FE, i_06e3fe),
    (0x06E402, i_06e402),
    (0x06E406, i_06e406),
    (0x06E40A, i_06e40a),
    (0x06E410, i_06e410),
    (0x06E412, i_06e412),
    (0x06E416, i_06e416),
    (0x06E418, i_06e418),
    (0x06E41C, i_06e41c),
    (0x06E422, i_06e422),
    (0x06E428, i_06e428),
    (0x06E42E, i_06e42e),
    (0x06E430, i_06e430),
    (0x06E432, i_06e432),
    (0x06E434, i_06e434),
    (0x06E436, i_06e436),
    (0x06E43C, i_06e43c),
    (0x06E442, i_06e442),
    (0x06E448, i_06e448),
    (0x06E44E, i_06e44e),
    (0x06E454, i_06e454),
    (0x06E456, i_06e456),
    (0x06E45C, i_06e45c),
    (0x06E462, i_06e462),
    (0x06E468, i_06e468),
    (0x06E46E, i_06e46e),
    (0x06E470, i_06e470),
    (0x06E476, i_06e476),
    (0x06E47C, i_06e47c),
    (0x06E482, i_06e482),
    (0x06E484, i_06e484),
    (0x06E488, i_06e488),
    (0x06E48C, i_06e48c),
    (0x06E492, i_06e492),
    (0x06E496, i_06e496),
    (0x06E49A, i_06e49a),
    (0x06E4A0, i_06e4a0),
    (0x06E4A6, i_06e4a6),
    (0x06E4A8, i_06e4a8),
    (0x06E4AC, i_06e4ac),
    (0x06E4AE, i_06e4ae),
    (0x06E4B2, i_06e4b2),
    (0x06E4B8, i_06e4b8),
    (0x06E4BE, i_06e4be),
    (0x06E4C4, i_06e4c4),
    (0x06E4C6, i_06e4c6),
    (0x06E4C8, i_06e4c8),
    (0x06E4CA, i_06e4ca),
    (0x06E4CC, i_06e4cc),
    (0x06E4D0, i_06e4d0),
    (0x06E4D6, i_06e4d6),
    (0x06E4DC, i_06e4dc),
    (0x06E4E2, i_06e4e2),
    (0x06E4E6, i_06e4e6),
    (0x06E4EA, i_06e4ea),
    (0x06E4EC, i_06e4ec),
    (0x06E4F0, i_06e4f0),
    (0x06E4F4, i_06e4f4),
    (0x06E4F8, i_06e4f8),
    (0x06E4FA, i_06e4fa),
    (0x06E4FC, i_06e4fc),
    (0x06E500, i_06e500),
    (0x06E502, i_06e502),
    (0x06E506, i_06e506),
    (0x06E50C, i_06e50c),
    (0x06E50E, i_06e50e),
    (0x06E512, i_06e512),
    (0x06E516, i_06e516),
    (0x06E51A, i_06e51a),
    (0x06E51E, i_06e51e),
    (0x06E522, i_06e522),
    (0x06E524, i_06e524),
    (0x06E528, i_06e528),
    (0x06E52A, i_06e52a),
    (0x06E52E, i_06e52e),
    (0x06E534, i_06e534),
    (0x06E53A, i_06e53a),
    (0x06E540, i_06e540),
    (0x06E546, i_06e546),
    (0x06E54A, i_06e54a),
    (0x06E54E, i_06e54e),
    (0x06E550, i_06e550),
    (0x06E554, i_06e554),
    (0x06E55A, i_06e55a),
    (0x06E55E, i_06e55e),
    (0x06E564, i_06e564),
    (0x06E56A, i_06e56a),
    (0x06E56E, i_06e56e),
    (0x06E572, i_06e572),
    (0x06E578, i_06e578),
    (0x06E57E, i_06e57e),
    (0x06E584, i_06e584),
    (0x06E58C, i_06e58c),
    (0x06E590, i_06e590),
    (0x06E594, i_06e594),
    (0x06E59A, i_06e59a),
    (0x06E5A0, i_06e5a0),
    (0x06E5A6, i_06e5a6),
    (0x06E5AC, i_06e5ac),
    (0x06E5B0, i_06e5b0),
    (0x06E5B6, i_06e5b6),
    (0x06E5BC, i_06e5bc),
    (0x06E5C0, i_06e5c0),
    (0x06E5C6, i_06e5c6),
    (0x06E5CC, i_06e5cc),
    (0x06E5D2, i_06e5d2),
    (0x06E5D8, i_06e5d8),
    (0x06E5DE, i_06e5de),
    (0x06E5E4, i_06e5e4),
    (0x06E5E8, i_06e5e8),
    (0x06E5EA, i_06e5ea),
    (0x06E5EE, i_06e5ee),
    (0x06E5F2, i_06e5f2),
    (0x06E5F8, i_06e5f8),
    (0x06E5FC, i_06e5fc),
    (0x06E602, i_06e602),
    (0x06E606, i_06e606),
    (0x06E60A, i_06e60a),
    (0x06E60C, i_06e60c),
    (0x06E610, i_06e610),
    (0x06E616, i_06e616),
    (0x06E61C, i_06e61c),
    (0x06E620, i_06e620),
    (0x06E626, i_06e626),
    (0x06E62C, i_06e62c),
    (0x06E632, i_06e632),
    (0x06E638, i_06e638),
    (0x06E63E, i_06e63e),
    (0x06E644, i_06e644),
    (0x06E648, i_06e648),
    (0x06E64C, i_06e64c),
    (0x06E64E, i_06e64e),
    (0x06E654, i_06e654),
    (0x06E65A, i_06e65a),
    (0x06E660, i_06e660),
    (0x06E664, i_06e664),
    (0x06E668, i_06e668),
    (0x06E66A, i_06e66a),
    (0x06E66C, i_06e66c),
    (0x06E670, i_06e670),
    (0x06E674, i_06e674),
    (0x06E678, i_06e678),
    (0x06E67E, i_06e67e),
    (0x06E682, i_06e682),
    (0x06E686, i_06e686),
    (0x06E688, i_06e688),
    (0x06E68C, i_06e68c),
    (0x06E690, i_06e690),
    (0x06E696, i_06e696),
    (0x06E69A, i_06e69a),
    (0x06E6A0, i_06e6a0),
    (0x06E6A6, i_06e6a6),
    (0x06E6AC, i_06e6ac),
    (0x06E6B4, i_06e6b4),
    (0x06E6BA, i_06e6ba),
    (0x06E6BE, i_06e6be),
    (0x06E6C4, i_06e6c4),
    (0x06E6C8, i_06e6c8),
    (0x06E6CC, i_06e6cc),
    (0x06E6D0, i_06e6d0),
    (0x06E6D4, i_06e6d4),
    (0x06E6DA, i_06e6da),
    (0x06E6DE, i_06e6de),
    (0x06E6E0, i_06e6e0),
    (0x06E6E4, i_06e6e4),
    (0x06E6EA, i_06e6ea),
    (0x06E6F0, i_06e6f0),
    (0x06E6F4, i_06e6f4),
    (0x06E6F6, i_06e6f6),
    (0x06E6FC, i_06e6fc),
    (0x06E700, i_06e700),
    (0x06E704, i_06e704),
    (0x06E706, i_06e706),
    (0x06E70A, i_06e70a),
    (0x06E710, i_06e710),
    (0x06E716, i_06e716),
    (0x06E71C, i_06e71c),
    (0x06E720, i_06e720),
    (0x06E724, i_06e724),
    (0x06E72A, i_06e72a),
    (0x06E72E, i_06e72e),
    (0x06E732, i_06e732),
    (0x06E734, i_06e734),
    (0x06E73A, i_06e73a),
    (0x06E740, i_06e740),
    (0x06E746, i_06e746),
    (0x06E74A, i_06e74a),
    (0x06E74E, i_06e74e),
    (0x06E750, i_06e750),
    (0x06E752, i_06e752),
    (0x06E758, i_06e758),
    (0x06E75E, i_06e75e),
    (0x06E764, i_06e764),
    (0x06E76A, i_06e76a),
    (0x06E770, i_06e770),
    (0x06E774, i_06e774),
    (0x06E778, i_06e778),
    (0x06E77C, i_06e77c),
    (0x06E780, i_06e780),
    (0x06E786, i_06e786),
    (0x06E78A, i_06e78a),
    (0x06E790, i_06e790),
    (0x06E796, i_06e796),
    (0x06E79C, i_06e79c),
    (0x06E7A0, i_06e7a0),
    (0x06E7A6, i_06e7a6),
    (0x06E7AC, i_06e7ac),
    (0x06E7B2, i_06e7b2),
    (0x06E7B8, i_06e7b8),
    (0x06E7BE, i_06e7be),
    (0x06E7C4, i_06e7c4),
    (0x06E7C8, i_06e7c8),
    (0x06E7CE, i_06e7ce),
    (0x06E7D4, i_06e7d4),
    (0x06E7D8, i_06e7d8),
    (0x06E7DE, i_06e7de),
    (0x06E7E4, i_06e7e4),
    (0x06E7EA, i_06e7ea),
    (0x06E7EE, i_06e7ee),
    (0x06E7F4, i_06e7f4),
    (0x06E7FA, i_06e7fa),
    (0x06E7FE, i_06e7fe),
    (0x06E804, i_06e804),
    (0x06E80A, i_06e80a),
    (0x06E810, i_06e810),
    (0x06E816, i_06e816),
    (0x06E818, i_06e818),
    (0x06E81E, i_06e81e),
    (0x06E820, i_06e820),
    (0x06E824, i_06e824),
    (0x06E82A, i_06e82a),
    (0x06E82E, i_06e82e),
    (0x06E832, i_06e832),
    (0x06E838, i_06e838),
    (0x06E83C, i_06e83c),
    (0x06E842, i_06e842),
    (0x06E848, i_06e848),
    (0x06E84E, i_06e84e),
    (0x06E854, i_06e854),
    (0x06E85A, i_06e85a),
    (0x06E860, i_06e860),
    (0x06E866, i_06e866),
    (0x06E86C, i_06e86c),
    (0x06E870, i_06e870),
    (0x06E874, i_06e874),
    (0x06E876, i_06e876),
    (0x06E87A, i_06e87a),
    (0x06E880, i_06e880),
    (0x06E884, i_06e884),
    (0x06E888, i_06e888),
    (0x06E88A, i_06e88a),
    (0x06E88C, i_06e88c),
    (0x06E890, i_06e890),
    (0x06E896, i_06e896),
    (0x06E89C, i_06e89c),
    (0x06E8A2, i_06e8a2),
    (0x06E8A6, i_06e8a6),
    (0x06E8A8, i_06e8a8),
    (0x06E8AC, i_06e8ac),
    (0x06E8B2, i_06e8b2),
    (0x06E8B8, i_06e8b8),
    (0x06E8BC, i_06e8bc),
    (0x06E8C0, i_06e8c0),
    (0x06E8C2, i_06e8c2),
    (0x06E8C6, i_06e8c6),
    (0x06E8CA, i_06e8ca),
    (0x06E8CE, i_06e8ce),
    (0x06E8D0, i_06e8d0),
    (0x06E8D4, i_06e8d4),
    (0x06E8D8, i_06e8d8),
    (0x06E8DC, i_06e8dc),
    (0x06E8E0, i_06e8e0),
    (0x06E8E6, i_06e8e6),
    (0x06E8EA, i_06e8ea),
    (0x06E8F0, i_06e8f0),
    (0x06E8F6, i_06e8f6),
    (0x06E8FC, i_06e8fc),
    (0x06E900, i_06e900),
    (0x06E904, i_06e904),
    (0x06E90A, i_06e90a),
    (0x06E90E, i_06e90e),
    (0x06E912, i_06e912),
    (0x06E916, i_06e916),
    (0x06E91A, i_06e91a),
    (0x06E91E, i_06e91e),
    (0x06E920, i_06e920),
    (0x06E926, i_06e926),
    (0x06E92A, i_06e92a),
    (0x06E92E, i_06e92e),
    (0x06E930, i_06e930),
    (0x06E932, i_06e932),
    (0x06E936, i_06e936),
    (0x06E93C, i_06e93c),
    (0x06E942, i_06e942),
    (0x06E946, i_06e946),
    (0x06E948, i_06e948),
    (0x06E94E, i_06e94e),
    (0x06E954, i_06e954),
    (0x06E95A, i_06e95a),
    (0x06E95E, i_06e95e),
    (0x06E962, i_06e962),
    (0x06E964, i_06e964),
    (0x06E966, i_06e966),
    (0x06E96A, i_06e96a),
    (0x06E970, i_06e970),
    (0x06E976, i_06e976),
    (0x06E97C, i_06e97c),
    (0x06E982, i_06e982),
    (0x06E988, i_06e988),
    (0x06E98C, i_06e98c),
    (0x06E98E, i_06e98e),
    (0x06E994, i_06e994),
    (0x06E99A, i_06e99a),
    (0x06E99E, i_06e99e),
    (0x06E9A4, i_06e9a4),
    (0x06E9A8, i_06e9a8),
    (0x06E9AE, i_06e9ae),
    (0x06E9B2, i_06e9b2),
    (0x06E9B6, i_06e9b6),
    (0x06E9B8, i_06e9b8),
    (0x06E9BE, i_06e9be),
    (0x06E9C0, i_06e9c0),
    (0x06E9C6, i_06e9c6),
    (0x06E9CA, i_06e9ca),
    (0x06E9CE, i_06e9ce),
    (0x06E9D2, i_06e9d2),
    (0x06E9D8, i_06e9d8),
    (0x06E9DC, i_06e9dc),
    (0x06E9E0, i_06e9e0),
    (0x06E9E4, i_06e9e4),
    (0x06E9EA, i_06e9ea),
    (0x06E9F0, i_06e9f0),
    (0x06E9F6, i_06e9f6),
    (0x06E9FC, i_06e9fc),
    (0x06EA02, i_06ea02),
    (0x06EA06, i_06ea06),
    (0x06EA0A, i_06ea0a),
    (0x06EA10, i_06ea10),
    (0x06EA16, i_06ea16),
    (0x06EA1C, i_06ea1c),
    (0x06EA24, i_06ea24),
    (0x06EA2A, i_06ea2a),
    (0x06EA30, i_06ea30),
    (0x06EA34, i_06ea34),
    (0x06EA3A, i_06ea3a),
    (0x06EA40, i_06ea40),
    (0x06EA44, i_06ea44),
    (0x06EA46, i_06ea46),
    (0x06EA4C, i_06ea4c),
    (0x06EA50, i_06ea50),
    (0x06EA56, i_06ea56),
    (0x06EA5A, i_06ea5a),
    (0x06EA5E, i_06ea5e),
    (0x06EA62, i_06ea62),
    (0x06EA64, i_06ea64),
    (0x06EA68, i_06ea68),
    (0x06EA6E, i_06ea6e),
    (0x06EA72, i_06ea72),
    (0x06EA76, i_06ea76),
    (0x06EA78, i_06ea78),
    (0x06EA7E, i_06ea7e),
    (0x06EA84, i_06ea84),
    (0x06EA8A, i_06ea8a),
    (0x06EA8E, i_06ea8e),
    (0x06EA92, i_06ea92),
    (0x06EA94, i_06ea94),
    (0x06EA96, i_06ea96),
    (0x06EA9C, i_06ea9c),
    (0x06EAA2, i_06eaa2),
    (0x06EAA6, i_06eaa6),
    (0x06EAA8, i_06eaa8),
    (0x06EAAE, i_06eaae),
    (0x06EAB2, i_06eab2),
    (0x06EAB8, i_06eab8),
    (0x06EABC, i_06eabc),
    (0x06EAC0, i_06eac0),
    (0x06EAC2, i_06eac2),
    (0x06EAC6, i_06eac6),
    (0x06EACA, i_06eaca),
    (0x06EACE, i_06eace),
    (0x06EAD0, i_06ead0),
    (0x06EAD2, i_06ead2),
    (0x06EAD6, i_06ead6),
    (0x06EADC, i_06eadc),
    (0x06EAE2, i_06eae2),
    (0x06EAE8, i_06eae8),
    (0x06EAEE, i_06eaee),
    (0x06EAF4, i_06eaf4),
    (0x06EAFA, i_06eafa),
    (0x06EAFE, i_06eafe),
    (0x06EB00, i_06eb00),
    (0x06EB06, i_06eb06),
    (0x06EB0A, i_06eb0a),
    (0x06EB0E, i_06eb0e),
    (0x06EB12, i_06eb12),
    (0x06EB14, i_06eb14),
    (0x06EB1A, i_06eb1a),
    (0x06EB20, i_06eb20),
    (0x06EB26, i_06eb26),
    (0x06EB2A, i_06eb2a),
    (0x06EB2E, i_06eb2e),
    (0x06EB30, i_06eb30),
    (0x06EB36, i_06eb36),
    (0x06EB3A, i_06eb3a),
    (0x06EB40, i_06eb40),
    (0x06EB46, i_06eb46),
    (0x06EB4C, i_06eb4c),
    (0x06EB52, i_06eb52),
    (0x06EB56, i_06eb56),
    (0x06EB5A, i_06eb5a),
    (0x06EB60, i_06eb60),
    (0x06EB64, i_06eb64),
    (0x06EB66, i_06eb66),
    (0x06EB6C, i_06eb6c),
    (0x06EB72, i_06eb72),
    (0x06EB78, i_06eb78),
    (0x06EB7C, i_06eb7c),
    (0x06EB80, i_06eb80),
    (0x06EB82, i_06eb82),
    (0x06EBD0, i_06ebd0),
    (0x06EBD8, i_06ebd8),
    (0x06EBDC, i_06ebdc),
    (0x06EBDE, i_06ebde),
    (0x06EBE0, i_06ebe0),
    (0x06EBE4, i_06ebe4),
    (0x06EBEA, i_06ebea),
    (0x06EBF0, i_06ebf0),
    (0x06EBF4, i_06ebf4),
    (0x06EBF6, i_06ebf6),
    (0x06EBFA, i_06ebfa),
    (0x06EBFE, i_06ebfe),
    (0x06EC04, i_06ec04),
    (0x06EC0A, i_06ec0a),
    (0x06EC10, i_06ec10),
    (0x06EC14, i_06ec14),
    (0x06EC18, i_06ec18),
    (0x06EC1C, i_06ec1c),
    (0x06EC1E, i_06ec1e),
    (0x06EC22, i_06ec22),
    (0x06EC28, i_06ec28),
    (0x06EC2C, i_06ec2c),
    (0x06EC30, i_06ec30),
    (0x06EC34, i_06ec34),
    (0x06EC38, i_06ec38),
    (0x06EC3A, i_06ec3a),
    (0x06EC40, i_06ec40),
    (0x06EC46, i_06ec46),
    (0x06EC4A, i_06ec4a),
    (0x06EC50, i_06ec50),
    (0x06EC54, i_06ec54),
    (0x06EC58, i_06ec58),
    (0x06EC5E, i_06ec5e),
    (0x06EC64, i_06ec64),
    (0x06EC6A, i_06ec6a),
    (0x06EC70, i_06ec70),
    (0x06EC78, i_06ec78),
    (0x06EC80, i_06ec80),
    (0x06EC84, i_06ec84),
    (0x06EC88, i_06ec88),
    (0x06EC8C, i_06ec8c),
    (0x06EC92, i_06ec92),
    (0x06EC98, i_06ec98),
    (0x06EC9C, i_06ec9c),
    (0x06EC9E, i_06ec9e),
    (0x06ECA4, i_06eca4),
    (0x06ECAA, i_06ecaa),
    (0x06ECB0, i_06ecb0),
    (0x06ECB6, i_06ecb6),
    (0x06ECBA, i_06ecba),
    (0x06ECC0, i_06ecc0),
    (0x06ECC6, i_06ecc6),
    (0x06ECCC, i_06eccc),
    (0x06ECD2, i_06ecd2),
    (0x06ECD8, i_06ecd8),
    (0x06ECDC, i_06ecdc),
    (0x06ECDE, i_06ecde),
    (0x06ECE2, i_06ece2),
    (0x06ECE6, i_06ece6),
    (0x06ECEC, i_06ecec),
    (0x06ECF0, i_06ecf0),
    (0x06ECF4, i_06ecf4),
    (0x06ECFA, i_06ecfa),
    (0x06ED00, i_06ed00),
    (0x06ED04, i_06ed04),
    (0x06ED08, i_06ed08),
    (0x06ED0A, i_06ed0a),
    (0x06ED10, i_06ed10),
    (0x06ED14, i_06ed14),
    (0x06ED18, i_06ed18),
    (0x06ED1E, i_06ed1e),
    (0x06ED22, i_06ed22),
    (0x06ED24, i_06ed24),
    (0x06ED2A, i_06ed2a),
    (0x06ED30, i_06ed30),
    (0x06ED36, i_06ed36),
    (0x06ED3A, i_06ed3a),
    (0x06ED3E, i_06ed3e),
    (0x06ED40, i_06ed40),
    (0x06ED42, i_06ed42),
    (0x06ED46, i_06ed46),
    (0x06ED4A, i_06ed4a),
    (0x06ED52, i_06ed52),
    (0x06ED5A, i_06ed5a),
    (0x06ED5E, i_06ed5e),
    (0x06ED62, i_06ed62),
    (0x06ED66, i_06ed66),
    (0x06ED6A, i_06ed6a),
    (0x06ED6E, i_06ed6e),
    (0x06ED74, i_06ed74),
    (0x06ED7A, i_06ed7a),
    (0x06ED7E, i_06ed7e),
    (0x06ED80, i_06ed80),
    (0x06ED84, i_06ed84),
    (0x06ED88, i_06ed88),
    (0x06ED8E, i_06ed8e),
    (0x06ED94, i_06ed94),
    (0x06ED98, i_06ed98),
    (0x06ED9C, i_06ed9c),
    (0x06ED9E, i_06ed9e),
    (0x06EDA2, i_06eda2),
    (0x06EDA6, i_06eda6),
    (0x06EDAA, i_06edaa),
    (0x06EDB0, i_06edb0),
    (0x06EDB4, i_06edb4),
    (0x06EDB8, i_06edb8),
    (0x06EDBC, i_06edbc),
    (0x06EDC0, i_06edc0),
    (0x06EDC4, i_06edc4),
    (0x06EDC8, i_06edc8),
    (0x06EDCA, i_06edca),
    (0x06EDCE, i_06edce),
    (0x06EDD2, i_06edd2),
    (0x06EDD8, i_06edd8),
    (0x06EDDE, i_06edde),
    (0x06EDE2, i_06ede2),
    (0x06EDE6, i_06ede6),
    (0x06EDEA, i_06edea),
    (0x06EDF0, i_06edf0),
    (0x06EDF4, i_06edf4),
    (0x06EDF8, i_06edf8),
    (0x06EDFC, i_06edfc),
    (0x06EE02, i_06ee02),
    (0x06EE08, i_06ee08),
    (0x06EE0C, i_06ee0c),
    (0x06EE12, i_06ee12),
    (0x06EE18, i_06ee18),
    (0x06EE1C, i_06ee1c),
    (0x06EE22, i_06ee22),
    (0x06EE28, i_06ee28),
    (0x06EE2C, i_06ee2c),
    (0x06EE32, i_06ee32),
    (0x06EE38, i_06ee38),
    (0x06EE3E, i_06ee3e),
    (0x06EE42, i_06ee42),
    (0x06EE48, i_06ee48),
    (0x06EE4E, i_06ee4e),
    (0x06EE52, i_06ee52),
    (0x06EE58, i_06ee58),
    (0x06EE5E, i_06ee5e),
    (0x06EE64, i_06ee64),
    (0x06EE68, i_06ee68),
    (0x06EE6A, i_06ee6a),
    (0x06EE70, i_06ee70),
    (0x06EE76, i_06ee76),
    (0x06EE7A, i_06ee7a),
    (0x06EE7E, i_06ee7e),
    (0x06EE80, i_06ee80),
    (0x06EE82, i_06ee82),
    (0x06EE86, i_06ee86),
    (0x06EE8A, i_06ee8a),
    (0x06EE8E, i_06ee8e),
    (0x06EE94, i_06ee94),
    (0x06EE9A, i_06ee9a),
    (0x06EEA0, i_06eea0),
    (0x06EEA8, i_06eea8),
    (0x06EEAE, i_06eeae),
    (0x06EEB2, i_06eeb2),
    (0x06EEB6, i_06eeb6),
    (0x06EEB8, i_06eeb8),
    (0x06EEBE, i_06eebe),
    (0x06EEC4, i_06eec4),
    (0x06EEC8, i_06eec8),
    (0x06EECC, i_06eecc),
    (0x06EECE, i_06eece),
    (0x06EED2, i_06eed2),
    (0x06EED6, i_06eed6),
    (0x06EEDA, i_06eeda),
    (0x06EEDE, i_06eede),
    (0x06EEE2, i_06eee2),
    (0x06EEE6, i_06eee6),
    (0x06EEEA, i_06eeea),
    (0x06EEEC, i_06eeec),
    (0x06EEF2, i_06eef2),
    (0x06EEF6, i_06eef6),
    (0x06EEFA, i_06eefa),
    (0x06EEFC, i_06eefc),
    (0x06EF02, i_06ef02),
    (0x06EF06, i_06ef06),
    (0x06EF0C, i_06ef0c),
    (0x06EF0E, i_06ef0e),
    (0x06EF10, i_06ef10),
    (0x06EF16, i_06ef16),
    (0x06EF1A, i_06ef1a),
    (0x06EF1C, i_06ef1c),
    (0x06EF22, i_06ef22),
    (0x06EF26, i_06ef26),
    (0x06EF2A, i_06ef2a),
    (0x06EF2C, i_06ef2c),
    (0x06EF30, i_06ef30),
    (0x06EF34, i_06ef34),
    (0x06EF38, i_06ef38),
    (0x06EF3A, i_06ef3a),
    (0x06EF40, i_06ef40),
    (0x06EF44, i_06ef44),
    (0x06EF48, i_06ef48),
    (0x06EF4C, i_06ef4c),
    (0x06EF50, i_06ef50),
    (0x06EF54, i_06ef54),
    (0x06EF5A, i_06ef5a),
    (0x06EF5E, i_06ef5e),
    (0x06EF60, i_06ef60),
    (0x06EF64, i_06ef64),
    (0x06EF66, i_06ef66),
    (0x06EF6A, i_06ef6a),
    (0x06EF70, i_06ef70),
    (0x06EF74, i_06ef74),
    (0x06EF78, i_06ef78),
    (0x06EF7A, i_06ef7a),
    (0x06EF7E, i_06ef7e),
    (0x06EF80, i_06ef80),
    (0x06EF84, i_06ef84),
];
