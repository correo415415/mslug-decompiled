// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


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
/// $C12EDC: move.b $380000.l, d0
pub fn i_c12edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EE2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C12EE2: andi.b #$40, d0
pub fn i_c12ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EE6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12EE6: beq.b $c12ef0
pub fn i_c12ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EE8;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12EF0; return 10; }
    8
}
/// $C12EE8: move.b #$85, $ac6(a5)
pub fn i_c12ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EEE;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12EEE: rts
pub fn i_c12eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12EF0: jsr $c13978(pc)
pub fn i_c12ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12EF4);
    cpu.pc = 0xC13978;
    16
}
/// $C12EF4: tst.b $ac6(a5)
pub fn i_c12ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EF8;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12EF8: beq.b $c12efc
pub fn i_c12ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EFA;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12EFC; return 10; }
    8
}
/// $C12EFA: rts
pub fn i_c12efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12EFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12EFC: jsr $c13726(pc)
pub fn i_c12efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F00);
    cpu.pc = 0xC13726;
    16
}
/// $C12F00: tst.b $ac6(a5)
pub fn i_c12f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F04;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12F04: bne.b $c12efa
pub fn i_c12f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F06;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12EFA; return 10; }
    8
}
/// $C12F06: andi.b #$f, $ad0(a5)
pub fn i_c12f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F0C;
    cpu.ir = 0x22D;
    let ad1 = cpu.a[5].wrapping_add(0xAD0);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C12F0C: jsr $c13e58(pc)
pub fn i_c12f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F10);
    cpu.pc = 0xC13E58;
    16
}
/// $C12F10: movea.l $a0a(a5), a1
pub fn i_c12f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F14;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C12F14: movea.l $a04(a5), a0
pub fn i_c12f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F18;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12F18: move.w $a08(a5), d7
pub fn i_c12f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F1C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12F1C: tst.b $a15(a5)
pub fn i_c12f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F20;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12F20: bmi.w $c130d6
pub fn i_c12f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F24;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC130D6; return 10; }
    8
}
/// $C12F24: bne.w $c12fca
pub fn i_c12f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12FCA; return 10; }
    8
}
/// $C12F28: jsr $c135de(pc)
pub fn i_c12f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F2C);
    cpu.pc = 0xC135DE;
    16
}
/// $C12F2C: tst.b d0
pub fn i_c12f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F2E;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C12F2E: beq.b $c12f38
pub fn i_c12f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F30;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12F38; return 10; }
    8
}
/// $C12F30: move.b #$84, $ac6(a5)
pub fn i_c12f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F36;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12F36: rts
pub fn i_c12f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12F38: jsr $c1355a(pc)
pub fn i_c12f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F3C);
    cpu.pc = 0xC1355A;
    16
}
/// $C12F3C: tst.b d0
pub fn i_c12f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F3E;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C12F3E: bne.b $c12f30
pub fn i_c12f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F40;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12F30; return 10; }
    8
}
/// $C12F40: move.w d3, d5
pub fn i_c12f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F42;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12F42: move.w #$1, (a1, d3.w)
pub fn i_c12f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F48;
    cpu.ir = 0x33BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C12F48: lsr.w #$1, d3
pub fn i_c12f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F4A;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12F4A: move.w d3, $6(a0)
pub fn i_c12f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F4E;
    cpu.ir = 0x3143;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12F4E: jsr $c13616(pc)
pub fn i_c12f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F52);
    cpu.pc = 0xC13616;
    16
}
/// $C12F52: moveq #$3f, d6
pub fn i_c12f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F54;
    cpu.ir = 0x7C3F;
    cpu.d[6] = 0x3F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12F54: movea.l $ac8(a5), a4
pub fn i_c12f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F58;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C12F58: move.w $acc(a5), d7
pub fn i_c12f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F5C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12F5C: move.b (a4)+, d0
pub fn i_c12f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F5E;
    cpu.ir = 0x101C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C12F5E: move.w d0, (a3)+
pub fn i_c12f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F60;
    cpu.ir = 0x36C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12F60: subq.w #$1, d7
pub fn i_c12f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F62;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12F62: beq.b $c12f94
pub fn i_c12f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F64;
    cpu.ir = 0x6730;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12F94; return 10; }
    8
}
/// $C12F64: dbra d6, $c12f5c
pub fn i_c12f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F68;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12F68; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12F68; return 14; }
    cpu.pc = 0xC12F5C;
    10
}
/// $C12F68: jsr $c1355a(pc)
pub fn i_c12f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F6C);
    cpu.pc = 0xC1355A;
    16
}
/// $C12F6C: tst.b d0
pub fn i_c12f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F6E;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C12F6E: beq.b $c12f76
pub fn i_c12f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F70;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12F76; return 10; }
    8
}
/// $C12F70: jsr $c137a0(pc)
pub fn i_c12f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F74);
    cpu.pc = 0xC137A0;
    16
}
/// $C12F74: bra.b $c12f30
pub fn i_c12f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F76;
    cpu.ir = 0x60BA;
    cpu.pc = 0xC12F30;
    10
}
/// $C12F76: move.w #$1, (a1, d3.w)
pub fn i_c12f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F7C;
    cpu.ir = 0x33BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C12F7C: lsr.w #$1, d3
pub fn i_c12f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F7E;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12F7E: move.w d3, (a1, d5.w)
pub fn i_c12f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F82;
    cpu.ir = 0x3383;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C12F82: move.w d3, d5
pub fn i_c12f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F84;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12F84: add.w d5, d5
pub fn i_c12f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F86;
    cpu.ir = 0xDA45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C12F86: jsr $c13616(pc)
pub fn i_c12f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F8A);
    cpu.pc = 0xC13616;
    16
}
/// $C12F8A: move.b d0, $300001.l
pub fn i_c12f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F90;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12F90: moveq #$3f, d6
pub fn i_c12f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F92;
    cpu.ir = 0x7C3F;
    cpu.d[6] = 0x3F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12F92: bra.b $c12f5c
pub fn i_c12f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F94;
    cpu.ir = 0x60C8;
    cpu.pc = 0xC12F5C;
    10
}
/// $C12F94: movea.l a0, a4
pub fn i_c12f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F96;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $C12F96: jsr $c13674(pc)
pub fn i_c12f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12F9A);
    cpu.pc = 0xC13674;
    16
}
/// $C12F9A: tst.w d3
pub fn i_c12f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F9C;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C12F9C: beq.b $c12fb8
pub fn i_c12f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12F9E;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12FB8; return 10; }
    8
}
/// $C12F9E: move.b $ace(a5), d0
pub fn i_c12f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FA2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C12FA2: move.w d0, $2(a4)
pub fn i_c12fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FA6;
    cpu.ir = 0x3940;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12FA6: move.b $acf(a5), d0
pub fn i_c12fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FAA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C12FAA: move.w d0, $4(a4)
pub fn i_c12faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FAE;
    cpu.ir = 0x3940;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12FAE: move.b $ad0(a5), d0
pub fn i_c12fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FB2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C12FB2: move.w d0, (a4)
pub fn i_c12fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FB4;
    cpu.ir = 0x3880;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C12FB4: bra.w $c1376c
pub fn i_c12fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FB8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1376C;
    10
}
/// $C12FB8: move.w $6(a0), d3
pub fn i_c12fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FBC;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12FBC: jsr $c134ca(pc)
pub fn i_c12fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12FC0);
    cpu.pc = 0xC134CA;
    16
}
/// $C12FC0: move.w $6(a4), $6(a0)
pub fn i_c12fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FC6;
    cpu.ir = 0x316C;
    let ad2 = cpu.a[4].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C12FC6: bra.w $c1376c
pub fn i_c12fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FCA;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1376C;
    10
}
/// $C12FCA: jsr $c135ec(pc)
pub fn i_c12fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FCE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12FCE);
    cpu.pc = 0xC135EC;
    16
}
/// $C12FCE: tst.b d0
pub fn i_c12fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FD0;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C12FD0: bne.w $c12f30
pub fn i_c12fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12F30; return 10; }
    8
}
/// $C12FD4: jsr $c1357a(pc)
pub fn i_c12fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12FD8);
    cpu.pc = 0xC1357A;
    16
}
/// $C12FD8: tst.b d0
pub fn i_c12fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FDA;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C12FDA: bne.w $c12f30
pub fn i_c12fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FDE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12F30; return 10; }
    8
}
/// $C12FDE: move.w $2(a0), d0
pub fn i_c12fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FE2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12FE2: move.b d3, d0
pub fn i_c12fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FE4;
    cpu.ir = 0x1003;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C12FE4: move.w d0, $2(a0)
pub fn i_c12fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FE8;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12FE8: move.w d3, d5
pub fn i_c12fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FEA;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12FEA: btst.b #$0, d3
pub fn i_c12fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FEE;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C12FEE: bne.b $c13002
pub fn i_c12fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FF0;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13002; return 10; }
    8
}
/// $C12FF0: move.w (a1, d3.w), d0
pub fn i_c12ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FF4;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C12FF4: andi.w #$ff, d0
pub fn i_c12ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C12FF8: ori.w #$100, d0
pub fn i_c12ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12FFC;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x100) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C12FFC: move.w d0, (a1, d3.w)
pub fn i_c12ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13000;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13000: bra.b $c1300e
pub fn i_c13000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13002;
    cpu.ir = 0x600C;
    cpu.pc = 0xC1300E;
    10
}
/// $C13002: move.w -$1(a1, d3.w), d0
pub fn i_c13002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13006;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13006: move.b #$1, d0
pub fn i_c13006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1300A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1300A: move.w d0, -$1(a1, d3.w)
pub fn i_c1300a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1300E;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C1300E: jsr $c1363c(pc)
pub fn i_c1300e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13012;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13012);
    cpu.pc = 0xC1363C;
    16
}
/// $C13012: moveq #$1f, d6
pub fn i_c13012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13014;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13014: movea.l $ac8(a5), a4
pub fn i_c13014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13018;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C13018: move.w $acc(a5), d7
pub fn i_c13018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1301C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1301C: addq.w #$1, d7
pub fn i_c1301c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1301E;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1301E: lsr.w #$1, d7
pub fn i_c1301e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13020;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13020: move.w (a4)+, (a3)+
pub fn i_c13020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13022;
    cpu.ir = 0x36DC;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C13022: subq.w #$1, d7
pub fn i_c13022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13024;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13024: beq.b $c13092
pub fn i_c13024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13026;
    cpu.ir = 0x676C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13092; return 10; }
    8
}
/// $C13026: dbra d6, $c13020
pub fn i_c13026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1302A;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1302A; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1302A; return 14; }
    cpu.pc = 0xC13020;
    10
}
/// $C1302A: jsr $c1357a(pc)
pub fn i_c1302a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1302E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1302E);
    cpu.pc = 0xC1357A;
    16
}
/// $C1302E: tst.b d0
pub fn i_c1302e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13030;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C13030: beq.b $c1303a
pub fn i_c13030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13032;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1303A; return 10; }
    8
}
/// $C13032: jsr $c13858(pc)
pub fn i_c13032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13036;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13036);
    cpu.pc = 0xC13858;
    16
}
/// $C13036: bra.w $c12f30
pub fn i_c13036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1303A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC12F30;
    10
}
/// $C1303A: btst.b #$0, d3
pub fn i_c1303a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1303E;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1303E: bne.b $c13066
pub fn i_c1303e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13040;
    cpu.ir = 0x6626;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13066; return 10; }
    8
}
/// $C13040: move.w (a1, d3.w), d0
pub fn i_c13040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13044;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13044: andi.w #$ff, d0
pub fn i_c13044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13048;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13048: ori.w #$100, d0
pub fn i_c13048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1304C;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x100) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1304C: move.w d0, (a1, d3.w)
pub fn i_c1304c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13050;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13050: btst.b #$0, d5
pub fn i_c13050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13054;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13054: bne.b $c13078
pub fn i_c13054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13056;
    cpu.ir = 0x6622;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13078; return 10; }
    8
}
/// $C13056: move.w d3, d1
pub fn i_c13056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13058;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13058: lsl.w #$8, d1
pub fn i_c13058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1305A;
    cpu.ir = 0xE149;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C1305A: move.w (a1, d5.w), d0
pub fn i_c1305a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1305E;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1305E: move.b d0, d1
pub fn i_c1305e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13060;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13060: move.w d1, (a1, d5.w)
pub fn i_c13060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13064;
    cpu.ir = 0x3381;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13064: bra.b $c13082
pub fn i_c13064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13066;
    cpu.ir = 0x601C;
    cpu.pc = 0xC13082;
    10
}
/// $C13066: move.w -$1(a1, d3.w), d0
pub fn i_c13066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1306A;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1306A: move.b #$1, d0
pub fn i_c1306a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1306E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1306E: move.w d0, -$1(a1, d3.w)
pub fn i_c1306e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13072;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13072: btst.b #$0, d5
pub fn i_c13072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13076;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13076: beq.b $c13056
pub fn i_c13076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13078;
    cpu.ir = 0x67DE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13056; return 10; }
    8
}
/// $C13078: move.w -$1(a1, d5.w), d0
pub fn i_c13078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1307C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1307C: move.b d3, d0
pub fn i_c1307c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1307E;
    cpu.ir = 0x1003;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1307E: move.w d0, -$1(a1, d5.w)
pub fn i_c1307e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13082;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13082: move.w d3, d5
pub fn i_c13082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13084;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13084: jsr $c1363c(pc)
pub fn i_c13084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13088;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13088);
    cpu.pc = 0xC1363C;
    16
}
/// $C13088: move.b d0, $300001.l
pub fn i_c13088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1308E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1308E: moveq #$1f, d6
pub fn i_c1308e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13090;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13090: bra.b $c13020
pub fn i_c13090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13092;
    cpu.ir = 0x608E;
    cpu.pc = 0xC13020;
    10
}
/// $C13092: movea.l a0, a4
pub fn i_c13092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13094;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $C13094: jsr $c13674(pc)
pub fn i_c13094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13098;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13098);
    cpu.pc = 0xC13674;
    16
}
/// $C13098: tst.w d3
pub fn i_c13098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1309A;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C1309A: beq.b $c130bc
pub fn i_c1309a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1309C;
    cpu.ir = 0x6720;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC130BC; return 10; }
    8
}
/// $C1309C: move.b $ad0(a5), d0
pub fn i_c1309c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130A0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C130A0: lsl.w #$8, d0
pub fn i_c130a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130A2;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C130A2: move.b $ace(a5), d0
pub fn i_c130a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130A6;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C130A6: move.w d0, (a4)
pub fn i_c130a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130A8;
    cpu.ir = 0x3880;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C130A8: move.b $acf(a5), d0
pub fn i_c130a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130AC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C130AC: lsl.w #$8, d0
pub fn i_c130ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130AE;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C130AE: move.w $2(a4), d1
pub fn i_c130ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130B2;
    cpu.ir = 0x322C;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C130B2: move.b d1, d0
pub fn i_c130b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130B4;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C130B4: move.w d0, $2(a4)
pub fn i_c130b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130B8;
    cpu.ir = 0x3940;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C130B8: bra.w $c13820
pub fn i_c130b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130BC;
    cpu.ir = 0x6000;
    cpu.pc = 0xC13820;
    10
}
/// $C130BC: move.w $2(a0), d3
pub fn i_c130bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130C0;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C130C0: jsr $c134e4(pc)
pub fn i_c130c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC130C4);
    cpu.pc = 0xC134E4;
    16
}
/// $C130C4: move.w $2(a4), d0
pub fn i_c130c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130C8;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C130C8: move.w $2(a0), d1
pub fn i_c130c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130CC;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C130CC: move.b d0, d1
pub fn i_c130cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130CE;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C130CE: move.w d1, $2(a0)
pub fn i_c130ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130D2;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C130D2: bra.w $c13820
pub fn i_c130d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130D6;
    cpu.ir = 0x6000;
    cpu.pc = 0xC13820;
    10
}
/// $C130D6: jsr $c135fc(pc)
pub fn i_c130d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC130DA);
    cpu.pc = 0xC135FC;
    16
}
/// $C130DA: tst.b d0
pub fn i_c130da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130DC;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C130DC: bne.w $c12f30
pub fn i_c130dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12F30; return 10; }
    8
}
/// $C130E0: jsr $c135aa(pc)
pub fn i_c130e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC130E4);
    cpu.pc = 0xC135AA;
    16
}
/// $C130E4: tst.b d0
pub fn i_c130e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130E6;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C130E6: bne.w $c12f30
pub fn i_c130e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12F30; return 10; }
    8
}
/// $C130EA: move.w $4(a0), d0
pub fn i_c130ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130EE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C130EE: move.b d3, d0
pub fn i_c130ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130F0;
    cpu.ir = 0x1003;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C130F0: move.w d0, $4(a0)
