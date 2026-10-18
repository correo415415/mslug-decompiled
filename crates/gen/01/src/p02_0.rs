// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== ChildRank_CmpByte10_024e00  ($024E00, 16 B, entity/childrank_cmp_islands.s)
/// $024E00: movea.l $8(a6), a1
pub fn i_024e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E04;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $024E04: move.b $10(a6), d0
pub fn i_024e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $024E08: cmp.b $10(a1), d0
pub fn i_024e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E0C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $024E0C: bcs.w $24e16
pub fn i_024e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E10;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x24E16; return 10; }
    8
}
// ===== ClearXN_024e10  ($024E10, 6 B, thunks/ccr_helpers.c)
/// $024E10: andi.b #$ee, ccr
pub fn i_024e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E14;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $024E14: rts
pub fn i_024e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_024e16  ($024E16, 6 B, thunks/ccr_helpers.c)
/// $024E16: ori.b #$11, ccr
pub fn i_024e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E1A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $024E1A: rts
pub fn i_024e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_024e1c  ($024E1C, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $024E1C: movea.l $8(a6), a1
pub fn i_024e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E20;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $024E20: move.b $10(a6), d0
pub fn i_024e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E24;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $024E24: cmp.b $10(a1), d0
pub fn i_024e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E28;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $024E28: bcs.w $24e32
pub fn i_024e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E2C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x24E32; return 10; }
    8
}
// ===== ClearXN_024e2c  ($024E2C, 6 B, thunks/ccr_helpers.c)
/// $024E2C: andi.b #$ee, ccr
pub fn i_024e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E30;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $024E30: rts
pub fn i_024e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_024e32  ($024E32, 6 B, thunks/ccr_helpers.c)
/// $024E32: ori.b #$11, ccr
pub fn i_024e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E36;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $024E36: rts
pub fn i_024e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TitleModeInit_024E38  ($024E38, 54 B, input/start_button_dispatcher_024e38.s)
/// $024E38: jsr $2126.l
pub fn i_024e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x24E3E);
    cpu.pc = 0x2126;
    16
}
/// $024E3E: move.b #$1, $106ecc.l
pub fn i_024e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E46;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECC, v1 as u8);
    16
}
/// $024E46: move.b #$1, $106ecd.l
pub fn i_024e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E4E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECD, v1 as u8);
    16
}
/// $024E4E: clr.b $10007b.l
pub fn i_024e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E54;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10007B) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10007B, r2 as u8);
    16
}
/// $024E54: jsr $9773c.l
pub fn i_024e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x24E5A);
    cpu.pc = 0x9773C;
    16
}
/// $024E5A: tst.b $10fd82.l
pub fn i_024e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E60;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024E60: bne.w $24e6e
pub fn i_024e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24E6E; return 10; }
    8
}
/// $024E64: jsr $99ae2.l
pub fn i_024e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x24E6A);
    cpu.pc = 0x99AE2;
    16
}
/// $024E6A: bra.w $24e74
pub fn i_024e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x24E74;
    10
}
// ===== JsrAbsThunk_024e6e  ($024E6E, 8 B, thunks/jsr_abs_thunks.c)
/// $024E6E: jsr $99afc.l
pub fn i_024e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x24E74);
    cpu.pc = 0x99AFC;
    16
}
/// $024E74: rts
pub fn i_024e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Start_Inner_024E76  ($024E76, 272 B, input/start_button_dispatcher_024e38.s)
/// $024E76: move.b $10fdb4.l, d0
pub fn i_024e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E7C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FDB4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $024E7C: andi.w #$3, d0
pub fn i_024e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E80;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $024E80: lsl.w #$2, d0
pub fn i_024e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E82;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $024E82: bclr.b #$0, $10fdb4.l
pub fn i_024e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E8A;
    cpu.ir = 0x8B9;
    let mk: u32 = 1u32 << ((0) % 8);
    let v1 = bus.read8(0x10FDB4) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    bus.write8(0x10FDB4, r2 as u8);
    20
}
/// $024E8A: bclr.b #$1, $10fdb4.l
pub fn i_024e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E92;
    cpu.ir = 0x8B9;
    let mk: u32 = 1u32 << ((1) % 8);
    let v1 = bus.read8(0x10FDB4) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    bus.write8(0x10FDB4, r2 as u8);
    20
}
/// $024E92: lea.l $10fdb6.l, a1
pub fn i_024e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E98;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10FDB6;
    20
}
/// $024E98: lea.l $24fa6(pc), a0
pub fn i_024e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E9C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x24FA6;
    16
}
/// $024E9C: cmpi.b #$1, (a1)
pub fn i_024e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EA0;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    12
}
/// $024EA0: beq.w $24f02
pub fn i_024ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F02; return 10; }
    8
}
/// $024EA4: cmpi.b #$3, (a1)
pub fn i_024ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EA8;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $024EA8: beq.w $24f02
pub fn i_024ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F02; return 10; }
    8
}
/// $024EAC: tst.b $106ecc.l
pub fn i_024eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EB2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ECC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024EB2: bne.w $24f02
pub fn i_024eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24F02; return 10; }
    8
}
/// $024EB6: tst.b $10fd82.l
pub fn i_024eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EBC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024EBC: bne.w $24eca
pub fn i_024ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24ECA; return 10; }
    8
}
/// $024EC0: tst.b $1081bf.l
pub fn i_024ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EC6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x1081BF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024EC6: beq.w $24f02
pub fn i_024ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24ECA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F02; return 10; }
    8
}
/// $024ECA: cmpi.b #$0, $2(a0, d0.w)
pub fn i_024eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24ED0;
    cpu.ir = 0xC30;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    18
}
/// $024ED0: beq.w $24f02
pub fn i_024ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24ED4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F02; return 10; }
    8
}
/// $024ED4: move.b $2(a0, d0.w), (a1)
pub fn i_024ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24ED8;
    cpu.ir = 0x12B0;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v3 as u8);
    18
}
/// $024ED8: beq.w $24f02
pub fn i_024ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F02; return 10; }
    8
}
/// $024EDC: bset.b #$0, $10fdb4.l
pub fn i_024edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EE4;
    cpu.ir = 0x8F9;
    let mk: u32 = 1u32 << ((0) % 8);
    let v1 = bus.read8(0x10FDB4) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    bus.write8(0x10FDB4, r2 as u8);
    20
}
/// $024EE4: tst.b $10fd82.l
pub fn i_024ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EEA;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024EEA: bne.w $24f02
pub fn i_024eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24F02; return 10; }
    8
}
/// $024EEE: move.l d0, -(a7)
pub fn i_024eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EF0;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $024EF0: move.b $1081bf.l, d0
pub fn i_024ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EF6;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1081BF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $024EF6: jsr $24f86(pc)
pub fn i_024ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x24EFA);
    cpu.pc = 0x24F86;
    16
}
/// $024EFA: move.b d0, $1081bf.l
pub fn i_024efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F00;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081BF, v1 as u8);
    16
}
/// $024F00: move.l (a7)+, d0
pub fn i_024f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F02;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $024F02: cmpi.b #$1, $1(a1)
pub fn i_024f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F08;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $024F08: beq.w $24f6e
pub fn i_024f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F0C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F6E; return 10; }
    8
}
/// $024F0C: cmpi.b #$3, $1(a1)
pub fn i_024f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F12;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $024F12: beq.w $24f6e
pub fn i_024f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F6E; return 10; }
    8
}
/// $024F16: tst.b $106ecd.l
pub fn i_024f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F1C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ECD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024F1C: bne.w $24f6e
pub fn i_024f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24F6E; return 10; }
    8
}
/// $024F20: tst.b $10fd82.l
pub fn i_024f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F26;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024F26: bne.w $24f34
pub fn i_024f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24F34; return 10; }
    8
}
/// $024F2A: tst.b $1081c0.l
pub fn i_024f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F30;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x1081C0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024F30: beq.w $24f6e
pub fn i_024f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F6E; return 10; }
    8
}
/// $024F34: cmpi.b #$0, $3(a0, d0.w)
pub fn i_024f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F3A;
    cpu.ir = 0xC30;
    let ad2 = cpu.a[0].wrapping_add(0x3).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    18
}
/// $024F3A: beq.w $24f6e
pub fn i_024f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F6E; return 10; }
    8
}
/// $024F3E: move.b $3(a0, d0.w), $1(a1)
pub fn i_024f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F44;
    cpu.ir = 0x1370;
    let ad2 = cpu.a[0].wrapping_add(0x3).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x1);
    bus.write8(ad4, v3 as u8);
    22
}
/// $024F44: beq.w $24f6e
pub fn i_024f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F6E; return 10; }
    8
}
/// $024F48: bset.b #$1, $10fdb4.l
pub fn i_024f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F50;
    cpu.ir = 0x8F9;
    let mk: u32 = 1u32 << ((1) % 8);
    let v1 = bus.read8(0x10FDB4) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    bus.write8(0x10FDB4, r2 as u8);
    20
}
/// $024F50: tst.b $10fd82.l
pub fn i_024f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F56;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $024F56: bne.w $24f6e
pub fn i_024f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F5A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24F6E; return 10; }
    8
}
/// $024F5A: move.l d0, -(a7)
pub fn i_024f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F5C;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $024F5C: move.b $1081c0.l, d0
pub fn i_024f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F62;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1081C0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $024F62: jsr $24f86(pc)
pub fn i_024f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x24F66);
    cpu.pc = 0x24F86;
    16
}
/// $024F66: move.b d0, $1081c0.l
pub fn i_024f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F6C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081C0, v1 as u8);
    16
}
/// $024F6C: move.l (a7)+, d0
pub fn i_024f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F6E;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $024F6E: move.b $10fdb4.l, d1
pub fn i_024f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F74;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FDB4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $024F74: andi.b #$3, d1
pub fn i_024f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F78;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $024F78: beq.w $24f84
pub fn i_024f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F7C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x24F84; return 10; }
    8
}
/// $024F7C: move.b (a0, d0.w), $10fdaf.l
pub fn i_024f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F84;
    cpu.ir = 0x13F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDAF, v3 as u8);
    26
}
/// $024F84: rts
pub fn i_024f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Start_Decoder_024F86  ($024F86, 32 B, input/start_button_dispatcher_024e38.s)
/// $024F86: move.l d1, -(a7)
pub fn i_024f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F88;
    cpu.ir = 0x2F01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $024F88: move.b d0, d1
pub fn i_024f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F8A;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $024F8A: andi.b #$f, d0
pub fn i_024f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F8E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $024F8E: andi.b #$f0, d1
pub fn i_024f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F92;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $024F92: subq.b #$1, d0
pub fn i_024f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F94;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $024F94: bpl.w $24fa0
pub fn i_024f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F98;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x24FA0; return 10; }
    8
}
/// $024F98: move.b #$9, d0
pub fn i_024f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F9C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $024F9C: subi.b #$10, d1
pub fn i_024f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FA0;
    cpu.ir = 0x401;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $024FA0: or.b d1, d0
pub fn i_024fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FA2;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $024FA2: move.l (a7)+, d1
pub fn i_024fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FA4;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $024FA4: rts
pub fn i_024fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00024FB6  ($024FB6, 2 B, thunks/trivial_rts.c)
/// $024FB6: rts
pub fn i_024fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerCtx_ResetAndSetRepeat1002_024fb8  ($024FB8, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $024FB8: moveq #$ff, d0
pub fn i_024fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FBA;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $024FBA: bsr.w $24fec
pub fn i_024fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FBE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x24FBE);
    cpu.pc = 0x24FEC;
    18
}
/// $024FBE: move.w #$1002, $106ec8.l
pub fn i_024fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FC6;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC8, v1 as u16);
    16
}
/// $024FC6: rts
pub fn i_024fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerCtx_SetRepeatRate_024fc8  ($024FC8, 8 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $024FC8: move.w d0, $106ec8.l
pub fn i_024fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FCE;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC8, v1 as u16);
    16
}
/// $024FCE: rts
pub fn i_024fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerCtx_IsModeSingle_024fd0  ($024FD0, 12 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $024FD0: cmpi.b #$2, $106eca.l
pub fn i_024fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FD8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECA) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $024FD8: bne.w $24fe6
pub fn i_024fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24FE6; return 10; }
    8
}
// ===== SetC_024fdc  ($024FDC, 6 B, thunks/ccr_helpers.c)
/// $024FDC: ori.b #$1, ccr
pub fn i_024fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FE0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $024FE0: rts
pub fn i_024fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerCtx_ResetTwoBlocks_Thunk_024fe2  ($024FE2, 4 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $024FE2: bra.w $24fec
pub fn i_024fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x24FEC;
    10
}
// ===== ClearC_024fe6  ($024FE6, 6 B, thunks/ccr_helpers.c)
/// $024FE6: andi.b #$fe, ccr
pub fn i_024fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FEA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $024FEA: rts
pub fn i_024fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerCtx_ResetTwoBlocks_024FEC  ($024FEC, 38 B, entity/misc_helpers_024fec.s)
/// $024FEC: lea.l $106eb0.l, a0
pub fn i_024fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB0;
    20
}
/// $024FF2: clr.l (a0)+
pub fn i_024ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FF4;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $024FF4: clr.w (a0)
pub fn i_024ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FF6;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $024FF6: lea.l $106eb6.l, a0
pub fn i_024ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB6;
    20
}
/// $024FFC: clr.l (a0)+
pub fn i_024ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FFE;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $024FFE: clr.w (a0)
pub fn i_024ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25000;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $025000: cmpi.w #$2, d0
pub fn i_025000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25004;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $025004: bcs.w $25012
pub fn i_025004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25008;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25012; return 10; }
    8
}
/// $025008: move.b #$0, $106eca.l
pub fn i_025008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25010;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECA, v1 as u8);
    16
}
/// $025010: rts
pub fn i_025010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25012;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerCtx_InitExtended_025012  ($025012, 84 B, video/camera_list_ctx_helpers_025012.s)
/// $025012: move.b #$1, $106eca.l
pub fn i_025012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2501A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECA, v1 as u8);
    16
}
/// $02501A: add.w d0, d0
pub fn i_02501a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2501C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02501C: add.w d0, d0
pub fn i_02501c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2501E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02501E: lea.l $e7c00.l, a0
pub fn i_02501e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25024;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xE7C00;
    20
}
/// $025024: move.l (a0, d0.w), $106ebe.l
pub fn i_025024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2502C;
    cpu.ir = 0x23F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(0x106EBE, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    34
}
/// $02502C: clr.w d0
pub fn i_02502c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2502E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02502E: move.w d0, $106ec2.l
pub fn i_02502e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25034;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC2, v1 as u16);
    16
}
/// $025034: move.w d0, $106ec4.l
pub fn i_025034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2503A;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC4, v1 as u16);
    16
}
/// $02503A: move.w d0, $106ebc.l
pub fn i_02503a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25040;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EBC, v1 as u16);
    16
}
/// $025040: move.b d0, $106ec6.l
pub fn i_025040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25046;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EC6, v1 as u8);
    16
}
/// $025046: tst.b $25118.l
pub fn i_025046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2504C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x25118) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $02504C: beq.w $25064
pub fn i_02504c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25050;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25064; return 10; }
    8
}
/// $025050: movea.l $106ebe.l, a0
pub fn i_025050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25056;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x106EBE);
    cpu.a[0] = v2;
    20
}
/// $025056: move.b #$ff, d1
pub fn i_025056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2505A;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02505A: move.w #$1ff, d5
pub fn i_02505a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2505E;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02505E: move.b d1, (a0)+
pub fn i_02505e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25060;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $025060: dbra d5, $2505e
pub fn i_025060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25064;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x25064; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x25064; return 14; }
    cpu.pc = 0x2505E;
    10
}
/// $025064: rts
pub fn i_025064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25066;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_Poll_LiveMode_025066  ($025066, 178 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025066: cmpi.b #$0, $106eca.l
pub fn i_025066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2506E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECA) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02506E: bne.w $2511a
pub fn i_02506e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25072;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2511A; return 10; }
    8
}
/// $025072: move.b $10fd96.l, d0
pub fn i_025072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25078;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD96) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $025078: lea.l $106eb0.l, a0
pub fn i_025078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2507E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB0;
    20
}
/// $02507E: move.b $2(a0), d1
pub fn i_02507e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25082;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $025082: move.b d1, $1(a0)
pub fn i_025082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25086;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025086: move.b d0, $2(a0)
pub fn i_025086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2508A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02508A: move.b d0, d2
pub fn i_02508a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2508C;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02508C: eor.b d2, d1
pub fn i_02508c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2508E;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02508E: and.b d2, d1
pub fn i_02508e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25090;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $025090: move.b d1, $3(a0)
pub fn i_025090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25094;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025094: beq.w $250a8
pub fn i_025094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25098;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x250A8; return 10; }
    8
}
/// $025098: move.b d1, $4(a0)
pub fn i_025098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2509C;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02509C: move.b $106ec8.l, $5(a0)
pub fn i_02509c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250A4;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0250A4: bra.w $250c4
pub fn i_0250a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x250C4;
    10
}
/// $0250A8: subq.b #$1, $5(a0)
pub fn i_0250a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250AC;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0250AC: bne.w $250c0
pub fn i_0250ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x250C0; return 10; }
    8
}
/// $0250B0: move.b d2, $4(a0)
pub fn i_0250b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250B4;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0250B4: move.b $106ec9.l, $5(a0)
pub fn i_0250b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250BC;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0250BC: bra.w $250c4
pub fn i_0250bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x250C4;
    10
}
/// $0250C0: clr.b $4(a0)
pub fn i_0250c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250C4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0250C4: move.b $10fd9c.l, d0
pub fn i_0250c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250CA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD9C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0250CA: lea.l $106eb6.l, a0
pub fn i_0250ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB6;
    20
}
/// $0250D0: move.b $2(a0), d1
pub fn i_0250d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250D4;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0250D4: move.b d1, $1(a0)
pub fn i_0250d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250D8;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0250D8: move.b d0, $2(a0)
pub fn i_0250d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250DC;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0250DC: move.b d0, d2
pub fn i_0250dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250DE;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0250DE: eor.b d2, d1
pub fn i_0250de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250E0;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0250E0: and.b d2, d1
pub fn i_0250e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250E2;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0250E2: move.b d1, $3(a0)
pub fn i_0250e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250E6;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0250E6: beq.w $250fa
pub fn i_0250e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x250FA; return 10; }
    8
}
/// $0250EA: move.b d1, $4(a0)
pub fn i_0250ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250EE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0250EC: ori.b #$79, d4
pub fn i_0250ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250F0;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x79) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0250EE: move.b $106ec8.l, $5(a0)
pub fn i_0250ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250F6;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0250F0: ori.b #$c8, (a0)
pub fn i_0250f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250F4;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xC8) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0250F4: ori.b #$0, d5
pub fn i_0250f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250F8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0250F6: bra.w $25116
pub fn i_0250f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x25116;
    10
}
/// $0250F8: ori.b #$28, (a6)+
pub fn i_0250f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250FC;
    cpu.ir = 0x1E;
    let ad1 = cpu.a[6]; cpu.a[6] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x28) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0250FA: subq.b #$1, $5(a0)
pub fn i_0250fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250FE;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0250FC: ori.b #$0, d5
pub fn i_0250fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25100;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0250FE: bne.w $25112
pub fn i_0250fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25102;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25112; return 10; }
    8
}
/// $025100: ori.b #$42, (a2)
pub fn i_025100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25104;
    cpu.ir = 0x12;
    let ad1 = cpu.a[2];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x42) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025102: move.b d2, $4(a0)
pub fn i_025102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25106;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025104: ori.b #$79, d4
pub fn i_025104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25108;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x79) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $025106: move.b $106ec9.l, $5(a0)
pub fn i_025106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2510E;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $025108: ori.b #$c9, (a0)
pub fn i_025108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2510C;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xC9) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02510C: ori.b #$0, d5
pub fn i_02510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25110;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02510E: bra.w $25116
pub fn i_02510e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25112;
    cpu.ir = 0x6000;
    cpu.pc = 0x25116;
    10
}
/// $025110: ori.b #$28, d6
pub fn i_025110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25114;
    cpu.ir = 0x6;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 | 0x28) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $025112: clr.b $4(a0)
pub fn i_025112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25116;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025114: ori.b #$75, d4
pub fn i_025114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25118;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x75) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $025116: rts
pub fn i_025116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25118;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_ReplayFlag_025118  ($025118, 362 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025118: ori.b #$39, d0
pub fn i_025118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2511C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x39) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02511A: cmpi.b #$1, $106eca.l
pub fn i_02511a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25122;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECA) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $02511C: ori.b #$10, d1
pub fn i_02511c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25120;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $025120: bgt.b $250ec
pub fn i_025120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25122;
    cpu.ir = 0x6ECA;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x250EC; return 10; }
    8
}
/// $025122: bne.w $251e4
pub fn i_025122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25126;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x251E4; return 10; }
    8
}
/// $025126: tst.b $25118.l
pub fn i_025126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2512C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x25118) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $02512C: beq.w $25138
pub fn i_02512c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25130;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25138; return 10; }
    8
}
/// $025130: bsr.w $25282
pub fn i_025130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25134;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25134);
    cpu.pc = 0x25282;
    18
}
/// $025134: bra.w $2513c
pub fn i_025134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25138;
    cpu.ir = 0x6000;
    cpu.pc = 0x2513C;
    10
}
/// $025138: bsr.w $2531e
pub fn i_025138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2513C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x2513C);
    cpu.pc = 0x2531E;
    18
}
/// $02513C: move.b $106ebc.l, d0
pub fn i_02513c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25142;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106EBC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $025142: lea.l $106eb0.l, a0
pub fn i_025142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25148;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB0;
    20
}
/// $025148: move.b $2(a0), d1
pub fn i_025148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2514C;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02514C: move.b d1, $1(a0)
pub fn i_02514c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25150;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025150: move.b d0, $2(a0)
pub fn i_025150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25154;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025154: move.b d0, d2
pub fn i_025154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25156;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025156: eor.b d2, d1
pub fn i_025156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25158;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $025158: and.b d2, d1
pub fn i_025158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2515A;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $02515A: move.b d1, $3(a0)
pub fn i_02515a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2515E;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02515E: beq.w $25172
pub fn i_02515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25162;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25172; return 10; }
    8
}
/// $025162: move.b d1, $4(a0)
pub fn i_025162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25166;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025166: move.b $106ec8.l, $5(a0)
pub fn i_025166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2516E;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $02516E: bra.w $2518e
pub fn i_02516e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25172;
    cpu.ir = 0x6000;
    cpu.pc = 0x2518E;
    10
}
/// $025172: subq.b #$1, $5(a0)
pub fn i_025172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25176;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $025176: bne.w $2518a
pub fn i_025176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2517A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2518A; return 10; }
    8
}
/// $02517A: move.b d2, $4(a0)
pub fn i_02517a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2517E;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02517E: move.b $106ec9.l, $5(a0)
pub fn i_02517e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25186;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $025186: bra.w $2518e
pub fn i_025186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2518A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2518E;
    10
}
/// $02518A: clr.b $4(a0)
pub fn i_02518a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2518E;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02518E: move.b $106ebd.l, d0
pub fn i_02518e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25194;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106EBD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $025194: lea.l $106eb6.l, a0
pub fn i_025194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2519A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB6;
    20
}
/// $02519A: move.b $2(a0), d1
pub fn i_02519a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2519E;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02519E: move.b d1, $1(a0)
pub fn i_02519e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251A2;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0251A2: move.b d0, $2(a0)
pub fn i_0251a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251A6;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0251A6: move.b d0, d2
pub fn i_0251a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251A8;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0251A8: eor.b d2, d1
pub fn i_0251a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251AA;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0251AA: and.b d2, d1
pub fn i_0251aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251AC;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0251AC: move.b d1, $3(a0)
pub fn i_0251ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251B0;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0251B0: beq.w $251c4
pub fn i_0251b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x251C4; return 10; }
    8
}
/// $0251B4: move.b d1, $4(a0)
pub fn i_0251b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251B8;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0251B8: move.b $106ec8.l, $5(a0)
pub fn i_0251b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251C0;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0251C0: bra.w $251e0
pub fn i_0251c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x251E0;
    10
}
/// $0251C4: subq.b #$1, $5(a0)
pub fn i_0251c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251C8;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0251C8: bne.w $251dc
pub fn i_0251c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x251DC; return 10; }
    8
}
/// $0251CC: move.b d2, $4(a0)
pub fn i_0251cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251D0;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0251D0: move.b $106ec9.l, $5(a0)
pub fn i_0251d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251D8;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0251D8: bra.w $251e0
pub fn i_0251d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x251E0;
    10
}
/// $0251DC: clr.b $4(a0)
pub fn i_0251dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251E0;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0251E0: bra.w $25280
pub fn i_0251e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x25280;
    10
}
/// $0251E4: clr.b d0
pub fn i_0251e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251E6;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0251E6: lea.l $106eb0.l, a0
pub fn i_0251e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB0;
    20
}
/// $0251EC: move.b $2(a0), d1
pub fn i_0251ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251F0;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0251F0: move.b d1, $1(a0)
pub fn i_0251f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251F4;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0251F4: move.b d0, $2(a0)
pub fn i_0251f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251F8;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0251F8: move.b d0, d2
pub fn i_0251f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251FA;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0251FA: eor.b d2, d1
pub fn i_0251fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251FC;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0251FC: and.b d2, d1
pub fn i_0251fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251FE;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0251FE: move.b d1, $3(a0)
pub fn i_0251fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25202;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025202: beq.w $25216
pub fn i_025202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25206;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25216; return 10; }
    8
}
/// $025206: move.b d1, $4(a0)
pub fn i_025206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2520A;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02520A: move.b $106ec8.l, $5(a0)
pub fn i_02520a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25212;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $025212: bra.w $25232
pub fn i_025212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25216;
    cpu.ir = 0x6000;
    cpu.pc = 0x25232;
    10
}
/// $025216: subq.b #$1, $5(a0)
pub fn i_025216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2521A;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02521A: bne.w $2522e
pub fn i_02521a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2521E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2522E; return 10; }
    8
}
/// $02521E: move.b d2, $4(a0)
pub fn i_02521e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25222;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025222: move.b $106ec9.l, $5(a0)
pub fn i_025222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2522A;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $02522A: bra.w $25232
pub fn i_02522a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2522E;
    cpu.ir = 0x6000;
    cpu.pc = 0x25232;
    10
}
/// $02522E: clr.b $4(a0)
pub fn i_02522e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25232;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025232: clr.b d0
pub fn i_025232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25234;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $025234: lea.l $106eb6.l, a0
pub fn i_025234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2523A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB6;
    20
}
/// $02523A: move.b $2(a0), d1
pub fn i_02523a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2523E;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02523E: move.b d1, $1(a0)
pub fn i_02523e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25242;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025242: move.b d0, $2(a0)
pub fn i_025242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25246;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025246: move.b d0, d2
pub fn i_025246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25248;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025248: eor.b d2, d1
pub fn i_025248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2524A;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02524A: and.b d2, d1
pub fn i_02524a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2524C;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $02524C: move.b d1, $3(a0)
pub fn i_02524c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25250;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025250: beq.w $25264
pub fn i_025250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25254;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25264; return 10; }
    8
}
/// $025254: move.b d1, $4(a0)
pub fn i_025254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25258;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025258: move.b $106ec8.l, $5(a0)
pub fn i_025258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25260;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $025260: bra.w $25280
pub fn i_025260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25264;
    cpu.ir = 0x6000;
    cpu.pc = 0x25280;
    10
}
/// $025264: subq.b #$1, $5(a0)
pub fn i_025264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25268;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $025268: bne.w $2527c
pub fn i_025268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2526C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2527C; return 10; }
    8
}
/// $02526C: move.b d2, $4(a0)
pub fn i_02526c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25270;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025270: move.b $106ec9.l, $5(a0)
pub fn i_025270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25278;
    cpu.ir = 0x1179;
    let v2 = bus.read8(0x106EC9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad3, v2 as u8);
    24
}
/// $025278: bra.w $25280
pub fn i_025278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2527C;
    cpu.ir = 0x6000;
    cpu.pc = 0x25280;
    10
}
/// $02527C: clr.b $4(a0)
pub fn i_02527c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25280;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025280: rts
pub fn i_025280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputRec_RecordFrame_025282  ($025282, 156 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025282: movea.l $106ebe.l, a0
pub fn i_025282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25288;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x106EBE);
    cpu.a[0] = v2;
    20
}
/// $025288: move.b #$f, d1
pub fn i_025288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2528C;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02528C: move.w $106ec2.l, d0
pub fn i_02528c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25292;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106EC2) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $025292: cmp.w $106ec4.l, d0
pub fn i_025292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25298;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106EC4) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $025298: beq.w $252a0
pub fn i_025298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2529C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x252A0; return 10; }
    8
}
/// $02529C: move.b #$ff, d1
pub fn i_02529c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252A0;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0252A0: cmp.b $106ec6.l, d1
pub fn i_0252a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252A6;
    cpu.ir = 0xB239;
    let v2 = bus.read8(0x106EC6) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $0252A6: bhi.w $252ae
