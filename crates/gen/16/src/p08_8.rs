// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $08F2B2: neg.w d1
pub fn i_08f2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2B4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F2B4: add.w $106f54.l, d1
pub fn i_08f2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2BA;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F2BA: lea.l $10e2f2.l, a0
pub fn i_08f2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F2C0: lea.l $8(a0), a1
pub fn i_08f2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C4;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F2C4: move.w (a0), d7
pub fn i_08f2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C6;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F2C6: cmp.w $4(a0), d7
pub fn i_08f2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2CA;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F2CA: beq.w $8f302
pub fn i_08f2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F302; return 10; }
    8
}
/// $08F2CE: cmp.w (a1, d7.w), d0
pub fn i_08f2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2D2;
    cpu.ir = 0xB071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2D2: bcs.w $8f2f4
pub fn i_08f2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2D6: cmp.w $4(a1, d7.w), d0
pub fn i_08f2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2DA;
    cpu.ir = 0xB071;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2DA: bhi.w $8f2f4
pub fn i_08f2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2DE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2DE: cmp.w $2(a1, d7.w), d1
pub fn i_08f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2E2;
    cpu.ir = 0xB271;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2E2: bcs.w $8f2f4
pub fn i_08f2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2E6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2E6: cmp.w $6(a1, d7.w), d1
pub fn i_08f2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2EA;
    cpu.ir = 0xB271;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2EA: bhi.w $8f2f4
pub fn i_08f2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2EE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8F2F4; return 10; }
    8
}
// ===== SetC_08f2ee  ($08F2EE, 6 B, thunks/ccr_helpers.c)
/// $08F2EE: ori.b #$1, ccr
pub fn i_08f2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F2F2: rts
pub fn i_08f2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_HitTest_Next_08f2f4  ($08F2F4, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F2F4: addq.w #$8, d7
pub fn i_08f2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F6;
    cpu.ir = 0x5047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F2F6: cmpi.w #$40, d7
pub fn i_08f2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2FA;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F2FA: bcs.w $8f300
pub fn i_08f2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2FE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F300; return 10; }
    8
}
/// $08F2FE: clr.w d7
pub fn i_08f2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F300;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F300: bra.b $8f2c6
pub fn i_08f300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F302;
    cpu.ir = 0x60C4;
    cpu.pc = 0x8F2C6;
    10
}
// ===== ClearC_08f302  ($08F302, 6 B, thunks/ccr_helpers.c)
/// $08F302: andi.b #$fe, ccr
pub fn i_08f302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F306;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F306: rts
pub fn i_08f306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F308;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_PushCapped_08F308  ($08F308, 60 B, kernel/pos_ring_push_capped_08f308.s)
/// $08F308: lea.l $10e33a.l, a0
pub fn i_08f308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F30E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F30E: lea.l $8(a0), a1
pub fn i_08f30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F312;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F312: adda.w $2(a0), a1
pub fn i_08f312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F316;
    cpu.ir = 0xD2E8;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $08F316: move.w $22(a6), (a1)
pub fn i_08f316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F31A;
    cpu.ir = 0x32AE;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v3 as u16);
    16
}
/// $08F31A: move.w $24(a6), $2(a1)
pub fn i_08f31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F320;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F320: cmpi.w #$4, $6(a0)
pub fn i_08f320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F326;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F326: bcc.w $8f342
pub fn i_08f326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F32A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F342; return 10; }
    8
}
/// $08F32A: move.w $2(a0), d7
pub fn i_08f32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F32E;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F32E: addq.w #$4, d7
pub fn i_08f32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F330;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F330: cmpi.w #$20, d7
pub fn i_08f330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F334;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $08F334: bcs.w $8f33a
pub fn i_08f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F338;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F33A; return 10; }
    8
}
/// $08F338: clr.w d7
pub fn i_08f338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F33A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F33A: move.w d7, $2(a0)
pub fn i_08f33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F33E;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F33E: addq.w #$1, $6(a0)
pub fn i_08f33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F342;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F342: rts
pub fn i_08f342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F344;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_FindNear_08f344  ($08F344, 72 B, enemies/critters_rings_08e4e6.s)
/// $08F344: lea.l $10e33a.l, a0
pub fn i_08f344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F34A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F34A: lea.l $8(a0), a1
pub fn i_08f34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F34E;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F34E: move.w (a0), d7
pub fn i_08f34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F350;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F350: cmp.w $4(a0), d7
pub fn i_08f350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F354;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F354: beq.w $8f3a0
pub fn i_08f354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F358;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F3A0; return 10; }
    8
}
/// $08F358: move.w (a1, d7.w), d0
pub fn i_08f358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F35C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F35C: move.w $2(a1, d7.w), d1
pub fn i_08f35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F360;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F360: sub.w $22(a6), d0
pub fn i_08f360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F364;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F364: smi.b d2
pub fn i_08f364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F366;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $08F366: sub.w $24(a6), d1
pub fn i_08f366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F36A;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F36A: addi.w #$40, d0
pub fn i_08f36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F36E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F36E: addi.w #$30, d1
pub fn i_08f36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F372;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F372: cmpi.w #$80, d0
pub fn i_08f372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F376;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $08F376: bcc.w $8f392
pub fn i_08f376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F37A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F392; return 10; }
    8
}
/// $08F37A: cmpi.w #$60, d1
pub fn i_08f37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F37E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $08F37E: bcc.w $8f392
pub fn i_08f37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F382;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F392; return 10; }
    8
}
/// $08F382: btst.b #$0, $3a(a6)
pub fn i_08f382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F388;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08F388: seq.b d0
pub fn i_08f388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F38A;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $08F38A: eor.b d2, d0
pub fn i_08f38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F38C;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetC_08f38c  ($08F38C, 6 B, thunks/ccr_helpers.c)
/// $08F38C: ori.b #$1, ccr
pub fn i_08f38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F390;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F390: rts
pub fn i_08f390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_FindNear_Next_08f392  ($08F392, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F392: addq.w #$4, d7
pub fn i_08f392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F394;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F394: cmpi.w #$20, d7
pub fn i_08f394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F398;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $08F398: bcs.w $8f39e
pub fn i_08f398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F39C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F39E; return 10; }
    8
}
/// $08F39C: clr.w d7
pub fn i_08f39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F39E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F39E: bra.b $8f350
pub fn i_08f39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A0;
    cpu.ir = 0x60B0;
    cpu.pc = 0x8F350;
    10
}
// ===== ClearC_08f3a0  ($08F3A0, 6 B, thunks/ccr_helpers.c)
/// $08F3A0: andi.b #$fe, ccr
pub fn i_08f3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F3A4: rts
pub fn i_08f3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_NewId_08f3a6  ($08F3A6, 24 B, enemies/critters_rings_08e4e6.s)
/// $08F3A6: move.b $10e2f0.l, d0
pub fn i_08f3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3AC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E2F0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F3AC: addq.b #$1, d0
pub fn i_08f3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3AE;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F3AE: andi.b #$3f, d0
pub fn i_08f3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3B2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08F3B2: addi.b #$40, d0
pub fn i_08f3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3B6;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08F3B6: move.b d0, $10e2f0.l
pub fn i_08f3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3BC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2F0, v1 as u8);
    16
}
/// $08F3BC: rts
pub fn i_08f3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_Register_08f3be  ($08F3BE, 178 B, enemies/critters_rings_08e4e6.s)
/// $08F3BE: lea.l $10e362.l, a0
pub fn i_08f3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F3C4: lea.l $8(a0), a1
pub fn i_08f3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3C8;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F3C8: move.b #$fe, d3
pub fn i_08f3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3CC;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F3CC: move.w (a0), d7
pub fn i_08f3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3CE;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F3CE: cmp.w $4(a0), d7
pub fn i_08f3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3D2;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F3D2: beq.w $8f3f8
pub fn i_08f3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F3F8; return 10; }
    8
}
/// $08F3D6: cmp.b $4(a1, d7.w), d2
pub fn i_08f3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3DA;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F3DA: bne.w $8f3ea
pub fn i_08f3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F3EA; return 10; }
    8
}
/// $08F3DE: move.b $5(a1, d7.w), d4
pub fn i_08f3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3E2;
    cpu.ir = 0x1831;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F3E2: bmi.w $8f3ea
pub fn i_08f3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3E6;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x8F3EA; return 10; }
    8
}
/// $08F3E6: move.b #$ff, d3
pub fn i_08f3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3EA;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F3EA: addq.w #$6, d7
pub fn i_08f3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3EC;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F3EC: cmpi.w #$30, d7
pub fn i_08f3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F0;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F3F0: bcs.w $8f3f6
pub fn i_08f3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F3F6; return 10; }
    8
}
/// $08F3F4: clr.w d7
pub fn i_08f3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F6;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F3F6: bra.b $8f3ce
pub fn i_08f3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3F8;
    cpu.ir = 0x60D6;
    cpu.pc = 0x8F3CE;
    10
}
/// $08F3F8: move.w $22(a6), d5
pub fn i_08f3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3FC;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F3FC: subi.w #$10, d5
pub fn i_08f3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F400;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F400: subi.w #$130, d5
pub fn i_08f400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F404;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x130, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F404: bcc.w $8f46e
pub fn i_08f404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F408;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F46E; return 10; }
    8
}
/// $08F408: cmp.w $2(a0), d7
pub fn i_08f408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F40C;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F40C: beq.w $8f43c
pub fn i_08f40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F410;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F43C; return 10; }
    8
}
/// $08F410: cmp.b $4(a1, d7.w), d2
pub fn i_08f410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F414;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F414: bne.w $8f42e
pub fn i_08f414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F418;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F42E; return 10; }
    8
}
/// $08F418: move.b $5(a1, d7.w), d5
pub fn i_08f418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F41C;
    cpu.ir = 0x1A31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F41C: bpl.w $8f42c
pub fn i_08f41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F420;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8F42C; return 10; }
    8
}
/// $08F420: move.w d0, (a1, d7.w)
pub fn i_08f420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F424;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $08F424: move.w d1, $2(a1, d7.w)
pub fn i_08f424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F428;
    cpu.ir = 0x3381;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $08F428: move.b d4, $5(a1, d7.w)
pub fn i_08f428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F42C;
    cpu.ir = 0x1384;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F42C: rts
pub fn i_08f42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F42E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08F42E: addq.w #$6, d7
pub fn i_08f42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F430;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F430: cmpi.w #$30, d7
pub fn i_08f430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F434;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F434: bcs.w $8f43a
pub fn i_08f434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F438;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F43A; return 10; }
    8
}
/// $08F438: clr.w d7
pub fn i_08f438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F43A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F43A: bra.b $8f408
pub fn i_08f43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F43C;
    cpu.ir = 0x60CC;
    cpu.pc = 0x8F408;
    10
}
/// $08F43C: adda.w d7, a1
pub fn i_08f43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F43E;
    cpu.ir = 0xD2C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $08F43E: move.w d0, (a1)
pub fn i_08f43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F440;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $08F440: move.w d1, $2(a1)
pub fn i_08f440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F444;
    cpu.ir = 0x3341;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F444: move.b d2, $4(a1)
pub fn i_08f444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F448;
    cpu.ir = 0x1342;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F448: move.b d3, $5(a1)
pub fn i_08f448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F44C;
    cpu.ir = 0x1343;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F44C: cmpi.w #$4, $6(a0)
pub fn i_08f44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F452;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F452: bcc.w $8f46e
pub fn i_08f452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F456;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F46E; return 10; }
    8
}
/// $08F456: move.w $2(a0), d7
pub fn i_08f456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F45A;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F45A: addq.w #$6, d7
pub fn i_08f45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F45C;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F45C: cmpi.w #$30, d7
pub fn i_08f45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F460;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F460: bcs.w $8f466
pub fn i_08f460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F464;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F466; return 10; }
    8
}
/// $08F464: clr.w d7
pub fn i_08f464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F466;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F466: move.w d7, $2(a0)
pub fn i_08f466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F46A;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F46A: addq.w #$1, $6(a0)
pub fn i_08f46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F46E;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F46E: rts
pub fn i_08f46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F470;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_FindPending_08f470  ($08F470, 108 B, enemies/critters_rings_08e4e6.s)
/// $08F470: lea.l $10e362.l, a0
pub fn i_08f470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F476;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F476: lea.l $8(a0), a1
pub fn i_08f476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F47A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F47A: move.w (a0), d7
pub fn i_08f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F47C;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F47C: cmpa.l #$100440, a6
pub fn i_08f47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F482;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F482: beq.w $8f4bc
pub fn i_08f482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F486;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4BC; return 10; }
    8
}
/// $08F486: cmpa.l #$1004e0, a6
pub fn i_08f486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F48C;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F48C: beq.w $8f4bc
pub fn i_08f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F490;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4BC; return 10; }
    8
}
/// $08F490: cmp.w $4(a0), d7
pub fn i_08f490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F494;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F494: beq.w $8f4b8
pub fn i_08f494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F498;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4B8; return 10; }
    8
}
/// $08F498: tst.b $4(a1, d7.w)
pub fn i_08f498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F49C;
    cpu.ir = 0x4A31;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $08F49C: bpl.w $8f4aa
pub fn i_08f49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4A0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8F4AA; return 10; }
    8
}
/// $08F4A0: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4A6;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F4A6: beq.w $8f4e2
pub fn i_08f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4E2; return 10; }
    8
}
/// $08F4AA: addq.w #$6, d7
pub fn i_08f4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4AC;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4AC: cmpi.w #$30, d7
pub fn i_08f4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B0;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F4B0: bcs.w $8f4b6
pub fn i_08f4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F4B6; return 10; }
    8
}
/// $08F4B4: clr.w d7
pub fn i_08f4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B6;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4B6: bra.b $8f490
pub fn i_08f4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B8;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F490;
    10
}
/// $08F4B8: bra.w $8f4dc
pub fn i_08f4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F4DC;
    10
}
/// $08F4BC: cmp.w $4(a0), d7
pub fn i_08f4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4C0;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F4C0: beq.w $8f4dc
pub fn i_08f4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4DC; return 10; }
    8
}
/// $08F4C4: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4CA;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F4CA: beq.w $8f4e2
pub fn i_08f4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4E2; return 10; }
    8
}
/// $08F4CE: addq.w #$6, d7
pub fn i_08f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D0;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4D0: cmpi.w #$30, d7
pub fn i_08f4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D4;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F4D4: bcs.w $8f4da
pub fn i_08f4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F4DA; return 10; }
    8
}
/// $08F4D8: clr.w d7
pub fn i_08f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4DA;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4DA: bra.b $8f4bc
pub fn i_08f4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4DC;
    cpu.ir = 0x60E0;
    cpu.pc = 0x8F4BC;
    10
}
// ===== ClearC_08f4dc  ($08F4DC, 6 B, thunks/ccr_helpers.c)
/// $08F4DC: andi.b #$fe, ccr
pub fn i_08f4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F4E0: rts
pub fn i_08f4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_InRange_08f4e2  ($08F4E2, 56 B, enemies/critters_rings_08e4e6.s)
/// $08F4E2: lea.l (a1, d7.w), a2
pub fn i_08f4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E6;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $08F4E6: move.w (a2), d0
pub fn i_08f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E8;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F4E8: move.w $2(a2), d1
pub fn i_08f4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4EC;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F4EC: moveq #$ff, d2
pub fn i_08f4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4EE;
    cpu.ir = 0x74FF;
    cpu.d[2] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F4EE: move.b $4(a2), d2
