// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C1194E: jsr $c11990(pc)
pub fn i_c1194e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11952;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11952);
    cpu.pc = 0xC11990;
    16
}
/// $C11952: movea.l -$7e58(a4), a2
pub fn i_c11952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11956;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A8);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11956: moveq #$0, d4
pub fn i_c11956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11958;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11958: move.b -$7fa6(a4), d4
pub fn i_c11958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1195C;
    cpu.ir = 0x182C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1195C: addq.l #$1, (a2, d4.w)
pub fn i_c1195c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11960;
    cpu.ir = 0x52B2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C11960: addq.l #$4, a0
pub fn i_c11960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11962;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C11962: rts
pub fn i_c11962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11964;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11990: addq.l #$1, $4(a2)
pub fn i_c11990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11994;
    cpu.ir = 0x52AA;
    let ad1 = cpu.a[2].wrapping_add(0x4);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11994: tst.b d2
pub fn i_c11994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11996;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11996: beq.b $c1199c
pub fn i_c11996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11998;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1199C; return 10; }
    8
}
/// $C11998: addq.l #$1, $8(a2)
pub fn i_c11998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1199C;
    cpu.ir = 0x52AA;
    let ad1 = cpu.a[2].wrapping_add(0x8);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C1199C: rts
pub fn i_c1199c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1199E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1199E: tst.b $be1(a5)
pub fn i_c1199e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119A2;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119A2: bne.w $c119e0
pub fn i_c119a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119A6: cmpi.b #$2, $a81(a5)
pub fn i_c119a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119AC;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $C119AC: bcs.w $c119e0
pub fn i_c119ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119B0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119B0: tst.b $bdb(a5)
pub fn i_c119b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119B4;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBDB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119B4: bne.b $c119e0
pub fn i_c119b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119B6;
    cpu.ir = 0x662A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119B6: tst.b $be2(a5)
pub fn i_c119b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119BA;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBE2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119BA: beq.b $c119e0
pub fn i_c119ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119BC;
    cpu.ir = 0x6724;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119BC: subq.b #$1, $be2(a5)
pub fn i_c119bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119C0;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xBE2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C119C0: tst.b $b8d(a5)
pub fn i_c119c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119C4;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119C4: beq.b $c119d0
pub fn i_c119c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119C6;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119D0; return 10; }
    8
}
/// $C119C6: move.b #$7f, $320000.l
pub fn i_c119c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119CE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C119CE: bra.b $c119e0
pub fn i_c119ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119D0;
    cpu.ir = 0x6010;
    cpu.pc = 0xC119E0;
    10
}
/// $C119D0: jsr $134.w
pub fn i_c119d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119D4;
    cpu.ir = 0x4EB8;
    cpu.push32(bus, 0xC119D4);
    cpu.pc = 0x134;
    16
}
/// $C119D4: lea.l $d08000.l, a4
pub fn i_c119d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119DA;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C119DA: lea.l $10f300.l, a5
pub fn i_c119da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119E0;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C119E0: rts
pub fn i_c119e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C119E2: cmpi.b #$1, $a81(a5)
pub fn i_c119e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119E8;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C119E8: beq.b $c119f2
pub fn i_c119e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119EA;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119F2; return 10; }
    8
}
/// $C119EA: cmpi.b #$1, $aaf(a5)
pub fn i_c119ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119F0;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C119F0: bne.b $c119fe
pub fn i_c119f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119F2;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C119F2: move.b $380000.l, d0
pub fn i_c119f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119F8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C119F8: andi.b #$30, d0
pub fn i_c119f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119FC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C119FC: beq.b $c11a00
pub fn i_c119fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119FE;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11A00; return 10; }
    8
}
/// $C119FE: rts
pub fn i_c119fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11A00: btst.b #$1, $bdd(a5)
pub fn i_c11a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A06;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C11A06: beq.b $c119fe
pub fn i_c11a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A08;
    cpu.ir = 0x67F6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A08: lea.l $d08000.l, a4
pub fn i_c11a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A0E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11A0E: lea.l $800000.l, a6
pub fn i_c11a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A14;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C11A14: lea.l $40(a6), a0
pub fn i_c11a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A18;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x40);
    cpu.a[0] = ad1;
    16
}
/// $C11A18: lea.l $c11b62(pc), a1
pub fn i_c11a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11B62;
    16
}
/// $C11A1C: moveq #$7, d7
pub fn i_c11a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A1E;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11A1E: move.w (a0), d0
pub fn i_c11a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A20;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C11A20: cmp.b (a1)+, d0
pub fn i_c11a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A22;
    cpu.ir = 0xB019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    8
}
/// $C11A22: bne.b $c119fe
pub fn i_c11a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A24;
    cpu.ir = 0x66DA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A24: addq.l #$4, a0
pub fn i_c11a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A26;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C11A26: dbra d7, $c11a1e
pub fn i_c11a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A2A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11A2A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11A2A; return 14; }
    cpu.pc = 0xC11A1E;
    10
}
/// $C11A2A: move.w $10(a6), d0
pub fn i_c11a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C11A2E: andi.b #$f0, d0
pub fn i_c11a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A32;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11A32: cmpi.b #$20, d0
pub fn i_c11a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A36;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $C11A36: bne.b $c119fe
pub fn i_c11a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A38;
    cpu.ir = 0x66C6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A38: move.l -$7e54(a4), d0
pub fn i_c11a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A3C;
    cpu.ir = 0x202C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81AC);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11A3C: cmp.l $70(a6), d0
pub fn i_c11a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A40;
    cpu.ir = 0xB0AE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $C11A40: bne.b $c119fe
pub fn i_c11a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A42;
    cpu.ir = 0x66BC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A42: move.w -$7e50(a4), d0
pub fn i_c11a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A46;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81B0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C11A46: bmi.w $c11a54
pub fn i_c11a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A4A;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC11A54; return 10; }
    8
}
/// $C11A4A: cmp.w $74(a6), d0
pub fn i_c11a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A4E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C11A4E: bhi.b $c119fe
pub fn i_c11a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A50;
    cpu.ir = 0x62AE;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A50: bra.w $c11a94
pub fn i_c11a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A54;
    cpu.ir = 0x6000;
    cpu.pc = 0xC11A94;
    10
}
/// $C11A54: move.w $74(a6), d0
pub fn i_c11a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A58;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C11A58: cmpi.w #$a, d0
pub fn i_c11a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A5C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $C11A5C: bcc.b $c119fe
pub fn i_c11a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A5E;
    cpu.ir = 0x64A0;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A5E: move.w #$2700, sr
pub fn i_c11a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A62;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C11A62: moveq #$0, d7
pub fn i_c11a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A64;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11A64: move.b d7, $3a0019.l
pub fn i_c11a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A6A;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C11A6A: move.b d7, $380011.l
pub fn i_c11a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A70;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C11A70: move.b d7, $3a0005.l
pub fn i_c11a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A76;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C11A76: move.b d7, $3a0017.l
pub fn i_c11a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A7C;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C11A7C: move.b d7, $3a001d.l
pub fn i_c11a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A82;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11A82: move.w d0, -$7e50(a4)
pub fn i_c11a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A86;
    cpu.ir = 0x3940;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81B0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C11A86: move.b d7, $3a000d.l
pub fn i_c11a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A8C;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C11A8C: move.w d0, d1
pub fn i_c11a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A8E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11A8E: addq.w #$1, d1
pub fn i_c11a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A90;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11A90: move.w d1, $74(a6)
pub fn i_c11a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A94;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C11A94: move.w #$2700, sr
pub fn i_c11a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A98;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C11A98: moveq #$0, d7
pub fn i_c11a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A9A;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11A9A: move.b d7, $bdd(a5)
pub fn i_c11a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A9E;
    cpu.ir = 0x1B47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11A9E: move.b d7, $3a0019.l
pub fn i_c11a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AA4;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C11AA4: move.b d7, $380011.l
pub fn i_c11aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AAA;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C11AAA: move.b d7, $3a0005.l
pub fn i_c11aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AB0;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C11AB0: move.b d7, $3a0017.l
pub fn i_c11ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AB6;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C11AB6: move.w #$2800, d1
pub fn i_c11ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ABA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11ABA: mulu.w d0, d1
pub fn i_c11aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ABC;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C11ABC: lea.l $100(a6), a0
pub fn i_c11abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AC0;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x100);
    cpu.a[0] = ad1;
    16
}
/// $C11AC0: adda.l d1, a0
pub fn i_c11ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AC2;
    cpu.ir = 0xD1C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C11AC2: lea.l $300001.l, a3
pub fn i_c11ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AC8;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x300001;
    20
}
/// $C11AC8: movem.l d0/a0-a6, -(a7)
pub fn i_c11ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ACC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C11ACC: move.b d0, (a3)
pub fn i_c11acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ACE;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C11ACE: jsr $c11c42(pc)
pub fn i_c11ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11AD2);
    cpu.pc = 0xC11C42;
    16
}
/// $C11AD2: movem.l (a7)+, d0/a0-a6
pub fn i_c11ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AD6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C11AD6: move.l $ad2(a5), (a0)+
pub fn i_c11ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ADA;
    cpu.ir = 0x20ED;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C11ADA: move.l $ad6(a5), (a0)+
pub fn i_c11ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ADE;
    cpu.ir = 0x20ED;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C11ADE: move.l #$25, d7
pub fn i_c11ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AE4;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x25;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11AE4: lea.l -$7ee6(a4), a1
pub fn i_c11ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AE8;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF811A);
    cpu.a[1] = ad1;
    16
}
/// $C11AE8: jsr $c11b58(pc)
pub fn i_c11ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11AEC);
    cpu.pc = 0xC11B58;
    16
}
/// $C11AEC: move.l #$90, d7
pub fn i_c11aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AF2;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x90;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11AF2: lea.l -$7e00(a4), a1
pub fn i_c11af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AF6;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8200);
    cpu.a[1] = ad1;
    16
}
/// $C11AF6: jsr $c11b58(pc)
pub fn i_c11af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11AFA);
    cpu.pc = 0xC11B58;
    16
}
/// $C11AFA: move.l #$10f0, d7
pub fn i_c11afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B00;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x10F0;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11B00: lea.l $320(a4), a1
pub fn i_c11b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B04;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0x320);
    cpu.a[1] = ad1;
    16
}
/// $C11B04: jsr $c11b58(pc)
pub fn i_c11b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11B08);
    cpu.pc = 0xC11B58;
    16
}
/// $C11B08: move.b d7, $3a0007.l
pub fn i_c11b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B0E;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0007, v1 as u8);
    16
}
/// $C11B0E: move.b d7, $3a0015.l
pub fn i_c11b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B14;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0015, v1 as u8);
    16
}
/// $C11B14: move.l #$eee0000, $400002.l
pub fn i_c11b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B1E;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C11B1E: jsr $c12a0e(pc)
pub fn i_c11b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11B22);
    cpu.pc = 0xC12A0E;
    16
}
/// $C11B22: lea.l $c00(a5), a0
pub fn i_c11b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B26;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C11B26: lea.l $c11b6a(pc), a1
pub fn i_c11b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11B6A;
    16
}
/// $C11B2A: tst.b $a83(a5)
pub fn i_c11b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B2E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C11B2E: beq.b $c11b34
pub fn i_c11b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B30;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11B34; return 10; }
    8
}
/// $C11B30: lea.l $c11b7a(pc), a1
pub fn i_c11b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11B7A;
    16
}
/// $C11B34: move.l a1, (a0)+
pub fn i_c11b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B36;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C11B36: move.l a0, $abe(a5)
pub fn i_c11b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B3A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11B3A: clr.b $abe(a5)
pub fn i_c11b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B3E;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xABE);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11B3E: jsr $c17290(pc)
pub fn i_c11b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11B42);
    cpu.pc = 0xC17290;
    16
}
/// $C11B42: move.l #$f4240, d7
pub fn i_c11b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B48;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0xF4240;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11B48: move.b d0, $300001.l
pub fn i_c11b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B4E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C11B4E: subq.l #$1, d7
pub fn i_c11b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B50;
    cpu.ir = 0x5387;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[7] = r2;
    4
}
/// $C11B50: bne.b $c11b48
pub fn i_c11b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B52;
    cpu.ir = 0x66F6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11B48; return 10; }
    8
}
/// $C11B52: jmp $c102d2.l
pub fn i_c11b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B58;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C11B58: move.b d0, (a3)
pub fn i_c11b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B5A;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C11B5A: move.w (a1)+, (a0)+
pub fn i_c11b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B5C;
    cpu.ir = 0x30D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C11B5C: subq.l #$1, d7
pub fn i_c11b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B5E;
    cpu.ir = 0x5387;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[7] = r2;
    4
}
/// $C11B5E: bne.b $c11b58
pub fn i_c11b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B60;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11B58; return 10; }
    8
}
/// $C11B60: rts
pub fn i_c11b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11BB0: move.b d0, $3a001d.l
pub fn i_c11bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BB6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11BB6: lea.l $d08000.l, a4
pub fn i_c11bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BBC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11BBC: lea.l $10f300.l, a5
pub fn i_c11bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BC2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C11BC2: cmpi.b #$1, $aba(a5)
pub fn i_c11bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BC8;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xABA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C11BC8: beq.b $c11bd2
pub fn i_c11bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BCA;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11BD2; return 10; }
    8
}
/// $C11BCA: cmpi.b #$1, $abb(a5)
pub fn i_c11bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD0;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xABB);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C11BD0: bne.b $c11be2
pub fn i_c11bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD2;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11BE2; return 10; }
    8
}
/// $C11BD2: subq.b #$1, -$7fb4(a4)
pub fn i_c11bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD6;
    cpu.ir = 0x532C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF804C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11BD6: bpl.b $c11be2
pub fn i_c11bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD8;
    cpu.ir = 0x6A0A;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC11BE2; return 10; }
    8
}
/// $C11BD8: move.b #$3b, -$7fb4(a4)
pub fn i_c11bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BDE;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF804C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11BDE: addq.l #$1, -$7fb8(a4)
pub fn i_c11bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BE2;
    cpu.ir = 0x52AC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8048);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11BE2: move.l $ab6(a5), $aba(a5)
pub fn i_c11be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BE8;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAB6);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xABA);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C11BE8: jsr $c11190(pc)
pub fn i_c11be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11BEC);
    cpu.pc = 0xC11190;
    16
}
/// $C11BEC: rts
pub fn i_c11bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11BEE: lea.l $d08000.l, a4
pub fn i_c11bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BF4;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11BF4: move.b #$1, $bbf(a5)
pub fn i_c11bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BFA;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11BFA: move.b d0, $3a001d.l
pub fn i_c11bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C00;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11C00: move.l -$7fb8(a4), d0
pub fn i_c11c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C04;
    cpu.ir = 0x202C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8048);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11C04: movea.l -$7ef6(a4), a2
pub fn i_c11c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C08;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810A);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C08: add.l d0, $c(a2)
pub fn i_c11c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C0C;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C0C: movea.l -$7ef2(a4), a2
pub fn i_c11c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C10;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810E);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C10: add.l d0, $c(a2)
pub fn i_c11c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C14;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C14: movea.l -$7e5c(a4), a2
pub fn i_c11c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C18;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C18: add.l d0, $c(a2)
pub fn i_c11c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C1C;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C1C: movea.l -$7eee(a4), a2
pub fn i_c11c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C20;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8112);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C20: add.l d0, $24c(a2)
pub fn i_c11c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C24;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0x24C);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C24: movea.l -$7eea(a4), a2
pub fn i_c11c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C28;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8116);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C28: add.l d0, $cc(a2)
pub fn i_c11c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C2C;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xCC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C2C: clr.l -$7fb8(a4)
pub fn i_c11c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C30;
    cpu.ir = 0x42AC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8048);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C11C30: move.b #$3b, -$7fb4(a4)
pub fn i_c11c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C36;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF804C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C36: clr.b $bbf(a5)
pub fn i_c11c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C3A;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11C3A: move.b d0, $3a000d.l
pub fn i_c11c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C40;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C11C40: rts
pub fn i_c11c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11C42: moveq #$3, d0
pub fn i_c11c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C44;
    cpu.ir = 0x7003;
    cpu.d[0] = 0x3;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11C44: jsr $c11d3a(pc)
pub fn i_c11c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C48);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11C48: bsr.w $c17066
pub fn i_c11c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C4C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11C4C);
    cpu.pc = 0xC17066;
    18
}
/// $C11C4C: moveq #$1, d0
pub fn i_c11c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C4E;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11C4E: jsr $c11d3a(pc)
pub fn i_c11c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C52);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11C52: bsr.w $c17066
pub fn i_c11c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C56;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11C56);
    cpu.pc = 0xC17066;
    18
}
/// $C11C56: lea.l $10fdd2.l, a0
pub fn i_c11c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C5C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDD2;
    20
}
/// $C11C5C: jsr $c11ca6(pc)
pub fn i_c11c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C60);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C60: move.b d0, $6(a0)
pub fn i_c11c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C64;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C64: jsr $c11ca6(pc)
pub fn i_c11c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C68);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C68: move.b d0, $5(a0)
pub fn i_c11c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C6C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C6C: jsr $c11ca6(pc)
pub fn i_c11c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C70);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C70: move.b d0, $4(a0)
pub fn i_c11c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C74;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C74: jsr $c11ca6(pc)
pub fn i_c11c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C78);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C78: move.b d0, $2(a0)
pub fn i_c11c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C7C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C7C: jsr $c11ca6(pc)
pub fn i_c11c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C80);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C80: move.b d0, d1
pub fn i_c11c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C82;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11C82: andi.b #$f, d1
pub fn i_c11c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C86;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11C86: move.b d1, $3(a0)
pub fn i_c11c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C8A;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C8A: lsr.b #$4, d0
pub fn i_c11c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C8C;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C11C8C: cmpi.b #$a, d0
pub fn i_c11c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C90;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C11C90: bcs.b $c11c94
pub fn i_c11c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C92;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC11C94; return 10; }
    8
}
/// $C11C92: addq.b #$6, d0
pub fn i_c11c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C94;
    cpu.ir = 0x5C00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11C94: move.b d0, $1(a0)
pub fn i_c11c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C98;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C98: jsr $c11ca6(pc)
pub fn i_c11c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C9C);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C9C: move.b d0, (a0)
pub fn i_c11c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C9E;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $C11C9E: moveq #$3, d7
pub fn i_c11c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA0;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11CA0: jsr $c11ca8(pc)
pub fn i_c11ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CA4);
    cpu.pc = 0xC11CA8;
    16
}
/// $C11CA4: rts
pub fn i_c11ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11CA6: moveq #$7, d7
pub fn i_c11ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA8;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11CA8: move.b $320001.l, d2
pub fn i_c11ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CAE;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x320001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C11CAE: jsr $c11d6c(pc)
pub fn i_c11cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CB2);
    cpu.pc = 0xC11D6C;
    16
}
/// $C11CB2: andi.b #$80, d2
pub fn i_c11cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CB6;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11CB6: lsr.b #$1, d0
pub fn i_c11cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CB8;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11CB8: or.b d2, d0
pub fn i_c11cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CBA;
    cpu.ir = 0x8002;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11CBA: dbra d7, $c11ca8
pub fn i_c11cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CBE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11CBE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11CBE; return 14; }
    cpu.pc = 0xC11CA8;
    10
}
/// $C11CBE: rts
pub fn i_c11cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11CC0: lea.l $10f300.l, a5
pub fn i_c11cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CC6;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C11CC6: jsr $c11eca(pc)
pub fn i_c11cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CCA);
    cpu.pc = 0xC11ECA;
    16
}
/// $C11CCA: tst.b d7
pub fn i_c11cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CCC;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11CCC: bne.w $c11d14
pub fn i_c11ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11D14; return 10; }
    8
}
/// $C11CD0: moveq #$1, d0
pub fn i_c11cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CD2;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11CD2: jsr $c11d3a(pc)
pub fn i_c11cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CD6);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11CD6: move.b $ad8(a5), d0
pub fn i_c11cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CDA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CDA: jsr $c11d16(pc)
pub fn i_c11cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CDE);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CDE: move.b $ad7(a5), d0
pub fn i_c11cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CE2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CE2: jsr $c11d16(pc)
pub fn i_c11ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CE6);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CE6: move.b $ad6(a5), d0
pub fn i_c11ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CEA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CEA: jsr $c11d16(pc)
pub fn i_c11cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CEE);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CEE: move.b $ad4(a5), d0
pub fn i_c11cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CF2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CF2: jsr $c11d16(pc)
pub fn i_c11cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CF6);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CF6: move.b $a01(a5), d0
pub fn i_c11cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CFA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA01);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CFA: jsr $c11d16(pc)
pub fn i_c11cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CFE);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CFE: move.b $ad2(a5), d0
pub fn i_c11cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D02;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11D02: jsr $c11d16(pc)
pub fn i_c11d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D06);
    cpu.pc = 0xC11D16;
    16
}
/// $C11D06: moveq #$2, d0
pub fn i_c11d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D08;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D08: jsr $c11d3a(pc)
pub fn i_c11d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D0C);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11D0C: moveq #$0, d0
pub fn i_c11d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D0E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D0E: jsr $c11d3a(pc)
pub fn i_c11d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D12);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11D12: moveq #$0, d7
pub fn i_c11d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D14;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D14: rts
pub fn i_c11d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D16: moveq #$7, d7
pub fn i_c11d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D18;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D18: move.b d0, d1
pub fn i_c11d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D1A;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D1A: andi.b #$1, d1
pub fn i_c11d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D1E;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D1E: jsr $c11d7e(pc)
pub fn i_c11d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D22);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D22: ori.b #$2, d1
pub fn i_c11d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D26;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D26: jsr $c11d7e(pc)
pub fn i_c11d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D2A);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D2A: eori.b #$2, d1
pub fn i_c11d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D2E;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D2E: jsr $c11d7e(pc)
pub fn i_c11d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D32);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D32: lsr.b #$1, d0
pub fn i_c11d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D34;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11D34: dbra d7, $c11d18
pub fn i_c11d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D38;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11D38; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11D38; return 14; }
    cpu.pc = 0xC11D18;
    10
}
/// $C11D38: rts
pub fn i_c11d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D3A: moveq #$3, d7
pub fn i_c11d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D3C;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D3C: move.b d0, d1
pub fn i_c11d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D3E;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D3E: andi.b #$1, d1
pub fn i_c11d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D42;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D42: jsr $c11d7e(pc)
pub fn i_c11d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D46);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D46: ori.b #$2, d1
pub fn i_c11d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D4A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D4A: jsr $c11d7e(pc)
pub fn i_c11d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D4E);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D4E: eori.b #$2, d1
pub fn i_c11d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D52;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D52: jsr $c11d7e(pc)
pub fn i_c11d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D56);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D56: lsr.b #$1, d0
pub fn i_c11d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D58;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11D58: dbra d7, $c11d3c
pub fn i_c11d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D5C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11D5C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11D5C; return 14; }
    cpu.pc = 0xC11D3C;
    10
}
/// $C11D5C: move.b #$4, d1
pub fn i_c11d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D60;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D60: jsr $c11d7e(pc)
pub fn i_c11d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D64);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D64: moveq #$0, d1
pub fn i_c11d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D66;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D66: jsr $c11d7e(pc)
pub fn i_c11d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D6A);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D6A: rts
pub fn i_c11d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D6C: move.b #$2, d1
pub fn i_c11d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D70;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D70: jsr $c11d7e(pc)
pub fn i_c11d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D74);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D74: move.b #$0, d1
pub fn i_c11d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D78;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D78: jsr $c11d7e(pc)
pub fn i_c11d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D7C);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D7C: rts
pub fn i_c11d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D7E: move.b d1, $380051.l
pub fn i_c11d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D84;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380051, v1 as u8);
    16
}
/// $C11D84: rts
pub fn i_c11d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D86: moveq #$3, d0
pub fn i_c11d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D88;
    cpu.ir = 0x7003;
    cpu.d[0] = 0x3;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D88: jsr $c11e7e(pc)
pub fn i_c11d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D8C);
    cpu.pc = 0xC11E7E;
    16
}
/// $C11D8C: bsr.w $c17066
pub fn i_c11d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D90;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11D90);
    cpu.pc = 0xC17066;
    18
}
/// $C11D90: moveq #$1, d0
pub fn i_c11d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D92;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D92: jsr $c11e7e(pc)
pub fn i_c11d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D96);
    cpu.pc = 0xC11E7E;
    16
}
/// $C11D96: bsr.w $c17066
pub fn i_c11d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D9A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11D9A);
    cpu.pc = 0xC17066;
    18
}
/// $C11D9A: lea.l $10fdd2.l, a0
pub fn i_c11d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDD2;
    20
}
/// $C11DA0: jsr $c11dea(pc)
pub fn i_c11da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DA4);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DA4: move.b d0, $6(a0)
pub fn i_c11da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DA8;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DA8: jsr $c11dea(pc)
pub fn i_c11da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DAC);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DAC: move.b d0, $5(a0)
pub fn i_c11dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DB0;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DB0: jsr $c11dea(pc)
pub fn i_c11db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DB4);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DB4: move.b d0, $4(a0)
pub fn i_c11db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DB8;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DB8: jsr $c11dea(pc)
pub fn i_c11db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DBC);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DBC: move.b d0, $2(a0)
pub fn i_c11dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DC0;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DC0: jsr $c11dea(pc)
pub fn i_c11dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DC4);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DC4: move.b d0, d1
pub fn i_c11dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DC6;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11DC6: andi.b #$f, d1
pub fn i_c11dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DCA;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11DCA: move.b d1, $3(a0)
pub fn i_c11dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DCE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DCE: lsr.b #$4, d0
pub fn i_c11dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD0;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C11DD0: cmpi.b #$a, d0
pub fn i_c11dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C11DD4: bcs.b $c11dd8
pub fn i_c11dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD6;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC11DD8; return 10; }
    8
}
/// $C11DD6: addq.b #$6, d0
pub fn i_c11dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD8;
    cpu.ir = 0x5C00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11DD8: move.b d0, $1(a0)