pub fn i_0252a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252AA;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x252AE; return 10; }
    8
}
/// $0252AA: jsr $25366(pc)
pub fn i_0252aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x252AE);
    cpu.pc = 0x25366;
    16
}
/// $0252AE: clr.w d0
pub fn i_0252ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252B0;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0252B0: clr.w d1
pub fn i_0252b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252B2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0252B2: move.b $10fd96.l, d0
pub fn i_0252b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252B8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD96) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0252B8: move.b $106ebc.l, d2
pub fn i_0252b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252BE;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x106EBC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0252BE: move.b d0, $106ebc.l
pub fn i_0252be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252C4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EBC, v1 as u8);
    16
}
/// $0252C4: eor.b d2, d0
pub fn i_0252c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252C6;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0252C6: move.b $10fd9c.l, d1
pub fn i_0252c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252CC;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FD9C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0252CC: move.b $106ebd.l, d2
pub fn i_0252cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252D2;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x106EBD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0252D2: move.b d1, $106ebd.l
pub fn i_0252d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252D8;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EBD, v1 as u8);
    16
}
/// $0252D8: eor.b d2, d1
pub fn i_0252d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252DA;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0252DA: lsl.w #$8, d1
pub fn i_0252da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252DC;
    cpu.ir = 0xE149;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0252DC: or.w d1, d0
pub fn i_0252dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252DE;
    cpu.ir = 0x8041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0252DE: move.w #$f, d6
pub fn i_0252de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252E2;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0252E2: move.l d6, -(a7)
pub fn i_0252e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252E4;
    cpu.ir = 0x2F06;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0252E4: lsl.w #$1, d0
pub fn i_0252e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252E6;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0252E6: bcc.w $252f4
pub fn i_0252e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x252F4; return 10; }
    8
}
/// $0252EA: move.w d6, d1
pub fn i_0252ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252EC;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0252EC: move.l d0, -(a7)
pub fn i_0252ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252EE;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0252EE: jsr $25390(pc)
pub fn i_0252ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x252F2);
    cpu.pc = 0x25390;
    16
}
/// $0252F2: move.l (a7)+, d0
pub fn i_0252f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252F4;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0252F4: move.l (a7)+, d6
pub fn i_0252f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252F6;
    cpu.ir = 0x2C1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0252F6: dbra d6, $252e2
pub fn i_0252f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252FA;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x252FA; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x252FA; return 14; }
    cpu.pc = 0x252E2;
    10
}
/// $0252FA: addq.b #$1, $106ec6.l
pub fn i_0252fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25300;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x106EC6) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x106EC6, r2 as u8);
    16
}
/// $025300: cmpi.w #$200, $106ec4.l
pub fn i_025300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25308;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106EC4) as u32;
    sub_with_flags(cpu, v2, 0x200, Size::Word, false);
    20
}
/// $025308: bcs.w $2531c
pub fn i_025308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2530C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2531C; return 10; }
    8
}
/// $02530C: clr.w d0
pub fn i_02530c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2530E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02530E: move.w d0, $106ebc.l
pub fn i_02530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25314;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EBC, v1 as u16);
    16
}
/// $025314: move.b #$2, $106eca.l
pub fn i_025314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2531C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECA, v1 as u8);
    16
}
/// $02531C: rts
pub fn i_02531c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2531E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputRec_PlaybackFrame_02531e  ($02531E, 72 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02531E: movea.l $106ebe.l, a0
pub fn i_02531e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25324;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x106EBE);
    cpu.a[0] = v2;
    20
}
/// $025324: jsr $253be(pc)
pub fn i_025324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25328;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x25328);
    cpu.pc = 0x253BE;
    16
}
/// $025328: bcc.w $2535e
pub fn i_025328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2532C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2535E; return 10; }
    8
}
/// $02532C: moveq #$0, d1
pub fn i_02532c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2532E;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02532E: andi.w #$f, d0
pub fn i_02532e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25332;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025332: btst.b #$3, d0
pub fn i_025332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25336;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $025336: bne.w $2534a
pub fn i_025336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2533A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2534A; return 10; }
    8
}
/// $02533A: move.b $106ebc.l, d1
pub fn i_02533a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25340;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106EBC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $025340: bchg.b d0, d1
pub fn i_025340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25342;
    cpu.ir = 0x141;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 ^ mk;
    cpu.d[1] = r2;
    8
}
/// $025342: move.b d1, $106ebc.l
pub fn i_025342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25348;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EBC, v1 as u8);
    16
}
/// $025348: bra.b $2531e
pub fn i_025348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2534A;
    cpu.ir = 0x60D4;
    cpu.pc = 0x2531E;
    10
}
/// $02534A: andi.w #$7, d0
pub fn i_02534a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2534E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02534E: move.b $106ebd.l, d1
pub fn i_02534e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25354;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106EBD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $025354: bchg.b d0, d1
pub fn i_025354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25356;
    cpu.ir = 0x141;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 ^ mk;
    cpu.d[1] = r2;
    8
}
/// $025356: move.b d1, $106ebd.l
pub fn i_025356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2535C;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EBD, v1 as u8);
    16
}
/// $02535C: bra.b $2531e
pub fn i_02535c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2535E;
    cpu.ir = 0x60C0;
    cpu.pc = 0x2531E;
    10
}
/// $02535E: addq.b #$1, $106ec6.l
pub fn i_02535e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25364;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x106EC6) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x106EC6, r2 as u8);
    16
}
/// $025364: rts
pub fn i_025364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25366;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputRec_FlushRun_025366  ($025366, 42 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025366: move.w $106ec4.l, d0
pub fn i_025366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2536C;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106EC4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $02536C: cmpi.w #$200, d0
pub fn i_02536c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25370;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Word, false);
    8
}
/// $025370: bcc.w $2538e
pub fn i_025370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25374;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2538E; return 10; }
    8
}
/// $025374: move.b $106ec6.l, d2
pub fn i_025374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2537A;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x106EC6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02537A: move.b d2, (a0, d0.w)
pub fn i_02537a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2537E;
    cpu.ir = 0x1182;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $02537E: move.b #$0, $106ec6.l
pub fn i_02537e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25386;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EC6, v1 as u8);
    16
}
/// $025386: addq.w #$1, d0
pub fn i_025386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25388;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $025388: move.w d0, $106ec4.l
pub fn i_025388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2538E;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC4, v1 as u16);
    16
}
/// $02538E: rts
pub fn i_02538e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25390;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputRec_StoreBitToggle_025390  ($025390, 46 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025390: jsr $25366(pc)
pub fn i_025390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25394;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x25394);
    cpu.pc = 0x25366;
    16
}
/// $025394: move.w $106ec2.l, d0
pub fn i_025394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2539A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106EC2) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $02539A: cmpi.w #$200, d0
pub fn i_02539a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2539E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Word, false);
    8
}
/// $02539E: bcc.w $253bc
pub fn i_02539e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x253BC; return 10; }
    8
}
/// $0253A2: lsl.b #$4, d1
pub fn i_0253a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253A4;
    cpu.ir = 0xE909;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $0253A4: move.b (a0, d0.w), d2
pub fn i_0253a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253A8;
    cpu.ir = 0x1430;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0253A8: andi.b #$f, d2
pub fn i_0253a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253AC;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0253AC: or.b d2, d1
pub fn i_0253ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253AE;
    cpu.ir = 0x8202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0253AE: move.b d1, (a0, d0.w)
pub fn i_0253ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253B2;
    cpu.ir = 0x1181;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $0253B2: move.w $106ec4.l, $106ec2.l
pub fn i_0253b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253BC;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106EC4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC2, v2 as u16);
    28
}
/// $0253BC: rts
pub fn i_0253bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputRec_ReadNextRun_0253be  ($0253BE, 98 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0253BE: move.w $106ec4.l, d0
pub fn i_0253be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253C4;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106EC4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0253C4: cmpi.w #$200, d0
pub fn i_0253c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253C8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Word, false);
    8
}
/// $0253C8: bcc.w $25432
pub fn i_0253c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25432; return 10; }
    8
}
/// $0253CC: move.b (a0, d0.w), d1
pub fn i_0253cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253D0;
    cpu.ir = 0x1230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0253D0: move.b #$ff, d2
pub fn i_0253d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253D4;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0253D4: cmp.w $106ec2.l, d0
pub fn i_0253d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253DA;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106EC2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $0253DA: bne.w $253e4
pub fn i_0253da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x253E4; return 10; }
    8
}
/// $0253DE: move.b #$f, d2
pub fn i_0253de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253E2;
    cpu.ir = 0x143C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0253E2: and.b d2, d1
pub fn i_0253e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253E4;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0253E4: cmp.b $106ec6.l, d1
pub fn i_0253e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253EA;
    cpu.ir = 0xB239;
    let v2 = bus.read8(0x106EC6) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $0253EA: bne.w $25426
pub fn i_0253ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25426; return 10; }
    8
}
/// $0253EE: move.b #$0, $106ec6.l
pub fn i_0253ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253F6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EC6, v1 as u8);
    16
}
/// $0253F6: addq.w #$1, d0
pub fn i_0253f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253F8;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0253F8: cmp.b d2, d1
pub fn i_0253f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253FA;
    cpu.ir = 0xB202;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0253FA: bne.w $25408
pub fn i_0253fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25408; return 10; }
    8
}
/// $0253FE: tst.b (a0, d0.w)
pub fn i_0253fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25402;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $025402: bne.w $25426
pub fn i_025402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25406;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25426; return 10; }
    8
}
/// $025406: addq.w #$1, d0
pub fn i_025406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25408;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $025408: move.w d0, $106ec4.l
pub fn i_025408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2540E;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC4, v1 as u16);
    16
}
/// $02540E: move.w $106ec2.l, d1
pub fn i_02540e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25414;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106EC2) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $025414: move.w d0, $106ec2.l
pub fn i_025414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2541A;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC2, v1 as u16);
    16
}
/// $02541A: move.b (a0, d1.w), d0
pub fn i_02541a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2541E;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02541E: lsr.b #$4, d0
pub fn i_02541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25420;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
// ===== SetC_025420  ($025420, 6 B, thunks/ccr_helpers.c)
/// $025420: ori.b #$1, ccr
pub fn i_025420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25424;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $025424: rts
pub fn i_025424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25426;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputRec_SetReadPos_025426  ($025426, 6 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025426: move.w d0, $106ec4.l
pub fn i_025426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2542C;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EC4, v1 as u16);
    16
}
// ===== ClearC_02542c  ($02542C, 6 B, thunks/ccr_helpers.c)
/// $02542C: andi.b #$fe, ccr
pub fn i_02542c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25430;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $025430: rts
pub fn i_025430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25432;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputRec_SetModeDisabled_025432  ($025432, 8 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025432: move.b #$2, $106eca.l
pub fn i_025432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2543A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECA, v1 as u8);
    16
}
// ===== ClearC_02543a  ($02543A, 6 B, thunks/ccr_helpers.c)
/// $02543A: andi.b #$fe, ccr
pub fn i_02543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2543E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $02543E: rts
pub fn i_02543e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25440;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_025440  ($025440, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025440: movea.l $8(a6), a1
pub fn i_025440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25444;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $025444: move.b $10(a6), d0
pub fn i_025444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25448;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $025448: cmp.b $10(a1), d0
pub fn i_025448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2544C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02544C: bcs.w $25456
pub fn i_02544c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25450;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25456; return 10; }
    8
}
// ===== ClearXN_025450  ($025450, 6 B, thunks/ccr_helpers.c)
/// $025450: andi.b #$ee, ccr
pub fn i_025450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25454;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $025454: rts
pub fn i_025454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25456;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_025456  ($025456, 6 B, thunks/ccr_helpers.c)
/// $025456: ori.b #$11, ccr
pub fn i_025456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2545A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02545A: rts
pub fn i_02545a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2545C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlotIndex_02545c  ($02545C, 44 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02545C: ori.b #$1, d0
pub fn i_02545c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25460;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $025460: ori.b #$3, d2
pub fn i_025460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25464;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $025464: ori.b #$5, d4
pub fn i_025464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25468;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $025468: ori.b #$4a, d3
pub fn i_025468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2546C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02546C: ori.b #$40, (a0)
pub fn i_02546c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25470;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x40) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025470: ori.b #$0, (a0)
pub fn i_025470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25474;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025474: ?
pub fn i_025474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25476;
    cpu.ir = 0x1FF;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $025476: dc.w $ffff
pub fn i_025476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25478;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== PlayerSlotDesc_Idle_025488  ($025488, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025488: ori.b #$0, d0
pub fn i_025488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2548C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02548C: ori.b #$40, (a0)
pub fn i_02548c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25490;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x40) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025490: dc.w $ffff
pub fn i_025490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25492;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== PlayerSlotDesc_Idle2_025498  ($025498, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025498: ori.b #$0, d0
pub fn i_025498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2549C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02549C: ori.b #$e0, (a0)
pub fn i_02549c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254A0;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xE0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0254A0: dc.w $ffff
pub fn i_0254a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254A2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== PlayerSlotDesc_P1Rows_0254a8  ($0254A8, 224 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0254A8: ori.b #$4a, d3
pub fn i_0254a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254AC;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0254AC: ori.b #$40, (a0)
pub fn i_0254ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254B0;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x40) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0254B0: ori.b #$0, (a0)
pub fn i_0254b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254B4;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0254B4: ?
pub fn i_0254b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254B6;
    cpu.ir = 0x1FF;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $0254B6: dc.w $ffff
pub fn i_0254b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254B8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== PlayerSlotDesc_P2Rows_025588  ($025588, 382 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025588: ori.b #$dc, d3
pub fn i_025588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2558C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xDC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02558C: ori.b #$e0, (a0)
pub fn i_02558c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25590;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xE0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025590: ori.b #$6, (a0)
pub fn i_025590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25594;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x6) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025594: ?
pub fn i_025594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25596;
    cpu.ir = 0x1FF;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $025596: dc.w $ffff
pub fn i_025596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25598;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $0256B0: lea.l $10fd84.l, a3
pub fn i_0256b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256B6;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10FD84;
    20
}
/// $0256B6: move.b $4(a3), d1
pub fn i_0256b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256BA;
    cpu.ir = 0x122B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0256BA: move.b d1, $77(a6)
pub fn i_0256ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256BE;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0256BE: jsr $25ed6.l
pub fn i_0256be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x256C4);
    cpu.pc = 0x25ED6;
    16
}
/// $0256C4: cmpi.b #$2, $10fdaf.l
pub fn i_0256c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256CC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $0256CC: bne.w $256e2
pub fn i_0256cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x256E2; return 10; }
    8
}
/// $0256D0: jsr $25f02.l
pub fn i_0256d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x256D6);
    cpu.pc = 0x25F02;
    16
}
/// $0256D6: jsr $260e2.l
pub fn i_0256d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x256DC);
    cpu.pc = 0x260E2;
    16
}
/// $0256DC: jsr $2614e.l
pub fn i_0256dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x256E2);
    cpu.pc = 0x2614E;
    16
}
/// $0256E2: jsr $2630e.l
pub fn i_0256e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x256E8);
    cpu.pc = 0x2630E;
    16
}
/// $0256E8: lea.l $254a8(pc), a0
pub fn i_0256e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x254A8;
    16
}
/// $0256EC: moveq #$0, d0
pub fn i_0256ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256EE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0256EE: move.b $106ece.l, d0
pub fn i_0256ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256F4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0256F4: cmpi.b #$ff, d0
pub fn i_0256f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256F8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0256F8: bne.w $25702
pub fn i_0256f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25702; return 10; }
    8
}
/// $0256FC: lea.l $25488(pc), a0
pub fn i_0256fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25700;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25488;
    16
}
/// $025700: moveq #$0, d0
pub fn i_025700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25702;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $025702: lsl.w #$4, d0
pub fn i_025702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25704;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $025704: rts
pub fn i_025704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25706;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_Task_Init_025706  ($025706, 86 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025706: lea.l $10fd84.l, a3
pub fn i_025706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2570C;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10FD84;
    20
}
/// $02570C: move.b $4(a3), d1
pub fn i_02570c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25710;
    cpu.ir = 0x122B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $025710: move.b d1, $77(a6)
pub fn i_025710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25714;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025714: jsr $25ed6.l
pub fn i_025714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2571A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2571A);
    cpu.pc = 0x25ED6;
    16
}
/// $02571A: cmpi.b #$2, $10fdaf.l
pub fn i_02571a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25722;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $025722: bne.w $25738
pub fn i_025722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25726;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25738; return 10; }
    8
}
/// $025726: jsr $25f4a.l
pub fn i_025726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2572C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2572C);
    cpu.pc = 0x25F4A;
    16
}
/// $02572C: jsr $260ea.l
pub fn i_02572c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25732;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25732);
    cpu.pc = 0x260EA;
    16
}
/// $025732: jsr $26156.l
pub fn i_025732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25738);
    cpu.pc = 0x26156;
    16
}
/// $025738: jsr $26372.l
pub fn i_025738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2573E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2573E);
    cpu.pc = 0x26372;
    16
}
/// $02573E: lea.l $25588(pc), a0
pub fn i_02573e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25742;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25588;
    16
}
/// $025742: moveq #$0, d0
pub fn i_025742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25744;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $025744: move.b $106ece.l, d0
pub fn i_025744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2574A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02574A: cmpi.b #$ff, d0
pub fn i_02574a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2574E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $02574E: bne.w $25758
pub fn i_02574e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25752;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25758; return 10; }
    8
}
/// $025752: lea.l $25498(pc), a0
pub fn i_025752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25756;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25498;
    16
}
/// $025756: clr.w d0
pub fn i_025756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25758;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $025758: lsl.w #$4, d0
pub fn i_025758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2575A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $02575A: rts
pub fn i_02575a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2575C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_Task_StartP1_02575c  ($02575C, 10 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02575C: move.w #$0, $70(a6)
pub fn i_02575c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25762;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $025762: jmp $25780(pc)
pub fn i_025762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25766;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x25780;
    8
}
// ===== HUD_Task_StartP2_025766  ($025766, 10 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025766: move.w #$1, $70(a6)
pub fn i_025766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2576C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02576C: jmp $25780(pc)
pub fn i_02576c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25770;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x25780;
    8
}
/// $025780: lea.l $257ec(pc), a1
pub fn i_025780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x257EC;
    16
}
/// $025784: move.l a1, (a6)
pub fn i_025784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25786;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025786: bsr.w $25e84
pub fn i_025786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2578A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x2578A);
    cpu.pc = 0x25E84;
    18
}
/// $02578A: cmpi.b #$1, d0
pub fn i_02578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2578E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $02578E: bne.w $25798
pub fn i_02578e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25792;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25798; return 10; }
    8
}
/// $025792: lea.l $259b8(pc), a1
pub fn i_025792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25796;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x259B8;
    16
}
/// $025796: move.l a1, (a6)
pub fn i_025796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25798;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025798: cmpi.b #$1, $10fdaf.l
pub fn i_025798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257A0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0257A0: bne.w $257dc
pub fn i_0257a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x257DC; return 10; }
    8
}
/// $0257A4: cmpi.b #$0, d0
pub fn i_0257a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257A8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0257A8: bne.w $257dc
pub fn i_0257a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x257DC; return 10; }
    8
}
/// $0257AC: lea.l $259b8(pc), a1
pub fn i_0257ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x259B8;
    16
}
/// $0257B0: move.l a1, (a6)
pub fn i_0257b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0257B2: lea.l $25770(pc), a0
pub fn i_0257b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257B6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25770;
    16
}
/// $0257B6: move.b $10007b.l, d0
pub fn i_0257b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257BC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10007B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0257BC: andi.l #$1, d0
pub fn i_0257bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257C2;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $0257C2: lsl.w #$3, d0
pub fn i_0257c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257C4;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0257C4: move.w $70(a6), d1
pub fn i_0257c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257C8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0257C8: lsl.w #$2, d1
pub fn i_0257c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257CA;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0257CA: add.w d0, d1
pub fn i_0257ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257CC;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0257CC: movea.l (a0, d1.w), a1
pub fn i_0257cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257D0;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0257D0: cmpa.l #$ffffffff, a1
pub fn i_0257d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257D6;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0257D6: beq.w $257dc
pub fn i_0257d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x257DC; return 10; }
    8
}
/// $0257DA: move.l a1, (a6)
pub fn i_0257da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0257DC: lea.l $2594a(pc), a0
pub fn i_0257dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2594A;
    16
}
/// $0257E0: bsr.w $25eac
pub fn i_0257e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257E4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x257E4);
    cpu.pc = 0x25EAC;
    18
}
// ===== JsrAbsThunk_0257e4  ($0257E4, 8 B, thunks/jsr_abs_thunks.c)
/// $0257E4: jsr $5180c.l
pub fn i_0257e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x257EA);
    cpu.pc = 0x5180C;
    16
}
/// $0257EA: rts
pub fn i_0257ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_InsertCoin_0257ec  ($0257EC, 140 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0257EC: lea.l $25670(pc), a0
pub fn i_0257ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257F0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25670;
    16
}
/// $0257F0: bsr.w $25e92
pub fn i_0257f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x257F4);
    cpu.pc = 0x25E92;
    18
}
/// $0257F4: lea.l $257fa(pc), a1
pub fn i_0257f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x257FA;
    16
}
/// $0257F8: move.l a1, (a6)
pub fn i_0257f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0257FA: jsr $1e28.l
pub fn i_0257fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25800;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25800);
    cpu.pc = 0x1E28;
    16
}
/// $025800: bcs.w $25824
pub fn i_025800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25804;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25824; return 10; }
    8
}
/// $025804: lea.l $25e48(pc), a1
pub fn i_025804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25808;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25E48;
    16
}
/// $025808: move.l a1, (a6)
pub fn i_025808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2580A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02580A: move.l #$257ec, $7c(a6)
pub fn i_02580a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25812;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x257EC;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $025812: lea.l $106ecc.l, a0
pub fn i_025812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25818;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106ECC;
    20
}
/// $025818: adda.w $70(a6), a0
pub fn i_025818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2581C;
    cpu.ir = 0xD0EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[0] = cpu.a[0].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $02581C: move.b (a0), $82(a6)
