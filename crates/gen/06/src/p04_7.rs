// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $04D3A8: jsr $27cee.l
pub fn i_04d3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3AE);
    cpu.pc = 0x27CEE;
    16
}
/// $04D3AE: jsr $28d70.l
pub fn i_04d3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3B4);
    cpu.pc = 0x28D70;
    16
}
/// $04D3B4: bcs.w $4d3c6
pub fn i_04d3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3B8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D3C6; return 10; }
    8
}
/// $04D3B8: movea.l $70(a6), a0
pub fn i_04d3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3BC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D3BC: jsr $5dd56.l
pub fn i_04d3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3C2);
    cpu.pc = 0x5DD56;
    16
}
/// $04D3C2: bcc.w $4d3cc
pub fn i_04d3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D3CC; return 10; }
    8
}
/// $04D3C6: jmp $518.l
pub fn i_04d3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D3CC: rts
pub fn i_04d3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SyncPosIfBase_04d3ce  ($04D3CE, 10 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D3CE: cmpi.b #$44, $20(a6)
pub fn i_04d3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D3D4: beq.w $4d3e0
pub fn i_04d3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D3E0; return 10; }
    8
}
// ===== JsrAbsThunk_04d3d8  ($04D3D8, 8 B, thunks/jsr_abs_thunks.c)
/// $04D3D8: jsr $2783a.l
pub fn i_04d3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3DE);
    cpu.pc = 0x2783A;
    16
}
/// $04D3DE: rts
pub fn i_04d3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SyncPosToParent_04d3e0  ($04D3E0, 24 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D3E0: movea.l $c(a6), a0
pub fn i_04d3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3E4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D3E4: move.w $22(a0), d0
pub fn i_04d3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3E8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D3E8: subi.w #$23, d0
pub fn i_04d3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3EC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x23, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D3EC: move.w d0, $22(a6)
pub fn i_04d3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D3F0: move.w $24(a0), d0
pub fn i_04d3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3F4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D3F4: addi.w #$2d, d0
pub fn i_04d3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3F8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetTaskW_04d3f8  ($04D3F8, 6 B, kernel/task_setters_w.c)
/// $04D3F8: move.w d0, $24(a6)
pub fn i_04d3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D3FC: rts
pub fn i_04d3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ProbeGuided_04d3fe  ($04D3FE, 66 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D3FE: movea.l $c(a6), a0
pub fn i_04d3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D402;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D402: cmpi.b #$44, $20(a0)
pub fn i_04d402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D408;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D408: bne.w $4d44c
pub fn i_04d408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D40C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D44C; return 10; }
    8
}
/// $04D40C: bset.b #$6, $13(a6)
pub fn i_04d40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D412;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D412: move.w $34(a6), d0
pub fn i_04d412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D416;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D416: movem.w d0, -(a7)
pub fn i_04d416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D41A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $04D41A: jsr $27bc8.l
pub fn i_04d41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D420);
    cpu.pc = 0x27BC8;
    16
}
/// $04D420: bcc.w $4d42c
pub fn i_04d420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D42C; return 10; }
    8
}
/// $04D424: move.b #$1, d1
pub fn i_04d424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D428;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04D428: bra.w $4d430
pub fn i_04d428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D42C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D430;
    10
}
/// $04D42C: move.b #$0, d1
pub fn i_04d42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D430;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04D430: movem.w (a7)+, d0
pub fn i_04d430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D434;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04D434: move.w d0, $34(a6)
pub fn i_04d434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D438;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D438: cmpi.b #$0, d1
pub fn i_04d438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D43C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $04D43C: beq.w $4d446
pub fn i_04d43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D440;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D446; return 10; }
    8
}
// ===== SetXN_04d440  ($04D440, 6 B, thunks/ccr_helpers.c)
/// $04D440: ori.b #$11, ccr
pub fn i_04d440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D444;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D444: rts
pub fn i_04d444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04d446  ($04D446, 6 B, thunks/ccr_helpers.c)
/// $04D446: andi.b #$ee, ccr
pub fn i_04d446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D44A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D44A: rts
pub fn i_04d44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D44C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_04d44c  ($04D44C, 8 B, thunks/jsr_abs_thunks.c)
/// $04D44C: jsr $27cee.l
pub fn i_04d44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D452);
    cpu.pc = 0x27CEE;
    16
}
/// $04D452: rts
pub fn i_04d452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_IsBaseIdleDone_04d454  ($04D454, 20 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D454: cmpi.b #$44, $20(a6)
pub fn i_04d454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D45A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D45A: bne.w $4d46e
pub fn i_04d45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D45E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D46E; return 10; }
    8
}
/// $04D45E: cmpi.b #$0, $80(a6)
pub fn i_04d45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D464;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04D464: bne.w $4d46e
pub fn i_04d464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D468;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D46E; return 10; }
    8
}
// ===== SetXN_04d468  ($04D468, 6 B, thunks/ccr_helpers.c)
/// $04D468: ori.b #$11, ccr
pub fn i_04d468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D46C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D46C: rts
pub fn i_04d46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D46E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04d46e  ($04D46E, 6 B, thunks/ccr_helpers.c)
/// $04D46E: andi.b #$ee, ccr
pub fn i_04d46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D472;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D472: rts
pub fn i_04d472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D474;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ClearParentStateIfBase_04d474  ($04D474, 22 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D474: cmpi.b #$44, $20(a6)
pub fn i_04d474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D47A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D47A: bne.w $4d488
pub fn i_04d47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D47E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D488; return 10; }
    8
}
/// $04D47E: movea.l $c(a6), a0
pub fn i_04d47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D482;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D482: move.b #$0, $20(a0)
pub fn i_04d482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D488;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D488: rts
pub fn i_04d488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D48A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ParentOffworld_04d48a  ($04D48A, 22 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D48A: movem.l a6, -(a7)
pub fn i_04d48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D48E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04D48E: movea.l $c(a6), a6
pub fn i_04d48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D492;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $04D492: jsr $5e45a.l
pub fn i_04d492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D498);
    cpu.pc = 0x5E45A;
    16
}
/// $04D498: bcs.w $4d4a6
pub fn i_04d498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D49C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D4A6; return 10; }
    8
}
/// $04D49C: movem.l (a7)+, a6
pub fn i_04d49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4A0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_04d4a0  ($04D4A0, 6 B, thunks/ccr_helpers.c)
/// $04D4A0: andi.b #$ee, ccr
pub fn i_04d4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4A4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D4A4: rts
pub fn i_04d4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ParentOffworldRestore_04d4a6  ($04D4A6, 4 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4A6: movem.l (a7)+, a6
pub fn i_04d4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4AA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_04d4aa  ($04D4AA, 6 B, thunks/ccr_helpers.c)
/// $04D4AA: ori.b #$11, ccr
pub fn i_04d4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4AE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D4AE: rts
pub fn i_04d4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleStep16_04d4b0  ($04D4B0, 32 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4B0: move.w $34(a6), d1
pub fn i_04d4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4B4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D4B4: sub.w d1, d0
pub fn i_04d4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4B6;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04D4B6: beq.w $4d4ce
pub fn i_04d4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D4CE; return 10; }
    8
}
/// $04D4BA: btst.b #$4, d0
pub fn i_04d4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4BE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D4BE: bne.w $4d4ca
pub fn i_04d4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D4CA; return 10; }
    8
}
/// $04D4C2: move.w #$1, d0
pub fn i_04d4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4C6: bra.w $4d4ce
pub fn i_04d4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D4CE;
    10
}
/// $04D4CA: move.w #$ff, d0
pub fn i_04d4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4CE: rts
pub fn i_04d4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleStep8_04d4d0  ($04D4D0, 32 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4D0: move.w $34(a6), d1
pub fn i_04d4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D4D4: sub.w d1, d0
pub fn i_04d4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4D6;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04D4D6: beq.w $4d4ee
pub fn i_04d4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D4EE; return 10; }
    8
}
/// $04D4DA: btst.b #$3, d0
pub fn i_04d4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4DE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D4DE: bne.w $4d4ea
pub fn i_04d4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D4EA; return 10; }
    8
}
/// $04D4E2: move.w #$1, d0
pub fn i_04d4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4E6: bra.w $4d4ee
pub fn i_04d4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D4EE;
    10
}
/// $04D4EA: move.w #$ff, d0
pub fn i_04d4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4EE: rts
pub fn i_04d4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleToTarget_04d4f0  ($04D4F0, 18 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4F0: jsr $5e136.l
pub fn i_04d4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D4F6);
    cpu.pc = 0x5E136;
    16
}
/// $04D4F6: addi.w #$8, d0
pub fn i_04d4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4FA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D4FA: asr.w #$3, d0
pub fn i_04d4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4FC;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D4FC: andi.w #$1f, d0
pub fn i_04d4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D500;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D500: rts
pub fn i_04d500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D502;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_VelFromAngle_04d502  ($04D502, 26 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D502: move.w $34(a6), d0
pub fn i_04d502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D506;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D506: asl.w #$3, d0
pub fn i_04d506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D508;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D508: move.w $36(a6), d1
pub fn i_04d508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D50C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D50C: jsr $13c0e.l
pub fn i_04d50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D512);
    cpu.pc = 0x13C0E;
    16
}
/// $04D512: move.w d1, $28(a6)
pub fn i_04d512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D516;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D516: move.w d2, $2a(a6)
pub fn i_04d516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D51A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D51A: rts
pub fn i_04d51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D51C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ShiftAngleHistory_04d51c  ($04D51C, 98 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D51C: move.w #$97, d1
pub fn i_04d51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D520;
    cpu.ir = 0x323C;
    let v1: u32 = 0x97;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D520: move.b -$1(a6, d1.w), (a6, d1.w)
pub fn i_04d520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D526;
    cpu.ir = 0x1DB6;
    let ad2 = cpu.a[6].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    24
}
/// $04D526: subq.w #$1, d1
pub fn i_04d526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D528;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D528: cmpi.w #$88, d1
pub fn i_04d528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D52C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x88, Size::Word, false);
    8
}
/// $04D52C: bgt.b $4d520
pub fn i_04d52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D52E;
    cpu.ir = 0x6EF2;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D520; return 10; }
    8
}
/// $04D52E: move.w $34(a6), d0
pub fn i_04d52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D532;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D532: move.b d0, $88(a6)
pub fn i_04d532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D536;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D536: move.b $97(a6), d0
pub fn i_04d536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D53A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04D53A: andi.w #$1f, d0
pub fn i_04d53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D53E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D53E: asl.w #$3, d0
pub fn i_04d53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D540;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D540: move.w $36(a6), d1
pub fn i_04d540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D544;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D544: jsr $4d668(pc)
pub fn i_04d544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D548;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D548);
    cpu.pc = 0x4D668;
    16
}
/// $04D548: bcc.w $4d558
pub fn i_04d548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D54C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D558; return 10; }
    8
}
/// $04D54C: btst.b #$4, $97(a6)
pub fn i_04d54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D552;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04D552: beq.w $4d558
pub fn i_04d552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D556;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D558; return 10; }
    8
}
/// $04D556: asr.w #$1, d1
pub fn i_04d556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D558;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04D558: jsr $13c0e.l
pub fn i_04d558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D55E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D55E);
    cpu.pc = 0x13C0E;
    16
}
/// $04D55E: move.w d1, $28(a6)
pub fn i_04d55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D562;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D562: move.w d2, $2a(a6)
pub fn i_04d562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D566;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D566: move.w #$0, $2e(a6)
pub fn i_04d566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D56C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D56C: btst.b #$4, $97(a6)
pub fn i_04d56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D572;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04D572: beq.w $4d57c
pub fn i_04d572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D576;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D57C; return 10; }
    8
}
/// $04D576: move.w #$ffa0, $2e(a6)
pub fn i_04d576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D57C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D57C: rts
pub fn i_04d57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D57E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Angle16_04d57e  ($04D57E, 4 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D57E: move.w #$10, d0
pub fn i_04d57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D582;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04d582  ($04D582, 8 B, thunks/jsr_abs_thunks.c)
/// $04D582: jsr $5e9e4.l
pub fn i_04d582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D588;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D588);
    cpu.pc = 0x5E9E4;
    16
}
/// $04D588: rts
pub fn i_04d588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D58A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleToTargetMirror_04d58a  ($04D58A, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D58A: jsr $5e136.l
pub fn i_04d58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D590);
    cpu.pc = 0x5E136;
    16
}
/// $04D590: addi.w #$8, d0
pub fn i_04d590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D594;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D594: asr.w #$3, d0
pub fn i_04d594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D596;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D596: btst.b #$4, d0
pub fn i_04d596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D59A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D59A: bne.w $4d5a6
pub fn i_04d59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D59E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D5A6; return 10; }
    8
}
/// $04D59E: neg.w d0
pub fn i_04d59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D5A0: clr.w d1
pub fn i_04d5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D5A2: add.w d0, d1
pub fn i_04d5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A4;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04D5A4: move.w d1, d0
pub fn i_04d5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A6;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D5A6: andi.w #$1f, d0
pub fn i_04d5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5AA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D5AA: rts
pub fn i_04d5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_OffworldA_04d5ac  ($04D5AC, 16 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5AC: lea.l $293fd4.l, a0
pub fn i_04d5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x293FD4;
    20
}
/// $04D5B2: jsr $5dd5c.l
pub fn i_04d5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D5B8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04D5B8: bcc.w $4d5c2
pub fn i_04d5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D5C2; return 10; }
    8
}
// ===== SetC_04d5bc  ($04D5BC, 6 B, thunks/ccr_helpers.c)
/// $04D5BC: ori.b #$1, ccr
pub fn i_04d5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04D5C0: rts
pub fn i_04d5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_04d5c2  ($04D5C2, 6 B, thunks/ccr_helpers.c)
/// $04D5C2: andi.b #$fe, ccr
pub fn i_04d5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04D5C6: rts
pub fn i_04d5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_OffworldB_04d5c8  ($04D5C8, 16 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5C8: lea.l $293fde.l, a0
pub fn i_04d5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x293FDE;
    20
}
/// $04D5CE: jsr $5dd56.l
pub fn i_04d5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D5D4);
    cpu.pc = 0x5DD56;
    16
}
/// $04D5D4: bcc.w $4d5de
pub fn i_04d5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D5DE; return 10; }
    8
}
// ===== SetC_04d5d8  ($04D5D8, 6 B, thunks/ccr_helpers.c)
/// $04D5D8: ori.b #$1, ccr
pub fn i_04d5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5DC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04D5DC: rts
pub fn i_04d5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_04d5de  ($04D5DE, 6 B, thunks/ccr_helpers.c)
/// $04D5DE: andi.b #$fe, ccr
pub fn i_04d5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5E2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04D5E2: rts
pub fn i_04d5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_CopyParentAngle_04d5e4  ($04D5E4, 12 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5E4: movea.l $c(a6), a0
pub fn i_04d5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D5E8: move.w $34(a0), $34(a6)
pub fn i_04d5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5EE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04D5EE: rts
pub fn i_04d5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AddMuzzleOffset_04d5f0  ($04D5F0, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5F0: lea.l $294044.l, a1
pub fn i_04d5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x294044;
    20
}
/// $04D5F6: move.w $34(a6), d0
pub fn i_04d5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D5FA: andi.w #$1f, d0
pub fn i_04d5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5FE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D5FE: asl.w #$2, d0
pub fn i_04d5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D600;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04D600: move.w (a1, d0.w), d1
pub fn i_04d600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D604;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D604: add.w d1, $22(a0)
pub fn i_04d604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D608;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D608: move.w $2(a1, d0.w), d1
pub fn i_04d608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D60C;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D60C: add.w d1, $24(a0)
pub fn i_04d60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D610;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D610: rts
pub fn i_04d610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D612;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SpriteIdxByAngle_04d612  ($04D612, 18 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D612: andi.w #$1f, d0
pub fn i_04d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D616;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D616: lea.l $2941c4.l, a0
pub fn i_04d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D61C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2941C4;
    20
}
/// $04D61C: asl.w #$1, d0
pub fn i_04d61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D61E;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04D61E: move.w (a0, d0.w), d0
pub fn i_04d61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D622;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D622: rts
pub fn i_04d622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D624;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AddOffsetA_04d624  ($04D624, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D624: lea.l $2940c4.l, a1
pub fn i_04d624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D62A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2940C4;
    20
}
/// $04D62A: move.w $34(a6), d0
pub fn i_04d62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D62E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D62E: andi.w #$1f, d0
pub fn i_04d62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D632;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D632: asl.w #$2, d0
pub fn i_04d632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D634;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04D634: move.w (a1, d0.w), d1
pub fn i_04d634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D638;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D638: add.w d1, $22(a6)
pub fn i_04d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D63C;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D63C: move.w $2(a1, d0.w), d1
pub fn i_04d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D640;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D640: add.w d1, $24(a6)
pub fn i_04d640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D644;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D644: rts
pub fn i_04d644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D646;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AddOffsetB_04d646  ($04D646, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D646: lea.l $294144.l, a1
pub fn i_04d646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D64C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x294144;
    20
}
/// $04D64C: move.w $34(a6), d0
pub fn i_04d64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D650;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D650: andi.w #$1f, d0
pub fn i_04d650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D654;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D654: asl.w #$2, d0
pub fn i_04d654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D656;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04D656: move.w (a1, d0.w), d1
pub fn i_04d656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D65A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D65A: add.w d1, $22(a6)
pub fn i_04d65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D65E;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D65E: move.w $2(a1, d0.w), d1
pub fn i_04d65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D662;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D662: add.w d1, $24(a6)
pub fn i_04d662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D666;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D666: rts
pub fn i_04d666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D668;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_GroundProbeUp_04d668  ($04D668, 52 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D668: movem.w d0-d1, -(a7)
pub fn i_04d668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D66C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $04D66C: move.w $22(a6), d1
pub fn i_04d66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D670;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D670: move.w $24(a6), d2
pub fn i_04d670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D674;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D674: move.w #$7, d5
pub fn i_04d674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D678;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D678: subi.w #$8, d2
pub fn i_04d678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D67C;
    cpu.ir = 0x442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D67C: subi.w #$1, d5
pub fn i_04d67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D680;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D680: beq.w $4d6a2
pub fn i_04d680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D684;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D6A2; return 10; }
    8
}
/// $04D684: movem.w d5, -(a7)
pub fn i_04d684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D688;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    cpu.a[7] = a;
    20
}
/// $04D688: jsr $280c6.l
pub fn i_04d688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D68E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D68E);
    cpu.pc = 0x280C6;
    16
}
/// $04D68E: movem.w (a7)+, d5
pub fn i_04d68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D692;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04D692: cmpi.b #$1, d0
pub fn i_04d692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D696;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $04D696: bne.b $4d678
pub fn i_04d696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D698;
    cpu.ir = 0x66E0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D678; return 10; }
    8
}
/// $04D698: movem.w (a7)+, d0-d1
pub fn i_04d698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D69C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetC_04d69c  ($04D69C, 6 B, thunks/ccr_helpers.c)
/// $04D69C: ori.b #$1, ccr
pub fn i_04d69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6A0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04D6A0: rts
pub fn i_04d6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_GroundProbeRestore_04d6a2  ($04D6A2, 4 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6A2: movem.w (a7)+, d0-d1
pub fn i_04d6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6A6;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== ClearC_04d6a6  ($04D6A6, 6 B, thunks/ccr_helpers.c)
/// $04D6A6: andi.b #$fe, ccr
pub fn i_04d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6AA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04D6AA: rts
pub fn i_04d6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ClampAngle_04d6ac  ($04D6AC, 36 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6AC: cmpi.w #$11, $34(a6)
pub fn i_04d6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6B2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Word, false);
    16
}
/// $04D6B2: bne.w $4d6be
pub fn i_04d6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D6BE; return 10; }
    8
}
/// $04D6B6: addi.w #$2, $34(a6)
pub fn i_04d6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6BC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D6BC: rts
pub fn i_04d6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04D6BE: cmpi.w #$1f, $34(a6)
pub fn i_04d6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6C4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Word, false);
    16
}
/// $04D6C4: bne.w $4d6ce
pub fn i_04d6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D6CE; return 10; }
    8
}
/// $04D6C8: subi.w #$2, $34(a6)
pub fn i_04d6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6CE;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D6CE: rts
pub fn i_04d6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SlotPrioCheck_04d6d0  ($04D6D0, 16 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6D0: movea.l $8(a6), a1
pub fn i_04d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6D4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D6D4: move.b $10(a6), d0
pub fn i_04d6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04D6D8: cmp.b $10(a1), d0
pub fn i_04d6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6DC;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04D6DC: bcs.w $4d6e6
pub fn i_04d6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D6E6; return 10; }
    8
}
// ===== ClearXN_04d6e0  ($04D6E0, 6 B, thunks/ccr_helpers.c)
/// $04D6E0: andi.b #$ee, ccr
pub fn i_04d6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6E4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D6E4: rts
pub fn i_04d6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04d6e6  ($04D6E6, 6 B, thunks/ccr_helpers.c)
/// $04D6E6: ori.b #$11, ccr
pub fn i_04d6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6EA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D6EA: rts
pub fn i_04d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Static_04d6ec  ($04D6EC, 94 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6EC: movea.l $3c(a6), a1
pub fn i_04d6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6F0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D6F0: jsr $2942a.l
pub fn i_04d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D6F6);
    cpu.pc = 0x2942A;
    16
}
/// $04D6F6: move.w #$8000, $38(a6)
pub fn i_04d6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D6FC: jsr $267e2.l
pub fn i_04d6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D702);
    cpu.pc = 0x267E2;
    16
}
/// $04D702: jsr $27cee.l
pub fn i_04d702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D708);
    cpu.pc = 0x27CEE;
    16
}
/// $04D708: bra.w $4d716
pub fn i_04d708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D70C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D716;
    10
}
/// $04D70C: movea.l $3c(a6), a1
pub fn i_04d70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D710;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D710: jsr $2942a.l
pub fn i_04d710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D716);
    cpu.pc = 0x2942A;
    16
}
/// $04D716: move.b #$ff, $32(a6)
pub fn i_04d716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D71C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D71C: move.b #$ff, $33(a6)
pub fn i_04d71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D722;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D722: move.b #$0, $3a(a6)
pub fn i_04d722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D728;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D728: lea.l $4d72e(pc), a1
pub fn i_04d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D72C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D72E;
    16
}
/// $04D72C: move.l a1, (a6)
pub fn i_04d72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D72E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D72E: jsr $2783a.l
pub fn i_04d72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D734);
    cpu.pc = 0x2783A;
    16
}
/// $04D734: jsr $28d70.l
pub fn i_04d734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D73A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D73A);
    cpu.pc = 0x28D70;
    16
}
/// $04D73A: jsr $4fa70(pc)
pub fn i_04d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D73E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D73E);
    cpu.pc = 0x4FA70;
    16
}
/// $04D73E: bcc.w $4d748
pub fn i_04d73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D742;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D748; return 10; }
    8
}
/// $04D742: jmp $518.l
pub fn i_04d742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D748;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D748: rts
pub fn i_04d748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D74A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Lamp_04d74a  ($04D74A, 56 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D74A: movea.l $3c(a6), a1
pub fn i_04d74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D74E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D74E: jsr $2942a.l
pub fn i_04d74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D754);
    cpu.pc = 0x2942A;
    16
}
/// $04D754: move.w #$41, d1
pub fn i_04d754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D758;
    cpu.ir = 0x323C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D758: jsr $236e.l