pub fn i_08f4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F2;
    cpu.ir = 0x142A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F4F2: move.w $22(a6), d4
pub fn i_08f4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F6;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F4F6: sub.w d0, d4
pub fn i_08f4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F8;
    cpu.ir = 0x9840;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F4F8: addi.w #$18, d4
pub fn i_08f4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4FC;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F4FC: cmpi.w #$30, d4
pub fn i_08f4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F500;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F500: bcc.w $8f51a
pub fn i_08f500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F504;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F51A; return 10; }
    8
}
/// $08F504: move.w $24(a6), d4
pub fn i_08f504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F508;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F508: sub.w d1, d4
pub fn i_08f508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F50A;
    cpu.ir = 0x9841;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F50A: addi.w #$20, d4
pub fn i_08f50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F50E;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F50E: cmpi.w #$40, d4
pub fn i_08f50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F512;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F512: bcc.w $8f51a
pub fn i_08f512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F516;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F51A; return 10; }
    8
}
/// $08F516: andi.w #$ff, d2
pub fn i_08f516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F51A;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetC_08f51a  ($08F51A, 6 B, thunks/ccr_helpers.c)
/// $08F51A: ori.b #$1, ccr
pub fn i_08f51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F51E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F51E: rts
pub fn i_08f51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F520;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_08f520  ($08F520, 84 B, enemies/critters_rings_08e4e6.s)
/// $08F520: lea.l $10e362.l, a0
pub fn i_08f520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F526;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F526: lea.l $8(a0), a1
pub fn i_08f526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F52A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F52A: move.w (a0), d7
pub fn i_08f52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F52C;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F52C: move.b #$0, d3
pub fn i_08f52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F530;
    cpu.ir = 0x163C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F530: cmpa.l #$100440, a6
pub fn i_08f530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F536;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F536: beq.w $8f54c
pub fn i_08f536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F53A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F54C; return 10; }
    8
}
/// $08F53A: move.b #$1, d3
pub fn i_08f53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F53E;
    cpu.ir = 0x163C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F53E: cmpa.l #$1004e0, a6
pub fn i_08f53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F544;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F544: beq.w $8f54c
pub fn i_08f544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F548;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F54C; return 10; }
    8
}
/// $08F548: move.b #$2, d3
pub fn i_08f548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F54C;
    cpu.ir = 0x163C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F54C: cmp.w $4(a0), d7
pub fn i_08f54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F550;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F550: beq.w $8f574
pub fn i_08f550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F554;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F574; return 10; }
    8
}
/// $08F554: cmp.b $4(a1, d7.w), d2
pub fn i_08f554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F558;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F558: bne.w $8f566
pub fn i_08f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F55C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F566; return 10; }
    8
}
/// $08F55C: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F562;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F562: beq.w $8f57a
pub fn i_08f562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F566;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F57A; return 10; }
    8
}
/// $08F566: addq.w #$6, d7
pub fn i_08f566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F568;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F568: cmpi.w #$30, d7
pub fn i_08f568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F56C;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F56C: bcs.w $8f572
pub fn i_08f56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F570;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F572; return 10; }
    8
}
/// $08F570: clr.w d7
pub fn i_08f570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F572;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F572: bra.b $8f54c
pub fn i_08f572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F574;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F54C;
    10
}
// ===== ClearC_08f574  ($08F574, 6 B, thunks/ccr_helpers.c)
/// $08F574: andi.b #$fe, ccr
pub fn i_08f574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F578;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F578: rts
pub fn i_08f578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F57A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_Mark_08f57a  ($08F57A, 30 B, enemies/critters_rings_08e4e6.s)
/// $08F57A: move.b #$ff, $5(a1, d7.w)
pub fn i_08f57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F580;
    cpu.ir = 0x13BC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F580: move.w $4(a0), d7
pub fn i_08f580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F584;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F584: cmp.w $2(a0), d7
pub fn i_08f584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F588;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F588: beq.w $8f5ac
pub fn i_08f588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F58C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F5AC; return 10; }
    8
}
/// $08F58C: cmp.b $4(a1, d7.w), d2
pub fn i_08f58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F590;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F590: bne.w $8f59e
pub fn i_08f590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F594;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F59E; return 10; }
    8
}
/// $08F594: move.b d3, $5(a1, d7.w)
pub fn i_08f594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F598;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
// ===== SetC_08f598  ($08F598, 6 B, thunks/ccr_helpers.c)
/// $08F598: ori.b #$1, ccr
pub fn i_08f598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F59C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F59C: rts
pub fn i_08f59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F59E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_Next_08f59e  ($08F59E, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F59E: addq.w #$6, d7
pub fn i_08f59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A0;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5A0: cmpi.w #$30, d7
pub fn i_08f5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A4;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F5A4: bcs.w $8f5aa
pub fn i_08f5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F5AA; return 10; }
    8
}
/// $08F5A8: clr.w d7
pub fn i_08f5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5AA;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5AA: bra.b $8f584
pub fn i_08f5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5AC;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F584;
    10
}
// ===== TargetRing_ClaimById_Append_08f5ac  ($08F5AC, 42 B, enemies/critters_rings_08e4e6.s)
/// $08F5AC: move.b d2, $4(a1, d7.w)
pub fn i_08f5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5B0;
    cpu.ir = 0x1382;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F5B0: move.b d3, $5(a1, d7.w)
pub fn i_08f5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5B4;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F5B4: cmpi.w #$4, $6(a0)
pub fn i_08f5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5BA;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F5BA: bcc.w $8f5d6
pub fn i_08f5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F5D6; return 10; }
    8
}
/// $08F5BE: move.w $2(a0), d7
pub fn i_08f5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C2;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F5C2: addq.w #$6, d7
pub fn i_08f5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C4;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5C4: cmpi.w #$30, d7
pub fn i_08f5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C8;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F5C8: bcs.w $8f5ce
pub fn i_08f5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F5CE; return 10; }
    8
}
/// $08F5CC: clr.w d7
pub fn i_08f5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5CE;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5CE: move.w d7, $2(a0)
pub fn i_08f5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5D2;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F5D2: addq.w #$1, $6(a0)
pub fn i_08f5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5D6;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetC_08f5d6  ($08F5D6, 6 B, thunks/ccr_helpers.c)
/// $08F5D6: ori.b #$1, ccr
pub fn i_08f5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5DA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F5DA: rts
pub fn i_08f5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimByKey_08f5dc  ($08F5DC, 50 B, enemies/critters_rings_08e4e6.s)
/// $08F5DC: lea.l $10e362.l, a0
pub fn i_08f5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F5E2: lea.l $8(a0), a1
pub fn i_08f5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5E6;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F5E6: move.w (a0), d7
pub fn i_08f5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5E8;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F5E8: cmp.w $4(a0), d7
pub fn i_08f5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5EC;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F5EC: beq.w $8f60e
pub fn i_08f5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F60E; return 10; }
    8
}
/// $08F5F0: cmp.b $4(a1, d7.w), d0
pub fn i_08f5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5F4;
    cpu.ir = 0xB031;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F5F4: bne.w $8f600
pub fn i_08f5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F600; return 10; }
    8
}
/// $08F5F8: tst.b $5(a1, d7.w)
pub fn i_08f5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5FC;
    cpu.ir = 0x4A31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $08F5FC: bpl.w $8f614
pub fn i_08f5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F600;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8F614; return 10; }
    8
}
/// $08F600: addq.w #$6, d7
pub fn i_08f600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F602;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F602: cmpi.w #$30, d7
pub fn i_08f602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F606;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F606: bcs.w $8f60c
pub fn i_08f606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F60A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F60C; return 10; }
    8
}
/// $08F60A: clr.w d7
pub fn i_08f60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F60C;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F60C: bra.b $8f5e8
pub fn i_08f60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F60E;
    cpu.ir = 0x60DA;
    cpu.pc = 0x8F5E8;
    10
}
// ===== ClearC_08f60e  ($08F60E, 6 B, thunks/ccr_helpers.c)
/// $08F60E: andi.b #$fe, ccr
pub fn i_08f60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F612;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F612: rts
pub fn i_08f612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F614;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimByKey_Found_08f614  ($08F614, 66 B, enemies/critters_rings_08e4e6.s)
/// $08F614: move.b d0, d2
pub fn i_08f614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F616;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F616: move.w (a1, d7.w), d0
pub fn i_08f616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F61A;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F61A: move.w $2(a1, d7.w), d1
pub fn i_08f61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F61E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F61E: move.b #$0, d3
pub fn i_08f61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F622;
    cpu.ir = 0x163C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F622: cmpa.l #$100440, a6
pub fn i_08f622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F628;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F628: beq.w $8f63e
pub fn i_08f628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F62C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F63E; return 10; }
    8
}
/// $08F62C: move.b #$1, d3
pub fn i_08f62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F630;
    cpu.ir = 0x163C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F630: cmpa.l #$1004e0, a6
pub fn i_08f630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F636;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F636: beq.w $8f63e
pub fn i_08f636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F63A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F63E; return 10; }
    8
}
/// $08F63A: move.b #$2, d3
pub fn i_08f63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F63E;
    cpu.ir = 0x163C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F63E: move.w $4(a0), d7
pub fn i_08f63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F642;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F642: cmp.w $2(a0), d7
pub fn i_08f642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F646;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F646: beq.w $8f66a
pub fn i_08f646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F64A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F66A; return 10; }
    8
}
/// $08F64A: cmp.b $4(a1, d7.w), d2
pub fn i_08f64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F64E;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F64E: bne.w $8f65c
pub fn i_08f64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F652;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F65C; return 10; }
    8
}
/// $08F652: move.b d3, $5(a1, d7.w)
pub fn i_08f652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F656;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
// ===== SetC_08f656  ($08F656, 6 B, thunks/ccr_helpers.c)
/// $08F656: ori.b #$1, ccr
pub fn i_08f656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F65A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F65A: rts
pub fn i_08f65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F65C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimByKey_Next_08f65c  ($08F65C, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F65C: addq.w #$6, d7
pub fn i_08f65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F65E;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F65E: cmpi.w #$30, d7
pub fn i_08f65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F662;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F662: bcs.w $8f668
pub fn i_08f662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F666;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F668; return 10; }
    8
}
/// $08F666: clr.w d7
pub fn i_08f666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F668;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F668: bra.b $8f642
pub fn i_08f668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F66A;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F642;
    10
}
// ===== TargetRing_ClaimByKey_Append_08f66a  ($08F66A, 44 B, enemies/critters_rings_08e4e6.s)
/// $08F66A: move.b d2, $4(a1, d7.w)
pub fn i_08f66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F66E;
    cpu.ir = 0x1382;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F66E: move.b #$ff, $5(a1, d7.w)
pub fn i_08f66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F674;
    cpu.ir = 0x13BC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F674: cmpi.w #$4, $6(a0)
pub fn i_08f674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F67A;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F67A: bcc.w $8f696
pub fn i_08f67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F67E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F696; return 10; }
    8
}
/// $08F67E: move.w $2(a0), d7
pub fn i_08f67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F682;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F682: addq.w #$6, d7
pub fn i_08f682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F684;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F684: cmpi.w #$30, d7
pub fn i_08f684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F688;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F688: bcs.w $8f68e
pub fn i_08f688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F68C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F68E; return 10; }
    8
}
/// $08F68C: clr.w d7
pub fn i_08f68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F68E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F68E: move.w d7, $2(a0)
pub fn i_08f68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F692;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F692: addq.w #$1, $6(a0)
pub fn i_08f692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F696;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetC_08f696  ($08F696, 6 B, thunks/ccr_helpers.c)
/// $08F696: ori.b #$1, ccr
pub fn i_08f696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F69A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F69A: rts
pub fn i_08f69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F69C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_SndByState_08f69c  ($08F69C, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F69C: move.w #$134, d1
pub fn i_08f69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6A0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x134;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F6A0: tst.b d4
pub fn i_08f6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6A2;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F6A2: beq.w $8f6b4
pub fn i_08f6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F6B4; return 10; }
    8
}
/// $08F6A6: move.w #$152, d1
pub fn i_08f6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x152;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F6AA: subq.b #$1, d4
pub fn i_08f6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6AC;
    cpu.ir = 0x5304;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F6AC: beq.w $8f6b4
pub fn i_08f6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F6B4; return 10; }
    8
}
/// $08F6B0: move.w #$e, d1
pub fn i_08f6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F6B4: rts
pub fn i_08f6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08f6b6  ($08F6B6, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F6B6: movea.l $8(a6), a1
pub fn i_08f6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6BA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08F6BA: move.b $10(a6), d0
pub fn i_08f6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F6BE: cmp.b $10(a1), d0
pub fn i_08f6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6C2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08F6C2: bcs.w $8f6cc
pub fn i_08f6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F6CC; return 10; }
    8
}
// ===== ClearXN_08f6c6  ($08F6C6, 6 B, thunks/ccr_helpers.c)
/// $08F6C6: andi.b #$ee, ccr
pub fn i_08f6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F6CA: rts
pub fn i_08f6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08f6cc  ($08F6CC, 6 B, thunks/ccr_helpers.c)
/// $08F6CC: ori.b #$11, ccr
pub fn i_08f6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6D0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F6D0: rts
pub fn i_08f6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_MaskF0F0_08f6d2  ($08F6D2, 8 B, mission/gameover_continue_08f6d2.s)
/// $08F6D2: move.w #$f0f0, $96(a6)
pub fn i_08f6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0F0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F6D8: rts
pub fn i_08f6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_MaskCurF0_08f6da  ($08F6DA, 24 B, mission/gameover_continue_08f6d2.s)
/// $08F6DA: move.b $106f28.l, d0
pub fn i_08f6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F6E0: not.b d0
pub fn i_08f6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E2;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F6E2: andi.w #$1, d0
pub fn i_08f6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6E6: addi.w #$96, d0
pub fn i_08f6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6EA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6EA: move.b #$f0, (a6, d0.w)
pub fn i_08f6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F0;
    cpu.ir = 0x1DBC;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F6F0: rts