pub fn i_02581c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25820;
    cpu.ir = 0x1D50;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad3, v2 as u8);
    16
}
/// $025820: move.b #$1, (a0)
pub fn i_025820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25824;
    cpu.ir = 0x10BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $025824: move.b $106f28.l, d0
pub fn i_025824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2582A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02582A: andi.b #$1f, d0
pub fn i_02582a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2582E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02582E: bne.w $2583e
pub fn i_02582e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25832;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2583E; return 10; }
    8
}
/// $025832: lea.l $25670(pc), a0
pub fn i_025832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25836;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25670;
    16
}
/// $025836: bsr.w $25e92
pub fn i_025836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2583A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x2583A);
    cpu.pc = 0x25E92;
    18
}
/// $02583A: bra.w $2584e
pub fn i_02583a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2583E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2584E;
    10
}
/// $02583E: cmpi.b #$18, d0
pub fn i_02583e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25842;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x18, Size::Byte, false);
    8
}
/// $025842: bne.w $2584e
pub fn i_025842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25846;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2584E; return 10; }
    8
}
/// $025846: lea.l $25668(pc), a0
pub fn i_025846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2584A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25668;
    16
}
/// $02584A: bsr.w $25e92
pub fn i_02584a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2584E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x2584E);
    cpu.pc = 0x25E92;
    18
}
/// $02584E: bsr.w $25e84
pub fn i_02584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25852;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25852);
    cpu.pc = 0x25E84;
    18
}
/// $025852: cmpi.b #$1, d0
pub fn i_025852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25856;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $025856: bne.w $25866
pub fn i_025856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2585A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25866; return 10; }
    8
}
/// $02585A: lea.l $259b8(pc), a1
pub fn i_02585a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2585E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x259B8;
    16
}
/// $02585E: move.l a1, (a6)
pub fn i_02585e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25860;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025860: clr.b $20(a6)
pub fn i_025860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25864;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025864: rts
pub fn i_025864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25866;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $025866: move.b #$1, d2
pub fn i_025866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2586A;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02586A: move.w $70(a6), d1
pub fn i_02586a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2586E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02586E: jsr $26752.l
pub fn i_02586e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25874);
    cpu.pc = 0x26752;
    16
}
/// $025874: bcc.w $25880
pub fn i_025874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25878;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25880; return 10; }
    8
}
// ===== SetTaskHandler_025878  ($025878, 8 B, kernel/task_handlers.c)
/// $025878: lea.l $25882(pc), a1
pub fn i_025878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2587C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25882;
    16
}
/// $02587C: move.l a1, (a6)
pub fn i_02587c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2587E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02587E: rts
pub fn i_02587e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25880;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00025880  ($025880, 2 B, thunks/trivial_rts.c)
/// $025880: rts
pub fn i_025880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25882;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_Continue_025882  ($025882, 152 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025882: lea.l $25678(pc), a0
pub fn i_025882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25886;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25678;
    16
}
/// $025886: bsr.w $25e92
pub fn i_025886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2588A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x2588A);
    cpu.pc = 0x25E92;
    18
}
/// $02588A: lea.l $25890(pc), a1
pub fn i_02588a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2588E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25890;
    16
}
/// $02588E: move.l a1, (a6)
pub fn i_02588e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25890;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025890: jsr $1e28.l
pub fn i_025890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25896);
    cpu.pc = 0x1E28;
    16
}
/// $025896: bcs.w $258ba
pub fn i_025896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2589A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x258BA; return 10; }
    8
}
/// $02589A: lea.l $25e48(pc), a1
pub fn i_02589a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2589E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25E48;
    16
}
/// $02589E: move.l a1, (a6)
pub fn i_02589e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0258A0: move.l #$25882, $7c(a6)
pub fn i_0258a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258A8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x25882;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0258A8: lea.l $106ecc.l, a0
pub fn i_0258a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106ECC;
    20
}
/// $0258AE: adda.w $70(a6), a0
pub fn i_0258ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258B2;
    cpu.ir = 0xD0EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[0] = cpu.a[0].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $0258B2: move.b (a0), $82(a6)
pub fn i_0258b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258B6;
    cpu.ir = 0x1D50;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad3, v2 as u8);
    16
}
/// $0258B6: move.b #$1, (a0)
pub fn i_0258b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258BA;
    cpu.ir = 0x10BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $0258BA: move.b $106f28.l, d0
pub fn i_0258ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258C0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0258C0: andi.b #$1f, d0
pub fn i_0258c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258C4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0258C4: bne.w $258d4
pub fn i_0258c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x258D4; return 10; }
    8
}
/// $0258C8: lea.l $25678(pc), a0
pub fn i_0258c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258CC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25678;
    16
}
/// $0258CC: bsr.w $25e92
pub fn i_0258cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258D0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x258D0);
    cpu.pc = 0x25E92;
    18
}
/// $0258D0: bra.w $258e4
pub fn i_0258d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x258E4;
    10
}
/// $0258D4: cmpi.b #$18, d0
pub fn i_0258d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258D8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x18, Size::Byte, false);
    8
}
/// $0258D8: bne.w $258e4
pub fn i_0258d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x258E4; return 10; }
    8
}
/// $0258DC: lea.l $25668(pc), a0
pub fn i_0258dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25668;
    16
}
/// $0258E0: bsr.w $25e92
pub fn i_0258e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258E4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x258E4);
    cpu.pc = 0x25E92;
    18
}
/// $0258E4: bsr.w $25e84
pub fn i_0258e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258E8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x258E8);
    cpu.pc = 0x25E84;
    18
}
/// $0258E8: cmpi.b #$1, d0
pub fn i_0258e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258EC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0258EC: bne.w $25908
pub fn i_0258ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25908; return 10; }
    8
}
/// $0258F0: lea.l $2594a(pc), a0
pub fn i_0258f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258F4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2594A;
    16
}
/// $0258F4: bsr.w $25eac
pub fn i_0258f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258F8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x258F8);
    cpu.pc = 0x25EAC;
    18
}
/// $0258F8: jsr $5180c.l
pub fn i_0258f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x258FE);
    cpu.pc = 0x5180C;
    16
}
/// $0258FE: lea.l $259b8(pc), a1
pub fn i_0258fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25902;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x259B8;
    16
}
/// $025902: move.l a1, (a6)
pub fn i_025902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25904;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025904: bra.w $25920
pub fn i_025904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25908;
    cpu.ir = 0x6000;
    cpu.pc = 0x25920;
    10
}
/// $025908: move.b #$1, d2
pub fn i_025908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2590C;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02590C: move.w $70(a6), d1
pub fn i_02590c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25910;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025910: jsr $26752.l
pub fn i_025910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25916;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25916);
    cpu.pc = 0x26752;
    16
}
/// $025916: bcs.w $25920
pub fn i_025916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2591A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25920; return 10; }
    8
}
// ===== SetTaskHandler_02591a  ($02591A, 8 B, kernel/task_handlers.c)
/// $02591A: lea.l $257ec(pc), a1
pub fn i_02591a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2591E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x257EC;
    16
}
/// $02591E: move.l a1, (a6)
pub fn i_02591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25920;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025920: rts
pub fn i_025920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25922;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_WaitPlayerSpawn_025952  ($025952, 70 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025952: jsr $1e4c.l
pub fn i_025952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25958);
    cpu.pc = 0x1E4C;
    16
}
/// $025958: tst.w $106e92.l
pub fn i_025958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2595E;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $02595E: bne.w $25964
pub fn i_02595e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25962;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25964; return 10; }
    8
}
/// $025962: rts
pub fn i_025962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25964;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $025964: lea.l $2596a(pc), a1
pub fn i_025964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2596A;
    16
}
/// $025968: move.l a1, (a6)
pub fn i_025968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2596A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02596A: jsr $1e4c.l
pub fn i_02596a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25970);
    cpu.pc = 0x1E4C;
    16
}
/// $025970: movea.l $78(a6), a0
pub fn i_025970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25974;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $025974: cmpi.l #$400, (a0)
pub fn i_025974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2597A;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $02597A: bne.w $2599e
pub fn i_02597a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2597E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2599E; return 10; }
    8
}
/// $02597E: cmpi.b #$1, $10fdaf.l
pub fn i_02597e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25986;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $025986: beq.w $259a0
pub fn i_025986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2598A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x259A0; return 10; }
    8
}
/// $02598A: subi.b #$1, $77(a6)
pub fn i_02598a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25990;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $025990: tst.b $77(a6)
pub fn i_025990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25994;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $025994: bne.w $259a0
pub fn i_025994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25998;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x259A0; return 10; }
    8
}
// ===== SetTaskHandler_025998  ($025998, 8 B, kernel/task_handlers.c)
/// $025998: lea.l $25b34(pc), a1
pub fn i_025998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2599C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25B34;
    16
}
/// $02599C: move.l a1, (a6)
pub fn i_02599c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2599E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02599E: rts
pub fn i_02599e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_BindPlayer_0259a0  ($0259A0, 102 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0259A0: lea.l $25932(pc), a0
pub fn i_0259a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259A4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25932;
    16
}
/// $0259A4: bsr.w $25e9e
pub fn i_0259a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259A8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x259A8);
    cpu.pc = 0x25E9E;
    18
}
/// $0259A8: bra.w $259c0
pub fn i_0259a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x259C0;
    10
}
/// $0259AC: lea.l $2593a(pc), a0
pub fn i_0259ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259B0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2593A;
    16
}
/// $0259B0: bsr.w $25e9e
pub fn i_0259b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259B4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x259B4);
    cpu.pc = 0x25E9E;
    18
}
/// $0259B4: bra.w $259c0
pub fn i_0259b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x259C0;
    10
}
/// $0259B8: lea.l $2592a(pc), a0
pub fn i_0259b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259BC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2592A;
    16
}
/// $0259BC: jsr $25e9e(pc)
pub fn i_0259bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x259C0);
    cpu.pc = 0x25E9E;
    16
}
/// $0259C0: movea.l $8(a0, d0.w), a1
pub fn i_0259c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259C4;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0259C4: move.l a1, $72(a6)
pub fn i_0259c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259C8;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0259C8: movea.l (a0, d0.w), a1
pub fn i_0259c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259CC;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0259CC: movea.l $4(a0, d0.w), a2
pub fn i_0259cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259D0;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $0259D0: move.l a6, $c(a2)
pub fn i_0259d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259D4;
    cpu.ir = 0x254E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0259D4: move.l a2, $78(a6)
pub fn i_0259d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259D8;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0259D8: move.l a1, (a2)
pub fn i_0259d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259DA;
    cpu.ir = 0x2489;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[2], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0259DA: move.w #$1, $66(a2)
pub fn i_0259da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259E0;
    cpu.ir = 0x357C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0259E0: jsr $79970.l
pub fn i_0259e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x259E6);
    cpu.pc = 0x79970;
    16
}
/// $0259E6: cmp.w $106e92.l, d0
pub fn i_0259e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259EC;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106E92) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $0259EC: bcs.w $259f6
pub fn i_0259ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259F0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x259F6; return 10; }
    8
}
/// $0259F0: move.w d0, $106e92.l
pub fn i_0259f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259F6;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106E92, v1 as u16);
    16
}
/// $0259F6: lea.l $259fc(pc), a1
pub fn i_0259f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x259FC;
    16
}
/// $0259FA: move.l a1, (a6)
pub fn i_0259fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0259FC: tst.b $106ed2.l
pub fn i_0259fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A02;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $025A02: bne.w $25a0e
pub fn i_025a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25A0E; return 10; }
    8
}
// ===== SetTaskHandler_025a06  ($025A06, 8 B, kernel/task_handlers.c)
/// $025A06: lea.l $25ad8(pc), a1
pub fn i_025a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25AD8;
    16
}
/// $025A0A: move.l a1, (a6)
pub fn i_025a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025A0C: rts
pub fn i_025a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_PlayerDeath_025a0e  ($025A0E, 186 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025A0E: tst.b $106ed3.l
pub fn i_025a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A14;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $025A14: bne.w $25a2e
pub fn i_025a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25A2E; return 10; }
    8
}
/// $025A18: movea.l $78(a6), a0
pub fn i_025a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A1C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $025A1C: ori.b #$3c, $45(a0)
pub fn i_025a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A22;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x3C) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $025A22: bclr.b #$0, $13(a0)
pub fn i_025a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A28;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $025A28: bclr.b #$3, $13(a0)
pub fn i_025a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A2E;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $025A2E: movea.l $78(a6), a0
pub fn i_025a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A32;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $025A32: btst.b #$0, $13(a0)
pub fn i_025a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A38;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $025A38: beq.w $25aae
pub fn i_025a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25AAE; return 10; }
    8
}
/// $025A3C: lea.l $25942(pc), a0
pub fn i_025a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A40;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25942;
    16
}
/// $025A40: bsr.w $25eac
pub fn i_025a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A44;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25A44);
    cpu.pc = 0x25EAC;
    18
}
/// $025A44: clr.w (a1)
pub fn i_025a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A46;
    cpu.ir = 0x4251;
    let ad1 = cpu.a[1];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $025A46: cmpi.b #$2, $106f2a.l
pub fn i_025a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A4E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $025A4E: bne.w $25aa2
pub fn i_025a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25AA2; return 10; }
    8
}
/// $025A52: move.w #$0, d0
pub fn i_025a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A56;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025A56: jsr $5e3a2.l
pub fn i_025a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25A5C);
    cpu.pc = 0x5E3A2;
    16
}
/// $025A5C: bcc.w $25a7e
pub fn i_025a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25A7E; return 10; }
    8
}
/// $025A60: move.w #$1, d0
pub fn i_025a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A64;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025A64: jsr $5e3a2.l
pub fn i_025a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25A6A);
    cpu.pc = 0x5E3A2;
    16
}
/// $025A6A: bcc.w $25a7a
pub fn i_025a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25A7A; return 10; }
    8
}
/// $025A6E: clr.b d0
pub fn i_025a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A70;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $025A70: jsr $1e56.l
pub fn i_025a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25A76);
    cpu.pc = 0x1E56;
    16
}
/// $025A76: bra.w $25aa2
pub fn i_025a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x25AA2;
    10
}
/// $025A7A: bra.w $25a98
pub fn i_025a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x25A98;
    10
}
/// $025A7E: move.w #$1, d0
pub fn i_025a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A82;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025A82: jsr $5e3a2.l
pub fn i_025a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25A88);
    cpu.pc = 0x5E3A2;
    16
}
/// $025A88: bcs.w $25a98
pub fn i_025a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A8C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25A98; return 10; }
    8
}
/// $025A8C: clr.b d0
pub fn i_025a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A8E;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $025A8E: jsr $1e56.l
pub fn i_025a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25A94);
    cpu.pc = 0x1E56;
    16
}
/// $025A94: bra.w $25aa2
pub fn i_025a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A98;
    cpu.ir = 0x6000;
    cpu.pc = 0x25AA2;
    10
}
/// $025A98: move.b #$1e, d0
pub fn i_025a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A9C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025A9C: jsr $1e56.l
pub fn i_025a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25AA2);
    cpu.pc = 0x1E56;
    16
}
/// $025AA2: jsr $1e4c.l
pub fn i_025aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25AA8);
    cpu.pc = 0x1E4C;
    16
}
/// $025AA8: lea.l $25952(pc), a1
pub fn i_025aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25952;
    16
}
/// $025AAC: move.l a1, (a6)
pub fn i_025aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025AAE: lea.l $25698(pc), a0
pub fn i_025aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AB2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25698;
    16
}
/// $025AB2: bsr.w $25e92
pub fn i_025ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AB6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25AB6);
    cpu.pc = 0x25E92;
    18
}
/// $025AB6: lea.l $256a0(pc), a0
pub fn i_025ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ABA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x256A0;
    16
}
/// $025ABA: bsr.w $25e92
pub fn i_025aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ABE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25ABE);
    cpu.pc = 0x25E92;
    18
}
/// $025ABE: lea.l $256a8(pc), a0
pub fn i_025abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AC2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x256A8;
    16
}
/// $025AC2: bsr.w $25e92
pub fn i_025ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AC6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25AC6);
    cpu.pc = 0x25E92;
    18
}
/// $025AC6: rts
pub fn i_025ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_Respawn_025ad8  ($025AD8, 84 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025AD8: lea.l $256a0(pc), a0
pub fn i_025ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ADC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x256A0;
    16
}
/// $025ADC: bsr.w $25e92
pub fn i_025adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AE0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25AE0);
    cpu.pc = 0x25E92;
    18
}
/// $025AE0: lea.l $256a8(pc), a0
pub fn i_025ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AE4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x256A8;
    16
}
/// $025AE4: bsr.w $25e92
pub fn i_025ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AE8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25AE8);
    cpu.pc = 0x25E92;
    18
}
/// $025AE8: movea.l $78(a6), a0
pub fn i_025ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $025AEC: cmpi.l #$400, (a0)
pub fn i_025aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AF2;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $025AF2: bne.w $25b00
pub fn i_025af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25B00; return 10; }
    8
}
/// $025AF6: tst.b $106ed2.l
pub fn i_025af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AFC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $025AFC: bne.w $25b02
pub fn i_025afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25B02; return 10; }
    8
}
/// $025B00: rts
pub fn i_025b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $025B02: jsr $1e28.l
pub fn i_025b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25B08);
    cpu.pc = 0x1E28;
    16
}
/// $025B08: bcs.w $25b0e
pub fn i_025b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B0C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25B0E; return 10; }
    8
}
/// $025B0C: rts
pub fn i_025b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $025B0E: lea.l $25ac8(pc), a4
pub fn i_025b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B12;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x25AC8;
    16
}
/// $025B12: move.w $70(a6), d1
pub fn i_025b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B16;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025B16: lsl.w #$3, d1
pub fn i_025b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B18;
    cpu.ir = 0xE749;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $025B18: movea.l (a4, d1.w), a0
pub fn i_025b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B1C;
    cpu.ir = 0x2074;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $025B1C: movea.l $4(a4, d1.w), a1
pub fn i_025b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B20;
    cpu.ir = 0x2274;
    let ad2 = cpu.a[4].wrapping_add(0x4).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $025B20: move.l a1, (a0)
pub fn i_025b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B22;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025B22: jsr $5fe.l
pub fn i_025b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25B28);
    cpu.pc = 0x5FE;
    16
}
/// $025B28: jmp $259ac(pc)
pub fn i_025b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B2C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x259AC;
    8
}
// ===== HUD_State_GameOverEntry_025b34  ($025B34, 272 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025B34: lea.l $25668(pc), a0
pub fn i_025b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B38;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25668;
    16
}
/// $025B38: bsr.w $25e92
pub fn i_025b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B3C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25B3C);
    cpu.pc = 0x25E92;
    18
}
/// $025B3C: tst.b $10fd82.l
pub fn i_025b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B42;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $025B42: beq.w $25b54
pub fn i_025b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25B54; return 10; }
    8
}
/// $025B46: tst.b $10fd8a.l
pub fn i_025b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B4C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $025B4C: beq.w $25d64
pub fn i_025b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25D64; return 10; }
    8
}
/// $025B50: bra.w $25b66
pub fn i_025b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B54;
    cpu.ir = 0x6000;
    cpu.pc = 0x25B66;
    10
}
/// $025B54: move.b #$1, d2
pub fn i_025b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B58;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025B58: move.w $70(a6), d1
pub fn i_025b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B5C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025B5C: jsr $26752.l
pub fn i_025b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25B62);
    cpu.pc = 0x26752;
    16
}
/// $025B62: bcc.w $25d64
pub fn i_025b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25D64; return 10; }
    8
}
/// $025B66: lea.l $9b872.l, a1
pub fn i_025b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x9B872;
    20
}
/// $025B6C: jsr $4498e.l
pub fn i_025b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25B72);
    cpu.pc = 0x4498E;
    16
}
/// $025B72: move.w $70(a6), $98(a0)
pub fn i_025b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B78;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write16(ad4, v3 as u16);
    20
}
/// $025B78: move.b #$2, d0
pub fn i_025b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B7C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025B7C: jsr $25e74(pc)
pub fn i_025b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x25B80);
    cpu.pc = 0x25E74;
    16
}
/// $025B80: move.b #$1, d2
pub fn i_025b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B84;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025B84: move.w $70(a6), d1
pub fn i_025b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B88;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025B88: jsr $26752.l
pub fn i_025b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25B8E);
    cpu.pc = 0x26752;
    16
}
/// $025B8E: bcs.w $25c44
pub fn i_025b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B92;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25C44; return 10; }
    8
}
/// $025B92: lea.l $25b2c(pc), a4
pub fn i_025b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B96;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x25B2C;
    16
}
/// $025B96: move.w $70(a6), d1
pub fn i_025b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B9A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025B9A: lsl.w #$2, d1
pub fn i_025b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B9C;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $025B9C: movea.l (a4, d1.w), a1
pub fn i_025b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BA0;
    cpu.ir = 0x2274;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $025BA0: jsr $4ae.l
pub fn i_025ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25BA6);
    cpu.pc = 0x4AE;
    16
}
/// $025BA6: move.b #$ff, $21(a6)
pub fn i_025ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BAC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025BAC: move.b #$ff, $20(a6)
pub fn i_025bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025BB2: lea.l $25cce(pc), a1
pub fn i_025bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25CCE;
    16
}
/// $025BB6: jsr $4ae.l
pub fn i_025bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25BBC);
    cpu.pc = 0x4AE;
    16
}
/// $025BBC: move.b $21(a6), $21(a0)
pub fn i_025bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BC2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
/// $025BC2: move.b $20(a6), $20(a0)
pub fn i_025bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BC8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $025BC8: lea.l $25bce(pc), a1
pub fn i_025bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25BCE;
    16
}
/// $025BCC: move.l a1, (a6)
pub fn i_025bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025BCE: jsr $1e4c.l
pub fn i_025bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25BD4);
    cpu.pc = 0x1E4C;
    16
}
/// $025BD4: bsr.w $25e84
pub fn i_025bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BD8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25BD8);
    cpu.pc = 0x25E84;
    18
}
/// $025BD8: cmpi.b #$1, d0
pub fn i_025bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BDC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $025BDC: bne.w $25bfc
pub fn i_025bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25BFC; return 10; }
    8
}
/// $025BE0: lea.l $259b8(pc), a1
pub fn i_025be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x259B8;
    16
}
/// $025BE4: move.l a1, (a6)
pub fn i_025be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025BE6: clr.b $20(a6)
pub fn i_025be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BEA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025BEA: lea.l $2594a(pc), a0
pub fn i_025bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BEE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2594A;
    16
}
/// $025BEE: bsr.w $25eac
pub fn i_025bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BF2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25BF2);
    cpu.pc = 0x25EAC;
    18
}
/// $025BF2: jsr $51a86.l
pub fn i_025bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25BF8);
    cpu.pc = 0x51A86;
    16
}
/// $025BF8: bra.w $25c42
pub fn i_025bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x25C42;
    10
}
/// $025BFC: move.b #$1, d2
pub fn i_025bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C00;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025C00: move.w $70(a6), d1
pub fn i_025c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C04;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025C04: jsr $26752.l
pub fn i_025c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25C0A);
    cpu.pc = 0x26752;
    16
}
/// $025C0A: bcc.w $25c1c
pub fn i_025c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25C1C; return 10; }
    8
}
/// $025C0E: lea.l $25c44(pc), a1
pub fn i_025c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25C44;
    16
}
/// $025C12: move.l a1, (a6)
pub fn i_025c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025C14: clr.b $20(a6)
pub fn i_025c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C18;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025C18: bra.w $25c42
pub fn i_025c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x25C42;
    10
}
/// $025C1C: bsr.w $25e84
pub fn i_025c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C20;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25C20);
    cpu.pc = 0x25E84;
    18
}
/// $025C20: cmpi.b #$1, d0
pub fn i_025c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C24;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $025C24: beq.w $25c42
pub fn i_025c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25C42; return 10; }
    8
}
/// $025C28: tst.b $21(a6)
pub fn i_025c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C2C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $025C2C: bne.w $25c42
pub fn i_025c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25C42; return 10; }
    8
}
/// $025C30: lea.l $25d64(pc), a1
pub fn i_025c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25D64;
    16
}
/// $025C34: move.l a1, (a6)
pub fn i_025c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025C36: move.b #$1, d0
pub fn i_025c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C3A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025C3A: jsr $25e74(pc)
pub fn i_025c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x25C3E);
    cpu.pc = 0x25E74;
    16
}
/// $025C3E: bra.w $25c42
pub fn i_025c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C42;
    cpu.ir = 0x6000;
    cpu.pc = 0x25C42;
    10
}
/// $025C42: rts
pub fn i_025c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_ContinueCountdown_025c44  ($025C44, 122 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025C44: move.w $70(a6), d1
pub fn i_025c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C48;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025C48: lea.l $25b2c(pc), a4
pub fn i_025c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C4C;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x25B2C;
    16
}
/// $025C4C: lsl.w #$2, d1
pub fn i_025c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C4E;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $025C4E: movea.l (a4, d1.w), a1
pub fn i_025c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C52;
    cpu.ir = 0x2274;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $025C52: jsr $4ae.l