pub fn i_04d758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D75E);
    cpu.pc = 0x236E;
    16
}
/// $04D75E: move.b #$ff, $32(a6)
pub fn i_04d75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D764;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D764: move.b #$ff, $33(a6)
pub fn i_04d764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D76A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D76A: move.b #$0, $3a(a6)
pub fn i_04d76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D770;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D770: lea.l $294280.l, a0
pub fn i_04d770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D776;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294280;
    20
}
/// $04D776: jsr $28cd4.l
pub fn i_04d776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D77C);
    cpu.pc = 0x28CD4;
    16
}
/// $04D77C: lea.l $4d782(pc), a1
pub fn i_04d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D780;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D782;
    16
}
/// $04D780: move.l a1, (a6)
pub fn i_04d780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D782;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Prop_LampIdle_04d782  ($04D782, 90 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D782: jsr $2783a.l
pub fn i_04d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D788);
    cpu.pc = 0x2783A;
    16
}
/// $04D788: jsr $28d70.l
pub fn i_04d788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D78E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D78E);
    cpu.pc = 0x28D70;
    16
}
/// $04D78E: jsr $2870a.l
pub fn i_04d78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D794;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D794);
    cpu.pc = 0x2870A;
    16
}
/// $04D794: bcc.w $4d7cc
pub fn i_04d794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D798;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D7CC; return 10; }
    8
}
/// $04D798: move.w #$10a9, d0
pub fn i_04d798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D79C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D79C: jsr $2352.l
pub fn i_04d79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7A2);
    cpu.pc = 0x2352;
    16
}
/// $04D7A2: jsr $5e9b6.l
pub fn i_04d7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7A8);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D7A8: andi.w #$3, d0
pub fn i_04d7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7AC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D7AC: bne.w $4d7c6
pub fn i_04d7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D7C6; return 10; }
    8
}
/// $04D7B0: lea.l $4d81a(pc), a1
pub fn i_04d7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D81A;
    16
}
/// $04D7B4: jsr $6fe.l
pub fn i_04d7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7BA);
    cpu.pc = 0x6FE;
    16
}
/// $04D7BA: jsr $5dd02.l
pub fn i_04d7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7C0);
    cpu.pc = 0x5DD02;
    16
}
/// $04D7C0: addi.w #$18, $24(a0)
pub fn i_04d7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7C6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D7C6: lea.l $4d7dc(pc), a1
pub fn i_04d7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D7DC;
    16
}
/// $04D7CA: move.l a1, (a6)
pub fn i_04d7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D7CC: jsr $4fa70(pc)
pub fn i_04d7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D7D0);
    cpu.pc = 0x4FA70;
    16
}
/// $04D7D0: bcc.w $4d7da
pub fn i_04d7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D7DA; return 10; }
    8
}
/// $04D7D4: jmp $518.l
pub fn i_04d7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D7DA: rts
pub fn i_04d7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_LampHit_04d7dc  ($04D7DC, 62 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D7DC: lea.l $294296.l, a0
pub fn i_04d7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294296;
    20
}
/// $04D7E2: jsr $28cd4.l
pub fn i_04d7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7E8);
    cpu.pc = 0x28CD4;
    16
}
/// $04D7E8: lea.l $4d7ee(pc), a1
pub fn i_04d7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D7EE;
    16
}
/// $04D7EC: move.l a1, (a6)
pub fn i_04d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D7EE: jsr $2783a.l
pub fn i_04d7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7F4);
    cpu.pc = 0x2783A;
    16
}
/// $04D7F4: jsr $28d70.l
pub fn i_04d7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7FA);
    cpu.pc = 0x28D70;
    16
}
/// $04D7FA: bcc.w $4d80a
pub fn i_04d7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D80A; return 10; }
    8
}
/// $04D7FE: bclr.b #$3, $13(a6)
pub fn i_04d7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D804;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D804: lea.l $4d782(pc), a1
pub fn i_04d804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D808;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D782;
    16
}
/// $04D808: move.l a1, (a6)
pub fn i_04d808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D80A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D80A: jsr $4fa70(pc)
pub fn i_04d80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D80E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D80E);
    cpu.pc = 0x4FA70;
    16
}
/// $04D80E: bcc.w $4d818
pub fn i_04d80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D812;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D818; return 10; }
    8
}
/// $04D812: jmp $518.l
pub fn i_04d812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D818;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D818: rts
pub fn i_04d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D81A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_LampSpark_04d81a  ($04D81A, 216 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D81A: move.w #$41, d1
pub fn i_04d81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D81E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D81E: jsr $236e.l
pub fn i_04d81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D824;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D824);
    cpu.pc = 0x236E;
    16
}
/// $04D824: bset.b #$6, $12(a6)
pub fn i_04d824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D82A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D82A: move.b #$ff, $32(a6)
pub fn i_04d82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D830;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D830: move.b #$ff, $33(a6)
pub fn i_04d830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D836;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D836: move.b #$0, $3a(a6)
pub fn i_04d836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D83C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D83C: jsr $267e2.l
pub fn i_04d83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D842;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D842);
    cpu.pc = 0x267E2;
    16
}
/// $04D842: move.w #$f0, $72(a6)
pub fn i_04d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D848: jsr $5e9b6.l
pub fn i_04d848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D84E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D84E);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D84E: andi.w #$1f, d0
pub fn i_04d84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D852;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D852: add.w d0, $72(a6)
pub fn i_04d852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D856;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D856: jsr $5e9b6.l
pub fn i_04d856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D85C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D85C);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D85C: andi.w #$1f, d0
pub fn i_04d85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D860;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D860: btst.b #$0, d0
pub fn i_04d860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D864;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D864: beq.w $4d86a
pub fn i_04d864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D868;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D86A; return 10; }
    8
}
/// $04D868: neg.w d0
pub fn i_04d868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D86A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D86A: move.w d0, $28(a6)
pub fn i_04d86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D86E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D86E: jsr $5e9b6.l
pub fn i_04d86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D874);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D874: andi.w #$3f, d0
pub fn i_04d874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D878;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D878: btst.b #$0, d0
pub fn i_04d878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D87C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D87C: beq.w $4d882
pub fn i_04d87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D880;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D882; return 10; }
    8
}
/// $04D880: neg.w d0
pub fn i_04d880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D882;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D882: move.w d0, $2a(a6)
pub fn i_04d882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D886;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D886: lea.l $294378.l, a0
pub fn i_04d886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D88C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294378;
    20
}
/// $04D88C: jsr $28cd4.l
pub fn i_04d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D892);
    cpu.pc = 0x28CD4;
    16
}
/// $04D892: lea.l $4d898(pc), a1
pub fn i_04d892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D896;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D898;
    16
}
/// $04D896: move.l a1, (a6)
pub fn i_04d896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D898;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D898: jsr $27cee.l
pub fn i_04d898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D89E);
    cpu.pc = 0x27CEE;
    16
}
/// $04D89E: cmpi.w #$14, $72(a6)
pub fn i_04d89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $04D8A4: bgt.w $4d8be
pub fn i_04d8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8A8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D8BE; return 10; }
    8
}
/// $04D8A8: move.w $72(a6), d0
pub fn i_04d8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D8AC: btst.b #$0, d0
pub fn i_04d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8B0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D8B0: beq.w $4d8be
pub fn i_04d8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D8BE; return 10; }
    8
}
/// $04D8B4: subi.b #$1, $46(a6)
pub fn i_04d8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8BA;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D8BA: bra.w $4d8c4
pub fn i_04d8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D8C4;
    10
}
/// $04D8BE: jsr $28d70.l
pub fn i_04d8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D8C4);
    cpu.pc = 0x28D70;
    16
}
/// $04D8C4: subi.w #$1, $72(a6)
pub fn i_04d8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8CA;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D8CA: cmpi.w #$0, $72(a6)
pub fn i_04d8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8D0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04D8D0: bgt.w $4d8da
pub fn i_04d8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8D4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D8DA; return 10; }
    8
}
/// $04D8D4: jmp $518.l
pub fn i_04d8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D8DA: lea.l $296254.l, a0
pub fn i_04d8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296254;
    20
}
/// $04D8E0: jsr $5dd5c.l
pub fn i_04d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D8E6);
    cpu.pc = 0x5DD5C;
    16
}
/// $04D8E6: bcc.w $4d8f0
pub fn i_04d8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D8F0; return 10; }
    8
}
/// $04D8EA: jmp $518.l
pub fn i_04d8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D8F0: rts
pub fn i_04d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Hut_04d8f2  ($04D8F2, 164 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D8F2: movea.l $3c(a6), a1
pub fn i_04d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8F6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D8F6: jsr $2942a.l
pub fn i_04d8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D8FC);
    cpu.pc = 0x2942A;
    16
}
/// $04D8FC: move.w #$3f, d1
pub fn i_04d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D900;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D900: jsr $236e.l
pub fn i_04d900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D906);
    cpu.pc = 0x236E;
    16
}
/// $04D906: move.b #$ff, $32(a6)
pub fn i_04d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D90C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D90C: move.b #$ff, $33(a6)
pub fn i_04d90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D912;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D912: move.b #$0, $3a(a6)
pub fn i_04d912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D918;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D918: move.b #$0, $21(a6)
pub fn i_04d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D91E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D91E: lea.l $5fa00.l, a1
pub fn i_04d91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D924;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5FA00;
    20
}
/// $04D924: jsr $4ae.l
pub fn i_04d924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D92A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D92A);
    cpu.pc = 0x4AE;
    16
}
/// $04D92A: jsr $5dd22.l
pub fn i_04d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D930);
    cpu.pc = 0x5DD22;
    16
}
/// $04D930: addi.w #$28, $22(a0)
pub fn i_04d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D936;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D936: addi.w #$1, $24(a0)
pub fn i_04d936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D93C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D93C: lea.l $4d9d6(pc), a1
pub fn i_04d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D940;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D9D6;
    16
}
/// $04D940: jsr $4ae.l
pub fn i_04d940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D946);
    cpu.pc = 0x4AE;
    16
}
/// $04D946: jsr $5dd22.l
pub fn i_04d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D94C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D94C);
    cpu.pc = 0x5DD22;
    16
}
/// $04D94C: addi.w #$28, $22(a0)
pub fn i_04d94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D952;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D952: addi.w #$1, $24(a0)
pub fn i_04d952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D958;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D958: lea.l $294234.l, a0
pub fn i_04d958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D95E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294234;
    20
}
/// $04D95E: jsr $28cd4.l
pub fn i_04d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D964);
    cpu.pc = 0x28CD4;
    16
}
/// $04D964: lea.l $4d96a(pc), a1
pub fn i_04d964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D96A;
    16
}
/// $04D968: move.l a1, (a6)
pub fn i_04d968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D96A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D96A: jsr $2783a.l
pub fn i_04d96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D970);
    cpu.pc = 0x2783A;
    16
}
/// $04D970: jsr $28d70.l
pub fn i_04d970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D976);
    cpu.pc = 0x28D70;
    16
}
/// $04D976: cmpi.b #$ff, $21(a6)
pub fn i_04d976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D97C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D97C: bne.w $4d986
pub fn i_04d97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D980;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D986; return 10; }
    8
}
/// $04D980: lea.l $4d996(pc), a1
pub fn i_04d980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D984;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D996;
    16
}
/// $04D984: move.l a1, (a6)
pub fn i_04d984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D986;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D986: jsr $4fa70(pc)
pub fn i_04d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D98A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D98A);
    cpu.pc = 0x4FA70;
    16
}
/// $04D98A: bcc.w $4d994
pub fn i_04d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D98E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D994; return 10; }
    8
}
/// $04D98E: jmp $518.l
pub fn i_04d98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D994;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D994: rts
pub fn i_04d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D996;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HutWreck_04d996  ($04D996, 64 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D996: jsr $2783a.l
pub fn i_04d996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D99C);
    cpu.pc = 0x2783A;
    16
}
/// $04D99C: lea.l $295d38.l, a1
pub fn i_04d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295D38;
    20
}
/// $04D9A2: jsr $43fac.l
pub fn i_04d9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9A8);
    cpu.pc = 0x43FAC;
    16
}
/// $04D9A8: lea.l $294244.l, a0
pub fn i_04d9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294244;
    20
}
/// $04D9AE: jsr $28cd4.l
pub fn i_04d9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9B4);
    cpu.pc = 0x28CD4;
    16
}
/// $04D9B4: lea.l $4d9ba(pc), a1
pub fn i_04d9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D9BA;
    16
}
/// $04D9B8: move.l a1, (a6)
pub fn i_04d9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D9BA: jsr $2783a.l
pub fn i_04d9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9C0);
    cpu.pc = 0x2783A;
    16
}
/// $04D9C0: jsr $28d70.l
pub fn i_04d9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9C6);
    cpu.pc = 0x28D70;
    16
}
/// $04D9C6: jsr $4fa70(pc)
pub fn i_04d9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D9CA);
    cpu.pc = 0x4FA70;
    16
}
/// $04D9CA: bcc.w $4d9d4
pub fn i_04d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D9D4; return 10; }
    8
}
/// $04D9CE: jmp $518.l
pub fn i_04d9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D9D4: rts
pub fn i_04d9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HutRoof_04d9d6  ($04D9D6, 124 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D9D6: move.w #$3d, d1
pub fn i_04d9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9DA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D9DA: jsr $236e.l
pub fn i_04d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9E0);
    cpu.pc = 0x236E;
    16
}
/// $04D9E0: move.b #$ff, $32(a6)
pub fn i_04d9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D9E6: move.b #$ff, $33(a6)
pub fn i_04d9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D9EC: move.b #$0, $3a(a6)
pub fn i_04d9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D9F2: move.w #$14, $66(a6)
pub fn i_04d9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D9F8: lea.l $294254.l, a0
pub fn i_04d9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294254;
    20
}
/// $04D9FE: jsr $28cd4.l
pub fn i_04d9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA04);
    cpu.pc = 0x28CD4;
    16
}
/// $04DA04: lea.l $4da0a(pc), a1
pub fn i_04da04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DA0A;
    16
}
/// $04DA08: move.l a1, (a6)
pub fn i_04da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DA0A: jsr $2783a.l
pub fn i_04da0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA10);
    cpu.pc = 0x2783A;
    16
}
/// $04DA10: jsr $28d70.l
pub fn i_04da10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA16);
    cpu.pc = 0x28D70;
    16
}
/// $04DA16: jsr $2870a.l
pub fn i_04da16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA1C);
    cpu.pc = 0x2870A;
    16
}
/// $04DA1C: bcc.w $4da32
pub fn i_04da1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DA32; return 10; }
    8
}
/// $04DA20: lea.l $5e766.l, a0
pub fn i_04da20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DA26: jsr $5e770.l
pub fn i_04da26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA2C);
    cpu.pc = 0x5E770;
    16
}
/// $04DA2C: bclr.b #$3, $13(a6)
pub fn i_04da2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA32;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DA32: jsr $28758.l
pub fn i_04da32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA38);
    cpu.pc = 0x28758;
    16
}
/// $04DA38: bcc.w $4da42
pub fn i_04da38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DA42; return 10; }
    8
}
/// $04DA3C: lea.l $4da52(pc), a1
pub fn i_04da3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DA52;
    16
}
/// $04DA40: move.l a1, (a6)
pub fn i_04da40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DA42: jsr $4fa70(pc)
pub fn i_04da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DA46);
    cpu.pc = 0x4FA70;
    16
}
/// $04DA46: bcc.w $4da50
pub fn i_04da46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DA50; return 10; }
    8
}
/// $04DA4A: jmp $518.l
pub fn i_04da4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA50;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DA50: rts
pub fn i_04da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HutRoofFall_04da52  ($04DA52, 120 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DA52: move.l #$1000, d0
pub fn i_04da52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA58;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DA58: jsr $51a28.l
pub fn i_04da58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA5E);
    cpu.pc = 0x51A28;
    16
}
/// $04DA5E: lea.l $295fba.l, a0
pub fn i_04da5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x295FBA;
    20
}
/// $04DA64: move.l a0, $4c(a6)
pub fn i_04da64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA68;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DA68: jsr $283ca.l
pub fn i_04da68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA6E);
    cpu.pc = 0x283CA;
    16
}
/// $04DA6E: jsr $283ca.l
pub fn i_04da6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA74);
    cpu.pc = 0x283CA;
    16
}
/// $04DA74: jsr $283d8.l
pub fn i_04da74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA7A);
    cpu.pc = 0x283D8;
    16
}
/// $04DA7A: lea.l $29426a.l, a0
pub fn i_04da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29426A;
    20
}
/// $04DA80: jsr $28cd4.l
pub fn i_04da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA86);
    cpu.pc = 0x28CD4;
    16
}
/// $04DA86: lea.l $4da8c(pc), a1
pub fn i_04da86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DA8C;
    16
}
/// $04DA8A: move.l a1, (a6)
pub fn i_04da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DA8C: jsr $2783a.l
pub fn i_04da8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA92);
    cpu.pc = 0x2783A;
    16
}
/// $04DA92: jsr $28d70.l
pub fn i_04da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA98);
    cpu.pc = 0x28D70;
    16
}
/// $04DA98: bcc.w $4dad0
pub fn i_04da98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DAD0; return 10; }
    8
}
/// $04DA9C: lea.l $ffff.w, a0
pub fn i_04da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAA0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DAA0: move.l a0, $4c(a6)
pub fn i_04daa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAA4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DAA4: jsr $283ca.l
pub fn i_04daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAAA);
    cpu.pc = 0x283CA;
    16
}
/// $04DAAA: move.w #$1028, d0
pub fn i_04daaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DAAE: jsr $2352.l
pub fn i_04daae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAB4);
    cpu.pc = 0x2352;
    16
}
/// $04DAB4: movea.l $c(a6), a0
pub fn i_04dab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAB8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DAB8: move.b #$ff, $21(a0)
pub fn i_04dab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DABE;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DABE: lea.l $295bd8.l, a1
pub fn i_04dabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAC4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BD8;
    20
}
/// $04DAC4: jsr $77c7e.l
pub fn i_04dac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DACA);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetTaskHandler_04daca  ($04DACA, 8 B, kernel/task_handlers.c)
/// $04DACA: lea.l $4f2a4(pc), a1
pub fn i_04daca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DACE: move.l a1, (a6)
pub fn i_04dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DAD0: rts
pub fn i_04dad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Tower_04dad2  ($04DAD2, 154 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DAD2: movea.l $3c(a6), a1
pub fn i_04dad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAD6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04DAD6: jsr $2942a.l
pub fn i_04dad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DADC);
    cpu.pc = 0x2942A;
    16
}
/// $04DADC: lea.l $4f2c2(pc), a1
pub fn i_04dadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2C2;
    16
}
/// $04DAE0: jsr $4ae.l
pub fn i_04dae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAE6);
    cpu.pc = 0x4AE;
    16
}
/// $04DAE6: jsr $5dd22.l
pub fn i_04dae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAEC);
    cpu.pc = 0x5DD22;
    16
}
/// $04DAEC: addi.w #$5c, $22(a0)
pub fn i_04daec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAF2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x5C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DAF2: addi.w #$22, $24(a0)
pub fn i_04daf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAF8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x22, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DAF8: lea.l $4db72(pc), a1
pub fn i_04daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DB72;
    16
}
/// $04DAFC: jsr $4ae.l
pub fn i_04dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB02);
    cpu.pc = 0x4AE;
    16
}
/// $04DB02: jsr $5dd22.l
pub fn i_04db02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB08);
    cpu.pc = 0x5DD22;
    16
}
/// $04DB08: lea.l $4dce6(pc), a1
pub fn i_04db08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DCE6;
    16
}
/// $04DB0C: jsr $4ae.l
pub fn i_04db0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB12);
    cpu.pc = 0x4AE;
    16
}
/// $04DB12: jsr $5dd22.l
pub fn i_04db12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB18);
    cpu.pc = 0x5DD22;
    16
}
/// $04DB18: addi.w #$80, $22(a0)
pub fn i_04db18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB1E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DB1E: addi.w #$30, $24(a0)
pub fn i_04db1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB24;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DB24: move.b #$0, $21(a6)
pub fn i_04db24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB2A: move.b #$0, $20(a6)
pub fn i_04db2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB30: lea.l $4db36(pc), a1
pub fn i_04db30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DB36;
    16
}
/// $04DB34: move.l a1, (a6)
pub fn i_04db34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DB36: cmpi.b #$ff, $21(a6)
pub fn i_04db36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB3C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04DB3C: bne.w $4db48
pub fn i_04db3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4DB48; return 10; }
    8
}
/// $04DB40: jmp $518.l
pub fn i_04db40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB46;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DB48: move.b $20(a6), d0
pub fn i_04db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB4C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04DB4C: cmp.b $21(a6), d0
pub fn i_04db4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB50;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04DB50: beq.w $4db58
pub fn i_04db50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB54;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4DB58; return 10; }
    8
}
/// $04DB54: jsr $4fa8a(pc)
pub fn i_04db54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DB58);
    cpu.pc = 0x4FA8A;
    16
}
/// $04DB58: cmpi.b #$12, $21(a6)
pub fn i_04db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB5E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x12, Size::Byte, false);
    16
}
/// $04DB5E: bne.w $4db68
pub fn i_04db5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4DB68; return 10; }
    8
}
/// $04DB62: jmp $518.l
pub fn i_04db62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DB68: move.b $21(a6), d0
pub fn i_04db68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB6C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
// ===== SetTaskB_04db6c  ($04DB6C, 6 B, kernel/task_setters_b.c)
/// $04DB6C: move.b d0, $20(a6)
pub fn i_04db6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB70;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB70: rts
pub fn i_04db70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerTop_04db72  ($04DB72, 208 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DB72: lea.l $e92b2.l, a1
pub fn i_04db72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE92B2;
    20
}
/// $04DB78: move.w #$75, d0
pub fn i_04db78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB7C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DB7C: move.b #$0, $75(a6)
pub fn i_04db7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB82;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB82: jsr $4429e.l
pub fn i_04db82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB88);
    cpu.pc = 0x4429E;
    16
}
/// $04DB88: move.w #$50, $70(a6)
pub fn i_04db88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DB8E: move.w #$3c, $66(a6)
pub fn i_04db8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DB94: move.b #$0, $20(a6)
pub fn i_04db94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB9A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB9A: lea.l $294446.l, a0
pub fn i_04db9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294446;
    20
}
/// $04DBA0: jsr $28cd4.l
pub fn i_04dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBA6);
    cpu.pc = 0x28CD4;
    16
}
/// $04DBA6: lea.l $4dbac(pc), a1
pub fn i_04dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DBAC;
    16
}
/// $04DBAA: move.l a1, (a6)
pub fn i_04dbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DBAC: jsr $2783a.l
pub fn i_04dbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBB2);
    cpu.pc = 0x2783A;
    16
}
/// $04DBB2: jsr $28d70.l
pub fn i_04dbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBB8);
    cpu.pc = 0x28D70;
    16
}
/// $04DBB8: jsr $2870a.l
pub fn i_04dbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBBE);
    cpu.pc = 0x2870A;
    16
}
/// $04DBBE: bcc.w $4dbd4
pub fn i_04dbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DBD4; return 10; }
    8
}
/// $04DBC2: lea.l $5e766.l, a0
pub fn i_04dbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DBC8: jsr $5e770.l
pub fn i_04dbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBCE);
    cpu.pc = 0x5E770;
    16
}
/// $04DBCE: bclr.b #$3, $13(a6)
pub fn i_04dbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBD4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DBD4: jsr $28758.l
pub fn i_04dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBDA);
    cpu.pc = 0x28758;
    16
}
/// $04DBDA: bcc.w $4dc2a
pub fn i_04dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DC2A; return 10; }
    8
}
/// $04DBDE: addi.w #$10, $22(a6)
pub fn i_04dbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBE4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DBE4: lea.l $295b5a.l, a1
pub fn i_04dbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B5A;
    20
}
/// $04DBEA: jsr $77c7e.l
pub fn i_04dbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBF0);
    cpu.pc = 0x77C7E;
    16
}
/// $04DBF0: lea.l $295b6c.l, a1
pub fn i_04dbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBF6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B6C;
    20
}
/// $04DBF6: jsr $77c7e.l
pub fn i_04dbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBFC);
    cpu.pc = 0x77C7E;
    16
}
/// $04DBFC: lea.l $295cb0.l, a1
pub fn i_04dbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC02;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04DC02: jsr $77c7e.l
pub fn i_04dc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC08);
    cpu.pc = 0x77C7E;
    16
}
/// $04DC08: addi.w #$20, $22(a0)
pub fn i_04dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC0E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DC0E: addi.w #$30, $24(a0)
pub fn i_04dc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC14;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DC14: movea.l $c(a6), a0
pub fn i_04dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC18;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DC18: addi.b #$10, $21(a0)
pub fn i_04dc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC1E;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DC1E: move.b #$ff, $20(a6)
pub fn i_04dc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC24;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DC24: lea.l $4dc4a(pc), a1
pub fn i_04dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DC4A;
    16
}
/// $04DC28: move.l a1, (a6)
pub fn i_04dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DC2A: jsr $4fa70(pc)
pub fn i_04dc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DC2E);
    cpu.pc = 0x4FA70;
    16
}
/// $04DC2E: bcc.w $4dc48
pub fn i_04dc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DC48; return 10; }
    8
}
/// $04DC32: move.b #$ff, $20(a6)
pub fn i_04dc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC38;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DC38: movea.l $c(a6), a0
pub fn i_04dc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC3C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DC3C: ori.b #$f0, $21(a0)
pub fn i_04dc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC42;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_04dc42  ($04DC42, 8 B, kernel/task_handlers.c)
/// $04DC42: lea.l $4dc88(pc), a1
pub fn i_04dc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DC88;
    16
}
/// $04DC46: move.l a1, (a6)
pub fn i_04dc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DC48: rts
pub fn i_04dc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerTopWreck_04dc4a  ($04DC4A, 62 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DC4A: move.l #$1000, d0
pub fn i_04dc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC50;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DC50: jsr $51a28.l
pub fn i_04dc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC56);
    cpu.pc = 0x51A28;
    16
}
/// $04DC56: lea.l $ffff.w, a0
pub fn i_04dc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC5A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DC5A: move.l a0, $48(a6)
pub fn i_04dc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC5E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DC5E: move.b #$ff, $75(a6)
pub fn i_04dc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC64;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DC64: move.w #$1030, d0
pub fn i_04dc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DC68: jsr $2352.l
pub fn i_04dc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC6E);
    cpu.pc = 0x2352;
    16
}
/// $04DC6E: lea.l $29556a.l, a2
pub fn i_04dc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC74;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29556A;
    20
}
/// $04DC74: jsr $5022a(pc)
pub fn i_04dc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DC78);
    cpu.pc = 0x5022A;
    16
}
/// $04DC78: lea.l $2955e2.l, a2
pub fn i_04dc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC7E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2955E2;
    20
}
/// $04DC7E: jsr $5022a(pc)
pub fn i_04dc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DC82);
    cpu.pc = 0x5022A;
    16
}
/// $04DC82: lea.l $4dc88(pc), a1
pub fn i_04dc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DC88;
    16
}
/// $04DC86: move.l a1, (a6)
pub fn i_04dc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Prop_TowerTopWreckLoop_04dc88  ($04DC88, 6 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DC88: jsr $2783a.l
pub fn i_04dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC8E);
    cpu.pc = 0x2783A;
    16
}
// ===== SetTaskHandler_04dc8e  ($04DC8E, 8 B, kernel/task_handlers.c)
/// $04DC8E: lea.l $4f2a4(pc), a1
pub fn i_04dc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DC92: move.l a1, (a6)
pub fn i_04dc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DC94: rts
pub fn i_04dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerFlag_04dc96  ($04DC96, 80 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DC96: move.w #$40, d1
pub fn i_04dc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC9A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DC9A: jsr $236e.l
pub fn i_04dc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCA0);
    cpu.pc = 0x236E;
    16
}
/// $04DCA0: move.b #$ff, $32(a6)
pub fn i_04dca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCA6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DCA6: move.b #$ff, $33(a6)
pub fn i_04dca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCAC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DCAC: move.b #$0, $3a(a6)
pub fn i_04dcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DCB2: lea.l $29440a.l, a0
pub fn i_04dcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29440A;
    20
}
/// $04DCB8: jsr $28cd4.l
pub fn i_04dcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCBE);
    cpu.pc = 0x28CD4;
    16
}
/// $04DCBE: lea.l $4dcc4(pc), a1
pub fn i_04dcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DCC4;
    16
}
/// $04DCC2: move.l a1, (a6)
pub fn i_04dcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DCC4: jsr $2783a.l
pub fn i_04dcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCCA);
    cpu.pc = 0x2783A;
    16
}
/// $04DCCA: jsr $28d70.l
pub fn i_04dcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCD0);
    cpu.pc = 0x28D70;
    16
}
/// $04DCD0: movea.l $c(a6), a0
pub fn i_04dcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DCD4: cmpi.b #$ff, $20(a0)
pub fn i_04dcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCDA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04DCDA: bne.w $4dce4
pub fn i_04dcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCDE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4DCE4; return 10; }
    8
}
/// $04DCDE: jmp $518.l
pub fn i_04dcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCE4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DCE4: rts
pub fn i_04dce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBase_04dce6  ($04DCE6, 254 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DCE6: lea.l $5f384.l, a1
pub fn i_04dce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCEC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5F384;
    20
}
/// $04DCEC: jsr $4ae.l
pub fn i_04dcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCF2);
    cpu.pc = 0x4AE;
    16
}
/// $04DCF2: jsr $5dd22.l
pub fn i_04dcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCF8);
    cpu.pc = 0x5DD22;
    16
}
/// $04DCF8: addi.w #$29, $22(a0)
pub fn i_04dcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCFE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x29, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DCFE: addi.w #$9, $24(a0)
pub fn i_04dcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD04;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x9, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DD04: move.w #$50, $70(a6)
pub fn i_04dd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DD0A: move.w #$3c, $66(a6)
pub fn i_04dd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DD10: move.b #$0, $21(a6)
pub fn i_04dd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD16: move.b #$0, $20(a6)
pub fn i_04dd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD1C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD1C: lea.l $29445c.l, a0
pub fn i_04dd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29445C;
    20
}
/// $04DD22: jsr $28cd4.l
pub fn i_04dd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD28);
    cpu.pc = 0x28CD4;
    16
}
/// $04DD28: lea.l $4dd2e(pc), a1
pub fn i_04dd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DD2E;
    16
}
/// $04DD2C: move.l a1, (a6)
pub fn i_04dd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DD2E: jsr $2783a.l
pub fn i_04dd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD34);
    cpu.pc = 0x2783A;
    16
}
/// $04DD34: jsr $28d70.l
pub fn i_04dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD3A);
    cpu.pc = 0x28D70;
    16
}
/// $04DD3A: cmpi.b #$1, $20(a6)
pub fn i_04dd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD40;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04DD40: bge.w $4dd84
pub fn i_04dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD44;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x4DD84; return 10; }
    8
}
/// $04DD44: cmpi.w #$1e, $66(a6)
pub fn i_04dd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD4A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $04DD4A: bgt.w $4dd84
pub fn i_04dd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD4E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4DD84; return 10; }
    8
}
/// $04DD4E: move.w #$1028, d0
pub fn i_04dd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DD52: jsr $2352.l
pub fn i_04dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD58);
    cpu.pc = 0x2352;
    16
}
/// $04DD58: lea.l $295ba2.l, a1
pub fn i_04dd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD5E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BA2;
    20
}
/// $04DD5E: jsr $77c7e.l
pub fn i_04dd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD64);
    cpu.pc = 0x77C7E;
    16
}
/// $04DD64: lea.l $2955f6.l, a2
pub fn i_04dd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD6A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2955F6;
    20
}
/// $04DD6A: jsr $5022a(pc)
pub fn i_04dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DD6E);
    cpu.pc = 0x5022A;
    16
}
/// $04DD6E: move.b #$ff, $21(a6)
pub fn i_04dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD74: movea.l $c(a6), a0
pub fn i_04dd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD78;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DD78: addi.b #$1, $21(a0)
pub fn i_04dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD7E;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DD7E: move.b #$1, $20(a6)
pub fn i_04dd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD84;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD84: jsr $2870a.l
pub fn i_04dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD8A);
    cpu.pc = 0x2870A;
    16
}
/// $04DD8A: bcc.w $4dda0
pub fn i_04dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DDA0; return 10; }
    8
}
/// $04DD8E: lea.l $5e766.l, a0
pub fn i_04dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DD94: jsr $5e770.l
pub fn i_04dd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD9A);
    cpu.pc = 0x5E770;
    16
}
/// $04DD9A: bclr.b #$3, $13(a6)
pub fn i_04dd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDA0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DDA0: jsr $28758.l
pub fn i_04dda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDA6);
    cpu.pc = 0x28758;
    16
}
/// $04DDA6: bcc.w $4ddd2
pub fn i_04dda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DDD2; return 10; }
    8
}
/// $04DDAA: lea.l $295ba2.l, a1
pub fn i_04ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDB0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BA2;
    20
}
/// $04DDB0: jsr $77c7e.l
pub fn i_04ddb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDB6);
    cpu.pc = 0x77C7E;
    16
}
/// $04DDB6: lea.l $295bb4.l, a1
pub fn i_04ddb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDBC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BB4;
    20
}
/// $04DDBC: jsr $77c7e.l
pub fn i_04ddbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDC2);
    cpu.pc = 0x77C7E;
    16
}
/// $04DDC2: movea.l $c(a6), a0
pub fn i_04ddc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDC6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DDC6: addi.b #$1, $21(a0)
pub fn i_04ddc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDCC;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DDCC: lea.l $4ddec(pc), a1
pub fn i_04ddcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DDEC;
    16
}
/// $04DDD0: move.l a1, (a6)
pub fn i_04ddd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DDD2: jsr $4fa70(pc)
pub fn i_04ddd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DDD6);
    cpu.pc = 0x4FA70;
    16
}
/// $04DDD6: bcc.w $4ddea
pub fn i_04ddd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DDEA; return 10; }
    8
}
/// $04DDDA: movea.l $c(a6), a0
pub fn i_04ddda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDDE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DDDE: ori.b #$f, $21(a0)
pub fn i_04ddde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDE4;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_04dde4  ($04DDE4, 8 B, kernel/task_handlers.c)
/// $04DDE4: lea.l $4de32(pc), a1
pub fn i_04dde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DE32;
    16
}
/// $04DDE8: move.l a1, (a6)
pub fn i_04dde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DDEA: rts
pub fn i_04ddea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBaseWreck_04ddec  ($04DDEC, 70 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DDEC: move.l #$1000, d0
pub fn i_04ddec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDF2;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DDF2: jsr $51a28.l
pub fn i_04ddf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDF8);
    cpu.pc = 0x51A28;
    16
}
/// $04DDF8: lea.l $ffff.w, a0
pub fn i_04ddf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDFC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DDFC: move.l a0, $48(a6)
pub fn i_04ddfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE00;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DE00: move.w #$1028, d0
pub fn i_04de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DE04: jsr $2352.l
pub fn i_04de04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE0A);
    cpu.pc = 0x2352;
    16
}
/// $04DE0A: lea.l $29560a.l, a2
pub fn i_04de0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29560A;
    20
}
/// $04DE10: jsr $5022a(pc)
pub fn i_04de10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DE14);
    cpu.pc = 0x5022A;
    16
}
/// $04DE14: lea.l $295cb0.l, a1
pub fn i_04de14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE1A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04DE1A: jsr $77c7e.l
pub fn i_04de1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE20);
    cpu.pc = 0x77C7E;
    16
}
/// $04DE20: addi.w #$ffe0, $22(a0)
pub fn i_04de20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE26;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DE26: addi.w #$30, $24(a0)
pub fn i_04de26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE2C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DE2C: lea.l $4de32(pc), a1
pub fn i_04de2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DE32;
    16
}
/// $04DE30: move.l a1, (a6)
pub fn i_04de30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Prop_TowerBaseWreckLoop_04de32  ($04DE32, 6 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DE32: jsr $2783a.l
pub fn i_04de32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE38);
    cpu.pc = 0x2783A;
    16
}
// ===== SetTaskHandler_04de38  ($04DE38, 8 B, kernel/task_handlers.c)
/// $04DE38: lea.l $4f2a4(pc), a1
pub fn i_04de38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DE3C: move.l a1, (a6)
pub fn i_04de3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DE3E: rts
pub fn i_04de3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Bunker_04de40  ($04DE40, 234 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DE40: movea.l $3c(a6), a1
pub fn i_04de40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE44;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04DE44: jsr $2942a.l
pub fn i_04de44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE4A);
    cpu.pc = 0x2942A;
    16
}
/// $04DE4A: lea.l $e92ea.l, a1
pub fn i_04de4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE92EA;
    20
}
/// $04DE50: move.w #$75, d0
pub fn i_04de50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE54;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DE54: move.b #$0, $75(a6)
pub fn i_04de54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE5A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DE5A: jsr $4429e.l
pub fn i_04de5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE60);
    cpu.pc = 0x4429E;
    16
}
/// $04DE60: move.w #$70, $70(a6)
pub fn i_04de60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE66;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DE66: move.w #$3c, $66(a6)
pub fn i_04de66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DE6C: move.b #$0, $20(a6)
pub fn i_04de6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DE72: move.b #$0, $21(a6)
pub fn i_04de72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DE78: lea.l $294472.l, a0
pub fn i_04de78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294472;
    20
}
/// $04DE7E: jsr $28cd4.l
pub fn i_04de7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE84);
    cpu.pc = 0x28CD4;
    16
}
/// $04DE84: lea.l $4de8a(pc), a1
pub fn i_04de84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DE8A;
    16
}
/// $04DE88: move.l a1, (a6)
pub fn i_04de88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DE8A: jsr $2783a.l
pub fn i_04de8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE90);
    cpu.pc = 0x2783A;
    16
}
/// $04DE90: jsr $28d70.l
pub fn i_04de90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE96);
    cpu.pc = 0x28D70;
    16
}
/// $04DE96: jsr $2870a.l
pub fn i_04de96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE9C);
    cpu.pc = 0x2870A;
    16
}
/// $04DE9C: bcc.w $4deb2
pub fn i_04de9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DEB2; return 10; }
    8
}
/// $04DEA0: lea.l $5e766.l, a0
pub fn i_04dea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DEA6: jsr $5e770.l
pub fn i_04dea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEAC);
    cpu.pc = 0x5E770;
    16
}
/// $04DEAC: bclr.b #$3, $13(a6)
pub fn i_04deac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEB2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DEB2: jsr $28758.l
pub fn i_04deb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEB8);
    cpu.pc = 0x28758;
    16
}
/// $04DEB8: bcc.w $4df0a
pub fn i_04deb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DF0A; return 10; }
    8
}
/// $04DEBC: move.b #$ff, $75(a6)
pub fn i_04debc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DEC2: addi.b #$10, $21(a6)
pub fn i_04dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEC8;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DEC8: move.b #$ff, $20(a6)
pub fn i_04dec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DECE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DECE: addi.w #$10, $22(a6)
pub fn i_04dece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DED4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DED4: lea.l $295b5a.l, a1
pub fn i_04ded4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEDA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B5A;
    20
}
/// $04DEDA: jsr $77c7e.l
pub fn i_04deda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEE0);
    cpu.pc = 0x77C7E;
    16
}
/// $04DEE0: lea.l $295b6c.l, a1
pub fn i_04dee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B6C;
    20
}
/// $04DEE6: jsr $77c7e.l
pub fn i_04dee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEEC);
    cpu.pc = 0x77C7E;
    16
}
/// $04DEEC: lea.l $295cb0.l, a1
pub fn i_04deec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEF2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04DEF2: jsr $77c7e.l
pub fn i_04def2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEF8);
    cpu.pc = 0x77C7E;
    16
}
/// $04DEF8: addi.w #$20, $22(a0)
pub fn i_04def8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEFE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DEFE: addi.w #$20, $24(a0)
pub fn i_04defe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF04;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DF04: lea.l $4df30(pc), a1
pub fn i_04df04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DF30;
    16
}
/// $04DF08: move.l a1, (a6)
pub fn i_04df08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF0A: jsr $4fa70(pc)
pub fn i_04df0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF0E);
    cpu.pc = 0x4FA70;
    16
}
/// $04DF0E: bcc.w $4df1e
pub fn i_04df0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DF1E; return 10; }
    8
}
/// $04DF12: move.b #$ff, $20(a6)
pub fn i_04df12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DF18: lea.l $4df72(pc), a1
pub fn i_04df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DF72;
    16
}
/// $04DF1C: move.l a1, (a6)
pub fn i_04df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF1E: move.b $74(a6), d0
pub fn i_04df1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF22;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04DF22: cmp.b $21(a6), d0
pub fn i_04df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF26;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04DF26: beq.w $4df2e
pub fn i_04df26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4DF2E; return 10; }
    8
}
// ===== JsrPcThunk_04df2a  ($04DF2A, 6 B, thunks/jsr_pc_thunks.c)
/// $04DF2A: jsr $4faf8(pc)
pub fn i_04df2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF2E);
    cpu.pc = 0x4FAF8;
    16
}
/// $04DF2E: rts
pub fn i_04df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BunkerWreck_04df30  ($04DF30, 98 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DF30: move.l #$1000, d0
pub fn i_04df30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF36;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DF36: jsr $51a28.l
pub fn i_04df36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DF3C);
    cpu.pc = 0x51A28;
    16
}
/// $04DF3C: lea.l $ffff.w, a0
pub fn i_04df3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF40;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DF40: move.l a0, $48(a6)
pub fn i_04df40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF44;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DF44: move.w #$1030, d0
pub fn i_04df44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DF48: jsr $2352.l
pub fn i_04df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DF4E);
    cpu.pc = 0x2352;
    16
}
/// $04DF4E: cmpi.w #$0, $22(a6)
pub fn i_04df4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF54;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04DF54: bmi.w $4df62
pub fn i_04df54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF58;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4DF62; return 10; }
    8
}
/// $04DF58: lea.l $29561e.l, a2
pub fn i_04df58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF5E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29561E;
    20
}
/// $04DF5E: bra.w $4df68
pub fn i_04df5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF62;
    cpu.ir = 0x6000;
    cpu.pc = 0x4DF68;
    10
}
/// $04DF62: lea.l $295632.l, a2
pub fn i_04df62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF68;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295632;
    20
}
/// $04DF68: jsr $5022a(pc)
pub fn i_04df68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF6C);
    cpu.pc = 0x5022A;
    16
}
/// $04DF6C: lea.l $4df72(pc), a1
pub fn i_04df6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DF72;
    16
}
/// $04DF70: move.l a1, (a6)
pub fn i_04df70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF72: jsr $2783a.l
pub fn i_04df72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DF78);
    cpu.pc = 0x2783A;
    16
}
/// $04DF78: jsr $4fa70(pc)
pub fn i_04df78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF7C);
    cpu.pc = 0x4FA70;
    16
}
/// $04DF7C: bcc.w $4df86
pub fn i_04df7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DF86; return 10; }
    8
}
/// $04DF80: lea.l $4f2a4(pc), a1
pub fn i_04df80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DF84: move.l a1, (a6)
pub fn i_04df84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF86: move.b $74(a6), d0
pub fn i_04df86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF8A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04DF8A: cmp.b $21(a6), d0
pub fn i_04df8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF8E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04DF8E: beq.w $4df96
pub fn i_04df8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4DF96; return 10; }
    8
}
// ===== JsrPcThunk_04df92  ($04DF92, 6 B, thunks/jsr_pc_thunks.c)
/// $04DF92: jsr $4faf8(pc)
pub fn i_04df92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF96);
    cpu.pc = 0x4FAF8;
    16
}
/// $04DF96: rts
pub fn i_04df96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Bridge_04df98  ($04DF98, 198 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DF98: movea.l $3c(a6), a1
pub fn i_04df98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF9C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04DF9C: jsr $2942a.l
pub fn i_04df9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFA2);
    cpu.pc = 0x2942A;
    16
}
/// $04DFA2: lea.l $4e12c(pc), a1
pub fn i_04dfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E12C;
    16
}
/// $04DFA6: jsr $4ae.l
pub fn i_04dfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFAC);
    cpu.pc = 0x4AE;
    16
}
/// $04DFAC: jsr $5dd22.l
pub fn i_04dfac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFB2);
    cpu.pc = 0x5DD22;
    16
}
/// $04DFB2: lea.l $4e12c(pc), a1
pub fn i_04dfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E12C;
    16
}
/// $04DFB6: jsr $4ae.l
pub fn i_04dfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFBC);
    cpu.pc = 0x4AE;
    16
}
/// $04DFBC: jsr $5dd22.l
pub fn i_04dfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFC2);
    cpu.pc = 0x5DD22;
    16
}
/// $04DFC2: addi.w #$d0, $22(a0)
pub fn i_04dfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFC8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xD0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DFC8: ori.b #$1, $3a(a0)
pub fn i_04dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFCE;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DFCE: move.w #$60, $70(a6)
pub fn i_04dfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFD4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DFD4: move.b #$0, $21(a6)
pub fn i_04dfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DFDA: lea.l $e9310.l, a1
pub fn i_04dfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFE0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE9310;
    20
}
/// $04DFE0: move.w #$75, d0
pub fn i_04dfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DFE4: move.b #$0, $75(a6)
pub fn i_04dfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DFEA: jsr $4429e.l
pub fn i_04dfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFF0);
    cpu.pc = 0x4429E;
    16
}
/// $04DFF0: lea.l $4dff6(pc), a1
pub fn i_04dff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DFF6;
    16
}
/// $04DFF4: move.l a1, (a6)
pub fn i_04dff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DFF6: jsr $2783a.l
pub fn i_04dff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFFC);
    cpu.pc = 0x2783A;
    16
}
/// $04DFFC: cmpi.b #$11, $21(a6)
pub fn i_04dffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E002;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Byte, false);
    16
}
/// $04E002: bne.w $4e04e
pub fn i_04e002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E006;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4E04E; return 10; }
    8
}
/// $04E006: move.w #$1030, d0
pub fn i_04e006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E00A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E00A: jsr $2352.l
pub fn i_04e00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E010);
    cpu.pc = 0x2352;
    16
}
/// $04E010: lea.l $295bea.l, a1
pub fn i_04e010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E016;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BEA;
    20
}
/// $04E016: jsr $77c7e.l
pub fn i_04e016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E01C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E01C);
    cpu.pc = 0x77C7E;
    16
}
/// $04E01C: lea.l $295bfc.l, a1
pub fn i_04e01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E022;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BFC;
    20
}
/// $04E022: jsr $77c7e.l
pub fn i_04e022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E028;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E028);
    cpu.pc = 0x77C7E;
    16
}
/// $04E028: lea.l $295cc2.l, a1
pub fn i_04e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E02E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CC2;
    20
}
/// $04E02E: jsr $77c7e.l
pub fn i_04e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E034);
    cpu.pc = 0x77C7E;
    16
}
/// $04E034: lea.l $2956aa.l, a2
pub fn i_04e034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E03A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956AA;
    20
}
/// $04E03A: jsr $5022a(pc)
pub fn i_04e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E03E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E03E);
    cpu.pc = 0x5022A;
    16
}
/// $04E03E: lea.l $2956e6.l, a2
pub fn i_04e03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E044;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956E6;
    20
}
/// $04E044: jsr $5022a(pc)
pub fn i_04e044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E048;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E048);
    cpu.pc = 0x5022A;
    16
}
/// $04E048: lea.l $4e05e(pc), a1
pub fn i_04e048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E04C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E05E;
    16
}
/// $04E04C: move.l a1, (a6)
pub fn i_04e04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E04E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E04E: jsr $4fa70(pc)
pub fn i_04e04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E052;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E052);
    cpu.pc = 0x4FA70;
    16
}
/// $04E052: bcc.w $4e05c
pub fn i_04e052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E056;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E05C; return 10; }
    8
}
/// $04E056: jmp $518.l
pub fn i_04e056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E05C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E05C: rts
pub fn i_04e05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E05E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BridgeCollapse_04e05e  ($04E05E, 206 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E05E: move.w #$3c, $66(a6)
pub fn i_04e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E064;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E064: lea.l $294488.l, a0
pub fn i_04e064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E06A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294488;
    20
}
/// $04E06A: jsr $28cd4.l
pub fn i_04e06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E070;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E070);
    cpu.pc = 0x28CD4;
    16
}
/// $04E070: lea.l $4e076(pc), a1
pub fn i_04e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E074;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E076;
    16
}
/// $04E074: move.l a1, (a6)
pub fn i_04e074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E076;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E076: jsr $2783a.l
pub fn i_04e076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E07C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E07C);
    cpu.pc = 0x2783A;
    16
}
/// $04E07C: jsr $28d70.l
pub fn i_04e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E082);
    cpu.pc = 0x28D70;
    16
}
/// $04E082: jsr $2870a.l
pub fn i_04e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E088);
    cpu.pc = 0x2870A;
    16
}
/// $04E088: bcc.w $4e09e
pub fn i_04e088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E08C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E09E; return 10; }
    8
}
/// $04E08C: lea.l $5e766.l, a0
pub fn i_04e08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E092;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E092: jsr $5e770.l
pub fn i_04e092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E098);
    cpu.pc = 0x5E770;
    16
}
/// $04E098: bclr.b #$3, $13(a6)
pub fn i_04e098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E09E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E09E: jsr $28758.l
pub fn i_04e09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0A4);
    cpu.pc = 0x28758;
    16
}
/// $04E0A4: bcc.w $4e116
pub fn i_04e0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E116; return 10; }
    8
}
/// $04E0A8: lea.l $ffff.w, a0
pub fn i_04e0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0AC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E0AC: move.l a0, $48(a6)
pub fn i_04e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E0B0: move.l #$5000, d0
pub fn i_04e0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0B6;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E0B6: jsr $51a28.l
pub fn i_04e0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0BC);
    cpu.pc = 0x51A28;
    16
}
/// $04E0BC: move.w #$1028, d0
pub fn i_04e0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E0C0: jsr $2352.l
pub fn i_04e0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0C6);
    cpu.pc = 0x2352;
    16
}
/// $04E0C6: lea.l $295c0e.l, a1
pub fn i_04e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C0E;
    20
}
/// $04E0CC: jsr $77c7e.l
pub fn i_04e0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0D2);
    cpu.pc = 0x77C7E;
    16
}
/// $04E0D2: lea.l $295c20.l, a1
pub fn i_04e0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C20;
    20
}
/// $04E0D8: jsr $77c7e.l
pub fn i_04e0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0DE);
    cpu.pc = 0x77C7E;
    16
}
/// $04E0DE: lea.l $295cc2.l, a1
pub fn i_04e0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CC2;
    20
}
/// $04E0E4: jsr $77c7e.l
pub fn i_04e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0EA);
    cpu.pc = 0x77C7E;
    16
}
/// $04E0EA: lea.l $2956be.l, a2
pub fn i_04e0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0F0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956BE;
    20
}
/// $04E0F0: jsr $5022a(pc)
pub fn i_04e0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E0F4);
    cpu.pc = 0x5022A;
    16
}
/// $04E0F4: lea.l $2956fa.l, a2
pub fn i_04e0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0FA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956FA;
    20
}
/// $04E0FA: jsr $5022a(pc)
pub fn i_04e0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E0FE);
    cpu.pc = 0x5022A;
    16
}
/// $04E0FE: lea.l $295722.l, a2
pub fn i_04e0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E104;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295722;
    20
}
/// $04E104: jsr $5022a(pc)
pub fn i_04e104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E108);
    cpu.pc = 0x5022A;
    16
}
/// $04E108: lea.l $29574a.l, a2
pub fn i_04e108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E10E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29574A;
    20
}
/// $04E10E: jsr $5022a(pc)
pub fn i_04e10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E112);
    cpu.pc = 0x5022A;
    16
}
/// $04E112: bra.w $4e11e
pub fn i_04e112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E116;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E11E;
    10
}
/// $04E116: jsr $4fa70(pc)
pub fn i_04e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E11A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E11A);
    cpu.pc = 0x4FA70;
    16
}
/// $04E11A: bcc.w $4e12a
pub fn i_04e11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E11E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E12A; return 10; }
    8
}
/// $04E11E: move.b #$ff, $75(a6)
pub fn i_04e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E124;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E124: jmp $518.l
pub fn i_04e124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E12A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E12A: rts
pub fn i_04e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E12C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BridgePillar_04e12c  ($04E12C, 284 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E12C: lea.l $5f38a.l, a1
pub fn i_04e12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E132;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5F38A;
    20
}
/// $04E132: jsr $4ae.l
pub fn i_04e132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E138;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E138);
    cpu.pc = 0x4AE;
    16
}
/// $04E138: jsr $5dd22.l
pub fn i_04e138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E13E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E13E);
    cpu.pc = 0x5DD22;
    16
}
/// $04E13E: btst.b #$0, $3a(a6)
pub fn i_04e13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E144;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04E144: bne.w $4e158
pub fn i_04e144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E148;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4E158; return 10; }
    8
}
/// $04E148: addi.w #$2b, $22(a0)
pub fn i_04e148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E14E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2B, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E14E: subi.w #$17, $24(a0)
pub fn i_04e14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E154;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x17, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E154: bra.w $4e164
pub fn i_04e154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E158;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E164;
    10
}
/// $04E158: subi.w #$14, $22(a0)
pub fn i_04e158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E15E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E15E: subi.w #$17, $24(a0)
pub fn i_04e15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E164;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x17, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E164: move.b #$0, $21(a6)
pub fn i_04e164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E16A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E16A: move.w #$3c, $66(a6)
pub fn i_04e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E170;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E170: move.w #$70, $70(a6)
pub fn i_04e170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E176;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E176: lea.l $29449e.l, a0
pub fn i_04e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E17C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29449E;
    20
}
/// $04E17C: jsr $28cd4.l
pub fn i_04e17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E182);
    cpu.pc = 0x28CD4;
    16
}
/// $04E182: lea.l $4e188(pc), a1
pub fn i_04e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E186;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E188;
    16
}
/// $04E186: move.l a1, (a6)
pub fn i_04e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E188;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E188: jsr $2783a.l
pub fn i_04e188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E18E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E18E);
    cpu.pc = 0x2783A;
    16
}
/// $04E18E: jsr $28d70.l
pub fn i_04e18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E194);
    cpu.pc = 0x28D70;
    16
}
/// $04E194: jsr $2870a.l
pub fn i_04e194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E19A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E19A);
    cpu.pc = 0x2870A;
    16
}
/// $04E19A: bcc.w $4e1b0
pub fn i_04e19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E19E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E1B0; return 10; }
    8
}
/// $04E19E: lea.l $5e766.l, a0
pub fn i_04e19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E1A4: jsr $5e770.l
pub fn i_04e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1AA);
    cpu.pc = 0x5E770;
    16
}
/// $04E1AA: bclr.b #$3, $13(a6)
pub fn i_04e1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1B0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E1B0: jsr $28758.l
pub fn i_04e1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1B6);
    cpu.pc = 0x28758;
    16
}
/// $04E1B6: bcc.w $4e238
pub fn i_04e1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E238; return 10; }
    8
}
/// $04E1BA: move.w #$102f, d0
pub fn i_04e1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1BE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E1BE: jsr $2352.l
pub fn i_04e1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1C4);
    cpu.pc = 0x2352;
    16
}
/// $04E1C4: lea.l $ffff.w, a0
pub fn i_04e1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1C8;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E1C8: move.l a0, $48(a6)
pub fn i_04e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1CC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E1CC: lea.l $295b7e.l, a1
pub fn i_04e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B7E;
    20
}
/// $04E1D2: jsr $77c7e.l
pub fn i_04e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1D8);
    cpu.pc = 0x77C7E;
    16
}
/// $04E1D8: subi.w #$20, $24(a0)
pub fn i_04e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1DE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E1DE: lea.l $295b90.l, a1
pub fn i_04e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B90;
    20
}
/// $04E1E4: jsr $77c7e.l
pub fn i_04e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1EA);
    cpu.pc = 0x77C7E;
    16
}
/// $04E1EA: subi.w #$20, $24(a0)
pub fn i_04e1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1F0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E1F0: lea.l $295cb0.l, a1
pub fn i_04e1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04E1F6: jsr $77c7e.l
pub fn i_04e1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1FC);
    cpu.pc = 0x77C7E;
    16
}
/// $04E1FC: move.w #$20, d0
pub fn i_04e1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E200;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E200: btst.b #$0, $3a(a6)
pub fn i_04e200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E206;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04E206: beq.w $4e20c
pub fn i_04e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E20A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4E20C; return 10; }
    8
}
/// $04E20A: neg.w d0
pub fn i_04e20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E20C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04E20C: add.w d0, $22(a0)
pub fn i_04e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E210;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04E210: jsr $4fb3c(pc)
pub fn i_04e210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E214;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E214);
    cpu.pc = 0x4FB3C;
    16
}
/// $04E214: movea.l $c(a6), a0
pub fn i_04e214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E218;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04E218: move.b #$10, d0
pub fn i_04e218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E21C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04E21C: btst.b #$0, $3a(a6)
pub fn i_04e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E222;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04E222: beq.w $4e22a
pub fn i_04e222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E226;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4E22A; return 10; }
    8
}
/// $04E226: move.b #$1, d0
pub fn i_04e226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E22A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04E22A: or.b d0, $21(a0)
pub fn i_04e22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E22E;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04E22E: move.b #$ff, $21(a6)
pub fn i_04e22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E234;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E234: bra.w $4e240
pub fn i_04e234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E238;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E240;
    10
}
/// $04E238: jsr $4fa70(pc)
pub fn i_04e238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E23C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E23C);
    cpu.pc = 0x4FA70;
    16
}
/// $04E23C: bcc.w $4e246
pub fn i_04e23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E240;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E246; return 10; }
    8
}
/// $04E240: jmp $518.l
pub fn i_04e240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E246;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E246: rts
pub fn i_04e246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E248;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Nest_04e248  ($04E248, 96 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E248: movea.l $3c(a6), a1
pub fn i_04e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E24C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04E24C: jsr $2942a.l
pub fn i_04e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E252);
    cpu.pc = 0x2942A;
    16
}
/// $04E252: move.w #$40, d1
pub fn i_04e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E256;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E256: jsr $236e.l
pub fn i_04e256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E25C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E25C);
    cpu.pc = 0x236E;
    16
}
/// $04E25C: move.w #$70, $70(a6)
pub fn i_04e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E262;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E262: move.w #$12c, $66(a6)
pub fn i_04e262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E268;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E268: lea.l $29441a.l, a0
pub fn i_04e268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E26E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29441A;
    20
}
/// $04E26E: jsr $28cd4.l
pub fn i_04e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E274);
    cpu.pc = 0x28CD4;
    16
}
/// $04E274: lea.l $8e738.l, a1
pub fn i_04e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E27A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x8E738;
    20
}
/// $04E27A: jsr $4ae.l
pub fn i_04e27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E280);
    cpu.pc = 0x4AE;
    16
}
/// $04E280: jsr $5dd22.l
pub fn i_04e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E286);
    cpu.pc = 0x5DD22;
    16
}
/// $04E286: addi.w #$98, $22(a0)
pub fn i_04e286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E28C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x98, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E28C: addi.w #$38, $24(a0)
pub fn i_04e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E292;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E292: lea.l $4e298(pc), a1
pub fn i_04e292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E296;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E298;
    16
}
/// $04E296: move.l a1, (a6)
pub fn i_04e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E298;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E298: cmpi.w #$110, $22(a6)
pub fn i_04e298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E29E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $04E29E: blt.w $4e2b0
pub fn i_04e29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2A2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4E2B0; return 10; }
    8
}
/// $04E2A2: jsr $2783a.l
pub fn i_04e2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2A8);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_04e2a8  ($04E2A8, 8 B, thunks/jsr_abs_thunks.c)
/// $04E2A8: jsr $28d70.l
pub fn i_04e2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2AE);
    cpu.pc = 0x28D70;
    16
}
/// $04E2AE: rts
pub fn i_04e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NestActive_04e2b0  ($04E2B0, 124 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E2B0: lea.l $4e2b6(pc), a1
pub fn i_04e2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E2B6;
    16
}
/// $04E2B4: move.l a1, (a6)
pub fn i_04e2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E2B6: jsr $2783a.l
pub fn i_04e2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2BC);
    cpu.pc = 0x2783A;
    16
}
/// $04E2BC: jsr $28d70.l
pub fn i_04e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2C2);
    cpu.pc = 0x28D70;
    16
}
/// $04E2C2: jsr $2870a.l
pub fn i_04e2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2C8);
    cpu.pc = 0x2870A;
    16
}
/// $04E2C8: bcc.w $4e2de
pub fn i_04e2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E2DE; return 10; }
    8
}
/// $04E2CC: lea.l $5e766.l, a0
pub fn i_04e2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E2D2: jsr $5e770.l
pub fn i_04e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2D8);
    cpu.pc = 0x5E770;
    16
}
/// $04E2D8: bclr.b #$3, $13(a6)
pub fn i_04e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2DE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E2DE: jsr $28758.l
pub fn i_04e2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2E4);
    cpu.pc = 0x28758;
    16
}
/// $04E2E4: bcc.w $4e31c
pub fn i_04e2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E31C; return 10; }
    8
}
/// $04E2E8: move.w #$102f, d0
pub fn i_04e2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E2EC: jsr $2352.l
pub fn i_04e2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2F2);
    cpu.pc = 0x2352;
    16
}
/// $04E2F2: lea.l $295c56.l, a1
pub fn i_04e2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2F8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C56;
    20
}
/// $04E2F8: jsr $77c7e.l
pub fn i_04e2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2FE);
    cpu.pc = 0x77C7E;
    16
}
/// $04E2FE: lea.l $295c68.l, a1
pub fn i_04e2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E304;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C68;
    20
}
/// $04E304: jsr $77c7e.l
pub fn i_04e304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E30A);
    cpu.pc = 0x77C7E;
    16
}
/// $04E30A: lea.l $295ce6.l, a1
pub fn i_04e30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E310;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CE6;
    20
}
/// $04E310: jsr $77c7e.l
pub fn i_04e310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E316);
    cpu.pc = 0x77C7E;
    16
}
/// $04E316: lea.l $4e32c(pc), a1
pub fn i_04e316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E31A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E32C;
    16
}
/// $04E31A: move.l a1, (a6)
pub fn i_04e31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E31C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E31C: jsr $4fa70(pc)
pub fn i_04e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E320;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E320);
    cpu.pc = 0x4FA70;
    16
}
/// $04E320: bcc.w $4e32a
pub fn i_04e320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E324;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E32A; return 10; }
    8
}
/// $04E324: jmp $518.l
pub fn i_04e324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E32A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E32A: rts
pub fn i_04e32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E32C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NestWreck_04e32c  ($04E32C, 94 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E32C: move.l #$3000, d0
pub fn i_04e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E332;
    cpu.ir = 0x203C;
    let v1: u32 = 0x3000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E332: jsr $51a28.l
pub fn i_04e332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E338);
    cpu.pc = 0x51A28;
    16
}
/// $04E338: lea.l $ffff.w, a0
pub fn i_04e338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E33C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E33C: move.l a0, $48(a6)
pub fn i_04e33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E340;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E340: jsr $2783a.l
pub fn i_04e340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E346);
    cpu.pc = 0x2783A;
    16
}
/// $04E346: lea.l $295d78.l, a1
pub fn i_04e346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E34C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295D78;
    20
}
/// $04E34C: jsr $43fac.l
pub fn i_04e34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E352);
    cpu.pc = 0x43FAC;
    16
}
/// $04E352: move.w #$1030, d0
pub fn i_04e352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E356;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E356: jsr $2352.l
pub fn i_04e356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E35C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E35C);
    cpu.pc = 0x2352;
    16
}
/// $04E35C: lea.l $294430.l, a0
pub fn i_04e35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E362;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294430;
    20
}
/// $04E362: jsr $28cd4.l
pub fn i_04e362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E368);
    cpu.pc = 0x28CD4;
    16
}
/// $04E368: lea.l $4e36e(pc), a1
pub fn i_04e368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E36C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E36E;
    16
}
/// $04E36C: move.l a1, (a6)
pub fn i_04e36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E36E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E36E: jsr $2783a.l
pub fn i_04e36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E374);
    cpu.pc = 0x2783A;
    16
}
/// $04E374: jsr $28d70.l
pub fn i_04e374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E37A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E37A);
    cpu.pc = 0x28D70;
    16
}
/// $04E37A: jsr $4fa70(pc)
pub fn i_04e37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E37E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E37E);
    cpu.pc = 0x4FA70;
    16
}
/// $04E37E: bcc.w $4e388
pub fn i_04e37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E382;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E388; return 10; }
    8
}
/// $04E382: jmp $518.l
pub fn i_04e382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E388;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E388: rts
pub fn i_04e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E38A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Shed_04e38a  ($04E38A, 198 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E38A: movea.l $3c(a6), a1
pub fn i_04e38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E38E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04E38E: jsr $2942a.l
pub fn i_04e38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E394);
    cpu.pc = 0x2942A;
    16
}
/// $04E394: lea.l $4f2c2(pc), a1
pub fn i_04e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E398;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2C2;
    16
}
/// $04E398: jsr $4ae.l
pub fn i_04e398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E39E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E39E);
    cpu.pc = 0x4AE;
    16
}
/// $04E39E: jsr $5dd22.l
pub fn i_04e39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E3A4);
    cpu.pc = 0x5DD22;
    16
}
/// $04E3A4: addi.w #$40, $22(a0)
pub fn i_04e3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E3AA: addi.w #$49, $24(a0)
pub fn i_04e3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3B0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x49, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E3B0: move.w #$70, $70(a6)
pub fn i_04e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E3B6: move.w #$3c, d0
pub fn i_04e3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E3BA: asr.w #$1, d0
pub fn i_04e3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3BC;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04E3BC: move.w d0, $66(a6)
pub fn i_04e3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E3C0: lea.l $2944b4.l, a0
pub fn i_04e3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2944B4;
    20
}
/// $04E3C6: jsr $28cd4.l
pub fn i_04e3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E3CC);
    cpu.pc = 0x28CD4;
    16
}
/// $04E3CC: lea.l $4e3d2(pc), a1
pub fn i_04e3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E3D2;
    16
}
/// $04E3D0: move.l a1, (a6)
pub fn i_04e3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E3D2: jsr $2783a.l
pub fn i_04e3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E3D8);
    cpu.pc = 0x2783A;
    16
}
/// $04E3D8: jsr $28d70.l
pub fn i_04e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E3DE);
    cpu.pc = 0x28D70;
    16
}
/// $04E3DE: jsr $2870a.l
pub fn i_04e3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E3E4);
    cpu.pc = 0x2870A;
    16
}
/// $04E3E4: bcc.w $4e3fa
pub fn i_04e3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E3FA; return 10; }
    8
}
/// $04E3E8: lea.l $5e766.l, a0
pub fn i_04e3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E3EE: jsr $5e770.l
pub fn i_04e3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E3F4);
    cpu.pc = 0x5E770;
    16
}
/// $04E3F4: bclr.b #$3, $13(a6)
pub fn i_04e3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3FA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E3FA: jsr $28758.l
pub fn i_04e3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E400);
    cpu.pc = 0x28758;
    16
}
/// $04E400: bcc.w $4e440
pub fn i_04e400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E404;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E440; return 10; }
    8
}
/// $04E404: bclr.b #$0, $13(a6)
pub fn i_04e404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E40A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E40A: lea.l $295ba2.l, a1
pub fn i_04e40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E410;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BA2;
    20
}
/// $04E410: jsr $77c7e.l
pub fn i_04e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E416);
    cpu.pc = 0x77C7E;
    16
}
/// $04E416: addi.w #$20, $22(a0)
pub fn i_04e416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E41C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E41C: addi.w #$30, $24(a0)
pub fn i_04e41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E422;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E422: lea.l $295cb0.l, a1
pub fn i_04e422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E428;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04E428: jsr $77c7e.l
pub fn i_04e428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E42E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E42E);
    cpu.pc = 0x77C7E;
    16
}
/// $04E42E: addi.w #$20, $22(a0)
pub fn i_04e42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E434;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E434: addi.w #$30, $24(a0)
pub fn i_04e434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E43A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E43A: lea.l $4e450(pc), a1
pub fn i_04e43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E43E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E450;
    16
}
/// $04E43E: move.l a1, (a6)
pub fn i_04e43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E440;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E440: jsr $4fa70(pc)
pub fn i_04e440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E444;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E444);
    cpu.pc = 0x4FA70;
    16
}
/// $04E444: bcc.w $4e44e
pub fn i_04e444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E448;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E44E; return 10; }
    8
}
/// $04E448: jmp $518.l
pub fn i_04e448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E44E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E44E: rts
pub fn i_04e44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_ShedDamaged_04e450  ($04E450, 186 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E450: move.w #$1028, d0
pub fn i_04e450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E454;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E454: jsr $2352.l
pub fn i_04e454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E45A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E45A);
    cpu.pc = 0x2352;
    16
}
/// $04E45A: lea.l $295786.l, a2
pub fn i_04e45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E460;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295786;
    20
}
/// $04E460: jsr $5022a(pc)
pub fn i_04e460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E464;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E464);
    cpu.pc = 0x5022A;
    16
}
/// $04E464: move.w #$3c, d0
pub fn i_04e464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E468;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E468: asr.w #$1, d0
pub fn i_04e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E46A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04E46A: move.w d0, $66(a6)
pub fn i_04e46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E46E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E46E: lea.l $4e474(pc), a1
pub fn i_04e46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E472;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E474;
    16
}
/// $04E472: move.l a1, (a6)
pub fn i_04e472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E474;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E474: jsr $2783a.l
pub fn i_04e474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E47A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E47A);
    cpu.pc = 0x2783A;
    16
}
/// $04E47A: jsr $28d70.l
pub fn i_04e47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E480;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E480);
    cpu.pc = 0x28D70;
    16
}
/// $04E480: jsr $2870a.l
pub fn i_04e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E486);
    cpu.pc = 0x2870A;
    16
}
/// $04E486: bcc.w $4e49c
pub fn i_04e486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E48A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E49C; return 10; }
    8
}
/// $04E48A: lea.l $5e766.l, a0
pub fn i_04e48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E490;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E490: jsr $5e770.l
pub fn i_04e490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E496);
    cpu.pc = 0x5E770;
    16
}
/// $04E496: bclr.b #$3, $13(a6)
pub fn i_04e496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E49C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E49C: jsr $28758.l
pub fn i_04e49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E4A2);
    cpu.pc = 0x28758;
    16
}
/// $04E4A2: bcc.w $4e502
pub fn i_04e4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E502; return 10; }
    8
}
/// $04E4A6: move.l #$1000, d0
pub fn i_04e4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4AC;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E4AC: jsr $51a28.l
pub fn i_04e4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E4B2);
    cpu.pc = 0x51A28;
    16
}
/// $04E4B2: move.w #$1030, d0
pub fn i_04e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E4B6: jsr $2352.l
pub fn i_04e4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E4BC);
    cpu.pc = 0x2352;
    16
}
/// $04E4BC: lea.l $ffff.w, a0
pub fn i_04e4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4C0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E4C0: move.l a0, $48(a6)
pub fn i_04e4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E4C4: lea.l $295ba2.l, a1
pub fn i_04e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4CA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BA2;
    20
}
/// $04E4CA: jsr $77c7e.l
pub fn i_04e4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E4D0);
    cpu.pc = 0x77C7E;
    16
}
/// $04E4D0: addi.w #$20, $22(a0)
pub fn i_04e4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4D6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E4D6: addi.w #$30, $24(a0)
pub fn i_04e4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4DC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E4DC: lea.l $295cb0.l, a1
pub fn i_04e4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04E4E2: jsr $77c7e.l
pub fn i_04e4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E4E8);
    cpu.pc = 0x77C7E;
    16
}
/// $04E4E8: addi.w #$20, $22(a0)
pub fn i_04e4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4EE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E4EE: addi.w #$30, $24(a0)
pub fn i_04e4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4F4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E4F4: lea.l $29579a.l, a2
pub fn i_04e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4FA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29579A;
    20
}
/// $04E4FA: jsr $5022a(pc)
pub fn i_04e4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E4FE);
    cpu.pc = 0x5022A;
    16
}
/// $04E4FE: jsr $4e50a(pc)
pub fn i_04e4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E502;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E502);
    cpu.pc = 0x4E50A;
    16
}
/// $04E502: jsr $4fa70(pc)
pub fn i_04e502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E506;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E506);
    cpu.pc = 0x4FA70;
    16
}
/// $04E506: bcc.w $4e510
pub fn i_04e506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E50A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E510; return 10; }
    8
}
// ===== Prop_FreeOrRts_04e50a  ($04E50A, 8 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E50A: jmp $518.l
pub fn i_04e50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E510;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E510: rts
pub fn i_04e510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E512;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Barrier_04e512  ($04E512, 102 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E512: movea.l $3c(a6), a1
pub fn i_04e512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E516;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04E516: jsr $2942a.l
pub fn i_04e516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E51C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E51C);
    cpu.pc = 0x2942A;
    16
}
/// $04E51C: move.w #$40, d1
pub fn i_04e51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E520;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E520: jsr $236e.l
pub fn i_04e520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E526);
    cpu.pc = 0x236E;
    16
}
/// $04E526: lea.l $2944ca.l, a0
pub fn i_04e526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E52C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2944CA;
    20
}
/// $04E52C: jsr $28cd4.l
pub fn i_04e52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E532);
    cpu.pc = 0x28CD4;
    16
}
/// $04E532: bset.b #$6, $12(a6)
pub fn i_04e532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E538;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E538: move.l #$296272, $60(a6)
pub fn i_04e538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E540;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x296272;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E540: lea.l $4ed90(pc), a1
pub fn i_04e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4ED90;
    16
}
/// $04E544: jsr $4ae.l
pub fn i_04e544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E54A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E54A);
    cpu.pc = 0x4AE;
    16
}
/// $04E54A: jsr $5dd22.l
pub fn i_04e54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E550);
    cpu.pc = 0x5DD22;
    16
}
/// $04E550: addi.w #$10, $24(a0)
pub fn i_04e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E556;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E556: lea.l $4e55c(pc), a1
pub fn i_04e556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E55A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E55C;
    16
}
/// $04E55A: move.l a1, (a6)
pub fn i_04e55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E55C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E55C: jsr $2783a.l
pub fn i_04e55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E562;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E562);
    cpu.pc = 0x2783A;
    16
}
/// $04E562: move.l $106f50.l, d0
pub fn i_04e562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E568;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04E568: swap d0
pub fn i_04e568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E56A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E56A: cmpi.w #$a10, d0
pub fn i_04e56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E56E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA10, Size::Word, false);
    8
}
/// $04E56E: bgt.w $4e580
pub fn i_04e56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E572;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4E580; return 10; }
    8
}
/// $04E572: jsr $28998.l
pub fn i_04e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E578);
    cpu.pc = 0x28998;
    16
}
// ===== JsrAbsThunk_04e578  ($04E578, 8 B, thunks/jsr_abs_thunks.c)
/// $04E578: jsr $28d70.l
pub fn i_04e578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E57E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E57E);
    cpu.pc = 0x28D70;
    16
}
/// $04E57E: rts
pub fn i_04e57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E580;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BarrierActive_04e580  ($04E580, 266 B, props/fortress_04e580.s)
/// $04E580: lea.l $294e7c.l, a0
pub fn i_04e580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E586;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294E7C;
    20
}
/// $04E586: move.l a0, $48(a6)
pub fn i_04e586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E58A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E58A: move.w #$80, $70(a6)
pub fn i_04e58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E590;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E590: move.w #$64, $66(a6)
pub fn i_04e590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E596;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E596: move.b #$0, $21(a6)
pub fn i_04e596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E59C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E59C: move.b #$0, $20(a6)
pub fn i_04e59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E5A2: jsr $4fb8a(pc)
pub fn i_04e5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E5A6);
    cpu.pc = 0x4FB8A;
    16
}
/// $04E5A6: jsr $4fbe0(pc)
pub fn i_04e5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E5AA);
    cpu.pc = 0x4FBE0;
    16
}
/// $04E5AA: jsr $4fc36(pc)
pub fn i_04e5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E5AE);
    cpu.pc = 0x4FC36;
    16
}
/// $04E5AE: jsr $4fc8c(pc)
pub fn i_04e5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E5B2);
    cpu.pc = 0x4FC8C;
    16
}
/// $04E5B2: jsr $4fcd8(pc)
pub fn i_04e5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E5B6);
    cpu.pc = 0x4FCD8;
    16
}
/// $04E5B6: jsr $4fd2c(pc)
pub fn i_04e5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E5BA);
    cpu.pc = 0x4FD2C;
    16
}
/// $04E5BA: lea.l $4ee0a(pc), a1
pub fn i_04e5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EE0A;
    16
}
/// $04E5BE: jsr $4ae.l
pub fn i_04e5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E5C4);
    cpu.pc = 0x4AE;
    16
}
/// $04E5C4: jsr $5dd22.l
pub fn i_04e5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E5CA);
    cpu.pc = 0x5DD22;
    16
}
/// $04E5CA: addi.w #$20, $22(a0)
pub fn i_04e5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5D0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E5D0: addi.w #$20, $24(a0)
pub fn i_04e5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5D6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E5D6: lea.l $e9348.l, a1
pub fn i_04e5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5DC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE9348;
    20
}
/// $04E5DC: move.w #$75, d0
pub fn i_04e5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E5E0: move.b #$0, $75(a6)
pub fn i_04e5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E5E6: jsr $4429e.l
pub fn i_04e5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E5EC);
    cpu.pc = 0x4429E;
    16
}
/// $04E5EC: lea.l $4e5f2(pc), a1
pub fn i_04e5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E5F2;
    16
}
/// $04E5F0: move.l a1, (a6)
pub fn i_04e5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E5F2: clr.b $10e39a.l
pub fn i_04e5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5F8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04E5F8: jsr $2783a.l
pub fn i_04e5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E5FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E5FE);
    cpu.pc = 0x2783A;
    16
}
/// $04E5FE: jsr $28998.l
pub fn i_04e5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E604);
    cpu.pc = 0x28998;
    16
}
/// $04E604: jsr $28d70.l
pub fn i_04e604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E60A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E60A);
    cpu.pc = 0x28D70;
    16
}
/// $04E60A: jsr $2870a.l
pub fn i_04e60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E610;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E610);
    cpu.pc = 0x2870A;
    16
}
/// $04E610: bcc.w $4e626
pub fn i_04e610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E614;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E626; return 10; }
    8
}
/// $04E614: lea.l $5e766.l, a0
pub fn i_04e614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E61A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E61A: jsr $5e770.l
pub fn i_04e61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E620);
    cpu.pc = 0x5E770;
    16
}
/// $04E620: bclr.b #$3, $13(a6)
pub fn i_04e620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E626;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E626: jsr $28758.l
pub fn i_04e626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E62C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E62C);
    cpu.pc = 0x28758;
    16
}
/// $04E62C: bcc.w $4e640
pub fn i_04e62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E630;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E640; return 10; }
    8
}
/// $04E630: move.b #$ff, $20(a6)
pub fn i_04e630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E636;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E636: bclr.b #$0, $13(a6)
pub fn i_04e636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E63C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E63C: bra.w $4e64a
pub fn i_04e63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E640;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E64A;
    10
}
/// $04E640: cmpi.b #$f, $21(a6)
pub fn i_04e640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E646;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Byte, false);
    16
}
/// $04E646: bne.w $4e67a
pub fn i_04e646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E64A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4E67A; return 10; }
    8
}
/// $04E64A: lea.l $295c32.l, a1
pub fn i_04e64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E650;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C32;
    20
}
/// $04E650: jsr $77c7e.l
pub fn i_04e650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E656);
    cpu.pc = 0x77C7E;
    16
}
/// $04E656: lea.l $295c44.l, a1
pub fn i_04e656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E65C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C44;
    20
}
/// $04E65C: jsr $77c7e.l
pub fn i_04e65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E662);
    cpu.pc = 0x77C7E;
    16
}
/// $04E662: lea.l $295cd4.l, a1
pub fn i_04e662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E668;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CD4;
    20
}
/// $04E668: jsr $77c7e.l
pub fn i_04e668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E66E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E66E);
    cpu.pc = 0x77C7E;
    16
}
/// $04E66E: move.b #$ff, $20(a6)
pub fn i_04e66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E674;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E674: lea.l $4e68a(pc), a1
pub fn i_04e674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E678;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E68A;
    16
}
/// $04E678: move.l a1, (a6)
pub fn i_04e678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E67A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E67A: jsr $4fa70(pc)
pub fn i_04e67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E67E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E67E);
    cpu.pc = 0x4FA70;
    16
}
/// $04E67E: bcc.w $4e688
pub fn i_04e67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E682;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E688; return 10; }
    8
}
/// $04E682: jmp $518.l
pub fn i_04e682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E688;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E688: rts
pub fn i_04e688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E68A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BarrierDamaged_04e68a  ($04E68A, 194 B, props/fortress_04e580.s)
/// $04E68A: move.w #$102f, d0
pub fn i_04e68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E68E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E68E: jsr $2352.l
pub fn i_04e68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E694);
    cpu.pc = 0x2352;
    16
}
/// $04E694: move.w #$64, $66(a6)
pub fn i_04e694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E69A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E69A: bclr.b #$0, $13(a6)
pub fn i_04e69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6A0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E6A0: lea.l $2957c2.l, a2
pub fn i_04e6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6A6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2957C2;
    20
}
/// $04E6A6: jsr $5022a(pc)
pub fn i_04e6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E6AA);
    cpu.pc = 0x5022A;
    16
}
/// $04E6AA: lea.l $2957ea.l, a2
pub fn i_04e6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6B0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2957EA;
    20
}
/// $04E6B0: jsr $5022a(pc)
pub fn i_04e6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E6B4);
    cpu.pc = 0x5022A;
    16
}
/// $04E6B4: lea.l $4e6ba(pc), a1
pub fn i_04e6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E6BA;
    16
}
/// $04E6B8: move.l a1, (a6)
pub fn i_04e6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E6BA: clr.b $10e39a.l
pub fn i_04e6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6C0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04E6C0: jsr $2783a.l
pub fn i_04e6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E6C6);
    cpu.pc = 0x2783A;
    16
}
/// $04E6C6: jsr $28998.l
pub fn i_04e6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E6CC);
    cpu.pc = 0x28998;
    16
}
/// $04E6CC: jsr $28d70.l
pub fn i_04e6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E6D2);
    cpu.pc = 0x28D70;
    16
}
/// $04E6D2: jsr $2870a.l
pub fn i_04e6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E6D8);
    cpu.pc = 0x2870A;
    16
}
/// $04E6D8: bcc.w $4e6ee
pub fn i_04e6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E6EE; return 10; }
    8
}
/// $04E6DC: lea.l $5e766.l, a0
pub fn i_04e6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E6E2: jsr $5e770.l
pub fn i_04e6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E6E8);
    cpu.pc = 0x5E770;
    16
}
/// $04E6E8: bclr.b #$3, $13(a6)
pub fn i_04e6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6EE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E6EE: jsr $28758.l
pub fn i_04e6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E6F4);
    cpu.pc = 0x28758;
    16
}
/// $04E6F4: bcc.w $4e73c
pub fn i_04e6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E73C; return 10; }
    8
}
/// $04E6F8: lea.l $295c32.l, a1
pub fn i_04e6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6FE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C32;
    20
}
/// $04E6FE: jsr $77c7e.l
pub fn i_04e6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E704;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E704);
    cpu.pc = 0x77C7E;
    16
}
/// $04E704: lea.l $295c44.l, a1
pub fn i_04e704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E70A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C44;
    20
}
/// $04E70A: jsr $77c7e.l
pub fn i_04e70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E710);
    cpu.pc = 0x77C7E;
    16
}
/// $04E710: lea.l $295cd4.l, a1
pub fn i_04e710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E716;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CD4;
    20
}
/// $04E716: jsr $77c7e.l
pub fn i_04e716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E71C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E71C);
    cpu.pc = 0x77C7E;
    16
}
/// $04E71C: lea.l $295e4e.l, a1
pub fn i_04e71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E722;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295E4E;
    20
}
/// $04E722: jsr $43fac.l
pub fn i_04e722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E728);
    cpu.pc = 0x43FAC;
    16
}
/// $04E728: move.b #$88, $20(a6)
pub fn i_04e728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E72E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E72E: move.b #$1, $10e39e.l
pub fn i_04e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E736;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $04E736: lea.l $4e74c(pc), a1
pub fn i_04e736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E73A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E74C;
    16
}
/// $04E73A: move.l a1, (a6)
pub fn i_04e73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E73C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E73C: jsr $4fa70(pc)
pub fn i_04e73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E740;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E740);
    cpu.pc = 0x4FA70;
    16
}
/// $04E740: bcc.w $4e74a
pub fn i_04e740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E744;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E74A; return 10; }
    8
}
/// $04E744: jmp $518.l
pub fn i_04e744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E74A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E74A: rts
pub fn i_04e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E74C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BarrierWreck_04e74c  ($04E74C, 178 B, props/fortress_04e580.s)
/// $04E74C: move.l #$1000, d0
pub fn i_04e74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E752;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E752: jsr $51a28.l
pub fn i_04e752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E758);
    cpu.pc = 0x51A28;
    16
}
/// $04E758: move.b #$0, $10e39e.l
pub fn i_04e758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E760;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $04E760: lea.l $296102.l, a0
pub fn i_04e760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E766;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296102;
    20
}
/// $04E766: move.l a0, $4c(a6)
pub fn i_04e766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E76A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E76A: jsr $283ca.l
pub fn i_04e76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E770);
    cpu.pc = 0x283CA;
    16
}
/// $04E770: jsr $283ca.l
pub fn i_04e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E776);
    cpu.pc = 0x283CA;
    16
}
/// $04E776: jsr $283d8.l
pub fn i_04e776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E77C);
    cpu.pc = 0x283D8;
    16
}
/// $04E77C: jsr $2783a.l
pub fn i_04e77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E782);
    cpu.pc = 0x2783A;
    16
}
/// $04E782: lea.l $295dee.l, a1
pub fn i_04e782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E788;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295DEE;
    20
}
/// $04E788: jsr $43fac.l
pub fn i_04e788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E78E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E78E);
    cpu.pc = 0x43FAC;
    16
}
/// $04E78E: move.w #$1030, d0
pub fn i_04e78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E792;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E792: jsr $2352.l
pub fn i_04e792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E798);
    cpu.pc = 0x2352;
    16
}
/// $04E798: lea.l $2957d6.l, a2
pub fn i_04e798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E79E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2957D6;
    20
}
/// $04E79E: jsr $5022a(pc)
pub fn i_04e79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E7A2);
    cpu.pc = 0x5022A;
    16
}
/// $04E7A2: lea.l $2957fe.l, a2
pub fn i_04e7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7A8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2957FE;
    20
}
/// $04E7A8: jsr $5022a(pc)
pub fn i_04e7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E7AC);
    cpu.pc = 0x5022A;
    16
}
/// $04E7AC: lea.l $4e7fe(pc), a1
pub fn i_04e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E7FE;
    16
}
/// $04E7B0: jsr $4ae.l
pub fn i_04e7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E7B6);
    cpu.pc = 0x4AE;
    16
}
/// $04E7B6: jsr $5dd22.l
pub fn i_04e7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E7BC);
    cpu.pc = 0x5DD22;
    16
}
/// $04E7BC: addi.w #$40, $22(a0)
pub fn i_04e7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7C2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E7C2: lea.l $2944da.l, a0
pub fn i_04e7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2944DA;
    20
}
/// $04E7C8: jsr $28cd4.l
pub fn i_04e7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E7CE);
    cpu.pc = 0x28CD4;
    16
}
/// $04E7CE: move.b #$ff, $75(a6)
pub fn i_04e7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E7D4: lea.l $ffff.w, a0
pub fn i_04e7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7D8;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E7D8: move.l a0, $48(a6)
pub fn i_04e7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7DC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E7DC: lea.l $4e7e2(pc), a1
pub fn i_04e7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E7E2;
    16
}
/// $04E7E0: move.l a1, (a6)
pub fn i_04e7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E7E2: jsr $2783a.l
pub fn i_04e7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E7E8);
    cpu.pc = 0x2783A;
    16
}
/// $04E7E8: jsr $28d70.l
pub fn i_04e7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E7EE);
    cpu.pc = 0x28D70;
    16
}
/// $04E7EE: jsr $4fa70(pc)
pub fn i_04e7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E7F2);
    cpu.pc = 0x4FA70;
    16
}
/// $04E7F2: bcc.w $4e7fc
pub fn i_04e7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E7FC; return 10; }
    8
}
/// $04E7F6: jmp $518.l
pub fn i_04e7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7FC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E7FC: rts
pub fn i_04e7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E7FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Gatehouse_04e7fe  ($04E7FE, 410 B, props/fortress_04e580.s)
/// $04E7FE: jsr $2783a.l
pub fn i_04e7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E804);
    cpu.pc = 0x2783A;
    16
}
/// $04E804: lea.l $4eae8(pc), a1
pub fn i_04e804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E808;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EAE8;
    16
}
/// $04E808: jsr $4ae.l
pub fn i_04e808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E80E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E80E);
    cpu.pc = 0x4AE;
    16
}
/// $04E80E: jsr $5dd22.l
pub fn i_04e80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E814);
    cpu.pc = 0x5DD22;
    16
}
/// $04E814: addi.w #$34, $22(a0)
pub fn i_04e814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E81A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x34, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E81A: lea.l $4e820(pc), a1
pub fn i_04e81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E81E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E820;
    16
}
/// $04E81E: move.l a1, (a6)
pub fn i_04e81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E820: jsr $2783a.l
pub fn i_04e820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E826);
    cpu.pc = 0x2783A;
    16
}
/// $04E826: move.l $106f50.l, d0
pub fn i_04e826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E82C;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04E82C: swap d0
pub fn i_04e82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E82E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E82E: cmpi.w #$a20, d0
pub fn i_04e82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E832;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA20, Size::Word, false);
    8
}
/// $04E832: bgt.w $4e838
pub fn i_04e832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E836;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4E838; return 10; }
    8
}
/// $04E836: rts
pub fn i_04e836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E838;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04E838: move.w #$3a0, d0
pub fn i_04e838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E83C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E83C: move.w #$e0, d1
pub fn i_04e83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E840;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E840: jsr $44022.l
pub fn i_04e840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E846);
    cpu.pc = 0x44022;
    16
}
/// $04E846: move.w d0, $22(a6)
pub fn i_04e846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E84A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E84A: move.w d1, $24(a6)
pub fn i_04e84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E84E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E84E: move.b #$0, $21(a6)
pub fn i_04e84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E854;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E854: move.b #$0, $20(a6)
pub fn i_04e854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E85A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E85A: lea.l $4f5be(pc), a1
pub fn i_04e85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E85E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F5BE;
    16
}
/// $04E85E: jsr $4ae.l
pub fn i_04e85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E864;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E864);
    cpu.pc = 0x4AE;
    16
}
/// $04E864: jsr $5dd22.l
pub fn i_04e864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E86A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E86A);
    cpu.pc = 0x5DD22;
    16
}
/// $04E86A: addi.w #$58, $22(a0)
pub fn i_04e86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E870;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x58, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E870: addi.w #$15, $24(a0)
pub fn i_04e870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E876;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x15, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E876: move.w #$40, d1
pub fn i_04e876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E87A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E87A: jsr $236e.l
pub fn i_04e87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E880;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E880);
    cpu.pc = 0x236E;
    16
}
/// $04E880: move.w #$70, $70(a6)
pub fn i_04e880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E886;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E886: move.w #$b4, $66(a6)
pub fn i_04e886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E88C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E88C: bset.b #$6, $12(a6)
pub fn i_04e88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E892;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E892: move.w #$f000, $38(a6)
pub fn i_04e892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E898;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E898: jsr $4fd8a(pc)
pub fn i_04e898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E89C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E89C);
    cpu.pc = 0x4FD8A;
    16
}
/// $04E89C: jsr $4fdde(pc)
pub fn i_04e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E8A0);
    cpu.pc = 0x4FDDE;
    16
}
/// $04E8A0: jsr $4fe32(pc)
pub fn i_04e8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E8A4);
    cpu.pc = 0x4FE32;
    16
}
/// $04E8A4: jsr $4fe86(pc)
pub fn i_04e8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E8A8);
    cpu.pc = 0x4FE86;
    16
}
/// $04E8A8: jsr $4feda(pc)
pub fn i_04e8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E8AC);
    cpu.pc = 0x4FEDA;
    16
}
/// $04E8AC: jsr $4ff2e(pc)
pub fn i_04e8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E8B0);
    cpu.pc = 0x4FF2E;
    16
}
/// $04E8B0: jsr $4ff82(pc)
pub fn i_04e8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E8B4);
    cpu.pc = 0x4FF82;
    16
}
/// $04E8B4: jsr $4ffd6(pc)
pub fn i_04e8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E8B8);
    cpu.pc = 0x4FFD6;
    16
}
/// $04E8B8: lea.l $4ef0a(pc), a1
pub fn i_04e8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EF0A;
    16
}
/// $04E8BC: jsr $4ae.l
pub fn i_04e8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E8C2);
    cpu.pc = 0x4AE;
    16
}
/// $04E8C2: jsr $5dd22.l
pub fn i_04e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E8C8);
    cpu.pc = 0x5DD22;
    16
}
/// $04E8C8: lea.l $4efe6(pc), a1
pub fn i_04e8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EFE6;
    16
}
/// $04E8CC: jsr $4ae.l
pub fn i_04e8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E8D2);
    cpu.pc = 0x4AE;
    16
}
/// $04E8D2: jsr $5dd22.l
pub fn i_04e8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E8D8);
    cpu.pc = 0x5DD22;
    16
}
/// $04E8D8: lea.l $2944fa.l, a0
pub fn i_04e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2944FA;
    20
}
/// $04E8DE: jsr $28cd4.l
pub fn i_04e8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E8E4);
    cpu.pc = 0x28CD4;
    16
}
/// $04E8E4: lea.l $e93b0.l, a1
pub fn i_04e8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE93B0;
    20
}
/// $04E8EA: move.w #$75, d0
pub fn i_04e8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E8EE: move.b #$0, $75(a6)
pub fn i_04e8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E8F4: jsr $4429e.l
pub fn i_04e8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E8FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E8FA);
    cpu.pc = 0x4429E;
    16
}
/// $04E8FA: jsr $2783a.l
pub fn i_04e8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E900);
    cpu.pc = 0x2783A;
    16
}
/// $04E900: lea.l $295f12.l, a1
pub fn i_04e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E906;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295F12;
    20
}
/// $04E906: jsr $43fac.l
pub fn i_04e906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E90C);
    cpu.pc = 0x43FAC;
    16
}
/// $04E90C: lea.l $4e912(pc), a1
pub fn i_04e90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E910;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E912;
    16
}
/// $04E910: move.l a1, (a6)
pub fn i_04e910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E912;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E912: jsr $2783a.l
pub fn i_04e912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E918);
    cpu.pc = 0x2783A;
    16
}
/// $04E918: jsr $28d70.l
pub fn i_04e918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E91E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E91E);
    cpu.pc = 0x28D70;
    16
}
/// $04E91E: jsr $2870a.l
pub fn i_04e91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E924);
    cpu.pc = 0x2870A;
    16
}
/// $04E924: bcc.w $4e93a
pub fn i_04e924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E928;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E93A; return 10; }
    8
}
/// $04E928: lea.l $5e766.l, a0
pub fn i_04e928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E92E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E92E: jsr $5e770.l
pub fn i_04e92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E934);
    cpu.pc = 0x5E770;
    16
}
/// $04E934: bclr.b #$3, $13(a6)
pub fn i_04e934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E93A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E93A: jsr $28758.l
pub fn i_04e93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E940);
    cpu.pc = 0x28758;
    16
}
/// $04E940: bcc.w $4e94e
pub fn i_04e940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E944;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E94E; return 10; }
    8
}
/// $04E944: bclr.b #$0, $13(a6)
pub fn i_04e944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E94A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E94A: bra.w $4e958
pub fn i_04e94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E94E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E958;
    10
}
/// $04E94E: cmpi.b #$ff, $21(a6)
pub fn i_04e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E954;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04E954: bne.w $4e988
pub fn i_04e954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E958;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4E988; return 10; }
    8
}
/// $04E958: move.b #$ff, $20(a6)
pub fn i_04e958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E95E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E95E: lea.l $295c32.l, a1
pub fn i_04e95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E964;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C32;
    20
}
/// $04E964: jsr $77c7e.l
pub fn i_04e964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E96A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E96A);
    cpu.pc = 0x77C7E;
    16
}
/// $04E96A: lea.l $295c44.l, a1
pub fn i_04e96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E970;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C44;
    20
}
/// $04E970: jsr $77c7e.l
pub fn i_04e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E976);
    cpu.pc = 0x77C7E;
    16
}
/// $04E976: lea.l $295cd4.l, a1
pub fn i_04e976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E97C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CD4;
    20
}
/// $04E97C: jsr $77c7e.l
pub fn i_04e97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E982;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E982);
    cpu.pc = 0x77C7E;
    16
}
/// $04E982: lea.l $4e998(pc), a1
pub fn i_04e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E986;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E998;
    16
}
/// $04E986: move.l a1, (a6)
pub fn i_04e986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E988;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E988: jsr $4fa70(pc)
pub fn i_04e988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E98C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E98C);
    cpu.pc = 0x4FA70;
    16
}
/// $04E98C: bcc.w $4e996
pub fn i_04e98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E990;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E996; return 10; }
    8
}
/// $04E990: jmp $518.l
pub fn i_04e990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E996;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E996: rts
pub fn i_04e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E998;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GatehouseDamaged_04e998  ($04E998, 166 B, props/fortress_04e580.s)
/// $04E998: move.w #$102f, d0
pub fn i_04e998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E99C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E99C: jsr $2352.l
pub fn i_04e99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E9A2);
    cpu.pc = 0x2352;
    16
}
/// $04E9A2: move.w #$b4, $66(a6)
pub fn i_04e9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E9A8: bclr.b #$0, $13(a6)
pub fn i_04e9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E9AE: jsr $501d8(pc)
pub fn i_04e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E9B2);
    cpu.pc = 0x501D8;
    16
}
/// $04E9B2: lea.l $29451c.l, a0
pub fn i_04e9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29451C;
    20
}
/// $04E9B8: jsr $28cd4.l
pub fn i_04e9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E9BE);
    cpu.pc = 0x28CD4;
    16
}
/// $04E9BE: lea.l $4e9c4(pc), a1
pub fn i_04e9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E9C4;
    16
}
/// $04E9C2: move.l a1, (a6)
pub fn i_04e9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E9C4: jsr $2783a.l
pub fn i_04e9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E9CA);
    cpu.pc = 0x2783A;
    16
}
/// $04E9CA: jsr $28d70.l
pub fn i_04e9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E9D0);
    cpu.pc = 0x28D70;
    16
}
/// $04E9D0: jsr $2870a.l
pub fn i_04e9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E9D6);
    cpu.pc = 0x2870A;
    16
}
/// $04E9D6: bcc.w $4e9ec
pub fn i_04e9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E9EC; return 10; }
    8
}
/// $04E9DA: lea.l $5e766.l, a0
pub fn i_04e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E9E0: jsr $5e770.l
pub fn i_04e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E9E6);
    cpu.pc = 0x5E770;
    16
}
/// $04E9E6: bclr.b #$3, $13(a6)
pub fn i_04e9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9EC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E9EC: jsr $28758.l
pub fn i_04e9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E9F2);
    cpu.pc = 0x28758;
    16
}
/// $04E9F2: bcc.w $4ea2e
pub fn i_04e9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EA2E; return 10; }
    8
}
/// $04E9F6: lea.l $295c32.l, a1
pub fn i_04e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E9FC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C32;
    20
}
/// $04E9FC: jsr $77c7e.l
pub fn i_04e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA02);
    cpu.pc = 0x77C7E;
    16
}
/// $04EA02: lea.l $295c44.l, a1
pub fn i_04ea02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA08;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C44;
    20
}
/// $04EA08: jsr $77c7e.l
pub fn i_04ea08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA0E);
    cpu.pc = 0x77C7E;
    16
}
/// $04EA0E: lea.l $295cd4.l, a1
pub fn i_04ea0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA14;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CD4;
    20
}
/// $04EA14: jsr $77c7e.l
pub fn i_04ea14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA1A);
    cpu.pc = 0x77C7E;
    16
}
/// $04EA1A: move.b #$88, $20(a6)
pub fn i_04ea1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04EA20: move.b #$1, $10e39e.l
pub fn i_04ea20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA28;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $04EA28: lea.l $4ea3e(pc), a1
pub fn i_04ea28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EA3E;
    16
}
/// $04EA2C: move.l a1, (a6)
pub fn i_04ea2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EA2E: jsr $4fa70(pc)
pub fn i_04ea2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EA32);
    cpu.pc = 0x4FA70;
    16
}
/// $04EA32: bcc.w $4ea3c
pub fn i_04ea32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EA3C; return 10; }
    8
}
/// $04EA36: jmp $518.l
pub fn i_04ea36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA3C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EA3C: rts
pub fn i_04ea3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GatehouseWreck_04ea3e  ($04EA3E, 170 B, props/fortress_04e580.s)
/// $04EA3E: move.l #$2000, d0
pub fn i_04ea3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA44;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04EA44: jsr $51a28.l
pub fn i_04ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA4A);
    cpu.pc = 0x51A28;
    16
}
/// $04EA4A: move.b #$2, $10e39a.l
pub fn i_04ea4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA52;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $04EA52: move.b #$0, $10e39e.l
pub fn i_04ea52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA5A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $04EA5A: jsr $2783a.l
pub fn i_04ea5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA60);
    cpu.pc = 0x2783A;
    16
}
/// $04EA60: lea.l $295e04.l, a1
pub fn i_04ea60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA66;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295E04;
    20
}
/// $04EA66: jsr $43fac.l
pub fn i_04ea66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA6C);
    cpu.pc = 0x43FAC;
    16
}
/// $04EA6C: lea.l $ffff.w, a0
pub fn i_04ea6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA70;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04EA70: move.l a0, $48(a6)
pub fn i_04ea70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA74;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04EA74: move.w #$1030, d0
pub fn i_04ea74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA78;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04EA78: jsr $2352.l
pub fn i_04ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA7E);
    cpu.pc = 0x2352;
    16
}
/// $04EA7E: lea.l $295f24.l, a1
pub fn i_04ea7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA84;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295F24;
    20
}
/// $04EA84: jsr $43fac.l
pub fn i_04ea84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EA8A);
    cpu.pc = 0x43FAC;
    16
}
/// $04EA8A: lea.l $295812.l, a2
pub fn i_04ea8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA90;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295812;
    20
}
/// $04EA90: jsr $5022a(pc)
pub fn i_04ea90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EA94);
    cpu.pc = 0x5022A;
    16
}
/// $04EA94: lea.l $2958da.l, a2
pub fn i_04ea94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA9A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2958DA;
    20
}
/// $04EA9A: jsr $5022a(pc)
pub fn i_04ea9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EA9E);
    cpu.pc = 0x5022A;
    16
}
/// $04EA9E: lea.l $4eb18(pc), a1
pub fn i_04ea9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EB18;
    16
}
/// $04EAA2: jsr $4ae.l
pub fn i_04eaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EAA8);
    cpu.pc = 0x4AE;
    16
}
/// $04EAA8: jsr $5dd22.l
pub fn i_04eaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EAAE);
    cpu.pc = 0x5DD22;
    16
}
/// $04EAAE: addi.w #$a0, $22(a0)
pub fn i_04eaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAB4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04EAB4: lea.l $29453e.l, a0
pub fn i_04eab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EABA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29453E;
    20
}
/// $04EABA: jsr $28cd4.l
pub fn i_04eaba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EAC0);
    cpu.pc = 0x28CD4;
    16
}
/// $04EAC0: move.b #$ff, $75(a6)
pub fn i_04eac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04EAC6: lea.l $4eacc(pc), a1
pub fn i_04eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EACA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EACC;
    16
}
/// $04EACA: move.l a1, (a6)
pub fn i_04eaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EACC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EACC: jsr $2783a.l
pub fn i_04eacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EAD2);
    cpu.pc = 0x2783A;
    16
}
/// $04EAD2: jsr $28d70.l
pub fn i_04ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EAD8);
    cpu.pc = 0x28D70;
    16
}
/// $04EAD8: jsr $4fa70(pc)
pub fn i_04ead8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EADC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EADC);
    cpu.pc = 0x4FA70;
    16
}
/// $04EADC: bcc.w $4eae6
pub fn i_04eadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EAE6; return 10; }
    8
}
/// $04EAE0: jmp $518.l
pub fn i_04eae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAE6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EAE6: rts
pub fn i_04eae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GatehouseBlocker_04eae8  ($04EAE8, 48 B, props/fortress_04e580.s)
/// $04EAE8: move.l #$2962be, $60(a6)
pub fn i_04eae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAF0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2962BE;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04EAF0: lea.l $4eaf6(pc), a1
pub fn i_04eaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EAF6;
    16
}
/// $04EAF4: move.l a1, (a6)
pub fn i_04eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EAF6: jsr $28998.l
pub fn i_04eaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EAFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EAFC);
    cpu.pc = 0x28998;
    16
}
/// $04EAFC: jsr $2783a.l
pub fn i_04eafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EB02);
    cpu.pc = 0x2783A;
    16
}
/// $04EB02: movea.l $c(a6), a0
pub fn i_04eb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB06;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04EB06: cmpi.b #$88, $20(a0)
pub fn i_04eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB0C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04EB0C: bne.w $4eb16
pub fn i_04eb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4EB16; return 10; }
    8
}
/// $04EB10: jmp $518.l
pub fn i_04eb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB16;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EB16: rts
pub fn i_04eb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Fortress_04eb18  ($04EB18, 116 B, props/fortress_04e580.s)
/// $04EB18: move.w #$440, d0
pub fn i_04eb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x440;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04EB1C: move.w #$e0, d1
pub fn i_04eb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB20;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04EB20: jsr $44022.l
pub fn i_04eb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EB26);
    cpu.pc = 0x44022;
    16
}
/// $04EB26: move.w d0, $22(a6)
pub fn i_04eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB2A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EB2A: move.w d1, $24(a6)
pub fn i_04eb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB2E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EB2E: move.b #$0, $21(a6)
pub fn i_04eb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04EB34: move.b #$0, $20(a6)
pub fn i_04eb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04EB3A: lea.l $4f030(pc), a1
pub fn i_04eb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F030;
    16
}
/// $04EB3E: jsr $4ae.l
pub fn i_04eb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EB44);
    cpu.pc = 0x4AE;
    16
}
/// $04EB44: jsr $5dd22.l
pub fn i_04eb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EB4A);
    cpu.pc = 0x5DD22;
    16
}
/// $04EB4A: subi.w #$10, $22(a0)
pub fn i_04eb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB50;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04EB50: addi.w #$51, $24(a0)
pub fn i_04eb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB56;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x51, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04EB56: move.w #$40, d1
pub fn i_04eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB5A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04EB5A: jsr $236e.l
pub fn i_04eb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EB60);
    cpu.pc = 0x236E;
    16
}
/// $04EB60: move.w #$0, $72(a6)
pub fn i_04eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB66;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EB66: move.w $72(a6), d0
pub fn i_04eb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB6A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04EB6A: andi.l #$3, d0
pub fn i_04eb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB70;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $04EB70: movea.l #$2945c4, a0
pub fn i_04eb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB76;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2945C4;
    cpu.a[0] = v1;
    4
}
/// $04EB76: lsl.w #$2, d0
pub fn i_04eb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB78;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04EB78: movea.l (a0, d0.w), a0
pub fn i_04eb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB7C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04EB7C: cmpa.l #$ffffffff, a0
pub fn i_04eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB82;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04EB82: beq.w $4eb8c
pub fn i_04eb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4EB8C; return 10; }
    8
}
/// $04EB86: jsr $28cd4.l
pub fn i_04eb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EB8C);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_04eb8c  ($04EB8C, 8 B, kernel/task_handlers.c)
/// $04EB8C: lea.l $4eb94(pc), a1
pub fn i_04eb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EB94;
    16
}
/// $04EB90: move.l a1, (a6)
pub fn i_04eb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EB92: rts
pub fn i_04eb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FortressWaitScroll_04eb94  ($04EB94, 32 B, props/fortress_04e580.s)
/// $04EB94: cmpi.w #$130, $22(a6)
pub fn i_04eb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB9A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x130, Size::Word, false);
    16
}
/// $04EB9A: ble.w $4ebbc
pub fn i_04eb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EB9E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4EBBC; return 10; }
    8
}
/// $04EB9E: clr.b $10e39a.l
pub fn i_04eb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBA4;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04EBA4: jsr $2783a.l
pub fn i_04eba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EBAA);
    cpu.pc = 0x2783A;
    16
}
/// $04EBAA: cmpi.w #$d0, $22(a6)
pub fn i_04ebaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD0, Size::Word, false);
    16
}
/// $04EBB0: bgt.w $4ebba
pub fn i_04ebb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBB4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4EBBA; return 10; }
    8
}
// ===== JsrAbsThunk_04ebb4  ($04EBB4, 8 B, thunks/jsr_abs_thunks.c)
/// $04EBB4: jsr $28d70.l
pub fn i_04ebb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EBBA);
    cpu.pc = 0x28D70;
    16
}
/// $04EBBA: rts
pub fn i_04ebba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FortressActive_04ebbc  ($04EBBC, 324 B, props/fortress_04e580.s)
/// $04EBBC: move.w #$120, $70(a6)
pub fn i_04ebbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBC2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EBC2: lea.l $2c0010.l, a0
pub fn i_04ebc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0010;
    20
}
/// $04EBC8: jsr $799de.l
pub fn i_04ebc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EBCE);
    cpu.pc = 0x799DE;
    16
}
/// $04EBCE: move.w d0, $66(a6)
pub fn i_04ebce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBD2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EBD2: bset.b #$6, $12(a6)
pub fn i_04ebd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBD8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EBD8: move.w #$f000, $38(a6)
pub fn i_04ebd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EBDE: jsr $5002a(pc)
pub fn i_04ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EBE2);
    cpu.pc = 0x5002A;
    16
}
/// $04EBE2: jsr $5007e(pc)
pub fn i_04ebe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EBE6);
    cpu.pc = 0x5007E;
    16
}
/// $04EBE6: jsr $500d2(pc)
pub fn i_04ebe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EBEA);
    cpu.pc = 0x500D2;
    16
}
/// $04EBEA: jsr $50126(pc)
pub fn i_04ebea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EBEE);
    cpu.pc = 0x50126;
    16
}
/// $04EBEE: jsr $5017a(pc)
pub fn i_04ebee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EBF2);
    cpu.pc = 0x5017A;
    16
}
/// $04EBF2: lea.l $e9442.l, a1
pub fn i_04ebf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBF8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE9442;
    20
}
/// $04EBF8: move.w #$75, d0
pub fn i_04ebf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EBFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04EBFC: move.b #$0, $75(a6)
pub fn i_04ebfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04EC02: jsr $4429e.l
pub fn i_04ec02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC08);
    cpu.pc = 0x4429E;
    16
}
/// $04EC08: lea.l $4edda(pc), a1
pub fn i_04ec08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EDDA;
    16
}
/// $04EC0C: jsr $4ae.l
pub fn i_04ec0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC12);
    cpu.pc = 0x4AE;
    16
}
/// $04EC12: jsr $5dd22.l
pub fn i_04ec12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC18);
    cpu.pc = 0x5DD22;
    16
}
/// $04EC18: addi.w #$78, $22(a0)
pub fn i_04ec18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC1E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x78, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04EC1E: lea.l $4ec24(pc), a1
pub fn i_04ec1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EC24;
    16
}
/// $04EC22: move.l a1, (a6)
pub fn i_04ec22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EC24: clr.b $10e39a.l
pub fn i_04ec24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC2A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04EC2A: jsr $2783a.l
pub fn i_04ec2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC30);
    cpu.pc = 0x2783A;
    16
}
/// $04EC30: jsr $28d70.l
pub fn i_04ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC36);
    cpu.pc = 0x28D70;
    16
}
/// $04EC36: jsr $2870a.l
pub fn i_04ec36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC3C);
    cpu.pc = 0x2870A;
    16
}
/// $04EC3C: bcc.w $4ec52
pub fn i_04ec3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EC52; return 10; }
    8
}
/// $04EC40: lea.l $5e766.l, a0
pub fn i_04ec40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04EC46: jsr $5e770.l
pub fn i_04ec46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC4C);
    cpu.pc = 0x5E770;
    16
}
/// $04EC4C: bclr.b #$3, $13(a6)
pub fn i_04ec4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC52;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EC52: jsr $28758.l
pub fn i_04ec52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC58);
    cpu.pc = 0x28758;
    16
}
/// $04EC58: bcc.w $4ecb0
pub fn i_04ec58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4ECB0; return 10; }
    8
}
/// $04EC5C: bclr.b #$0, $13(a6)
pub fn i_04ec5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC62;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EC62: addq.w #$1, $72(a6)
pub fn i_04ec62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC66;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04EC66: move.w $72(a6), d0
pub fn i_04ec66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC6A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04EC6A: andi.l #$3, d0
pub fn i_04ec6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC70;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $04EC70: movea.l #$2945c4, a0
pub fn i_04ec70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC76;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2945C4;
    cpu.a[0] = v1;
    4
}
/// $04EC76: lsl.w #$2, d0
pub fn i_04ec76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC78;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04EC78: movea.l (a0, d0.w), a0
pub fn i_04ec78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC7C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04EC7C: cmpa.l #$ffffffff, a0
pub fn i_04ec7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC82;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04EC82: beq.w $4ec8c
pub fn i_04ec82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4EC8C; return 10; }
    8
}
/// $04EC86: jsr $28cd4.l
pub fn i_04ec86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC8C);
    cpu.pc = 0x28CD4;
    16
}
/// $04EC8C: lea.l $2c0010.l, a0
pub fn i_04ec8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC92;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0010;
    20
}
/// $04EC92: jsr $799de.l
pub fn i_04ec92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EC98);
    cpu.pc = 0x799DE;
    16
}
/// $04EC98: move.w d0, $66(a6)
pub fn i_04ec98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EC9C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EC9C: cmpi.w #$3, $72(a6)
pub fn i_04ec9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECA2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $04ECA2: blt.w $4ecb0
pub fn i_04eca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECA6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4ECB0; return 10; }
    8
}
/// $04ECA6: move.b #$ff, $20(a6)
pub fn i_04eca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECAC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04ECAC: bra.w $4ecc0
pub fn i_04ecac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4ECC0;
    10
}
/// $04ECB0: cmpi.b #$1f, $21(a6)
pub fn i_04ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECB6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Byte, false);
    16
}
/// $04ECB6: bne.w $4ecf0
pub fn i_04ecb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4ECF0; return 10; }
    8
}
/// $04ECBA: move.b #$ff, $20(a6)
pub fn i_04ecba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECC0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04ECC0: lea.l $295c32.l, a1
pub fn i_04ecc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECC6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C32;
    20
}
/// $04ECC6: jsr $77c7e.l
pub fn i_04ecc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ECCC);
    cpu.pc = 0x77C7E;
    16
}
/// $04ECCC: lea.l $295c44.l, a1
pub fn i_04eccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECD2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C44;
    20
}
/// $04ECD2: jsr $77c7e.l
pub fn i_04ecd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ECD8);
    cpu.pc = 0x77C7E;
    16
}
/// $04ECD8: lea.l $295cd4.l, a1
pub fn i_04ecd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECDE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CD4;
    20
}
/// $04ECDE: jsr $77c7e.l
pub fn i_04ecde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ECE4);
    cpu.pc = 0x77C7E;
    16
}
/// $04ECE4: jsr $434dc.l
pub fn i_04ece4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ECEA);
    cpu.pc = 0x434DC;
    16
}
/// $04ECEA: lea.l $4ed00(pc), a1
pub fn i_04ecea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4ED00;
    16
}
/// $04ECEE: move.l a1, (a6)
pub fn i_04ecee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04ECF0: jsr $4fa70(pc)
pub fn i_04ecf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4ECF4);
    cpu.pc = 0x4FA70;
    16
}
/// $04ECF4: bcc.w $4ecfe
pub fn i_04ecf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4ECFE; return 10; }
    8
}
/// $04ECF8: jmp $518.l
pub fn i_04ecf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ECFE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04ECFE: rts
pub fn i_04ecfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FortressWreck_04ed00  ($04ED00, 144 B, props/fortress_04e580.s)
/// $04ED00: move.l #$5000, d0
pub fn i_04ed00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED06;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04ED06: jsr $51a28.l
pub fn i_04ed06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED0C);
    cpu.pc = 0x51A28;
    16
}
/// $04ED0C: lea.l $86586.l, a1
pub fn i_04ed0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86586;
    20
}
/// $04ED12: jsr $4ae.l
pub fn i_04ed12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED18);
    cpu.pc = 0x4AE;
    16
}
/// $04ED18: jsr $2783a.l
pub fn i_04ed18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED1E);
    cpu.pc = 0x2783A;
    16
}
/// $04ED1E: lea.l $295e14.l, a1
pub fn i_04ed1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295E14;
    20
}
/// $04ED24: jsr $43fac.l
pub fn i_04ed24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED2A);
    cpu.pc = 0x43FAC;
    16
}
/// $04ED2A: lea.l $295e38.l, a1
pub fn i_04ed2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED30;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295E38;
    20
}
/// $04ED30: jsr $43fac.l
pub fn i_04ed30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED36);
    cpu.pc = 0x43FAC;
    16
}
/// $04ED36: lea.l $ffff.w, a0
pub fn i_04ed36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED3A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04ED3A: move.l a0, $48(a6)
pub fn i_04ed3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED3E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04ED3E: move.w #$1023, d0
pub fn i_04ed3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04ED42: jsr $2352.l
pub fn i_04ed42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED48);
    cpu.pc = 0x2352;
    16
}
/// $04ED48: lea.l $295a42.l, a2
pub fn i_04ed48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED4E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A42;
    20
}
/// $04ED4E: jsr $5022a(pc)
pub fn i_04ed4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4ED52);
    cpu.pc = 0x5022A;
    16
}
/// $04ED52: lea.l $295a56.l, a2
pub fn i_04ed52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED58;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A56;
    20
}
/// $04ED58: jsr $5022a(pc)
pub fn i_04ed58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4ED5C);
    cpu.pc = 0x5022A;
    16
}
/// $04ED5C: lea.l $2945b4.l, a0
pub fn i_04ed5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2945B4;
    20
}
/// $04ED62: jsr $28cd4.l
pub fn i_04ed62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED68);
    cpu.pc = 0x28CD4;
    16
}
/// $04ED68: move.b #$ff, $75(a6)
pub fn i_04ed68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04ED6E: lea.l $4ed74(pc), a1
pub fn i_04ed6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4ED74;
    16
}
/// $04ED72: move.l a1, (a6)
pub fn i_04ed72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04ED74: jsr $2783a.l
pub fn i_04ed74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED7A);
    cpu.pc = 0x2783A;
    16
}
/// $04ED7A: jsr $28d70.l
pub fn i_04ed7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED80);
    cpu.pc = 0x28D70;
    16
}
/// $04ED80: jsr $4fa70(pc)
pub fn i_04ed80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4ED84);
    cpu.pc = 0x4FA70;
    16
}
/// $04ED84: bcc.w $4ed8e
pub fn i_04ed84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4ED8E; return 10; }
    8
}
/// $04ED88: jmp $518.l
pub fn i_04ed88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED8E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04ED8E: rts
pub fn i_04ed8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BarrierPost_04ed90  ($04ED90, 74 B, props/fortress_04e580.s)
/// $04ED90: move.w #$3f, d1
pub fn i_04ed90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED94;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04ED94: jsr $236e.l
pub fn i_04ed94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ED9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ED9A);
    cpu.pc = 0x236E;
    16
}
/// $04ED9A: jsr $267e2.l
pub fn i_04ed9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EDA0);
    cpu.pc = 0x267E2;
    16
}
/// $04EDA0: move.w #$0, $38(a6)
pub fn i_04eda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDA6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EDA6: lea.l $2944ea.l, a0
pub fn i_04eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2944EA;
    20
}
/// $04EDAC: jsr $28cd4.l
pub fn i_04edac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EDB2);
    cpu.pc = 0x28CD4;
    16
}
/// $04EDB2: lea.l $4edb8(pc), a1
pub fn i_04edb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EDB8;
    16
}
/// $04EDB6: move.l a1, (a6)
pub fn i_04edb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EDB8: jsr $2783a.l
pub fn i_04edb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EDBE);
    cpu.pc = 0x2783A;
    16
}
/// $04EDBE: jsr $28d70.l
pub fn i_04edbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EDC4);
    cpu.pc = 0x28D70;
    16
}
/// $04EDC4: movea.l $c(a6), a0
pub fn i_04edc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04EDC8: cmpi.b #$88, $20(a0)
pub fn i_04edc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDCE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04EDCE: bne.w $4edd8
pub fn i_04edce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDD2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4EDD8; return 10; }
    8
}
/// $04EDD2: jmp $518.l
pub fn i_04edd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDD8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EDD8: rts
pub fn i_04edd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FortressBlocker_04edda  ($04EDDA, 48 B, props/fortress_04e580.s)
/// $04EDDA: move.l #$29630a, $60(a6)
pub fn i_04edda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDE2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29630A;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04EDE2: lea.l $4ede8(pc), a1
pub fn i_04ede2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EDE8;
    16
}
/// $04EDE6: move.l a1, (a6)
pub fn i_04ede6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDE8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EDE8: jsr $28998.l
pub fn i_04ede8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EDEE);
    cpu.pc = 0x28998;
    16
}
/// $04EDEE: jsr $2783a.l
pub fn i_04edee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EDF4);
    cpu.pc = 0x2783A;
    16
}
/// $04EDF4: movea.l $c(a6), a0
pub fn i_04edf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDF8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04EDF8: cmpi.b #$ff, $20(a0)
pub fn i_04edf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EDFE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04EDFE: bne.w $4ee08
pub fn i_04edfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE02;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4EE08; return 10; }
    8
}
/// $04EE02: jmp $518.l
pub fn i_04ee02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE08;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EE08: rts
pub fn i_04ee08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BarrierLight_04ee0a  ($04EE0A, 192 B, props/fortress_04e580.s)
/// $04EE0A: move.w #$80, $70(a6)
pub fn i_04ee0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EE10: move.w #$3c, $66(a6)
pub fn i_04ee10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE16;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EE16: lea.l $294620.l, a0
pub fn i_04ee16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294620;
    20
}
/// $04EE1C: jsr $28cd4.l
pub fn i_04ee1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EE22);
    cpu.pc = 0x28CD4;
    16
}
/// $04EE22: bset.b #$6, $12(a6)
pub fn i_04ee22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE28;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EE28: lea.l $4ee2e(pc), a1
pub fn i_04ee28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EE2E;
    16
}
/// $04EE2C: move.l a1, (a6)
pub fn i_04ee2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EE2E: jsr $2783a.l
pub fn i_04ee2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EE34);
    cpu.pc = 0x2783A;
    16
}
/// $04EE34: jsr $28d70.l
pub fn i_04ee34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EE3A);
    cpu.pc = 0x28D70;
    16
}
/// $04EE3A: movea.l $c(a6), a0
pub fn i_04ee3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE3E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04EE3E: cmpi.b #$88, $20(a0)
pub fn i_04ee3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE44;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04EE44: bne.w $4ee56
pub fn i_04ee44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE48;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4EE56; return 10; }
    8
}
/// $04EE48: lea.l $295b46.l, a2
pub fn i_04ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE4E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295B46;
    20
}
/// $04EE4E: jsr $5022a(pc)
pub fn i_04ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EE52);
    cpu.pc = 0x5022A;
    16
}
/// $04EE52: bra.w $4eec2
pub fn i_04ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE56;
    cpu.ir = 0x6000;
    cpu.pc = 0x4EEC2;
    10
}
/// $04EE56: cmpi.b #$5, $58(a6)
pub fn i_04ee56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE5C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $04EE5C: beq.w $4ee86
pub fn i_04ee5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4EE86; return 10; }
    8
}
/// $04EE60: jsr $2870a.l
pub fn i_04ee60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EE66);
    cpu.pc = 0x2870A;
    16
}
/// $04EE66: bcc.w $4ee7c
pub fn i_04ee66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EE7C; return 10; }
    8
}
/// $04EE6A: lea.l $5e766.l, a0
pub fn i_04ee6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04EE70: jsr $5e770.l
pub fn i_04ee70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EE76);
    cpu.pc = 0x5E770;
    16
}
/// $04EE76: bclr.b #$3, $13(a6)
pub fn i_04ee76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EE7C: jsr $28758.l
pub fn i_04ee7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EE82);
    cpu.pc = 0x28758;
    16
}
/// $04EE82: bcc.w $4eeba
pub fn i_04ee82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EEBA; return 10; }
    8
}
/// $04EE86: bclr.b #$3, $13(a6)
pub fn i_04ee86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE8C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EE8C: lea.l $295ae2.l, a2
pub fn i_04ee8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE92;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295AE2;
    20
}
/// $04EE92: jsr $5022a(pc)
pub fn i_04ee92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EE96);
    cpu.pc = 0x5022A;
    16
}
/// $04EE96: lea.l $295b0a.l, a2
pub fn i_04ee96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE9C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295B0A;
    20
}
/// $04EE9C: jsr $5022a(pc)
pub fn i_04ee9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EEA0);
    cpu.pc = 0x5022A;
    16
}
/// $04EEA0: lea.l $29584e.l, a2
pub fn i_04eea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEA6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29584E;
    20
}
/// $04EEA6: jsr $5022a(pc)
pub fn i_04eea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EEAA);
    cpu.pc = 0x5022A;
    16
}
/// $04EEAA: lea.l $2958c6.l, a2
pub fn i_04eeaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEB0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2958C6;
    20
}
/// $04EEB0: jsr $5022a(pc)
pub fn i_04eeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EEB4);
    cpu.pc = 0x5022A;
    16
}
/// $04EEB4: lea.l $4eeca(pc), a1
pub fn i_04eeb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EECA;
    16
}
/// $04EEB8: move.l a1, (a6)
pub fn i_04eeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EEBA: jsr $4fa70(pc)
pub fn i_04eeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EEBE);
    cpu.pc = 0x4FA70;
    16
}
/// $04EEBE: bcc.w $4eec8
pub fn i_04eebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EEC8; return 10; }
    8
}
/// $04EEC2: jmp $518.l
pub fn i_04eec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEC8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EEC8: rts
pub fn i_04eec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EECA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BarrierLightWreck_04eeca  ($04EECA, 64 B, props/fortress_04e580.s)
/// $04EECA: lea.l $ffff.w, a0
pub fn i_04eeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EECE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04EECE: move.l a0, $48(a6)
pub fn i_04eece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EED2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04EED2: lea.l $4eed8(pc), a1
pub fn i_04eed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EED6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EED8;
    16
}
/// $04EED6: move.l a1, (a6)
pub fn i_04eed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EED8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EED8: jsr $2783a.l
pub fn i_04eed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EEDE);
    cpu.pc = 0x2783A;
    16
}
/// $04EEDE: movea.l $c(a6), a0
pub fn i_04eede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEE2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04EEE2: cmpi.b #$88, $20(a0)
pub fn i_04eee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEE8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04EEE8: bne.w $4eefa
pub fn i_04eee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4EEFA; return 10; }
    8
}
/// $04EEEC: lea.l $295b46.l, a2
pub fn i_04eeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEF2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295B46;
    20
}
/// $04EEF2: jsr $5022a(pc)
pub fn i_04eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EEF6);
    cpu.pc = 0x5022A;
    16
}
/// $04EEF6: bra.w $4ef02
pub fn i_04eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4EF02;
    10
}
/// $04EEFA: jsr $4fa70(pc)
pub fn i_04eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EEFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EEFE);
    cpu.pc = 0x4FA70;
    16
}
/// $04EEFE: bcc.w $4ef08
pub fn i_04eefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF02;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EF08; return 10; }
    8
}
/// $04EF02: jmp $518.l
pub fn i_04ef02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF08;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EF08: rts
pub fn i_04ef08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GatehouseRoof_04ef0a  ($04EF0A, 152 B, props/fortress_04e580.s)
/// $04EF0A: move.w #$3f, d1
pub fn i_04ef0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF0E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04EF0E: jsr $236e.l
pub fn i_04ef0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EF14);
    cpu.pc = 0x236E;
    16
}
/// $04EF14: move.w #$40, $70(a6)
pub fn i_04ef14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF1A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EF1A: move.w #$78, $66(a6)
pub fn i_04ef1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EF20: lea.l $294636.l, a0
pub fn i_04ef20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294636;
    20
}
/// $04EF26: jsr $28cd4.l
pub fn i_04ef26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EF2C);
    cpu.pc = 0x28CD4;
    16
}
/// $04EF2C: move.w #$bfff, $38(a6)
pub fn i_04ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF32;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xBFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04EF32: lea.l $4ef38(pc), a1
pub fn i_04ef32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EF38;
    16
}
/// $04EF36: move.l a1, (a6)
pub fn i_04ef36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EF38: jsr $2783a.l
pub fn i_04ef38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EF3E);
    cpu.pc = 0x2783A;
    16
}
/// $04EF3E: jsr $28d70.l
pub fn i_04ef3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EF44);
    cpu.pc = 0x28D70;
    16
}
/// $04EF44: movea.l $c(a6), a0
pub fn i_04ef44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF48;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04EF48: cmpi.b #$ff, $20(a0)
pub fn i_04ef48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF4E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04EF4E: bne.w $4ef56
pub fn i_04ef4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4EF56; return 10; }
    8
}
/// $04EF52: bra.w $4ef82
pub fn i_04ef52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF56;
    cpu.ir = 0x6000;
    cpu.pc = 0x4EF82;
    10
}
/// $04EF56: jsr $2870a.l
pub fn i_04ef56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EF5C);
    cpu.pc = 0x2870A;
    16
}
/// $04EF5C: bcc.w $4ef72
pub fn i_04ef5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EF72; return 10; }
    8
}
/// $04EF60: lea.l $5e766.l, a0
pub fn i_04ef60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04EF66: jsr $5e770.l
pub fn i_04ef66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EF6C);
    cpu.pc = 0x5E770;
    16
}
/// $04EF6C: bclr.b #$3, $13(a6)
pub fn i_04ef6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF72;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EF72: jsr $28758.l
pub fn i_04ef72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EF78);
    cpu.pc = 0x28758;
    16
}
/// $04EF78: bcc.w $4ef92
pub fn i_04ef78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF7C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EF92; return 10; }
    8
}
/// $04EF7C: bclr.b #$0, $13(a6)
pub fn i_04ef7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF82;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04EF82: lea.l $295a2e.l, a2
pub fn i_04ef82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF88;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A2E;
    20
}
/// $04EF88: jsr $5022a(pc)
pub fn i_04ef88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EF8C);
    cpu.pc = 0x5022A;
    16
}
/// $04EF8C: lea.l $4efa2(pc), a1
pub fn i_04ef8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EFA2;
    16
}
/// $04EF90: move.l a1, (a6)
pub fn i_04ef90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EF92: jsr $4fa70(pc)
pub fn i_04ef92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EF96);
    cpu.pc = 0x4FA70;
    16
}
/// $04EF96: bcc.w $4efa0
pub fn i_04ef96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EF9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EFA0; return 10; }
    8
}
/// $04EF9A: jmp $518.l
pub fn i_04ef9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFA0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EFA0: rts
pub fn i_04efa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GatehouseRoofWreck_04efa2  ($04EFA2, 68 B, props/fortress_04e580.s)
/// $04EFA2: lea.l $29464c.l, a0
pub fn i_04efa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29464C;
    20
}
/// $04EFA8: jsr $28cd4.l
pub fn i_04efa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EFAE);
    cpu.pc = 0x28CD4;
    16
}
/// $04EFAE: lea.l $ffff.w, a0
pub fn i_04efae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFB2;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04EFB2: move.l a0, $48(a6)
pub fn i_04efb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFB6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04EFB6: lea.l $4efbc(pc), a1
pub fn i_04efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4EFBC;
    16
}
/// $04EFBA: move.l a1, (a6)
pub fn i_04efba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04EFBC: jsr $2783a.l
pub fn i_04efbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EFC2);
    cpu.pc = 0x2783A;
    16
}
/// $04EFC2: jsr $28d70.l
pub fn i_04efc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EFC8);
    cpu.pc = 0x28D70;
    16
}
/// $04EFC8: movea.l $c(a6), a0
pub fn i_04efc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFCC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04EFCC: cmpi.b #$88, $20(a0)
pub fn i_04efcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFD2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04EFD2: beq.w $4efde
pub fn i_04efd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFD6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4EFDE; return 10; }
    8
}
/// $04EFD6: jsr $4fa70(pc)
pub fn i_04efd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4EFDA);
    cpu.pc = 0x4FA70;
    16
}
/// $04EFDA: bcc.w $4efe4
pub fn i_04efda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4EFE4; return 10; }
    8
}
/// $04EFDE: jmp $518.l
pub fn i_04efde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFE4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04EFE4: rts
pub fn i_04efe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GatehouseDoor_04efe6  ($04EFE6, 74 B, props/fortress_04e580.s)
/// $04EFE6: move.w #$40, d1
pub fn i_04efe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFEA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04EFEA: jsr $236e.l
pub fn i_04efea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EFF0);
    cpu.pc = 0x236E;
    16
}
/// $04EFF0: lea.l $29465c.l, a0
pub fn i_04eff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29465C;
    20
}
/// $04EFF6: jsr $28cd4.l
pub fn i_04eff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EFFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4EFFC);
    cpu.pc = 0x28CD4;
    16
}
/// $04EFFC: bset.b #$6, $12(a6)
pub fn i_04effc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F002;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x04D3A8, i_04d3a8),
    (0x04D3AE, i_04d3ae),
    (0x04D3B4, i_04d3b4),
    (0x04D3B8, i_04d3b8),
    (0x04D3BC, i_04d3bc),
    (0x04D3C2, i_04d3c2),
    (0x04D3C6, i_04d3c6),
    (0x04D3CC, i_04d3cc),
    (0x04D3CE, i_04d3ce),
    (0x04D3D4, i_04d3d4),
    (0x04D3D8, i_04d3d8),
    (0x04D3DE, i_04d3de),
    (0x04D3E0, i_04d3e0),
    (0x04D3E4, i_04d3e4),
    (0x04D3E8, i_04d3e8),
    (0x04D3EC, i_04d3ec),
    (0x04D3F0, i_04d3f0),
    (0x04D3F4, i_04d3f4),
    (0x04D3F8, i_04d3f8),
    (0x04D3FC, i_04d3fc),
    (0x04D3FE, i_04d3fe),
    (0x04D402, i_04d402),
    (0x04D408, i_04d408),
    (0x04D40C, i_04d40c),
    (0x04D412, i_04d412),
    (0x04D416, i_04d416),
    (0x04D41A, i_04d41a),
    (0x04D420, i_04d420),
    (0x04D424, i_04d424),
    (0x04D428, i_04d428),
    (0x04D42C, i_04d42c),
    (0x04D430, i_04d430),
    (0x04D434, i_04d434),
    (0x04D438, i_04d438),
    (0x04D43C, i_04d43c),
    (0x04D440, i_04d440),
    (0x04D444, i_04d444),
    (0x04D446, i_04d446),
    (0x04D44A, i_04d44a),
    (0x04D44C, i_04d44c),
    (0x04D452, i_04d452),
    (0x04D454, i_04d454),
    (0x04D45A, i_04d45a),
    (0x04D45E, i_04d45e),
    (0x04D464, i_04d464),
    (0x04D468, i_04d468),
    (0x04D46C, i_04d46c),
    (0x04D46E, i_04d46e),
    (0x04D472, i_04d472),
    (0x04D474, i_04d474),
    (0x04D47A, i_04d47a),
    (0x04D47E, i_04d47e),
    (0x04D482, i_04d482),
    (0x04D488, i_04d488),
    (0x04D48A, i_04d48a),
    (0x04D48E, i_04d48e),
    (0x04D492, i_04d492),
    (0x04D498, i_04d498),
    (0x04D49C, i_04d49c),
    (0x04D4A0, i_04d4a0),
    (0x04D4A4, i_04d4a4),
    (0x04D4A6, i_04d4a6),
    (0x04D4AA, i_04d4aa),
    (0x04D4AE, i_04d4ae),
    (0x04D4B0, i_04d4b0),
    (0x04D4B4, i_04d4b4),
    (0x04D4B6, i_04d4b6),
    (0x04D4BA, i_04d4ba),
    (0x04D4BE, i_04d4be),
    (0x04D4C2, i_04d4c2),
    (0x04D4C6, i_04d4c6),
    (0x04D4CA, i_04d4ca),
    (0x04D4CE, i_04d4ce),
    (0x04D4D0, i_04d4d0),
    (0x04D4D4, i_04d4d4),
    (0x04D4D6, i_04d4d6),
    (0x04D4DA, i_04d4da),
    (0x04D4DE, i_04d4de),
    (0x04D4E2, i_04d4e2),
    (0x04D4E6, i_04d4e6),
    (0x04D4EA, i_04d4ea),
    (0x04D4EE, i_04d4ee),
    (0x04D4F0, i_04d4f0),
    (0x04D4F6, i_04d4f6),
    (0x04D4FA, i_04d4fa),
    (0x04D4FC, i_04d4fc),
    (0x04D500, i_04d500),
    (0x04D502, i_04d502),
    (0x04D506, i_04d506),
    (0x04D508, i_04d508),
    (0x04D50C, i_04d50c),
    (0x04D512, i_04d512),
    (0x04D516, i_04d516),
    (0x04D51A, i_04d51a),
    (0x04D51C, i_04d51c),
    (0x04D520, i_04d520),
    (0x04D526, i_04d526),
    (0x04D528, i_04d528),
    (0x04D52C, i_04d52c),
    (0x04D52E, i_04d52e),
    (0x04D532, i_04d532),
    (0x04D536, i_04d536),
    (0x04D53A, i_04d53a),
    (0x04D53E, i_04d53e),
    (0x04D540, i_04d540),
    (0x04D544, i_04d544),
    (0x04D548, i_04d548),
    (0x04D54C, i_04d54c),
    (0x04D552, i_04d552),
    (0x04D556, i_04d556),
    (0x04D558, i_04d558),
    (0x04D55E, i_04d55e),
    (0x04D562, i_04d562),
    (0x04D566, i_04d566),
    (0x04D56C, i_04d56c),
    (0x04D572, i_04d572),
    (0x04D576, i_04d576),
    (0x04D57C, i_04d57c),
    (0x04D57E, i_04d57e),
    (0x04D582, i_04d582),
    (0x04D588, i_04d588),
    (0x04D58A, i_04d58a),
    (0x04D590, i_04d590),
    (0x04D594, i_04d594),
    (0x04D596, i_04d596),
    (0x04D59A, i_04d59a),
    (0x04D59E, i_04d59e),
    (0x04D5A0, i_04d5a0),
    (0x04D5A2, i_04d5a2),
    (0x04D5A4, i_04d5a4),
    (0x04D5A6, i_04d5a6),
    (0x04D5AA, i_04d5aa),
    (0x04D5AC, i_04d5ac),
    (0x04D5B2, i_04d5b2),
    (0x04D5B8, i_04d5b8),
    (0x04D5BC, i_04d5bc),
    (0x04D5C0, i_04d5c0),
    (0x04D5C2, i_04d5c2),
    (0x04D5C6, i_04d5c6),
    (0x04D5C8, i_04d5c8),
    (0x04D5CE, i_04d5ce),
    (0x04D5D4, i_04d5d4),
    (0x04D5D8, i_04d5d8),
    (0x04D5DC, i_04d5dc),
    (0x04D5DE, i_04d5de),
    (0x04D5E2, i_04d5e2),
    (0x04D5E4, i_04d5e4),
    (0x04D5E8, i_04d5e8),
    (0x04D5EE, i_04d5ee),
    (0x04D5F0, i_04d5f0),
    (0x04D5F6, i_04d5f6),
    (0x04D5FA, i_04d5fa),
    (0x04D5FE, i_04d5fe),
    (0x04D600, i_04d600),
    (0x04D604, i_04d604),
    (0x04D608, i_04d608),
    (0x04D60C, i_04d60c),
    (0x04D610, i_04d610),
    (0x04D612, i_04d612),
    (0x04D616, i_04d616),
    (0x04D61C, i_04d61c),
    (0x04D61E, i_04d61e),
    (0x04D622, i_04d622),
    (0x04D624, i_04d624),
    (0x04D62A, i_04d62a),
    (0x04D62E, i_04d62e),
    (0x04D632, i_04d632),
    (0x04D634, i_04d634),
    (0x04D638, i_04d638),
    (0x04D63C, i_04d63c),
    (0x04D640, i_04d640),
    (0x04D644, i_04d644),
    (0x04D646, i_04d646),
    (0x04D64C, i_04d64c),
    (0x04D650, i_04d650),
    (0x04D654, i_04d654),
    (0x04D656, i_04d656),
    (0x04D65A, i_04d65a),
    (0x04D65E, i_04d65e),
    (0x04D662, i_04d662),
    (0x04D666, i_04d666),
    (0x04D668, i_04d668),
    (0x04D66C, i_04d66c),
    (0x04D670, i_04d670),
    (0x04D674, i_04d674),
    (0x04D678, i_04d678),
    (0x04D67C, i_04d67c),
    (0x04D680, i_04d680),
    (0x04D684, i_04d684),
    (0x04D688, i_04d688),
    (0x04D68E, i_04d68e),
    (0x04D692, i_04d692),
    (0x04D696, i_04d696),
    (0x04D698, i_04d698),
    (0x04D69C, i_04d69c),
    (0x04D6A0, i_04d6a0),
    (0x04D6A2, i_04d6a2),
    (0x04D6A6, i_04d6a6),
    (0x04D6AA, i_04d6aa),
    (0x04D6AC, i_04d6ac),
    (0x04D6B2, i_04d6b2),
    (0x04D6B6, i_04d6b6),
    (0x04D6BC, i_04d6bc),
    (0x04D6BE, i_04d6be),
    (0x04D6C4, i_04d6c4),
    (0x04D6C8, i_04d6c8),
    (0x04D6CE, i_04d6ce),
    (0x04D6D0, i_04d6d0),
    (0x04D6D4, i_04d6d4),
    (0x04D6D8, i_04d6d8),
    (0x04D6DC, i_04d6dc),
    (0x04D6E0, i_04d6e0),
    (0x04D6E4, i_04d6e4),
    (0x04D6E6, i_04d6e6),
    (0x04D6EA, i_04d6ea),
    (0x04D6EC, i_04d6ec),
    (0x04D6F0, i_04d6f0),
    (0x04D6F6, i_04d6f6),
    (0x04D6FC, i_04d6fc),
    (0x04D702, i_04d702),
    (0x04D708, i_04d708),
    (0x04D70C, i_04d70c),
    (0x04D710, i_04d710),
    (0x04D716, i_04d716),
    (0x04D71C, i_04d71c),
    (0x04D722, i_04d722),
    (0x04D728, i_04d728),
    (0x04D72C, i_04d72c),
    (0x04D72E, i_04d72e),
    (0x04D734, i_04d734),
    (0x04D73A, i_04d73a),
    (0x04D73E, i_04d73e),
    (0x04D742, i_04d742),
    (0x04D748, i_04d748),
    (0x04D74A, i_04d74a),
    (0x04D74E, i_04d74e),
    (0x04D754, i_04d754),
    (0x04D758, i_04d758),
    (0x04D75E, i_04d75e),
    (0x04D764, i_04d764),
    (0x04D76A, i_04d76a),
    (0x04D770, i_04d770),
    (0x04D776, i_04d776),
    (0x04D77C, i_04d77c),
    (0x04D780, i_04d780),
    (0x04D782, i_04d782),
    (0x04D788, i_04d788),
    (0x04D78E, i_04d78e),
    (0x04D794, i_04d794),
    (0x04D798, i_04d798),
    (0x04D79C, i_04d79c),
    (0x04D7A2, i_04d7a2),
    (0x04D7A8, i_04d7a8),
    (0x04D7AC, i_04d7ac),
    (0x04D7B0, i_04d7b0),
    (0x04D7B4, i_04d7b4),
    (0x04D7BA, i_04d7ba),
    (0x04D7C0, i_04d7c0),
    (0x04D7C6, i_04d7c6),
    (0x04D7CA, i_04d7ca),
    (0x04D7CC, i_04d7cc),
    (0x04D7D0, i_04d7d0),
    (0x04D7D4, i_04d7d4),
    (0x04D7DA, i_04d7da),
    (0x04D7DC, i_04d7dc),
    (0x04D7E2, i_04d7e2),
    (0x04D7E8, i_04d7e8),
    (0x04D7EC, i_04d7ec),
    (0x04D7EE, i_04d7ee),
    (0x04D7F4, i_04d7f4),
    (0x04D7FA, i_04d7fa),
    (0x04D7FE, i_04d7fe),
    (0x04D804, i_04d804),
    (0x04D808, i_04d808),
    (0x04D80A, i_04d80a),
    (0x04D80E, i_04d80e),
    (0x04D812, i_04d812),
    (0x04D818, i_04d818),
    (0x04D81A, i_04d81a),
    (0x04D81E, i_04d81e),
    (0x04D824, i_04d824),
    (0x04D82A, i_04d82a),
    (0x04D830, i_04d830),
    (0x04D836, i_04d836),
    (0x04D83C, i_04d83c),
    (0x04D842, i_04d842),
    (0x04D848, i_04d848),
    (0x04D84E, i_04d84e),
    (0x04D852, i_04d852),
    (0x04D856, i_04d856),
    (0x04D85C, i_04d85c),
    (0x04D860, i_04d860),
    (0x04D864, i_04d864),
    (0x04D868, i_04d868),
    (0x04D86A, i_04d86a),
    (0x04D86E, i_04d86e),
    (0x04D874, i_04d874),
    (0x04D878, i_04d878),
    (0x04D87C, i_04d87c),
    (0x04D880, i_04d880),
    (0x04D882, i_04d882),
    (0x04D886, i_04d886),
    (0x04D88C, i_04d88c),
    (0x04D892, i_04d892),
    (0x04D896, i_04d896),
    (0x04D898, i_04d898),
    (0x04D89E, i_04d89e),
    (0x04D8A4, i_04d8a4),
    (0x04D8A8, i_04d8a8),
    (0x04D8AC, i_04d8ac),
    (0x04D8B0, i_04d8b0),
    (0x04D8B4, i_04d8b4),
    (0x04D8BA, i_04d8ba),
    (0x04D8BE, i_04d8be),
    (0x04D8C4, i_04d8c4),
    (0x04D8CA, i_04d8ca),
    (0x04D8D0, i_04d8d0),
    (0x04D8D4, i_04d8d4),
    (0x04D8DA, i_04d8da),
    (0x04D8E0, i_04d8e0),
    (0x04D8E6, i_04d8e6),
    (0x04D8EA, i_04d8ea),
    (0x04D8F0, i_04d8f0),
    (0x04D8F2, i_04d8f2),
    (0x04D8F6, i_04d8f6),
    (0x04D8FC, i_04d8fc),
    (0x04D900, i_04d900),
    (0x04D906, i_04d906),
    (0x04D90C, i_04d90c),
    (0x04D912, i_04d912),
    (0x04D918, i_04d918),
    (0x04D91E, i_04d91e),
    (0x04D924, i_04d924),
    (0x04D92A, i_04d92a),
    (0x04D930, i_04d930),
    (0x04D936, i_04d936),
    (0x04D93C, i_04d93c),
    (0x04D940, i_04d940),
    (0x04D946, i_04d946),
    (0x04D94C, i_04d94c),
    (0x04D952, i_04d952),
    (0x04D958, i_04d958),
    (0x04D95E, i_04d95e),
    (0x04D964, i_04d964),
    (0x04D968, i_04d968),
    (0x04D96A, i_04d96a),
    (0x04D970, i_04d970),
    (0x04D976, i_04d976),
    (0x04D97C, i_04d97c),
    (0x04D980, i_04d980),
    (0x04D984, i_04d984),
    (0x04D986, i_04d986),
    (0x04D98A, i_04d98a),
    (0x04D98E, i_04d98e),
    (0x04D994, i_04d994),
    (0x04D996, i_04d996),
    (0x04D99C, i_04d99c),
    (0x04D9A2, i_04d9a2),
    (0x04D9A8, i_04d9a8),
    (0x04D9AE, i_04d9ae),
    (0x04D9B4, i_04d9b4),
    (0x04D9B8, i_04d9b8),
    (0x04D9BA, i_04d9ba),
    (0x04D9C0, i_04d9c0),
    (0x04D9C6, i_04d9c6),
    (0x04D9CA, i_04d9ca),
    (0x04D9CE, i_04d9ce),
    (0x04D9D4, i_04d9d4),
    (0x04D9D6, i_04d9d6),
    (0x04D9DA, i_04d9da),
    (0x04D9E0, i_04d9e0),
    (0x04D9E6, i_04d9e6),
    (0x04D9EC, i_04d9ec),
    (0x04D9F2, i_04d9f2),
    (0x04D9F8, i_04d9f8),
    (0x04D9FE, i_04d9fe),
    (0x04DA04, i_04da04),
    (0x04DA08, i_04da08),
    (0x04DA0A, i_04da0a),
    (0x04DA10, i_04da10),
    (0x04DA16, i_04da16),
    (0x04DA1C, i_04da1c),
    (0x04DA20, i_04da20),
    (0x04DA26, i_04da26),
    (0x04DA2C, i_04da2c),
    (0x04DA32, i_04da32),
    (0x04DA38, i_04da38),
    (0x04DA3C, i_04da3c),
    (0x04DA40, i_04da40),
    (0x04DA42, i_04da42),
    (0x04DA46, i_04da46),
    (0x04DA4A, i_04da4a),
    (0x04DA50, i_04da50),
    (0x04DA52, i_04da52),
    (0x04DA58, i_04da58),
    (0x04DA5E, i_04da5e),
    (0x04DA64, i_04da64),
    (0x04DA68, i_04da68),
    (0x04DA6E, i_04da6e),
    (0x04DA74, i_04da74),
    (0x04DA7A, i_04da7a),
    (0x04DA80, i_04da80),
    (0x04DA86, i_04da86),
    (0x04DA8A, i_04da8a),
    (0x04DA8C, i_04da8c),
    (0x04DA92, i_04da92),
    (0x04DA98, i_04da98),
    (0x04DA9C, i_04da9c),
    (0x04DAA0, i_04daa0),
    (0x04DAA4, i_04daa4),
    (0x04DAAA, i_04daaa),
    (0x04DAAE, i_04daae),
    (0x04DAB4, i_04dab4),
    (0x04DAB8, i_04dab8),
    (0x04DABE, i_04dabe),
    (0x04DAC4, i_04dac4),
    (0x04DACA, i_04daca),
    (0x04DACE, i_04dace),
    (0x04DAD0, i_04dad0),
    (0x04DAD2, i_04dad2),
    (0x04DAD6, i_04dad6),
    (0x04DADC, i_04dadc),
    (0x04DAE0, i_04dae0),
    (0x04DAE6, i_04dae6),
    (0x04DAEC, i_04daec),
    (0x04DAF2, i_04daf2),
    (0x04DAF8, i_04daf8),
    (0x04DAFC, i_04dafc),
    (0x04DB02, i_04db02),
    (0x04DB08, i_04db08),
    (0x04DB0C, i_04db0c),
    (0x04DB12, i_04db12),
    (0x04DB18, i_04db18),
    (0x04DB1E, i_04db1e),
    (0x04DB24, i_04db24),
    (0x04DB2A, i_04db2a),
    (0x04DB30, i_04db30),
    (0x04DB34, i_04db34),
    (0x04DB36, i_04db36),
    (0x04DB3C, i_04db3c),
    (0x04DB40, i_04db40),
    (0x04DB48, i_04db48),
    (0x04DB4C, i_04db4c),
    (0x04DB50, i_04db50),
    (0x04DB54, i_04db54),
    (0x04DB58, i_04db58),
    (0x04DB5E, i_04db5e),
    (0x04DB62, i_04db62),
    (0x04DB68, i_04db68),
    (0x04DB6C, i_04db6c),
    (0x04DB70, i_04db70),
    (0x04DB72, i_04db72),
    (0x04DB78, i_04db78),
    (0x04DB7C, i_04db7c),
    (0x04DB82, i_04db82),
    (0x04DB88, i_04db88),
    (0x04DB8E, i_04db8e),
    (0x04DB94, i_04db94),
    (0x04DB9A, i_04db9a),
    (0x04DBA0, i_04dba0),
    (0x04DBA6, i_04dba6),
    (0x04DBAA, i_04dbaa),
    (0x04DBAC, i_04dbac),
    (0x04DBB2, i_04dbb2),
    (0x04DBB8, i_04dbb8),
    (0x04DBBE, i_04dbbe),
    (0x04DBC2, i_04dbc2),
    (0x04DBC8, i_04dbc8),
    (0x04DBCE, i_04dbce),
    (0x04DBD4, i_04dbd4),
    (0x04DBDA, i_04dbda),
    (0x04DBDE, i_04dbde),
    (0x04DBE4, i_04dbe4),
    (0x04DBEA, i_04dbea),
    (0x04DBF0, i_04dbf0),
    (0x04DBF6, i_04dbf6),
    (0x04DBFC, i_04dbfc),
    (0x04DC02, i_04dc02),
    (0x04DC08, i_04dc08),
    (0x04DC0E, i_04dc0e),
    (0x04DC14, i_04dc14),
    (0x04DC18, i_04dc18),
    (0x04DC1E, i_04dc1e),
    (0x04DC24, i_04dc24),
    (0x04DC28, i_04dc28),
    (0x04DC2A, i_04dc2a),
    (0x04DC2E, i_04dc2e),
    (0x04DC32, i_04dc32),
    (0x04DC38, i_04dc38),
    (0x04DC3C, i_04dc3c),
    (0x04DC42, i_04dc42),
    (0x04DC46, i_04dc46),
    (0x04DC48, i_04dc48),
    (0x04DC4A, i_04dc4a),
    (0x04DC50, i_04dc50),
    (0x04DC56, i_04dc56),
    (0x04DC5A, i_04dc5a),
    (0x04DC5E, i_04dc5e),
    (0x04DC64, i_04dc64),
    (0x04DC68, i_04dc68),
    (0x04DC6E, i_04dc6e),
    (0x04DC74, i_04dc74),
    (0x04DC78, i_04dc78),
    (0x04DC7E, i_04dc7e),
    (0x04DC82, i_04dc82),
    (0x04DC86, i_04dc86),
    (0x04DC88, i_04dc88),
    (0x04DC8E, i_04dc8e),
    (0x04DC92, i_04dc92),
    (0x04DC94, i_04dc94),
    (0x04DC96, i_04dc96),
    (0x04DC9A, i_04dc9a),
    (0x04DCA0, i_04dca0),
    (0x04DCA6, i_04dca6),
    (0x04DCAC, i_04dcac),
    (0x04DCB2, i_04dcb2),
    (0x04DCB8, i_04dcb8),
    (0x04DCBE, i_04dcbe),
    (0x04DCC2, i_04dcc2),
    (0x04DCC4, i_04dcc4),
    (0x04DCCA, i_04dcca),
    (0x04DCD0, i_04dcd0),
    (0x04DCD4, i_04dcd4),
    (0x04DCDA, i_04dcda),
    (0x04DCDE, i_04dcde),
    (0x04DCE4, i_04dce4),
    (0x04DCE6, i_04dce6),
    (0x04DCEC, i_04dcec),
    (0x04DCF2, i_04dcf2),
    (0x04DCF8, i_04dcf8),
    (0x04DCFE, i_04dcfe),
    (0x04DD04, i_04dd04),
    (0x04DD0A, i_04dd0a),
    (0x04DD10, i_04dd10),
    (0x04DD16, i_04dd16),
    (0x04DD1C, i_04dd1c),
    (0x04DD22, i_04dd22),
    (0x04DD28, i_04dd28),
    (0x04DD2C, i_04dd2c),
    (0x04DD2E, i_04dd2e),
    (0x04DD34, i_04dd34),
    (0x04DD3A, i_04dd3a),
    (0x04DD40, i_04dd40),
    (0x04DD44, i_04dd44),
    (0x04DD4A, i_04dd4a),
    (0x04DD4E, i_04dd4e),
    (0x04DD52, i_04dd52),
    (0x04DD58, i_04dd58),
    (0x04DD5E, i_04dd5e),
    (0x04DD64, i_04dd64),
    (0x04DD6A, i_04dd6a),
    (0x04DD6E, i_04dd6e),
    (0x04DD74, i_04dd74),
    (0x04DD78, i_04dd78),
    (0x04DD7E, i_04dd7e),
    (0x04DD84, i_04dd84),
    (0x04DD8A, i_04dd8a),
    (0x04DD8E, i_04dd8e),
    (0x04DD94, i_04dd94),
    (0x04DD9A, i_04dd9a),
    (0x04DDA0, i_04dda0),
    (0x04DDA6, i_04dda6),
    (0x04DDAA, i_04ddaa),
    (0x04DDB0, i_04ddb0),
    (0x04DDB6, i_04ddb6),
    (0x04DDBC, i_04ddbc),
    (0x04DDC2, i_04ddc2),
    (0x04DDC6, i_04ddc6),
    (0x04DDCC, i_04ddcc),
    (0x04DDD0, i_04ddd0),
    (0x04DDD2, i_04ddd2),
    (0x04DDD6, i_04ddd6),
    (0x04DDDA, i_04ddda),
    (0x04DDDE, i_04ddde),
    (0x04DDE4, i_04dde4),
    (0x04DDE8, i_04dde8),
    (0x04DDEA, i_04ddea),
    (0x04DDEC, i_04ddec),
    (0x04DDF2, i_04ddf2),
    (0x04DDF8, i_04ddf8),
    (0x04DDFC, i_04ddfc),
    (0x04DE00, i_04de00),
    (0x04DE04, i_04de04),
    (0x04DE0A, i_04de0a),
    (0x04DE10, i_04de10),
    (0x04DE14, i_04de14),
    (0x04DE1A, i_04de1a),
    (0x04DE20, i_04de20),
    (0x04DE26, i_04de26),
    (0x04DE2C, i_04de2c),
    (0x04DE30, i_04de30),
    (0x04DE32, i_04de32),
    (0x04DE38, i_04de38),
    (0x04DE3C, i_04de3c),
    (0x04DE3E, i_04de3e),
    (0x04DE40, i_04de40),
    (0x04DE44, i_04de44),
    (0x04DE4A, i_04de4a),
    (0x04DE50, i_04de50),
    (0x04DE54, i_04de54),
    (0x04DE5A, i_04de5a),
    (0x04DE60, i_04de60),
    (0x04DE66, i_04de66),
    (0x04DE6C, i_04de6c),
    (0x04DE72, i_04de72),
    (0x04DE78, i_04de78),
    (0x04DE7E, i_04de7e),
    (0x04DE84, i_04de84),
    (0x04DE88, i_04de88),
    (0x04DE8A, i_04de8a),
    (0x04DE90, i_04de90),
    (0x04DE96, i_04de96),
    (0x04DE9C, i_04de9c),
    (0x04DEA0, i_04dea0),
    (0x04DEA6, i_04dea6),
    (0x04DEAC, i_04deac),
    (0x04DEB2, i_04deb2),
    (0x04DEB8, i_04deb8),
    (0x04DEBC, i_04debc),
    (0x04DEC2, i_04dec2),
    (0x04DEC8, i_04dec8),
    (0x04DECE, i_04dece),
    (0x04DED4, i_04ded4),
    (0x04DEDA, i_04deda),
    (0x04DEE0, i_04dee0),
    (0x04DEE6, i_04dee6),
    (0x04DEEC, i_04deec),
    (0x04DEF2, i_04def2),
    (0x04DEF8, i_04def8),
    (0x04DEFE, i_04defe),
    (0x04DF04, i_04df04),
    (0x04DF08, i_04df08),
    (0x04DF0A, i_04df0a),
    (0x04DF0E, i_04df0e),
    (0x04DF12, i_04df12),
    (0x04DF18, i_04df18),
    (0x04DF1C, i_04df1c),
    (0x04DF1E, i_04df1e),
    (0x04DF22, i_04df22),
    (0x04DF26, i_04df26),
    (0x04DF2A, i_04df2a),
    (0x04DF2E, i_04df2e),
    (0x04DF30, i_04df30),
    (0x04DF36, i_04df36),
    (0x04DF3C, i_04df3c),
    (0x04DF40, i_04df40),
    (0x04DF44, i_04df44),
    (0x04DF48, i_04df48),
    (0x04DF4E, i_04df4e),
    (0x04DF54, i_04df54),
    (0x04DF58, i_04df58),
    (0x04DF5E, i_04df5e),
    (0x04DF62, i_04df62),
    (0x04DF68, i_04df68),
    (0x04DF6C, i_04df6c),
    (0x04DF70, i_04df70),
    (0x04DF72, i_04df72),
    (0x04DF78, i_04df78),
    (0x04DF7C, i_04df7c),
    (0x04DF80, i_04df80),
    (0x04DF84, i_04df84),
    (0x04DF86, i_04df86),
    (0x04DF8A, i_04df8a),
    (0x04DF8E, i_04df8e),
    (0x04DF92, i_04df92),
    (0x04DF96, i_04df96),
    (0x04DF98, i_04df98),
    (0x04DF9C, i_04df9c),
    (0x04DFA2, i_04dfa2),
    (0x04DFA6, i_04dfa6),
    (0x04DFAC, i_04dfac),
    (0x04DFB2, i_04dfb2),
    (0x04DFB6, i_04dfb6),
    (0x04DFBC, i_04dfbc),
    (0x04DFC2, i_04dfc2),
    (0x04DFC8, i_04dfc8),
    (0x04DFCE, i_04dfce),
    (0x04DFD4, i_04dfd4),
    (0x04DFDA, i_04dfda),
    (0x04DFE0, i_04dfe0),
    (0x04DFE4, i_04dfe4),
    (0x04DFEA, i_04dfea),
    (0x04DFF0, i_04dff0),
    (0x04DFF4, i_04dff4),
    (0x04DFF6, i_04dff6),
    (0x04DFFC, i_04dffc),
    (0x04E002, i_04e002),
    (0x04E006, i_04e006),
    (0x04E00A, i_04e00a),
    (0x04E010, i_04e010),
    (0x04E016, i_04e016),
    (0x04E01C, i_04e01c),
    (0x04E022, i_04e022),
    (0x04E028, i_04e028),
    (0x04E02E, i_04e02e),
    (0x04E034, i_04e034),
    (0x04E03A, i_04e03a),
    (0x04E03E, i_04e03e),
    (0x04E044, i_04e044),
    (0x04E048, i_04e048),
    (0x04E04C, i_04e04c),
    (0x04E04E, i_04e04e),
    (0x04E052, i_04e052),
    (0x04E056, i_04e056),
    (0x04E05C, i_04e05c),
    (0x04E05E, i_04e05e),
    (0x04E064, i_04e064),
    (0x04E06A, i_04e06a),
    (0x04E070, i_04e070),
    (0x04E074, i_04e074),
    (0x04E076, i_04e076),
    (0x04E07C, i_04e07c),
    (0x04E082, i_04e082),
    (0x04E088, i_04e088),
    (0x04E08C, i_04e08c),
    (0x04E092, i_04e092),
    (0x04E098, i_04e098),
    (0x04E09E, i_04e09e),
    (0x04E0A4, i_04e0a4),
    (0x04E0A8, i_04e0a8),
    (0x04E0AC, i_04e0ac),
    (0x04E0B0, i_04e0b0),
    (0x04E0B6, i_04e0b6),
    (0x04E0BC, i_04e0bc),
    (0x04E0C0, i_04e0c0),
    (0x04E0C6, i_04e0c6),
    (0x04E0CC, i_04e0cc),
    (0x04E0D2, i_04e0d2),
    (0x04E0D8, i_04e0d8),
    (0x04E0DE, i_04e0de),
    (0x04E0E4, i_04e0e4),
    (0x04E0EA, i_04e0ea),
    (0x04E0F0, i_04e0f0),
    (0x04E0F4, i_04e0f4),
    (0x04E0FA, i_04e0fa),
    (0x04E0FE, i_04e0fe),
    (0x04E104, i_04e104),
    (0x04E108, i_04e108),
    (0x04E10E, i_04e10e),
    (0x04E112, i_04e112),
    (0x04E116, i_04e116),
    (0x04E11A, i_04e11a),
    (0x04E11E, i_04e11e),
    (0x04E124, i_04e124),
    (0x04E12A, i_04e12a),
    (0x04E12C, i_04e12c),
    (0x04E132, i_04e132),
    (0x04E138, i_04e138),
    (0x04E13E, i_04e13e),
    (0x04E144, i_04e144),
    (0x04E148, i_04e148),
    (0x04E14E, i_04e14e),
    (0x04E154, i_04e154),
    (0x04E158, i_04e158),
    (0x04E15E, i_04e15e),
    (0x04E164, i_04e164),
    (0x04E16A, i_04e16a),
    (0x04E170, i_04e170),
    (0x04E176, i_04e176),
    (0x04E17C, i_04e17c),
    (0x04E182, i_04e182),
    (0x04E186, i_04e186),
    (0x04E188, i_04e188),
    (0x04E18E, i_04e18e),
    (0x04E194, i_04e194),
    (0x04E19A, i_04e19a),
    (0x04E19E, i_04e19e),
    (0x04E1A4, i_04e1a4),
    (0x04E1AA, i_04e1aa),
    (0x04E1B0, i_04e1b0),
    (0x04E1B6, i_04e1b6),
    (0x04E1BA, i_04e1ba),
    (0x04E1BE, i_04e1be),
    (0x04E1C4, i_04e1c4),
    (0x04E1C8, i_04e1c8),
    (0x04E1CC, i_04e1cc),
    (0x04E1D2, i_04e1d2),
    (0x04E1D8, i_04e1d8),
    (0x04E1DE, i_04e1de),
    (0x04E1E4, i_04e1e4),
    (0x04E1EA, i_04e1ea),
    (0x04E1F0, i_04e1f0),
    (0x04E1F6, i_04e1f6),
    (0x04E1FC, i_04e1fc),
    (0x04E200, i_04e200),
    (0x04E206, i_04e206),
    (0x04E20A, i_04e20a),
    (0x04E20C, i_04e20c),
    (0x04E210, i_04e210),
    (0x04E214, i_04e214),
    (0x04E218, i_04e218),
    (0x04E21C, i_04e21c),
    (0x04E222, i_04e222),
    (0x04E226, i_04e226),
    (0x04E22A, i_04e22a),
    (0x04E22E, i_04e22e),
    (0x04E234, i_04e234),
    (0x04E238, i_04e238),
    (0x04E23C, i_04e23c),
    (0x04E240, i_04e240),
    (0x04E246, i_04e246),
    (0x04E248, i_04e248),
    (0x04E24C, i_04e24c),
    (0x04E252, i_04e252),
    (0x04E256, i_04e256),
    (0x04E25C, i_04e25c),
    (0x04E262, i_04e262),
    (0x04E268, i_04e268),
    (0x04E26E, i_04e26e),
    (0x04E274, i_04e274),
    (0x04E27A, i_04e27a),
    (0x04E280, i_04e280),
    (0x04E286, i_04e286),
    (0x04E28C, i_04e28c),
    (0x04E292, i_04e292),
    (0x04E296, i_04e296),
    (0x04E298, i_04e298),
    (0x04E29E, i_04e29e),
    (0x04E2A2, i_04e2a2),
    (0x04E2A8, i_04e2a8),
    (0x04E2AE, i_04e2ae),
    (0x04E2B0, i_04e2b0),
    (0x04E2B4, i_04e2b4),
    (0x04E2B6, i_04e2b6),
    (0x04E2BC, i_04e2bc),
    (0x04E2C2, i_04e2c2),
    (0x04E2C8, i_04e2c8),
    (0x04E2CC, i_04e2cc),
    (0x04E2D2, i_04e2d2),
    (0x04E2D8, i_04e2d8),
    (0x04E2DE, i_04e2de),
    (0x04E2E4, i_04e2e4),
    (0x04E2E8, i_04e2e8),
    (0x04E2EC, i_04e2ec),
    (0x04E2F2, i_04e2f2),
    (0x04E2F8, i_04e2f8),
    (0x04E2FE, i_04e2fe),
    (0x04E304, i_04e304),
    (0x04E30A, i_04e30a),
    (0x04E310, i_04e310),
    (0x04E316, i_04e316),
    (0x04E31A, i_04e31a),
    (0x04E31C, i_04e31c),
    (0x04E320, i_04e320),
    (0x04E324, i_04e324),
    (0x04E32A, i_04e32a),
    (0x04E32C, i_04e32c),
    (0x04E332, i_04e332),
    (0x04E338, i_04e338),
    (0x04E33C, i_04e33c),
    (0x04E340, i_04e340),
    (0x04E346, i_04e346),
    (0x04E34C, i_04e34c),
    (0x04E352, i_04e352),
    (0x04E356, i_04e356),
    (0x04E35C, i_04e35c),
    (0x04E362, i_04e362),
    (0x04E368, i_04e368),
    (0x04E36C, i_04e36c),
    (0x04E36E, i_04e36e),
    (0x04E374, i_04e374),
    (0x04E37A, i_04e37a),
    (0x04E37E, i_04e37e),
    (0x04E382, i_04e382),
    (0x04E388, i_04e388),
    (0x04E38A, i_04e38a),
    (0x04E38E, i_04e38e),
    (0x04E394, i_04e394),
    (0x04E398, i_04e398),
    (0x04E39E, i_04e39e),
    (0x04E3A4, i_04e3a4),
    (0x04E3AA, i_04e3aa),
    (0x04E3B0, i_04e3b0),
    (0x04E3B6, i_04e3b6),
    (0x04E3BA, i_04e3ba),
    (0x04E3BC, i_04e3bc),
    (0x04E3C0, i_04e3c0),
    (0x04E3C6, i_04e3c6),
    (0x04E3CC, i_04e3cc),
    (0x04E3D0, i_04e3d0),
    (0x04E3D2, i_04e3d2),
    (0x04E3D8, i_04e3d8),
    (0x04E3DE, i_04e3de),
    (0x04E3E4, i_04e3e4),
    (0x04E3E8, i_04e3e8),
    (0x04E3EE, i_04e3ee),
    (0x04E3F4, i_04e3f4),
    (0x04E3FA, i_04e3fa),
    (0x04E400, i_04e400),
    (0x04E404, i_04e404),
    (0x04E40A, i_04e40a),
    (0x04E410, i_04e410),
    (0x04E416, i_04e416),
    (0x04E41C, i_04e41c),
    (0x04E422, i_04e422),
    (0x04E428, i_04e428),
    (0x04E42E, i_04e42e),
    (0x04E434, i_04e434),
    (0x04E43A, i_04e43a),
    (0x04E43E, i_04e43e),
    (0x04E440, i_04e440),
    (0x04E444, i_04e444),
    (0x04E448, i_04e448),
    (0x04E44E, i_04e44e),
    (0x04E450, i_04e450),
    (0x04E454, i_04e454),
    (0x04E45A, i_04e45a),
    (0x04E460, i_04e460),
    (0x04E464, i_04e464),
    (0x04E468, i_04e468),
    (0x04E46A, i_04e46a),
    (0x04E46E, i_04e46e),
    (0x04E472, i_04e472),
    (0x04E474, i_04e474),
    (0x04E47A, i_04e47a),
    (0x04E480, i_04e480),
    (0x04E486, i_04e486),
    (0x04E48A, i_04e48a),
    (0x04E490, i_04e490),
    (0x04E496, i_04e496),
    (0x04E49C, i_04e49c),
    (0x04E4A2, i_04e4a2),
    (0x04E4A6, i_04e4a6),
    (0x04E4AC, i_04e4ac),
    (0x04E4B2, i_04e4b2),
    (0x04E4B6, i_04e4b6),
    (0x04E4BC, i_04e4bc),
    (0x04E4C0, i_04e4c0),
    (0x04E4C4, i_04e4c4),
    (0x04E4CA, i_04e4ca),
    (0x04E4D0, i_04e4d0),
    (0x04E4D6, i_04e4d6),
    (0x04E4DC, i_04e4dc),
    (0x04E4E2, i_04e4e2),
    (0x04E4E8, i_04e4e8),
    (0x04E4EE, i_04e4ee),
    (0x04E4F4, i_04e4f4),
    (0x04E4FA, i_04e4fa),
    (0x04E4FE, i_04e4fe),
    (0x04E502, i_04e502),
    (0x04E506, i_04e506),
    (0x04E50A, i_04e50a),
    (0x04E510, i_04e510),
    (0x04E512, i_04e512),
    (0x04E516, i_04e516),
    (0x04E51C, i_04e51c),
    (0x04E520, i_04e520),
    (0x04E526, i_04e526),
    (0x04E52C, i_04e52c),
    (0x04E532, i_04e532),
    (0x04E538, i_04e538),
    (0x04E540, i_04e540),
    (0x04E544, i_04e544),
    (0x04E54A, i_04e54a),
    (0x04E550, i_04e550),
    (0x04E556, i_04e556),
    (0x04E55A, i_04e55a),
    (0x04E55C, i_04e55c),
    (0x04E562, i_04e562),
    (0x04E568, i_04e568),
    (0x04E56A, i_04e56a),
    (0x04E56E, i_04e56e),
    (0x04E572, i_04e572),
    (0x04E578, i_04e578),
    (0x04E57E, i_04e57e),
    (0x04E580, i_04e580),
    (0x04E586, i_04e586),
    (0x04E58A, i_04e58a),
    (0x04E590, i_04e590),
    (0x04E596, i_04e596),
    (0x04E59C, i_04e59c),
    (0x04E5A2, i_04e5a2),
    (0x04E5A6, i_04e5a6),
    (0x04E5AA, i_04e5aa),
    (0x04E5AE, i_04e5ae),
    (0x04E5B2, i_04e5b2),
    (0x04E5B6, i_04e5b6),
    (0x04E5BA, i_04e5ba),
    (0x04E5BE, i_04e5be),
    (0x04E5C4, i_04e5c4),
    (0x04E5CA, i_04e5ca),
    (0x04E5D0, i_04e5d0),
    (0x04E5D6, i_04e5d6),
    (0x04E5DC, i_04e5dc),
    (0x04E5E0, i_04e5e0),
    (0x04E5E6, i_04e5e6),
    (0x04E5EC, i_04e5ec),
    (0x04E5F0, i_04e5f0),
    (0x04E5F2, i_04e5f2),
    (0x04E5F8, i_04e5f8),
    (0x04E5FE, i_04e5fe),
    (0x04E604, i_04e604),
    (0x04E60A, i_04e60a),
    (0x04E610, i_04e610),
    (0x04E614, i_04e614),
    (0x04E61A, i_04e61a),
    (0x04E620, i_04e620),
    (0x04E626, i_04e626),
    (0x04E62C, i_04e62c),
    (0x04E630, i_04e630),
    (0x04E636, i_04e636),
    (0x04E63C, i_04e63c),
    (0x04E640, i_04e640),
    (0x04E646, i_04e646),
    (0x04E64A, i_04e64a),
    (0x04E650, i_04e650),
    (0x04E656, i_04e656),
    (0x04E65C, i_04e65c),
    (0x04E662, i_04e662),
    (0x04E668, i_04e668),
    (0x04E66E, i_04e66e),
    (0x04E674, i_04e674),
    (0x04E678, i_04e678),
    (0x04E67A, i_04e67a),
    (0x04E67E, i_04e67e),
    (0x04E682, i_04e682),
    (0x04E688, i_04e688),
    (0x04E68A, i_04e68a),
    (0x04E68E, i_04e68e),
    (0x04E694, i_04e694),
    (0x04E69A, i_04e69a),
    (0x04E6A0, i_04e6a0),
    (0x04E6A6, i_04e6a6),
    (0x04E6AA, i_04e6aa),
    (0x04E6B0, i_04e6b0),
    (0x04E6B4, i_04e6b4),
    (0x04E6B8, i_04e6b8),
    (0x04E6BA, i_04e6ba),
    (0x04E6C0, i_04e6c0),
    (0x04E6C6, i_04e6c6),
    (0x04E6CC, i_04e6cc),
    (0x04E6D2, i_04e6d2),
    (0x04E6D8, i_04e6d8),
    (0x04E6DC, i_04e6dc),
    (0x04E6E2, i_04e6e2),
    (0x04E6E8, i_04e6e8),
    (0x04E6EE, i_04e6ee),
    (0x04E6F4, i_04e6f4),
    (0x04E6F8, i_04e6f8),
    (0x04E6FE, i_04e6fe),
    (0x04E704, i_04e704),
    (0x04E70A, i_04e70a),
    (0x04E710, i_04e710),
    (0x04E716, i_04e716),
    (0x04E71C, i_04e71c),
    (0x04E722, i_04e722),
    (0x04E728, i_04e728),
    (0x04E72E, i_04e72e),
    (0x04E736, i_04e736),
    (0x04E73A, i_04e73a),
    (0x04E73C, i_04e73c),
    (0x04E740, i_04e740),
    (0x04E744, i_04e744),
    (0x04E74A, i_04e74a),
    (0x04E74C, i_04e74c),
    (0x04E752, i_04e752),
    (0x04E758, i_04e758),
    (0x04E760, i_04e760),
    (0x04E766, i_04e766),
    (0x04E76A, i_04e76a),
    (0x04E770, i_04e770),
    (0x04E776, i_04e776),
    (0x04E77C, i_04e77c),
    (0x04E782, i_04e782),
    (0x04E788, i_04e788),
    (0x04E78E, i_04e78e),
    (0x04E792, i_04e792),
    (0x04E798, i_04e798),
    (0x04E79E, i_04e79e),
    (0x04E7A2, i_04e7a2),
    (0x04E7A8, i_04e7a8),
    (0x04E7AC, i_04e7ac),
    (0x04E7B0, i_04e7b0),
    (0x04E7B6, i_04e7b6),
    (0x04E7BC, i_04e7bc),
    (0x04E7C2, i_04e7c2),
    (0x04E7C8, i_04e7c8),
    (0x04E7CE, i_04e7ce),
    (0x04E7D4, i_04e7d4),
    (0x04E7D8, i_04e7d8),
    (0x04E7DC, i_04e7dc),
    (0x04E7E0, i_04e7e0),
    (0x04E7E2, i_04e7e2),
    (0x04E7E8, i_04e7e8),
    (0x04E7EE, i_04e7ee),
    (0x04E7F2, i_04e7f2),
    (0x04E7F6, i_04e7f6),
    (0x04E7FC, i_04e7fc),
    (0x04E7FE, i_04e7fe),
    (0x04E804, i_04e804),
    (0x04E808, i_04e808),
    (0x04E80E, i_04e80e),
    (0x04E814, i_04e814),
    (0x04E81A, i_04e81a),
    (0x04E81E, i_04e81e),
    (0x04E820, i_04e820),
    (0x04E826, i_04e826),
    (0x04E82C, i_04e82c),
    (0x04E82E, i_04e82e),
    (0x04E832, i_04e832),
    (0x04E836, i_04e836),
    (0x04E838, i_04e838),
    (0x04E83C, i_04e83c),
    (0x04E840, i_04e840),
    (0x04E846, i_04e846),
    (0x04E84A, i_04e84a),
    (0x04E84E, i_04e84e),
    (0x04E854, i_04e854),
    (0x04E85A, i_04e85a),
    (0x04E85E, i_04e85e),
    (0x04E864, i_04e864),
    (0x04E86A, i_04e86a),
    (0x04E870, i_04e870),
    (0x04E876, i_04e876),
    (0x04E87A, i_04e87a),
    (0x04E880, i_04e880),
    (0x04E886, i_04e886),
    (0x04E88C, i_04e88c),
    (0x04E892, i_04e892),
    (0x04E898, i_04e898),
    (0x04E89C, i_04e89c),
    (0x04E8A0, i_04e8a0),
    (0x04E8A4, i_04e8a4),
    (0x04E8A8, i_04e8a8),
    (0x04E8AC, i_04e8ac),
    (0x04E8B0, i_04e8b0),
    (0x04E8B4, i_04e8b4),
    (0x04E8B8, i_04e8b8),
    (0x04E8BC, i_04e8bc),
    (0x04E8C2, i_04e8c2),
    (0x04E8C8, i_04e8c8),
    (0x04E8CC, i_04e8cc),
    (0x04E8D2, i_04e8d2),
    (0x04E8D8, i_04e8d8),
    (0x04E8DE, i_04e8de),
    (0x04E8E4, i_04e8e4),
    (0x04E8EA, i_04e8ea),
    (0x04E8EE, i_04e8ee),
    (0x04E8F4, i_04e8f4),
    (0x04E8FA, i_04e8fa),
    (0x04E900, i_04e900),
    (0x04E906, i_04e906),
    (0x04E90C, i_04e90c),
    (0x04E910, i_04e910),
    (0x04E912, i_04e912),
    (0x04E918, i_04e918),
    (0x04E91E, i_04e91e),
    (0x04E924, i_04e924),
    (0x04E928, i_04e928),
    (0x04E92E, i_04e92e),
    (0x04E934, i_04e934),
    (0x04E93A, i_04e93a),
    (0x04E940, i_04e940),
    (0x04E944, i_04e944),
    (0x04E94A, i_04e94a),
    (0x04E94E, i_04e94e),
    (0x04E954, i_04e954),
    (0x04E958, i_04e958),
    (0x04E95E, i_04e95e),
    (0x04E964, i_04e964),
    (0x04E96A, i_04e96a),
    (0x04E970, i_04e970),
    (0x04E976, i_04e976),
    (0x04E97C, i_04e97c),
    (0x04E982, i_04e982),
    (0x04E986, i_04e986),
    (0x04E988, i_04e988),
    (0x04E98C, i_04e98c),
    (0x04E990, i_04e990),
    (0x04E996, i_04e996),
    (0x04E998, i_04e998),
    (0x04E99C, i_04e99c),
    (0x04E9A2, i_04e9a2),
    (0x04E9A8, i_04e9a8),
    (0x04E9AE, i_04e9ae),
    (0x04E9B2, i_04e9b2),
    (0x04E9B8, i_04e9b8),
    (0x04E9BE, i_04e9be),
    (0x04E9C2, i_04e9c2),
    (0x04E9C4, i_04e9c4),
    (0x04E9CA, i_04e9ca),
    (0x04E9D0, i_04e9d0),
    (0x04E9D6, i_04e9d6),
    (0x04E9DA, i_04e9da),
    (0x04E9E0, i_04e9e0),
    (0x04E9E6, i_04e9e6),
    (0x04E9EC, i_04e9ec),
    (0x04E9F2, i_04e9f2),
    (0x04E9F6, i_04e9f6),
    (0x04E9FC, i_04e9fc),
    (0x04EA02, i_04ea02),
    (0x04EA08, i_04ea08),
    (0x04EA0E, i_04ea0e),
    (0x04EA14, i_04ea14),
    (0x04EA1A, i_04ea1a),
    (0x04EA20, i_04ea20),
    (0x04EA28, i_04ea28),
    (0x04EA2C, i_04ea2c),
    (0x04EA2E, i_04ea2e),
    (0x04EA32, i_04ea32),
    (0x04EA36, i_04ea36),
    (0x04EA3C, i_04ea3c),
    (0x04EA3E, i_04ea3e),
    (0x04EA44, i_04ea44),
    (0x04EA4A, i_04ea4a),
    (0x04EA52, i_04ea52),
    (0x04EA5A, i_04ea5a),
    (0x04EA60, i_04ea60),
    (0x04EA66, i_04ea66),
    (0x04EA6C, i_04ea6c),
    (0x04EA70, i_04ea70),
    (0x04EA74, i_04ea74),
    (0x04EA78, i_04ea78),
    (0x04EA7E, i_04ea7e),
    (0x04EA84, i_04ea84),
    (0x04EA8A, i_04ea8a),
    (0x04EA90, i_04ea90),
    (0x04EA94, i_04ea94),
    (0x04EA9A, i_04ea9a),
    (0x04EA9E, i_04ea9e),
    (0x04EAA2, i_04eaa2),
    (0x04EAA8, i_04eaa8),
    (0x04EAAE, i_04eaae),
    (0x04EAB4, i_04eab4),
    (0x04EABA, i_04eaba),
    (0x04EAC0, i_04eac0),
    (0x04EAC6, i_04eac6),
    (0x04EACA, i_04eaca),
    (0x04EACC, i_04eacc),
    (0x04EAD2, i_04ead2),
    (0x04EAD8, i_04ead8),
    (0x04EADC, i_04eadc),
    (0x04EAE0, i_04eae0),
    (0x04EAE6, i_04eae6),
    (0x04EAE8, i_04eae8),
    (0x04EAF0, i_04eaf0),
    (0x04EAF4, i_04eaf4),
    (0x04EAF6, i_04eaf6),
    (0x04EAFC, i_04eafc),
    (0x04EB02, i_04eb02),
    (0x04EB06, i_04eb06),
    (0x04EB0C, i_04eb0c),
    (0x04EB10, i_04eb10),
    (0x04EB16, i_04eb16),
    (0x04EB18, i_04eb18),
    (0x04EB1C, i_04eb1c),
    (0x04EB20, i_04eb20),
    (0x04EB26, i_04eb26),
    (0x04EB2A, i_04eb2a),
    (0x04EB2E, i_04eb2e),
    (0x04EB34, i_04eb34),
    (0x04EB3A, i_04eb3a),
    (0x04EB3E, i_04eb3e),
    (0x04EB44, i_04eb44),
    (0x04EB4A, i_04eb4a),
    (0x04EB50, i_04eb50),
    (0x04EB56, i_04eb56),
    (0x04EB5A, i_04eb5a),
    (0x04EB60, i_04eb60),
    (0x04EB66, i_04eb66),
    (0x04EB6A, i_04eb6a),
    (0x04EB70, i_04eb70),
    (0x04EB76, i_04eb76),
    (0x04EB78, i_04eb78),
    (0x04EB7C, i_04eb7c),
    (0x04EB82, i_04eb82),
    (0x04EB86, i_04eb86),
    (0x04EB8C, i_04eb8c),
    (0x04EB90, i_04eb90),
    (0x04EB92, i_04eb92),
    (0x04EB94, i_04eb94),
    (0x04EB9A, i_04eb9a),
    (0x04EB9E, i_04eb9e),
    (0x04EBA4, i_04eba4),
    (0x04EBAA, i_04ebaa),
    (0x04EBB0, i_04ebb0),
    (0x04EBB4, i_04ebb4),
    (0x04EBBA, i_04ebba),
    (0x04EBBC, i_04ebbc),
    (0x04EBC2, i_04ebc2),
    (0x04EBC8, i_04ebc8),
    (0x04EBCE, i_04ebce),
    (0x04EBD2, i_04ebd2),
    (0x04EBD8, i_04ebd8),
    (0x04EBDE, i_04ebde),
    (0x04EBE2, i_04ebe2),
    (0x04EBE6, i_04ebe6),
    (0x04EBEA, i_04ebea),
    (0x04EBEE, i_04ebee),
    (0x04EBF2, i_04ebf2),
    (0x04EBF8, i_04ebf8),
    (0x04EBFC, i_04ebfc),
    (0x04EC02, i_04ec02),
    (0x04EC08, i_04ec08),
    (0x04EC0C, i_04ec0c),
    (0x04EC12, i_04ec12),
    (0x04EC18, i_04ec18),
    (0x04EC1E, i_04ec1e),
    (0x04EC22, i_04ec22),
    (0x04EC24, i_04ec24),
    (0x04EC2A, i_04ec2a),
    (0x04EC30, i_04ec30),
    (0x04EC36, i_04ec36),
    (0x04EC3C, i_04ec3c),
    (0x04EC40, i_04ec40),
    (0x04EC46, i_04ec46),
    (0x04EC4C, i_04ec4c),
    (0x04EC52, i_04ec52),
    (0x04EC58, i_04ec58),
    (0x04EC5C, i_04ec5c),
    (0x04EC62, i_04ec62),
    (0x04EC66, i_04ec66),
    (0x04EC6A, i_04ec6a),
    (0x04EC70, i_04ec70),
    (0x04EC76, i_04ec76),
    (0x04EC78, i_04ec78),
    (0x04EC7C, i_04ec7c),
    (0x04EC82, i_04ec82),
    (0x04EC86, i_04ec86),
    (0x04EC8C, i_04ec8c),
    (0x04EC92, i_04ec92),
    (0x04EC98, i_04ec98),
    (0x04EC9C, i_04ec9c),
    (0x04ECA2, i_04eca2),
    (0x04ECA6, i_04eca6),
    (0x04ECAC, i_04ecac),
    (0x04ECB0, i_04ecb0),
    (0x04ECB6, i_04ecb6),
    (0x04ECBA, i_04ecba),
    (0x04ECC0, i_04ecc0),
    (0x04ECC6, i_04ecc6),
    (0x04ECCC, i_04eccc),
    (0x04ECD2, i_04ecd2),
    (0x04ECD8, i_04ecd8),
    (0x04ECDE, i_04ecde),
    (0x04ECE4, i_04ece4),
    (0x04ECEA, i_04ecea),
    (0x04ECEE, i_04ecee),
    (0x04ECF0, i_04ecf0),
    (0x04ECF4, i_04ecf4),
    (0x04ECF8, i_04ecf8),
    (0x04ECFE, i_04ecfe),
    (0x04ED00, i_04ed00),
    (0x04ED06, i_04ed06),
    (0x04ED0C, i_04ed0c),
    (0x04ED12, i_04ed12),
    (0x04ED18, i_04ed18),
    (0x04ED1E, i_04ed1e),
    (0x04ED24, i_04ed24),
    (0x04ED2A, i_04ed2a),
    (0x04ED30, i_04ed30),
    (0x04ED36, i_04ed36),
    (0x04ED3A, i_04ed3a),
    (0x04ED3E, i_04ed3e),
    (0x04ED42, i_04ed42),
    (0x04ED48, i_04ed48),
    (0x04ED4E, i_04ed4e),
    (0x04ED52, i_04ed52),
    (0x04ED58, i_04ed58),
    (0x04ED5C, i_04ed5c),
    (0x04ED62, i_04ed62),
    (0x04ED68, i_04ed68),
    (0x04ED6E, i_04ed6e),
    (0x04ED72, i_04ed72),
    (0x04ED74, i_04ed74),
    (0x04ED7A, i_04ed7a),
    (0x04ED80, i_04ed80),
    (0x04ED84, i_04ed84),
    (0x04ED88, i_04ed88),
    (0x04ED8E, i_04ed8e),
    (0x04ED90, i_04ed90),
    (0x04ED94, i_04ed94),
    (0x04ED9A, i_04ed9a),
    (0x04EDA0, i_04eda0),
    (0x04EDA6, i_04eda6),
    (0x04EDAC, i_04edac),
    (0x04EDB2, i_04edb2),
    (0x04EDB6, i_04edb6),
    (0x04EDB8, i_04edb8),
    (0x04EDBE, i_04edbe),
    (0x04EDC4, i_04edc4),
    (0x04EDC8, i_04edc8),
    (0x04EDCE, i_04edce),
    (0x04EDD2, i_04edd2),
    (0x04EDD8, i_04edd8),
    (0x04EDDA, i_04edda),
    (0x04EDE2, i_04ede2),
    (0x04EDE6, i_04ede6),
    (0x04EDE8, i_04ede8),
    (0x04EDEE, i_04edee),
    (0x04EDF4, i_04edf4),
    (0x04EDF8, i_04edf8),
    (0x04EDFE, i_04edfe),
    (0x04EE02, i_04ee02),
    (0x04EE08, i_04ee08),
    (0x04EE0A, i_04ee0a),
    (0x04EE10, i_04ee10),
    (0x04EE16, i_04ee16),
    (0x04EE1C, i_04ee1c),
    (0x04EE22, i_04ee22),
    (0x04EE28, i_04ee28),
    (0x04EE2C, i_04ee2c),
    (0x04EE2E, i_04ee2e),
    (0x04EE34, i_04ee34),
    (0x04EE3A, i_04ee3a),
    (0x04EE3E, i_04ee3e),
    (0x04EE44, i_04ee44),
    (0x04EE48, i_04ee48),
    (0x04EE4E, i_04ee4e),
    (0x04EE52, i_04ee52),
    (0x04EE56, i_04ee56),
    (0x04EE5C, i_04ee5c),
    (0x04EE60, i_04ee60),
    (0x04EE66, i_04ee66),
    (0x04EE6A, i_04ee6a),
    (0x04EE70, i_04ee70),
    (0x04EE76, i_04ee76),
    (0x04EE7C, i_04ee7c),
    (0x04EE82, i_04ee82),
    (0x04EE86, i_04ee86),
    (0x04EE8C, i_04ee8c),
    (0x04EE92, i_04ee92),
    (0x04EE96, i_04ee96),
    (0x04EE9C, i_04ee9c),
    (0x04EEA0, i_04eea0),
    (0x04EEA6, i_04eea6),
    (0x04EEAA, i_04eeaa),
    (0x04EEB0, i_04eeb0),
    (0x04EEB4, i_04eeb4),
    (0x04EEB8, i_04eeb8),
    (0x04EEBA, i_04eeba),
    (0x04EEBE, i_04eebe),
    (0x04EEC2, i_04eec2),
    (0x04EEC8, i_04eec8),
    (0x04EECA, i_04eeca),
    (0x04EECE, i_04eece),
    (0x04EED2, i_04eed2),
    (0x04EED6, i_04eed6),
    (0x04EED8, i_04eed8),
    (0x04EEDE, i_04eede),
    (0x04EEE2, i_04eee2),
    (0x04EEE8, i_04eee8),
    (0x04EEEC, i_04eeec),
    (0x04EEF2, i_04eef2),
    (0x04EEF6, i_04eef6),
    (0x04EEFA, i_04eefa),
    (0x04EEFE, i_04eefe),
    (0x04EF02, i_04ef02),
    (0x04EF08, i_04ef08),
    (0x04EF0A, i_04ef0a),
    (0x04EF0E, i_04ef0e),
    (0x04EF14, i_04ef14),
    (0x04EF1A, i_04ef1a),
    (0x04EF20, i_04ef20),
    (0x04EF26, i_04ef26),
    (0x04EF2C, i_04ef2c),
    (0x04EF32, i_04ef32),
    (0x04EF36, i_04ef36),
    (0x04EF38, i_04ef38),
    (0x04EF3E, i_04ef3e),
    (0x04EF44, i_04ef44),
    (0x04EF48, i_04ef48),
    (0x04EF4E, i_04ef4e),
    (0x04EF52, i_04ef52),
    (0x04EF56, i_04ef56),
    (0x04EF5C, i_04ef5c),
    (0x04EF60, i_04ef60),
    (0x04EF66, i_04ef66),
    (0x04EF6C, i_04ef6c),
    (0x04EF72, i_04ef72),
    (0x04EF78, i_04ef78),
    (0x04EF7C, i_04ef7c),
    (0x04EF82, i_04ef82),
    (0x04EF88, i_04ef88),
    (0x04EF8C, i_04ef8c),
    (0x04EF90, i_04ef90),
    (0x04EF92, i_04ef92),
    (0x04EF96, i_04ef96),
    (0x04EF9A, i_04ef9a),
    (0x04EFA0, i_04efa0),
    (0x04EFA2, i_04efa2),
    (0x04EFA8, i_04efa8),
    (0x04EFAE, i_04efae),
    (0x04EFB2, i_04efb2),
    (0x04EFB6, i_04efb6),
    (0x04EFBA, i_04efba),
    (0x04EFBC, i_04efbc),
    (0x04EFC2, i_04efc2),
    (0x04EFC8, i_04efc8),
    (0x04EFCC, i_04efcc),
    (0x04EFD2, i_04efd2),
    (0x04EFD6, i_04efd6),
    (0x04EFDA, i_04efda),
    (0x04EFDE, i_04efde),
    (0x04EFE4, i_04efe4),
    (0x04EFE6, i_04efe6),
    (0x04EFEA, i_04efea),
    (0x04EFF0, i_04eff0),
    (0x04EFF6, i_04eff6),
    (0x04EFFC, i_04effc),
];