pub fn i_08f6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_SetLowNibble_08f6f2  ($08F6F2, 34 B, mission/gameover_continue_08f6d2.s)
/// $08F6F2: move.b $106f28.l, d0
pub fn i_08f6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F6F8: not.b d0
pub fn i_08f6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6FA;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F6FA: andi.w #$1, d0
pub fn i_08f6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6FE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6FE: addi.w #$96, d0
pub fn i_08f6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F702;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F702: lsl.b #$4, d1
pub fn i_08f702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F704;
    cpu.ir = 0xE909;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $08F704: move.b (a6, d0.w), d2
pub fn i_08f704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F708;
    cpu.ir = 0x1436;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F708: andi.b #$f0, d2
pub fn i_08f708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F70C;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08F70C: or.b d2, d1
pub fn i_08f70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F70E;
    cpu.ir = 0x8202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F70E: or.b d1, (a6, d0.w)
pub fn i_08f70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F712;
    cpu.ir = 0x8336;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $08F712: rts
pub fn i_08f712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F714;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_TestMaskCur_08f714  ($08F714, 24 B, mission/gameover_continue_08f6d2.s)
/// $08F714: move.b $106f28.l, d0
pub fn i_08f714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F71A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F71A: andi.w #$1, d0
pub fn i_08f71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F71E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F71E: addi.w #$96, d0
pub fn i_08f71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F722;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F722: move.b (a6, d0.w), d0
pub fn i_08f722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F726;
    cpu.ir = 0x1036;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F726: and.b d0, d1
pub fn i_08f726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F728;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F728: beq.w $8f732
pub fn i_08f728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F72C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F732; return 10; }
    8
}
// ===== SetXN_08f72c  ($08F72C, 6 B, thunks/ccr_helpers.c)
/// $08F72C: ori.b #$11, ccr
pub fn i_08f72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F730;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F730: rts
pub fn i_08f730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F732;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_08f732  ($08F732, 6 B, thunks/ccr_helpers.c)
/// $08F732: andi.b #$ee, ccr
pub fn i_08f732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F736;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F736: rts
pub fn i_08f736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F738;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anchor_GetWorldPos_08f738  ($08F738, 54 B, mission/gameover_continue_08f6d2.s)
/// $08F738: move.l $70(a0), d1
pub fn i_08f738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F73C;
    cpu.ir = 0x2228;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08F73C: bmi.w $8f774
pub fn i_08f73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F740;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x8F774; return 10; }
    8
}
/// $08F740: movea.l d1, a1
pub fn i_08f740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F742;
    cpu.ir = 0x2241;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08F742: movea.l (a1), a2
pub fn i_08f742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F744;
    cpu.ir = 0x2451;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[2] = v2;
    12
}
/// $08F744: andi.w #$ff, d0
pub fn i_08f744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F748;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F748: add.w d0, d0
pub fn i_08f748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F74A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F74A: add.w d0, d0
pub fn i_08f74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F74C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F74C: lea.l (a2, d0.w), a2
pub fn i_08f74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F750;
    cpu.ir = 0x45F2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $08F750: move.w (a2), d0
pub fn i_08f750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F752;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F752: move.w $2(a2), d1
pub fn i_08f752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F756;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F756: move.w d0, d2
pub fn i_08f756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F758;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F758: or.w d1, d2
pub fn i_08f758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F75A;
    cpu.ir = 0x8441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $08F75A: beq.w $8f774
pub fn i_08f75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F75E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F774; return 10; }
    8
}
/// $08F75E: add.w $22(a0), d0
pub fn i_08f75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F762;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F762: add.w $24(a0), d1
pub fn i_08f762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F766;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F766: add.w $4(a1), d0
pub fn i_08f766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F76A;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F76A: add.w $6(a1), d1
pub fn i_08f76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F76E;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
// ===== SetC_08f76e  ($08F76E, 6 B, thunks/ccr_helpers.c)
/// $08F76E: ori.b #$1, ccr
pub fn i_08f76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F772;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F772: rts
pub fn i_08f772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F774;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08f774  ($08F774, 6 B, thunks/ccr_helpers.c)
/// $08F774: andi.b #$fe, ccr
pub fn i_08f774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F778;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F778: rts
pub fn i_08f778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F77A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anchor_GetWorldPosOrDefault_08f77a  ($08F77A, 22 B, mission/gameover_continue_08f6d2.s)
/// $08F77A: move.b d0, d7
pub fn i_08f77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F77C;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F77C: bsr.b $8f738
pub fn i_08f77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F77E;
    cpu.ir = 0x61BA;
    cpu.push32(bus, 0x8F77E);
    cpu.pc = 0x8F738;
    18
}
/// $08F77E: bcc.w $8f794
pub fn i_08f77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F782;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F794; return 10; }
    8
}
/// $08F782: tst.b d7
pub fn i_08f782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F784;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F784: bne.w $8f790
pub fn i_08f784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F788;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F790; return 10; }
    8
}
/// $08F788: addi.w #$10, d0
pub fn i_08f788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F78C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F78C: addi.w #$e, d1
pub fn i_08f78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F790;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xE, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetC_08f790  ($08F790, 6 B, thunks/ccr_helpers.c)
/// $08F790: ori.b #$1, ccr
pub fn i_08f790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F794;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F794: rts
pub fn i_08f794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F796;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08f796  ($08F796, 6 B, thunks/ccr_helpers.c)
/// $08F796: andi.b #$fe, ccr
pub fn i_08f796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F79A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F79A: rts
pub fn i_08f79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F79C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_FoldMask_08f79c  ($08F79C, 16 B, mission/gameover_continue_08f6d2.s)
/// $08F79C: move.b $96(a0), d0
pub fn i_08f79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A0;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F7A0: or.b $97(a0), d0
pub fn i_08f7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A4;
    cpu.ir = 0x8028;
    let ad2 = cpu.a[0].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08F7A4: move.b d0, d1
pub fn i_08f7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A6;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F7A6: lsr.b #$4, d1
pub fn i_08f7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A8;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $08F7A8: or.b d1, d0
pub fn i_08f7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7AA;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F7AA: rts
pub fn i_08f7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_Resolve_08f7ac  ($08F7AC, 34 B, mission/gameover_continue_08f6d2.s)
/// $08F7AC: move.l a0, d0
pub fn i_08f7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7AE;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F7AE: beq.w $8f7cc
pub fn i_08f7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F7CC; return 10; }
    8
}
/// $08F7B2: jsr $2ac0e.l
pub fn i_08f7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F7B8);
    cpu.pc = 0x2AC0E;
    16
}
/// $08F7B8: bcc.w $8f7c6
pub fn i_08f7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F7C6; return 10; }
    8
}
/// $08F7BC: lea.l $100580.l, a0
pub fn i_08f7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $08F7C2: bra.w $8f7cc
pub fn i_08f7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F7CC;
    10
}
/// $08F7C6: movea.l #$0, a0
pub fn i_08f7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x0;
    cpu.a[0] = v1;
    4
}
/// $08F7CC: rts
pub fn i_08f7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_FindFreeAnchor_08f7ce  ($08F7CE, 88 B, mission/gameover_continue_08f6d2.s)
/// $08F7CE: bsr.b $8f7ac
pub fn i_08f7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D0;
    cpu.ir = 0x61DC;
    cpu.push32(bus, 0x8F7D0);
    cpu.pc = 0x8F7AC;
    18
}
/// $08F7D0: move.l a0, d0
pub fn i_08f7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D2;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F7D2: beq.w $8f826
pub fn i_08f7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F826; return 10; }
    8
}
/// $08F7D6: bsr.b $8f79c
pub fn i_08f7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D8;
    cpu.ir = 0x61C4;
    cpu.push32(bus, 0x8F7D8);
    cpu.pc = 0x8F79C;
    18
}
/// $08F7D8: move.w d0, d7
pub fn i_08f7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7DA;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F7DA: btst.b #$0, d7
pub fn i_08f7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7DE;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F7DE: bne.w $8f7f2
pub fn i_08f7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F7F2; return 10; }
    8
}
/// $08F7E2: move.b #$0, d0
pub fn i_08f7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7E6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F7E6: bsr.w $8f738
pub fn i_08f7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F7EA);
    cpu.pc = 0x8F738;
    18
}
/// $08F7EA: bcs.w $8f7f2
pub fn i_08f7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F7F2; return 10; }
    8
}
/// $08F7EE: bset.b #$0, d7
pub fn i_08f7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7F2;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $08F7F2: btst.b #$1, d7
pub fn i_08f7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7F6;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F7F6: bne.w $8f80a
pub fn i_08f7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F80A; return 10; }
    8
}
/// $08F7FA: move.b #$1, d0
pub fn i_08f7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7FE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F7FE: bsr.w $8f738
pub fn i_08f7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F802;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F802);
    cpu.pc = 0x8F738;
    18
}
/// $08F802: bcs.w $8f80a
pub fn i_08f802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F806;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F80A; return 10; }
    8
}
/// $08F806: bset.b #$1, d7
pub fn i_08f806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F80A;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $08F80A: btst.b #$2, d7
pub fn i_08f80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F80E;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F80E: bne.w $8f822
pub fn i_08f80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F812;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F822; return 10; }
    8
}
/// $08F812: move.b #$2, d0
pub fn i_08f812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F816;
    cpu.ir = 0x103C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F816: bsr.w $8f738
pub fn i_08f816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F81A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F81A);
    cpu.pc = 0x8F738;
    18
}
/// $08F81A: bcs.w $8f822
pub fn i_08f81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F81E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F822; return 10; }
    8
}
/// $08F81E: bset.b #$2, d7
pub fn i_08f81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F822;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $08F822: move.b d7, d0
pub fn i_08f822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F824;
    cpu.ir = 0x1007;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F824: rts
pub fn i_08f824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F826;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RetMinus1_0008F826  ($08F826, 4 B, thunks/trivial_returns.c)
/// $08F826: moveq #$ff, d0
pub fn i_08f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F828;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F828: rts
pub fn i_08f828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F82A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_TryAnchor_08f82a  ($08F82A, 44 B, mission/gameover_continue_08f6d2.s)
/// $08F82A: movem.w d0, -(a7)
pub fn i_08f82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F82E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $08F82E: bsr.w $8f7ac
pub fn i_08f82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F832;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F832);
    cpu.pc = 0x8F7AC;
    18
}
/// $08F832: move.l a0, d0
pub fn i_08f832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F834;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F834: bne.w $8f840
pub fn i_08f834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F838;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F840; return 10; }
    8
}
/// $08F838: movem.w (a7)+, d0
pub fn i_08f838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F83C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08F83C: bra.w $8f856
pub fn i_08f83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F840;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F856;
    10
}
/// $08F840: bsr.w $8f79c
pub fn i_08f840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F844;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F844);
    cpu.pc = 0x8F79C;
    18
}
/// $08F844: movem.w (a7)+, d1
pub fn i_08f844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F848;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08F848: btst.l d1, d0
pub fn i_08f848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F84A;
    cpu.ir = 0x300;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F84A: bne.w $8f856
pub fn i_08f84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F84E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F856; return 10; }
    8
}
/// $08F84E: move.b d1, d0
pub fn i_08f84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F850;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F850: bsr.w $8f77a
pub fn i_08f850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F854;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F854);
    cpu.pc = 0x8F77A;
    18
}
/// $08F854: rts
pub fn i_08f854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F856;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08f856  ($08F856, 6 B, thunks/ccr_helpers.c)
/// $08F856: andi.b #$fe, ccr
pub fn i_08f856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F85A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F85A: rts
pub fn i_08f85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F85C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_ClaimAnchor_08f85c  ($08F85C, 34 B, mission/gameover_continue_08f6d2.s)
/// $08F85C: movem.w d0, -(a7)
pub fn i_08f85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F860;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $08F860: bsr.b $8f82a
pub fn i_08f860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F862;
    cpu.ir = 0x61C8;
    cpu.push32(bus, 0x8F862);
    cpu.pc = 0x8F82A;
    18
}
/// $08F862: movem.w (a7)+, d2
pub fn i_08f862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F866;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08F866: bcc.w $8f882
pub fn i_08f866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F86A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F882; return 10; }
    8
}
/// $08F86A: move.b $106f28.l, d3
pub fn i_08f86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F870;
    cpu.ir = 0x1639;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F870: not.b d3
pub fn i_08f870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F872;
    cpu.ir = 0x4603;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F872: andi.w #$1, d3
pub fn i_08f872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F876;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F876: addi.w #$96, d3
pub fn i_08f876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F87A;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F87A: bset.b d2, (a0, d3.w)
pub fn i_08f87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F87E;
    cpu.ir = 0x5F0;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    18
}
// ===== SetC_08f87e  ($08F87E, 6 B, thunks/ccr_helpers.c)
/// $08F87E: ori.b #$1, ccr
pub fn i_08f87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F882;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F882: rts
pub fn i_08f882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F884;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_ClaimAnchorIfFree_08f884  ($08F884, 56 B, mission/gameover_continue_08f6d2.s)
/// $08F884: move.w d0, -(a7)
pub fn i_08f884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F886;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $08F886: bsr.w $8f7ac
pub fn i_08f886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F88A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F88A);
    cpu.pc = 0x8F7AC;
    18
}
/// $08F88A: move.w (a7)+, d0
pub fn i_08f88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F88C;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $08F88C: move.l a0, d7
pub fn i_08f88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F88E;
    cpu.ir = 0x2E08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F88E: beq.w $8f8bc
pub fn i_08f88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F892;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F8BC; return 10; }
    8
}
/// $08F892: move.b $96(a0), d2
pub fn i_08f892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F896;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F896: or.b $97(a0), d2
pub fn i_08f896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F89A;
    cpu.ir = 0x8428;
    let ad2 = cpu.a[0].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08F89A: lsr.b #$4, d2
pub fn i_08f89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F89C;
    cpu.ir = 0xE80A;
    let h1 = cpu.d[2] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    14
}
/// $08F89C: btst.l d0, d2
pub fn i_08f89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F89E;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F89E: bne.w $8f8bc
pub fn i_08f89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F8BC; return 10; }
    8
}
/// $08F8A2: move.b $106f28.l, d3
pub fn i_08f8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8A8;
    cpu.ir = 0x1639;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F8A8: not.b d3
pub fn i_08f8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8AA;
    cpu.ir = 0x4603;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F8AA: andi.w #$1, d3