pub fn i_c11dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DDC;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DDC: jsr $c11dea(pc)
pub fn i_c11ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DE0);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DE0: move.b d0, (a0)
pub fn i_c11de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE2;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $C11DE2: moveq #$3, d7
pub fn i_c11de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE4;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11DE4: jsr $c11dec(pc)
pub fn i_c11de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DE8);
    cpu.pc = 0xC11DEC;
    16
}
/// $C11DE8: rts
pub fn i_c11de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11DEA: moveq #$7, d7
pub fn i_c11dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DEC;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11DEC: move.b $220001.l, d2
pub fn i_c11dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DF2;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x220001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C11DF2: jsr $c11eb0(pc)
pub fn i_c11df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DF6);
    cpu.pc = 0xC11EB0;
    16
}
/// $C11DF6: andi.b #$80, d2
pub fn i_c11df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DFA;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11DFA: lsr.b #$1, d0
pub fn i_c11dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DFC;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11DFC: or.b d2, d0
pub fn i_c11dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DFE;
    cpu.ir = 0x8002;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11DFE: dbra d7, $c11dec
pub fn i_c11dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E02;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11E02; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11E02; return 14; }
    cpu.pc = 0xC11DEC;
    10
}
/// $C11E02: rts
pub fn i_c11e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11E7E: moveq #$3, d7
pub fn i_c11e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E80;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11E80: move.b d0, d1
pub fn i_c11e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E82;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11E82: andi.b #$1, d1
pub fn i_c11e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E86;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E86: jsr $c11ec2(pc)
pub fn i_c11e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E8A);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E8A: ori.b #$2, d1
pub fn i_c11e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E8E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E8E: jsr $c11ec2(pc)
pub fn i_c11e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E92);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E92: eori.b #$2, d1
pub fn i_c11e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E96;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E96: jsr $c11ec2(pc)
pub fn i_c11e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E9A);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E9A: lsr.b #$1, d0
pub fn i_c11e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E9C;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11E9C: dbra d7, $c11e80
pub fn i_c11e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EA0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11EA0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11EA0; return 14; }
    cpu.pc = 0xC11E80;
    10
}
/// $C11EA0: move.b #$4, d1
pub fn i_c11ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EA4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EA4: jsr $c11ec2(pc)
pub fn i_c11ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EA8);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EA8: moveq #$0, d1
pub fn i_c11ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EAA;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11EAA: jsr $c11ec2(pc)
pub fn i_c11eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EAE);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EAE: rts
pub fn i_c11eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11EB0: move.b #$2, d1
pub fn i_c11eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EB4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EB4: jsr $c11ec2(pc)
pub fn i_c11eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EB8);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EB8: move.b #$0, d1
pub fn i_c11eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EBC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EBC: jsr $c11ec2(pc)
pub fn i_c11ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EC0);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EC0: rts
pub fn i_c11ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11EC2: move.b d1, $280051.l
pub fn i_c11ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EC8;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x280051, v1 as u8);
    16
}
/// $C11EC8: rts
pub fn i_c11ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ECA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11ECA: lea.l $c11fbe(pc), a1
pub fn i_c11eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ECE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11FBE;
    16
}
/// $C11ECE: move.b $ad2(a5), d0
pub fn i_c11ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ED2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11ED2: jsr $c1217c(pc)
pub fn i_c11ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ED6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11ED6);
    cpu.pc = 0xC1217C;
    16
}
/// $C11ED6: move.b d0, $a00(a5)
pub fn i_c11ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EDA;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11EDA: move.w d0, d1
pub fn i_c11eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EDC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11EDC: move.b $ad3(a5), d0
pub fn i_c11edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11EE0: jsr $c1217c(pc)
pub fn i_c11ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EE4);
    cpu.pc = 0xC1217C;
    16
}
/// $C11EE4: move.w d0, d2
pub fn i_c11ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE6;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11EE6: move.w d2, d3
pub fn i_c11ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE8;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11EE8: lsl.b #$4, d0
pub fn i_c11ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EEA;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C11EEA: move.b d0, $a01(a5)
pub fn i_c11eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EEE;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA01);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11EEE: cmpi.b #$2, d3
pub fn i_c11eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EF2;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C11EF2: bne.b $c11efe
pub fn i_c11ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EF4;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11EFE; return 10; }
    8
}
/// $C11EF4: move.b d1, d0
pub fn i_c11ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EF6;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EF6: andi.b #$3, d0
pub fn i_c11ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EFA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11EFA: bne.b $c11efe
pub fn i_c11efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EFC;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11EFE; return 10; }
    8
}
/// $C11EFC: moveq #$0, d3
pub fn i_c11efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EFE;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11EFE: move.b $ad4(a5), d0
pub fn i_c11efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F02;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11F02: jsr $c1217c(pc)
pub fn i_c11f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11F06);
    cpu.pc = 0xC1217C;
    16
}
/// $C11F06: move.b d0, $a02(a5)
pub fn i_c11f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F0A;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA02);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11F0A: cmp.b (a1, d3.w), d0
pub fn i_c11f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F0E;
    cpu.ir = 0xB031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $C11F0E: bhi.w $c11f56
pub fn i_c11f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F12;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC11F56; return 10; }
    8
}
/// $C11F12: moveq #$0, d6
pub fn i_c11f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F14;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F14: move.l d6, d7
pub fn i_c11f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F16;
    cpu.ir = 0x2E06;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F16: moveq #$1, d5
pub fn i_c11f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F18;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F18: andi.b #$3, d1
pub fn i_c11f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F1C;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F1C: bne.b $c11f20
pub fn i_c11f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F1E;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11F20; return 10; }
    8
}
/// $C11F1E: moveq #$0, d3
pub fn i_c11f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F20;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F20: cmp.b d5, d2
pub fn i_c11f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F22;
    cpu.ir = 0xB405;
    let v1 = cpu.d[5] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C11F22: beq.b $c11f3a
pub fn i_c11f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F24;
    cpu.ir = 0x6716;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11F3A; return 10; }
    8
}
/// $C11F24: move.b (a1, d5.w), d6
pub fn i_c11f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F28;
    cpu.ir = 0x1C31;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11F28: cmpi.b #$2, d5
pub fn i_c11f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F2C;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C11F2C: bne.b $c11f34
pub fn i_c11f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F2E;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11F34; return 10; }
    8
}
/// $C11F2E: tst.b d3
pub fn i_c11f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F30;
    cpu.ir = 0x4A03;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11F30: bne.b $c11f34
pub fn i_c11f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F32;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11F34; return 10; }
    8
}
/// $C11F32: moveq #$1d, d6
pub fn i_c11f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F34;
    cpu.ir = 0x7C1D;
    cpu.d[6] = 0x1D;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F34: add.w d6, d7
pub fn i_c11f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F36;
    cpu.ir = 0xDE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11F36: addq.b #$1, d5
pub fn i_c11f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F38;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11F38: bra.b $c11f20
pub fn i_c11f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F3A;
    cpu.ir = 0x60E6;
    cpu.pc = 0xC11F20;
    10
}
/// $C11F3A: add.w d0, d7
pub fn i_c11f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F3C;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11F3C: subq.w #$1, d7
pub fn i_c11f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F3E;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11F3E: move.b $a00(a5), d0
pub fn i_c11f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F42;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11F42: move.b $c11f5a(pc, d0.w), d0
pub fn i_c11f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F46;
    cpu.ir = 0x103B;
    let ad2 = 0xC11F5A_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11F46: add.w d0, d7
pub fn i_c11f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F48;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11F48: divu.w #$7, d7
pub fn i_c11f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F4C;
    cpu.ir = 0x8EFC;
    let v1: u32 = 0x7;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[7] / dv; let rm = cpu.d[7] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[7] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C11F4C: swap d7
pub fn i_c11f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F4E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F4E: add.b d7, $a01(a5)
pub fn i_c11f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F52;
    cpu.ir = 0xDF2D;
    let ad1 = cpu.a[5].wrapping_add(0xA01);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11F52: moveq #$0, d7
pub fn i_c11f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F54;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F54: rts
pub fn i_c11f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11F56: moveq #$1, d7
pub fn i_c11f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F58;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F58: rts
pub fn i_c11f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11FCC: lea.l $100.w, a0
pub fn i_c11fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x100;
    16
}
/// $C11FD0: lea.l $c11ffe(pc), a1
pub fn i_c11fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11FFE;
    16
}
/// $C11FD4: moveq #$6, d7
pub fn i_c11fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD6;
    cpu.ir = 0x7E06;
    cpu.d[7] = 0x6;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11FD6: cmpm.b (a0)+, (a1)+
pub fn i_c11fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD8;
    cpu.ir = 0xB308;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read8(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C11FD8: bne.b $c12006
pub fn i_c11fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FDA;
    cpu.ir = 0x662C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12006; return 10; }
    8
}
/// $C11FDA: dbra d7, $c11fd6
pub fn i_c11fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FDE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11FDE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11FDE; return 14; }
    cpu.pc = 0xC11FD6;
    10
}
/// $C11FDE: movea.l $182.w, a0
pub fn i_c11fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FE2;
    cpu.ir = 0x2078;
    let v2 = bus.read32(0x182);
    cpu.a[0] = v2;
    16
}
/// $C11FE2: move.w a0, d0
pub fn i_c11fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FE4;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11FE4: btst.b #$0, d0
pub fn i_c11fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FE8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11FE8: bne.b $c11ffc
pub fn i_c11fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FEA;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FFC; return 10; }
    8
}
/// $C11FEA: lea.l $c13674(pc), a1
pub fn i_c11fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC13674;
    16
}
/// $C11FEE: lea.l $c136ee(pc), a2
pub fn i_c11fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF2;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC136EE;
    16
}
/// $C11FF2: cmpm.w (a0)+, (a1)+
pub fn i_c11ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF4;
    cpu.ir = 0xB348;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(2);
    let v1 = bus.read16(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read16(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(2);
    sub_with_flags(cpu, v2, v1, Size::Word, false);
    12
}
/// $C11FF4: bne.b $c11ffc
pub fn i_c11ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF6;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FFC; return 10; }
    8
}
/// $C11FF6: cmpa.l a1, a2
pub fn i_c11ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF8;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $C11FF8: bne.b $c11ff2
pub fn i_c11ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FFA;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FF2; return 10; }
    8
}
/// $C11FFA: moveq #$0, d7
pub fn i_c11ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FFC;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11FFC: rts
pub fn i_c11ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12006: lea.l $100.w, a0
pub fn i_c12006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1200A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x100;
    16
}
/// $C1200A: lea.l $c1206c(pc), a1
pub fn i_c1200a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1200E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1206C;
    16
}
/// $C1200E: moveq #$6, d7
pub fn i_c1200e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12010;
    cpu.ir = 0x7E06;
    cpu.d[7] = 0x6;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12010: cmpm.b (a0)+, (a1)+
pub fn i_c12010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12012;
    cpu.ir = 0xB308;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read8(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C12012: bne.b $c12036
pub fn i_c12012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12014;
    cpu.ir = 0x6622;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12036; return 10; }
    8
}
/// $C12014: dbra d7, $c12010
pub fn i_c12014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12018;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12018; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12018; return 14; }
    cpu.pc = 0xC12010;
    10
}
/// $C12018: moveq #$0, d0
pub fn i_c12018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1201A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1201A: move.b $d00108.l, $d0005e.l
pub fn i_c1201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12024;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0xD00108) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0xD0005E, v2 as u8);
    28
}
/// $C12024: clr.l $d00010.l
pub fn i_c12024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1202A;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0xD00010);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0xD00010, r2);
    20
}
/// $C1202A: jsr $c11d86(pc)
pub fn i_c1202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1202E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1202E);
    cpu.pc = 0xC11D86;
    16
}
/// $C1202E: jsr $c11cc0(pc)
pub fn i_c1202e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12032;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12032);
    cpu.pc = 0xC11CC0;
    16
}
/// $C12032: jmp $c16034(pc)
pub fn i_c12032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12036;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC16034;
    8
}
/// $C12036: lea.l $100.w, a0
pub fn i_c12036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1203A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x100;
    16
}
/// $C1203A: lea.l $c12074(pc), a1
pub fn i_c1203a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1203E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC12074;
    16
}
/// $C1203E: moveq #$6, d7
pub fn i_c1203e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12040;
    cpu.ir = 0x7E06;
    cpu.d[7] = 0x6;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12040: cmpm.b (a0)+, (a1)+
pub fn i_c12040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12042;
    cpu.ir = 0xB308;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read8(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C12042: bne.b $c11ffc
pub fn i_c12042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12044;
    cpu.ir = 0x66B8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FFC; return 10; }
    8
}
/// $C12044: dbra d7, $c12040
pub fn i_c12044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12048;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12048; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12048; return 14; }
    cpu.pc = 0xC12040;
    10
}
/// $C12048: move.w #$1, $d00060.l
pub fn i_c12048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12050;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0xD00060, v1 as u16);
    16
}
/// $C12050: clr.l $d00010.l
pub fn i_c12050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12056;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0xD00010);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0xD00010, r2);
    20
}
/// $C12056: lea.l $d00124.l, a0
pub fn i_c12056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1205C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xD00124;
    20
}
/// $C1205C: moveq #$7, d7
pub fn i_c1205c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1205E;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1205E: move.l #$ffff, (a0)+
pub fn i_c1205e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12064;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C12064: dbra d7, $c1205e
pub fn i_c12064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12068;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12068; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12068; return 14; }
    cpu.pc = 0xC1205E;
    10
}
/// $C12068: moveq #$0, d7
pub fn i_c12068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1206A;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1206A: rts
pub fn i_c1206a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1206C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1207C: move.w d4, -$2(a1)
pub fn i_c1207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12080;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12080: move.w #$5555, d0
pub fn i_c12080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12084;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12084: move.w d5, d7
pub fn i_c12084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12086;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12086: move.w d0, (a1)
pub fn i_c12086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12088;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C12088: move.b d0, (a2)
pub fn i_c12088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1208A;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1208A: dbra d7, $c12086
pub fn i_c1208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1208E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1208E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1208E; return 14; }
    cpu.pc = 0xC12086;
    10
}
/// $C1208E: move.w d4, -$2(a1)
pub fn i_c1208e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12092;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12092: movea.l d4, a0
pub fn i_c12092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12094;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12094: move.w d5, d7
pub fn i_c12094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12096;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12096: move.b d0, (a2)
pub fn i_c12096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12098;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12098: move.w (a1), d1
pub fn i_c12098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1209A;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1209A: move.w d1, (a1)
pub fn i_c1209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1209C;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C1209C: cmp.w d0, d1
pub fn i_c1209c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1209E;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1209E: bne.w $c15aac
pub fn i_c1209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C120A2: addq.l #$1, a0
pub fn i_c120a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120A4;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C120A4: dbra d7, $c12096
pub fn i_c120a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120A8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120A8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120A8; return 14; }
    cpu.pc = 0xC12096;
    10
}
/// $C120A8: move.w d4, -$2(a1)
pub fn i_c120a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120AC;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120AC: move.w #$aaaa, d0
pub fn i_c120ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120B0: move.w d5, d7
pub fn i_c120b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B2;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120B2: move.w d0, (a1)
pub fn i_c120b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B4;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120B4: move.b d0, (a2)
pub fn i_c120b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B6;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120B6: dbra d7, $c120b2
pub fn i_c120b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120BA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120BA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120BA; return 14; }
    cpu.pc = 0xC120B2;
    10
}
/// $C120BA: move.w d4, -$2(a1)
pub fn i_c120ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120BE;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120BE: movea.l d4, a0
pub fn i_c120be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C0;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C120C0: move.w d5, d7
pub fn i_c120c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C2;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120C2: move.b d0, (a2)
pub fn i_c120c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C4;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120C4: move.w (a1), d1
pub fn i_c120c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C6;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C120C6: move.w d1, (a1)
pub fn i_c120c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C8;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120C8: cmp.w d0, d1
pub fn i_c120c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120CA;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C120CA: bne.w $c15aac
pub fn i_c120ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C120CE: addq.l #$1, a0
pub fn i_c120ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120D0;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C120D0: dbra d7, $c120c2
pub fn i_c120d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120D4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120D4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120D4; return 14; }
    cpu.pc = 0xC120C2;
    10
}
/// $C120D4: move.w d4, -$2(a1)
pub fn i_c120d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120D8;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120D8: moveq #$0, d0
pub fn i_c120d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120DA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C120DA: move.w d5, d7
pub fn i_c120da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120DC;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120DC: move.b d0, (a2)
pub fn i_c120dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120DE;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120DE: move.w d0, (a1)
pub fn i_c120de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120E0;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120E0: addq.w #$1, d0
pub fn i_c120e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120E2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C120E2: dbra d7, $c120dc
pub fn i_c120e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120E6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120E6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120E6; return 14; }
    cpu.pc = 0xC120DC;
    10
}
/// $C120E6: move.w d4, -$2(a1)
pub fn i_c120e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120EA;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120EA: movea.l d4, a0
pub fn i_c120ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120EC;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C120EC: moveq #$0, d0
pub fn i_c120ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120EE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C120EE: move.w d5, d7
pub fn i_c120ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F0;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120F0: move.b d0, (a2)
pub fn i_c120f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F2;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120F2: move.w (a1), d1
pub fn i_c120f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F4;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C120F4: move.w d1, (a1)
pub fn i_c120f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F6;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120F6: cmp.w d0, d1
pub fn i_c120f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F8;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C120F8: bne.w $c15aac
pub fn i_c120f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C120FC: addq.l #$1, a0
pub fn i_c120fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120FE;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C120FE: addq.w #$1, d0
pub fn i_c120fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12100;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12100: dbra d7, $c120f0
pub fn i_c12100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12104;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12104; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12104; return 14; }
    cpu.pc = 0xC120F0;
    10
}
/// $C12104: rts
pub fn i_c12104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12106;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12106: lea.l $300001.l, a2
pub fn i_c12106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1210C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x300001;
    20
}
/// $C1210C: movea.l (a7)+, a6
pub fn i_c1210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1210E;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $C1210E: movea.l a0, a1
pub fn i_c1210e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12110;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C12110: move.w #$5555, d0
pub fn i_c12110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12114;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12114: move.w d5, d7
pub fn i_c12114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12116;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12116: move.b d0, (a2)
pub fn i_c12116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12118;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12118: move.w d0, (a0)+
pub fn i_c12118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1211A;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1211A: dbra d7, $c12116
pub fn i_c1211a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1211E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1211E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1211E; return 14; }
    cpu.pc = 0xC12116;
    10
}
/// $C1211E: movea.l a1, a0
pub fn i_c1211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12120;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12120: move.w d5, d7
pub fn i_c12120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12122;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12122: move.b d0, (a2)
pub fn i_c12122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12124;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12124: move.w (a0), d1
pub fn i_c12124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12126;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12126: cmp.w d0, d1
pub fn i_c12126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12128;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C12128: bne.w $c15aac
pub fn i_c12128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1212C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C1212C: addq.l #$2, a0
pub fn i_c1212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1212E;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C1212E: dbra d7, $c12122
pub fn i_c1212e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12132;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12132; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12132; return 14; }
    cpu.pc = 0xC12122;
    10
}
/// $C12132: movea.l a1, a0
pub fn i_c12132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12134;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12134: move.w #$aaaa, d0
pub fn i_c12134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12138;
    cpu.ir = 0x303C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12138: move.w d5, d7
pub fn i_c12138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1213A;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1213A: move.b d0, (a2)
pub fn i_c1213a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1213C;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1213C: move.w d0, (a0)+
pub fn i_c1213c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1213E;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1213E: dbra d7, $c1213a
pub fn i_c1213e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12142;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12142; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12142; return 14; }
    cpu.pc = 0xC1213A;
    10
}
/// $C12142: movea.l a1, a0
pub fn i_c12142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12144;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12144: move.w d5, d7
pub fn i_c12144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12146;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12146: move.b d0, (a2)
pub fn i_c12146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12148;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12148: move.w (a0), d1
pub fn i_c12148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1214A;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1214A: cmp.w d0, d1
pub fn i_c1214a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1214C;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1214C: bne.w $c15aac
pub fn i_c1214c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12150;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C12150: addq.l #$2, a0
pub fn i_c12150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12152;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C12152: dbra d7, $c12146
pub fn i_c12152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12156;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12156; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12156; return 14; }
    cpu.pc = 0xC12146;
    10
}
/// $C12156: movea.l a1, a0
pub fn i_c12156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12158;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12158: move.w d5, d7
pub fn i_c12158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1215A;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1215A: move.b d0, (a2)
pub fn i_c1215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1215C;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1215C: move.w a0, d0
pub fn i_c1215c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1215E;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1215E: move.w d0, (a0)+
pub fn i_c1215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12160;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12160: dbra d7, $c1215a
pub fn i_c12160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12164;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12164; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12164; return 14; }
    cpu.pc = 0xC1215A;
    10
}
/// $C12164: movea.l a1, a0
pub fn i_c12164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12166;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12166: move.w d5, d7
pub fn i_c12166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12168;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12168: move.b d0, (a2)
pub fn i_c12168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1216A;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1216A: move.w a0, d0
pub fn i_c1216a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1216C;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1216C: move.w (a0), d1
pub fn i_c1216c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1216E;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1216E: cmp.w d0, d1
pub fn i_c1216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12170;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C12170: bne.w $c15aac
pub fn i_c12170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12174;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C12174: addq.l #$2, a0
pub fn i_c12174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12176;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C12176: dbra d7, $c12168
pub fn i_c12176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1217A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1217A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1217A; return 14; }
    cpu.pc = 0xC12168;
    10
}
/// $C1217A: jmp (a6)
pub fn i_c1217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1217C;
    cpu.ir = 0x4ED6;
    cpu.jump_to(cpu.a[6]);
    8
}
/// $C1217C: andi.w #$ff, d0
pub fn i_c1217c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12180;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C12180: move.b $c12186(pc, d0.w), d0
pub fn i_c12180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12184;
    cpu.ir = 0x103B;
    let ad2 = 0xC12186_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C12184: rts
pub fn i_c12184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12186;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1231E: lea.l $10f300.l, a5
pub fn i_c1231e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12324;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C12324: pea.l $c1238c(pc)
pub fn i_c12324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12328;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC1238C);
    24
}
/// $C12328: move.l a7, $a00(a5)
pub fn i_c12328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1232C;
    cpu.ir = 0x2B4F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1232C: move.b #$ff, $9f4(a5)
pub fn i_c1232c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12332;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9F4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12332: move.b $380000.l, d0
pub fn i_c12332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12338;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C12338: andi.b #$30, d0
pub fn i_c12338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1233C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1233C: beq.b $c12346
pub fn i_c1233c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1233E;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12346; return 10; }
    8
}
/// $C1233E: move.b #$80, $ac6(a5)
pub fn i_c1233e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12344;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12344: rts
pub fn i_c12344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12346: move.b #$0, $ac6(a5)
pub fn i_c12346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1234C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1234C: lea.l $800000.l, a6
pub fn i_c1234c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12352;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C12352: moveq #$0, d0
pub fn i_c12352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12354;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12354: move.b d0, $3a0019.l
pub fn i_c12354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1235A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C1235A: move.b d0, $380011.l
pub fn i_c1235a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12360;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C12360: move.b $ac4(a5), d0
pub fn i_c12360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12364;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C12364: lsl.w #$2, d0
pub fn i_c12364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12366;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C12366: movea.l $c1236c(pc, d0.w), a0
pub fn i_c12366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1236A;
    cpu.ir = 0x207B;
    let ad2 = 0xC1236C_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C1236A: jmp (a0)
pub fn i_c1236a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1236C;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C1238C: jsr $c13e66(pc)
pub fn i_c1238c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12390;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12390);
    cpu.pc = 0xC13E66;
    16
}
/// $C12390: rts
pub fn i_c12390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12392: lea.l $10f300.l, a5
pub fn i_c12392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12398;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C12398: move.l a7, $a00(a5)
pub fn i_c12398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1239C;
    cpu.ir = 0x2B4F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1239C: moveq #$0, d0
pub fn i_c1239c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1239E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1239E: move.b $ac6(a5), d0
pub fn i_c1239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123A2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C123A2: move.b d0, $ac7(a5)
pub fn i_c123a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123A6;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC7);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C123A6: subi.b #$81, d0
pub fn i_c123a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123AA;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x81, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C123AA: bcs.b $c123b2
pub fn i_c123aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123AC;
    cpu.ir = 0x6506;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC123B2; return 10; }
    8
}
/// $C123AC: cmpi.b #$5, d0
pub fn i_c123ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123B0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $C123B0: bcs.b $c123dc
pub fn i_c123b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123B2;
    cpu.ir = 0x652A;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC123DC; return 10; }
    8
}
/// $C123B2: rts
pub fn i_c123b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C123DC: lsl.w #$3, d0
pub fn i_c123dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123DE;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C123DE: move.b $ac4(a5), d1
pub fn i_c123de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123E2;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C123E2: andi.b #$7, d1
pub fn i_c123e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123E6;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C123E6: or.b d1, d0
pub fn i_c123e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123E8;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C123E8: move.b $c123b4(pc, d0.w), d0
pub fn i_c123e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123EC;
    cpu.ir = 0x103B;
    let ad2 = 0xC123B4_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C123EC: bmi.b $c123b2