pub fn i_c130f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130F4;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C130F4: move.w d3, d5
pub fn i_c130f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130F6;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C130F6: add.w d3, d3
pub fn i_c130f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130F8;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C130F8: btst.b #$1, d3
pub fn i_c130f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130FC;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C130FC: bne.b $c13110
pub fn i_c130fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC130FE;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13110; return 10; }
    8
}
/// $C130FE: move.w (a1, d3.w), d0
pub fn i_c130fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13102;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13102: andi.w #$ff, d0
pub fn i_c13102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13106;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13106: ori.w #$100, d0
pub fn i_c13106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1310A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x100) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1310A: move.w d0, (a1, d3.w)
pub fn i_c1310a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1310E;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C1310E: bra.b $c1311c
pub fn i_c1310e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13110;
    cpu.ir = 0x600C;
    cpu.pc = 0xC1311C;
    10
}
/// $C13110: move.w (a1, d3.w), d0
pub fn i_c13110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13114;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13114: move.b #$1, d0
pub fn i_c13114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13118;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13118: move.w d0, (a1, d3.w)
pub fn i_c13118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1311C;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C1311C: lsr.w #$1, d3
pub fn i_c1311c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1311E;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C1311E: jsr $c13664(pc)
pub fn i_c1311e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13122;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13122);
    cpu.pc = 0xC13664;
    16
}
/// $C13122: moveq #$1f, d6
pub fn i_c13122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13124;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13124: movea.l $ac8(a5), a4
pub fn i_c13124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13128;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C13128: move.w $acc(a5), d7
pub fn i_c13128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1312C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1312C: addq.w #$1, d7
pub fn i_c1312c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1312E;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1312E: lsr.w #$1, d7
pub fn i_c1312e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13130;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13130: move.w (a4)+, (a3)
pub fn i_c13130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13132;
    cpu.ir = 0x369C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[3], v3 as u16);
    12
}
/// $C13132: addq.l #$4, a3
pub fn i_c13132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13134;
    cpu.ir = 0x588B;
    cpu.a[3] = cpu.a[3].wrapping_add(4);
    8
}
/// $C13134: subq.w #$1, d7
pub fn i_c13134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13136;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13136: beq.b $c131ae
pub fn i_c13136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13138;
    cpu.ir = 0x6776;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC131AE; return 10; }
    8
}
/// $C13138: dbra d6, $c13130
pub fn i_c13138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1313C;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1313C; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1313C; return 14; }
    cpu.pc = 0xC13130;
    10
}
/// $C1313C: jsr $c135aa(pc)
pub fn i_c1313c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13140;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13140);
    cpu.pc = 0xC135AA;
    16
}
/// $C13140: tst.b d0
pub fn i_c13140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13142;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C13142: beq.b $c1314c
pub fn i_c13142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13144;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1314C; return 10; }
    8
}
/// $C13144: jsr $c13920(pc)
pub fn i_c13144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13148;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13148);
    cpu.pc = 0xC13920;
    16
}
/// $C13148: bra.w $c12f30
pub fn i_c13148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1314C;
    cpu.ir = 0x6000;
    cpu.pc = 0xC12F30;
    10
}
/// $C1314C: add.w d3, d3
pub fn i_c1314c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1314E;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1314E: add.w d5, d5
pub fn i_c1314e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13150;
    cpu.ir = 0xDA45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C13150: btst.b #$1, d3
pub fn i_c13150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13154;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13154: bne.b $c1317c
pub fn i_c13154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13156;
    cpu.ir = 0x6626;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1317C; return 10; }
    8
}
/// $C13156: move.w (a1, d3.w), d0
pub fn i_c13156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1315A;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1315A: andi.w #$ff, d0
pub fn i_c1315a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1315E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1315E: ori.w #$100, d0
pub fn i_c1315e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13162;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x100) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13162: move.w d0, (a1, d3.w)
pub fn i_c13162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13166;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13166: btst.b #$1, d5
pub fn i_c13166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1316A;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1316A: bne.b $c1318e
pub fn i_c1316a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1316C;
    cpu.ir = 0x6622;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1318E; return 10; }
    8
}
/// $C1316C: move.w d3, d1
pub fn i_c1316c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1316E;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1316E: lsl.w #$7, d1
pub fn i_c1316e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13170;
    cpu.ir = 0xEF49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $C13170: move.w (a1, d5.w), d0
pub fn i_c13170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13174;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13174: move.b d0, d1
pub fn i_c13174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13176;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13176: move.w d1, (a1, d5.w)
pub fn i_c13176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1317A;
    cpu.ir = 0x3381;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C1317A: bra.b $c1319c
pub fn i_c1317a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1317C;
    cpu.ir = 0x6020;
    cpu.pc = 0xC1319C;
    10
}
/// $C1317C: move.w (a1, d3.w), d0
pub fn i_c1317c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13180;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13180: move.b #$1, d0
pub fn i_c13180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13184;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13184: move.w d0, (a1, d3.w)
pub fn i_c13184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13188;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13188: btst.b #$1, d5
pub fn i_c13188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1318C;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1318C: beq.b $c1316c
pub fn i_c1318c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1318E;
    cpu.ir = 0x67DE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1316C; return 10; }
    8
}
/// $C1318E: move.w (a1, d5.w), d0
pub fn i_c1318e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13192;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13192: lsr.w #$1, d3
pub fn i_c13192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13194;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13194: move.b d3, d0
pub fn i_c13194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13196;
    cpu.ir = 0x1003;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13196: add.w d3, d3
pub fn i_c13196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13198;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C13198: move.w d0, (a1, d5.w)
pub fn i_c13198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1319C;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C1319C: lsr.w #$1, d3
pub fn i_c1319c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1319E;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C1319E: move.w d3, d5
pub fn i_c1319e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131A0;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C131A0: jsr $c13664(pc)
pub fn i_c131a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC131A4);
    cpu.pc = 0xC13664;
    16
}
/// $C131A4: move.b d0, $300001.l
pub fn i_c131a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131AA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C131AA: moveq #$1f, d6
pub fn i_c131aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131AC;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C131AC: bra.b $c13130
pub fn i_c131ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131AE;
    cpu.ir = 0x6082;
    cpu.pc = 0xC13130;
    10
}
/// $C131AE: movea.l a0, a4
pub fn i_c131ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131B0;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $C131B0: jsr $c136ee(pc)
pub fn i_c131b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC131B4);
    cpu.pc = 0xC136EE;
    16
}
/// $C131B4: tst.w d3
pub fn i_c131b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131B6;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C131B6: beq.b $c131d8
pub fn i_c131b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131B8;
    cpu.ir = 0x6720;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC131D8; return 10; }
    8
}
/// $C131B8: move.b $ad0(a5), d0
pub fn i_c131b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131BC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C131BC: lsl.w #$8, d0
pub fn i_c131bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131BE;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C131BE: move.b $ace(a5), d0
pub fn i_c131be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131C2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C131C2: move.w d0, (a4)
pub fn i_c131c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131C4;
    cpu.ir = 0x3880;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C131C4: move.b $acf(a5), d0
pub fn i_c131c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131C8;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C131C8: lsl.w #$8, d0
pub fn i_c131c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131CA;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C131CA: move.w $4(a4), d1
pub fn i_c131ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131CE;
    cpu.ir = 0x322C;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C131CE: move.b d1, d0
pub fn i_c131ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131D0;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C131D0: move.w d0, $4(a4)
pub fn i_c131d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131D4;
    cpu.ir = 0x3940;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C131D4: bra.w $c138de
pub fn i_c131d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131D8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC138DE;
    10
}
/// $C131D8: move.w $4(a0), d3
pub fn i_c131d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131DC;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C131DC: jsr $c1351e(pc)
pub fn i_c131dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC131E0);
    cpu.pc = 0xC1351E;
    16
}
/// $C131E0: move.w $4(a4), d0
pub fn i_c131e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131E4;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C131E4: move.w $4(a0), d1
pub fn i_c131e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131E8;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C131E8: move.b d0, d1
pub fn i_c131e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131EA;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C131EA: move.w d1, $4(a0)
pub fn i_c131ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131EE;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C131EE: bra.w $c138de
pub fn i_c131ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131F2;
    cpu.ir = 0x6000;
    cpu.pc = 0xC138DE;
    10
}
/// $C131F2: move.b $380000.l, d0
pub fn i_c131f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131F8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C131F8: andi.b #$40, d0
pub fn i_c131f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131FC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C131FC: beq.b $c13206
pub fn i_c131fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC131FE;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13206; return 10; }
    8
}
/// $C131FE: move.b #$85, $ac6(a5)
pub fn i_c131fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13204;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13204: rts
pub fn i_c13204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13206;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13206: jsr $c13978(pc)
pub fn i_c13206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1320A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1320A);
    cpu.pc = 0xC13978;
    16
}
/// $C1320A: tst.b $ac6(a5)
pub fn i_c1320a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1320E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1320E: beq.b $c13212
pub fn i_c1320e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13210;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13212; return 10; }
    8
}
/// $C13210: rts
pub fn i_c13210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13212;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13212: jsr $c13726(pc)
pub fn i_c13212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13216;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13216);
    cpu.pc = 0xC13726;
    16
}
/// $C13216: tst.b $ac6(a5)
pub fn i_c13216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1321A;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1321A: bne.b $c13210
pub fn i_c1321a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1321C;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13210; return 10; }
    8
}
/// $C1321C: tst.b $a15(a5)
pub fn i_c1321c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13220;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C13220: bmi.w $c13262
pub fn i_c13220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13224;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC13262; return 10; }
    8
}
/// $C13224: jsr $c13674(pc)
pub fn i_c13224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13228;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13228);
    cpu.pc = 0xC13674;
    16
}
/// $C13228: tst.w d3
pub fn i_c13228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1322A;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C1322A: beq.b $c13234
pub fn i_c1322a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1322C;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13234; return 10; }
    8
}
/// $C1322C: move.b #$82, $ac6(a5)
pub fn i_c1322c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13232;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13232: rts
pub fn i_c13232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13234;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13234: jsr $c13e58(pc)
pub fn i_c13234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13238;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13238);
    cpu.pc = 0xC13E58;
    16
}
/// $C13238: movea.l $a0a(a5), a1
pub fn i_c13238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1323C;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C1323C: tst.b $a15(a5)
pub fn i_c1323c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13240;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C13240: bne.b $c13252
pub fn i_c13240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13242;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13252; return 10; }
    8
}
/// $C13242: move.w #$ff, (a0)
pub fn i_c13242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13246;
    cpu.ir = 0x30BC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13246: move.w $6(a0), d3
pub fn i_c13246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1324A;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1324A: jsr $c134ca(pc)
pub fn i_c1324a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1324E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1324E);
    cpu.pc = 0xC134CA;
    16
}
/// $C1324E: bra.w $c1376c
pub fn i_c1324e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13252;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1376C;
    10
}
/// $C13252: move.w #$ff00, (a0)
pub fn i_c13252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13256;
    cpu.ir = 0x30BC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13256: move.w $2(a0), d3
pub fn i_c13256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1325A;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1325A: jsr $c134e4(pc)
pub fn i_c1325a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1325E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1325E);
    cpu.pc = 0xC134E4;
    16
}
/// $C1325E: bra.w $c13820
pub fn i_c1325e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13262;
    cpu.ir = 0x6000;
    cpu.pc = 0xC13820;
    10
}
/// $C13262: jsr $c136ee(pc)
pub fn i_c13262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13266;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13266);
    cpu.pc = 0xC136EE;
    16
}
/// $C13266: tst.w d3
pub fn i_c13266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13268;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C13268: beq.b $c13272
pub fn i_c13268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1326A;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13272; return 10; }
    8
}
/// $C1326A: move.b #$82, $ac6(a5)
pub fn i_c1326a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13270;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13270: rts
pub fn i_c13270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13272;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13272: jsr $c13e58(pc)
pub fn i_c13272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13276;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13276);
    cpu.pc = 0xC13E58;
    16
}
/// $C13276: movea.l $a0a(a5), a1
pub fn i_c13276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1327A;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C1327A: move.w #$ff00, (a0)
pub fn i_c1327a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1327E;
    cpu.ir = 0x30BC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C1327E: move.w $4(a0), d3
pub fn i_c1327e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13282;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13282: jsr $c1351e(pc)
pub fn i_c13282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13286;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13286);
    cpu.pc = 0xC1351E;
    16
}
/// $C13286: bra.w $c138de
pub fn i_c13286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1328A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC138DE;
    10
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
/// $C133E0: jsr $c13978(pc)
pub fn i_c133e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC133E4);
    cpu.pc = 0xC13978;
    16
}
/// $C133E4: tst.b $ac6(a5)
pub fn i_c133e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133E8;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C133E8: bne.b $c133fc
pub fn i_c133e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133EA;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC133FC; return 10; }
    8
}
/// $C133EA: move.b $380000.l, d0
pub fn i_c133ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133F0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C133F0: andi.b #$40, d0
pub fn i_c133f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133F4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C133F4: beq.b $c133fe
pub fn i_c133f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133F6;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC133FE; return 10; }
    8
}
/// $C133F6: move.b #$85, $ac6(a5)
pub fn i_c133f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133FC;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C133FC: rts
pub fn i_c133fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC133FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C133FE: jsr $c13e58(pc)
pub fn i_c133fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13402;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13402);
    cpu.pc = 0xC13E58;
    16
}
/// $C13402: tst.b $a15(a5)
pub fn i_c13402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13406;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C13406: bmi.w $c13422
pub fn i_c13406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1340A;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC13422; return 10; }
    8
}
/// $C1340A: movea.l $ac8(a5), a4
pub fn i_c1340a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1340E;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C1340E: lea.l $1e(a6), a3
pub fn i_c1340e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13412;
    cpu.ir = 0x47EE;
    let ad1 = cpu.a[6].wrapping_add(0x1E);
    cpu.a[3] = ad1;
    16
}
/// $C13412: move.w #$ff, (a3)+
pub fn i_c13412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13416;
    cpu.ir = 0x36FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13416: moveq #$f, d7
pub fn i_c13416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13418;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13418: move.b (a4)+, d0
pub fn i_c13418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1341A;
    cpu.ir = 0x101C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1341A: move.w d0, (a3)+
pub fn i_c1341a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1341C;
    cpu.ir = 0x36C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1341C: dbra d7, $c13418
pub fn i_c1341c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13420;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13420; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13420; return 14; }
    cpu.pc = 0xC13418;
    10
}
/// $C13420: rts
pub fn i_c13420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13422;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13422: movea.l $ac8(a5), a4
pub fn i_c13422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13426;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C13426: move.w $18(a6), d0
pub fn i_c13426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1342A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1342A: ori.w #$ff00, d0
pub fn i_c1342a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1342E;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xFF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1342E: move.w d0, $18(a6)
pub fn i_c1342e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13432;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13432: lea.l $20(a6), a3
pub fn i_c13432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13436;
    cpu.ir = 0x47EE;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    cpu.a[3] = ad1;
    16
}
/// $C13436: moveq #$7, d7
pub fn i_c13436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13438;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13438: move.w (a4)+, d0
pub fn i_c13438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1343A;
    cpu.ir = 0x301C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C1343A: move.w d0, (a3)
pub fn i_c1343a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1343C;
    cpu.ir = 0x3680;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[3], v1 as u16);
    8
}
/// $C1343C: addq.l #$4, a3
pub fn i_c1343c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1343E;
    cpu.ir = 0x588B;
    cpu.a[3] = cpu.a[3].wrapping_add(4);
    8
}
/// $C1343E: dbra d7, $c13438
pub fn i_c1343e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13442;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13442; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13442; return 14; }
    cpu.pc = 0xC13438;
    10
}
/// $C13442: rts
pub fn i_c13442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13444;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13444: jsr $c13978(pc)
pub fn i_c13444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13448;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13448);
    cpu.pc = 0xC13978;
    16
}
/// $C13448: tst.b $ac6(a5)
pub fn i_c13448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1344C;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1344C: bne.b $c133fc
pub fn i_c1344c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1344E;
    cpu.ir = 0x66AE;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC133FC; return 10; }
    8
}
/// $C1344E: tst.b $a15(a5)
pub fn i_c1344e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13452;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C13452: bmi.w $c13480
pub fn i_c13452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13456;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC13480; return 10; }
    8
}
/// $C13456: movea.l $ac8(a5), a4
pub fn i_c13456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1345A;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C1345A: lea.l $1e(a6), a3
pub fn i_c1345a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1345E;
    cpu.ir = 0x47EE;
    let ad1 = cpu.a[6].wrapping_add(0x1E);
    cpu.a[3] = ad1;
    16
}
/// $C1345E: move.w (a3)+, d0
pub fn i_c1345e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13460;
    cpu.ir = 0x301B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13460: tst.b d0