pub fn i_025c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25C58);
    cpu.pc = 0x4AE;
    16
}
/// $025C58: move.b #$ff, $21(a6)
pub fn i_025c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C5E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025C5E: move.b #$ff, $20(a6)
pub fn i_025c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C64;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025C64: lea.l $25cce(pc), a1
pub fn i_025c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25CCE;
    16
}
/// $025C68: jsr $4ae.l
pub fn i_025c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25C6E);
    cpu.pc = 0x4AE;
    16
}
/// $025C6E: move.b $21(a6), $21(a0)
pub fn i_025c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C74;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
/// $025C74: move.b $20(a6), $20(a0)
pub fn i_025c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C7A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $025C7A: lea.l $25c80(pc), a1
pub fn i_025c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25C80;
    16
}
/// $025C7E: move.l a1, (a6)
pub fn i_025c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025C80: jsr $1e4c.l
pub fn i_025c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25C86);
    cpu.pc = 0x1E4C;
    16
}
/// $025C86: bsr.w $25e84
pub fn i_025c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C8A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25C8A);
    cpu.pc = 0x25E84;
    18
}
/// $025C8A: cmpi.b #$1, d0
pub fn i_025c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C8E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $025C8E: bne.w $25cae
pub fn i_025c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25CAE; return 10; }
    8
}
/// $025C92: clr.b $20(a6)
pub fn i_025c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C96;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025C96: lea.l $2594a(pc), a0
pub fn i_025c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C9A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2594A;
    16
}
/// $025C9A: bsr.w $25eac
pub fn i_025c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C9E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25C9E);
    cpu.pc = 0x25EAC;
    18
}
/// $025C9E: jsr $51a86.l
pub fn i_025c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25CA4);
    cpu.pc = 0x51A86;
    16
}
/// $025CA4: lea.l $259b8(pc), a1
pub fn i_025ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x259B8;
    16
}
/// $025CA8: move.l a1, (a6)
pub fn i_025ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025CAA: bra.w $25cc4
pub fn i_025caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x25CC4;
    10
}
/// $025CAE: tst.b $21(a6)
pub fn i_025cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CB2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $025CB2: bne.w $25cc4
pub fn i_025cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25CC4; return 10; }
    8
}
/// $025CB6: move.b #$1, d0
pub fn i_025cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CBA;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025CBA: bsr.w $25e74
pub fn i_025cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CBE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25CBE);
    cpu.pc = 0x25E74;
    18
}
// ===== SetTaskHandler_025cbe  ($025CBE, 8 B, kernel/task_handlers.c)
/// $025CBE: lea.l $25d64(pc), a1
pub fn i_025cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25D64;
    16
}
/// $025CC2: move.l a1, (a6)
pub fn i_025cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025CC4: rts
pub fn i_025cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $025CCE: lea.l $10fdb6.l, a4
pub fn i_025cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CD4;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10FDB6;
    20
}
/// $025CD4: movea.l $c(a6), a0
pub fn i_025cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CD8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $025CD8: move.w $70(a0), d1
pub fn i_025cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CDC;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025CDC: eori.w #$1, d1
pub fn i_025cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CE0;
    cpu.ir = 0xA41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025CE0: move.b (a4, d1.w), d0
pub fn i_025ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CE4;
    cpu.ir = 0x1034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $025CE4: cmpi.b #$1, d0
pub fn i_025ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CE8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $025CE8: beq.w $25d5c
pub fn i_025ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25D5C; return 10; }
    8
}
/// $025CEC: cmpi.b #$2, d0
pub fn i_025cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CF0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $025CF0: beq.w $25d5c
pub fn i_025cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25D5C; return 10; }
    8
}
/// $025CF4: move.w $70(a0), d1
pub fn i_025cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CF8;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025CF8: lea.l $25cc6(pc), a0
pub fn i_025cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CFC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25CC6;
    16
}
/// $025CFC: lsl.w #$2, d1
pub fn i_025cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CFE;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $025CFE: movea.l (a0, d1.w), a1
pub fn i_025cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D02;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $025D02: jsr $6fe.l
pub fn i_025d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25D08);
    cpu.pc = 0x6FE;
    16
}
/// $025D08: lea.l $25d0e(pc), a1
pub fn i_025d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25D0E;
    16
}
/// $025D0C: move.l a1, (a6)
pub fn i_025d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025D0E: movea.l $c(a6), a0
pub fn i_025d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D12;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $025D12: move.b $20(a0), $20(a6)
pub fn i_025d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D18;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $025D18: lea.l $10fdb6.l, a4
pub fn i_025d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D1E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10FDB6;
    20
}
/// $025D1E: move.w $70(a0), d1
pub fn i_025d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D22;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025D22: eori.w #$1, d1
pub fn i_025d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D26;
    cpu.ir = 0xA41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025D26: move.b (a4, d1.w), d0
pub fn i_025d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D2A;
    cpu.ir = 0x1034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $025D2A: cmpi.b #$0, d0
pub fn i_025d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D2E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $025D2E: beq.w $25d3e
pub fn i_025d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25D3E; return 10; }
    8
}
/// $025D32: cmpi.b #$3, d0
pub fn i_025d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D36;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $025D36: beq.w $25d3e
pub fn i_025d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25D3E; return 10; }
    8
}
/// $025D3A: clr.b $20(a6)
pub fn i_025d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D3E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025D3E: tst.b $20(a6)
pub fn i_025d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D42;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $025D42: bne.w $25d4c
pub fn i_025d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25D4C; return 10; }
    8
}
/// $025D46: lea.l $25d5c(pc), a1
pub fn i_025d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25D5C;
    16
}
/// $025D4A: move.l a1, (a6)
pub fn i_025d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025D4C: tst.b $21(a6)
pub fn i_025d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D50;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $025D50: bne.w $25d5a
pub fn i_025d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25D5A; return 10; }
    8
}
// ===== SetTaskHandler_025d54  ($025D54, 8 B, kernel/task_handlers.c)
/// $025D54: lea.l $25d5c(pc), a1
pub fn i_025d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25D5C;
    16
}
/// $025D58: move.l a1, (a6)
pub fn i_025d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025D5A: rts
pub fn i_025d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_025d5c  ($025D5C, 8 B, kernel/jmp_to_scheduler.c)
/// $025D5C: jmp $518.l
pub fn i_025d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D62;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== HUD_State_GameOverFinal_025d64  ($025D64, 104 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025D64: move.b #$1, d0
pub fn i_025d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D68;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025D68: jsr $25e74(pc)
pub fn i_025d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x25D6C);
    cpu.pc = 0x25E74;
    16
}
/// $025D6C: move.b #$ff, $21(a6)
pub fn i_025d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025D72: move.b #$ff, $20(a6)
pub fn i_025d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025D78: move.w $70(a6), d1
pub fn i_025d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D7C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025D7C: lea.l $25922(pc), a0
pub fn i_025d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D80;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25922;
    16
}
/// $025D80: bsr.w $25e9e
pub fn i_025d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D84;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25D84);
    cpu.pc = 0x25E9E;
    18
}
/// $025D84: lea.l $25d8a(pc), a1
pub fn i_025d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25D8A;
    16
}
/// $025D88: move.l a1, (a6)
pub fn i_025d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025D8A: jsr $1e4c.l
pub fn i_025d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25D90);
    cpu.pc = 0x1E4C;
    16
}
/// $025D90: tst.b $21(a6)
pub fn i_025d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D94;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $025D94: bne.w $25dd0
pub fn i_025d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25DD0; return 10; }
    8
}
/// $025D98: lea.l $25668(pc), a0
pub fn i_025d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D9C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25668;
    16
}
/// $025D9C: bsr.w $25e92
pub fn i_025d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DA0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25DA0);
    cpu.pc = 0x25E92;
    18
}
/// $025DA0: cmpi.b #$6, $106ed0.l
pub fn i_025da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DA8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED0) as u32;
    sub_with_flags(cpu, v2, 0x6, Size::Byte, false);
    20
}
/// $025DA8: bcc.w $25db6
pub fn i_025da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25DB6; return 10; }
    8
}
/// $025DAC: lea.l $25dd2(pc), a1
pub fn i_025dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25DD2;
    16
}
/// $025DB0: move.l a1, (a6)
pub fn i_025db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025DB2: bra.w $25dc4
pub fn i_025db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x25DC4;
    10
}
/// $025DB6: jsr $5b6.l
pub fn i_025db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25DBC);
    cpu.pc = 0x5B6;
    16
}
/// $025DBC: lea.l $400.l, a1
pub fn i_025dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $025DC2: move.l a1, (a6)
pub fn i_025dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025DC4: clr.b $20(a6)
pub fn i_025dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DC8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025DC8: move.b #$3, d0
pub fn i_025dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DCC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== JsrPcThunk_025dcc  ($025DCC, 6 B, thunks/jsr_pc_thunks.c)
/// $025DCC: jsr $25e74(pc)
pub fn i_025dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x25DD0);
    cpu.pc = 0x25E74;
    16
}
/// $025DD0: rts
pub fn i_025dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_GameOverDone_025dd2  ($025DD2, 110 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025DD2: move.w $70(a6), d1
pub fn i_025dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DD6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025DD6: lea.l $25690(pc), a4
pub fn i_025dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DDA;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x25690;
    16
}
/// $025DDA: lsl.w #$2, d1
pub fn i_025dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DDC;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $025DDC: movea.l (a4, d1.w), a0
pub fn i_025ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DE0;
    cpu.ir = 0x2074;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $025DE0: jsr (a0)
pub fn i_025de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DE2;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x25DE2);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $025DE2: lea.l $25942(pc), a0
pub fn i_025de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DE6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25942;
    16
}
/// $025DE6: bsr.w $25eac
pub fn i_025de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DEA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25DEA);
    cpu.pc = 0x25EAC;
    18
}
/// $025DEA: clr.w (a1)
pub fn i_025dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DEC;
    cpu.ir = 0x4251;
    let ad1 = cpu.a[1];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $025DEC: lea.l $256a0(pc), a0
pub fn i_025dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DF0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x256A0;
    16
}
/// $025DF0: bsr.w $25e92
pub fn i_025df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DF4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25DF4);
    cpu.pc = 0x25E92;
    18
}
/// $025DF4: move.b #$1e, $44(a6)
pub fn i_025df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DFA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025DFA: lea.l $25e00(pc), a1
pub fn i_025dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25E00;
    16
}
/// $025DFE: move.l a1, (a6)
pub fn i_025dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025E00: tst.b $44(a6)
pub fn i_025e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E04;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $025E04: bne.w $25e46
pub fn i_025e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25E46; return 10; }
    8
}
/// $025E08: jsr $1e28.l
pub fn i_025e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25E0E);
    cpu.pc = 0x1E28;
    16
}
/// $025E0E: bcc.w $25e46
pub fn i_025e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25E46; return 10; }
    8
}
/// $025E12: move.b #$0, d0
pub fn i_025e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E16;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025E16: jsr $25e74(pc)
pub fn i_025e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x25E1A);
    cpu.pc = 0x25E74;
    16
}
/// $025E1A: move.b #$1, d2
pub fn i_025e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E1E;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025E1E: move.w $70(a6), d1
pub fn i_025e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E22;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025E22: jsr $26752.l
pub fn i_025e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25E28);
    cpu.pc = 0x26752;
    16
}
/// $025E28: bcc.w $25e36
pub fn i_025e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25E36; return 10; }
    8
}
/// $025E2C: lea.l $25882(pc), a1
pub fn i_025e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25882;
    16
}
/// $025E30: move.l a1, (a6)
pub fn i_025e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025E32: bra.w $25e46
pub fn i_025e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E36;
    cpu.ir = 0x6000;
    cpu.pc = 0x25E46;
    10
}
/// $025E36: tst.b $10fd82.l
pub fn i_025e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E3C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $025E3C: beq.w $25e46
pub fn i_025e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x25E46; return 10; }
    8
}
// ===== SetTaskHandler_025e40  ($025E40, 8 B, kernel/task_handlers.c)
/// $025E40: lea.l $257ec(pc), a1
pub fn i_025e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x257EC;
    16
}
/// $025E44: move.l a1, (a6)
pub fn i_025e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025E46: rts
pub fn i_025e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_State_PushStartBlink_025e48  ($025E48, 44 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025E48: lea.l $25680(pc), a0
pub fn i_025e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E4C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x25680;
    16
}
/// $025E4C: bsr.w $25e92
pub fn i_025e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E50;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25E50);
    cpu.pc = 0x25E92;
    18
}
/// $025E50: lea.l $25e56(pc), a1
pub fn i_025e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x25E56;
    16
}
/// $025E54: move.l a1, (a6)
pub fn i_025e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $025E56: jsr $1e28.l
pub fn i_025e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25E5C);
    cpu.pc = 0x1E28;
    16
}
/// $025E5C: bcc.w $25e72
pub fn i_025e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x25E72; return 10; }
    8
}
/// $025E60: move.l $7c(a6), (a6)
pub fn i_025e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E64;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $025E64: lea.l $106ecc.l, a0
pub fn i_025e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106ECC;
    20
}
/// $025E6A: adda.w $70(a6), a0
pub fn i_025e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E6E;
    cpu.ir = 0xD0EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[0] = cpu.a[0].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $025E6E: move.b $82(a6), (a0)
pub fn i_025e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E72;
    cpu.ir = 0x10AE;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v3 as u8);
    16
}
/// $025E72: rts
pub fn i_025e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_SetStartMask_025e74  ($025E74, 30 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025E74: lea.l $10fdb6.l, a0
pub fn i_025e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $025E7A: move.w $70(a6), d1
pub fn i_025e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E7E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025E7E: move.b d0, (a0, d1.w)
pub fn i_025e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E82;
    cpu.ir = 0x1180;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $025E82: rts
pub fn i_025e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $025E84: lea.l $10fdb6.l, a0
pub fn i_025e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $025E8A: adda.w $70(a6), a0
pub fn i_025e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E8E;
    cpu.ir = 0xD0EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[0] = cpu.a[0].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $025E8E: move.b (a0), d0
pub fn i_025e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E90;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $025E90: rts
pub fn i_025e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_CallPerPlayer_IfMode2_025e92  ($025E92, 26 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025E92: cmpi.b #$2, $10fdaf.l
pub fn i_025e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E9A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $025E9A: bne.w $25eaa
pub fn i_025e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25EAA; return 10; }
    8
}
/// $025E9E: move.w $70(a6), d1
pub fn i_025e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EA2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025EA2: lsl.w #$2, d1
pub fn i_025ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EA4;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $025EA4: movea.l (a0, d1.w), a0
pub fn i_025ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EA8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $025EA8: jmp (a0)
pub fn i_025ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EAA;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $025EAA: rts
pub fn i_025eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_LoadPerPlayerPtr_025eac  ($025EAC, 14 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025EAC: move.w $70(a6), d1
pub fn i_025eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EB0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $025EB0: add.w d1, d1
pub fn i_025eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EB2;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $025EB2: add.w d1, d1
pub fn i_025eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EB4;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $025EB4: movea.l (a0, d1.w), a1
pub fn i_025eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EB8;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $025EB8: rts
pub fn i_025eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_025eba  ($025EBA, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025EBA: movea.l $8(a6), a1
pub fn i_025eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EBE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $025EBE: move.b $10(a6), d0
pub fn i_025ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EC2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $025EC2: cmp.b $10(a1), d0
pub fn i_025ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EC6;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $025EC6: bcs.w $25ed0
pub fn i_025ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ECA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x25ED0; return 10; }
    8
}
// ===== ClearXN_025eca  ($025ECA, 6 B, thunks/ccr_helpers.c)
/// $025ECA: andi.b #$ee, ccr
pub fn i_025eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ECE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $025ECE: rts
pub fn i_025ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ED0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_025ed0  ($025ED0, 6 B, thunks/ccr_helpers.c)
/// $025ED0: ori.b #$11, ccr
pub fn i_025ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ED4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $025ED4: rts
pub fn i_025ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25ED6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_ResetFields_025ed6  ($025ED6, 44 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025ED6: clr.b $76(a6)
pub fn i_025ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EDA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025EDA: move.b #$1, $84(a6)
pub fn i_025eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EE0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025EE0: move.b #$1, $85(a6)
pub fn i_025ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025EE6: clr.b $93(a6)
pub fn i_025ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EEA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x93);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $025EEA: clr.w $8e(a6)
pub fn i_025eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EEE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $025EEE: clr.w $90(a6)
pub fn i_025eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EF2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $025EF2: clr.l $86(a6)
pub fn i_025ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EF6;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $025EF6: clr.l $8a(a6)
pub fn i_025ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EFA;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $025EFA: move.b #$0, $92(a6)
pub fn i_025efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $025F00: rts
pub fn i_025f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_DrawPlayerLabels_025f02  ($025F02, 144 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025F02: movea.w #$7065, a1
pub fn i_025f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F06;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7065;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $025F06: move.w #$5300, d0
pub fn i_025f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F0A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F0A: lea.l $2785b8.l, a2
pub fn i_025f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785B8;
    20
}
/// $025F10: jsr $5dad8.l
pub fn i_025f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25F16);
    cpu.pc = 0x5DAD8;
    16
}
/// $025F16: move.w #$7044, d0
pub fn i_025f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F1A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7044;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F1A: bsr.w $25f92
pub fn i_025f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F1E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25F1E);
    cpu.pc = 0x25F92;
    18
}
/// $025F1E: movea.w #$7143, a1
pub fn i_025f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F22;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7143;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $025F22: move.w #$7ab0, d0
pub fn i_025f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F26;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7AB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F26: move.w #$8, d1
pub fn i_025f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F2A: move.w #$3, d2
pub fn i_025f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F2E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F2E: jsr $5da56.l
pub fn i_025f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25F34);
    cpu.pc = 0x5DA56;
    16
}
/// $025F34: movea.w #$71c3, a1
pub fn i_025f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F38;
    cpu.ir = 0x327C;
    let v1: u32 = 0x71C3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $025F38: move.w #$7ae4, d0
pub fn i_025f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F3C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7AE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F3C: move.w #$4, d1
pub fn i_025f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F40;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F40: move.w #$1, d2
pub fn i_025f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F44;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F44: jmp $5da56.l
pub fn i_025f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F4A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DA56;
    8
}
/// $025F4A: movea.w #$73e5, a1
pub fn i_025f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F4E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x73E5;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $025F4E: move.w #$5300, d0
pub fn i_025f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F52: lea.l $2785be.l, a2
pub fn i_025f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F58;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785BE;
    20
}
/// $025F58: jsr $5dad8.l
pub fn i_025f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25F5E);
    cpu.pc = 0x5DAD8;
    16
}
/// $025F5E: move.w #$73c4, d0
pub fn i_025f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x73C4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F62: bsr.w $25f92
pub fn i_025f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F66;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x25F66);
    cpu.pc = 0x25F92;
    18
}
/// $025F66: movea.w #$72c3, a1
pub fn i_025f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F6A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72C3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $025F6A: move.w #$7ab0, d0
pub fn i_025f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7AB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F6E: move.w #$8, d1
pub fn i_025f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F72;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F72: move.w #$3, d2
pub fn i_025f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F76;
    cpu.ir = 0x343C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F76: jsr $5da56.l
pub fn i_025f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x25F7C);
    cpu.pc = 0x5DA56;
    16
}
/// $025F7C: movea.w #$7343, a1
pub fn i_025f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F80;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7343;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $025F80: move.w #$7ae4, d0
pub fn i_025f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7AE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F84: move.w #$4, d1
pub fn i_025f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F88;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F88: move.w #$1, d2
pub fn i_025f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F8C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F8C: jmp $5da56.l
pub fn i_025f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F92;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DA56;
    8
}
// ===== HUD_DrawLifeBarFrame_025f92  ($025F92, 100 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025F92: move.w #$7aa5, d7
pub fn i_025f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F96;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7AA5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $025F96: movem.w d0/d7, $3c0000.l
pub fn i_025f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F9E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $025F9E: addi.w #$20, d0
pub fn i_025f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FA2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025FA2: move.b #$a6, d7
pub fn i_025fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FA6;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xA6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025FA6: movem.w d0/d7, $3c0000.l
pub fn i_025fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FAE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $025FAE: addi.w #$20, d0
pub fn i_025fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FB2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025FB2: movem.w d0/d7, $3c0000.l
pub fn i_025fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FBA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $025FBA: addi.w #$20, d0
pub fn i_025fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FBE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025FBE: movem.w d0/d7, $3c0000.l
pub fn i_025fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FC6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $025FC6: addi.w #$20, d0
pub fn i_025fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FCA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025FCA: movem.w d0/d7, $3c0000.l
pub fn i_025fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FD2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $025FD2: addi.w #$20, d0
pub fn i_025fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FD6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025FD6: movem.w d0/d7, $3c0000.l
pub fn i_025fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FDE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $025FDE: addi.w #$20, d0
pub fn i_025fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FE2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025FE2: movem.w d0/d7, $3c0000.l
pub fn i_025fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FEA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $025FEA: addi.w #$20, d0
pub fn i_025fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FEE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $025FEE: move.b #$af, d7
pub fn i_025fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FF2;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $025FF2: movem.w d0/d7, $3c0000.l
pub fn i_025ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FFA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
// ===== NopCCR_025ff6  ($025FF6, 6 B, thunks/ccr_helpers.c)
/// $025FF6: ori.b #$0, ccr
pub fn i_025ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FFA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x0);
    20
}
/// $025FFA: rts
pub fn i_025ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_Msg_InsertCoin_P1_025ffc  ($025FFC, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $025FFC: movea.w #$7063, a1
pub fn i_025ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26000;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $026000: move.b #$3, d1
pub fn i_026000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26004;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026004: lea.l $2785c4.l, a2
pub fn i_026004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2600A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785C4;
    20
}
/// $02600A: jmp $477fc.l
pub fn i_02600a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26010;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_InsertCoin_P2_026010  ($026010, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026010: movea.w #$7343, a1
pub fn i_026010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26014;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7343;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $026014: move.b #$3, d1
pub fn i_026014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26018;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026018: lea.l $2785c4.l, a2
pub fn i_026018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2601E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785C4;
    20
}
/// $02601E: jmp $477fc.l
pub fn i_02601e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26024;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_PushStart_P1_026024  ($026024, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026024: movea.w #$7063, a1
pub fn i_026024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26028;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $026028: move.b #$3, d1
pub fn i_026028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2602C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02602C: lea.l $2785d0.l, a2
pub fn i_02602c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26032;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785D0;
    20
}
/// $026032: jmp $477fc.l
pub fn i_026032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26038;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_PushStart_P2_026038  ($026038, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026038: movea.w #$7343, a1
pub fn i_026038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2603C;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7343;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $02603C: move.b #$3, d1
pub fn i_02603c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26040;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026040: lea.l $2785d0.l, a2
pub fn i_026040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26046;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785D0;
    20
}
/// $026046: jmp $477fc.l
pub fn i_026046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2604C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_Continue_P1_02604c  ($02604C, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02604C: movea.w #$7063, a1
pub fn i_02604c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26050;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $026050: move.b #$3, d1
pub fn i_026050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26054;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026054: lea.l $2785dc.l, a2
pub fn i_026054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2605A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785DC;
    20
}
/// $02605A: jmp $477fc.l
pub fn i_02605a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26060;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_Continue_P2_026060  ($026060, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026060: movea.w #$7343, a1
pub fn i_026060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26064;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7343;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $026064: move.b #$3, d1
pub fn i_026064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26068;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026068: lea.l $2785dc.l, a2
pub fn i_026068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2606E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785DC;
    20
}
/// $02606E: jmp $477fc.l
pub fn i_02606e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26074;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_GameOver_P1_026074  ($026074, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026074: movea.w #$7063, a1
pub fn i_026074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26078;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $026078: move.b #$3, d1
pub fn i_026078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2607C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02607C: lea.l $2785e8.l, a2
pub fn i_02607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26082;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785E8;
    20
}
/// $026082: jmp $477fc.l
pub fn i_026082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26088;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_GameOver_P2_026088  ($026088, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026088: movea.w #$7343, a1
pub fn i_026088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2608C;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7343;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $02608C: move.b #$3, d1
pub fn i_02608c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26090;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026090: lea.l $2785e8.l, a2
pub fn i_026090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26096;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785E8;
    20
}
/// $026096: jmp $477fc.l
pub fn i_026096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2609C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_PleaseWait_P1_02609c  ($02609C, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02609C: movea.w #$7063, a1
pub fn i_02609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260A0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $0260A0: move.b #$3, d1
pub fn i_0260a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260A4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0260A4: lea.l $2785f4.l, a2
pub fn i_0260a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260AA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785F4;
    20
}
/// $0260AA: jmp $477fc.l
pub fn i_0260aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260B0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_Msg_PleaseWait_P2_0260b0  ($0260B0, 20 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0260B0: movea.w #$7343, a1
pub fn i_0260b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260B4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7343;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $0260B4: move.b #$3, d1
pub fn i_0260b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260B8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0260B8: lea.l $2785f4.l, a2
pub fn i_0260b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260BE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2785F4;
    20
}
/// $0260BE: jmp $477fc.l
pub fn i_0260be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x477FC;
    8
}
// ===== HUD_ClearMsgRow_0260c4  ($0260C4, 30 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0260C4: movea.w #$7043, a1
pub fn i_0260c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260C8;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7043;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $0260C8: bra.w $260d0
pub fn i_0260c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x260D0;
    10
}
/// $0260CC: movea.w #$72c3, a1
pub fn i_0260cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260D0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72C3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $0260D0: move.w #$300, d0
pub fn i_0260d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0260D4: move.w #$10, d1
pub fn i_0260d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0260D8: move.w #$3, d2
pub fn i_0260d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260DC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0260DC: jmp $5da9c.l
pub fn i_0260dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260E2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DA9C;
    8
}
// ===== HUD_DrawContinueDigits_0260e2  ($0260E2, 70 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0260E2: move.w #$70e5, d2
pub fn i_0260e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260E6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x70E5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0260E6: bra.w $260ee
pub fn i_0260e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x260EE;
    10
}
/// $0260EA: move.w #$7465, d2
pub fn i_0260ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260EE;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7465;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0260EE: move.b $77(a6), d0
pub fn i_0260ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260F2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0260F2: andi.w #$ff, d0
pub fn i_0260f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260F6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0260F6: beq.w $260fc
pub fn i_0260f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x260FC; return 10; }
    8
}
/// $0260FA: subq.w #$1, d0
pub fn i_0260fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260FC;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0260FC: move.w d2, -(a7)
pub fn i_0260fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260FE;
    cpu.ir = 0x3F02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0260FE: jsr $47656.l