pub fn i_c123ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123EE;
    cpu.ir = 0x6BC4;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC123B2; return 10; }
    8
}
/// $C123EE: move.l #$10ff00, $abe(a5)
pub fn i_c123ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123F6;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C123F6: clr.b $ac2(a5)
pub fn i_c123f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123FA;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C123FA: clr.b $bba(a5)
pub fn i_c123fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123FE;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBA);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C123FE: pea.l $c1242c(pc)
pub fn i_c123fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12402;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC1242C);
    24
}
/// $C12402: move.w $a80(a5), $b8e(a5)
pub fn i_c12402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12408;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xA80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xB8E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C12408: bclr.b #$7, $a80(a5)
pub fn i_c12408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1240E;
    cpu.ir = 0x8AD;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[5].wrapping_add(0xA80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C1240E: move.w #$2000, sr
pub fn i_c1240e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12412;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C12412: movea.l $c12418(pc, d0.w), a0
pub fn i_c12412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12416;
    cpu.ir = 0x207B;
    let ad2 = 0xC12418_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C12416: jmp (a0)
pub fn i_c12416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12418;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C12A0E: addq.b #$1, $ac2(a5)
pub fn i_c12a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A12;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12A12: move.l #$10ff00, $abe(a5)
pub fn i_c12a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A1A;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12A1A: lea.l $3c0002.l, a1
pub fn i_c12a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C12A20: move.w #$1, $2(a1)
pub fn i_c12a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A26;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12A26: move.w #$20, d0
pub fn i_c12a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A2A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12A2A: move.w #$7000, -$2(a1)
pub fn i_c12a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A30;
    cpu.ir = 0x337C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12A30: move.w #$4ff, d7
pub fn i_c12a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A34;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x4FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12A34: move.b d0, $300001.l
pub fn i_c12a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A3A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12A3A: move.w d0, (a1)
pub fn i_c12a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A3C;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C12A3C: dbra d7, $c12a34
pub fn i_c12a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A40;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12A40; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12A40; return 14; }
    cpu.pc = 0xC12A34;
    10
}
/// $C12A40: subq.b #$1, $ac2(a5)
pub fn i_c12a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A44;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12A44: rts
pub fn i_c12a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12DCC: jsr $c13978(pc)
pub fn i_c12dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12DD0);
    cpu.pc = 0xC13978;
    16
}
/// $C12DD0: tst.b $ac6(a5)
pub fn i_c12dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DD4;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12DD4: beq.b $c12dd8
pub fn i_c12dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DD6;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12DD8; return 10; }
    8
}
/// $C12DD6: rts
pub fn i_c12dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12DD8: jsr $c13726(pc)
pub fn i_c12dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12DDC);
    cpu.pc = 0xC13726;
    16
}
/// $C12DDC: tst.b $ac6(a5)
pub fn i_c12ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DE0;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12DE0: bne.b $c12dd6
pub fn i_c12de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DE2;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12DD6; return 10; }
    8
}
/// $C12DE2: andi.b #$f, $ad0(a5)
pub fn i_c12de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DE8;
    cpu.ir = 0x22D;
    let ad1 = cpu.a[5].wrapping_add(0xAD0);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C12DE8: tst.b $a15(a5)
pub fn i_c12de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DEC;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12DEC: bmi.w $c12e82
pub fn i_c12dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DF0;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC12E82; return 10; }
    8
}
/// $C12DF0: jsr $c13674(pc)
pub fn i_c12df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12DF4);
    cpu.pc = 0xC13674;
    16
}
/// $C12DF4: tst.w d3
pub fn i_c12df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DF6;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C12DF6: beq.b $c12e00
pub fn i_c12df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DF8;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12E00; return 10; }
    8
}
/// $C12DF8: move.b #$82, $ac6(a5)
pub fn i_c12df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DFE;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12DFE: rts
pub fn i_c12dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12E00: movea.l $ac8(a5), a4
pub fn i_c12e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E04;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C12E04: move.w $acc(a5), d7
pub fn i_c12e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E08;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12E08: movea.l $a0a(a5), a1
pub fn i_c12e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E0C;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C12E0C: tst.b $a15(a5)
pub fn i_c12e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E10;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12E10: bne.w $c12e46
pub fn i_c12e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12E46; return 10; }
    8
}
/// $C12E14: move.w $6(a0), d3
pub fn i_c12e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E18;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12E18: jsr $c13616(pc)
pub fn i_c12e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12E1C);
    cpu.pc = 0xC13616;
    16
}
/// $C12E1C: moveq #$3f, d6
pub fn i_c12e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E1E;
    cpu.ir = 0x7C3F;
    cpu.d[6] = 0x3F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12E1E: move.w (a3)+, d0
pub fn i_c12e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E20;
    cpu.ir = 0x301B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C12E20: move.b d0, (a4)+
pub fn i_c12e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E22;
    cpu.ir = 0x18C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C12E22: subq.w #$1, d7
pub fn i_c12e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E24;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12E24: beq.b $c12e40
pub fn i_c12e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E26;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12E40; return 10; }
    8
}
/// $C12E26: dbra d6, $c12e1e
pub fn i_c12e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E2A;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12E2A; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12E2A; return 14; }
    cpu.pc = 0xC12E1E;
    10
}
/// $C12E2A: jsr $c1360c(pc)
pub fn i_c12e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12E2E);
    cpu.pc = 0xC1360C;
    16
}
/// $C12E2E: cmpi.b #$3, d3
pub fn i_c12e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E32;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C12E32: bcs.w $c12e40
pub fn i_c12e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E36;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC12E40; return 10; }
    8
}
/// $C12E36: move.b d0, $300001.l
pub fn i_c12e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E3C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12E3C: moveq #$3f, d6
pub fn i_c12e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E3E;
    cpu.ir = 0x7C3F;
    cpu.d[6] = 0x3F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12E3E: bra.b $c12e1e
pub fn i_c12e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E40;
    cpu.ir = 0x60DE;
    cpu.pc = 0xC12E1E;
    10
}
/// $C12E40: move.l a4, $ac8(a5)
pub fn i_c12e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E44;
    cpu.ir = 0x2B4C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12E44: rts
pub fn i_c12e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12E46: move.w $2(a0), d3
pub fn i_c12e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E4A;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12E4A: jsr $c1363c(pc)
pub fn i_c12e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12E4E);
    cpu.pc = 0xC1363C;
    16
}
/// $C12E4E: moveq #$1f, d6
pub fn i_c12e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E50;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12E50: move.w (a3)+, d0
pub fn i_c12e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E52;
    cpu.ir = 0x301B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C12E52: move.w d0, d1
pub fn i_c12e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E54;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12E54: lsr.w #$8, d0
pub fn i_c12e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E56;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12E56: move.b d0, (a4)+
pub fn i_c12e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E58;
    cpu.ir = 0x18C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C12E58: subq.w #$1, d7
pub fn i_c12e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E5A;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12E5A: beq.b $c12e7c
pub fn i_c12e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E5C;
    cpu.ir = 0x6720;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12E7C; return 10; }
    8
}
/// $C12E5C: move.b d1, (a4)+
pub fn i_c12e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E5E;
    cpu.ir = 0x18C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C12E5E: subq.w #$1, d7
pub fn i_c12e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E60;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12E60: beq.b $c12e7c
pub fn i_c12e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E62;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12E7C; return 10; }
    8
}
/// $C12E62: dbra d6, $c12e50
pub fn i_c12e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E66;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12E66; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12E66; return 14; }
    cpu.pc = 0xC12E50;
    10
}
/// $C12E66: jsr $c13626(pc)
pub fn i_c12e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12E6A);
    cpu.pc = 0xC13626;
    16
}
/// $C12E6A: cmpi.b #$3, d3
pub fn i_c12e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E6E;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C12E6E: bcs.w $c12e7c
pub fn i_c12e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E72;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC12E7C; return 10; }
    8
}
/// $C12E72: move.b d0, $300001.l
pub fn i_c12e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E78;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12E78: moveq #$1f, d6
pub fn i_c12e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E7A;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12E7A: bra.b $c12e50
pub fn i_c12e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E7C;
    cpu.ir = 0x60D4;
    cpu.pc = 0xC12E50;
    10
}
/// $C12E7C: move.l a4, $ac8(a5)
pub fn i_c12e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E80;
    cpu.ir = 0x2B4C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12E80: rts
pub fn i_c12e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12E82: jsr $c136ee(pc)
pub fn i_c12e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12E86);
    cpu.pc = 0xC136EE;
    16
}
/// $C12E86: tst.w d3
pub fn i_c12e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E88;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C12E88: beq.b $c12e92
pub fn i_c12e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E8A;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12E92; return 10; }
    8
}
/// $C12E8A: move.b #$82, $ac6(a5)
pub fn i_c12e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E90;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12E90: rts
pub fn i_c12e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12E92: movea.l $ac8(a5), a4
pub fn i_c12e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E96;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C12E96: move.w $acc(a5), d7
pub fn i_c12e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E9A;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12E9A: movea.l $a0a(a5), a1
pub fn i_c12e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12E9E;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C12E9E: move.w $4(a0), d3
pub fn i_c12e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EA2;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12EA2: jsr $c13664(pc)
pub fn i_c12ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12EA6);
    cpu.pc = 0xC13664;
    16
}
/// $C12EA6: moveq #$1f, d6
pub fn i_c12ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EA8;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12EA8: move.w (a3), d0
pub fn i_c12ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EAA;
    cpu.ir = 0x3013;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12EAA: addq.l #$4, a3
pub fn i_c12eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EAC;
    cpu.ir = 0x588B;
    cpu.a[3] = cpu.a[3].wrapping_add(4);
    8
}
/// $C12EAC: move.w d0, d1
pub fn i_c12eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EAE;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12EAE: lsr.w #$8, d0
pub fn i_c12eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EB0;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12EB0: move.b d0, (a4)+
pub fn i_c12eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EB2;
    cpu.ir = 0x18C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C12EB2: subq.w #$1, d7
pub fn i_c12eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EB4;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12EB4: beq.b $c12ed6
pub fn i_c12eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EB6;
    cpu.ir = 0x6720;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12ED6; return 10; }
    8
}
/// $C12EB6: move.b d1, (a4)+
pub fn i_c12eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EB8;
    cpu.ir = 0x18C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C12EB8: subq.w #$1, d7
pub fn i_c12eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EBA;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12EBA: beq.b $c12ed6
pub fn i_c12eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EBC;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12ED6; return 10; }
    8
}
/// $C12EBC: dbra d6, $c12ea8
pub fn i_c12ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EC0;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12EC0; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12EC0; return 14; }
    cpu.pc = 0xC12EA8;
    10
}
/// $C12EC0: jsr $c1364c(pc)
pub fn i_c12ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12EC4);
    cpu.pc = 0xC1364C;
    16
}
/// $C12EC4: cmpi.b #$3, d3
pub fn i_c12ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EC8;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C12EC8: bcs.w $c12ed6
pub fn i_c12ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12ECC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC12ED6; return 10; }
    8
}
/// $C12ECC: move.b d0, $300001.l
pub fn i_c12ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12ED2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12ED2: moveq #$1f, d6
pub fn i_c12ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12ED4;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12ED4: bra.b $c12ea8
pub fn i_c12ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12ED6;
    cpu.ir = 0x60D2;
    cpu.pc = 0xC12EA8;
    10
}
/// $C12ED6: move.l a4, $ac8(a5)
pub fn i_c12ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EDA;
    cpu.ir = 0x2B4C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12EDA: rts
pub fn i_c12eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1328A: jsr $c13978(pc)
pub fn i_c1328a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1328E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1328E);
    cpu.pc = 0xC13978;
    16
}
/// $C1328E: tst.b $ac6(a5)
pub fn i_c1328e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13292;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C13292: beq.b $c13296
pub fn i_c13292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13294;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13296; return 10; }
    8
}
/// $C13294: rts
pub fn i_c13294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13296;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13296: jsr $c13726(pc)
pub fn i_c13296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1329A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1329A);
    cpu.pc = 0xC13726;
    16
}
/// $C1329A: tst.b $ac6(a5)
pub fn i_c1329a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1329E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1329E: bne.b $c13294
pub fn i_c1329e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132A0;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13294; return 10; }
    8
}
/// $C132A0: movea.l $ac8(a5), a4
pub fn i_c132a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132A4;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C132A4: move.w $acc(a5), d6
pub fn i_c132a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132A8;
    cpu.ir = 0x3C2D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C132A8: addq.w #$1, d6
pub fn i_c132a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132AA;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C132AA: movea.l $a04(a5), a0
pub fn i_c132aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132AE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C132AE: move.w $a08(a5), d7
pub fn i_c132ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132B2;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C132B2: tst.b $a15(a5)
pub fn i_c132b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132B6;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C132B6: bmi.w $c1337e
pub fn i_c132b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132BA;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC1337E; return 10; }
    8
}
/// $C132BA: bne.w $c13312
pub fn i_c132ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13312; return 10; }
    8
}
/// $C132BE: move.b d0, $300001.l
pub fn i_c132be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132C4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C132C4: move.w (a0), d0
pub fn i_c132c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132C6;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C132C6: cmpi.b #$ff, d0
pub fn i_c132c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132CA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C132CA: beq.b $c132d0
pub fn i_c132ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132CC;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC132D0; return 10; }
    8
}
/// $C132CC: subq.w #$1, d6
pub fn i_c132cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132CE;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C132CE: beq.b $c132de
pub fn i_c132ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132D0;
    cpu.ir = 0x670E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC132DE; return 10; }
    8
}
/// $C132D0: addq.l #$8, a0
pub fn i_c132d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132D2;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C132D2: dbra d7, $c132be
pub fn i_c132d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132D6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC132D6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC132D6; return 14; }
    cpu.pc = 0xC132BE;
    10
}
/// $C132D6: move.b #$82, $ac6(a5)
pub fn i_c132d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132DC;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C132DC: rts
pub fn i_c132dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C132DE: movea.l $a0a(a5), a1
pub fn i_c132de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132E2;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C132E2: move.w $6(a0), d3
pub fn i_c132e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132E6;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C132E6: jsr $c13616(pc)
pub fn i_c132e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC132EA);
    cpu.pc = 0xC13616;
    16
}
/// $C132EA: moveq #$13, d7
pub fn i_c132ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132EC;
    cpu.ir = 0x7E13;
    cpu.d[7] = 0x13;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C132EC: move.w (a3)+, d0
pub fn i_c132ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132EE;
    cpu.ir = 0x301B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C132EE: cmpi.b #$ff, d0
pub fn i_c132ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132F2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C132F2: bne.b $c132f6
pub fn i_c132f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132F4;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC132F6; return 10; }
    8
}
/// $C132F4: addq.b #$1, d0
pub fn i_c132f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132F6;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C132F6: move.b d0, (a4)+
pub fn i_c132f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132F8;
    cpu.ir = 0x18C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C132F8: dbra d7, $c132ec
pub fn i_c132f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC132FC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC132FC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC132FC; return 14; }
    cpu.pc = 0xC132EC;
    10
}
/// $C132FC: move.w $2(a0), d0
pub fn i_c132fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13300;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13300: move.b d0, $ace(a5)
pub fn i_c13300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13304;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13304: move.w $4(a0), d0
pub fn i_c13304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13308;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13308: move.b d0, $acf(a5)
pub fn i_c13308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1330C;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1330C: move.w (a0), d0
pub fn i_c1330c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1330E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1330E: bra.w $c13370
pub fn i_c1330e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13312;
    cpu.ir = 0x6000;
    cpu.pc = 0xC13370;
    10
}
/// $C13312: move.b d0, $300001.l
pub fn i_c13312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13318;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13318: move.w (a0), d0
pub fn i_c13318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1331A;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1331A: lsr.w #$8, d0
pub fn i_c1331a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1331C;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C1331C: cmpi.b #$ff, d0
pub fn i_c1331c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13320;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C13320: beq.b $c13326
pub fn i_c13320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13322;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13326; return 10; }
    8
}
/// $C13322: subq.w #$1, d6
pub fn i_c13322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13324;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13324: beq.b $c13334
pub fn i_c13324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13326;
    cpu.ir = 0x670E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13334; return 10; }
    8
}
/// $C13326: addq.l #$4, a0
pub fn i_c13326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13328;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13328: dbra d7, $c13312
pub fn i_c13328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1332C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1332C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1332C; return 14; }
    cpu.pc = 0xC13312;
    10
}
/// $C1332C: move.b #$82, $ac6(a5)
pub fn i_c1332c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13332;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13332: rts
pub fn i_c13332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13334;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13334: movea.l $a0a(a5), a1
pub fn i_c13334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13338;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C13338: move.w $2(a0), d3
pub fn i_c13338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1333C;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1333C: jsr $c1363c(pc)
pub fn i_c1333c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13340;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13340);
    cpu.pc = 0xC1363C;
    16
}
/// $C13340: moveq #$9, d7
pub fn i_c13340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13342;
    cpu.ir = 0x7E09;
    cpu.d[7] = 0x9;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13342: move.w (a3)+, d0
pub fn i_c13342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13344;
    cpu.ir = 0x301B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13344: move.w d0, d1
pub fn i_c13344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13346;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13346: addq.b #$1, d1
pub fn i_c13346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13348;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C13348: bne.b $c1334c
pub fn i_c13348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1334A;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1334C; return 10; }
    8
}
/// $C1334A: clr.b d0
pub fn i_c1334a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1334C;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1334C: clr.b d1
pub fn i_c1334c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1334E;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1334E: addi.w #$100, d1
pub fn i_c1334e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13352;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13352: bne.b $c13358
pub fn i_c13352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13354;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13358; return 10; }
    8
}
/// $C13354: andi.w #$ff, d0
pub fn i_c13354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13358;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13358: move.w d0, (a4)+
pub fn i_c13358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1335A;
    cpu.ir = 0x38C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1335A: dbra d7, $c13342
pub fn i_c1335a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1335E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1335E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1335E; return 14; }
    cpu.pc = 0xC13342;
    10
}
/// $C1335E: move.w $2(a0), d0
pub fn i_c1335e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13362;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13362: lsr.w #$8, d0
pub fn i_c13362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13364;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13364: move.b d0, $acf(a5)
pub fn i_c13364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13368;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13368: move.w (a0), d0
pub fn i_c13368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1336A;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1336A: move.b d0, $ace(a5)
pub fn i_c1336a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1336E;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1336E: lsr.w #$8, d0
pub fn i_c1336e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13370;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13370: andi.w #$f, d0
pub fn i_c13370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13374;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13374: move.b d0, $ad0(a5)
pub fn i_c13374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13378;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13378: addq.w #$1, $acc(a5)
pub fn i_c13378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1337C;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0xACC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1337C: rts
pub fn i_c1337c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1337E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1337E: move.b d0, $300001.l
pub fn i_c1337e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13384;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13384: move.w (a0), d0
pub fn i_c13384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13386;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13386: lsr.w #$8, d0
pub fn i_c13386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13388;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13388: cmpi.b #$ff, d0
pub fn i_c13388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1338C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C1338C: beq.b $c13392
pub fn i_c1338c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1338E;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13392; return 10; }
    8
}
/// $C1338E: subq.w #$1, d6
pub fn i_c1338e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13390;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13390: beq.b $c133a0
pub fn i_c13390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13392;
    cpu.ir = 0x670E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC133A0; return 10; }
    8
}
/// $C13392: addq.l #$8, a0
pub fn i_c13392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13394;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C13394: dbra d7, $c1337e
pub fn i_c13394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13398;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13398; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13398; return 14; }
    cpu.pc = 0xC1337E;
    10
}
/// $C13398: move.b #$82, $ac6(a5)
pub fn i_c13398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1339E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1339E: rts
pub fn i_c1339e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C133A0: movea.l $a0a(a5), a1
pub fn i_c133a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133A4;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C133A4: move.w $4(a0), d3
pub fn i_c133a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133A8;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C133A8: jsr $c13664(pc)
pub fn i_c133a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC133AC);
    cpu.pc = 0xC13664;
    16
}
/// $C133AC: moveq #$9, d7
pub fn i_c133ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133AE;
    cpu.ir = 0x7E09;
    cpu.d[7] = 0x9;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C133AE: move.w (a3), d0
pub fn i_c133ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133B0;
    cpu.ir = 0x3013;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C133B0: addq.l #$4, a3
pub fn i_c133b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133B2;
    cpu.ir = 0x588B;
    cpu.a[3] = cpu.a[3].wrapping_add(4);
    8
}
/// $C133B2: move.w d0, d1
pub fn i_c133b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133B4;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C133B4: addq.b #$1, d1
pub fn i_c133b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133B6;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C133B6: bne.b $c133ba
pub fn i_c133b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133B8;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC133BA; return 10; }
    8
}
/// $C133B8: clr.b d0
pub fn i_c133b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133BA;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C133BA: clr.b d1
pub fn i_c133ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133BC;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C133BC: addi.w #$100, d1
pub fn i_c133bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133C0;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C133C0: bne.b $c133c6
pub fn i_c133c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133C2;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC133C6; return 10; }
    8
}
/// $C133C2: andi.w #$ff, d0
pub fn i_c133c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C133C6: move.w d0, (a4)+
pub fn i_c133c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133C8;
    cpu.ir = 0x38C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C133C8: dbra d7, $c133ae
pub fn i_c133c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133CC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC133CC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC133CC; return 14; }
    cpu.pc = 0xC133AE;
    10
}
/// $C133CC: move.w $4(a0), d0
pub fn i_c133cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133D0;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C133D0: lsr.w #$8, d0
pub fn i_c133d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133D2;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C133D2: move.b d0, $acf(a5)
pub fn i_c133d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133D6;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C133D6: move.w (a0), d0
pub fn i_c133d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133D8;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C133D8: move.b d0, $ace(a5)
pub fn i_c133d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133DC;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C133DC: lsr.w #$8, d0
pub fn i_c133dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133DE;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C133DE: bra.b $c13370
pub fn i_c133de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133E0;
    cpu.ir = 0x6090;
    cpu.pc = 0xC13370;
    10
}
/// $C1360C: andi.w #$ff, d3
pub fn i_c1360c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13610;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13610: add.w d3, d3
pub fn i_c13610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13612;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C13612: move.w (a1, d3.w), d3
pub fn i_c13612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13616;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13616: move.w d3, d4
pub fn i_c13616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13618;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13618: andi.l #$ff, d4
pub fn i_c13618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1361E;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $C1361E: lsl.l #$7, d4
pub fn i_c1361e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13620;
    cpu.ir = 0xEF8C;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[4] = r;
    20
}
/// $C13620: movea.l a6, a3
pub fn i_c13620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13622;
    cpu.ir = 0x264E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $C13622: adda.l d4, a3
pub fn i_c13622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13624;
    cpu.ir = 0xD7C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $C13624: rts
pub fn i_c13624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13626;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13626: andi.w #$ff, d3
pub fn i_c13626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1362A;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1362A: btst.b #$0, d3
pub fn i_c1362a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1362E;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1362E: bne.b $c13638
pub fn i_c1362e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13630;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13638; return 10; }
    8
}
/// $C13630: move.w (a1, d3.w), d3
pub fn i_c13630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13634;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13634: lsr.w #$8, d3
pub fn i_c13634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13636;
    cpu.ir = 0xE04B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13636: bra.b $c1363c
pub fn i_c13636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13638;
    cpu.ir = 0x6004;
    cpu.pc = 0xC1363C;
    10
}
/// $C13638: move.w -$1(a1, d3.w), d3
pub fn i_c13638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1363C;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1363C: move.w d3, d4
pub fn i_c1363c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1363E;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1363E: andi.l #$ff, d4
pub fn i_c1363e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13644;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $C13644: lsl.l #$6, d4
pub fn i_c13644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13646;
    cpu.ir = 0xED8C;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[4] = r;
    18
}
/// $C13646: movea.l a6, a3
pub fn i_c13646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13648;
    cpu.ir = 0x264E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $C13648: adda.l d4, a3
pub fn i_c13648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1364A;
    cpu.ir = 0xD7C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $C1364A: rts
pub fn i_c1364a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1364C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1364C: andi.w #$ff, d3
pub fn i_c1364c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13650;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13650: add.w d3, d3
pub fn i_c13650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13652;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C13652: btst.b #$1, d3
pub fn i_c13652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13656;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13656: bne.b $c13660
pub fn i_c13656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13658;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13660; return 10; }
    8
}
/// $C13658: move.w (a1, d3.w), d3
pub fn i_c13658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1365C;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1365C: lsr.w #$8, d3
pub fn i_c1365c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1365E;
    cpu.ir = 0xE04B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C1365E: bra.b $c13664
pub fn i_c1365e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13660;
    cpu.ir = 0x6004;
    cpu.pc = 0xC13664;
    10
}
/// $C13660: move.w (a1, d3.w), d3
pub fn i_c13660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13664;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13664: move.w d3, d4
pub fn i_c13664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13666;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13666: andi.l #$ff, d4
pub fn i_c13666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1366C;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $C1366C: lsl.l #$7, d4
pub fn i_c1366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1366E;
    cpu.ir = 0xEF8C;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[4] = r;
    20
}
/// $C1366E: movea.l a6, a3
pub fn i_c1366e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13670;
    cpu.ir = 0x264E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $C13670: adda.l d4, a3
pub fn i_c13670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13672;
    cpu.ir = 0xD7C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $C13672: rts
pub fn i_c13672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13674;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13674: moveq #$0, d3
pub fn i_c13674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13676;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13676: tst.w $a14(a5)
pub fn i_c13676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1367A;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xA14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C1367A: bne.w $c136b8
pub fn i_c1367a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1367E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC136B8; return 10; }
    8
}
/// $C1367E: movea.l $a04(a5), a0
pub fn i_c1367e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13682;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C13682: move.w $a08(a5), d7
pub fn i_c13682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13686;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13686: move.b d0, $300001.l
pub fn i_c13686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1368C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1368C: move.w (a0), d1
pub fn i_c1368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1368E;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1368E: cmpi.b #$ff, d1
pub fn i_c1368e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13692;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C13692: beq.b $c136ae
pub fn i_c13692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13694;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC136AE; return 10; }
    8
}
/// $C13694: move.w $2(a0), d0
pub fn i_c13694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13698;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13698: cmp.b $ace(a5), d0
pub fn i_c13698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1369C;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C1369C: bne.b $c136ae
pub fn i_c1369c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1369E;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC136AE; return 10; }
    8
}
/// $C1369E: move.w $4(a0), d0
pub fn i_c1369e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136A2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C136A2: cmp.b $acf(a5), d0
pub fn i_c136a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136A6;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C136A6: bne.b $c136ae
pub fn i_c136a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136A8;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC136AE; return 10; }
    8
}
/// $C136A8: cmp.b $ad0(a5), d1
pub fn i_c136a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136AC;
    cpu.ir = 0xB22D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C136AC: beq.b $c136b6