pub fn i_08f8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8AE;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8AE: addi.w #$96, d3
pub fn i_08f8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8B2;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8B2: bset.b d0, (a0, d3.w)
pub fn i_08f8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8B6;
    cpu.ir = 0x1F0;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    18
}
/// $08F8B6: bsr.w $8f77a
pub fn i_08f8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8BA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F8BA);
    cpu.pc = 0x8F77A;
    18
}
/// $08F8BA: rts
pub fn i_08f8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08f8bc  ($08F8BC, 6 B, thunks/ccr_helpers.c)
/// $08F8BC: andi.b #$fe, ccr
pub fn i_08f8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8C0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F8C0: rts
pub fn i_08f8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_SetBit3Cur_08f8c2  ($08F8C2, 32 B, mission/gameover_continue_08f6d2.s)
/// $08F8C2: bsr.w $8f7ac
pub fn i_08f8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8C6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F8C6);
    cpu.pc = 0x8F7AC;
    18
}
/// $08F8C6: move.l a0, d0
pub fn i_08f8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8C8;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F8C8: beq.b $8f8bc
pub fn i_08f8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8CA;
    cpu.ir = 0x67F2;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F8BC; return 10; }
    8
}
/// $08F8CA: move.b $106f28.l, d0
pub fn i_08f8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8D0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F8D0: not.b d0
pub fn i_08f8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8D2;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F8D2: andi.w #$1, d0
pub fn i_08f8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8D6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8D6: addi.w #$96, d0
pub fn i_08f8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8DA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8DA: bset.b #$3, (a0, d0.w)
pub fn i_08f8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8E0;
    cpu.ir = 0x8F0;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    18
}
/// $08F8E0: rts
pub fn i_08f8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_ClearP3Cur_08f8e2  ($08F8E2, 28 B, mission/gameover_continue_08f6d2.s)
/// $08F8E2: move.b $106f28.l, d0
pub fn i_08f8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8E8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F8E8: not.b d0
pub fn i_08f8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8EA;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F8EA: andi.w #$1, d0
pub fn i_08f8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8EE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8EE: addi.w #$96, d0
pub fn i_08f8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8F2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8F2: lea.l $100580.l, a0
pub fn i_08f8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $08F8F8: clr.b (a0, d0.w)
pub fn i_08f8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8FC;
    cpu.ir = 0x4230;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $08F8FC: rts
pub fn i_08f8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08f8fe  ($08F8FE, 16 B, mission/gameover_continue_08f6d2.s)
/// $08F8FE: movea.l $8(a6), a1
pub fn i_08f8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F902;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08F902: move.b $10(a6), d0
pub fn i_08f902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F906;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F906: cmp.b $10(a1), d0
pub fn i_08f906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F90A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08F90A: bcs.w $8f914
pub fn i_08f90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F90E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F914; return 10; }
    8
}
// ===== ClearXN_08f90e  ($08F90E, 6 B, thunks/ccr_helpers.c)
/// $08F90E: andi.b #$ee, ccr
pub fn i_08f90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F912;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F912: rts
pub fn i_08f912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F914;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08f914  ($08F914, 6 B, thunks/ccr_helpers.c)
/// $08F914: ori.b #$11, ccr
pub fn i_08f914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F918;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F918: rts
pub fn i_08f918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F91A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Boot_08f91a  ($08F91A, 72 B, mission/gameover_continue_08f6d2.s)
/// $08F91A: move.b #$a, d0
pub fn i_08f91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F91E;
    cpu.ir = 0x103C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F91E: jsr $43568.l
pub fn i_08f91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F924);
    cpu.pc = 0x43568;
    16
}
/// $08F924: lea.l $2f4a32.l, a0
pub fn i_08f924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F92A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4A32;
    20
}
/// $08F92A: jsr $2b58.l
pub fn i_08f92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F930);
    cpu.pc = 0x2B58;
    16
}
/// $08F930: move.w #$90, $22(a6)
pub fn i_08f930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F936;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F936: move.w #$1c0, $24(a6)
pub fn i_08f936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F93C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F93C: move.w #$ff00, $2a(a6)
pub fn i_08f93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F942;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F942: moveq #$0, d0
pub fn i_08f942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F944;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F944: move.b d0, $20(a6)
pub fn i_08f944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F948;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F948: move.b d0, $21(a6)
pub fn i_08f948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F94C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F94C: move.w #$e, $82(a6)
pub fn i_08f94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F952;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F952: move.w d0, $86(a6)
pub fn i_08f952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F956;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F956: move.w d0, $84(a6)
pub fn i_08f956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F95A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F95A: moveq #$2, d0
pub fn i_08f95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F95C;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F95C: jsr $523b2.l
pub fn i_08f95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F962);
    cpu.pc = 0x523B2;
    16
}
// ===== SetTaskHandler_08f962  ($08F962, 8 B, kernel/task_handlers.c)
/// $08F962: lea.l $8f96a(pc), a1
pub fn i_08f962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F966;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8F96A;
    16
}
/// $08F966: move.l a1, (a6)
pub fn i_08f966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F968;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08F968: rts
pub fn i_08f968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F96A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Spawn_08f96a  ($08F96A, 824 B, mission/gameover_continue_08f6d2.s)
/// $08F96A: lea.l $91514(pc), a1
pub fn i_08f96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F96E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91514;
    16
}
/// $08F96E: jsr $4ae.l
pub fn i_08f96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F974);
    cpu.pc = 0x4AE;
    16
}
/// $08F974: lea.l $90db0(pc), a1
pub fn i_08f974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F978;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90DB0;
    16
}
/// $08F978: jsr $4ae.l
pub fn i_08f978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F97E);
    cpu.pc = 0x4AE;
    16
}
/// $08F97E: jsr $5dd02.l
pub fn i_08f97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F984);
    cpu.pc = 0x5DD02;
    16
}
/// $08F984: lea.l $90de2(pc), a1
pub fn i_08f984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F988;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90DE2;
    16
}
/// $08F988: jsr $4ae.l
pub fn i_08f988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F98E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F98E);
    cpu.pc = 0x4AE;
    16
}
/// $08F98E: jsr $5dd02.l
pub fn i_08f98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F994;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F994);
    cpu.pc = 0x5DD02;
    16
}
/// $08F994: lea.l $90f00(pc), a1
pub fn i_08f994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F998;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90F00;
    16
}
/// $08F998: jsr $4ae.l
pub fn i_08f998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F99E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F99E);
    cpu.pc = 0x4AE;
    16
}
/// $08F99E: jsr $5dd02.l
pub fn i_08f99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9A4);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9A4: move.w #$0, $80(a0)
pub fn i_08f9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9AA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9AA: lea.l $90f68(pc), a1
pub fn i_08f9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90F68;
    16
}
/// $08F9AE: jsr $4ae.l
pub fn i_08f9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9B4);
    cpu.pc = 0x4AE;
    16
}
/// $08F9B4: jsr $5dd02.l
pub fn i_08f9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9BA);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9BA: move.w #$0, $80(a0)
pub fn i_08f9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9C0: lea.l $90f00(pc), a1
pub fn i_08f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90F00;
    16
}
/// $08F9C4: jsr $4ae.l
pub fn i_08f9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9CA);
    cpu.pc = 0x4AE;
    16
}
/// $08F9CA: jsr $5dd02.l
pub fn i_08f9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9D0);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9D0: move.w #$1, $80(a0)
pub fn i_08f9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9D6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9D6: lea.l $90e86(pc), a1
pub fn i_08f9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08F9DA: jsr $4ae.l
pub fn i_08f9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9E0);
    cpu.pc = 0x4AE;
    16
}
/// $08F9E0: jsr $5dd02.l
pub fn i_08f9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9E6);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9E6: move.w #$0, $80(a0)
pub fn i_08f9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9EC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9EC: lea.l $90e86(pc), a1
pub fn i_08f9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08F9F0: jsr $4ae.l
pub fn i_08f9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9F6);
    cpu.pc = 0x4AE;
    16
}
/// $08F9F6: jsr $5dd02.l
pub fn i_08f9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9FC);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9FC: move.w #$1, $80(a0)
pub fn i_08f9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA02;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA02: lea.l $90e86(pc), a1
pub fn i_08fa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA06: jsr $4ae.l
pub fn i_08fa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA0C);
    cpu.pc = 0x4AE;
    16
}
/// $08FA0C: jsr $5dd02.l
pub fn i_08fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA12);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA12: move.w #$2, $80(a0)
pub fn i_08fa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA18;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA18: lea.l $90e86(pc), a1
pub fn i_08fa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA1C: jsr $4ae.l
pub fn i_08fa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA22);
    cpu.pc = 0x4AE;
    16
}
/// $08FA22: jsr $5dd02.l
pub fn i_08fa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA28);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA28: move.w #$3, $80(a0)
pub fn i_08fa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA2E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA2E: lea.l $90e86(pc), a1
pub fn i_08fa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA32: jsr $4ae.l
pub fn i_08fa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA38);
    cpu.pc = 0x4AE;
    16
}
/// $08FA38: jsr $5dd02.l
pub fn i_08fa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA3E);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA3E: move.w #$4, $80(a0)
pub fn i_08fa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA44;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA44: lea.l $90e86(pc), a1
pub fn i_08fa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA48: jsr $4ae.l
pub fn i_08fa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA4E);
    cpu.pc = 0x4AE;
    16
}
/// $08FA4E: jsr $5dd02.l
pub fn i_08fa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA54);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA54: move.w #$5, $80(a0)
pub fn i_08fa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA5A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA5A: lea.l $90e86(pc), a1
pub fn i_08fa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA5E: jsr $4ae.l
pub fn i_08fa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA64);
    cpu.pc = 0x4AE;
    16
}
/// $08FA64: jsr $5dd02.l
pub fn i_08fa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA6A);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA6A: move.w #$6, $80(a0)
pub fn i_08fa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA70;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA70: lea.l $90e86(pc), a1
pub fn i_08fa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA74: jsr $4ae.l
pub fn i_08fa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA7A);
    cpu.pc = 0x4AE;
    16
}
/// $08FA7A: jsr $5dd02.l
pub fn i_08fa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA80);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA80: move.w #$7, $80(a0)
pub fn i_08fa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA86;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA86: lea.l $90e86(pc), a1
pub fn i_08fa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA8A: jsr $4ae.l
pub fn i_08fa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA90);
    cpu.pc = 0x4AE;
    16
}
/// $08FA90: jsr $5dd02.l
pub fn i_08fa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA96);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA96: move.w #$8, $80(a0)
pub fn i_08fa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA9C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA9C: lea.l $90e86(pc), a1
pub fn i_08fa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FAA0: jsr $4ae.l
pub fn i_08faa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAA6);
    cpu.pc = 0x4AE;
    16
}
/// $08FAA6: jsr $5dd02.l
pub fn i_08faa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAAC);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAAC: move.w #$9, $80(a0)
pub fn i_08faac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAB2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FAB2: lea.l $9076e(pc), a1
pub fn i_08fab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9076E;
    16
}
/// $08FAB6: jsr $4ae.l
pub fn i_08fab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FABC);
    cpu.pc = 0x4AE;
    16
}
/// $08FABC: jsr $5dd02.l
pub fn i_08fabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAC2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAC2: lea.l $90728(pc), a1
pub fn i_08fac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90728;
    16
}
/// $08FAC6: jsr $4ae.l
pub fn i_08fac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FACC);
    cpu.pc = 0x4AE;
    16
}
/// $08FACC: jsr $5dd02.l
pub fn i_08facc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAD2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAD2: lea.l $907b4(pc), a1
pub fn i_08fad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x907B4;
    16
}
/// $08FAD6: jsr $4ae.l
pub fn i_08fad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FADC);
    cpu.pc = 0x4AE;
    16
}
/// $08FADC: jsr $5dd02.l
pub fn i_08fadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAE2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAE2: lea.l $907fa(pc), a1
pub fn i_08fae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x907FA;
    16
}
/// $08FAE6: jsr $4ae.l
pub fn i_08fae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAEC);
    cpu.pc = 0x4AE;
    16
}
/// $08FAEC: jsr $5dd02.l
pub fn i_08faec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAF2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAF2: lea.l $90840(pc), a1
pub fn i_08faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90840;
    16
}
/// $08FAF6: jsr $4ae.l
pub fn i_08faf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAFC);
    cpu.pc = 0x4AE;
    16
}
/// $08FAFC: jsr $5dd02.l
pub fn i_08fafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB02);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB02: lea.l $9088e(pc), a1
pub fn i_08fb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9088E;
    16
}
/// $08FB06: jsr $4ae.l
pub fn i_08fb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB0C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB0C: jsr $5dd02.l
pub fn i_08fb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB12);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB12: lea.l $908d4(pc), a1
pub fn i_08fb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x908D4;
    16
}
/// $08FB16: jsr $4ae.l
pub fn i_08fb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB1C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB1C: jsr $5dd02.l
pub fn i_08fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB22);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB22: lea.l $9091a(pc), a1
pub fn i_08fb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9091A;
    16
}
/// $08FB26: jsr $4ae.l
pub fn i_08fb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB2C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB2C: jsr $5dd02.l
pub fn i_08fb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB32);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB32: lea.l $90968(pc), a1
pub fn i_08fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90968;
    16
}
/// $08FB36: jsr $4ae.l
pub fn i_08fb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB3C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB3C: jsr $5dd02.l
pub fn i_08fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB42);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB42: lea.l $909ae(pc), a1
pub fn i_08fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x909AE;
    16
}
/// $08FB46: jsr $4ae.l
pub fn i_08fb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB4C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB4C: jsr $5dd02.l
pub fn i_08fb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB52);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB52: lea.l $909f4(pc), a1
pub fn i_08fb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x909F4;
    16
}
/// $08FB56: jsr $4ae.l
pub fn i_08fb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB5C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB5C: jsr $5dd02.l
pub fn i_08fb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB62);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB62: lea.l $90a3a(pc), a1
pub fn i_08fb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90A3A;
    16
}
/// $08FB66: jsr $4ae.l
pub fn i_08fb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB6C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB6C: jsr $5dd02.l
pub fn i_08fb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB72);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB72: lea.l $90a80(pc), a1
pub fn i_08fb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90A80;
    16
}
/// $08FB76: jsr $4ae.l
pub fn i_08fb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB7C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB7C: jsr $5dd02.l
pub fn i_08fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB82);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB82: lea.l $90ac6(pc), a1
pub fn i_08fb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90AC6;
    16
}
/// $08FB86: jsr $4ae.l
pub fn i_08fb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB8C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB8C: jsr $5dd02.l
pub fn i_08fb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB92);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB92: move.w #$10df, d0
pub fn i_08fb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB96;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FB96: jsr $2352.l
pub fn i_08fb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB9C);
    cpu.pc = 0x2352;
    16
}
/// $08FB9C: lea.l $8fba2(pc), a1
pub fn i_08fb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FBA2;
    16
}
/// $08FBA0: move.l a1, (a6)
pub fn i_08fba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FBA2: addq.w #$1, $86(a6)
pub fn i_08fba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBA6;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FBA6: move.w $86(a6), d0
pub fn i_08fba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBAA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FBAA: andi.w #$1, d0
pub fn i_08fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBAE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08FBAE: beq.w $8fc8c
pub fn i_08fbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FBB2: cmpi.w #$178, $24(a6)
pub fn i_08fbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBB8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x178, Size::Word, false);
    16
}
/// $08FBB8: ble.w $8fc8c
pub fn i_08fbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBBC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FBBC: jsr $8fcaa(pc)
pub fn i_08fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FBC0);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FBC0: cmpi.w #$1a8, $24(a6)
pub fn i_08fbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBC6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1A8, Size::Word, false);
    16
}
/// $08FBC6: bne.w $8fc26
pub fn i_08fbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FC26; return 10; }
    8
}
/// $08FBCA: cmpi.b #$2, $21(a6)
pub fn i_08fbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBD0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08FBD0: beq.w $8fc26
pub fn i_08fbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FC26; return 10; }
    8
}
/// $08FBD4: move.b #$2, $21(a6)
pub fn i_08fbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FBDA: move.w #$10df, d0
pub fn i_08fbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FBDE: jsr $2222.l
pub fn i_08fbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FBE4);
    cpu.pc = 0x2222;
    16
}
/// $08FBE4: lea.l $905c2(pc), a1
pub fn i_08fbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x905C2;
    16
}
/// $08FBE8: jsr $4ae.l
pub fn i_08fbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FBEE);
    cpu.pc = 0x4AE;
    16
}
/// $08FBEE: jsr $5dd02.l
pub fn i_08fbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FBF4);
    cpu.pc = 0x5DD02;
    16
}
/// $08FBF4: move.w #$20, $82(a0)
pub fn i_08fbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBFA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FBFA: lea.l $9068c(pc), a1
pub fn i_08fbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9068C;
    16
}
/// $08FBFE: jsr $4ae.l
pub fn i_08fbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC04);
    cpu.pc = 0x4AE;
    16
}
/// $08FC04: jsr $5dd02.l
pub fn i_08fc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC0A);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC0A: move.w #$20, $82(a0)
pub fn i_08fc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC10;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC10: lea.l $9034e(pc), a1
pub fn i_08fc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9034E;
    16
}
/// $08FC14: jsr $4ae.l
pub fn i_08fc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC1A);
    cpu.pc = 0x4AE;
    16
}
/// $08FC1A: jsr $5dd02.l
pub fn i_08fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC20);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC20: move.w #$20, $82(a0)
pub fn i_08fc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC26;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC26: cmpi.w #$190, $24(a6)
pub fn i_08fc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC2C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x190, Size::Word, false);
    16
}
/// $08FC2C: bne.w $8fc8c
pub fn i_08fc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FC30: cmpi.b #$2, $21(a6)
pub fn i_08fc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC36;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08FC36: beq.w $8fc8c
pub fn i_08fc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FC3A: move.b #$2, $21(a6)
pub fn i_08fc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FC40: move.w #$10df, d0
pub fn i_08fc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC44;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FC44: jsr $2222.l
pub fn i_08fc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC4A);
    cpu.pc = 0x2222;
    16
}
/// $08FC4A: lea.l $905c2(pc), a1
pub fn i_08fc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x905C2;
    16
}
/// $08FC4E: jsr $4ae.l
pub fn i_08fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC54);
    cpu.pc = 0x4AE;
    16
}
/// $08FC54: jsr $5dd02.l
pub fn i_08fc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC5A);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC5A: move.w #$1e, $82(a0)
pub fn i_08fc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC60;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC60: lea.l $906e8(pc), a1
pub fn i_08fc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x906E8;
    16
}
/// $08FC64: jsr $4ae.l
pub fn i_08fc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC6A);
    cpu.pc = 0x4AE;
    16
}
/// $08FC6A: jsr $5dd02.l
pub fn i_08fc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC70);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC70: move.w #$1e, $82(a0)
pub fn i_08fc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC76;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC76: lea.l $904e4(pc), a1
pub fn i_08fc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x904E4;
    16
}
/// $08FC7A: jsr $4ae.l
pub fn i_08fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC80);
    cpu.pc = 0x4AE;
    16
}
/// $08FC80: jsr $5dd02.l
pub fn i_08fc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC86);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC86: move.w #$1e, $82(a0)
pub fn i_08fc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC8C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC8C: cmpi.w #$178, $24(a6)
pub fn i_08fc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC92;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x178, Size::Word, false);
    16
}
/// $08FC92: bgt.w $8fca8
pub fn i_08fc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC96;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8FCA8; return 10; }
    8
}
/// $08FC96: move.b #$1, $21(a6)
pub fn i_08fc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FC9C: move.w #$10, $82(a6)
pub fn i_08fc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08fca2  ($08FCA2, 8 B, kernel/task_handlers.c)
/// $08FCA2: lea.l $8fcca(pc), a1
pub fn i_08fca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FCCA;
    16
}
/// $08FCA6: move.l a1, (a6)
pub fn i_08fca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FCA8: rts
pub fn i_08fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_IntegrateY_08fcaa  ($08FCAA, 32 B, mission/gameover_continue_08f6d2.s)
/// $08FCAA: cmpi.w #$140, $24(a6)
pub fn i_08fcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08FCB0: ble.w $8fcc8
pub fn i_08fcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCB4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8FCC8; return 10; }
    8
}
/// $08FCB4: move.w $2a(a6), d0
pub fn i_08fcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FCB8: move.w d0, d1
pub fn i_08fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCBA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FCBA: asr.w #$8, d1
pub fn i_08fcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCBC;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08FCBC: add.b d0, $27(a6)
pub fn i_08fcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC0;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08FCC0: moveq #$0, d0
pub fn i_08fcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FCC2: addx.w d1, d0
pub fn i_08fcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC4;
    cpu.ir = 0xD141;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[1] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FCC4: add.w d0, $24(a6)