pub fn i_c13460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13462;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C13462: bne.b $c1346c
pub fn i_c13462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13464;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1346C; return 10; }
    8
}
/// $C13464: move.b #$82, $ac6(a5)
pub fn i_c13464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1346A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1346A: rts
pub fn i_c1346a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1346C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1346C: moveq #$f, d7
pub fn i_c1346c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1346E;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1346E: move.w (a3)+, d0
pub fn i_c1346e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13470;
    cpu.ir = 0x301B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13470: cmpi.b #$ff, d0
pub fn i_c13470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13474;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C13474: bne.b $c13478
pub fn i_c13474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13476;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13478; return 10; }
    8
}
/// $C13476: clr.b d0
pub fn i_c13476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13478;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C13478: move.b d0, (a4)+
pub fn i_c13478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1347A;
    cpu.ir = 0x18C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1347A: dbra d7, $c1346e
pub fn i_c1347a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1347E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1347E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1347E; return 14; }
    cpu.pc = 0xC1346E;
    10
}
/// $C1347E: rts
pub fn i_c1347e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13480;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13480: movea.l $ac8(a5), a4
pub fn i_c13480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13484;
    cpu.ir = 0x286D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $C13484: move.w $18(a6), d0
pub fn i_c13484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13488;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13488: andi.w #$ff00, d0
pub fn i_c13488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1348C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1348C: bne.b $c13496
pub fn i_c1348c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1348E;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13496; return 10; }
    8
}
/// $C1348E: move.b #$82, $ac6(a5)
pub fn i_c1348e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13494;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C13494: rts
pub fn i_c13494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13496;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13496: lea.l $20(a6), a3
pub fn i_c13496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1349A;
    cpu.ir = 0x47EE;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    cpu.a[3] = ad1;
    16
}
/// $C1349A: moveq #$7, d7
pub fn i_c1349a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1349C;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1349C: move.w (a3), d0
pub fn i_c1349c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1349E;
    cpu.ir = 0x3013;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1349E: cmpi.b #$ff, d0
pub fn i_c1349e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134A2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C134A2: bne.b $c134a6
pub fn i_c134a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134A4;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC134A6; return 10; }
    8
}
/// $C134A4: clr.b d0
pub fn i_c134a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134A6;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C134A6: addi.w #$100, d0
pub fn i_c134a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134AA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C134AA: bcs.b $c134b0
pub fn i_c134aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134AC;
    cpu.ir = 0x6504;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC134B0; return 10; }
    8
}
/// $C134AC: subi.w #$100, d0
pub fn i_c134ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134B0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C134B0: move.w d0, (a4)+
pub fn i_c134b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134B2;
    cpu.ir = 0x38C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C134B2: addq.l #$4, a3
pub fn i_c134b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134B4;
    cpu.ir = 0x588B;
    cpu.a[3] = cpu.a[3].wrapping_add(4);
    8
}
/// $C134B4: dbra d7, $c1349c
pub fn i_c134b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134B8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC134B8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC134B8; return 14; }
    cpu.pc = 0xC1349C;
    10
}
/// $C134B8: rts
pub fn i_c134b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C134BA: trap #$5
pub fn i_c134ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134BC;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C134BC: dc.w $4f2d
pub fn i_c134bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134BE;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C134BE: ?
pub fn i_c134be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134C0;
    cpu.ir = 0x4745;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C134C0: chk.w d0, d7
pub fn i_c134c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134C2;
    cpu.ir = 0x4F80;
    let v1 = cpu.d[0] & 0xFFFF;
    let bd = v1 as u16 as i16 as i32; let vv = cpu.d[7] as u16 as i16 as i32;
    cpu.sr.set(StatusRegister::Z, false); cpu.sr.set(StatusRegister::V, false); cpu.sr.set(StatusRegister::C, false);
    if vv < 0 { cpu.sr.set(StatusRegister::N, true); cpu.enter_exception(bus, Exception::Chk); return 40; }
    if vv > bd { cpu.sr.set(StatusRegister::N, false); cpu.enter_exception(bus, Exception::Chk); return 40; }
    cpu.sr.set(StatusRegister::N, false);
    10
}
/// $C134C2: subq.w #$1, a6
pub fn i_c134c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134C4;
    cpu.ir = 0x534E;
    cpu.a[6] = cpu.a[6].wrapping_sub(1);
    8
}
/// $C134C4: dc.w $4b20
pub fn i_c134c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134C6;
    cpu.ir = 0x4B20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C134C6: addq.w #$1, a7
pub fn i_c134c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134C8;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C134C8: dc.w $4d20
pub fn i_c134c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134CA;
    cpu.ir = 0x4D20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C134CA: andi.w #$ff, d3
pub fn i_c134ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134CE;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C134CE: add.w d3, d3
pub fn i_c134ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134D0;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C134D0: move.w (a1, d3.w), d4
pub fn i_c134d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134D4;
    cpu.ir = 0x3831;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C134D4: move.w #$0, (a1, d3.w)
pub fn i_c134d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134DA;
    cpu.ir = 0x33BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C134DA: move.w d4, d3
pub fn i_c134da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134DC;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C134DC: cmpi.b #$3, d4
pub fn i_c134dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134E0;
    cpu.ir = 0xC04;
    let v1 = cpu.d[4] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C134E0: bcc.b $c134ca
pub fn i_c134e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134E2;
    cpu.ir = 0x64E8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC134CA; return 10; }
    8
}
/// $C134E2: rts
pub fn i_c134e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C134E4: andi.w #$ff, d3
pub fn i_c134e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134E8;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C134E8: btst.b #$0, d3
pub fn i_c134e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134EC;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C134EC: bne.b $c13508
pub fn i_c134ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134EE;
    cpu.ir = 0x661A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13508; return 10; }
    8
}
/// $C134EE: move.w (a1, d3.w), d4
pub fn i_c134ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134F2;
    cpu.ir = 0x3831;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C134F2: move.w d4, d0
pub fn i_c134f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134F4;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C134F4: andi.w #$ff, d0
pub fn i_c134f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C134F8: move.w d0, (a1, d3.w)
pub fn i_c134f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134FC;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C134FC: lsr.w #$8, d4
pub fn i_c134fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC134FE;
    cpu.ir = 0xE04C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C134FE: move.w d4, d3
pub fn i_c134fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13500;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13500: cmpi.b #$3, d3
pub fn i_c13500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13504;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C13504: bcc.b $c134e4
pub fn i_c13504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13506;
    cpu.ir = 0x64DE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC134E4; return 10; }
    8
}
/// $C13506: rts
pub fn i_c13506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13508;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13508: move.w -$1(a1, d3.w), d4
pub fn i_c13508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1350C;
    cpu.ir = 0x3831;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1350C: move.w d4, d0
pub fn i_c1350c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1350E;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1350E: clr.b d0
pub fn i_c1350e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13510;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C13510: move.w d0, -$1(a1, d3.w)
pub fn i_c13510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13514;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13514: move.w d4, d3
pub fn i_c13514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13516;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13516: cmpi.b #$3, d3
pub fn i_c13516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1351A;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C1351A: bcc.b $c134e4
pub fn i_c1351a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1351C;
    cpu.ir = 0x64C8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC134E4; return 10; }
    8
}
/// $C1351C: rts
pub fn i_c1351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1351E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1351E: andi.w #$ff, d3
pub fn i_c1351e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13522;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13522: add.w d3, d3
pub fn i_c13522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13524;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C13524: btst.b #$1, d3
pub fn i_c13524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13528;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13528: bne.b $c13544
pub fn i_c13528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1352A;
    cpu.ir = 0x661A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13544; return 10; }
    8
}
/// $C1352A: move.w (a1, d3.w), d4
pub fn i_c1352a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1352E;
    cpu.ir = 0x3831;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1352E: move.w d4, d0
pub fn i_c1352e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13530;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13530: andi.w #$ff, d0
pub fn i_c13530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13534;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13534: move.w d0, (a1, d3.w)
pub fn i_c13534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13538;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13538: lsr.w #$8, d4
pub fn i_c13538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1353A;
    cpu.ir = 0xE04C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C1353A: move.w d4, d3
pub fn i_c1353a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1353C;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1353C: cmpi.b #$3, d3
pub fn i_c1353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13540;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C13540: bcc.b $c1351e
pub fn i_c13540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13542;
    cpu.ir = 0x64DC;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1351E; return 10; }
    8
}
/// $C13542: rts
pub fn i_c13542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13544;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13544: move.w (a1, d3.w), d4
pub fn i_c13544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13548;
    cpu.ir = 0x3831;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C13548: move.w d4, d0
pub fn i_c13548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1354A;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1354A: clr.b d0
pub fn i_c1354a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1354C;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1354C: move.w d0, (a1, d3.w)
pub fn i_c1354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13550;
    cpu.ir = 0x3380;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C13550: move.w d4, d3
pub fn i_c13550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13552;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13552: cmpi.b #$3, d3
pub fn i_c13552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13556;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $C13556: bcc.b $c1351e
pub fn i_c13556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13558;
    cpu.ir = 0x64C6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1351E; return 10; }
    8
}
/// $C13558: rts
pub fn i_c13558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1355A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1355A: swap d7
pub fn i_c1355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1355C;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1355C: moveq #$0, d3
pub fn i_c1355c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1355E;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1355E: move.w $a08(a5), d7
pub fn i_c1355e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13562;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13562: move.b d0, $300001.l
pub fn i_c13562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13568;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13568: move.w (a1, d3.w), d0
pub fn i_c13568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1356C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1356C: tst.b d0
pub fn i_c1356c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1356E;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1356E: beq.b $c13576
pub fn i_c1356e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13570;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13576; return 10; }
    8
}
/// $C13570: addq.w #$2, d3
pub fn i_c13570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13572;
    cpu.ir = 0x5443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13572: dbra d7, $c13562
pub fn i_c13572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13576;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13576; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13576; return 14; }
    cpu.pc = 0xC13562;
    10
}
/// $C13576: swap d7
pub fn i_c13576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13578;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13578: rts
pub fn i_c13578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1357A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1357A: swap d7
pub fn i_c1357a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1357C;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1357C: moveq #$0, d3
pub fn i_c1357c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1357E;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1357E: move.w $a08(a5), d7
pub fn i_c1357e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13582;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C13582: move.b d0, $300001.l
pub fn i_c13582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13588;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13588: move.w (a1, d3.w), d0
pub fn i_c13588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1358C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1358C: move.w d0, d1
pub fn i_c1358c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1358E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1358E: lsr.w #$8, d0
pub fn i_c1358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13590;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13590: beq.b $c135a6
pub fn i_c13590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13592;
    cpu.ir = 0x6714;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC135A6; return 10; }
    8
}
/// $C13592: addq.w #$1, d3
pub fn i_c13592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13594;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13594: dbra d7, $c1359c
pub fn i_c13594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13598;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13598; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13598; return 14; }
    cpu.pc = 0xC1359C;
    10
}
/// $C13598: swap d7
pub fn i_c13598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1359A;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1359A: rts
pub fn i_c1359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1359C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1359C: move.b d1, d0
pub fn i_c1359c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1359E;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1359E: beq.b $c135a6
pub fn i_c1359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135A0;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC135A6; return 10; }
    8
}
/// $C135A0: addq.w #$1, d3
pub fn i_c135a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135A2;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C135A2: dbra d7, $c13582
pub fn i_c135a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135A6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC135A6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC135A6; return 14; }
    cpu.pc = 0xC13582;
    10
}
/// $C135A6: swap d7
pub fn i_c135a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135A8;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C135A8: rts
pub fn i_c135a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C135AA: swap d7
pub fn i_c135aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135AC;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C135AC: moveq #$0, d3
pub fn i_c135ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135AE;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C135AE: move.w $a08(a5), d7
pub fn i_c135ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135B2;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C135B2: move.b d0, $300001.l
pub fn i_c135b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135B8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C135B8: add.w d3, d3
pub fn i_c135b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135BA;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C135BA: move.w (a1, d3.w), d0
pub fn i_c135ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135BE;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C135BE: lsr.w #$1, d3
pub fn i_c135be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135C0;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C135C0: move.w d0, d1
pub fn i_c135c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135C2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C135C2: lsr.w #$8, d0
pub fn i_c135c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135C4;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C135C4: beq.b $c135da
pub fn i_c135c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135C6;
    cpu.ir = 0x6714;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC135DA; return 10; }
    8
}
/// $C135C6: addq.w #$1, d3
pub fn i_c135c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135C8;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C135C8: dbra d7, $c135d0
pub fn i_c135c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135CC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC135CC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC135CC; return 14; }
    cpu.pc = 0xC135D0;
    10
}
/// $C135CC: swap d7
pub fn i_c135cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135CE;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C135CE: rts
pub fn i_c135ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C135D0: move.b d1, d0
pub fn i_c135d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135D2;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C135D2: beq.b $c135da
pub fn i_c135d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135D4;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC135DA; return 10; }
    8
}
/// $C135D4: addq.w #$1, d3
pub fn i_c135d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135D6;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C135D6: dbra d7, $c135b2
pub fn i_c135d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135DA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC135DA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC135DA; return 14; }
    cpu.pc = 0xC135B2;
    10
}
/// $C135DA: swap d7
pub fn i_c135da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135DC;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C135DC: rts
pub fn i_c135dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C135DE: move.w (a0), d0
pub fn i_c135de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135E0;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C135E0: addq.b #$1, d0
pub fn i_c135e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135E2;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C135E2: beq.b $c135ea
pub fn i_c135e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135E4;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC135EA; return 10; }
    8
}
/// $C135E4: addq.l #$8, a0
pub fn i_c135e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135E6;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C135E6: dbra d7, $c135de
pub fn i_c135e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135EA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC135EA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC135EA; return 14; }
    cpu.pc = 0xC135DE;
    10
}
/// $C135EA: rts
pub fn i_c135ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C135EC: move.w (a0), d0
pub fn i_c135ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135EE;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C135EE: lsr.w #$8, d0
pub fn i_c135ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135F0;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C135F0: addq.b #$1, d0
pub fn i_c135f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135F2;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C135F2: beq.b $c135ea
pub fn i_c135f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135F4;
    cpu.ir = 0x67F6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC135EA; return 10; }
    8
}
/// $C135F4: addq.l #$4, a0
pub fn i_c135f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135F6;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C135F6: dbra d7, $c135ec
pub fn i_c135f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135FA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC135FA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC135FA; return 14; }
    cpu.pc = 0xC135EC;
    10
}
/// $C135FA: rts
pub fn i_c135fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C135FC: move.w (a0), d0
pub fn i_c135fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC135FE;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C135FE: lsr.w #$8, d0
pub fn i_c135fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13600;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13600: addq.b #$1, d0
pub fn i_c13600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13602;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C13602: beq.b $c135ea
pub fn i_c13602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13604;
    cpu.ir = 0x67E6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC135EA; return 10; }
    8
}
/// $C13604: addq.l #$8, a0
pub fn i_c13604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13606;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C13606: dbra d7, $c135fc
pub fn i_c13606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1360A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1360A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1360A; return 14; }
    cpu.pc = 0xC135FC;
    10
}
/// $C1360A: rts
pub fn i_c1360a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1360C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
/// $C13A0C: movea.l a6, a0
pub fn i_c13a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A0E;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13A0E: moveq #$0, d6
pub fn i_c13a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A10;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13A10: move.w d6, $a14(a5)
pub fn i_c13a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A14;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13A14: lea.l $c134c2(pc), a1
pub fn i_c13a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC134C2;
    16
}
/// $C13A18: moveq #$3, d7
pub fn i_c13a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A1A;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13A1A: cmpm.w (a0)+, (a1)+
pub fn i_c13a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A1C;
    cpu.ir = 0xB348;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(2);
    let v1 = bus.read16(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read16(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(2);
    sub_with_flags(cpu, v2, v1, Size::Word, false);
    12
}
/// $C13A1C: bne.b $c13a32
pub fn i_c13a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A1E;
    cpu.ir = 0x6614;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13A32; return 10; }
    8
}
/// $C13A1E: dbra d7, $c13a1a
pub fn i_c13a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A22;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13A22; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13A22; return 14; }
    cpu.pc = 0xC13A1A;
    10
}
/// $C13A22: move.w (a1)+, d6
pub fn i_c13a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A24;
    cpu.ir = 0x3C19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13A24: bne.b $c13a28
pub fn i_c13a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A26;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13A28; return 10; }
    8
}
/// $C13A26: rts
pub fn i_c13a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13A28: adda.l (a1)+, a6
pub fn i_c13a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A2A;
    cpu.ir = 0xDDD9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = cpu.a[6].wrapping_add(v3);
    16
}
/// $C13A2A: move.w (a1)+, d6
pub fn i_c13a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A2C;
    cpu.ir = 0x3C19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13A2C: move.w d6, $a14(a5)