pub fn i_c136ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136AE;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC136B6; return 10; }
    8
}
/// $C136AE: addq.l #$8, a0
pub fn i_c136ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136B0;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C136B0: dbra d7, $c13686
pub fn i_c136b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136B4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC136B4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC136B4; return 14; }
    cpu.pc = 0xC13686;
    10
}
/// $C136B4: move.w d7, d3
pub fn i_c136b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136B6;
    cpu.ir = 0x3607;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C136B6: rts
pub fn i_c136b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C136B8: movea.l $a04(a5), a0
pub fn i_c136b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136BC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C136BC: move.w $a08(a5), d7
pub fn i_c136bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136C0;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C136C0: move.w (a0), d1
pub fn i_c136c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136C2;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C136C2: lsr.w #$8, d1
pub fn i_c136c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136C4;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C136C4: cmpi.b #$ff, d1
pub fn i_c136c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136C8;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C136C8: beq.b $c136e4
pub fn i_c136c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136CA;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC136E4; return 10; }
    8
}
/// $C136CA: move.w (a0), d0
pub fn i_c136ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136CC;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C136CC: cmp.b $ace(a5), d0
pub fn i_c136cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136D0;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C136D0: bne.b $c136e4
pub fn i_c136d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136D2;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC136E4; return 10; }
    8
}
/// $C136D2: move.w $2(a0), d0
pub fn i_c136d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136D6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C136D6: lsr.w #$8, d0
pub fn i_c136d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136D8;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C136D8: cmp.b $acf(a5), d0
pub fn i_c136d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136DC;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C136DC: bne.b $c136e4
pub fn i_c136dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136DE;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC136E4; return 10; }
    8
}
/// $C136DE: cmp.b $ad0(a5), d1
pub fn i_c136de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136E2;
    cpu.ir = 0xB22D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C136E2: beq.b $c136ec
pub fn i_c136e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136E4;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC136EC; return 10; }
    8
}
/// $C136E4: addq.l #$4, a0
pub fn i_c136e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136E6;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C136E6: dbra d7, $c136c0
pub fn i_c136e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136EA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC136EA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC136EA; return 14; }
    cpu.pc = 0xC136C0;
    10
}
/// $C136EA: move.w d7, d3
pub fn i_c136ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136EC;
    cpu.ir = 0x3607;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C136EC: rts
pub fn i_c136ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C136EE: moveq #$0, d3
pub fn i_c136ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136F0;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C136F0: movea.l $a04(a5), a0
pub fn i_c136f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136F4;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C136F4: move.w $a08(a5), d7
pub fn i_c136f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136F8;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C136F8: move.w (a0), d1
pub fn i_c136f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136FA;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C136FA: lsr.w #$8, d1
pub fn i_c136fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC136FC;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C136FC: cmpi.b #$ff, d1
pub fn i_c136fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13700;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C13700: beq.b $c1371c
pub fn i_c13700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13702;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1371C; return 10; }
    8
}
/// $C13702: move.w (a0), d0
pub fn i_c13702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13704;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13704: cmp.b $ace(a5), d0
pub fn i_c13704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13708;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C13708: bne.b $c1371c
pub fn i_c13708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1370A;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1371C; return 10; }
    8
}
/// $C1370A: move.w $4(a0), d0
pub fn i_c1370a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1370E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1370E: lsr.w #$8, d0
pub fn i_c1370e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13710;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13710: cmp.b $acf(a5), d0
pub fn i_c13710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13714;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C13714: bne.b $c1371c
pub fn i_c13714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13716;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1371C; return 10; }
    8
}
/// $C13716: cmp.b $ad0(a5), d1
pub fn i_c13716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1371A;
    cpu.ir = 0xB22D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C1371A: beq.b $c13724
pub fn i_c1371a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1371C;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13724; return 10; }
    8
}
/// $C1371C: addq.l #$8, a0
pub fn i_c1371c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1371E;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C1371E: dbra d7, $c136f8
pub fn i_c1371e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13722;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13722; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13722; return 14; }
    cpu.pc = 0xC136F8;
    10
}
/// $C13722: move.w d7, d3
pub fn i_c13722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13724;
    cpu.ir = 0x3607;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13724: rts
pub fn i_c13724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13726;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13726: tst.b $a15(a5)
pub fn i_c13726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1372A;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1372A: bmi.w $c138a2
pub fn i_c1372a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1372E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC138A2; return 10; }
    8
}
/// $C1372E: bne.w $c137e6
pub fn i_c1372e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13732;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC137E6; return 10; }
    8
}
/// $C13732: moveq #$0, d5
pub fn i_c13732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13734;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13734: movea.l $a0a(a5), a0
pub fn i_c13734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13738;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C13738: jsr $c137cc(pc)
pub fn i_c13738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1373C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1373C);
    cpu.pc = 0xC137CC;
    16
}
/// $C1373C: move.w $1a(a6), d1
pub fn i_c1373c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13740;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13740: cmp.b d0, d1
pub fn i_c13740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13742;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C13742: beq.b $c13746
pub fn i_c13742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13744;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13746; return 10; }
    8
}
/// $C13744: moveq #$1, d5
pub fn i_c13744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13746;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13746: jsr $c137cc(pc)
pub fn i_c13746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1374A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1374A);
    cpu.pc = 0xC137CC;
    16
}
/// $C1374A: move.w $1c(a6), d1
pub fn i_c1374a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1374E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1374E: cmp.b d0, d1
pub fn i_c1374e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13750;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C13750: beq.b $c13754
pub fn i_c13750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13752;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13754; return 10; }
    8
}
/// $C13752: addq.b #$2, d5
pub fn i_c13752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13754;
    cpu.ir = 0x5405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C13754: tst.b d5
pub fn i_c13754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13756;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C13756: beq.w $c1376a
pub fn i_c13756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1375A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1376A; return 10; }
    8
}
/// $C1375A: cmpi.b #$2, d5
pub fn i_c1375a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1375E;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C1375E: beq.b $c1376c
pub fn i_c1375e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13760;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1376C; return 10; }
    8
}
/// $C13760: bcs.w $c137a0
pub fn i_c13760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13764;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC137A0; return 10; }
    8
}
/// $C13764: move.b #$83, $ac6(a5)
pub fn i_c13764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1376A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1376A: rts
pub fn i_c1376a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1376C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1376C: move.b d0, $300001.l
pub fn i_c1376c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13772;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13772: jsr $c13e58(pc)
pub fn i_c13772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13776;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13776);
    cpu.pc = 0xC13E58;
    16
}
/// $C13776: movea.l $a0a(a5), a0
pub fn i_c13776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1377A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1377A: movea.l a0, a1
pub fn i_c1377a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1377C;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C1377C: moveq #$0, d0
pub fn i_c1377c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1377E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1377E: move.w $a0e(a5), d0
pub fn i_c1377e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13782;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13782: move.w d0, d7
pub fn i_c13782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13784;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13784: lsr.w #$1, d7
pub fn i_c13784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13786;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13786: subq.w #$1, d7
pub fn i_c13786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13788;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13788: adda.l d0, a1
pub fn i_c13788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1378A;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C1378A: moveq #$0, d0
pub fn i_c1378a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1378C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1378C: move.w (a0)+, d1
pub fn i_c1378c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1378E;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C1378E: move.w d1, (a1)+
pub fn i_c1378e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13790;
    cpu.ir = 0x32C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13790: add.b d1, d0
pub fn i_c13790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13792;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C13792: dbra d7, $c1378c
pub fn i_c13792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13796;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13796; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13796; return 14; }
    cpu.pc = 0xC1378C;
    10
}
/// $C13796: move.w d0, $1a(a6)
pub fn i_c13796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1379A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1379A: move.w d0, $1c(a6)
pub fn i_c1379a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1379E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1379E: rts
pub fn i_c1379e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C137A0: move.b d0, $300001.l
pub fn i_c137a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137A6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C137A6: jsr $c13e58(pc)
pub fn i_c137a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC137AA);
    cpu.pc = 0xC13E58;
    16
}
/// $C137AA: movea.l $a0a(a5), a0
pub fn i_c137aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137AE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C137AE: movea.l a0, a1
pub fn i_c137ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137B0;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C137B0: moveq #$0, d0
pub fn i_c137b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137B2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C137B2: move.w $a0e(a5), d0
pub fn i_c137b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137B6;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C137B6: move.w d0, d7
pub fn i_c137b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137B8;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C137B8: lsr.w #$1, d7
pub fn i_c137b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137BA;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C137BA: subq.w #$1, d7
pub fn i_c137ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137BC;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C137BC: adda.l d0, a1
pub fn i_c137bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137BE;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C137BE: move.w (a1)+, (a0)+
pub fn i_c137be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137C0;
    cpu.ir = 0x30D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C137C0: dbra d7, $c137be
pub fn i_c137c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137C4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC137C4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC137C4; return 14; }
    cpu.pc = 0xC137BE;
    10
}
/// $C137C4: move.w $1c(a6), $1a(a6)
pub fn i_c137c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137CA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x1A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C137CA: rts
pub fn i_c137ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C137CC: move.b d0, $300001.l
pub fn i_c137cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137D2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C137D2: moveq #$0, d0
pub fn i_c137d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137D4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C137D4: move.w $a0e(a5), d7
pub fn i_c137d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137D8;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C137D8: lsr.w #$1, d7
pub fn i_c137d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137DA;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C137DA: subq.w #$1, d7
pub fn i_c137da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137DC;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C137DC: move.w (a0)+, d1
pub fn i_c137dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137DE;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C137DE: add.b d1, d0
pub fn i_c137de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137E0;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C137E0: dbra d7, $c137dc
pub fn i_c137e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137E4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC137E4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC137E4; return 14; }
    cpu.pc = 0xC137DC;
    10
}
/// $C137E4: rts
pub fn i_c137e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C137E6: moveq #$0, d5
pub fn i_c137e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137E8;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C137E8: movea.l $a0a(a5), a0
pub fn i_c137e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137EC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C137EC: jsr $c13884(pc)
pub fn i_c137ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC137F0);
    cpu.pc = 0xC13884;
    16
}
/// $C137F0: move.w $1a(a6), d1
pub fn i_c137f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137F4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C137F4: cmp.b d0, d1
pub fn i_c137f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137F6;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C137F6: beq.b $c137fa
pub fn i_c137f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137F8;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC137FA; return 10; }
    8
}
/// $C137F8: moveq #$1, d5
pub fn i_c137f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137FA;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C137FA: jsr $c13884(pc)
pub fn i_c137fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC137FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC137FE);
    cpu.pc = 0xC13884;
    16
}
/// $C137FE: move.w $1c(a6), d1
pub fn i_c137fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13802;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13802: cmp.b d0, d1
pub fn i_c13802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13804;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C13804: beq.b $c13808
pub fn i_c13804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13806;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13808; return 10; }
    8
}
/// $C13806: addq.b #$2, d5
pub fn i_c13806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13808;
    cpu.ir = 0x5405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C13808: tst.b d5
pub fn i_c13808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1380A;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1380A: beq.w $c1381e
pub fn i_c1380a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1380E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1381E; return 10; }
    8
}
/// $C1380E: cmpi.b #$2, d5
pub fn i_c1380e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13812;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C13812: beq.b $c13820
pub fn i_c13812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13814;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13820; return 10; }
    8
}
/// $C13814: bcs.w $c13858
pub fn i_c13814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13818;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC13858; return 10; }
    8
}
/// $C13818: move.b #$83, $ac6(a5)
pub fn i_c13818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1381E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1381E: rts
pub fn i_c1381e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13820;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13820: move.b d0, $300001.l
pub fn i_c13820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13826;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13826: jsr $c13e58(pc)
pub fn i_c13826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1382A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1382A);
    cpu.pc = 0xC13E58;
    16
}
/// $C1382A: movea.l $a0a(a5), a0
pub fn i_c1382a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1382E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1382E: movea.l a0, a1
pub fn i_c1382e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13830;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C13830: moveq #$0, d0
pub fn i_c13830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13832;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13832: move.w $a0e(a5), d0
pub fn i_c13832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13836;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13836: move.w d0, d7
pub fn i_c13836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13838;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13838: lsr.w #$1, d7
pub fn i_c13838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1383A;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C1383A: subq.w #$1, d7
pub fn i_c1383a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1383C;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1383C: adda.l d0, a1
pub fn i_c1383c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1383E;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C1383E: moveq #$0, d0
pub fn i_c1383e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13840;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13840: move.w (a0)+, d1
pub fn i_c13840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13842;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13842: move.w d1, (a1)+
pub fn i_c13842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13844;
    cpu.ir = 0x32C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13844: add.b d1, d0
pub fn i_c13844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13846;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C13846: lsr.w #$8, d1
pub fn i_c13846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13848;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13848: add.b d1, d0
pub fn i_c13848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1384A;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C1384A: dbra d7, $c13840
pub fn i_c1384a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1384E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1384E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1384E; return 14; }
    cpu.pc = 0xC13840;
    10
}
/// $C1384E: move.w d0, $1a(a6)
pub fn i_c1384e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13852;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13852: move.w d0, $1c(a6)
pub fn i_c13852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13856;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13856: rts
pub fn i_c13856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13858;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13858: move.b d0, $300001.l
pub fn i_c13858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1385E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1385E: jsr $c13e58(pc)
pub fn i_c1385e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13862;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13862);
    cpu.pc = 0xC13E58;
    16
}
/// $C13862: movea.l $a0a(a5), a0
pub fn i_c13862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13866;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C13866: movea.l a0, a1
pub fn i_c13866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13868;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C13868: moveq #$0, d0
pub fn i_c13868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1386A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1386A: move.w $a0e(a5), d0
pub fn i_c1386a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1386E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1386E: move.w d0, d7
pub fn i_c1386e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13870;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13870: lsr.w #$1, d7
pub fn i_c13870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13872;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13872: subq.w #$1, d7
pub fn i_c13872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13874;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13874: adda.l d0, a1
pub fn i_c13874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13876;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C13876: move.w (a1)+, (a0)+
pub fn i_c13876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13878;
    cpu.ir = 0x30D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C13878: dbra d7, $c13876
pub fn i_c13878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1387C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1387C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1387C; return 14; }
    cpu.pc = 0xC13876;
    10
}
/// $C1387C: move.w $1c(a6), $1a(a6)
pub fn i_c1387c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13882;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x1A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C13882: rts
pub fn i_c13882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13884;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13884: move.b d0, $300001.l
pub fn i_c13884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1388A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1388A: moveq #$0, d0
pub fn i_c1388a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1388C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1388C: move.w $a0e(a5), d7
pub fn i_c1388c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13890;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13890: lsr.w #$1, d7
pub fn i_c13890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13892;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13892: subq.w #$1, d7
pub fn i_c13892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13894;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13894: move.w (a0)+, d1
pub fn i_c13894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13896;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13896: add.b d1, d0
pub fn i_c13896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13898;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C13898: lsr.w #$8, d1
pub fn i_c13898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1389A;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C1389A: add.b d1, d0
pub fn i_c1389a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1389C;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C1389C: dbra d7, $c13894
pub fn i_c1389c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138A0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC138A0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC138A0; return 14; }
    cpu.pc = 0xC13894;
    10
}
/// $C138A0: rts
pub fn i_c138a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C138A2: moveq #$0, d5
pub fn i_c138a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138A4;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C138A4: movea.l $a0a(a5), a0
pub fn i_c138a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138A8;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C138A8: jsr $c13958(pc)
pub fn i_c138a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC138AC);
    cpu.pc = 0xC13958;
    16
}
/// $C138AC: move.w $1c(a6), d1
pub fn i_c138ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138B0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C138B0: lsr.w #$8, d1
pub fn i_c138b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138B2;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C138B2: cmp.b d0, d1
pub fn i_c138b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138B4;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C138B4: beq.b $c138b8
pub fn i_c138b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138B6;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC138B8; return 10; }
    8
}
/// $C138B6: moveq #$1, d5
pub fn i_c138b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138B8;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C138B8: jsr $c13958(pc)
pub fn i_c138b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC138BC);
    cpu.pc = 0xC13958;
    16
}
/// $C138BC: move.w $1c(a6), d1
pub fn i_c138bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138C0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C138C0: cmp.b d0, d1
pub fn i_c138c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138C2;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C138C2: beq.b $c138c6
pub fn i_c138c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138C4;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC138C6; return 10; }
    8
}
/// $C138C4: addq.b #$2, d5
pub fn i_c138c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138C6;
    cpu.ir = 0x5405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C138C6: tst.b d5
pub fn i_c138c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138C8;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C138C8: beq.w $c138dc
pub fn i_c138c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC138DC; return 10; }
    8
}
/// $C138CC: cmpi.b #$2, d5
pub fn i_c138cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138D0;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C138D0: beq.b $c138de
pub fn i_c138d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138D2;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC138DE; return 10; }
    8
}
/// $C138D2: bcs.w $c13920
pub fn i_c138d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC13920; return 10; }
    8
}
/// $C138D6: move.b #$83, $ac6(a5)
pub fn i_c138d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138DC;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C138DC: rts
pub fn i_c138dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C138DE: move.b d0, $300001.l
pub fn i_c138de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138E4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C138E4: jsr $c13e58(pc)
pub fn i_c138e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC138E8);
    cpu.pc = 0xC13E58;
    16
}
/// $C138E8: movea.l $a0a(a5), a0
pub fn i_c138e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138EC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C138EC: movea.l a0, a1
pub fn i_c138ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138EE;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C138EE: moveq #$0, d0
pub fn i_c138ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138F0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C138F0: move.w $a0e(a5), d0
pub fn i_c138f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138F4;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C138F4: move.w d0, d7
pub fn i_c138f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138F6;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C138F6: lsr.w #$1, d7
pub fn i_c138f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138F8;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C138F8: subq.w #$1, d7
pub fn i_c138f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138FA;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C138FA: adda.l d0, a1
pub fn i_c138fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138FC;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C138FC: moveq #$0, d0
pub fn i_c138fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC138FE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C138FE: move.w (a0), d1
pub fn i_c138fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13900;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13900: move.w d1, (a1)
pub fn i_c13900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13902;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C13902: addq.l #$4, a0
pub fn i_c13902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13904;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13904: addq.l #$4, a1
pub fn i_c13904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13906;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $C13906: add.b d1, d0
pub fn i_c13906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13908;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C13908: lsr.w #$8, d1
pub fn i_c13908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1390A;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C1390A: add.b d1, d0
pub fn i_c1390a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1390C;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C1390C: dbra d7, $c138fe
pub fn i_c1390c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13910;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13910; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13910; return 14; }
    cpu.pc = 0xC138FE;
    10
}
/// $C13910: move.b d0, d1
pub fn i_c13910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13912;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13912: lsl.w #$8, d0
pub fn i_c13912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13914;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13914: move.b d1, d0
pub fn i_c13914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13916;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13916: move.w d0, $1a(a6)
pub fn i_c13916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1391A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1391A: move.w d0, $1c(a6)
pub fn i_c1391a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1391E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1391E: rts
pub fn i_c1391e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13920;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13920: move.b d0, $300001.l
pub fn i_c13920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13926;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13926: jsr $c13e58(pc)
pub fn i_c13926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1392A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1392A);
    cpu.pc = 0xC13E58;
    16
}
/// $C1392A: movea.l $a0a(a5), a0
pub fn i_c1392a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1392E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1392E: movea.l a0, a1
pub fn i_c1392e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13930;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C13930: moveq #$0, d0
pub fn i_c13930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13932;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13932: move.w $a0e(a5), d0
pub fn i_c13932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13936;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13936: move.w d0, d7
pub fn i_c13936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13938;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13938: lsr.w #$1, d7
pub fn i_c13938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1393A;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C1393A: subq.w #$1, d7
pub fn i_c1393a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1393C;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1393C: adda.l d0, a1
pub fn i_c1393c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1393E;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C1393E: move.w (a1), (a0)
pub fn i_c1393e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13940;
    cpu.ir = 0x3091;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v2 as u16);
    12
}
/// $C13940: addq.l #$4, a0
pub fn i_c13940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13942;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13942: addq.l #$4, a1
pub fn i_c13942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13944;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $C13944: dbra d7, $c1393e
pub fn i_c13944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13948;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13948; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13948; return 14; }
    cpu.pc = 0xC1393E;
    10
}
/// $C13948: move.w $1c(a6), d0
pub fn i_c13948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1394C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1394C: move.w d0, d1
pub fn i_c1394c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1394E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1394E: lsl.w #$8, d0
pub fn i_c1394e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13950;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13950: move.b d1, d0
pub fn i_c13950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13952;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13952: move.w d0, $1c(a6)
pub fn i_c13952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13956;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13956: rts
pub fn i_c13956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13958: move.b d0, $300001.l
pub fn i_c13958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1395E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1395E: moveq #$0, d0
pub fn i_c1395e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13960;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13960: move.w $a0e(a5), d7
pub fn i_c13960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13964;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13964: lsr.w #$1, d7
pub fn i_c13964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13966;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13966: subq.w #$1, d7
pub fn i_c13966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13968;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13968: move.w (a0), d1
pub fn i_c13968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1396A;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1396A: addq.l #$4, a0
pub fn i_c1396a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1396C;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C1396C: add.b d1, d0
pub fn i_c1396c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1396E;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C1396E: lsr.w #$8, d1
pub fn i_c1396e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13970;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13970: add.b d1, d0
pub fn i_c13970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13972;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C13972: dbra d7, $c13968
pub fn i_c13972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13976;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13976; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13976; return 14; }
    cpu.pc = 0xC13968;
    10
}
/// $C13976: rts
pub fn i_c13976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13978;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13978: movea.l a6, a0
pub fn i_c13978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1397A;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1397A: moveq #$0, d6
pub fn i_c1397a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1397C;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1397C: lea.l $c134c2(pc), a1
pub fn i_c1397c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC134C2;
    16
}
/// $C13980: moveq #$3, d7
pub fn i_c13980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13982;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13982: cmpm.w (a0)+, (a1)+
pub fn i_c13982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13984;
    cpu.ir = 0xB348;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(2);
    let v1 = bus.read16(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read16(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(2);
    sub_with_flags(cpu, v2, v1, Size::Word, false);
    12
}
/// $C13984: bne.b $c139a0
pub fn i_c13984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13986;
    cpu.ir = 0x661A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC139A0; return 10; }
    8
}
/// $C13986: dbra d7, $c13982
pub fn i_c13986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1398A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1398A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1398A; return 14; }
    cpu.pc = 0xC13982;
    10
}
/// $C1398A: tst.w (a0)+
pub fn i_c1398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1398C;
    cpu.ir = 0x4A58;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C1398C: bne.b $c1399e
pub fn i_c1398c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1398E;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1399E; return 10; }
    8
}
/// $C1398E: move.b #$85, $ac6(a5)
pub fn i_c1398e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13994;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13994: rts
pub fn i_c13994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13996;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13996: move.b #$81, $ac6(a5)
pub fn i_c13996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1399C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x81;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1399C: rts
pub fn i_c1399c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1399E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1399E: adda.l (a0), a6
pub fn i_c1399e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139A0;
    cpu.ir = 0xDDD0;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[6] = cpu.a[6].wrapping_add(v2);
    16
}
/// $C139A0: move.b d0, $300001.l
pub fn i_c139a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139A6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C139A6: lea.l $40(a6), a0
pub fn i_c139a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139AA;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x40);
    cpu.a[0] = ad1;
    16
}
/// $C139AA: lea.l $c134ba(pc), a1
pub fn i_c139aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC134BA;
    16
}
/// $C139AE: moveq #$7, d7
pub fn i_c139ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139B0;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C139B0: move.w (a0), d0
pub fn i_c139b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139B2;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C139B2: addq.l #$4, a0
pub fn i_c139b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139B4;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C139B4: cmp.b (a1)+, d0
pub fn i_c139b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139B6;
    cpu.ir = 0xB019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    8
}
/// $C139B6: bne.b $c13996
pub fn i_c139b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139B8;
    cpu.ir = 0x66DE;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13996; return 10; }
    8
}
/// $C139B8: dbra d7, $c139b0
pub fn i_c139b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139BC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC139BC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC139BC; return 14; }
    cpu.pc = 0xC139B0;
    10
}
/// $C139BC: move.w $10(a6), d0
pub fn i_c139bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139C0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C139C0: move.b d0, $9f4(a5)
pub fn i_c139c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139C4;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9F4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C139C4: cmpi.b #$10, d0
pub fn i_c139c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139C8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $C139C8: bcc.b $c1398e
pub fn i_c139c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139CA;
    cpu.ir = 0x64C4;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1398E; return 10; }
    8
}
/// $C139CA: move.w $c(a6), d0
pub fn i_c139ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C139CE: tst.b d0
pub fn i_c139ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139D0;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C139D0: bne.w $c139f8
pub fn i_c139d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC139F8; return 10; }
    8
}
/// $C139D4: move.w $14(a6), d6
pub fn i_c139d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139D8;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C139D8: lsl.w #$8, d6
pub fn i_c139d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139DA;
    cpu.ir = 0xE14E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C139DA: move.w $16(a6), d5