pub fn i_0260fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26104;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26104);
    cpu.pc = 0x47656;
    16
}
/// $026104: move.w (a7)+, d2
pub fn i_026104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26106;
    cpu.ir = 0x341F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $026106: tst.w d1
pub fn i_026106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26108;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $026108: beq.w $2612e
pub fn i_026108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2610C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2612E; return 10; }
    8
}
/// $02610C: move.w #$5330, d3
pub fn i_02610c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26110;
    cpu.ir = 0x363C;
    let v1: u32 = 0x5330;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026110: add.b d1, d3
pub fn i_026110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26112;
    cpu.ir = 0xD601;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    4
}
/// $026112: movem.w d2-d3, $3c0000.l
pub fn i_026112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2611A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $02611A: addi.w #$20, d2
pub fn i_02611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2611E;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02611E: move.b #$30, d3
pub fn i_02611e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26122;
    cpu.ir = 0x163C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026122: add.b d0, d3
pub fn i_026122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26124;
    cpu.ir = 0xD600;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    4
}
/// $026124: movem.w d2-d3, $3c0000.l
pub fn i_026124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2612C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
// ===== NopCCR_026128  ($026128, 6 B, thunks/ccr_helpers.c)
/// $026128: ori.b #$0, ccr
pub fn i_026128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2612C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x0);
    20
}
/// $02612C: rts
pub fn i_02612c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2612E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_DrawContinueDigit_Low_02612e  ($02612E, 26 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02612E: move.w #$5330, d3
pub fn i_02612e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26132;
    cpu.ir = 0x363C;
    let v1: u32 = 0x5330;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026132: add.b d0, d3
pub fn i_026132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26134;
    cpu.ir = 0xD600;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    4
}
/// $026134: movem.w d2-d3, $3c0000.l
pub fn i_026134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2613C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $02613C: addi.w #$20, d2
pub fn i_02613c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26140;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026140: move.b #$20, d3
pub fn i_026140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26144;
    cpu.ir = 0x163C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026144: movem.w d2-d3, $3c0000.l
pub fn i_026144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2614C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
// ===== NopCCR_026148  ($026148, 6 B, thunks/ccr_helpers.c)
/// $026148: ori.b #$0, ccr
pub fn i_026148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2614C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x0);
    20
}
/// $02614C: rts
pub fn i_02614c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2614E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_DrawLifeBar_02614e  ($02614E, 442 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02614E: move.w #$7044, d0
pub fn i_02614e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26152;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7044;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026152: bra.w $2615a
pub fn i_026152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26156;
    cpu.ir = 0x6000;
    cpu.pc = 0x2615A;
    10
}
/// $026156: move.w #$73c4, d0
pub fn i_026156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2615A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x73C4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02615A: move.w d0, -(a7)
pub fn i_02615a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2615C;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $02615C: movea.l $78(a6), a0
pub fn i_02615c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26160;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $026160: jsr $2ac0e.l
pub fn i_026160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26166);
    cpu.pc = 0x2AC0E;
    16
}
/// $026166: scs.b d1
pub fn i_026166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26168;
    cpu.ir = 0x55C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $026168: move.b $92(a6), d2
pub fn i_026168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2616C;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02616C: cmpi.b #$0, d2
pub fn i_02616c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26170;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $026170: beq.w $26180
pub fn i_026170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26174;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26180; return 10; }
    8
}
/// $026174: cmpi.b #$1, d2
pub fn i_026174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26178;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $026178: beq.w $2619c
pub fn i_026178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2617C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2619C; return 10; }
    8
}
/// $02617C: bra.w $261f2
pub fn i_02617c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26180;
    cpu.ir = 0x6000;
    cpu.pc = 0x261F2;
    10
}
/// $026180: tst.b d1
pub fn i_026180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26182;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026182: beq.w $2618c
pub fn i_026182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26186;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2618C; return 10; }
    8
}
/// $026186: move.b #$1, $92(a6)
pub fn i_026186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2618C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02618C: move.w $86(a6), d5
pub fn i_02618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26190;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026190: moveq #$0, d1
pub fn i_026190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26192;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026192: move.l d1, $86(a6)
pub fn i_026192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26196;
    cpu.ir = 0x2D41;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $026196: clr.b d2
pub fn i_026196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26198;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $026198: bra.w $26270
pub fn i_026198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2619C;
    cpu.ir = 0x6000;
    cpu.pc = 0x26270;
    10
}
/// $02619C: tst.b d1
pub fn i_02619c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2619E;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $02619E: bne.w $261ae
pub fn i_02619e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x261AE; return 10; }
    8
}
/// $0261A2: move.b #$1, d5
pub fn i_0261a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261A6;
    cpu.ir = 0x1A3C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0261A6: move.b #$0, $92(a6)
pub fn i_0261a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0261AC: bra.b $26190
pub fn i_0261ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261AE;
    cpu.ir = 0x60E2;
    cpu.pc = 0x26190;
    10
}
/// $0261AE: move.w $1005e6.l, d1
pub fn i_0261ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261B4;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x1005E6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0261B4: sub.w $86(a6), d1
pub fn i_0261b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261B8;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0261B8: move.w d1, d5
pub fn i_0261b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261BA;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0261BA: ext.l d5
pub fn i_0261ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261BC;
    cpu.ir = 0x48C5;
    cpu.d[5] = cpu.d[5] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0261BC: swap d5
pub fn i_0261bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261BE;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0261BE: eor.w d5, d1
pub fn i_0261be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261C0;
    cpu.ir = 0xBB41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0261C0: sub.w d5, d1
pub fn i_0261c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261C2;
    cpu.ir = 0x9245;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0261C2: cmpi.w #$2, d1
pub fn i_0261c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261C6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $0261C6: bcs.w $261ce
pub fn i_0261c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261CA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x261CE; return 10; }
    8
}
/// $0261CA: move.w #$2, d1
pub fn i_0261ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0261CE: eor.w d5, d1
pub fn i_0261ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261D0;
    cpu.ir = 0xBB41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0261D0: sub.w d5, d1
pub fn i_0261d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261D2;
    cpu.ir = 0x9245;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0261D2: move.w d1, d5
pub fn i_0261d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261D4;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0261D4: add.w d1, $86(a6)
pub fn i_0261d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261D8;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0261D8: move.w $86(a6), d1
pub fn i_0261d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261DC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0261DC: tst.w d5
pub fn i_0261dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261DE;
    cpu.ir = 0x4A45;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0261DE: bne.w $261ec
pub fn i_0261de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x261EC; return 10; }
    8
}
/// $0261E2: move.b #$2, $92(a6)
pub fn i_0261e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0261E8: clr.l $8a(a6)
pub fn i_0261e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261EC;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $0261EC: clr.b d2
pub fn i_0261ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261EE;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0261EE: bra.w $26270
pub fn i_0261ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x26270;
    10
}
/// $0261F2: tst.b d1
pub fn i_0261f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261F4;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0261F4: beq.w $26208
pub fn i_0261f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26208; return 10; }
    8
}
/// $0261F8: move.w $1005e6.l, d1
pub fn i_0261f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261FE;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x1005E6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0261FE: cmpi.w #$11, d1
pub fn i_0261fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26202;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x11, Size::Word, false);
    8
}
/// $026202: scs.b d2
pub fn i_026202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26204;
    cpu.ir = 0x55C2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $026204: bra.w $2621a
pub fn i_026204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26208;
    cpu.ir = 0x6000;
    cpu.pc = 0x2621A;
    10
}
/// $026208: move.b #$0, $92(a6)
pub fn i_026208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2620E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02620E: clr.l $8a(a6)
pub fn i_02620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26212;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $026212: move.b #$1, d5
pub fn i_026212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26216;
    cpu.ir = 0x1A3C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026216: bra.w $26190
pub fn i_026216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2621A;
    cpu.ir = 0x6000;
    cpu.pc = 0x26190;
    10
}
/// $02621A: move.w d1, d3
pub fn i_02621a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2621C;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02621C: move.w d1, d6
pub fn i_02621c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2621E;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02621E: swap d3
pub fn i_02621e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26220;
    cpu.ir = 0x4843;
    cpu.d[3] = cpu.d[3].rotate_left(16);
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026220: clr.w d3
pub fn i_026220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26222;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026222: sub.l $86(a6), d3
pub fn i_026222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26226;
    cpu.ir = 0x96AE;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Long, true);
    cpu.d[3] = r;
    16
}
/// $026226: neg.l d3
pub fn i_026226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26228;
    cpu.ir = 0x4483;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = r2;
    4
}
/// $026228: move.l $8a(a6), d4
pub fn i_026228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2622C;
    cpu.ir = 0x282E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02622C: move.l d4, d5
pub fn i_02622c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2622E;
    cpu.ir = 0x2A04;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02622E: lsl.l #$1, d3
pub fn i_02622e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26230;
    cpu.ir = 0xE38B;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 1, true);
    cpu.d[3] = r;
    8
}
/// $026230: asr.l #$2, d4
pub fn i_026230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26232;
    cpu.ir = 0xE484;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, false);
    cpu.d[4] = r;
    10
}
/// $026232: add.l d3, d4
pub fn i_026232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26234;
    cpu.ir = 0xD883;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[4] = r;
    4
}
/// $026234: sub.l d4, d5
pub fn i_026234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26236;
    cpu.ir = 0x9A84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $026236: move.l d5, $8a(a6)
pub fn i_026236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2623A;
    cpu.ir = 0x2D45;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02623A: move.l d5, d1
pub fn i_02623a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2623C;
    cpu.ir = 0x2205;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02623C: add.l $86(a6), d1
pub fn i_02623c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26240;
    cpu.ir = 0xD2AE;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Long, true);
    cpu.d[1] = r;
    16
}
/// $026240: move.l d1, $86(a6)
pub fn i_026240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26244;
    cpu.ir = 0x2D41;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $026244: swap d1
pub fn i_026244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26246;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026246: swap d5
pub fn i_026246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26248;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026248: swap d3
pub fn i_026248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2624A;
    cpu.ir = 0x4843;
    cpu.d[3] = cpu.d[3].rotate_left(16);
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02624A: or.w d5, d3
pub fn i_02624a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2624C;
    cpu.ir = 0x8645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFF;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02624C: bne.w $26268
pub fn i_02624c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26250;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26268; return 10; }
    8
}
/// $026250: swap d6
pub fn i_026250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26252;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026252: clr.w d6
pub fn i_026252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26254;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026254: cmp.l $86(a6), d6
pub fn i_026254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26258;
    cpu.ir = 0xBCAE;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $026258: beq.w $26260
pub fn i_026258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2625C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26260; return 10; }
    8
}
/// $02625C: move.w #$1, d5
pub fn i_02625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26260;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026260: move.l d6, $86(a6)
pub fn i_026260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26264;
    cpu.ir = 0x2D46;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $026264: clr.l $8a(a6)
pub fn i_026264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26268;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $026268: tst.w d1
pub fn i_026268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2626A;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02626A: bpl.w $26270
pub fn i_02626a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2626E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x26270; return 10; }
    8
}
/// $02626E: clr.w d1
pub fn i_02626e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26270;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026270: move.w (a7)+, d0
pub fn i_026270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26272;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $026272: tst.b d2
pub fn i_026272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26274;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026274: beq.w $2629a
pub fn i_026274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26278;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2629A; return 10; }
    8
}
/// $026278: move.b $106f28.l, d4
pub fn i_026278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2627E;
    cpu.ir = 0x1839;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02627E: move.w #$da00, d7
pub fn i_02627e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26282;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xDA00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026282: andi.b #$7, d4
pub fn i_026282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26286;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026286: beq.w $262a6
pub fn i_026286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2628A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x262A6; return 10; }
    8
}
/// $02628A: move.w #$7a00, d7
pub fn i_02628a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2628E;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7A00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02628E: cmpi.b #$2, d4
pub fn i_02628e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26292;
    cpu.ir = 0xC04;
    let v1 = cpu.d[4] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $026292: beq.w $262a6
pub fn i_026292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26296;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x262A6; return 10; }
    8
}
/// $026296: bra.w $2629e
pub fn i_026296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2629A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2629E;
    10
}
/// $02629A: move.w #$7a00, d7
pub fn i_02629a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2629E;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7A00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02629E: tst.w d5
pub fn i_02629e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262A0;
    cpu.ir = 0x4A45;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0262A0: bne.w $262a6
pub fn i_0262a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x262A6; return 10; }
    8
}
/// $0262A4: rts
pub fn i_0262a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0262A6: move.b #$a5, d7
pub fn i_0262a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262AA;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xA5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0262AA: movem.w d0/d7, $3c0000.l
pub fn i_0262aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262B2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0262B2: addi.w #$20, d0
pub fn i_0262b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262B6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0262B6: move.w #$6, d2
pub fn i_0262b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262BA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0262BA: move.b #$ae, d7
pub fn i_0262ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262BE;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xAE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0262BE: subq.w #$8, d1
pub fn i_0262be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262C0;
    cpu.ir = 0x5141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0262C0: bmi.w $262d8
pub fn i_0262c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262C4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x262D8; return 10; }
    8
}
/// $0262C4: movem.w d0/d7, $3c0000.l
pub fn i_0262c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262CC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0262CC: addi.w #$20, d0
pub fn i_0262cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262D0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0262D0: subq.w #$1, d2
pub fn i_0262d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262D2;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0262D2: beq.w $26300
pub fn i_0262d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26300; return 10; }
    8
}
/// $0262D6: bra.b $262be
pub fn i_0262d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262D8;
    cpu.ir = 0x60E6;
    cpu.pc = 0x262BE;
    10
}
/// $0262D8: add.b d1, d7
pub fn i_0262d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262DA;
    cpu.ir = 0xDE01;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[7] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    4
}
/// $0262DA: movem.w d0/d7, $3c0000.l
pub fn i_0262da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262E2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0262E2: addi.w #$20, d0
pub fn i_0262e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262E6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0262E6: subq.w #$1, d2
pub fn i_0262e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262E8;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0262E8: beq.w $26300
pub fn i_0262e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26300; return 10; }
    8
}
/// $0262EC: move.b #$a6, d7
pub fn i_0262ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262F0;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xA6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0262F0: movem.w d0/d7, $3c0000.l
pub fn i_0262f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262F8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0262F8: addi.w #$20, d0
pub fn i_0262f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262FC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0262FC: subq.w #$1, d2
pub fn i_0262fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262FE;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0262FE: bne.b $262f0
pub fn i_0262fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26300;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x262F0; return 10; }
    8
}
/// $026300: move.b #$af, d7
pub fn i_026300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26304;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026304: movem.w d0/d7, $3c0000.l
pub fn i_026304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2630C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
// ===== NopCCR_026308  ($026308, 6 B, thunks/ccr_helpers.c)
/// $026308: ori.b #$0, ccr
pub fn i_026308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2630C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x0);
    20
}
/// $02630C: rts
pub fn i_02630c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2630E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_DrawBombGauge_P1_02630e  ($02630E, 100 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02630E: move.w $106f4c.l, d0
pub fn i_02630e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26314;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F4C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $026314: cmp.b $76(a6), d0
pub fn i_026314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26318;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $026318: bne.w $2631e
pub fn i_026318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2631C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2631E; return 10; }
    8
}
/// $02631C: rts
pub fn i_02631c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2631E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02631E: move.b d0, $76(a6)
pub fn i_02631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26322;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026322: move.w #$707c, d1
pub fn i_026322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26326;
    cpu.ir = 0x323C;
    let v1: u32 = 0x707C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026326: move.w #$238a, d2
pub fn i_026326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2632A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x238A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02632A: cmpi.b #$e, d0
pub fn i_02632a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2632E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $02632E: bls.w $26336
pub fn i_02632e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26332;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x26336; return 10; }
    8
}
/// $026332: move.b #$e, d0
pub fn i_026332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26336;
    cpu.ir = 0x103C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026336: move.b #$e, d3
pub fn i_026336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2633A;
    cpu.ir = 0x163C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02633A: sub.b d0, d3
pub fn i_02633a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2633C;
    cpu.ir = 0x9600;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    4
}
/// $02633C: cmpi.b #$0, d0
pub fn i_02633c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26340;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $026340: ble.w $26354
pub fn i_026340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26344;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x26354; return 10; }
    8
}
/// $026344: movem.w d1-d2, $3c0000.l
pub fn i_026344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2634C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $02634C: addi.w #$20, d1
pub fn i_02634c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26350;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026350: subq.b #$1, d0
pub fn i_026350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26352;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $026352: bra.b $2633c
pub fn i_026352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26354;
    cpu.ir = 0x60E8;
    cpu.pc = 0x2633C;
    10
}
/// $026354: move.w #$2320, d2
pub fn i_026354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26358;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026358: cmpi.b #$0, d3
pub fn i_026358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2635C;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02635C: ble.w $26370
pub fn i_02635c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26360;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x26370; return 10; }
    8
}
/// $026360: movem.w d1-d2, $3c0000.l
pub fn i_026360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26368;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $026368: addi.w #$20, d1
pub fn i_026368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2636C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02636C: subq.b #$1, d3
pub fn i_02636c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2636E;
    cpu.ir = 0x5303;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02636E: bra.b $26358
pub fn i_02636e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26370;
    cpu.ir = 0x60E8;
    cpu.pc = 0x26358;
    10
}
/// $026370: rts
pub fn i_026370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26372;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_DrawBombGauge_P2_026372  ($026372, 100 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026372: move.w $106f4e.l, d0
pub fn i_026372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26378;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F4E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $026378: cmp.b $76(a6), d0
pub fn i_026378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2637C;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02637C: bne.w $26382
pub fn i_02637c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26380;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26382; return 10; }
    8
}
/// $026380: rts
pub fn i_026380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26382;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $026382: move.b d0, $76(a6)
pub fn i_026382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26386;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026386: move.w #$72fc, d1
pub fn i_026386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2638A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02638A: move.w #$2320, d2
pub fn i_02638a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2638E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02638E: cmpi.b #$e, d0
pub fn i_02638e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26392;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $026392: bls.w $2639a
pub fn i_026392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26396;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2639A; return 10; }
    8
}
/// $026396: move.b #$e, d0
pub fn i_026396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2639A;
    cpu.ir = 0x103C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02639A: move.b #$e, d3
pub fn i_02639a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2639E;
    cpu.ir = 0x163C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02639E: sub.b d0, d3
pub fn i_02639e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263A0;
    cpu.ir = 0x9600;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    4
}
/// $0263A0: cmpi.b #$0, d3
pub fn i_0263a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263A4;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0263A4: ble.w $263b8
pub fn i_0263a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263A8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x263B8; return 10; }
    8
}
/// $0263A8: movem.w d1-d2, $3c0000.l
pub fn i_0263a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263B0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0263B0: addi.w #$20, d1
pub fn i_0263b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263B4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0263B4: subq.b #$1, d3
pub fn i_0263b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263B6;
    cpu.ir = 0x5303;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0263B6: bra.b $263a0
pub fn i_0263b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263B8;
    cpu.ir = 0x60E8;
    cpu.pc = 0x263A0;
    10
}
/// $0263B8: move.w #$238a, d2
pub fn i_0263b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263BC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x238A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0263BC: cmpi.b #$0, d0
pub fn i_0263bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263C0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0263C0: ble.w $263d4
pub fn i_0263c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263C4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x263D4; return 10; }
    8
}
/// $0263C4: movem.w d1-d2, $3c0000.l
pub fn i_0263c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263CC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0263CC: addi.w #$20, d1
pub fn i_0263cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263D0;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0263D0: subq.b #$1, d0
pub fn i_0263d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263D2;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0263D2: bra.b $263bc
pub fn i_0263d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263D4;
    cpu.ir = 0x60E8;
    cpu.pc = 0x263BC;
    10
}
/// $0263D4: rts
pub fn i_0263d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_DrawScore_0263d6  ($0263D6, 264 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0263D6: jsr $266cc(pc)
pub fn i_0263d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x263DA);
    cpu.pc = 0x266CC;
    16
}
/// $0263DA: bcc.w $263e0
pub fn i_0263da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x263E0; return 10; }
    8
}
/// $0263DE: rts
pub fn i_0263de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0263E0: jsr $264e4(pc)
pub fn i_0263e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x263E4);
    cpu.pc = 0x264E4;
    16
}
/// $0263E4: move.w #$73c3, d0
pub fn i_0263e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263E8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x73C3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0263E8: lea.l $106e9c.l, a1
pub fn i_0263e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263EE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x106E9C;
    20
}
/// $0263EE: bra.w $2640a
pub fn i_0263ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2640A;
    10
}
/// $0263F2: jsr $266cc(pc)
pub fn i_0263f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x263F6);
    cpu.pc = 0x266CC;
    16
}
/// $0263F6: bcc.w $263fc
pub fn i_0263f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x263FC; return 10; }
    8
}
/// $0263FA: rts
pub fn i_0263fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0263FC: jsr $264e4(pc)
pub fn i_0263fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26400;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26400);
    cpu.pc = 0x264E4;
    16
}
/// $026400: move.w #$7063, d0
pub fn i_026400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26404;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7063;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026404: lea.l $106e94.l, a1
pub fn i_026404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2640A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x106E94;
    20
}
/// $02640A: adda.w #$1, a1
pub fn i_02640a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2640E;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x1;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $02640E: move.w #$2300, d1
pub fn i_02640e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26412;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026412: clr.w d2
pub fn i_026412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26414;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026414: move.b (a1)+, d1
pub fn i_026414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26416;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $026416: or.b d1, d2
pub fn i_026416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26418;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026418: bne.w $26424
pub fn i_026418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2641C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26424; return 10; }
    8
}
/// $02641C: move.b #$20, d1
pub fn i_02641c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26420;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026420: bra.w $26428
pub fn i_026420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26424;
    cpu.ir = 0x6000;
    cpu.pc = 0x26428;
    10
}
/// $026424: addi.b #$30, d1
pub fn i_026424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26428;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026428: movem.w d0-d1, $3c0000.l
pub fn i_026428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26430;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $026430: addi.w #$20, d0
pub fn i_026430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26434;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026434: move.b (a1)+, d1
pub fn i_026434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26436;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $026436: or.b d1, d2
pub fn i_026436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26438;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026438: bne.w $26444
pub fn i_026438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2643C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26444; return 10; }
    8
}
/// $02643C: move.b #$20, d1
pub fn i_02643c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26440;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026440: bra.w $26448
pub fn i_026440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26444;
    cpu.ir = 0x6000;
    cpu.pc = 0x26448;
    10
}
/// $026444: addi.b #$30, d1
pub fn i_026444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26448;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026448: movem.w d0-d1, $3c0000.l
pub fn i_026448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26450;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $026450: addi.w #$20, d0
pub fn i_026450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26454;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026454: move.b (a1)+, d1
pub fn i_026454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26456;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $026456: or.b d1, d2
pub fn i_026456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26458;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026458: bne.w $26464
pub fn i_026458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2645C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26464; return 10; }
    8
}
/// $02645C: move.b #$20, d1
pub fn i_02645c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26460;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026460: bra.w $26468
pub fn i_026460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26464;
    cpu.ir = 0x6000;
    cpu.pc = 0x26468;
    10
}
/// $026464: addi.b #$30, d1
pub fn i_026464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26468;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026468: movem.w d0-d1, $3c0000.l
pub fn i_026468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26470;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $026470: addi.w #$20, d0
pub fn i_026470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26474;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026474: move.b (a1)+, d1
pub fn i_026474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26476;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $026476: or.b d1, d2
pub fn i_026476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26478;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026478: bne.w $26484
pub fn i_026478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2647C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26484; return 10; }
    8
}
/// $02647C: move.b #$20, d1
pub fn i_02647c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26480;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026480: bra.w $26488
pub fn i_026480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26484;
    cpu.ir = 0x6000;
    cpu.pc = 0x26488;
    10
}
/// $026484: addi.b #$30, d1
pub fn i_026484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26488;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026488: movem.w d0-d1, $3c0000.l
pub fn i_026488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26490;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $026490: addi.w #$20, d0
pub fn i_026490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26494;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026494: move.b (a1)+, d1
pub fn i_026494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26496;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $026496: or.b d1, d2
pub fn i_026496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26498;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026498: bne.w $264a4
pub fn i_026498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2649C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x264A4; return 10; }
    8
}
/// $02649C: move.b #$20, d1
pub fn i_02649c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264A0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0264A0: bra.w $264a8
pub fn i_0264a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x264A8;
    10
}
/// $0264A4: addi.b #$30, d1
pub fn i_0264a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264A8;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0264A8: movem.w d0-d1, $3c0000.l
pub fn i_0264a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264B0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0264B0: addi.w #$20, d0
pub fn i_0264b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264B4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0264B4: move.b (a1)+, d1
pub fn i_0264b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264B6;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0264B6: or.b d1, d2
pub fn i_0264b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264B8;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0264B8: bne.w $264c4
pub fn i_0264b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x264C4; return 10; }
    8
}
/// $0264BC: move.b #$20, d1
pub fn i_0264bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264C0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0264C0: bra.w $264c8
pub fn i_0264c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x264C8;
    10
}
/// $0264C4: addi.b #$30, d1
pub fn i_0264c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264C8;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0264C8: movem.w d0-d1, $3c0000.l
pub fn i_0264c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264D0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0264D0: addi.w #$20, d0
pub fn i_0264d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264D4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0264D4: move.b (a1), d1
pub fn i_0264d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264D6;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0264D6: addi.b #$30, d1
pub fn i_0264d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264DA;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0264DA: movem.w d0-d1, $3c0000.l
pub fn i_0264da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264E2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
// ===== NopCCR_0264de  ($0264DE, 6 B, thunks/ccr_helpers.c)
/// $0264DE: ori.b #$0, ccr
pub fn i_0264de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264E2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x0);
    20
}
/// $0264E2: rts
pub fn i_0264e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_DrawAmmoAndBombs_0264e4  ($0264E4, 482 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0264E4: movea.l $78(a6), a0
pub fn i_0264e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0264E8: jsr $32fba.l
pub fn i_0264e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x264EE);
    cpu.pc = 0x32FBA;
    16
}
/// $0264EE: tst.w d0
pub fn i_0264ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264F0;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0264F0: bpl.w $264f6
pub fn i_0264f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264F4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x264F6; return 10; }
    8
}
/// $0264F4: clr.w d0
pub fn i_0264f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264F6;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0264F6: cmp.w $8e(a6), d0
pub fn i_0264f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264FA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0264FA: beq.w $26508
pub fn i_0264fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26508; return 10; }
    8
}
/// $0264FE: move.w d0, $8e(a6)
pub fn i_0264fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26502;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026502: move.b #$4, $84(a6)
pub fn i_026502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26508;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026508: move.b $84(a6), d1
pub fn i_026508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2650C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02650C: beq.w $265c2
pub fn i_02650c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26510;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x265C2; return 10; }
    8
}
/// $026510: subq.b #$1, d1
pub fn i_026510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26512;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $026512: move.b d1, $84(a6)
pub fn i_026512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26516;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026516: andi.b #$1, d1
pub fn i_026516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2651A;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02651A: bne.w $26526
pub fn i_02651a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2651E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26526; return 10; }
    8
}
/// $02651E: move.w #$e300, d7
pub fn i_02651e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26522;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xE300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026522: bra.w $2652a
pub fn i_026522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26526;
    cpu.ir = 0x6000;
    cpu.pc = 0x2652A;
    10
}
/// $026526: move.w #$f300, d7
pub fn i_026526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2652A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xF300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02652A: tst.w $70(a6)
pub fn i_02652a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2652E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02652E: bne.w $2653a
pub fn i_02652e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26532;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2653A; return 10; }
    8
}
/// $026532: move.w #$71a4, d6
pub fn i_026532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26536;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x71A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026536: bra.w $2653e
pub fn i_026536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2653A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2653E;
    10
}
/// $02653A: move.w #$7324, d6
pub fn i_02653a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2653E;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7324;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02653E: tst.w d0
pub fn i_02653e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26540;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $026540: bne.w $26570
pub fn i_026540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26544;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26570; return 10; }
    8
}
/// $026544: move.w #$e3f2, d7
pub fn i_026544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26548;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xE3F2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026548: movem.w d6-d7, $3c0000.l
pub fn i_026548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26550;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $026550: subi.w #$20, d6
pub fn i_026550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26554;
    cpu.ir = 0x446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026554: subq.w #$1, d7