pub fn i_c13a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A30;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13A30: rts
pub fn i_c13a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13A32: jsr $c13e58(pc)
pub fn i_c13a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13A36);
    cpu.pc = 0xC13E58;
    16
}
/// $C13A36: movea.l a6, a0
pub fn i_c13a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A38;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13A38: move.w #$aaaa, (a0)
pub fn i_c13a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A3C;
    cpu.ir = 0x30BC;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13A3C: move.w #$5555, $4(a0)
pub fn i_c13a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A42;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13A42: move.w (a0), d0
pub fn i_c13a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A44;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13A44: cmpi.w #$aaaa, d0
pub fn i_c13a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A48;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xAAAA, Size::Word, false);
    8
}
/// $C13A48: beq.w $c13b3c
pub fn i_c13a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13B3C; return 10; }
    8
}
/// $C13A4C: cmpi.b #$aa, d0
pub fn i_c13a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A50;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xAA, Size::Byte, false);
    8
}
/// $C13A50: beq.b $c13a54
pub fn i_c13a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A52;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13A54; return 10; }
    8
}
/// $C13A52: rts
pub fn i_c13a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13A54: move.w #$55, (a0)
pub fn i_c13a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A58;
    cpu.ir = 0x30BC;
    let v1: u32 = 0x55;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13A58: move.w (a0), d0
pub fn i_c13a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A5A;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13A5A: cmpi.b #$55, d0
pub fn i_c13a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A5E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x55, Size::Byte, false);
    8
}
/// $C13A5E: beq.b $c13a62
pub fn i_c13a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A60;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13A62; return 10; }
    8
}
/// $C13A60: rts
pub fn i_c13a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13A62: move.l #$1000, d5
pub fn i_c13a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A68;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x1000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13A68: move.b d0, $300001.l
pub fn i_c13a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A6E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13A6E: add.l d5, d6
pub fn i_c13a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A70;
    cpu.ir = 0xDC85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $C13A70: cmpi.w #$8000, d6
pub fn i_c13a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A74;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8000, Size::Word, false);
    8
}
/// $C13A74: beq.b $c13a94
pub fn i_c13a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A76;
    cpu.ir = 0x671E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13A94; return 10; }
    8
}
/// $C13A76: adda.l d5, a0
pub fn i_c13a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A78;
    cpu.ir = 0xD1C5;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C13A78: move.w #$aa, (a0)
pub fn i_c13a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A7C;
    cpu.ir = 0x30BC;
    let v1: u32 = 0xAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13A7C: move.w #$0, (a6)
pub fn i_c13a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A80;
    cpu.ir = 0x3CBC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C13A80: move.w (a0), d0
pub fn i_c13a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A82;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13A82: cmpi.b #$aa, d0
pub fn i_c13a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A86;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xAA, Size::Byte, false);
    8
}
/// $C13A86: bne.b $c13a94
pub fn i_c13a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A88;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13A94; return 10; }
    8
}
/// $C13A88: move.w #$55, (a0)
pub fn i_c13a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A8C;
    cpu.ir = 0x30BC;
    let v1: u32 = 0x55;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13A8C: move.w (a0), d0
pub fn i_c13a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A8E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13A8E: cmpi.b #$55, d0
pub fn i_c13a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A92;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x55, Size::Byte, false);
    8
}
/// $C13A92: beq.b $c13a68
pub fn i_c13a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A94;
    cpu.ir = 0x67D4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13A68; return 10; }
    8
}
/// $C13A94: tst.w d6
pub fn i_c13a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A96;
    cpu.ir = 0x4A46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C13A96: beq.w $c13b38
pub fn i_c13a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13B38; return 10; }
    8
}
/// $C13A9A: move.w d6, d5
pub fn i_c13a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A9C;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13A9C: lsr.w #$1, d5
pub fn i_c13a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13A9E;
    cpu.ir = 0xE24D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13A9E: subq.w #$1, d5
pub fn i_c13a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AA0;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13AA0: movea.l a6, a0
pub fn i_c13aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AA2;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13AA2: move.w d5, d7
pub fn i_c13aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AA4;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13AA4: move.b #$55, d0
pub fn i_c13aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AA8;
    cpu.ir = 0x103C;
    let v1: u32 = 0x55;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13AA8: move.b d0, $300001.l
pub fn i_c13aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AAE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13AAE: move.w d0, (a0)+
pub fn i_c13aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AB0;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13AB0: dbra d7, $c13aa8
pub fn i_c13ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AB4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13AB4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13AB4; return 14; }
    cpu.pc = 0xC13AA8;
    10
}
/// $C13AB4: movea.l a6, a0
pub fn i_c13ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AB6;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13AB6: move.w d5, d7
pub fn i_c13ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AB8;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13AB8: move.b d0, $300001.l
pub fn i_c13ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ABE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13ABE: move.w (a0)+, d1
pub fn i_c13abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AC0;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13AC0: cmp.b d1, d0
pub fn i_c13ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AC2;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C13AC2: bne.w $c13b38
pub fn i_c13ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13B38; return 10; }
    8
}
/// $C13AC6: dbra d7, $c13ab8
pub fn i_c13ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ACA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13ACA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13ACA; return 14; }
    cpu.pc = 0xC13AB8;
    10
}
/// $C13ACA: move.w d5, d7
pub fn i_c13aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ACC;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13ACC: move.b #$aa, d0
pub fn i_c13acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AD0;
    cpu.ir = 0x103C;
    let v1: u32 = 0xAA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13AD0: movea.l a6, a0
pub fn i_c13ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AD2;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13AD2: move.b d0, $300001.l
pub fn i_c13ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AD8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13AD8: move.w d0, (a0)+
pub fn i_c13ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ADA;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13ADA: dbra d7, $c13ad2
pub fn i_c13ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ADE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13ADE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13ADE; return 14; }
    cpu.pc = 0xC13AD2;
    10
}
/// $C13ADE: movea.l a6, a0
pub fn i_c13ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AE0;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13AE0: move.w d5, d7
pub fn i_c13ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AE2;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13AE2: move.b d0, $300001.l
pub fn i_c13ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AE8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13AE8: move.w (a0)+, d1
pub fn i_c13ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AEA;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13AEA: cmp.b d1, d0
pub fn i_c13aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AEC;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C13AEC: bne.w $c13b38
pub fn i_c13aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AF0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13B38; return 10; }
    8
}
/// $C13AF0: dbra d7, $c13ae2
pub fn i_c13af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AF4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13AF4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13AF4; return 14; }
    cpu.pc = 0xC13AE2;
    10
}
/// $C13AF4: movea.l a6, a0
pub fn i_c13af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AF6;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13AF6: addq.w #$1, d5
pub fn i_c13af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AF8;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13AF8: lsr.w #$1, d5
pub fn i_c13af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AFA;
    cpu.ir = 0xE24D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13AFA: subq.w #$1, d5
pub fn i_c13afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AFC;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13AFC: move.w d5, d7
pub fn i_c13afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13AFE;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13AFE: move.b d0, $300001.l
pub fn i_c13afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B04;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13B04: move.w a0, d0
pub fn i_c13b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B06;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13B06: move.w a0, d1
pub fn i_c13b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B08;
    cpu.ir = 0x3208;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13B08: lsr.w #$8, d0
pub fn i_c13b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B0A;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13B0A: move.w d0, (a0)+
pub fn i_c13b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B0C;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13B0C: move.w d1, (a0)+
pub fn i_c13b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B0E;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13B0E: dbra d7, $c13afe
pub fn i_c13b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B12;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13B12; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13B12; return 14; }
    cpu.pc = 0xC13AFE;
    10
}
/// $C13B12: movea.l a6, a0
pub fn i_c13b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B14;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13B14: move.w d5, d7
pub fn i_c13b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B16;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13B16: move.b d0, $300001.l
pub fn i_c13b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B1C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13B1C: move.w a0, d2
pub fn i_c13b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B1E;
    cpu.ir = 0x3408;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13B1E: move.w d2, d3
pub fn i_c13b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B20;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13B20: lsr.w #$8, d2
pub fn i_c13b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B22;
    cpu.ir = 0xE04A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13B22: move.w (a0)+, d0
pub fn i_c13b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B24;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13B24: cmp.b d0, d2
pub fn i_c13b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B26;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C13B26: bne.w $c13b38
pub fn i_c13b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13B38; return 10; }
    8
}
/// $C13B2A: move.w (a0)+, d0
pub fn i_c13b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B2C;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13B2C: cmp.b d0, d3
pub fn i_c13b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B2E;
    cpu.ir = 0xB600;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C13B2E: bne.w $c13b38
pub fn i_c13b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13B38; return 10; }
    8
}
/// $C13B32: dbra d7, $c13b16
pub fn i_c13b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B36;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13B36; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13B36; return 14; }
    cpu.pc = 0xC13B16;
    10
}
/// $C13B36: rts
pub fn i_c13b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13B38: moveq #$0, d6
pub fn i_c13b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B3A;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13B3A: rts
pub fn i_c13b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13B3C: move.w #$5555, $2(a0)
pub fn i_c13b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B42;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13B42: cmpi.w #$5555, (a0)
pub fn i_c13b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B46;
    cpu.ir = 0xC50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x5555, Size::Word, false);
    12
}
/// $C13B46: beq.w $c13c20
pub fn i_c13b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13C20; return 10; }
    8
}
/// $C13B4A: move.w #$5555, (a0)
pub fn i_c13b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B4E;
    cpu.ir = 0x30BC;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13B4E: move.w (a0), d0
pub fn i_c13b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B50;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13B50: cmpi.w #$5555, d0
pub fn i_c13b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B54;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5555, Size::Word, false);
    8
}
/// $C13B54: beq.b $c13b58
pub fn i_c13b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B56;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13B58; return 10; }
    8
}
/// $C13B56: rts
pub fn i_c13b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13B58: move.w #$1, $a14(a5)
pub fn i_c13b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B5E;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13B5E: move.l #$800, d5
pub fn i_c13b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B64;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x800;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13B64: move.b d0, $300001.l
pub fn i_c13b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B6A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13B6A: add.l d5, d6
pub fn i_c13b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B6C;
    cpu.ir = 0xDC85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $C13B6C: cmpi.w #$4000, d6
pub fn i_c13b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B70;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x4000, Size::Word, false);
    8
}
/// $C13B70: beq.b $c13b90
pub fn i_c13b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B72;
    cpu.ir = 0x671E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13B90; return 10; }
    8
}
/// $C13B72: adda.l d5, a0
pub fn i_c13b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B74;
    cpu.ir = 0xD1C5;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C13B74: move.w #$aaaa, (a0)
pub fn i_c13b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B78;
    cpu.ir = 0x30BC;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13B78: move.w #$0, (a6)
pub fn i_c13b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B7C;
    cpu.ir = 0x3CBC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C13B7C: move.w (a0), d0
pub fn i_c13b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B7E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13B7E: cmpi.w #$aaaa, d0
pub fn i_c13b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B82;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xAAAA, Size::Word, false);
    8
}
/// $C13B82: bne.b $c13b90
pub fn i_c13b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B84;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13B90; return 10; }
    8
}
/// $C13B84: move.w #$5555, (a0)
pub fn i_c13b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B88;
    cpu.ir = 0x30BC;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13B88: move.w (a0), d0
pub fn i_c13b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B8A;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13B8A: cmpi.w #$5555, d0
pub fn i_c13b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B8E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5555, Size::Word, false);
    8
}
/// $C13B8E: beq.b $c13b64
pub fn i_c13b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B90;
    cpu.ir = 0x67D4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13B64; return 10; }
    8
}
/// $C13B90: tst.w d6
pub fn i_c13b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B92;
    cpu.ir = 0x4A46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C13B92: beq.w $c13c1c
pub fn i_c13b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13C1C; return 10; }
    8
}
/// $C13B96: move.w d6, d5
pub fn i_c13b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B98;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13B98: lsr.w #$1, d5
pub fn i_c13b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B9A;
    cpu.ir = 0xE24D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13B9A: subq.w #$1, d5
pub fn i_c13b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B9C;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13B9C: movea.l a6, a0
pub fn i_c13b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13B9E;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13B9E: move.w d5, d7
pub fn i_c13b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BA0;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BA0: move.w #$5555, d0
pub fn i_c13ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BA4: move.b d0, $300001.l
pub fn i_c13ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BAA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13BAA: move.w d0, (a0)+
pub fn i_c13baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BAC;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13BAC: dbra d7, $c13ba4
pub fn i_c13bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BB0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13BB0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13BB0; return 14; }
    cpu.pc = 0xC13BA4;
    10
}
/// $C13BB0: movea.l a6, a0
pub fn i_c13bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BB2;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13BB2: move.w d5, d7
pub fn i_c13bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BB4;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BB4: move.b d0, $300001.l
pub fn i_c13bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BBA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13BBA: move.w (a0)+, d1
pub fn i_c13bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BBC;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13BBC: cmp.w d1, d0
pub fn i_c13bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BBE;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C13BBE: bne.w $c13c1c
pub fn i_c13bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BC2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13C1C; return 10; }
    8
}
/// $C13BC2: dbra d7, $c13bb4
pub fn i_c13bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BC6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13BC6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13BC6; return 14; }
    cpu.pc = 0xC13BB4;
    10
}
/// $C13BC6: move.w d5, d7
pub fn i_c13bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BC8;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BC8: move.w #$aaaa, d0
pub fn i_c13bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BCC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BCC: movea.l a6, a0
pub fn i_c13bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BCE;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13BCE: move.b d0, $300001.l
pub fn i_c13bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BD4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13BD4: move.w d0, (a0)+
pub fn i_c13bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BD6;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13BD6: dbra d7, $c13bce
pub fn i_c13bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BDA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13BDA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13BDA; return 14; }
    cpu.pc = 0xC13BCE;
    10
}
/// $C13BDA: movea.l a6, a0
pub fn i_c13bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BDC;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13BDC: move.w d5, d7
pub fn i_c13bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BDE;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BDE: move.b d0, $300001.l
pub fn i_c13bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BE4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13BE4: move.w (a0)+, d1
pub fn i_c13be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BE6;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13BE6: cmp.w d1, d0
pub fn i_c13be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BE8;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C13BE8: bne.w $c13c1c
pub fn i_c13be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13C1C; return 10; }
    8
}
/// $C13BEC: dbra d7, $c13bde
pub fn i_c13bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BF0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13BF0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13BF0; return 14; }
    cpu.pc = 0xC13BDE;
    10
}
/// $C13BF0: movea.l a6, a0
pub fn i_c13bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BF2;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13BF2: move.w d5, d7
pub fn i_c13bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BF4;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BF4: move.b d0, $300001.l
pub fn i_c13bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BFA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13BFA: move.w a0, d0
pub fn i_c13bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BFC;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13BFC: move.w d0, (a0)+
pub fn i_c13bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13BFE;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13BFE: dbra d7, $c13bf4
pub fn i_c13bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C02;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13C02; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13C02; return 14; }
    cpu.pc = 0xC13BF4;
    10
}
/// $C13C02: movea.l a6, a0
pub fn i_c13c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C04;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13C04: move.w d5, d7
pub fn i_c13c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C06;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13C06: move.b d0, $300001.l
pub fn i_c13c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C0C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13C0C: move.w a0, d2
pub fn i_c13c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C0E;
    cpu.ir = 0x3408;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13C0E: move.w (a0)+, d0
pub fn i_c13c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C10;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C13C10: cmp.w d0, d2
pub fn i_c13c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C12;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C13C12: bne.w $c13c1c
pub fn i_c13c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13C1C; return 10; }
    8
}
/// $C13C16: dbra d7, $c13c06
pub fn i_c13c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C1A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13C1A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13C1A; return 14; }
    cpu.pc = 0xC13C06;
    10
}
/// $C13C1A: rts
pub fn i_c13c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13C1C: moveq #$0, d6
pub fn i_c13c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C1E;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13C1E: rts
pub fn i_c13c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13C20: move.w #$5555, (a0)
pub fn i_c13c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C24;
    cpu.ir = 0x30BC;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13C24: move.w (a0), d0
pub fn i_c13c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C26;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13C26: cmpi.w #$5555, d0
pub fn i_c13c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C2A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5555, Size::Word, false);
    8
}
/// $C13C2A: beq.b $c13c2e
pub fn i_c13c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C2C;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13C2E; return 10; }
    8
}
/// $C13C2C: rts
pub fn i_c13c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13C2E: move.w #$80, $a14(a5)
pub fn i_c13c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C34;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C13C34: move.l #$1000, d5
pub fn i_c13c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C3A;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x1000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13C3A: move.b d0, $300001.l
pub fn i_c13c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C40;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13C40: add.l d5, d6
pub fn i_c13c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C42;
    cpu.ir = 0xDC85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $C13C42: cmpi.w #$8000, d6