pub fn i_c139da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139DE;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C139DE: move.b d5, d6
pub fn i_c139de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139E0;
    cpu.ir = 0x1C05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C139E0: move.w d6, $a16(a5)
pub fn i_c139e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139E4;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C139E4: move.w $12(a6), d0
pub fn i_c139e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139E8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C139E8: andi.w #$ff, d0
pub fn i_c139e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139EC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C139EC: move.w d0, $a14(a5)
pub fn i_c139ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139F0;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C139F0: beq.w $c13d02
pub fn i_c139f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13D02; return 10; }
    8
}
/// $C139F4: bra.w $c13dba
pub fn i_c139f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139F8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC13DBA;
    10
}
/// $C139F8: move.w $14(a6), d6
pub fn i_c139f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC139FC;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C139FC: move.w d6, $a16(a5)
pub fn i_c139fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A00;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13A00: move.w #$80, d0
pub fn i_c13a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13A04: move.w d0, $a14(a5)
pub fn i_c13a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A08;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13A08: bra.w $c13d02
pub fn i_c13a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A0C;
    cpu.ir = 0x6000;
    cpu.pc = 0xC13D02;
    10
}
/// $C13D02: lsr.w #$8, d6
pub fn i_c13d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D04;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13D04: andi.w #$fff0, d6
pub fn i_c13d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D08;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFFF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13D08: lea.l $c13d2a(pc, d6.w), a0
pub fn i_c13d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D0C;
    cpu.ir = 0x41FB;
    let ad1 = 0xC13D2A_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C13D0C: move.l a6, d0
pub fn i_c13d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D0E;
    cpu.ir = 0x200E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13D0E: add.l (a0)+, d0
pub fn i_c13d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D10;
    cpu.ir = 0xD098;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Long, true);
    cpu.d[0] = r;
    12
}
/// $C13D10: move.l d0, $a04(a5)
pub fn i_c13d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D14;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C13D14: move.w (a0)+, $a08(a5)
pub fn i_c13d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D18;
    cpu.ir = 0x3B58;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA08);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C13D18: add.l (a0)+, d0
pub fn i_c13d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D1A;
    cpu.ir = 0xD098;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Long, true);
    cpu.d[0] = r;
    12
}
/// $C13D1A: move.l d0, $a0a(a5)
pub fn i_c13d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D1E;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C13D1E: move.w (a0)+, $a0e(a5)
pub fn i_c13d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D22;
    cpu.ir = 0x3B58;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA0E);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C13D22: add.l (a0), d0
pub fn i_c13d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D24;
    cpu.ir = 0xD090;
    let v2 = bus.read32(cpu.a[0]);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Long, true);
    cpu.d[0] = r;
    12
}
/// $C13D24: move.l d0, $a10(a5)
pub fn i_c13d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D28;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA10);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C13D28: rts
pub fn i_c13d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13DBA: lsr.w #$7, d6
pub fn i_c13dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DBC;
    cpu.ir = 0xEE4E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $C13DBC: andi.w #$fff0, d6
pub fn i_c13dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DC0;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFFF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13DC0: lea.l $c13dc8(pc, d6.w), a0
pub fn i_c13dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DC4;
    cpu.ir = 0x41FB;
    let ad1 = 0xC13DC8_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C13DC4: bra.w $c13d0c
pub fn i_c13dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DC8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC13D0C;
    10
}
/// $C13E58: move.b d0, $3a0005.l
pub fn i_c13e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E5E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C13E5E: move.b d0, $3a0017.l
pub fn i_c13e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E64;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C13E64: rts
pub fn i_c13e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13E66: move.b d0, $3a0007.l
pub fn i_c13e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E6C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0007, v1 as u8);
    16
}
/// $C13E6C: move.b d0, $3a0015.l
pub fn i_c13e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E72;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0015, v1 as u8);
    16
}
/// $C13E72: rts
pub fn i_c13e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15A7C: move.b (a1)+, d0
pub fn i_c15a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A7E;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C15A7E: jsr $c16f54(pc)
pub fn i_c15a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15A82);
    cpu.pc = 0xC16F54;
    16
}
/// $C15A82: move.b d0, (a0)+
pub fn i_c15a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A84;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15A84: move.b d1, (a0)+
pub fn i_c15a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A86;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15A86: rts
pub fn i_c15a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15AAC: lea.l $10f300.l, a5
pub fn i_c15aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AB2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C15AB2: move.w #$7, $3c000c.l
pub fn i_c15ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ABA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C15ABA: tst.b $c00400.l
pub fn i_c15aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AC0;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xC00400) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C15AC0: beq.w $c15b9c
pub fn i_c15ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15B9C; return 10; }
    8
}
/// $C15AC4: move.l a0, $a00(a5)
pub fn i_c15ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AC8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15AC8: move.w d0, $a04(a5)
pub fn i_c15ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ACC;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C15ACC: move.w d1, $a06(a5)
pub fn i_c15acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AD0;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA06);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C15AD0: move.w d6, $a08(a5)
pub fn i_c15ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AD4;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C15AD4: move.w #$7000, $3c0000.l
pub fn i_c15ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ADC;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $C15ADC: move.b d0, $300001.l
pub fn i_c15adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AE2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15AE2: move.w #$1, $3c0004.l
pub fn i_c15ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AEA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0004, v1 as u16);
    16
}
/// $C15AEA: move.w #$f020, d0
pub fn i_c15aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AEE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15AEE: move.w #$4ff, d7
pub fn i_c15aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AF2;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x4FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15AF2: move.w d0, $3c0002.l
pub fn i_c15af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AF8;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $C15AF8: dbra d7, $c15af2
pub fn i_c15af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AFC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15AFC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15AFC; return 14; }
    cpu.pc = 0xC15AF2;
    10
}
/// $C15AFC: move.l #$eee0000, $4001e2.l
pub fn i_c15afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B06;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x4001E2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C15B06: lea.l $c00(a5), a0
pub fn i_c15b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B0A;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C15B0A: move.w $a08(a5), d0
pub fn i_c15b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B0E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C15B0E: lsl.w #$2, d0
pub fn i_c15b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B10;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C15B10: lea.l $c15a88(pc), a1
pub fn i_c15b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC15A88;
    16
}
/// $C15B14: move.l (a1, d0.w), (a0)+
pub fn i_c15b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B18;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C15B18: cmpi.w #$14, d0
pub fn i_c15b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B1C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x14, Size::Word, false);
    8
}
/// $C15B1C: bcc.w $c15b68
pub fn i_c15b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC15B68; return 10; }
    8
}
/// $C15B20: lea.l $a00(a5), a1
pub fn i_c15b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B24;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA00);
    cpu.a[1] = ad1;
    16
}
/// $C15B24: clr.l (a0)+
pub fn i_c15b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B26;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15B26: move.l #$1f0ff, (a0)+
pub fn i_c15b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B2C;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15B2C: move.w #$2002, (a0)+
pub fn i_c15b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B30;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B30: move.l #$3708d, (a0)+
pub fn i_c15b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B36;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3708D;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15B36: move.w #$7, (a0)+
pub fn i_c15b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B3A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B3A: jsr $c15a7c(pc)
pub fn i_c15b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B3E);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B3E: jsr $c15a7c(pc)
pub fn i_c15b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B42);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B42: jsr $c15a7c(pc)
pub fn i_c15b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B46);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B46: jsr $c15a7c(pc)
pub fn i_c15b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B4A);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B4A: move.w #$2020, (a0)+
pub fn i_c15b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B4E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B4E: jsr $c15a7c(pc)
pub fn i_c15b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B52);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B52: jsr $c15a7c(pc)
pub fn i_c15b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B56);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B56: move.w #$2020, (a0)+
pub fn i_c15b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B5A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B5A: jsr $c15a7c(pc)
pub fn i_c15b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B5E);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B5E: jsr $c15a7c(pc)
pub fn i_c15b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B62);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B62: move.l #$20ff0000, (a0)+
pub fn i_c15b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B68;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x20FF0000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15B68: move.l a0, $abe(a5)
pub fn i_c15b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B6C;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15B6C: clr.b $ac2(a5)
pub fn i_c15b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B70;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15B70: jsr $c17290(pc)
pub fn i_c15b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B74);
    cpu.pc = 0xC17290;
    16
}
/// $C15B74: move.w #$7, $3c000c.l
pub fn i_c15b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B7C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C15B7C: move.b d0, $300001.l
pub fn i_c15b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B82;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15B82: bra.b $c15b7c
pub fn i_c15b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B84;
    cpu.ir = 0x60F8;
    cpu.pc = 0xC15B7C;
    10
}
/// $C15B9C: add.w d6, d6
pub fn i_c15b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B9E;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C15B9E: add.w d6, d6
pub fn i_c15b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BA0;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C15BA0: move.l $c15b84(pc, d6.w), d5
pub fn i_c15ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BA4;
    cpu.ir = 0x2A3B;
    let ad2 = 0xC15B84_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C15BA4: move.b d0, $3a000f.l
pub fn i_c15ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BAA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C15BAA: lea.l $400000.l, a0
pub fn i_c15baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x400000;
    20
}
/// $C15BB0: movea.l a0, a1
pub fn i_c15bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BB2;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C15BB2: move.w #$7ff, d7
pub fn i_c15bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BB6;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15BB6: move.b d0, $300001.l
pub fn i_c15bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BBC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15BBC: move.l d5, (a0)+
pub fn i_c15bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BBE;
    cpu.ir = 0x20C5;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15BBE: dbra d7, $c15bb6
pub fn i_c15bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BC2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15BC2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15BC2; return 14; }
    cpu.pc = 0xC15BB6;
    10
}
/// $C15BC2: move.b d0, $3a001f.l
pub fn i_c15bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BC8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C15BC8: move.w #$7ff, d7
pub fn i_c15bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BCC;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15BCC: move.b d0, $300001.l
pub fn i_c15bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BD2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15BD2: move.l d5, (a1)+
pub fn i_c15bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BD4;
    cpu.ir = 0x22C5;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15BD4: dbra d7, $c15bcc
pub fn i_c15bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BD8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15BD8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15BD8; return 14; }
    cpu.pc = 0xC15BCC;
    10
}
/// $C15BD8: clr.w $400000.l
pub fn i_c15bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BDE;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x400000) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x400000, r2 as u16);
    16
}
/// $C15BDE: bra.b $c15b74
pub fn i_c15bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BE0;
    cpu.ir = 0x6094;
    cpu.pc = 0xC15B74;
    10
}
/// $C16034: move.w #$2700, sr
pub fn i_c16034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16038;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C16038: move.b d0, $3a0003.l
pub fn i_c16038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1603E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C1603E: move.b d0, $3a000b.l
pub fn i_c1603e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16044;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C16044: moveq #$0, d0
pub fn i_c16044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16046;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16046: move.l d0, d1
pub fn i_c16046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16048;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16048: move.l d0, d2
pub fn i_c16048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1604A;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1604A: move.l d0, d3
pub fn i_c1604a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1604C;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1604C: lea.l $110000.l, a0
pub fn i_c1604c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16052;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x110000;
    20
}
/// $C16052: move.w #$1ff, d7
pub fn i_c16052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16056;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16056: move.b d0, $300001.l
pub fn i_c16056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1605C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1605C: movem.l d0-d3, -(a0)
pub fn i_c1605c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16060;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16060: movem.l d0-d3, -(a0)
pub fn i_c16060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16064;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16064: movem.l d0-d3, -(a0)
pub fn i_c16064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16068;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16068: movem.l d0-d3, -(a0)
pub fn i_c16068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1606C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1606C: movem.l d0-d3, -(a0)
pub fn i_c1606c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16070;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16070: movem.l d0-d3, -(a0)
pub fn i_c16070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16074;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16074: movem.l d0-d3, -(a0)
pub fn i_c16074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16078;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16078: movem.l d0-d3, -(a0)
pub fn i_c16078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1607C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1607C: dbra d7, $c16056
pub fn i_c1607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16080;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16080; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16080; return 14; }
    cpu.pc = 0xC16056;
    10
}
/// $C16080: move.b d0, $3a000f.l
pub fn i_c16080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16086;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C16086: lea.l $402000.l, a0
pub fn i_c16086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1608C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C1608C: move.w #$7f, d7
pub fn i_c1608c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16090;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16090: movem.l d0-d3, -(a0)
pub fn i_c16090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16094;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16094: movem.l d0-d3, -(a0)
pub fn i_c16094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16098;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16098: movem.l d0-d3, -(a0)
pub fn i_c16098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1609C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1609C: movem.l d0-d3, -(a0)
pub fn i_c1609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160A0;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160A0: dbra d7, $c16090
pub fn i_c160a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160A4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC160A4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC160A4; return 14; }
    cpu.pc = 0xC16090;
    10
}
/// $C160A4: move.b d0, $300001.l
pub fn i_c160a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160AA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C160AA: move.b d0, $3a001f.l
pub fn i_c160aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160B0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C160B0: lea.l $402000.l, a0
pub fn i_c160b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C160B6: move.w #$7f, d7
pub fn i_c160b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160BA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C160BA: movem.l d0-d3, -(a0)
pub fn i_c160ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160BE;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160BE: movem.l d0-d3, -(a0)
pub fn i_c160be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160C2;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160C2: movem.l d0-d3, -(a0)
pub fn i_c160c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160C6;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160C6: movem.l d0-d3, -(a0)
pub fn i_c160c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160CA;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160CA: dbra d7, $c160ba
pub fn i_c160ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160CE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC160CE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC160CE; return 14; }
    cpu.pc = 0xC160BA;
    10
}
/// $C160CE: lea.l $10f300.l, a5
pub fn i_c160ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160D4;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C160D4: move.b #$ff, $b8d(a5)
pub fn i_c160d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160DA;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C160DA: move.b #$1, $bbc(a5)
pub fn i_c160da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160E0;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C160E0: move.b $c00401.l, $a83(a5)
pub fn i_c160e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160E8;
    cpu.ir = 0x1B79;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA83);
    bus.write8(ad3, v2 as u8);
    24
}
/// $C160E8: move.b #$80, $10fd82.l
pub fn i_c160e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160F0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD82, v1 as u8);
    16
}
/// $C160F0: move.l #$10ff00, $abe(a5)
pub fn i_c160f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160F8;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C160F8: move.b d0, $3a0001.l
pub fn i_c160f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160FE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C160FE: move.w #$7, $3c000c.l
pub fn i_c160fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16106;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C16106: move.w #$2000, sr
pub fn i_c16106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1610A;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1610A: tst.b $d0005e.l
pub fn i_c1610a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16110;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD0005E) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C16110: bmi.b $c1611c
pub fn i_c16110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16112;
    cpu.ir = 0x6B0A;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC1611C; return 10; }
    8
}
/// $C16112: move.w #$c, $9d8(a5)
pub fn i_c16112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16118;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16118: clr.w $ae4(a5)
pub fn i_c16118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1611C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1611C: jsr $c1727e(pc)
pub fn i_c1611c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16120;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16120);
    cpu.pc = 0xC1727E;
    16
}
/// $C16120: cmpi.w #$2, $9d8(a5)
pub fn i_c16120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16126;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $C16126: bcs.w $c16130
pub fn i_c16126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1612A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC16130; return 10; }
    8
}
/// $C1612A: jsr $c17f40.l
pub fn i_c1612a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC16130);
    cpu.pc = 0xC17F40;
    16
}
/// $C16130: lea.l $10f300.l, a5
pub fn i_c16130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16136;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C16136: move.w $9d8(a5), d0
pub fn i_c16136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1613A;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1613A: add.w d0, d0
pub fn i_c1613a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1613C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1613C: add.w d0, d0
pub fn i_c1613c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1613E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1613E: movea.l $c16146(pc, d0.w), a0
pub fn i_c1613e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16142;
    cpu.ir = 0x207B;
    let ad2 = 0xC16146_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C16142: jsr (a0)
pub fn i_c16142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16144;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0xC16144);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $C16144: bra.b $c1611c
pub fn i_c16144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16146;
    cpu.ir = 0x60D6;
    cpu.pc = 0xC1611C;
    10
}
/// $C169B0: move.b #$3, $320000.l
pub fn i_c169b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169B8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C169B8: lea.l $10f300.l, a5
pub fn i_c169b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169BE;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C169BE: moveq #$0, d0
pub fn i_c169be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169C0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C169C0: move.w d0, $9d8(a5)
pub fn i_c169c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169C4;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C169C4: move.w d0, $9e6(a5)
pub fn i_c169c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169C8;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C169C8: move.b d0, $3a0001.l
pub fn i_c169c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169CE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C169CE: andi.w #$f0ff, sr
pub fn i_c169ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169D2;
    cpu.ir = 0x27C;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let h1 = cpu.sr.0 & 0xF0FF;
    write_sr(cpu, h1);
    20
}
/// $C169D2: jsr $c1727e(pc)
pub fn i_c169d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC169D6);
    cpu.pc = 0xC1727E;
    16
}
/// $C169D6: addq.b #$1, $ac2(a5)
pub fn i_c169d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169DA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C169DA: movea.l $abe(a5), a0
pub fn i_c169da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169DE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C169DE: move.l #$c16a7a, (a0)+
pub fn i_c169de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169E4;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC16A7A;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C169E4: move.l a0, $abe(a5)
pub fn i_c169e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169E8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C169E8: subq.b #$1, $ac2(a5)
pub fn i_c169e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169EC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C169EC: jsr $c1727e(pc)
pub fn i_c169ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC169F0);
    cpu.pc = 0xC1727E;
    16
}
/// $C169F0: lea.l $10f300.l, a5
pub fn i_c169f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169F6;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C169F6: tst.b $a82(a5)
pub fn i_c169f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169FA;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C169FA: beq.b $c16a0e
pub fn i_c169fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169FC;
    cpu.ir = 0x6712;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16A0E; return 10; }
    8
}
/// $C169FC: tst.w $d00034.l
pub fn i_c169fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A02;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C16A02: beq.b $c16a20
pub fn i_c16a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A04;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16A20; return 10; }
    8
}
/// $C16A04: bra.w $c16e20
pub fn i_c16a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A08;
    cpu.ir = 0x6000;
    cpu.pc = 0xC16E20;
    10
}
/// $C16A08: jmp $c102d2.l
pub fn i_c16a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A0E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C16A0E: move.b $beb(a5), d0
pub fn i_c16a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A12;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16A12: andi.b #$f0, d0
pub fn i_c16a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A16;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16A16: bne.b $c16a08
pub fn i_c16a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A18;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16A08; return 10; }
    8
}
/// $C16A18: btst.b #$0, $aad(a5)
pub fn i_c16a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A1E;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xAAD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C16A1E: bne.b $c16a08
pub fn i_c16a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A20;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16A08; return 10; }
    8
}
/// $C16A20: tst.w $9e6(a5)
pub fn i_c16a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A24;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C16A24: beq.b $c16a44
pub fn i_c16a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A26;
    cpu.ir = 0x671E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16A44; return 10; }
    8
}
/// $C16A26: movea.l $9e8(a5), a0
pub fn i_c16a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A2A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16A2A: lea.l $4001e2.l, a1
pub fn i_c16a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A30;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4001E2;
    20
}
/// $C16A30: moveq #$4, d6
pub fn i_c16a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A32;
    cpu.ir = 0x7C04;
    cpu.d[6] = 0x4;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A32: move.w (a0)+, (a1)+
pub fn i_c16a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A34;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C16A34: dbra d6, $c16a32
pub fn i_c16a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A38;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16A38; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16A38; return 14; }
    cpu.pc = 0xC16A32;
    10
}
/// $C16A38: move.w (a0), $401ffe.l
pub fn i_c16a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A3E;
    cpu.ir = 0x33D0;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x401FFE, v2 as u16);
    20
}
/// $C16A3E: clr.w $10fce6.l
pub fn i_c16a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A44;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10FCE6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10FCE6, r2 as u16);
    16
}
/// $C16A44: move.w $9d8(a5), d0
pub fn i_c16a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A48;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16A48: lsl.w #$2, d0
pub fn i_c16a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A4A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C16A4A: movea.l $c16a52(pc, d0.w), a0
pub fn i_c16a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A4E;
    cpu.ir = 0x207B;
    let ad2 = 0xC16A52_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C16A4E: jsr (a0)
pub fn i_c16a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A50;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0xC16A50);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $C16A50: bra.b $c169d6
pub fn i_c16a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A52;
    cpu.ir = 0x6084;
    cpu.pc = 0xC169D6;
    10
}
/// $C16A98: moveq #$0, d0
pub fn i_c16a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A9A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A9A: move.l d0, d1
pub fn i_c16a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A9C;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A9C: move.l d0, d2
pub fn i_c16a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A9E;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A9E: move.l d0, d3
pub fn i_c16a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AA0;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16AA0: move.b d0, $3a000f.l
pub fn i_c16aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AA6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C16AA6: move.w #$7f, d7
pub fn i_c16aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AAA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16AAA: lea.l $402000.l, a0
pub fn i_c16aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C16AB0: movem.l d0-d3, -(a0)
pub fn i_c16ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AB4;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AB4: movem.l d0-d3, -(a0)
pub fn i_c16ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AB8;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AB8: movem.l d0-d3, -(a0)
pub fn i_c16ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ABC;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16ABC: movem.l d0-d3, -(a0)
pub fn i_c16abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AC0;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AC0: dbra d7, $c16ab0
pub fn i_c16ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AC4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16AC4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16AC4; return 14; }
    cpu.pc = 0xC16AB0;
    10
}
/// $C16AC4: move.b d0, $3a001f.l
pub fn i_c16ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ACA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C16ACA: move.w #$7f, d7
pub fn i_c16aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ACE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16ACE: lea.l $402000.l, a0
pub fn i_c16ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C16AD4: movem.l d0-d3, -(a0)
pub fn i_c16ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AD8;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AD8: movem.l d0-d3, -(a0)
pub fn i_c16ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ADC;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16ADC: movem.l d0-d3, -(a0)
pub fn i_c16adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AE0;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AE0: movem.l d0-d3, -(a0)
pub fn i_c16ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AE4;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AE4: dbra d7, $c16ad4
pub fn i_c16ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AE8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16AE8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16AE8; return 14; }
    cpu.pc = 0xC16AD4;
    10
}
/// $C16AE8: addq.b #$1, $ac2(a5)
pub fn i_c16ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AEC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16AEC: jsr $c16ed0(pc)
pub fn i_c16aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16AF0);
    cpu.pc = 0xC16ED0;
    16
}
/// $C16AF0: jsr $c16f0c(pc)
pub fn i_c16af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16AF4);
    cpu.pc = 0xC16F0C;
    16
}
/// $C16AF4: move.l #$ddd0000, $400002.l
pub fn i_c16af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AFE;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xDDD0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16AFE: lea.l $c1fa7e.l, a0
pub fn i_c16afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1FA7E;
    20
}
/// $C16B04: move.w #$40, d0
pub fn i_c16b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B08;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B08: move.b $115.w, d2
pub fn i_c16b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B0C;
    cpu.ir = 0x1438;
    let v2 = bus.read8(0x115) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    12
}
/// $C16B0C: lsl.w #$8, d2
pub fn i_c16b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B0E;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C16B0E: move.w #$f01, d1
pub fn i_c16b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B12;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF01;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B12: jsr $c16e7c(pc)
pub fn i_c16b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16B16);
    cpu.pc = 0xC16E7C;
    16
}
/// $C16B16: lea.l $c1fac6.l, a0
pub fn i_c16b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1FAC6;
    20
}
/// $C16B1C: move.w #$4c0, d0
pub fn i_c16b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B20: move.w #$f00, d1
pub fn i_c16b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B24;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B24: jsr $c16e7c(pc)
pub fn i_c16b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16B28);
    cpu.pc = 0xC16E7C;
    16
}
/// $C16B28: move.w #$0, -$2(a1)
pub fn i_c16b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B2E;
    cpu.ir = 0x337C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B2E: move.w #$2, $2(a1)
pub fn i_c16b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B34;
    cpu.ir = 0x337C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B34: moveq #$1f, d6
pub fn i_c16b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B36;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B36: move.w d2, (a1)
pub fn i_c16b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B38;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16B38: dbra d6, $c16b36
pub fn i_c16b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B3C;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B3C; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B3C; return 14; }
    cpu.pc = 0xC16B36;
    10
}
/// $C16B3C: movea.l $abe(a5), a0
pub fn i_c16b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B40;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16B40: move.l #$c1fb0e, (a0)+
pub fn i_c16b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B46;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1FB0E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16B46: move.l a0, $abe(a5)
pub fn i_c16b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B4A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16B4A: lea.l $948(a5), a0
pub fn i_c16b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B4E;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x948);
    cpu.a[0] = ad1;
    16
}
/// $C16B4E: move.w #$e404, (a0)+
pub fn i_c16b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B52;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xE404;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B52: moveq #$44, d0
pub fn i_c16b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B54;
    cpu.ir = 0x7044;
    cpu.d[0] = 0x44;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B54: moveq #$22, d7
pub fn i_c16b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B56;
    cpu.ir = 0x7E22;
    cpu.d[7] = 0x22;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B56: move.w d0, (a0)+