pub fn i_08fcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FCC8: rts
pub fn i_08fcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Wait_08fcca  ($08FCCA, 92 B, mission/gameover_continue_08f6d2.s)
/// $08FCCA: jsr $8fcaa(pc)
pub fn i_08fcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCCE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FCCE);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FCCE: subq.w #$1, $82(a6)
pub fn i_08fcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCD2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FCD2: bne.w $8fd2c
pub fn i_08fcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FD2C; return 10; }
    8
}
/// $08FCD6: move.w #$10df, d0
pub fn i_08fcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCDA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FCDA: jsr $2222.l
pub fn i_08fcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FCE0);
    cpu.pc = 0x2222;
    16
}
/// $08FCE0: move.b #$2, $21(a6)
pub fn i_08fce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FCE6: moveq #$0, d0
pub fn i_08fce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCE8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FCE8: moveq #$0, d1
pub fn i_08fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCEA;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FCEA: jsr $437da.l
pub fn i_08fcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FCF0);
    cpu.pc = 0x437DA;
    16
}
/// $08FCF0: move.w #$18, $82(a6)
pub fn i_08fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCF6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FCF6: lea.l $90630(pc), a1
pub fn i_08fcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90630;
    16
}
/// $08FCFA: jsr $4ae.l
pub fn i_08fcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD00);
    cpu.pc = 0x4AE;
    16
}
/// $08FD00: jsr $5dd02.l
pub fn i_08fd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD06);
    cpu.pc = 0x5DD02;
    16
}
/// $08FD06: lea.l $900e4(pc), a1
pub fn i_08fd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x900E4;
    16
}
/// $08FD0A: jsr $4ae.l
pub fn i_08fd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD10);
    cpu.pc = 0x4AE;
    16
}
/// $08FD10: jsr $5dd02.l
pub fn i_08fd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD16);
    cpu.pc = 0x5DD02;
    16
}
/// $08FD16: lea.l $9016c(pc), a1
pub fn i_08fd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9016C;
    16
}
/// $08FD1A: jsr $4ae.l
pub fn i_08fd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD20);
    cpu.pc = 0x4AE;
    16
}
/// $08FD20: jsr $5dd02.l
pub fn i_08fd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD26);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_08fd26  ($08FD26, 8 B, kernel/task_handlers.c)
/// $08FD26: lea.l $8fd2e(pc), a1
pub fn i_08fd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FD2E;
    16
}
/// $08FD2A: move.l a1, (a6)
pub fn i_08fd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FD2C: rts
pub fn i_08fd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Wait2_08fd2e  ($08FD2E, 50 B, mission/gameover_continue_08f6d2.s)
/// $08FD2E: jsr $8fcaa(pc)
pub fn i_08fd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FD32);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FD32: subq.w #$1, $82(a6)
pub fn i_08fd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD36;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FD36: bne.w $8fd66
pub fn i_08fd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FD66; return 10; }
    8
}
/// $08FD3A: move.w #$10df, d0
pub fn i_08fd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD3E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FD3E: jsr $2352.l
pub fn i_08fd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD44);
    cpu.pc = 0x2352;
    16
}
/// $08FD44: move.b #$3, $20(a6)
pub fn i_08fd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD4A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FD4A: clr.b $21(a6)
pub fn i_08fd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD4E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08FD4E: move.w #$1b, $82(a6)
pub fn i_08fd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD54;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FD54: lea.l $106f6c.l, a0
pub fn i_08fd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $08FD5A: jsr $51ed6.l
pub fn i_08fd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD60);
    cpu.pc = 0x51ED6;
    16
}
// ===== SetTaskHandler_08fd60  ($08FD60, 8 B, kernel/task_handlers.c)
/// $08FD60: lea.l $8fd68(pc), a1
pub fn i_08fd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FD68;
    16
}
/// $08FD64: move.l a1, (a6)
pub fn i_08fd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FD66: rts
pub fn i_08fd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Final_08fd68  ($08FD68, 58 B, mission/gameover_continue_08f6d2.s)
/// $08FD68: jsr $8fcaa(pc)
pub fn i_08fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FD6C);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FD6C: cmpi.w #$3, $82(a6)
pub fn i_08fd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD72;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08FD72: bne.w $8fd84
pub fn i_08fd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FD84; return 10; }
    8
}
/// $08FD76: move.b #$1, $10a2d1.l
pub fn i_08fd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD7E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $08FD7E: move.b #$4, $21(a6)
pub fn i_08fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD84;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FD84: subq.w #$1, $82(a6)
pub fn i_08fd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD88;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FD88: bne.w $8fda8
pub fn i_08fd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FDA8; return 10; }
    8
}
/// $08FD8C: move.w #$10de, d0
pub fn i_08fd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD90;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FD90: jsr $2352.l
pub fn i_08fd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD96);
    cpu.pc = 0x2352;
    16
}
/// $08FD96: move.b #$3, $21(a6)
pub fn i_08fd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FD9C: move.w #$10, $82(a6)
pub fn i_08fd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08fda2  ($08FDA2, 8 B, kernel/task_handlers.c)
/// $08FDA2: lea.l $8fdaa(pc), a1
pub fn i_08fda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FDAA;
    16
}
/// $08FDA6: move.l a1, (a6)
pub fn i_08fda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FDA8: rts
pub fn i_08fda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_WaitCredit_08fdaa  ($08FDAA, 70 B, mission/gameover_continue_08f6d2.s)
/// $08FDAA: tst.b $10a2cf.l
pub fn i_08fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDB0;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $08FDB0: beq.w $8fdf6
pub fn i_08fdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDB4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FDF6; return 10; }
    8
}
/// $08FDB4: subq.w #$1, $82(a6)
pub fn i_08fdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDB8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FDB8: bne.w $8fdf6
pub fn i_08fdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FDF6; return 10; }
    8
}
/// $08FDBC: jsr $5b6.l
pub fn i_08fdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDC2);
    cpu.pc = 0x5B6;
    16
}
/// $08FDC2: clr.b $20(a6)
pub fn i_08fdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDC6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08FDC6: lea.l $91514(pc), a1
pub fn i_08fdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91514;
    16
}
/// $08FDCA: jsr $4ae.l
pub fn i_08fdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDD0);
    cpu.pc = 0x4AE;
    16
}
/// $08FDD0: lea.l $9004c(pc), a1
pub fn i_08fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9004C;
    16
}
/// $08FDD4: jsr $4ae.l
pub fn i_08fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDDA);
    cpu.pc = 0x4AE;
    16
}
/// $08FDDA: jsr $5dd02.l
pub fn i_08fdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDE0);
    cpu.pc = 0x5DD02;
    16
}
/// $08FDE0: lea.l $90cba(pc), a1
pub fn i_08fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90CBA;
    16
}
/// $08FDE4: jsr $4ae.l
pub fn i_08fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDEA);
    cpu.pc = 0x4AE;
    16
}
/// $08FDEA: jsr $5dd02.l
pub fn i_08fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDF0);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_08fdf0  ($08FDF0, 8 B, kernel/task_handlers.c)
/// $08FDF0: lea.l $8fdf8(pc), a1
pub fn i_08fdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FDF8;
    16
}
/// $08FDF4: move.l a1, (a6)
pub fn i_08fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FDF6: rts
pub fn i_08fdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Continue_08fdf8  ($08FDF8, 50 B, mission/gameover_continue_08f6d2.s)
/// $08FDF8: cmpi.b #$1, $20(a6)
pub fn i_08fdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDFE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08FDFE: bne.w $8fe30
pub fn i_08fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE02;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FE30; return 10; }
    8
}
/// $08FE02: lea.l $8fff6(pc), a1
pub fn i_08fe02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FFF6;
    16
}
/// $08FE06: jsr $4ae.l
pub fn i_08fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE0C);
    cpu.pc = 0x4AE;
    16
}
/// $08FE0C: jsr $5dd02.l
pub fn i_08fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE12);
    cpu.pc = 0x5DD02;
    16
}
/// $08FE12: move.w $22(a6), $88(a6)
pub fn i_08fe12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE18;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FE18: move.w $24(a6), $8a(a6)
pub fn i_08fe18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE1E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FE1E: move.w #$11, $82(a6)
pub fn i_08fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE24: move.w #$0, $86(a6)
pub fn i_08fe24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08fe2a  ($08FE2A, 8 B, kernel/task_handlers.c)
/// $08FE2A: lea.l $8fe32(pc), a1
pub fn i_08fe2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FE32;
    16
}
/// $08FE2E: move.l a1, (a6)
pub fn i_08fe2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FE30: rts
pub fn i_08fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_ContinuePath_08fe32  ($08FE32, 98 B, mission/gameover_continue_08f6d2.s)
/// $08FE32: cmpi.w #$14, $86(a6)
pub fn i_08fe32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE38;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08FE38: bge.w $8fe66
pub fn i_08fe38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE3C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8FE66; return 10; }
    8
}
/// $08FE3C: lea.l $2f4bf8.l, a0
pub fn i_08fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4BF8;
    20
}
/// $08FE42: move.w $86(a6), d0
pub fn i_08fe42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FE46: add.w d0, d0
pub fn i_08fe46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE48;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FE48: add.w d0, d0
pub fn i_08fe48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE4A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FE4A: move.w (a0, d0.w), d1
pub fn i_08fe4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE4E;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08FE4E: add.w $88(a6), d1
pub fn i_08fe4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE52;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08FE52: move.w d1, $22(a6)
pub fn i_08fe52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE56;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE56: move.w $2(a0, d0.w), d1
pub fn i_08fe56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE5A;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08FE5A: add.w $8a(a6), d1
pub fn i_08fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE5E;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08FE5E: move.w d1, $24(a6)
pub fn i_08fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE62;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE62: addq.w #$1, $86(a6)
pub fn i_08fe62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE66;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FE66: subq.w #$1, $82(a6)
pub fn i_08fe66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE6A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FE6A: bne.w $8fe9a
pub fn i_08fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE6E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FE9A; return 10; }
    8
}
/// $08FE6E: move.w #$18, $82(a6)
pub fn i_08fe6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE74: lea.l $8fecc(pc), a1
pub fn i_08fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FECC;
    16
}
/// $08FE78: jsr $4ae.l
pub fn i_08fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE7E);
    cpu.pc = 0x4AE;
    16
}
/// $08FE7E: jsr $5dd02.l
pub fn i_08fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE84);
    cpu.pc = 0x5DD02;
    16
}
/// $08FE84: lea.l $8ff2c(pc), a1
pub fn i_08fe84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FF2C;
    16
}
/// $08FE88: jsr $4ae.l
pub fn i_08fe88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE8E);
    cpu.pc = 0x4AE;
    16
}
/// $08FE8E: jsr $5dd02.l
pub fn i_08fe8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE94);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_08fe94  ($08FE94, 8 B, kernel/task_handlers.c)
/// $08FE94: lea.l $8fe9c(pc), a1
pub fn i_08fe94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FE9C;
    16
}
/// $08FE98: move.l a1, (a6)
pub fn i_08fe98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FE9A: rts
pub fn i_08fe9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_ContinueHold_08fe9c  ($08FE9C, 18 B, mission/gameover_continue_08f6d2.s)
/// $08FE9C: subq.w #$1, $82(a6)
pub fn i_08fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEA0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FEA0: bne.w $8feb4
pub fn i_08fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FEB4; return 10; }
    8
}
/// $08FEA4: move.w #$96, $82(a6)
pub fn i_08fea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEAA: jsr $91618(pc)
pub fn i_08feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FEAE);
    cpu.pc = 0x91618;
    16
}
// ===== SetTaskHandler_08feae  ($08FEAE, 8 B, kernel/task_handlers.c)
/// $08FEAE: lea.l $8feb6(pc), a1
pub fn i_08feae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FEB6;
    16
}
/// $08FEB2: move.l a1, (a6)
pub fn i_08feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FEB4: rts
pub fn i_08feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_ContinueEnd_08feb6  ($08FEB6, 14 B, mission/gameover_continue_08f6d2.s)
/// $08FEB6: subq.w #$1, $82(a6)
pub fn i_08feb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEBA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FEBA: bne.w $8feca
pub fn i_08feba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FECA; return 10; }
    8
}
/// $08FEBE: clr.b $106ed2.l
pub fn i_08febe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEC4;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
// ===== SetTaskHandler_08fec4  ($08FEC4, 8 B, kernel/task_handlers.c)
/// $08FEC4: lea.l $8feca(pc), a1
pub fn i_08fec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FECA;
    16
}
/// $08FEC8: move.l a1, (a6)
pub fn i_08fec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FECA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FECA: rts
pub fn i_08feca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FECC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Sprite_Right_08fecc  ($08FECC, 96 B, mission/gameover_continue_08f6d2.s)
/// $08FECC: move.w #$d9, d1
pub fn i_08fecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FED0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FED0: jsr $236e.l
pub fn i_08fed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FED6);
    cpu.pc = 0x236E;
    16
}
/// $08FED6: move.w #$130, $22(a6)
pub fn i_08fed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEDC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x130;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEDC: move.w #$180, $24(a6)
pub fn i_08fedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEE2: move.w $22(a6), $88(a6)
pub fn i_08fee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEE8;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FEE8: move.w #$d8, $8c(a6)
pub fn i_08fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEEE: move.w #$ffe8, $28(a6)
pub fn i_08feee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEF4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEF4: move.w #$0, $80(a6)
pub fn i_08fef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEFA: move.w #$0, $86(a6)
pub fn i_08fefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF00;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF00: move.w #$d000, d0
pub fn i_08ff00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF04;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FF04: jsr $28134.l
pub fn i_08ff04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF0A);
    cpu.pc = 0x28134;
    16
}
/// $08FF0A: andi.w #$ffe3, $38(a6)
pub fn i_08ff0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF10;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF10: ori.w #$0, $38(a6)
pub fn i_08ff10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF16;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF16: lea.l $2f503e.l, a0
pub fn i_08ff16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F503E;
    20
}
/// $08FF1C: jsr $28cd4.l
pub fn i_08ff1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF22);
    cpu.pc = 0x28CD4;
    16
}
/// $08FF22: lea.l $8ff88(pc), a1
pub fn i_08ff22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FF88;
    16
}
/// $08FF26: move.l a1, (a6)
pub fn i_08ff26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FF28: bra.w $8ff88
pub fn i_08ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8FF88;
    10
}
// ===== GO_Sprite_Left_08ff2c  ($08FF2C, 92 B, mission/gameover_continue_08f6d2.s)
/// $08FF2C: move.w #$da, d1
pub fn i_08ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF30;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FF30: jsr $236e.l
pub fn i_08ff30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF36);
    cpu.pc = 0x236E;
    16
}
/// $08FF36: move.w #$20, $22(a6)
pub fn i_08ff36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF3C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF3C: move.w #$138, $24(a6)
pub fn i_08ff3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x138;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF42: move.w $22(a6), $88(a6)
pub fn i_08ff42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF48;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FF48: move.w #$ff28, $8c(a6)
pub fn i_08ff48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF4E: move.w #$18, $28(a6)
pub fn i_08ff4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF54;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF54: move.w #$2, $80(a6)
pub fn i_08ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF5A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF5A: move.w #$0, $86(a6)
pub fn i_08ff5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF60;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF60: move.w #$e000, d0
pub fn i_08ff60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF64;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FF64: jsr $28134.l
pub fn i_08ff64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF6A);
    cpu.pc = 0x28134;
    16
}
/// $08FF6A: andi.w #$ffe3, $38(a6)
pub fn i_08ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF70;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF70: ori.w #$0, $38(a6)
pub fn i_08ff70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF76;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF76: lea.l $2f504a.l, a0
pub fn i_08ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F504A;
    20
}
/// $08FF7C: jsr $28cd4.l
pub fn i_08ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF82);
    cpu.pc = 0x28CD4;
    16
}
/// $08FF82: lea.l $8ff88(pc), a1
pub fn i_08ff82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FF88;
    16
}
/// $08FF86: move.l a1, (a6)
pub fn i_08ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== GO_Sprite_Slide_08ff88  ($08FF88, 102 B, mission/gameover_continue_08f6d2.s)
/// $08FF88: move.w $88(a6), d0
pub fn i_08ff88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF8C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FF8C: add.w $8c(a6), d0
pub fn i_08ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF90;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08FF90: move.w d0, $22(a6)
pub fn i_08ff90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF94;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF94: tst.w $8c(a6)
pub fn i_08ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF98;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08FF98: beq.w $8ffa8
pub fn i_08ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FFA8; return 10; }
    8
}
/// $08FF9C: move.w $28(a6), d0
pub fn i_08ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FFA0: add.w d0, $8c(a6)
pub fn i_08ffa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFA4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FFA4: bra.w $8ffee
pub fn i_08ffa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x8FFEE;
    10
}
/// $08FFA8: cmpi.w #$14, $86(a6)
pub fn i_08ffa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFAE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08FFAE: bge.w $8ffee
pub fn i_08ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFB2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8FFEE; return 10; }
    8
}
/// $08FFB2: tst.w $86(a6)
pub fn i_08ffb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFB6;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08FFB6: bne.w $8ffcc
pub fn i_08ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FFCC; return 10; }
    8
}
/// $08FFBA: move.b #$6, $10a2d1.l
pub fn i_08ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFC2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $08FFC2: move.w #$10e0, d0
pub fn i_08ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FFC6: jsr $2352.l
pub fn i_08ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FFCC);
    cpu.pc = 0x2352;
    16
}
/// $08FFCC: lea.l $2f4c48.l, a0
pub fn i_08ffcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4C48;
    20
}
/// $08FFD2: moveq #$0, d0
pub fn i_08ffd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFD4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FFD4: move.w $86(a6), d0
pub fn i_08ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFD8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FFD8: add.w d0, d0
pub fn i_08ffd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFDA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FFDA: add.w d0, d0
pub fn i_08ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFDC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FFDC: adda.l d0, a0
pub fn i_08ffdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFDE;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $08FFDE: move.w $80(a6), d0
pub fn i_08ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFE2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FFE2: move.w (a0, d0.w), d1
pub fn i_08ffe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFE6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08FFE6: add.w d1, $22(a6)
pub fn i_08ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFEA;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FFEA: addq.w #$1, $86(a6)
pub fn i_08ffea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFEE;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrAbsThunk_08ffee  ($08FFEE, 8 B, thunks/jsr_abs_thunks.c)
/// $08FFEE: jsr $28d70.l
pub fn i_08ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FFF4);
    cpu.pc = 0x28D70;
    16
}
/// $08FFF4: rts
pub fn i_08fff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Sprite_Mid_08fff6  ($08FFF6, 78 B, mission/gameover_continue_08f6d2.s)
/// $08FFF6: move.w #$db, d1
pub fn i_08fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FFFA: jsr $236e.l
pub fn i_08fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90000);
    cpu.pc = 0x236E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x08F2B2, i_08f2b2),
    (0x08F2B4, i_08f2b4),
    (0x08F2BA, i_08f2ba),
    (0x08F2C0, i_08f2c0),
    (0x08F2C4, i_08f2c4),
    (0x08F2C6, i_08f2c6),
    (0x08F2CA, i_08f2ca),
    (0x08F2CE, i_08f2ce),
    (0x08F2D2, i_08f2d2),
    (0x08F2D6, i_08f2d6),
    (0x08F2DA, i_08f2da),
    (0x08F2DE, i_08f2de),
    (0x08F2E2, i_08f2e2),
    (0x08F2E6, i_08f2e6),
    (0x08F2EA, i_08f2ea),
    (0x08F2EE, i_08f2ee),
    (0x08F2F2, i_08f2f2),
    (0x08F2F4, i_08f2f4),
    (0x08F2F6, i_08f2f6),
    (0x08F2FA, i_08f2fa),
    (0x08F2FE, i_08f2fe),
    (0x08F300, i_08f300),
    (0x08F302, i_08f302),
    (0x08F306, i_08f306),
    (0x08F308, i_08f308),
    (0x08F30E, i_08f30e),
    (0x08F312, i_08f312),
    (0x08F316, i_08f316),
    (0x08F31A, i_08f31a),
    (0x08F320, i_08f320),
    (0x08F326, i_08f326),
    (0x08F32A, i_08f32a),
    (0x08F32E, i_08f32e),
    (0x08F330, i_08f330),
    (0x08F334, i_08f334),
    (0x08F338, i_08f338),
    (0x08F33A, i_08f33a),
    (0x08F33E, i_08f33e),
    (0x08F342, i_08f342),
    (0x08F344, i_08f344),
    (0x08F34A, i_08f34a),
    (0x08F34E, i_08f34e),
    (0x08F350, i_08f350),
    (0x08F354, i_08f354),
    (0x08F358, i_08f358),
    (0x08F35C, i_08f35c),
    (0x08F360, i_08f360),
    (0x08F364, i_08f364),
    (0x08F366, i_08f366),
    (0x08F36A, i_08f36a),
    (0x08F36E, i_08f36e),
    (0x08F372, i_08f372),
    (0x08F376, i_08f376),
    (0x08F37A, i_08f37a),
    (0x08F37E, i_08f37e),
    (0x08F382, i_08f382),
    (0x08F388, i_08f388),
    (0x08F38A, i_08f38a),
    (0x08F38C, i_08f38c),
    (0x08F390, i_08f390),
    (0x08F392, i_08f392),
    (0x08F394, i_08f394),
    (0x08F398, i_08f398),
    (0x08F39C, i_08f39c),
    (0x08F39E, i_08f39e),
    (0x08F3A0, i_08f3a0),
    (0x08F3A4, i_08f3a4),
    (0x08F3A6, i_08f3a6),
    (0x08F3AC, i_08f3ac),
    (0x08F3AE, i_08f3ae),
    (0x08F3B2, i_08f3b2),
    (0x08F3B6, i_08f3b6),
    (0x08F3BC, i_08f3bc),
    (0x08F3BE, i_08f3be),
    (0x08F3C4, i_08f3c4),
    (0x08F3C8, i_08f3c8),
    (0x08F3CC, i_08f3cc),
    (0x08F3CE, i_08f3ce),
    (0x08F3D2, i_08f3d2),
    (0x08F3D6, i_08f3d6),
    (0x08F3DA, i_08f3da),
    (0x08F3DE, i_08f3de),
    (0x08F3E2, i_08f3e2),
    (0x08F3E6, i_08f3e6),
    (0x08F3EA, i_08f3ea),
    (0x08F3EC, i_08f3ec),
    (0x08F3F0, i_08f3f0),
    (0x08F3F4, i_08f3f4),
    (0x08F3F6, i_08f3f6),
    (0x08F3F8, i_08f3f8),
    (0x08F3FC, i_08f3fc),
    (0x08F400, i_08f400),
    (0x08F404, i_08f404),
    (0x08F408, i_08f408),
    (0x08F40C, i_08f40c),
    (0x08F410, i_08f410),
    (0x08F414, i_08f414),
    (0x08F418, i_08f418),
    (0x08F41C, i_08f41c),
    (0x08F420, i_08f420),
    (0x08F424, i_08f424),
    (0x08F428, i_08f428),
    (0x08F42C, i_08f42c),
    (0x08F42E, i_08f42e),
    (0x08F430, i_08f430),
    (0x08F434, i_08f434),
    (0x08F438, i_08f438),
    (0x08F43A, i_08f43a),
    (0x08F43C, i_08f43c),
    (0x08F43E, i_08f43e),
    (0x08F440, i_08f440),
    (0x08F444, i_08f444),
    (0x08F448, i_08f448),
    (0x08F44C, i_08f44c),
    (0x08F452, i_08f452),
    (0x08F456, i_08f456),
    (0x08F45A, i_08f45a),
    (0x08F45C, i_08f45c),
    (0x08F460, i_08f460),
    (0x08F464, i_08f464),
    (0x08F466, i_08f466),
    (0x08F46A, i_08f46a),
    (0x08F46E, i_08f46e),
    (0x08F470, i_08f470),
    (0x08F476, i_08f476),
    (0x08F47A, i_08f47a),
    (0x08F47C, i_08f47c),
    (0x08F482, i_08f482),
    (0x08F486, i_08f486),
    (0x08F48C, i_08f48c),
    (0x08F490, i_08f490),
    (0x08F494, i_08f494),
    (0x08F498, i_08f498),
    (0x08F49C, i_08f49c),
    (0x08F4A0, i_08f4a0),
    (0x08F4A6, i_08f4a6),
    (0x08F4AA, i_08f4aa),
    (0x08F4AC, i_08f4ac),
    (0x08F4B0, i_08f4b0),
    (0x08F4B4, i_08f4b4),
    (0x08F4B6, i_08f4b6),
    (0x08F4B8, i_08f4b8),
    (0x08F4BC, i_08f4bc),
    (0x08F4C0, i_08f4c0),
    (0x08F4C4, i_08f4c4),
    (0x08F4CA, i_08f4ca),
    (0x08F4CE, i_08f4ce),
    (0x08F4D0, i_08f4d0),
    (0x08F4D4, i_08f4d4),
    (0x08F4D8, i_08f4d8),
    (0x08F4DA, i_08f4da),
    (0x08F4DC, i_08f4dc),
    (0x08F4E0, i_08f4e0),
    (0x08F4E2, i_08f4e2),
    (0x08F4E6, i_08f4e6),
    (0x08F4E8, i_08f4e8),
    (0x08F4EC, i_08f4ec),
    (0x08F4EE, i_08f4ee),
    (0x08F4F2, i_08f4f2),
    (0x08F4F6, i_08f4f6),
    (0x08F4F8, i_08f4f8),
    (0x08F4FC, i_08f4fc),
    (0x08F500, i_08f500),
    (0x08F504, i_08f504),
    (0x08F508, i_08f508),
    (0x08F50A, i_08f50a),
    (0x08F50E, i_08f50e),
    (0x08F512, i_08f512),
    (0x08F516, i_08f516),
    (0x08F51A, i_08f51a),
    (0x08F51E, i_08f51e),
    (0x08F520, i_08f520),
    (0x08F526, i_08f526),
    (0x08F52A, i_08f52a),
    (0x08F52C, i_08f52c),
    (0x08F530, i_08f530),
    (0x08F536, i_08f536),
    (0x08F53A, i_08f53a),
    (0x08F53E, i_08f53e),
    (0x08F544, i_08f544),
    (0x08F548, i_08f548),
    (0x08F54C, i_08f54c),
    (0x08F550, i_08f550),
    (0x08F554, i_08f554),
    (0x08F558, i_08f558),
    (0x08F55C, i_08f55c),
    (0x08F562, i_08f562),
    (0x08F566, i_08f566),
    (0x08F568, i_08f568),
    (0x08F56C, i_08f56c),
    (0x08F570, i_08f570),
    (0x08F572, i_08f572),
    (0x08F574, i_08f574),
    (0x08F578, i_08f578),
    (0x08F57A, i_08f57a),
    (0x08F580, i_08f580),
    (0x08F584, i_08f584),
    (0x08F588, i_08f588),
    (0x08F58C, i_08f58c),
    (0x08F590, i_08f590),
    (0x08F594, i_08f594),
    (0x08F598, i_08f598),
    (0x08F59C, i_08f59c),
    (0x08F59E, i_08f59e),
    (0x08F5A0, i_08f5a0),
    (0x08F5A4, i_08f5a4),
    (0x08F5A8, i_08f5a8),
    (0x08F5AA, i_08f5aa),
    (0x08F5AC, i_08f5ac),
    (0x08F5B0, i_08f5b0),
    (0x08F5B4, i_08f5b4),
    (0x08F5BA, i_08f5ba),
    (0x08F5BE, i_08f5be),
    (0x08F5C2, i_08f5c2),
    (0x08F5C4, i_08f5c4),
    (0x08F5C8, i_08f5c8),
    (0x08F5CC, i_08f5cc),
    (0x08F5CE, i_08f5ce),
    (0x08F5D2, i_08f5d2),
    (0x08F5D6, i_08f5d6),
    (0x08F5DA, i_08f5da),
    (0x08F5DC, i_08f5dc),
    (0x08F5E2, i_08f5e2),
    (0x08F5E6, i_08f5e6),
    (0x08F5E8, i_08f5e8),
    (0x08F5EC, i_08f5ec),
    (0x08F5F0, i_08f5f0),
    (0x08F5F4, i_08f5f4),
    (0x08F5F8, i_08f5f8),
    (0x08F5FC, i_08f5fc),
    (0x08F600, i_08f600),
    (0x08F602, i_08f602),
    (0x08F606, i_08f606),
    (0x08F60A, i_08f60a),
    (0x08F60C, i_08f60c),
    (0x08F60E, i_08f60e),
    (0x08F612, i_08f612),
    (0x08F614, i_08f614),
    (0x08F616, i_08f616),
    (0x08F61A, i_08f61a),
    (0x08F61E, i_08f61e),
    (0x08F622, i_08f622),
    (0x08F628, i_08f628),
    (0x08F62C, i_08f62c),
    (0x08F630, i_08f630),
    (0x08F636, i_08f636),
    (0x08F63A, i_08f63a),
    (0x08F63E, i_08f63e),
    (0x08F642, i_08f642),
    (0x08F646, i_08f646),
    (0x08F64A, i_08f64a),
    (0x08F64E, i_08f64e),
    (0x08F652, i_08f652),
    (0x08F656, i_08f656),
    (0x08F65A, i_08f65a),
    (0x08F65C, i_08f65c),
    (0x08F65E, i_08f65e),
    (0x08F662, i_08f662),
    (0x08F666, i_08f666),
    (0x08F668, i_08f668),
    (0x08F66A, i_08f66a),
    (0x08F66E, i_08f66e),
    (0x08F674, i_08f674),
    (0x08F67A, i_08f67a),
    (0x08F67E, i_08f67e),
    (0x08F682, i_08f682),
    (0x08F684, i_08f684),
    (0x08F688, i_08f688),
    (0x08F68C, i_08f68c),
    (0x08F68E, i_08f68e),
    (0x08F692, i_08f692),
    (0x08F696, i_08f696),
    (0x08F69A, i_08f69a),
    (0x08F69C, i_08f69c),
    (0x08F6A0, i_08f6a0),
    (0x08F6A2, i_08f6a2),
    (0x08F6A6, i_08f6a6),
    (0x08F6AA, i_08f6aa),
    (0x08F6AC, i_08f6ac),
    (0x08F6B0, i_08f6b0),
    (0x08F6B4, i_08f6b4),
    (0x08F6B6, i_08f6b6),
    (0x08F6BA, i_08f6ba),
    (0x08F6BE, i_08f6be),
    (0x08F6C2, i_08f6c2),
    (0x08F6C6, i_08f6c6),
    (0x08F6CA, i_08f6ca),
    (0x08F6CC, i_08f6cc),
    (0x08F6D0, i_08f6d0),
    (0x08F6D2, i_08f6d2),
    (0x08F6D8, i_08f6d8),
    (0x08F6DA, i_08f6da),
    (0x08F6E0, i_08f6e0),
    (0x08F6E2, i_08f6e2),
    (0x08F6E6, i_08f6e6),
    (0x08F6EA, i_08f6ea),
    (0x08F6F0, i_08f6f0),
    (0x08F6F2, i_08f6f2),
    (0x08F6F8, i_08f6f8),
    (0x08F6FA, i_08f6fa),
    (0x08F6FE, i_08f6fe),
    (0x08F702, i_08f702),
    (0x08F704, i_08f704),
    (0x08F708, i_08f708),
    (0x08F70C, i_08f70c),
    (0x08F70E, i_08f70e),
    (0x08F712, i_08f712),
    (0x08F714, i_08f714),
    (0x08F71A, i_08f71a),
    (0x08F71E, i_08f71e),
    (0x08F722, i_08f722),
    (0x08F726, i_08f726),
    (0x08F728, i_08f728),
    (0x08F72C, i_08f72c),
    (0x08F730, i_08f730),
    (0x08F732, i_08f732),
    (0x08F736, i_08f736),
    (0x08F738, i_08f738),
    (0x08F73C, i_08f73c),
    (0x08F740, i_08f740),
    (0x08F742, i_08f742),
    (0x08F744, i_08f744),
    (0x08F748, i_08f748),
    (0x08F74A, i_08f74a),
    (0x08F74C, i_08f74c),
    (0x08F750, i_08f750),
    (0x08F752, i_08f752),
    (0x08F756, i_08f756),
    (0x08F758, i_08f758),
    (0x08F75A, i_08f75a),
    (0x08F75E, i_08f75e),
    (0x08F762, i_08f762),
    (0x08F766, i_08f766),
    (0x08F76A, i_08f76a),
    (0x08F76E, i_08f76e),
    (0x08F772, i_08f772),
    (0x08F774, i_08f774),
    (0x08F778, i_08f778),
    (0x08F77A, i_08f77a),
    (0x08F77C, i_08f77c),
    (0x08F77E, i_08f77e),
    (0x08F782, i_08f782),
    (0x08F784, i_08f784),
    (0x08F788, i_08f788),
    (0x08F78C, i_08f78c),
    (0x08F790, i_08f790),
    (0x08F794, i_08f794),
    (0x08F796, i_08f796),
    (0x08F79A, i_08f79a),
    (0x08F79C, i_08f79c),
    (0x08F7A0, i_08f7a0),
    (0x08F7A4, i_08f7a4),
    (0x08F7A6, i_08f7a6),
    (0x08F7A8, i_08f7a8),
    (0x08F7AA, i_08f7aa),
    (0x08F7AC, i_08f7ac),
    (0x08F7AE, i_08f7ae),
    (0x08F7B2, i_08f7b2),
    (0x08F7B8, i_08f7b8),
    (0x08F7BC, i_08f7bc),
    (0x08F7C2, i_08f7c2),
    (0x08F7C6, i_08f7c6),
    (0x08F7CC, i_08f7cc),
    (0x08F7CE, i_08f7ce),
    (0x08F7D0, i_08f7d0),
    (0x08F7D2, i_08f7d2),
    (0x08F7D6, i_08f7d6),
    (0x08F7D8, i_08f7d8),
    (0x08F7DA, i_08f7da),
    (0x08F7DE, i_08f7de),
    (0x08F7E2, i_08f7e2),
    (0x08F7E6, i_08f7e6),
    (0x08F7EA, i_08f7ea),
    (0x08F7EE, i_08f7ee),
    (0x08F7F2, i_08f7f2),
    (0x08F7F6, i_08f7f6),
    (0x08F7FA, i_08f7fa),
    (0x08F7FE, i_08f7fe),
    (0x08F802, i_08f802),
    (0x08F806, i_08f806),
    (0x08F80A, i_08f80a),
    (0x08F80E, i_08f80e),
    (0x08F812, i_08f812),
    (0x08F816, i_08f816),
    (0x08F81A, i_08f81a),
    (0x08F81E, i_08f81e),
    (0x08F822, i_08f822),
    (0x08F824, i_08f824),
    (0x08F826, i_08f826),
    (0x08F828, i_08f828),
    (0x08F82A, i_08f82a),
    (0x08F82E, i_08f82e),
    (0x08F832, i_08f832),
    (0x08F834, i_08f834),
    (0x08F838, i_08f838),
    (0x08F83C, i_08f83c),
    (0x08F840, i_08f840),
    (0x08F844, i_08f844),
    (0x08F848, i_08f848),
    (0x08F84A, i_08f84a),
    (0x08F84E, i_08f84e),
    (0x08F850, i_08f850),
    (0x08F854, i_08f854),
    (0x08F856, i_08f856),
    (0x08F85A, i_08f85a),
    (0x08F85C, i_08f85c),
    (0x08F860, i_08f860),
    (0x08F862, i_08f862),
    (0x08F866, i_08f866),
    (0x08F86A, i_08f86a),
    (0x08F870, i_08f870),
    (0x08F872, i_08f872),
    (0x08F876, i_08f876),
    (0x08F87A, i_08f87a),
    (0x08F87E, i_08f87e),
    (0x08F882, i_08f882),
    (0x08F884, i_08f884),
    (0x08F886, i_08f886),
    (0x08F88A, i_08f88a),
    (0x08F88C, i_08f88c),
    (0x08F88E, i_08f88e),
    (0x08F892, i_08f892),
    (0x08F896, i_08f896),
    (0x08F89A, i_08f89a),
    (0x08F89C, i_08f89c),
    (0x08F89E, i_08f89e),
    (0x08F8A2, i_08f8a2),
    (0x08F8A8, i_08f8a8),
    (0x08F8AA, i_08f8aa),
    (0x08F8AE, i_08f8ae),
    (0x08F8B2, i_08f8b2),
    (0x08F8B6, i_08f8b6),
    (0x08F8BA, i_08f8ba),
    (0x08F8BC, i_08f8bc),
    (0x08F8C0, i_08f8c0),
    (0x08F8C2, i_08f8c2),
    (0x08F8C6, i_08f8c6),
    (0x08F8C8, i_08f8c8),
    (0x08F8CA, i_08f8ca),
    (0x08F8D0, i_08f8d0),
    (0x08F8D2, i_08f8d2),
    (0x08F8D6, i_08f8d6),
    (0x08F8DA, i_08f8da),
    (0x08F8E0, i_08f8e0),
    (0x08F8E2, i_08f8e2),
    (0x08F8E8, i_08f8e8),
    (0x08F8EA, i_08f8ea),
    (0x08F8EE, i_08f8ee),
    (0x08F8F2, i_08f8f2),
    (0x08F8F8, i_08f8f8),
    (0x08F8FC, i_08f8fc),
    (0x08F8FE, i_08f8fe),
    (0x08F902, i_08f902),
    (0x08F906, i_08f906),
    (0x08F90A, i_08f90a),
    (0x08F90E, i_08f90e),
    (0x08F912, i_08f912),
    (0x08F914, i_08f914),
    (0x08F918, i_08f918),
    (0x08F91A, i_08f91a),
    (0x08F91E, i_08f91e),
    (0x08F924, i_08f924),
    (0x08F92A, i_08f92a),
    (0x08F930, i_08f930),
    (0x08F936, i_08f936),
    (0x08F93C, i_08f93c),
    (0x08F942, i_08f942),
    (0x08F944, i_08f944),
    (0x08F948, i_08f948),
    (0x08F94C, i_08f94c),
    (0x08F952, i_08f952),
    (0x08F956, i_08f956),
    (0x08F95A, i_08f95a),
    (0x08F95C, i_08f95c),
    (0x08F962, i_08f962),
    (0x08F966, i_08f966),
    (0x08F968, i_08f968),
    (0x08F96A, i_08f96a),
    (0x08F96E, i_08f96e),
    (0x08F974, i_08f974),
    (0x08F978, i_08f978),
    (0x08F97E, i_08f97e),
    (0x08F984, i_08f984),
    (0x08F988, i_08f988),
    (0x08F98E, i_08f98e),
    (0x08F994, i_08f994),
    (0x08F998, i_08f998),
    (0x08F99E, i_08f99e),
    (0x08F9A4, i_08f9a4),
    (0x08F9AA, i_08f9aa),
    (0x08F9AE, i_08f9ae),
    (0x08F9B4, i_08f9b4),
    (0x08F9BA, i_08f9ba),
    (0x08F9C0, i_08f9c0),
    (0x08F9C4, i_08f9c4),
    (0x08F9CA, i_08f9ca),
    (0x08F9D0, i_08f9d0),
    (0x08F9D6, i_08f9d6),
    (0x08F9DA, i_08f9da),
    (0x08F9E0, i_08f9e0),
    (0x08F9E6, i_08f9e6),
    (0x08F9EC, i_08f9ec),
    (0x08F9F0, i_08f9f0),
    (0x08F9F6, i_08f9f6),
    (0x08F9FC, i_08f9fc),
    (0x08FA02, i_08fa02),
    (0x08FA06, i_08fa06),
    (0x08FA0C, i_08fa0c),
    (0x08FA12, i_08fa12),
    (0x08FA18, i_08fa18),
    (0x08FA1C, i_08fa1c),
    (0x08FA22, i_08fa22),
    (0x08FA28, i_08fa28),
    (0x08FA2E, i_08fa2e),
    (0x08FA32, i_08fa32),
    (0x08FA38, i_08fa38),
    (0x08FA3E, i_08fa3e),
    (0x08FA44, i_08fa44),
    (0x08FA48, i_08fa48),
    (0x08FA4E, i_08fa4e),
    (0x08FA54, i_08fa54),
    (0x08FA5A, i_08fa5a),
    (0x08FA5E, i_08fa5e),
    (0x08FA64, i_08fa64),
    (0x08FA6A, i_08fa6a),
    (0x08FA70, i_08fa70),
    (0x08FA74, i_08fa74),
    (0x08FA7A, i_08fa7a),
    (0x08FA80, i_08fa80),
    (0x08FA86, i_08fa86),
    (0x08FA8A, i_08fa8a),
    (0x08FA90, i_08fa90),
    (0x08FA96, i_08fa96),
    (0x08FA9C, i_08fa9c),
    (0x08FAA0, i_08faa0),
    (0x08FAA6, i_08faa6),
    (0x08FAAC, i_08faac),
    (0x08FAB2, i_08fab2),
    (0x08FAB6, i_08fab6),
    (0x08FABC, i_08fabc),
    (0x08FAC2, i_08fac2),
    (0x08FAC6, i_08fac6),
    (0x08FACC, i_08facc),
    (0x08FAD2, i_08fad2),
    (0x08FAD6, i_08fad6),
    (0x08FADC, i_08fadc),
    (0x08FAE2, i_08fae2),
    (0x08FAE6, i_08fae6),
    (0x08FAEC, i_08faec),
    (0x08FAF2, i_08faf2),
    (0x08FAF6, i_08faf6),
    (0x08FAFC, i_08fafc),
    (0x08FB02, i_08fb02),
    (0x08FB06, i_08fb06),
    (0x08FB0C, i_08fb0c),
    (0x08FB12, i_08fb12),
    (0x08FB16, i_08fb16),
    (0x08FB1C, i_08fb1c),
    (0x08FB22, i_08fb22),
    (0x08FB26, i_08fb26),
    (0x08FB2C, i_08fb2c),
    (0x08FB32, i_08fb32),
    (0x08FB36, i_08fb36),
    (0x08FB3C, i_08fb3c),
    (0x08FB42, i_08fb42),
    (0x08FB46, i_08fb46),
    (0x08FB4C, i_08fb4c),
    (0x08FB52, i_08fb52),
    (0x08FB56, i_08fb56),
    (0x08FB5C, i_08fb5c),
    (0x08FB62, i_08fb62),
    (0x08FB66, i_08fb66),
    (0x08FB6C, i_08fb6c),
    (0x08FB72, i_08fb72),
    (0x08FB76, i_08fb76),
    (0x08FB7C, i_08fb7c),
    (0x08FB82, i_08fb82),
    (0x08FB86, i_08fb86),
    (0x08FB8C, i_08fb8c),
    (0x08FB92, i_08fb92),
    (0x08FB96, i_08fb96),
    (0x08FB9C, i_08fb9c),
    (0x08FBA0, i_08fba0),
    (0x08FBA2, i_08fba2),
    (0x08FBA6, i_08fba6),
    (0x08FBAA, i_08fbaa),
    (0x08FBAE, i_08fbae),
    (0x08FBB2, i_08fbb2),
    (0x08FBB8, i_08fbb8),
    (0x08FBBC, i_08fbbc),
    (0x08FBC0, i_08fbc0),
    (0x08FBC6, i_08fbc6),
    (0x08FBCA, i_08fbca),
    (0x08FBD0, i_08fbd0),
    (0x08FBD4, i_08fbd4),
    (0x08FBDA, i_08fbda),
    (0x08FBDE, i_08fbde),
    (0x08FBE4, i_08fbe4),
    (0x08FBE8, i_08fbe8),
    (0x08FBEE, i_08fbee),
    (0x08FBF4, i_08fbf4),
    (0x08FBFA, i_08fbfa),
    (0x08FBFE, i_08fbfe),
    (0x08FC04, i_08fc04),
    (0x08FC0A, i_08fc0a),
    (0x08FC10, i_08fc10),
    (0x08FC14, i_08fc14),
    (0x08FC1A, i_08fc1a),
    (0x08FC20, i_08fc20),
    (0x08FC26, i_08fc26),
    (0x08FC2C, i_08fc2c),
    (0x08FC30, i_08fc30),
    (0x08FC36, i_08fc36),
    (0x08FC3A, i_08fc3a),
    (0x08FC40, i_08fc40),
    (0x08FC44, i_08fc44),
    (0x08FC4A, i_08fc4a),
    (0x08FC4E, i_08fc4e),
    (0x08FC54, i_08fc54),
    (0x08FC5A, i_08fc5a),
    (0x08FC60, i_08fc60),
    (0x08FC64, i_08fc64),
    (0x08FC6A, i_08fc6a),
    (0x08FC70, i_08fc70),
    (0x08FC76, i_08fc76),
    (0x08FC7A, i_08fc7a),
    (0x08FC80, i_08fc80),
    (0x08FC86, i_08fc86),
    (0x08FC8C, i_08fc8c),
    (0x08FC92, i_08fc92),
    (0x08FC96, i_08fc96),
    (0x08FC9C, i_08fc9c),
    (0x08FCA2, i_08fca2),
    (0x08FCA6, i_08fca6),
    (0x08FCA8, i_08fca8),
    (0x08FCAA, i_08fcaa),
    (0x08FCB0, i_08fcb0),
    (0x08FCB4, i_08fcb4),
    (0x08FCB8, i_08fcb8),
    (0x08FCBA, i_08fcba),
    (0x08FCBC, i_08fcbc),
    (0x08FCC0, i_08fcc0),
    (0x08FCC2, i_08fcc2),
    (0x08FCC4, i_08fcc4),
    (0x08FCC8, i_08fcc8),
    (0x08FCCA, i_08fcca),
    (0x08FCCE, i_08fcce),
    (0x08FCD2, i_08fcd2),
    (0x08FCD6, i_08fcd6),
    (0x08FCDA, i_08fcda),
    (0x08FCE0, i_08fce0),
    (0x08FCE6, i_08fce6),
    (0x08FCE8, i_08fce8),
    (0x08FCEA, i_08fcea),
    (0x08FCF0, i_08fcf0),
    (0x08FCF6, i_08fcf6),
    (0x08FCFA, i_08fcfa),
    (0x08FD00, i_08fd00),
    (0x08FD06, i_08fd06),
    (0x08FD0A, i_08fd0a),
    (0x08FD10, i_08fd10),
    (0x08FD16, i_08fd16),
    (0x08FD1A, i_08fd1a),
    (0x08FD20, i_08fd20),
    (0x08FD26, i_08fd26),
    (0x08FD2A, i_08fd2a),
    (0x08FD2C, i_08fd2c),
    (0x08FD2E, i_08fd2e),
    (0x08FD32, i_08fd32),
    (0x08FD36, i_08fd36),
    (0x08FD3A, i_08fd3a),
    (0x08FD3E, i_08fd3e),
    (0x08FD44, i_08fd44),
    (0x08FD4A, i_08fd4a),
    (0x08FD4E, i_08fd4e),
    (0x08FD54, i_08fd54),
    (0x08FD5A, i_08fd5a),
    (0x08FD60, i_08fd60),
    (0x08FD64, i_08fd64),
    (0x08FD66, i_08fd66),
    (0x08FD68, i_08fd68),
    (0x08FD6C, i_08fd6c),
    (0x08FD72, i_08fd72),
    (0x08FD76, i_08fd76),
    (0x08FD7E, i_08fd7e),
    (0x08FD84, i_08fd84),
    (0x08FD88, i_08fd88),
    (0x08FD8C, i_08fd8c),
    (0x08FD90, i_08fd90),
    (0x08FD96, i_08fd96),
    (0x08FD9C, i_08fd9c),
    (0x08FDA2, i_08fda2),
    (0x08FDA6, i_08fda6),
    (0x08FDA8, i_08fda8),
    (0x08FDAA, i_08fdaa),
    (0x08FDB0, i_08fdb0),
    (0x08FDB4, i_08fdb4),
    (0x08FDB8, i_08fdb8),
    (0x08FDBC, i_08fdbc),
    (0x08FDC2, i_08fdc2),
    (0x08FDC6, i_08fdc6),
    (0x08FDCA, i_08fdca),
    (0x08FDD0, i_08fdd0),
    (0x08FDD4, i_08fdd4),
    (0x08FDDA, i_08fdda),
    (0x08FDE0, i_08fde0),
    (0x08FDE4, i_08fde4),
    (0x08FDEA, i_08fdea),
    (0x08FDF0, i_08fdf0),
    (0x08FDF4, i_08fdf4),
    (0x08FDF6, i_08fdf6),
    (0x08FDF8, i_08fdf8),
    (0x08FDFE, i_08fdfe),
    (0x08FE02, i_08fe02),
    (0x08FE06, i_08fe06),
    (0x08FE0C, i_08fe0c),
    (0x08FE12, i_08fe12),
    (0x08FE18, i_08fe18),
    (0x08FE1E, i_08fe1e),
    (0x08FE24, i_08fe24),
    (0x08FE2A, i_08fe2a),
    (0x08FE2E, i_08fe2e),
    (0x08FE30, i_08fe30),
    (0x08FE32, i_08fe32),
    (0x08FE38, i_08fe38),
    (0x08FE3C, i_08fe3c),
    (0x08FE42, i_08fe42),
    (0x08FE46, i_08fe46),
    (0x08FE48, i_08fe48),
    (0x08FE4A, i_08fe4a),
    (0x08FE4E, i_08fe4e),
    (0x08FE52, i_08fe52),
    (0x08FE56, i_08fe56),
    (0x08FE5A, i_08fe5a),
    (0x08FE5E, i_08fe5e),
    (0x08FE62, i_08fe62),
    (0x08FE66, i_08fe66),
    (0x08FE6A, i_08fe6a),
    (0x08FE6E, i_08fe6e),
    (0x08FE74, i_08fe74),
    (0x08FE78, i_08fe78),
    (0x08FE7E, i_08fe7e),
    (0x08FE84, i_08fe84),
    (0x08FE88, i_08fe88),
    (0x08FE8E, i_08fe8e),
    (0x08FE94, i_08fe94),
    (0x08FE98, i_08fe98),
    (0x08FE9A, i_08fe9a),
    (0x08FE9C, i_08fe9c),
    (0x08FEA0, i_08fea0),
    (0x08FEA4, i_08fea4),
    (0x08FEAA, i_08feaa),
    (0x08FEAE, i_08feae),
    (0x08FEB2, i_08feb2),
    (0x08FEB4, i_08feb4),
    (0x08FEB6, i_08feb6),
    (0x08FEBA, i_08feba),
    (0x08FEBE, i_08febe),
    (0x08FEC4, i_08fec4),
    (0x08FEC8, i_08fec8),
    (0x08FECA, i_08feca),
    (0x08FECC, i_08fecc),
    (0x08FED0, i_08fed0),
    (0x08FED6, i_08fed6),
    (0x08FEDC, i_08fedc),
    (0x08FEE2, i_08fee2),
    (0x08FEE8, i_08fee8),
    (0x08FEEE, i_08feee),
    (0x08FEF4, i_08fef4),
    (0x08FEFA, i_08fefa),
    (0x08FF00, i_08ff00),
    (0x08FF04, i_08ff04),
    (0x08FF0A, i_08ff0a),
    (0x08FF10, i_08ff10),
    (0x08FF16, i_08ff16),
    (0x08FF1C, i_08ff1c),
    (0x08FF22, i_08ff22),
    (0x08FF26, i_08ff26),
    (0x08FF28, i_08ff28),
    (0x08FF2C, i_08ff2c),
    (0x08FF30, i_08ff30),
    (0x08FF36, i_08ff36),
    (0x08FF3C, i_08ff3c),
    (0x08FF42, i_08ff42),
    (0x08FF48, i_08ff48),
    (0x08FF4E, i_08ff4e),
    (0x08FF54, i_08ff54),
    (0x08FF5A, i_08ff5a),
    (0x08FF60, i_08ff60),
    (0x08FF64, i_08ff64),
    (0x08FF6A, i_08ff6a),
    (0x08FF70, i_08ff70),
    (0x08FF76, i_08ff76),
    (0x08FF7C, i_08ff7c),
    (0x08FF82, i_08ff82),
    (0x08FF86, i_08ff86),
    (0x08FF88, i_08ff88),
    (0x08FF8C, i_08ff8c),
    (0x08FF90, i_08ff90),
    (0x08FF94, i_08ff94),
    (0x08FF98, i_08ff98),
    (0x08FF9C, i_08ff9c),
    (0x08FFA0, i_08ffa0),
    (0x08FFA4, i_08ffa4),
    (0x08FFA8, i_08ffa8),
    (0x08FFAE, i_08ffae),
    (0x08FFB2, i_08ffb2),
    (0x08FFB6, i_08ffb6),
    (0x08FFBA, i_08ffba),
    (0x08FFC2, i_08ffc2),
    (0x08FFC6, i_08ffc6),
    (0x08FFCC, i_08ffcc),
    (0x08FFD2, i_08ffd2),
    (0x08FFD4, i_08ffd4),
    (0x08FFD8, i_08ffd8),
    (0x08FFDA, i_08ffda),
    (0x08FFDC, i_08ffdc),
    (0x08FFDE, i_08ffde),
    (0x08FFE2, i_08ffe2),
    (0x08FFE6, i_08ffe6),
    (0x08FFEA, i_08ffea),
    (0x08FFEE, i_08ffee),
    (0x08FFF4, i_08fff4),
    (0x08FFF6, i_08fff6),
    (0x08FFFA, i_08fffa),
];