pub fn i_c13c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C46;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8000, Size::Word, false);
    8
}
/// $C13C46: beq.b $c13c66
pub fn i_c13c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C48;
    cpu.ir = 0x671E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13C66; return 10; }
    8
}
/// $C13C48: adda.l d5, a0
pub fn i_c13c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C4A;
    cpu.ir = 0xD1C5;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C13C4A: move.w #$aaaa, (a0)
pub fn i_c13c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C4E;
    cpu.ir = 0x30BC;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13C4E: move.w #$0, (a6)
pub fn i_c13c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C52;
    cpu.ir = 0x3CBC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C13C52: move.w (a0), d0
pub fn i_c13c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C54;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13C54: cmpi.w #$aaaa, d0
pub fn i_c13c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C58;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xAAAA, Size::Word, false);
    8
}
/// $C13C58: bne.b $c13c66
pub fn i_c13c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C5A;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13C66; return 10; }
    8
}
/// $C13C5A: move.w #$5555, (a0)
pub fn i_c13c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C5E;
    cpu.ir = 0x30BC;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13C5E: move.w (a0), d0
pub fn i_c13c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C60;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13C60: cmpi.w #$5555, d0
pub fn i_c13c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C64;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5555, Size::Word, false);
    8
}
/// $C13C64: beq.b $c13c3a
pub fn i_c13c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C66;
    cpu.ir = 0x67D4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13C3A; return 10; }
    8
}
/// $C13C66: tst.w d6
pub fn i_c13c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C68;
    cpu.ir = 0x4A46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C13C68: beq.w $c13cfe
pub fn i_c13c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC13CFE; return 10; }
    8
}
/// $C13C6C: move.w d6, d5
pub fn i_c13c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C6E;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13C6E: lsr.w #$1, d5
pub fn i_c13c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C70;
    cpu.ir = 0xE24D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13C70: subq.w #$1, d5
pub fn i_c13c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C72;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C13C72: movea.l a6, a0
pub fn i_c13c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C74;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13C74: move.w d5, d7
pub fn i_c13c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C76;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13C76: move.w #$5555, d0
pub fn i_c13c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C7A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13C7A: move.b d0, $300001.l
pub fn i_c13c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C80;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13C80: move.w d0, (a0)
pub fn i_c13c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C82;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13C82: addq.l #$4, a0
pub fn i_c13c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C84;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13C84: dbra d7, $c13c7a
pub fn i_c13c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C88;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13C88; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13C88; return 14; }
    cpu.pc = 0xC13C7A;
    10
}
/// $C13C88: movea.l a6, a0
pub fn i_c13c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C8A;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13C8A: move.w d5, d7
pub fn i_c13c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C8C;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13C8C: move.b d0, $300001.l
pub fn i_c13c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C92;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13C92: move.w (a0), d1
pub fn i_c13c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C94;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13C94: addq.l #$4, a0
pub fn i_c13c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C96;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13C96: cmp.w d1, d0
pub fn i_c13c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C98;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C13C98: bne.w $c13cfe
pub fn i_c13c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13C9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13CFE; return 10; }
    8
}
/// $C13C9C: dbra d7, $c13c8c
pub fn i_c13c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CA0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13CA0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13CA0; return 14; }
    cpu.pc = 0xC13C8C;
    10
}
/// $C13CA0: move.w d5, d7
pub fn i_c13ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CA2;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13CA2: move.w #$aaaa, d0
pub fn i_c13ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CA6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13CA6: movea.l a6, a0
pub fn i_c13ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CA8;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13CA8: move.b d0, $300001.l
pub fn i_c13ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CAE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13CAE: move.w d0, (a0)
pub fn i_c13cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CB0;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13CB0: addq.l #$4, a0
pub fn i_c13cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CB2;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13CB2: dbra d7, $c13ca8
pub fn i_c13cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CB6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13CB6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13CB6; return 14; }
    cpu.pc = 0xC13CA8;
    10
}
/// $C13CB6: movea.l a6, a0
pub fn i_c13cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CB8;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13CB8: move.w d5, d7
pub fn i_c13cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CBA;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13CBA: move.b d0, $300001.l
pub fn i_c13cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CC0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13CC0: move.w (a0), d1
pub fn i_c13cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CC2;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13CC2: addq.l #$4, a0
pub fn i_c13cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CC4;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13CC4: cmp.w d1, d0
pub fn i_c13cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CC6;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C13CC6: bne.w $c13cfe
pub fn i_c13cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13CFE; return 10; }
    8
}
/// $C13CCA: dbra d7, $c13cba
pub fn i_c13cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CCE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13CCE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13CCE; return 14; }
    cpu.pc = 0xC13CBA;
    10
}
/// $C13CCE: movea.l a6, a0
pub fn i_c13cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CD0;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13CD0: move.w d5, d7
pub fn i_c13cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CD2;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13CD2: move.b d0, $300001.l
pub fn i_c13cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CD8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13CD8: move.w a0, d0
pub fn i_c13cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CDA;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13CDA: move.w d0, (a0)
pub fn i_c13cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CDC;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C13CDC: addq.l #$4, a0
pub fn i_c13cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CDE;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13CDE: dbra d7, $c13cd2
pub fn i_c13cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CE2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13CE2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13CE2; return 14; }
    cpu.pc = 0xC13CD2;
    10
}
/// $C13CE2: movea.l a6, a0
pub fn i_c13ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CE4;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C13CE4: move.w d5, d7
pub fn i_c13ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CE6;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13CE6: move.b d0, $300001.l
pub fn i_c13ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CEC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C13CEC: move.w a0, d2
pub fn i_c13cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CEE;
    cpu.ir = 0x3408;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C13CEE: move.w (a0), d0
pub fn i_c13cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CF0;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C13CF0: addq.l #$4, a0
pub fn i_c13cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CF2;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C13CF2: cmp.w d0, d2
pub fn i_c13cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CF4;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C13CF4: bne.w $c13cfe
pub fn i_c13cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC13CFE; return 10; }
    8
}
/// $C13CF8: dbra d7, $c13ce6
pub fn i_c13cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CFC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13CFC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13CFC; return 14; }
    cpu.pc = 0xC13CE6;
    10
}
/// $C13CFC: rts
pub fn i_c13cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13CFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13CFE: moveq #$0, d6
pub fn i_c13cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D00;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13D00: rts
pub fn i_c13d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
/// $C13D2A: ori.b #$0, d0
pub fn i_c13d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D2E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D2E: dc.w $ffff
pub fn i_c13d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D30;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C13D30: ori.b #$0, d0
pub fn i_c13d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D34;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D34: dc.w $ffff
pub fn i_c13d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D36;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C13D36: ori.b #$0, d0
pub fn i_c13d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D3A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D3A: ori.b #$80, d0
pub fn i_c13d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D3E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D3E: ori.b #$0, (a7)+
pub fn i_c13d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D42;
    cpu.ir = 0x1F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C13D42: btst.l d0, d0
pub fn i_c13d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D44;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13D44: ori.l #$100, d0
pub fn i_c13d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D4A;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0x100) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C13D4A: ori.b #$80, d0
pub fn i_c13d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D4E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D4E: ?
pub fn i_c13d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D52;
    cpu.ir = 0x3F;
    let v1: u32 = 0x0;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C13D52: andi.b #$80, d0
pub fn i_c13d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D56;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D56: ori.b #$0, d0
pub fn i_c13d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D5A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D5A: ori.b #$80, d0
pub fn i_c13d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D5E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D5E: ori.w #$0, (a7)+
pub fn i_c13d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D62;
    cpu.ir = 0x5F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C13D62: btst.l d1, d0
pub fn i_c13d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D64;
    cpu.ir = 0x300;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13D64: btst.l d0, d0
pub fn i_c13d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D66;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13D66: ori.b #$0, d0
pub fn i_c13d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D6A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D6A: ori.b #$80, d0
pub fn i_c13d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D6E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D6E: ?
pub fn i_c13d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D72;
    cpu.ir = 0x7F;
    let v1: u32 = 0x0;
    let r2 = (v1 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C13D72: subi.b #$0, d0
pub fn i_c13d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D76;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x0, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D76: ori.b #$0, d0
pub fn i_c13d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D7A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D7A: ori.b #$80, d0
pub fn i_c13d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D7E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D7E: ori.l #$500, (a7)+
pub fn i_c13d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D84;
    cpu.ir = 0x9F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | 0x500) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    16
}
/// $C13D84: bclr.b d0, d0
pub fn i_c13d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D86;
    cpu.ir = 0x180;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C13D86: ori.b #$0, d0
pub fn i_c13d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D8A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D8A: ori.b #$80, d0
pub fn i_c13d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D8E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D8E: ?
pub fn i_c13d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D94;
    cpu.ir = 0xBF;
    let v1: u32 = 0x0;
    let r2 = (v1 | 0x600) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    8
}
/// $C13D94: bclr.b d0, d0
pub fn i_c13d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D96;
    cpu.ir = 0x180;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C13D96: ori.b #$0, d0
pub fn i_c13d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D9A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D9A: ori.b #$80, d0
pub fn i_c13d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13D9E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13D9E: ?
pub fn i_c13d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DA0;
    cpu.ir = 0xDF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C13DA0: ori.b #$0, d0
pub fn i_c13da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DA4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13DA4: andi.b #$0, d0
pub fn i_c13da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DA8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13DA8: subi.b #$0, d0
pub fn i_c13da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DAC;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x0, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13DAC: ori.l #$ff0000, d0
pub fn i_c13dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DB2;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0xFF0000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C13DB2: ?
pub fn i_c13db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DB6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13DB6: ori.b #$0, d0
pub fn i_c13db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DBA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
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
/// $C13DC8: ori.b #$0, d0
pub fn i_c13dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DCC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13DCC: dc.w $ffff
pub fn i_c13dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DCE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C13DCE: ori.b #$0, d0
pub fn i_c13dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DD2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13DD2: dc.w $ffff
pub fn i_c13dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DD4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C13DD4: ori.b #$0, d0
pub fn i_c13dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DD8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13DD8: ori.b #$80, d0
pub fn i_c13dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DDC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13DDC: ori.b #$0, (a7)+
pub fn i_c13ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DE0;
    cpu.ir = 0x1F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C13DE0: ori.l #$400000, d0
pub fn i_c13de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DE6;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0x400000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C13DE6: ori.l #$80, d0
pub fn i_c13de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DEC;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0x80) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C13DEC: ?
pub fn i_c13dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DF0;
    cpu.ir = 0x3F;
    let v1: u32 = 0x0;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C13DF0: btst.l d0, d0
pub fn i_c13df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DF2;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13DF2: ori.w #$0, d0
pub fn i_c13df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DF6;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C13DF6: ori.l #$80, d0
pub fn i_c13df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13DFC;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0x80) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C13DFC: ori.w #$0, (a7)+
pub fn i_c13dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E00;
    cpu.ir = 0x5F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C13E00: bclr.b d0, d0
pub fn i_c13e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E02;
    cpu.ir = 0x180;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C13E02: ori.l #$100, d0
pub fn i_c13e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E08;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0x100) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C13E08: ori.b #$80, d0
pub fn i_c13e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E0C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E0C: ?
pub fn i_c13e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E10;
    cpu.ir = 0x7F;
    let v1: u32 = 0x0;
    let r2 = (v1 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C13E10: andi.b #$80, d0
pub fn i_c13e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E14;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E14: ori.b #$0, d0
pub fn i_c13e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E18;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E18: ori.b #$80, d0
pub fn i_c13e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E1C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E1C: ori.l #$280, (a7)+
pub fn i_c13e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E22;
    cpu.ir = 0x9F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | 0x280) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    16
}
/// $C13E22: ?
pub fn i_c13e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E24;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C13E24: ori.b #$80, d0
pub fn i_c13e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E28;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E28: ori.b #$80, d0
pub fn i_c13e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E2C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E2C: ?
pub fn i_c13e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E32;
    cpu.ir = 0xBF;
    let v1: u32 = 0x0;
    let r2 = (v1 | 0x300) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    8
}
/// $C13E32: ?
pub fn i_c13e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E34;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C13E34: ori.b #$80, d0
pub fn i_c13e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E38;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E38: ori.b #$80, d0
pub fn i_c13e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E3C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E3C: ?
pub fn i_c13e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E3E;
    cpu.ir = 0xDF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C13E3E: ori.b #$80, d0
pub fn i_c13e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E42;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E42: btst.l d0, d0
pub fn i_c13e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E44;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13E44: ori.b #$0, d0
pub fn i_c13e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E48;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E48: ori.b #$80, d0
pub fn i_c13e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E4C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E4C: ?
pub fn i_c13e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E4E;
    cpu.ir = 0xFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C13E4E: ori.b #$0, d0
pub fn i_c13e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E52;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13E52: btst.l d0, d0
pub fn i_c13e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E54;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C13E54: ori.b #$0, d0
pub fn i_c13e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E58;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
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
/// $C13E74: move.w #$ff00, $3c0006.l
pub fn i_c13e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E7C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0006, v1 as u16);
    16
}
/// $C13E7C: lea.l $10f300.l, a5
pub fn i_c13e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E82;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C13E82: move.l #$eee0000, $400002.l
pub fn i_c13e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E8C;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C13E8C: move.w #$2000, sr
pub fn i_c13e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E90;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C13E90: clr.b $ac2(a5)
pub fn i_c13e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E94;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C13E94: jsr $c17290(pc)
pub fn i_c13e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13E98);
    cpu.pc = 0xC17290;
    16
}
/// $C13E98: jsr $c12a0e(pc)
pub fn i_c13e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13E9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13E9C);
    cpu.pc = 0xC12A0E;
    16
}
/// $C13E9C: addq.b #$1, $ac2(a5)
pub fn i_c13e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EA0;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C13EA0: movea.l $abe(a5), a0
pub fn i_c13ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EA4;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C13EA4: move.l #$c13f1e, (a0)+
pub fn i_c13ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EAA;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC13F1E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C13EAA: move.l a0, $abe(a5)
pub fn i_c13eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EAE;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C13EAE: subq.b #$1, $ac2(a5)
pub fn i_c13eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EB2;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C13EB2: jsr $c1727e(pc)
pub fn i_c13eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13EB6);
    cpu.pc = 0xC1727E;
    16
}
/// $C13EB6: addq.b #$1, $ac2(a5)
pub fn i_c13eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EBA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C13EBA: movea.l $abe(a5), a0
pub fn i_c13eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EBE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C13EBE: clr.l (a0)+
pub fn i_c13ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EC0;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C13EC0: move.l #$101000a, (a0)+
pub fn i_c13ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EC6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x101000A;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C13EC6: move.w #$102, (a0)+
pub fn i_c13ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ECA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x102;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13ECA: move.w #$3, (a0)+
pub fn i_c13eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ECE;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13ECE: move.w #$722a, (a0)+
pub fn i_c13ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ED2;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x722A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13ED2: move.b $aac(a5), d1
pub fn i_c13ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ED6;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xAAC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C13ED6: lsl.w #$8, d1
pub fn i_c13ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13ED8;
    cpu.ir = 0xE149;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C13ED8: move.b $a96(a5), d1
pub fn i_c13ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EDC;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xA96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C13EDC: jsr $c13f04(pc)
pub fn i_c13edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13EE0);
    cpu.pc = 0xC13F04;
    16
}
/// $C13EE0: move.w #$3, (a0)+
pub fn i_c13ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EE4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13EE4: move.w #$72ca, (a0)+
pub fn i_c13ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EE8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x72CA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13EE8: move.b $aac(a5), d1
pub fn i_c13ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EEC;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xAAC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C13EEC: lsl.w #$6, d1
pub fn i_c13eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EEE;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $C13EEE: move.b $a9c(a5), d1
pub fn i_c13eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EF2;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xA9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C13EF2: jsr $c13f04(pc)
pub fn i_c13ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC13EF6);
    cpu.pc = 0xC13F04;
    16
}
/// $C13EF6: clr.w (a0)+
pub fn i_c13ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EF8;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C13EF8: move.l a0, $abe(a5)
pub fn i_c13ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13EFC;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C13EFC: subq.b #$1, $ac2(a5)
pub fn i_c13efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F00;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C13F00: jmp $c13eb2(pc)
pub fn i_c13f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F04;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC13EB2;
    8
}
/// $C13F04: move.w #$7, (a0)+
pub fn i_c13f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F08;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C13F08: moveq #$9, d7
pub fn i_c13f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F0A;
    cpu.ir = 0x7E09;
    cpu.d[7] = 0x9;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13F0A: move.b #$30, d0
pub fn i_c13f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F0E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13F0E: lsr.w #$1, d1
pub fn i_c13f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F10;
    cpu.ir = 0xE249;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C13F10: bcc.b $c13f16