pub fn i_c16b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B58;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B58: dbra d7, $c16b56
pub fn i_c16b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B5C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B5C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B5C; return 14; }
    cpu.pc = 0xC16B56;
    10
}
/// $C16B5C: clr.w $96c(a5)
pub fn i_c16b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B60;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x96C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16B60: move.w #$4b80, $900(a5)
pub fn i_c16b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B66;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x4B80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B66: move.w #$ff, d0
pub fn i_c16b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B6A: lea.l $990(a5), a0
pub fn i_c16b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B6E;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x990);
    cpu.a[0] = ad1;
    16
}
/// $C16B6E: moveq #$11, d7
pub fn i_c16b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B70;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B70: move.w d0, (a0)+
pub fn i_c16b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B72;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B72: dbra d7, $c16b70
pub fn i_c16b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B76;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B76; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B76; return 14; }
    cpu.pc = 0xC16B70;
    10
}
/// $C16B76: move.w #$fff, d0
pub fn i_c16b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B7A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B7A: moveq #$11, d7
pub fn i_c16b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B7C;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B7C: move.w d0, (a0)+
pub fn i_c16b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B7E;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B7E: dbra d7, $c16b7c
pub fn i_c16b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B82;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B82; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B82; return 14; }
    cpu.pc = 0xC16B7C;
    10
}
/// $C16B82: move.w #$0, d0
pub fn i_c16b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B86;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B86: bsr.w $c16eb8
pub fn i_c16b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B8A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC16B8A);
    cpu.pc = 0xC16EB8;
    18
}
/// $C16B8A: move.w #$0, $9dc(a5)
pub fn i_c16b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B90;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B90: move.w #$1, $9d8(a5)
pub fn i_c16b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B96;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B96: subq.b #$1, $ac2(a5)
pub fn i_c16b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B9A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16B9A: rts
pub fn i_c16b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16B9C: addq.w #$1, $9dc(a5)
pub fn i_c16b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BA0;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16BA0: move.w $9dc(a5), d0
pub fn i_c16ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BA4;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16BA4: cmpi.w #$5, d0
pub fn i_c16ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BA8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C16BA8: bne.b $c16bc0
pub fn i_c16ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BAA;
    cpu.ir = 0x6616;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BC0; return 10; }
    8
}
/// $C16BAA: tst.b $a82(a5)
pub fn i_c16baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BAE;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C16BAE: beq.b $c16bb8
pub fn i_c16bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BB0;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16BB8; return 10; }
    8
}
/// $C16BB0: tst.b $d00046.l
pub fn i_c16bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BB6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00046) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C16BB6: bne.b $c16bc0
pub fn i_c16bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BB8;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BC0; return 10; }
    8
}
/// $C16BB8: move.b #$2, $320000.l
pub fn i_c16bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BC0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C16BC0: andi.w #$1, d0
pub fn i_c16bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BC4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16BC4: bne.b $c16bfc
pub fn i_c16bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BC6;
    cpu.ir = 0x6636;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BFC; return 10; }
    8
}
/// $C16BC6: lea.l $990(a5), a0
pub fn i_c16bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BCA;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x990);
    cpu.a[0] = ad1;
    16
}
/// $C16BCA: move.w $900(a5), d1
pub fn i_c16bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BCE;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16BCE: subi.w #$480, d1
pub fn i_c16bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BD2;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x480, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16BD2: move.w d1, $900(a5)
pub fn i_c16bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BD6;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BD6: move.w (a0), d0
pub fn i_c16bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BD8;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C16BD8: addi.w #$100, d0
pub fn i_c16bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BDC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16BDC: moveq #$11, d7
pub fn i_c16bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BDE;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16BDE: move.w d0, (a0)+
pub fn i_c16bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BE0;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16BE0: dbra d7, $c16bde
pub fn i_c16be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BE4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16BE4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16BE4; return 14; }
    cpu.pc = 0xC16BDE;
    10
}
/// $C16BE4: cmpi.w #$fff, d0
pub fn i_c16be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BE8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF, Size::Word, false);
    8
}
/// $C16BE8: bne.b $c16bfc
pub fn i_c16be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BEA;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BFC; return 10; }
    8
}
/// $C16BEA: move.w #$0, $9da(a5)
pub fn i_c16bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BF0;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BF0: move.w #$a, $9dc(a5)
pub fn i_c16bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BF6;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BF6: move.w #$2, $9d8(a5)
pub fn i_c16bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BFC;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BFC: rts
pub fn i_c16bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16BFE: subq.w #$1, $9dc(a5)
pub fn i_c16bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C02;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16C02: bne.b $c16c0a
pub fn i_c16c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C04;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16C0A; return 10; }
    8
}
/// $C16C04: move.w #$3, $9d8(a5)
pub fn i_c16c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C0A;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C0A: rts
pub fn i_c16c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16C0C: addq.w #$1, $9dc(a5)
pub fn i_c16c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C10;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16C10: move.w $9dc(a5), d0
pub fn i_c16c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C14;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16C14: andi.w #$1, d0
pub fn i_c16c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C18;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16C18: bne.b $c16c64
pub fn i_c16c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C1A;
    cpu.ir = 0x664A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16C64; return 10; }
    8
}
/// $C16C1A: jsr $c16e62(pc)
pub fn i_c16c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16C1E);
    cpu.pc = 0xC16E62;
    16
}
/// $C16C1E: subi.w #$100, $948(a5)
pub fn i_c16c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C24;
    cpu.ir = 0x46D;
    let ad1 = cpu.a[5].wrapping_add(0x948);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $C16C24: subi.b #$10, $991(a5)
pub fn i_c16c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C2A;
    cpu.ir = 0x42D;
    let ad1 = cpu.a[5].wrapping_add(0x991);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $C16C2A: bcc.b $c16c64
pub fn i_c16c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C2C;
    cpu.ir = 0x6438;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC16C64; return 10; }
    8
}
/// $C16C2C: move.w #$f00, $9b4(a5)
pub fn i_c16c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C32;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9B4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C32: move.w $948(a5), $96c(a5)
pub fn i_c16c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C38;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x948);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x96C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C16C38: move.w #$6404, $948(a5)
pub fn i_c16c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C3E;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x6404;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x948);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C3E: move.w #$800, $924(a5)
pub fn i_c16c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C44;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x924);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C44: move.w #$4, $9d8(a5)
pub fn i_c16c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C4A;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C4A: move.w #$0, $9dc(a5)
pub fn i_c16c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C50;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C50: move.w #$0, $9de(a5)
pub fn i_c16c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C56;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C56: move.w #$71b0, $9e0(a5)
pub fn i_c16c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C5C;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x71B0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C5C: move.l #$c1fb4a, $9e2(a5)
pub fn i_c16c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C64;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0xC1FB4A;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16C64: rts
pub fn i_c16c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16C66: addq.w #$1, $9dc(a5)
pub fn i_c16c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C6A;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16C6A: move.w $9dc(a5), d0
pub fn i_c16c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C6E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16C6E: andi.w #$1, d0
pub fn i_c16c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C72;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16C72: bne.b $c16ca2
pub fn i_c16c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C74;
    cpu.ir = 0x662E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16CA2; return 10; }
    8
}
/// $C16C74: jsr $c16e62(pc)
pub fn i_c16c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16C78);
    cpu.pc = 0xC16E62;
    16
}
/// $C16C78: addi.w #$100, $96c(a5)
pub fn i_c16c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C7E;
    cpu.ir = 0x66D;
    let ad1 = cpu.a[5].wrapping_add(0x96C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $C16C7E: move.b $9b5(a5), d0
pub fn i_c16c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C82;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0x9B5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16C82: bne.b $c16c88
pub fn i_c16c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C84;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16C88; return 10; }
    8
}
/// $C16C84: move.b #$ff, d0
pub fn i_c16c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C88;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C16C88: addi.b #$10, d0
pub fn i_c16c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C8C;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16C8C: move.b d0, $9b5(a5)
pub fn i_c16c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C90;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9B5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16C90: cmpi.b #$ff, d0
pub fn i_c16c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C94;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C16C94: bne.b $c16ca2
pub fn i_c16c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C96;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16CA2; return 10; }
    8
}
/// $C16C96: move.w #$0, $9dc(a5)
pub fn i_c16c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C9C;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C9C: move.w #$5, $9d8(a5)
pub fn i_c16c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CA2;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16CA2: rts
pub fn i_c16ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16CA4: addq.w #$1, $9dc(a5)
pub fn i_c16ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CA8;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16CA8: move.w $9dc(a5), d0
pub fn i_c16ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CAC;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16CAC: andi.w #$3, d0
pub fn i_c16cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CB0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16CB0: bne.w $c16d1c
pub fn i_c16cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D1C; return 10; }
    8
}
/// $C16CB4: addq.b #$1, $ac2(a5)
pub fn i_c16cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CB8;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16CB8: movea.l $abe(a5), a0
pub fn i_c16cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CBC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16CBC: move.l #$0, (a0)+
pub fn i_c16cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CC2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16CC2: move.l #$3010002, (a0)+
pub fn i_c16cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CC8;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3010002;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16CC8: move.w #$102, (a0)+
pub fn i_c16cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CCC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x102;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CCC: move.w #$3, (a0)+
pub fn i_c16ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CD0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CD0: move.w $9e0(a5), (a0)+
pub fn i_c16cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CD4;
    cpu.ir = 0x30ED;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C16CD4: move.w #$4, (a0)+
pub fn i_c16cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CD8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CD8: move.l $9e2(a5), (a0)+
pub fn i_c16cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CDC;
    cpu.ir = 0x20ED;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C16CDC: move.w #$0, (a0)+
pub fn i_c16cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CE0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CE0: move.l a0, $abe(a5)
pub fn i_c16ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CE4;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16CE4: subq.b #$1, $ac2(a5)
pub fn i_c16ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CE8;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16CE8: addq.l #$4, $9e2(a5)
pub fn i_c16ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CEC;
    cpu.ir = 0x58AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 4, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C16CEC: addi.w #$20, $9e0(a5)
pub fn i_c16cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CF2;
    cpu.ir = 0x66D;
    let ad1 = cpu.a[5].wrapping_add(0x9E0);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $C16CF2: addq.w #$1, $9de(a5)
pub fn i_c16cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CF6;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DE);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16CF6: move.w $9de(a5), d0
pub fn i_c16cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CFA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16CFA: cmpi.w #$f, d0
pub fn i_c16cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CFE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF, Size::Word, false);
    8
}
/// $C16CFE: bne.b $c16d06
pub fn i_c16cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D00;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D06; return 10; }
    8
}
/// $C16D00: move.w #$7192, $9e0(a5)
pub fn i_c16d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D06;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x7192;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D06: move.w $9de(a5), d0
pub fn i_c16d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D0A;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16D0A: cmpi.w #$20, d0
pub fn i_c16d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D0E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C16D0E: bne.b $c16d1c
pub fn i_c16d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D10;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D1C; return 10; }
    8
}
/// $C16D10: move.w #$6, $9d8(a5)
pub fn i_c16d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D16;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D16: move.w #$14, $9dc(a5)
pub fn i_c16d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D1C;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D1C: rts
pub fn i_c16d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16E20: cmpi.w #$9, $9d8(a5)
pub fn i_c16e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E26;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x9, Size::Word, false);
    16
}
/// $C16E26: bne.w $c16e5c
pub fn i_c16e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16E5C; return 10; }
    8
}
/// $C16E2A: tst.b $a83(a5)
pub fn i_c16e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E2E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C16E2E: beq.w $c16e5c
pub fn i_c16e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16E5C; return 10; }
    8
}
/// $C16E32: clr.l $400002.l
pub fn i_c16e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E38;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x400002);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x400002, r2);
    20
}
/// $C16E38: jsr $c12a0e(pc)
pub fn i_c16e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E3C);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16E3C: jsr $c10fae(pc)
pub fn i_c16e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E40);
    cpu.pc = 0xC10FAE;
    16
}
/// $C16E40: jsr $c1727e(pc)
pub fn i_c16e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E44);
    cpu.pc = 0xC1727E;
    16
}
/// $C16E44: move.b #$3, $320000.l
pub fn i_c16e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E4C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C16E4C: move.b d0, $3a001b.l
pub fn i_c16e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E52;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C16E52: move.b d0, $3a0013.l
pub fn i_c16e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E58;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C16E58: clr.b $b8d(a5)
pub fn i_c16e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E5C;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xB8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16E5C: jmp $c102d2.l
pub fn i_c16e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E62;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C16E62: move.w $9da(a5), d1
pub fn i_c16e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E66;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16E66: lea.l $c1f80e.l, a0
pub fn i_c16e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1F80E;
    20
}
/// $C16E6C: moveq #$0, d0
pub fn i_c16e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E6E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E6E: move.b (a0, d1.w), d0
pub fn i_c16e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E72;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C16E72: jsr $c16eb8(pc)
pub fn i_c16e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E76);
    cpu.pc = 0xC16EB8;
    16
}
/// $C16E76: addq.w #$1, $9da(a5)
pub fn i_c16e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E7A;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DA);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16E7A: rts
pub fn i_c16e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16E7C: lea.l $3c0002.l, a1
pub fn i_c16e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E82;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C16E82: move.w #$1, $2(a1)
pub fn i_c16e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E88;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16E88: moveq #$11, d7
pub fn i_c16e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E8A;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E8A: move.w d0, -$2(a1)
pub fn i_c16e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E8E;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16E8E: moveq #$3, d6
pub fn i_c16e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E90;
    cpu.ir = 0x7C03;
    cpu.d[6] = 0x3;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E90: move.b (a0)+, d2
pub fn i_c16e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E92;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C16E92: move.w d2, (a1)
pub fn i_c16e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E94;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16E94: nop
pub fn i_c16e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E96;
    cpu.ir = 0x4E71;
    4
}
/// $C16E96: move.w d1, (a1)
pub fn i_c16e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E98;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16E98: dbra d6, $c16e90
pub fn i_c16e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E9C;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16E9C; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16E9C; return 14; }
    cpu.pc = 0xC16E90;
    10
}
/// $C16E9C: moveq #$1b, d6
pub fn i_c16e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E9E;
    cpu.ir = 0x7C1B;
    cpu.d[6] = 0x1B;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E9E: move.b #$ff, d2
pub fn i_c16e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA2;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C16EA2: move.w d2, (a1)
pub fn i_c16ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA4;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16EA4: nop
pub fn i_c16ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA6;
    cpu.ir = 0x4E71;
    4
}
/// $C16EA6: move.w #$0, (a1)
pub fn i_c16ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EAA;
    cpu.ir = 0x32BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16EAA: dbra d6, $c16ea2