pub fn i_026554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26556;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026556: movem.w d6-d7, $3c0000.l
pub fn i_026556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2655E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $02655E: subi.w #$20, d6
pub fn i_02655e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26562;
    cpu.ir = 0x446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026562: subq.w #$1, d7
pub fn i_026562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26564;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026564: movem.w d6-d7, $3c0000.l
pub fn i_026564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2656C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $02656C: bra.w $265c2
pub fn i_02656c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26570;
    cpu.ir = 0x6000;
    cpu.pc = 0x265C2;
    10
}
/// $026570: andi.l #$ffff, d0
pub fn i_026570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26576;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $026576: divu.w #$a, d0
pub fn i_026576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2657A;
    cpu.ir = 0x80FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $02657A: swap d0
pub fn i_02657a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2657C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02657C: addi.b #$30, d0
pub fn i_02657c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26580;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026580: move.b d0, d7
pub fn i_026580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26582;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $026582: movem.w d6-d7, $3c0000.l
pub fn i_026582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2658A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $02658A: subi.w #$20, d6
pub fn i_02658a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2658E;
    cpu.ir = 0x446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02658E: clr.w d0
pub fn i_02658e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26590;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026590: swap d0
pub fn i_026590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26592;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026592: divu.w #$a, d0
pub fn i_026592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26596;
    cpu.ir = 0x80FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $026596: swap d0
pub fn i_026596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26598;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026598: addi.b #$30, d0
pub fn i_026598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2659C;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02659C: move.b d0, d7
pub fn i_02659c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2659E;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02659E: movem.w d6-d7, $3c0000.l
pub fn i_02659e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265A6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0265A6: subi.w #$20, d6
pub fn i_0265a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265AA;
    cpu.ir = 0x446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0265AA: clr.w d0
pub fn i_0265aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265AC;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0265AC: swap d0
pub fn i_0265ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265AE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0265AE: divu.w #$a, d0
pub fn i_0265ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265B2;
    cpu.ir = 0x80FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $0265B2: swap d0
pub fn i_0265b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265B4;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0265B4: addi.b #$30, d0
pub fn i_0265b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265B8;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0265B8: move.b d0, d7
pub fn i_0265b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265BA;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0265BA: movem.w d6-d7, $3c0000.l
pub fn i_0265ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265C2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0265C2: movea.l $78(a6), a0
pub fn i_0265c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265C6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0265C6: jsr $2ac0e.l
pub fn i_0265c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x265CC);
    cpu.pc = 0x2AC0E;
    16
}
/// $0265CC: bcc.w $265dc
pub fn i_0265cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x265DC; return 10; }
    8
}
/// $0265D0: jsr $2abd2.l
pub fn i_0265d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x265D6);
    cpu.pc = 0x2ABD2;
    16
}
/// $0265D6: scc.b d0
pub fn i_0265d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265D8;
    cpu.ir = 0x54C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0265D8: bra.w $265de
pub fn i_0265d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x265DE;
    10
}
/// $0265DC: clr.b d0
pub fn i_0265dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265DE;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0265DE: cmp.b $93(a6), d0
pub fn i_0265de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265E2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0265E2: beq.w $26624
pub fn i_0265e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26624; return 10; }
    8
}
/// $0265E6: move.b d0, $93(a6)
pub fn i_0265e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265EA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0265EA: tst.w $70(a6)
pub fn i_0265ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265EE;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0265EE: bne.w $265fa
pub fn i_0265ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x265FA; return 10; }
    8
}
/// $0265F2: movea.w #$71c3, a1
pub fn i_0265f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265F6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x71C3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $0265F6: bra.w $265fe
pub fn i_0265f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x265FE;
    10
}
/// $0265FA: movea.w #$7343, a1
pub fn i_0265fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265FE;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7343;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $0265FE: move.w #$7ae0, d0
pub fn i_0265fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26602;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7AE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026602: tst.b $93(a6)
pub fn i_026602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26606;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $026606: bne.w $2660c
pub fn i_026606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2660A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2660C; return 10; }
    8
}
/// $02660A: addq.b #$4, d0
pub fn i_02660a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2660C;
    cpu.ir = 0x5800;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02660C: move.w #$4, d1
pub fn i_02660c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26610;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026610: move.w #$1, d2
pub fn i_026610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26614;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026614: jsr $5da56.l
pub fn i_026614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2661A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2661A);
    cpu.pc = 0x5DA56;
    16
}
/// $02661A: move.b #$4, $85(a6)
pub fn i_02661a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26620;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026620: bra.w $2665e
pub fn i_026620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26624;
    cpu.ir = 0x6000;
    cpu.pc = 0x2665E;
    10
}
/// $026624: tst.b $93(a6)
pub fn i_026624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26628;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $026628: bne.w $2663a
pub fn i_026628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2662C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2663A; return 10; }
    8
}
/// $02662C: movea.l $78(a6), a0
pub fn i_02662c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26630;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $026630: jsr $32fce.l
pub fn i_026630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26636);
    cpu.pc = 0x32FCE;
    16
}
/// $026636: bra.w $26640
pub fn i_026636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2663A;
    cpu.ir = 0x6000;
    cpu.pc = 0x26640;
    10
}
/// $02663A: jsr $2a2ac.l
pub fn i_02663a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26640);
    cpu.pc = 0x2A2AC;
    16
}
/// $026640: tst.b d0
pub fn i_026640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26642;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $026642: bpl.w $26648
pub fn i_026642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26646;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x26648; return 10; }
    8
}
/// $026646: clr.b d0
pub fn i_026646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26648;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $026648: andi.w #$ff, d0
pub fn i_026648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2664C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02664C: cmp.w $90(a6), d0
pub fn i_02664c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26650;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $026650: beq.w $2665e
pub fn i_026650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26654;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2665E; return 10; }
    8
}
/// $026654: move.w d0, $90(a6)
pub fn i_026654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26658;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026658: move.b #$4, $85(a6)
pub fn i_026658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2665E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02665E: move.b $85(a6), d0
pub fn i_02665e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26662;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026662: beq.w $266ca
pub fn i_026662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26666;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x266CA; return 10; }
    8
}
/// $026666: subq.b #$1, d0
pub fn i_026666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26668;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $026668: move.b d0, $85(a6)
pub fn i_026668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2666C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02666C: andi.b #$1, d0
pub fn i_02666c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26670;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026670: bne.w $2667c
pub fn i_026670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26674;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2667C; return 10; }
    8
}
/// $026674: move.w #$e300, d7
pub fn i_026674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26678;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xE300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026678: bra.w $26680
pub fn i_026678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2667C;
    cpu.ir = 0x6000;
    cpu.pc = 0x26680;
    10
}
/// $02667C: move.w #$f300, d7
pub fn i_02667c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26680;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xF300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026680: tst.w $70(a6)
pub fn i_026680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26684;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026684: bne.w $26690
pub fn i_026684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26688;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26690; return 10; }
    8
}
/// $026688: move.w #$7204, d6
pub fn i_026688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2668C;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7204;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02668C: bra.w $26694
pub fn i_02668c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26690;
    cpu.ir = 0x6000;
    cpu.pc = 0x26694;
    10
}
/// $026690: move.w #$7384, d6
pub fn i_026690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26694;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7384;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026694: moveq #$0, d0
pub fn i_026694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26696;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026696: move.w $90(a6), d0
pub fn i_026696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2669A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02669A: divu.w #$a, d0
pub fn i_02669a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2669E;
    cpu.ir = 0x80FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $02669E: swap d0
pub fn i_02669e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266A0;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0266A0: addi.b #$30, d0
pub fn i_0266a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266A4;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0266A4: move.b d0, d7
pub fn i_0266a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266A6;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0266A6: movem.w d6-d7, $3c0000.l
pub fn i_0266a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266AE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0266AE: subi.w #$20, d6
pub fn i_0266ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266B2;
    cpu.ir = 0x446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0266B2: clr.w d0
pub fn i_0266b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266B4;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0266B4: swap d0
pub fn i_0266b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266B6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0266B6: divu.w #$a, d0
pub fn i_0266b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266BA;
    cpu.ir = 0x80FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $0266BA: swap d0
pub fn i_0266ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266BC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0266BC: addi.b #$30, d0
pub fn i_0266bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266C0;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0266C0: move.b d0, d7
pub fn i_0266c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266C2;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0266C2: movem.w d6-d7, $3c0000.l
pub fn i_0266c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266CA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
// ===== NopCCR_0266c6  ($0266C6, 6 B, thunks/ccr_helpers.c)
/// $0266C6: ori.b #$0, ccr
pub fn i_0266c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266CA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x0);
    20
}
/// $0266CA: rts
pub fn i_0266ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HUD_IsSceneBCD_0266cc  ($0266CC, 30 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0266CC: move.b $106ece.l, d0
pub fn i_0266cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266D2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0266D2: cmpi.b #$b, d0
pub fn i_0266d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266D6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $0266D6: beq.w $266f0
pub fn i_0266d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x266F0; return 10; }
    8
}
/// $0266DA: cmpi.b #$c, d0
pub fn i_0266da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266DE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0266DE: beq.w $266f0
pub fn i_0266de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x266F0; return 10; }
    8
}
/// $0266E2: cmpi.b #$d, d0
pub fn i_0266e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266E6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $0266E6: beq.w $266f0
pub fn i_0266e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x266F0; return 10; }
    8
}
// ===== ClearXN_0266ea  ($0266EA, 6 B, thunks/ccr_helpers.c)
/// $0266EA: andi.b #$ee, ccr
pub fn i_0266ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0266EE: rts
pub fn i_0266ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0266f0  ($0266F0, 6 B, thunks/ccr_helpers.c)
/// $0266F0: ori.b #$11, ccr
pub fn i_0266f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266F4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0266F4: rts
pub fn i_0266f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_0266f6  ($0266F6, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0266F6: movea.l $8(a6), a1
pub fn i_0266f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266FA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0266FA: move.b $10(a6), d0
pub fn i_0266fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266FE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0266FE: cmp.b $10(a1), d0
pub fn i_0266fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26702;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $026702: bcs.w $2670c
pub fn i_026702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26706;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2670C; return 10; }
    8
}
// ===== ClearXN_026706  ($026706, 6 B, thunks/ccr_helpers.c)
/// $026706: andi.b #$ee, ccr
pub fn i_026706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2670A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02670A: rts
pub fn i_02670a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2670C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_02670c  ($02670C, 6 B, thunks/ccr_helpers.c)
/// $02670C: ori.b #$11, ccr
pub fn i_02670c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26710;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $026710: rts
pub fn i_026710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26712;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_026712  ($026712, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026712: movea.l $8(a6), a1
pub fn i_026712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26716;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $026716: move.b $10(a6), d0
pub fn i_026716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2671A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02671A: cmp.b $10(a1), d0
pub fn i_02671a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2671E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02671E: bcs.w $26728
pub fn i_02671e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26722;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x26728; return 10; }
    8
}
// ===== ClearXN_026722  ($026722, 6 B, thunks/ccr_helpers.c)
/// $026722: andi.b #$ee, ccr
pub fn i_026722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26726;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $026726: rts
pub fn i_026726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26728;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_026728  ($026728, 6 B, thunks/ccr_helpers.c)
/// $026728: ori.b #$11, ccr
pub fn i_026728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2672C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02672C: rts
pub fn i_02672c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2672E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_02672e  ($02672E, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $02672E: movea.l $8(a6), a1
pub fn i_02672e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26732;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $026732: move.b $10(a6), d0
pub fn i_026732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26736;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026736: cmp.b $10(a1), d0
pub fn i_026736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2673A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02673A: bcs.w $26744
pub fn i_02673a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2673E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x26744; return 10; }
    8
}
// ===== ClearXN_02673e  ($02673E, 6 B, thunks/ccr_helpers.c)
/// $02673E: andi.b #$ee, ccr
pub fn i_02673e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26742;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $026742: rts
pub fn i_026742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26744;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_026744  ($026744, 6 B, thunks/ccr_helpers.c)
/// $026744: ori.b #$11, ccr
pub fn i_026744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26748;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $026748: rts
pub fn i_026748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2674A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $026752: tst.b $10fd82.l
pub fn i_026752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26758;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $026758: bne.w $2676c
pub fn i_026758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2675C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2676C; return 10; }
    8
}
/// $02675C: lea.l $2674a(pc), a4
pub fn i_02675c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26760;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x2674A;
    16
}
/// $026760: lsl.w #$2, d1
pub fn i_026760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26762;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $026762: movea.l (a4, d1.w), a4
pub fn i_026762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26766;
    cpu.ir = 0x2874;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    18
}
/// $026766: move.b (a4), d2
pub fn i_026766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26768;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $026768: bra.w $267b4
pub fn i_026768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2676C;
    cpu.ir = 0x6000;
    cpu.pc = 0x267B4;
    10
}
/// $02676C: lea.l $10fdb0.l, a4
pub fn i_02676c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26772;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10FDB0;
    20
}
/// $026772: move.b d2, (a4)
pub fn i_026772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26774;
    cpu.ir = 0x1882;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[4], v1 as u8);
    8
}
/// $026774: move.b d2, $1(a4)
pub fn i_026774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26778;
    cpu.ir = 0x1942;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026778: clr.b $2(a4)
pub fn i_026778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2677C;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0x2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02677C: clr.b $3(a4)
pub fn i_02677c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26780;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0x3);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $026780: move.w d1, -(a7)
pub fn i_026780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26782;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $026782: jsr $c00450.l
pub fn i_026782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26788);
    cpu.pc = 0xC00450;
    16
}
/// $026788: move.w (a7)+, d1
pub fn i_026788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2678A;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $02678A: cmpi.b #$1, $10fd83.l
pub fn i_02678a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26792;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $026792: bne.w $2679e
pub fn i_026792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26796;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2679E; return 10; }
    8
}
/// $026796: move.b (a4, d1.w), d2
pub fn i_026796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2679A;
    cpu.ir = 0x1434;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02679A: bra.w $267b4
pub fn i_02679a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2679E;
    cpu.ir = 0x6000;
    cpu.pc = 0x267B4;
    10
}
/// $02679E: cmpi.b #$2, $10fdaf.l
pub fn i_02679e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267A6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $0267A6: bne.w $267b0
pub fn i_0267a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x267B0; return 10; }
    8
}
/// $0267AA: move.b (a4), d2
pub fn i_0267aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267AC;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0267AC: bra.w $267b4
pub fn i_0267ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x267B4;
    10
}
/// $0267B0: move.b (a4, d1.w), d2
pub fn i_0267b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267B4;
    cpu.ir = 0x1434;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0267B4: tst.b d2