pub fn i_c13f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F12;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC13F16; return 10; }
    8
}
/// $C13F12: move.b #$31, d0
pub fn i_c13f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F16;
    cpu.ir = 0x103C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C13F16: move.b d0, (a0)+
pub fn i_c13f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F18;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C13F18: dbra d7, $c13f0a
pub fn i_c13f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F1C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC13F1C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC13F1C; return 14; }
    cpu.pc = 0xC13F0A;
    10
}
/// $C13F1C: rts
pub fn i_c13f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C13F1E: ori.b #$ff, d1
pub fn i_c13f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F22;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13F22: move.l d2, d0
pub fn i_c13f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F24;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C13F24: ori.b #$46, d3
pub fn i_c13f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F28;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x46) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13F28: ori.b #$4f, d7
pub fn i_c13f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F2C;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C13F2C: link.w a4, #$524f
pub fn i_c13f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F30;
    cpu.ir = 0x4E54;
    cpu.push32(bus, cpu.a[4]); cpu.a[4] = cpu.a[7]; cpu.a[7] = cpu.a[7].wrapping_add(0x524F);
    16
}
/// $C13F30: ?
pub fn i_c13f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F32;
    cpu.ir = 0x4C4C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C13F32: ?
pub fn i_c13f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F34;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C13F34: movea.l (a4), a0
pub fn i_c13f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F36;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C13F36: ?
pub fn i_c13f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC13F38;
    cpu.ir = 0x4553;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14092: lea.l $c14cdc(pc), a2
pub fn i_c14092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14096;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14CDC;
    16
}
/// $C14096: jsr $c14958(pc)
pub fn i_c14096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1409A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1409A);
    cpu.pc = 0xC14958;
    16
}
/// $C1409A: jsr $c14918(pc)
pub fn i_c1409a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1409E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1409E);
    cpu.pc = 0xC14918;
    16
}
/// $C1409E: btst.b #$5, d0
pub fn i_c1409e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140A2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C140A2: bne.w $c1410a
pub fn i_c140a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1410A; return 10; }
    8
}
/// $C140A6: btst.b #$4, d0
pub fn i_c140a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140AA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C140AA: beq.b $c1409a
pub fn i_c140aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140AC;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1409A; return 10; }
    8
}
/// $C140AC: jsr $c1494c(pc)
pub fn i_c140ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC140B0);
    cpu.pc = 0xC1494C;
    16
}
/// $C140B0: bne.b $c1409a
pub fn i_c140b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140B2;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1409A; return 10; }
    8
}
/// $C140B2: move.l #$10fe9c, $ac8(a5)
pub fn i_c140b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140BA;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FE9C;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C140BA: clr.w $acc(a5)
pub fn i_c140ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140BE;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xACC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C140BE: move.b #$5, $ac4(a5)
pub fn i_c140be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140C4;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C140C4: jsr $c1231e(pc)
pub fn i_c140c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC140C8);
    cpu.pc = 0xC1231E;
    16
}
/// $C140C8: tst.b $ac6(a5)
pub fn i_c140c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140CC;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C140CC: bne.b $c140ec
pub fn i_c140cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140CE;
    cpu.ir = 0x661E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC140EC; return 10; }
    8
}
/// $C140CE: jsr $c12a0e(pc)
pub fn i_c140ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC140D2);
    cpu.pc = 0xC12A0E;
    16
}
/// $C140D2: lea.l $c14d2c(pc), a2
pub fn i_c140d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140D6;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14D2C;
    16
}
/// $C140D6: jsr $c14958(pc)
pub fn i_c140d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC140DA);
    cpu.pc = 0xC14958;
    16
}
/// $C140DA: jsr $c14918(pc)
pub fn i_c140da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC140DE);
    cpu.pc = 0xC14918;
    16
}
/// $C140DE: btst.b #$5, d0
pub fn i_c140de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140E2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C140E2: bne.w $c1410a
pub fn i_c140e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1410A; return 10; }
    8
}
/// $C140E6: btst.b #$4, d0
pub fn i_c140e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140EA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C140EA: beq.b $c140da
pub fn i_c140ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140EC;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC140DA; return 10; }
    8
}
/// $C140EC: move.b #$0, $ac4(a5)
pub fn i_c140ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140F2;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C140F2: jsr $c1231e(pc)
pub fn i_c140f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC140F6);
    cpu.pc = 0xC1231E;
    16
}
/// $C140F6: moveq #$0, d0
pub fn i_c140f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140F8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C140F8: move.b $ac6(a5), d0
pub fn i_c140f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC140FC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C140FC: cmpi.b #$85, d0
pub fn i_c140fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14100;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x85, Size::Byte, false);
    8
}
/// $C14100: bne.b $c1410a
pub fn i_c14100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14102;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1410A; return 10; }
    8
}
/// $C14102: lea.l $c15144(pc), a2
pub fn i_c14102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14106;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC15144;
    16
}
/// $C14106: jsr $c14810(pc)
pub fn i_c14106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1410A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1410A);
    cpu.pc = 0xC14810;
    16
}
/// $C1410A: rts
pub fn i_c1410a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1410C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1410C: jsr $c12a0e(pc)
pub fn i_c1410c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14110;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14110);
    cpu.pc = 0xC12A0E;
    16
}
/// $C14110: jsr $c1494c(pc)
pub fn i_c14110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14114;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14114);
    cpu.pc = 0xC1494C;
    16
}
/// $C14114: beq.w $c14134
pub fn i_c14114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14118;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14134; return 10; }
    8
}
/// $C14118: lea.l $c150b4(pc), a2
pub fn i_c14118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1411C;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC150B4;
    16
}
/// $C1411C: jsr $c14958(pc)
pub fn i_c1411c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14120;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14120);
    cpu.pc = 0xC14958;
    16
}
/// $C14120: jsr $c14918(pc)
pub fn i_c14120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14124;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14124);
    cpu.pc = 0xC14918;
    16
}
/// $C14124: btst.b #$5, d0
pub fn i_c14124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14128;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C14128: bne.b $c1414a
pub fn i_c14128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1412A;
    cpu.ir = 0x6620;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1414A; return 10; }
    8
}
/// $C1412A: jsr $c1494c(pc)
pub fn i_c1412a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1412E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1412E);
    cpu.pc = 0xC1494C;
    16
}
/// $C1412E: bne.b $c14120
pub fn i_c1412e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14130;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC14120; return 10; }
    8
}
/// $C14130: jsr $c12a0e(pc)
pub fn i_c14130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14134;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14134);
    cpu.pc = 0xC12A0E;
    16
}
/// $C14134: jsr $c14838(pc)
pub fn i_c14134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14138;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14138);
    cpu.pc = 0xC14838;
    16
}
/// $C14138: lea.l $c14db8(pc), a2
pub fn i_c14138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1413C;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14DB8;
    16
}
/// $C1413C: jsr $c1488c(pc)
pub fn i_c1413c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14140;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14140);
    cpu.pc = 0xC1488C;
    16
}
/// $C14140: jsr $c14918(pc)
pub fn i_c14140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14144;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14144);
    cpu.pc = 0xC14918;
    16
}
/// $C14144: btst.b #$5, d0
pub fn i_c14144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14148;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C14148: beq.b $c1414c
pub fn i_c14148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1414A;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1414C; return 10; }
    8
}
/// $C1414A: rts
pub fn i_c1414a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1414C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1414C: andi.b #$3, d0
pub fn i_c1414c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14150;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14150: beq.b $c14140
pub fn i_c14150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14152;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14140; return 10; }
    8
}
/// $C14152: move.w $bb2(a5), d1
pub fn i_c14152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14156;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C14156: andi.b #$2, d0
pub fn i_c14156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1415A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1415A: bne.b $c1417a
pub fn i_c1415a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1415C;
    cpu.ir = 0x661E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1417A; return 10; }
    8
}
/// $C1415C: cmpi.w #$7, d1
pub fn i_c1415c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14160;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7, Size::Word, false);
    8
}
/// $C14160: bcs.b $c14140
pub fn i_c14160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14162;
    cpu.ir = 0x65DE;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC14140; return 10; }
    8
}
/// $C14162: subq.w #$7, d1
pub fn i_c14162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14164;
    cpu.ir = 0x5F41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 7, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C14164: andi.l #$ffff, d1
pub fn i_c14164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1416A;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $C1416A: move.w d1, d2
pub fn i_c1416a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1416C;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1416C: divu.w #$6, d1
pub fn i_c1416c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14170;
    cpu.ir = 0x82FC;
    let v1: u32 = 0x6;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[1] / dv; let rm = cpu.d[1] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[1] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C14170: swap d1
pub fn i_c14170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14172;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14172: sub.w d1, d2
pub fn i_c14172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14174;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C14174: move.w d2, $bb2(a5)
pub fn i_c14174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14178;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14178: bra.b $c14138
pub fn i_c14178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1417A;
    cpu.ir = 0x60BE;
    cpu.pc = 0xC14138;
    10
}
/// $C1417A: cmp.w $bb0(a5), d1
pub fn i_c1417a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1417E;
    cpu.ir = 0xB26D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C1417E: bhi.b $c14140