pub fn i_c16eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EAE;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16EAE; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16EAE; return 14; }
    cpu.pc = 0xC16EA2;
    10
}
/// $C16EAE: addi.w #$40, d0
pub fn i_c16eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16EB2: dbra d7, $c16e8a
pub fn i_c16eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16EB6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16EB6; return 14; }
    cpu.pc = 0xC16E8A;
    10
}
/// $C16EB6: rts
pub fn i_c16eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16EB8: move.w #$c, d1
pub fn i_c16eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EBC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EBC: mulu.w d1, d0
pub fn i_c16ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EBE;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C16EBE: addi.l #$c1f83e, d0
pub fn i_c16ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EC4;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC1F83E, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C16EC4: move.l d0, $9e8(a5)
pub fn i_c16ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EC8;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC1194E, i_c1194e),
    (0xC11952, i_c11952),
    (0xC11956, i_c11956),
    (0xC11958, i_c11958),
    (0xC1195C, i_c1195c),
    (0xC11960, i_c11960),
    (0xC11962, i_c11962),
    (0xC11990, i_c11990),
    (0xC11994, i_c11994),
    (0xC11996, i_c11996),
    (0xC11998, i_c11998),
    (0xC1199C, i_c1199c),
    (0xC1199E, i_c1199e),
    (0xC119A2, i_c119a2),
    (0xC119A6, i_c119a6),
    (0xC119AC, i_c119ac),
    (0xC119B0, i_c119b0),
    (0xC119B4, i_c119b4),
    (0xC119B6, i_c119b6),
    (0xC119BA, i_c119ba),
    (0xC119BC, i_c119bc),
    (0xC119C0, i_c119c0),
    (0xC119C4, i_c119c4),
    (0xC119C6, i_c119c6),
    (0xC119CE, i_c119ce),
    (0xC119D0, i_c119d0),
    (0xC119D4, i_c119d4),
    (0xC119DA, i_c119da),
    (0xC119E0, i_c119e0),
    (0xC119E2, i_c119e2),
    (0xC119E8, i_c119e8),
    (0xC119EA, i_c119ea),
    (0xC119F0, i_c119f0),
    (0xC119F2, i_c119f2),
    (0xC119F8, i_c119f8),
    (0xC119FC, i_c119fc),
    (0xC119FE, i_c119fe),
    (0xC11A00, i_c11a00),
    (0xC11A06, i_c11a06),
    (0xC11A08, i_c11a08),
    (0xC11A0E, i_c11a0e),
    (0xC11A14, i_c11a14),
    (0xC11A18, i_c11a18),
    (0xC11A1C, i_c11a1c),
    (0xC11A1E, i_c11a1e),
    (0xC11A20, i_c11a20),
    (0xC11A22, i_c11a22),
    (0xC11A24, i_c11a24),
    (0xC11A26, i_c11a26),
    (0xC11A2A, i_c11a2a),
    (0xC11A2E, i_c11a2e),
    (0xC11A32, i_c11a32),
    (0xC11A36, i_c11a36),
    (0xC11A38, i_c11a38),
    (0xC11A3C, i_c11a3c),
    (0xC11A40, i_c11a40),
    (0xC11A42, i_c11a42),
    (0xC11A46, i_c11a46),
    (0xC11A4A, i_c11a4a),
    (0xC11A4E, i_c11a4e),
    (0xC11A50, i_c11a50),
    (0xC11A54, i_c11a54),
    (0xC11A58, i_c11a58),
    (0xC11A5C, i_c11a5c),
    (0xC11A5E, i_c11a5e),
    (0xC11A62, i_c11a62),
    (0xC11A64, i_c11a64),
    (0xC11A6A, i_c11a6a),
    (0xC11A70, i_c11a70),
    (0xC11A76, i_c11a76),
    (0xC11A7C, i_c11a7c),
    (0xC11A82, i_c11a82),
    (0xC11A86, i_c11a86),
    (0xC11A8C, i_c11a8c),
    (0xC11A8E, i_c11a8e),
    (0xC11A90, i_c11a90),
    (0xC11A94, i_c11a94),
    (0xC11A98, i_c11a98),
    (0xC11A9A, i_c11a9a),
    (0xC11A9E, i_c11a9e),
    (0xC11AA4, i_c11aa4),
    (0xC11AAA, i_c11aaa),
    (0xC11AB0, i_c11ab0),
    (0xC11AB6, i_c11ab6),
    (0xC11ABA, i_c11aba),
    (0xC11ABC, i_c11abc),
    (0xC11AC0, i_c11ac0),
    (0xC11AC2, i_c11ac2),
    (0xC11AC8, i_c11ac8),
    (0xC11ACC, i_c11acc),
    (0xC11ACE, i_c11ace),
    (0xC11AD2, i_c11ad2),
    (0xC11AD6, i_c11ad6),
    (0xC11ADA, i_c11ada),
    (0xC11ADE, i_c11ade),
    (0xC11AE4, i_c11ae4),
    (0xC11AE8, i_c11ae8),
    (0xC11AEC, i_c11aec),
    (0xC11AF2, i_c11af2),
    (0xC11AF6, i_c11af6),
    (0xC11AFA, i_c11afa),
    (0xC11B00, i_c11b00),
    (0xC11B04, i_c11b04),
    (0xC11B08, i_c11b08),
    (0xC11B0E, i_c11b0e),
    (0xC11B14, i_c11b14),
    (0xC11B1E, i_c11b1e),
    (0xC11B22, i_c11b22),
    (0xC11B26, i_c11b26),
    (0xC11B2A, i_c11b2a),
    (0xC11B2E, i_c11b2e),
    (0xC11B30, i_c11b30),
    (0xC11B34, i_c11b34),
    (0xC11B36, i_c11b36),
    (0xC11B3A, i_c11b3a),
    (0xC11B3E, i_c11b3e),
    (0xC11B42, i_c11b42),
    (0xC11B48, i_c11b48),
    (0xC11B4E, i_c11b4e),
    (0xC11B50, i_c11b50),
    (0xC11B52, i_c11b52),
    (0xC11B58, i_c11b58),
    (0xC11B5A, i_c11b5a),
    (0xC11B5C, i_c11b5c),
    (0xC11B5E, i_c11b5e),
    (0xC11B60, i_c11b60),
    (0xC11BB0, i_c11bb0),
    (0xC11BB6, i_c11bb6),
    (0xC11BBC, i_c11bbc),
    (0xC11BC2, i_c11bc2),
    (0xC11BC8, i_c11bc8),
    (0xC11BCA, i_c11bca),
    (0xC11BD0, i_c11bd0),
    (0xC11BD2, i_c11bd2),
    (0xC11BD6, i_c11bd6),
    (0xC11BD8, i_c11bd8),
    (0xC11BDE, i_c11bde),
    (0xC11BE2, i_c11be2),
    (0xC11BE8, i_c11be8),
    (0xC11BEC, i_c11bec),
    (0xC11BEE, i_c11bee),
    (0xC11BF4, i_c11bf4),
    (0xC11BFA, i_c11bfa),
    (0xC11C00, i_c11c00),
    (0xC11C04, i_c11c04),
    (0xC11C08, i_c11c08),
    (0xC11C0C, i_c11c0c),
    (0xC11C10, i_c11c10),
    (0xC11C14, i_c11c14),
    (0xC11C18, i_c11c18),
    (0xC11C1C, i_c11c1c),
    (0xC11C20, i_c11c20),
    (0xC11C24, i_c11c24),
    (0xC11C28, i_c11c28),
    (0xC11C2C, i_c11c2c),
    (0xC11C30, i_c11c30),
    (0xC11C36, i_c11c36),
    (0xC11C3A, i_c11c3a),
    (0xC11C40, i_c11c40),
    (0xC11C42, i_c11c42),
    (0xC11C44, i_c11c44),
    (0xC11C48, i_c11c48),
    (0xC11C4C, i_c11c4c),
    (0xC11C4E, i_c11c4e),
    (0xC11C52, i_c11c52),
    (0xC11C56, i_c11c56),
    (0xC11C5C, i_c11c5c),
    (0xC11C60, i_c11c60),
    (0xC11C64, i_c11c64),
    (0xC11C68, i_c11c68),
    (0xC11C6C, i_c11c6c),
    (0xC11C70, i_c11c70),
    (0xC11C74, i_c11c74),
    (0xC11C78, i_c11c78),
    (0xC11C7C, i_c11c7c),
    (0xC11C80, i_c11c80),
    (0xC11C82, i_c11c82),
    (0xC11C86, i_c11c86),
    (0xC11C8A, i_c11c8a),
    (0xC11C8C, i_c11c8c),
    (0xC11C90, i_c11c90),
    (0xC11C92, i_c11c92),
    (0xC11C94, i_c11c94),
    (0xC11C98, i_c11c98),
    (0xC11C9C, i_c11c9c),
    (0xC11C9E, i_c11c9e),
    (0xC11CA0, i_c11ca0),
    (0xC11CA4, i_c11ca4),
    (0xC11CA6, i_c11ca6),
    (0xC11CA8, i_c11ca8),
    (0xC11CAE, i_c11cae),
    (0xC11CB2, i_c11cb2),
    (0xC11CB6, i_c11cb6),
    (0xC11CB8, i_c11cb8),
    (0xC11CBA, i_c11cba),
    (0xC11CBE, i_c11cbe),
    (0xC11CC0, i_c11cc0),
    (0xC11CC6, i_c11cc6),
    (0xC11CCA, i_c11cca),
    (0xC11CCC, i_c11ccc),
    (0xC11CD0, i_c11cd0),
    (0xC11CD2, i_c11cd2),
    (0xC11CD6, i_c11cd6),
    (0xC11CDA, i_c11cda),
    (0xC11CDE, i_c11cde),
    (0xC11CE2, i_c11ce2),
    (0xC11CE6, i_c11ce6),
    (0xC11CEA, i_c11cea),
    (0xC11CEE, i_c11cee),
    (0xC11CF2, i_c11cf2),
    (0xC11CF6, i_c11cf6),
    (0xC11CFA, i_c11cfa),
    (0xC11CFE, i_c11cfe),
    (0xC11D02, i_c11d02),
    (0xC11D06, i_c11d06),
    (0xC11D08, i_c11d08),
    (0xC11D0C, i_c11d0c),
    (0xC11D0E, i_c11d0e),
    (0xC11D12, i_c11d12),
    (0xC11D14, i_c11d14),
    (0xC11D16, i_c11d16),
    (0xC11D18, i_c11d18),
    (0xC11D1A, i_c11d1a),
    (0xC11D1E, i_c11d1e),
    (0xC11D22, i_c11d22),
    (0xC11D26, i_c11d26),
    (0xC11D2A, i_c11d2a),
    (0xC11D2E, i_c11d2e),
    (0xC11D32, i_c11d32),
    (0xC11D34, i_c11d34),
    (0xC11D38, i_c11d38),
    (0xC11D3A, i_c11d3a),
    (0xC11D3C, i_c11d3c),
    (0xC11D3E, i_c11d3e),
    (0xC11D42, i_c11d42),
    (0xC11D46, i_c11d46),
    (0xC11D4A, i_c11d4a),
    (0xC11D4E, i_c11d4e),
    (0xC11D52, i_c11d52),
    (0xC11D56, i_c11d56),
    (0xC11D58, i_c11d58),
    (0xC11D5C, i_c11d5c),
    (0xC11D60, i_c11d60),
    (0xC11D64, i_c11d64),
    (0xC11D66, i_c11d66),
    (0xC11D6A, i_c11d6a),
    (0xC11D6C, i_c11d6c),
    (0xC11D70, i_c11d70),
    (0xC11D74, i_c11d74),
    (0xC11D78, i_c11d78),
    (0xC11D7C, i_c11d7c),
    (0xC11D7E, i_c11d7e),
    (0xC11D84, i_c11d84),
    (0xC11D86, i_c11d86),
    (0xC11D88, i_c11d88),
    (0xC11D8C, i_c11d8c),
    (0xC11D90, i_c11d90),
    (0xC11D92, i_c11d92),
    (0xC11D96, i_c11d96),
    (0xC11D9A, i_c11d9a),
    (0xC11DA0, i_c11da0),
    (0xC11DA4, i_c11da4),
    (0xC11DA8, i_c11da8),
    (0xC11DAC, i_c11dac),
    (0xC11DB0, i_c11db0),
    (0xC11DB4, i_c11db4),
    (0xC11DB8, i_c11db8),
    (0xC11DBC, i_c11dbc),
    (0xC11DC0, i_c11dc0),
    (0xC11DC4, i_c11dc4),
    (0xC11DC6, i_c11dc6),
    (0xC11DCA, i_c11dca),
    (0xC11DCE, i_c11dce),
    (0xC11DD0, i_c11dd0),
    (0xC11DD4, i_c11dd4),
    (0xC11DD6, i_c11dd6),
    (0xC11DD8, i_c11dd8),
    (0xC11DDC, i_c11ddc),
    (0xC11DE0, i_c11de0),
    (0xC11DE2, i_c11de2),
    (0xC11DE4, i_c11de4),
    (0xC11DE8, i_c11de8),
    (0xC11DEA, i_c11dea),
    (0xC11DEC, i_c11dec),
    (0xC11DF2, i_c11df2),
    (0xC11DF6, i_c11df6),
    (0xC11DFA, i_c11dfa),
    (0xC11DFC, i_c11dfc),
    (0xC11DFE, i_c11dfe),
    (0xC11E02, i_c11e02),
    (0xC11E7E, i_c11e7e),
    (0xC11E80, i_c11e80),
    (0xC11E82, i_c11e82),
    (0xC11E86, i_c11e86),
    (0xC11E8A, i_c11e8a),
    (0xC11E8E, i_c11e8e),
    (0xC11E92, i_c11e92),
    (0xC11E96, i_c11e96),
    (0xC11E9A, i_c11e9a),
    (0xC11E9C, i_c11e9c),
    (0xC11EA0, i_c11ea0),
    (0xC11EA4, i_c11ea4),
    (0xC11EA8, i_c11ea8),
    (0xC11EAA, i_c11eaa),
    (0xC11EAE, i_c11eae),
    (0xC11EB0, i_c11eb0),
    (0xC11EB4, i_c11eb4),
    (0xC11EB8, i_c11eb8),
    (0xC11EBC, i_c11ebc),
    (0xC11EC0, i_c11ec0),
    (0xC11EC2, i_c11ec2),
    (0xC11EC8, i_c11ec8),
    (0xC11ECA, i_c11eca),
    (0xC11ECE, i_c11ece),
    (0xC11ED2, i_c11ed2),
    (0xC11ED6, i_c11ed6),
    (0xC11EDA, i_c11eda),
    (0xC11EDC, i_c11edc),
    (0xC11EE0, i_c11ee0),
    (0xC11EE4, i_c11ee4),
    (0xC11EE6, i_c11ee6),
    (0xC11EE8, i_c11ee8),
    (0xC11EEA, i_c11eea),
    (0xC11EEE, i_c11eee),
    (0xC11EF2, i_c11ef2),
    (0xC11EF4, i_c11ef4),
    (0xC11EF6, i_c11ef6),
    (0xC11EFA, i_c11efa),
    (0xC11EFC, i_c11efc),
    (0xC11EFE, i_c11efe),
    (0xC11F02, i_c11f02),
    (0xC11F06, i_c11f06),
    (0xC11F0A, i_c11f0a),
    (0xC11F0E, i_c11f0e),
    (0xC11F12, i_c11f12),
    (0xC11F14, i_c11f14),
    (0xC11F16, i_c11f16),
    (0xC11F18, i_c11f18),
    (0xC11F1C, i_c11f1c),
    (0xC11F1E, i_c11f1e),
    (0xC11F20, i_c11f20),
    (0xC11F22, i_c11f22),
    (0xC11F24, i_c11f24),
    (0xC11F28, i_c11f28),
    (0xC11F2C, i_c11f2c),
    (0xC11F2E, i_c11f2e),
    (0xC11F30, i_c11f30),
    (0xC11F32, i_c11f32),
    (0xC11F34, i_c11f34),
    (0xC11F36, i_c11f36),
    (0xC11F38, i_c11f38),
    (0xC11F3A, i_c11f3a),
    (0xC11F3C, i_c11f3c),
    (0xC11F3E, i_c11f3e),
    (0xC11F42, i_c11f42),
    (0xC11F46, i_c11f46),
    (0xC11F48, i_c11f48),
    (0xC11F4C, i_c11f4c),
    (0xC11F4E, i_c11f4e),
    (0xC11F52, i_c11f52),
    (0xC11F54, i_c11f54),
    (0xC11F56, i_c11f56),
    (0xC11F58, i_c11f58),
    (0xC11FCC, i_c11fcc),
    (0xC11FD0, i_c11fd0),
    (0xC11FD4, i_c11fd4),
    (0xC11FD6, i_c11fd6),
    (0xC11FD8, i_c11fd8),
    (0xC11FDA, i_c11fda),
    (0xC11FDE, i_c11fde),
    (0xC11FE2, i_c11fe2),
    (0xC11FE4, i_c11fe4),
    (0xC11FE8, i_c11fe8),
    (0xC11FEA, i_c11fea),
    (0xC11FEE, i_c11fee),
    (0xC11FF2, i_c11ff2),
    (0xC11FF4, i_c11ff4),
    (0xC11FF6, i_c11ff6),
    (0xC11FF8, i_c11ff8),
    (0xC11FFA, i_c11ffa),
    (0xC11FFC, i_c11ffc),
    (0xC12006, i_c12006),
    (0xC1200A, i_c1200a),
    (0xC1200E, i_c1200e),
    (0xC12010, i_c12010),
    (0xC12012, i_c12012),
    (0xC12014, i_c12014),
    (0xC12018, i_c12018),
    (0xC1201A, i_c1201a),
    (0xC12024, i_c12024),
    (0xC1202A, i_c1202a),
    (0xC1202E, i_c1202e),
    (0xC12032, i_c12032),
    (0xC12036, i_c12036),
    (0xC1203A, i_c1203a),
    (0xC1203E, i_c1203e),
    (0xC12040, i_c12040),
    (0xC12042, i_c12042),
    (0xC12044, i_c12044),
    (0xC12048, i_c12048),
    (0xC12050, i_c12050),
    (0xC12056, i_c12056),
    (0xC1205C, i_c1205c),
    (0xC1205E, i_c1205e),
    (0xC12064, i_c12064),
    (0xC12068, i_c12068),
    (0xC1206A, i_c1206a),
    (0xC1207C, i_c1207c),
    (0xC12080, i_c12080),
    (0xC12084, i_c12084),
    (0xC12086, i_c12086),
    (0xC12088, i_c12088),
    (0xC1208A, i_c1208a),
    (0xC1208E, i_c1208e),
    (0xC12092, i_c12092),
    (0xC12094, i_c12094),
    (0xC12096, i_c12096),
    (0xC12098, i_c12098),
    (0xC1209A, i_c1209a),
    (0xC1209C, i_c1209c),
    (0xC1209E, i_c1209e),
    (0xC120A2, i_c120a2),
    (0xC120A4, i_c120a4),
    (0xC120A8, i_c120a8),
    (0xC120AC, i_c120ac),
    (0xC120B0, i_c120b0),
    (0xC120B2, i_c120b2),
    (0xC120B4, i_c120b4),
    (0xC120B6, i_c120b6),
    (0xC120BA, i_c120ba),
    (0xC120BE, i_c120be),
    (0xC120C0, i_c120c0),
    (0xC120C2, i_c120c2),
    (0xC120C4, i_c120c4),
    (0xC120C6, i_c120c6),
    (0xC120C8, i_c120c8),
    (0xC120CA, i_c120ca),
    (0xC120CE, i_c120ce),
    (0xC120D0, i_c120d0),
    (0xC120D4, i_c120d4),
    (0xC120D8, i_c120d8),
    (0xC120DA, i_c120da),
    (0xC120DC, i_c120dc),
    (0xC120DE, i_c120de),
    (0xC120E0, i_c120e0),
    (0xC120E2, i_c120e2),
    (0xC120E6, i_c120e6),
    (0xC120EA, i_c120ea),
    (0xC120EC, i_c120ec),
    (0xC120EE, i_c120ee),
    (0xC120F0, i_c120f0),
    (0xC120F2, i_c120f2),
    (0xC120F4, i_c120f4),
    (0xC120F6, i_c120f6),
    (0xC120F8, i_c120f8),
    (0xC120FC, i_c120fc),
    (0xC120FE, i_c120fe),
    (0xC12100, i_c12100),
    (0xC12104, i_c12104),
    (0xC12106, i_c12106),
    (0xC1210C, i_c1210c),
    (0xC1210E, i_c1210e),
    (0xC12110, i_c12110),
    (0xC12114, i_c12114),
    (0xC12116, i_c12116),
    (0xC12118, i_c12118),
    (0xC1211A, i_c1211a),
    (0xC1211E, i_c1211e),
    (0xC12120, i_c12120),
    (0xC12122, i_c12122),
    (0xC12124, i_c12124),
    (0xC12126, i_c12126),
    (0xC12128, i_c12128),
    (0xC1212C, i_c1212c),
    (0xC1212E, i_c1212e),
    (0xC12132, i_c12132),
    (0xC12134, i_c12134),
    (0xC12138, i_c12138),
    (0xC1213A, i_c1213a),
    (0xC1213C, i_c1213c),
    (0xC1213E, i_c1213e),
    (0xC12142, i_c12142),
    (0xC12144, i_c12144),
    (0xC12146, i_c12146),
    (0xC12148, i_c12148),
    (0xC1214A, i_c1214a),
    (0xC1214C, i_c1214c),
    (0xC12150, i_c12150),
    (0xC12152, i_c12152),
    (0xC12156, i_c12156),
    (0xC12158, i_c12158),
    (0xC1215A, i_c1215a),
    (0xC1215C, i_c1215c),
    (0xC1215E, i_c1215e),
    (0xC12160, i_c12160),
    (0xC12164, i_c12164),
    (0xC12166, i_c12166),
    (0xC12168, i_c12168),
    (0xC1216A, i_c1216a),
    (0xC1216C, i_c1216c),
    (0xC1216E, i_c1216e),
    (0xC12170, i_c12170),
    (0xC12174, i_c12174),
    (0xC12176, i_c12176),
    (0xC1217A, i_c1217a),
    (0xC1217C, i_c1217c),
    (0xC12180, i_c12180),
    (0xC12184, i_c12184),
    (0xC1231E, i_c1231e),
    (0xC12324, i_c12324),
    (0xC12328, i_c12328),
    (0xC1232C, i_c1232c),
    (0xC12332, i_c12332),
    (0xC12338, i_c12338),
    (0xC1233C, i_c1233c),
    (0xC1233E, i_c1233e),
    (0xC12344, i_c12344),
    (0xC12346, i_c12346),
    (0xC1234C, i_c1234c),
    (0xC12352, i_c12352),
    (0xC12354, i_c12354),
    (0xC1235A, i_c1235a),
    (0xC12360, i_c12360),
    (0xC12364, i_c12364),
    (0xC12366, i_c12366),
    (0xC1236A, i_c1236a),
    (0xC1238C, i_c1238c),
    (0xC12390, i_c12390),
    (0xC12392, i_c12392),
    (0xC12398, i_c12398),
    (0xC1239C, i_c1239c),
    (0xC1239E, i_c1239e),
    (0xC123A2, i_c123a2),
    (0xC123A6, i_c123a6),
    (0xC123AA, i_c123aa),
    (0xC123AC, i_c123ac),
    (0xC123B0, i_c123b0),
    (0xC123B2, i_c123b2),
    (0xC123DC, i_c123dc),
    (0xC123DE, i_c123de),
    (0xC123E2, i_c123e2),
    (0xC123E6, i_c123e6),
    (0xC123E8, i_c123e8),
    (0xC123EC, i_c123ec),
    (0xC123EE, i_c123ee),
    (0xC123F6, i_c123f6),
    (0xC123FA, i_c123fa),
    (0xC123FE, i_c123fe),
    (0xC12402, i_c12402),
    (0xC12408, i_c12408),
    (0xC1240E, i_c1240e),
    (0xC12412, i_c12412),
    (0xC12416, i_c12416),
    (0xC12A0E, i_c12a0e),
    (0xC12A12, i_c12a12),
    (0xC12A1A, i_c12a1a),
    (0xC12A20, i_c12a20),
    (0xC12A26, i_c12a26),
    (0xC12A2A, i_c12a2a),
    (0xC12A30, i_c12a30),
    (0xC12A34, i_c12a34),
    (0xC12A3A, i_c12a3a),
    (0xC12A3C, i_c12a3c),
    (0xC12A40, i_c12a40),
    (0xC12A44, i_c12a44),
    (0xC12DCC, i_c12dcc),
    (0xC12DD0, i_c12dd0),
    (0xC12DD4, i_c12dd4),
    (0xC12DD6, i_c12dd6),
    (0xC12DD8, i_c12dd8),
    (0xC12DDC, i_c12ddc),
    (0xC12DE0, i_c12de0),
    (0xC12DE2, i_c12de2),
    (0xC12DE8, i_c12de8),
    (0xC12DEC, i_c12dec),
    (0xC12DF0, i_c12df0),
    (0xC12DF4, i_c12df4),
    (0xC12DF6, i_c12df6),
    (0xC12DF8, i_c12df8),
    (0xC12DFE, i_c12dfe),
    (0xC12E00, i_c12e00),
    (0xC12E04, i_c12e04),
    (0xC12E08, i_c12e08),
    (0xC12E0C, i_c12e0c),
    (0xC12E10, i_c12e10),
    (0xC12E14, i_c12e14),
    (0xC12E18, i_c12e18),
    (0xC12E1C, i_c12e1c),
    (0xC12E1E, i_c12e1e),
    (0xC12E20, i_c12e20),
    (0xC12E22, i_c12e22),
    (0xC12E24, i_c12e24),
    (0xC12E26, i_c12e26),
    (0xC12E2A, i_c12e2a),
    (0xC12E2E, i_c12e2e),
    (0xC12E32, i_c12e32),
    (0xC12E36, i_c12e36),
    (0xC12E3C, i_c12e3c),
    (0xC12E3E, i_c12e3e),
    (0xC12E40, i_c12e40),
    (0xC12E44, i_c12e44),
    (0xC12E46, i_c12e46),
    (0xC12E4A, i_c12e4a),
    (0xC12E4E, i_c12e4e),
    (0xC12E50, i_c12e50),
    (0xC12E52, i_c12e52),
    (0xC12E54, i_c12e54),
    (0xC12E56, i_c12e56),
    (0xC12E58, i_c12e58),
    (0xC12E5A, i_c12e5a),
    (0xC12E5C, i_c12e5c),
    (0xC12E5E, i_c12e5e),
    (0xC12E60, i_c12e60),
    (0xC12E62, i_c12e62),
    (0xC12E66, i_c12e66),
    (0xC12E6A, i_c12e6a),
    (0xC12E6E, i_c12e6e),
    (0xC12E72, i_c12e72),
    (0xC12E78, i_c12e78),
    (0xC12E7A, i_c12e7a),
    (0xC12E7C, i_c12e7c),
    (0xC12E80, i_c12e80),
    (0xC12E82, i_c12e82),
    (0xC12E86, i_c12e86),
    (0xC12E88, i_c12e88),
    (0xC12E8A, i_c12e8a),
    (0xC12E90, i_c12e90),
    (0xC12E92, i_c12e92),
    (0xC12E96, i_c12e96),
    (0xC12E9A, i_c12e9a),
    (0xC12E9E, i_c12e9e),
    (0xC12EA2, i_c12ea2),
    (0xC12EA6, i_c12ea6),
    (0xC12EA8, i_c12ea8),
    (0xC12EAA, i_c12eaa),
    (0xC12EAC, i_c12eac),
    (0xC12EAE, i_c12eae),
    (0xC12EB0, i_c12eb0),
    (0xC12EB2, i_c12eb2),
    (0xC12EB4, i_c12eb4),
    (0xC12EB6, i_c12eb6),
    (0xC12EB8, i_c12eb8),
    (0xC12EBA, i_c12eba),
    (0xC12EBC, i_c12ebc),
    (0xC12EC0, i_c12ec0),
    (0xC12EC4, i_c12ec4),
    (0xC12EC8, i_c12ec8),
    (0xC12ECC, i_c12ecc),
    (0xC12ED2, i_c12ed2),
    (0xC12ED4, i_c12ed4),
    (0xC12ED6, i_c12ed6),
    (0xC12EDA, i_c12eda),
    (0xC1328A, i_c1328a),
    (0xC1328E, i_c1328e),
    (0xC13292, i_c13292),
    (0xC13294, i_c13294),
    (0xC13296, i_c13296),
    (0xC1329A, i_c1329a),
    (0xC1329E, i_c1329e),
    (0xC132A0, i_c132a0),
    (0xC132A4, i_c132a4),
    (0xC132A8, i_c132a8),
    (0xC132AA, i_c132aa),
    (0xC132AE, i_c132ae),
    (0xC132B2, i_c132b2),
    (0xC132B6, i_c132b6),
    (0xC132BA, i_c132ba),
    (0xC132BE, i_c132be),
    (0xC132C4, i_c132c4),
    (0xC132C6, i_c132c6),
    (0xC132CA, i_c132ca),
    (0xC132CC, i_c132cc),
    (0xC132CE, i_c132ce),
    (0xC132D0, i_c132d0),
    (0xC132D2, i_c132d2),
    (0xC132D6, i_c132d6),
    (0xC132DC, i_c132dc),
    (0xC132DE, i_c132de),
    (0xC132E2, i_c132e2),
    (0xC132E6, i_c132e6),
    (0xC132EA, i_c132ea),
    (0xC132EC, i_c132ec),
    (0xC132EE, i_c132ee),
    (0xC132F2, i_c132f2),
    (0xC132F4, i_c132f4),
    (0xC132F6, i_c132f6),
    (0xC132F8, i_c132f8),
    (0xC132FC, i_c132fc),
    (0xC13300, i_c13300),
    (0xC13304, i_c13304),
    (0xC13308, i_c13308),
    (0xC1330C, i_c1330c),
    (0xC1330E, i_c1330e),
    (0xC13312, i_c13312),
    (0xC13318, i_c13318),
    (0xC1331A, i_c1331a),
    (0xC1331C, i_c1331c),
    (0xC13320, i_c13320),
    (0xC13322, i_c13322),
    (0xC13324, i_c13324),
    (0xC13326, i_c13326),
    (0xC13328, i_c13328),
    (0xC1332C, i_c1332c),
    (0xC13332, i_c13332),
    (0xC13334, i_c13334),
    (0xC13338, i_c13338),
    (0xC1333C, i_c1333c),
    (0xC13340, i_c13340),
    (0xC13342, i_c13342),
    (0xC13344, i_c13344),
    (0xC13346, i_c13346),
    (0xC13348, i_c13348),
    (0xC1334A, i_c1334a),
    (0xC1334C, i_c1334c),
    (0xC1334E, i_c1334e),
    (0xC13352, i_c13352),
    (0xC13354, i_c13354),
    (0xC13358, i_c13358),
    (0xC1335A, i_c1335a),
    (0xC1335E, i_c1335e),
    (0xC13362, i_c13362),
    (0xC13364, i_c13364),
    (0xC13368, i_c13368),
    (0xC1336A, i_c1336a),
    (0xC1336E, i_c1336e),
    (0xC13370, i_c13370),
    (0xC13374, i_c13374),
    (0xC13378, i_c13378),
    (0xC1337C, i_c1337c),
    (0xC1337E, i_c1337e),
    (0xC13384, i_c13384),
    (0xC13386, i_c13386),
    (0xC13388, i_c13388),
    (0xC1338C, i_c1338c),
    (0xC1338E, i_c1338e),
    (0xC13390, i_c13390),
    (0xC13392, i_c13392),
    (0xC13394, i_c13394),
    (0xC13398, i_c13398),
    (0xC1339E, i_c1339e),
    (0xC133A0, i_c133a0),
    (0xC133A4, i_c133a4),
    (0xC133A8, i_c133a8),
    (0xC133AC, i_c133ac),
    (0xC133AE, i_c133ae),
    (0xC133B0, i_c133b0),
    (0xC133B2, i_c133b2),
    (0xC133B4, i_c133b4),
    (0xC133B6, i_c133b6),
    (0xC133B8, i_c133b8),
    (0xC133BA, i_c133ba),
    (0xC133BC, i_c133bc),
    (0xC133C0, i_c133c0),
    (0xC133C2, i_c133c2),
    (0xC133C6, i_c133c6),
    (0xC133C8, i_c133c8),
    (0xC133CC, i_c133cc),
    (0xC133D0, i_c133d0),
    (0xC133D2, i_c133d2),
    (0xC133D6, i_c133d6),
    (0xC133D8, i_c133d8),
    (0xC133DC, i_c133dc),
    (0xC133DE, i_c133de),
    (0xC1360C, i_c1360c),
    (0xC13610, i_c13610),
    (0xC13612, i_c13612),
    (0xC13616, i_c13616),
    (0xC13618, i_c13618),
    (0xC1361E, i_c1361e),
    (0xC13620, i_c13620),
    (0xC13622, i_c13622),
    (0xC13624, i_c13624),
    (0xC13626, i_c13626),
    (0xC1362A, i_c1362a),
    (0xC1362E, i_c1362e),
    (0xC13630, i_c13630),
    (0xC13634, i_c13634),
    (0xC13636, i_c13636),
    (0xC13638, i_c13638),
    (0xC1363C, i_c1363c),
    (0xC1363E, i_c1363e),
    (0xC13644, i_c13644),
    (0xC13646, i_c13646),
    (0xC13648, i_c13648),
    (0xC1364A, i_c1364a),
    (0xC1364C, i_c1364c),
    (0xC13650, i_c13650),
    (0xC13652, i_c13652),
    (0xC13656, i_c13656),
    (0xC13658, i_c13658),
    (0xC1365C, i_c1365c),
    (0xC1365E, i_c1365e),
    (0xC13660, i_c13660),
    (0xC13664, i_c13664),
    (0xC13666, i_c13666),
    (0xC1366C, i_c1366c),
    (0xC1366E, i_c1366e),
    (0xC13670, i_c13670),
    (0xC13672, i_c13672),
    (0xC13674, i_c13674),
    (0xC13676, i_c13676),
    (0xC1367A, i_c1367a),
    (0xC1367E, i_c1367e),
    (0xC13682, i_c13682),
    (0xC13686, i_c13686),
    (0xC1368C, i_c1368c),
    (0xC1368E, i_c1368e),
    (0xC13692, i_c13692),
    (0xC13694, i_c13694),
    (0xC13698, i_c13698),
    (0xC1369C, i_c1369c),
    (0xC1369E, i_c1369e),
    (0xC136A2, i_c136a2),
    (0xC136A6, i_c136a6),
    (0xC136A8, i_c136a8),
    (0xC136AC, i_c136ac),
    (0xC136AE, i_c136ae),
    (0xC136B0, i_c136b0),
    (0xC136B4, i_c136b4),
    (0xC136B6, i_c136b6),
    (0xC136B8, i_c136b8),
    (0xC136BC, i_c136bc),
    (0xC136C0, i_c136c0),
    (0xC136C2, i_c136c2),
    (0xC136C4, i_c136c4),
    (0xC136C8, i_c136c8),
    (0xC136CA, i_c136ca),
    (0xC136CC, i_c136cc),
    (0xC136D0, i_c136d0),
    (0xC136D2, i_c136d2),
    (0xC136D6, i_c136d6),
    (0xC136D8, i_c136d8),
    (0xC136DC, i_c136dc),
    (0xC136DE, i_c136de),
    (0xC136E2, i_c136e2),
    (0xC136E4, i_c136e4),
    (0xC136E6, i_c136e6),
    (0xC136EA, i_c136ea),
    (0xC136EC, i_c136ec),
    (0xC136EE, i_c136ee),
    (0xC136F0, i_c136f0),
    (0xC136F4, i_c136f4),
    (0xC136F8, i_c136f8),
    (0xC136FA, i_c136fa),
    (0xC136FC, i_c136fc),
    (0xC13700, i_c13700),
    (0xC13702, i_c13702),
    (0xC13704, i_c13704),
    (0xC13708, i_c13708),
    (0xC1370A, i_c1370a),
    (0xC1370E, i_c1370e),
    (0xC13710, i_c13710),
    (0xC13714, i_c13714),
    (0xC13716, i_c13716),
    (0xC1371A, i_c1371a),
    (0xC1371C, i_c1371c),
    (0xC1371E, i_c1371e),
    (0xC13722, i_c13722),
    (0xC13724, i_c13724),
    (0xC13726, i_c13726),
    (0xC1372A, i_c1372a),
    (0xC1372E, i_c1372e),
    (0xC13732, i_c13732),
    (0xC13734, i_c13734),
    (0xC13738, i_c13738),
    (0xC1373C, i_c1373c),
    (0xC13740, i_c13740),
    (0xC13742, i_c13742),
    (0xC13744, i_c13744),
    (0xC13746, i_c13746),
    (0xC1374A, i_c1374a),
    (0xC1374E, i_c1374e),
    (0xC13750, i_c13750),
    (0xC13752, i_c13752),
    (0xC13754, i_c13754),
    (0xC13756, i_c13756),
    (0xC1375A, i_c1375a),
    (0xC1375E, i_c1375e),
    (0xC13760, i_c13760),
    (0xC13764, i_c13764),
    (0xC1376A, i_c1376a),
    (0xC1376C, i_c1376c),
    (0xC13772, i_c13772),
    (0xC13776, i_c13776),
    (0xC1377A, i_c1377a),
    (0xC1377C, i_c1377c),
    (0xC1377E, i_c1377e),
    (0xC13782, i_c13782),
    (0xC13784, i_c13784),
    (0xC13786, i_c13786),
    (0xC13788, i_c13788),
    (0xC1378A, i_c1378a),
    (0xC1378C, i_c1378c),
    (0xC1378E, i_c1378e),
    (0xC13790, i_c13790),
    (0xC13792, i_c13792),
    (0xC13796, i_c13796),
    (0xC1379A, i_c1379a),
    (0xC1379E, i_c1379e),
    (0xC137A0, i_c137a0),
    (0xC137A6, i_c137a6),
    (0xC137AA, i_c137aa),
    (0xC137AE, i_c137ae),
    (0xC137B0, i_c137b0),
    (0xC137B2, i_c137b2),
    (0xC137B6, i_c137b6),
    (0xC137B8, i_c137b8),
    (0xC137BA, i_c137ba),
    (0xC137BC, i_c137bc),
    (0xC137BE, i_c137be),
    (0xC137C0, i_c137c0),
    (0xC137C4, i_c137c4),
    (0xC137CA, i_c137ca),
    (0xC137CC, i_c137cc),
    (0xC137D2, i_c137d2),
    (0xC137D4, i_c137d4),
    (0xC137D8, i_c137d8),
    (0xC137DA, i_c137da),
    (0xC137DC, i_c137dc),
    (0xC137DE, i_c137de),
    (0xC137E0, i_c137e0),
    (0xC137E4, i_c137e4),
    (0xC137E6, i_c137e6),
    (0xC137E8, i_c137e8),
    (0xC137EC, i_c137ec),
    (0xC137F0, i_c137f0),
    (0xC137F4, i_c137f4),
    (0xC137F6, i_c137f6),
    (0xC137F8, i_c137f8),
    (0xC137FA, i_c137fa),
    (0xC137FE, i_c137fe),
    (0xC13802, i_c13802),
    (0xC13804, i_c13804),
    (0xC13806, i_c13806),
    (0xC13808, i_c13808),
    (0xC1380A, i_c1380a),
    (0xC1380E, i_c1380e),
    (0xC13812, i_c13812),
    (0xC13814, i_c13814),
    (0xC13818, i_c13818),
    (0xC1381E, i_c1381e),
    (0xC13820, i_c13820),
    (0xC13826, i_c13826),
    (0xC1382A, i_c1382a),
    (0xC1382E, i_c1382e),
    (0xC13830, i_c13830),
    (0xC13832, i_c13832),
    (0xC13836, i_c13836),
    (0xC13838, i_c13838),
    (0xC1383A, i_c1383a),
    (0xC1383C, i_c1383c),
    (0xC1383E, i_c1383e),
    (0xC13840, i_c13840),
    (0xC13842, i_c13842),
    (0xC13844, i_c13844),
    (0xC13846, i_c13846),
    (0xC13848, i_c13848),
    (0xC1384A, i_c1384a),
    (0xC1384E, i_c1384e),
    (0xC13852, i_c13852),
    (0xC13856, i_c13856),
    (0xC13858, i_c13858),
    (0xC1385E, i_c1385e),
    (0xC13862, i_c13862),
    (0xC13866, i_c13866),
    (0xC13868, i_c13868),
    (0xC1386A, i_c1386a),
    (0xC1386E, i_c1386e),
    (0xC13870, i_c13870),
    (0xC13872, i_c13872),
    (0xC13874, i_c13874),
    (0xC13876, i_c13876),
    (0xC13878, i_c13878),
    (0xC1387C, i_c1387c),
    (0xC13882, i_c13882),
    (0xC13884, i_c13884),
    (0xC1388A, i_c1388a),
    (0xC1388C, i_c1388c),
    (0xC13890, i_c13890),
    (0xC13892, i_c13892),
    (0xC13894, i_c13894),
    (0xC13896, i_c13896),
    (0xC13898, i_c13898),
    (0xC1389A, i_c1389a),
    (0xC1389C, i_c1389c),
    (0xC138A0, i_c138a0),
    (0xC138A2, i_c138a2),
    (0xC138A4, i_c138a4),
    (0xC138A8, i_c138a8),
    (0xC138AC, i_c138ac),
    (0xC138B0, i_c138b0),
    (0xC138B2, i_c138b2),
    (0xC138B4, i_c138b4),
    (0xC138B6, i_c138b6),
    (0xC138B8, i_c138b8),
    (0xC138BC, i_c138bc),
    (0xC138C0, i_c138c0),
    (0xC138C2, i_c138c2),
    (0xC138C4, i_c138c4),
    (0xC138C6, i_c138c6),
    (0xC138C8, i_c138c8),
    (0xC138CC, i_c138cc),
    (0xC138D0, i_c138d0),
    (0xC138D2, i_c138d2),
    (0xC138D6, i_c138d6),
    (0xC138DC, i_c138dc),
    (0xC138DE, i_c138de),
    (0xC138E4, i_c138e4),
    (0xC138E8, i_c138e8),
    (0xC138EC, i_c138ec),
    (0xC138EE, i_c138ee),
    (0xC138F0, i_c138f0),
    (0xC138F4, i_c138f4),
    (0xC138F6, i_c138f6),
    (0xC138F8, i_c138f8),
    (0xC138FA, i_c138fa),
    (0xC138FC, i_c138fc),
    (0xC138FE, i_c138fe),
    (0xC13900, i_c13900),
    (0xC13902, i_c13902),
    (0xC13904, i_c13904),
    (0xC13906, i_c13906),
    (0xC13908, i_c13908),
    (0xC1390A, i_c1390a),
    (0xC1390C, i_c1390c),
    (0xC13910, i_c13910),
    (0xC13912, i_c13912),
    (0xC13914, i_c13914),
    (0xC13916, i_c13916),
    (0xC1391A, i_c1391a),
    (0xC1391E, i_c1391e),
    (0xC13920, i_c13920),
    (0xC13926, i_c13926),
    (0xC1392A, i_c1392a),
    (0xC1392E, i_c1392e),
    (0xC13930, i_c13930),
    (0xC13932, i_c13932),
    (0xC13936, i_c13936),
    (0xC13938, i_c13938),
    (0xC1393A, i_c1393a),
    (0xC1393C, i_c1393c),
    (0xC1393E, i_c1393e),
    (0xC13940, i_c13940),
    (0xC13942, i_c13942),
    (0xC13944, i_c13944),
    (0xC13948, i_c13948),
    (0xC1394C, i_c1394c),
    (0xC1394E, i_c1394e),
    (0xC13950, i_c13950),
    (0xC13952, i_c13952),
    (0xC13956, i_c13956),
    (0xC13958, i_c13958),
    (0xC1395E, i_c1395e),
    (0xC13960, i_c13960),
    (0xC13964, i_c13964),
    (0xC13966, i_c13966),
    (0xC13968, i_c13968),
    (0xC1396A, i_c1396a),
    (0xC1396C, i_c1396c),
    (0xC1396E, i_c1396e),
    (0xC13970, i_c13970),
    (0xC13972, i_c13972),
    (0xC13976, i_c13976),
    (0xC13978, i_c13978),
    (0xC1397A, i_c1397a),
    (0xC1397C, i_c1397c),
    (0xC13980, i_c13980),
    (0xC13982, i_c13982),
    (0xC13984, i_c13984),
    (0xC13986, i_c13986),
    (0xC1398A, i_c1398a),
    (0xC1398C, i_c1398c),
    (0xC1398E, i_c1398e),
    (0xC13994, i_c13994),
    (0xC13996, i_c13996),
    (0xC1399C, i_c1399c),
    (0xC1399E, i_c1399e),
    (0xC139A0, i_c139a0),
    (0xC139A6, i_c139a6),
    (0xC139AA, i_c139aa),
    (0xC139AE, i_c139ae),
    (0xC139B0, i_c139b0),
    (0xC139B2, i_c139b2),
    (0xC139B4, i_c139b4),
    (0xC139B6, i_c139b6),
    (0xC139B8, i_c139b8),
    (0xC139BC, i_c139bc),
    (0xC139C0, i_c139c0),
    (0xC139C4, i_c139c4),
    (0xC139C8, i_c139c8),
    (0xC139CA, i_c139ca),
    (0xC139CE, i_c139ce),
    (0xC139D0, i_c139d0),
    (0xC139D4, i_c139d4),
    (0xC139D8, i_c139d8),
    (0xC139DA, i_c139da),
    (0xC139DE, i_c139de),
    (0xC139E0, i_c139e0),
    (0xC139E4, i_c139e4),
    (0xC139E8, i_c139e8),
    (0xC139EC, i_c139ec),
    (0xC139F0, i_c139f0),
    (0xC139F4, i_c139f4),
    (0xC139F8, i_c139f8),
    (0xC139FC, i_c139fc),
    (0xC13A00, i_c13a00),
    (0xC13A04, i_c13a04),
    (0xC13A08, i_c13a08),
    (0xC13D02, i_c13d02),
    (0xC13D04, i_c13d04),
    (0xC13D08, i_c13d08),
    (0xC13D0C, i_c13d0c),
    (0xC13D0E, i_c13d0e),
    (0xC13D10, i_c13d10),
    (0xC13D14, i_c13d14),
    (0xC13D18, i_c13d18),
    (0xC13D1A, i_c13d1a),
    (0xC13D1E, i_c13d1e),
    (0xC13D22, i_c13d22),
    (0xC13D24, i_c13d24),
    (0xC13D28, i_c13d28),
    (0xC13DBA, i_c13dba),
    (0xC13DBC, i_c13dbc),
    (0xC13DC0, i_c13dc0),
    (0xC13DC4, i_c13dc4),
    (0xC13E58, i_c13e58),
    (0xC13E5E, i_c13e5e),
    (0xC13E64, i_c13e64),
    (0xC13E66, i_c13e66),
    (0xC13E6C, i_c13e6c),
    (0xC13E72, i_c13e72),
    (0xC15A7C, i_c15a7c),
    (0xC15A7E, i_c15a7e),
    (0xC15A82, i_c15a82),
    (0xC15A84, i_c15a84),
    (0xC15A86, i_c15a86),
    (0xC15AAC, i_c15aac),
    (0xC15AB2, i_c15ab2),
    (0xC15ABA, i_c15aba),
    (0xC15AC0, i_c15ac0),
    (0xC15AC4, i_c15ac4),
    (0xC15AC8, i_c15ac8),
    (0xC15ACC, i_c15acc),
    (0xC15AD0, i_c15ad0),
    (0xC15AD4, i_c15ad4),
    (0xC15ADC, i_c15adc),
    (0xC15AE2, i_c15ae2),
    (0xC15AEA, i_c15aea),
    (0xC15AEE, i_c15aee),
    (0xC15AF2, i_c15af2),
    (0xC15AF8, i_c15af8),
    (0xC15AFC, i_c15afc),
    (0xC15B06, i_c15b06),
    (0xC15B0A, i_c15b0a),
    (0xC15B0E, i_c15b0e),
    (0xC15B10, i_c15b10),
    (0xC15B14, i_c15b14),
    (0xC15B18, i_c15b18),
    (0xC15B1C, i_c15b1c),
    (0xC15B20, i_c15b20),
    (0xC15B24, i_c15b24),
    (0xC15B26, i_c15b26),
    (0xC15B2C, i_c15b2c),
    (0xC15B30, i_c15b30),
    (0xC15B36, i_c15b36),
    (0xC15B3A, i_c15b3a),
    (0xC15B3E, i_c15b3e),
    (0xC15B42, i_c15b42),
    (0xC15B46, i_c15b46),
    (0xC15B4A, i_c15b4a),
    (0xC15B4E, i_c15b4e),
    (0xC15B52, i_c15b52),
    (0xC15B56, i_c15b56),
    (0xC15B5A, i_c15b5a),
    (0xC15B5E, i_c15b5e),
    (0xC15B62, i_c15b62),
    (0xC15B68, i_c15b68),
    (0xC15B6C, i_c15b6c),
    (0xC15B70, i_c15b70),
    (0xC15B74, i_c15b74),
    (0xC15B7C, i_c15b7c),
    (0xC15B82, i_c15b82),
    (0xC15B9C, i_c15b9c),
    (0xC15B9E, i_c15b9e),
    (0xC15BA0, i_c15ba0),
    (0xC15BA4, i_c15ba4),
    (0xC15BAA, i_c15baa),
    (0xC15BB0, i_c15bb0),
    (0xC15BB2, i_c15bb2),
    (0xC15BB6, i_c15bb6),
    (0xC15BBC, i_c15bbc),
    (0xC15BBE, i_c15bbe),
    (0xC15BC2, i_c15bc2),
    (0xC15BC8, i_c15bc8),
    (0xC15BCC, i_c15bcc),
    (0xC15BD2, i_c15bd2),
    (0xC15BD4, i_c15bd4),
    (0xC15BD8, i_c15bd8),
    (0xC15BDE, i_c15bde),
    (0xC16034, i_c16034),
    (0xC16038, i_c16038),
    (0xC1603E, i_c1603e),
    (0xC16044, i_c16044),
    (0xC16046, i_c16046),
    (0xC16048, i_c16048),
    (0xC1604A, i_c1604a),
    (0xC1604C, i_c1604c),
    (0xC16052, i_c16052),
    (0xC16056, i_c16056),
    (0xC1605C, i_c1605c),
    (0xC16060, i_c16060),
    (0xC16064, i_c16064),
    (0xC16068, i_c16068),
    (0xC1606C, i_c1606c),
    (0xC16070, i_c16070),
    (0xC16074, i_c16074),
    (0xC16078, i_c16078),
    (0xC1607C, i_c1607c),
    (0xC16080, i_c16080),
    (0xC16086, i_c16086),
    (0xC1608C, i_c1608c),
    (0xC16090, i_c16090),
    (0xC16094, i_c16094),
    (0xC16098, i_c16098),
    (0xC1609C, i_c1609c),
    (0xC160A0, i_c160a0),
    (0xC160A4, i_c160a4),
    (0xC160AA, i_c160aa),
    (0xC160B0, i_c160b0),
    (0xC160B6, i_c160b6),
    (0xC160BA, i_c160ba),
    (0xC160BE, i_c160be),
    (0xC160C2, i_c160c2),
    (0xC160C6, i_c160c6),
    (0xC160CA, i_c160ca),
    (0xC160CE, i_c160ce),
    (0xC160D4, i_c160d4),
    (0xC160DA, i_c160da),
    (0xC160E0, i_c160e0),
    (0xC160E8, i_c160e8),
    (0xC160F0, i_c160f0),
    (0xC160F8, i_c160f8),
    (0xC160FE, i_c160fe),
    (0xC16106, i_c16106),
    (0xC1610A, i_c1610a),
    (0xC16110, i_c16110),
    (0xC16112, i_c16112),
    (0xC16118, i_c16118),
    (0xC1611C, i_c1611c),
    (0xC16120, i_c16120),
    (0xC16126, i_c16126),
    (0xC1612A, i_c1612a),
    (0xC16130, i_c16130),
    (0xC16136, i_c16136),
    (0xC1613A, i_c1613a),
    (0xC1613C, i_c1613c),
    (0xC1613E, i_c1613e),
    (0xC16142, i_c16142),
    (0xC16144, i_c16144),
    (0xC169B0, i_c169b0),
    (0xC169B8, i_c169b8),
    (0xC169BE, i_c169be),
    (0xC169C0, i_c169c0),
    (0xC169C4, i_c169c4),
    (0xC169C8, i_c169c8),
    (0xC169CE, i_c169ce),
    (0xC169D2, i_c169d2),
    (0xC169D6, i_c169d6),
    (0xC169DA, i_c169da),
    (0xC169DE, i_c169de),
    (0xC169E4, i_c169e4),
    (0xC169E8, i_c169e8),
    (0xC169EC, i_c169ec),
    (0xC169F0, i_c169f0),
    (0xC169F6, i_c169f6),
    (0xC169FA, i_c169fa),
    (0xC169FC, i_c169fc),
    (0xC16A02, i_c16a02),
    (0xC16A04, i_c16a04),
    (0xC16A08, i_c16a08),
    (0xC16A0E, i_c16a0e),
    (0xC16A12, i_c16a12),
    (0xC16A16, i_c16a16),
    (0xC16A18, i_c16a18),
    (0xC16A1E, i_c16a1e),
    (0xC16A20, i_c16a20),
    (0xC16A24, i_c16a24),
    (0xC16A26, i_c16a26),
    (0xC16A2A, i_c16a2a),
    (0xC16A30, i_c16a30),
    (0xC16A32, i_c16a32),
    (0xC16A34, i_c16a34),
    (0xC16A38, i_c16a38),
    (0xC16A3E, i_c16a3e),
    (0xC16A44, i_c16a44),
    (0xC16A48, i_c16a48),
    (0xC16A4A, i_c16a4a),
    (0xC16A4E, i_c16a4e),
    (0xC16A50, i_c16a50),
    (0xC16A98, i_c16a98),
    (0xC16A9A, i_c16a9a),
    (0xC16A9C, i_c16a9c),
    (0xC16A9E, i_c16a9e),
    (0xC16AA0, i_c16aa0),
    (0xC16AA6, i_c16aa6),
    (0xC16AAA, i_c16aaa),
    (0xC16AB0, i_c16ab0),
    (0xC16AB4, i_c16ab4),
    (0xC16AB8, i_c16ab8),
    (0xC16ABC, i_c16abc),
    (0xC16AC0, i_c16ac0),
    (0xC16AC4, i_c16ac4),
    (0xC16ACA, i_c16aca),
    (0xC16ACE, i_c16ace),
    (0xC16AD4, i_c16ad4),
    (0xC16AD8, i_c16ad8),
    (0xC16ADC, i_c16adc),
    (0xC16AE0, i_c16ae0),
    (0xC16AE4, i_c16ae4),
    (0xC16AE8, i_c16ae8),
    (0xC16AEC, i_c16aec),
    (0xC16AF0, i_c16af0),
    (0xC16AF4, i_c16af4),
    (0xC16AFE, i_c16afe),
    (0xC16B04, i_c16b04),
    (0xC16B08, i_c16b08),
    (0xC16B0C, i_c16b0c),
    (0xC16B0E, i_c16b0e),
    (0xC16B12, i_c16b12),
    (0xC16B16, i_c16b16),
    (0xC16B1C, i_c16b1c),
    (0xC16B20, i_c16b20),
    (0xC16B24, i_c16b24),
    (0xC16B28, i_c16b28),
    (0xC16B2E, i_c16b2e),
    (0xC16B34, i_c16b34),
    (0xC16B36, i_c16b36),
    (0xC16B38, i_c16b38),
    (0xC16B3C, i_c16b3c),
    (0xC16B40, i_c16b40),
    (0xC16B46, i_c16b46),
    (0xC16B4A, i_c16b4a),
    (0xC16B4E, i_c16b4e),
    (0xC16B52, i_c16b52),
    (0xC16B54, i_c16b54),
    (0xC16B56, i_c16b56),
    (0xC16B58, i_c16b58),
    (0xC16B5C, i_c16b5c),
    (0xC16B60, i_c16b60),
    (0xC16B66, i_c16b66),
    (0xC16B6A, i_c16b6a),
    (0xC16B6E, i_c16b6e),
    (0xC16B70, i_c16b70),
    (0xC16B72, i_c16b72),
    (0xC16B76, i_c16b76),
    (0xC16B7A, i_c16b7a),
    (0xC16B7C, i_c16b7c),
    (0xC16B7E, i_c16b7e),
    (0xC16B82, i_c16b82),
    (0xC16B86, i_c16b86),
    (0xC16B8A, i_c16b8a),
    (0xC16B90, i_c16b90),
    (0xC16B96, i_c16b96),
    (0xC16B9A, i_c16b9a),
    (0xC16B9C, i_c16b9c),
    (0xC16BA0, i_c16ba0),
    (0xC16BA4, i_c16ba4),
    (0xC16BA8, i_c16ba8),
    (0xC16BAA, i_c16baa),
    (0xC16BAE, i_c16bae),
    (0xC16BB0, i_c16bb0),
    (0xC16BB6, i_c16bb6),
    (0xC16BB8, i_c16bb8),
    (0xC16BC0, i_c16bc0),
    (0xC16BC4, i_c16bc4),
    (0xC16BC6, i_c16bc6),
    (0xC16BCA, i_c16bca),
    (0xC16BCE, i_c16bce),
    (0xC16BD2, i_c16bd2),
    (0xC16BD6, i_c16bd6),
    (0xC16BD8, i_c16bd8),
    (0xC16BDC, i_c16bdc),
    (0xC16BDE, i_c16bde),
    (0xC16BE0, i_c16be0),
    (0xC16BE4, i_c16be4),
    (0xC16BE8, i_c16be8),
    (0xC16BEA, i_c16bea),
    (0xC16BF0, i_c16bf0),
    (0xC16BF6, i_c16bf6),
    (0xC16BFC, i_c16bfc),
    (0xC16BFE, i_c16bfe),
    (0xC16C02, i_c16c02),
    (0xC16C04, i_c16c04),
    (0xC16C0A, i_c16c0a),
    (0xC16C0C, i_c16c0c),
    (0xC16C10, i_c16c10),
    (0xC16C14, i_c16c14),
    (0xC16C18, i_c16c18),
    (0xC16C1A, i_c16c1a),
    (0xC16C1E, i_c16c1e),
    (0xC16C24, i_c16c24),
    (0xC16C2A, i_c16c2a),
    (0xC16C2C, i_c16c2c),
    (0xC16C32, i_c16c32),
    (0xC16C38, i_c16c38),
    (0xC16C3E, i_c16c3e),
    (0xC16C44, i_c16c44),
    (0xC16C4A, i_c16c4a),
    (0xC16C50, i_c16c50),
    (0xC16C56, i_c16c56),
    (0xC16C5C, i_c16c5c),
    (0xC16C64, i_c16c64),
    (0xC16C66, i_c16c66),
    (0xC16C6A, i_c16c6a),
    (0xC16C6E, i_c16c6e),
    (0xC16C72, i_c16c72),
    (0xC16C74, i_c16c74),
    (0xC16C78, i_c16c78),
    (0xC16C7E, i_c16c7e),
    (0xC16C82, i_c16c82),
    (0xC16C84, i_c16c84),
    (0xC16C88, i_c16c88),
    (0xC16C8C, i_c16c8c),
    (0xC16C90, i_c16c90),
    (0xC16C94, i_c16c94),
    (0xC16C96, i_c16c96),
    (0xC16C9C, i_c16c9c),
    (0xC16CA2, i_c16ca2),
    (0xC16CA4, i_c16ca4),
    (0xC16CA8, i_c16ca8),
    (0xC16CAC, i_c16cac),
    (0xC16CB0, i_c16cb0),
    (0xC16CB4, i_c16cb4),
    (0xC16CB8, i_c16cb8),
    (0xC16CBC, i_c16cbc),
    (0xC16CC2, i_c16cc2),
    (0xC16CC8, i_c16cc8),
    (0xC16CCC, i_c16ccc),
    (0xC16CD0, i_c16cd0),
    (0xC16CD4, i_c16cd4),
    (0xC16CD8, i_c16cd8),
    (0xC16CDC, i_c16cdc),
    (0xC16CE0, i_c16ce0),
    (0xC16CE4, i_c16ce4),
    (0xC16CE8, i_c16ce8),
    (0xC16CEC, i_c16cec),
    (0xC16CF2, i_c16cf2),
    (0xC16CF6, i_c16cf6),
    (0xC16CFA, i_c16cfa),
    (0xC16CFE, i_c16cfe),
    (0xC16D00, i_c16d00),
    (0xC16D06, i_c16d06),
    (0xC16D0A, i_c16d0a),
    (0xC16D0E, i_c16d0e),
    (0xC16D10, i_c16d10),
    (0xC16D16, i_c16d16),
    (0xC16D1C, i_c16d1c),
    (0xC16E20, i_c16e20),
    (0xC16E26, i_c16e26),
    (0xC16E2A, i_c16e2a),
    (0xC16E2E, i_c16e2e),
    (0xC16E32, i_c16e32),
    (0xC16E38, i_c16e38),
    (0xC16E3C, i_c16e3c),
    (0xC16E40, i_c16e40),
    (0xC16E44, i_c16e44),
    (0xC16E4C, i_c16e4c),
    (0xC16E52, i_c16e52),
    (0xC16E58, i_c16e58),
    (0xC16E5C, i_c16e5c),
    (0xC16E62, i_c16e62),
    (0xC16E66, i_c16e66),
    (0xC16E6C, i_c16e6c),
    (0xC16E6E, i_c16e6e),
    (0xC16E72, i_c16e72),
    (0xC16E76, i_c16e76),
    (0xC16E7A, i_c16e7a),
    (0xC16E7C, i_c16e7c),
    (0xC16E82, i_c16e82),
    (0xC16E88, i_c16e88),
    (0xC16E8A, i_c16e8a),
    (0xC16E8E, i_c16e8e),
    (0xC16E90, i_c16e90),
    (0xC16E92, i_c16e92),
    (0xC16E94, i_c16e94),
    (0xC16E96, i_c16e96),
    (0xC16E98, i_c16e98),
    (0xC16E9C, i_c16e9c),
    (0xC16E9E, i_c16e9e),
    (0xC16EA2, i_c16ea2),
    (0xC16EA4, i_c16ea4),
    (0xC16EA6, i_c16ea6),
    (0xC16EAA, i_c16eaa),
    (0xC16EAE, i_c16eae),
    (0xC16EB2, i_c16eb2),
    (0xC16EB6, i_c16eb6),
    (0xC16EB8, i_c16eb8),
    (0xC16EBC, i_c16ebc),
    (0xC16EBE, i_c16ebe),
    (0xC16EC4, i_c16ec4),
];