pub fn i_0267b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267B6;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0267B6: beq.w $267c0
pub fn i_0267b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x267C0; return 10; }
    8
}
// ===== SetXN_0267ba  ($0267BA, 6 B, thunks/ccr_helpers.c)
/// $0267BA: ori.b #$11, ccr
pub fn i_0267ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0267BE: rts
pub fn i_0267be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0267c0  ($0267C0, 6 B, thunks/ccr_helpers.c)
/// $0267C0: andi.b #$ee, ccr
pub fn i_0267c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267C4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0267C4: rts
pub fn i_0267c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_0267c6  ($0267C6, 16 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0267C6: movea.l $8(a6), a1
pub fn i_0267c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267CA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0267CA: move.b $10(a6), d0
pub fn i_0267ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267CE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0267CE: cmp.b $10(a1), d0
pub fn i_0267ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267D2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0267D2: bcs.w $267dc
pub fn i_0267d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x267DC; return 10; }
    8
}
// ===== ClearXN_0267d6  ($0267D6, 6 B, thunks/ccr_helpers.c)
/// $0267D6: andi.b #$ee, ccr
pub fn i_0267d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267DA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0267DA: rts
pub fn i_0267da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0267dc  ($0267DC, 6 B, thunks/ccr_helpers.c)
/// $0267DC: ori.b #$11, ccr
pub fn i_0267dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267E0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0267E0: rts
pub fn i_0267e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityResetState  ($0267E2, 18 B, entity/entity_reset_state.c)
/// $0267E2: clr.w $2e(a6)
pub fn i_0267e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267E6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0267E6: clr.w $2c(a6)
pub fn i_0267e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0267EA: clr.l $28(a6)
pub fn i_0267ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267EE;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $0267EE: clr.w $26(a6)
pub fn i_0267ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267F2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0267F2: rts
pub fn i_0267f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClampVelocity_Default_0267f4  ($0267F4, 4 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0267F4: bra.w $267fc
pub fn i_0267f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x267FC;
    10
}
// ===== ClampVelocity_0267f8  ($0267F8, 28 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0267F8: move.w #$800, d1
pub fn i_0267f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267FC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0267FC: cmp.w d0, d1
pub fn i_0267fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267FE;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0267FE: bge.w $26808
pub fn i_0267fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26802;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x26808; return 10; }
    8
}
/// $026802: move.w d1, d0
pub fn i_026802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26804;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026804: bra.w $26812
pub fn i_026804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26808;
    cpu.ir = 0x6000;
    cpu.pc = 0x26812;
    10
}
/// $026808: neg.w d1
pub fn i_026808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2680A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02680A: cmp.w d0, d1
pub fn i_02680a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2680C;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02680C: ble.w $26812
pub fn i_02680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26810;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x26812; return 10; }
    8
}
/// $026810: move.w d1, d0
pub fn i_026810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26812;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026812: rts
pub fn i_026812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26814;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_IntegrateVelocity_026814  ($026814, 28 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026814: move.w $2c(a6), d0
pub fn i_026814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26818;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x024E00, i_024e00),
    (0x024E04, i_024e04),
    (0x024E08, i_024e08),
    (0x024E0C, i_024e0c),
    (0x024E10, i_024e10),
    (0x024E14, i_024e14),
    (0x024E16, i_024e16),
    (0x024E1A, i_024e1a),
    (0x024E1C, i_024e1c),
    (0x024E20, i_024e20),
    (0x024E24, i_024e24),
    (0x024E28, i_024e28),
    (0x024E2C, i_024e2c),
    (0x024E30, i_024e30),
    (0x024E32, i_024e32),
    (0x024E36, i_024e36),
    (0x024E38, i_024e38),
    (0x024E3E, i_024e3e),
    (0x024E46, i_024e46),
    (0x024E4E, i_024e4e),
    (0x024E54, i_024e54),
    (0x024E5A, i_024e5a),
    (0x024E60, i_024e60),
    (0x024E64, i_024e64),
    (0x024E6A, i_024e6a),
    (0x024E6E, i_024e6e),
    (0x024E74, i_024e74),
    (0x024E76, i_024e76),
    (0x024E7C, i_024e7c),
    (0x024E80, i_024e80),
    (0x024E82, i_024e82),
    (0x024E8A, i_024e8a),
    (0x024E92, i_024e92),
    (0x024E98, i_024e98),
    (0x024E9C, i_024e9c),
    (0x024EA0, i_024ea0),
    (0x024EA4, i_024ea4),
    (0x024EA8, i_024ea8),
    (0x024EAC, i_024eac),
    (0x024EB2, i_024eb2),
    (0x024EB6, i_024eb6),
    (0x024EBC, i_024ebc),
    (0x024EC0, i_024ec0),
    (0x024EC6, i_024ec6),
    (0x024ECA, i_024eca),
    (0x024ED0, i_024ed0),
    (0x024ED4, i_024ed4),
    (0x024ED8, i_024ed8),
    (0x024EDC, i_024edc),
    (0x024EE4, i_024ee4),
    (0x024EEA, i_024eea),
    (0x024EEE, i_024eee),
    (0x024EF0, i_024ef0),
    (0x024EF6, i_024ef6),
    (0x024EFA, i_024efa),
    (0x024F00, i_024f00),
    (0x024F02, i_024f02),
    (0x024F08, i_024f08),
    (0x024F0C, i_024f0c),
    (0x024F12, i_024f12),
    (0x024F16, i_024f16),
    (0x024F1C, i_024f1c),
    (0x024F20, i_024f20),
    (0x024F26, i_024f26),
    (0x024F2A, i_024f2a),
    (0x024F30, i_024f30),
    (0x024F34, i_024f34),
    (0x024F3A, i_024f3a),
    (0x024F3E, i_024f3e),
    (0x024F44, i_024f44),
    (0x024F48, i_024f48),
    (0x024F50, i_024f50),
    (0x024F56, i_024f56),
    (0x024F5A, i_024f5a),
    (0x024F5C, i_024f5c),
    (0x024F62, i_024f62),
    (0x024F66, i_024f66),
    (0x024F6C, i_024f6c),
    (0x024F6E, i_024f6e),
    (0x024F74, i_024f74),
    (0x024F78, i_024f78),
    (0x024F7C, i_024f7c),
    (0x024F84, i_024f84),
    (0x024F86, i_024f86),
    (0x024F88, i_024f88),
    (0x024F8A, i_024f8a),
    (0x024F8E, i_024f8e),
    (0x024F92, i_024f92),
    (0x024F94, i_024f94),
    (0x024F98, i_024f98),
    (0x024F9C, i_024f9c),
    (0x024FA0, i_024fa0),
    (0x024FA2, i_024fa2),
    (0x024FA4, i_024fa4),
    (0x024FB6, i_024fb6),
    (0x024FB8, i_024fb8),
    (0x024FBA, i_024fba),
    (0x024FBE, i_024fbe),
    (0x024FC6, i_024fc6),
    (0x024FC8, i_024fc8),
    (0x024FCE, i_024fce),
    (0x024FD0, i_024fd0),
    (0x024FD8, i_024fd8),
    (0x024FDC, i_024fdc),
    (0x024FE0, i_024fe0),
    (0x024FE2, i_024fe2),
    (0x024FE6, i_024fe6),
    (0x024FEA, i_024fea),
    (0x024FEC, i_024fec),
    (0x024FF2, i_024ff2),
    (0x024FF4, i_024ff4),
    (0x024FF6, i_024ff6),
    (0x024FFC, i_024ffc),
    (0x024FFE, i_024ffe),
    (0x025000, i_025000),
    (0x025004, i_025004),
    (0x025008, i_025008),
    (0x025010, i_025010),
    (0x025012, i_025012),
    (0x02501A, i_02501a),
    (0x02501C, i_02501c),
    (0x02501E, i_02501e),
    (0x025024, i_025024),
    (0x02502C, i_02502c),
    (0x02502E, i_02502e),
    (0x025034, i_025034),
    (0x02503A, i_02503a),
    (0x025040, i_025040),
    (0x025046, i_025046),
    (0x02504C, i_02504c),
    (0x025050, i_025050),
    (0x025056, i_025056),
    (0x02505A, i_02505a),
    (0x02505E, i_02505e),
    (0x025060, i_025060),
    (0x025064, i_025064),
    (0x025066, i_025066),
    (0x02506E, i_02506e),
    (0x025072, i_025072),
    (0x025078, i_025078),
    (0x02507E, i_02507e),
    (0x025082, i_025082),
    (0x025086, i_025086),
    (0x02508A, i_02508a),
    (0x02508C, i_02508c),
    (0x02508E, i_02508e),
    (0x025090, i_025090),
    (0x025094, i_025094),
    (0x025098, i_025098),
    (0x02509C, i_02509c),
    (0x0250A4, i_0250a4),
    (0x0250A8, i_0250a8),
    (0x0250AC, i_0250ac),
    (0x0250B0, i_0250b0),
    (0x0250B4, i_0250b4),
    (0x0250BC, i_0250bc),
    (0x0250C0, i_0250c0),
    (0x0250C4, i_0250c4),
    (0x0250CA, i_0250ca),
    (0x0250D0, i_0250d0),
    (0x0250D4, i_0250d4),
    (0x0250D8, i_0250d8),
    (0x0250DC, i_0250dc),
    (0x0250DE, i_0250de),
    (0x0250E0, i_0250e0),
    (0x0250E2, i_0250e2),
    (0x0250E6, i_0250e6),
    (0x0250EA, i_0250ea),
    (0x0250EC, i_0250ec),
    (0x0250EE, i_0250ee),
    (0x0250F0, i_0250f0),
    (0x0250F4, i_0250f4),
    (0x0250F6, i_0250f6),
    (0x0250F8, i_0250f8),
    (0x0250FA, i_0250fa),
    (0x0250FC, i_0250fc),
    (0x0250FE, i_0250fe),
    (0x025100, i_025100),
    (0x025102, i_025102),
    (0x025104, i_025104),
    (0x025106, i_025106),
    (0x025108, i_025108),
    (0x02510C, i_02510c),
    (0x02510E, i_02510e),
    (0x025110, i_025110),
    (0x025112, i_025112),
    (0x025114, i_025114),
    (0x025116, i_025116),
    (0x025118, i_025118),
    (0x02511A, i_02511a),
    (0x02511C, i_02511c),
    (0x025120, i_025120),
    (0x025122, i_025122),
    (0x025126, i_025126),
    (0x02512C, i_02512c),
    (0x025130, i_025130),
    (0x025134, i_025134),
    (0x025138, i_025138),
    (0x02513C, i_02513c),
    (0x025142, i_025142),
    (0x025148, i_025148),
    (0x02514C, i_02514c),
    (0x025150, i_025150),
    (0x025154, i_025154),
    (0x025156, i_025156),
    (0x025158, i_025158),
    (0x02515A, i_02515a),
    (0x02515E, i_02515e),
    (0x025162, i_025162),
    (0x025166, i_025166),
    (0x02516E, i_02516e),
    (0x025172, i_025172),
    (0x025176, i_025176),
    (0x02517A, i_02517a),
    (0x02517E, i_02517e),
    (0x025186, i_025186),
    (0x02518A, i_02518a),
    (0x02518E, i_02518e),
    (0x025194, i_025194),
    (0x02519A, i_02519a),
    (0x02519E, i_02519e),
    (0x0251A2, i_0251a2),
    (0x0251A6, i_0251a6),
    (0x0251A8, i_0251a8),
    (0x0251AA, i_0251aa),
    (0x0251AC, i_0251ac),
    (0x0251B0, i_0251b0),
    (0x0251B4, i_0251b4),
    (0x0251B8, i_0251b8),
    (0x0251C0, i_0251c0),
    (0x0251C4, i_0251c4),
    (0x0251C8, i_0251c8),
    (0x0251CC, i_0251cc),
    (0x0251D0, i_0251d0),
    (0x0251D8, i_0251d8),
    (0x0251DC, i_0251dc),
    (0x0251E0, i_0251e0),
    (0x0251E4, i_0251e4),
    (0x0251E6, i_0251e6),
    (0x0251EC, i_0251ec),
    (0x0251F0, i_0251f0),
    (0x0251F4, i_0251f4),
    (0x0251F8, i_0251f8),
    (0x0251FA, i_0251fa),
    (0x0251FC, i_0251fc),
    (0x0251FE, i_0251fe),
    (0x025202, i_025202),
    (0x025206, i_025206),
    (0x02520A, i_02520a),
    (0x025212, i_025212),
    (0x025216, i_025216),
    (0x02521A, i_02521a),
    (0x02521E, i_02521e),
    (0x025222, i_025222),
    (0x02522A, i_02522a),
    (0x02522E, i_02522e),
    (0x025232, i_025232),
    (0x025234, i_025234),
    (0x02523A, i_02523a),
    (0x02523E, i_02523e),
    (0x025242, i_025242),
    (0x025246, i_025246),
    (0x025248, i_025248),
    (0x02524A, i_02524a),
    (0x02524C, i_02524c),
    (0x025250, i_025250),
    (0x025254, i_025254),
    (0x025258, i_025258),
    (0x025260, i_025260),
    (0x025264, i_025264),
    (0x025268, i_025268),
    (0x02526C, i_02526c),
    (0x025270, i_025270),
    (0x025278, i_025278),
    (0x02527C, i_02527c),
    (0x025280, i_025280),
    (0x025282, i_025282),
    (0x025288, i_025288),
    (0x02528C, i_02528c),
    (0x025292, i_025292),
    (0x025298, i_025298),
    (0x02529C, i_02529c),
    (0x0252A0, i_0252a0),
    (0x0252A6, i_0252a6),
    (0x0252AA, i_0252aa),
    (0x0252AE, i_0252ae),
    (0x0252B0, i_0252b0),
    (0x0252B2, i_0252b2),
    (0x0252B8, i_0252b8),
    (0x0252BE, i_0252be),
    (0x0252C4, i_0252c4),
    (0x0252C6, i_0252c6),
    (0x0252CC, i_0252cc),
    (0x0252D2, i_0252d2),
    (0x0252D8, i_0252d8),
    (0x0252DA, i_0252da),
    (0x0252DC, i_0252dc),
    (0x0252DE, i_0252de),
    (0x0252E2, i_0252e2),
    (0x0252E4, i_0252e4),
    (0x0252E6, i_0252e6),
    (0x0252EA, i_0252ea),
    (0x0252EC, i_0252ec),
    (0x0252EE, i_0252ee),
    (0x0252F2, i_0252f2),
    (0x0252F4, i_0252f4),
    (0x0252F6, i_0252f6),
    (0x0252FA, i_0252fa),
    (0x025300, i_025300),
    (0x025308, i_025308),
    (0x02530C, i_02530c),
    (0x02530E, i_02530e),
    (0x025314, i_025314),
    (0x02531C, i_02531c),
    (0x02531E, i_02531e),
    (0x025324, i_025324),
    (0x025328, i_025328),
    (0x02532C, i_02532c),
    (0x02532E, i_02532e),
    (0x025332, i_025332),
    (0x025336, i_025336),
    (0x02533A, i_02533a),
    (0x025340, i_025340),
    (0x025342, i_025342),
    (0x025348, i_025348),
    (0x02534A, i_02534a),
    (0x02534E, i_02534e),
    (0x025354, i_025354),
    (0x025356, i_025356),
    (0x02535C, i_02535c),
    (0x02535E, i_02535e),
    (0x025364, i_025364),
    (0x025366, i_025366),
    (0x02536C, i_02536c),
    (0x025370, i_025370),
    (0x025374, i_025374),
    (0x02537A, i_02537a),
    (0x02537E, i_02537e),
    (0x025386, i_025386),
    (0x025388, i_025388),
    (0x02538E, i_02538e),
    (0x025390, i_025390),
    (0x025394, i_025394),
    (0x02539A, i_02539a),
    (0x02539E, i_02539e),
    (0x0253A2, i_0253a2),
    (0x0253A4, i_0253a4),
    (0x0253A8, i_0253a8),
    (0x0253AC, i_0253ac),
    (0x0253AE, i_0253ae),
    (0x0253B2, i_0253b2),
    (0x0253BC, i_0253bc),
    (0x0253BE, i_0253be),
    (0x0253C4, i_0253c4),
    (0x0253C8, i_0253c8),
    (0x0253CC, i_0253cc),
    (0x0253D0, i_0253d0),
    (0x0253D4, i_0253d4),
    (0x0253DA, i_0253da),
    (0x0253DE, i_0253de),
    (0x0253E2, i_0253e2),
    (0x0253E4, i_0253e4),
    (0x0253EA, i_0253ea),
    (0x0253EE, i_0253ee),
    (0x0253F6, i_0253f6),
    (0x0253F8, i_0253f8),
    (0x0253FA, i_0253fa),
    (0x0253FE, i_0253fe),
    (0x025402, i_025402),
    (0x025406, i_025406),
    (0x025408, i_025408),
    (0x02540E, i_02540e),
    (0x025414, i_025414),
    (0x02541A, i_02541a),
    (0x02541E, i_02541e),
    (0x025420, i_025420),
    (0x025424, i_025424),
    (0x025426, i_025426),
    (0x02542C, i_02542c),
    (0x025430, i_025430),
    (0x025432, i_025432),
    (0x02543A, i_02543a),
    (0x02543E, i_02543e),
    (0x025440, i_025440),
    (0x025444, i_025444),
    (0x025448, i_025448),
    (0x02544C, i_02544c),
    (0x025450, i_025450),
    (0x025454, i_025454),
    (0x025456, i_025456),
    (0x02545A, i_02545a),
    (0x02545C, i_02545c),
    (0x025460, i_025460),
    (0x025464, i_025464),
    (0x025468, i_025468),
    (0x02546C, i_02546c),
    (0x025470, i_025470),
    (0x025474, i_025474),
    (0x025476, i_025476),
    (0x025488, i_025488),
    (0x02548C, i_02548c),
    (0x025490, i_025490),
    (0x025498, i_025498),
    (0x02549C, i_02549c),
    (0x0254A0, i_0254a0),
    (0x0254A8, i_0254a8),
    (0x0254AC, i_0254ac),
    (0x0254B0, i_0254b0),
    (0x0254B4, i_0254b4),
    (0x0254B6, i_0254b6),
    (0x025588, i_025588),
    (0x02558C, i_02558c),
    (0x025590, i_025590),
    (0x025594, i_025594),
    (0x025596, i_025596),
    (0x0256B0, i_0256b0),
    (0x0256B6, i_0256b6),
    (0x0256BA, i_0256ba),
    (0x0256BE, i_0256be),
    (0x0256C4, i_0256c4),
    (0x0256CC, i_0256cc),
    (0x0256D0, i_0256d0),
    (0x0256D6, i_0256d6),
    (0x0256DC, i_0256dc),
    (0x0256E2, i_0256e2),
    (0x0256E8, i_0256e8),
    (0x0256EC, i_0256ec),
    (0x0256EE, i_0256ee),
    (0x0256F4, i_0256f4),
    (0x0256F8, i_0256f8),
    (0x0256FC, i_0256fc),
    (0x025700, i_025700),
    (0x025702, i_025702),
    (0x025704, i_025704),
    (0x025706, i_025706),
    (0x02570C, i_02570c),
    (0x025710, i_025710),
    (0x025714, i_025714),
    (0x02571A, i_02571a),
    (0x025722, i_025722),
    (0x025726, i_025726),
    (0x02572C, i_02572c),
    (0x025732, i_025732),
    (0x025738, i_025738),
    (0x02573E, i_02573e),
    (0x025742, i_025742),
    (0x025744, i_025744),
    (0x02574A, i_02574a),
    (0x02574E, i_02574e),
    (0x025752, i_025752),
    (0x025756, i_025756),
    (0x025758, i_025758),
    (0x02575A, i_02575a),
    (0x02575C, i_02575c),
    (0x025762, i_025762),
    (0x025766, i_025766),
    (0x02576C, i_02576c),
    (0x025780, i_025780),
    (0x025784, i_025784),
    (0x025786, i_025786),
    (0x02578A, i_02578a),
    (0x02578E, i_02578e),
    (0x025792, i_025792),
    (0x025796, i_025796),
    (0x025798, i_025798),
    (0x0257A0, i_0257a0),
    (0x0257A4, i_0257a4),
    (0x0257A8, i_0257a8),
    (0x0257AC, i_0257ac),
    (0x0257B0, i_0257b0),
    (0x0257B2, i_0257b2),
    (0x0257B6, i_0257b6),
    (0x0257BC, i_0257bc),
    (0x0257C2, i_0257c2),
    (0x0257C4, i_0257c4),
    (0x0257C8, i_0257c8),
    (0x0257CA, i_0257ca),
    (0x0257CC, i_0257cc),
    (0x0257D0, i_0257d0),
    (0x0257D6, i_0257d6),
    (0x0257DA, i_0257da),
    (0x0257DC, i_0257dc),
    (0x0257E0, i_0257e0),
    (0x0257E4, i_0257e4),
    (0x0257EA, i_0257ea),
    (0x0257EC, i_0257ec),
    (0x0257F0, i_0257f0),
    (0x0257F4, i_0257f4),
    (0x0257F8, i_0257f8),
    (0x0257FA, i_0257fa),
    (0x025800, i_025800),
    (0x025804, i_025804),
    (0x025808, i_025808),
    (0x02580A, i_02580a),
    (0x025812, i_025812),
    (0x025818, i_025818),
    (0x02581C, i_02581c),
    (0x025820, i_025820),
    (0x025824, i_025824),
    (0x02582A, i_02582a),
    (0x02582E, i_02582e),
    (0x025832, i_025832),
    (0x025836, i_025836),
    (0x02583A, i_02583a),
    (0x02583E, i_02583e),
    (0x025842, i_025842),
    (0x025846, i_025846),
    (0x02584A, i_02584a),
    (0x02584E, i_02584e),
    (0x025852, i_025852),
    (0x025856, i_025856),
    (0x02585A, i_02585a),
    (0x02585E, i_02585e),
    (0x025860, i_025860),
    (0x025864, i_025864),
    (0x025866, i_025866),
    (0x02586A, i_02586a),
    (0x02586E, i_02586e),
    (0x025874, i_025874),
    (0x025878, i_025878),
    (0x02587C, i_02587c),
    (0x02587E, i_02587e),
    (0x025880, i_025880),
    (0x025882, i_025882),
    (0x025886, i_025886),
    (0x02588A, i_02588a),
    (0x02588E, i_02588e),
    (0x025890, i_025890),
    (0x025896, i_025896),
    (0x02589A, i_02589a),
    (0x02589E, i_02589e),
    (0x0258A0, i_0258a0),
    (0x0258A8, i_0258a8),
    (0x0258AE, i_0258ae),
    (0x0258B2, i_0258b2),
    (0x0258B6, i_0258b6),
    (0x0258BA, i_0258ba),
    (0x0258C0, i_0258c0),
    (0x0258C4, i_0258c4),
    (0x0258C8, i_0258c8),
    (0x0258CC, i_0258cc),
    (0x0258D0, i_0258d0),
    (0x0258D4, i_0258d4),
    (0x0258D8, i_0258d8),
    (0x0258DC, i_0258dc),
    (0x0258E0, i_0258e0),
    (0x0258E4, i_0258e4),
    (0x0258E8, i_0258e8),
    (0x0258EC, i_0258ec),
    (0x0258F0, i_0258f0),
    (0x0258F4, i_0258f4),
    (0x0258F8, i_0258f8),
    (0x0258FE, i_0258fe),
    (0x025902, i_025902),
    (0x025904, i_025904),
    (0x025908, i_025908),
    (0x02590C, i_02590c),
    (0x025910, i_025910),
    (0x025916, i_025916),
    (0x02591A, i_02591a),
    (0x02591E, i_02591e),
    (0x025920, i_025920),
    (0x025952, i_025952),
    (0x025958, i_025958),
    (0x02595E, i_02595e),
    (0x025962, i_025962),
    (0x025964, i_025964),
    (0x025968, i_025968),
    (0x02596A, i_02596a),
    (0x025970, i_025970),
    (0x025974, i_025974),
    (0x02597A, i_02597a),
    (0x02597E, i_02597e),
    (0x025986, i_025986),
    (0x02598A, i_02598a),
    (0x025990, i_025990),
    (0x025994, i_025994),
    (0x025998, i_025998),
    (0x02599C, i_02599c),
    (0x02599E, i_02599e),
    (0x0259A0, i_0259a0),
    (0x0259A4, i_0259a4),
    (0x0259A8, i_0259a8),
    (0x0259AC, i_0259ac),
    (0x0259B0, i_0259b0),
    (0x0259B4, i_0259b4),
    (0x0259B8, i_0259b8),
    (0x0259BC, i_0259bc),
    (0x0259C0, i_0259c0),
    (0x0259C4, i_0259c4),
    (0x0259C8, i_0259c8),
    (0x0259CC, i_0259cc),
    (0x0259D0, i_0259d0),
    (0x0259D4, i_0259d4),
    (0x0259D8, i_0259d8),
    (0x0259DA, i_0259da),
    (0x0259E0, i_0259e0),
    (0x0259E6, i_0259e6),
    (0x0259EC, i_0259ec),
    (0x0259F0, i_0259f0),
    (0x0259F6, i_0259f6),
    (0x0259FA, i_0259fa),
    (0x0259FC, i_0259fc),
    (0x025A02, i_025a02),
    (0x025A06, i_025a06),
    (0x025A0A, i_025a0a),
    (0x025A0C, i_025a0c),
    (0x025A0E, i_025a0e),
    (0x025A14, i_025a14),
    (0x025A18, i_025a18),
    (0x025A1C, i_025a1c),
    (0x025A22, i_025a22),
    (0x025A28, i_025a28),
    (0x025A2E, i_025a2e),
    (0x025A32, i_025a32),
    (0x025A38, i_025a38),
    (0x025A3C, i_025a3c),
    (0x025A40, i_025a40),
    (0x025A44, i_025a44),
    (0x025A46, i_025a46),
    (0x025A4E, i_025a4e),
    (0x025A52, i_025a52),
    (0x025A56, i_025a56),
    (0x025A5C, i_025a5c),
    (0x025A60, i_025a60),
    (0x025A64, i_025a64),
    (0x025A6A, i_025a6a),
    (0x025A6E, i_025a6e),
    (0x025A70, i_025a70),
    (0x025A76, i_025a76),
    (0x025A7A, i_025a7a),
    (0x025A7E, i_025a7e),
    (0x025A82, i_025a82),
    (0x025A88, i_025a88),
    (0x025A8C, i_025a8c),
    (0x025A8E, i_025a8e),
    (0x025A94, i_025a94),
    (0x025A98, i_025a98),
    (0x025A9C, i_025a9c),
    (0x025AA2, i_025aa2),
    (0x025AA8, i_025aa8),
    (0x025AAC, i_025aac),
    (0x025AAE, i_025aae),
    (0x025AB2, i_025ab2),
    (0x025AB6, i_025ab6),
    (0x025ABA, i_025aba),
    (0x025ABE, i_025abe),
    (0x025AC2, i_025ac2),
    (0x025AC6, i_025ac6),
    (0x025AD8, i_025ad8),
    (0x025ADC, i_025adc),
    (0x025AE0, i_025ae0),
    (0x025AE4, i_025ae4),
    (0x025AE8, i_025ae8),
    (0x025AEC, i_025aec),
    (0x025AF2, i_025af2),
    (0x025AF6, i_025af6),
    (0x025AFC, i_025afc),
    (0x025B00, i_025b00),
    (0x025B02, i_025b02),
    (0x025B08, i_025b08),
    (0x025B0C, i_025b0c),
    (0x025B0E, i_025b0e),
    (0x025B12, i_025b12),
    (0x025B16, i_025b16),
    (0x025B18, i_025b18),
    (0x025B1C, i_025b1c),
    (0x025B20, i_025b20),
    (0x025B22, i_025b22),
    (0x025B28, i_025b28),
    (0x025B34, i_025b34),
    (0x025B38, i_025b38),
    (0x025B3C, i_025b3c),
    (0x025B42, i_025b42),
    (0x025B46, i_025b46),
    (0x025B4C, i_025b4c),
    (0x025B50, i_025b50),
    (0x025B54, i_025b54),
    (0x025B58, i_025b58),
    (0x025B5C, i_025b5c),
    (0x025B62, i_025b62),
    (0x025B66, i_025b66),
    (0x025B6C, i_025b6c),
    (0x025B72, i_025b72),
    (0x025B78, i_025b78),
    (0x025B7C, i_025b7c),
    (0x025B80, i_025b80),
    (0x025B84, i_025b84),
    (0x025B88, i_025b88),
    (0x025B8E, i_025b8e),
    (0x025B92, i_025b92),
    (0x025B96, i_025b96),
    (0x025B9A, i_025b9a),
    (0x025B9C, i_025b9c),
    (0x025BA0, i_025ba0),
    (0x025BA6, i_025ba6),
    (0x025BAC, i_025bac),
    (0x025BB2, i_025bb2),
    (0x025BB6, i_025bb6),
    (0x025BBC, i_025bbc),
    (0x025BC2, i_025bc2),
    (0x025BC8, i_025bc8),
    (0x025BCC, i_025bcc),
    (0x025BCE, i_025bce),
    (0x025BD4, i_025bd4),
    (0x025BD8, i_025bd8),
    (0x025BDC, i_025bdc),
    (0x025BE0, i_025be0),
    (0x025BE4, i_025be4),
    (0x025BE6, i_025be6),
    (0x025BEA, i_025bea),
    (0x025BEE, i_025bee),
    (0x025BF2, i_025bf2),
    (0x025BF8, i_025bf8),
    (0x025BFC, i_025bfc),
    (0x025C00, i_025c00),
    (0x025C04, i_025c04),
    (0x025C0A, i_025c0a),
    (0x025C0E, i_025c0e),
    (0x025C12, i_025c12),
    (0x025C14, i_025c14),
    (0x025C18, i_025c18),
    (0x025C1C, i_025c1c),
    (0x025C20, i_025c20),
    (0x025C24, i_025c24),
    (0x025C28, i_025c28),
    (0x025C2C, i_025c2c),
    (0x025C30, i_025c30),
    (0x025C34, i_025c34),
    (0x025C36, i_025c36),
    (0x025C3A, i_025c3a),
    (0x025C3E, i_025c3e),
    (0x025C42, i_025c42),
    (0x025C44, i_025c44),
    (0x025C48, i_025c48),
    (0x025C4C, i_025c4c),
    (0x025C4E, i_025c4e),
    (0x025C52, i_025c52),
    (0x025C58, i_025c58),
    (0x025C5E, i_025c5e),
    (0x025C64, i_025c64),
    (0x025C68, i_025c68),
    (0x025C6E, i_025c6e),
    (0x025C74, i_025c74),
    (0x025C7A, i_025c7a),
    (0x025C7E, i_025c7e),
    (0x025C80, i_025c80),
    (0x025C86, i_025c86),
    (0x025C8A, i_025c8a),
    (0x025C8E, i_025c8e),
    (0x025C92, i_025c92),
    (0x025C96, i_025c96),
    (0x025C9A, i_025c9a),
    (0x025C9E, i_025c9e),
    (0x025CA4, i_025ca4),
    (0x025CA8, i_025ca8),
    (0x025CAA, i_025caa),
    (0x025CAE, i_025cae),
    (0x025CB2, i_025cb2),
    (0x025CB6, i_025cb6),
    (0x025CBA, i_025cba),
    (0x025CBE, i_025cbe),
    (0x025CC2, i_025cc2),
    (0x025CC4, i_025cc4),
    (0x025CCE, i_025cce),
    (0x025CD4, i_025cd4),
    (0x025CD8, i_025cd8),
    (0x025CDC, i_025cdc),
    (0x025CE0, i_025ce0),
    (0x025CE4, i_025ce4),
    (0x025CE8, i_025ce8),
    (0x025CEC, i_025cec),
    (0x025CF0, i_025cf0),
    (0x025CF4, i_025cf4),
    (0x025CF8, i_025cf8),
    (0x025CFC, i_025cfc),
    (0x025CFE, i_025cfe),
    (0x025D02, i_025d02),
    (0x025D08, i_025d08),
    (0x025D0C, i_025d0c),
    (0x025D0E, i_025d0e),
    (0x025D12, i_025d12),
    (0x025D18, i_025d18),
    (0x025D1E, i_025d1e),
    (0x025D22, i_025d22),
    (0x025D26, i_025d26),
    (0x025D2A, i_025d2a),
    (0x025D2E, i_025d2e),
    (0x025D32, i_025d32),
    (0x025D36, i_025d36),
    (0x025D3A, i_025d3a),
    (0x025D3E, i_025d3e),
    (0x025D42, i_025d42),
    (0x025D46, i_025d46),
    (0x025D4A, i_025d4a),
    (0x025D4C, i_025d4c),
    (0x025D50, i_025d50),
    (0x025D54, i_025d54),
    (0x025D58, i_025d58),
    (0x025D5A, i_025d5a),
    (0x025D5C, i_025d5c),
    (0x025D64, i_025d64),
    (0x025D68, i_025d68),
    (0x025D6C, i_025d6c),
    (0x025D72, i_025d72),
    (0x025D78, i_025d78),
    (0x025D7C, i_025d7c),
    (0x025D80, i_025d80),
    (0x025D84, i_025d84),
    (0x025D88, i_025d88),
    (0x025D8A, i_025d8a),
    (0x025D90, i_025d90),
    (0x025D94, i_025d94),
    (0x025D98, i_025d98),
    (0x025D9C, i_025d9c),
    (0x025DA0, i_025da0),
    (0x025DA8, i_025da8),
    (0x025DAC, i_025dac),
    (0x025DB0, i_025db0),
    (0x025DB2, i_025db2),
    (0x025DB6, i_025db6),
    (0x025DBC, i_025dbc),
    (0x025DC2, i_025dc2),
    (0x025DC4, i_025dc4),
    (0x025DC8, i_025dc8),
    (0x025DCC, i_025dcc),
    (0x025DD0, i_025dd0),
    (0x025DD2, i_025dd2),
    (0x025DD6, i_025dd6),
    (0x025DDA, i_025dda),
    (0x025DDC, i_025ddc),
    (0x025DE0, i_025de0),
    (0x025DE2, i_025de2),
    (0x025DE6, i_025de6),
    (0x025DEA, i_025dea),
    (0x025DEC, i_025dec),
    (0x025DF0, i_025df0),
    (0x025DF4, i_025df4),
    (0x025DFA, i_025dfa),
    (0x025DFE, i_025dfe),
    (0x025E00, i_025e00),
    (0x025E04, i_025e04),
    (0x025E08, i_025e08),
    (0x025E0E, i_025e0e),
    (0x025E12, i_025e12),
    (0x025E16, i_025e16),
    (0x025E1A, i_025e1a),
    (0x025E1E, i_025e1e),
    (0x025E22, i_025e22),
    (0x025E28, i_025e28),
    (0x025E2C, i_025e2c),
    (0x025E30, i_025e30),
    (0x025E32, i_025e32),
    (0x025E36, i_025e36),
    (0x025E3C, i_025e3c),
    (0x025E40, i_025e40),
    (0x025E44, i_025e44),
    (0x025E46, i_025e46),
    (0x025E48, i_025e48),
    (0x025E4C, i_025e4c),
    (0x025E50, i_025e50),
    (0x025E54, i_025e54),
    (0x025E56, i_025e56),
    (0x025E5C, i_025e5c),
    (0x025E60, i_025e60),
    (0x025E64, i_025e64),
    (0x025E6A, i_025e6a),
    (0x025E6E, i_025e6e),
    (0x025E72, i_025e72),
    (0x025E74, i_025e74),
    (0x025E7A, i_025e7a),
    (0x025E7E, i_025e7e),
    (0x025E82, i_025e82),
    (0x025E84, i_025e84),
    (0x025E8A, i_025e8a),
    (0x025E8E, i_025e8e),
    (0x025E90, i_025e90),
    (0x025E92, i_025e92),
    (0x025E9A, i_025e9a),
    (0x025E9E, i_025e9e),
    (0x025EA2, i_025ea2),
    (0x025EA4, i_025ea4),
    (0x025EA8, i_025ea8),
    (0x025EAA, i_025eaa),
    (0x025EAC, i_025eac),
    (0x025EB0, i_025eb0),
    (0x025EB2, i_025eb2),
    (0x025EB4, i_025eb4),
    (0x025EB8, i_025eb8),
    (0x025EBA, i_025eba),
    (0x025EBE, i_025ebe),
    (0x025EC2, i_025ec2),
    (0x025EC6, i_025ec6),
    (0x025ECA, i_025eca),
    (0x025ECE, i_025ece),
    (0x025ED0, i_025ed0),
    (0x025ED4, i_025ed4),
    (0x025ED6, i_025ed6),
    (0x025EDA, i_025eda),
    (0x025EE0, i_025ee0),
    (0x025EE6, i_025ee6),
    (0x025EEA, i_025eea),
    (0x025EEE, i_025eee),
    (0x025EF2, i_025ef2),
    (0x025EF6, i_025ef6),
    (0x025EFA, i_025efa),
    (0x025F00, i_025f00),
    (0x025F02, i_025f02),
    (0x025F06, i_025f06),
    (0x025F0A, i_025f0a),
    (0x025F10, i_025f10),
    (0x025F16, i_025f16),
    (0x025F1A, i_025f1a),
    (0x025F1E, i_025f1e),
    (0x025F22, i_025f22),
    (0x025F26, i_025f26),
    (0x025F2A, i_025f2a),
    (0x025F2E, i_025f2e),
    (0x025F34, i_025f34),
    (0x025F38, i_025f38),
    (0x025F3C, i_025f3c),
    (0x025F40, i_025f40),
    (0x025F44, i_025f44),
    (0x025F4A, i_025f4a),
    (0x025F4E, i_025f4e),
    (0x025F52, i_025f52),
    (0x025F58, i_025f58),
    (0x025F5E, i_025f5e),
    (0x025F62, i_025f62),
    (0x025F66, i_025f66),
    (0x025F6A, i_025f6a),
    (0x025F6E, i_025f6e),
    (0x025F72, i_025f72),
    (0x025F76, i_025f76),
    (0x025F7C, i_025f7c),
    (0x025F80, i_025f80),
    (0x025F84, i_025f84),
    (0x025F88, i_025f88),
    (0x025F8C, i_025f8c),
    (0x025F92, i_025f92),
    (0x025F96, i_025f96),
    (0x025F9E, i_025f9e),
    (0x025FA2, i_025fa2),
    (0x025FA6, i_025fa6),
    (0x025FAE, i_025fae),
    (0x025FB2, i_025fb2),
    (0x025FBA, i_025fba),
    (0x025FBE, i_025fbe),
    (0x025FC6, i_025fc6),
    (0x025FCA, i_025fca),
    (0x025FD2, i_025fd2),
    (0x025FD6, i_025fd6),
    (0x025FDE, i_025fde),
    (0x025FE2, i_025fe2),
    (0x025FEA, i_025fea),
    (0x025FEE, i_025fee),
    (0x025FF2, i_025ff2),
    (0x025FF6, i_025ff6),
    (0x025FFA, i_025ffa),
    (0x025FFC, i_025ffc),
    (0x026000, i_026000),
    (0x026004, i_026004),
    (0x02600A, i_02600a),
    (0x026010, i_026010),
    (0x026014, i_026014),
    (0x026018, i_026018),
    (0x02601E, i_02601e),
    (0x026024, i_026024),
    (0x026028, i_026028),
    (0x02602C, i_02602c),
    (0x026032, i_026032),
    (0x026038, i_026038),
    (0x02603C, i_02603c),
    (0x026040, i_026040),
    (0x026046, i_026046),
    (0x02604C, i_02604c),
    (0x026050, i_026050),
    (0x026054, i_026054),
    (0x02605A, i_02605a),
    (0x026060, i_026060),
    (0x026064, i_026064),
    (0x026068, i_026068),
    (0x02606E, i_02606e),
    (0x026074, i_026074),
    (0x026078, i_026078),
    (0x02607C, i_02607c),
    (0x026082, i_026082),
    (0x026088, i_026088),
    (0x02608C, i_02608c),
    (0x026090, i_026090),
    (0x026096, i_026096),
    (0x02609C, i_02609c),
    (0x0260A0, i_0260a0),
    (0x0260A4, i_0260a4),
    (0x0260AA, i_0260aa),
    (0x0260B0, i_0260b0),
    (0x0260B4, i_0260b4),
    (0x0260B8, i_0260b8),
    (0x0260BE, i_0260be),
    (0x0260C4, i_0260c4),
    (0x0260C8, i_0260c8),
    (0x0260CC, i_0260cc),
    (0x0260D0, i_0260d0),
    (0x0260D4, i_0260d4),
    (0x0260D8, i_0260d8),
    (0x0260DC, i_0260dc),
    (0x0260E2, i_0260e2),
    (0x0260E6, i_0260e6),
    (0x0260EA, i_0260ea),
    (0x0260EE, i_0260ee),
    (0x0260F2, i_0260f2),
    (0x0260F6, i_0260f6),
    (0x0260FA, i_0260fa),
    (0x0260FC, i_0260fc),
    (0x0260FE, i_0260fe),
    (0x026104, i_026104),
    (0x026106, i_026106),
    (0x026108, i_026108),
    (0x02610C, i_02610c),
    (0x026110, i_026110),
    (0x026112, i_026112),
    (0x02611A, i_02611a),
    (0x02611E, i_02611e),
    (0x026122, i_026122),
    (0x026124, i_026124),
    (0x026128, i_026128),
    (0x02612C, i_02612c),
    (0x02612E, i_02612e),
    (0x026132, i_026132),
    (0x026134, i_026134),
    (0x02613C, i_02613c),
    (0x026140, i_026140),
    (0x026144, i_026144),
    (0x026148, i_026148),
    (0x02614C, i_02614c),
    (0x02614E, i_02614e),
    (0x026152, i_026152),
    (0x026156, i_026156),
    (0x02615A, i_02615a),
    (0x02615C, i_02615c),
    (0x026160, i_026160),
    (0x026166, i_026166),
    (0x026168, i_026168),
    (0x02616C, i_02616c),
    (0x026170, i_026170),
    (0x026174, i_026174),
    (0x026178, i_026178),
    (0x02617C, i_02617c),
    (0x026180, i_026180),
    (0x026182, i_026182),
    (0x026186, i_026186),
    (0x02618C, i_02618c),
    (0x026190, i_026190),
    (0x026192, i_026192),
    (0x026196, i_026196),
    (0x026198, i_026198),
    (0x02619C, i_02619c),
    (0x02619E, i_02619e),
    (0x0261A2, i_0261a2),
    (0x0261A6, i_0261a6),
    (0x0261AC, i_0261ac),
    (0x0261AE, i_0261ae),
    (0x0261B4, i_0261b4),
    (0x0261B8, i_0261b8),
    (0x0261BA, i_0261ba),
    (0x0261BC, i_0261bc),
    (0x0261BE, i_0261be),
    (0x0261C0, i_0261c0),
    (0x0261C2, i_0261c2),
    (0x0261C6, i_0261c6),
    (0x0261CA, i_0261ca),
    (0x0261CE, i_0261ce),
    (0x0261D0, i_0261d0),
    (0x0261D2, i_0261d2),
    (0x0261D4, i_0261d4),
    (0x0261D8, i_0261d8),
    (0x0261DC, i_0261dc),
    (0x0261DE, i_0261de),
    (0x0261E2, i_0261e2),
    (0x0261E8, i_0261e8),
    (0x0261EC, i_0261ec),
    (0x0261EE, i_0261ee),
    (0x0261F2, i_0261f2),
    (0x0261F4, i_0261f4),
    (0x0261F8, i_0261f8),
    (0x0261FE, i_0261fe),
    (0x026202, i_026202),
    (0x026204, i_026204),
    (0x026208, i_026208),
    (0x02620E, i_02620e),
    (0x026212, i_026212),
    (0x026216, i_026216),
    (0x02621A, i_02621a),
    (0x02621C, i_02621c),
    (0x02621E, i_02621e),
    (0x026220, i_026220),
    (0x026222, i_026222),
    (0x026226, i_026226),
    (0x026228, i_026228),
    (0x02622C, i_02622c),
    (0x02622E, i_02622e),
    (0x026230, i_026230),
    (0x026232, i_026232),
    (0x026234, i_026234),
    (0x026236, i_026236),
    (0x02623A, i_02623a),
    (0x02623C, i_02623c),
    (0x026240, i_026240),
    (0x026244, i_026244),
    (0x026246, i_026246),
    (0x026248, i_026248),
    (0x02624A, i_02624a),
    (0x02624C, i_02624c),
    (0x026250, i_026250),
    (0x026252, i_026252),
    (0x026254, i_026254),
    (0x026258, i_026258),
    (0x02625C, i_02625c),
    (0x026260, i_026260),
    (0x026264, i_026264),
    (0x026268, i_026268),
    (0x02626A, i_02626a),
    (0x02626E, i_02626e),
    (0x026270, i_026270),
    (0x026272, i_026272),
    (0x026274, i_026274),
    (0x026278, i_026278),
    (0x02627E, i_02627e),
    (0x026282, i_026282),
    (0x026286, i_026286),
    (0x02628A, i_02628a),
    (0x02628E, i_02628e),
    (0x026292, i_026292),
    (0x026296, i_026296),
    (0x02629A, i_02629a),
    (0x02629E, i_02629e),
    (0x0262A0, i_0262a0),
    (0x0262A4, i_0262a4),
    (0x0262A6, i_0262a6),
    (0x0262AA, i_0262aa),
    (0x0262B2, i_0262b2),
    (0x0262B6, i_0262b6),
    (0x0262BA, i_0262ba),
    (0x0262BE, i_0262be),
    (0x0262C0, i_0262c0),
    (0x0262C4, i_0262c4),
    (0x0262CC, i_0262cc),
    (0x0262D0, i_0262d0),
    (0x0262D2, i_0262d2),
    (0x0262D6, i_0262d6),
    (0x0262D8, i_0262d8),
    (0x0262DA, i_0262da),
    (0x0262E2, i_0262e2),
    (0x0262E6, i_0262e6),
    (0x0262E8, i_0262e8),
    (0x0262EC, i_0262ec),
    (0x0262F0, i_0262f0),
    (0x0262F8, i_0262f8),
    (0x0262FC, i_0262fc),
    (0x0262FE, i_0262fe),
    (0x026300, i_026300),
    (0x026304, i_026304),
    (0x026308, i_026308),
    (0x02630C, i_02630c),
    (0x02630E, i_02630e),
    (0x026314, i_026314),
    (0x026318, i_026318),
    (0x02631C, i_02631c),
    (0x02631E, i_02631e),
    (0x026322, i_026322),
    (0x026326, i_026326),
    (0x02632A, i_02632a),
    (0x02632E, i_02632e),
    (0x026332, i_026332),
    (0x026336, i_026336),
    (0x02633A, i_02633a),
    (0x02633C, i_02633c),
    (0x026340, i_026340),
    (0x026344, i_026344),
    (0x02634C, i_02634c),
    (0x026350, i_026350),
    (0x026352, i_026352),
    (0x026354, i_026354),
    (0x026358, i_026358),
    (0x02635C, i_02635c),
    (0x026360, i_026360),
    (0x026368, i_026368),
    (0x02636C, i_02636c),
    (0x02636E, i_02636e),
    (0x026370, i_026370),
    (0x026372, i_026372),
    (0x026378, i_026378),
    (0x02637C, i_02637c),
    (0x026380, i_026380),
    (0x026382, i_026382),
    (0x026386, i_026386),
    (0x02638A, i_02638a),
    (0x02638E, i_02638e),
    (0x026392, i_026392),
    (0x026396, i_026396),
    (0x02639A, i_02639a),
    (0x02639E, i_02639e),
    (0x0263A0, i_0263a0),
    (0x0263A4, i_0263a4),
    (0x0263A8, i_0263a8),
    (0x0263B0, i_0263b0),
    (0x0263B4, i_0263b4),
    (0x0263B6, i_0263b6),
    (0x0263B8, i_0263b8),
    (0x0263BC, i_0263bc),
    (0x0263C0, i_0263c0),
    (0x0263C4, i_0263c4),
    (0x0263CC, i_0263cc),
    (0x0263D0, i_0263d0),
    (0x0263D2, i_0263d2),
    (0x0263D4, i_0263d4),
    (0x0263D6, i_0263d6),
    (0x0263DA, i_0263da),
    (0x0263DE, i_0263de),
    (0x0263E0, i_0263e0),
    (0x0263E4, i_0263e4),
    (0x0263E8, i_0263e8),
    (0x0263EE, i_0263ee),
    (0x0263F2, i_0263f2),
    (0x0263F6, i_0263f6),
    (0x0263FA, i_0263fa),
    (0x0263FC, i_0263fc),
    (0x026400, i_026400),
    (0x026404, i_026404),
    (0x02640A, i_02640a),
    (0x02640E, i_02640e),
    (0x026412, i_026412),
    (0x026414, i_026414),
    (0x026416, i_026416),
    (0x026418, i_026418),
    (0x02641C, i_02641c),
    (0x026420, i_026420),
    (0x026424, i_026424),
    (0x026428, i_026428),
    (0x026430, i_026430),
    (0x026434, i_026434),
    (0x026436, i_026436),
    (0x026438, i_026438),
    (0x02643C, i_02643c),
    (0x026440, i_026440),
    (0x026444, i_026444),
    (0x026448, i_026448),
    (0x026450, i_026450),
    (0x026454, i_026454),
    (0x026456, i_026456),
    (0x026458, i_026458),
    (0x02645C, i_02645c),
    (0x026460, i_026460),
    (0x026464, i_026464),
    (0x026468, i_026468),
    (0x026470, i_026470),
    (0x026474, i_026474),
    (0x026476, i_026476),
    (0x026478, i_026478),
    (0x02647C, i_02647c),
    (0x026480, i_026480),
    (0x026484, i_026484),
    (0x026488, i_026488),
    (0x026490, i_026490),
    (0x026494, i_026494),
    (0x026496, i_026496),
    (0x026498, i_026498),
    (0x02649C, i_02649c),
    (0x0264A0, i_0264a0),
    (0x0264A4, i_0264a4),
    (0x0264A8, i_0264a8),
    (0x0264B0, i_0264b0),
    (0x0264B4, i_0264b4),
    (0x0264B6, i_0264b6),
    (0x0264B8, i_0264b8),
    (0x0264BC, i_0264bc),
    (0x0264C0, i_0264c0),
    (0x0264C4, i_0264c4),
    (0x0264C8, i_0264c8),
    (0x0264D0, i_0264d0),
    (0x0264D4, i_0264d4),
    (0x0264D6, i_0264d6),
    (0x0264DA, i_0264da),
    (0x0264DE, i_0264de),
    (0x0264E2, i_0264e2),
    (0x0264E4, i_0264e4),
    (0x0264E8, i_0264e8),
    (0x0264EE, i_0264ee),
    (0x0264F0, i_0264f0),
    (0x0264F4, i_0264f4),
    (0x0264F6, i_0264f6),
    (0x0264FA, i_0264fa),
    (0x0264FE, i_0264fe),
    (0x026502, i_026502),
    (0x026508, i_026508),
    (0x02650C, i_02650c),
    (0x026510, i_026510),
    (0x026512, i_026512),
    (0x026516, i_026516),
    (0x02651A, i_02651a),
    (0x02651E, i_02651e),
    (0x026522, i_026522),
    (0x026526, i_026526),
    (0x02652A, i_02652a),
    (0x02652E, i_02652e),
    (0x026532, i_026532),
    (0x026536, i_026536),
    (0x02653A, i_02653a),
    (0x02653E, i_02653e),
    (0x026540, i_026540),
    (0x026544, i_026544),
    (0x026548, i_026548),
    (0x026550, i_026550),
    (0x026554, i_026554),
    (0x026556, i_026556),
    (0x02655E, i_02655e),
    (0x026562, i_026562),
    (0x026564, i_026564),
    (0x02656C, i_02656c),
    (0x026570, i_026570),
    (0x026576, i_026576),
    (0x02657A, i_02657a),
    (0x02657C, i_02657c),
    (0x026580, i_026580),
    (0x026582, i_026582),
    (0x02658A, i_02658a),
    (0x02658E, i_02658e),
    (0x026590, i_026590),
    (0x026592, i_026592),
    (0x026596, i_026596),
    (0x026598, i_026598),
    (0x02659C, i_02659c),
    (0x02659E, i_02659e),
    (0x0265A6, i_0265a6),
    (0x0265AA, i_0265aa),
    (0x0265AC, i_0265ac),
    (0x0265AE, i_0265ae),
    (0x0265B2, i_0265b2),
    (0x0265B4, i_0265b4),
    (0x0265B8, i_0265b8),
    (0x0265BA, i_0265ba),
    (0x0265C2, i_0265c2),
    (0x0265C6, i_0265c6),
    (0x0265CC, i_0265cc),
    (0x0265D0, i_0265d0),
    (0x0265D6, i_0265d6),
    (0x0265D8, i_0265d8),
    (0x0265DC, i_0265dc),
    (0x0265DE, i_0265de),
    (0x0265E2, i_0265e2),
    (0x0265E6, i_0265e6),
    (0x0265EA, i_0265ea),
    (0x0265EE, i_0265ee),
    (0x0265F2, i_0265f2),
    (0x0265F6, i_0265f6),
    (0x0265FA, i_0265fa),
    (0x0265FE, i_0265fe),
    (0x026602, i_026602),
    (0x026606, i_026606),
    (0x02660A, i_02660a),
    (0x02660C, i_02660c),
    (0x026610, i_026610),
    (0x026614, i_026614),
    (0x02661A, i_02661a),
    (0x026620, i_026620),
    (0x026624, i_026624),
    (0x026628, i_026628),
    (0x02662C, i_02662c),
    (0x026630, i_026630),
    (0x026636, i_026636),
    (0x02663A, i_02663a),
    (0x026640, i_026640),
    (0x026642, i_026642),
    (0x026646, i_026646),
    (0x026648, i_026648),
    (0x02664C, i_02664c),
    (0x026650, i_026650),
    (0x026654, i_026654),
    (0x026658, i_026658),
    (0x02665E, i_02665e),
    (0x026662, i_026662),
    (0x026666, i_026666),
    (0x026668, i_026668),
    (0x02666C, i_02666c),
    (0x026670, i_026670),
    (0x026674, i_026674),
    (0x026678, i_026678),
    (0x02667C, i_02667c),
    (0x026680, i_026680),
    (0x026684, i_026684),
    (0x026688, i_026688),
    (0x02668C, i_02668c),
    (0x026690, i_026690),
    (0x026694, i_026694),
    (0x026696, i_026696),
    (0x02669A, i_02669a),
    (0x02669E, i_02669e),
    (0x0266A0, i_0266a0),
    (0x0266A4, i_0266a4),
    (0x0266A6, i_0266a6),
    (0x0266AE, i_0266ae),
    (0x0266B2, i_0266b2),
    (0x0266B4, i_0266b4),
    (0x0266B6, i_0266b6),
    (0x0266BA, i_0266ba),
    (0x0266BC, i_0266bc),
    (0x0266C0, i_0266c0),
    (0x0266C2, i_0266c2),
    (0x0266C6, i_0266c6),
    (0x0266CA, i_0266ca),
    (0x0266CC, i_0266cc),
    (0x0266D2, i_0266d2),
    (0x0266D6, i_0266d6),
    (0x0266DA, i_0266da),
    (0x0266DE, i_0266de),
    (0x0266E2, i_0266e2),
    (0x0266E6, i_0266e6),
    (0x0266EA, i_0266ea),
    (0x0266EE, i_0266ee),
    (0x0266F0, i_0266f0),
    (0x0266F4, i_0266f4),
    (0x0266F6, i_0266f6),
    (0x0266FA, i_0266fa),
    (0x0266FE, i_0266fe),
    (0x026702, i_026702),
    (0x026706, i_026706),
    (0x02670A, i_02670a),
    (0x02670C, i_02670c),
    (0x026710, i_026710),
    (0x026712, i_026712),
    (0x026716, i_026716),
    (0x02671A, i_02671a),
    (0x02671E, i_02671e),
    (0x026722, i_026722),
    (0x026726, i_026726),
    (0x026728, i_026728),
    (0x02672C, i_02672c),
    (0x02672E, i_02672e),
    (0x026732, i_026732),
    (0x026736, i_026736),
    (0x02673A, i_02673a),
    (0x02673E, i_02673e),
    (0x026742, i_026742),
    (0x026744, i_026744),
    (0x026748, i_026748),
    (0x026752, i_026752),
    (0x026758, i_026758),
    (0x02675C, i_02675c),
    (0x026760, i_026760),
    (0x026762, i_026762),
    (0x026766, i_026766),
    (0x026768, i_026768),
    (0x02676C, i_02676c),
    (0x026772, i_026772),
    (0x026774, i_026774),
    (0x026778, i_026778),
    (0x02677C, i_02677c),
    (0x026780, i_026780),
    (0x026782, i_026782),
    (0x026788, i_026788),
    (0x02678A, i_02678a),
    (0x026792, i_026792),
    (0x026796, i_026796),
    (0x02679A, i_02679a),
    (0x02679E, i_02679e),
    (0x0267A6, i_0267a6),
    (0x0267AA, i_0267aa),
    (0x0267AC, i_0267ac),
    (0x0267B0, i_0267b0),
    (0x0267B4, i_0267b4),
    (0x0267B6, i_0267b6),
    (0x0267BA, i_0267ba),
    (0x0267BE, i_0267be),
    (0x0267C0, i_0267c0),
    (0x0267C4, i_0267c4),
    (0x0267C6, i_0267c6),
    (0x0267CA, i_0267ca),
    (0x0267CE, i_0267ce),
    (0x0267D2, i_0267d2),
    (0x0267D6, i_0267d6),
    (0x0267DA, i_0267da),
    (0x0267DC, i_0267dc),
    (0x0267E0, i_0267e0),
    (0x0267E2, i_0267e2),
    (0x0267E6, i_0267e6),
    (0x0267EA, i_0267ea),
    (0x0267EE, i_0267ee),
    (0x0267F2, i_0267f2),
    (0x0267F4, i_0267f4),
    (0x0267F8, i_0267f8),
    (0x0267FC, i_0267fc),
    (0x0267FE, i_0267fe),
    (0x026802, i_026802),
    (0x026804, i_026804),
    (0x026808, i_026808),
    (0x02680A, i_02680a),
    (0x02680C, i_02680c),
    (0x026810, i_026810),
    (0x026812, i_026812),
    (0x026814, i_026814),
];