pub fn i_c1417e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14180;
    cpu.ir = 0x62C0;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC14140; return 10; }
    8
}
/// $C14180: bra.b $c14138
pub fn i_c14180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14182;
    cpu.ir = 0x60B6;
    cpu.pc = 0xC14138;
    10
}
/// $C14182: jsr $c1494c(pc)
pub fn i_c14182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14186;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14186);
    cpu.pc = 0xC1494C;
    16
}
/// $C14186: beq.w $c141a6
pub fn i_c14186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1418A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC141A6; return 10; }
    8
}
/// $C1418A: lea.l $c150b4(pc), a2
pub fn i_c1418a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1418E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC150B4;
    16
}
/// $C1418E: jsr $c14958(pc)
pub fn i_c1418e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14192;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14192);
    cpu.pc = 0xC14958;
    16
}
/// $C14192: jsr $c14918(pc)
pub fn i_c14192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14196;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14196);
    cpu.pc = 0xC14918;
    16
}
/// $C14196: btst.b #$5, d0
pub fn i_c14196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1419A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1419A: bne.b $c141c0
pub fn i_c1419a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1419C;
    cpu.ir = 0x6624;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC141C0; return 10; }
    8
}
/// $C1419C: jsr $c1494c(pc)
pub fn i_c1419c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC141A0);
    cpu.pc = 0xC1494C;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
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
    (0xC12EDC, i_c12edc),
    (0xC12EE2, i_c12ee2),
    (0xC12EE6, i_c12ee6),
    (0xC12EE8, i_c12ee8),
    (0xC12EEE, i_c12eee),
    (0xC12EF0, i_c12ef0),
    (0xC12EF4, i_c12ef4),
    (0xC12EF8, i_c12ef8),
    (0xC12EFA, i_c12efa),
    (0xC12EFC, i_c12efc),
    (0xC12F00, i_c12f00),
    (0xC12F04, i_c12f04),
    (0xC12F06, i_c12f06),
    (0xC12F0C, i_c12f0c),
    (0xC12F10, i_c12f10),
    (0xC12F14, i_c12f14),
    (0xC12F18, i_c12f18),
    (0xC12F1C, i_c12f1c),
    (0xC12F20, i_c12f20),
    (0xC12F24, i_c12f24),
    (0xC12F28, i_c12f28),
    (0xC12F2C, i_c12f2c),
    (0xC12F2E, i_c12f2e),
    (0xC12F30, i_c12f30),
    (0xC12F36, i_c12f36),
    (0xC12F38, i_c12f38),
    (0xC12F3C, i_c12f3c),
    (0xC12F3E, i_c12f3e),
    (0xC12F40, i_c12f40),
    (0xC12F42, i_c12f42),
    (0xC12F48, i_c12f48),
    (0xC12F4A, i_c12f4a),
    (0xC12F4E, i_c12f4e),
    (0xC12F52, i_c12f52),
    (0xC12F54, i_c12f54),
    (0xC12F58, i_c12f58),
    (0xC12F5C, i_c12f5c),
    (0xC12F5E, i_c12f5e),
    (0xC12F60, i_c12f60),
    (0xC12F62, i_c12f62),
    (0xC12F64, i_c12f64),
    (0xC12F68, i_c12f68),
    (0xC12F6C, i_c12f6c),
    (0xC12F6E, i_c12f6e),
    (0xC12F70, i_c12f70),
    (0xC12F74, i_c12f74),
    (0xC12F76, i_c12f76),
    (0xC12F7C, i_c12f7c),
    (0xC12F7E, i_c12f7e),
    (0xC12F82, i_c12f82),
    (0xC12F84, i_c12f84),
    (0xC12F86, i_c12f86),
    (0xC12F8A, i_c12f8a),
    (0xC12F90, i_c12f90),
    (0xC12F92, i_c12f92),
    (0xC12F94, i_c12f94),
    (0xC12F96, i_c12f96),
    (0xC12F9A, i_c12f9a),
    (0xC12F9C, i_c12f9c),
    (0xC12F9E, i_c12f9e),
    (0xC12FA2, i_c12fa2),
    (0xC12FA6, i_c12fa6),
    (0xC12FAA, i_c12faa),
    (0xC12FAE, i_c12fae),
    (0xC12FB2, i_c12fb2),
    (0xC12FB4, i_c12fb4),
    (0xC12FB8, i_c12fb8),
    (0xC12FBC, i_c12fbc),
    (0xC12FC0, i_c12fc0),
    (0xC12FC6, i_c12fc6),
    (0xC12FCA, i_c12fca),
    (0xC12FCE, i_c12fce),
    (0xC12FD0, i_c12fd0),
    (0xC12FD4, i_c12fd4),
    (0xC12FD8, i_c12fd8),
    (0xC12FDA, i_c12fda),
    (0xC12FDE, i_c12fde),
    (0xC12FE2, i_c12fe2),
    (0xC12FE4, i_c12fe4),
    (0xC12FE8, i_c12fe8),
    (0xC12FEA, i_c12fea),
    (0xC12FEE, i_c12fee),
    (0xC12FF0, i_c12ff0),
    (0xC12FF4, i_c12ff4),
    (0xC12FF8, i_c12ff8),
    (0xC12FFC, i_c12ffc),
    (0xC13000, i_c13000),
    (0xC13002, i_c13002),
    (0xC13006, i_c13006),
    (0xC1300A, i_c1300a),
    (0xC1300E, i_c1300e),
    (0xC13012, i_c13012),
    (0xC13014, i_c13014),
    (0xC13018, i_c13018),
    (0xC1301C, i_c1301c),
    (0xC1301E, i_c1301e),
    (0xC13020, i_c13020),
    (0xC13022, i_c13022),
    (0xC13024, i_c13024),
    (0xC13026, i_c13026),
    (0xC1302A, i_c1302a),
    (0xC1302E, i_c1302e),
    (0xC13030, i_c13030),
    (0xC13032, i_c13032),
    (0xC13036, i_c13036),
    (0xC1303A, i_c1303a),
    (0xC1303E, i_c1303e),
    (0xC13040, i_c13040),
    (0xC13044, i_c13044),
    (0xC13048, i_c13048),
    (0xC1304C, i_c1304c),
    (0xC13050, i_c13050),
    (0xC13054, i_c13054),
    (0xC13056, i_c13056),
    (0xC13058, i_c13058),
    (0xC1305A, i_c1305a),
    (0xC1305E, i_c1305e),
    (0xC13060, i_c13060),
    (0xC13064, i_c13064),
    (0xC13066, i_c13066),
    (0xC1306A, i_c1306a),
    (0xC1306E, i_c1306e),
    (0xC13072, i_c13072),
    (0xC13076, i_c13076),
    (0xC13078, i_c13078),
    (0xC1307C, i_c1307c),
    (0xC1307E, i_c1307e),
    (0xC13082, i_c13082),
    (0xC13084, i_c13084),
    (0xC13088, i_c13088),
    (0xC1308E, i_c1308e),
    (0xC13090, i_c13090),
    (0xC13092, i_c13092),
    (0xC13094, i_c13094),
    (0xC13098, i_c13098),
    (0xC1309A, i_c1309a),
    (0xC1309C, i_c1309c),
    (0xC130A0, i_c130a0),
    (0xC130A2, i_c130a2),
    (0xC130A6, i_c130a6),
    (0xC130A8, i_c130a8),
    (0xC130AC, i_c130ac),
    (0xC130AE, i_c130ae),
    (0xC130B2, i_c130b2),
    (0xC130B4, i_c130b4),
    (0xC130B8, i_c130b8),
    (0xC130BC, i_c130bc),
    (0xC130C0, i_c130c0),
    (0xC130C4, i_c130c4),
    (0xC130C8, i_c130c8),
    (0xC130CC, i_c130cc),
    (0xC130CE, i_c130ce),
    (0xC130D2, i_c130d2),
    (0xC130D6, i_c130d6),
    (0xC130DA, i_c130da),
    (0xC130DC, i_c130dc),
    (0xC130E0, i_c130e0),
    (0xC130E4, i_c130e4),
    (0xC130E6, i_c130e6),
    (0xC130EA, i_c130ea),
    (0xC130EE, i_c130ee),
    (0xC130F0, i_c130f0),
    (0xC130F4, i_c130f4),
    (0xC130F6, i_c130f6),
    (0xC130F8, i_c130f8),
    (0xC130FC, i_c130fc),
    (0xC130FE, i_c130fe),
    (0xC13102, i_c13102),
    (0xC13106, i_c13106),
    (0xC1310A, i_c1310a),
    (0xC1310E, i_c1310e),
    (0xC13110, i_c13110),
    (0xC13114, i_c13114),
    (0xC13118, i_c13118),
    (0xC1311C, i_c1311c),
    (0xC1311E, i_c1311e),
    (0xC13122, i_c13122),
    (0xC13124, i_c13124),
    (0xC13128, i_c13128),
    (0xC1312C, i_c1312c),
    (0xC1312E, i_c1312e),
    (0xC13130, i_c13130),
    (0xC13132, i_c13132),
    (0xC13134, i_c13134),
    (0xC13136, i_c13136),
    (0xC13138, i_c13138),
    (0xC1313C, i_c1313c),
    (0xC13140, i_c13140),
    (0xC13142, i_c13142),
    (0xC13144, i_c13144),
    (0xC13148, i_c13148),
    (0xC1314C, i_c1314c),
    (0xC1314E, i_c1314e),
    (0xC13150, i_c13150),
    (0xC13154, i_c13154),
    (0xC13156, i_c13156),
    (0xC1315A, i_c1315a),
    (0xC1315E, i_c1315e),
    (0xC13162, i_c13162),
    (0xC13166, i_c13166),
    (0xC1316A, i_c1316a),
    (0xC1316C, i_c1316c),
    (0xC1316E, i_c1316e),
    (0xC13170, i_c13170),
    (0xC13174, i_c13174),
    (0xC13176, i_c13176),
    (0xC1317A, i_c1317a),
    (0xC1317C, i_c1317c),
    (0xC13180, i_c13180),
    (0xC13184, i_c13184),
    (0xC13188, i_c13188),
    (0xC1318C, i_c1318c),
    (0xC1318E, i_c1318e),
    (0xC13192, i_c13192),
    (0xC13194, i_c13194),
    (0xC13196, i_c13196),
    (0xC13198, i_c13198),
    (0xC1319C, i_c1319c),
    (0xC1319E, i_c1319e),
    (0xC131A0, i_c131a0),
    (0xC131A4, i_c131a4),
    (0xC131AA, i_c131aa),
    (0xC131AC, i_c131ac),
    (0xC131AE, i_c131ae),
    (0xC131B0, i_c131b0),
    (0xC131B4, i_c131b4),
    (0xC131B6, i_c131b6),
    (0xC131B8, i_c131b8),
    (0xC131BC, i_c131bc),
    (0xC131BE, i_c131be),
    (0xC131C2, i_c131c2),
    (0xC131C4, i_c131c4),
    (0xC131C8, i_c131c8),
    (0xC131CA, i_c131ca),
    (0xC131CE, i_c131ce),
    (0xC131D0, i_c131d0),
    (0xC131D4, i_c131d4),
    (0xC131D8, i_c131d8),
    (0xC131DC, i_c131dc),
    (0xC131E0, i_c131e0),
    (0xC131E4, i_c131e4),
    (0xC131E8, i_c131e8),
    (0xC131EA, i_c131ea),
    (0xC131EE, i_c131ee),
    (0xC131F2, i_c131f2),
    (0xC131F8, i_c131f8),
    (0xC131FC, i_c131fc),
    (0xC131FE, i_c131fe),
    (0xC13204, i_c13204),
    (0xC13206, i_c13206),
    (0xC1320A, i_c1320a),
    (0xC1320E, i_c1320e),
    (0xC13210, i_c13210),
    (0xC13212, i_c13212),
    (0xC13216, i_c13216),
    (0xC1321A, i_c1321a),
    (0xC1321C, i_c1321c),
    (0xC13220, i_c13220),
    (0xC13224, i_c13224),
    (0xC13228, i_c13228),
    (0xC1322A, i_c1322a),
    (0xC1322C, i_c1322c),
    (0xC13232, i_c13232),
    (0xC13234, i_c13234),
    (0xC13238, i_c13238),
    (0xC1323C, i_c1323c),
    (0xC13240, i_c13240),
    (0xC13242, i_c13242),
    (0xC13246, i_c13246),
    (0xC1324A, i_c1324a),
    (0xC1324E, i_c1324e),
    (0xC13252, i_c13252),
    (0xC13256, i_c13256),
    (0xC1325A, i_c1325a),
    (0xC1325E, i_c1325e),
    (0xC13262, i_c13262),
    (0xC13266, i_c13266),
    (0xC13268, i_c13268),
    (0xC1326A, i_c1326a),
    (0xC13270, i_c13270),
    (0xC13272, i_c13272),
    (0xC13276, i_c13276),
    (0xC1327A, i_c1327a),
    (0xC1327E, i_c1327e),
    (0xC13282, i_c13282),
    (0xC13286, i_c13286),
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
    (0xC133E0, i_c133e0),
    (0xC133E4, i_c133e4),
    (0xC133E8, i_c133e8),
    (0xC133EA, i_c133ea),
    (0xC133F0, i_c133f0),
    (0xC133F4, i_c133f4),
    (0xC133F6, i_c133f6),
    (0xC133FC, i_c133fc),
    (0xC133FE, i_c133fe),
    (0xC13402, i_c13402),
    (0xC13406, i_c13406),
    (0xC1340A, i_c1340a),
    (0xC1340E, i_c1340e),
    (0xC13412, i_c13412),
    (0xC13416, i_c13416),
    (0xC13418, i_c13418),
    (0xC1341A, i_c1341a),
    (0xC1341C, i_c1341c),
    (0xC13420, i_c13420),
    (0xC13422, i_c13422),
    (0xC13426, i_c13426),
    (0xC1342A, i_c1342a),
    (0xC1342E, i_c1342e),
    (0xC13432, i_c13432),
    (0xC13436, i_c13436),
    (0xC13438, i_c13438),
    (0xC1343A, i_c1343a),
    (0xC1343C, i_c1343c),
    (0xC1343E, i_c1343e),
    (0xC13442, i_c13442),
    (0xC13444, i_c13444),
    (0xC13448, i_c13448),
    (0xC1344C, i_c1344c),
    (0xC1344E, i_c1344e),
    (0xC13452, i_c13452),
    (0xC13456, i_c13456),
    (0xC1345A, i_c1345a),
    (0xC1345E, i_c1345e),
    (0xC13460, i_c13460),
    (0xC13462, i_c13462),
    (0xC13464, i_c13464),
    (0xC1346A, i_c1346a),
    (0xC1346C, i_c1346c),
    (0xC1346E, i_c1346e),
    (0xC13470, i_c13470),
    (0xC13474, i_c13474),
    (0xC13476, i_c13476),
    (0xC13478, i_c13478),
    (0xC1347A, i_c1347a),
    (0xC1347E, i_c1347e),
    (0xC13480, i_c13480),
    (0xC13484, i_c13484),
    (0xC13488, i_c13488),
    (0xC1348C, i_c1348c),
    (0xC1348E, i_c1348e),
    (0xC13494, i_c13494),
    (0xC13496, i_c13496),
    (0xC1349A, i_c1349a),
    (0xC1349C, i_c1349c),
    (0xC1349E, i_c1349e),
    (0xC134A2, i_c134a2),
    (0xC134A4, i_c134a4),
    (0xC134A6, i_c134a6),
    (0xC134AA, i_c134aa),
    (0xC134AC, i_c134ac),
    (0xC134B0, i_c134b0),
    (0xC134B2, i_c134b2),
    (0xC134B4, i_c134b4),
    (0xC134B8, i_c134b8),
    (0xC134BA, i_c134ba),
    (0xC134BC, i_c134bc),
    (0xC134BE, i_c134be),
    (0xC134C0, i_c134c0),
    (0xC134C2, i_c134c2),
    (0xC134C4, i_c134c4),
    (0xC134C6, i_c134c6),
    (0xC134C8, i_c134c8),
    (0xC134CA, i_c134ca),
    (0xC134CE, i_c134ce),
    (0xC134D0, i_c134d0),
    (0xC134D4, i_c134d4),
    (0xC134DA, i_c134da),
    (0xC134DC, i_c134dc),
    (0xC134E0, i_c134e0),
    (0xC134E2, i_c134e2),
    (0xC134E4, i_c134e4),
    (0xC134E8, i_c134e8),
    (0xC134EC, i_c134ec),
    (0xC134EE, i_c134ee),
    (0xC134F2, i_c134f2),
    (0xC134F4, i_c134f4),
    (0xC134F8, i_c134f8),
    (0xC134FC, i_c134fc),
    (0xC134FE, i_c134fe),
    (0xC13500, i_c13500),
    (0xC13504, i_c13504),
    (0xC13506, i_c13506),
    (0xC13508, i_c13508),
    (0xC1350C, i_c1350c),
    (0xC1350E, i_c1350e),
    (0xC13510, i_c13510),
    (0xC13514, i_c13514),
    (0xC13516, i_c13516),
    (0xC1351A, i_c1351a),
    (0xC1351C, i_c1351c),
    (0xC1351E, i_c1351e),
    (0xC13522, i_c13522),
    (0xC13524, i_c13524),
    (0xC13528, i_c13528),
    (0xC1352A, i_c1352a),
    (0xC1352E, i_c1352e),
    (0xC13530, i_c13530),
    (0xC13534, i_c13534),
    (0xC13538, i_c13538),
    (0xC1353A, i_c1353a),
    (0xC1353C, i_c1353c),
    (0xC13540, i_c13540),
    (0xC13542, i_c13542),
    (0xC13544, i_c13544),
    (0xC13548, i_c13548),
    (0xC1354A, i_c1354a),
    (0xC1354C, i_c1354c),
    (0xC13550, i_c13550),
    (0xC13552, i_c13552),
    (0xC13556, i_c13556),
    (0xC13558, i_c13558),
    (0xC1355A, i_c1355a),
    (0xC1355C, i_c1355c),
    (0xC1355E, i_c1355e),
    (0xC13562, i_c13562),
    (0xC13568, i_c13568),
    (0xC1356C, i_c1356c),
    (0xC1356E, i_c1356e),
    (0xC13570, i_c13570),
    (0xC13572, i_c13572),
    (0xC13576, i_c13576),
    (0xC13578, i_c13578),
    (0xC1357A, i_c1357a),
    (0xC1357C, i_c1357c),
    (0xC1357E, i_c1357e),
    (0xC13582, i_c13582),
    (0xC13588, i_c13588),
    (0xC1358C, i_c1358c),
    (0xC1358E, i_c1358e),
    (0xC13590, i_c13590),
    (0xC13592, i_c13592),
    (0xC13594, i_c13594),
    (0xC13598, i_c13598),
    (0xC1359A, i_c1359a),
    (0xC1359C, i_c1359c),
    (0xC1359E, i_c1359e),
    (0xC135A0, i_c135a0),
    (0xC135A2, i_c135a2),
    (0xC135A6, i_c135a6),
    (0xC135A8, i_c135a8),
    (0xC135AA, i_c135aa),
    (0xC135AC, i_c135ac),
    (0xC135AE, i_c135ae),
    (0xC135B2, i_c135b2),
    (0xC135B8, i_c135b8),
    (0xC135BA, i_c135ba),
    (0xC135BE, i_c135be),
    (0xC135C0, i_c135c0),
    (0xC135C2, i_c135c2),
    (0xC135C4, i_c135c4),
    (0xC135C6, i_c135c6),
    (0xC135C8, i_c135c8),
    (0xC135CC, i_c135cc),
    (0xC135CE, i_c135ce),
    (0xC135D0, i_c135d0),
    (0xC135D2, i_c135d2),
    (0xC135D4, i_c135d4),
    (0xC135D6, i_c135d6),
    (0xC135DA, i_c135da),
    (0xC135DC, i_c135dc),
    (0xC135DE, i_c135de),
    (0xC135E0, i_c135e0),
    (0xC135E2, i_c135e2),
    (0xC135E4, i_c135e4),
    (0xC135E6, i_c135e6),
    (0xC135EA, i_c135ea),
    (0xC135EC, i_c135ec),
    (0xC135EE, i_c135ee),
    (0xC135F0, i_c135f0),
    (0xC135F2, i_c135f2),
    (0xC135F4, i_c135f4),
    (0xC135F6, i_c135f6),
    (0xC135FA, i_c135fa),
    (0xC135FC, i_c135fc),
    (0xC135FE, i_c135fe),
    (0xC13600, i_c13600),
    (0xC13602, i_c13602),
    (0xC13604, i_c13604),
    (0xC13606, i_c13606),
    (0xC1360A, i_c1360a),
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
    (0xC13A0C, i_c13a0c),
    (0xC13A0E, i_c13a0e),
    (0xC13A10, i_c13a10),
    (0xC13A14, i_c13a14),
    (0xC13A18, i_c13a18),
    (0xC13A1A, i_c13a1a),
    (0xC13A1C, i_c13a1c),
    (0xC13A1E, i_c13a1e),
    (0xC13A22, i_c13a22),
    (0xC13A24, i_c13a24),
    (0xC13A26, i_c13a26),
    (0xC13A28, i_c13a28),
    (0xC13A2A, i_c13a2a),
    (0xC13A2C, i_c13a2c),
    (0xC13A30, i_c13a30),
    (0xC13A32, i_c13a32),
    (0xC13A36, i_c13a36),
    (0xC13A38, i_c13a38),
    (0xC13A3C, i_c13a3c),
    (0xC13A42, i_c13a42),
    (0xC13A44, i_c13a44),
    (0xC13A48, i_c13a48),
    (0xC13A4C, i_c13a4c),
    (0xC13A50, i_c13a50),
    (0xC13A52, i_c13a52),
    (0xC13A54, i_c13a54),
    (0xC13A58, i_c13a58),
    (0xC13A5A, i_c13a5a),
    (0xC13A5E, i_c13a5e),
    (0xC13A60, i_c13a60),
    (0xC13A62, i_c13a62),
    (0xC13A68, i_c13a68),
    (0xC13A6E, i_c13a6e),
    (0xC13A70, i_c13a70),
    (0xC13A74, i_c13a74),
    (0xC13A76, i_c13a76),
    (0xC13A78, i_c13a78),
    (0xC13A7C, i_c13a7c),
    (0xC13A80, i_c13a80),
    (0xC13A82, i_c13a82),
    (0xC13A86, i_c13a86),
    (0xC13A88, i_c13a88),
    (0xC13A8C, i_c13a8c),
    (0xC13A8E, i_c13a8e),
    (0xC13A92, i_c13a92),
    (0xC13A94, i_c13a94),
    (0xC13A96, i_c13a96),
    (0xC13A9A, i_c13a9a),
    (0xC13A9C, i_c13a9c),
    (0xC13A9E, i_c13a9e),
    (0xC13AA0, i_c13aa0),
    (0xC13AA2, i_c13aa2),
    (0xC13AA4, i_c13aa4),
    (0xC13AA8, i_c13aa8),
    (0xC13AAE, i_c13aae),
    (0xC13AB0, i_c13ab0),
    (0xC13AB4, i_c13ab4),
    (0xC13AB6, i_c13ab6),
    (0xC13AB8, i_c13ab8),
    (0xC13ABE, i_c13abe),
    (0xC13AC0, i_c13ac0),
    (0xC13AC2, i_c13ac2),
    (0xC13AC6, i_c13ac6),
    (0xC13ACA, i_c13aca),
    (0xC13ACC, i_c13acc),
    (0xC13AD0, i_c13ad0),
    (0xC13AD2, i_c13ad2),
    (0xC13AD8, i_c13ad8),
    (0xC13ADA, i_c13ada),
    (0xC13ADE, i_c13ade),
    (0xC13AE0, i_c13ae0),
    (0xC13AE2, i_c13ae2),
    (0xC13AE8, i_c13ae8),
    (0xC13AEA, i_c13aea),
    (0xC13AEC, i_c13aec),
    (0xC13AF0, i_c13af0),
    (0xC13AF4, i_c13af4),
    (0xC13AF6, i_c13af6),
    (0xC13AF8, i_c13af8),
    (0xC13AFA, i_c13afa),
    (0xC13AFC, i_c13afc),
    (0xC13AFE, i_c13afe),
    (0xC13B04, i_c13b04),
    (0xC13B06, i_c13b06),
    (0xC13B08, i_c13b08),
    (0xC13B0A, i_c13b0a),
    (0xC13B0C, i_c13b0c),
    (0xC13B0E, i_c13b0e),
    (0xC13B12, i_c13b12),
    (0xC13B14, i_c13b14),
    (0xC13B16, i_c13b16),
    (0xC13B1C, i_c13b1c),
    (0xC13B1E, i_c13b1e),
    (0xC13B20, i_c13b20),
    (0xC13B22, i_c13b22),
    (0xC13B24, i_c13b24),
    (0xC13B26, i_c13b26),
    (0xC13B2A, i_c13b2a),
    (0xC13B2C, i_c13b2c),
    (0xC13B2E, i_c13b2e),
    (0xC13B32, i_c13b32),
    (0xC13B36, i_c13b36),
    (0xC13B38, i_c13b38),
    (0xC13B3A, i_c13b3a),
    (0xC13B3C, i_c13b3c),
    (0xC13B42, i_c13b42),
    (0xC13B46, i_c13b46),
    (0xC13B4A, i_c13b4a),
    (0xC13B4E, i_c13b4e),
    (0xC13B50, i_c13b50),
    (0xC13B54, i_c13b54),
    (0xC13B56, i_c13b56),
    (0xC13B58, i_c13b58),
    (0xC13B5E, i_c13b5e),
    (0xC13B64, i_c13b64),
    (0xC13B6A, i_c13b6a),
    (0xC13B6C, i_c13b6c),
    (0xC13B70, i_c13b70),
    (0xC13B72, i_c13b72),
    (0xC13B74, i_c13b74),
    (0xC13B78, i_c13b78),
    (0xC13B7C, i_c13b7c),
    (0xC13B7E, i_c13b7e),
    (0xC13B82, i_c13b82),
    (0xC13B84, i_c13b84),
    (0xC13B88, i_c13b88),
    (0xC13B8A, i_c13b8a),
    (0xC13B8E, i_c13b8e),
    (0xC13B90, i_c13b90),
    (0xC13B92, i_c13b92),
    (0xC13B96, i_c13b96),
    (0xC13B98, i_c13b98),
    (0xC13B9A, i_c13b9a),
    (0xC13B9C, i_c13b9c),
    (0xC13B9E, i_c13b9e),
    (0xC13BA0, i_c13ba0),
    (0xC13BA4, i_c13ba4),
    (0xC13BAA, i_c13baa),
    (0xC13BAC, i_c13bac),
    (0xC13BB0, i_c13bb0),
    (0xC13BB2, i_c13bb2),
    (0xC13BB4, i_c13bb4),
    (0xC13BBA, i_c13bba),
    (0xC13BBC, i_c13bbc),
    (0xC13BBE, i_c13bbe),
    (0xC13BC2, i_c13bc2),
    (0xC13BC6, i_c13bc6),
    (0xC13BC8, i_c13bc8),
    (0xC13BCC, i_c13bcc),
    (0xC13BCE, i_c13bce),
    (0xC13BD4, i_c13bd4),
    (0xC13BD6, i_c13bd6),
    (0xC13BDA, i_c13bda),
    (0xC13BDC, i_c13bdc),
    (0xC13BDE, i_c13bde),
    (0xC13BE4, i_c13be4),
    (0xC13BE6, i_c13be6),
    (0xC13BE8, i_c13be8),
    (0xC13BEC, i_c13bec),
    (0xC13BF0, i_c13bf0),
    (0xC13BF2, i_c13bf2),
    (0xC13BF4, i_c13bf4),
    (0xC13BFA, i_c13bfa),
    (0xC13BFC, i_c13bfc),
    (0xC13BFE, i_c13bfe),
    (0xC13C02, i_c13c02),
    (0xC13C04, i_c13c04),
    (0xC13C06, i_c13c06),
    (0xC13C0C, i_c13c0c),
    (0xC13C0E, i_c13c0e),
    (0xC13C10, i_c13c10),
    (0xC13C12, i_c13c12),
    (0xC13C16, i_c13c16),
    (0xC13C1A, i_c13c1a),
    (0xC13C1C, i_c13c1c),
    (0xC13C1E, i_c13c1e),
    (0xC13C20, i_c13c20),
    (0xC13C24, i_c13c24),
    (0xC13C26, i_c13c26),
    (0xC13C2A, i_c13c2a),
    (0xC13C2C, i_c13c2c),
    (0xC13C2E, i_c13c2e),
    (0xC13C34, i_c13c34),
    (0xC13C3A, i_c13c3a),
    (0xC13C40, i_c13c40),
    (0xC13C42, i_c13c42),
    (0xC13C46, i_c13c46),
    (0xC13C48, i_c13c48),
    (0xC13C4A, i_c13c4a),
    (0xC13C4E, i_c13c4e),
    (0xC13C52, i_c13c52),
    (0xC13C54, i_c13c54),
    (0xC13C58, i_c13c58),
    (0xC13C5A, i_c13c5a),
    (0xC13C5E, i_c13c5e),
    (0xC13C60, i_c13c60),
    (0xC13C64, i_c13c64),
    (0xC13C66, i_c13c66),
    (0xC13C68, i_c13c68),
    (0xC13C6C, i_c13c6c),
    (0xC13C6E, i_c13c6e),
    (0xC13C70, i_c13c70),
    (0xC13C72, i_c13c72),
    (0xC13C74, i_c13c74),
    (0xC13C76, i_c13c76),
    (0xC13C7A, i_c13c7a),
    (0xC13C80, i_c13c80),
    (0xC13C82, i_c13c82),
    (0xC13C84, i_c13c84),
    (0xC13C88, i_c13c88),
    (0xC13C8A, i_c13c8a),
    (0xC13C8C, i_c13c8c),
    (0xC13C92, i_c13c92),
    (0xC13C94, i_c13c94),
    (0xC13C96, i_c13c96),
    (0xC13C98, i_c13c98),
    (0xC13C9C, i_c13c9c),
    (0xC13CA0, i_c13ca0),
    (0xC13CA2, i_c13ca2),
    (0xC13CA6, i_c13ca6),
    (0xC13CA8, i_c13ca8),
    (0xC13CAE, i_c13cae),
    (0xC13CB0, i_c13cb0),
    (0xC13CB2, i_c13cb2),
    (0xC13CB6, i_c13cb6),
    (0xC13CB8, i_c13cb8),
    (0xC13CBA, i_c13cba),
    (0xC13CC0, i_c13cc0),
    (0xC13CC2, i_c13cc2),
    (0xC13CC4, i_c13cc4),
    (0xC13CC6, i_c13cc6),
    (0xC13CCA, i_c13cca),
    (0xC13CCE, i_c13cce),
    (0xC13CD0, i_c13cd0),
    (0xC13CD2, i_c13cd2),
    (0xC13CD8, i_c13cd8),
    (0xC13CDA, i_c13cda),
    (0xC13CDC, i_c13cdc),
    (0xC13CDE, i_c13cde),
    (0xC13CE2, i_c13ce2),
    (0xC13CE4, i_c13ce4),
    (0xC13CE6, i_c13ce6),
    (0xC13CEC, i_c13cec),
    (0xC13CEE, i_c13cee),
    (0xC13CF0, i_c13cf0),
    (0xC13CF2, i_c13cf2),
    (0xC13CF4, i_c13cf4),
    (0xC13CF8, i_c13cf8),
    (0xC13CFC, i_c13cfc),
    (0xC13CFE, i_c13cfe),
    (0xC13D00, i_c13d00),
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
    (0xC13D2A, i_c13d2a),
    (0xC13D2E, i_c13d2e),
    (0xC13D30, i_c13d30),
    (0xC13D34, i_c13d34),
    (0xC13D36, i_c13d36),
    (0xC13D3A, i_c13d3a),
    (0xC13D3E, i_c13d3e),
    (0xC13D42, i_c13d42),
    (0xC13D44, i_c13d44),
    (0xC13D4A, i_c13d4a),
    (0xC13D4E, i_c13d4e),
    (0xC13D52, i_c13d52),
    (0xC13D56, i_c13d56),
    (0xC13D5A, i_c13d5a),
    (0xC13D5E, i_c13d5e),
    (0xC13D62, i_c13d62),
    (0xC13D64, i_c13d64),
    (0xC13D66, i_c13d66),
    (0xC13D6A, i_c13d6a),
    (0xC13D6E, i_c13d6e),
    (0xC13D72, i_c13d72),
    (0xC13D76, i_c13d76),
    (0xC13D7A, i_c13d7a),
    (0xC13D7E, i_c13d7e),
    (0xC13D84, i_c13d84),
    (0xC13D86, i_c13d86),
    (0xC13D8A, i_c13d8a),
    (0xC13D8E, i_c13d8e),
    (0xC13D94, i_c13d94),
    (0xC13D96, i_c13d96),
    (0xC13D9A, i_c13d9a),
    (0xC13D9E, i_c13d9e),
    (0xC13DA0, i_c13da0),
    (0xC13DA4, i_c13da4),
    (0xC13DA8, i_c13da8),
    (0xC13DAC, i_c13dac),
    (0xC13DB2, i_c13db2),
    (0xC13DB6, i_c13db6),
    (0xC13DBA, i_c13dba),
    (0xC13DBC, i_c13dbc),
    (0xC13DC0, i_c13dc0),
    (0xC13DC4, i_c13dc4),
    (0xC13DC8, i_c13dc8),
    (0xC13DCC, i_c13dcc),
    (0xC13DCE, i_c13dce),
    (0xC13DD2, i_c13dd2),
    (0xC13DD4, i_c13dd4),
    (0xC13DD8, i_c13dd8),
    (0xC13DDC, i_c13ddc),
    (0xC13DE0, i_c13de0),
    (0xC13DE6, i_c13de6),
    (0xC13DEC, i_c13dec),
    (0xC13DF0, i_c13df0),
    (0xC13DF2, i_c13df2),
    (0xC13DF6, i_c13df6),
    (0xC13DFC, i_c13dfc),
    (0xC13E00, i_c13e00),
    (0xC13E02, i_c13e02),
    (0xC13E08, i_c13e08),
    (0xC13E0C, i_c13e0c),
    (0xC13E10, i_c13e10),
    (0xC13E14, i_c13e14),
    (0xC13E18, i_c13e18),
    (0xC13E1C, i_c13e1c),
    (0xC13E22, i_c13e22),
    (0xC13E24, i_c13e24),
    (0xC13E28, i_c13e28),
    (0xC13E2C, i_c13e2c),
    (0xC13E32, i_c13e32),
    (0xC13E34, i_c13e34),
    (0xC13E38, i_c13e38),
    (0xC13E3C, i_c13e3c),
    (0xC13E3E, i_c13e3e),
    (0xC13E42, i_c13e42),
    (0xC13E44, i_c13e44),
    (0xC13E48, i_c13e48),
    (0xC13E4C, i_c13e4c),
    (0xC13E4E, i_c13e4e),
    (0xC13E52, i_c13e52),
    (0xC13E54, i_c13e54),
    (0xC13E58, i_c13e58),
    (0xC13E5E, i_c13e5e),
    (0xC13E64, i_c13e64),
    (0xC13E66, i_c13e66),
    (0xC13E6C, i_c13e6c),
    (0xC13E72, i_c13e72),
    (0xC13E74, i_c13e74),
    (0xC13E7C, i_c13e7c),
    (0xC13E82, i_c13e82),
    (0xC13E8C, i_c13e8c),
    (0xC13E90, i_c13e90),
    (0xC13E94, i_c13e94),
    (0xC13E98, i_c13e98),
    (0xC13E9C, i_c13e9c),
    (0xC13EA0, i_c13ea0),
    (0xC13EA4, i_c13ea4),
    (0xC13EAA, i_c13eaa),
    (0xC13EAE, i_c13eae),
    (0xC13EB2, i_c13eb2),
    (0xC13EB6, i_c13eb6),
    (0xC13EBA, i_c13eba),
    (0xC13EBE, i_c13ebe),
    (0xC13EC0, i_c13ec0),
    (0xC13EC6, i_c13ec6),
    (0xC13ECA, i_c13eca),
    (0xC13ECE, i_c13ece),
    (0xC13ED2, i_c13ed2),
    (0xC13ED6, i_c13ed6),
    (0xC13ED8, i_c13ed8),
    (0xC13EDC, i_c13edc),
    (0xC13EE0, i_c13ee0),
    (0xC13EE4, i_c13ee4),
    (0xC13EE8, i_c13ee8),
    (0xC13EEC, i_c13eec),
    (0xC13EEE, i_c13eee),
    (0xC13EF2, i_c13ef2),
    (0xC13EF6, i_c13ef6),
    (0xC13EF8, i_c13ef8),
    (0xC13EFC, i_c13efc),
    (0xC13F00, i_c13f00),
    (0xC13F04, i_c13f04),
    (0xC13F08, i_c13f08),
    (0xC13F0A, i_c13f0a),
    (0xC13F0E, i_c13f0e),
    (0xC13F10, i_c13f10),
    (0xC13F12, i_c13f12),
    (0xC13F16, i_c13f16),
    (0xC13F18, i_c13f18),
    (0xC13F1C, i_c13f1c),
    (0xC13F1E, i_c13f1e),
    (0xC13F22, i_c13f22),
    (0xC13F24, i_c13f24),
    (0xC13F28, i_c13f28),
    (0xC13F2C, i_c13f2c),
    (0xC13F30, i_c13f30),
    (0xC13F32, i_c13f32),
    (0xC13F34, i_c13f34),
    (0xC13F36, i_c13f36),
    (0xC14092, i_c14092),
    (0xC14096, i_c14096),
    (0xC1409A, i_c1409a),
    (0xC1409E, i_c1409e),
    (0xC140A2, i_c140a2),
    (0xC140A6, i_c140a6),
    (0xC140AA, i_c140aa),
    (0xC140AC, i_c140ac),
    (0xC140B0, i_c140b0),
    (0xC140B2, i_c140b2),
    (0xC140BA, i_c140ba),
    (0xC140BE, i_c140be),
    (0xC140C4, i_c140c4),
    (0xC140C8, i_c140c8),
    (0xC140CC, i_c140cc),
    (0xC140CE, i_c140ce),
    (0xC140D2, i_c140d2),
    (0xC140D6, i_c140d6),
    (0xC140DA, i_c140da),
    (0xC140DE, i_c140de),
    (0xC140E2, i_c140e2),
    (0xC140E6, i_c140e6),
    (0xC140EA, i_c140ea),
    (0xC140EC, i_c140ec),
    (0xC140F2, i_c140f2),
    (0xC140F6, i_c140f6),
    (0xC140F8, i_c140f8),
    (0xC140FC, i_c140fc),
    (0xC14100, i_c14100),
    (0xC14102, i_c14102),
    (0xC14106, i_c14106),
    (0xC1410A, i_c1410a),
    (0xC1410C, i_c1410c),
    (0xC14110, i_c14110),
    (0xC14114, i_c14114),
    (0xC14118, i_c14118),
    (0xC1411C, i_c1411c),
    (0xC14120, i_c14120),
    (0xC14124, i_c14124),
    (0xC14128, i_c14128),
    (0xC1412A, i_c1412a),
    (0xC1412E, i_c1412e),
    (0xC14130, i_c14130),
    (0xC14134, i_c14134),
    (0xC14138, i_c14138),
    (0xC1413C, i_c1413c),
    (0xC14140, i_c14140),
    (0xC14144, i_c14144),
    (0xC14148, i_c14148),
    (0xC1414A, i_c1414a),
    (0xC1414C, i_c1414c),
    (0xC14150, i_c14150),
    (0xC14152, i_c14152),
    (0xC14156, i_c14156),
    (0xC1415A, i_c1415a),
    (0xC1415C, i_c1415c),
    (0xC14160, i_c14160),
    (0xC14162, i_c14162),
    (0xC14164, i_c14164),
    (0xC1416A, i_c1416a),
    (0xC1416C, i_c1416c),
    (0xC14170, i_c14170),
    (0xC14172, i_c14172),
    (0xC14174, i_c14174),
    (0xC14178, i_c14178),
    (0xC1417A, i_c1417a),
    (0xC1417E, i_c1417e),
    (0xC14180, i_c14180),
    (0xC14182, i_c14182),
    (0xC14186, i_c14186),
    (0xC1418A, i_c1418a),
    (0xC1418E, i_c1418e),
    (0xC14192, i_c14192),
    (0xC14196, i_c14196),
    (0xC1419A, i_c1419a),
    (0xC1419C, i_c1419c),
];
